use log::warn;
use tauri::{AppHandle, Emitter, Manager};

use crate::audio::record_audio;
use crate::audio::types::RecordingMode;
use crate::cli::types::CliCommand;
use crate::shortcuts::shortcuts::{
    ensure_llm_mode_ready, force_cancel_recording, spawn_paste_last_transcript,
    toggle_recording_action,
};
use crate::shortcuts::types::{recording_state, RecordingSource, ShortcutState};

/// Reuses the same backend toggle path as internal shortcuts (cooldown,
/// focus capture, ShortcutState toggling, UI flow) to guarantee parity.
pub fn dispatch(app: &AppHandle, cmd: &CliCommand) {
    // CLI invocations always toggle: a single OS-level shortcut event cannot
    // express press/release, so PushToTalk is not supported from the CLI.
    match cmd {
        CliCommand::Transcription => cli_toggle_recording(app, RecordingMode::Standard),
        CliCommand::TranscriptionCommand => cli_toggle_recording(app, RecordingMode::Command),
        CliCommand::PasteLast => spawn_paste_last_transcript(app),
        CliCommand::Cancel => cancel(app),
        CliCommand::VoiceMode => {
            let _ = app.emit("voice-mode-toggle-requested", ());
        }
        CliCommand::LlmMode(n) => {
            // CLI exposes 1-based indices; backend uses 0-based.
            let index = (*n as usize).saturating_sub(1);
            if ensure_llm_mode_ready(app, index, true).is_err() {
                return;
            }
            crate::llm::switch_active_mode_silent(app, index);
            cli_toggle_recording(app, RecordingMode::Llm);
        }
        CliCommand::LlmTransform(n) => {
            let index = (*n as usize).saturating_sub(1);
            crate::llm::spawn_transform_selection(app, index);
        }
        CliCommand::Import { .. } => {
            warn!("cli_dispatch::dispatch called with Import; handled separately");
        }
        CliCommand::Transcribe { .. } => {
            warn!("cli_dispatch::dispatch called with Transcribe; handled separately");
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

fn cancel(app: &AppHandle) {
    let recording_source = {
        let source = recording_state().source.lock();
        *source
    };
    if recording_source != RecordingSource::None {
        force_cancel_recording(app);
    }
}
