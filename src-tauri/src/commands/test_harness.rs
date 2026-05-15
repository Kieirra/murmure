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

// Interval between history polls in `__test_wait_for_transcription`. Short
// enough to keep test wall-clock low, long enough to avoid spinning.
const TRANSCRIPTION_POLL_INTERVAL: Duration = Duration::from_millis(100);

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

#[command]
pub fn __test_press_record_shortcut(app: AppHandle) -> Result<(), String> {
    record_audio(&app, RecordingMode::Standard);
    Ok(())
}

#[command]
pub fn __test_release_record_shortcut(app: AppHandle) -> Result<(), String> {
    // `stop_recording` blocks on transcription and paste, which can exceed the
    // Tauri command timeout. Run it on a detached thread so the IPC call
    // returns immediately, matching the keyboard release path.
    std::thread::spawn(move || {
        let _ = stop_recording(&app);
    });
    Ok(())
}

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
        std::thread::sleep(TRANSCRIPTION_POLL_INTERVAL);
    }
}
