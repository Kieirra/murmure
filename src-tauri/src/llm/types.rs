use serde::{Deserialize, Serialize};
use std::fmt;

/// Wrapper that redacts sensitive values in Debug/Display output
/// to prevent accidental leaking in logs.
#[derive(Clone, Default, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    #[default]
    Local,
    Remote,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct LLMConnectSettings {
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prompt: String,
    pub modes: Vec<LLMMode>,
    pub active_mode_index: usize,
    pub onboarding_completed: bool,
    #[serde(default)]
    pub remote_url: String,
    #[serde(default)]
    pub remote_privacy_acknowledged: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMMode {
    pub name: String,
    pub prompt: String,
    pub model: String,
    pub shortcut: String,
    #[serde(default)]
    pub provider: LLMProvider,
    #[serde(default)]
    pub wake_word: String,
}

impl Default for LLMConnectSettings {
    fn default() -> Self {
        Self {
            url: "http://localhost:11434/api".to_string(),
            model: String::new(),
            prompt: String::new(),
            modes: Vec::new(),
            active_mode_index: 0,
            onboarding_completed: false,
            remote_url: String::new(),
            remote_privacy_acknowledged: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OllamaOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaOptions {
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaGenerateResponse {
    pub response: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaTagsResponse {
    pub models: Vec<OllamaModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaPullRequest {
    pub model: String,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OllamaPullResponse {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChatRequest {
    pub model: String,
    pub messages: Vec<OpenAIChatMessage>,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChatResponse {
    pub choices: Vec<OpenAIChatChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChatChoice {
    pub message: OpenAIChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIModelsResponse {
    pub data: Vec<OpenAIModelEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIModelEntry {
    pub id: String,
}
