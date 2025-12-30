use crate::audio::{record_audio, stop_recording, write_last_transcription};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, LLMRecordShortcutKeys, LastTranscriptShortcutKeys, RecordShortcutKeys,
};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::initialize_shortcut_states;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows_sys::Win32::UI::Input::{
    GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE,
    RAWINPUTDEVICE_FLAGS, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT, RIM_TYPEHID,
    RIM_TYPEKEYBOARD,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassExW,
    TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, MSG, WINDOW_EX_STYLE, WM_INPUT,
    WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

use once_cell::sync::Lazy;

// Global state for HID key presses (thread-safe)
static HID_KEYS_PRESSED: Lazy<Arc<RwLock<HashSet<i32>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashSet::new())));

// HID Usage Page for Generic Desktop Controls
const HID_USAGE_PAGE_GENERIC: u16 = 0x01;
// HID Usage for Keyboard
const HID_USAGE_GENERIC_KEYBOARD: u16 = 0x06;

fn check_keys_pressed(keys: &[i32]) -> bool {
    let hid_keys = HID_KEYS_PRESSED.read().unwrap_or_else(|e| e.into_inner());
    keys.iter().all(|&vk| {
        // Check GetAsyncKeyState first (standard keyboards)
        let async_pressed = (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0;
        // Also check HID keys from RawInput
        let hid_pressed = hid_keys.contains(&vk);
        async_pressed || hid_pressed
    })
}

// Window procedure for handling RawInput messages
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_INPUT {
        process_raw_input(lparam as HRAWINPUT);
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn process_raw_input(h_raw_input: HRAWINPUT) {
    let mut size: u32 = 0;

    // Get required buffer size
    let result = unsafe {
        GetRawInputData(
            h_raw_input,
            RID_INPUT,
            std::ptr::null_mut(),
            &mut size,
            std::mem::size_of::<RAWINPUTHEADER>() as u32,
        )
    };

    if result != 0 || size == 0 {
        return;
    }

    // Allocate buffer and get the data
    let mut buffer = vec![0u8; size as usize];
    let result = unsafe {
        GetRawInputData(
            h_raw_input,
            RID_INPUT,
            buffer.as_mut_ptr() as *mut _,
            &mut size,
            std::mem::size_of::<RAWINPUTHEADER>() as u32,
        )
    };

    if result == u32::MAX {
        return;
    }

    let raw_input = unsafe { &*(buffer.as_ptr() as *const RAWINPUT) };

    match raw_input.header.dwType {
        RIM_TYPEKEYBOARD => {
            let keyboard = unsafe { &raw_input.data.keyboard };
            let vkey = keyboard.VKey as i32;
            let is_key_down = keyboard.Message == 0x0100 || keyboard.Message == 0x0104; // WM_KEYDOWN or WM_SYSKEYDOWN

            if let Ok(mut hid_keys) = HID_KEYS_PRESSED.write() {
                if is_key_down {
                    hid_keys.insert(vkey);
                } else {
                    hid_keys.remove(&vkey);
                }
            }
        }
        RIM_TYPEHID => {
            // HID device - SpeechMike sends data here
            // The data format depends on the device, but many HID devices
            // send keyboard-like scan codes
            let hid = unsafe { &raw_input.data.hid };
            if hid.dwCount > 0 && hid.dwSizeHid > 0 {
                // Get pointer to HID data (after the RAWHID header)
                let hid_data_ptr = unsafe {
                    (raw_input as *const RAWINPUT as *const u8)
                        .add(std::mem::size_of::<RAWINPUTHEADER>() + 8) // 8 = size of dwSizeHid + dwCount
                };

                // Process each HID report
                for i in 0..hid.dwCount {
                    let report_ptr =
                        unsafe { hid_data_ptr.add((i * hid.dwSizeHid) as usize) };
                    let report =
                        unsafe { std::slice::from_raw_parts(report_ptr, hid.dwSizeHid as usize) };

                    // Log HID data for debugging (helps identify SpeechMike button codes)
                    println!(
                        "[RawInput HID] Device: {:?}, Report: {:02X?}",
                        raw_input.header.hDevice, report
                    );

                    // Common HID keyboard report format:
                    // Byte 0: Modifier keys
                    // Byte 1: Reserved
                    // Bytes 2-7: Key codes (up to 6 simultaneous keys)
                    if report.len() >= 3 {
                        // Check for key presses in standard HID keyboard format
                        if let Ok(mut hid_keys) = HID_KEYS_PRESSED.write() {
                            // Clear previous HID keys for this device type
                            // This is a simple approach - might need refinement for your specific device
                            for byte_idx in 2..report.len().min(8) {
                                let key_code = report[byte_idx];
                                if key_code != 0 {
                                    // Convert HID usage to virtual key code
                                    // This is a simplified mapping - SpeechMike may need specific mapping
                                    let vkey = hid_usage_to_vkey(key_code);
                                    if vkey != 0 {
                                        hid_keys.insert(vkey);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

// Convert HID keyboard usage code to Windows virtual key code
fn hid_usage_to_vkey(usage: u8) -> i32 {
    // Standard HID keyboard usage to VK mapping (subset)
    // See: https://usb.org/sites/default/files/hut1_3_0.pdf (Chapter 10)
    match usage {
        0x04..=0x1D => (usage - 0x04 + 0x41) as i32, // A-Z -> VK_A - VK_Z
        0x1E..=0x27 => (usage - 0x1E + 0x31) as i32, // 1-0 -> VK_1 - VK_0
        0x28 => 0x0D,                                 // Enter -> VK_RETURN
        0x29 => 0x1B,                                 // Escape -> VK_ESCAPE
        0x2A => 0x08,                                 // Backspace -> VK_BACK
        0x2B => 0x09,                                 // Tab -> VK_TAB
        0x2C => 0x20,                                 // Space -> VK_SPACE
        0x3A..=0x45 => (usage - 0x3A + 0x70) as i32,  // F1-F12 -> VK_F1 - VK_F12
        0x46 => 0x2C,                                 // PrintScreen -> VK_SNAPSHOT
        0x47 => 0x91,                                 // ScrollLock -> VK_SCROLL
        0x48 => 0x13,                                 // Pause -> VK_PAUSE
        0x49 => 0x2D,                                 // Insert -> VK_INSERT
        0x4A => 0x24,                                 // Home -> VK_HOME
        0x4B => 0x21,                                 // PageUp -> VK_PRIOR
        0x4C => 0x2E,                                 // Delete -> VK_DELETE
        0x4D => 0x23,                                 // End -> VK_END
        0x4E => 0x22,                                 // PageDown -> VK_NEXT
        0x4F => 0x27,                                 // RightArrow -> VK_RIGHT
        0x50 => 0x25,                                 // LeftArrow -> VK_LEFT
        0x51 => 0x28,                                 // DownArrow -> VK_DOWN
        0x52 => 0x26,                                 // UpArrow -> VK_UP
        0xE0 => 0xA2,                                 // LeftControl -> VK_LCONTROL
        0xE1 => 0xA0,                                 // LeftShift -> VK_LSHIFT
        0xE2 => 0xA4,                                 // LeftAlt -> VK_LMENU
        0xE3 => 0x5B,                                 // LeftGUI -> VK_LWIN
        0xE4 => 0xA3,                                 // RightControl -> VK_RCONTROL
        0xE5 => 0xA1,                                 // RightShift -> VK_RSHIFT
        0xE6 => 0xA5,                                 // RightAlt -> VK_RMENU
        0xE7 => 0x5C,                                 // RightGUI -> VK_RWIN
        _ => 0,
    }
}

fn create_raw_input_window() {
    std::thread::spawn(|| {
        unsafe {
            let h_instance = GetModuleHandleW(std::ptr::null());

            // Create a unique class name
            let class_name: Vec<u16> = "MurmureRawInputClass\0"
                .encode_utf16()
                .collect();

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: 0,
                hCursor: 0,
                hbrBackground: 0,
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: 0,
            };

            if RegisterClassExW(&wc) == 0 {
                eprintln!("[RawInput] Failed to register window class");
                return;
            }

            let window_name: Vec<u16> = "MurmureRawInput\0".encode_utf16().collect();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name.as_ptr(),
                window_name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                h_instance,
                std::ptr::null(),
            );

            if hwnd == 0 {
                eprintln!("[RawInput] Failed to create window");
                return;
            }

            // Register for raw input from keyboards and HID devices
            let devices = [
                // Standard keyboards
                RAWINPUTDEVICE {
                    usUsagePage: HID_USAGE_PAGE_GENERIC,
                    usUsage: HID_USAGE_GENERIC_KEYBOARD,
                    dwFlags: RIDEV_INPUTSINK as RAWINPUTDEVICE_FLAGS,
                    hwndTarget: hwnd,
                },
                // Generic HID devices (catches SpeechMike and similar)
                RAWINPUTDEVICE {
                    usUsagePage: 0x0C, // Consumer Control (media keys, dictation devices)
                    usUsage: 0x01,     // Consumer Control
                    dwFlags: RIDEV_INPUTSINK as RAWINPUTDEVICE_FLAGS,
                    hwndTarget: hwnd,
                },
            ];

            let result = RegisterRawInputDevices(
                devices.as_ptr(),
                devices.len() as u32,
                std::mem::size_of::<RAWINPUTDEVICE>() as u32,
            );

            if result == 0 {
                eprintln!("[RawInput] Failed to register raw input devices");
                return;
            }

            println!("[RawInput] Successfully registered for keyboard and HID input");

            // Message loop
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });
}

pub fn init_shortcuts(app: AppHandle) {
    // Start RawInput listener in separate thread
    create_raw_input_window();

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
                    // Priority: LLM record > Standard record
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
                    // Check if recording limit was reached
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
                    // Check if recording limit was reached
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
