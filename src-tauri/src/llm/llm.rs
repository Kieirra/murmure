use crate::dictionary;
use crate::llm::helpers::{
    is_url_secure_for_api_key, load_llm_connect_settings, load_remote_api_key, validate_url,
};
use crate::llm::types::{
    LLMProvider, OllamaGenerateRequest, OllamaGenerateResponse, OllamaModel, OllamaOptions,
    OllamaPullRequest, OllamaPullResponse, OllamaTagsResponse, OpenAIChatMessage,
    OpenAIChatRequest, OpenAIChatResponse, OpenAIModelsResponse,
};
use log::warn;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

async fn generate_local(url: &str, model: &str, prompt: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let url = format!("{}/generate", url.trim_end_matches('/'));

    let request_body = OllamaGenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options: Some(OllamaOptions { temperature: 0.0 }),
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
    api_key: Option<&str>,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    validate_url(remote_url)?;

    let has_key = api_key.map(|k| !k.is_empty()).unwrap_or(false);
    if has_key && !is_url_secure_for_api_key(remote_url) {
        return Err(
            "Cannot send API key over an unencrypted HTTP connection. Use HTTPS or a local address."
                .to_string(),
        );
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("{}/chat/completions", remote_url.trim_end_matches('/'));

    let request_body = OpenAIChatRequest {
        model: model.to_string(),
        messages: vec![OpenAIChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        temperature: 0.0,
        stream: false,
    };

    let mut request = client.post(&url).json(&request_body);
    if let Some(key) = api_key {
        if !key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to connect to remote server: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        return match status.as_u16() {
            401 | 403 => Err("Authentication failed. Check your API key.".to_string()),
            _ => Err(format!("Remote API returned error: {}", status)),
        };
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

    if active_mode.model.is_empty() {
        return Err("No model selected".to_string());
    }

    let _ = app.emit("llm-processing-start", ());

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

    let result = match active_mode.provider {
        LLMProvider::Local => generate_local(&settings.url, &active_mode.model, &prompt).await,
        LLMProvider::Remote => {
            let api_key = load_remote_api_key();
            generate_remote(
                &settings.remote_url,
                api_key.as_deref(),
                &active_mode.model,
                &prompt,
            )
            .await
        }
    };

    let _ = app.emit("llm-processing-end", ());
    result
}

pub async fn process_command_with_llm(app: &AppHandle, prompt: String) -> Result<String, String> {
    let settings = load_llm_connect_settings(app);
    let active_mode = settings
        .modes
        .get(settings.active_mode_index)
        .ok_or("No active mode selected")?;

    if active_mode.model.is_empty() {
        return Err("No model selected".to_string());
    }

    let _ = app.emit("llm-processing-start", ());

    let result = match active_mode.provider {
        LLMProvider::Local => generate_local(&settings.url, &active_mode.model, &prompt).await,
        LLMProvider::Remote => {
            let api_key = load_remote_api_key();
            generate_remote(
                &settings.remote_url,
                api_key.as_deref(),
                &active_mode.model,
                &prompt,
            )
            .await
        }
    };

    let _ = app.emit("llm-processing-end", ());
    result
}

pub async fn test_ollama_connection(url: String) -> Result<bool, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;
    let test_url = format!("{}/tags", url.trim_end_matches('/'));

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
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;
    let tags_url = format!("{}/tags", url.trim_end_matches('/'));

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

pub async fn test_remote_connection(url: String, api_key: Option<String>) -> Result<usize, String> {
    validate_url(&url)?;

    let has_key = api_key
        .as_ref()
        .map(|k| !k.is_empty())
        .unwrap_or(false);
    if has_key && !is_url_secure_for_api_key(&url) {
        return Err(
            "Cannot send API key over an unencrypted HTTP connection. Use HTTPS or a local address."
                .to_string(),
        );
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    let models_url = format!("{}/models", url.trim_end_matches('/'));

    let mut request = client.get(&models_url);
    if let Some(ref key) = api_key {
        if !key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    if !response.status().is_success() {
        return match response.status().as_u16() {
            401 | 403 => Err("Authentication failed. Check your API key.".to_string()),
            _ => Err(format!("Server returned error: {}", response.status())),
        };
    }

    let models_response: OpenAIModelsResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(models_response.data.len())
}

pub async fn fetch_remote_models(
    url: String,
    api_key: Option<String>,
) -> Result<Vec<OllamaModel>, String> {
    validate_url(&url)?;

    let has_key = api_key
        .as_ref()
        .map(|k| !k.is_empty())
        .unwrap_or(false);
    if has_key && !is_url_secure_for_api_key(&url) {
        return Err(
            "Cannot send API key over an unencrypted HTTP connection. Use HTTPS or a local address."
                .to_string(),
        );
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    let models_url = format!("{}/models", url.trim_end_matches('/'));

    let mut request = client.get(&models_url);
    if let Some(ref key) = api_key {
        if !key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to fetch remote models: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }

    let models_response: OpenAIModelsResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(models_response
        .data
        .into_iter()
        .map(|m| OllamaModel { name: m.id })
        .collect())
}

pub async fn pull_ollama_model(app: AppHandle, url: String, model: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let pull_url = format!("{}/pull", url.trim_end_matches('/'));

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
    let url = format!("{}/generate", settings.url.trim_end_matches('/'));

    let request_body = OllamaGenerateRequest {
        model: active_mode.model.clone(),
        prompt: " ".to_string(),
        stream: false,
        options: Some(OllamaOptions { temperature: 0.0 }),
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
