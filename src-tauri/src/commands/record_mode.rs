use tauri::{command, AppHandle, Manager};
use crate::settings;
use crate::shortcuts::types::ShortcutState;

#[command]
pub fn get_record_mode(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_mode)
}

#[command]
pub fn set_record_mode(app_handle: AppHandle, mode: String) {
    let state = app_handle.state::<ShortcutState>();
    state.set_toggle_required(mode == "toggle_to_talk");
    
    let mut s = crate::settings::load_settings(&app_handle);
    s.record_mode = mode;
    let _ = crate::settings::save_settings(&app_handle, &s);
}