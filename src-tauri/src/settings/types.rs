use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OnboardingState {
    #[serde(default)]
    pub used_home_shortcut: bool,
    #[serde(default)]
    pub transcribed_outside_app: bool,
    #[serde(default)]
    pub added_dictionary_word: bool,
    #[serde(default)]
    pub congrats_dismissed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AppSettings {
    pub record_shortcut: String,
    pub last_transcript_shortcut: String,
    pub llm_record_shortcut: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dictionary: Vec<String>,
    pub record_mode: String,      // "push_to_talk" | "toggle_to_talk"
    pub overlay_mode: String,     // "hidden" | "recording" | "always"
    pub overlay_position: String, // "top" | "bottom"
    pub api_enabled: bool,        // Enable local HTTP API
    pub api_port: u16,            // Port for local HTTP API
    pub copy_to_clipboard: bool,  // Keep transcription in clipboard after recording finishes
    #[serde(default)]
    pub persist_history: bool, // Persist last 5 transcriptions to disk
    #[serde(default)]
    pub language: String, // UI language code (e.g., "en", "fr")
    #[serde(default)]
    pub sound_enabled: bool,
    #[serde(default)]
    pub onboarding: OnboardingState,
    pub mic_id: Option<String>, // Optional microphone device ID
    pub log_level: String,      // "info" | "debug" | "trace" | "warn" | "error"
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            record_shortcut: "ctrl+space".to_string(),
            last_transcript_shortcut: "ctrl+shift+space".to_string(),
            llm_record_shortcut: "ctrl+alt+space".to_string(),
            dictionary: Vec::new(),
            record_mode: "push_to_talk".to_string(),
            overlay_mode: "recording".to_string(),
            overlay_position: "bottom".to_string(),
            api_enabled: false,
            api_port: 4800,
            copy_to_clipboard: false,
            persist_history: true,
            language: "default".to_string(),
            sound_enabled: true,
            onboarding: OnboardingState::default(),
            mic_id: None,
            log_level: "info".to_string(),
        }
    }
}
