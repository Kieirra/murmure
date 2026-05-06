mod wayland;
mod x11;

use log::info;
use tauri::AppHandle;

use crate::utils::platform::is_wayland_session;

pub fn init(app: AppHandle, use_wayland_portal: bool) {
    if is_wayland_session() {
        if use_wayland_portal {
            wayland::init(app);
        } else {
            // CLI mode: user binds shortcuts at OS level, Murmure stays passive.
            info!("Wayland CLI mode: no shortcuts registered by Murmure");
        }
    } else {
        x11::init(app);
    }
}
