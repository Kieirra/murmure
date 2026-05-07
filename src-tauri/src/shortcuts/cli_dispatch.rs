use log::{info, warn};
use tauri::{AppHandle, Emitter, Manager};

use crate::audio::record_audio;
use crate::audio::types::RecordingMode;
use crate::cli::types::CliCommand;
use crate::shortcuts::shortcuts::{force_cancel_recording, toggle_recording_action};
use crate::shortcuts::types::{recording_state, RecordingSource, ShortcutState};

/// Reuses the same backend toggle path as internal shortcuts (cooldown,
/// focus capture, ShortcutState toggling, UI flow) to guarantee parity.
pub fn dispatch(app: &AppHandle, cmd: &CliCommand) {
    // CLI invocations always toggle: a single OS-level shortcut event cannot
    // express press/release, so PushToTalk is not supported from the CLI.
    match cmd {
        CliCommand::Transcription => cli_toggle_recording(app, RecordingMode::Standard),
        CliCommand::TranscriptionLlm => cli_toggle_recording(app, RecordingMode::Llm),
        CliCommand::TranscriptionCommand => cli_toggle_recording(app, RecordingMode::Command),
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

fn cli_toggle_recording(app: &AppHandle, mode: RecordingMode) {
    let target = match mode {
        RecordingMode::Standard => RecordingSource::Standard,
        RecordingMode::Llm => RecordingSource::Llm,
        RecordingMode::Command => RecordingSource::Command,
    };
    let shortcut_state = app.state::<ShortcutState>();
    let app_for_fn = app.clone();
    toggle_recording_action(app, target, shortcut_state.inner(), move || {
        record_audio(&app_for_fn, mode);
    });
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
