use enigo::{Enigo, Key, Keyboard, Settings};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn paste(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    let clipboard = app_handle.clipboard();
    let clipboard_content = clipboard.read_text().unwrap_or_default();
    clipboard
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_millis(100));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(50));

    send_paste()?;

    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_millis(200));
    #[cfg(target_os = "windows")]
    std::thread::sleep(std::time::Duration::from_millis(100));

    clipboard
        .write_text(&clipboard_content)
        .map_err(|e| format!("Failed to restore clipboard: {}", e))?;

    Ok(())
}

fn send_paste() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let (modifier_key, v_key_code) = (Key::Meta, Key::Other(9));
    #[cfg(target_os = "windows")]
    let (modifier_key, v_key_code) = (Key::Control, Key::Other(0x56));
    #[cfg(target_os = "linux")]
    let (modifier_key, v_key_code) = (Key::Control, Key::Unicode('v'));

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

    enigo
        .key(modifier_key, enigo::Direction::Press)
        .map_err(|e| format!("Failed to press modifier key: {}", e))?;

    enigo
        .key(v_key_code, enigo::Direction::Press)
        .map_err(|e| format!("Failed to press V key: {}", e))?;

    enigo
        .key(v_key_code, enigo::Direction::Release)
        .map_err(|e| format!("Failed to release V key: {}", e))?;

    enigo
        .key(modifier_key, enigo::Direction::Release)
        .map_err(|e| format!("Failed to release modifier key: {}", e))?;

    Ok(())
}
