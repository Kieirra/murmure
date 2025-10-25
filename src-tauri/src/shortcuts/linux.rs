use crate::audio::write_transcription;
use crate::audio::{record_audio, stop_recording};
use crate::history::get_last_transcription;
use crate::shortcuts::{
    keys_to_string, LastTranscriptShortcutKeys, RecordShortcutKeys, TranscriptionSuspended,
};
use parking_lot::RwLock;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

fn rdev_key_to_vk(key: &Key) -> Option<i32> {
    match key {
        Key::MetaLeft | Key::MetaRight => Some(0x5B),
        Key::ControlLeft | Key::ControlRight => Some(0x11),
        Key::Alt | Key::AltGr => Some(0x12),
        Key::ShiftLeft | Key::ShiftRight => Some(0x10),
        Key::KeyA => Some(0x41),
        Key::KeyB => Some(0x42),
        Key::KeyC => Some(0x43),
        Key::KeyD => Some(0x44),
        Key::KeyE => Some(0x45),
        Key::KeyF => Some(0x46),
        Key::KeyG => Some(0x47),
        Key::KeyH => Some(0x48),
        Key::KeyI => Some(0x49),
        Key::KeyJ => Some(0x4A),
        Key::KeyK => Some(0x4B),
        Key::KeyL => Some(0x4C),
        Key::KeyM => Some(0x4D),
        Key::KeyN => Some(0x4E),
        Key::KeyO => Some(0x4F),
        Key::KeyP => Some(0x50),
        Key::KeyQ => Some(0x51),
        Key::KeyR => Some(0x52),
        Key::KeyS => Some(0x53),
        Key::KeyT => Some(0x54),
        Key::KeyU => Some(0x55),
        Key::KeyV => Some(0x56),
        Key::KeyW => Some(0x57),
        Key::KeyX => Some(0x58),
        Key::KeyY => Some(0x59),
        Key::KeyZ => Some(0x5A),
        Key::Num0 => Some(0x30),
        Key::Num1 => Some(0x31),
        Key::Num2 => Some(0x32),
        Key::Num3 => Some(0x33),
        Key::Num4 => Some(0x34),
        Key::Num5 => Some(0x35),
        Key::Num6 => Some(0x36),
        Key::Num7 => Some(0x37),
        Key::Num8 => Some(0x38),
        Key::Num9 => Some(0x39),
        Key::F1 => Some(0x70),
        Key::F2 => Some(0x71),
        Key::F3 => Some(0x72),
        Key::F4 => Some(0x73),
        Key::F5 => Some(0x74),
        Key::F6 => Some(0x75),
        Key::F7 => Some(0x76),
        Key::F8 => Some(0x77),
        Key::F9 => Some(0x78),
        Key::F10 => Some(0x79),
        Key::F11 => Some(0x7A),
        Key::F12 => Some(0x7B),
        Key::Space => Some(0x20),
        Key::Return => Some(0x0D),
        Key::Escape => Some(0x1B),
        Key::Tab => Some(0x09),
        Key::Backspace => Some(0x08),
        Key::Delete => Some(0x2E),
        Key::Insert => Some(0x2D),
        Key::Home => Some(0x24),
        Key::End => Some(0x23),
        Key::PageUp => Some(0x21),
        Key::PageDown => Some(0x22),
        Key::UpArrow => Some(0x26),
        Key::DownArrow => Some(0x28),
        Key::LeftArrow => Some(0x25),
        Key::RightArrow => Some(0x27),
        _ => None,
    }
}

pub fn init_shortcuts(app: AppHandle) {
    let pressed_keys: Arc<RwLock<HashSet<i32>>> = Arc::new(RwLock::new(HashSet::new()));
    let pressed_keys_listener = pressed_keys.clone();
    let pressed_keys_checker = pressed_keys.clone();

    std::thread::spawn(move || {
        if let Err(error) = listen(move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                if let Some(vk) = rdev_key_to_vk(&key) {
                    pressed_keys_listener.write().insert(vk);
                }
            }
            EventType::KeyRelease(key) => {
                if let Some(vk) = rdev_key_to_vk(&key) {
                    pressed_keys_listener.write().remove(&vk);
                }
            }
            _ => {}
        }) {
            eprintln!("Error starting keyboard listener: {:?}", error);
        }
    });

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

            let pressed = pressed_keys_checker.read();
            let all_record_keys_down = record_required_keys.iter().all(|k| pressed.contains(k));
            let all_last_transcript_keys_down = !last_transcript_required_keys.is_empty()
                && last_transcript_required_keys
                    .iter()
                    .all(|k| pressed.contains(k));

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
