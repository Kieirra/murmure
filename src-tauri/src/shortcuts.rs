use crate::audio::{record_audio, stop_recording};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use crate::history::get_last_transcription;
use crate::audio::write_transcription;

// Import Windows-specific
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

// Import Linux-specific
#[cfg(target_os = "linux")]
use rdev::{listen, Event, EventType, Key};
#[cfg(target_os = "linux")]
use std::collections::HashSet;
#[cfg(target_os = "linux")]
use parking_lot::RwLock;

pub struct RecordShortcutKeys(pub Arc<Mutex<Vec<i32>>>);

impl RecordShortcutKeys {
    pub fn new(keys: Vec<i32>) -> Self {
        Self(Arc::new(Mutex::new(keys)))
    }
    pub fn get(&self) -> Vec<i32> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, keys: Vec<i32>) {
        *self.0.lock().unwrap() = keys;
    }
}

pub struct LastTranscriptShortcutKeys(pub Arc<Mutex<Vec<i32>>>);

impl LastTranscriptShortcutKeys {
    pub fn new(keys: Vec<i32>) -> Self {
        Self(Arc::new(Mutex::new(keys)))
    }
    pub fn get(&self) -> Vec<i32> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, keys: Vec<i32>) {
        *self.0.lock().unwrap() = keys;
    }
}

fn key_name_to_vk(name: &str) -> Option<i32> {
    match name.trim().to_lowercase().as_str() {
        "win" | "meta" | "super" => Some(0x5B),
        "ctrl" | "control" => Some(0x11),
        "alt" | "menu" => Some(0x12),
        "shift" => Some(0x10),
        "a" => Some(0x41),
        "b" => Some(0x42),
        "c" => Some(0x43),
        "d" => Some(0x44),
        "e" => Some(0x45),
        "f" => Some(0x46),
        "g" => Some(0x47),
        "h" => Some(0x48),
        "i" => Some(0x49),
        "j" => Some(0x4A),
        "k" => Some(0x4B),
        "l" => Some(0x4C),
        "m" => Some(0x4D),
        "n" => Some(0x4E),
        "o" => Some(0x4F),
        "p" => Some(0x50),
        "q" => Some(0x51),
        "r" => Some(0x52),
        "s" => Some(0x53),
        "t" => Some(0x54),
        "u" => Some(0x55),
        "v" => Some(0x56),
        "w" => Some(0x57),
        "x" => Some(0x58),
        "y" => Some(0x59),
        "z" => Some(0x5A),
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),
        "f1" => Some(0x70),
        "f2" => Some(0x71),
        "f3" => Some(0x72),
        "f4" => Some(0x73),
        "f5" => Some(0x74),
        "f6" => Some(0x75),
        "f7" => Some(0x76),
        "f8" => Some(0x77),
        "f9" => Some(0x78),
        "f10" => Some(0x79),
        "f11" => Some(0x7A),
        "f12" => Some(0x7B),
        "space" => Some(0x20),
        "enter" | "return" => Some(0x0D),
        "escape" | "esc" => Some(0x1B),
        "tab" => Some(0x09),
        "backspace" => Some(0x08),
        "delete" | "del" => Some(0x2E),
        "insert" | "ins" => Some(0x2D),
        "home" => Some(0x24),
        "end" => Some(0x23),
        "pageup" => Some(0x21),
        "pagedown" => Some(0x22),
        "arrowup" | "up" => Some(0x26),
        "arrowdown" | "down" => Some(0x28),
        "arrowleft" | "left" => Some(0x25),
        "arrowright" | "right" => Some(0x27),
        "mousebutton1" | "lmb" | "leftclick" => Some(0x01),
        "mousebutton2" | "rmb" | "rightclick" => Some(0x02),
        "mousebutton3" | "mmb" | "middleclick" => Some(0x04),
        "mousebutton4" | "mb4" => Some(0x05),
        "mousebutton5" | "mb5" => Some(0x06),
        _ => None,
    }
}

fn vk_to_key_name(vk: i32) -> String {
    match vk {
        0x5B => "win".to_string(),
        0x11 => "ctrl".to_string(),
        0x12 => "alt".to_string(),
        0x10 => "shift".to_string(),
        0x41..=0x5A => {
            let offset = (vk - 0x41) as u8;
            ((b'a' + offset) as char).to_string()
        }
        0x30..=0x39 => {
            let offset = (vk - 0x30) as u8;
            ((b'0' + offset) as char).to_string()
        }
        0x70..=0x7B => format!("f{}", vk - 0x70 + 1),
        0x20 => "space".to_string(),
        0x0D => "enter".to_string(),
        0x1B => "escape".to_string(),
        0x09 => "tab".to_string(),
        0x08 => "backspace".to_string(),
        0x2E => "delete".to_string(),
        0x2D => "insert".to_string(),
        0x24 => "home".to_string(),
        0x23 => "end".to_string(),
        0x21 => "pageup".to_string(),
        0x22 => "pagedown".to_string(),
        0x26 => "arrowup".to_string(),
        0x28 => "arrowdown".to_string(),
        0x25 => "arrowleft".to_string(),
        0x27 => "arrowright".to_string(),
        0x01 => "mousebutton1".to_string(),
        0x02 => "mousebutton2".to_string(),
        0x04 => "mousebutton3".to_string(),
        0x05 => "mousebutton4".to_string(),
        0x06 => "mousebutton5".to_string(),
        _ => format!("key{}", vk),
    }
}

pub fn parse_binding_keys(binding: &str) -> Vec<i32> {
    let mut keys = Vec::new();
    for token in binding.split('+') {
        if let Some(vk) = key_name_to_vk(token) {
            if !keys.contains(&vk) {
                keys.push(vk);
            }
        }
    }
    keys
}

pub fn keys_to_string(keys: &[i32]) -> String {
    keys.iter().map(|vk| vk_to_key_name(*vk)).collect::<Vec<_>>().join("+")
}

// Version Windows de la vérification des touches
#[cfg(target_os = "windows")]
fn check_keys_pressed(keys: &[i32]) -> bool {
    keys.iter().all(|&vk| {
        (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0
    })
}

// Version Windows de init_shortcuts
#[cfg(target_os = "windows")]
pub fn init_shortcuts(app: AppHandle) {
    std::thread::spawn(move || {
        let app_handle = app.clone();
        let mut is_recording = false;
        let mut last_transcript_pressed = false;

        loop {
            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let last_transcript_required_keys = app_handle.state::<LastTranscriptShortcutKeys>().get();

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

// Conversion des touches rdev vers virtual key codes
#[cfg(target_os = "linux")]
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

// Version Linux de init_shortcuts
#[cfg(target_os = "linux")]
pub fn init_shortcuts(app: AppHandle) {
    // État partagé pour les touches pressées
    let pressed_keys: Arc<RwLock<HashSet<i32>>> = Arc::new(RwLock::new(HashSet::new()));
    let pressed_keys_listener = pressed_keys.clone();
    let pressed_keys_checker = pressed_keys.clone();

    // Thread pour écouter les événements clavier avec rdev
    std::thread::spawn(move || {
        println!("Starting keyboard listener thread...");
        if let Err(error) = listen(move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    if let Some(vk) = rdev_key_to_vk(&key) {
                        pressed_keys_listener.write().insert(vk);
                        println!("Key pressed: {:?} (vk: {})", key, vk);
                    }
                }
                EventType::KeyRelease(key) => {
                    if let Some(vk) = rdev_key_to_vk(&key) {
                        pressed_keys_listener.write().remove(&vk);
                        println!("Key released: {:?} (vk: {})", key, vk);
                    }
                }
                _ => {}
            }
        }) {
            eprintln!("Error starting keyboard listener: {:?}", error);
        }
    });

    // Thread principal pour vérifier les raccourcis
    std::thread::spawn(move || {
        let app_handle = app.clone();
        let mut is_recording = false;
        let mut last_transcript_pressed = false;

        println!("Starting shortcut checker thread...");

        loop {
            let record_required_keys = app_handle.state::<RecordShortcutKeys>().get();
            let last_transcript_required_keys = app_handle.state::<LastTranscriptShortcutKeys>().get();

            if record_required_keys.is_empty() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let pressed = pressed_keys_checker.read();
            let all_record_keys_down = record_required_keys.iter().all(|k| pressed.contains(k));
            let all_last_transcript_keys_down = !last_transcript_required_keys.is_empty()
                && last_transcript_required_keys.iter().all(|k| pressed.contains(k));

            // Gestion du raccourci d'enregistrement
            if !is_recording && all_record_keys_down {
                println!("Recording started with keys: {:?}", record_required_keys);
                record_audio(&app_handle);
                is_recording = true;
                let _ = app_handle.emit("shortcut:start", keys_to_string(&record_required_keys));
            }
            if is_recording && !all_record_keys_down {
                println!("Recording stopped");
                let _ = stop_recording(&app_handle);
                is_recording = false;
                let _ = app_handle.emit("shortcut:stop", keys_to_string(&record_required_keys));
            }

            // Gestion du raccourci de dernière transcription
            if !last_transcript_pressed && all_last_transcript_keys_down {
                println!("Last transcript shortcut triggered");
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