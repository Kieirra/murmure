// The enum and `as_str()` stay cross-platform so the Tauri command
// `get_linux_session_type` keeps the same wire values on every OS (the
// frontend hook parses them as strings). Variants are never constructed
// off-Linux, hence the targeted `allow(dead_code)`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub enum LinuxSessionType {
    Wayland,
    X11,
    Unknown,
}

impl LinuxSessionType {
    pub const fn as_str(self) -> &'static str {
        match self {
            LinuxSessionType::Wayland => "wayland",
            LinuxSessionType::X11 => "x11",
            LinuxSessionType::Unknown => "unknown",
        }
    }
}

#[cfg(target_os = "linux")]
pub fn is_wayland_session() -> bool {
    matches!(get_linux_session_type(), Some(LinuxSessionType::Wayland))
}

pub fn get_linux_session_type() -> Option<LinuxSessionType> {
    #[cfg(target_os = "linux")]
    {
        use std::sync::OnceLock;
        static CACHED: OnceLock<LinuxSessionType> = OnceLock::new();
        Some(*CACHED.get_or_init(|| {
            let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
            let xdg_session_type = std::env::var("XDG_SESSION_TYPE").ok();
            let x11_display = std::env::var("DISPLAY").ok();
            get_linux_session_type_from_values(
                wayland_display.as_deref(),
                xdg_session_type.as_deref(),
                x11_display.as_deref(),
            )
        }))
    }

    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn has_non_empty_value(value: Option<&str>) -> bool {
    value.is_some_and(|v| !v.trim().is_empty())
}

#[cfg(target_os = "linux")]
fn get_linux_session_type_from_values(
    wayland_display: Option<&str>,
    xdg_session_type: Option<&str>,
    x11_display: Option<&str>,
) -> LinuxSessionType {
    if has_non_empty_value(wayland_display) {
        return LinuxSessionType::Wayland;
    }

    match xdg_session_type {
        Some(value) if value.trim().eq_ignore_ascii_case("wayland") => LinuxSessionType::Wayland,
        Some(value) if value.trim().eq_ignore_ascii_case("x11") => LinuxSessionType::X11,
        _ => {
            if has_non_empty_value(x11_display) {
                LinuxSessionType::X11
            } else {
                LinuxSessionType::Unknown
            }
        }
    }
}
