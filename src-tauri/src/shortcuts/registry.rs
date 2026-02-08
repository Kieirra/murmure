use crate::shortcuts::helpers::parse_binding_keys;
use crate::shortcuts::types::{ActivationMode, ShortcutAction, ShortcutBinding, ShortcutRegistry};
use parking_lot::RwLock;
use std::sync::atomic::Ordering;

impl ShortcutRegistry {
    pub fn from_settings(settings: &crate::settings::types::AppSettings) -> Self {
        let activation_mode = if settings.record_mode == "toggle_to_talk" {
            ActivationMode::ToggleToTalk
        } else {
            ActivationMode::PushToTalk
        };

        let mut bindings = vec![
            ShortcutBinding {
                keys: parse_binding_keys(&settings.record_shortcut),
                action: ShortcutAction::StartRecording,
                activation_mode: activation_mode.clone(),
            },
            ShortcutBinding {
                keys: parse_binding_keys(&settings.llm_record_shortcut),
                action: ShortcutAction::StartRecordingLLM,
                activation_mode: activation_mode.clone(),
            },
            ShortcutBinding {
                keys: parse_binding_keys(&settings.command_shortcut),
                action: ShortcutAction::StartRecordingCommand,
                activation_mode: activation_mode.clone(),
            },
            ShortcutBinding {
                keys: parse_binding_keys(&settings.last_transcript_shortcut),
                action: ShortcutAction::PasteLastTranscript,
                activation_mode: ActivationMode::PushToTalk,
            },
        ];

        let mode_shortcuts = [
            (&settings.llm_mode_1_shortcut, 0),
            (&settings.llm_mode_2_shortcut, 1),
            (&settings.llm_mode_3_shortcut, 2),
            (&settings.llm_mode_4_shortcut, 3),
        ];

        for (shortcut_str, index) in mode_shortcuts {
            let keys = parse_binding_keys(shortcut_str);
            if !keys.is_empty() {
                bindings.push(ShortcutBinding {
                    keys,
                    action: ShortcutAction::SwitchLLMMode(index),
                    activation_mode: ActivationMode::PushToTalk,
                });
            }
        }

        // Sort bindings by key count descending so that more specific shortcuts
        // (e.g. Ctrl+A+Space) are matched before less specific ones (e.g. Ctrl+Space)
        bindings.sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));

        Self { bindings }
    }
}

pub struct ShortcutRegistryState(pub RwLock<ShortcutRegistry>);

impl ShortcutRegistryState {
    pub fn new(registry: ShortcutRegistry) -> Self {
        Self(RwLock::new(registry))
    }

    pub fn update_binding(&self, action: ShortcutAction, new_keys: Vec<i32>) {
        let mut registry = self.0.write();
        if let Some(binding) = registry.bindings.iter_mut().find(|b| b.action == action) {
            binding.keys = new_keys;
        }
        registry.bindings.sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));
    }

    pub fn set_activation_mode(&self, mode: ActivationMode) {
        let mut registry = self.0.write();
        for binding in &mut registry.bindings {
            match binding.action {
                ShortcutAction::StartRecording
                | ShortcutAction::StartRecordingLLM
                | ShortcutAction::StartRecordingCommand => {
                    binding.activation_mode = mode.clone();
                }
                _ => {}
            }
        }
    }
}

impl crate::shortcuts::types::ShortcutState {
    pub fn new() -> Self {
        Self {
            suspended: std::sync::atomic::AtomicBool::new(false),
            is_toggled: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::SeqCst)
    }

    pub fn set_suspended(&self, value: bool) {
        self.suspended.store(value, Ordering::SeqCst)
    }

    pub fn set_toggled(&self, value: bool) {
        self.is_toggled.store(value, Ordering::SeqCst)
    }
}
