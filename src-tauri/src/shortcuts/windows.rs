use crate::audio::{record_audio, stop_recording, write_last_transcription, write_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, LastTranscriptShortcutKeys, LLMRecordShortcutKeys, RecordShortcutKeys,
    TranscriptionSuspended,
};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::initialize_shortcut_states;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

fn check_keys_pressed(keys: &[i32]) -> bool {
    keys.iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0)
}

pub fn init_shortcuts(app: AppHandle) {
    std::thread::spawn(move || {
        let app_handle = app.clone();
        #[derive(PartialEq)]
        enum RecordingSource {
            None,
            Standard,
            LLM,
        }
        let mut recording_source = RecordingSource::None;
        let mut last_transcript_pressed = false;

        initialize_shortcut_states(&app_handle);

        loop {
            if app_handle.state::<TranscriptionSuspended>().get() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let llm_record_required_keys = app_handle.state::<LLMRecordShortcutKeys>().get();
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();

            if record_required_keys.is_empty() && llm_record_required_keys.is_empty() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let all_record_keys_down = !record_required_keys.is_empty() && check_keys_pressed(&record_required_keys);
            let all_llm_record_keys_down = !llm_record_required_keys.is_empty() && check_keys_pressed(&llm_record_required_keys);
            let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);

            match recording_source {
                RecordingSource::None => {
                    // Priority: LLM record > Standard record
                    if all_llm_record_keys_down {
                        crate::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_llm(&app_handle);
                        recording_source = RecordingSource::LLM;
                        let _ = app_handle.emit("shortcut:llm-record", keys_to_string(&llm_record_required_keys));
                    } else if all_record_keys_down {
                        crate::onboarding::capture_focus_at_record_start(&app_handle);
                        record_audio(&app_handle);
                        recording_source = RecordingSource::Standard;
                        let _ = app_handle.emit("shortcut:start", keys_to_string(&record_required_keys));
                    }
                }
                RecordingSource::Standard => {
                    if !all_record_keys_down {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ = app_handle.emit("shortcut:stop", keys_to_string(&record_required_keys));
                    }
                }
                RecordingSource::LLM => {
                    if !all_llm_record_keys_down {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ = app_handle.emit("shortcut:llm-record-released", keys_to_string(&llm_record_required_keys));
                    }
                }
            }

            if !last_transcript_pressed && all_last_transcript_keys_down {
                if let Ok(last_transcript) = get_last_transcription(&app_handle) {
                    let _ = write_last_transcription(&app_handle, &last_transcript);
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
