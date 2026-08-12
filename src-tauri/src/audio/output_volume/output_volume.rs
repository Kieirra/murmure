use super::platform;
use super::types::LoweredOutput;
use log::debug;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn unsupported_reason() -> Option<String> {
    platform::unsupported_reason()
}

pub fn lower_and_persist(app: &AppHandle, percent: u8) -> Option<LoweredOutput> {
    match platform::lower(percent) {
        Some(state) => {
            debug!(
                "Lowered output volume from {:?} to {:?}",
                state.original, state.applied
            );
            if let Err(e) = persist(app, &state) {
                debug!("Failed to persist output volume marker: {}", e);
            }
            Some(state)
        }
        None => {
            debug!("Output volume not lowered, recording continues at full volume");
            None
        }
    }
}

pub fn restore_and_clear(app: &AppHandle, state: &LoweredOutput) {
    platform::restore(state);
    clear(app);
}

pub fn restore_pending(app: &AppHandle) {
    let Ok(path) = marker_path(app) else {
        return;
    };
    if let Ok(content) = fs::read_to_string(&path) {
        match serde_json::from_str::<LoweredOutput>(&content) {
            Ok(state) => {
                debug!("Restoring output volume left over by a previous run");
                platform::restore(&state);
            }
            Err(e) => debug!("Discarding unreadable output volume marker: {}", e),
        }
    }
    let _ = fs::remove_file(&path);
}

fn marker_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Err(e) = fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all failed: {}", e));
    }
    Ok(dir.join("output-volume-restore.json"))
}

fn persist(app: &AppHandle, state: &LoweredOutput) -> Result<(), String> {
    let path = marker_path(app)?;
    let content = serde_json::to_string(state).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

fn clear(app: &AppHandle) {
    if let Ok(path) = marker_path(app) {
        let _ = fs::remove_file(path);
    }
}
