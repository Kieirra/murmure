// The enum and `as_str()` stay cross-platform so the Tauri command
// `get_linux_session_type` keeps the same wire values on every OS (the
// frontend hook parses them as strings). Variants are never constructed
// off-Linux, hence the targeted `allow(dead_code)`.

use serde::Serialize;

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

/// Desktop environment detected from `XDG_CURRENT_DESKTOP`.
///
/// Wire values are produced by [`DesktopEnvironment::as_str`] and consumed by
/// the frontend hook `useLinuxDistroInfo`. Keep the two ends in sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
#[serde(rename_all = "lowercase")]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Cinnamon,
    Xfce,
    Mate,
    Hyprland,
    Sway,
    I3,
    Other,
}

impl DesktopEnvironment {
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub const fn as_str(self) -> &'static str {
        match self {
            DesktopEnvironment::Gnome => "gnome",
            DesktopEnvironment::Kde => "kde",
            DesktopEnvironment::Cinnamon => "cinnamon",
            DesktopEnvironment::Xfce => "xfce",
            DesktopEnvironment::Mate => "mate",
            DesktopEnvironment::Hyprland => "hyprland",
            DesktopEnvironment::Sway => "sway",
            DesktopEnvironment::I3 => "i3",
            DesktopEnvironment::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub struct LinuxDistroInfo {
    pub os_name: Option<String>,
    pub desktop_env: DesktopEnvironment,
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
pub fn get_linux_distro_info() -> Option<LinuxDistroInfo> {
    use std::sync::OnceLock;
    static CACHED: OnceLock<LinuxDistroInfo> = OnceLock::new();
    Some(
        CACHED
            .get_or_init(|| {
                let os_release = std::fs::read_to_string("/etc/os-release").ok();
                let os_name = os_release
                    .as_deref()
                    .and_then(parse_os_name_from_os_release);
                let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").ok();
                let desktop_env =
                    detect_desktop_environment_from_value(xdg_current_desktop.as_deref());
                LinuxDistroInfo {
                    os_name,
                    desktop_env,
                }
            })
            .clone(),
    )
}

#[cfg(not(target_os = "linux"))]
pub fn get_linux_distro_info() -> Option<LinuxDistroInfo> {
    None
}

#[cfg(target_os = "linux")]
fn parse_os_name_from_os_release(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("NAME=") {
            let trimmed = rest.trim();
            let unquoted = trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| {
                    trimmed
                        .strip_prefix('\'')
                        .and_then(|s| s.strip_suffix('\''))
                })
                .unwrap_or(trimmed);
            let unquoted = unquoted.trim();
            if unquoted.is_empty() {
                return None;
            }
            return Some(unquoted.to_string());
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn detect_desktop_environment_from_value(xdg_current_desktop: Option<&str>) -> DesktopEnvironment {
    let Some(value) = xdg_current_desktop else {
        return DesktopEnvironment::Other;
    };
    let last_segment = value
        .split(':')
        .next_back()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if last_segment.is_empty() {
        return DesktopEnvironment::Other;
    }
    if last_segment.contains("gnome") {
        DesktopEnvironment::Gnome
    } else if last_segment.contains("kde") {
        DesktopEnvironment::Kde
    } else if last_segment.contains("cinnamon") {
        DesktopEnvironment::Cinnamon
    } else if last_segment.contains("xfce") {
        DesktopEnvironment::Xfce
    } else if last_segment.contains("mate") {
        DesktopEnvironment::Mate
    } else if last_segment.contains("hyprland") {
        DesktopEnvironment::Hyprland
    } else if last_segment.contains("sway") {
        DesktopEnvironment::Sway
    } else if last_segment.contains("i3") {
        DesktopEnvironment::I3
    } else {
        DesktopEnvironment::Other
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

    #[test]
    fn parses_os_name_from_quoted_value() {
        let content = "PRETTY_NAME=\"Linux Mint 22\"\nNAME=\"Linux Mint\"\nID=linuxmint\n";
        assert_eq!(
            parse_os_name_from_os_release(content),
            Some("Linux Mint".to_string())
        );
    }

    #[test]
    fn parses_os_name_from_unquoted_value() {
        let content = "NAME=Ubuntu\nVERSION=\"24.04\"\n";
        assert_eq!(
            parse_os_name_from_os_release(content),
            Some("Ubuntu".to_string())
        );
    }

    #[test]
    fn parses_os_name_when_field_is_not_first_line() {
        let content = "PRETTY_NAME=\"Fedora Linux 41 (Workstation Edition)\"\nNAME=\"Fedora Linux\"\nVERSION=\"41\"\n";
        assert_eq!(
            parse_os_name_from_os_release(content),
            Some("Fedora Linux".to_string())
        );
    }

    #[test]
    fn returns_none_when_name_is_absent() {
        let content = "ID=arch\nPRETTY_NAME=\"Arch Linux\"\n";
        assert_eq!(parse_os_name_from_os_release(content), None);
    }

    #[test]
    fn returns_none_when_name_is_empty() {
        let content = "NAME=\"\"\n";
        assert_eq!(parse_os_name_from_os_release(content), None);
    }

    #[test]
    fn maps_gnome_from_xdg_current_desktop() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("GNOME")),
            DesktopEnvironment::Gnome
        );
    }

    #[test]
    fn maps_gnome_from_ubuntu_prefix() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("ubuntu:GNOME")),
            DesktopEnvironment::Gnome
        );
    }

    #[test]
    fn maps_cinnamon_from_x_cinnamon() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("X-Cinnamon")),
            DesktopEnvironment::Cinnamon
        );
    }

    #[test]
    fn maps_kde() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("KDE")),
            DesktopEnvironment::Kde
        );
    }

    #[test]
    fn maps_xfce() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("XFCE")),
            DesktopEnvironment::Xfce
        );
    }

    #[test]
    fn maps_mate() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("MATE")),
            DesktopEnvironment::Mate
        );
    }

    #[test]
    fn maps_hyprland() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("Hyprland")),
            DesktopEnvironment::Hyprland
        );
    }

    #[test]
    fn maps_sway() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("sway")),
            DesktopEnvironment::Sway
        );
    }

    #[test]
    fn maps_i3() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("i3")),
            DesktopEnvironment::I3
        );
    }

    #[test]
    fn maps_other_when_unknown() {
        assert_eq!(
            detect_desktop_environment_from_value(Some("Budgie")),
            DesktopEnvironment::Other
        );
    }

    #[test]
    fn maps_other_when_none() {
        assert_eq!(
            detect_desktop_environment_from_value(None),
            DesktopEnvironment::Other
        );
    }

    #[test]
    fn takes_last_segment_after_colon() {
        // Pop:GNOME → last segment GNOME → Gnome
        assert_eq!(
            detect_desktop_environment_from_value(Some("Pop:GNOME")),
            DesktopEnvironment::Gnome
        );
    }
}
