use crate::history::{self, HistoryEntry};
use tauri::{command, AppHandle};

#[command]
pub fn get_recent_transcriptions(app: AppHandle) -> Result<Vec<HistoryEntry>, String> {
    history::get_recent_transcriptions(&app).map_err(|e| format!("{:#}", e))
}

#[command]
pub fn clear_history(app: AppHandle) -> Result<(), String> {
    history::clear_history(&app).map_err(|e| format!("{:#}", e))
}

#[command]
pub fn get_persist_history(app: AppHandle) -> Result<bool, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.persist_history)
}

#[command]
pub fn set_persist_history(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.persist_history = enabled;
    crate::settings::save_settings(&app, &s)?;
    if !enabled {
        let _ = history::clear_history(&app);
        let _ = history::purge_history_file(&app);
    }
    Ok(())
}
