use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn resolve_resource_path(app: &AppHandle, relative_path: &str) -> Option<PathBuf> {
    log::debug!("--- Resolving Resource: {} ---", relative_path);

    // 1. Try standard Tauri resolution
    if let Ok(path) = app
        .path()
        .resolve(relative_path, tauri::path::BaseDirectory::Resource)
    {
        if path.exists() {
            log::debug!("FOUND (Tauri Resource): {:?}", path);
            return Some(path);
        }
    }

    // 2. Try common development patterns (Relative to current dir)
    let project_name = "murmure";
    let possible_patterns = vec![
        format!("{}/resources/{}", project_name, relative_path),
        format!("resources/{}", relative_path),
        format!("src-tauri/resources/{}", relative_path),
        format!("../resources/{}", relative_path),
        format!("../../resources/{}", relative_path),
    ];

    for rel in possible_patterns {
        let p = PathBuf::from(&rel);
        if p.exists() {
            if let Ok(canon) = p.canonicalize() {
                log::debug!("FOUND (Pattern {}): {:?}", rel, canon);
                return Some(canon);
            }
        }
    }

    // 3. Absolute fallback for Windows dev environment
    let base_dir = "f:/AzWorkspace/murmure";
    let fallbacks = vec![
        format!("{}/resources/{}", base_dir, relative_path),
        format!("{}/src-tauri/resources/{}", base_dir, relative_path),
    ];

    for fb in fallbacks {
        let p = PathBuf::from(&fb);
        if p.exists() {
            log::debug!("FOUND (Absolute Fallback): {:?}", p);
            return Some(p);
        }
    }

    log::warn!("FAILED to resolve resource: {}", relative_path);
    None
}
