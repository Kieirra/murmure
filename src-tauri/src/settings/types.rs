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
    pub command_shortcut: String,
    pub llm_mode_1_shortcut: String,
    pub llm_mode_2_shortcut: String,
    pub llm_mode_3_shortcut: String,
    pub llm_mode_4_shortcut: String,
    pub voice_mode_toggle_shortcut: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dictionary: Vec<String>,
    pub record_mode: String,      // "push_to_talk" | "toggle_to_talk"
    pub overlay_mode: String,     // "hidden" | "recording" | "always"
    pub overlay_position: String, // "top" | "bottom"
    pub api_enabled: bool,
    pub api_port: u16,
    pub copy_to_clipboard: bool, // Keeps transcription in clipboard after recording finishes
    pub paste_method: PasteMethod,
    pub persist_history: bool, // Persists last 5 transcriptions to disk
    pub language: String,      // UI language code (e.g., "en", "fr")
    pub sound_enabled: bool,
    pub onboarding: OnboardingState,
    pub cancel_shortcut: String,
    pub mic_id: Option<String>,
    pub mic_label: Option<String>, // Persisted so the disconnected-mic UI can still show a friendly name
    pub log_level: String,         // "info" | "debug" | "trace" | "warn" | "error"
    // Debug aid: keep the recorded WAV files in the system temp dir after
    // transcription instead of deleting them.
    pub keep_recordings: bool,
    pub wake_word_enabled: bool,
    pub wake_word_record: String,
    pub wake_word_command: String,
    pub wake_word_cancel: String,
    pub wake_word_validate: String,
    pub wake_word_submit: String,
    pub auto_enter_after_wake_word: bool,
    pub silence_timeout_ms: u64,
    pub show_in_dock: bool,
    pub smartmic_enabled: bool,
    pub smartmic_port: u16,
    pub smartmic_relay_enabled: bool, // Relay URL is ignored when false
    pub smartmic_relay_url: Option<String>, // e.g. "https://smartmic.hospital.com"
    pub smartmic_machine_id_enabled: bool, // Gates inclusion of machine_id in the relay URL
    pub smartmic_machine_id: Option<String>,
    pub smartmic_token_ttl_hours: Option<u64>, // None or 0 means infinite
    pub smartmic_bind_address: Option<String>, // None means auto-detect
    pub streaming_preview: bool,
    pub long_dictation_enabled: bool,
    pub long_dictation_silence_ms: u64,
    pub overlay_size: String, // "small" | "medium" | "large"
    pub streaming_text_width: u32,
    pub streaming_font_size: u32,
    pub streaming_max_lines: u32,
    // Linux only. Persists the user's dismissal of `WaylandModeNotice`
    // so the onboarding banner does not reappear on next launch.
    pub wayland_notice_dismissed: bool,
    // Linux only. Persists the user's dismissal of the Wayland clipboard
    // fallback onboarding card (manual Ctrl+V hint when auto-paste fails).
    pub wayland_clipboard_fallback_dismissed: bool,
}

impl Default for AppSettings {
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
            dictionary: Vec::new(),
            // Wayland CLI mode binds shortcuts at the OS level, so Murmure receives no key-release events. Push-to-talk is impossible.
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
            // gtk-layer-shell handles focus/positioning only on wlr-layer-shell
            // compositors. Mutter (GNOME) and Muffin (Cinnamon) lack it, so the
            // Tauri fallback misplaces the overlay and steals focus there.
            overlay_mode: {
                #[cfg(target_os = "linux")]
                {
                    if crate::utils::platform::is_wayland_session()
                        && !crate::utils::platform::is_portal_reliable_desktop()
                    {
                        "hidden".to_string()
                    } else {
                        "recording".to_string()
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    "recording".to_string()
                }
            },
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
            cancel_shortcut: "ctrl+backspace".to_string(),
            mic_id: None,
            mic_label: None,
            log_level: "info".to_string(),
            keep_recordings: false,
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
            streaming_preview: true,
            long_dictation_enabled: false,
            long_dictation_silence_ms: 800,
            overlay_size: "small".to_string(),
            streaming_text_width: 450,
            streaming_font_size: 11,
            streaming_max_lines: 5,
            wayland_notice_dismissed: false,
            wayland_clipboard_fallback_dismissed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_shortcut_survives_serde_round_trip() {
        let mut settings = AppSettings::default();
        settings.record_shortcut = String::new();

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.record_shortcut, "");
    }

    #[test]
    fn long_dictation_defaults() {
        let settings = AppSettings::default();
        assert!(!settings.long_dictation_enabled);
        assert_eq!(settings.long_dictation_silence_ms, 800);
    }

    #[test]
    fn long_dictation_missing_fields_load_defaults() {
        let json = serde_json::to_string(&AppSettings::default()).unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = value.as_object_mut().unwrap();
        obj.remove("long_dictation_enabled");
        obj.remove("long_dictation_silence_ms");

        let parsed: AppSettings = serde_json::from_value(value).unwrap();

        assert!(!parsed.long_dictation_enabled);
        assert_eq!(parsed.long_dictation_silence_ms, 800);
    }
}
