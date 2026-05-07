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
pub fn is_gnome_session() -> bool {
    use std::sync::OnceLock;
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        std::env::var("XDG_CURRENT_DESKTOP")
            .ok()
            .as_deref()
            .map(|v| v.split(':').any(|s| s.trim().eq_ignore_ascii_case("GNOME")))
            .unwrap_or(false)
    })
}

// GNOME defaults to CLI because Mutter's portal callbacks are unreliable;
// other Wayland compositors default to XDG Portal.
#[cfg(target_os = "linux")]
pub fn default_use_wayland_portal() -> bool {
    is_wayland_session() && !is_gnome_session()
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

    // Wayland detection: WAYLAND_DISPLAY wins outright, regardless of other
    // env vars. This is what the spec relies on to default GNOME to CLI mode.

    #[test]
    fn returns_wayland_when_wayland_display_is_set() {
        // Given - WAYLAND_DISPLAY is populated and other vars are absent
        // When - resolving the session type
        let result = get_linux_session_type_from_values(Some("wayland-0"), None, None);
        // Then - Wayland is detected
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn returns_wayland_when_wayland_display_is_set_even_if_x11_display_is_set() {
        // Given - both WAYLAND_DISPLAY and DISPLAY are set (XWayland scenario)
        // When - resolving the session type
        let result = get_linux_session_type_from_values(Some("wayland-0"), Some("x11"), Some(":0"));
        // Then - Wayland wins
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn ignores_empty_wayland_display() {
        // Given - WAYLAND_DISPLAY is present but blank (whitespace)
        // When - resolving the session type
        let result = get_linux_session_type_from_values(Some("   "), Some("x11"), Some(":0"));
        // Then - falls back to XDG_SESSION_TYPE/DISPLAY
        assert_eq!(result, LinuxSessionType::X11);
    }

    // XDG_SESSION_TYPE acts as the secondary signal when WAYLAND_DISPLAY is absent.

    #[test]
    fn returns_wayland_when_xdg_session_type_is_wayland() {
        // Given - only XDG_SESSION_TYPE indicates Wayland
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("wayland"), None);
        // Then - Wayland is detected
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn returns_x11_when_xdg_session_type_is_x11() {
        // Given - only XDG_SESSION_TYPE indicates X11
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("x11"), None);
        // Then - X11 is detected
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn matches_xdg_session_type_case_insensitively() {
        // Given - XDG_SESSION_TYPE is uppercase
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("WAYLAND"), None);
        // Then - still detected as Wayland
        assert_eq!(result, LinuxSessionType::Wayland);
    }

    #[test]
    fn trims_whitespace_around_xdg_session_type() {
        // Given - XDG_SESSION_TYPE has surrounding whitespace
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("  x11  "), None);
        // Then - X11 is detected
        assert_eq!(result, LinuxSessionType::X11);
    }

    // DISPLAY is the last-resort fallback when nothing else is conclusive.

    #[test]
    fn returns_x11_when_only_display_is_set() {
        // Given - neither WAYLAND_DISPLAY nor XDG_SESSION_TYPE are usable
        // When - DISPLAY is set
        let result = get_linux_session_type_from_values(None, None, Some(":0"));
        // Then - X11 is detected from DISPLAY alone
        assert_eq!(result, LinuxSessionType::X11);
    }

    #[test]
    fn returns_unknown_when_no_signal_is_present() {
        // Given - all env vars are absent
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, None, None);
        // Then - Unknown is returned
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    #[test]
    fn returns_unknown_when_xdg_session_type_is_unrecognized_and_no_display() {
        // Given - XDG_SESSION_TYPE has an unknown value and DISPLAY is absent
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("tty"), None);
        // Then - Unknown is returned (no fallback signal)
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    // Edge cases tied to env-var quirks shells can produce.

    #[test]
    fn ignores_empty_display() {
        // Given - DISPLAY is set but blank
        // When - resolving the session type with no other signal
        let result = get_linux_session_type_from_values(None, None, Some(""));
        // Then - Unknown, blank DISPLAY is not a usable signal
        assert_eq!(result, LinuxSessionType::Unknown);
    }

    #[test]
    fn xdg_session_type_takes_precedence_over_display_fallback() {
        // Given - XDG_SESSION_TYPE says wayland but DISPLAY is also set
        // When - resolving the session type
        let result = get_linux_session_type_from_values(None, Some("wayland"), Some(":0"));
        // Then - XDG_SESSION_TYPE wins over DISPLAY fallback
        assert_eq!(result, LinuxSessionType::Wayland);
    }
}
