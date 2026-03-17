use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum PasteMethod {
    #[default]
    CtrlV,
    CtrlShiftV,
    Direct,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct OnboardingState {
    pub used_home_shortcut: bool,
    pub transcribed_outside_app: bool,
    pub added_dictionary_word: bool,
    pub congrats_dismissed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AppSettings {
    pub record_shortcut: String,
    pub last_transcript_shortcut: String,
    pub llm_record_shortcut: String,
    pub command_shortcut: String,
    pub llm_mode_1_shortcut: String,
    pub llm_mode_2_shortcut: String,
    pub llm_mode_3_shortcut: String,
    pub llm_mode_4_shortcut: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dictionary: Vec<String>,
    pub record_mode: String,       // "push_to_talk" | "toggle_to_talk"
    pub overlay_mode: String,      // "hidden" | "recording" | "always"
    pub overlay_position: String,  // "top" | "bottom"
    pub api_enabled: bool,         // Enable local HTTP API
    pub api_port: u16,             // Port for local HTTP API
    pub copy_to_clipboard: bool,   // Keep transcription in clipboard after recording finishes
    pub paste_method: PasteMethod, // Paste method: CtrlV or CtrlShiftV (for terminals)
    pub persist_history: bool,     // Persist last 5 transcriptions to disk
    pub language: String,          // UI language code (e.g., "en", "fr")
    pub sound_enabled: bool,
    pub onboarding: OnboardingState,
    pub cancel_shortcut: String,   // Shortcut to cancel active recording
    pub mic_id: Option<String>,    // Optional microphone device ID
    pub mic_label: Option<String>, // Friendly name of the selected microphone (persisted for disconnected state)
    pub log_level: String,         // "info" | "debug" | "trace" | "warn" | "error"
    pub wake_word_enabled: bool,
    pub wake_word_record: String,
    pub wake_word_command: String,
    pub wake_word_cancel: String,
    pub wake_word_validate: String,
    pub auto_enter_after_wake_word: bool,
    pub speak_shortcut: String,
    pub tts_speed: f32,
    pub tts_voice: String,
    pub history_limit: usize,
    pub audio_export_folder: String,
}

impl Default for AppSettings {
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
            dictionary: Vec::new(),
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
            onboarding: OnboardingState::default(),
            cancel_shortcut: "escape".to_string(),
            mic_id: None,
            mic_label: None,
            log_level: "info".to_string(),
            wake_word_enabled: false,
            wake_word_record: "ok alix".to_string(),
            wake_word_command: "alix command".to_string(),
            wake_word_cancel: "alix cancel".to_string(),
            wake_word_validate: "alix validate".to_string(),
            auto_enter_after_wake_word: false,
            speak_shortcut: "ctrl+intlbackslash".to_string(),
            tts_speed: 1.0,
            tts_voice: "af_heart".to_string(),
            history_limit: 5,
            audio_export_folder: String::new(),
        }
    }
}
