use crate::settings;
use crate::settings::PasteMethod;
use tauri::{command, AppHandle};

#[command]
pub fn set_copy_to_clipboard(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.copy_to_clipboard = enabled;
    settings::save_settings(&app, &s)
}

#[command]
pub fn set_paste_method(app: AppHandle, method: String) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.paste_method = match method.to_lowercase().as_str() {
        "ctrl_shift_v" | "ctrlshiftv" => PasteMethod::CtrlShiftV,
        "direct" => PasteMethod::Direct,
        _ => PasteMethod::CtrlV,
    };
    settings::save_settings(&app, &s)
}
