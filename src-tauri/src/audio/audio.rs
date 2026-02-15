use crate::audio::helpers::{cleanup_recordings, ensure_recordings_dir, generate_unique_wav_name};
use crate::audio::pipeline::process_recording;
use crate::audio::recorder::AudioRecorder;
use crate::audio::types::{AudioState, RecordingMode};
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
                    if let Err(e) = write_transcription(app, &final_text) {
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

        return path;
    } else {
        debug!("Recording stopped (no active file)");
    }
    None
}

pub fn cancel_recording(app: &AppHandle) {
    debug!("Cancelling audio recording...");
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

    // Get and discard the file name without processing
    let file_name_opt = state.current_file_name.lock().take();

    if let Some(file_name) = file_name_opt {
        if let Ok(recordings_dir) = ensure_recordings_dir(app) {
            let file_path = recordings_dir.join(&file_name);
            // Delete the file without processing it
            if let Err(e) = std::fs::remove_file(&file_path) {
                warn!("Failed to delete cancelled recording file: {}", e);
            } else {
                info!("Recording cancelled and file discarded: {}", file_path.display());
            }
        }
    }

    // Play cancel sound
    crate::audio::sound::play_sound(app, crate::audio::sound::Sound::CancelRecording);

    // Reset UI
    let _ = app.emit("mic-level", 0.0f32);
    // Reset overlay mode to standard for next recording
    let _ = app.emit("overlay-mode", "standard");

    // Show overlay (or keep it visible) so the cancel animation is visible,
    // emit the cancel event, then hide after the animation finishes (1500ms).
    let s = crate::settings::load_settings(app);
    let overlay_is_recording_mode = s.overlay_mode.as_str() == "recording";
    overlay::show_recording_overlay(app);
    if let Some(overlay_win) = app.get_webview_window("recording_overlay") {
        let _ = overlay_win.emit("recording-cancelled", ());
    }
    let app_clone = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(700));
        if overlay_is_recording_mode {
            overlay::hide_recording_overlay(&app_clone);
        }
    });
}

pub fn write_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    if let Err(e) = clipboard::paste(transcription, app) {
        error!("Failed to paste text: {}", e);
    }

    if let Err(e) = cleanup_recordings(app) {
        error!("Failed to cleanup recordings: {}", e);
    } else {
        info!("Temporary audio files successfully cleaned up");
    }

    debug!("Transcription written to clipboard {}", transcription);
    Ok(())
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
