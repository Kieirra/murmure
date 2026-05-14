//! Best-effort layout detection cascade. Each step may fail silently
//! and falls through to the next, then to US fallback. Probes use only
//! desktop-bundled CLI tools (gsettings, hyprctl, etc) or config files.

use super::types::LayoutInfo;
use log::debug;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Hard cap on external probe duration. Each step short-circuits within
/// this budget so a stuck command never blocks Tauri's setup.
const PROBE_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub layout: LayoutInfo,
    pub used_fallback: bool,
}

/// Run the full cascade. Returns the first successful detection or a
/// US fallback when every step failed.
pub fn detect_layout() -> DetectionResult {
    let xdg = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let desktop = xdg
        .split(':')
        .next()
        .unwrap_or("")
        .to_lowercase();

    if desktop.contains("gnome") {
        if let Some(info) = try_gsettings() {
            debug!("layout_detect: GNOME gsettings hit {:?}", info);
            return DetectionResult { layout: info, used_fallback: false };
        }
    }
    if desktop.contains("kde") {
        if let Some(info) = try_kxkbrc() {
            debug!("layout_detect: KDE kxkbrc hit {:?}", info);
            return DetectionResult { layout: info, used_fallback: false };
        }
    }
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        if let Some(info) = try_hyprctl() {
            debug!("layout_detect: Hyprland hyprctl hit {:?}", info);
            return DetectionResult { layout: info, used_fallback: false };
        }
    }
    if std::env::var("SWAYSOCK").is_ok() {
        if let Some(info) = try_swaymsg() {
            debug!("layout_detect: sway swaymsg hit {:?}", info);
            return DetectionResult { layout: info, used_fallback: false };
        }
    }

    if let Some(info) = try_setxkbmap() {
        debug!("layout_detect: setxkbmap hit {:?}", info);
        return DetectionResult { layout: info, used_fallback: false };
    }
    if let Some(info) = try_localectl() {
        debug!("layout_detect: localectl hit {:?}", info);
        return DetectionResult { layout: info, used_fallback: false };
    }
    if let Some(info) = try_etc_default_keyboard() {
        debug!("layout_detect: /etc/default/keyboard hit {:?}", info);
        return DetectionResult { layout: info, used_fallback: false };
    }

    DetectionResult {
        layout: LayoutInfo::us_fallback(),
        used_fallback: true,
    }
}

fn run_capture(program: &str, args: &[&str]) -> Option<String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let output = child.wait_with_output().ok()?;
                return Some(String::from_utf8_lossy(&output.stdout).into_owned());
            }
            Ok(Some(_)) => return None,
            Ok(None) => {
                if start.elapsed() > PROBE_TIMEOUT {
                    let _ = child.kill();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

fn try_gsettings() -> Option<LayoutInfo> {
    let out = run_capture("gsettings", &["get", "org.gnome.desktop.input-sources", "mru-sources"])?;
    if let Some(info) = parse_gsettings_output(&out) {
        return Some(info);
    }
    let sources = run_capture("gsettings", &["get", "org.gnome.desktop.input-sources", "sources"])?;
    parse_gsettings_output(&sources)
}

/// Parse a GVariant-encoded list of (kind, id) tuples and pick the first
/// XKB entry. Accepts the trailing-typed empty form `@a(ss) []`.
pub fn parse_gsettings_output(s: &str) -> Option<LayoutInfo> {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed.ends_with("[]") {
        return None;
    }
    // Walk every (a, b) tuple, keep the first whose kind == "xkb".
    let mut rest = trimmed;
    while let Some(open) = rest.find('(') {
        let after = &rest[open + 1..];
        let close = after.find(')')?;
        let body = &after[..close];
        rest = &after[close + 1..];

        let mut parts = body.split(',').map(str::trim);
        let kind = parts.next()?.trim_matches('\'').trim_matches('"');
        let id = parts.next()?.trim().trim_matches('\'').trim_matches('"');
        if kind == "xkb" && !id.is_empty() {
            return Some(parse_layout_variant(id));
        }
    }
    None
}

fn try_kxkbrc() -> Option<LayoutInfo> {
    let home = std::env::var("HOME").ok()?;
    let path = format!("{}/.config/kxkbrc", home);
    let content = std::fs::read_to_string(&path).ok()?;
    parse_kxkbrc(&content)
}

pub fn parse_kxkbrc(s: &str) -> Option<LayoutInfo> {
    let mut in_layout_section = false;
    let mut layouts: Option<String> = None;
    let mut variants: Option<String> = None;
    for line in s.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_layout_section = trimmed.eq_ignore_ascii_case("[Layout]");
            continue;
        }
        if !in_layout_section {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("LayoutList=") {
            layouts = Some(rest.to_string());
        } else if let Some(rest) = trimmed.strip_prefix("VariantList=") {
            variants = Some(rest.to_string());
        }
    }
    let layouts = layouts?;
    let layout = layouts.split(',').next()?.trim().to_string();
    if layout.is_empty() {
        return None;
    }
    let variant = variants
        .as_deref()
        .and_then(|v| v.split(',').next())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string);
    Some(LayoutInfo::new(layout, variant))
}

fn try_hyprctl() -> Option<LayoutInfo> {
    let layout_json = run_capture("hyprctl", &["getoption", "input:kb_layout", "-j"])?;
    let layout = extract_hyprctl_str(&layout_json)?;
    let variant_json = run_capture("hyprctl", &["getoption", "input:kb_variant", "-j"]);
    let variant = variant_json
        .as_deref()
        .and_then(extract_hyprctl_str)
        .filter(|v| !v.is_empty());
    Some(LayoutInfo::new(layout, variant))
}

/// Tolerant JSON sniffer: `hyprctl` may use the `str` key on recent
/// versions and other keys on older builds; we grab the first quoted
/// string after the first `:` we find that looks like a value.
pub fn extract_hyprctl_str(json: &str) -> Option<String> {
    // Accept any first-level string under "str" or "value".
    let candidates = ["\"str\"", "\"value\""];
    for needle in candidates {
        if let Some(idx) = json.find(needle) {
            let after = &json[idx + needle.len()..];
            if let Some(colon) = after.find(':') {
                let tail = &after[colon + 1..];
                if let Some(start) = tail.find('"') {
                    let body = &tail[start + 1..];
                    if let Some(end) = body.find('"') {
                        let raw = &body[..end];
                        // Pick first comma-separated layout (e.g. "fr,us").
                        let first = raw.split(',').next()?.trim();
                        if !first.is_empty() {
                            return Some(first.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn try_swaymsg() -> Option<LayoutInfo> {
    let out = run_capture("swaymsg", &["-t", "get_inputs"])?;
    parse_swaymsg_output(&out)
}

pub fn parse_swaymsg_output(s: &str) -> Option<LayoutInfo> {
    // Tolerant scan: find the first keyboard input and read its
    // xkb_layout_names array (preferred over the long-name field).
    let mut cursor = s;
    while let Some(idx) = cursor.find("\"type\"") {
        let segment = &cursor[idx..];
        let segment_end = segment.find('}').unwrap_or(segment.len());
        let item = &segment[..segment_end];
        cursor = &segment[segment_end..];

        if !item.contains("\"keyboard\"") {
            continue;
        }
        if let Some(names_idx) = item.find("\"xkb_layout_names\"") {
            let after = &item[names_idx..];
            if let Some(open) = after.find('[') {
                let arr = &after[open + 1..];
                if let Some(close) = arr.find(']') {
                    let body = &arr[..close];
                    let first = body
                        .split(',')
                        .map(str::trim)
                        .find(|t| !t.is_empty())?;
                    let cleaned = first.trim_matches('"').trim_matches('\'').trim();
                    if !cleaned.is_empty() {
                        return Some(parse_layout_variant(cleaned));
                    }
                }
            }
        }
    }
    None
}

fn try_setxkbmap() -> Option<LayoutInfo> {
    let out = run_capture("setxkbmap", &["-query"])?;
    parse_setxkbmap_output(&out)
}

pub fn parse_setxkbmap_output(s: &str) -> Option<LayoutInfo> {
    let mut layout: Option<String> = None;
    let mut variant: Option<String> = None;
    for line in s.lines() {
        let mut parts = line.splitn(2, ':');
        let key = parts.next()?.trim();
        let Some(value) = parts.next() else {
            continue;
        };
        let value = value.trim();
        match key {
            "layout" => {
                let first = value.split(',').next().unwrap_or("").trim();
                if !first.is_empty() {
                    layout = Some(first.to_string());
                }
            }
            "variant" => {
                let first = value.split(',').next().unwrap_or("").trim();
                if !first.is_empty() {
                    variant = Some(first.to_string());
                }
            }
            _ => {}
        }
    }
    let layout = layout?;
    Some(LayoutInfo::new(layout, variant))
}

fn try_localectl() -> Option<LayoutInfo> {
    let out = run_capture("localectl", &["status"])?;
    parse_localectl_output(&out)
}

pub fn parse_localectl_output(s: &str) -> Option<LayoutInfo> {
    let mut layout: Option<String> = None;
    let mut variant: Option<String> = None;
    for line in s.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("X11 Layout:") {
            let first = rest.trim().split(',').next().unwrap_or("").trim();
            if !first.is_empty() {
                layout = Some(first.to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("X11 Variant:") {
            let first = rest.trim().split(',').next().unwrap_or("").trim();
            if !first.is_empty() {
                variant = Some(first.to_string());
            }
        }
    }
    let layout = layout?;
    Some(LayoutInfo::new(layout, variant))
}

fn try_etc_default_keyboard() -> Option<LayoutInfo> {
    let content = std::fs::read_to_string("/etc/default/keyboard").ok()?;
    parse_etc_default_keyboard(&content)
}

pub fn parse_etc_default_keyboard(s: &str) -> Option<LayoutInfo> {
    let mut layout: Option<String> = None;
    let mut variant: Option<String> = None;
    for line in s.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("XKBLAYOUT=") {
            let raw = rest.trim().trim_matches('"').trim_matches('\'');
            let first = raw.split(',').next().unwrap_or("").trim();
            if !first.is_empty() {
                layout = Some(first.to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("XKBVARIANT=") {
            let raw = rest.trim().trim_matches('"').trim_matches('\'');
            let first = raw.split(',').next().unwrap_or("").trim();
            if !first.is_empty() {
                variant = Some(first.to_string());
            }
        }
    }
    let layout = layout?;
    Some(LayoutInfo::new(layout, variant))
}

/// Split an `xkb`-style id (`"fr+oss"`, `"de+nodeadkeys"`, `"us"`) into
/// layout + optional variant.
fn parse_layout_variant(id: &str) -> LayoutInfo {
    let mut parts = id.splitn(2, '+');
    let layout = parts.next().unwrap_or("us").trim().to_string();
    let variant = parts.next().map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    LayoutInfo::new(layout, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gsettings_mru_sources_picks_first_xkb() {
        let out = "[('xkb', 'fr+oss'), ('xkb', 'us')]";
        let info = parse_gsettings_output(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_gsettings_with_no_variant() {
        let out = "[('xkb', 'us')]";
        let info = parse_gsettings_output(out).unwrap();
        assert_eq!(info.layout, "us");
        assert!(info.variant.is_none());
    }

    #[test]
    fn parse_gsettings_empty_returns_none() {
        assert!(parse_gsettings_output("@a(ss) []").is_none());
        assert!(parse_gsettings_output("[]").is_none());
        assert!(parse_gsettings_output("").is_none());
    }

    #[test]
    fn parse_gsettings_skips_non_xkb_kinds() {
        let out = "[('ibus', 'mozc-jp'), ('xkb', 'jp')]";
        let info = parse_gsettings_output(out).unwrap();
        assert_eq!(info.layout, "jp");
    }

    #[test]
    fn parse_setxkbmap_query_extracts_layout_and_variant() {
        let out = "rules:      evdev\nmodel:      pc105\nlayout:     fr\nvariant:    oss\n";
        let info = parse_setxkbmap_output(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_setxkbmap_query_handles_missing_variant() {
        let out = "rules:      evdev\nmodel:      pc105\nlayout:     us\n";
        let info = parse_setxkbmap_output(out).unwrap();
        assert_eq!(info.layout, "us");
        assert!(info.variant.is_none());
    }

    #[test]
    fn parse_setxkbmap_query_picks_first_layout_when_multiple() {
        let out = "layout:     fr,us\nvariant:    oss,\n";
        let info = parse_setxkbmap_output(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_localectl_extracts_layout_and_variant() {
        let out = "   System Locale: LANG=en_US.UTF-8\n     VC Keymap: us\n    X11 Layout: fr\n   X11 Variant: oss\n";
        let info = parse_localectl_output(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_localectl_layout_only_returns_no_variant() {
        let out = "    X11 Layout: us\n";
        let info = parse_localectl_output(out).unwrap();
        assert_eq!(info.layout, "us");
        assert!(info.variant.is_none());
    }

    #[test]
    fn parse_etc_default_keyboard_strips_quotes() {
        let out = "XKBMODEL=\"pc105\"\nXKBLAYOUT=\"fr\"\nXKBVARIANT=\"oss\"\nBACKSPACE=\"guess\"\n";
        let info = parse_etc_default_keyboard(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_etc_default_keyboard_handles_empty_variant() {
        let out = "XKBLAYOUT=\"us\"\nXKBVARIANT=\"\"\n";
        let info = parse_etc_default_keyboard(out).unwrap();
        assert_eq!(info.layout, "us");
        assert!(info.variant.is_none());
    }

    #[test]
    fn parse_kxkbrc_picks_first_layout_and_variant() {
        let out = "[Layout]\nLayoutList=fr,us\nVariantList=oss,\nModel=pc105\n";
        let info = parse_kxkbrc(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_kxkbrc_ignores_other_sections() {
        let out = "[Other]\nLayoutList=zz\n[Layout]\nLayoutList=de\nVariantList=\n";
        let info = parse_kxkbrc(out).unwrap();
        assert_eq!(info.layout, "de");
        assert!(info.variant.is_none());
    }

    #[test]
    fn extract_hyprctl_str_handles_str_key() {
        let json = r#"{"option":"input:kb_layout","str":"fr","set":true}"#;
        assert_eq!(extract_hyprctl_str(json).as_deref(), Some("fr"));
    }

    #[test]
    fn extract_hyprctl_str_handles_value_key() {
        let json = r#"{"option":"input:kb_layout","value":"de","set":true}"#;
        assert_eq!(extract_hyprctl_str(json).as_deref(), Some("de"));
    }

    #[test]
    fn extract_hyprctl_str_picks_first_when_comma_separated() {
        let json = r#"{"str":"fr,us"}"#;
        assert_eq!(extract_hyprctl_str(json).as_deref(), Some("fr"));
    }

    #[test]
    fn parse_swaymsg_uses_xkb_layout_names_array() {
        let out = r#"[{"type":"keyboard","xkb_active_layout_name":"French (alt.)","xkb_layout_names":["fr+oss","us"]}]"#;
        let info = parse_swaymsg_output(out).unwrap();
        assert_eq!(info.layout, "fr");
        assert_eq!(info.variant.as_deref(), Some("oss"));
    }

    #[test]
    fn parse_swaymsg_skips_non_keyboard_inputs() {
        let out = r#"[{"type":"pointer","xkb_layout_names":["zz"]},{"type":"keyboard","xkb_layout_names":["us"]}]"#;
        let info = parse_swaymsg_output(out).unwrap();
        assert_eq!(info.layout, "us");
    }

}
