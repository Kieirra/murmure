use crate::tts::engine::KokoroEngine;
use crate::tts::TtsState;
use log::{debug, error, info};
use tauri::{command, AppHandle, Manager};

#[command]
pub async fn neural_speak(app: AppHandle, text: String) -> Result<(), String> {
    let state = app.state::<TtsState>();

    // Increment generation ID and get our unique version
    let my_gen_id = state
        .generation_id
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        + 1;

    // Stop current playback immediately
    state.player.stop();

    let mut engine_guard = state.engine.lock().await;

    if engine_guard.is_none() {
        info!("Kokoro engine not found, initializing...");
        let model_path = crate::utils::resources::resolve_resource_path(&app, "tts/kokoro.onnx")
            .ok_or_else(|| "Model kokoro.onnx not found".to_string())?;

        let voices_path = crate::utils::resources::resolve_resource_path(&app, "tts/voices.bin")
            .ok_or_else(|| "Voices.bin not found".to_string())?;

        let engine = KokoroEngine::new(&model_path, &voices_path)
            .await
            .map_err(|e| format!("Engine creation failed: {}", e))?;
        *engine_guard = Some(engine);
    }

    let engine = engine_guard.as_ref().unwrap();
    let settings = crate::settings::load_settings(&app);
    let speed = settings.tts_speed;
    let (lang_code, voice) = match settings.language.as_str() {
        "en" => {
            let selected = if settings.tts_voice.is_empty() {
                "af_heart"
            } else {
                settings.tts_voice.as_str()
            };
            ("en-us", selected)
        }
        "fr" => ("fr", "ff_siwis"),
        _ => {
            if let Some(info) = whatlang::detect(&text) {
                if info.lang() == whatlang::Lang::Fra {
                    ("fr", "ff_siwis")
                } else {
                    let selected = if settings.tts_voice.is_empty() {
                        "af_heart"
                    } else {
                        settings.tts_voice.as_str()
                    };
                    ("en-us", selected)
                }
            } else {
                ("fr", "ff_siwis")
            }
        }
    };

    info!(
        "Streaming synthesized speech (gen_id={}, speed={}, lang={}, voice={})",
        my_gen_id, speed, lang_code, voice
    );

    // Split text into sentences for streaming
    let sentences: Vec<String> = text
        .split_inclusive(&['.', '!', '?', '\n'][..])
        .map(|s| s.to_string())
        .collect();

    for sentence in sentences {
        // Check if a new generation has started
        if state
            .generation_id
            .load(std::sync::atomic::Ordering::SeqCst)
            != my_gen_id
        {
            debug!("Generation {} cancelled, stopping synthesis.", my_gen_id);
            return Ok(());
        }

        let trimmed = sentence.trim();
        if trimmed.is_empty() {
            continue;
        }

        debug!("Synthesizing chunk: \"{}\"", trimmed);
        match engine.generate_audio(trimmed, lang_code, voice, speed) {
            Ok(samples) => {
                if !samples.is_empty() {
                    state.player.play(samples);
                }
            }
            Err(e) => {
                error!("Chunk synthesis failed: {}", e);
            }
        }
    }

    // Await playback completion so that the frontend knows it's still "speaking"
    while state.player.is_playing() {
        if state
            .generation_id
            .load(std::sync::atomic::Ordering::SeqCst)
            != my_gen_id
        {
            debug!("Generation {} cancelled during playback.", my_gen_id);
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    Ok(())
}

#[command]
pub async fn neural_export_wav(
    app: AppHandle,
    text: String,
    file_path: String,
) -> Result<(), String> {
    let state = app.state::<TtsState>();
    let mut engine_guard = state.engine.lock().await;

    if engine_guard.is_none() {
        info!("Kokoro engine not found, initializing for export...");
        let model_path = crate::utils::resources::resolve_resource_path(&app, "tts/kokoro.onnx")
            .ok_or_else(|| "Model kokoro.onnx not found".to_string())?;

        let voices_path = crate::utils::resources::resolve_resource_path(&app, "tts/voices.bin")
            .ok_or_else(|| "Voices.bin not found".to_string())?;

        let engine = KokoroEngine::new(&model_path, &voices_path)
            .await
            .map_err(|e| format!("Engine creation failed: {}", e))?;
        *engine_guard = Some(engine);
    }

    let engine = engine_guard.as_ref().unwrap();
    let settings = crate::settings::load_settings(&app);
    let speed = settings.tts_speed;
    let (lang_code, voice) = match settings.language.as_str() {
        "en" => {
            let selected = if settings.tts_voice.is_empty() {
                "af_heart"
            } else {
                settings.tts_voice.as_str()
            };
            ("en-us", selected)
        }
        "fr" => ("fr", "ff_siwis"),
        _ => {
            if let Some(info) = whatlang::detect(&text) {
                if info.lang() == whatlang::Lang::Fra {
                    ("fr", "ff_siwis")
                } else {
                    let selected = if settings.tts_voice.is_empty() {
                        "af_heart"
                    } else {
                        settings.tts_voice.as_str()
                    };
                    ("en-us", selected)
                }
            } else {
                ("fr", "ff_siwis")
            }
        }
    };

    info!(
        "Exporting synthesized speech to: {} (lang={}, voice={})",
        file_path, lang_code, voice
    );

    match engine.generate_audio(&text, lang_code, voice, speed) {
        Ok(samples) => {
            if samples.is_empty() {
                return Err("Generated audio is empty".to_string());
            }

            // Save as .wav (24000 Hz, mono, 32-bit float)
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 24000,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };

            let mut writer = hound::WavWriter::create(&file_path, spec)
                .map_err(|e| format!("Failed to create WAV file: {}", e))?;

            for &sample in &samples {
                writer
                    .write_sample(sample)
                    .map_err(|e| format!("Failed to write sample: {}", e))?;
            }
            writer
                .finalize()
                .map_err(|e| format!("Failed to finalize WAV file: {}", e))?;

            info!("Successfully exported audio to {}", file_path);
        }
        Err(e) => {
            error!("Export synthesis failed: {}", e);
            return Err(e.to_string());
        }
    }

    Ok(())
}

#[command]
pub fn is_neural_tts_available(app: AppHandle) -> bool {
    let res = crate::utils::resources::resolve_resource_path(&app, "tts/kokoro.onnx");
    info!(
        "Check neural TTS availability: kokoro.onnx exists? {}",
        res.is_some()
    );
    if let Some(ref path) = res {
        debug!("Found kokoro.onnx at: {:?}", path);
    }
    res.is_some()
}

#[command]
pub fn stop_neural_speak(app: AppHandle) {
    let state = app.state::<TtsState>();
    
    // Increment generation ID to cancel any ongoing synthesis loop
    state.generation_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    // Stop the audio player immediately
    state.player.stop();
    debug!("Neural speech playback stopped manually by user");
}

#[command]
pub fn get_audio_export_folder(app: AppHandle) -> Result<String, String> {
    let s = crate::settings::load_settings(&app);
    Ok(s.audio_export_folder)
}

#[command]
pub fn set_audio_export_folder(app: AppHandle, path: String) -> Result<(), String> {
    let mut s = crate::settings::load_settings(&app);
    s.audio_export_folder = path;
    crate::settings::save_settings(&app, &s)?;
    Ok(())
}
