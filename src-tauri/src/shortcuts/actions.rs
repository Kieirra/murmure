use crate::shortcuts::types::ShortcutState;
use tauri::{Emitter, Manager};

pub fn force_stop_recording(app: &tauri::AppHandle) {
    // Reset toggle state
    let shortcut_state = app.state::<ShortcutState>();
    shortcut_state.set_toggled(false);

    // Stop recording
    crate::audio::stop_recording(app);

    // Emit correct event to update UI
    let audio_state = app.state::<crate::audio::types::AudioState>();
    if audio_state.get_use_llm_shortcut() {
        let _ = app.emit("shortcut:llm-record-released", "");
    } else {
        let _ = app.emit("shortcut:stop", "");
    }
}
