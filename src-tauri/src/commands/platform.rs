use log::warn;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_linux_session_type() -> Option<String> {
    crate::utils::platform::get_linux_session_type().map(|session| session.as_str().to_string())
}

#[command]
pub fn is_xwayland_fallback() -> bool {
    #[cfg(target_os = "linux")]
    {
        crate::utils::platform::is_xwayland_fallback()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[command]
pub fn refresh_main_window(app: AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        warn!("refresh_main_window: main window not found");
        return;
    };
    if let Err(e) = window.hide() {
        warn!("refresh_main_window hide failed: {}", e);
    }
    if let Err(e) = window.unminimize() {
        warn!("refresh_main_window unminimize failed: {}", e);
    }
    if let Err(e) = window.show() {
        warn!("refresh_main_window show failed: {}", e);
    }
    if let Err(e) = window.set_focus() {
        warn!("refresh_main_window focus failed: {}", e);
    }
}
