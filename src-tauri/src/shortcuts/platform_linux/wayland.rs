use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use log::{debug, error, info, warn};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{ActivationMode, KeyEventType, ShortcutAction, ShortcutState};

pub struct WaylandShortcutsState {
    _manager: Arc<GlobalHotKeyManager>,
    action_by_id: HashMap<u32, ShortcutAction>,
}

#[derive(Clone, Serialize)]
struct WaylandShortcutsUnavailablePayload {
    reason: String,
}

pub fn init(app: AppHandle) {
    let manager = match GlobalHotKeyManager::new() {
        Ok(m) => Arc::new(m),
        Err(e) => {
            error!("Failed to initialize Wayland shortcut manager: {}", e);
            emit_unavailable(&app, format!("manager init failed: {e}"));
            return;
        }
    };

    let (hotkeys, action_by_id) = build_hotkeys(&app);
    if hotkeys.is_empty() {
        warn!("No Wayland shortcut actions available to register");
        return;
    }

    if let Err(e) = manager.register_all(&hotkeys) {
        error!("Wayland portal shortcut registration failed: {}", e);
        emit_unavailable(&app, format!("portal registration failed: {e}"));
        return;
    }

    app.manage(WaylandShortcutsState {
        _manager: manager,
        action_by_id,
    });

    info!(
        "Wayland portal shortcuts registered ({} actions)",
        hotkeys.len()
    );

    spawn_event_listener(app);
}

fn emit_unavailable(app: &AppHandle, reason: String) {
    let _ = app.emit(
        "wayland-shortcuts-unavailable",
        WaylandShortcutsUnavailablePayload { reason },
    );
}

fn build_hotkeys(app: &AppHandle) -> (Vec<HotKey>, HashMap<u32, ShortcutAction>) {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read();

    let mut hotkeys = Vec::new();
    let mut action_by_id = HashMap::new();

    for binding in &registry.bindings {
        let Some(hotkey) = hotkey_from_binding_keys(&binding.keys) else {
            continue;
        };
        // Last-wins on id collision (two actions mapped to the same combo).
        action_by_id.insert(hotkey.id, binding.action.clone());
        hotkeys.push(hotkey);
    }

    (hotkeys, action_by_id)
}

fn activation_mode_for_action(app: &AppHandle, action: &ShortcutAction) -> ActivationMode {
    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read();
    for binding in &registry.bindings {
        if binding.action == *action {
            return binding.activation_mode.clone();
        }
    }

    warn!(
        "No binding found for action {:?}; defaulting to PushToTalk",
        action
    );
    ActivationMode::PushToTalk
}

fn spawn_event_listener(app: AppHandle) {
    std::thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();
        debug!("Wayland global shortcut listener started");

        loop {
            match receiver.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(event) => {
                    if app.state::<ShortcutState>().is_suspended() {
                        continue;
                    }

                    let shortcuts_state = app.state::<WaylandShortcutsState>();
                    let Some(action) = shortcuts_state.action_by_id.get(&event.id).cloned() else {
                        continue;
                    };

                    let key_event = match event.state {
                        HotKeyState::Pressed => KeyEventType::Pressed,
                        HotKeyState::Released => KeyEventType::Released,
                    };

                    let activation_mode = activation_mode_for_action(&app, &action);
                    crate::shortcuts::handle_shortcut_event(
                        &app,
                        &action,
                        &activation_mode,
                        key_event,
                    );
                }
                Err(err) => {
                    if err.is_timeout() {
                        continue;
                    }
                    warn!("Wayland shortcut listener stopped: {}", err);
                    break;
                }
            }
        }
    });
}

fn hotkey_from_binding_keys(binding_keys: &[i32]) -> Option<HotKey> {
    if binding_keys.is_empty() {
        return None;
    }

    let mut modifiers = Modifiers::empty();
    let mut non_modifier_key: Option<i32> = None;

    for key in binding_keys {
        match key {
            0x11 => modifiers |= Modifiers::CONTROL,
            0x10 => modifiers |= Modifiers::SHIFT,
            0x12 => modifiers |= Modifiers::ALT,
            0x5B => modifiers |= Modifiers::SUPER,
            _ => {
                if non_modifier_key.is_some() {
                    warn!(
                        "Wayland hotkey dropped: binding has multiple non-modifier keys ({:?})",
                        binding_keys
                    );
                    return None;
                }
                non_modifier_key = Some(*key);
            }
        }
    }

    let vk = non_modifier_key?;
    let Some(code) = vk_to_code(vk) else {
        warn!(
            "Wayland hotkey dropped: unsupported virtual key 0x{:02X}",
            vk
        );
        return None;
    };
    let mods = if modifiers.is_empty() {
        None
    } else {
        Some(modifiers)
    };
    Some(HotKey::new(mods, code))
}

fn vk_to_code(vk: i32) -> Option<Code> {
    match vk {
        0x41 => Some(Code::KeyA),
        0x42 => Some(Code::KeyB),
        0x43 => Some(Code::KeyC),
        0x44 => Some(Code::KeyD),
        0x45 => Some(Code::KeyE),
        0x46 => Some(Code::KeyF),
        0x47 => Some(Code::KeyG),
        0x48 => Some(Code::KeyH),
        0x49 => Some(Code::KeyI),
        0x4A => Some(Code::KeyJ),
        0x4B => Some(Code::KeyK),
        0x4C => Some(Code::KeyL),
        0x4D => Some(Code::KeyM),
        0x4E => Some(Code::KeyN),
        0x4F => Some(Code::KeyO),
        0x50 => Some(Code::KeyP),
        0x51 => Some(Code::KeyQ),
        0x52 => Some(Code::KeyR),
        0x53 => Some(Code::KeyS),
        0x54 => Some(Code::KeyT),
        0x55 => Some(Code::KeyU),
        0x56 => Some(Code::KeyV),
        0x57 => Some(Code::KeyW),
        0x58 => Some(Code::KeyX),
        0x59 => Some(Code::KeyY),
        0x5A => Some(Code::KeyZ),
        0x30 => Some(Code::Digit0),
        0x31 => Some(Code::Digit1),
        0x32 => Some(Code::Digit2),
        0x33 => Some(Code::Digit3),
        0x34 => Some(Code::Digit4),
        0x35 => Some(Code::Digit5),
        0x36 => Some(Code::Digit6),
        0x37 => Some(Code::Digit7),
        0x38 => Some(Code::Digit8),
        0x39 => Some(Code::Digit9),
        0x70 => Some(Code::F1),
        0x71 => Some(Code::F2),
        0x72 => Some(Code::F3),
        0x73 => Some(Code::F4),
        0x74 => Some(Code::F5),
        0x75 => Some(Code::F6),
        0x76 => Some(Code::F7),
        0x77 => Some(Code::F8),
        0x78 => Some(Code::F9),
        0x79 => Some(Code::F10),
        0x7A => Some(Code::F11),
        0x7B => Some(Code::F12),
        0x7C => Some(Code::F13),
        0x7D => Some(Code::F14),
        0x7E => Some(Code::F15),
        0x7F => Some(Code::F16),
        0x80 => Some(Code::F17),
        0x81 => Some(Code::F18),
        0x82 => Some(Code::F19),
        0x83 => Some(Code::F20),
        0x60 => Some(Code::Numpad0),
        0x61 => Some(Code::Numpad1),
        0x62 => Some(Code::Numpad2),
        0x63 => Some(Code::Numpad3),
        0x64 => Some(Code::Numpad4),
        0x65 => Some(Code::Numpad5),
        0x66 => Some(Code::Numpad6),
        0x67 => Some(Code::Numpad7),
        0x68 => Some(Code::Numpad8),
        0x69 => Some(Code::Numpad9),
        0x6A => Some(Code::NumpadMultiply),
        0x6B => Some(Code::NumpadAdd),
        0x6D => Some(Code::NumpadSubtract),
        0x6F => Some(Code::NumpadDivide),
        0x20 => Some(Code::Space),
        0x0D => Some(Code::Enter),
        0x1B => Some(Code::Escape),
        0x09 => Some(Code::Tab),
        0x08 => Some(Code::Backspace),
        0x2E => Some(Code::Delete),
        0x2D => Some(Code::Insert),
        0x24 => Some(Code::Home),
        0x23 => Some(Code::End),
        0x21 => Some(Code::PageUp),
        0x22 => Some(Code::PageDown),
        0x26 => Some(Code::ArrowUp),
        0x28 => Some(Code::ArrowDown),
        0x25 => Some(Code::ArrowLeft),
        0x27 => Some(Code::ArrowRight),
        0xBD => Some(Code::Minus),
        0xBB => Some(Code::Equal),
        0xDB => Some(Code::BracketLeft),
        0xDD => Some(Code::BracketRight),
        0xBA => Some(Code::Semicolon),
        0xDE => Some(Code::Quote),
        0xBC => Some(Code::Comma),
        0xBE => Some(Code::Period),
        0xBF => Some(Code::Slash),
        0xDC => Some(Code::Backslash),
        0xC0 => Some(Code::Backquote),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{hotkey_from_binding_keys, vk_to_code};

    #[test]
    fn creates_hotkey_from_modifier_and_key() {
        let hotkey = hotkey_from_binding_keys(&[0x11, 0x10, 0x41]);
        assert!(hotkey.is_some());
    }

    #[test]
    fn rejects_multiple_non_modifier_keys() {
        assert!(hotkey_from_binding_keys(&[0x41, 0x42]).is_none());
    }

    #[test]
    fn rejects_unsupported_virtual_key() {
        assert!(hotkey_from_binding_keys(&[0x11, 0xFE]).is_none());
    }

    #[test]
    fn rejects_empty_binding() {
        assert!(hotkey_from_binding_keys(&[]).is_none());
    }

    #[test]
    fn maps_f13_through_f20() {
        for vk in 0x7C..=0x83 {
            assert!(
                vk_to_code(vk).is_some(),
                "expected vk_to_code(0x{:02X}) to be Some",
                vk
            );
        }
    }

    #[test]
    fn distinct_hotkeys_have_distinct_ids() {
        let a = hotkey_from_binding_keys(&[0x11, 0x41]).unwrap();
        let b = hotkey_from_binding_keys(&[0x11, 0x42]).unwrap();
        assert_ne!(a.id, b.id);
    }
}
