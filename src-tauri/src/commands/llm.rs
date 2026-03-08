use crate::llm::{self, LLMConnectSettings, OllamaModel, SecretString};
use tauri::{command, AppHandle, Emitter};

#[command]
pub fn get_llm_connect_settings(app: AppHandle) -> Result<LLMConnectSettings, String> {
    Ok(llm::load_llm_connect_settings(&app))
}

#[command]
pub fn set_llm_connect_settings(
    app: AppHandle,
    settings: LLMConnectSettings,
) -> Result<(), String> {
    llm::save_llm_connect_settings(&app, &settings)?;
    let _ = app.emit("llm-settings-updated", &settings);
    llm::helpers::restart_wake_word_if_active(&app);
    Ok(())
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
pub async fn test_remote_connection(url: String) -> Result<usize, String> {
    let api_key = llm::helpers::load_remote_api_key();
    llm::test_remote_connection(url, api_key).await
}

#[command]
pub async fn fetch_remote_models(url: String) -> Result<Vec<OllamaModel>, String> {
    let api_key = llm::helpers::load_remote_api_key();
    llm::fetch_remote_models(url, api_key).await
}

#[command]
pub fn store_remote_api_key(api_key: SecretString) -> Result<(), String> {
    llm::helpers::store_remote_api_key(api_key.expose())
}

#[command]
pub fn has_remote_api_key() -> bool {
    llm::helpers::has_remote_api_key()
}

#[command]
pub fn get_remote_api_key_masked() -> String {
    llm::helpers::load_remote_api_key_masked()
}

#[command]
pub async fn pull_ollama_model(app: AppHandle, url: String, model: String) -> Result<(), String> {
    llm::llm::pull_ollama_model(app, url, model).await
}
