use log::{info, warn};
use tauri::{AppHandle, Emitter};

use crate::audio::record_audio;
use crate::audio::types::RecordingMode;
use crate::cli::types::CliCommand;
use crate::shortcuts::shortcuts::{force_cancel_recording, force_stop_recording};
use crate::shortcuts::types::{recording_state, RecordingSource};

/// Reuses the same backend functions as internal shortcuts to guarantee parity.
pub fn dispatch(app: &AppHandle, cmd: &CliCommand) {
    match cmd {
        CliCommand::Transcription => toggle_recording(app, RecordingMode::Standard),
        CliCommand::TranscriptionLlm => toggle_recording(app, RecordingMode::Llm),
        CliCommand::TranscriptionCommand => toggle_recording(app, RecordingMode::Command),
        CliCommand::PasteLast => paste_last(app),
        CliCommand::Cancel => cancel(app),
        CliCommand::VoiceMode => {
            let _ = app.emit("voice-mode-toggle-requested", ());
        }
        CliCommand::LlmMode(n) => {
            // CLI exposes 1-based indices; backend uses 0-based.
            let index = (*n as usize).saturating_sub(1);
            crate::llm::switch_active_mode(app, index);
            info!("CLI: switched to LLM mode {}", n);
        }
        CliCommand::Import { .. } => {
            warn!("cli_dispatch::dispatch called with Import; handled separately");
        }
    }
}

fn toggle_recording(app: &AppHandle, mode: RecordingMode) {
    let mut source = recording_state().source.lock();
    if *source != RecordingSource::None {
        drop(source);
        force_stop_recording(app);
    } else {
        // Set the source before spawning so a follow-up `--cancel`
        // sees the recording in progress.
        *source = match mode {
            RecordingMode::Standard => RecordingSource::Standard,
            RecordingMode::Llm => RecordingSource::Llm,
            RecordingMode::Command => RecordingSource::Command,
        };
        drop(source);
        let app_clone = app.clone();
        std::thread::spawn(move || {
            record_audio(&app_clone, mode);
        });
    }
}

fn paste_last(app: &AppHandle) {
    match crate::history::get_last_transcription(app) {
        Ok(transcript) => {
            let _ = crate::audio::write_last_transcription(app, &transcript);
        }
        Err(e) => {
            warn!("CLI paste-last: no transcript available ({})", e);
        }
    }
}

fn cancel(app: &AppHandle) {
    let recording_source = {
        let source = recording_state().source.lock();
        *source
    };
    if recording_source != RecordingSource::None {
        force_cancel_recording(app);
    }
}
