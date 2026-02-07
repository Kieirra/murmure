//! macOS keyboard shortcut handling using CGEventTap (Quartz)
//!
//! Uses Core Graphics CGEventTap for global keyboard event capture,
//! the same low-level approach used by Discord and OBS Studio.
//! Requires Accessibility permissions on macOS.

use core_foundation::base::CFRelease;
use core_foundation::string::UniChar;
use core_foundation_sys::data::CFDataGetBytePtr;
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use std::collections::HashSet;
use std::ffi::c_void;
use std::os::raw::c_uint;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

// ─── Core Graphics / CoreFoundation FFI ─────────────────────────────────

type CGEventRef = *mut c_void;
type CGEventTapProxy = *mut c_void;
type CFMachPortRef = *mut c_void;
type CGEventMask = u64;
type CGEventType = u32;

// CGEventTapLocation
const K_CG_SESSION_EVENT_TAP: u32 = 1;
// CGEventTapPlacement
const K_CG_HEAD_INSERT_EVENT_TAP: u32 = 0;
// CGEventTapOptions
const K_CG_EVENT_TAP_OPTION_LISTEN_ONLY: u32 = 1;

// CGEventType values
const K_CG_EVENT_KEY_DOWN: CGEventType = 10;
const K_CG_EVENT_KEY_UP: CGEventType = 11;
const K_CG_EVENT_FLAGS_CHANGED: CGEventType = 12;
const K_CG_EVENT_TAP_DISABLED_BY_TIMEOUT: CGEventType = 0xFFFFFFFE;

// CGEventField
const K_CG_KEYBOARD_EVENT_KEYCODE: u32 = 9;

// NX device-specific modifier masks (for accurate left/right press/release tracking)
const NX_DEVICELCTLKEYMASK: u64 = 0x00000001;
const NX_DEVICELSHIFTKEYMASK: u64 = 0x00000002;
const NX_DEVICERSHIFTKEYMASK: u64 = 0x00000004;
const NX_DEVICELCMDKEYMASK: u64 = 0x00000008;
const NX_DEVICERCMDKEYMASK: u64 = 0x00000010;
const NX_DEVICELALTKEYMASK: u64 = 0x00000020;
const NX_DEVICERALTKEYMASK: u64 = 0x00000040;
const NX_DEVICERCTLKEYMASK: u64 = 0x00002000;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: CGEventMask,
        callback: extern "C" fn(
            CGEventTapProxy,
            CGEventType,
            CGEventRef,
            *mut c_void,
        ) -> CGEventRef,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
    fn CGEventGetFlags(event: CGEventRef) -> u64;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFMachPortCreateRunLoopSource(
        allocator: *const c_void,
        port: CFMachPortRef,
        order: i64,
    ) -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *const c_void);
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopRun();
    static kCFRunLoopCommonModes: *const c_void;
}

// ─── Keyboard layout FFI (Carbon/Cocoa) ─────────────────────────────────

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

use crate::shortcuts::accessibility_macos;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState};

// ─── Stored tap port for re-enabling after timeout ──────────────────────

static TAP_PORT: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

// ─── Keyboard layout conversion ────────────────────────────────────────

/// Convert a macOS keycode to the logical character based on current keyboard layout.
/// This handles AZERTY/QWERTY conversion by using UCKeyTranslate with no modifiers.
fn keycode_to_char(keycode: u32) -> Option<char> {
    unsafe {
        // Try different input source methods (same order as rdev)
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

        // Use modifier_state = 0 to get the base character without modifiers
        let _retval = UCKeyTranslate(
            layout_ptr,
            keycode as u16,
            kUCKeyActionDown,
            0, // modifier_state = 0: ignore modifiers to get base character
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

        // Convert UTF-16 to char
        String::from_utf16(&buff[..length])
            .ok()
            .and_then(|s| s.chars().next())
    }
}

// ─── Event processing ──────────────────────────────────────────────────

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

// ─── CGEventTap callback ───────────────────────────────────────────────

extern "C" fn event_tap_callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef {
    // Re-enable tap if it was disabled by timeout
    if event_type == K_CG_EVENT_TAP_DISABLED_BY_TIMEOUT {
        let tap = TAP_PORT.load(Ordering::Relaxed);
        if !tap.is_null() {
            unsafe { CGEventTapEnable(tap, true) };
            debug!("[macOS shortcuts] Re-enabled CGEventTap after timeout");
        }
        return event;
    }

    let tx = unsafe { &*(user_info as *const mpsc::Sender<(i32, bool)>) };
    let keycode =
        unsafe { CGEventGetIntegerValueField(event, K_CG_KEYBOARD_EVENT_KEYCODE) } as u32;

    let result = match event_type {
        K_CG_EVENT_KEY_DOWN => convert_keycode(keycode).map(|vk| (vk, true)),
        K_CG_EVENT_KEY_UP => convert_keycode(keycode).map(|vk| (vk, false)),
        K_CG_EVENT_FLAGS_CHANGED => {
            let flags = unsafe { CGEventGetFlags(event) };
            convert_modifier(keycode, flags)
        }
        _ => None,
    };

    if let Some((vk, is_pressed)) = result {
        let _ = tx.send((vk, is_pressed));
    }

    event
}

// ─── Initialization ────────────────────────────────────────────────────

pub fn init(app: AppHandle) {
    // Check Accessibility permission first
    if !accessibility_macos::check_and_log_permission() {
        warn!("Accessibility permission not granted - emitting event to frontend");
        let _ = app.emit("accessibility-permission-missing", ());
        return;
    }

    // Log registered bindings for debugging
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

    let processor = Arc::new(EventProcessor::new(app.clone()));
    let (tx, rx) = mpsc::channel::<(i32, bool)>();

    // Thread 1: CGEventTap listener on its own CFRunLoop
    std::thread::spawn(move || {
        debug!("[macOS shortcuts] Starting CGEventTap listener...");
        unsafe {
            let tx_ptr = Box::into_raw(Box::new(tx));

            let mask: CGEventMask =
                (1 << K_CG_EVENT_KEY_DOWN) | (1 << K_CG_EVENT_KEY_UP) | (1 << K_CG_EVENT_FLAGS_CHANGED);

            let tap = CGEventTapCreate(
                K_CG_SESSION_EVENT_TAP,
                K_CG_HEAD_INSERT_EVENT_TAP,
                K_CG_EVENT_TAP_OPTION_LISTEN_ONLY,
                mask,
                event_tap_callback,
                tx_ptr as *mut c_void,
            );

            if tap.is_null() {
                error!(
                    "[macOS shortcuts] Failed to create CGEventTap - check Accessibility permissions"
                );
                let _ = Box::from_raw(tx_ptr);
                return;
            }

            // Store tap port so the callback can re-enable it after timeout
            TAP_PORT.store(tap, Ordering::Relaxed);

            let source = CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0);
            if source.is_null() {
                error!("[macOS shortcuts] Failed to create CFRunLoopSource");
                let _ = Box::from_raw(tx_ptr);
                return;
            }

            CFRunLoopAddSource(CFRunLoopGetCurrent(), source, kCFRunLoopCommonModes);
            CGEventTapEnable(tap, true);

            debug!("[macOS shortcuts] CGEventTap active, entering run loop");
            CFRunLoopRun();

            warn!("[macOS shortcuts] CGEventTap run loop has stopped!");
        }
    });

    // Thread 2: Event processor (receives keycodes from the tap callback)
    std::thread::spawn(move || {
        debug!("[macOS shortcuts] Shortcut processor thread started");
        while let Ok((key, is_pressed)) = rx.recv() {
            trace!(
                "[macOS shortcuts] Key event: key=0x{:X}, pressed={}",
                key, is_pressed
            );
            if is_pressed {
                processor.handle_key_press(key);
            } else {
                processor.handle_key_release(key);
            }
        }
        warn!("[macOS shortcuts] Shortcut processor has stopped!");
    });

    debug!("[macOS shortcuts] Initialization complete");
}

// ─── Keycode conversion ────────────────────────────────────────────────

/// Convert a macOS hardware keycode to VK code.
/// First tries layout-aware conversion (handles AZERTY/QWERTY), then falls back to direct mapping.
fn convert_keycode(keycode: u32) -> Option<i32> {
    // Layout-aware: converts keycode to the logical character for the active keyboard layout
    if let Some(c) = keycode_to_char(keycode) {
        if let Some(vk) = char_to_vk(c) {
            return Some(vk);
        }
    }
    // Fallback: direct hardware keycode mapping (function keys, arrows, special keys)
    macos_keycode_to_vk(keycode)
}

/// Convert a modifier key event (kCGEventFlagsChanged) to (VK code, is_pressed).
/// Uses NX device-specific flags to correctly distinguish left/right modifier press/release.
fn convert_modifier(keycode: u32, flags: u64) -> Option<(i32, bool)> {
    match keycode {
        0x38 => Some((0x10, flags & NX_DEVICELSHIFTKEYMASK != 0)), // Left Shift
        0x3C => Some((0x10, flags & NX_DEVICERSHIFTKEYMASK != 0)), // Right Shift
        0x3B => Some((0x11, flags & NX_DEVICELCTLKEYMASK != 0)),   // Left Control
        0x3E => Some((0x11, flags & NX_DEVICERCTLKEYMASK != 0)),   // Right Control
        0x3A => Some((0x12, flags & NX_DEVICELALTKEYMASK != 0)),   // Left Option
        0x3D => Some((0x12, flags & NX_DEVICERALTKEYMASK != 0)),   // Right Option
        0x37 => Some((0x5B, flags & NX_DEVICELCMDKEYMASK != 0)),   // Left Command
        0x36 => Some((0x5B, flags & NX_DEVICERCMDKEYMASK != 0)),   // Right Command
        _ => None,
    }
}

/// Convert a unicode character to VK code.
/// This handles keyboard layout properly (e.g., AZERTY vs QWERTY).
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

/// Map macOS hardware keycodes to VK codes for non-character keys.
/// Character keys are handled by keycode_to_char() + char_to_vk() above.
fn macos_keycode_to_vk(keycode: u32) -> Option<i32> {
    match keycode {
        // Modifiers
        0x37 | 0x36 => Some(0x5B), // Command (Left/Right)
        0x3B | 0x3E => Some(0x11), // Control (Left/Right)
        0x3A | 0x3D => Some(0x12), // Option/Alt (Left/Right)
        0x38 | 0x3C => Some(0x10), // Shift (Left/Right)

        // Function keys
        0x7A => Some(0x70), // F1
        0x78 => Some(0x71), // F2
        0x63 => Some(0x72), // F3
        0x76 => Some(0x73), // F4
        0x60 => Some(0x74), // F5
        0x61 => Some(0x75), // F6
        0x62 => Some(0x76), // F7
        0x64 => Some(0x77), // F8
        0x65 => Some(0x78), // F9
        0x6D => Some(0x79), // F10
        0x67 => Some(0x7A), // F11
        0x6F => Some(0x7B), // F12

        // Special keys
        0x24 => Some(0x0D), // Return
        0x30 => Some(0x09), // Tab
        0x31 => Some(0x20), // Space
        0x33 => Some(0x08), // Delete (Backspace)
        0x35 => Some(0x1B), // Escape
        0x75 => Some(0x2E), // Forward Delete
        0x72 => Some(0x2D), // Insert (Help key on Mac)
        0x73 => Some(0x24), // Home
        0x77 => Some(0x23), // End
        0x74 => Some(0x21), // Page Up
        0x79 => Some(0x22), // Page Down

        // Arrow keys
        0x7E => Some(0x26), // Up
        0x7D => Some(0x28), // Down
        0x7B => Some(0x25), // Left
        0x7C => Some(0x27), // Right

        _ => None,
    }
}
