//! Process-wide keymap state used by the Direct paste path. Compiling
//! an XKB keymap costs ~10 ms, so we do it once at setup and again only
//! when the user re-enters Direct mode, never on every paste.

use super::char_map::build_char_map;
use super::layout_detect::detect_layout;
use super::types::{CharMap, CharStrokes, LayoutInfo};
use log::{debug, error, info, warn};
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
            warn!(
                "wayland_xkb: failed to emit wayland-layout-fallback: {}",
                err
            );
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

// libxkbcommon can return a keymap that produces no printable char at
// all (unknown layout on some versions). Only the injected control keys
// survive, and Direct mode would type nothing, so treat it as a failure
// and let the caller retry with US.
fn compile_usable(info: &LayoutInfo) -> Result<CharMap, String> {
    let cm = build_char_map(info)?;
    if cm.map.keys().all(|c| c.is_control()) {
        return Err(format!("keymap for {:?} maps no typable char", info));
    }
    Ok(cm)
}

fn compile_with_fallback() -> Result<(CharMap, Option<&'static str>), String> {
    let detected = detect_layout();
    let detection_failed = detected.used_fallback;
    match compile_usable(&detected.layout) {
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
                "wayland_xkb: unusable XKB keymap for {:?}: {}, retrying with US fallback",
                detected.layout, e
            );
            let fallback = LayoutInfo::us_fallback();
            match compile_usable(&fallback) {
                Ok(mut cm) => {
                    cm.is_fallback = true;
                    cm.fallback_reason = Some("compile_failed");
                    Ok((cm, Some("compile_failed")))
                }
                Err(e2) => {
                    warn!(
                        "wayland_xkb: unusable XKB keymap even for the US fallback: {}, Direct mode has no char map and will skip every char",
                        e2
                    );
                    Err(e2)
                }
            }
        }
    }
}

// Returns `None` when the layout cannot produce the char or the
// keymap is not ready (init failed and no successful retry occurred).
pub fn lookup(c: char) -> Option<CharStrokes> {
    let slot = CHAR_MAP.get()?;
    let guard = slot.lock().ok()?;
    let cm = guard.as_ref()?;
    cm.map.get(&c).copied()
}

// Counts and logs here rather than in `wayland_inject::type_text`:
// untypable chars are already gone by the time the text reaches it.
// Only counts are logged, never any part of the text.
pub fn resolve_for_typing(text: &str) -> String {
    let (resolved, dropped) = super::normalize::resolve_for_typing(text, |c| lookup(c).is_some());

    if dropped > 0 {
        debug!(
            "wayland_xkb: {} of {} char(s) dropped, neither typable nor foldable in the active layout",
            dropped,
            text.chars().count()
        );
    }

    if resolved.is_empty() && !text.is_empty() {
        match current_layout_label() {
            Some(label) => warn!(
                "wayland_xkb: layout {} can type none of the {} char(s) submitted, Direct mode types nothing",
                label,
                text.chars().count()
            ),
            None => error!(
                "wayland_xkb: no char map available (XKB keymap compilation failed at startup), Direct mode types nothing and skips all {} char(s)",
                text.chars().count()
            ),
        }
    }

    resolved
}

fn current_layout_label() -> Option<String> {
    let slot = CHAR_MAP.get()?;
    let guard = slot.lock().ok()?;
    Some(format_layout_label(&guard.as_ref()?.layout))
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

    // Both libxkbcommon behaviours for an unknown layout (null keymap,
    // or a keymap holding only the control keys) must be rejected so
    // `compile_with_fallback` retries with US.
    #[test]
    fn compile_usable_rejects_a_layout_without_typable_chars() {
        let info = LayoutInfo::new("zz_definitely_not_a_real_layout".to_string(), None);
        assert!(compile_usable(&info).is_err());
    }

    #[test]
    fn compile_usable_accepts_the_us_fallback() {
        let cm = compile_usable(&LayoutInfo::us_fallback()).expect("US keymap must compile");
        assert!(cm.map.contains_key(&'a'));
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
