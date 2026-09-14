use crate::http_api::{spawn_http_api_thread, HttpApiState};
use crate::settings;
use log::info;
use std::sync::atomic::Ordering;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_api_enabled(app: AppHandle) -> Result<bool, String> {
    let s = settings::load_settings(&app);
    Ok(s.api_enabled)
}

#[command]
pub fn set_api_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.api_enabled = enabled;
    settings::save_settings(&app, &s)
}

#[command]
pub fn get_api_port(app: AppHandle) -> Result<u16, String> {
    let s = settings::load_settings(&app);
    Ok(s.api_port)
}

#[command]
pub fn set_api_port(app: AppHandle, port: u16) -> Result<(), String> {
    if port < 1024 {
        return Err("Port must be >= 1024".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.api_port = port;
    settings::save_settings(&app, &s)
}

fn stop_http_api_and_wait(app: &AppHandle) {
    let state = app.state::<HttpApiState>();
    state.stop();
    info!("HTTP API server stop signal sent");
    for _ in 0..20 {
        if !state.is_running.load(Ordering::SeqCst) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[command]
pub fn start_http_api_server(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    if !s.api_enabled {
        return Err("HTTP API is disabled".to_string());
    }

    let state = app.state::<HttpApiState>().inner().clone();
    if state
        .is_running
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Ok(format!(
            "HTTP API server already running on port {}",
            s.api_port
        ));
    }

    spawn_http_api_thread(app.clone(), s.api_port, state);
    Ok(format!("HTTP API server starting on port {}", s.api_port))
}

#[command]
pub fn stop_http_api_server(app: AppHandle) -> Result<(), String> {
    stop_http_api_and_wait(&app);
    Ok(())
}

#[command]
pub fn sync_http_api_server(app: AppHandle) -> Result<(), String> {
    stop_http_api_and_wait(&app);
    let s = settings::load_settings(&app);
    if s.api_enabled {
        start_http_api_server(app)?;
    }
    Ok(())
}
