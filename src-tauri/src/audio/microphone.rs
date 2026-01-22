use cpal::traits::{DeviceTrait, HostTrait};
use cpal::SampleFormat;
use log::info;
use std::collections::HashSet;
use tauri::Manager;

fn is_valid_input_device(device: &cpal::Device) -> bool {
    let configs = match device.supported_input_configs() {
        Ok(c) => c,
        Err(_) => return false,
    };

    for config in configs {
        let channels = config.channels();
        let min_rate = config.min_sample_rate();
        let max_rate = config.max_sample_rate();
        let format = config.sample_format();

        let valid_channels = channels == 1 || channels == 2;
        let valid_rate = min_rate == 44100 && max_rate >= 44100;
        let valid_format = matches!(format, SampleFormat::I16 | SampleFormat::F32);

        if valid_channels && valid_rate && valid_format {
            return true;
        }
    }

    false
}

fn get_device_name(device: &cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|desc| desc.name().to_string())
}

pub fn get_mic_list() -> Vec<String> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|d| get_device_name(&d));

    match host.input_devices() {
        Ok(devices) => {
            let mut device_names = Vec::new();
            let mut seen_names = HashSet::new();

            for device in devices {
                if !is_valid_input_device(&device) {
                    continue;
                }

                if let Some(name) = get_device_name(&device) {
                    if !seen_names.contains(&name) {
                        seen_names.insert(name.clone());
                        device_names.push(name);
                    }
                }
            }

            if let Some(ref default) = default_name {
                if let Some(pos) = device_names.iter().position(|n| n == default) {
                    device_names.remove(pos);
                    device_names.insert(0, default.clone());
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
                    if let Some(name) = get_device_name(&device) {
                        if name == *id {
                            found_device = Some(device);
                            break;
                        }
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
                    if let Some(name) = get_device_name(&device) {
                        if name == id {
                            let audio_state = app_handle.state::<crate::audio::types::AudioState>();
                            audio_state.set_cached_device(Some(device));
                            info!("Microphone cache initialized: {}", name);
                            break;
                        }
                    }
                }
            }
        });
    }
}
