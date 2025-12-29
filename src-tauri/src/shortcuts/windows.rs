use crate::audio::{record_audio, stop_recording, write_last_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, initialize_shortcut_states, LLMRecordShortcutKeys, LastTranscriptShortcutKeys,
    RecordShortcutKeys,
};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

static PRESSED_KEYS: once_cell::sync::Lazy<Arc<Mutex<HashSet<i32>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashSet::new())));
static HOOK_RUNNING: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn keyboard_hook_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kb_struct = &*(l_param as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode as i32;

        match w_param as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                if let Ok(mut keys) = PRESSED_KEYS.try_lock() {
                    keys.insert(vk_code);
                }
            }
            WM_KEYUP | WM_SYSKEYUP => {
                if let Ok(mut keys) = PRESSED_KEYS.try_lock() {
                    keys.remove(&vk_code);
                }
            }
            _ => {}
        }
    }
    CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
}

fn check_keys_pressed(keys: &[i32]) -> bool {
    if let Ok(pressed) = PRESSED_KEYS.try_lock() {
        keys.iter().all(|vk| pressed.contains(vk))
    } else {
        false
    }
}

fn start_hook_thread() {
    if HOOK_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    std::thread::spawn(|| unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), std::ptr::null_mut(), 0);
        if hook.is_null() {
            eprintln!("Failed to install keyboard hook");
            HOOK_RUNNING.store(false, Ordering::SeqCst);
            return;
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

pub fn init_shortcuts(app: AppHandle) {
    start_hook_thread();

    std::thread::spawn(move || {
        let app_handle = app.clone();
        #[derive(PartialEq)]
        enum RecordingSource {
            None,
            Standard,
            Llm,
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
            let last_transcript_required_keys =
                app_handle.state::<LastTranscriptShortcutKeys>().get();

            if record_required_keys.is_empty() && llm_record_required_keys.is_empty() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let all_record_keys_down =
                !record_required_keys.is_empty() && check_keys_pressed(&record_required_keys);
            let all_llm_record_keys_down = !llm_record_required_keys.is_empty()
                && check_keys_pressed(&llm_record_required_keys);
            let all_last_transcript_keys_down = check_keys_pressed(&last_transcript_required_keys);

            if all_record_keys_down || all_llm_record_keys_down {
                if shortcut_state.is_toggle_required() {
                    let current_toggle = shortcut_state.is_toggled();
                    shortcut_state.set_toggled(!current_toggle);
                    println!("Is recording toggled {}", !current_toggle);
                    std::thread::sleep(Duration::from_millis(150));
                    let _ = app_handle.emit("shortcut:toggle-recording", "".to_string());
                }
            }

            let should_record = if shortcut_state.is_toggle_required() {
                shortcut_state.is_toggled()
            } else {
                true
            };

            match recording_source {
                RecordingSource::None => {
                    if all_llm_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        crate::audio::record_audio_with_llm(&app_handle);
                        recording_source = RecordingSource::Llm;
                        let _ = app_handle.emit(
                            "shortcut:llm-record",
                            keys_to_string(&llm_record_required_keys),
                        );
                    } else if all_record_keys_down && should_record {
                        crate::onboarding::onboarding::capture_focus_at_record_start(&app_handle);
                        record_audio(&app_handle);
                        recording_source = RecordingSource::Standard;
                        let _ = app_handle
                            .emit("shortcut:start", keys_to_string(&record_required_keys));
                    }
                }
                RecordingSource::Standard => {
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ =
                            app_handle.emit("shortcut:stop", keys_to_string(&record_required_keys));
                    } else if !all_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ =
                            app_handle.emit("shortcut:stop", keys_to_string(&record_required_keys));
                    }
                }
                RecordingSource::Llm => {
                    let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                    if audio_state.is_limit_reached() {
                        crate::shortcuts::actions::force_stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ = app_handle.emit(
                            "shortcut:llm-record-released",
                            keys_to_string(&llm_record_required_keys),
                        );
                    } else if !all_llm_record_keys_down && !shortcut_state.is_toggled() {
                        let _ = stop_recording(&app_handle);
                        recording_source = RecordingSource::None;
                        let _ = app_handle.emit(
                            "shortcut:llm-record-released",
                            keys_to_string(&llm_record_required_keys),
                        );
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
