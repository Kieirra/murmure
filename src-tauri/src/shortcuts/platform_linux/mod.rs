mod wayland;
mod x11;

use tauri::AppHandle;

use crate::utils::platform::use_wayland_portal_shortcuts;

pub fn init(app: AppHandle) {
    if use_wayland_portal_shortcuts() {
        wayland::init(app);
    } else {
        x11::init(app);
    }
}
