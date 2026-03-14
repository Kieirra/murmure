use crate::settings;
use crate::settings::PasteMethod;
use enigo::{Enigo, Key, Keyboard, Settings};
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

    send_paste(&app_settings.paste_method)?;

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
fn send_paste(paste_method: &PasteMethod) -> Result<(), String> {
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

    Ok(())
}

pub fn get_selected_text(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let clipboard = app_handle.clipboard();
    let original_content = clipboard.read_text().unwrap_or_default();
    debug!(
        "Previous clipboard content length: {}",
        original_content.len()
    );

    // Write a unique marker to detect if copy actually did something
    let marker = "__MURMURE_EMPTY_MARKER__";
    let _ = clipboard.write_text(marker);

    // Give the OS a tiny moment to register the clipboard change
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Simulate Ctrl+C
    send_copy()?;

    // Wait for the OS to process the copy event and update the clipboard
    std::thread::sleep(std::time::Duration::from_millis(200));

    let copied_content = clipboard.read_text().unwrap_or_default();

    // Restore original clipboard
    let _ = clipboard.write_text(&original_content);

    if copied_content == marker {
        // Nothing was actually copied (no selection)
        debug!("No text was selected (clipboard still has marker).");
        Ok(String::new())
    } else {
        debug!("Selected text length: {}", copied_content.len());
        Ok(copied_content)
    }
}

fn send_copy() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

    #[cfg(target_os = "macos")]
    let modifier_key = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier_key = Key::Control;

    // Press modifier
    enigo
        .key(modifier_key, enigo::Direction::Press)
        .map_err(|e| format!("Failed to press modifier key: {}", e))?;

    // Use string type for the 'c' char instead of relying on hex arrays across platforms
    enigo
        .key(Key::Unicode('c'), enigo::Direction::Click)
        .map_err(|e| format!("Failed to press C key: {}", e))?;

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Release modifier
    enigo
        .key(modifier_key, enigo::Direction::Release)
        .map_err(|e| format!("Failed to release modifier key: {}", e))?;

    Ok(())
}
