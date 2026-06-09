use ndarray::{Array, Array1, Array2, ArrayD, ArrayViewD, IxDyn};
use once_cell::sync::Lazy;
use ort::execution_providers::CPUExecutionProvider;
use ort::inputs;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::TensorRef;
use regex::Regex;

use std::fs;
use std::path::Path;

use super::boost_tree::BoostTree;
use super::helpers::{load_tokenizer, tokenize_word_to_ids, word_variants};
use super::types::{DecoderState, ParakeetError, ParakeetModel, TimestampedResult};

const SUBSAMPLING_FACTOR: usize = 8;
const WINDOW_SIZE: f32 = 0.01;
const MAX_TOKENS_PER_STEP: usize = 10;

// Fusion weight bounds applied to boost scores before argmax. Alpha decays
// with dictionary size: more words means more first-tokens armed at the root,
// so we lower the volume to keep false positives down on large dictionaries.
const BOOST_ALPHA_MAX: f32 = 3.5;
const BOOST_ALPHA_MIN: f32 = 1.0;
// A dictionary token is only boosted when it already ranks within the top-K of
// the raw logits. In greedy decoding only near-misses are recoverable, so the
// gate stays tight to avoid spurious dictionary insertions.
const BOOST_TOP_K: usize = 5;

fn degressive_alpha(word_count: usize) -> f32 {
    (BOOST_ALPHA_MAX - (word_count as f32 / 5.0).log10()).clamp(BOOST_ALPHA_MIN, BOOST_ALPHA_MAX)
}

fn in_top_k(logits: &[f32], token: usize, k: usize) -> bool {
    match logits.get(token) {
        Some(&target) => logits.iter().filter(|&&l| l > target).count() < k,
        None => false,
    }
}

static DECODE_SPACE_RE: Lazy<Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"\A\s|\s\B|(\s)\b"));

fn argmax_token(logits: &[f32], blank_idx: i32) -> i32 {
    logits
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(idx, _)| idx as i32)
        .unwrap_or(blank_idx)
}

impl Drop for ParakeetModel {
    fn drop(&mut self) {
        log::debug!(
            "Dropping ParakeetModel with {} vocab tokens",
            self.vocab.len()
        );
    }
}

impl ParakeetModel {
    pub fn new<P: AsRef<Path>>(model_dir: P, quantized: bool) -> Result<Self, ParakeetError> {
        let encoder = Self::init_session(&model_dir, "encoder-model", None, quantized)?;
        let decoder_joint = Self::init_session(&model_dir, "decoder_joint-model", None, quantized)?;
        let preprocessor = Self::init_session(&model_dir, "nemo128", None, false)?;

        let (vocab, blank_idx) = Self::load_vocab(&model_dir)?;
        let vocab_size = vocab.len();

        log::trace!(
            "Loaded vocabulary with {} tokens, blank_idx={}",
            vocab_size,
            blank_idx
        );

        let tokenizer = load_tokenizer(model_dir.as_ref());

        Ok(Self {
            encoder,
            decoder_joint,
            preprocessor,
            vocab,
            blank_idx,
            vocab_size,
            tokenizer,
            boost_tree: None,
            boost_alpha: BOOST_ALPHA_MAX,
        })
    }

    /// Rebuild the phrase-boosting automaton from the user dictionary words.
    /// Each word is expanded into casing/accent variants (see `word_variants`),
    /// every tokenizable variant becoming an independent boosted phrase.
    /// Variants that cannot be tokenized are skipped; an empty result clears it.
    pub fn set_boost_words(&mut self, words: &[String]) {
        let tokenizer = match self.tokenizer.as_ref() {
            Some(tokenizer) => tokenizer,
            None => {
                self.boost_tree = None;
                return;
            }
        };
        let mut phrases: Vec<Vec<i32>> = Vec::new();
        for word in words {
            let variants = word_variants(word);
            let mut tokenized: Vec<(String, Vec<i32>)> = Vec::new();
            for variant in variants {
                if let Some(ids) = tokenize_word_to_ids(tokenizer, &variant) {
                    tokenized.push((variant, ids));
                }
            }
            if log::log_enabled!(log::Level::Debug) {
                let detail: Vec<(&str, Vec<&str>)> = tokenized
                    .iter()
                    .map(|(v, ids)| {
                        (
                            v.as_str(),
                            ids.iter().map(|&id| self.token_str(id)).collect(),
                        )
                    })
                    .collect();
                log::debug!("Boost word {:?} -> variants {:?}", word, detail);
            }
            phrases.extend(tokenized.into_iter().map(|(_, ids)| ids));
        }

        // Alpha is driven by the dictionary size, not the variant count: more
        // variants per word must not lower the boost volume.
        self.boost_tree = if phrases.is_empty() {
            None
        } else {
            self.boost_alpha = degressive_alpha(words.len());
            log::debug!(
                "Phrase boosting active for {} word(s), {} phrase(s), alpha={}",
                words.len(),
                phrases.len(),
                self.boost_alpha
            );
            Some(BoostTree::new(&phrases))
        };
    }

    fn token_str(&self, id: i32) -> &str {
        self.vocab
            .get(id as usize)
            .map(String::as_str)
            .unwrap_or("?")
    }

    // Diagnostic only: one compact line per emitted token listing the 5
    // best-ranked dictionary candidates and their raw rank (* = boosted, i.e.
    // within the top-K gate), to see where a word's tokens really stand.
    fn log_boost_step(&self, logits: &[f32], candidates: &[(i32, f32)], emitted: i32) {
        let rank_of = |tok: i32| {
            let raw = logits
                .get(tok as usize)
                .copied()
                .unwrap_or(f32::NEG_INFINITY);
            logits.iter().filter(|&&l| l > raw).count()
        };
        let mut ranked: Vec<(i32, usize)> = candidates
            .iter()
            .map(|&(tok, _)| (tok, rank_of(tok)))
            .collect();
        ranked.sort_by_key(|&(_, rank)| rank);
        let shown: Vec<String> = ranked
            .iter()
            .take(5)
            .map(|&(tok, rank)| {
                let mark = if in_top_k(logits, tok as usize, BOOST_TOP_K) {
                    "*"
                } else {
                    ""
                };
                format!("{:?}#{}{}", self.token_str(tok), rank, mark)
            })
            .collect();
        log::debug!(
            "boost: out={:?} | cand {}",
            self.token_str(emitted),
            shown.join(" ")
        );
    }

    fn init_session<P: AsRef<Path>>(
        model_dir: P,
        model_name: &str,
        intra_threads: Option<usize>,
        try_quantized: bool,
    ) -> Result<Session, ParakeetError> {
        let providers = vec![CPUExecutionProvider::default().build()];

        // Try quantized version first if requested, fallback to regular version
        let model_filename = if try_quantized {
            let quantized_name = format!("{}.int8.onnx", model_name);
            let quantized_path = model_dir.as_ref().join(&quantized_name);
            if quantized_path.exists() {
                log::trace!("Loading quantized model from {}...", quantized_name);
                quantized_name
            } else {
                let regular_name = format!("{}.onnx", model_name);
                log::trace!(
                    "Quantized model not found, loading regular model from {}...",
                    regular_name
                );
                regular_name
            }
        } else {
            let regular_name = format!("{}.onnx", model_name);
            log::trace!("Loading model from {}...", regular_name);
            regular_name
        };

        let mut builder = Session::builder()?
            .with_config_entry("session.log_severity_level", "3")?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_execution_providers(providers)?
            .with_memory_pattern(false)?
            .with_parallel_execution(false)?;

        if let Some(threads) = intra_threads {
            builder = builder
                .with_intra_threads(threads)?
                .with_inter_threads(threads)?;
        }

        let session = builder.commit_from_file(model_dir.as_ref().join(&model_filename))?;

        for input in &session.inputs {
            log::trace!(
                "Model '{}' input: name={}, type={:?}",
                model_filename,
                input.name,
                input.input_type
            );
        }

        Ok(session)
    }

    fn load_vocab<P: AsRef<Path>>(model_dir: P) -> Result<(Vec<String>, i32), ParakeetError> {
        let vocab_path = model_dir.as_ref().join("vocab.txt");
        let content = fs::read_to_string(vocab_path)?;

        let mut max_id = 0;
        let mut tokens_with_ids: Vec<(String, usize)> = Vec::new();
        let mut blank_idx: Option<usize> = None;

        for line in content.lines() {
            let parts: Vec<&str> = line.trim_end().split(' ').collect();
            if parts.len() >= 2 {
                let token = parts[0].to_string();
                if let Ok(id) = parts[1].parse::<usize>() {
                    if token == "<blk>" {
                        blank_idx = Some(id);
                    }
                    tokens_with_ids.push((token, id));
                    max_id = max_id.max(id);
                }
            }
        }

        // Create vocab vector with \u2581 replaced with space
        let mut vocab = vec![String::new(); max_id + 1];
        for (token, id) in tokens_with_ids {
            vocab[id] = token.replace('\u{2581}', " ");
        }

        let blank_idx = blank_idx.ok_or_else(|| {
            ParakeetError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing <blk> token in vocabulary",
            ))
        })? as i32;

        Ok((vocab, blank_idx))
    }

    pub fn preprocess(
        &mut self,
        waveforms: &ArrayViewD<f32>,
        waveforms_lens: &ArrayViewD<i64>,
    ) -> Result<(ArrayD<f32>, ArrayD<i64>), ParakeetError> {
        log::trace!("Running preprocessor inference...");
        let inputs = inputs![
            "waveforms" => TensorRef::from_array_view(waveforms.view())?,
            "waveforms_lens" => TensorRef::from_array_view(waveforms_lens.view())?,
        ];
        let outputs = self.preprocessor.run(inputs)?;

        let features = outputs
            .get("features")
            .ok_or_else(|| ParakeetError::OutputNotFound("features".to_string()))?
            .try_extract_array()?;
        let features_lens = outputs
            .get("features_lens")
            .ok_or_else(|| ParakeetError::OutputNotFound("features_lens".to_string()))?
            .try_extract_array()?;

        Ok((features.to_owned(), features_lens.to_owned()))
    }

    pub fn encode(
        &mut self,
        audio_signal: &ArrayViewD<f32>,
        length: &ArrayViewD<i64>,
    ) -> Result<(ArrayD<f32>, ArrayD<i64>), ParakeetError> {
        log::trace!("Running encoder inference...");
        let inputs = inputs![
            "audio_signal" => TensorRef::from_array_view(audio_signal.view())?,
            "length" => TensorRef::from_array_view(length.view())?,
        ];
        let outputs = self.encoder.run(inputs)?;

        let encoder_output = outputs
            .get("outputs")
            .ok_or_else(|| ParakeetError::OutputNotFound("outputs".to_string()))?
            .try_extract_array()?;
        let encoded_lengths = outputs
            .get("encoded_lengths")
            .ok_or_else(|| ParakeetError::OutputNotFound("encoded_lengths".to_string()))?
            .try_extract_array()?;

        let encoder_output = encoder_output.permuted_axes(IxDyn(&[0, 2, 1]));

        Ok((encoder_output.to_owned(), encoded_lengths.to_owned()))
    }

    pub fn create_decoder_state(&self) -> Result<DecoderState, ParakeetError> {
        // Get input shapes from decoder model
        let inputs = &self.decoder_joint.inputs;

        let state1_shape = inputs
            .iter()
            .find(|input| input.name == "input_states_1")
            .ok_or_else(|| ParakeetError::InputNotFound("input_states_1".to_string()))?
            .input_type
            .tensor_shape()
            .ok_or_else(|| ParakeetError::TensorShape("input_states_1".to_string()))?;

        let state2_shape = inputs
            .iter()
            .find(|input| input.name == "input_states_2")
            .ok_or_else(|| ParakeetError::InputNotFound("input_states_2".to_string()))?
            .input_type
            .tensor_shape()
            .ok_or_else(|| ParakeetError::TensorShape("input_states_2".to_string()))?;

        // Create zero states with batch_size=1
        // Shape is [2, -1, 640] so we use [2, 1, 640] for batch_size=1
        let state1 = Array::zeros((
            state1_shape[0] as usize,
            1, // batch_size = 1
            state1_shape[2] as usize,
        ));

        let state2 = Array::zeros((
            state2_shape[0] as usize,
            1, // batch_size = 1
            state2_shape[2] as usize,
        ));

        Ok((state1, state2))
    }

    pub fn decode_step(
        &mut self,
        prev_tokens: &[i32],
        prev_state: &DecoderState,
        encoder_out: &ArrayViewD<f32>, // [time_steps, 1024]
    ) -> Result<(ArrayD<f32>, DecoderState), ParakeetError> {
        log::trace!("Running decoder inference...");

        // Get last token or blank_idx if empty
        let target_token = prev_tokens.last().copied().unwrap_or(self.blank_idx);

        // Prepare inputs matching Python: encoder_out[None, :, None] -> [1, time_steps, 1]
        let encoder_outputs = encoder_out
            .to_owned()
            .insert_axis(ndarray::Axis(0))
            .insert_axis(ndarray::Axis(2));
        let targets = Array2::from_shape_vec((1, 1), vec![target_token])?;
        let target_length = Array1::from_vec(vec![1]);

        let inputs = inputs![
            "encoder_outputs" => TensorRef::from_array_view(encoder_outputs.view())?,
            "targets" => TensorRef::from_array_view(targets.view())?,
            "target_length" => TensorRef::from_array_view(target_length.view())?,
            "input_states_1" => TensorRef::from_array_view(prev_state.0.view())?,
            "input_states_2" => TensorRef::from_array_view(prev_state.1.view())?,
        ];

        let outputs = self.decoder_joint.run(inputs)?;

        let logits = outputs
            .get("outputs")
            .ok_or_else(|| ParakeetError::OutputNotFound("outputs".to_string()))?
            .try_extract_array()?;
        log::trace!(
            "Logits shape: {:?}, vocab_size: {}",
            logits.shape(),
            self.vocab_size
        );
        let state1 = outputs
            .get("output_states_1")
            .ok_or_else(|| ParakeetError::OutputNotFound("output_states_1".to_string()))?
            .try_extract_array()?;
        let state2 = outputs
            .get("output_states_2")
            .ok_or_else(|| ParakeetError::OutputNotFound("output_states_2".to_string()))?
            .try_extract_array()?;

        // Squeeze outputs like Python (remove batch dimension)
        let logits = logits.remove_axis(ndarray::Axis(0));

        // Convert ArrayD back to Array3 to match expected return type
        let state1_3d = state1.to_owned().into_dimensionality::<ndarray::Ix3>()?;
        let state2_3d = state2.to_owned().into_dimensionality::<ndarray::Ix3>()?;

        Ok((logits.to_owned(), (state1_3d, state2_3d)))
    }

    pub fn recognize_batch(
        &mut self,
        waveforms: &ArrayViewD<f32>,
        waveforms_len: &ArrayViewD<i64>,
    ) -> Result<Vec<TimestampedResult>, ParakeetError> {
        // Preprocess and encode
        let (features, features_lens) = self.preprocess(waveforms, waveforms_len)?;
        let (encoder_out, encoder_out_lens) =
            self.encode(&features.view(), &features_lens.view())?;

        // Decode for each batch item
        let mut results = Vec::new();
        for (encodings, &encodings_len) in encoder_out.outer_iter().zip(encoder_out_lens.iter()) {
            let (tokens, timestamps) =
                self.decode_sequence(&encodings.view(), encodings_len as usize)?;
            let result = self.decode_tokens(tokens, timestamps);
            results.push(result);
        }

        Ok(results)
    }

    fn decode_sequence(
        &mut self,
        encodings: &ArrayViewD<f32>, // [time_steps, 1024]
        encodings_len: usize,
    ) -> Result<(Vec<i32>, Vec<usize>), ParakeetError> {
        // Logit boosting only runs when a boost tree is active. With no boost
        // tree the greedy path must stay bit-exact (default path, streaming),
        // so it lives in its own function with no boost code on it.
        match self.boost_tree.is_some() {
            true => self.decode_sequence_boosted(encodings, encodings_len),
            false => self.decode_sequence_greedy(encodings, encodings_len),
        }
    }

    // Pure greedy TDT decode, no boosting. Bit-exact reference path.
    fn decode_sequence_greedy(
        &mut self,
        encodings: &ArrayViewD<f32>,
        encodings_len: usize,
    ) -> Result<(Vec<i32>, Vec<usize>), ParakeetError> {
        let mut prev_state = self.create_decoder_state()?;
        let mut tokens = Vec::new();
        let mut timestamps = Vec::new();

        let mut t = 0;
        let mut emitted_tokens = 0;

        while t < encodings_len {
            let encoder_step = encodings.slice(ndarray::s![t, ..]);
            let encoder_step_dyn = encoder_step.to_owned().into_dyn();
            let (probs, new_state) =
                self.decode_step(&tokens, &prev_state, &encoder_step_dyn.view())?;

            let vocab_logits = Self::vocab_logits(&probs, self.vocab_size)?;
            let token = argmax_token(vocab_logits, self.blank_idx);

            if token != self.blank_idx {
                prev_state = new_state;
                tokens.push(token);
                timestamps.push(t);
                emitted_tokens += 1;
            }

            if token == self.blank_idx || emitted_tokens == MAX_TOKENS_PER_STEP {
                t += 1;
                emitted_tokens = 0;
            }
        }

        Ok((tokens, timestamps))
    }

    // Extract the vocabulary logits slice from a (possibly TDT) decoder output.
    fn vocab_logits(probs: &ArrayD<f32>, vocab_size: usize) -> Result<&[f32], ParakeetError> {
        let slice = probs.as_slice().ok_or_else(|| {
            ParakeetError::Shape(ndarray::ShapeError::from_kind(
                ndarray::ErrorKind::IncompatibleShape,
            ))
        })?;
        if probs.len() > vocab_size {
            Ok(&slice[..vocab_size])
        } else {
            Ok(slice)
        }
    }

    // Greedy decode with dictionary logit boosting. At each frame the boost
    // tree biases the expected dictionary tokens on the non-blank logits (gated
    // by the top-K guard) before the argmax; the boost state advances along the
    // trie with every emitted token. One decode_step per frame, no parallel
    // hypotheses.
    fn decode_sequence_boosted(
        &mut self,
        encodings: &ArrayViewD<f32>,
        encodings_len: usize,
    ) -> Result<(Vec<i32>, Vec<usize>), ParakeetError> {
        let tree = match self.boost_tree.take() {
            Some(tree) => tree,
            None => return self.decode_sequence_greedy(encodings, encodings_len),
        };

        let result = self.run_boosted_decode(encodings, encodings_len, &tree);
        self.boost_tree = Some(tree);
        result
    }

    fn run_boosted_decode(
        &mut self,
        encodings: &ArrayViewD<f32>,
        encodings_len: usize,
        tree: &BoostTree,
    ) -> Result<(Vec<i32>, Vec<usize>), ParakeetError> {
        let mut prev_state = self.create_decoder_state()?;
        let mut tokens: Vec<i32> = Vec::new();
        let mut timestamps: Vec<usize> = Vec::new();

        let mut t = 0;
        let mut emitted_tokens = 0;
        let mut boost_state = tree.root();

        while t < encodings_len {
            let encoder_step = encodings.slice(ndarray::s![t, ..]);
            let encoder_step_dyn = encoder_step.to_owned().into_dyn();

            let (probs, new_state) =
                self.decode_step(&tokens, &prev_state, &encoder_step_dyn.view())?;
            let vocab_logits = Self::vocab_logits(&probs, self.vocab_size)?.to_vec();

            let candidates = tree.bias(boost_state);
            let mut boosted = vocab_logits.clone();
            for &(boost_token, score) in &candidates {
                let idx = boost_token as usize;
                if boost_token != self.blank_idx
                    && idx < boosted.len()
                    && in_top_k(&vocab_logits, idx, BOOST_TOP_K)
                {
                    boosted[idx] += self.boost_alpha * score;
                }
            }
            let token = argmax_token(&boosted, self.blank_idx);
            if log::log_enabled!(log::Level::Debug)
                && token != self.blank_idx
                && !candidates.is_empty()
            {
                self.log_boost_step(&vocab_logits, &candidates, token);
            }

            if token != self.blank_idx {
                prev_state = new_state;
                tokens.push(token);
                timestamps.push(t);
                emitted_tokens += 1;
                boost_state = tree.advance(boost_state, token);
            }

            if token == self.blank_idx || emitted_tokens == MAX_TOKENS_PER_STEP {
                t += 1;
                emitted_tokens = 0;
            }
        }

        Ok((tokens, timestamps))
    }

    fn decode_tokens(&self, ids: Vec<i32>, timestamps: Vec<usize>) -> TimestampedResult {
        let tokens: Vec<String> = ids
            .iter()
            .filter_map(|&id| {
                let idx = id as usize;
                if idx < self.vocab.len() {
                    Some(self.vocab[idx].clone())
                } else {
                    None
                }
            })
            .collect();

        let text = match &*DECODE_SPACE_RE {
            Ok(regex) => regex
                .replace_all(&tokens.join(""), |caps: &regex::Captures| {
                    if caps.get(1).is_some() {
                        " "
                    } else {
                        ""
                    }
                })
                .to_string(),
            Err(_) => tokens.join(""), // Fallback if regex failed to compile
        };

        let float_timestamps: Vec<f32> = timestamps
            .iter()
            .map(|&t| WINDOW_SIZE * SUBSAMPLING_FACTOR as f32 * t as f32)
            .collect();

        TimestampedResult {
            text,
            timestamps: float_timestamps,
            tokens,
        }
    }

    pub fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
    ) -> Result<TimestampedResult, ParakeetError> {
        let batch_size = 1;
        let samples_len = samples.len();

        // Create waveforms array [batch_size, samples_len]
        let waveforms = Array2::from_shape_vec((batch_size, samples_len), samples)?.into_dyn();

        // Create waveforms_lens array [batch_size] with the actual length
        let waveforms_lens = Array1::from_vec(vec![samples_len as i64]).into_dyn();

        // Run recognition to get detailed results
        let results = self.recognize_batch(&waveforms.view(), &waveforms_lens.view())?;

        // Extract the first (and only) result
        let timestamped_result = results.into_iter().next().ok_or_else(|| {
            ParakeetError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No transcription result returned",
            ))
        })?;

        Ok(timestamped_result)
    }
}

// TranscriptionEngine trait implementation
use super::helpers::convert_timestamps;
use super::transcription_engine::{TranscriptionEngine, TranscriptionResult};
use super::types::{
    ParakeetEngine, ParakeetInferenceParams, ParakeetModelParams, QuantizationType,
};
use std::path::Path as StdPath;

impl ParakeetEngine {
    pub fn set_boost_words(&mut self, words: &[String]) {
        if let Some(model) = self.model.as_mut() {
            model.set_boost_words(words);
        }
    }
}

impl TranscriptionEngine for ParakeetEngine {
    type InferenceParams = ParakeetInferenceParams;
    type ModelParams = ParakeetModelParams;

    fn load_model_with_params(
        &mut self,
        model_path: &StdPath,
        params: Self::ModelParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let quantized = match params.quantization {
            QuantizationType::FP32 => false,
            QuantizationType::Int8 => true,
        };
        let model = ParakeetModel::new(model_path, quantized)?;

        self.model = Some(model);
        self.loaded_model_path = Some(model_path.to_path_buf());
        Ok(())
    }

    fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
        params: Option<Self::InferenceParams>,
    ) -> Result<TranscriptionResult, Box<dyn std::error::Error>> {
        let model: &mut ParakeetModel = self
            .model
            .as_mut()
            .ok_or("Model not loaded. Call load_model_with_params() first.")?;

        let parakeet_params = params.unwrap_or_default();

        // Get the timestamped result from the model
        let timestamped_result = model.transcribe_samples(samples)?;

        // Convert timestamps based on requested granularity
        let segments =
            convert_timestamps(&timestamped_result, parakeet_params.timestamp_granularity);

        Ok(TranscriptionResult {
            text: timestamped_result.text,
            segments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degressive_alpha_decays_with_word_count() {
        assert_eq!(degressive_alpha(5), 3.5);
        assert_eq!(degressive_alpha(50), 2.5);
        assert_eq!(degressive_alpha(500), 1.5);
        assert_eq!(degressive_alpha(50000), 1.0);
    }

    #[test]
    fn degressive_alpha_is_clamped_at_both_ends() {
        assert_eq!(degressive_alpha(1), 3.5);
        assert_eq!(degressive_alpha(5000), 1.0);
    }

    #[test]
    fn in_top_k_accepts_ranked_token_and_rejects_outranked() {
        let logits = [0.1, 5.0, 2.0, 3.0];
        assert!(in_top_k(&logits, 1, 1));
        assert!(in_top_k(&logits, 3, 2));
        assert!(!in_top_k(&logits, 0, 3));
    }
}
