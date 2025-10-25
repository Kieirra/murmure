use crate::audio::write_transcription;
use crate::audio::{record_audio, stop_recording};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended,
};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

fn check_keys_pressed(keys: &[i32]) -> bool {
    keys.iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0)
}

pub fn init_shortcuts(app: AppHandle) {
    std::thread::spawn(move || {
        let app_handle = app.clone();
        let mut is_recording = false;
        let mut last_transcript_pressed = false;

        loop {
            if app_handle.state::<TranscriptionSuspended>().get() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();

            if record_required_keys.is_empty() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let all_record_keys_down = check_keys_pressed(&record_required_keys);
            let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);

            if !is_recording && all_record_keys_down {
                record_audio(&app_handle);
                is_recording = true;
                let _ = app_handle.emit("shortcut:start", keys_to_string(&record_required_keys));
            }
            if is_recording && !all_record_keys_down {
                let _ = stop_recording(&app_handle);
                is_recording = false;
                let _ = app_handle.emit("shortcut:stop", keys_to_string(&record_required_keys));
            }

            if !last_transcript_pressed && all_last_transcript_keys_down {
                if let Ok(last_transcript) = get_last_transcription(&app_handle) {
                    let _ = write_transcription(&app_handle, &last_transcript);
                }
                last_transcript_pressed = true;
            }
            if last_transcript_pressed && !all_last_transcript_keys_down {
                last_transcript_pressed = false;
            }

            std::thread::sleep(Duration::from_millis(32));
        }
    });
}
