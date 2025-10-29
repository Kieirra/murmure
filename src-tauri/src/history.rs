use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

const MAX_HISTORY_ENTRIES: usize = 5;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: u64,
    pub timestamp: i64,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HistoryData {
    entries: Vec<HistoryEntry>,
    next_id: u64,
}

impl Default for HistoryData {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }
}

fn get_history_file_path(app: &AppHandle) -> Result<PathBuf> {
    let app_data_dir = app.path().app_data_dir()?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)?;
    }
    Ok(app_data_dir.join("history.json"))
}

fn read_history(app: &AppHandle) -> Result<HistoryData> {
    let path = get_history_file_path(app)?;
    if !path.exists() {
        return Ok(HistoryData::default());
    }
    let content = fs::read_to_string(path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

fn write_history(app: &AppHandle, data: &HistoryData) -> Result<()> {
    let path = get_history_file_path(app)?;
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn add_transcription(app: &AppHandle, text: String) -> Result<()> {
    let mut data = read_history(app)?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    let entry = HistoryEntry {
        id: data.next_id,
        timestamp,
        text,
    };

    data.entries.insert(0, entry);
    data.next_id += 1;

    if data.entries.len() > MAX_HISTORY_ENTRIES {
        data.entries.truncate(MAX_HISTORY_ENTRIES);
    }

    write_history(app, &data)?;

    let _ = app.emit("history-updated", ());

    Ok(())
}

pub fn get_recent_transcriptions(app: &AppHandle) -> Result<Vec<HistoryEntry>> {
    let data = read_history(app)?;
    Ok(data.entries)
}

pub fn get_last_transcription(app: &AppHandle) -> Result<String> {
    let data = read_history(app)?;
    Ok(data.entries.first().unwrap().text.clone())
}

/// Clears all transcription history entries and emits an event to notify the frontend.
pub fn clear_history(app: &AppHandle) -> Result<()> {
    let mut data = read_history(app)?;
    data.entries.clear();
    write_history(app, &data)?;
    let _ = app.emit("history-updated", ());
    Ok(())
}
