use log::{debug, error, info, warn};
use rodio::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::{RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

const STREAM_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
const STREAM_WARMUP_DURATION: Duration = Duration::from_millis(200);

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

enum SoundRequest {
    Play(Sound),
    Prewarm,
}

pub struct SoundManager {
    tx: Sender<SoundRequest>,
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

fn open_output_stream() -> Option<rodio::MixerDeviceSink> {
    match rodio::DeviceSinkBuilder::from_default_device() {
        Ok(builder) => match builder.open_sink_or_fallback() {
            Ok(stream) => {
                info!("Audio output stream opened");
                Some(stream)
            }
            Err(e) => {
                error!("Failed to open audio output stream: {}", e);
                None
            }
        },
        Err(e) => {
            error!("Failed to get default audio device: {}", e);
            None
        }
    }
}

pub fn init_sound_system(app: &AppHandle) {
    let (tx, rx) = std::sync::mpsc::channel::<SoundRequest>();
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

        loop {
            let received = if stream_handle.is_some() {
                rx.recv_timeout(STREAM_IDLE_TIMEOUT)
            } else {
                rx.recv().map_err(|_| RecvTimeoutError::Disconnected)
            };

            match received {
                Ok(request) => {
                    let just_opened = stream_handle.is_none();
                    if just_opened {
                        stream_handle = open_output_stream();
                    }
                    let Some(ref sh) = stream_handle else {
                        continue;
                    };

                    if just_opened {
                        // The device drops samples while waking up from a cold
                        // open (ALSA dmix, PipeWire, CoreAudio). Play a quiet
                        // tone and wait for it before the actual sound.
                        let warmup = rodio::Player::connect_new(sh.mixer());
                        warmup.append(
                            rodio::source::SineWave::new(440.0)
                                .take_duration(STREAM_WARMUP_DURATION)
                                .amplify(0.001),
                        );
                        warmup.detach();
                        thread::sleep(STREAM_WARMUP_DURATION);
                    }

                    let SoundRequest::Play(sound) = request else {
                        continue;
                    };

                    let filename = sound.filename();
                    if let Some(Some(bytes)) = sound_cache.get(filename) {
                        let cursor = std::io::Cursor::new(bytes.clone());
                        if let Ok(source) = rodio::Decoder::new(cursor) {
                            let sink = rodio::Player::connect_new(sh.mixer());
                            sink.append(source);
                            sink.detach();
                        } else {
                            error!("Failed to decode sound: {}", filename);
                        }
                    } else {
                        warn!("Sound not found in cache: {}", filename);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    // Timeout can only fire while the stream is open.
                    info!("Audio output stream idle; closing to allow sleep");
                    stream_handle = None;
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    });

    app.manage(SoundManager { tx });
}

pub fn play_sound(app: &AppHandle, sound: Sound) {
    if let Some(manager) = app.try_state::<SoundManager>() {
        let _ = manager.tx.send(SoundRequest::Play(sound));
    } else {
        warn!("SoundManager not initialized");
    }
}

/// Opens and warms up the output stream ahead of the next sound.
/// No-op when sounds are disabled.
pub fn prewarm(app: &AppHandle) {
    if !crate::settings::load_settings(app).sound_enabled {
        return;
    }
    if let Some(manager) = app.try_state::<SoundManager>() {
        let _ = manager.tx.send(SoundRequest::Prewarm);
    }
}
