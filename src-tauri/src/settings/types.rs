use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PasteMethod {
    #[default]
    #[serde(alias = "CtrlV")]
    CtrlV,
    #[serde(alias = "CtrlShiftV")]
    CtrlShiftV,
    #[serde(alias = "Direct")]
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
    pub wake_word_submit: String,
    pub auto_enter_after_wake_word: bool,
    pub silence_timeout_ms: u64,
    pub show_in_dock: bool,
    pub smartmic_enabled: bool,       // Enable SmartMic remote server
    pub smartmic_port: u16,           // Port for SmartMic HTTPS server
    pub smartmic_relay_enabled: bool, // Gate relay usage (relay URL is ignored when false)
    pub smartmic_relay_url: Option<String>, // Relay proxy URL (e.g. "https://smartmic.hospital.com")
    pub smartmic_machine_id_enabled: bool,  // Gate inclusion of machine_id segment in the relay URL
    pub smartmic_machine_id: Option<String>, // Machine identifier included in relay URL when set
    pub smartmic_token_ttl_hours: Option<u64>, // Token TTL in hours (None or 0 = infinite)
    pub smartmic_bind_address: Option<String>, // Explicit IPv4 to bind the SmartMic server on (None = auto-detect)
    pub streaming_preview: bool,
    pub overlay_size: String, // "small" | "medium" | "large"
    pub streaming_text_width: u32,
    pub streaming_font_size: u32,
    pub streaming_max_lines: u32,
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
            // Toggle to Talk is safer on Wayland: the portal does not
            // deliver reliable key-release events on every compositor.
            record_mode: {
                #[cfg(target_os = "linux")]
                {
                    if crate::utils::platform::is_wayland_session() {
                        "toggle_to_talk".to_string()
                    } else {
                        "push_to_talk".to_string()
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    "push_to_talk".to_string()
                }
            },
            overlay_mode: "recording".to_string(),
            overlay_position: "bottom".to_string(),
            api_enabled: false,
            api_port: 4800,
            // Default to true on Wayland so transcriptions remain accessible
            // via manual Ctrl+V when enigo's key injection cannot reach native
            // Wayland apps. Users remain free to disable it.
            copy_to_clipboard: {
                #[cfg(target_os = "linux")]
                {
                    crate::utils::platform::is_wayland_session()
                }
                #[cfg(not(target_os = "linux"))]
                {
                    false
                }
            },
            paste_method: PasteMethod::default(),
            persist_history: false,
            language: "default".to_string(),
            sound_enabled: true,
            onboarding: OnboardingState::default(),
            // Escape alone would be grabbed system-wide by the portal on
            // Wayland, breaking the key in every other app. Chord avoids
            // that while keeping the historical default elsewhere.
            cancel_shortcut: {
                #[cfg(target_os = "linux")]
                {
                    if crate::utils::platform::is_wayland_session() {
                        "ctrl+shift+escape".to_string()
                    } else {
                        "escape".to_string()
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    "escape".to_string()
                }
            },
            mic_id: None,
            mic_label: None,
            log_level: "info".to_string(),
            wake_word_enabled: false,
            wake_word_record: "ok alix".to_string(),
            wake_word_command: "alix command".to_string(),
            wake_word_cancel: "alix cancel".to_string(),
            wake_word_validate: "alix validate".to_string(),
            wake_word_submit: "thank you alix".to_string(),
            auto_enter_after_wake_word: false,
            silence_timeout_ms: 1500,
            show_in_dock: true,
            smartmic_enabled: false,
            smartmic_port: 4801,
            smartmic_relay_enabled: false,
            smartmic_relay_url: None,
            smartmic_machine_id_enabled: false,
            smartmic_machine_id: None,
            smartmic_token_ttl_hours: None,
            smartmic_bind_address: None,
            streaming_preview: false,
            overlay_size: "small".to_string(),
            streaming_text_width: 450,
            streaming_font_size: 11,
            streaming_max_lines: 5,
        }
    }
}
