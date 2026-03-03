use crate::llm::{self, LLMConnectSettings, OllamaModel};
use tauri::{command, AppHandle};

#[command]
pub fn get_llm_connect_settings(app: AppHandle) -> Result<LLMConnectSettings, String> {
    Ok(llm::load_llm_connect_settings(&app))
}

#[command]
pub fn set_llm_connect_settings(
    app: AppHandle,
    settings: LLMConnectSettings,
) -> Result<(), String> {
    llm::save_llm_connect_settings(&app, &settings)
}

#[command]
pub async fn test_llm_connection(url: String) -> Result<bool, String> {
    llm::test_ollama_connection(url).await
}

#[command]
pub async fn fetch_ollama_models(url: String) -> Result<Vec<OllamaModel>, String> {
    llm::fetch_ollama_models(url).await
}

#[command]
pub async fn test_remote_connection(url: String, api_key: Option<String>) -> Result<usize, String> {
    llm::test_remote_connection(url, api_key).await
}

#[command]
pub async fn fetch_remote_models(
    url: String,
    api_key: Option<String>,
) -> Result<Vec<OllamaModel>, String> {
    llm::fetch_remote_models(url, api_key).await
}

#[command]
pub fn store_remote_api_key(api_key: String) -> Result<(), String> {
    llm::helpers::store_remote_api_key(&api_key)
}

#[command]
pub fn has_remote_api_key() -> bool {
    llm::helpers::has_remote_api_key()
}

#[command]
pub fn get_remote_api_key() -> Result<String, String> {
    llm::helpers::load_remote_api_key().ok_or_else(|| "No API key stored".to_string())
}
