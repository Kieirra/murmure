//! macOS keyboard shortcut handling using polling
//!
//! This implementation polls keyboard state using CGEventSourceKeyState
//! and CGEventSourceFlagsState, similar to the Windows GetAsyncKeyState approach.
//! This avoids event corruption issues with rdev when enigo simulates key events.

use core_foundation::base::CFRelease;
use core_foundation::string::UniChar;
use core_foundation_sys::data::CFDataGetBytePtr;
use log::debug;
use std::collections::{HashMap, HashSet};
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
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: u64,
        callback: extern "C" fn(*mut c_void, u32, *mut c_void, *mut c_void) -> *mut c_void,
        user_info: *mut c_void,
    ) -> *mut c_void;
    fn CGEventGetIntegerValueField(event: *mut c_void, field: u32) -> i64;
    fn CGEventGetFlags(event: *mut c_void) -> u64;
    fn CFMachPortCreateRunLoopSource(
        allocator: *mut c_void,
        port: *mut c_void,
        order: i64,
    ) -> *mut c_void;
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *mut c_void);
    fn CFRunLoopRun();
    static kCFRunLoopCommonModes: *mut c_void;
}

const CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x00040000;
const CG_EVENT_FLAG_MASK_SHIFT: u64 = 0x00020000;
const CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x00080000;
const CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x00100000;

// Use both session state AND HID state for maximum reliability.
// Session state (0) can flicker with modal windows, HID state (1) can flicker
// in other contexts. By OR-ing both, a key is only considered "released" when
// BOTH sources agree — this prevents false releases in modal/overlay windows.
#[allow(non_upper_case_globals)]
const kCGEventSourceStateCombinedSessionState: i32 = 0;
#[allow(non_upper_case_globals)]
const kCGEventSourceStateHIDSystemState: i32 = 1;

const MODIFIER_KEYS: &[i32] = &[0x11, 0x10, 0x12, 0x5B];

use crate::shortcuts::accessibility_macos;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState};

// CGEventTap constants
const K_CG_SESSION_EVENT_TAP: u32 = 1;
const K_CG_HEAD_INSERT_EVENT_TAP: u32 = 0;
const K_CG_EVENT_TAP_OPTION_DEFAULT: u32 = 0;
const K_CG_EVENT_KEY_DOWN: u64 = 1 << 10;
const K_CG_EVENT_KEY_UP: u64 = 1 << 11;
const K_CG_KEYBOARD_EVENT_KEYCODE: u32 = 9;

/// Context passed to the CGEventTap callback for event suppression.
struct TapContext {
    /// Reverse map: macOS physical keycode → Windows VK code
    reverse_keycode_map: HashMap<u16, i32>,
    app_handle: AppHandle,
}

/// CGEventTap callback that suppresses keyboard events matching registered shortcuts.
/// This prevents macOS from playing the alert "bip" sound when a shortcut key
/// combination is not recognized by the frontmost application.
/// Returns NULL to suppress, or the event pointer to pass through.
extern "C" fn suppress_callback(
    _proxy: *mut c_void,
    event_type: u32,
    event: *mut c_void,
    user_info: *mut c_void,
) -> *mut c_void {
    if user_info.is_null() || event.is_null() {
        return event;
    }

    // Only suppress key down and key up events (10 and 11)
    if event_type != 10 && event_type != 11 {
        return event;
    }

    let ctx = unsafe { &*(user_info as *const TapContext) };

    let keycode = unsafe { CGEventGetIntegerValueField(event, K_CG_KEYBOARD_EVENT_KEYCODE) } as u16;
    let key_vk = match ctx.reverse_keycode_map.get(&keycode) {
        Some(&vk) => vk,
        None => return event,
    };

    // Skip if the key itself is a modifier (modifiers don't cause the bip)
    if MODIFIER_KEYS.contains(&key_vk) {
        return event;
    }

    let flags = unsafe { CGEventGetFlags(event) };
    let mut pressed_vks: Vec<i32> = Vec::with_capacity(5);
    if flags & CG_EVENT_FLAG_MASK_CONTROL != 0 {
        pressed_vks.push(0x11);
    }
    if flags & CG_EVENT_FLAG_MASK_SHIFT != 0 {
        pressed_vks.push(0x10);
    }
    if flags & CG_EVENT_FLAG_MASK_ALTERNATE != 0 {
        pressed_vks.push(0x12);
    }
    if flags & CG_EVENT_FLAG_MASK_COMMAND != 0 {
        pressed_vks.push(0x5B);
    }
    pressed_vks.push(key_vk);

    let registry_state = ctx.app_handle.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read();

    for binding in &registry.bindings {
        if binding.keys.is_empty() {
            continue;
        }
        let all_match = binding.keys.iter().all(|k| pressed_vks.contains(k));
        let no_extra = MODIFIER_KEYS
            .iter()
            .filter(|k| pressed_vks.contains(k))
            .all(|k| binding.keys.contains(k));
        if all_match && no_extra {
            return std::ptr::null_mut(); // Suppress
        }
    }

    event
}

// Wrapper to send *mut c_void across threads safely.
// Safety: the pointer is a heap-allocated TapContext that lives for the process lifetime.
struct SendPtr(*mut c_void);
unsafe impl Send for SendPtr {}

/// Start a CGEventTap that suppresses keyboard events matching registered shortcuts.
/// This runs on a dedicated thread with its own CFRunLoop.
/// If the tap cannot be created (e.g. missing Input Monitoring permission), it is skipped.
fn start_event_suppressor(app: &AppHandle, keycode_map: &HashMap<i32, u16>) {
    // Build reverse map: macOS keycode → VK code
    let mut reverse_map = HashMap::new();
    for (&vk, &keycode) in keycode_map {
        reverse_map.insert(keycode, vk);
    }

    let ctx = Box::new(TapContext {
        reverse_keycode_map: reverse_map,
        app_handle: app.clone(),
    });
    let ctx_ptr = SendPtr(Box::into_raw(ctx) as *mut c_void);

    std::thread::spawn(move || {
        let ctx_ptr = ctx_ptr.0;
        let tap = unsafe {
            CGEventTapCreate(
                K_CG_SESSION_EVENT_TAP,
                K_CG_HEAD_INSERT_EVENT_TAP,
                K_CG_EVENT_TAP_OPTION_DEFAULT,
                K_CG_EVENT_KEY_DOWN | K_CG_EVENT_KEY_UP,
                suppress_callback,
                ctx_ptr,
            )
        };

        if tap.is_null() {
            log::warn!("[macOS shortcuts] Could not create CGEventTap for event suppression (Input Monitoring permission may be required)");
            // Clean up the leaked context
            unsafe {
                let _ = Box::from_raw(ctx_ptr as *mut TapContext);
            }
            return;
        }

        unsafe {
            let source = CFMachPortCreateRunLoopSource(std::ptr::null_mut(), tap, 0);
            if source.is_null() {
                log::warn!("[macOS shortcuts] Could not create run loop source for event tap");
                return;
            }
            let run_loop = CFRunLoopGetCurrent();
            CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
            debug!("[macOS shortcuts] Event suppressor tap started");
            CFRunLoopRun();
        }
    });
}

/// Convert a macOS physical keycode to the logical character using the current keyboard layout.
/// IMPORTANT: This uses Carbon TIS/UCKeyTranslate APIs which are NOT thread-safe.
/// Must be called from the main thread or a thread with a CFRunLoop.
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

/// Build mapping from Windows VK codes to macOS physical keycodes.
/// IMPORTANT: Must be called from the main thread (TIS/UCKeyTranslate are not thread-safe).
/// Layout-dependent keys (letters, digits, OEM) are resolved via UCKeyTranslate
/// so AZERTY/QWERTY is handled correctly.
fn build_vk_to_keycode_map() -> HashMap<i32, u16> {
    let mut map = HashMap::new();

    // Layout-independent keys (fixed physical position)
    map.insert(0x20, 0x31); // Space
    map.insert(0x0D, 0x24); // Return
    map.insert(0x1B, 0x35); // Escape
    map.insert(0x09, 0x30); // Tab
    map.insert(0x08, 0x33); // Backspace
    map.insert(0x2E, 0x75); // Forward Delete
    map.insert(0x2D, 0x72); // Insert (Help on Mac)
    map.insert(0x24, 0x73); // Home
    map.insert(0x23, 0x77); // End
    map.insert(0x21, 0x74); // Page Up
    map.insert(0x22, 0x79); // Page Down
    map.insert(0x26, 0x7E); // Up Arrow
    map.insert(0x28, 0x7D); // Down Arrow
    map.insert(0x25, 0x7B); // Left Arrow
    map.insert(0x27, 0x7C); // Right Arrow
    map.insert(0xC0, 0x32); // BackQuote/Grave
    map.insert(0xE2, 0x0A); // IntlBackslash (ISO keyboards)

    // F-keys
    map.insert(0x70, 0x7A); // F1
    map.insert(0x71, 0x78); // F2
    map.insert(0x72, 0x63); // F3
    map.insert(0x73, 0x76); // F4
    map.insert(0x74, 0x60); // F5
    map.insert(0x75, 0x61); // F6
    map.insert(0x76, 0x62); // F7
    map.insert(0x77, 0x64); // F8
    map.insert(0x78, 0x65); // F9
    map.insert(0x79, 0x6D); // F10
    map.insert(0x7A, 0x67); // F11
    map.insert(0x7B, 0x6F); // F12
    map.insert(0x7C, 0x69); // F13
    map.insert(0x7D, 0x6B); // F14
    map.insert(0x7E, 0x71); // F15
    map.insert(0x7F, 0x6A); // F16
    map.insert(0x80, 0x40); // F17
    map.insert(0x81, 0x4F); // F18
    map.insert(0x82, 0x50); // F19
    map.insert(0x83, 0x5A); // F20

    // Numpad
    map.insert(0x60, 0x52); // Numpad 0
    map.insert(0x61, 0x53); // Numpad 1
    map.insert(0x62, 0x54); // Numpad 2
    map.insert(0x63, 0x55); // Numpad 3
    map.insert(0x64, 0x56); // Numpad 4
    map.insert(0x65, 0x57); // Numpad 5
    map.insert(0x66, 0x58); // Numpad 6
    map.insert(0x67, 0x59); // Numpad 7
    map.insert(0x68, 0x5B); // Numpad 8
    map.insert(0x69, 0x5C); // Numpad 9
    map.insert(0x6A, 0x43); // Numpad Multiply
    map.insert(0x6B, 0x45); // Numpad Plus
    map.insert(0x6D, 0x4E); // Numpad Minus
    map.insert(0x6F, 0x4B); // Numpad Divide

    // Layout-dependent keys: scan all macOS keycodes and use UCKeyTranslate
    // to find the correct physical keycode for each logical character.
    // This handles AZERTY/QWERTY correctly.
    for keycode in 0..128u16 {
        if let Some(c) = keycode_to_char(keycode as u32) {
            if let Some(vk) = char_to_vk(c) {
                map.entry(vk).or_insert(keycode);
            }
        }
    }

    debug!(
        "[macOS shortcuts] Built keycode map with {} entries",
        map.len()
    );
    map
}

fn is_modifier_pressed(vk: i32) -> bool {
    // OR both sources: pressed if either session state or HID state reports it
    let session_flags = unsafe { CGEventSourceFlagsState(kCGEventSourceStateCombinedSessionState) };
    let hid_flags = unsafe { CGEventSourceFlagsState(kCGEventSourceStateHIDSystemState) };
    let flags = session_flags | hid_flags;
    match vk {
        0x11 => flags & CG_EVENT_FLAG_MASK_CONTROL != 0,
        0x10 => flags & CG_EVENT_FLAG_MASK_SHIFT != 0,
        0x12 => flags & CG_EVENT_FLAG_MASK_ALTERNATE != 0,
        0x5B => flags & CG_EVENT_FLAG_MASK_COMMAND != 0,
        _ => false,
    }
}

fn is_key_pressed(vk: i32, keycode_map: &HashMap<i32, u16>) -> bool {
    if MODIFIER_KEYS.contains(&vk) {
        return is_modifier_pressed(vk);
    }
    // Mouse buttons (CGMouseButton: 0=Left, 1=Right, 2=Middle, 3=Back, 4=Forward)
    match vk {
        0x01 => {
            return unsafe {
                CGEventSourceButtonState(kCGEventSourceStateCombinedSessionState, 0)
                    || CGEventSourceButtonState(kCGEventSourceStateHIDSystemState, 0)
            }
        }
        0x02 => {
            return unsafe {
                CGEventSourceButtonState(kCGEventSourceStateCombinedSessionState, 1)
                    || CGEventSourceButtonState(kCGEventSourceStateHIDSystemState, 1)
            }
        }
        0x04 => {
            return unsafe {
                CGEventSourceButtonState(kCGEventSourceStateCombinedSessionState, 2)
                    || CGEventSourceButtonState(kCGEventSourceStateHIDSystemState, 2)
            }
        }
        0x05 => {
            return unsafe {
                CGEventSourceButtonState(kCGEventSourceStateCombinedSessionState, 3)
                    || CGEventSourceButtonState(kCGEventSourceStateHIDSystemState, 3)
            }
        }
        0x06 => {
            return unsafe {
                CGEventSourceButtonState(kCGEventSourceStateCombinedSessionState, 4)
                    || CGEventSourceButtonState(kCGEventSourceStateHIDSystemState, 4)
            }
        }
        _ => {}
    }
    if let Some(&keycode) = keycode_map.get(&vk) {
        // OR both sources: only "released" when both agree
        unsafe {
            CGEventSourceKeyState(kCGEventSourceStateCombinedSessionState, keycode)
                || CGEventSourceKeyState(kCGEventSourceStateHIDSystemState, keycode)
        }
    } else {
        false
    }
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

    // Build keycode map on the main thread — TIS/UCKeyTranslate APIs are NOT thread-safe
    let keycode_map = build_vk_to_keycode_map();

    // Start event suppressor to prevent macOS alert sounds on shortcut keys
    start_event_suppressor(&app, &keycode_map);

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

                let all_pressed = binding
                    .keys
                    .iter()
                    .all(|&k| is_key_pressed(k, &keycode_map));
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
