use crate::audio::helpers::{cleanup_recordings, ensure_recordings_dir, generate_unique_wav_name};
use crate::audio::pipeline::process_recording;
use crate::audio::recorder::AudioRecorder;
use crate::audio::types::{AudioState, RecordingMode, RecordingTrigger};
use crate::audio::{ChunkPipeline, WriteStrategy};
use crate::clipboard;
use crate::engine::ParakeetEngine;
use crate::model::Model;
use crate::overlay::overlay;
use crate::wake_word::wake_word::normalize_text;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use strsim::levenshtein;
use tauri::{AppHandle, Emitter, Manager};

pub fn record_audio(app: &AppHandle, mode: RecordingMode) {
    let state = app.state::<AudioState>();
    state.set_recording_mode(mode);
    if state.get_recording_trigger() != RecordingTrigger::WakeWord {
        state.set_recording_trigger(RecordingTrigger::Keyboard);
    }
    // Wake word listener stays active: validate/cancel words work during keyboard-triggered recording

    if matches!(mode, RecordingMode::Llm | RecordingMode::Command) {
        crate::llm::warmup_ollama_model_background(app);
    }

    // Three exclusive recording paths:
    // - Wake word: single-shot, no chunking, no long dictation.
    // - Live Text Mode (Standard + toggle on): write each segment on silence.
    // - Otherwise (keyboard): buffered chunking, preview drives the overlay.
    let s = crate::settings::load_settings(app);
    let live = s.long_dictation_enabled && mode == RecordingMode::Standard;
    if state.get_recording_trigger() == RecordingTrigger::WakeWord {
        state.long_dictation_active.store(false, Ordering::SeqCst);
        *state.chunk_pipeline.lock() = None;
    } else if live {
        state.long_dictation_active.store(true, Ordering::SeqCst);
        *state.chunk_pipeline.lock() = None;
    } else {
        state.long_dictation_active.store(false, Ordering::SeqCst);
        start_chunk_pipeline(app, &state);
    }

    internal_record_audio(app);
}

/// Buffers every chunk and writes the assembled text once at finalize. The
/// real-time preview (or visualizer) drives the overlay, so no buffered overlay
/// event is emitted here.
fn start_chunk_pipeline(app: &AppHandle, state: &AudioState) {
    let strategy = WriteStrategy::Buffered;
    debug!("Chunk pipeline started: strategy={:?}", strategy);
    let pipeline = ChunkPipeline::start(app, strategy);
    *state.chunk_pipeline.lock() = Some(pipeline);
}

enum RecorderStartError {
    Busy,
    DirUnavailable,
    InitFailed,
    StartFailed,
}

/// The caller must not already hold the recorder lock.
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
        // Only a failed device init pops the mic-error overlay, preserving the
        // original behavior: a busy recorder or a failed stream start stays silent.
        Err(RecorderStartError::InitFailed) => notify_recording_error(app),
        Err(_) => {}
    }
}

fn notify_recording_error(app: &AppHandle) {
    let s = crate::settings::load_settings(app);
    let mic_name = s.mic_label.or(s.mic_id).unwrap_or_default();
    overlay::clear_pending_flash(app);
    overlay::show_recording_overlay(app);
    let app_clone = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let _ = app_clone.emit("recording-error", mic_name);
        std::thread::sleep(std::time::Duration::from_millis(1500));
        overlay::hide_recording_overlay(&app_clone);
    });
}

pub fn stop_recording(app: &AppHandle) -> Option<std::path::PathBuf> {
    debug!("Stopping audio recording...");
    let state = app.state::<AudioState>();

    crate::audio::sound::prewarm(app);
    crate::audio::streaming::stop_streaming(app, &state);

    // Stopping the recorder drains the writer thread, so every chunk (including
    // the final remainder it flushes) is queued before the pipeline finalizes.
    {
        let mut recorder_guard = state.recorder.lock();
        if let Some(recorder) = recorder_guard.as_mut() {
            if let Err(e) = recorder.stop(true) {
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

    match path.as_ref() {
        Some(p) => {
            info!(
                "Audio recording stopped; file written to temporary path: {}",
                p.display()
            );
            match pipeline {
                Some(pipeline) => finalize_chunked_session(app, &state, pipeline, p),
                None => finalize_single_recording(app, &state, p),
            }
        }
        None => {
            debug!("Recording stopped (no active file)");
            reset_recording_ui(app);
        }
    }

    // Reset after processing so the last long-dictation segment is still
    // formatted with the flag active (no short-text correction on it).
    state.long_dictation_active.store(false, Ordering::SeqCst);

    path
}

/// Chunked session: the worker buffered every chunk. Drain it for the stitched
/// text, post-process once, then paste the whole block. The overlay is driven by
/// the preview/visualizer, so no buffered/inserting/done event is emitted.
fn finalize_chunked_session(
    app: &AppHandle,
    state: &AudioState,
    pipeline: ChunkPipeline,
    path: &std::path::Path,
) {
    let accumulated = pipeline.finalize();
    let mode = state.get_recording_mode();

    match crate::audio::pipeline::finalize_recording(app, accumulated, path, mode) {
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

/// Non-chunking session (wake word): single-shot transcription of the full WAV.
fn finalize_single_recording(app: &AppHandle, state: &AudioState, path: &std::path::Path) {
    match process_recording(app, path) {
        Ok(result) => {
            let text = strip_and_record(app, state, result.text);
            if let Err(e) = write_transcription(app, &text) {
                error!("Failed to use clipboard: {}", e);
            }
            finish_recording_ui(app, result.llm_error);
        }
        Err(e) => {
            error!("Processing failed: {}", e);
            reset_recording_ui(app);
        }
    }
}

/// Strips a trailing validation/submit wake word from the final concatenated
/// text and keeps history in sync. Applied once, never per chunk.
fn strip_and_record(app: &AppHandle, state: &AudioState, text: String) -> String {
    match state.strip_word.lock().take() {
        Some(word) => {
            let stripped = strip_trailing_wake_word(&text, &word);
            if stripped != text {
                if let Err(e) = crate::history::update_last_transcription(app, stripped.clone()) {
                    error!("Failed to update history after wake word strip: {}", e);
                }
            }
            stripped
        }
        None => text,
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
    state.long_dictation_active.store(false, Ordering::SeqCst);

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

pub fn flush_and_continue_dictation(app: &AppHandle) {
    let state = app.state::<AudioState>();
    if !state.long_dictation_active.load(Ordering::SeqCst) {
        return;
    }

    let old_path = {
        let mut recorder_guard = state.recorder.lock();
        let recorder = match recorder_guard.as_mut() {
            Some(recorder) => recorder,
            None => return,
        };
        if let Err(e) = recorder.stop(false) {
            error!("Long dictation: failed to stop segment recorder: {}", e);
        }
        *recorder_guard = None;
        state
            .current_file_name
            .lock()
            .take()
            .and_then(|name| ensure_recordings_dir(app).map(|dir| dir.join(name)).ok())
    };

    let restarted = restart_dictation_recorder(app);

    match old_path {
        Some(path) => {
            let app = app.clone();
            std::thread::spawn(move || {
                // Paste directly instead of write_transcription: the latter runs a
                // global cleanup that would delete the next segment's WAV (already
                // being recorded). Remove only this segment's file.
                match process_recording(&app, &path) {
                    Ok(result) => {
                        if !result.text.trim().is_empty() {
                            if let Err(e) = clipboard::paste(&result.text, &app) {
                                error!("Long dictation: failed to paste segment: {}", e);
                            }
                        }
                    }
                    Err(e) => error!("Long dictation segment transcription failed: {}", e),
                }
                if let Err(e) = std::fs::remove_file(&path) {
                    error!("Long dictation: failed to remove segment WAV: {}", e);
                }
                // Stop only after the last segment is pasted, so it is still
                // formatted with the long-dictation flag active.
                if !restarted {
                    abort_long_dictation(&app);
                }
            });
        }
        None => {
            if !restarted {
                abort_long_dictation(app);
            }
        }
    }
}

/// Returns false when the session can no longer record. A busy recorder is not
/// a failure: one is already running, keep going.
fn restart_dictation_recorder(app: &AppHandle) -> bool {
    !matches!(
        start_new_recorder(app, false),
        Err(RecorderStartError::DirUnavailable
            | RecorderStartError::InitFailed
            | RecorderStartError::StartFailed)
    )
}

fn abort_long_dictation(app: &AppHandle) {
    error!("Long dictation: could not start next segment, stopping session");
    let state = app.state::<AudioState>();
    crate::audio::streaming::stop_streaming(app, &state);
    state.long_dictation_active.store(false, Ordering::SeqCst);
    reset_recording_state(app);
    crate::overlay::tray::set_tray_idle(app);
    notify_recording_error(app);
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
    // Linux+Wayland only: hide BEFORE paste so KWin/Mutter hand
    // keyboard focus back to the target app before Ctrl+V fires.
    // The 400 ms settle in `clipboard::paste_with_delay` relies on
    // this order via `overlay::millis_since_last_overlay_hide`.
    // Other platforms keep the original timing (hide after paste in
    // `reset_recording_ui*`).
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

fn strip_trailing_wake_word(text: &str, wake_word: &str) -> String {
    let ww = wake_word.trim();
    if ww.is_empty() {
        return text.to_string();
    }

    let trimmed = text.trim();
    let text_words: Vec<&str> = trimmed.split_whitespace().collect();

    let ww_normalized = normalize_text(ww);
    let ww_words: Vec<&str> = ww_normalized.split_whitespace().collect();

    if text_words.len() < ww_words.len() {
        return trimmed.to_string();
    }

    // Search within the last words with a margin of 2 for trailing noise from STT
    let margin = 2;
    let earliest_start = text_words.len().saturating_sub(ww_words.len() + margin);

    for start in earliest_start..=(text_words.len() - ww_words.len()) {
        let candidate = &text_words[start..start + ww_words.len()];

        let all_match = candidate.iter().zip(ww_words.iter()).all(|(tw, ww_w)| {
            let tw_norm = normalize_text(tw);
            let max_distance = if ww_w.len() <= 3 { 1 } else { 2 };
            levenshtein(&tw_norm, ww_w) <= max_distance
        });

        if all_match {
            // Remove everything from the matched position to the end
            let result = text_words[..start].join(" ");
            debug!(
                "Stripped trailing wake word \"{}\" from transcription",
                wake_word
            );
            return result;
        }
    }

    trimmed.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_exact_match_single_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour validate", "validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_exact_match_multi_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde alix validate", "alix validate"),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_fuzzy_match_accent() {
        // STT transcribes "validé" instead of "validate" — Levenshtein ≤ 2
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validé", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_fuzzy_match_typo() {
        // STT transcribes "validatte" — Levenshtein ≤ 2
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validatte", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_fuzzy_match_missing_char() {
        // STT transcribes "validat" — Levenshtein = 1
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validat", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_with_trailing_noise() {
        // Trailing noise word after wake word — margin handles it
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validate ok", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_case_insensitive() {
        assert_eq!(
            strip_trailing_wake_word("bonjour Alix Validate", "alix validate"),
            "bonjour"
        );
    }

    #[test]
    fn strip_no_match_returns_original() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde", "alix validate"),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_empty_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("bonjour le monde", ""),
            "bonjour le monde"
        );
    }

    #[test]
    fn strip_text_shorter_than_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("validate", "alix validate"),
            "validate"
        );
    }

    #[test]
    fn strip_only_wake_word() {
        assert_eq!(
            strip_trailing_wake_word("alix validate", "alix validate"),
            ""
        );
    }

    #[test]
    fn strip_with_punctuation_from_stt() {
        assert_eq!(
            strip_trailing_wake_word("bonjour alix validate.", "alix validate"),
            "bonjour"
        );
    }
}
