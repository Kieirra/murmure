use crate::llm_connect::{self, LLMConnectSettings, OllamaModel};
use tauri::{AppHandle, command};

#[command]
pub fn get_llm_connect_settings(
    app: AppHandle,
) -> Result<LLMConnectSettings, String> {
    Ok(llm_connect::load_llm_connect_settings(&app))
}

#[command]
pub fn set_llm_connect_settings(
    app: AppHandle,
    settings: LLMConnectSettings,
) -> Result<(), String> {
    llm_connect::save_llm_connect_settings(&app, &settings)
}

#[command]
pub async fn test_llm_connection(url: String) -> Result<bool, String> {
    llm_connect::test_ollama_connection(url).await
}

#[command]
pub async fn fetch_ollama_models(
    url: String,
) -> Result<Vec<OllamaModel>, String> {
    llm_connect::fetch_ollama_models(url).await
}
