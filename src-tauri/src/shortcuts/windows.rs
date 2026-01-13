use crate::audio::{record_audio, stop_recording, write_last_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    CommandShortcutKeys, LLMRecordShortcutKeys, LastTranscriptShortcutKeys, RecordShortcutKeys,
};
use log::debug;
use std::time::Duration;
use tauri::{AppHandle, Manager};

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
            Llm,
            Command,
        }
        let mut recording_source = RecordingSource::None;
        let mut last_transcript_pressed = false;

        initialize_shortcut_states(&app_handle);

        loop {
            let shortcut_state = app_handle.state::<crate::shortcuts::types::ShortcutState>();
            if shortcut_state.is_suspended() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let llm_record_required_keys = app_handle.state::<LLMRecordShortcutKeys>().get();
            let command_required_keys = app_handle.state::<CommandShortcutKeys>().get();
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();

            if record_required_keys.is_empty()
                && llm_record_required_keys.is_empty()
                && command_required_keys.is_empty()
            {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let all_record_keys_down =
                !record_required_keys.is_empty() && check_keys_pressed(&record_required_keys);
            let all_llm_record_keys_down = !llm_record_required_keys.is_empty()
                && check_keys_pressed(&llm_record_required_keys);
            let all_command_keys_down = !command_required_keys.is_empty()
                && check_keys_pressed(&command_required_keys);
            let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);

            if all_record_keys_down || all_llm_record_keys_down || all_command_keys_down {
                if shortcut_state.is_toggle_required() {
                    let current_toggle = shortcut_state.is_toggled();
                    shortcut_state.set_toggled(!current_toggle);
                    debug!("Is recording toggled {}", !current_toggle);
                    std::thread::sleep(Duration::from_millis(150));
                }
            }

            let should_record = if shortcut_state.is_toggle_required() {
                shortcut_state.is_toggled()
            } else {
                true
            };

            match recording_source {
                RecordingSource::None => {
                    // Priority: LLM record > Standard record
                    if all_llm_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_llm(&app_handle);
                        recording_source = RecordingSource::Llm;
                    } else if all_command_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_command(&app_handle);
                        recording_source = RecordingSource::Command;
                    } else if all_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        record_audio(&app_handle);
                        recording_source = RecordingSource::Standard;
                    }
                }
                RecordingSource::Standard => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    }
                }
                RecordingSource::Llm => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_llm_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    }
                }
                RecordingSource::Command => {
                    // Check if recording limit was reached
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                    } else if !all_command_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
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
