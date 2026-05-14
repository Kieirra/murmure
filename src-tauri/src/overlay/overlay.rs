use crate::formatting_rules::highlighter::HighlightRange;
use crate::settings;
use enigo::Mouse;
use log::{debug, error, warn};
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindowBuilder};

#[cfg(target_os = "linux")]
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
#[cfg(target_os = "linux")]
use std::sync::atomic::AtomicBool;

#[derive(Serialize)]
struct EmptyStreamingTranscript {
    text: String,
    highlights: Vec<HighlightRange>,
}

// Cold-start handoff: the webview consumes this on mount when the
// flash shortcut fires before the overlay window exists. Ignored on
// the hot path (the `mode-flash` event is emitted directly).
#[derive(Default)]
pub struct PendingFlashState(pub Mutex<Option<String>>);

const OVERLAY_HEIGHT: f64 = 200.0;
const OVERLAY_WIDTH: f64 = 350.0;
const OVERLAY_TOP_OFFSET_PCT: f64 = 0.05;
const OVERLAY_BOTTOM_OFFSET_PCT: f64 = 0.05;

// Read by the clipboard paste path via `millis_since_last_overlay_hide`
// to decide whether KWin still needs extra time to restore focus.
static OVERLAY_LAST_HIDE_MS: AtomicU64 = AtomicU64::new(0);

// Reset on every destroy to mirror the actual GTK window lifecycle.
#[cfg(target_os = "linux")]
static GTK_LAYER_SHELL_ACTIVE: AtomicBool = AtomicBool::new(false);

fn is_layer_shell_active() -> bool {
    #[cfg(target_os = "linux")]
    { GTK_LAYER_SHELL_ACTIVE.load(Ordering::Relaxed) }
    #[cfg(not(target_os = "linux"))]
    { false }
}

// 32 px approximates the 5% offset used by the Tauri-native fallback.
#[cfg(target_os = "linux")]
const OVERLAY_LAYER_SHELL_MARGIN_PX: i32 = 32;

#[cfg(target_os = "linux")]
fn is_gtk_layer_shell_disabled() -> bool {
    match std::env::var("MURMURE_NO_GTK_LAYER_SHELL") {
        Ok(value) => {
            let v = value.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "yes"
        }
        Err(_) => false,
    }
}

// Left/Right anchors both false: the compositor centres the surface horizontally.
#[cfg(target_os = "linux")]
fn apply_gtk_layer_shell_anchors(overlay_window: &tauri::WebviewWindow) {
    let gtk_window = match overlay_window.gtk_window() {
        Ok(w) => w,
        Err(e) => {
            warn!("Could not retrieve GTK window for anchor update: {}", e);
            return;
        }
    };
    let s = settings::load_settings(overlay_window.app_handle());
    let anchor_top = s.overlay_position == "top";
    gtk_window.set_anchor(Edge::Top, anchor_top);
    gtk_window.set_anchor(Edge::Bottom, !anchor_top);
    gtk_window.set_anchor(Edge::Left, false);
    gtk_window.set_anchor(Edge::Right, false);
    // Reset inactive edge to avoid stale margin on toggle.
    let (top_margin, bottom_margin) = if anchor_top {
        (OVERLAY_LAYER_SHELL_MARGIN_PX, 0)
    } else {
        (0, OVERLAY_LAYER_SHELL_MARGIN_PX)
    };
    gtk_window.set_layer_shell_margin(Edge::Top, top_margin);
    gtk_window.set_layer_shell_margin(Edge::Bottom, bottom_margin);
}

// Must be called on the GTK main thread.
#[cfg(target_os = "linux")]
fn init_gtk_layer_shell(overlay_window: &tauri::WebviewWindow) -> bool {
    if is_gtk_layer_shell_disabled() {
        debug!("gtk-layer-shell disabled via MURMURE_NO_GTK_LAYER_SHELL");
        return false;
    }
    if !gtk_layer_shell::is_supported() {
        debug!("gtk-layer-shell not supported by this compositor, using Tauri native overlay");
        return false;
    }
    let gtk_window = match overlay_window.gtk_window() {
        Ok(w) => w,
        Err(e) => {
            warn!(
                "Could not retrieve GTK window for overlay, falling back: {}",
                e
            );
            return false;
        }
    };
    gtk_window.init_layer_shell();
    gtk_window.set_layer(Layer::Overlay);
    gtk_window.set_keyboard_mode(KeyboardMode::None);
    // Overlay other windows without pushing them away.
    gtk_window.set_exclusive_zone(0);
    apply_gtk_layer_shell_anchors(overlay_window);
    true
}

fn now_unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// Returns `u64::MAX` when the overlay has never been hidden this
// session so callers can use a plain `< threshold` comparison.
pub fn millis_since_last_overlay_hide() -> u64 {
    let hide = OVERLAY_LAST_HIDE_MS.load(Ordering::Relaxed);
    if hide == 0 {
        u64::MAX
    } else {
        now_unix_millis().saturating_sub(hide)
    }
}

fn get_cursor_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    // Wayland hides cursor coords from non-privileged clients, skip
    // the probe and let get_active_monitor pick the primary monitor.
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            return None;
        }
    }

    let mouse_location = crate::utils::enigo_session::with_enigo(app_handle, |enigo| {
        enigo.location().map_err(|e| e.to_string())
    })
    .ok()?;

    let monitors = match app_handle.available_monitors() {
        Ok(m) => m,
        Err(_) => return None,
    };
    monitors
        .into_iter()
        .find(|monitor| is_mouse_within_monitor(mouse_location, monitor.position(), monitor.size()))
}

fn get_active_monitor(app_handle: &AppHandle) -> Option<tauri::Monitor> {
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            // Wayland blocks cursor-position probing; anchor to the
            // main window's monitor instead.
            if let Some(main_window) = app_handle.get_webview_window("main") {
                if let Ok(Some(monitor)) = main_window.current_monitor() {
                    return Some(monitor);
                }
            }
            return app_handle
                .available_monitors()
                .ok()
                .and_then(|m| m.into_iter().next());
        }
    }

    get_cursor_monitor(app_handle)
        .or_else(|| app_handle.primary_monitor().ok().flatten())
        .or_else(|| {
            warn!("No cursor or primary monitor found, using first available monitor");
            app_handle
                .available_monitors()
                .ok()
                .and_then(|m| m.into_iter().next())
        })
}

fn is_mouse_within_monitor(
    mouse_pos: (i32, i32),
    monitor_pos: &PhysicalPosition<i32>,
    monitor_size: &PhysicalSize<u32>,
) -> bool {
    let (mouse_x, mouse_y) = mouse_pos;
    let PhysicalPosition {
        x: monitor_x,
        y: monitor_y,
    } = *monitor_pos;
    let PhysicalSize {
        width: monitor_width,
        height: monitor_height,
    } = *monitor_size;
    mouse_x >= monitor_x
        && mouse_x < (monitor_x + monitor_width as i32)
        && mouse_y >= monitor_y
        && mouse_y < (monitor_y + monitor_height as i32)
}

fn calculate_overlay_geometry(app_handle: &AppHandle) -> Option<(i32, i32, u32, u32)> {
    if let Some(monitor) = get_active_monitor(app_handle) {
        let monitor_size = monitor.size();
        let monitor_pos = monitor.position();
        let scale = monitor.scale_factor();

        let work_w = monitor_size.width as f64;
        let work_h = monitor_size.height as f64;
        let work_x = monitor_pos.x as f64;
        let work_y = monitor_pos.y as f64;

        let s = settings::load_settings(app_handle);
        let overlay_w = OVERLAY_WIDTH.max(s.streaming_text_width as f64) * scale;
        let overlay_h = OVERLAY_HEIGHT * scale;

        let x = work_x + (work_w - overlay_w) / 2.0;
        let y = match s.overlay_position.as_str() {
            "top" => work_y + work_h * OVERLAY_TOP_OFFSET_PCT,
            _ => work_y + work_h * (1.0 - OVERLAY_BOTTOM_OFFSET_PCT) - overlay_h,
        };
        return Some((x as i32, y as i32, overlay_w as u32, overlay_h as u32));
    }
    None
}

pub fn create_recording_overlay(app_handle: &AppHandle) {
    let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) else {
        warn!("Could not determine overlay geometry (no monitor found), skipping overlay creation");
        return;
    };
    let res = WebviewWindowBuilder::new(
        app_handle,
        "recording_overlay",
        tauri::WebviewUrl::App("src/overlay/index.html".into()),
    )
    .title("Recording")
    .resizable(false)
    .shadow(false)
    .maximizable(false)
    .minimizable(false)
    .closable(false)
    .accept_first_mouse(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .transparent(true)
    .focusable(false)
    .visible(false)
    .build();
    match res {
        Ok(window) => {
            let _ =
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: w,
                height: h,
            }));
            #[cfg(target_os = "linux")]
            {
                let active = init_gtk_layer_shell(&window);
                GTK_LAYER_SHELL_ACTIVE.store(active, Ordering::Relaxed);
                if active {
                    debug!("Recording overlay initialised with gtk-layer-shell");
                }
            }
            debug!("Recording overlay window created (hidden)");
        }
        Err(e) => {
            error!("Failed to create recording overlay window: {}", e);
        }
    }
}

fn ensure_overlay(app_handle: &AppHandle) {
    if app_handle.get_webview_window("recording_overlay").is_none() {
        create_recording_overlay(app_handle);
    }
}

// Windows/macOS: keep the hidden overlay alive to skip WebView2/WebKit
// 200-400ms cold-start every recording.
// Linux/GTK: destroy after warmup, stale-frame artifacts otherwise.
pub fn warmup_overlay(app_handle: &AppHandle) {
    create_recording_overlay(app_handle);
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        if let Err(e) = window.destroy() {
            warn!("recording_overlay destroy during warmup failed: {}", e);
        }
        // The destroy drops the GTK surface that init_gtk_layer_shell just
        // configured; reset the flag so the next create_recording_overlay
        // observes the real state.
        #[cfg(target_os = "linux")]
        GTK_LAYER_SHELL_ACTIVE.store(false, Ordering::Relaxed);
    }
}

fn present_recording_overlay(app_handle: &AppHandle) {
    update_overlay_position(app_handle);
    let Some(window) = app_handle.get_webview_window("recording_overlay") else {
        warn!("recording_overlay window not found on present_recording_overlay");
        return;
    };
    let state = app_handle.state::<crate::audio::types::AudioState>();
    let mode_str = match state.get_recording_mode() {
        crate::audio::types::RecordingMode::Standard => "standard",
        crate::audio::types::RecordingMode::Llm => "llm",
        crate::audio::types::RecordingMode::Command => "command",
    };
    let _ = window.emit("recording-mode", mode_str);
    let _ = window.emit(
        "streaming-transcript",
        &EmptyStreamingTranscript {
            text: String::new(),
            highlights: vec![],
        },
    );
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_ignore_cursor_events(true);
}

pub fn show_recording_overlay(app_handle: &AppHandle) {
    // Skip the destroy/recreate dance when gtk-layer-shell is active:
    // the compositor manages mapping cleanly and we'd lose init state.
    // Otherwise (X11 fallback), destroy to avoid stale GTK transparent frames.
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    if !is_layer_shell_active() {
        if let Some(window) = app_handle.get_webview_window("recording_overlay") {
            if let Err(e) = window.destroy() {
                warn!("recording_overlay destroy before show failed: {}", e);
            }
            #[cfg(target_os = "linux")]
            GTK_LAYER_SHELL_ACTIVE.store(false, Ordering::Relaxed);
        }
    }

    // Always dispatch to main thread to avoid GTK threading assertions (SIGABRT)
    // when called from the shortcut handler thread.
    let app_for_thread = app_handle.clone();
    std::thread::spawn(move || {
        let app_for_main = app_for_thread.clone();
        if let Err(e) = app_for_thread.run_on_main_thread(move || {
            present_recording_overlay(&app_for_main);
        }) {
            error!("recording_overlay show scheduling failed: {}", e);
        }
    });
}

// Reuse the existing window on the hot path. Destroying+recreating
// per press caused a visible double-overlay flicker. React handles
// auto-hide via hide_overlay_if_idle.
pub fn flash_text_in_overlay_internal(app: &AppHandle, text: String) {
    if let Some(window) = app.get_webview_window("recording_overlay") {
        let app_clone = app.clone();
        let _ = app.run_on_main_thread(move || {
            let _ = window.emit("mode-flash", &text);
            let _ = window.show();
            let _ = window.set_always_on_top(true);
            let _ = window.set_ignore_cursor_events(true);
            update_overlay_position(&app_clone);
        });
        return;
    }

    // Cold start: no window yet. Stage the payload so the webview can pick
    // it up at mount via `consume_pending_mode_flash`.
    {
        let state = app.state::<PendingFlashState>();
        *state.0.lock() = Some(text);
    }
    show_recording_overlay(app);
}

pub fn update_overlay_position(app_handle: &AppHandle) {
    ensure_overlay(app_handle);

    #[cfg(target_os = "linux")]
    {
        if GTK_LAYER_SHELL_ACTIVE.load(Ordering::Relaxed) {
            if let Some(window) = app_handle.get_webview_window("recording_overlay") {
                // Refresh anchors in case overlay_position changed since
                // window creation. Size still needs Tauri because
                // layer-shell does not size the surface.
                let win_for_main = window.clone();
                let _ = app_handle.run_on_main_thread(move || {
                    apply_gtk_layer_shell_anchors(&win_for_main);
                });
                if let Some((_, _, w, h)) = calculate_overlay_geometry(app_handle) {
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                        width: w,
                        height: h,
                    }));
                }
                return;
            }
        }
    }

    if let Some((x, y, w, h)) = calculate_overlay_geometry(app_handle) {
        if let Some(window) = app_handle.get_webview_window("recording_overlay") {
            let _ =
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: w,
                height: h,
            }));
        }
    }
}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("recording_overlay") {
        // Windows/macOS: keep alive (hidden) so next show is instant.
        // layer-shell: hide to preserve init state across sessions.
        // X11 fallback: destroy to avoid stale GTK transparent frames.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        if let Err(e) = window.hide() {
            warn!("recording_overlay hide failed: {}", e);
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            if is_layer_shell_active() {
                if let Err(e) = window.hide() {
                    warn!("recording_overlay hide failed: {}", e);
                }
            } else {
                if let Err(e) = window.destroy() {
                    warn!("recording_overlay destroy on hide failed: {}", e);
                }
                #[cfg(target_os = "linux")]
                GTK_LAYER_SHELL_ACTIVE.store(false, Ordering::Relaxed);
            }
        }
        // Timestamp for the paste path; `.max(1)` keeps 0 reserved
        // as the "never hidden" sentinel.
        OVERLAY_LAST_HIDE_MS.store(now_unix_millis().max(1), Ordering::Relaxed);
    } else {
        debug!("recording_overlay already absent on hide_recording_overlay");
    }
    clear_pending_flash(app_handle);
}

pub fn clear_pending_flash(app_handle: &AppHandle) {
    *app_handle.state::<PendingFlashState>().0.lock() = None;
}
