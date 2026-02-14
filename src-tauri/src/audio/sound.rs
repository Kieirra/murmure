use log::{debug, error, info, warn};
use rodio::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use tauri::{AppHandle, Manager};

pub enum Sound {
    StartRecording,
    StopRecording,
    CancelRecording,
}

impl Sound {
    fn filename(&self) -> &'static str {
        match self {
            Sound::StartRecording => "start_record.mp3",
            Sound::StopRecording => "stop_record.mp3",
            Sound::CancelRecording => "cancel_record.mp3",
        }
    }
}

pub struct SoundManager {
    tx: Sender<Sound>,
}

fn resolve_sound_path(app: &AppHandle, filename: &str) -> Option<PathBuf> {
    crate::utils::resources::resolve_resource_path(app, &format!("audio/{}", filename))
}

fn load_sound_bytes(app: &AppHandle, filename: &str) -> Option<Vec<u8>> {
    if let Some(path) = resolve_sound_path(app, filename) {
        if let Ok(mut file) = File::open(&path) {
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                debug!("Loaded sound: {:?}", path);
                return Some(buffer);
            }
        }
    }
    warn!("Failed to load sound: {}", filename);
    None
}

pub fn init_sound_system(app: &AppHandle) {
    let (tx, rx) = std::sync::mpsc::channel::<Sound>();
    let app_handle = app.clone();

    thread::spawn(move || {
        // Init audio output stream with fallback for better macOS compatibility
        let stream_handle = match rodio::OutputStreamBuilder::from_default_device() {
            Ok(builder) => match builder.open_stream_or_fallback() {
                Ok(stream) => stream,
                Err(e) => {
                    error!("Failed to open audio output stream: {}", e);
                    while rx.recv().is_ok() {}
                    return;
                }
            },
            Err(e) => {
                error!("Failed to get default audio device: {}", e);
                while rx.recv().is_ok() {}
                return;
            }
        };

        info!("Audio output stream initialized successfully");

        // Preload sounds
        let mut sound_cache = HashMap::new();
        sound_cache.insert(
            Sound::StartRecording.filename(),
            load_sound_bytes(&app_handle, Sound::StartRecording.filename()),
        );
        sound_cache.insert(
            Sound::StopRecording.filename(),
            load_sound_bytes(&app_handle, Sound::StopRecording.filename()),
        );
        sound_cache.insert(
            Sound::CancelRecording.filename(),
            load_sound_bytes(&app_handle, Sound::CancelRecording.filename()),
        );

        // Warmup: Play a silent sound to wake up the audio device
        let warmup_sink = rodio::Sink::connect_new(stream_handle.mixer());
        warmup_sink.append(
            rodio::source::SineWave::new(440.0)
                .take_duration(std::time::Duration::from_millis(10))
                .amplify(0.0),
        );
        warmup_sink.detach();

        while let Ok(sound) = rx.recv() {
            let filename = sound.filename();
            if let Some(Some(bytes)) = sound_cache.get(filename) {
                // Create a cursor for the bytes
                let cursor = std::io::Cursor::new(bytes.clone());

                // Decode and play
                if let Ok(source) = rodio::Decoder::new(cursor) {
                    let sink = rodio::Sink::connect_new(stream_handle.mixer());
                    sink.append(source);
                    sink.detach();
                } else {
                    error!("Failed to decode sound: {}", filename);
                }
            } else {
                warn!("Sound not found in cache: {}", filename);
            }
        }
    });

    app.manage(SoundManager { tx });
}

pub fn play_sound(app: &AppHandle, sound: Sound) {
    if let Some(manager) = app.try_state::<SoundManager>() {
        let _ = manager.tx.send(sound);
    } else {
        warn!("SoundManager not initialized");
    }
}
