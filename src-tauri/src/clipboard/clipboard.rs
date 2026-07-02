use crate::settings;
use crate::settings::PasteMethod;
use enigo::{Key, Keyboard};
#[cfg(target_os = "linux")]
use log::warn;
use log::{debug, info};
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};
#[cfg(target_os = "linux")]
use std::sync::OnceLock;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn paste(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 100)
}

pub fn paste_last_transcript(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 400)
}

pub fn copy_to_clipboard(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    write_clipboard(text, app_handle)
}

#[allow(unused_variables)]
fn paste_with_delay(
    text: &str,
    app_handle: &tauri::AppHandle,
    macos_delay_ms: u64,
) -> Result<(), String> {
    let app_settings = settings::load_settings(app_handle);

    if app_settings.paste_method == PasteMethod::None {
        if app_settings.copy_to_clipboard {
            write_clipboard(text, app_handle)?;
        }
        return Ok(());
    }

    if app_settings.paste_method == PasteMethod::Direct {
        return paste_direct(text, app_handle);
    }

    let clipboard_content = app_handle.clipboard().read_text().unwrap_or_default();

    write_clipboard(text, app_handle)?;

    #[cfg(target_os = "linux")]
    {
        let sleep_ms = if crate::utils::platform::is_wayland_session() {
            wayland_post_clipboard_delay_ms()
        } else {
            150
        };
        std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
    }
    #[cfg(target_os = "macos")]
    std::thread::sleep(std::time::Duration::from_millis(macos_delay_ms));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(50));

    log::debug!(
        "paste_with_delay calling send_paste, method={:?}, text_len={}",
        app_settings.paste_method,
        text.len()
    );
    let shortcut_state = app_handle.state::<crate::shortcuts::types::ShortcutState>();
    shortcut_state.set_suspended(true);
    let paste_result = send_paste(&app_settings.paste_method, app_handle);
    log::debug!("paste_with_delay send_paste returned Ok");

    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_millis(200));
    #[cfg(target_os = "macos")]
    std::thread::sleep(std::time::Duration::from_millis(200));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(100));

    shortcut_state.set_suspended(false);
    paste_result?;

    if !app_settings.copy_to_clipboard {
        write_clipboard(&clipboard_content, app_handle)
            .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
    }
    Ok(())
}

fn write_clipboard(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            if is_wl_copy_available() {
                match write_clipboard_via_wl_copy(text) {
                    Ok(()) => {
                        info!("Clipboard written via wl-copy ({} bytes)", text.len());
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("wl-copy failed: {}, falling back to Tauri clipboard", e);
                    }
                }
            } else {
                warn_wl_copy_missing_once();
            }
        }
    }

    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))
}

#[cfg(target_os = "linux")]
fn is_wl_copy_available() -> bool {
    static CACHE: OnceLock<bool> = OnceLock::new();
    *CACHE.get_or_init(|| {
        Command::new("which")
            .arg("wl-copy")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

#[cfg(target_os = "linux")]
fn warn_wl_copy_missing_once() {
    static LOGGED: OnceLock<()> = OnceLock::new();
    LOGGED.get_or_init(|| {
        warn!(
            "wl-copy not found, falling back to Tauri clipboard (paste may fail under Wayland; \
             install the `wl-clipboard` package to fix it)"
        );
    });
}

// stdin (not argv) keeps the payload off ARG_MAX limits and out of `ps`.
// Stdio::null on stdout/stderr is required: wl-copy forks a persistent
// daemon that inherits the parent fds; piping them blocks wait() forever.
#[cfg(target_os = "linux")]
fn write_clipboard_via_wl_copy(text: &str) -> Result<(), String> {
    use std::io::Write;

    let mut child = Command::new("wl-copy")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn wl-copy: {}", e))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "wl-copy stdin unavailable".to_string())?;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| format!("Failed to write to wl-copy stdin: {}", e))?;
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for wl-copy: {}", e))?;

    if !status.success() {
        return Err(format!("wl-copy exited with status {:?}", status.code()));
    }

    Ok(())
}

#[allow(unused_variables)]
fn paste_direct(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            return paste_direct_wayland(text, app_handle);
        }
    }

    log::debug!("paste_direct: enigo path (len={})", text.len());
    crate::utils::enigo_session::with_enigo(app_handle, |enigo| {
        enigo
            .text(text)
            .map_err(|e| format!("Failed to type text: {}", e))
    })
}

// Normalise to ASCII so the keymap covers accented text. Fallback
// uses the ORIGINAL text to preserve accents if normalisation misses.
#[cfg(target_os = "linux")]
fn paste_direct_wayland(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    let normalized = crate::utils::wayland_xkb::normalize_for_direct_typing(text);
    if normalized.len() != text.len() {
        log::debug!(
            "paste_direct: normalized text from {} bytes to {} bytes",
            text.len(),
            normalized.len()
        );
    }
    log::debug!(
        "paste_direct: wayland type_text path (len={})",
        normalized.len()
    );

    match crate::utils::wayland_inject::type_text(&normalized) {
        Ok(()) => Ok(()),
        Err(e) => {
            warn!("paste_direct: {}, falling back to clipboard+Ctrl+V", e);
            wayland_fallback_clipboard_ctrlv(text, app_handle)
        }
    }
}

// Cannot delegate to `paste_with_delay`: that one re-routes to
// `paste_direct` when settings say Direct, which is the caller we are
// returning to.
#[cfg(target_os = "linux")]
fn wayland_fallback_clipboard_ctrlv(
    text: &str,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    let original = app_handle.clipboard().read_text().unwrap_or_default();
    write_clipboard(text, app_handle)?;

    std::thread::sleep(std::time::Duration::from_millis(
        wayland_post_clipboard_delay_ms(),
    ));

    crate::utils::wayland_inject::paste(false)?;
    std::thread::sleep(std::time::Duration::from_millis(200));

    let app_settings = settings::load_settings(app_handle);
    if !app_settings.copy_to_clipboard {
        write_clipboard(&original, app_handle)
            .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
    }
    Ok(())
}

// 150 ms lets the Wayland clipboard write propagate. Add 400 ms after a
// recent overlay destroy so KWin restores keyboard focus before Ctrl+V
// fires; otherwise synthetic keys land in the void.
#[cfg(target_os = "linux")]
fn wayland_post_clipboard_delay_ms() -> u64 {
    let mut sleep_ms: u64 = 150;
    if crate::overlay::overlay::millis_since_last_overlay_hide() < 2000 {
        sleep_ms += 400;
    }
    sleep_ms
}

#[allow(unused_variables)]
fn send_paste(paste_method: &PasteMethod, app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            let shift = *paste_method == PasteMethod::CtrlShiftV;
            log::debug!(
                "send_paste: Wayland path (uinput Ctrl+{}V)",
                if shift { "Shift+" } else { "" }
            );
            return crate::utils::wayland_inject::paste(shift);
        }
    }

    log::debug!("send_paste: enigo path ({:?})", paste_method);

    #[cfg(target_os = "macos")]
    let (modifier_key, key_code) = (Key::Meta, Key::Other(9));
    #[cfg(target_os = "windows")]
    let (modifier_key, key_code) = (Key::Control, Key::Other(0x56));
    #[cfg(target_os = "linux")]
    let (modifier_key, key_code) = (Key::Control, Key::Unicode('v'));

    crate::utils::enigo_session::with_enigo(app_handle, |enigo| {
        enigo
            .key(modifier_key, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press modifier key: {}", e))?;

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        if *paste_method == PasteMethod::CtrlShiftV {
            enigo
                .key(Key::Shift, enigo::Direction::Press)
                .map_err(|e| format!("Failed to press Shift key: {}", e))?;
        }

        enigo
            .key(key_code, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press V key: {}", e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        enigo
            .key(key_code, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release V key: {}", e))?;

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        if *paste_method == PasteMethod::CtrlShiftV {
            enigo
                .key(Key::Shift, enigo::Direction::Release)
                .map_err(|e| format!("Failed to release Shift key: {}", e))?;
        }

        enigo
            .key(modifier_key, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release modifier key: {}", e))?;

        Ok(())
    })
}

pub fn get_selected_text(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let clipboard = app_handle.clipboard();
    let original_content = clipboard.read_text().unwrap_or_default();
    debug!(
        "Previous clipboard content length: {}",
        original_content.len()
    );

    // Clear first: without this, an unchanged clipboard after Ctrl+C is
    // indistinguishable from "nothing was selected".
    clipboard
        .write_text("")
        .map_err(|e| format!("Failed to clear clipboard: {}", e))?;

    send_copy(app_handle)?;
    std::thread::sleep(std::time::Duration::from_millis(200));

    let selected_text = clipboard.read_text().unwrap_or_default();
    debug!("Selected text length: {}", selected_text.len());

    clipboard
        .write_text(&original_content)
        .map_err(|e| format!("Failed to restore clipboard in get_selected_text: {}", e))?;
    debug!("Restored original clipboard content");

    if !selected_text.is_empty() {
        Ok(selected_text)
    } else {
        // Inject failures are signalled separately via the
        // `wayland-inject-unavailable` event, so an empty clipboard
        // here means the user had no selection.
        debug!("No text was selected");
        Ok(String::new())
    }
}

#[allow(unused_variables)]
fn send_copy(app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            log::debug!("send_copy: Wayland path (uinput Ctrl+C)");
            return crate::utils::wayland_inject::copy();
        }
    }

    log::debug!("send_copy: enigo path");

    #[cfg(target_os = "macos")]
    let (modifier_key, key_code) = (Key::Meta, Key::Other(8)); // 0x08 is C
    #[cfg(target_os = "windows")]
    let (modifier_key, key_code) = (Key::Control, Key::Other(0x43)); // 0x43 is C
    #[cfg(target_os = "linux")]
    let (modifier_key, key_code) = (Key::Control, Key::Unicode('c'));

    crate::utils::enigo_session::with_enigo(app_handle, |enigo| {
        enigo
            .key(modifier_key, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press modifier key: {}", e))?;

        enigo
            .key(key_code, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press C key: {}", e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        enigo
            .key(key_code, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release C key: {}", e))?;

        enigo
            .key(modifier_key, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release modifier key: {}", e))?;

        Ok(())
    })
}
