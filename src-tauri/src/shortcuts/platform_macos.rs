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
            return;
        }

        let registry_state = self.app_handle.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        let pressed = self.pressed_keys.lock();
        let mut press_times = self.last_press_times.lock();
        let mut active = self.active_bindings.lock();

        while press_times.len() < registry.bindings.len() {
            press_times.push(Instant::now() - Duration::from_secs(1));
        }

        for (i, binding) in registry.bindings.iter().enumerate() {
            if binding.keys.is_empty() || active.contains(&i) {
                continue;
            }

            let all_pressed = binding.keys.iter().all(|k| pressed.contains(k));
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

    let processor = Arc::new(EventProcessor::new(app));
    let (tx, rx) = channel::<(i32, bool)>(); // (key, is_pressed)

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
            if is_pressed {
                processor.handle_key_press(key);
            } else {
                processor.handle_key_release(key);
            }
        }
        warn!("Shortcut processor stopped");
    });
}

fn convert_event(event: &Event) -> Option<(i32, bool)> {
    match event.event_type {
        EventType::KeyPress(key) => rdev_key_to_vk(&key).map(|k| (k, true)),
        EventType::KeyRelease(key) => rdev_key_to_vk(&key).map(|k| (k, false)),
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
