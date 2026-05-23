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

// Whitelist of desktop environments where the XDG Portal screencast/global
// shortcuts path is known to be reliable. Anything outside this list
// (GNOME/Mutter, Cinnamon, MATE, XFCE-Wayland, unknown desktop environments)
// defaults to CLI because there is no robust runtime probe for portal
// capability.
#[cfg(target_os = "linux")]
const PORTAL_RELIABLE_DESKTOPS: &[&str] = &["KDE", "Hyprland", "sway"];

#[cfg(target_os = "linux")]
pub fn is_portal_reliable_desktop() -> bool {
    use std::sync::OnceLock;
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        is_portal_reliable_desktop_from_value(std::env::var("XDG_CURRENT_DESKTOP").ok().as_deref())
    })
}

#[cfg(target_os = "linux")]
fn is_portal_reliable_desktop_from_value(xdg_current_desktop: Option<&str>) -> bool {
    match xdg_current_desktop {
        Some(value) => value.split(':').any(|token| {
            let token = token.trim();
            PORTAL_RELIABLE_DESKTOPS
                .iter()
                .any(|reliable| token.eq_ignore_ascii_case(reliable))
        }),
        None => false,
    }
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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn returns_wayland_when_wayland_display_is_set() {
        let result = get_linux_session_type_from_values(Some("wayland-0"), None, None);
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn returns_wayland_when_wayland_display_is_set_even_if_x11_display_is_set() {
        let result = get_linux_session_type_from_values(Some("wayland-0"), Some("x11"), Some(":0"));
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn ignores_empty_wayland_display() {
        let result = get_linux_session_type_from_values(Some("   "), Some("x11"), Some(":0"));
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn returns_wayland_when_xdg_session_type_is_wayland() {
        let result = get_linux_session_type_from_values(None, Some("wayland"), None);
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn returns_x11_when_xdg_session_type_is_x11() {
        let result = get_linux_session_type_from_values(None, Some("x11"), None);
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn matches_xdg_session_type_case_insensitively() {
        let result = get_linux_session_type_from_values(None, Some("WAYLAND"), None);
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn trims_whitespace_around_xdg_session_type() {
        let result = get_linux_session_type_from_values(None, Some("  x11  "), None);
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn returns_x11_when_only_display_is_set() {
        let result = get_linux_session_type_from_values(None, None, Some(":0"));
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn returns_unknown_when_no_signal_is_present() {
        let result = get_linux_session_type_from_values(None, None, None);
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    #[test]
    fn returns_unknown_when_xdg_session_type_is_unrecognized_and_no_display() {
        let result = get_linux_session_type_from_values(None, Some("tty"), None);
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    #[test]
    fn ignores_empty_display() {
        let result = get_linux_session_type_from_values(None, None, Some(""));
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    #[test]
    fn xdg_session_type_takes_precedence_over_display_fallback() {
        let result = get_linux_session_type_from_values(None, Some("wayland"), Some(":0"));
        assert_eq!(result, LinuxSessionType::Wayland);
    }
}
