use crate::shortcuts::types::ShortcutState;
use tauri::Manager;

pub fn force_stop_recording(app: &tauri::AppHandle) {
    // Reset toggle state
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);

    // Stop recording
    crate::audio::stop_recording(app);
}
