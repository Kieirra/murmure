use tauri::{command, AppHandle};

#[command]
pub fn get_linux_session_type() -> Option<String> {
    crate::utils::platform::get_linux_session_type().map(|session| session.as_str().to_string())
}

// Takes effect on next launch: shortcut dispatch and GDK_BACKEND are set at startup.
#[command]
pub fn set_use_wayland_portal(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.use_wayland_portal = enabled;
    crate::settings::save_settings(&app, &s)
}
