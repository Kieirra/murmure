use std::collections::HashMap;

use serde::Deserialize;

use crate::formatting_rules::types::FormattingSettings;
use crate::llm::types::LLMConnectSettings;
use crate::settings::types::PasteMethod;

#[derive(Debug)]
pub enum CliCommand {
    Import {
        file_path: String,
        strategy: ImportStrategy,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportStrategy {
    Replace,
    Merge,
}

#[derive(Deserialize)]
pub struct MurmureExportData {
    pub version: u32,
    #[allow(dead_code)]
    pub app_version: String,
    #[allow(dead_code)]
    pub exported_at: String,
    pub categories: ExportedCategories,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ExportedCategories {
    pub settings: Option<SystemSettings>,
    pub shortcuts: Option<ShortcutSettings>,
    pub formatting_rules: Option<FormattingSettings>,
    pub llm_connect: Option<LLMConnectSettings>,
    pub dictionary: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct SystemSettings {
    pub record_mode: String,
    pub overlay_mode: String,
    pub overlay_position: String,
    pub api_enabled: bool,
    pub api_port: u16,
    pub copy_to_clipboard: bool,
    pub paste_method: PasteMethod,
    pub persist_history: bool,
    pub language: String,
    pub sound_enabled: bool,
    pub log_level: String,
    pub show_in_dock: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            record_mode: "push_to_talk".to_string(),
            overlay_mode: "recording".to_string(),
            overlay_position: "bottom".to_string(),
            api_enabled: false,
            api_port: 4800,
            copy_to_clipboard: false,
            paste_method: PasteMethod::default(),
            persist_history: false,
            language: "default".to_string(),
            sound_enabled: true,
            log_level: "info".to_string(),
            show_in_dock: true,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ShortcutSettings {
    pub record_shortcut: String,
    pub last_transcript_shortcut: String,
    pub llm_record_shortcut: String,
    pub command_shortcut: String,
    pub llm_mode_1_shortcut: String,
    pub llm_mode_2_shortcut: String,
    pub llm_mode_3_shortcut: String,
    pub llm_mode_4_shortcut: String,
    pub cancel_shortcut: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            record_shortcut: "ctrl+space".to_string(),
            last_transcript_shortcut: "ctrl+shift+space".to_string(),
            llm_record_shortcut: "ctrl+alt+space".to_string(),
            command_shortcut: "ctrl+shift+x".to_string(),
            llm_mode_1_shortcut: "ctrl+shift+1".to_string(),
            llm_mode_2_shortcut: "ctrl+shift+2".to_string(),
            llm_mode_3_shortcut: "ctrl+shift+3".to_string(),
            llm_mode_4_shortcut: "ctrl+shift+4".to_string(),
            cancel_shortcut: "escape".to_string(),
        }
    }
}
