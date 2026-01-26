use crate::shortcuts::helpers::keys_to_string;
use crate::shortcuts::types::ShortcutRegistryState;
use log::warn;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

pub fn init(app: AppHandle) {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read().clone();

    for binding in registry.bindings {
        let keys_str = keys_to_string(&binding.keys);
        if let Ok(shortcut) = keys_str.parse::<Shortcut>() {
            let app_clone = app.clone();
            let action = binding.action.clone();
            let activation_mode = binding.activation_mode.clone();
            let keys = binding.keys.clone();

            if let Err(e) = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                crate::shortcuts::execute_action(&app_clone, &action, &activation_mode, &keys);
            }) {
                warn!("Failed to register shortcut {:?}: {}", action, e);
            }
        }
    }
}

pub fn register_shortcut(app: &AppHandle, shortcut: Shortcut, action: crate::shortcuts::types::ShortcutAction, mode: crate::shortcuts::types::ActivationMode) -> Result<(), String> {
    let app_clone = app.clone();
    let keys = vec![];

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, _event| {
            crate::shortcuts::execute_action(&app_clone, &action, &mode, &keys);
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;
    Ok(())
}
