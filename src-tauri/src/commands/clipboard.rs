use crate::settings;
use crate::settings::PasteMethod;
use tauri::{command, AppHandle};

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

#[command]
pub fn get_paste_method(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    let method = match s.paste_method {
        PasteMethod::CtrlV => "ctrl_v",
        PasteMethod::CtrlShiftV => "ctrl_shift_v",
        PasteMethod::Direct => "direct",
    };
    Ok(method.to_string())
}

#[command]
pub fn set_paste_method(app: AppHandle, method: String) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.paste_method = match method.as_str() {
        "ctrl_shift_v" => PasteMethod::CtrlShiftV,
        "direct" => PasteMethod::Direct,
        _ => PasteMethod::CtrlV,
    };
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_auto_send_enter(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.auto_send_enter)
}

#[command]
pub fn set_auto_send_enter(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.auto_send_enter = enabled;
    settings::save_settings(&app, &s)
}
