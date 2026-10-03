use crate::llm::types::{
    CustomPromptError, LLMConnectSettings, LLMProvider, PromptSelectionError, SecretString,
    API_INSTRUCTION_MAX_CHARS,
};
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

    let (mut settings, raw) = match fs::read_to_string(&path) {
        Ok(content) => (
            serde_json::from_str::<LLMConnectSettings>(&content).unwrap_or_default(),
            serde_json::from_str::<serde_json::Value>(&content).ok(),
        ),
        Err(_) => {
            let defaults = LLMConnectSettings::default();
            let _ = save_llm_connect_settings(app, &defaults);
            (defaults, None)
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

    if migrate_missing_keys(&mut settings, raw.as_ref()) {
        needs_save = true;
    }

    if needs_save {
        let _ = save_llm_connect_settings(app, &settings);
    }

    settings
}

pub fn migrate_missing_keys(
    settings: &mut LLMConnectSettings,
    raw: Option<&serde_json::Value>,
) -> bool {
    let has_key = |pointer: &str| raw.is_some_and(|value| value.pointer(pointer).is_some());
    let command_migrated = migrate_command(settings, has_key("/command"));
    let enabled_migrated =
        migrate_enabled(settings, has_key("/enabled"), has_key("/command/enabled"));
    command_migrated || enabled_migrated
}

fn migrate_command(settings: &mut LLMConnectSettings, has_command_key: bool) -> bool {
    if has_command_key || !settings.onboarding_completed {
        return false;
    }
    let inherited = settings
        .modes
        .get(settings.active_mode_index)
        .or_else(|| settings.modes.first())
        .map(|mode| (mode.provider.clone(), mode.model.clone()));
    if let Some((provider, model)) = inherited {
        settings.command.provider = provider;
        settings.command.model = model;
    }
    true
}

fn migrate_enabled(
    settings: &mut LLMConnectSettings,
    has_enabled_key: bool,
    has_command_enabled_key: bool,
) -> bool {
    if !has_enabled_key {
        settings.enabled = settings.onboarding_completed;
    }
    if !has_command_enabled_key {
        settings.command.enabled = settings.onboarding_completed;
    }
    !has_enabled_key || !has_command_enabled_key
}

pub fn active_prompt_name(app: &AppHandle) -> Option<String> {
    let settings = load_llm_connect_settings(app);
    if !settings.is_enabled() {
        return None;
    }
    settings
        .modes
        .get(settings.active_mode_index)
        .map(|m| m.name.clone())
}

pub fn resolve_api_prompt(
    settings: &LLMConnectSettings,
    name: &str,
) -> Result<usize, PromptSelectionError> {
    if !settings.is_enabled() {
        return Err(PromptSelectionError::Disabled);
    }
    let name = name.trim();
    let Some((index, mode)) = settings
        .modes
        .iter()
        .enumerate()
        .find(|(_, mode)| mode.name.trim() == name)
    else {
        return Err(PromptSelectionError::NotFound {
            name: name.to_string(),
            available: settings.mode_names(),
        });
    };
    if mode.prompt.trim().is_empty() || mode.model.trim().is_empty() {
        return Err(PromptSelectionError::NotConfigured {
            name: name.to_string(),
        });
    }
    Ok(index)
}

pub fn check_api_custom_prompt(
    settings: &LLMConnectSettings,
    provider: &LLMProvider,
    instruction: &str,
) -> Result<(), CustomPromptError> {
    if instruction.chars().count() > API_INSTRUCTION_MAX_CHARS {
        return Err(CustomPromptError::TooLong);
    }
    if !settings.is_enabled() {
        return Err(CustomPromptError::Disabled);
    }
    match provider {
        LLMProvider::Remote if settings.remote_url.trim().is_empty() => {
            Err(CustomPromptError::RemoteNotConfigured)
        }
        _ => Ok(()),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::types::{LLMMode, LLMProvider};

    fn configured_settings() -> LLMConnectSettings {
        LLMConnectSettings {
            modes: vec![LLMMode {
                name: "General".to_string(),
                prompt: "p".to_string(),
                model: "qwen3:8b".to_string(),
                provider: LLMProvider::Remote,
                wake_word: String::new(),
            }],
            onboarding_completed: true,
            ..LLMConnectSettings::default()
        }
    }

    fn mode(name: &str, prompt: &str, model: &str) -> LLMMode {
        LLMMode {
            name: name.to_string(),
            prompt: prompt.to_string(),
            model: model.to_string(),
            provider: LLMProvider::Local,
            wake_word: String::new(),
        }
    }

    fn prompt_mode_settings() -> LLMConnectSettings {
        LLMConnectSettings {
            modes: vec![
                mode("General", "p", "qwen3:8b"),
                mode("Email", "p", "qwen3:8b"),
            ],
            onboarding_completed: true,
            enabled: true,
            ..LLMConnectSettings::default()
        }
    }

    #[test]
    fn should_resolve_a_saved_prompt_when_the_name_matches_after_trim() {
        let settings = prompt_mode_settings();

        assert_eq!(resolve_api_prompt(&settings, "Email"), Ok(1));
        assert_eq!(resolve_api_prompt(&settings, "  Email  "), Ok(1));
    }

    #[test]
    fn should_list_the_saved_prompts_when_the_name_differs_in_case() {
        let settings = prompt_mode_settings();

        let result = resolve_api_prompt(&settings, "email");

        assert_eq!(
            result,
            Err(PromptSelectionError::NotFound {
                name: "email".to_string(),
                available: vec!["General".to_string(), "Email".to_string()],
            })
        );
    }

    #[test]
    fn should_refuse_a_saved_prompt_when_prompt_mode_is_disabled() {
        let mut settings = prompt_mode_settings();
        settings.enabled = false;

        let result = resolve_api_prompt(&settings, "Email");

        assert_eq!(result, Err(PromptSelectionError::Disabled));
    }

    #[test]
    fn should_refuse_a_saved_prompt_when_its_prompt_or_model_is_empty() {
        let mut settings = prompt_mode_settings();
        settings.modes[0].prompt = String::new();
        settings.modes[1].model = " ".to_string();

        let not_configured = |name: &str| {
            Err(PromptSelectionError::NotConfigured {
                name: name.to_string(),
            })
        };
        assert_eq!(
            resolve_api_prompt(&settings, "General"),
            not_configured("General")
        );
        assert_eq!(
            resolve_api_prompt(&settings, "Email"),
            not_configured("Email")
        );
    }

    #[test]
    fn should_accept_a_custom_prompt_when_the_instruction_has_the_maximum_length() {
        let settings = prompt_mode_settings();
        let instruction = "a".repeat(API_INSTRUCTION_MAX_CHARS);

        let result = check_api_custom_prompt(&settings, &LLMProvider::Local, &instruction);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn should_refuse_a_custom_prompt_when_the_instruction_is_too_long() {
        let settings = prompt_mode_settings();
        let instruction = "a".repeat(API_INSTRUCTION_MAX_CHARS + 1);

        let result = check_api_custom_prompt(&settings, &LLMProvider::Local, &instruction);

        assert_eq!(result, Err(CustomPromptError::TooLong));
    }

    #[test]
    fn should_refuse_a_custom_prompt_when_prompt_mode_is_disabled() {
        let mut settings = prompt_mode_settings();
        settings.enabled = false;

        let result = check_api_custom_prompt(&settings, &LLMProvider::Local, "Summarize");

        assert_eq!(result, Err(CustomPromptError::Disabled));
    }

    #[test]
    fn should_refuse_a_remote_custom_prompt_when_no_remote_server_is_set() {
        let settings = prompt_mode_settings();

        let result = check_api_custom_prompt(&settings, &LLMProvider::Remote, "Summarize");

        assert_eq!(result, Err(CustomPromptError::RemoteNotConfigured));
    }

    #[test]
    fn should_copy_the_active_mode_into_command_when_the_key_is_missing() {
        let mut settings = configured_settings();
        assert!(migrate_command(&mut settings, false));
        assert_eq!(settings.command.provider, LLMProvider::Remote);
        assert_eq!(settings.command.model, "qwen3:8b");
    }

    #[test]
    fn should_keep_command_when_the_key_is_present() {
        let mut settings = configured_settings();
        assert!(!migrate_command(&mut settings, true));
        assert_eq!(settings.command.provider, LLMProvider::Local);
        assert_eq!(settings.command.model, "");
    }

    #[test]
    fn should_keep_command_when_onboarding_is_not_completed() {
        let mut settings = configured_settings();
        settings.onboarding_completed = false;
        assert!(!migrate_command(&mut settings, false));
        assert_eq!(settings.command.provider, LLMProvider::Local);
        assert_eq!(settings.command.model, "");
    }

    #[test]
    fn should_enable_both_extensions_when_the_keys_are_missing_and_onboarding_is_done() {
        let mut settings = configured_settings();
        assert!(migrate_enabled(&mut settings, false, false));
        assert!(settings.enabled);
        assert!(settings.command.enabled);
    }

    #[test]
    fn should_keep_both_extensions_off_when_the_keys_are_missing_and_onboarding_is_not_done() {
        let mut settings = configured_settings();
        settings.onboarding_completed = false;
        assert!(migrate_enabled(&mut settings, false, false));
        assert!(!settings.enabled);
        assert!(!settings.command.enabled);
    }

    #[test]
    fn should_keep_the_enabled_flags_when_the_keys_are_present() {
        let mut settings = configured_settings();
        settings.command.enabled = true;
        assert!(!migrate_enabled(&mut settings, true, true));
        assert!(!settings.enabled);
        assert!(settings.command.enabled);
    }

    #[test]
    fn should_migrate_an_old_file_into_two_active_extensions() {
        let json = r#"{"modes":[{"name":"General","prompt":"p","model":"gpt-4.1-mini","provider":"remote"}],"active_mode_index":0,"onboarding_completed":true}"#;
        let raw: serde_json::Value = serde_json::from_str(json).unwrap();
        let mut settings: LLMConnectSettings = serde_json::from_str(json).unwrap();
        assert!(migrate_missing_keys(&mut settings, Some(&raw)));
        assert!(settings.enabled);
        assert!(settings.command.enabled);
        assert_eq!(settings.command.provider, LLMProvider::Remote);
        assert_eq!(settings.command.model, "gpt-4.1-mini");
    }

    #[test]
    fn should_only_fill_command_enabled_when_command_has_no_enabled_key() {
        let json = r#"{"modes":[],"onboarding_completed":true,"enabled":false,"command":{"provider":"local","model":"qwen3:8b"}}"#;
        let raw: serde_json::Value = serde_json::from_str(json).unwrap();
        let mut settings: LLMConnectSettings = serde_json::from_str(json).unwrap();
        assert!(migrate_missing_keys(&mut settings, Some(&raw)));
        assert!(!settings.enabled);
        assert!(settings.command.enabled);
        assert_eq!(settings.command.model, "qwen3:8b");
    }
}
