use crate::settings;
use tauri::{AppHandle, command};

#[command]
pub fn get_copy_to_clipboard(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.copy_to_clipboard)
}

#[command]
pub fn set_copy_to_clipboard(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.copy_to_clipboard = enabled;
    settings::save_settings(&app, &s)
}
