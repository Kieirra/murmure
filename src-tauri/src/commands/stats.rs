use crate::stats::UsageStats;
use tauri::{AppHandle, command};

#[command]
pub fn get_usage_stats(app: AppHandle) -> Result<UsageStats, String> {
    crate::stats::compute_stats(&app).map_err(|e| format!("{:#}", e))
}
