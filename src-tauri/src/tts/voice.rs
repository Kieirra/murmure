use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Voice {
    pub name: String,
    pub language: String,
    pub embedding: Vec<f32>, // [1, 1, 256] or [1, 256]
}
