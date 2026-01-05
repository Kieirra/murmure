use crate::dictionary;
use crate::llm::helpers::load_llm_connect_settings;
use crate::llm::types::{
    OllamaGenerateRequest, OllamaGenerateResponse, OllamaModel, OllamaOptions, OllamaPullRequest,
    OllamaPullResponse, OllamaTagsResponse,
};
use log::warn;
use tauri::{AppHandle, Emitter};

pub async fn post_process_with_llm(
    app: &AppHandle,
    transcription: String,
    force_bypass: bool,
) -> Result<String, String> {
    // If force_bypass is true, skip LLM processing entirely
    if force_bypass {
        return Ok(transcription);
    }

    let settings = load_llm_connect_settings(app);

    if settings.model.is_empty() {
        return Err("No model selected".to_string());
    }

    let _ = app.emit("llm-processing-start", ());

    // Load dictionary words and format as comma-separated list
    let dictionary_words = dictionary::load(app)
        .unwrap_or_default()
        .into_keys()
        .collect::<Vec<String>>()
        .join(", ");

    let prompt = settings
        .prompt
        .replace("{{TRANSCRIPT}}", &transcription)
        .replace("{{DICTIONARY}}", &dictionary_words);

    let client = reqwest::Client::new();
    let url = format!("{}/generate", settings.url.trim_end_matches('/'));

    let request_body = OllamaGenerateRequest {
        model: settings.model.clone(),
        prompt,
        stream: false,
        options: Some(OllamaOptions { temperature: 0.0 }),
    };

    let response = client.post(&url).json(&request_body).send().await;

    let response = match response {
        Ok(res) => res,
        Err(e) => {
            let _ = app.emit("llm-processing-end", ());
            return Err(format!("Failed to connect to Ollama: {}", e));
        }
    };

    if !response.status().is_success() {
        let _ = app.emit("llm-processing-end", ());
        return Err(format!("Ollama API returned error: {}", response.status()));
    }

    let ollama_response: Result<OllamaGenerateResponse, _> = response.json().await;

    let _ = app.emit("llm-processing-end", ());

    let ollama_response =
        ollama_response.map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    Ok(ollama_response.response.trim().to_string())
}

pub async fn test_ollama_connection(url: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
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
    let client = reqwest::Client::new();
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

#[tauri::command]
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

/// Warm up the configured Ollama model by issuing a minimal generate request.
/// This reduces the perceived latency on the first real call during LLM Connect.
pub async fn warmup_ollama_model(app: &AppHandle) -> Result<(), String> {
    let settings = load_llm_connect_settings(app);

    // Nothing to warm up if configuration is incomplete
    if settings.model.trim().is_empty() || settings.url.trim().is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/generate", settings.url.trim_end_matches('/'));

    // Minimal prompt, no streaming. We intentionally ignore the response body.
    let request_body = OllamaGenerateRequest {
        model: settings.model.clone(),
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

/// Fire-and-forget background warmup used at the beginning of LLM recording.
pub fn warmup_ollama_model_background(app: &AppHandle) {
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = warmup_ollama_model(&app_handle).await {
            warn!("LLM warmup failed: {}", e);
        }
    });
}
