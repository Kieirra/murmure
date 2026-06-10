use log::{debug, error, info, warn};
use rodio::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::{RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

const STREAM_IDLE_TIMEOUT: Duration = Duration::from_secs(5);

pub enum Sound {
    StartRecording,
    StopRecording,
}

impl Sound {
    fn filename(&self) -> &'static str {
        match self {
            Sound::StartRecording => "start_record.mp3",
            Sound::StopRecording => "stop_record.mp3",
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

        let mut stream_handle: Option<rodio::MixerDeviceSink> = None;
        let mut last_playback: Option<Instant> = None;

        loop {
            let timeout = if stream_handle.is_some() {
                STREAM_IDLE_TIMEOUT
            } else {
                Duration::from_secs(86400)
            };

            match rx.recv_timeout(timeout) {
                Ok(sound) => {
                    if stream_handle.is_none() {
                        match rodio::DeviceSinkBuilder::from_default_device() {
                            Ok(builder) => match builder.open_sink_or_fallback() {
                                Ok(stream) => {
                                    info!("Audio output stream opened");
                                    // ALSA dmix and CoreAudio skip strictly silent buffers,
                                    // dropping the very first post-cold-start sound.
                                    // 100ms at amplitude 0.001 wakes the device.
                                    let warmup_sink = rodio::Player::connect_new(stream.mixer());
                                    warmup_sink.append(
                                        rodio::source::SineWave::new(440.0)
                                            .take_duration(std::time::Duration::from_millis(100))
                                            .amplify(0.001),
                                    );
                                    warmup_sink.detach();
                                    stream_handle = Some(stream);
                                }
                                Err(e) => {
                                    error!("Failed to open audio output stream: {}", e);
                                    continue;
                                }
                            },
                            Err(e) => {
                                error!("Failed to get default audio device: {}", e);
                                continue;
                            }
                        }
                    }

                    let filename = sound.filename();
                    if let Some(Some(bytes)) = sound_cache.get(filename) {
                        if let Some(ref sh) = stream_handle {
                            let cursor = std::io::Cursor::new(bytes.clone());
                            if let Ok(source) = rodio::Decoder::new(cursor) {
                                let sink = rodio::Player::connect_new(sh.mixer());
                                sink.append(source);
                                sink.detach();
                                last_playback = Some(Instant::now());
                            } else {
                                error!("Failed to decode sound: {}", filename);
                            }
                        }
                    } else {
                        warn!("Sound not found in cache: {}", filename);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if let Some(_) = stream_handle {
                        if last_playback.map_or(false, |t| t.elapsed() >= STREAM_IDLE_TIMEOUT) {
                            info!("Audio output stream idle; closing to allow sleep");
                            stream_handle = None;
                            last_playback = None;
                        }
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
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
