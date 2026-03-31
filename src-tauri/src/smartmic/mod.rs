pub mod audio_bridge;
pub mod cert;
pub mod input_bridge;
pub mod pairing;
pub mod qr;
pub mod server;
pub mod types;
pub mod websocket;

pub use types::SmartMicState;

use log::error;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

/// Spawn the SmartMic HTTPS server on a dedicated thread (same pattern as http_api)
pub fn spawn_smartmic_thread(app_handle: AppHandle, port: u16, state: SmartMicState) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new();
        match rt {
            Ok(runtime) => {
                if let Err(e) = runtime.block_on(server::start_smartmic_server(
                    app_handle.clone(),
                    port,
                    state.clone(),
                )) {
                    let error_msg = e.to_string();
                    error!("SmartMic server error: {}", error_msg);

                    let is_port_conflict =
                        error_msg.to_lowercase().contains("address already in use")
                            || error_msg.contains("address in use")
                            || error_msg.contains("10048")
                            || error_msg.to_lowercase().contains("adresse de socket");

                    if is_port_conflict {
                        let msg = format!(
                            "Failed to start SmartMic server on port {}.\n\nThe port is already in use by another application.\n\nPlease change the port in Settings > System > SmartMic Port to an available port (1024-65535).",
                            port
                        );
                        let _ = app_handle
                            .dialog()
                            .message(&msg)
                            .title("SmartMic Error")
                            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                            .blocking_show();
                    } else {
                        let msg = format!("Failed to start SmartMic server: {}", error_msg);
                        let _ = app_handle
                            .dialog()
                            .message(&msg)
                            .title("SmartMic Error")
                            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                            .blocking_show();
                    }
                }
            }
            Err(e) => {
                error!("Failed to create async runtime for SmartMic: {}", e);
                let msg = format!("Failed to create async runtime for SmartMic: {}", e);
                let _ = app_handle
                    .dialog()
                    .message(&msg)
                    .title("SmartMic Error")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .blocking_show();
            }
        }
    });
}
