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
    let xwayland_fallback = murmure_lib::is_wayland_session()
        && !murmure_lib::portal_shortcuts_likely_functional();
    let force_x11 = xwayland_fallback || !murmure_lib::is_wayland_session();

    if force_x11 && std::env::var_os("GDK_BACKEND").is_none() {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    // Blank window on hybrid GPUs (#294).
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    // XWayland fallback WebKit freeze workarounds.
    if xwayland_fallback {
        if std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none() {
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
        if std::env::var_os("LIBGL_ALWAYS_SOFTWARE").is_none() {
            std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
        }
    }

    // Must match the installed `.desktop` basename. Set before any
    // Tauri thread spawns — `std::env::set_var` is not thread-safe.
    if std::env::var_os("GLOBAL_HOTKEY_APP_ID").is_none() {
        std::env::set_var("GLOBAL_HOTKEY_APP_ID", "com.al1x-ai.murmure");
    }
}
