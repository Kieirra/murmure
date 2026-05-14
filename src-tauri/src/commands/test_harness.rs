//! Tauri commands used exclusively by the e2e test harness.
//!
//! The module is only declared in `commands/mod.rs` under
//! `#[cfg(feature = "audio-injection")]`, so its contents never reach release
//! builds. The `__test_` prefix is a second visual barrier on top of that.

use crate::audio::types::{AudioState, RecordingMode};
use crate::audio::{record_audio, stop_recording};
use crate::history;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tauri::{command, AppHandle, Manager};

/// Register a WAV file as the audio source for the next recording session.
/// Returns an error if the file does not exist so the test fails fast.
#[command]
pub fn __test_set_audio_source(app: AppHandle, wav_path: String) -> Result<(), String> {
    let path = PathBuf::from(&wav_path);
    if !path.exists() {
        return Err(format!("WAV fixture not found: {}", path.display()));
    }
    let state = app.state::<AudioState>();
    *state.injected_wav_path.lock() = Some(path);
    Ok(())
}

/// Simulate a press on the global record shortcut. Mirrors what the keyboard
/// path does: call `record_audio` with the Standard mode.
#[command]
pub fn __test_press_record_shortcut(app: AppHandle) -> Result<(), String> {
    record_audio(&app, RecordingMode::Standard);
    Ok(())
}

/// Simulate a release on the global record shortcut. Runs the stop on a
/// dedicated thread because `stop_recording` blocks on transcription and paste.
#[command]
pub fn __test_release_record_shortcut(app: AppHandle) -> Result<(), String> {
    std::thread::spawn(move || {
        let _ = stop_recording(&app);
    });
    Ok(())
}

/// Poll the history backend until a transcription is available or the timeout
/// elapses. Returns the text of the most recent entry on success.
#[command]
pub fn __test_wait_for_transcription(app: AppHandle, timeout_ms: u64) -> Result<String, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        if let Ok(entries) = history::get_recent_transcriptions(&app) {
            if let Some(first) = entries.first() {
                if !first.text.trim().is_empty() {
                    return Ok(first.text.clone());
                }
            }
        }
        if Instant::now() >= deadline {
            return Err(format!("No transcription within {} ms", timeout_ms));
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
