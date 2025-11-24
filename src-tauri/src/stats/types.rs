use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub writing_speed_wpm: f64,
    pub words_current_month: u64,
    pub local_audio_mb: f64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregatedStats {
    #[serde(default)]
    pub wpm_sum: f64,
    #[serde(default)]
    pub wpm_count: u64,
    pub words_this_month: u64,
    pub last_reset_timestamp: i64,
    pub total_audio_bytes: u64,
}
