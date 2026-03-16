use crate::dictionary;
use crate::llm::helpers::{
    load_llm_connect_settings, load_remote_api_key, validate_remote_request, validate_url,
};
use crate::llm::types::SecretString;
use crate::llm::types::{
    LLMConnectSettings, LLMProvider, OllamaGenerateRequest, OllamaGenerateResponse, OllamaModel,
    OllamaOptions, OllamaPullRequest, OllamaPullResponse, OllamaTagsResponse, OpenAIChatMessage,
    OpenAIChatRequest, OpenAIChatResponse, OpenAIModelsResponse,
};
use log::warn;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

fn build_http_client(timeout_secs: u64) -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

fn with_bearer_auth(
    request: reqwest::RequestBuilder,
    api_key: Option<&str>,
) -> reqwest::RequestBuilder {
    match api_key {
        Some(key) if !key.is_empty() => request.header("Authorization", format!("Bearer {}", key)),
        _ => request,
    }
}

fn map_remote_http_error(status: reqwest::StatusCode) -> String {
    match status.as_u16() {
        401 | 403 => "Authentication failed. Check your API key.".to_string(),
        _ => format!("Server returned error: {}", status),
    }
}

/// Extracts the `<role>...</role>` block from the prompt as a system prompt.
/// Returns (system_prompt, user_prompt). If no `<role>` tag is found,
/// system_prompt is None and the full prompt is returned as user_prompt.
fn extract_system_prompt(prompt: &str) -> (Option<String>, String) {
    let Some(start) = prompt.find("<role>") else {
        return (None, prompt.to_string());
    };
    let Some(end) = prompt.find("</role>") else {
        return (None, prompt.to_string());
    };

    let system = prompt[start + "<role>".len()..end].trim().to_string();
    let user = format!(
        "{}{}",
        prompt[..start].trim(),
        prompt[end + "</role>".len()..].trim()
    )
    .trim()
    .to_string();

    if system.is_empty() {
        (None, user)
    } else {
        (Some(system), user)
    }
}

async fn generate_local(
    url: &str,
    model: &str,
    system_prompt: Option<&str>,
    user_prompt: &str,
) -> Result<String, String> {
    let client = build_http_client(120)?;
    let url = format!("{}/generate", normalize_url(url));

    let request_body = OllamaGenerateRequest {
        model: model.to_string(),
        prompt: user_prompt.to_string(),
        stream: false,
        options: Some(OllamaOptions { temperature: 0.0 }),
        system: system_prompt.map(|s| s.to_string()),
        think: false,
    };

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API returned error: {}", response.status()));
    }

    let ollama_response: OllamaGenerateResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    Ok(ollama_response.response.trim().to_string())
}

async fn generate_remote(
    remote_url: &str,
    api_key: Option<&SecretString>,
    model: &str,
    system_prompt: Option<&str>,
    user_prompt: &str,
) -> Result<String, String> {
    let key_str = api_key.map(|k| k.expose());
    validate_remote_request(remote_url, key_str)?;

    let client = build_http_client(60)?;
    let url = format!("{}/chat/completions", normalize_url(remote_url));

    let mut messages = Vec::new();
    if let Some(system) = system_prompt {
        messages.push(OpenAIChatMessage {
            role: "system".to_string(),
            content: system.to_string(),
        });
    }
    messages.push(OpenAIChatMessage {
        role: "user".to_string(),
        content: user_prompt.to_string(),
    });

    let request_body = OpenAIChatRequest {
        model: model.to_string(),
        messages,
        temperature: 0.0,
        stream: false,
        think: None,
    };

    let request = with_bearer_auth(client.post(&url).json(&request_body), key_str);

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to connect to remote server: {}", e))?;

    if !response.status().is_success() {
        return Err(map_remote_http_error(response.status()));
    }

    let chat_response: OpenAIChatResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse remote response: {}", e))?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string())
        .ok_or_else(|| "Remote server returned empty response".to_string())
}

async fn dispatch_to_llm(
    app: &AppHandle,
    settings: &LLMConnectSettings,
    system_prompt: Option<&str>,
    user_prompt: &str,
) -> Result<String, String> {
    let active_mode = settings
        .modes
        .get(settings.active_mode_index)
        .ok_or("No active mode selected")?;

    if active_mode.model.is_empty() {
        return Err("No model selected".to_string());
    }

    let _ = app.emit("llm-processing-start", ());

    let result = match active_mode.provider {
        LLMProvider::Local => {
            generate_local(
                &settings.url,
                &active_mode.model,
                system_prompt,
                user_prompt,
            )
            .await
        }
        LLMProvider::Remote => {
            let api_key = load_remote_api_key();
            generate_remote(
                &settings.remote_url,
                api_key.as_ref(),
                &active_mode.model,
                system_prompt,
                user_prompt,
            )
            .await
        }
    };

    let _ = app.emit("llm-processing-end", ());
    result
}

pub async fn post_process_with_llm(
    app: &AppHandle,
    transcription: String,
    force_bypass: bool,
) -> Result<String, String> {
    if force_bypass {
        return Ok(transcription);
    }

    let settings = load_llm_connect_settings(app);

    let active_mode = settings
        .modes
        .get(settings.active_mode_index)
        .ok_or("No active mode selected")?;

    let dictionary_words = dictionary::load(app)
        .unwrap_or_default()
        .into_keys()
        .collect::<Vec<String>>()
        .join(", ");

    let prompt = active_mode
        .prompt
        .replace("{{TRANSCRIPT}}", &transcription)
        .replace("{transcript}", &transcription)
        .replace("{{DICTIONARY}}", &dictionary_words)
        .replace("{dictionary}", &dictionary_words);

    let (system_prompt, user_prompt) = extract_system_prompt(&prompt);
    dispatch_to_llm(app, &settings, system_prompt.as_deref(), &user_prompt).await
}

pub async fn process_command_with_llm(
    app: &AppHandle,
    system_prompt: String,
    user_prompt: String,
) -> Result<String, String> {
    let settings = load_llm_connect_settings(app);
    dispatch_to_llm(app, &settings, Some(&system_prompt), &user_prompt).await
}

pub async fn test_ollama_connection(url: String) -> Result<bool, String> {
    validate_url(&url)?;
    let client = build_http_client(10)?;
    let test_url = format!("{}/tags", normalize_url(&url));

    let response = client
        .get(&test_url)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if response.status().is_success() {
        Ok(true)
    } else {
        Err(format!("Server returned error: {}", response.status()))
    }
}

pub async fn fetch_ollama_models(url: String) -> Result<Vec<OllamaModel>, String> {
    validate_url(&url)?;
    let client = build_http_client(10)?;
    let tags_url = format!("{}/tags", normalize_url(&url));

    let response = client
        .get(&tags_url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch models: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }

    let tags_response: OllamaTagsResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(tags_response.models)
}

async fn fetch_openai_models_raw(
    url: &str,
    api_key: Option<&SecretString>,
) -> Result<OpenAIModelsResponse, String> {
    let key_str = api_key.map(|k| k.expose());
    validate_remote_request(url, key_str)?;

    let client = build_http_client(10)?;
    let models_url = format!("{}/models", normalize_url(url));

    let request = with_bearer_auth(client.get(&models_url), key_str);

    let response = request
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if !response.status().is_success() {
        return Err(map_remote_http_error(response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn test_remote_connection(
    url: String,
    api_key: Option<SecretString>,
) -> Result<usize, String> {
    let response = fetch_openai_models_raw(&url, api_key.as_ref()).await?;
    Ok(response.data.len())
}

pub async fn fetch_remote_models(
    url: String,
    api_key: Option<SecretString>,
) -> Result<Vec<OllamaModel>, String> {
    let response = fetch_openai_models_raw(&url, api_key.as_ref()).await?;
    Ok(response
        .data
        .into_iter()
        .map(|m| OllamaModel { name: m.id })
        .collect())
}

pub async fn pull_ollama_model(app: AppHandle, url: String, model: String) -> Result<(), String> {
    validate_url(&url)?;
    let client = build_http_client(300)?;
    let pull_url = format!("{}/pull", normalize_url(&url));

    let request_body = OllamaPullRequest {
        model: model.clone(),
        stream: true,
    };

    let mut response = client
        .post(&pull_url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API returned error: {}", response.status()));
    }

    let mut buffer = String::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(pos) = buffer.find('\n') {
            let line: String = buffer.drain(..=pos).collect();
            if let Ok(pull_response) = serde_json::from_str::<OllamaPullResponse>(line.trim()) {
                let _ = app.emit("llm-pull-progress", pull_response);
            }
        }
    }

    Ok(())
}

pub async fn warmup_ollama_model(app: &AppHandle) -> Result<(), String> {
    let settings = load_llm_connect_settings(app);

    if settings.modes.is_empty() || settings.url.trim().is_empty() {
        return Ok(());
    }
    let active_mode = match settings.modes.get(settings.active_mode_index) {
        Some(mode) => mode,
        None => return Ok(()),
    };
    if active_mode.model.trim().is_empty() {
        return Ok(());
    }

    if active_mode.provider == LLMProvider::Remote {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/generate", normalize_url(&settings.url));

    let request_body = OllamaGenerateRequest {
        model: active_mode.model.clone(),
        prompt: " ".to_string(),
        stream: false,
        options: Some(OllamaOptions { temperature: 0.0 }),
        system: None,
        think: false,
    };

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama for warmup: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Ollama warmup returned error: {}",
            response.status()
        ));
    }

    Ok(())
}

pub fn warmup_ollama_model_background(app: &AppHandle) {
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = warmup_ollama_model(&app_handle).await {
            warn!("LLM warmup failed: {}", e);
        }
    });
}

pub fn switch_active_mode(app: &AppHandle, index: usize) {
    let mut settings = load_llm_connect_settings(app);

    if index < settings.modes.len() && settings.active_mode_index != index {
        settings.active_mode_index = index;
        let mode_name = settings.modes[index].name.clone();

        if crate::llm::helpers::save_llm_connect_settings(app, &settings).is_ok() {
            let _ = app.emit("llm-settings-updated", &settings);
            let _ = app.emit("overlay-feedback", mode_name);
            crate::overlay::overlay::show_recording_overlay(app);
            let app_handle = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(1000));
                let current_settings = crate::settings::load_settings(&app_handle);
                if current_settings.overlay_mode.as_str() == "always" {
                    return;
                }
                let is_recording = app_handle
                    .state::<crate::audio::types::AudioState>()
                    .recorder
                    .lock()
                    .is_some();
                if !is_recording {
                    crate::overlay::overlay::hide_recording_overlay(&app_handle);
                }
            });
        }
    }
}
