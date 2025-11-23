use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: u64,
    pub timestamp: i64,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryData {
    pub entries: Vec<HistoryEntry>,
    pub next_id: u64,
}

impl Default for HistoryData {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }
}
