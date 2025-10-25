use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

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
    keys.iter()
        .map(|vk| vk_to_key_name(*vk))
        .collect::<Vec<_>>()
        .join("+")
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::init_shortcuts;
#[cfg(target_os = "windows")]
pub use windows::init_shortcuts;

pub struct TranscriptionSuspended(pub Arc<AtomicBool>);

impl TranscriptionSuspended {
    pub fn new(suspended: bool) -> Self {
        Self(Arc::new(AtomicBool::new(suspended)))
    }
    pub fn get(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn set(&self, value: bool) {
        self.0.store(value, Ordering::SeqCst)
    }
}
