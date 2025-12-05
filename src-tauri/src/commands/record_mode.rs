use tauri::{command, AppHandle, Manager};
use crate::settings;
use crate::shortcuts::{IsToggleRequiredForRecording};

#[command]
pub fn get_record_mode(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_mode)
}

#[command]
pub fn set_record_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let allowed = ["push_to_talk", "toggle_to_talk"];
    if !allowed.contains(&mode.as_str()) {
        return Err("Invalid record mode".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.record_mode = mode;
    app.state::<IsToggleRequiredForRecording>().set(s.record_mode == "toggle_to_talk");
    settings::save_settings(&app, &s)
}