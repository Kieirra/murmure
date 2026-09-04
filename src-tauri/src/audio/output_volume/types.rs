use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoweredOutput {
    pub target: String,
    pub original: Vec<u32>,
    pub applied: Vec<u32>,
    #[serde(default)]
    pub application: Option<String>,
}
