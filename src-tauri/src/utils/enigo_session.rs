//! Shared `enigo::Enigo` instance.
//!
//! `Enigo` is lazily created on first use and kept alive for the whole app
//! lifetime via a Tauri-managed `Mutex`. Recreating it per keystroke on
//! Wayland (via the XDG RemoteDesktop portal) would re-prompt the user every
//! time — the shared instance keeps a single session open. The same
//! single-instance pattern also benefits macOS/Windows/Linux X11 (less
//! per-call setup, consistent modifier state).
//!
//! Note: with the default `x11rb` feature enigo on Linux uses the XTEST X11
//! path. That reaches XWayland clients under Wayland but does NOT reach
//! native Wayland windows. For those, Murmure writes directly to
//! `/dev/uinput` via `utils::wayland_inject` — no external daemon, no
//! shell-out.

use enigo::{Enigo, Settings};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct EnigoState(pub Mutex<Option<Enigo>>);

impl Default for EnigoState {
    fn default() -> Self {
        Self(Mutex::new(None))
    }
}

/// Run a closure with mutable access to the shared `Enigo`, creating it on
/// first use. The closure must be short-lived since it holds the mutex for
/// its duration; avoid doing any long async work inside.
pub fn with_enigo<F, R>(app: &AppHandle, f: F) -> Result<R, String>
where
    F: FnOnce(&mut Enigo) -> Result<R, String>,
{
    let state = app.state::<EnigoState>();
    let mut guard = state
        .0
        .lock()
        .map_err(|e| format!("EnigoState mutex poisoned: {}", e))?;

    if guard.is_none() {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;
        *guard = Some(enigo);
    }

    let enigo = guard
        .as_mut()
        .expect("Enigo was just initialised above if it was None");

    f(enigo)
}
