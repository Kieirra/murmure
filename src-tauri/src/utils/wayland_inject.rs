//! Bypass Wayland's cross-client injection ban via a long-lived
//! /dev/uinput virtual keyboard. Needs the uinput kernel module and
//! write access via packaging/linux/60-murmure-uinput.rules.

use crate::utils::wayland_xkb::types::{CharStrokes, KeyMapping};
use input_linux::sys::{input_event, timeval, EV_MSC, MSC_SCAN};
use input_linux::{
    EventKind, EventTime, InputId, Key, KeyEvent, KeyState, LedKind, MiscKind, RelativeAxis,
    SynchronizeEvent, UInputHandle,
};
use log::{debug, info, warn};
use std::fs::{File, OpenOptions};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEVICE_NAME: &[u8] = b"Murmure virtual keyboard";
const DEVICE_BUS: u16 = 0x03;
const DEVICE_VENDOR: u16 = 0x2333;
const DEVICE_PRODUCT: u16 = 0x6666;
const DEVICE_VERSION: u16 = 0x5b25;

// Wait for the kernel to enumerate the device after UI_DEV_CREATE.
const ENUMERATION_DELAY: Duration = Duration::from_millis(500);

const INTER_KEY_DELAY: Duration = Duration::from_millis(16);

// Below this hold, repeated keys (notably space and doubled letters)
// are intermittently dropped on some Wayland compositors.
const CHORD_HOLD_DELAY: Duration = Duration::from_millis(16);

// Space needs a longer hold than CHORD_HOLD_DELAY to land reliably
// when typed frequently across word boundaries.
const SPACE_HOLD_DELAY: Duration = Duration::from_millis(24);

static DEVICE: OnceLock<Mutex<Option<UInputHandle<File>>>> = OnceLock::new();

fn now_timeval() -> timeval {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    timeval {
        tv_sec: since_epoch.as_secs() as i64,
        tv_usec: since_epoch.subsec_micros() as i64,
    }
}

// Scancode mirrors a real USB keyboard so apps relying on `MSC_SCAN`
// for remap logic keep working. The kernel itself does not require it.
fn key_frame(key: Key, pressed: bool) -> [input_event; 3] {
    let tv = now_timeval();
    let time = EventTime::from_timeval(tv);
    let scan = input_event {
        time: tv,
        type_: EV_MSC as u16,
        code: MSC_SCAN as u16,
        value: 0x70000 | (key as u16 as i32),
    };
    let ev = KeyEvent::new(
        time,
        key,
        if pressed {
            KeyState::PRESSED
        } else {
            KeyState::RELEASED
        },
    );
    let sync = SynchronizeEvent::report(time);
    [scan, *ev.as_ref(), *sync.as_ref()]
}

fn build_device() -> std::io::Result<UInputHandle<File>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/uinput")?;
    let handle = UInputHandle::new(file);

    handle.set_evbit(EventKind::Key)?;
    handle.set_evbit(EventKind::Synchronize)?;
    handle.set_evbit(EventKind::Relative)?;
    for axis in [
        RelativeAxis::X,
        RelativeAxis::Y,
        RelativeAxis::Wheel,
        RelativeAxis::HorizontalWheel,
    ] {
        handle.set_relbit(axis)?;
    }
    handle.set_evbit(EventKind::Misc)?;
    handle.set_mscbit(MiscKind::Scancode)?;
    handle.set_evbit(EventKind::Led)?;
    for led in [LedKind::NumLock, LedKind::CapsLock, LedKind::ScrollLock] {
        handle.set_ledbit(led)?;
    }
    for code in 1u16..=0x2ff {
        if let Ok(key) = Key::from_code(code) {
            let _ = handle.set_keybit(key);
        }
    }

    let id = InputId {
        bustype: DEVICE_BUS,
        vendor: DEVICE_VENDOR,
        product: DEVICE_PRODUCT,
        version: DEVICE_VERSION,
    };
    handle.create(&id, DEVICE_NAME, 0, &[])?;

    std::thread::sleep(ENUMERATION_DELAY);

    Ok(handle)
}

pub fn init() -> Result<(), String> {
    let cell = DEVICE.get_or_init(|| Mutex::new(None));
    let mut guard = cell
        .lock()
        .map_err(|e| format!("uinput mutex poisoned: {}", e))?;

    if guard.is_some() {
        info!("wayland_inject: already initialised");
        return Ok(());
    }

    let handle = build_device().map_err(|e| {
        format!(
            "failed to open /dev/uinput: {}, ensure the user is in the `input` group \
             or the `TAG+=\"uaccess\"` udev rule is installed",
            e
        )
    })?;

    info!("wayland_inject: /dev/uinput virtual keyboard ready");
    *guard = Some(handle);
    Ok(())
}

fn with_device<F>(op: &str, f: F) -> Result<(), String>
where
    F: FnOnce(&UInputHandle<File>) -> std::io::Result<()>,
{
    let cell = DEVICE.get().ok_or_else(|| {
        "wayland_inject::init() was never called, device not available".to_string()
    })?;
    let guard = cell
        .lock()
        .map_err(|e| format!("uinput mutex poisoned: {}", e))?;
    let handle = guard
        .as_ref()
        .ok_or_else(|| "wayland_inject: no uinput device (init failed)".to_string())?;

    if let Err(e) = f(handle) {
        warn!("wayland_inject::{} failed: {}", op, e);
        return Err(format!("{}: {}", op, e));
    }
    Ok(())
}

// Holds the DEVICE mutex during ~80 ms of sleeps. Safe because every
// paste site runs on the single audio thread; revisit if paste ever
// becomes concurrent.
fn press_chord(handle: &UInputHandle<File>, keys: &[Key]) -> std::io::Result<()> {
    for key in keys {
        handle.write(&key_frame(*key, true))?;
        std::thread::sleep(INTER_KEY_DELAY);
    }
    std::thread::sleep(CHORD_HOLD_DELAY);
    for key in keys.iter().rev() {
        handle.write(&key_frame(*key, false))?;
        std::thread::sleep(INTER_KEY_DELAY);
    }
    Ok(())
}

pub fn paste(shift: bool) -> Result<(), String> {
    let keys: &[Key] = if shift {
        &[Key::LeftCtrl, Key::LeftShift, Key::V]
    } else {
        &[Key::LeftCtrl, Key::V]
    };
    with_device("paste", |h| press_chord(h, keys))
}

pub fn copy() -> Result<(), String> {
    with_device("copy", |h| press_chord(h, &[Key::LeftCtrl, Key::C]))
}

pub fn enter() -> Result<(), String> {
    with_device("enter", |h| press_chord(h, &[Key::Enter]))
}

#[derive(Debug)]
pub enum TypeTextError {
    DeviceUnavailable,
    IoError(String),
}

impl std::fmt::Display for TypeTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeTextError::DeviceUnavailable => write!(f, "uinput device unavailable"),
            TypeTextError::IoError(msg) => write!(f, "uinput IO error: {}", msg),
        }
    }
}

// A single write plus the pause that has to follow it.
type KeyStep = (Key, bool, Duration);

// Chars the layout cannot produce, and mappings whose keycode is not a
// valid evdev key, are skipped instead of aborting the whole text:
// Direct mode has no clipboard fallback to catch an error.
pub fn type_text(text: &str) -> Result<(), TypeTextError> {
    if text.is_empty() {
        return Ok(());
    }

    let cell = DEVICE.get().ok_or(TypeTextError::DeviceUnavailable)?;
    let guard = cell.lock().map_err(|_| TypeTextError::DeviceUnavailable)?;
    let handle = guard.as_ref().ok_or(TypeTextError::DeviceUnavailable)?;

    let mut skipped = 0usize;
    for c in text.chars() {
        match crate::utils::wayland_xkb::lookup(c).and_then(stroke_steps) {
            Some(steps) => {
                if let Err(e) = write_steps(handle, &steps) {
                    release_modifiers(handle);
                    return Err(e);
                }
            }
            None => skipped += 1,
        }
    }
    if skipped > 0 {
        debug!(
            "wayland_inject: {} char(s) skipped (not typable in the active layout or invalid evdev keycode)",
            skipped
        );
    }
    Ok(())
}

// A write dying mid-sequence would otherwise leave Shift or AltGr held
// down on the virtual device, shifting everything the user types next.
fn release_modifiers(handle: &UInputHandle<File>) {
    for key in [Key::LeftShift, Key::RightAlt] {
        let _ = handle.write(&key_frame(key, false));
    }
}

fn stroke_steps(strokes: CharStrokes) -> Option<Vec<KeyStep>> {
    match strokes {
        CharStrokes::Direct(mapping) => key_steps(mapping),
        CharStrokes::DeadKey { dead, base } => {
            let mut steps = key_steps(dead)?;
            steps.extend(key_steps(base)?);
            Some(steps)
        }
    }
}

// `None` when the keycode is outside the evdev key range, the caller
// then skips the char.
fn key_steps(mapping: KeyMapping) -> Option<Vec<KeyStep>> {
    let key = Key::from_code(mapping.evdev_keycode).ok()?;
    let hold = if key == Key::Space {
        SPACE_HOLD_DELAY
    } else {
        CHORD_HOLD_DELAY
    };

    let mut steps: Vec<KeyStep> = Vec::with_capacity(6);
    if mapping.needs_shift {
        steps.push((Key::LeftShift, true, INTER_KEY_DELAY));
    }
    if mapping.needs_altgr {
        steps.push((Key::RightAlt, true, INTER_KEY_DELAY));
    }
    steps.push((key, true, hold));
    steps.push((key, false, INTER_KEY_DELAY));
    if mapping.needs_altgr {
        steps.push((Key::RightAlt, false, INTER_KEY_DELAY));
    }
    if mapping.needs_shift {
        steps.push((Key::LeftShift, false, INTER_KEY_DELAY));
    }
    Some(steps)
}

fn write_steps(handle: &UInputHandle<File>, steps: &[KeyStep]) -> Result<(), TypeTextError> {
    for &(key, pressed, delay) in steps {
        handle.write(&key_frame(key, pressed)).map_err(io_err)?;
        std::thread::sleep(delay);
    }
    Ok(())
}

fn io_err(e: std::io::Error) -> TypeTextError {
    TypeTextError::IoError(e.to_string())
}

// Called from Tauri's `RunEvent::Exit` so `UI_DEV_DESTROY` fires
// promptly instead of waiting for the OS to reap the fds. Idempotent.
pub fn shutdown() {
    let Some(cell) = DEVICE.get() else {
        return;
    };
    let Ok(mut guard) = cell.lock() else {
        return;
    };
    if let Some(handle) = guard.take() {
        if let Err(e) = handle.dev_destroy() {
            warn!("wayland_inject shutdown: dev_destroy failed: {}", e);
        } else {
            info!("wayland_inject: virtual keyboard destroyed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_frame_carries_scancode_key_and_sync() {
        let frame = key_frame(Key::V, true);
        assert_eq!(frame.len(), 3);

        // [0] MSC_SCAN scancode, value = 0x70000 | keycode
        assert_eq!(frame[0].type_, EV_MSC as u16);
        assert_eq!(frame[0].code, MSC_SCAN as u16);
        assert_eq!(frame[0].value, 0x70000 | Key::V as u16 as i32);

        // [1] EV_KEY press for V. `input_linux::KeyEvent` serialises
        // as type=EV_KEY, code=<keycode>, value=1 for pressed.
        assert_eq!(frame[1].type_, EventKind::Key as u16);
        assert_eq!(frame[1].code, Key::V as u16);
        assert_eq!(frame[1].value, 1);

        // [2] SYN_REPORT terminator so the kernel flushes the batch.
        assert_eq!(frame[2].type_, EventKind::Synchronize as u16);
    }

    #[test]
    fn key_frame_release_has_value_zero() {
        let frame = key_frame(Key::V, false);
        assert_eq!(frame[1].value, 0);
    }

    fn mapping(evdev_keycode: u16, needs_shift: bool, needs_altgr: bool) -> KeyMapping {
        KeyMapping {
            evdev_keycode,
            needs_shift,
            needs_altgr,
        }
    }

    fn keys(steps: &[KeyStep]) -> Vec<(Key, bool)> {
        steps.iter().map(|&(k, pressed, _)| (k, pressed)).collect()
    }

    #[test]
    fn plain_char_presses_and_releases_a_single_key() {
        let steps = key_steps(mapping(Key::A as u16, false, false)).unwrap();
        assert_eq!(keys(&steps), vec![(Key::A, true), (Key::A, false)]);
    }

    #[test]
    fn shift_wraps_the_key_press() {
        let steps = key_steps(mapping(Key::A as u16, true, false)).unwrap();
        assert_eq!(
            keys(&steps),
            vec![
                (Key::LeftShift, true),
                (Key::A, true),
                (Key::A, false),
                (Key::LeftShift, false),
            ]
        );
    }

    #[test]
    fn altgr_chord_wraps_the_key_press_with_right_alt() {
        let steps = key_steps(mapping(Key::E as u16, false, true)).unwrap();
        assert_eq!(
            keys(&steps),
            vec![
                (Key::RightAlt, true),
                (Key::E, true),
                (Key::E, false),
                (Key::RightAlt, false),
            ]
        );
    }

    #[test]
    fn shift_altgr_chord_releases_modifiers_in_reverse_order() {
        let steps = key_steps(mapping(Key::E as u16, true, true)).unwrap();
        assert_eq!(
            keys(&steps),
            vec![
                (Key::LeftShift, true),
                (Key::RightAlt, true),
                (Key::E, true),
                (Key::E, false),
                (Key::RightAlt, false),
                (Key::LeftShift, false),
            ]
        );
    }

    #[test]
    fn dead_key_sequence_is_two_key_presses() {
        let strokes = CharStrokes::DeadKey {
            dead: mapping(Key::LeftBrace as u16, false, false),
            base: mapping(Key::E as u16, false, false),
        };
        let steps = stroke_steps(strokes).unwrap();
        assert_eq!(
            keys(&steps),
            vec![
                (Key::LeftBrace, true),
                (Key::LeftBrace, false),
                (Key::E, true),
                (Key::E, false),
            ]
        );
    }

    #[test]
    fn dead_key_sequence_keeps_each_half_own_modifiers() {
        let strokes = CharStrokes::DeadKey {
            dead: mapping(Key::LeftBrace as u16, false, true),
            base: mapping(Key::E as u16, true, false),
        };
        let steps = stroke_steps(strokes).unwrap();
        assert_eq!(
            keys(&steps),
            vec![
                (Key::RightAlt, true),
                (Key::LeftBrace, true),
                (Key::LeftBrace, false),
                (Key::RightAlt, false),
                (Key::LeftShift, true),
                (Key::E, true),
                (Key::E, false),
                (Key::LeftShift, false),
            ]
        );
    }

    #[test]
    fn space_gets_the_longer_hold() {
        let steps = key_steps(mapping(Key::Space as u16, false, false)).unwrap();
        assert_eq!(steps[0].2, SPACE_HOLD_DELAY);
        let steps = key_steps(mapping(Key::A as u16, false, false)).unwrap();
        assert_eq!(steps[0].2, CHORD_HOLD_DELAY);
    }

    #[test]
    fn invalid_keycode_yields_no_steps_so_the_char_is_skipped() {
        assert!(key_steps(mapping(u16::MAX, false, false)).is_none());
        assert!(stroke_steps(CharStrokes::Direct(mapping(u16::MAX, false, false))).is_none());
    }

    #[test]
    fn dead_key_sequence_is_skipped_whole_when_one_half_is_invalid() {
        let strokes = CharStrokes::DeadKey {
            dead: mapping(Key::LeftBrace as u16, false, false),
            base: mapping(u16::MAX, false, false),
        };
        assert!(stroke_steps(strokes).is_none());
    }
}
