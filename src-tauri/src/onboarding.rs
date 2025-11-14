use once_cell::sync::Lazy;
use parking_lot::Mutex;
use tauri::{AppHandle, Manager};

/// Snapshot of whether the main window was focused at the START of recording.
/// We store this at shortcut press time and consume it when history is written.
static RECORD_FOCUS_AT_START: Lazy<Mutex<Option<bool>>> =
    Lazy::new(|| Mutex::new(None));

pub fn set_record_focus_at_start(_app: &AppHandle, focused: bool) {
    *RECORD_FOCUS_AT_START.lock() = Some(focused);
}

pub fn record_focus_at_start_take() -> Option<bool> {
    RECORD_FOCUS_AT_START.lock().take()
}

/// Helper: snapshot main window focus and store it as \"record start\" state.
pub fn capture_focus_at_record_start(app: &AppHandle) {
    let focused = app
        .get_webview_window("main")
        .map(|w| w.is_focused().unwrap_or(false))
        .unwrap_or(false);
    set_record_focus_at_start(app, focused);
}

/// Mark onboarding flags when a transcription has been written to history.
/// Uses the focus snapshot captured at record start (if present).
/// Falls back to current focus status if no snapshot is available.
/// Special handling for Windows: if no onboarding task is completed yet,
/// assume the user is using the app normally (focused) to avoid false positives.
pub fn mark_onboarding_on_history_write(app: &AppHandle) {
    let mut s = crate::settings::load_settings(app);
    let mut changed = false;

    // Check if onboarding has started (any task completed)
    let onboarding_started = s.onboarding.used_home_shortcut
        || s.onboarding.transcribed_outside_app
        || s.onboarding.added_dictionary_word;

    let should_mark_home_shortcut = match record_focus_at_start_take() {
        Some(focused) => focused,
        None => {
            if let Some(win) = app.get_webview_window("main") {
                win.is_focused().unwrap_or(false)
            } else {
                false
            }
        }
    };

    // Special case for Windows: if onboarding hasn't started yet,
    // assume the user is using the app normally (focused) to prevent
    // marking "transcribed_outside_app" incorrectly on first use
    let should_mark_home_shortcut = if !onboarding_started && !should_mark_home_shortcut {
        #[cfg(target_os = "windows")]
        {
            true // On Windows, assume focused if onboarding hasn't started
        }
        #[cfg(not(target_os = "windows"))]
        {
            should_mark_home_shortcut
        }
    } else {
        should_mark_home_shortcut
    };

    if should_mark_home_shortcut {
        if !s.onboarding.used_home_shortcut {
            s.onboarding.used_home_shortcut = true;
            changed = true;
        }
    } else {
        if !s.onboarding.transcribed_outside_app {
            s.onboarding.transcribed_outside_app = true;
            changed = true;
        }
    }

    if changed {
        let _ = crate::settings::save_settings(app, &s);
    }
}



