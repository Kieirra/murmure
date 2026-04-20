use crate::settings;
use crate::settings::PasteMethod;
use enigo::{Key, Keyboard};
use log::debug;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn paste(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 100)
}

pub fn paste_last_transcript(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 400)
}

#[allow(unused_variables)]
fn paste_with_delay(
    text: &str,
    app_handle: &tauri::AppHandle,
    macos_delay_ms: u64,
) -> Result<(), String> {
    let mut app_settings = settings::load_settings(app_handle);

    // Auto-migrate direct → ctrl_v on Wayland: raw uinput cannot map
    // Unicode to layout-aware scancodes. Settings may have persisted
    // `direct` from X11, another OS, or pre-gate builds.
    #[cfg(target_os = "linux")]
    if app_settings.paste_method == PasteMethod::Direct
        && crate::utils::platform::is_wayland_session()
    {
        log::warn!(
            "paste_method=direct is unsupported on Wayland; falling back to clipboard Ctrl+V"
        );
        app_settings.paste_method = PasteMethod::CtrlV;
    }

    // Direct mode: type text character by character without using clipboard
    if app_settings.paste_method == PasteMethod::Direct {
        return paste_direct(text, app_handle);
    }

    let clipboard = app_handle.clipboard();
    let clipboard_content = clipboard.read_text().unwrap_or_default();

    clipboard
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    #[cfg(target_os = "linux")]
    {
        // 150 ms base lets the Wayland clipboard write propagate to
        // other clients. On Wayland + recent overlay destroy we add
        // 400 ms so KWin hands keyboard focus back before Ctrl+V fires
        // — synthetic keys sent before the transition land in the void.
        let mut sleep_ms: u64 = 150;
        if crate::utils::platform::is_wayland_session()
            && crate::overlay::overlay::millis_since_last_overlay_hide() < 2000
        {
            sleep_ms += 400;
        }
        std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
    }
    #[cfg(target_os = "macos")]
    std::thread::sleep(std::time::Duration::from_millis(macos_delay_ms));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(50));

    log::info!(
        "paste_with_delay calling send_paste, method={:?}, text_len={}",
        app_settings.paste_method,
        text.len()
    );
    send_paste(&app_settings.paste_method, app_handle)?;
    log::info!("paste_with_delay send_paste returned Ok");

    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_millis(200));
    #[cfg(target_os = "macos")]
    std::thread::sleep(std::time::Duration::from_millis(200));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(100));

    if !app_settings.copy_to_clipboard {
        clipboard
            .write_text(&clipboard_content)
            .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
    }
    Ok(())
}

#[allow(unused_variables)]
fn paste_direct(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            log::info!(
                "paste_direct: Wayland path (uinput type_text, len={})",
                text.len()
            );
            return crate::utils::wayland_inject::type_text(text);
        }
    }

    log::info!("paste_direct: enigo path (len={})", text.len());
    crate::utils::enigo_session::with_enigo(app_handle, |enigo| {
        enigo
            .text(text)
            .map_err(|e| format!("Failed to type text: {}", e))
    })
}

#[allow(unused_variables)]
fn send_paste(paste_method: &PasteMethod, app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            let shift = *paste_method == PasteMethod::CtrlShiftV;
            log::info!(
                "send_paste: Wayland path (uinput Ctrl+{}V)",
                if shift { "Shift+" } else { "" }
            );
            return crate::utils::wayland_inject::paste(shift);
        }
    }

    log::info!("send_paste: enigo path ({:?})", paste_method);

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

    // Clear clipboard before sending Ctrl+C to detect selection reliably.
    // Without this, if the selected text is identical to the current clipboard
    // content, we cannot distinguish "text was copied" from "nothing was selected".
    clipboard
        .write_text("")
        .map_err(|e| format!("Failed to clear clipboard: {}", e))?;

    send_copy(app_handle)?;
    std::thread::sleep(std::time::Duration::from_millis(200));

    let selected_text = clipboard.read_text().unwrap_or_default();
    debug!("Selected text length: {}", selected_text.len());

    // Restore original clipboard content in all cases
    clipboard
        .write_text(&original_content)
        .map_err(|e| format!("Failed to restore clipboard in get_selected_text: {}", e))?;
    debug!("Restored original clipboard content");

    if !selected_text.is_empty() {
        Ok(selected_text)
    } else {
        // With uinput (Linux Wayland) or XTEST (X11) now reaching the
        // focused window, an empty clipboard after Ctrl+C means the user
        // had no selection. Inject failures are signalled earlier via the
        // `wayland-inject-unavailable` event.
        debug!("No text was selected");
        Ok(String::new())
    }
}

#[allow(unused_variables)]
fn send_copy(app_handle: &tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            log::info!("send_copy: Wayland path (uinput Ctrl+C)");
            return crate::utils::wayland_inject::copy();
        }
    }

    log::info!("send_copy: enigo path");

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
