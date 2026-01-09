use crate::audio::helpers::read_wav_samples;
use crate::audio::types::AudioState;
use crate::dictionary::{fix_transcription_with_dictionary, get_cc_rules_path, Dictionary};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::engine::ParakeetModelParams;
use crate::formatting_rules;
use crate::history;
use crate::model::Model;
use crate::stats;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

pub fn process_recording(app: &AppHandle, file_path: &Path) -> Result<String> {
    // 1. Transcribe
    let raw_text = transcribe_audio(app, file_path)?;
    debug!("Raw transcription: {}", raw_text);

    if raw_text.trim().is_empty() {
        debug!("Transcription is empty, skipping further processing.");
        return Ok(raw_text);
    }

    // 2. Dictionary & CC Rules
    let text = apply_dictionary_and_rules(app, raw_text)?;
    debug!("Transcription fixed with dictionary: {}", text);

    // 3. LLM Post-processing
    let llm_text = apply_llm_processing(app, text)?;

    // 4. Apply formatting rules
    let final_text = apply_formatting_rules(app, llm_text);
    debug!("Transcription with formatting rules: {}", final_text);

    // 5. Save Stats & History
    save_stats_and_history(app, file_path, &final_text)?;

    Ok(final_text)
}

pub fn transcribe_audio(app: &AppHandle, audio_path: &Path) -> Result<String> {
    let _ = app.emit("llm-processing-start", ());

    let state = app.state::<AudioState>();

    // Ensure engine is loaded
    {
        let mut engine_guard = state.engine.lock();
        if engine_guard.is_none() {
            let model = app.state::<Arc<Model>>();
            let model_path = model
                .get_model_path()
                .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

            let mut new_engine = crate::engine::ParakeetEngine::new();
            new_engine
                .load_model_with_params(&model_path, ParakeetModelParams::int8())
                .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

            *engine_guard = Some(new_engine);
            info!("Model loaded and cached in memory");
        }
    }

    let samples = read_wav_samples(audio_path)?;

    let mut engine_guard = state.engine.lock();
    let engine = engine_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Engine not loaded"))?;

    let result = engine.transcribe_samples(samples, None).map_err(|e| {
        let _ = app.emit("llm-processing-end", ());
        anyhow::anyhow!("Transcription failed: {}", e)
    })?;
    let _ = app.emit("llm-processing-end", ());

    Ok(result.text)
}

fn apply_dictionary_and_rules(app: &AppHandle, text: String) -> Result<String> {
    let cc_rules_path = get_cc_rules_path(app).context("Failed to get CC rules path")?;
    let dictionary = app.state::<Dictionary>().get();

    Ok(fix_transcription_with_dictionary(
        text,
        dictionary,
        cc_rules_path,
    ))
}

fn apply_llm_processing(app: &AppHandle, text: String) -> Result<String> {
    let state = app.state::<AudioState>();
    let use_llm_shortcut = state.get_use_llm_shortcut();
    let force_bypass_llm = !use_llm_shortcut;

    let rt = tokio::runtime::Runtime::new().context("Failed to create tokio runtime")?;

    match rt.block_on(crate::llm::post_process_with_llm(
        app,
        text.clone(),
        force_bypass_llm,
    )) {
        Ok(llm_text) => {
            debug!("Transcription post-processed with LLM: {}", llm_text);
            Ok(llm_text)
        }
        Err(e) => {
            warn!(
                "LLM post-processing failed: {}. Using original transcription.",
                e
            );
            let _ = app.emit("llm-error", e.to_string());
            Ok(text)
        }
    }
}

fn apply_formatting_rules(app: &AppHandle, text: String) -> String {
    match formatting_rules::load(app) {
        Ok(settings) => formatting_rules::apply_formatting(text, &settings),
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
