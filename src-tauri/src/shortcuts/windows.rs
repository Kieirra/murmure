use crate::audio::write_transcription;
use crate::audio::{record_audio, stop_recording};
use crate::history::get_last_transcription;
use crate::settings;
use crate::shortcuts::{
    keys_to_string, LastTranscriptShortcutKeys, RecordShortcutKeys, StartRecordingShortcutKeys,
    StopRecordingShortcutKeys, TranscriptionSuspended,
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
        let mut start_recording_pressed = false;
        let mut stop_recording_pressed = false;

        loop {
            if app_handle.state::<TranscriptionSuspended>().get() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let settings = settings::load_settings(&app_handle);
            let keyboard_mode = settings.keyboard_mode.as_str();

            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();

            // Push-to-talk mode: use record shortcut as hold-to-record
            if keyboard_mode == "push-to-talk" {
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
            }
            // Toggle mode: separate start and stop shortcuts
            else if keyboard_mode == "toggle" {
                let start_keys = app_handle.state::<StartRecordingShortcutKeys>().get();
                let stop_keys = app_handle.state::<StopRecordingShortcutKeys>().get();

                let all_start_keys_down = check_keys_pressed(&start_keys);
                let all_stop_keys_down = check_keys_pressed(&stop_keys);

                // Start recording on start shortcut
                if !start_recording_pressed && all_start_keys_down {
                    if !is_recording {
                        record_audio(&app_handle);
                        is_recording = true;
                        let _ = app_handle.emit("shortcut:start", keys_to_string(&start_keys));
                    }
                    start_recording_pressed = true;
                }
                if start_recording_pressed && !all_start_keys_down {
                    start_recording_pressed = false;
                }

                // Stop recording on stop shortcut
                if !stop_recording_pressed && all_stop_keys_down {
                    if is_recording {
                        let _ = stop_recording(&app_handle);
                        is_recording = false;
                        let _ = app_handle.emit("shortcut:stop", keys_to_string(&stop_keys));
                    }
                    stop_recording_pressed = true;
                }
                if stop_recording_pressed && !all_stop_keys_down {
                    stop_recording_pressed = false;
                }

                // Handle last transcript shortcut
                if !last_transcript_pressed && !last_transcript_required_keys.is_empty() {
                    let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);
                    if all_last_transcript_keys_down {
                        if let Ok(last_transcript) = get_last_transcription(&app_handle) {
                            let _ = write_transcription(&app_handle, &last_transcript);
                        }
                        last_transcript_pressed = true;
                    }
                }
                if last_transcript_pressed && !last_transcript_required_keys.is_empty() {
                    let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);
                    if !all_last_transcript_keys_down {
                        last_transcript_pressed = false;
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(32));
        }
    });
}
