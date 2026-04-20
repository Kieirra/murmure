//! Wayland keystroke injection via a single long-lived `/dev/uinput`
//! virtual keyboard.
//!
//! Wayland forbids cross-client input injection by design; we bypass
//! it at the kernel level. The virtual keyboard is indistinguishable
//! from a USB keyboard to KWin / Mutter / wlroots, so the synthetic
//! events route to the focused window normally.
//!
//! Requires:
//!   * kernel `uinput` module (default on every modern distro),
//!   * write access to `/dev/uinput`, granted to the active GUI user
//!     by `packaging/linux/60-murmure-uinput.rules` (`TAG+="uaccess"`).
//!
//! Does not work inside a Flatpak / Snap sandbox without extra portal
//! permissions.

use input_linux::sys::{input_event, timeval, EV_MSC, MSC_SCAN};
use input_linux::{
    EventKind, EventTime, InputId, Key, KeyEvent, KeyState, LedKind, MiscKind, RelativeAxis,
    SynchronizeEvent, UInputHandle,
};
use log::{info, warn};
use std::fs::{File, OpenOptions};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEVICE_NAME: &[u8] = b"Murmure virtual keyboard";
const DEVICE_BUS: u16 = 0x03;
const DEVICE_VENDOR: u16 = 0x2333;
const DEVICE_PRODUCT: u16 = 0x6666;
const DEVICE_VERSION: u16 = 0x5b25;

/// Compositor device-enumeration delay after `UI_DEV_CREATE`; matches ydotoold.
const ENUMERATION_DELAY: Duration = Duration::from_millis(500);

/// Inter-key gap so Electron / Chromium don't miss the modifier state.
const INTER_KEY_DELAY: Duration = Duration::from_millis(12);

/// Hold-down window so apps with keypress deduplication register it.
const CHORD_HOLD_DELAY: Duration = Duration::from_millis(30);

/// Single shared virtual keyboard. Created once in `init()`, reused
/// for every `paste` / `copy` / `enter` call, and dropped when the
/// process exits (the `Drop` on `File` destroys the uinput device via
/// `close(2)`, which the kernel maps to `UI_DEV_DESTROY`).
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

/// `[EV_MSC scancode, EV_KEY, EV_SYN]`. The scancode mirrors a real
/// USB keyboard so apps that use `MSC_SCAN` for remap logic don't
/// break — the kernel itself doesn't require it.
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

/// Open `/dev/uinput` and create the virtual keyboard. Called
/// synchronously from Tauri's setup — the 500 ms enumeration delay
/// is hidden behind model preload so the first paste never races init.
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
            "failed to open /dev/uinput: {} — ensure the user is in the `input` group \
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
        "wayland_inject::init() was never called — device not available".to_string()
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

/// Press all keys in order, hold briefly, release in reverse. Holds
/// the DEVICE mutex during ~80 ms of sleeps — fine because every
/// paste site runs on the single audio thread. Reconsider if a UI
/// command ever calls paste concurrently.
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

/// Press Ctrl+V (or Ctrl+Shift+V with `shift = true`) in the focused window.
pub fn paste(shift: bool) -> Result<(), String> {
    let keys: &[Key] = if shift {
        &[Key::LeftCtrl, Key::LeftShift, Key::V]
    } else {
        &[Key::LeftCtrl, Key::V]
    };
    with_device("paste", |h| press_chord(h, keys))
}

/// Press Ctrl+C (used by `get_selected_text`).
pub fn copy() -> Result<(), String> {
    with_device("copy", |h| press_chord(h, &[Key::LeftCtrl, Key::C]))
}

/// Press Enter (wake-word Submit flow).
pub fn enter() -> Result<(), String> {
    with_device("enter", |h| press_chord(h, &[Key::Enter]))
}

/// Direct Unicode typing is not supported via raw uinput because
/// mapping every code point to an evdev scancode requires the target
/// keyboard layout (AZERTY types `q` where QWERTY types `a`, combining
/// accents need dead-key state, …). Callers must use the clipboard
/// path (`paste_method = "ctrl_v"`) instead.
pub fn type_text(_text: &str) -> Result<(), String> {
    Err("Direct typing is not supported on Wayland with uinput; \
         use clipboard paste (paste_method=\"ctrl_v\") instead."
        .to_string())
}

/// Explicitly destroy the virtual keyboard. Called from Tauri's
/// `RunEvent::Exit` so the kernel's `UI_DEV_DESTROY` fires promptly
/// instead of waiting for the OS to reap the process's file
/// descriptors. Safe to call any number of times (including when
/// `init()` failed) — subsequent calls find `None` and return.
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

    /// Regression guard: the documented chord key order. If someone
    /// later reorders `paste` to release Ctrl before V they'd leave
    /// apps with a latched modifier.
    #[test]
    fn paste_key_order() {
        // Plain Ctrl+V: Ctrl down, V down, V up, Ctrl up.
        let plain: &[Key] = &[Key::LeftCtrl, Key::V];
        assert_eq!(plain.first(), Some(&Key::LeftCtrl));
        assert_eq!(plain.last(), Some(&Key::V));

        // Ctrl+Shift+V: Ctrl, Shift, V — released V, Shift, Ctrl.
        let shifted: &[Key] = &[Key::LeftCtrl, Key::LeftShift, Key::V];
        assert_eq!(shifted[0], Key::LeftCtrl);
        assert_eq!(shifted[1], Key::LeftShift);
        assert_eq!(shifted[2], Key::V);
    }

    #[test]
    fn type_text_is_unsupported() {
        // Callers rely on the error to fall back to clipboard paste.
        assert!(type_text("hello").is_err());
    }
}
