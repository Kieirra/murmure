use crate::clipboard;
use crate::dictionary::{fix_transcription_with_dictionary, get_cc_rules_path, Dictionary};
use crate::engine::{
    engine::ParakeetEngine, engine::ParakeetModelParams, transcription_engine::TranscriptionEngine,
};
use crate::history;
use crate::model::Model;
use crate::overlay;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;

type WavWriterType = WavWriter<BufWriter<File>>;
type RecorderType = Mutex<Option<WavWriterType>>;

static RECORDER: Lazy<parking_lot::Mutex<Option<Arc<RecorderType>>>> =
    Lazy::new(|| parking_lot::Mutex::new(None));
static STREAM: Lazy<parking_lot::Mutex<Option<cpal::Stream>>> =
    Lazy::new(|| parking_lot::Mutex::new(None));
static CURRENT_FILE_NAME: Lazy<parking_lot::Mutex<Option<String>>> =
    Lazy::new(|| parking_lot::Mutex::new(None));
static ENGINE: Lazy<parking_lot::Mutex<Option<ParakeetEngine>>> =
    Lazy::new(|| parking_lot::Mutex::new(None));

pub fn record_audio(app: &tauri::AppHandle) {
    println!("Starting audio recording...");

    if RECORDER.lock().is_some() {
        println!("Already recording");
        return;
    }

    let recordings_dir = match ensure_recordings_dir(app) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to initialize recordings directory: {}", e);
            return;
        }
    };
    let file_name = generate_unique_wav_name();
    let file_path = recordings_dir.join(&file_name);
    *CURRENT_FILE_NAME.lock() = Some(file_name.clone());

    let host = cpal::default_host();
    let device = match host.default_input_device() {
        Some(d) => d,
        None => {
            eprintln!("No input device available");
            return;
        }
    };
    let config = match device.default_input_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("No input config available: {}", e);
            return;
        }
    };

    let file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create WAV file: {}", e);
            return;
        }
    };
    let writer = BufWriter::new(file);
    let spec = WavSpec {
        channels: 1,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let wav_writer = match WavWriter::new(writer, spec) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to create WAV writer: {}", e);
            return;
        }
    };

    let writer_arc = Arc::new(Mutex::new(Some(wav_writer)));

    *RECORDER.lock() = Some(writer_arc.clone());

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config, writer_arc, app.clone()),
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config, writer_arc, app.clone()),
        cpal::SampleFormat::I32 => build_stream::<i32>(&device, &config, writer_arc, app.clone()),
        _ => {
            eprintln!("Unsupported sample format");
            return;
        }
    };

    match stream.play() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to start stream: {}", e);
            return;
        }
    }
    *STREAM.lock() = Some(stream);

    println!("Recording started");
    let s = crate::settings::load_settings(app);
    if s.overlay_mode.as_str() == "recording" {
        overlay::show_recording_overlay(app);
    }
}

pub fn stop_recording(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    println!("Stopping audio recording...");

    if let Some(stream) = STREAM.lock().take() {
        drop(stream);
    }
    if let Some(recorder_arc) = RECORDER.lock().take() {
        let mut recorder = recorder_arc.lock();
        if let Some(writer) = recorder.take() {
            if let Err(e) = writer.finalize() {
                eprintln!("Failed to finalize WAV file: {}", e);
            }
        }
    }

    if let Some(file_name) = CURRENT_FILE_NAME.lock().take() {
        let path = ensure_recordings_dir(app)
            .map(|dir| dir.join(&file_name))
            .ok();
        if let Some(ref p) = path {
            println!("Recording stopped and saved as {}", p.display());

            match preload_engine(app) {
                Ok(_) => match transcribe_audio(p.as_path()) {
                    Ok(raw_text) => {
                        println!("Raw transcription: {}", raw_text);
                        match get_cc_rules_path(app) {
                            Ok(cc_rules_path) => {
                                let dictionary = app.state::<Dictionary>().get();
                                let text = fix_transcription_with_dictionary(
                                    raw_text,
                                    dictionary,
                                    cc_rules_path,
                                );
                                println!("Transcription fixed with dictionary: {}", text);
                                if let Err(e) = history::add_transcription(app, text.clone()) {
                                    eprintln!("Failed to save to history: {}", e);
                                }
                                if let Err(e) = write_transcription(app, &text) {
                                    eprintln!("Failed to use clipboard: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get CC rules path: {}", e);
                            }
                        }
                    }
                    Err(e) => eprintln!("Transcription failed: {}", e),
                },
                Err(e) => {
                    eprintln!(
                        "Cannot transcribe: Model not available. Please download a model first."
                    );
                    eprintln!("Error details: {}", e);
                }
            }
        } else {
            println!("Recording stopped and saved as {}", file_name);
        }
        // Emit a final zero level to let frontend reset visualizer
        let _ = app.emit("mic-level", 0.0f32);
        let s = crate::settings::load_settings(app);
        if s.overlay_mode.as_str() == "recording" {
            overlay::hide_recording_overlay(app);
        }
        return path;
    } else {
        println!("Recording stopped");
    }
    None
}

pub fn write_transcription(
    app: &tauri::AppHandle,
    transcription: &str,
) -> Result<(), anyhow::Error> {
    if let Err(e) = clipboard::paste(transcription, app) {
        eprintln!("Failed to paste text: {}", e);
    }

    if let Err(e) = cleanup_recordings(app) {
        eprintln!("Failed to cleanup recordings: {}", e);
    } else {
        println!("All recordings cleaned up");
    }

    println!("Transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn read_wav_samples(wav_path: &std::path::Path) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(wav_path)?;
    let spec = reader.spec();

    if spec.bits_per_sample != 16 {
        return Err(anyhow::anyhow!(
            "Expected 16 bits per sample, found {}",
            spec.bits_per_sample
        ));
    }

    if spec.sample_format != hound::SampleFormat::Int {
        return Err(anyhow::anyhow!(
            "Expected Int sample format, found {:?}",
            spec.sample_format
        ));
    }

    let raw_i16: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
    let mut raw_i16 = raw_i16?;

    if spec.channels > 1 {
        let ch = spec.channels as usize;
        let mut mono: Vec<i16> = Vec::with_capacity(raw_i16.len() / ch);
        for frame in raw_i16.chunks_exact(ch) {
            let sum: i32 = frame.iter().map(|&s| s as i32).sum();
            let avg = (sum / ch as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            mono.push(avg);
        }
        raw_i16 = mono;
    }

    let samples_f32: Vec<f32> = raw_i16
        .into_iter()
        .map(|s| s as f32 / i16::MAX as f32)
        .collect();

    let out = if spec.sample_rate != 16000 {
        resample_linear(&samples_f32, spec.sample_rate as usize, 16000)
    } else {
        samples_f32
    };

    Ok(out)
}

pub fn preload_engine(app: &tauri::AppHandle) -> Result<()> {
    let mut engine = ENGINE.lock();

    if engine.is_none() {
        let model = app.state::<Arc<Model>>();
        let model_path = model
            .get_model_path()
            .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

        let mut new_engine = ParakeetEngine::new();
        new_engine
            .load_model_with_params(&model_path, ParakeetModelParams::int8())
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        *engine = Some(new_engine);
        println!("Model loaded and cached in memory");
    }

    Ok(())
}

pub fn transcribe_audio(audio_path: &std::path::Path) -> Result<String> {
    let samples = read_wav_samples(audio_path)?;

    let mut engine = ENGINE.lock();
    let engine = engine
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Engine not loaded"))?;

    let result = engine
        .transcribe_samples(samples, None)
        .map_err(|e| anyhow::anyhow!("Transcription failed: {}", e))?;

    Ok(result.text)
}

fn ensure_recordings_dir(app: &tauri::AppHandle) -> Result<PathBuf> {
    let recordings = app
        .path()
        .app_data_dir()
        .context("Failed to resolve app data dir")?
        .join("recordings");

    if !recordings.exists() {
        std::fs::create_dir_all(&recordings).context("Failed to create recordings dir")?;
    }

    Ok(recordings)
}

fn cleanup_recordings(app: &tauri::AppHandle) -> Result<()> {
    let recordings_dir = ensure_recordings_dir(app)?;

    if !recordings_dir.exists() {
        return Ok(());
    }

    let entries =
        std::fs::read_dir(&recordings_dir).context("Failed to read recordings directory")?;

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    eprintln!("Failed to delete {}: {}", entry.path().display(), e);
                }
            }
        }
    }

    Ok(())
}

fn generate_unique_wav_name() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("murmure-{}.wav", ts)
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    writer: Arc<RecorderType>,
    app: AppHandle,
) -> cpal::Stream
where
    T: cpal::Sample + cpal::SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels() as usize;
    let _sample_rate = config.sample_rate().0 as f32;
    // State for simple RMS + EMA smoothing and throttled emission
    let mut acc_sum_squares: f32 = 0.0;
    let mut acc_count: usize = 0;
    let mut ema_level: f32 = 0.0;
    let alpha: f32 = 0.35; // smoothing factor
    let mut last_emit = std::time::Instant::now();

    device
        .build_input_stream(
            &config.clone().into(),
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let mut recorder = writer.lock();
                if let Some(writer) = recorder.as_mut() {
                    for frame in data.chunks_exact(channels) {
                        let sample = if channels == 1 {
                            frame[0].to_sample::<f32>()
                        } else {
                            frame.iter().map(|&s| s.to_sample::<f32>()).sum::<f32>()
                                / channels as f32
                        };

                        // write to WAV
                        let sample_i16 = (sample * i16::MAX as f32) as i16;
                        if let Err(e) = writer.write_sample(sample_i16) {
                            eprintln!("Error writing sample: {}", e);
                        }

                        // accumulate for RMS
                        acc_sum_squares += sample * sample;
                        acc_count += 1;
                    }
                }

                // Throttle to ~30 FPS
                if last_emit.elapsed() >= std::time::Duration::from_millis(33) {
                    if acc_count > 0 {
                        let rms = (acc_sum_squares / acc_count as f32).sqrt();
                        // Normalize a bit and clamp
                        let mut level = (rms * 1.5).min(1.0);
                        // simple noise gate
                        if level < 0.02 {
                            level = 0.0;
                        }
                        // EMA smoothing
                        ema_level = alpha * level + (1.0 - alpha) * ema_level;
                        let _ = app.emit("mic-level", ema_level);
                        // also forward to overlay window if present
                        if let Some(overlay_window) = app.get_webview_window("recording_overlay") {
                            let _ = overlay_window.emit("mic-level", ema_level);
                        }
                        acc_sum_squares = 0.0;
                        acc_count = 0;
                    } else {
                        let _ = app.emit("mic-level", 0.0f32);
                        if let Some(overlay_window) = app.get_webview_window("recording_overlay") {
                            let _ = overlay_window.emit("mic-level", 0.0f32);
                        }
                    }
                    last_emit = std::time::Instant::now();
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .expect("Failed to build input stream")
}

fn resample_linear(input: &[f32], src_hz: usize, dst_hz: usize) -> Vec<f32> {
    if input.is_empty() || src_hz == 0 || dst_hz == 0 {
        return Vec::new();
    }
    if src_hz == dst_hz {
        return input.to_vec();
    }
    let ratio = dst_hz as f64 / src_hz as f64;
    let out_len = ((input.len() as f64) * ratio).ceil() as usize;
    if out_len == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(out_len);
    let last_idx = input.len().saturating_sub(1);
    for i in 0..out_len {
        let t = (i as f64) / ratio;
        let idx = t.floor() as usize;
        let frac = (t - idx as f64) as f32;
        let a = input[idx];
        let b = input[std::cmp::min(idx + 1, last_idx)];
        out.push(a + (b - a) * frac);
    }
    out
}
