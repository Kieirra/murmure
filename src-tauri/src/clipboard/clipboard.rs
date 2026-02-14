use crate::settings;
use crate::settings::PasteMethod;
use enigo::{Enigo, Key, Keyboard, Settings};
use log::debug;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn paste(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 100, true)
}

pub fn paste_last_transcript(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    paste_with_delay(text, app_handle, 400, false)
}

#[allow(unused_variables)]
fn paste_with_delay(
    text: &str,
    app_handle: &tauri::AppHandle,
    macos_delay_ms: u64,
    send_enter: bool,
) -> Result<(), String> {
    let app_settings = settings::load_settings(app_handle);

    // Direct mode: type text character by character without using clipboard
    if app_settings.paste_method == PasteMethod::Direct {
        return paste_direct(text);
    }

    let clipboard = app_handle.clipboard();
    let clipboard_content = clipboard.read_text().unwrap_or_default();

    clipboard
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_millis(150));
    #[cfg(target_os = "macos")]
    std::thread::sleep(std::time::Duration::from_millis(macos_delay_ms));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(50));

    send_paste(&app_settings.paste_method, send_enter && app_settings.auto_send_enter)?;

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

fn paste_direct(text: &str) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

    enigo
        .text(text)
        .map_err(|e| format!("Failed to type text: {}", e))?;

    Ok(())
}

#[allow(unused_variables)]
fn send_paste(paste_method: &PasteMethod, auto_send_enter: bool) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let (modifier_key, key_code) = (Key::Meta, Key::Other(9));
    #[cfg(target_os = "windows")]
    let (modifier_key, key_code) = (Key::Control, Key::Other(0x56));
    #[cfg(target_os = "linux")]
    let (modifier_key, key_code) = (Key::Control, Key::Unicode('v'));

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

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

    // Send Enter key if auto_send_enter is enabled
    if auto_send_enter {
        std::thread::sleep(std::time::Duration::from_millis(150));
        enigo
            .key(Key::Return, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Enter key: {}", e))?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        enigo
            .key(Key::Return, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Enter key: {}", e))?;
    }

    Ok(())
}

pub fn get_selected_text(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let clipboard = app_handle.clipboard();
    let original_content = clipboard.read_text().unwrap_or_default();
    debug!("Previous clipboard content: {}", original_content);

    send_copy()?;
    std::thread::sleep(std::time::Duration::from_millis(150));

    let selected_text = clipboard.read_text().unwrap_or_default();
    debug!("Selected text: {}", selected_text);

    // If clipboard content changed after Ctrl+C, text was selected — restore original
    if selected_text != original_content {
        clipboard
            .write_text(&original_content)
            .map_err(|e| format!("Failed to restore clipboard in get_selected_text: {}", e))?;
        debug!("Restored clipboard content: {}", original_content);
        Ok(selected_text)
    } else {
        // Clipboard unchanged after copy: no text was selected
        Ok(String::new())
    }
}

fn send_copy() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let (modifier_key, key_code) = (Key::Meta, Key::Other(8)); // 0x08 is C
    #[cfg(target_os = "windows")]
    let (modifier_key, key_code) = (Key::Control, Key::Other(0x43)); // 0x43 is C
    #[cfg(target_os = "linux")]
    let (modifier_key, key_code) = (Key::Control, Key::Unicode('c'));

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

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
}
