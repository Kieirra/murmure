use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{ActivationMode, KeyEventType, ShortcutAction};
use log::warn;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn init(app: AppHandle) {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read().clone();

    for binding in registry.bindings {
        let keys_str = crate::shortcuts::helpers::keys_to_string(&binding.keys);
        if let Ok(shortcut) = keys_str.parse::<Shortcut>() {
            if let Err(e) = register_shortcut_with_action(
                &app,
                shortcut,
                binding.action.clone(),
                binding.activation_mode.clone(),
            ) {
                warn!("Failed to register shortcut {:?}: {}", binding.action, e);
            }
        }
    }
}

fn get_activation_mode(app: &AppHandle) -> ActivationMode {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read();
    registry
        .bindings
        .first()
        .map(|b| b.activation_mode.clone())
        .unwrap_or(ActivationMode::PushToTalk)
}

pub fn register_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let mode = get_activation_mode(app);
    register_shortcut_with_action(app, shortcut, ShortcutAction::StartRecording, mode)
}

pub fn register_llm_record_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let mode = get_activation_mode(app);
    register_shortcut_with_action(app, shortcut, ShortcutAction::StartRecordingLLM, mode)
}

pub fn register_command_shortcut(app: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
    let mode = get_activation_mode(app);
    register_shortcut_with_action(app, shortcut, ShortcutAction::StartRecordingCommand, mode)
}

pub fn register_last_transcript_shortcut(
    app: &AppHandle,
    shortcut: Shortcut,
) -> Result<(), String> {
    register_shortcut_with_action(
        app,
        shortcut,
        ShortcutAction::PasteLastTranscript,
        ActivationMode::PushToTalk,
    )
}

pub fn register_mode_switch_shortcut(
    app: &AppHandle,
    shortcut: Shortcut,
    mode_index: usize,
) -> Result<(), String> {
    register_shortcut_with_action(
        app,
        shortcut,
        ShortcutAction::SwitchLLMMode(mode_index),
        ActivationMode::PushToTalk,
    )
}

fn register_shortcut_with_action(
    app: &AppHandle,
    shortcut: Shortcut,
    action: ShortcutAction,
    mode: ActivationMode,
) -> Result<(), String> {
    let app_clone = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            let event_type = match event.state() {
                ShortcutState::Pressed => KeyEventType::Pressed,
                ShortcutState::Released => KeyEventType::Released,
            };
            crate::shortcuts::handle_shortcut_event(&app_clone, &action, &mode, event_type);
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))
}
