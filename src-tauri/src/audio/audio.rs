use crate::audio::clean_recording::strip_and_record;
use crate::audio::helpers::{cleanup_recordings, ensure_recordings_dir, generate_unique_wav_name};
use crate::audio::recorder::AudioRecorder;
use crate::audio::types::{AudioState, RecorderStartError, RecordingMode, RecordingTrigger};
use crate::audio::ChunkPipeline;
use crate::clipboard;
use crate::engine::ParakeetEngine;
use crate::model::Model;
use crate::overlay::overlay;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

pub fn record_audio(app: &AppHandle, mode: RecordingMode) {
    let state = app.state::<AudioState>();
    state.set_recording_mode(mode);
    if state.get_recording_trigger() != RecordingTrigger::WakeWord {
        state.set_recording_trigger(RecordingTrigger::Keyboard);
    }

    if matches!(mode, RecordingMode::Llm | RecordingMode::Command) {
        crate::llm::warmup_ollama_model_background(app);
    }

    let settings = crate::settings::load_settings(app);
    match state.get_recording_trigger() {
        _ if settings.long_dictation_enabled && mode == RecordingMode::Standard => {
            let app_cb = app.clone();
            let on_chunk: Arc<dyn Fn(String) + Send + Sync> = Arc::new(move |text: String| {
                if let Err(e) = crate::clipboard::paste(&text, &app_cb) {
                    error!("Long dictation: failed to paste chunk: {}", e);
                }
            });
            *state.chunk_pipeline.lock() = Some(ChunkPipeline::start(
                app,
                Some(on_chunk),
                crate::audio::chunking::LONG_DICTATION_SILENCE_ARM_SECS,
            ));
        }
        _ => {
            *state.chunk_pipeline.lock() = Some(ChunkPipeline::start(
                app,
                None,
                crate::audio::chunking::CHUNK_SILENCE_ARM_SECS,
            ));
        }
    }

    internal_record_audio(app);
}

fn internal_record_audio(app: &AppHandle) {
    debug!("Starting audio recording...");
    let state = app.state::<AudioState>();

    crate::audio::sound::prewarm(app);

    match start_new_recorder(app, true) {
        Ok(sample_rate) => {
            debug!("Recording started");
            let s = crate::settings::load_settings(app);
            if s.overlay_mode.as_str() == "recording" {
                overlay::clear_pending_flash(app);
                overlay::show_recording_overlay(app);
            }
            crate::overlay::tray::set_tray_recording(app);
            crate::audio::streaming::start_streaming(app, &state, sample_rate);
        }
        Err(RecorderStartError::InitFailed) => notify_recording_error(app),
        Err(_) => {}
    }
}

fn start_new_recorder(app: &AppHandle, play_sound: bool) -> Result<u32, RecorderStartError> {
    let state = app.state::<AudioState>();

    let recordings_dir = match ensure_recordings_dir(app) {
        Ok(dir) => dir,
        Err(e) => {
            error!("Failed to initialize recordings directory: {}", e);
            return Err(RecorderStartError::DirUnavailable);
        }
    };

    let file_name = generate_unique_wav_name();
    let file_path = recordings_dir.join(&file_name);
    let limit_reached = state.get_limit_reached_arc();

    // Hold the lock across check-and-install to serialize concurrent callers.
    let mut recorder_guard = state.recorder.lock();
    if recorder_guard.is_some() {
        warn!("Already recording");
        return Err(RecorderStartError::Busy);
    }

    let mut recorder = match AudioRecorder::new(app.clone(), &file_path, limit_reached) {
        Ok(recorder) => recorder,
        Err(e) => {
            error!("Failed to initialize recorder: {}", e);
            let _ = std::fs::remove_file(&file_path);
            return Err(RecorderStartError::InitFailed);
        }
    };

    if let Err(e) = recorder.start(play_sound) {
        error!("Failed to start recording: {}", e);
        let _ = std::fs::remove_file(&file_path);
        return Err(RecorderStartError::StartFailed);
    }

    let sample_rate = recorder.sample_rate();
    *state.current_file_name.lock() = Some(file_name);
    *recorder_guard = Some(recorder);
    Ok(sample_rate)
}

fn show_recording_notification(app: &AppHandle, event: &'static str, payload: String) {
    overlay::clear_pending_flash(app);
    overlay::show_recording_overlay(app);
    let app_clone = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let _ = app_clone.emit(event, payload);
        std::thread::sleep(std::time::Duration::from_millis(1500));
        overlay::hide_recording_overlay(&app_clone);
    });
}

fn notify_recording_error(app: &AppHandle) {
    let s = crate::settings::load_settings(app);
    let mic_name = s.mic_label.or(s.mic_id).unwrap_or_default();
    show_recording_notification(app, "recording-error", mic_name);
}

pub fn notify_recording_limit(app: &AppHandle) {
    show_recording_notification(app, "recording-limit-reached", String::new());
}

pub fn stop_recording(app: &AppHandle) -> Option<std::path::PathBuf> {
    debug!("Stopping audio recording...");
    let state = app.state::<AudioState>();

    crate::audio::sound::prewarm(app);

    crate::audio::sound::play_sound(app, crate::audio::sound::Sound::StopRecording);
    crate::audio::streaming::stop_streaming(app, &state);

    // Stopping the recorder drains the writer thread, so every chunk (including
    // the final remainder it flushes) is queued before the pipeline finalizes.
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop(false) {
                error!("Failed to stop recorder: {}", e);
            }
        }
        *recorder_guard = None;
    }

    let pipeline = state.chunk_pipeline.lock().take();

    let file_name_opt = state.current_file_name.lock().take();
    let path = ensure_recordings_dir(app)
        .ok()
        .zip(file_name_opt)
        .map(|(dir, name)| dir.join(name));

    match (path.as_ref(), pipeline) {
        (Some(p), Some(pipeline)) => {
            info!(
                "Audio recording stopped; file written to temporary path: {}",
                p.display()
            );
            finalize_chunked_session(app, &state, pipeline, p);
        }
        _ => {
            debug!("Recording stopped (no active file or pipeline)");
            reset_recording_ui(app);
        }
    }

    path
}

fn finalize_chunked_session(
    app: &AppHandle,
    state: &AudioState,
    pipeline: ChunkPipeline,
    path: &std::path::Path,
) {
    let _ = app.emit("llm-processing-start", ());
    let accumulated = pipeline.finalize();
    let _ = app.emit("llm-processing-end", ());
    let mode = state.get_recording_mode();

    match crate::audio::pipeline::merge_all_chunks(app, accumulated, path, mode) {
        Ok(result) => {
            let text = strip_and_record(app, state, result.text);
            if let Err(e) = write_transcription(app, &text) {
                error!("Failed to use clipboard: {}", e);
            }
            finish_recording_ui(app, result.llm_error);
        }
        Err(e) => {
            error!("Finalize failed: {}", e);
            reset_recording_ui(app);
        }
    }
}

fn finish_recording_ui(app: &AppHandle, llm_error: Option<String>) {
    match llm_error {
        Some(llm_err) => {
            let _ = app.emit("llm-error", llm_err);
            reset_recording_ui_delayed(app, 3000);
        }
        None => reset_recording_ui(app),
    }
}

pub fn cancel_recording(app: &AppHandle) {
    info!("Cancelling audio recording...");
    let state = app.state::<AudioState>();

    crate::audio::sound::prewarm(app);
    crate::audio::streaming::stop_streaming(app, &state);

    // Stop recorder without processing
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop(true) {
                error!("Failed to stop recorder on cancel: {}", e);
            }
        }
        *recorder_guard = None;
    }

    // Drop the pipeline without finalizing: the writer thread already exited, so
    // its sender is gone, and the worker drains its queue and stops.
    let _ = state.chunk_pipeline.lock().take();

    // Remove temporary WAV file
    let file_name_opt = state.current_file_name.lock().take();
    if let Some(file_name) = file_name_opt {
        if let Ok(dir) = ensure_recordings_dir(app) {
            let path = dir.join(&file_name);
            if let Err(e) = std::fs::remove_file(&path) {
                error!("Failed to remove cancelled recording file: {}", e);
            }
        }
    }

    reset_recording_ui(app);
    info!("Recording cancelled by user");
}

fn reset_recording_state(app: &AppHandle) {
    let state = app.state::<AudioState>();
    let _ = app.emit("mic-level", 0.0f32);
    state.set_recording_trigger(RecordingTrigger::Keyboard);
    crate::wake_word::resume_listener(app);
}

fn reset_recording_ui(app: &AppHandle) {
    reset_recording_state(app);
    crate::overlay::tray::set_tray_idle(app);
    let s = crate::settings::load_settings(app);
    if s.overlay_mode.as_str() == "recording" {
        overlay::hide_recording_overlay(app);
    }
}

fn reset_recording_ui_delayed(app: &AppHandle, delay_ms: u64) {
    reset_recording_state(app);
    crate::overlay::tray::set_tray_idle(app);
    let app_clone = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        let s = crate::settings::load_settings(&app_clone);
        if s.overlay_mode.as_str() == "recording" {
            overlay::hide_recording_overlay(&app_clone);
        }
    });
}

pub fn write_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    // Wayland: hide before paste so KWin/Mutter returns focus before Ctrl+V.
    // paste_with_delay's 400 ms settle relies on millis_since_last_overlay_hide.
    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            let s = crate::settings::load_settings(app);
            if s.overlay_mode.as_str() == "recording" {
                overlay::hide_recording_overlay(app);
            }
        }
    }

    if let Err(e) = clipboard::paste(transcription, app) {
        error!("Failed to paste text: {}", e);
    }

    if let Err(e) = cleanup_recordings(app) {
        error!("Failed to cleanup recordings: {}", e);
    }

    debug!("Transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn simulate_enter_key(app: &AppHandle) -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_millis(200));

    #[cfg(target_os = "linux")]
    {
        if crate::utils::platform::is_wayland_session() {
            log::info!("simulate_enter_key: Wayland path (uinput Enter)");
            return crate::utils::wayland_inject::enter();
        }
    }

    log::info!("simulate_enter_key: enigo path");
    use enigo::{Key, Keyboard};
    crate::utils::enigo_session::with_enigo(app, |enigo| {
        enigo
            .key(Key::Return, enigo::Direction::Click)
            .map_err(|e| format!("Failed to press Enter: {}", e))
    })
}

pub fn write_last_transcription(app: &AppHandle, transcription: &str) -> Result<()> {
    if let Err(e) = clipboard::paste_last_transcript(transcription, app) {
        error!("Failed to paste last transcription: {}", e);
    }

    debug!("Last transcription written to clipboard {}", transcription);
    Ok(())
}

pub fn preload_engine(app: &AppHandle) -> Result<()> {
    let state = app.state::<AudioState>();
    let mut engine = state.engine.lock();

    if engine.is_none() {
        let model = app.state::<Arc<Model>>();
        let model_path = model
            .get_model_path()
            .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

        let new_engine = ParakeetEngine::load_int8(&model_path, model.get_tokenizer_path())
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        *engine = Some(new_engine);
        info!("Model loaded and cached in memory");
    }

    Ok(())
}
