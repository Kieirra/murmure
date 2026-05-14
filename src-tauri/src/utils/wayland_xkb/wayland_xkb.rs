//! Process-wide keymap state used by the Direct paste path. Compiling
//! an XKB keymap costs ~10 ms, so we do it once at setup and again only
//! when the user re-enters Direct mode, never on every paste.

use super::char_map::build_char_map;
use super::layout_detect::detect_layout;
use super::types::{CharMap, KeyMapping, LayoutInfo};
use log::{info, warn};
use serde::Serialize;
use std::sync::{Mutex, OnceLock};
use tauri::Emitter;

static CHAR_MAP: OnceLock<Mutex<Option<CharMap>>> = OnceLock::new();

#[derive(Serialize, Clone)]
pub struct LayoutFallbackPayload {
    pub layout: String,
    pub variant: Option<String>,
    pub reason: &'static str,
}

fn slot() -> &'static Mutex<Option<CharMap>> {
    CHAR_MAP.get_or_init(|| Mutex::new(None))
}

// Emits `wayland-layout-fallback` when detection or compilation falls
// back to US.
pub fn init_char_map(app: &tauri::AppHandle) -> Result<(), String> {
    let payload = compile_and_store("ready")?;
    if let Some(payload) = payload {
        if let Err(err) = app.emit("wayland-layout-fallback", payload) {
            warn!("wayland_xkb: failed to emit wayland-layout-fallback: {}", err);
        }
    }
    Ok(())
}

// Picks up a system-level layout change without restarting Murmure
// when the user (re-)selects Direct mode.
pub fn recompile() -> Result<(), String> {
    compile_and_store("recompiled")?;
    Ok(())
}

// `phase` is embedded in the log so init vs recompile stay
// distinguishable. Returns the fallback payload so callers can decide
// whether to emit a UI event.
fn compile_and_store(phase: &str) -> Result<Option<LayoutFallbackPayload>, String> {
    let (char_map, reason) = compile_with_fallback()?;
    let layout_label = format_layout_label(&char_map.layout);
    let mapped_count = char_map.map.len();
    let is_fallback = char_map.is_fallback;
    let layout_for_payload = char_map.layout.clone();

    store(char_map);

    info!(
        "wayland_xkb: char map {} for layout {} ({} chars mapped, fallback={})",
        phase, layout_label, mapped_count, is_fallback
    );

    Ok(reason.map(|reason| LayoutFallbackPayload {
        layout: layout_for_payload.layout.clone(),
        variant: layout_for_payload.variant.clone(),
        reason,
    }))
}

fn compile_with_fallback() -> Result<(CharMap, Option<&'static str>), String> {
    let detected = detect_layout();
    let detection_failed = detected.used_fallback;
    match build_char_map(&detected.layout) {
        Ok(mut cm) => {
            cm.is_fallback = detection_failed;
            let reason = if detection_failed {
                Some("detection_failed")
            } else {
                None
            };
            cm.fallback_reason = reason;
            Ok((cm, reason))
        }
        Err(e) => {
            warn!(
                "wayland_xkb: XKB keymap compilation failed for {:?}: {}, retrying with US fallback",
                detected.layout, e
            );
            let fallback = LayoutInfo::us_fallback();
            match build_char_map(&fallback) {
                Ok(mut cm) => {
                    cm.is_fallback = true;
                    cm.fallback_reason = Some("compile_failed");
                    Ok((cm, Some("compile_failed")))
                }
                Err(e2) => {
                    warn!(
                        "wayland_xkb: XKB keymap compilation failed even for US fallback: {}, direct mode will always fall back to clipboard",
                        e2
                    );
                    Err(e2)
                }
            }
        }
    }
}

// Returns `None` when the char is outside the compiled subset or the
// keymap is not ready (init failed and no successful retry occurred).
pub fn lookup(c: char) -> Option<KeyMapping> {
    let slot = CHAR_MAP.get()?;
    let guard = slot.lock().ok()?;
    let cm = guard.as_ref()?;
    cm.map.get(&c).copied()
}

// Lets the UI rehydrate the fallback badge: the `wayland-layout-fallback`
// event fires once during setup and is lost if Settings mounts after.
pub fn current_fallback_payload() -> Option<LayoutFallbackPayload> {
    let slot = CHAR_MAP.get()?;
    let guard = slot.lock().ok()?;
    payload_from_char_map(guard.as_ref()?)
}

// Extracted so the conversion is unit-testable without poking the
// process-wide `CHAR_MAP` static.
fn payload_from_char_map(cm: &CharMap) -> Option<LayoutFallbackPayload> {
    if !cm.is_fallback {
        return None;
    }
    let reason = cm.fallback_reason?;
    Some(LayoutFallbackPayload {
        layout: cm.layout.layout.clone(),
        variant: cm.layout.variant.clone(),
        reason,
    })
}

fn store(cm: CharMap) {
    let s = slot();
    if let Ok(mut guard) = s.lock() {
        *guard = Some(cm);
    } else {
        warn!("wayland_xkb: char map mutex poisoned, keymap state may be inconsistent");
    }
}

fn format_layout_label(info: &LayoutInfo) -> String {
    match info.variant.as_deref() {
        Some(v) => format!("{}+{}", info.layout, v),
        None => info.layout.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_char_map(is_fallback: bool, fallback_reason: Option<&'static str>) -> CharMap {
        CharMap {
            layout: LayoutInfo::new("fr".to_string(), Some("oss".to_string())),
            map: HashMap::new(),
            is_fallback,
            fallback_reason,
        }
    }

    #[test]
    fn payload_none_when_not_fallback() {
        let cm = make_char_map(false, None);
        assert!(payload_from_char_map(&cm).is_none());
    }

    #[test]
    fn payload_none_when_fallback_without_reason() {
        let cm = make_char_map(true, None);
        assert!(payload_from_char_map(&cm).is_none());
    }

    #[test]
    fn payload_some_mirrors_event_shape_on_detection_failed() {
        let cm = make_char_map(true, Some("detection_failed"));
        let payload = payload_from_char_map(&cm).expect("payload should be Some");
        assert_eq!(payload.layout, "fr");
        assert_eq!(payload.variant.as_deref(), Some("oss"));
        assert_eq!(payload.reason, "detection_failed");
    }

}
