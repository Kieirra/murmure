// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    if murmure_lib::cli::try_handle_early_args() {
        return;
    }

    #[cfg(target_os = "linux")]
    setup_linux_env();

    murmure_lib::run()
}

#[cfg(target_os = "linux")]
fn setup_linux_env() {
    // Blank window on hybrid GPUs (#294).
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    // Must match the installed `.desktop` basename. Set before any
    // Tauri thread spawns. `std::env::set_var` is not thread-safe.
    if std::env::var_os("GLOBAL_HOTKEY_APP_ID").is_none() {
        std::env::set_var("GLOBAL_HOTKEY_APP_ID", "com.al1x-ai.murmure");
    }

    // Force XWayland when the user opts out of the portal, so rdev can capture keys.
    if murmure_lib::is_wayland_session()
        && !read_use_wayland_portal_or_default()
        && std::env::var_os("GDK_BACKEND").is_none()
    {
        std::env::set_var("GDK_BACKEND", "x11");
    }
}

// Pre-Tauri read of `use_wayland_portal` from settings.json, falls back to default.
#[cfg(target_os = "linux")]
fn read_use_wayland_portal_or_default() -> bool {
    let default = murmure_lib::is_wayland_session();

    let base = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share")));
    let Some(base) = base else { return default };

    let path = base.join("com.al1x-ai.murmure").join("settings.json");
    let Ok(content) = std::fs::read_to_string(&path) else {
        return default;
    };

    let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
        return default;
    };
    value
        .get("use_wayland_portal")
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}
