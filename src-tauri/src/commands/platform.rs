use tauri::{command, AppHandle};

/// Returns the current Linux session type as a wire string.
///
/// Wire values (`"wayland"` / `"x11"` / `"unknown"` / `null`) are produced by
/// [`LinuxSessionType::as_str`] and parsed by the frontend hook
/// `useLinuxSessionType` (`src/components/hooks/use-linux-session-type.ts`).
/// Keep the two ends in sync: any new variant added to `LinuxSessionType` must
/// also be accepted by the hook's narrowing check.
#[command]
pub fn get_linux_session_type() -> Option<String> {
    crate::utils::platform::get_linux_session_type().map(|session| session.as_str().to_string())
}

// Takes effect on next launch: shortcut dispatch and GDK_BACKEND are set at startup.
#[command]
pub fn set_use_wayland_portal(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    if s.use_wayland_portal != enabled {
        s.wayland_notice_dismissed = false;
    }
    s.use_wayland_portal = enabled;
    crate::settings::save_settings(&app, &s)
}

#[command]
pub fn dismiss_wayland_notice(app: AppHandle) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.wayland_notice_dismissed = true;
    crate::settings::save_settings(&app, &s)
}
