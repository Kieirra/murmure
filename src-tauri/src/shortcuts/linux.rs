use crate::audio::write_transcription;
use crate::audio::{record_audio, stop_recording};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn vk_to_hotkey_key(vk: i32) -> Option<String> {
    match vk {
        0x41..=0x5A => {
            let letter = (b'A' + (vk - 0x41) as u8) as char;
            Some(format!("Key{}", letter))
        }
        0x30..=0x39 => Some(format!("Digit{}", vk - 0x30)),
        0x70..=0x7B => Some(format!("F{}", vk - 0x70 + 1)),
        0x20 => Some("Space".into()),
        0x0D => Some("Enter".into()),
        0x1B => Some("Escape".into()),
        0x09 => Some("Tab".into()),
        0x08 => Some("Backspace".into()),
        0x2E => Some("Delete".into()),
        0x2D => Some("Insert".into()),
        0x24 => Some("Home".into()),
        0x23 => Some("End".into()),
        0x21 => Some("PageUp".into()),
        0x22 => Some("PageDown".into()),
        0x26 => Some("ArrowUp".into()),
        0x28 => Some("ArrowDown".into()),
        0x25 => Some("ArrowLeft".into()),
        0x27 => Some("ArrowRight".into()),
        _ => None,
    }
}

fn binding_to_hotkey(keys: &[i32]) -> Option<String> {
    if keys.is_empty() {
        return None;
    }

    let mut has_control = false;
    let mut has_shift = false;
    let mut has_alt = false;
    let mut has_super = false;
    let mut main_key: Option<String> = None;

    for &vk in keys {
        match vk {
            0x11 => has_control = true,
            0x10 => has_shift = true,
            0x12 => has_alt = true,
            0x5B => has_super = true,
            _ => {
                if main_key.is_some() {
                    return None;
                }
                main_key = vk_to_hotkey_key(vk);
                if main_key.is_none() {
                    return None;
                }
            }
        }
    }

    let key = main_key?;
    let mut parts = Vec::new();
    if has_control {
        parts.push("Control".to_string());
    }
    if has_shift {
        parts.push("Shift".to_string());
    }
    if has_alt {
        parts.push("Alt".to_string());
    }
    if has_super {
        parts.push("Super".to_string());
    }
    parts.push(key);

    Some(parts.join("+"))
}

pub fn init_shortcuts(app: AppHandle) {
    std::thread::spawn(move || {
        let mut registered_record: Option<(String, String, Arc<AtomicBool>)> = None;
        let mut registered_last: Option<(String, Arc<AtomicBool>)> = None;

        loop {
            let record_keys = app.state::<RecordShortcutKeys>().get();
            let record_binding = binding_to_hotkey(&record_keys);

            if record_binding
                .as_ref()
                .map(|s| s.as_str())
                != registered_record
                    .as_ref()
                    .map(|(binding, _, _)| binding.as_str())
            {
                if let Some((binding, display, active)) = registered_record.take() {
                    if active.swap(false, Ordering::SeqCst) {
                        let _ = stop_recording(&app);
                        let _ = app.emit("shortcut:stop", display.clone());
                    }
                    if let Err(error) = app.global_shortcut().unregister(binding.as_str()) {
                        eprintln!("Failed to unregister record shortcut: {}", error);
                    }
                }

                if let Some(binding) = record_binding.clone() {
                    let display = keys_to_string(&record_keys);
                    let active = Arc::new(AtomicBool::new(false));
                    let active_clone = Arc::clone(&active);
                    let display_clone = display.clone();

                    let register_result = {
                        let manager = app.global_shortcut();
                        manager.on_shortcut(binding.as_str(), move |handle, _, event| {
                            if matches!(event.state, ShortcutState::Pressed) {
                                if handle.state::<TranscriptionSuspended>().get() {
                                    return;
                                }
                                if !active_clone.swap(true, Ordering::SeqCst) {
                                    record_audio(handle);
                                    let _ =
                                        handle.emit("shortcut:start", display_clone.clone());
                                }
                            } else if matches!(event.state, ShortcutState::Released) {
                                if active_clone.swap(false, Ordering::SeqCst) {
                                    let _ = stop_recording(handle);
                                    let _ =
                                        handle.emit("shortcut:stop", display_clone.clone());
                                }
                            }
                        })
                    };

                    if let Err(error) = register_result {
                        eprintln!("Failed to register record shortcut: {}", error);
                    } else {
                        registered_record = Some((binding, display, active));
                    }
                }
            }

            let last_keys = app.state::<LastTranscriptShortcutKeys>().get();
            let last_binding = binding_to_hotkey(&last_keys);

            if last_binding.as_ref().map(|s| s.as_str())
                != registered_last.as_ref().map(|(binding, _)| binding.as_str())
            {
                if let Some((binding, active)) = registered_last.take() {
                    active.store(false, Ordering::SeqCst);
                    if let Err(error) = app.global_shortcut().unregister(binding.as_str()) {
                        eprintln!("Failed to unregister last transcript shortcut: {}", error);
                    }
                }

                if let Some(binding) = last_binding.clone() {
                    let active = Arc::new(AtomicBool::new(false));
                    let active_clone = Arc::clone(&active);

                    let register_result = {
                        let manager = app.global_shortcut();
                        manager.on_shortcut(binding.as_str(), move |handle, _, event| {
                            if matches!(event.state, ShortcutState::Pressed) {
                                if handle.state::<TranscriptionSuspended>().get() {
                                    return;
                                }
                                if !active_clone.swap(true, Ordering::SeqCst) {
                                    if let Ok(last_transcript) = get_last_transcription(handle) {
                                        let _ = write_transcription(handle, &last_transcript);
                                    }
                                }
                            } else if matches!(event.state, ShortcutState::Released) {
                                active_clone.store(false, Ordering::SeqCst);
                            }
                        })
                    };

                    if let Err(error) = register_result {
                        eprintln!("Failed to register last transcript shortcut: {}", error);
                    } else {
                        registered_last = Some((binding, active));
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(200));
        }
    });
}
