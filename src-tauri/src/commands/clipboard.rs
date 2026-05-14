use crate::settings;
use crate::settings::PasteMethod;
use tauri::{command, AppHandle};

#[cfg(target_os = "linux")]
use crate::utils::wayland_xkb::LayoutFallbackPayload;

#[command]
pub fn set_copy_to_clipboard(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.copy_to_clipboard = enabled;
    settings::save_settings(&app, &s)
}

#[command]
pub fn set_paste_method(app: AppHandle, method: String) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.paste_method = match method.to_lowercase().as_str() {
        "ctrl_shift_v" | "ctrlshiftv" => PasteMethod::CtrlShiftV,
        "direct" => PasteMethod::Direct,
        _ => PasteMethod::CtrlV,
    };
    settings::save_settings(&app, &s)?;

    // Toggling Direct off/on lets the user pick up a system-level
    // layout change without restarting Murmure.
    #[cfg(target_os = "linux")]
    if s.paste_method == PasteMethod::Direct && crate::utils::platform::is_wayland_session() {
        if let Err(e) = crate::utils::wayland_xkb::recompile() {
            log::warn!("set_paste_method: wayland_xkb recompile failed: {}", e);
        }
    }
    Ok(())
}

// Lets the Settings page rehydrate the fallback badge: the
// `wayland-layout-fallback` event fires once during setup and is lost
// if Settings mounts later.
#[cfg(target_os = "linux")]
#[command]
pub fn get_layout_fallback_state() -> Option<LayoutFallbackPayload> {
    crate::utils::wayland_xkb::current_fallback_payload()
}

#[cfg(not(target_os = "linux"))]
#[command]
pub fn get_layout_fallback_state() -> Option<()> {
    None
}
