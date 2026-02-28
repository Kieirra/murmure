use crate::audio::helpers::{cleanup_recordings, ensure_recordings_dir, generate_unique_wav_name};
use crate::audio::pipeline::process_recording;
use crate::audio::recorder::AudioRecorder;
use crate::audio::types::{AudioState, RecordingMode, RecordingTrigger};
use crate::clipboard;
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::engine::{ParakeetEngine, ParakeetModelParams};
use crate::model::Model;
use crate::overlay::overlay;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

pub fn record_audio(app: &AppHandle, mode: RecordingMode) {
    let state = app.state::<AudioState>();
    state.set_recording_mode(mode);
    if state.get_recording_trigger() != RecordingTrigger::WakeWord {
        state.set_recording_trigger(RecordingTrigger::Keyboard);
        crate::wake_word::pause_listener(app);
    }
    // Wake word triggers: listener stays active for cancel word detection

    if matches!(mode, RecordingMode::Llm | RecordingMode::Command) {
        crate::llm::warmup_ollama_model_background(app);
    }

    internal_record_audio(app);
}

fn internal_record_audio(app: &AppHandle) {
    debug!("Starting audio recording...");
    let state = app.state::<AudioState>();

    // Check if already recording
    if state.recorder.lock().is_some() {
        warn!("Already recording");
        return;
    }

    let recordings_dir = match ensure_recordings_dir(app) {
        Ok(dir) => dir,
        Err(e) => {
            error!("Failed to initialize recordings directory: {}", e);
            return;
        }
    };

    let file_name = generate_unique_wav_name();
    let file_path = recordings_dir.join(&file_name);
    *state.current_file_name.lock() = Some(file_name.clone());

    // Get the shared limit_reached flag
    let limit_reached = state.get_limit_reached_arc();

    match AudioRecorder::new(app.clone(), &file_path, limit_reached) {
        Ok(mut recorder) => {
            if let Err(e) = recorder.start() {
                error!("Failed to start recording: {}", e);
                return;
            }
            *state.recorder.lock() = Some(recorder);
            debug!("Recording started");

            // Emit the recording mode to the overlay for visual differentiation
            // This is emitted regardless of overlay visibility setting
            let mode_str = match state.get_recording_mode() {
                RecordingMode::Standard => "standard",
                RecordingMode::Llm => "llm",
                RecordingMode::Command => "command",
            };
            let _ = app.emit("overlay-mode", mode_str);

            let s = crate::settings::load_settings(app);
            if s.overlay_mode.as_str() == "recording" {
                overlay::show_recording_overlay(app);
            }
        }
        Err(e) => {
            error!("Failed to initialize recorder: {}", e);
        }
    }
}

pub fn stop_recording(app: &AppHandle) -> Option<std::path::PathBuf> {
    debug!("Stopping audio recording...");
    let state = app.state::<AudioState>();

    // Stop recorder
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop() {
                error!("Failed to stop recorder: {}", e);
            }
        }
        *recorder_guard = None;
    }

    let file_name_opt = state.current_file_name.lock().take();

    if let Some(file_name) = file_name_opt {
        let path = ensure_recordings_dir(app)
            .map(|dir| dir.join(&file_name))
            .ok();

        if let Some(ref p) = path {
            info!(
                "Audio recording stopped; file written to temporary path: {}",
                p.display()
            );

            // Process recording (Transcribe -> LLM -> History)
            match process_recording(app, p) {
                Ok(final_text) => {
                    let text = match state.strip_word.lock().take() {
                        Some(word) => strip_trailing_wake_word(&final_text, &word),
                        None => final_text,
                    };
                    if let Err(e) = write_transcription(app, &text) {
                        error!("Failed to use clipboard: {}", e);
                    }
                }
                Err(e) => {
                    error!("Processing failed: {}", e);
                }
            }
        }

        // Reset UI
        let _ = app.emit("mic-level", 0.0f32);
        // Reset overlay mode to standard for next recording
        let _ = app.emit("overlay-mode", "standard");
        let s = crate::settings::load_settings(app);
        if s.overlay_mode.as_str() == "recording" {
            overlay::hide_recording_overlay(app);
        }

        // Reset recording trigger and resume wake word listener
        state.set_recording_trigger(RecordingTrigger::Keyboard);
        crate::wake_word::resume_listener(app);

        return path;
    } else {
        debug!("Recording stopped (no active file)");
    }

    state.set_recording_trigger(RecordingTrigger::Keyboard);
    crate::wake_word::resume_listener(app);

    None
}

pub fn cancel_recording(app: &AppHandle) {
    info!("Cancelling audio recording...");
    let state = app.state::<AudioState>();

    // Stop recorder without processing
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop() {
                error!("Failed to stop recorder on cancel: {}", e);
            }
        }
        *recorder_guard = None;
    }

    // Remove temporary WAV file
    let file_name_opt = state.current_file_name.lock().take();
    if let Some(file_name) = file_name_opt {
        if let Ok(dir) = ensure_recordings_dir(app) {
            let path = dir.join(&file_name);
            if let Err(e) = std::fs::remove_file(&path) {
                error!("Failed to remove cancelled recording file: {}", e);
            }
        }
    }

    // Reset UI
    let _ = app.emit("mic-level", 0.0f32);
    let _ = app.emit("overlay-mode", "standard");
    let s = crate::settings::load_settings(app);
    if s.overlay_mode.as_str() == "recording" {
        overlay::hide_recording_overlay(app);
    }

    state.set_recording_trigger(RecordingTrigger::Keyboard);
    crate::wake_word::resume_listener(app);

    info!("Recording cancelled by user");
}

pub fn write_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    let state = app.state::<AudioState>();
    let trigger = state.get_recording_trigger();
    let mode = state.get_recording_mode();

    if let Err(e) = clipboard::paste(transcription, app) {
        error!("Failed to paste text: {}", e);
    }

    // Auto-enter: only for wake word trigger, non-Command mode, when setting enabled
    if trigger == RecordingTrigger::WakeWord && mode != RecordingMode::Command {
        let settings = crate::settings::load_settings(app);
        if settings.auto_enter_after_wake_word {
            if let Err(e) = simulate_enter_key() {
                error!("Failed to simulate Enter key: {}", e);
            } else {
                debug!("Auto-enter: Enter key simulated after wake word transcription");
            }
        }
    }

    if let Err(e) = cleanup_recordings(app) {
        error!("Failed to cleanup recordings: {}", e);
    } else {
        info!("Temporary audio files successfully cleaned up");
    }

    debug!("Transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn simulate_enter_key() -> Result<(), String> {
    use enigo::{Enigo, Key, Keyboard, Settings};

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    enigo
        .key(Key::Return, enigo::Direction::Click)
        .map_err(|e| format!("Failed to press Enter: {}", e))?;

    Ok(())
}

fn strip_trailing_wake_word(text: &str, wake_word: &str) -> String {
    let ww = wake_word.trim();
    if ww.is_empty() {
        return text.to_string();
    }

    let trimmed = text.trim();
    let text_words: Vec<&str> = trimmed.split_whitespace().collect();
    let ww_words: Vec<&str> = ww.split_whitespace().collect();

    if text_words.len() < ww_words.len() {
        return trimmed.to_string();
    }

    let start = text_words.len() - ww_words.len();
    let candidate = &text_words[start..];

    let matches = candidate.iter().zip(ww_words.iter()).all(|(tw, ww_w)| {
        let tw_clean = tw.trim_end_matches(|c: char| !c.is_alphanumeric());
        tw_clean.eq_ignore_ascii_case(ww_w)
    });

    if matches {
        let result = text_words[..start].join(" ");
        debug!(
            "Stripped trailing wake word \"{}\" from transcription",
            wake_word
        );
        result
    } else {
        trimmed.to_string()
    }
}

pub fn write_last_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    if let Err(e) = clipboard::paste_last_transcript(transcription, app) {
        error!("Failed to paste last transcription: {}", e);
    }

    debug!("Last transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn preload_engine(app: &AppHandle) -> Result<()> {
    let state = app.state::<AudioState>();
    let mut engine = state.engine.lock();

    if engine.is_none() {
        let model = app.state::<Arc<Model>>();
        let model_path = model
            .get_model_path()
            .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

        let mut new_engine = ParakeetEngine::new();
        new_engine
            .load_model_with_params(&model_path, ParakeetModelParams::int8())
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        *engine = Some(new_engine);
        info!("Model loaded and cached in memory");
    }

    Ok(())
}
