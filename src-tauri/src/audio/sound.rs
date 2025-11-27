use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use tauri::{AppHandle, Manager};

pub enum Sound {
    StartRecording,
    StopRecording,
}

impl Sound {
    fn filename(&self) -> &'static str {
        match self {
            Sound::StartRecording => "start_record.mp3",
            Sound::StopRecording => "start_record.mp3", // Using same sound for now as per user environment
        }
    }
}

fn resolve_sound_path(app: &AppHandle, filename: &str) -> Option<PathBuf> {
    let possible_paths = vec![
        // 1. Production bundle / Windows relative
        app.path().resolve(
            format!("../resources/audio/{}", filename),
            tauri::path::BaseDirectory::Resource,
        ),
        // 2. Development (tauri dev)
        app.path().resolve(
            format!("_up_/resources/audio/{}", filename),
            tauri::path::BaseDirectory::Resource,
        ),
        // 3. Standard resources
        app.path().resolve(
            format!("resources/audio/{}", filename),
            tauri::path::BaseDirectory::Resource,
        ),
    ];

    for path_result in possible_paths {
        match path_result {
            Ok(path) => {
                println!("Checking path: {:?}", path); // Minimal debug
                if path.exists() {
                    println!("Sound found at: {:?}", path);
                    return Some(path);
                }
            }
            Err(e) => println!("Error resolving path: {}", e),
        }
    }

    println!("Sound not found in any expected location for: {}", filename);
    None
}

pub fn play_sound(app: &AppHandle, sound: Sound) {
    let app_handle = app.clone();
    let filename = sound.filename();

    thread::spawn(move || {
        if let Some(path) = resolve_sound_path(&app_handle, filename) {
            if let Ok(file) = File::open(&path) {
                let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&stream_handle).unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);
                sink.sleep_until_end();
            } else {
                eprintln!("Failed to open sound file: {:?}", path);
            }
        } else {
            eprintln!("Sound file not found: {}", filename);
        }
    });
}
