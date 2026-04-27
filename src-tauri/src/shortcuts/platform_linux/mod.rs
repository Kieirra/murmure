mod wayland;
mod x11;

use tauri::AppHandle;

use crate::utils::platform::is_wayland_session;

pub fn init(app: AppHandle, use_wayland_portal: bool) {
    if use_wayland_portal && is_wayland_session() {
        wayland::init(app);
    } else {
        x11::init(app);
    }
}
