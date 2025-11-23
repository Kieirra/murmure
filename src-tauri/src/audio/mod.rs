pub mod helpers;
pub mod pipeline;
pub mod recorder;
pub mod state;

use crate::clipboard;
use crate::engine::engine::{ParakeetEngine, ParakeetModelParams};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::model::Model;
use crate::overlay;
use anyhow::Result;
use pipeline::process_recording;
pub use pipeline::transcribe_audio;
use recorder::AudioRecorder;
use state::AudioState;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use helpers::{cleanup_recordings, ensure_recordings_dir, generate_unique_wav_name};

pub fn record_audio(app: &AppHandle) {
    let state = app.state::<AudioState>();
    state.set_use_llm_shortcut(false);
    internal_record_audio(app);
}

pub fn record_audio_with_llm(app: &AppHandle) {
    let state = app.state::<AudioState>();
    state.set_use_llm_shortcut(true);
    internal_record_audio(app);
}

fn internal_record_audio(app: &AppHandle) {
    println!("Starting audio recording...");
    let state = app.state::<AudioState>();

    // Check if already recording
    if state.recorder.lock().is_some() {
        println!("Already recording");
        return;
    }

    let recordings_dir = match ensure_recordings_dir(app) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to initialize recordings directory: {}", e);
            return;
        }
    };

    let file_name = generate_unique_wav_name();
    let file_path = recordings_dir.join(&file_name);
    *state.current_file_name.lock() = Some(file_name.clone());

    match AudioRecorder::new(app.clone(), &file_path) {
        Ok(recorder) => {
            if let Err(e) = recorder.start() {
                eprintln!("Failed to start recording: {}", e);
                return;
            }
            *state.recorder.lock() = Some(recorder);
            println!("Recording started");

            let s = crate::settings::load_settings(app);
            if s.overlay_mode.as_str() == "recording" {
                overlay::show_recording_overlay(app);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize recorder: {}", e);
        }
    }
}

pub fn stop_recording(app: &AppHandle) -> Option<std::path::PathBuf> {
    println!("Stopping audio recording...");
    let state = app.state::<AudioState>();

    // Stop recorder
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop() {
                eprintln!("Failed to stop recorder: {}", e);
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
            println!("Recording stopped and saved as {}", p.display());

            // Process recording (Transcribe -> LLM -> History)
            match process_recording(app, p) {
                Ok(final_text) => {
                    if let Err(e) = write_transcription(app, &final_text) {
                        eprintln!("Failed to use clipboard: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Processing failed: {}", e);
                }
            }
        }

        // Reset UI
        let _ = app.emit("mic-level", 0.0f32);
        let s = crate::settings::load_settings(app);
        if s.overlay_mode.as_str() == "recording" {
            overlay::hide_recording_overlay(app);
        }

        return path;
    } else {
        println!("Recording stopped (no active file)");
    }
    None
}

pub fn write_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    if let Err(e) = clipboard::paste(transcription, app) {
        eprintln!("Failed to paste text: {}", e);
    }

    if let Err(e) = cleanup_recordings(app) {
        eprintln!("Failed to cleanup recordings: {}", e);
    } else {
        println!("All recordings cleaned up");
    }

    println!("Transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn write_last_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    if let Err(e) = clipboard::paste_last_transcript(transcription, app) {
        eprintln!("Failed to paste last transcription: {}", e);
    }

    println!("Last transcription written to clipboard {}", transcription);
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
        println!("Model loaded and cached in memory");
    }

    Ok(())
}
