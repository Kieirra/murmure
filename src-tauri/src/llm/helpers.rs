use crate::llm::types::{LLMConnectSettings, SecretString};
use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};
use tauri::{AppHandle, Manager};
use url::{Host, Url};

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
    let mut needs_save = false;

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
            wake_word: "alix general".to_string(),
        };
        settings.modes.push(mode);
        settings.active_mode_index = 0;

        // Clear legacy prompt to mark as migrated (optional, but cleaner)
        settings.prompt = String::new();

        needs_save = true;
    }

    // Migrate wake_word for existing modes that have an empty wake_word
    for mode in &mut settings.modes {
        if mode.wake_word.is_empty() {
            mode.wake_word = format!("alix {}", mode.name.to_lowercase());
            needs_save = true;
        }
    }

    if needs_save {
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

pub fn restart_wake_word_if_active(app: &AppHandle) {
    let app_settings = crate::settings::load_settings(app);
    if app_settings.wake_word_enabled {
        crate::wake_word::stop_listener(app);
        crate::wake_word::start_listener(app);
    }
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

pub fn load_remote_api_key() -> Option<SecretString> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_REMOTE_API_KEY).ok()?;
    entry.get_password().ok().map(SecretString::new)
}

pub fn has_remote_api_key() -> bool {
    load_remote_api_key()
        .map(|k| !k.is_empty())
        .unwrap_or(false)
}

pub fn load_remote_api_key_masked() -> String {
    match load_remote_api_key() {
        Some(key) if !key.is_empty() => {
            let exposed = key.expose();
            if exposed.chars().count() > 8 {
                let suffix: String = exposed
                    .chars()
                    .rev()
                    .take(4)
                    .collect::<Vec<char>>()
                    .into_iter()
                    .rev()
                    .collect();
                format!(
                    "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}{}",
                    suffix
                )
            } else {
                "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}".to_string()
            }
        }
        _ => String::new(),
    }
}

pub fn validate_url(url: &str) -> Result<(), String> {
    let parsed = Url::parse(url).map_err(|_| "Invalid URL format".to_string())?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Invalid URL: must use http:// or https://".to_string());
    }
    if parsed.host().is_none() {
        return Err("Invalid URL: missing host".to_string());
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("Invalid URL: userinfo is not allowed".to_string());
    }
    Ok(())
}

pub fn is_url_secure_for_api_key(url: &str) -> bool {
    let parsed = match Url::parse(url) {
        Ok(value) => value,
        Err(_) => return false,
    };

    if parsed.scheme() == "https" {
        return true;
    }
    if parsed.scheme() != "http" {
        return false;
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return false;
    }

    match parsed.host() {
        Some(Host::Domain(host)) => host.eq_ignore_ascii_case("localhost"),
        Some(Host::Ipv4(ipv4)) => {
            let ip = IpAddr::V4(ipv4);
            is_local_or_private_ip(ip)
        }
        Some(Host::Ipv6(ipv6)) => {
            let ip = IpAddr::V6(ipv6);
            is_local_or_private_ip(ip)
        }
        None => false,
    }
}

pub fn validate_remote_request(url: &str, api_key: Option<&str>) -> Result<(), String> {
    validate_url(url)?;
    let has_key = api_key.map(|k| !k.is_empty()).unwrap_or(false);
    if has_key && !is_url_secure_for_api_key(url) {
        return Err(
            "Cannot send API key over an unencrypted HTTP connection. Use HTTPS or a local address."
                .to_string(),
        );
    }
    Ok(())
}

fn is_local_or_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => is_local_or_private_ipv4(ipv4),
        IpAddr::V6(ipv6) => ipv6.is_loopback(),
    }
}

fn is_local_or_private_ipv4(ipv4: Ipv4Addr) -> bool {
    ipv4.is_loopback() || ipv4.is_private()
}
