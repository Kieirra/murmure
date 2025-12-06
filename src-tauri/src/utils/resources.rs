use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn resolve_resource_path(app: &AppHandle, relative_path: &str) -> Option<PathBuf> {
    let possible_paths = vec![
        // 1. Production bundle / Windows relative
        app.path().resolve(
            format!("../resources/{}", relative_path),
            tauri::path::BaseDirectory::Resource,
        ),
        // 2. Development (tauri dev)
        app.path().resolve(
            format!("_up_/resources/{}", relative_path),
            tauri::path::BaseDirectory::Resource,
        ),
        // 3. Standard resources
        app.path().resolve(
            format!("resources/{}", relative_path),
            tauri::path::BaseDirectory::Resource,
        ),
    ];

    possible_paths
        .into_iter()
        .flatten()
        .find(|path| path.exists())
}
