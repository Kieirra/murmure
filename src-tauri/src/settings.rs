use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

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
    #[serde(default)]
    pub llm_record_shortcut: String,
    pub dictionary: Vec<String>,
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
    pub onboarding: OnboardingState,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            record_shortcut: "ctrl+space".to_string(),
            last_transcript_shortcut: "ctrl+shift+space".to_string(),
            llm_record_shortcut: "ctrl+alt+space".to_string(),
            dictionary: Vec::new(),
            overlay_mode: "recording".to_string(),
            overlay_position: "bottom".to_string(),
            api_enabled: false,
            api_port: 4800,
            copy_to_clipboard: false,
            persist_history: true,
            language: "default".to_string(),
            onboarding: OnboardingState::default(),
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Err(e) = fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all failed: {}", e));
    }
    Ok(dir.join("settings.json"))
}

pub fn load_settings(app: &AppHandle) -> AppSettings {
    let path = match settings_path(app) {
        Ok(p) => p,
        Err(_) => return AppSettings::default(),
    };

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<AppSettings>(&content).unwrap_or_default(),
        Err(_) => {
            let defaults = AppSettings::default();
            let _ = save_settings(app, &defaults);
            defaults
        }
    }
}

pub fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}
