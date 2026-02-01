//! macOS keyboard shortcut handling using rdev
//!
//! This implementation uses rdev for global keyboard event capture,
//! which requires Accessibility permissions on macOS.

use log::{debug, error, warn};
use parking_lot::Mutex;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::accessibility_macos;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState};

struct EventProcessor {
    app_handle: AppHandle,
    pressed_keys: Mutex<HashSet<i32>>,
    last_press_times: Mutex<Vec<Instant>>,
    active_bindings: Mutex<HashSet<usize>>,
}

impl EventProcessor {
    fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            pressed_keys: Mutex::new(HashSet::new()),
            last_press_times: Mutex::new(Vec::new()),
            active_bindings: Mutex::new(HashSet::new()),
        }
    }

    fn handle_key_press(&self, key: i32) {
        self.pressed_keys.lock().insert(key);
        self.check_press();
    }

    fn handle_key_release(&self, key: i32) {
        self.check_release();
        self.pressed_keys.lock().remove(&key);
    }

    fn check_press(&self) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        if shortcut_state.is_suspended() {
            debug!("check_press: shortcuts are suspended, skipping");
            return;
        }

        let registry_state = self.app_handle.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        let pressed = self.pressed_keys.lock();
        let mut press_times = self.last_press_times.lock();
        let mut active = self.active_bindings.lock();

        debug!(
            "check_press: pressed_keys={:?}, bindings_count={}",
            &*pressed,
            registry.bindings.len()
        );

        while press_times.len() < registry.bindings.len() {
            press_times.push(Instant::now() - Duration::from_secs(1));
        }

        for (i, binding) in registry.bindings.iter().enumerate() {
            if binding.keys.is_empty() {
                debug!("check_press: binding {} has empty keys, skipping", i);
                continue;
            }
            if active.contains(&i) {
                continue;
            }

            let all_pressed = binding.keys.iter().all(|k| pressed.contains(k));
            debug!(
                "check_press: binding {} keys={:?}, all_pressed={}",
                i, binding.keys, all_pressed
            );
            if !all_pressed {
                continue;
            }

            // Debounce only for repeated presses (key auto-repeat)
            if press_times[i].elapsed() < Duration::from_millis(150) {
                continue;
            }

            debug!("Shortcut Pressed: {:?}", binding.action);
            press_times[i] = Instant::now();
            active.insert(i);

            drop(pressed);
            drop(press_times);
            drop(active);

            crate::shortcuts::handle_shortcut_event(
                &self.app_handle,
                &binding.action,
                &binding.activation_mode,
                KeyEventType::Pressed,
            );
            return;
        }
    }

    fn check_release(&self) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        if shortcut_state.is_suspended() {
            return;
        }

        let registry_state = self.app_handle.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        let pressed = self.pressed_keys.lock();
        let mut active = self.active_bindings.lock();

        for (i, binding) in registry.bindings.iter().enumerate() {
            if !active.contains(&i) {
                continue;
            }

            // Check if any key of this binding was released
            let all_still_pressed = binding.keys.iter().all(|k| pressed.contains(k));
            if all_still_pressed {
                continue;
            }

            debug!("Shortcut Released: {:?}", binding.action);
            active.remove(&i);

            drop(pressed);
            drop(active);

            crate::shortcuts::handle_shortcut_event(
                &self.app_handle,
                &binding.action,
                &binding.activation_mode,
                KeyEventType::Released,
            );
            return;
        }
    }
}

pub fn init(app: AppHandle) {
    // Check Accessibility permission first
    if !accessibility_macos::check_and_log_permission() {
        warn!("Accessibility permission not granted - emitting event to frontend");
        let _ = app.emit("accessibility-permission-missing", ());
        return;
    }

    let processor = Arc::new(EventProcessor::new(app.clone()));
    let (tx, rx) = channel::<(i32, bool)>(); // (key, is_pressed)

    // Log registered bindings for debugging
    {
        let registry_state = app.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        for (i, binding) in registry.bindings.iter().enumerate() {
            debug!(
                "Registered binding {}: action={:?}, keys={:?}",
                i, binding.action, binding.keys
            );
        }
    }

    std::thread::spawn(move || {
        debug!("Starting rdev keyboard listener (macOS)");
        if let Err(e) = listen(move |event: Event| {
            if let Some((key, is_pressed)) = convert_event(&event) {
                let _ = tx.send((key, is_pressed));
            }
        }) {
            error!("rdev listener error: {:?}", e);
        }
    });

    std::thread::spawn(move || {
        debug!("Starting shortcut processor (macOS)");
        while let Ok((key, is_pressed)) = rx.recv() {
            debug!("Key event received: key={}, pressed={}", key, is_pressed);
            if is_pressed {
                processor.handle_key_press(key);
            } else {
                processor.handle_key_release(key);
            }
            debug!("Key event processed successfully");
        }
        warn!("Shortcut processor stopped");
    });
}

/// Extract a single character from rdev's UnicodeInfo for shortcut matching.
/// Uses the decoded name (first character); skips dead keys.
fn unicode_info_to_char(info: &rdev::UnicodeInfo) -> Option<char> {
    info.name.as_ref().and_then(|s| s.chars().next())
}

fn convert_event(event: &Event) -> Option<(i32, bool)> {
    debug!(
        "convert_event: event_type={:?}, unicode={:?}",
        event.event_type, event.unicode
    );
    match &event.event_type {
        EventType::KeyPress(key) => {
            // Try to use event.unicode for alphanumeric keys (respects keyboard layout)
            if let Some(ref unicode_info) = event.unicode {
                if let Some(c) = unicode_info_to_char(unicode_info) {
                    if let Some(vk) = char_to_vk(c) {
                        debug!("convert_event: using unicode '{}' -> vk={}", c, vk);
                        return Some((vk, true));
                    }
                }
            }
            // Fall back to physical key mapping for modifiers and special keys
            let result = rdev_key_to_vk(key).map(|k| (k, true));
            debug!("convert_event: using rdev_key {:?} -> {:?}", key, result);
            result
        }
        EventType::KeyRelease(key) => {
            if let Some(ref unicode_info) = event.unicode {
                if let Some(c) = unicode_info_to_char(unicode_info) {
                    if let Some(vk) = char_to_vk(c) {
                        debug!("convert_event: using unicode '{}' -> vk={}", c, vk);
                        return Some((vk, false));
                    }
                }
            }
            let result = rdev_key_to_vk(key).map(|k| (k, false));
            debug!("convert_event: using rdev_key {:?} -> {:?}", key, result);
            result
        }
        _ => None,
    }
}

/// Convert a unicode character to VK code
/// This handles keyboard layout properly (e.g., AZERTY vs QWERTY)
fn char_to_vk(c: char) -> Option<i32> {
    match c.to_ascii_lowercase() {
        'a' => Some(0x41),
        'b' => Some(0x42),
        'c' => Some(0x43),
        'd' => Some(0x44),
        'e' => Some(0x45),
        'f' => Some(0x46),
        'g' => Some(0x47),
        'h' => Some(0x48),
        'i' => Some(0x49),
        'j' => Some(0x4A),
        'k' => Some(0x4B),
        'l' => Some(0x4C),
        'm' => Some(0x4D),
        'n' => Some(0x4E),
        'o' => Some(0x4F),
        'p' => Some(0x50),
        'q' => Some(0x51),
        'r' => Some(0x52),
        's' => Some(0x53),
        't' => Some(0x54),
        'u' => Some(0x55),
        'v' => Some(0x56),
        'w' => Some(0x57),
        'x' => Some(0x58),
        'y' => Some(0x59),
        'z' => Some(0x5A),
        '0' => Some(0x30),
        '1' => Some(0x31),
        '2' => Some(0x32),
        '3' => Some(0x33),
        '4' => Some(0x34),
        '5' => Some(0x35),
        '6' => Some(0x36),
        '7' => Some(0x37),
        '8' => Some(0x38),
        '9' => Some(0x39),
        ' ' => Some(0x20),
        _ => None,
    }
}

fn rdev_key_to_vk(key: &Key) -> Option<i32> {
    match key {
        // macOS: Command key maps to Meta
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
