use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoweredOutput {
    pub target: String,
    pub original: Vec<u32>,
    pub applied: Vec<u32>,
}
