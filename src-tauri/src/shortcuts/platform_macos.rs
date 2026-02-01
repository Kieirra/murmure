use core_foundation::base::TCFType;
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
    EventField,
};
use log::{debug, error, info, warn};
use parking_lot::Mutex;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState};

// FFI bindings for macOS Accessibility API
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

/// Check if the application has Accessibility permissions
pub fn has_accessibility_permissions() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Open System Preferences to the Accessibility pane
pub fn open_accessibility_preferences() {
    let _ = std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
}

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

    fn handle_flags_changed(&self, flags: u64, keycode: i32) {
        // Handle modifier key press/release via flags
        let mut pressed = self.pressed_keys.lock();

        // Check each modifier flag and update pressed_keys accordingly
        // CGEventFlags values:
        // Shift: 0x20000 (131072)
        // Control: 0x40000 (262144)
        // Option/Alt: 0x80000 (524288)
        // Command: 0x100000 (1048576)

        let shift_pressed = (flags & 0x20000) != 0;
        let ctrl_pressed = (flags & 0x40000) != 0;
        let alt_pressed = (flags & 0x80000) != 0;
        let cmd_pressed = (flags & 0x100000) != 0;

        // Map to Windows virtual key codes
        let shift_vk = 0x10; // VK_SHIFT
        let ctrl_vk = 0x11; // VK_CONTROL
        let alt_vk = 0x12; // VK_MENU
        let cmd_vk = 0x5B; // VK_LWIN (we use Win key for Command)

        if shift_pressed {
            pressed.insert(shift_vk);
        } else {
            pressed.remove(&shift_vk);
        }

        if ctrl_pressed {
            pressed.insert(ctrl_vk);
        } else {
            pressed.remove(&ctrl_vk);
        }

        if alt_pressed {
            pressed.insert(alt_vk);
        } else {
            pressed.remove(&alt_vk);
        }

        if cmd_pressed {
            pressed.insert(cmd_vk);
        } else {
            pressed.remove(&cmd_vk);
        }

        drop(pressed);

        // Determine if this was a press or release based on whether
        // the keycode corresponds to a modifier that's now pressed
        let is_press = match keycode {
            56 | 60 => shift_pressed, // Shift keys
            59 | 62 => ctrl_pressed,  // Control keys
            58 | 61 => alt_pressed,   // Option keys
            55 | 54 => cmd_pressed,   // Command keys
            _ => false,
        };

        if is_press {
            self.check_press();
        } else {
            self.check_release();
        }
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

#[derive(Debug)]
enum KeyEvent {
    KeyDown(i32),
    KeyUp(i32),
    FlagsChanged(u64, i32),
}

pub fn init(app: AppHandle) {
    // Check accessibility permissions at startup
    if !has_accessibility_permissions() {
        warn!("Accessibility permissions not granted. Global shortcuts will not work.");
        info!("Please grant Accessibility permissions in System Preferences > Security & Privacy > Privacy > Accessibility");

        // Emit event to frontend to show permission dialog
        let _ = app.emit("accessibility-permission-required", ());

        // Open System Preferences automatically
        open_accessibility_preferences();
    } else {
        info!("Accessibility permissions granted");
    }

    let processor = Arc::new(EventProcessor::new(app.clone()));
    let app_for_error = app.clone();
    let (tx, rx) = channel::<KeyEvent>();

    // Thread 1: CGEventTap listener
    std::thread::spawn(move || {
        debug!("Starting macOS CGEventTap keyboard listener");

        let tx_clone = tx.clone();
        let callback = move |_proxy: core_graphics::event::CGEventTapProxy,
                             event_type: CGEventType,
                             event: &CGEvent|
              -> Option<CGEvent> {
            let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as i32;

            match event_type {
                CGEventType::KeyDown => {
                    if let Some(vk) = macos_keycode_to_vk(keycode) {
                        let _ = tx_clone.send(KeyEvent::KeyDown(vk));
                    }
                }
                CGEventType::KeyUp => {
                    if let Some(vk) = macos_keycode_to_vk(keycode) {
                        let _ = tx_clone.send(KeyEvent::KeyUp(vk));
                    }
                }
                CGEventType::FlagsChanged => {
                    let flags = event.get_flags().bits();
                    let _ = tx_clone.send(KeyEvent::FlagsChanged(flags, keycode));
                }
                _ => {}
            }

            // Pass the event through unchanged
            Some(event.clone())
        };

        match CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::ListenOnly,
            vec![
                CGEventType::KeyDown,
                CGEventType::KeyUp,
                CGEventType::FlagsChanged,
            ],
            callback,
        ) {
            Ok(tap) => {
                unsafe {
                    let loop_source = tap
                        .mach_port
                        .create_runloop_source(0)
                        .expect("Failed to create run loop source");
                    let current = CFRunLoop::get_current();
                    current.add_source(&loop_source, kCFRunLoopCommonModes);
                    tap.enable();
                    debug!("CGEventTap enabled, starting run loop");
                    CFRunLoop::run_current();
                }
                warn!("CGEventTap run loop exited");
            }
            Err(()) => {
                error!(
                    "Failed to create CGEventTap. Make sure the app has Accessibility permissions."
                );
                // Emit event to frontend
                let _ = app_for_error.emit("accessibility-permission-required", ());
                // Try to open System Preferences
                open_accessibility_preferences();
            }
        }
    });

    // Thread 2: Event processor
    std::thread::spawn(move || {
        debug!("Starting macOS shortcut processor");
        while let Ok(event) = rx.recv() {
            match event {
                KeyEvent::KeyDown(vk) => processor.handle_key_press(vk),
                KeyEvent::KeyUp(vk) => processor.handle_key_release(vk),
                KeyEvent::FlagsChanged(flags, keycode) => {
                    processor.handle_flags_changed(flags, keycode)
                }
            }
        }
        warn!("macOS shortcut processor stopped");
    });
}

/// Convert macOS keycode to Windows virtual key code
fn macos_keycode_to_vk(keycode: i32) -> Option<i32> {
    match keycode {
        // Letters (macOS keycodes are not in alphabetical order)
        0 => Some(0x41),  // A
        11 => Some(0x42), // B
        8 => Some(0x43),  // C
        2 => Some(0x44),  // D
        14 => Some(0x45), // E
        3 => Some(0x46),  // F
        5 => Some(0x47),  // G
        4 => Some(0x48),  // H
        34 => Some(0x49), // I
        38 => Some(0x4A), // J
        40 => Some(0x4B), // K
        37 => Some(0x4C), // L
        46 => Some(0x4D), // M
        45 => Some(0x4E), // N
        31 => Some(0x4F), // O
        35 => Some(0x50), // P
        12 => Some(0x51), // Q
        15 => Some(0x52), // R
        1 => Some(0x53),  // S
        17 => Some(0x54), // T
        32 => Some(0x55), // U
        9 => Some(0x56),  // V
        13 => Some(0x57), // W
        7 => Some(0x58),  // X
        16 => Some(0x59), // Y
        6 => Some(0x5A),  // Z

        // Numbers (top row)
        29 => Some(0x30), // 0
        18 => Some(0x31), // 1
        19 => Some(0x32), // 2
        20 => Some(0x33), // 3
        21 => Some(0x34), // 4
        23 => Some(0x35), // 5
        22 => Some(0x36), // 6
        26 => Some(0x37), // 7
        28 => Some(0x38), // 8
        25 => Some(0x39), // 9

        // Function keys
        122 => Some(0x70), // F1
        120 => Some(0x71), // F2
        99 => Some(0x72),  // F3
        118 => Some(0x73), // F4
        96 => Some(0x74),  // F5
        97 => Some(0x75),  // F6
        98 => Some(0x76),  // F7
        100 => Some(0x77), // F8
        101 => Some(0x78), // F9
        109 => Some(0x79), // F10
        103 => Some(0x7A), // F11
        111 => Some(0x7B), // F12

        // Special keys
        49 => Some(0x20),  // Space
        36 => Some(0x0D),  // Return/Enter
        53 => Some(0x1B),  // Escape
        48 => Some(0x09),  // Tab
        51 => Some(0x08),  // Backspace/Delete
        117 => Some(0x2E), // Forward Delete
        114 => Some(0x2D), // Insert (Help key on Mac)
        115 => Some(0x24), // Home
        119 => Some(0x23), // End
        116 => Some(0x21), // Page Up
        121 => Some(0x22), // Page Down

        // Arrow keys
        126 => Some(0x26), // Up Arrow
        125 => Some(0x28), // Down Arrow
        123 => Some(0x25), // Left Arrow
        124 => Some(0x27), // Right Arrow

        // Modifier keys (handled via FlagsChanged, but included for completeness)
        56 | 60 => Some(0x10), // Shift (left/right)
        59 | 62 => Some(0x11), // Control (left/right)
        58 | 61 => Some(0x12), // Option/Alt (left/right)
        55 | 54 => Some(0x5B), // Command (left/right) -> Windows key

        _ => None,
    }
}
