use std::sync::LazyLock;

static IS_WAYLAND: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("XDG_SESSION_TYPE")
        .map(|v| v == "wayland")
        .unwrap_or(false)
        || std::env::var("WAYLAND_DISPLAY").is_ok()
});

pub fn is_wayland() -> bool {
    *IS_WAYLAND
}
