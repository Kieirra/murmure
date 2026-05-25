use serde::Serialize;
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

/// Wire-shape returned by the `get_linux_distro_info` Tauri command.
///
/// `desktop_env` is serialized as a lowercase string (e.g. `"gnome"`,
/// `"cinnamon"`, `"other"`). Parsed by the frontend hook
/// `useLinuxDistroInfo` (`src/components/hooks/use-linux-session-type.ts`).
#[derive(Debug, Clone, Serialize)]
pub struct LinuxDistroInfoDto {
    pub os_name: Option<String>,
    pub desktop_env: String,
}

/// Returns the Linux OS name (from `/etc/os-release`) and the detected
/// desktop environment (from `XDG_CURRENT_DESKTOP`).
///
/// Returns `None` on non-Linux platforms. The frontend hook only calls this
/// command after `get_linux_session_type` has returned a Linux session.
#[command]
pub fn get_linux_distro_info() -> Option<LinuxDistroInfoDto> {
    crate::utils::platform::get_linux_distro_info().map(|info| LinuxDistroInfoDto {
        os_name: info.os_name,
        desktop_env: info.desktop_env.as_str().to_string(),
    })
}

#[command]
pub fn dismiss_wayland_notice(app: AppHandle) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.wayland_notice_dismissed = true;
    crate::settings::save_settings(&app, &s)
}

#[command]
pub fn dismiss_wayland_clipboard_fallback(app: AppHandle) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.wayland_clipboard_fallback_dismissed = true;
    crate::settings::save_settings(&app, &s)
}
