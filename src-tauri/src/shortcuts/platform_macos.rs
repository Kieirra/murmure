//! macOS keyboard shortcut handling using polling
//!
//! This implementation polls keyboard state using CGEventSourceKeyState
//! and CGEventSourceFlagsState, similar to the Windows GetAsyncKeyState approach.
//! This avoids event corruption issues with rdev when enigo simulates key events.

use core_foundation::base::CFRelease;
use core_foundation::string::UniChar;
use core_foundation_sys::data::CFDataGetBytePtr;
use log::debug;
use std::collections::HashSet;
use std::ffi::c_void;
use std::os::raw::c_uint;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

// FFI types for keyboard layout conversion (needed for AZERTY/QWERTY mapping)
type TISInputSourceRef = *mut c_void;
type OptionBits = c_uint;

#[allow(non_upper_case_globals)]
const kUCKeyTranslateDeadKeysBit: OptionBits = 1 << 31;
#[allow(non_upper_case_globals)]
const kUCKeyActionDown: u16 = 0;
const BUF_LEN: usize = 4;

#[link(name = "Cocoa", kind = "framework")]
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
    fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> TISInputSourceRef;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, property: *const c_void)
        -> *mut c_void;
    fn UCKeyTranslate(
        layout: *const u8,
        code: u16,
        key_action: u16,
        modifier_state: u32,
        keyboard_type: u32,
        key_translate_options: OptionBits,
        dead_key_state: *mut u32,
        max_length: usize,
        actual_length: *mut usize,
        unicode_string: *mut [UniChar; BUF_LEN],
    ) -> i32;
    fn LMGetKbdType() -> u8;
    static kTISPropertyUnicodeKeyLayoutData: *mut c_void;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventSourceKeyState(stateID: i32, key: u16) -> bool;
    fn CGEventSourceFlagsState(stateID: i32) -> u64;
    fn CGEventSourceButtonState(stateID: i32, button: u32) -> bool;
}

const CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x00040000;
const CG_EVENT_FLAG_MASK_SHIFT: u64 = 0x00020000;
const CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x00080000;
const CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x00100000;

const MODIFIER_KEYS: &[i32] = &[0x11, 0x10, 0x12, 0x5B];

use crate::shortcuts::accessibility_macos;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState};

/// Convert a macOS physical keycode to the logical character using the current keyboard layout.
/// Used to resolve layout-dependent keys (letters, digits, OEM) for AZERTY/QWERTY support.
fn keycode_to_char(keycode: u32) -> Option<char> {
    unsafe {
        let mut keyboard = TISCopyCurrentKeyboardInputSource();
        let mut layout = std::ptr::null_mut();

        if !keyboard.is_null() {
            layout = TISGetInputSourceProperty(keyboard, kTISPropertyUnicodeKeyLayoutData);
        }

        if layout.is_null() {
            if !keyboard.is_null() {
                CFRelease(keyboard);
            }
            keyboard = TISCopyCurrentKeyboardLayoutInputSource();
            if !keyboard.is_null() {
                layout = TISGetInputSourceProperty(keyboard, kTISPropertyUnicodeKeyLayoutData);
            }
        }

        if layout.is_null() {
            if !keyboard.is_null() {
                CFRelease(keyboard);
            }
            keyboard = TISCopyCurrentASCIICapableKeyboardLayoutInputSource();
            if !keyboard.is_null() {
                layout = TISGetInputSourceProperty(keyboard, kTISPropertyUnicodeKeyLayoutData);
            }
        }

        if layout.is_null() {
            if !keyboard.is_null() {
                CFRelease(keyboard);
            }
            return None;
        }

        let layout_ptr = CFDataGetBytePtr(layout as _);
        if layout_ptr.is_null() {
            CFRelease(keyboard);
            return None;
        }

        let mut buff = [0_u16; BUF_LEN];
        let kb_type = LMGetKbdType();
        let mut length = 0;
        let mut dead_state = 0u32;

        let _retval = UCKeyTranslate(
            layout_ptr,
            keycode as u16,
            kUCKeyActionDown,
            0,
            kb_type as u32,
            kUCKeyTranslateDeadKeysBit,
            &mut dead_state,
            BUF_LEN,
            &mut length,
            &mut buff,
        );

        CFRelease(keyboard);

        if length == 0 {
            return None;
        }

        String::from_utf16(&buff[..length])
            .ok()
            .and_then(|s| s.chars().next())
    }
}

/// Map a VK code to the macOS physical keycode for layout-independent keys.
/// Returns None for layout-dependent keys (letters, digits, OEM) which need UCKeyTranslate.
fn vk_to_fixed_keycode(vk: i32) -> Option<u16> {
    match vk {
        0x20 => Some(0x31), // Space
        0x0D => Some(0x24), // Return
        0x1B => Some(0x35), // Escape
        0x09 => Some(0x30), // Tab
        0x08 => Some(0x33), // Backspace
        0x2E => Some(0x75), // Forward Delete
        0x2D => Some(0x72), // Insert (Help on Mac)
        0x24 => Some(0x73), // Home
        0x23 => Some(0x77), // End
        0x21 => Some(0x74), // Page Up
        0x22 => Some(0x79), // Page Down
        0x26 => Some(0x7E), // Up Arrow
        0x28 => Some(0x7D), // Down Arrow
        0x25 => Some(0x7B), // Left Arrow
        0x27 => Some(0x7C), // Right Arrow
        0xC0 => Some(0x32), // BackQuote/Grave
        0xE2 => Some(0x0A), // IntlBackslash (ISO keyboards)
        // F-keys
        0x70 => Some(0x7A), // F1
        0x71 => Some(0x78), // F2
        0x72 => Some(0x63), // F3
        0x73 => Some(0x76), // F4
        0x74 => Some(0x60), // F5
        0x75 => Some(0x61), // F6
        0x76 => Some(0x62), // F7
        0x77 => Some(0x64), // F8
        0x78 => Some(0x65), // F9
        0x79 => Some(0x6D), // F10
        0x7A => Some(0x67), // F11
        0x7B => Some(0x6F), // F12
        0x7C => Some(0x69), // F13
        0x7D => Some(0x6B), // F14
        0x7E => Some(0x71), // F15
        0x7F => Some(0x6A), // F16
        0x80 => Some(0x40), // F17
        0x81 => Some(0x4F), // F18
        0x82 => Some(0x50), // F19
        0x83 => Some(0x5A), // F20
        // Numpad
        0x60 => Some(0x52), // Numpad 0
        0x61 => Some(0x53), // Numpad 1
        0x62 => Some(0x54), // Numpad 2
        0x63 => Some(0x55), // Numpad 3
        0x64 => Some(0x56), // Numpad 4
        0x65 => Some(0x57), // Numpad 5
        0x66 => Some(0x58), // Numpad 6
        0x67 => Some(0x59), // Numpad 7
        0x68 => Some(0x5B), // Numpad 8
        0x69 => Some(0x5C), // Numpad 9
        0x6A => Some(0x43), // Numpad Multiply
        0x6B => Some(0x45), // Numpad Plus
        0x6D => Some(0x4E), // Numpad Minus
        0x6F => Some(0x4B), // Numpad Divide
        _ => None,
    }
}

/// Map a character to its Windows VK code.
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
        '-' => Some(0xBD),
        '=' => Some(0xBB),
        '[' => Some(0xDB),
        ']' => Some(0xDD),
        ';' => Some(0xBA),
        '\'' => Some(0xDE),
        ',' => Some(0xBC),
        '.' => Some(0xBE),
        '/' => Some(0xBF),
        '\\' => Some(0xDC),
        _ => None,
    }
}

/// Find the macOS physical keycode for a layout-dependent VK code by scanning
/// all keycodes with UCKeyTranslate. Returns the first keycode that produces
/// the character matching the given VK code.
fn find_layout_keycode(vk: i32) -> Option<u16> {
    let target_char = match vk {
        0x41..=0x5A => Some((b'a' + (vk - 0x41) as u8) as char),
        0x30..=0x39 => Some((b'0' + (vk - 0x30) as u8) as char),
        0xBD => Some('-'),
        0xBB => Some('='),
        0xDB => Some('['),
        0xDD => Some(']'),
        0xBA => Some(';'),
        0xDE => Some('\''),
        0xBC => Some(','),
        0xBE => Some('.'),
        0xBF => Some('/'),
        0xDC => Some('\\'),
        _ => None,
    }?;

    for keycode in 0..128u16 {
        if let Some(c) = keycode_to_char(keycode as u32) {
            if c.to_ascii_lowercase() == target_char {
                return Some(keycode);
            }
        }
    }
    None
}

fn is_modifier_pressed(vk: i32) -> bool {
    let flags = unsafe { CGEventSourceFlagsState(0) };
    match vk {
        0x11 => flags & CG_EVENT_FLAG_MASK_CONTROL != 0,
        0x10 => flags & CG_EVENT_FLAG_MASK_SHIFT != 0,
        0x12 => flags & CG_EVENT_FLAG_MASK_ALTERNATE != 0,
        0x5B => flags & CG_EVENT_FLAG_MASK_COMMAND != 0,
        _ => false,
    }
}

fn is_key_pressed(vk: i32) -> bool {
    if MODIFIER_KEYS.contains(&vk) {
        return is_modifier_pressed(vk);
    }
    // Mouse buttons (CGMouseButton: 0=Left, 1=Right, 2=Middle, 3=Back, 4=Forward)
    match vk {
        0x01 => return unsafe { CGEventSourceButtonState(0, 0) },
        0x02 => return unsafe { CGEventSourceButtonState(0, 1) },
        0x04 => return unsafe { CGEventSourceButtonState(0, 2) },
        0x05 => return unsafe { CGEventSourceButtonState(0, 3) },
        0x06 => return unsafe { CGEventSourceButtonState(0, 4) },
        _ => {}
    }
    // Fixed keys (layout-independent)
    if let Some(keycode) = vk_to_fixed_keycode(vk) {
        return unsafe { CGEventSourceKeyState(0, keycode) };
    }
    // Layout-dependent keys (letters, digits, OEM) - resolved via UCKeyTranslate
    if let Some(keycode) = find_layout_keycode(vk) {
        return unsafe { CGEventSourceKeyState(0, keycode) };
    }
    false
}

pub fn init(app: AppHandle) {
    if !accessibility_macos::check_and_log_permission() {
        log::warn!("Accessibility permission not granted - emitting event to frontend");
        let _ = app.emit("accessibility-permission-missing", ());
        return;
    }

    {
        let registry_state = app.state::<ShortcutRegistryState>();
        let registry = registry_state.0.read();
        debug!(
            "[macOS shortcuts] Registry has {} bindings",
            registry.bindings.len()
        );
        for (i, binding) in registry.bindings.iter().enumerate() {
            debug!(
                "[macOS shortcuts] Binding {}: action={:?}, keys={:?}",
                i, binding.action, binding.keys
            );
        }
    }

    std::thread::spawn(move || {
        debug!("[macOS shortcuts] Starting keyboard polling");

        let mut active_bindings: HashSet<usize> = HashSet::new();
        let mut last_press_times: Vec<Instant> = Vec::new();

        loop {
            let shortcut_state = app.state::<ShortcutState>();
            if shortcut_state.is_suspended() {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let registry_state = app.state::<ShortcutRegistryState>();
            let registry = registry_state.0.read();

            while last_press_times.len() < registry.bindings.len() {
                last_press_times.push(Instant::now() - Duration::from_secs(1));
            }

            for (i, binding) in registry.bindings.iter().enumerate() {
                if binding.keys.is_empty() {
                    continue;
                }

                let all_pressed = binding.keys.iter().all(|&k| is_key_pressed(k));
                let extra_modifier_pressed = MODIFIER_KEYS
                    .iter()
                    .any(|&vk| !binding.keys.contains(&vk) && is_modifier_pressed(vk));

                if all_pressed && !extra_modifier_pressed && !active_bindings.contains(&i) {
                    if last_press_times[i].elapsed() < Duration::from_millis(150) {
                        continue;
                    }

                    debug!("Shortcut Pressed: {:?}", binding.action);
                    last_press_times[i] = Instant::now();
                    active_bindings.insert(i);

                    let action = binding.action.clone();
                    let mode = binding.activation_mode.clone();
                    drop(registry);

                    crate::shortcuts::handle_shortcut_event(
                        &app,
                        &action,
                        &mode,
                        KeyEventType::Pressed,
                    );
                    break;
                } else if !all_pressed && active_bindings.contains(&i) {
                    debug!("Shortcut Released: {:?}", binding.action);
                    active_bindings.remove(&i);

                    let action = binding.action.clone();
                    let mode = binding.activation_mode.clone();
                    drop(registry);

                    crate::shortcuts::handle_shortcut_event(
                        &app,
                        &action,
                        &mode,
                        KeyEventType::Released,
                    );
                    break;
                }
            }

            std::thread::sleep(Duration::from_millis(32));
        }
    });

    debug!("[macOS shortcuts] Initialization complete");
}
