use log::{debug, error, warn};
use parking_lot::Mutex;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

use crate::shortcuts::types::{ShortcutRegistryState, ShortcutState};

#[derive(Debug)]
enum KeyEvent {
    Pressed(i32),
    Released(i32),
}

struct EventProcessor {
    app_handle: AppHandle,
    pressed_keys: Mutex<HashSet<i32>>,
    last_trigger_times: Mutex<Vec<Instant>>,
}

impl EventProcessor {
    fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            pressed_keys: Mutex::new(HashSet::new()),
            last_trigger_times: Mutex::new(Vec::new()),
        }
    }

    fn handle_event(&self, event: KeyEvent) {
        match event {
            KeyEvent::Pressed(key) => {
                self.pressed_keys.lock().insert(key);
                self.check_bindings();
            }
            KeyEvent::Released(key) => {
                self.pressed_keys.lock().remove(&key);
                crate::shortcuts::check_release_stop(&self.app_handle, key);
            }
        }
    }

    fn check_bindings(&self) {
        let shortcut_state = self.app_handle.state::<ShortcutState>();
        if shortcut_state.is_suspended() {
            return;
        }

        let registry_state = self.app_handle.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        let pressed = self.pressed_keys.lock();
        let mut trigger_times = self.last_trigger_times.lock();

        while trigger_times.len() < registry.bindings.len() {
            trigger_times.push(Instant::now() - Duration::from_secs(1));
        }

        for (i, binding) in registry.bindings.iter().enumerate() {
            if binding.keys.is_empty() {
                continue;
            }

            let all_pressed = binding.keys.iter().all(|k| pressed.contains(k));
            if !all_pressed {
                continue;
            }

            if trigger_times[i].elapsed() < Duration::from_millis(150) {
                continue;
            }

            debug!("Shortcut triggered: {:?}", binding.action);
            trigger_times[i] = Instant::now();
            drop(pressed);
            drop(trigger_times);

            crate::shortcuts::execute_action(
                &self.app_handle,
                &binding.action,
                &binding.activation_mode,
                &binding.keys,
            );
            return;
        }
    }
}

pub fn init(app: AppHandle) {
    let processor = Arc::new(EventProcessor::new(app));
    let (tx, rx) = channel::<KeyEvent>();

    std::thread::spawn(move || {
        debug!("Starting rdev keyboard listener");
        if let Err(e) = listen(move |event: Event| {
            if let Some(evt) = convert_event(&event) {
                let _ = tx.send(evt);
            }
        }) {
            error!("rdev listener error: {:?}", e);
        }
    });

    std::thread::spawn(move || {
        debug!("Starting shortcut processor");
        while let Ok(event) = rx.recv() {
            processor.handle_event(event);
        }
        warn!("Shortcut processor stopped");
    });
}

fn convert_event(event: &Event) -> Option<KeyEvent> {
    match event.event_type {
        EventType::KeyPress(key) => rdev_key_to_vk(&key).map(KeyEvent::Pressed),
        EventType::KeyRelease(key) => rdev_key_to_vk(&key).map(KeyEvent::Released),
        _ => None,
    }
}

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
