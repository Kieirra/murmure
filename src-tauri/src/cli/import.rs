use std::collections::HashMap;
use std::path::Path;

use log::info;
use tauri::{AppHandle, Emitter, Manager};

use super::types::{ImportStrategy, MurmureExportData};
use crate::dictionary::Dictionary;
use crate::formatting_rules::types::FormattingSettings;
use crate::llm::types::LLMConnectSettings;

const MAX_SUPPORTED_VERSION: u32 = 1;
const MAX_LLM_MODES: usize = 4;

fn validate_extension(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("murmure") => Ok(()),
        _ => Err("Error: File must have a .murmure extension.".to_string()),
    }
}

fn category_display_name(key: &str) -> &str {
    match key {
        "settings" => "System Settings",
        "shortcuts" => "Shortcuts",
        "formatting_rules" => "Formatting Rules",
        "llm_connect" => "LLM Connect",
        "dictionary" => "Dictionary",
        _ => key,
    }
}

pub fn execute_import(
    app: &AppHandle,
    file_path: &str,
    strategy: &ImportStrategy,
) -> Result<String, String> {
    validate_extension(file_path)?;

    let content = std::fs::read_to_string(file_path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => format!("Error: File not found: {}", file_path),
        std::io::ErrorKind::PermissionDenied => {
            format!("Error: Permission denied: {}", file_path)
        }
        _ => format!("Error: Failed to read file: {}", e),
    })?;

    let data: MurmureExportData =
        serde_json::from_str(&content).map_err(|_| "Error: Invalid file format.".to_string())?;

    if data.version > MAX_SUPPORTED_VERSION {
        return Err(format!(
            "Error: Unsupported file version ({}). Maximum supported: {}.",
            data.version, MAX_SUPPORTED_VERSION
        ));
    }

    let mut imported_categories: Vec<&str> = Vec::new();

    let has_settings = data.categories.settings.is_some();
    let has_shortcuts = data.categories.shortcuts.is_some();

    if has_settings || has_shortcuts {
        let mut current = crate::settings::load_settings(app);

        if let Some(ref s) = data.categories.settings {
            current.record_mode = s.record_mode.clone();
            current.overlay_mode = s.overlay_mode.clone();
            current.overlay_position = s.overlay_position.clone();
            current.api_enabled = s.api_enabled;
            current.api_port = s.api_port;
            current.copy_to_clipboard = s.copy_to_clipboard;
            current.paste_method = s.paste_method.clone();
            current.persist_history = s.persist_history;
            current.language = s.language.clone();
            current.sound_enabled = s.sound_enabled;
            current.log_level = s.log_level.clone();
            current.show_in_dock = s.show_in_dock;
            imported_categories.push("settings");
        }

        if let Some(ref sc) = data.categories.shortcuts {
            current.record_shortcut = sc.record_shortcut.clone();
            current.last_transcript_shortcut = sc.last_transcript_shortcut.clone();
            current.llm_record_shortcut = sc.llm_record_shortcut.clone();
            current.command_shortcut = sc.command_shortcut.clone();
            current.llm_mode_1_shortcut = sc.llm_mode_1_shortcut.clone();
            current.llm_mode_2_shortcut = sc.llm_mode_2_shortcut.clone();
            current.llm_mode_3_shortcut = sc.llm_mode_3_shortcut.clone();
            current.llm_mode_4_shortcut = sc.llm_mode_4_shortcut.clone();
            current.cancel_shortcut = sc.cancel_shortcut.clone();
            imported_categories.push("shortcuts");
        }

        crate::settings::save_settings(app, &current)?;
    }

    if let Some(ref imported) = data.categories.formatting_rules {
        apply_formatting_rules(app, imported, strategy)?;
        imported_categories.push("formatting_rules");
    }

    if let Some(ref imported) = data.categories.llm_connect {
        apply_llm_connect(app, imported, strategy)?;
        imported_categories.push("llm_connect");
    }

    if let Some(ref imported) = data.categories.dictionary {
        apply_dictionary(app, imported, strategy)?;
        imported_categories.push("dictionary");
    }

    if imported_categories.is_empty() {
        return Ok(
            "Configuration imported successfully.\nNo categories found in file.".to_string(),
        );
    }

    let display_names: Vec<&str> = imported_categories
        .iter()
        .map(|k| category_display_name(k))
        .collect();

    Ok(format!(
        "Configuration imported successfully.\nUpdated: {}.",
        display_names.join(", ")
    ))
}

fn apply_formatting_rules(
    app: &AppHandle,
    imported: &FormattingSettings,
    strategy: &ImportStrategy,
) -> Result<(), String> {
    match strategy {
        ImportStrategy::Replace => {
            crate::formatting_rules::save(app, imported)?;
        }
        ImportStrategy::Merge => {
            let current = crate::formatting_rules::load(app)?;

            let mut merged_rules = current.rules;
            for imported_rule in &imported.rules {
                if let Some(idx) = merged_rules.iter().position(|r| r.id == imported_rule.id) {
                    merged_rules[idx] = imported_rule.clone();
                } else {
                    merged_rules.push(imported_rule.clone());
                }
            }

            let merged = FormattingSettings {
                built_in: imported.built_in.clone(),
                rules: merged_rules,
            };
            crate::formatting_rules::save(app, &merged)?;
        }
    }
    Ok(())
}

fn apply_llm_connect(
    app: &AppHandle,
    imported: &LLMConnectSettings,
    strategy: &ImportStrategy,
) -> Result<(), String> {
    match strategy {
        ImportStrategy::Replace => {
            let mut settings = imported.clone();
            settings.model = String::new();
            settings.prompt = String::new();
            crate::llm::helpers::save_llm_connect_settings(app, &settings)?;
        }
        ImportStrategy::Merge => {
            let current = crate::llm::helpers::load_llm_connect_settings(app);

            let existing_names: Vec<String> = current
                .modes
                .iter()
                .map(|m| m.name.to_lowercase())
                .collect();

            let mut merged_modes = current.modes.clone();
            for mode in &imported.modes {
                if existing_names.contains(&mode.name.to_lowercase()) {
                    continue;
                }
                if merged_modes.len() >= MAX_LLM_MODES {
                    break;
                }
                merged_modes.push(mode.clone());
            }

            let url = if imported.url.is_empty() {
                current.url.clone()
            } else {
                imported.url.clone()
            };

            let remote_url = if imported.remote_url.is_empty() {
                current.remote_url.clone()
            } else {
                imported.remote_url.clone()
            };

            let settings = LLMConnectSettings {
                url,
                model: current.model.clone(),
                prompt: current.prompt.clone(),
                modes: merged_modes,
                active_mode_index: current.active_mode_index,
                onboarding_completed: imported.onboarding_completed,
                remote_url,
                remote_privacy_acknowledged: imported.remote_privacy_acknowledged,
            };
            crate::llm::helpers::save_llm_connect_settings(app, &settings)?;
        }
    }
    Ok(())
}

fn apply_dictionary(
    app: &AppHandle,
    imported: &HashMap<String, Vec<String>>,
    strategy: &ImportStrategy,
) -> Result<(), String> {
    match strategy {
        ImportStrategy::Replace => {
            crate::dictionary::store::save(app, imported)?;
        }
        ImportStrategy::Merge => {
            let current = crate::dictionary::store::load(app)?;
            let merged = merge_dictionaries(&current, imported);
            crate::dictionary::store::save(app, &merged)?;
        }
    }
    Ok(())
}

fn merge_dictionaries(
    current: &HashMap<String, Vec<String>>,
    imported: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut merged = current.clone();

    for (word, languages) in imported {
        let existing_key = merged
            .keys()
            .find(|k| k.eq_ignore_ascii_case(word))
            .cloned();

        match existing_key {
            Some(key) => {
                if let Some(existing_langs) = merged.get_mut(&key) {
                    for lang in languages {
                        if !existing_langs.iter().any(|l| l == lang) {
                            existing_langs.push(lang.clone());
                        }
                    }
                }
            }
            None => {
                merged.insert(word.clone(), languages.clone());
            }
        }
    }

    merged
}

pub fn apply_hot_reload_side_effects(app: &AppHandle) {
    crate::shortcuts::init_shortcuts(app.clone());

    let settings = crate::settings::load_settings(app);

    if settings.overlay_mode.as_str() == "always" {
        crate::overlay::overlay::show_recording_overlay(app);
    } else {
        crate::overlay::overlay::hide_recording_overlay(app);
    }

    crate::overlay::overlay::update_overlay_position(app);

    if let Ok(level) = settings.log_level.parse::<log::LevelFilter>() {
        log::set_max_level(level);
    }

    let llm_settings = crate::llm::helpers::load_llm_connect_settings(app);
    let _ = app.emit("llm-settings-updated", &llm_settings);

    crate::llm::helpers::restart_wake_word_if_active(app);

    match crate::dictionary::store::load(app) {
        Ok(dict) => {
            let dictionary_state = app.state::<Dictionary>();
            dictionary_state.set(dict);
            let _ = app.emit("dictionary:updated", ());
        }
        Err(e) => {
            log::error!("Failed to reload dictionary after CLI import: {}", e);
        }
    }

    let _ = app.emit("config-imported", ());

    info!("Hot-reload side effects applied after CLI import");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_extension_valid() {
        assert!(validate_extension("/tmp/config.murmure").is_ok());
        assert!(validate_extension("C:\\config\\standard.murmure").is_ok());
    }

    #[test]
    fn test_validate_extension_invalid() {
        let result = validate_extension("/tmp/config.json");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("File must have a .murmure extension"));
    }

    #[test]
    fn test_validate_extension_no_extension() {
        let result = validate_extension("/tmp/config");
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_dictionaries_new_word() {
        let mut current = HashMap::new();
        current.insert("Kubernetes".to_string(), vec!["english".to_string()]);

        let mut imported = HashMap::new();
        imported.insert("Docker".to_string(), vec!["english".to_string()]);

        let merged = merge_dictionaries(&current, &imported);
        assert_eq!(merged.len(), 2);
        assert!(merged.contains_key("Kubernetes"));
        assert!(merged.contains_key("Docker"));
    }

    #[test]
    fn test_merge_dictionaries_case_insensitive_union() {
        let mut current = HashMap::new();
        current.insert("Kubernetes".to_string(), vec!["english".to_string()]);

        let mut imported = HashMap::new();
        imported.insert("kubernetes".to_string(), vec!["french".to_string()]);

        let merged = merge_dictionaries(&current, &imported);
        assert_eq!(merged.len(), 1);
        let (_, langs) = merged.iter().next().unwrap();
        assert!(langs.contains(&"english".to_string()));
        assert!(langs.contains(&"french".to_string()));
    }

    #[test]
    fn test_merge_dictionaries_no_duplicate_languages() {
        let mut current = HashMap::new();
        current.insert(
            "Test".to_string(),
            vec!["english".to_string(), "french".to_string()],
        );

        let mut imported = HashMap::new();
        imported.insert("test".to_string(), vec!["english".to_string()]);

        let merged = merge_dictionaries(&current, &imported);
        assert_eq!(merged.len(), 1);
        let (_, langs) = merged.iter().next().unwrap();
        assert_eq!(langs.len(), 2);
    }

    #[test]
    fn test_merge_dictionaries_empty_current() {
        let current = HashMap::new();
        let mut imported = HashMap::new();
        imported.insert("Docker".to_string(), vec!["english".to_string()]);

        let merged = merge_dictionaries(&current, &imported);
        assert_eq!(merged.len(), 1);
        assert!(merged.contains_key("Docker"));
    }

    #[test]
    fn test_merge_dictionaries_empty_imported() {
        let mut current = HashMap::new();
        current.insert("Kubernetes".to_string(), vec!["english".to_string()]);
        let imported = HashMap::new();

        let merged = merge_dictionaries(&current, &imported);
        assert_eq!(merged.len(), 1);
        assert!(merged.contains_key("Kubernetes"));
    }

    #[test]
    fn test_category_display_name() {
        assert_eq!(category_display_name("settings"), "System Settings");
        assert_eq!(category_display_name("shortcuts"), "Shortcuts");
        assert_eq!(
            category_display_name("formatting_rules"),
            "Formatting Rules"
        );
        assert_eq!(category_display_name("llm_connect"), "LLM Connect");
        assert_eq!(category_display_name("dictionary"), "Dictionary");
        assert_eq!(category_display_name("unknown"), "unknown");
    }
}
