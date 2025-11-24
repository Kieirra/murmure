use crate::model::Model;
use std::sync::Arc;
use tauri::{State, command};

#[command]
pub fn is_model_available(model: State<Arc<Model>>) -> bool {
    model.is_available()
}

#[command]
pub fn get_model_path(model: State<Arc<Model>>) -> Result<String, String> {
    let path = model.get_model_path().map_err(|e| format!("{:#}", e))?;

    Ok(path.to_string_lossy().to_string())
}
