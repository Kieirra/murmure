//! macOS keyboard shortcut handling using tauri-plugin-global-shortcut
//!
//! Uses the official Tauri global shortcut plugin for macOS,
//! which provides reliable press/release detection.

use log::{debug, error, info, warn};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::shortcuts::accessibility_macos;
use crate::shortcuts::registry::ShortcutRegistryState;
use crate::shortcuts::types::{KeyEventType, ShortcutState as AppShortcutState};

/// Convert internal key format ("ctrl+space") to plugin format ("Control+Space").
fn to_plugin_format(internal: &str) -> String {
    internal
        .split('+')
        .map(|part| match part.trim().to_lowercase().as_str() {
            "ctrl" | "control" => "Control",
            "alt" | "menu" => "Alt",
            "shift" => "Shift",
            "win" | "meta" | "super" | "command" | "cmd" => "Super",
            "space" => "Space",
            "enter" | "return" => "Enter",
            "escape" | "esc" => "Escape",
            "tab" => "Tab",
            "backspace" => "Backspace",
            "delete" | "del" => "Delete",
            "insert" | "ins" => "Insert",
            "home" => "Home",
            "end" => "End",
            "pageup" => "PageUp",
            "pagedown" => "PageDown",
            "arrowup" | "up" => "ArrowUp",
            "arrowdown" | "down" => "ArrowDown",
            "arrowleft" | "left" => "ArrowLeft",
            "arrowright" | "right" => "ArrowRight",
            "f1" => "F1",
            "f2" => "F2",
            "f3" => "F3",
            "f4" => "F4",
            "f5" => "F5",
            "f6" => "F6",
            "f7" => "F7",
            "f8" => "F8",
            "f9" => "F9",
            "f10" => "F10",
            "f11" => "F11",
            "f12" => "F12",
            // Single letter/digit keys: uppercase them (e.g., "a" -> "A")
            other if other.len() == 1 => {
                // Will be handled by the String conversion below
                return other.to_uppercase();
            }
            other => return other.to_string(),
        })
        .collect::<Vec<_>>()
        .join("+")
}

pub fn init(app: AppHandle) {
    // Check Accessibility permission first
    if !accessibility_macos::check_and_log_permission() {
        warn!("Accessibility permission not granted - emitting event to frontend");
        let _ = app.emit("accessibility-permission-missing", ());
        return;
    }

    let registry_state = app.state::<ShortcutRegistryState>();
    let registry = registry_state.0.read();

    debug!(
        "[macOS shortcuts] Registry has {} bindings",
        registry.bindings.len()
    );

    // Collect shortcut strings and their binding indices
    let mut shortcut_entries: Vec<(Shortcut, usize)> = Vec::new();

    for (i, binding) in registry.bindings.iter().enumerate() {
        if binding.keys.is_empty() {
            continue;
        }

        let key_str = crate::shortcuts::keys_to_string(&binding.keys);
        let plugin_str = to_plugin_format(&key_str);

        match plugin_str.parse::<Shortcut>() {
            Ok(shortcut) => {
                debug!(
                    "[macOS shortcuts] Binding {}: action={:?}, shortcut={}",
                    i, binding.action, plugin_str
                );
                shortcut_entries.push((shortcut, i));
            }
            Err(e) => {
                error!(
                    "[macOS shortcuts] Failed to parse shortcut '{}' (from '{}'): {}",
                    plugin_str, key_str, e
                );
            }
        }
    }

    drop(registry);

    // Register all shortcuts with the plugin
    let app_for_handler = app.clone();
    for (shortcut, binding_index) in &shortcut_entries {
        let app_clone = app_for_handler.clone();
        let idx = *binding_index;

        if let Err(e) =
            app.global_shortcut()
                .on_shortcut(shortcut.clone(), move |_app, _shortcut, event| {
                    let shortcut_state = app_clone.state::<AppShortcutState>();
                    if shortcut_state.is_suspended() {
                        return;
                    }

                    let registry_state = app_clone.state::<ShortcutRegistryState>();
                    let registry = registry_state.0.read();

                    if let Some(binding) = registry.bindings.get(idx) {
                        let event_type = match event.state() {
                            ShortcutState::Pressed => KeyEventType::Pressed,
                            ShortcutState::Released => KeyEventType::Released,
                        };

                        let action = binding.action.clone();
                        let mode = binding.activation_mode.clone();
                        drop(registry);

                        crate::shortcuts::handle_shortcut_event(
                            &app_clone, &action, &mode, event_type,
                        );
                    }
                })
        {
            error!(
                "[macOS shortcuts] Failed to register shortcut for binding {}: {}",
                binding_index, e
            );
        }
    }

    info!(
        "[macOS shortcuts] Registered {} shortcuts via plugin",
        shortcut_entries.len()
    );
    debug!("[macOS shortcuts] Initialization complete");
}
