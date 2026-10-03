use super::helpers::wait_until_stopped;
use crate::http_api::{spawn_http_api_thread, HttpApiState, HttpApiStatus};
use crate::settings;
use log::info;
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
        return Err("Port must be between 1024 and 65535".to_string());
    }
    let mut s = settings::load_settings(&app);
    s.api_port = port;
    settings::save_settings(&app, &s)
}

#[command]
pub fn start_http_api_server(app: AppHandle) -> Result<(), String> {
    let s = settings::load_settings(&app);
    let state = app.state::<HttpApiState>().inner().clone();
    spawn_http_api_thread(app.clone(), s.api_port, state).map_err(|e| e.to_string())
}

#[command]
pub async fn stop_http_api_server(app: AppHandle) -> Result<(), String> {
    let state = app.state::<HttpApiState>();
    state.stop();
    info!("HTTP API server stop signal sent");

    wait_until_stopped(&state.is_running).await;

    Ok(())
}

#[command]
pub fn get_http_api_status(app: AppHandle) -> Result<HttpApiStatus, String> {
    Ok(app.state::<HttpApiState>().status())
}
