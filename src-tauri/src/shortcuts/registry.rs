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

        let mut bindings: Vec<ShortcutBinding> = Vec::new();

        let mut push_if_set = |shortcut: &str, action, mode| {
            let keys = parse_binding_keys(shortcut);
            if !keys.is_empty() {
                bindings.push(ShortcutBinding {
                    keys,
                    action,
                    activation_mode: mode,
                });
            }
        };

        push_if_set(
            &settings.record_shortcut,
            ShortcutAction::StartRecording,
            activation_mode.clone(),
        );
        push_if_set(
            &settings.command_shortcut,
            ShortcutAction::StartRecordingCommand,
            activation_mode.clone(),
        );
        push_if_set(
            &settings.last_transcript_shortcut,
            ShortcutAction::PasteLastTranscript,
            ActivationMode::PushToTalk,
        );
        push_if_set(
            &settings.cancel_shortcut,
            ShortcutAction::CancelRecording,
            ActivationMode::PushToTalk,
        );

        let mode_shortcuts = [
            (&settings.llm_mode_1_shortcut, 0),
            (&settings.llm_mode_2_shortcut, 1),
            (&settings.llm_mode_3_shortcut, 2),
            (&settings.llm_mode_4_shortcut, 3),
        ];

        for (shortcut_str, index) in mode_shortcuts {
            push_if_set(
                shortcut_str,
                ShortcutAction::StartRecordingLlmMode(index),
                activation_mode.clone(),
            );
        }

        push_if_set(
            &settings.voice_mode_toggle_shortcut,
            ShortcutAction::ToggleVoiceMode,
            ActivationMode::PushToTalk,
        );

        // Sort bindings by key count descending so that more specific shortcuts
        // (e.g. Ctrl+A+Space) are matched before less specific ones (e.g. Ctrl+Space)
        bindings.sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));

        Self { bindings }
    }

    fn is_global(action: &ShortcutAction) -> bool {
        matches!(
            action,
            ShortcutAction::StartRecording
                | ShortcutAction::StartRecordingCommand
                | ShortcutAction::StartRecordingLlmMode(_)
        )
    }

    fn activation_mode_for(&self, action: &ShortcutAction) -> ActivationMode {
        if Self::is_global(action) {
            self.global_activation_mode()
        } else {
            ActivationMode::PushToTalk
        }
    }

    fn global_activation_mode(&self) -> ActivationMode {
        self.bindings
            .iter()
            .find(|b| Self::is_global(&b.action))
            .map(|b| b.activation_mode.clone())
            .unwrap_or(ActivationMode::PushToTalk)
    }
}

pub struct ShortcutRegistryState(pub RwLock<ShortcutRegistry>);

impl ShortcutRegistryState {
    pub fn new(registry: ShortcutRegistry) -> Self {
        Self(RwLock::new(registry))
    }

    pub fn update_binding(&self, action: ShortcutAction, new_keys: Vec<i32>) {
        let mut registry = self.0.write();
        match registry.bindings.iter_mut().find(|b| b.action == action) {
            Some(binding) => binding.keys = new_keys,
            None => {
                let activation_mode = registry.activation_mode_for(&action);
                registry.bindings.push(ShortcutBinding {
                    keys: new_keys,
                    action,
                    activation_mode,
                });
            }
        }
        registry
            .bindings
            .sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));
    }

    pub fn set_activation_mode(&self, mode: ActivationMode) {
        let mut registry = self.0.write();
        for binding in &mut registry.bindings {
            if ShortcutRegistry::is_global(&binding.action) {
                binding.activation_mode = mode.clone();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::types::AppSettings;

    fn find(registry: &ShortcutRegistry, action: &ShortcutAction) -> Option<ShortcutBinding> {
        registry
            .bindings
            .iter()
            .find(|b| &b.action == action)
            .cloned()
    }

    #[test]
    fn from_settings_skips_empty_shortcut() {
        let mut settings = AppSettings::default();
        settings.cancel_shortcut = String::new();
        settings.record_shortcut = String::new();

        let registry = ShortcutRegistry::from_settings(&settings);

        assert!(find(&registry, &ShortcutAction::CancelRecording).is_none());
        assert!(find(&registry, &ShortcutAction::StartRecording).is_none());
    }

    #[test]
    fn update_binding_clears_keys_when_empty() {
        let settings = AppSettings::default();
        let state = ShortcutRegistryState::new(ShortcutRegistry::from_settings(&settings));

        state.update_binding(ShortcutAction::StartRecording, Vec::new());

        let registry = state.0.read();
        let binding = find(&registry, &ShortcutAction::StartRecording).unwrap();
        assert!(binding.keys.is_empty());
    }

    #[test]
    fn update_binding_inserts_when_action_absent() {
        let mut settings = AppSettings::default();
        settings.cancel_shortcut = String::new();
        let state = ShortcutRegistryState::new(ShortcutRegistry::from_settings(&settings));
        assert!(find(&state.0.read(), &ShortcutAction::CancelRecording).is_none());

        let keys = parse_binding_keys("ctrl+x");
        state.update_binding(ShortcutAction::CancelRecording, keys.clone());

        let registry = state.0.read();
        let binding = find(&registry, &ShortcutAction::CancelRecording).unwrap();
        assert_eq!(binding.keys, keys);
        assert_eq!(binding.activation_mode, ActivationMode::PushToTalk);
    }
}
