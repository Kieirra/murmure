use crate::audio::clean_recording::strip_fillers_and_repeats;
use crate::audio::helpers::{read_wav_samples, resample};
use crate::audio::types::{AudioState, RecordingMode};
use crate::dictionary::{correct_transcription, sync_boost_words, Dictionary};
use crate::engine::transcription_engine::{TranscriptionEngine, TranscriptionResult};
use crate::formatting_rules;
use crate::history;
use crate::model::Model;
use crate::stats;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

pub struct ProcessingResult {
    pub text: String,
    pub llm_error: Option<String>,
}

pub enum ChunkOutcome {
    Text(String),
    Empty,
    Failed,
}

/// Transcribes one chunk in isolation (fresh decoder state)
pub fn process_chunk(app: &AppHandle, samples: Vec<f32>, sample_rate: u32) -> ChunkOutcome {
    // 1. Resample to 16 kHz if needed
    let resampled = if sample_rate != 16000 {
        resample(&samples, sample_rate as usize, 16000)
    } else {
        samples
    };
    if resampled.is_empty() {
        return ChunkOutcome::Empty;
    }

    let dictionary = app.state::<Dictionary>().get();
    let state = app.state::<AudioState>();
    if let Err(e) = ensure_engine_loaded(app, &state) {
        error!("Chunk transcription: engine not available: {}", e);
        return ChunkOutcome::Failed;
    }

    let mut engine_guard = state.engine.lock();
    let Some(engine) = engine_guard.as_mut() else {
        return ChunkOutcome::Failed;
    };

    // 2. Sync dictionary boost words
    sync_boost_words(engine, &dictionary);

    // 3. Transcribe
    match engine.transcribe_samples(resampled, None) {
        Ok(result) => {
            let trimmed = result.text.trim();
            if trimmed.is_empty() {
                ChunkOutcome::Empty
            } else {
                // 4. Dictionary correction
                ChunkOutcome::Text(correct_transcription(
                    trimmed,
                    &dictionary,
                    &result.word_confidences,
                ))
            }
        }
        Err(e) => {
            error!("Chunk transcription failed: {}", e);
            ChunkOutcome::Failed
        }
    }
}

/// Post-processes the concatenated chunks at the end of transcription
pub fn merge_all_chunks(
    app: &AppHandle,
    accumulated: String,
    file_path: &Path,
    mode: RecordingMode,
) -> Result<ProcessingResult> {
    if accumulated.trim().is_empty() {
        return Ok(ProcessingResult {
            text: accumulated,
            llm_error: None,
        });
    }

    // 5. Strip fillers and repeated words
    let text = strip_fillers_and_repeats(&accumulated);
    // 6. LLM post-processing
    let (llm_text, llm_error) = apply_llm_processing_with_error(app, text, mode)?;
    // 7. Apply formatting rules
    let final_text = apply_formatting_rules(app, llm_text);
    // 8. Save stats & history
    save_stats_and_history(app, file_path, &final_text)?;

    Ok(ProcessingResult {
        text: final_text,
        llm_error,
    })
}

pub fn process_whole_recording(app: &AppHandle, file_path: &Path) -> Result<ProcessingResult> {
    // 1. Transcribe
    let result = transcribe_audio(app, file_path)?;
    let raw_text = result.text;
    debug!("Raw transcription: {}", raw_text);

    if raw_text.trim().is_empty() {
        debug!("Transcription is empty, skipping further processing.");
        return Ok(ProcessingResult {
            text: raw_text,
            llm_error: None,
        });
    }

    // 2. Deduplicate repeated words (transcription artifact cleanup)
    let text = strip_fillers_and_repeats(&raw_text);

    // 3. Dictionary correction
    let text = apply_dictionary_correction(app, text, &result.word_confidences)?;
    debug!("Transcription fixed with dictionary: {}", text);

    // 4. LLM Post-processing
    let state = app.state::<AudioState>();
    let (llm_text, llm_error) =
        apply_llm_processing_with_error(app, text, state.get_recording_mode())?;

    // 5. Apply formatting rules
    let final_text = apply_formatting_rules(app, llm_text);
    debug!("Transcription with formatting rules: {}", final_text);

    // 6. Save Stats & History (skipped per long-dictation segment to avoid
    //    flooding the 5-entry history and inflating stats).
    if !state.long_dictation_active.load(Ordering::SeqCst) {
        save_stats_and_history(app, file_path, &final_text)?;
    }

    Ok(ProcessingResult {
        text: final_text,
        llm_error,
    })
}

pub fn transcribe_audio(app: &AppHandle, audio_path: &Path) -> Result<TranscriptionResult> {
    let _ = app.emit("llm-processing-start", ());

    let state = app.state::<AudioState>();
    ensure_engine_loaded(app, &state)?;

    let samples = read_wav_samples(audio_path)?;

    let mut engine_guard = state.engine.lock();
    let engine = engine_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Engine not loaded"))?;

    sync_boost_words(engine, &app.state::<Dictionary>().get());

    let result = engine.transcribe_samples(samples, None).map_err(|e| {
        let _ = app.emit("llm-processing-end", ());
        anyhow::anyhow!("Transcription failed: {}", e)
    })?;
    let _ = app.emit("llm-processing-end", ());

    Ok(result)
}

fn apply_dictionary_correction(
    app: &AppHandle,
    text: String,
    word_confidences: &[(String, f32)],
) -> Result<String> {
    let dictionary = app.state::<Dictionary>().get();
    Ok(correct_transcription(&text, &dictionary, word_confidences))
}

fn apply_llm_processing_with_error(
    app: &AppHandle,
    text: String,
    mode: RecordingMode,
) -> Result<(String, Option<String>)> {
    match mode {
        RecordingMode::Command => {
            debug!("Processing audio in Command mode");
            let selected_text = match crate::clipboard::get_selected_text(app) {
                Ok(s) if !s.trim().is_empty() => Some(s),
                Ok(_) => {
                    warn!("Selected text was empty in command mode");
                    None
                }
                Err(e) => {
                    error!("Failed to capture selected text in command mode: {}", e);
                    None
                }
            };
            let system_prompt = format!(
                r#"You are a text transformation tool, not a conversational assistant.
Your ONLY job: apply the user instruction to the input text and return the result.
DO NOT explain, comment, or add any text beyond the transformation output.

Rules:
- Return ONLY the transformed text
- NO explanations, NO commentary, NO markdown formatting
- If the instruction is unclear or cannot be applied: return the input text UNCHANGED
- Never wrap the output in quotes, code blocks, or additional formatting

User instruction: {}"#,
                text
            );
            let user_prompt = selected_text.unwrap_or_else(|| text.clone());
            match tauri::async_runtime::block_on(crate::llm::process_command_with_llm(
                app,
                system_prompt,
                user_prompt,
            )) {
                Ok(response) => Ok((response, None)),
                Err(e) => {
                    warn!(
                        "Command LLM processing failed: {}. Using original transcription.",
                        e
                    );
                    Ok((text, Some(e.to_string())))
                }
            }
        }
        RecordingMode::Llm => {
            match tauri::async_runtime::block_on(crate::llm::post_process_with_llm(
                app,
                text.clone(),
                false,
            )) {
                Ok(llm_text) => Ok((llm_text, None)),
                Err(e) => {
                    warn!(
                        "LLM post-processing failed: {}. Using original transcription.",
                        e
                    );
                    Ok((text, Some(e.to_string())))
                }
            }
        }
        RecordingMode::Standard => Ok((text, None)),
    }
}

fn apply_llm_processing_with_mode(
    app: &AppHandle,
    text: String,
    mode: RecordingMode,
) -> Result<String> {
    let (result, _) = apply_llm_processing_with_error(app, text, mode)?;
    Ok(result)
}

fn apply_formatting_rules(app: &AppHandle, text: String) -> String {
    match formatting_rules::load(app) {
        Ok(mut settings) => {
            // Short-text correction strips the trailing space and lowercases each
            // segment; in long dictation that would glue successive utterances
            // together. Disable it for the session only, never persisted.
            if app
                .state::<AudioState>()
                .long_dictation_active
                .load(Ordering::SeqCst)
            {
                settings.built_in.short_text_correction = 0;
            }
            formatting_rules::apply_formatting(text, &settings)
        }
        Err(e) => {
            warn!("Failed to load formatting rules: {}. Skipping.", e);
            text
        }
    }
}

fn save_stats_and_history(app: &AppHandle, file_path: &Path, text: &str) -> Result<()> {
    // Calculate duration and size
    let (duration_seconds, wav_size_bytes) = match hound::WavReader::open(file_path) {
        Ok(reader) => {
            let spec = reader.spec();
            let total_samples = reader.duration() as f64;
            let seconds = if spec.sample_rate > 0 {
                total_samples / (spec.sample_rate as f64)
            } else {
                0.0
            };
            let size = std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);
            (seconds, size)
        }
        Err(_) => (0.0, 0),
    };

    let word_count: u64 = text.split_whitespace().filter(|s| !s.is_empty()).count() as u64;

    if let Err(e) = history::add_transcription(app, text.to_string()) {
        error!("Failed to save to history: {}", e);
    }

    if let Err(e) =
        stats::add_transcription_session(app, word_count, duration_seconds, wav_size_bytes)
    {
        error!("Failed to save stats session: {}", e);
    }

    Ok(())
}

/// Process a recording from raw samples (no WAV file), used by SmartMic.
/// Bypasses file I/O and enters the pipeline directly from PCM samples.
pub fn process_recording_from_samples(
    app: &AppHandle,
    samples: Vec<f32>,
    mode: RecordingMode,
) -> Result<String> {
    // 1. Transcribe directly from samples
    let result = transcribe_samples_direct(app, samples)?;
    let raw_text = result.text;

    if raw_text.trim().is_empty() {
        return Ok(raw_text);
    }

    // 2. Deduplicate repeated words
    let text = strip_fillers_and_repeats(&raw_text);

    // 3. Dictionary correction
    let text = apply_dictionary_correction(app, text, &result.word_confidences)?;

    // 4. LLM post-processing (pass mode directly, no global state mutation)
    let llm_text = apply_llm_processing_with_mode(app, text, mode)?;

    // 5. Formatting rules
    let final_text = apply_formatting_rules(app, llm_text);

    // Note: No save_stats_and_history (no WAV file, no duration)
    Ok(final_text)
}

fn transcribe_samples_direct(app: &AppHandle, samples: Vec<f32>) -> Result<TranscriptionResult> {
    let _ = app.emit("llm-processing-start", ());
    let state = app.state::<AudioState>();
    ensure_engine_loaded(app, &state)?;

    let mut engine_guard = state.engine.lock();
    let engine = engine_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Engine not loaded"))?;

    sync_boost_words(engine, &app.state::<Dictionary>().get());

    let result = engine.transcribe_samples(samples, None).map_err(|e| {
        let _ = app.emit("llm-processing-end", ());
        anyhow::anyhow!("Transcription failed: {}", e)
    })?;
    let _ = app.emit("llm-processing-end", ());

    Ok(result)
}

/// Load the transcription engine into the AudioState if not already loaded.
fn ensure_engine_loaded(app: &AppHandle, state: &AudioState) -> Result<()> {
    let mut engine_guard = state.engine.lock();
    if engine_guard.is_none() {
        let model = app.state::<Arc<Model>>();
        let model_path = model
            .get_model_path()
            .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

        let new_engine =
            crate::engine::ParakeetEngine::load_int8(&model_path, model.get_tokenizer_path())
                .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        *engine_guard = Some(new_engine);
        info!("Model loaded and cached in memory");
    }
    Ok(())
}
