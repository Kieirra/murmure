use crate::llm::types::LLMConnectSettings;
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

const KEYRING_SERVICE: &str = "murmure";
const KEYRING_REMOTE_API_KEY: &str = "remote_api_key";

/// Default prompt for the "General" mode when no prompt is configured.
/// This ensures LLM Connect works out-of-the-box at first installation.
const DEFAULT_GENERAL_PROMPT: &str = r#"<role>
Your role is to correct a transcription produced by an ASR. You are not a conversational assistant.
</role>

<instructions>
Correct only the following text according to these strict rules:
- Correct spelling and grammar.
- Remove repetitions and hesitations.
- Replace misrecognized words only if they are phonetically similar to a word from the dictionary. Here are the dictionary words: <lexicon>{{DICTIONARY}}</lexicon>
- Structure the text into paragraphs or bullet points only if it clearly improves readability.
- Never modify the meaning or the content.
- Do not answer questions and do not comment on them.
- Remove all '*' characters and never add any.
- Do not generate any comment or introduction.
- If you do not know or if there is nothing to modify, return the transcription as is.
</instructions>

<input>{{TRANSCRIPT}}</input>
"#;

fn llm_connect_settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Err(e) = fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all failed: {}", e));
    }
    Ok(dir.join("llm_connect.json"))
}

pub fn load_llm_connect_settings(app: &AppHandle) -> LLMConnectSettings {
    let path = match llm_connect_settings_path(app) {
        Ok(p) => p,
        Err(_) => return LLMConnectSettings::default(),
    };

    let mut settings = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<LLMConnectSettings>(&content).unwrap_or_default(),
        Err(_) => {
            let defaults = LLMConnectSettings::default();
            let _ = save_llm_connect_settings(app, &defaults);
            defaults
        }
    };

    // Migration / Initialization Logic
    if settings.modes.is_empty() {
        // Use default prompt if the legacy prompt field is empty
        let prompt = if settings.prompt.trim().is_empty() {
            DEFAULT_GENERAL_PROMPT.to_string()
        } else {
            settings.prompt.clone()
        };

        let mode = crate::llm::types::LLMMode {
            name: "General".to_string(),
            prompt,
            model: settings.model.clone(),
            shortcut: "Ctrl+Shift+1".to_string(),
            provider: crate::llm::types::LLMProvider::default(),
        };
        settings.modes.push(mode);
        settings.active_mode_index = 0;

        // Clear legacy prompt to mark as migrated (optional, but cleaner)
        settings.prompt = String::new();

        let _ = save_llm_connect_settings(app, &settings);
    }

    settings
}

pub fn save_llm_connect_settings(
    app: &AppHandle,
    settings: &LLMConnectSettings,
) -> Result<(), String> {
    let path = llm_connect_settings_path(app)?;
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn store_remote_api_key(api_key: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_REMOTE_API_KEY)
        .map_err(|e| format!("Failed to access keyring: {}", e))?;
    if api_key.is_empty() {
        let _ = entry.delete_credential();
        Ok(())
    } else {
        entry
            .set_password(api_key)
            .map_err(|e| format!("Failed to store API key: {}", e))
    }
}

pub fn load_remote_api_key() -> Option<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_REMOTE_API_KEY).ok()?;
    entry.get_password().ok()
}

pub fn has_remote_api_key() -> bool {
    load_remote_api_key()
        .map(|k| !k.is_empty())
        .unwrap_or(false)
}

pub fn load_remote_api_key_masked() -> String {
    match load_remote_api_key() {
        Some(key) if !key.is_empty() => {
            if key.len() > 8 {
                let suffix = &key[key.len() - 4..];
                format!("\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}{}", suffix)
            } else {
                "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}".to_string()
            }
        }
        _ => String::new(),
    }
}

pub fn validate_url(url: &str) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Invalid URL: must start with http:// or https://".to_string());
    }
    Ok(())
}

pub fn is_url_secure_for_api_key(url: &str) -> bool {
    if url.starts_with("https://") {
        return true;
    }

    // Extract hostname from URL
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    let host = without_scheme
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("");

    matches!(
        host,
        "localhost" | "127.0.0.1"
    ) || host.starts_with("192.168.")
        || host.starts_with("10.")
        || is_private_172(host)
}

fn is_private_172(host: &str) -> bool {
    if let Some(rest) = host.strip_prefix("172.") {
        if let Some(second_octet_str) = rest.split('.').next() {
            if let Ok(second_octet) = second_octet_str.parse::<u8>() {
                return (16..=31).contains(&second_octet);
            }
        }
    }
    false
}
