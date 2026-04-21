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
    // Reason: on X11 / Unknown sessions, force GTK to the X11 backend so the
    // rdev shortcut listener and enigo key injection keep working reliably.
    // On Wayland we let GTK pick its native backend so WebKit can render
    // without going through XWayland and the Phase 2 portal shortcuts take
    // over automatically via our session-based routing.
    // Respect an explicit override so power users can debug with a different
    // backend (e.g. GDK_BACKEND=wayland ./murmure).
    if !murmure_lib::is_wayland_session() && std::env::var_os("GDK_BACKEND").is_none() {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    // Reason: WebKit2GTK's dmabuf renderer fails on many hybrid GPU stacks
    // (Intel + Nvidia) under both X11 and Wayland, producing a blank window
    // (issue #294). The software fallback renders reliably everywhere at a
    // small compositing perf cost. If the UI is still blank, users can set
    // WEBKIT_DISABLE_COMPOSITING_MODE=1 externally (documented in the build
    // checklist); we do not force it here because it disables accelerated
    // compositing entirely. Respect an explicit override so users can
    // re-enable dmabuf for testing.
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    // Reason: 0-don/global-hotkey reads this on its worker thread to register
    // with the XDG GlobalShortcuts portal. Must be set here, before any Tauri
    // thread is spawned, because std::env::set_var is not thread-safe.
    if std::env::var_os("GLOBAL_HOTKEY_APP_ID").is_none() {
        std::env::set_var("GLOBAL_HOTKEY_APP_ID", "com.al1x-ai.murmure");
    }
}
