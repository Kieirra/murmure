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
