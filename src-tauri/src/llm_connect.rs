use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct LLMConnectSettings {
    pub enabled: bool,
    pub url: String,
    pub model: String,
    pub prompt: String,
}

impl Default for LLMConnectSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "http://localhost:11434/api".to_string(),
            model: String::new(),
            prompt: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
}

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

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str::<LLMConnectSettings>(&content).unwrap_or_default(),
        Err(_) => {
            let defaults = LLMConnectSettings::default();
            let _ = save_llm_connect_settings(app, &defaults);
            defaults
        }
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
