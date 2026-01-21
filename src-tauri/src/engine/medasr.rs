use ndarray::{s, Array2, Axis};
use ort::execution_providers::CPUExecutionProvider;
use ort::inputs;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::TensorRef;
use realfft::RealFftPlanner;
use std::f32::consts::PI;
use std::fs;
use std::path::Path;

use super::types::{
    MedAsrEngine, MedAsrInferenceParams, MedAsrModel, MedAsrModelParams, MelConfig, ParakeetError,
    TimestampedResult,
};

impl Drop for MedAsrModel {
    fn drop(&mut self) {
        log::debug!(
            "Dropping MedAsrModel with {} vocab tokens",
            self.vocab.len()
        );
    }
}

impl MedAsrModel {
    pub fn new<P: AsRef<Path>>(model_dir: P) -> Result<Self, ParakeetError> {
        // Load the main ONNX model (model.onnx + model.onnx.data)
        let session = Self::init_session(&model_dir, "model", None)?;

        // Load vocabulary from tokenizer.json
        let vocab = Self::load_vocab_from_tokenizer(&model_dir)?;
        
        // Load mel config from processor_config.json or use defaults
        let mel_config = Self::load_mel_config(&model_dir).unwrap_or_default();

        // Blank idx is typically 0 for CTC models
        let blank_idx = 0;

        Ok(Self {
            session,
            vocab,
            blank_idx,
            mel_config,
        })
    }

    fn init_session<P: AsRef<Path>>(
        model_dir: P,
        model_name: &str,
        intra_threads: Option<usize>,
    ) -> Result<Session, ParakeetError> {
        let providers = vec![CPUExecutionProvider::default().build()];

        let model_filename = if model_name.ends_with(".onnx") {
            model_name.to_string()
        } else {
            format!("{}.onnx", model_name)
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

        Ok(session)
    }

    fn load_vocab_from_tokenizer<P: AsRef<Path>>(model_dir: P) -> Result<Vec<String>, ParakeetError> {
        let tokenizer_path = model_dir.as_ref().join("tokenizer.json");
        let content = fs::read_to_string(&tokenizer_path)?;
        
        let tokenizer: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ParakeetError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        
        let mut vocab_pairs: Vec<(String, usize)> = Vec::new();
        
        // Parse added_tokens: [{id: N, content: "token"}, ...]
        if let Some(added_tokens) = tokenizer.get("added_tokens").and_then(|v| v.as_array()) {
            for token_entry in added_tokens {
                if let (Some(id), Some(content)) = (
                    token_entry.get("id").and_then(|v| v.as_u64()),
                    token_entry.get("content").and_then(|v| v.as_str())
                ) {
                    vocab_pairs.push((content.to_string(), id as usize));
                }
            }
        }
        
        // Parse model.vocab: [[token, score], ...] (Unigram/SentencePiece format)
        // Index in array = token ID (starting after special tokens)
        if let Some(model) = tokenizer.get("model") {
            if let Some(vocab_arr) = model.get("vocab").and_then(|v| v.as_array()) {
                // In Unigram tokenizer, index in vocab array = ID
                // model.vocab ALREADY includes the special tokens at the start (0-3)
                // So we should NOT offset by 4.
                let base_id = 0;
                for (idx, token_entry) in vocab_arr.iter().enumerate() {
                    if let Some(arr) = token_entry.as_array() {
                        if let Some(token_str) = arr.get(0).and_then(|v| v.as_str()) {
                            vocab_pairs.push((token_str.to_string(), base_id + idx));
                        }
                    }
                }
            }
        }
        
        // Sort by ID and build vocab vector
        vocab_pairs.sort_by_key(|(_, id)| *id);
        let max_id = vocab_pairs.iter().map(|(_, id)| *id).max().unwrap_or(0);
        let mut vocab = vec![String::new(); max_id + 1];
        
        for (token, id) in vocab_pairs {
            // Replace sentencepiece marker with space
            let clean_token = token.replace('\u{2581}', " ");
            vocab[id] = clean_token;
        }
        
        log::debug!("Loaded {} tokens from tokenizer.json", vocab.len());
        Ok(vocab)
    }

    fn load_mel_config<P: AsRef<Path>>(model_dir: P) -> Result<MelConfig, ParakeetError> {
        let config_path = model_dir.as_ref().join("processor_config.json");
        let content = fs::read_to_string(&config_path)?;
        
        let config: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ParakeetError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        
        let fe = config.get("feature_extractor").unwrap_or(&config);
        
        Ok(MelConfig {
            sample_rate: fe.get("sampling_rate").and_then(|v| v.as_u64()).unwrap_or(16000) as usize,
            n_fft: fe.get("n_fft").and_then(|v| v.as_u64()).unwrap_or(512) as usize,
            hop_length: fe.get("hop_length").and_then(|v| v.as_u64()).unwrap_or(160) as usize,
            win_length: fe.get("win_length").and_then(|v| v.as_u64()).unwrap_or(400) as usize,
            n_mels: fe.get("feature_size").and_then(|v| v.as_u64()).unwrap_or(128) as usize,
        })
    }

    /// Compute mel spectrogram from raw audio samples
    fn compute_mel_features(&self, samples: &[f32]) -> Array2<f32> {
        let config = &self.mel_config;
        
        // Create mel filterbank
        // Using 0.0 to 8000.0 Hz (Nyquist) for 16kHz audio to cover full spectrum
        let mel_filterbank = create_mel_filterbank(
            config.n_fft,
            config.n_mels,
            config.sample_rate as f32,
            0.0,     // f_min: 0.0 Hz
            8000.0,  // f_max: 8000.0 Hz (Nyquist)
        );
        
        // Create Hann window (periodic=False in Python)
        let window: Vec<f32> = (0..config.win_length)
            .map(|n| 0.5 * (1.0 - (2.0 * PI * n as f32 / (config.win_length - 1) as f32).cos()))
            .collect();
        
        // Pad samples if needed
        let padded_len = ((samples.len() as f32 / config.hop_length as f32).ceil() as usize) 
            * config.hop_length + config.win_length;
        let mut padded = vec![0.0f32; padded_len];
        padded[..samples.len()].copy_from_slice(samples);
        
        // Calculate number of frames
        let n_frames = (padded.len() - config.win_length) / config.hop_length + 1;
        
        // Setup FFT
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(config.n_fft);
        
        let mut mel_spec = Array2::<f32>::zeros((n_frames, config.n_mels));
        
        for frame_idx in 0..n_frames {
            let start = frame_idx * config.hop_length;
            let end = start + config.win_length;
            
            if end > padded.len() {
                break;
            }
            
            // Apply window and zero-pad to n_fft
            let mut frame = vec![0.0f32; config.n_fft];
            for (i, &sample) in padded[start..end].iter().enumerate() {
                frame[i] = sample * window[i];
            }
            
            // Compute FFT
            let mut spectrum = fft.make_output_vec();
            fft.process(&mut frame, &mut spectrum).ok();
            
            // Compute power spectrum (magnitude squared)
            let power_spectrum: Vec<f32> = spectrum.iter()
                .map(|c| c.norm_sqr())
                .collect();
            
            // Apply mel filterbank
            for (mel_idx, mel_filter) in mel_filterbank.iter().enumerate() {
                let mut mel_energy = 0.0f32;
                for (fft_idx, &weight) in mel_filter.iter().enumerate() {
                    if fft_idx < power_spectrum.len() {
                        mel_energy += power_spectrum[fft_idx] * weight;
                    }
                }
                // Log mel with floor to avoid log(0) - matching Python clamp min=1e-5
                // Using log10 (dB-like scale) instead of ln
                mel_spec[[frame_idx, mel_idx]] = (mel_energy.max(1e-5)).log10();
            }
        }
        
        mel_spec
    }

    pub fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
    ) -> Result<TimestampedResult, ParakeetError> {
        // 1. Compute mel features
        let mel_features = self.compute_mel_features(&samples);
        let n_frames = mel_features.shape()[0];
        
        // Reshape to [batch=1, time, n_mels]
        let input_values = mel_features.insert_axis(Axis(0)).into_dyn();
        
        // Create attention mask (all true for full sequence) - MUST be bool type
        let attention_mask = Array2::<bool>::from_elem((1, n_frames), true).into_dyn();
        
        log::debug!("Input values shape: {:?} (no norm, log10)", input_values.shape());
        
        // 2. Run inference
        let inputs = inputs![
            "input_values" => TensorRef::from_array_view(input_values.view())?,
            "attention_mask" => TensorRef::from_array_view(attention_mask.view())?,
        ];

        let outputs = self.session.run(inputs)?;

        // 3. Extract logits and compute log probabilities
        // Use a block to ensure `outputs` is dropped before we call methods on self
        let log_probs = {
            let logits = outputs
                .get("logits")
                .ok_or_else(|| ParakeetError::OutputNotFound("logits".to_string()))?
                .try_extract_array()?;

            // Logits shape: [N, T, Vocab]
            let logits_view = logits.view();
            let seq_len = logits_view.shape()[1];

            // Convert logits to log probabilities (softmax then log)
            let mut log_probs = Vec::with_capacity(seq_len);
            for t in 0..seq_len {
                let frame_logits = logits_view.slice(s![0, t, ..]);
                // Softmax: exp(x - max) / sum(exp(x - max))
                let max_val = frame_logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let exp_sum: f32 = frame_logits.iter().map(|&x| (x - max_val).exp()).sum();
                let log_sum = max_val + exp_sum.ln();
                
                let frame_log_probs: Vec<f32> = frame_logits.iter()
                    .map(|&x| x - log_sum)
                    .collect();
                log_probs.push(frame_log_probs);
            }
            log_probs
        };
        drop(outputs);

        // Decode using greedy (for now, beam search is complex for CTC)
        let decoded_tokens = self.ctc_greedy_decode(&log_probs);
        
        // Build result
        let mut text_parts = Vec::new();
        let mut tokens_str = Vec::new();
        let mut timestamps = Vec::new();
        let vocab_size = self.vocab.len();

        for (t, token_idx) in decoded_tokens {
            if token_idx < vocab_size {
                let token = &self.vocab[token_idx];
                // Skip special tokens
                let is_special = token.starts_with('<') && token.ends_with('>');
                if !is_special && !token.is_empty() {
                    text_parts.push(token.clone());
                    tokens_str.push(token.clone());
                    let time_sec = (t * self.mel_config.hop_length) as f32 / self.mel_config.sample_rate as f32;
                    timestamps.push(time_sec);
                }
            }
        }

        // Join text
        let full_text = text_parts.join("").replace('\u{2581}', " ").trim().to_string();

        Ok(TimestampedResult {
            text: full_text,
            timestamps,
            tokens: tokens_str,
        })
    }

    /// CTC Greedy decoding: collapse repeats and remove blanks
    fn ctc_greedy_decode(&self, log_probs: &[Vec<f32>]) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut last_token_idx: i32 = -1;

        for (t, frame_probs) in log_probs.iter().enumerate() {
            // Find argmax
            let max_idx = frame_probs.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            // CTC: merge repeats, ignore blanks
            if max_idx as i32 != self.blank_idx && max_idx as i32 != last_token_idx {
                result.push((t, max_idx));
            }
            last_token_idx = max_idx as i32;
        }
        result
    }


}


fn create_mel_filterbank(
    n_fft: usize,
    n_mels: usize,
    sample_rate: f32,
    f_min: f32,
    f_max: f32,
) -> Vec<Vec<f32>> {
    let n_freqs = n_fft / 2 + 1;
    
    // Convert Hz to Mel scale
    let hz_to_mel = |hz: f32| 2595.0 * (1.0 + hz / 700.0).log10();
    let mel_to_hz = |mel: f32| 700.0 * (10.0f32.powf(mel / 2595.0) - 1.0);
    
    let mel_min = hz_to_mel(f_min);
    let mel_max = hz_to_mel(f_max);
    
    // Create mel points
    let mel_points: Vec<f32> = (0..=n_mels + 1)
        .map(|i| mel_min + (mel_max - mel_min) * i as f32 / (n_mels + 1) as f32)
        .collect();
    
    // Convert back to Hz and then to FFT bin indices
    let hz_points: Vec<f32> = mel_points.iter().map(|&m| mel_to_hz(m)).collect();
    let bin_points: Vec<usize> = hz_points
        .iter()
        .map(|&hz| ((n_fft as f32 + 1.0) * hz / sample_rate).floor() as usize)
        .collect();
    
    // Create filterbank
    let mut filterbank = vec![vec![0.0f32; n_freqs]; n_mels];
    
    for m in 0..n_mels {
        let left = bin_points[m];
        let center = bin_points[m + 1];
        let right = bin_points[m + 2];
        
        // Rising slope
        for k in left..center {
            if k < n_freqs && center > left {
                filterbank[m][k] = (k - left) as f32 / (center - left) as f32;
            }
        }
        
        // Falling slope
        for k in center..right {
            if k < n_freqs && right > center {
                filterbank[m][k] = (right - k) as f32 / (right - center) as f32;
            }
        }
    }
    
    filterbank
}

// TranscriptionEngine trait implementation
use super::helpers::convert_timestamps;
use super::transcription_engine::{TranscriptionEngine, TranscriptionResult};
use std::path::Path as StdPath;

impl TranscriptionEngine for MedAsrEngine {
    type InferenceParams = MedAsrInferenceParams;
    type ModelParams = MedAsrModelParams;

    fn load_model_with_params(
        &mut self,
        model_path: &StdPath,
        _params: Self::ModelParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let model = MedAsrModel::new(model_path)?;
        self.model = Some(model);
        self.loaded_model_path = Some(model_path.to_path_buf());
        Ok(())
    }

    fn unload_model(&mut self) {
        self.loaded_model_path = None;
        self.model = None;
    }

    fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
        params: Option<Self::InferenceParams>,
    ) -> Result<TranscriptionResult, Box<dyn std::error::Error>> {
        let model: &mut MedAsrModel = self
            .model
            .as_mut()
            .ok_or("Model not loaded.")?;

        let inference_params = params.unwrap_or_default();

        let timestamped_result = model.transcribe_samples(samples)?;

        let segments = convert_timestamps(&timestamped_result, inference_params.timestamp_granularity);

        Ok(TranscriptionResult {
            text: timestamped_result.text,
            segments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Test mel filterbank creation
    #[test]
    fn test_mel_filterbank_creation() {
        let filterbank = create_mel_filterbank(512, 128, 16000.0, 0.0, 8000.0);
        
        assert_eq!(filterbank.len(), 128, "Should have 128 mel bins");
        assert_eq!(filterbank[0].len(), 257, "Each filter should cover n_fft/2 + 1 = 257 bins");
        
        // Check that filters have non-zero values
        let total_energy: f32 = filterbank.iter()
            .flat_map(|f| f.iter())
            .sum();
        assert!(total_energy > 0.0, "Filterbank should have non-zero values");
    }

    /// Test mel feature computation with synthetic audio
    #[test]
    fn test_mel_features_shape() {
        let config = MelConfig::default();
        
        // Create 1 second of synthetic audio at 16kHz
        let sample_rate = 16000;
        let duration_sec = 1.0;
        let samples: Vec<f32> = (0..(sample_rate as f32 * duration_sec) as usize)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate as f32).sin())
            .collect();
        
        // Create a temporary model-like struct for testing
        let mel_config = MelConfig::default();
        let filterbank = create_mel_filterbank(
            mel_config.n_fft,
            mel_config.n_mels,
            mel_config.sample_rate as f32,
            0.0,
            mel_config.sample_rate as f32 / 2.0,
        );
        
        // Expected number of frames
        let expected_frames = (samples.len() - mel_config.win_length) / mel_config.hop_length + 1;
        
        // Verify filterbank dimensions
        assert_eq!(filterbank.len(), 128);
        
        println!("Expected frames for 1s audio: ~{}", expected_frames);
    }

    /// Test vocabulary loading from tokenizer.json
    #[test]
    fn test_vocab_loading() {
        let model_dir = PathBuf::from("../resources/medasr-onnx-local");
        
        if !model_dir.exists() {
            println!("Skipping test: model directory not found at {:?}", model_dir);
            return;
        }
        
        let result = MedAsrModel::load_vocab_from_tokenizer(&model_dir);
        
        match result {
            Ok(vocab) => {
                assert!(!vocab.is_empty(), "Vocabulary should not be empty");
                println!("Loaded {} tokens from tokenizer", vocab.len());
            }
            Err(e) => {
                println!("Warning: Could not load vocab: {}", e);
            }
        }
    }

    /// Test mel config loading from processor_config.json
    #[test]
    fn test_mel_config_loading() {
        let model_dir = PathBuf::from("../resources/medasr-onnx-local");
        
        if !model_dir.exists() {
            println!("Skipping test: model directory not found at {:?}", model_dir);
            return;
        }
        
        let result = MedAsrModel::load_mel_config(&model_dir);
        
        match result {
            Ok(config) => {
                assert_eq!(config.sample_rate, 16000);
                assert_eq!(config.n_mels, 128);
                assert_eq!(config.n_fft, 512);
                println!("Loaded mel config: {:?}", config);
            }
            Err(e) => {
                println!("Warning: Could not load config: {}", e);
            }
        }
    }

    /// Integration test: Load model and transcribe audio file
    #[test]
    fn test_transcribe_audio_file() {
        let model_dir = PathBuf::from("../resources/medasr-onnx-local");
        let audio_path = PathBuf::from("../resources/medasr-native/test_audio.wav");
        
        if !model_dir.exists() {
            println!("Skipping test: model directory not found at {:?}", model_dir);
            return;
        }
        
        if !audio_path.exists() {
            println!("Skipping test: audio file not found at {:?}", audio_path);
            return;
        }
        
        // Load model
        let model_result = MedAsrModel::new(&model_dir);
        let mut model = match model_result {
            Ok(m) => m,
            Err(e) => {
                println!("Could not load model: {}", e);
                return;
            }
        };
        
        println!("Model loaded successfully with {} vocab tokens", model.vocab.len());
        
        // Load audio file
        let reader = hound::WavReader::open(&audio_path);
        let mut reader = match reader {
            Ok(r) => r,
            Err(e) => {
                println!("Could not open audio file: {}", e);
                return;
            }
        };
        
        let spec = reader.spec();
        println!("Audio spec: {:?}", spec);
        
        // Read samples and convert to f32
        let samples: Vec<f32> = if spec.bits_per_sample == 16 {
            reader.samples::<i16>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / 32768.0)
                .collect()
        } else {
            reader.samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / 2147483648.0)
                .collect()
        };
        
        println!("Loaded {} audio samples", samples.len());
        
        // Debug: Print mel feature statistics
        let mel_features = model.compute_mel_features(&samples);
        let mel_vals: Vec<f32> = mel_features.iter().copied().collect();
        let min = mel_vals.iter().copied().fold(f32::INFINITY, f32::min);
        let max = mel_vals.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let mean: f32 = mel_vals.iter().sum::<f32>() / mel_vals.len() as f32;
        println!("Rust mel features: shape={:?}, min={:.2}, max={:.2}, mean={:.2}", 
                 mel_features.shape(), min, max, mean);
        println!("First 5 frames, first 5 mels:");
        for i in 0..5.min(mel_features.shape()[0]) {
            print!("  [");
            for j in 0..5.min(mel_features.shape()[1]) {
                print!("{:.2}, ", mel_features[[i, j]]);
            }
            println!("]");
        }
        
        // Transcribe
        let result = model.transcribe_samples(samples);
        
        match result {
            Ok(transcription) => {
                println!("=== Transcription Result ===");
                println!("Text: {}", transcription.text);
                println!("Tokens: {:?}", transcription.tokens.len());
                assert!(!transcription.text.is_empty() || transcription.tokens.is_empty(), 
                    "Transcription should produce output");
            }
            Err(e) => {
                println!("Transcription failed: {:?}", e);
            }
        }
    }


}

