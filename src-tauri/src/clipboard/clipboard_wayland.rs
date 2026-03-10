use enigo::{Enigo, Keyboard, Settings};
use log::{debug, warn};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn paste_wayland(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    match paste_via_enigo(text) {
        Ok(()) => {
            debug!("Wayland: text injected via enigo/libei");
            Ok(())
        }
        Err(e) => {
            warn!(
                "Wayland: enigo/libei injection failed ({}), falling back to clipboard",
                e
            );
            paste_via_clipboard(text, app_handle)
        }
    }
}

fn paste_via_enigo(text: &str) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo on Wayland: {}", e))?;

    enigo
        .text(text)
        .map_err(|e| format!("Failed to type text on Wayland: {}", e))?;

    Ok(())
}

fn paste_via_clipboard(text: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    let clipboard = app_handle.clipboard();

    clipboard
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard on Wayland: {}", e))?;

    warn!("Wayland: text copied to clipboard as fallback, paste manually with Ctrl+V");
    Ok(())
}
