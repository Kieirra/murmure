use crate::shortcuts::helpers::keys_to_string;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::KeyEventType;
use log::warn;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn init(app: AppHandle) {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read().clone();

    for binding in registry.bindings {
        let keys_str = keys_to_string(&binding.keys);
        if let Ok(shortcut) = keys_str.parse::<Shortcut>() {
            let app_clone = app.clone();
            let action = binding.action.clone();
            let activation_mode = binding.activation_mode.clone();

            if let Err(e) = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                let event_type = match event.state() {
                    ShortcutState::Pressed => KeyEventType::Pressed,
                    ShortcutState::Released => KeyEventType::Released,
                };

                crate::shortcuts::handle_shortcut_event(
                    &app_clone,
                    &action,
                    &activation_mode,
                    event_type,
                );
            }) {
                warn!("Failed to register shortcut {:?}: {}", action, e);
            }
        }
    }
}
