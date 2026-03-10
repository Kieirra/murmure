use std::sync::OnceLock;

static IS_WAYLAND: OnceLock<bool> = OnceLock::new();

pub fn is_wayland_session() -> bool {
    *IS_WAYLAND.get_or_init(|| {
        let session_type = std::env::var("XDG_SESSION_TYPE")
            .unwrap_or_default()
            .to_lowercase();
        let wayland_display = std::env::var("WAYLAND_DISPLAY").is_ok();

        let is_wayland = session_type == "wayland" || wayland_display;

        if is_wayland {
            log::info!("Session type detected: Wayland");
        } else {
            log::info!("Session type detected: X11");
        }

        is_wayland
    })
}
