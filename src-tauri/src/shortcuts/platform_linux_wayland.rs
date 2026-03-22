use global_hotkey::wayland::WlNewHotKeyAction;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{ActivationMode, KeyEventType, ShortcutAction, ShortcutState};

const ACTION_ID_START_RECORDING: u32 = 1;
const ACTION_ID_START_RECORDING_LLM: u32 = 2;
const ACTION_ID_START_RECORDING_COMMAND: u32 = 3;
const ACTION_ID_PASTE_LAST_TRANSCRIPT: u32 = 4;
const ACTION_ID_CANCEL_RECORDING: u32 = 5;
const ACTION_ID_SWITCH_LLM_MODE_1: u32 = 6;
const ACTION_ID_SWITCH_LLM_MODE_2: u32 = 7;
const ACTION_ID_SWITCH_LLM_MODE_3: u32 = 8;
const ACTION_ID_SWITCH_LLM_MODE_4: u32 = 9;

fn build_actions(registry_state: &ShortcutRegistryState) -> Vec<WlNewHotKeyAction> {
    let registry = registry_state.0.read();
    let mut actions = Vec::new();

    for binding in &registry.bindings {
        let (id, description) = match &binding.action {
            ShortcutAction::StartRecording => {
                (ACTION_ID_START_RECORDING, "Start recording")
            }
            ShortcutAction::StartRecordingLLM => {
                (ACTION_ID_START_RECORDING_LLM, "Start LLM recording")
            }
            ShortcutAction::StartRecordingCommand => {
                (ACTION_ID_START_RECORDING_COMMAND, "Start command recording")
            }
            ShortcutAction::PasteLastTranscript => {
                (ACTION_ID_PASTE_LAST_TRANSCRIPT, "Paste last transcript")
            }
            ShortcutAction::CancelRecording => {
                (ACTION_ID_CANCEL_RECORDING, "Cancel recording")
            }
            ShortcutAction::SwitchLLMMode(0) => {
                (ACTION_ID_SWITCH_LLM_MODE_1, "Switch to LLM mode 1")
            }
            ShortcutAction::SwitchLLMMode(1) => {
                (ACTION_ID_SWITCH_LLM_MODE_2, "Switch to LLM mode 2")
            }
            ShortcutAction::SwitchLLMMode(2) => {
                (ACTION_ID_SWITCH_LLM_MODE_3, "Switch to LLM mode 3")
            }
            ShortcutAction::SwitchLLMMode(3) => {
                (ACTION_ID_SWITCH_LLM_MODE_4, "Switch to LLM mode 4")
            }
            _ => continue,
        };

        if binding.keys.is_empty() {
            continue;
        }

        actions.push(WlNewHotKeyAction::new(id, description, None));
    }

    actions
}

fn action_id_to_shortcut_event(id: u32) -> Option<ShortcutAction> {
    let action = match id {
        ACTION_ID_START_RECORDING => ShortcutAction::StartRecording,
        ACTION_ID_START_RECORDING_LLM => ShortcutAction::StartRecordingLLM,
        ACTION_ID_START_RECORDING_COMMAND => {
            ShortcutAction::StartRecordingCommand
        }
        ACTION_ID_PASTE_LAST_TRANSCRIPT => {
            ShortcutAction::PasteLastTranscript
        }
        ACTION_ID_CANCEL_RECORDING => ShortcutAction::CancelRecording,
        ACTION_ID_SWITCH_LLM_MODE_1 => ShortcutAction::SwitchLLMMode(0),
        ACTION_ID_SWITCH_LLM_MODE_2 => ShortcutAction::SwitchLLMMode(1),
        ACTION_ID_SWITCH_LLM_MODE_3 => ShortcutAction::SwitchLLMMode(2),
        ACTION_ID_SWITCH_LLM_MODE_4 => ShortcutAction::SwitchLLMMode(3),
        _ => return None,
    };

    Some(action)
}

pub fn init(app: AppHandle) {
    let manager = match GlobalHotKeyManager::new() {
        Ok(m) => Arc::new(m),
        Err(e) => {
            error!("Failed to create GlobalHotKeyManager for Wayland: {}", e);
            let _ = app.emit("wayland-shortcuts-unavailable", ());
            return;
        }
    };

    let registry_state = app.state::<ShortcutRegistryState>();
    let actions = build_actions(&registry_state);

    if actions.is_empty() {
        warn!("No shortcut actions to register on Wayland");
        return;
    }

    if let Err(e) = manager.wl_register_all("com.al1x.murmure", &actions) {
        error!("Failed to register Wayland shortcuts: {}", e);
        let _ = app.emit("wayland-shortcuts-unavailable", ());
        return;
    }

    info!(
        "Registered {} shortcut actions via XDG GlobalShortcuts portal",
        actions.len()
    );

    let app_handle = app.clone();
    std::thread::spawn(move || {
        debug!("Starting Wayland global shortcut event listener");
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            match receiver.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(event) => {
                    let shortcut_state = app_handle.state::<ShortcutState>();
                    if shortcut_state.is_suspended() {
                        continue;
                    }

                    if let Some(action) = action_id_to_shortcut_event(event.id) {
                        let key_event = match event.state {
                            HotKeyState::Pressed => KeyEventType::Pressed,
                            HotKeyState::Released => KeyEventType::Released,
                        };
                        debug!("Wayland shortcut triggered: {:?} ({:?})", action, key_event);
                        crate::shortcuts::handle_shortcut_event(
                            &app_handle,
                            &action,
                            &ActivationMode::ToggleToTalk,
                            key_event,
                        );
                    }
                }
                Err(e) => {
                    if e.is_timeout() {
                        continue;
                    }
                    warn!("Wayland shortcut event channel disconnected");
                    break;
                }
            }
        }
        warn!("Wayland shortcut event listener stopped");
    });
}
