use crate::settings;
use crate::shortcuts::{ActivationMode, ShortcutRegistryState};
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_record_mode(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_mode)
}

#[command]
pub fn set_record_mode(app_handle: AppHandle, mode: String) {
    let activation_mode = if mode == "toggle_to_talk" {
        ActivationMode::ToggleToTalk
    } else {
        ActivationMode::PushToTalk
    };

    app_handle
        .state::<ShortcutRegistryState>()
        .set_activation_mode(activation_mode);

    let mut s = crate::settings::load_settings(&app_handle);
    s.record_mode = mode;
    let _ = crate::settings::save_settings(&app_handle, &s);
}
