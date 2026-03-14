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

#[command]
pub fn delete_history_item(app: AppHandle, id: u64) -> Result<(), String> {
    history::delete_history_item(&app, id).map_err(|e| format!("{:#}", e))
}

#[command]
pub fn get_history_limit(app: AppHandle) -> Result<usize, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.history_limit)
}

#[command]
pub fn set_history_limit(app: AppHandle, limit: usize) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    let constrained = limit.clamp(0, 5); // Strictly enforce max 5
    s.history_limit = constrained;
    crate::settings::save_settings(&app, &s)?;
    
    // Immediately clear history if limit set to 0
    if constrained == 0 {
        let _ = history::clear_history(&app);
        if s.persist_history {
            let _ = history::purge_history_file(&app);
        }
    }
    
    Ok(())
}
