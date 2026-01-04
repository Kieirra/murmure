use cpal::traits::{DeviceTrait, HostTrait};
use log::info;
use tauri::Manager;

pub fn get_mic_list() -> Vec<String> {
    let host = cpal::default_host();
    match host.input_devices() {
        Ok(devices) => {
            let mut device_names = Vec::new();
            for device in devices {
                match device.name() {
                    Ok(name) => device_names.push(name),
                    Err(_) => continue,
                }
            }
            device_names
        }
        Err(_) => Vec::new(),
    }
}

pub fn update_mic_cache(app: &tauri::AppHandle, mic_id: Option<String>) {
    let audio_state = app.state::<crate::audio::types::AudioState>();
    match mic_id {
        Some(ref id) => {
            let host = cpal::default_host();
            let mut found_device = None;

            if let Ok(devices) = host.input_devices() {
                for device in devices {
                    match device.name() {
                        Ok(name) => {
                            if &name == id {
                                found_device = Some(device);
                                break;
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
            audio_state.set_cached_device(found_device);
        }
        None => {
            audio_state.set_cached_device(None);
        }
    }
}

pub fn init_mic_cache_if_needed(app: &tauri::AppHandle, mic_id: Option<String>) {
    if let Some(id) = mic_id {
        let app_handle = app.clone();
        std::thread::spawn(move || {
            let host = cpal::default_host();
            if let Ok(devices) = host.input_devices() {
                for device in devices {
                    match device.name() {
                        Ok(name) => {
                            if name == id {
                                let audio_state =
                                    app_handle.state::<crate::audio::types::AudioState>();
                                audio_state.set_cached_device(Some(device));
                                info!("Microphone cache initialized: {}", name);
                                break;
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        });
    }
}
