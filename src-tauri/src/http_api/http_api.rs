use log::error;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::http_api::types::HttpApiState;

pub fn spawn_http_api_thread(app_handle: AppHandle, port: u16, state: HttpApiState) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new();
        match rt {
            Ok(runtime) => {
                if let Err(e) = runtime.block_on(crate::http_api::server::start_http_api(
                    app_handle.clone(),
                    port,
                    state.clone(),
                )) {
                    let error_msg = e.to_string();
                    error!("HTTP API error: {}", error_msg);

                    let is_port_conflict =
                        error_msg.to_lowercase().contains("address already in use")
                            || error_msg.contains("address in use")
                            || error_msg.contains("10048")
                            || error_msg.to_lowercase().contains("adresse de socket");

                    if is_port_conflict {
                        let msg = format!(
                            "Failed to start HTTP API on port {}.\n\nThe port is already in use by another application.\n\nPlease change the port in Settings → System → API Port to an available port (1024-65535).",
                            port
                        );
                        let _ = app_handle
                            .dialog()
                            .message(&msg)
                            .title("HTTP API Error")
                            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                            .blocking_show();
                    } else {
                        let msg = format!("Failed to start HTTP API: {}", error_msg);
                        let _ = app_handle
                            .dialog()
                            .message(&msg)
                            .title("HTTP API Error")
                            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                            .blocking_show();
                    }
                }
            }
            Err(e) => {
                error!("Failed to create async runtime for HTTP API: {}", e);
                let msg = format!("Failed to create async runtime for HTTP API: {}", e);
                let _ = app_handle
                    .dialog()
                    .message(&msg)
                    .title("HTTP API Error")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .blocking_show();
            }
        }
    });
}
