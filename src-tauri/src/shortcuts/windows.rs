use crate::audio::{record_audio, stop_recording, write_last_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    initialize_shortcut_states, keys_to_string, LLMRecordShortcutKeys, LastTranscriptShortcutKeys,
    RecordShortcutKeys,
};
use log::{debug, error};
use parking_lot::RwLock;
use std::collections::HashSet;
use std::mem::{size_of, zeroed};
use std::ptr::null_mut;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

static mut PRESSED_KEYS: Option<Arc<RwLock<HashSet<i32>>>> = None;

fn vkey_to_normalized(vkey: u16) -> i32 {
    match vkey {
        0xA0 | 0xA1 => 0x10, // LShift/RShift -> Shift
        0xA2 | 0xA3 => 0x11, // LCtrl/RCtrl -> Ctrl
        0xA4 | 0xA5 => 0x12, // LAlt/RAlt -> Alt
        0x5B | 0x5C => 0x5B, // LWin/RWin -> Win
        _ => vkey as i32,
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_INPUT => unsafe {
            handle_raw_input(lparam);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn handle_raw_input(lparam: LPARAM) {
    let mut size: u32 = 0;
    let header_size = size_of::<RAWINPUTHEADER>() as u32;

    if GetRawInputData(lparam as _, RID_INPUT, null_mut(), &mut size, header_size) != 0 {
        return;
    }

    let mut buffer: Vec<u8> = vec![0; size as usize];
    let result = GetRawInputData(
        lparam as _,
        RID_INPUT,
        buffer.as_mut_ptr() as _,
        &mut size,
        header_size,
    );

    if result == u32::MAX || result == 0 {
        return;
    }

    let raw_input = &*(buffer.as_ptr() as *const RAWINPUT);

    if raw_input.header.dwType == RIM_TYPEKEYBOARD {
        let keyboard = &raw_input.data.keyboard;
        let is_break = (keyboard.Flags as u32 & RI_KEY_BREAK) != 0;
        let vk = vkey_to_normalized(keyboard.VKey);

        if let Some(ref keys) = PRESSED_KEYS {
            let mut pressed = keys.write();
            if is_break {
                pressed.remove(&vk);
            } else {
                pressed.insert(vk);
            }
        }
    }
}

fn start_raw_input_listener(pressed_keys: Arc<RwLock<HashSet<i32>>>) {
    std::thread::spawn(move || unsafe {
        PRESSED_KEYS = Some(pressed_keys);

        let instance = GetModuleHandleW(null_mut());
        let class_name: Vec<u16> = "MurmureRawInputClass\0".encode_utf16().collect();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: class_name.as_ptr(),
            style: 0,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0,
            hCursor: 0,
            hbrBackground: 0,
            lpszMenuName: null_mut(),
        };

        if RegisterClassW(&wnd_class) == 0 {
            error!("Failed to register window class for Raw Input");
            return;
        }

        let window_name: Vec<u16> = "MurmureRawInputListener\0".encode_utf16().collect();
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            0,
            instance,
            null_mut(),
        );

        if hwnd == 0 {
            error!("Failed to create message-only window for Raw Input");
            return;
        }

        let raw_input_devices = [
            RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x06, // Keyboard
                dwFlags: RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            },
            RAWINPUTDEVICE {
                usUsagePage: 0x0C,
                usUsage: 0x01, // Consumer Control (media keys, etc.)
                dwFlags: RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            },
        ];

        if RegisterRawInputDevices(
            raw_input_devices.as_ptr(),
            raw_input_devices.len() as u32,
            size_of::<RAWINPUTDEVICE>() as u32,
        ) == 0
        {
            error!("Failed to register Raw Input devices");
            return;
        }

        debug!("Raw Input listener started successfully");

        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

pub fn init_shortcuts(app: AppHandle) {
    let pressed_keys: Arc<RwLock<HashSet<i32>>> = Arc::new(RwLock::new(HashSet::new()));

    initialize_shortcut_states(&app);
    start_raw_input_listener(pressed_keys.clone());

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

            let pressed = pressed_keys.read();
            let all_record_keys_down = !record_required_keys.is_empty()
                && record_required_keys.iter().all(|k| pressed.contains(k));
            let all_llm_record_keys_down = !llm_record_required_keys.is_empty()
                && llm_record_required_keys.iter().all(|k| pressed.contains(k));
            let all_last_transcript_keys_down = !last_transcript_required_keys.is_empty()
                && last_transcript_required_keys
                    .iter()
                    .all(|k| pressed.contains(k));
            drop(pressed);

            if (all_record_keys_down || all_llm_record_keys_down)
                && shortcut_state.is_toggle_required()
            {
                let current_toggle = shortcut_state.is_toggled();
                shortcut_state.set_toggled(!current_toggle);
                debug!("Is recording toggled {}", !current_toggle);
                std::thread::sleep(Duration::from_millis(150));
                let _ = app_handle.emit("shortcut:toggle-recording", "".to_string());
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
