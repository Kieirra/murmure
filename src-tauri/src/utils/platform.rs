#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn get_linux_session_type() -> Option<LinuxSessionType> {
    #[cfg(target_os = "linux")]
    {
        let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
        let xdg_session_type = std::env::var("XDG_SESSION_TYPE").ok();
        let x11_display = std::env::var("DISPLAY").ok();

        return Some(get_linux_session_type_from_values(
            wayland_display.as_deref(),
            xdg_session_type.as_deref(),
            x11_display.as_deref(),
        ));
    }

    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

fn has_non_empty_value(value: Option<&str>) -> bool {
    match value {
        Some(raw_value) => !raw_value.trim().is_empty(),
        None => false,
    }
}

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

#[cfg(test)]
mod tests {
    use super::{get_linux_session_type_from_values, LinuxSessionType};

    #[test]
    fn returns_wayland_when_wayland_display_is_set() {
        assert_eq!(
            get_linux_session_type_from_values(Some("wayland-0"), None, None),
            LinuxSessionType::Wayland
        );
    }

    #[test]
    fn returns_wayland_when_session_type_is_wayland() {
        assert_eq!(
            get_linux_session_type_from_values(None, Some("wayland"), None),
            LinuxSessionType::Wayland
        );
        assert_eq!(
            get_linux_session_type_from_values(None, Some("Wayland"), None),
            LinuxSessionType::Wayland
        );
    }

    #[test]
    fn uses_session_type_when_wayland_display_is_empty() {
        assert_eq!(
            get_linux_session_type_from_values(Some(""), Some("wayland"), None),
            LinuxSessionType::Wayland
        );
    }

    #[test]
    fn returns_x11_when_session_type_is_x11() {
        assert_eq!(
            get_linux_session_type_from_values(None, Some("x11"), None),
            LinuxSessionType::X11
        );
    }

    #[test]
    fn returns_x11_when_display_is_set() {
        assert_eq!(
            get_linux_session_type_from_values(None, None, Some(":0")),
            LinuxSessionType::X11
        );
    }

    #[test]
    fn returns_unknown_when_vars_are_missing_or_empty() {
        assert_eq!(
            get_linux_session_type_from_values(None, None, None),
            LinuxSessionType::Unknown
        );
        assert_eq!(
            get_linux_session_type_from_values(Some(""), None, None),
            LinuxSessionType::Unknown
        );
        assert_eq!(
            get_linux_session_type_from_values(Some("   "), None, Some("   ")),
            LinuxSessionType::Unknown
        );
    }
}
