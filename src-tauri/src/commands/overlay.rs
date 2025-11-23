use crate::settings;
use tauri::{AppHandle, command};

#[command]
pub fn get_overlay_mode(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.overlay_mode)
}

#[command]
pub fn set_overlay_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let allowed = ["hidden", "recording", "always"];
    if !allowed.contains(&mode.as_str()) {
        return Err("Invalid overlay mode".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_mode = mode;
    let res = settings::save_settings(&app, &s);
    match s.overlay_mode.as_str() {
        "always" => {
            crate::overlay::overlay::show_recording_overlay(&app);
        }
        "hidden" | "recording" => {
            crate::overlay::overlay::hide_recording_overlay(&app);
        }
        _ => {}
    }
    res
}

#[command]
pub fn get_overlay_position(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.overlay_position)
}

#[command]
pub fn set_overlay_position(app: AppHandle, position: String) -> Result<(), String> {
    let allowed = ["top", "bottom"];
    if !allowed.contains(&position.as_str()) {
        return Err("Invalid overlay position".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.overlay_position = position;
    let res = settings::save_settings(&app, &s);
    crate::overlay::overlay::update_overlay_position(&app);
    res
}
