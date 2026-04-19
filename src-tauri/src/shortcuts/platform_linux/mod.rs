mod wayland;
mod x11;

use tauri::AppHandle;

use crate::utils::platform::is_wayland_session;

pub fn init(app: AppHandle) {
    if is_wayland_session() {
        wayland::init(app);
    } else {
        x11::init(app);
    }
}
