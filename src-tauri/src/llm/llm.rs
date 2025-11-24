use crate::llm::helpers::load_llm_connect_settings;
use crate::llm::types::{OllamaGenerateRequest, OllamaGenerateResponse, OllamaModel, OllamaTagsResponse};
use tauri::AppHandle;

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

    if !settings.enabled {
        return Ok(transcription);
    }

    if settings.model.is_empty() {
        return Err("No model selected".to_string());
    }

    // Replace {{TRANSCRIPT}} placeholder with actual transcription
    let prompt = settings.prompt.replace("{{TRANSCRIPT}}", &transcription);

    let client = reqwest::Client::new();
    let url = format!("{}/generate", settings.url.trim_end_matches('/'));

    let request_body = OllamaGenerateRequest {
        model: settings.model.clone(),
        prompt,
        stream: false,
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
