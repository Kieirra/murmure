use std::collections::HashMap;

use serde::Deserialize;

use crate::formatting_rules::types::FormattingSettings;
use crate::llm::types::LLMConnectSettings;
use crate::settings::types::PasteMethod;

#[derive(Debug, Clone, PartialEq)]
pub enum CliCommand {
    Import {
        file_path: String,
        strategy: ImportStrategy,
    },
    Transcription,
    TranscriptionCommand,
    PasteLast,
    Cancel,
    VoiceMode,
    LlmMode(u8),
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
    pub voice_mode: Option<VoiceModeSettings>,
    pub smartmic: Option<SmartMicSettings>,
    pub formatting_rules: Option<FormattingSettings>,
    pub llm_connect: Option<LLMConnectSettings>,
    pub dictionary: Option<DictionaryExport>,
}

/// Current backups store the dictionary as a word list; older ones as a
/// `{ word: languages }` map whose values are ignored.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum DictionaryExport {
    Words(Vec<String>),
    Legacy(HashMap<String, Vec<String>>),
}

impl DictionaryExport {
    pub fn words(&self) -> Vec<String> {
        match self {
            DictionaryExport::Words(words) => words.clone(),
            DictionaryExport::Legacy(map) => map.keys().cloned().collect(),
        }
    }
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
    pub streaming_preview: bool,
    pub overlay_size: String,
    pub streaming_text_width: u32,
    pub streaming_font_size: u32,
    pub streaming_max_lines: u32,
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
            streaming_preview: false,
            overlay_size: "small".to_string(),
            streaming_text_width: 450,
            streaming_font_size: 11,
            streaming_max_lines: 5,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct VoiceModeSettings {
    pub wake_word_enabled: bool,
    pub wake_word_record: String,
    pub wake_word_command: String,
    pub wake_word_cancel: String,
    pub wake_word_validate: String,
    pub wake_word_submit: String,
    pub auto_enter_after_wake_word: bool,
    pub silence_timeout_ms: u64,
}

impl Default for VoiceModeSettings {
    fn default() -> Self {
        Self {
            wake_word_enabled: false,
            wake_word_record: "ok alix".to_string(),
            wake_word_command: "alix command".to_string(),
            wake_word_cancel: "alix cancel".to_string(),
            wake_word_validate: "alix validate".to_string(),
            wake_word_submit: "thank you alix".to_string(),
            auto_enter_after_wake_word: false,
            silence_timeout_ms: 1500,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct SmartMicSettings {
    pub smartmic_enabled: bool,
    pub smartmic_port: u16,
    pub smartmic_relay_enabled: bool,
    pub smartmic_relay_url: Option<String>,
    pub smartmic_machine_id_enabled: bool,
    pub smartmic_machine_id: Option<String>,
    pub smartmic_token_ttl_hours: Option<u64>,
    pub smartmic_bind_address: Option<String>,
}

impl Default for SmartMicSettings {
    fn default() -> Self {
        Self {
            smartmic_enabled: false,
            smartmic_port: 4801,
            smartmic_relay_enabled: false,
            smartmic_relay_url: None,
            smartmic_machine_id_enabled: false,
            smartmic_machine_id: None,
            smartmic_token_ttl_hours: None,
            smartmic_bind_address: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ShortcutSettings {
    pub record_shortcut: String,
    pub last_transcript_shortcut: String,
    pub command_shortcut: String,
    pub llm_mode_1_shortcut: String,
    pub llm_mode_2_shortcut: String,
    pub llm_mode_3_shortcut: String,
    pub llm_mode_4_shortcut: String,
    pub voice_mode_toggle_shortcut: String,
    pub cancel_shortcut: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            record_shortcut: "ctrl+space".to_string(),
            last_transcript_shortcut: "ctrl+shift+space".to_string(),
            command_shortcut: "ctrl+shift+x".to_string(),
            llm_mode_1_shortcut: "ctrl+shift+1".to_string(),
            llm_mode_2_shortcut: "ctrl+shift+2".to_string(),
            llm_mode_3_shortcut: "ctrl+shift+3".to_string(),
            llm_mode_4_shortcut: "ctrl+shift+4".to_string(),
            voice_mode_toggle_shortcut: "ctrl+shift+0".to_string(),
            cancel_shortcut: "ctrl+backspace".to_string(),
        }
    }
}
