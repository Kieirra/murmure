use crate::audio::helpers::resample_linear;
use crate::audio::types::{AudioState, RecordingMode, RecordingTrigger};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::engine::ParakeetModelParams;
use crate::shortcuts::types::{recording_state, RecordingSource};
use crate::wake_word::types::{WakeWordAction, WakeWordEntry, WakeWordState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{debug, error, info, trace, warn};
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use strsim::levenshtein;
use tauri::{AppHandle, Emitter, Manager};
use unicode_normalization::UnicodeNormalization;

const SPEECH_THRESHOLD: f32 = 0.015;
const SILENCE_THRESHOLD: f32 = 0.01;
const SPEECH_START_DELAY_MS: u64 = 120;
const SPEECH_END_DELAY_MS: u64 = 400;
const MAX_SEGMENT_DURATION_S: f32 = 2.0;
/// Must be > SPEECH_START_DELAY_MS to avoid clipping the onset of speech.
const PRE_BUFFER_DURATION_MS: f32 = 400.0;
/// If no audio data is received for this duration, consider the stream dead.
const STREAM_INACTIVITY_TIMEOUT_S: u64 = 10;
/// Interval between early partial transcription checks during speech accumulation.
const EARLY_CHECK_INTERVAL_MS: u64 = 300;
/// Minimum buffer duration before the first early partial transcription check.
const EARLY_CHECK_MIN_BUFFER_MS: u64 = 400;
/// Smoothing factor for exponential moving average of RMS energy.
const EMA_ALPHA: f32 = 0.3;

pub(crate) fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .nfd()
        // NFD decomposes é into e + \u{0301}; filter out the combining marks
        .filter(|c| !('\u{0300}'..='\u{036F}').contains(c))
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|w| match w {
            "okay" => "ok",
            "alice" => "alix",
            _ => w,
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

fn matches_wake_word(transcription: &str, wake_word: &str) -> bool {
    if transcription.contains(wake_word) {
        return true;
    }

    let ww_words: Vec<&str> = wake_word.split_whitespace().collect();
    let tr_words: Vec<&str> = transcription.split_whitespace().collect();

    if ww_words.len() == 1 {
        // Single-word wake word: fuzzy match per word
        let max_distance = if wake_word.len() <= 3 { 1 } else { 2 };
        tr_words
            .iter()
            .any(|word| levenshtein(word, wake_word) <= max_distance)
    } else {
        // Multi-word wake word: sliding window with per-word fuzzy matching
        if tr_words.len() < ww_words.len() {
            return false;
        }
        for window in tr_words.windows(ww_words.len()) {
            let all_match = window.iter().zip(ww_words.iter()).all(|(tw, ww)| {
                let max_distance = if ww.len() <= 3 { 1 } else { 2 };
                levenshtein(tw, ww) <= max_distance
            });
            if all_match {
                return true;
            }
        }
        false
    }
}

pub fn start_listener(app: &AppHandle) {
    let state = app.state::<WakeWordState>();

    if state.is_active() {
        debug!("Wake word listener already active");
        return;
    }

    let settings = crate::settings::load_settings(app);

    let mut entries: Vec<WakeWordEntry> = Vec::new();
    if !settings.wake_word_record.trim().is_empty() {
        entries.push(WakeWordEntry {
            word: normalize_text(&settings.wake_word_record),
            action: WakeWordAction::Record(RecordingMode::Standard),
        });
    }
    let llm_settings = crate::llm::helpers::load_llm_connect_settings(app);
    for (index, mode) in llm_settings.modes.iter().enumerate() {
        if !mode.wake_word.trim().is_empty() {
            entries.push(WakeWordEntry {
                word: normalize_text(&mode.wake_word),
                action: WakeWordAction::RecordLlmMode(index),
            });
        }
    }
    if !settings.wake_word_command.trim().is_empty() {
        entries.push(WakeWordEntry {
            word: normalize_text(&settings.wake_word_command),
            action: WakeWordAction::Record(RecordingMode::Command),
        });
    }
    if !settings.wake_word_cancel.trim().is_empty() {
        entries.push(WakeWordEntry {
            word: normalize_text(&settings.wake_word_cancel),
            action: WakeWordAction::Cancel,
        });
    }
    if !settings.wake_word_validate.trim().is_empty() {
        entries.push(WakeWordEntry {
            word: normalize_text(&settings.wake_word_validate),
            action: WakeWordAction::Validate,
        });
    }

    if entries.is_empty() {
        warn!("No wake words configured, listener not started");
        return;
    }

    // Sort by word count descending so longer (more specific) wake words match first
    entries.sort_by(|a, b| {
        b.word
            .split_whitespace()
            .count()
            .cmp(&a.word.split_whitespace().count())
    });

    let stop_signal = state.stop_signal.clone();
    let active = state.active.clone();

    stop_signal.store(false, Ordering::SeqCst);

    let app_handle = app.clone();

    let handle = std::thread::spawn(move || {
        if let Err(e) = listener_loop(&app_handle, &entries, &stop_signal, &active) {
            error!("Wake word listener error: {}", e);
        }
        active.store(false, Ordering::SeqCst);
        if !stop_signal.load(Ordering::SeqCst) {
            trace!("Wake word stream died, restarting listener");
            start_listener(&app_handle);
        } else {
            debug!("Wake word listener thread exited");
        }
    });

    *state.thread_handle.lock() = Some(handle);

    let _ = app.emit("wake-word-listening", true);
    trace!("Wake word listener started");
}

pub fn stop_listener(app: &AppHandle) {
    let state = app.state::<WakeWordState>();

    if !state.is_active() {
        debug!("Wake word listener already inactive");
        state.stop_signal.store(true, Ordering::SeqCst);
        return;
    }

    state.stop_signal.store(true, Ordering::SeqCst);

    let handle = state.thread_handle.lock().take();
    if let Some(h) = handle {
        let _ = h.join();
    }

    let _ = app.emit("wake-word-listening", false);
    info!("Wake word listener stopped");
}

pub fn resume_listener(app: &AppHandle) {
    let settings = crate::settings::load_settings(app);
    if settings.wake_word_enabled {
        debug!("Resuming wake word listener");
        start_listener(app);
    }
}

/// Try to match the transcription against wake word entries and trigger the
/// corresponding action. Returns `true` if a wake word was matched and acted upon.
fn try_handle_wake_word(
    app: &AppHandle,
    text: &str,
    normalized: &str,
    entries: &[WakeWordEntry],
    source: &str,
) -> bool {
    let is_recording = {
        let audio_state = app.state::<AudioState>();
        let recording = audio_state.recorder.lock().is_some();
        recording
    };

    for entry in entries {
        if matches_wake_word(normalized, &entry.word) {
            match entry.action {
                WakeWordAction::Record(mode) if !is_recording => {
                    info!(
                        "Wake word detected ({}): \"{}\" -> mode {:?}",
                        source, text, mode
                    );
                    let _ = app.emit("wake-word-detected", ());
                    trigger_recording(app, mode);
                    return true;
                }
                WakeWordAction::RecordLlmMode(index) if !is_recording => {
                    info!(
                        "Wake word detected ({}): \"{}\" -> LLM mode {}",
                        source, text, index
                    );
                    let _ = app.emit("wake-word-detected", ());
                    crate::llm::switch_active_mode(app, index);
                    trigger_recording(app, RecordingMode::Llm);
                    return true;
                }
                WakeWordAction::Cancel if is_recording => {
                    info!("Cancel wake word detected ({}): \"{}\"", source, text);
                    let _ = app.emit("wake-word-detected", ());
                    trigger_cancel(app);
                    return true;
                }
                WakeWordAction::Validate if is_recording => {
                    info!("Validate wake word detected ({}): \"{}\"", source, text);
                    let _ = app.emit("wake-word-detected", ());
                    trigger_validate(app);
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}

fn listener_loop(
    app: &AppHandle,
    entries: &[WakeWordEntry],
    stop_signal: &Arc<std::sync::atomic::AtomicBool>,
    active: &Arc<std::sync::atomic::AtomicBool>,
) -> anyhow::Result<()> {
    let device = get_device(app)?;
    let config = device
        .default_input_config()
        .map_err(|e| anyhow::anyhow!("No input config: {}", e))?;

    let sample_rate = config.sample_rate() as usize;
    let channels = config.channels() as usize;

    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let stop = stop_signal.clone();

    let max_samples = (MAX_SEGMENT_DURATION_S * sample_rate as f32) as usize;
    let pre_buffer_capacity = (PRE_BUFFER_DURATION_MS / 1000.0 * sample_rate as f32) as usize;

    let stream_error = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let (stream, shared_buffer) = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let sb = new_shared_buffer(max_samples);
            let sb_ret = sb.clone();
            let mut vad_state = VadState::new(max_samples, pre_buffer_capacity, sb);
            let tx_clone = tx.clone();
            let stop_clone = stop.clone();
            let stream = device.build_input_stream(
                &config.clone().into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if stop_clone.load(Ordering::SeqCst) {
                        return;
                    }
                    process_audio_callback(data, channels, &mut vad_state, &tx_clone);
                },
                {
                    let se = stream_error.clone();
                    move |err| {
                        error!("Wake word stream error: {}", err);
                        se.store(true, Ordering::SeqCst);
                    }
                },
                None,
            )?;
            (stream, sb_ret)
        }
        cpal::SampleFormat::I16 => {
            let sb = new_shared_buffer(max_samples);
            let sb_ret = sb.clone();
            let mut vad_state = VadState::new(max_samples, pre_buffer_capacity, sb);
            let tx_clone = tx.clone();
            let stop_clone = stop.clone();
            let stream = device.build_input_stream(
                &config.clone().into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if stop_clone.load(Ordering::SeqCst) {
                        return;
                    }
                    let f32_data: Vec<f32> =
                        data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    process_audio_callback(&f32_data, channels, &mut vad_state, &tx_clone);
                },
                {
                    let se = stream_error.clone();
                    move |err| {
                        error!("Wake word stream error: {}", err);
                        se.store(true, Ordering::SeqCst);
                    }
                },
                None,
            )?;
            (stream, sb_ret)
        }
        f => return Err(anyhow::anyhow!("Unsupported sample format: {:?}", f)),
    };

    stream
        .play()
        .map_err(|e| anyhow::anyhow!("Failed to start wake word stream: {}", e))?;

    active.store(true, Ordering::SeqCst);
    trace!(
        "Wake word listener loop running (sample_rate={})",
        sample_rate
    );

    let mut last_audio_time = std::time::Instant::now();
    let mut last_early_check = std::time::Instant::now();
    let mut early_check_active = false;
    let min_early_samples =
        (EARLY_CHECK_MIN_BUFFER_MS as f32 / 1000.0 * sample_rate as f32) as usize;

    loop {
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        // Use a shorter timeout to allow periodic early checks
        match rx.recv_timeout(std::time::Duration::from_millis(50)) {
            Ok(segment) => {
                last_audio_time = std::time::Instant::now();
                // A completed segment arrived (silence cutoff or max duration).
                // Reset early check state since the segment is done.
                early_check_active = false;

                if stop_signal.load(Ordering::SeqCst) {
                    break;
                }

                if let Some((text, normalized)) =
                    resample_and_transcribe(app, &segment, sample_rate)
                {
                    trace!(
                        "Wake word segment transcription: \"{}\" (normalized: \"{}\")",
                        text,
                        normalized
                    );
                    try_handle_wake_word(app, &text, &normalized, entries, "segment");
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if stream_error.load(Ordering::SeqCst) {
                    warn!("Wake word stream error detected, exiting listener loop");
                    break;
                }
                if last_audio_time.elapsed()
                    >= std::time::Duration::from_secs(STREAM_INACTIVITY_TIMEOUT_S)
                {
                    trace!(
                        "No audio data received for {}s, stream presumed dead",
                        STREAM_INACTIVITY_TIMEOUT_S
                    );
                    break;
                }

                // Early partial transcription check: snapshot the in-progress
                // buffer and transcribe it to detect the wake word sooner.
                let snapshot = {
                    if let Ok(shared) = shared_buffer.try_lock() {
                        if shared.speech_active && shared.buffer.len() >= min_early_samples {
                            Some(shared.buffer.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(partial_buf) = snapshot {
                    if !early_check_active {
                        // First time we see enough buffer: start the early check timer
                        early_check_active = true;
                        last_early_check = std::time::Instant::now()
                            - std::time::Duration::from_millis(EARLY_CHECK_INTERVAL_MS);
                    }

                    if last_early_check.elapsed()
                        >= std::time::Duration::from_millis(EARLY_CHECK_INTERVAL_MS)
                    {
                        last_early_check = std::time::Instant::now();

                        if let Some((text, normalized)) =
                            resample_and_transcribe(app, &partial_buf, sample_rate)
                        {
                            trace!(
                                "Wake word early check: \"{}\" (normalized: \"{}\")",
                                text,
                                normalized
                            );
                            if try_handle_wake_word(
                                app,
                                &text,
                                &normalized,
                                entries,
                                "early",
                            ) {
                                // Wake word detected early, drain pending
                                // segments to avoid duplicate triggers
                                while rx.try_recv().is_ok() {}
                                early_check_active = false;
                            }
                        }
                    }
                } else {
                    early_check_active = false;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    drop(stream);
    Ok(())
}

/// Shared buffer state between the audio callback thread and the listener loop,
/// allowing the listener to periodically snapshot the in-progress buffer for
/// early partial transcription without waiting for silence or max duration.
struct SharedBufferInner {
    buffer: Vec<f32>,
    speech_active: bool,
}

type SharedBuffer = Arc<Mutex<SharedBufferInner>>;

fn new_shared_buffer(capacity: usize) -> SharedBuffer {
    Arc::new(Mutex::new(SharedBufferInner {
        buffer: Vec::with_capacity(capacity),
        speech_active: false,
    }))
}

struct VadState {
    buffer: Vec<f32>,
    max_samples: usize,
    pre_buffer: VecDeque<f32>,
    pre_buffer_capacity: usize,
    speech_active: bool,
    speech_start_time: Option<std::time::Instant>,
    silence_start_time: Option<std::time::Instant>,
    acc_sum_squares: f32,
    acc_count: usize,
    smoothed_rms: f32,
    last_check: std::time::Instant,
    shared_buffer: SharedBuffer,
}

impl VadState {
    fn new(max_samples: usize, pre_buffer_capacity: usize, shared_buffer: SharedBuffer) -> Self {
        Self {
            buffer: Vec::with_capacity(max_samples),
            max_samples,
            pre_buffer: VecDeque::with_capacity(pre_buffer_capacity),
            pre_buffer_capacity,
            speech_active: false,
            speech_start_time: None,
            silence_start_time: None,
            acc_sum_squares: 0.0,
            acc_count: 0,
            smoothed_rms: 0.0,
            last_check: std::time::Instant::now(),
            shared_buffer,
        }
    }

    /// Synchronize the local buffer state to the shared buffer so the listener
    /// loop can snapshot it for partial transcription.
    fn sync_shared_buffer(&self) {
        if let Ok(mut shared) = self.shared_buffer.try_lock() {
            shared.speech_active = self.speech_active;
            if self.speech_active {
                shared.buffer.clear();
                shared.buffer.extend_from_slice(&self.buffer);
            } else {
                shared.buffer.clear();
            }
        }
    }
}

fn process_audio_callback(
    data: &[f32],
    channels: usize,
    state: &mut VadState,
    tx: &mpsc::Sender<Vec<f32>>,
) {
    for frame in data.chunks_exact(channels) {
        let sample = if channels == 1 {
            frame[0]
        } else {
            frame.iter().sum::<f32>() / channels as f32
        };

        state.acc_sum_squares += sample * sample;
        state.acc_count += 1;

        if state.speech_active {
            if state.buffer.len() < state.max_samples {
                state.buffer.push(sample);
            }
        } else {
            if state.pre_buffer.len() >= state.pre_buffer_capacity {
                state.pre_buffer.pop_front();
            }
            state.pre_buffer.push_back(sample);
        }
    }

    if state.last_check.elapsed() < std::time::Duration::from_millis(33) {
        return;
    }
    state.last_check = std::time::Instant::now();

    if state.acc_count == 0 {
        return;
    }

    let rms = (state.acc_sum_squares / state.acc_count as f32).sqrt();
    state.acc_sum_squares = 0.0;
    state.acc_count = 0;

    state.smoothed_rms = EMA_ALPHA * rms + (1.0 - EMA_ALPHA) * state.smoothed_rms;

    if !state.speech_active {
        if state.smoothed_rms > SPEECH_THRESHOLD {
            match state.speech_start_time {
                Some(start) => {
                    if start.elapsed() >= std::time::Duration::from_millis(SPEECH_START_DELAY_MS) {
                        state.speech_active = true;
                        state.silence_start_time = None;

                        state.buffer.clear();
                        state.buffer.extend(state.pre_buffer.drain(..));
                        state.sync_shared_buffer();
                        trace!(
                            "Wake word VAD: speech started (pre-buffer: {} samples)",
                            state.buffer.len()
                        );
                    }
                }
                None => {
                    state.speech_start_time = Some(std::time::Instant::now());
                }
            }
        } else {
            state.speech_start_time = None;
        }
    } else {
        // Sync the shared buffer so the listener loop can snapshot it
        state.sync_shared_buffer();

        if rms < SILENCE_THRESHOLD {
            match state.silence_start_time {
                Some(start) => {
                    if start.elapsed() >= std::time::Duration::from_millis(SPEECH_END_DELAY_MS) {
                        let segment = std::mem::take(&mut state.buffer);
                        state.speech_active = false;
                        state.silence_start_time = None;
                        state.speech_start_time = None;
                        state.sync_shared_buffer();

                        if !segment.is_empty() {
                            let _ = tx.send(segment);
                        }
                    }
                }
                None => {
                    state.silence_start_time = Some(std::time::Instant::now());
                }
            }
        } else {
            state.silence_start_time = None;
        }

        if state.buffer.len() >= state.max_samples {
            let segment = std::mem::take(&mut state.buffer);
            state.speech_active = false;
            state.silence_start_time = None;
            state.speech_start_time = None;
            state.sync_shared_buffer();

            if !segment.is_empty() {
                let _ = tx.send(segment);
            }
        }
    }
}

/// Resample audio to 16 kHz if needed and transcribe it. Returns `(text, normalized)`
/// or `None` if the buffer is too short.
fn resample_and_transcribe(
    app: &AppHandle,
    samples: &[f32],
    sample_rate: usize,
) -> Option<(String, String)> {
    let samples_16k = if sample_rate != 16000 {
        resample_linear(samples, sample_rate, 16000)
    } else {
        samples.to_vec()
    };

    if samples_16k.len() < 1600 {
        return None;
    }

    match transcribe_segment(app, samples_16k) {
        Ok(text) => {
            let normalized = normalize_text(&text);
            Some((text, normalized))
        }
        Err(e) => {
            warn!("Wake word transcription failed: {}", e);
            None
        }
    }
}

fn transcribe_segment(app: &AppHandle, samples: Vec<f32>) -> anyhow::Result<String> {
    let audio_state = app.state::<AudioState>();

    {
        let mut engine_guard = audio_state.engine.lock();
        if engine_guard.is_none() {
            let model = app.state::<Arc<crate::model::Model>>();
            let model_path = model
                .get_model_path()
                .map_err(|e| anyhow::anyhow!("Failed to get model path: {}", e))?;

            let mut new_engine = crate::engine::ParakeetEngine::new();
            new_engine
                .load_model_with_params(&model_path, ParakeetModelParams::int8())
                .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

            *engine_guard = Some(new_engine);
            debug!("Model loaded for wake word detection");
        }
    }

    let mut engine_guard = audio_state.engine.lock();
    let engine = engine_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Engine not loaded"))?;

    let result = engine
        .transcribe_samples(samples, None)
        .map_err(|e| anyhow::anyhow!("Transcription failed: {}", e))?;

    Ok(result.text)
}

fn trigger_recording(app: &AppHandle, mode: RecordingMode) {
    let audio_state = app.state::<AudioState>();
    audio_state.set_recording_trigger(RecordingTrigger::WakeWord);

    crate::onboarding::onboarding::capture_focus_at_record_start(app);
    crate::audio::record_audio(app, mode);

    let source = match mode {
        RecordingMode::Standard => RecordingSource::Standard,
        RecordingMode::Llm => RecordingSource::Llm,
        RecordingMode::Command => RecordingSource::Command,
    };
    let mut src = recording_state().source.lock();
    *src = source;

    info!("Recording triggered by wake word (mode: {:?})", mode);
}

fn trigger_validate(app: &AppHandle) {
    // Set trigger to Keyboard so auto-enter in write_transcription won't double-fire
    let audio_state = app.state::<AudioState>();
    audio_state.set_recording_trigger(RecordingTrigger::Keyboard);

    // Set the wake word to strip from the transcription
    let settings = crate::settings::load_settings(app);
    *audio_state.strip_word.lock() = Some(settings.wake_word_validate);

    let mut source = recording_state().source.lock();
    *source = RecordingSource::None;
    drop(source);

    // Stop recording normally (transcribes + pastes, stripping the wake word)
    crate::audio::stop_recording(app);

    // Simulate Enter after transcription
    match crate::audio::simulate_enter_key() {
        Ok(()) => info!("Enter key simulated by validate wake word"),
        Err(e) => error!("Failed to simulate Enter key: {}", e),
    }
}

fn trigger_cancel(app: &AppHandle) {
    let mut source = recording_state().source.lock();
    *source = RecordingSource::None;
    drop(source);

    crate::audio::cancel_recording(app);
    info!("Recording cancelled by cancel wake word");
}

fn get_device(app: &AppHandle) -> anyhow::Result<cpal::Device> {
    let audio_state = app.state::<AudioState>();

    if let Some(device) = audio_state.get_cached_device() {
        return Ok(device);
    }

    let host = cpal::default_host();
    host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No default input device available"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text_lowercase() {
        assert_eq!(normalize_text("MURMURE"), "murmure");
    }

    #[test]
    fn test_normalize_text_accents() {
        assert_eq!(normalize_text("murmùre"), "murmure");
        assert_eq!(normalize_text("écoute"), "ecoute");
        assert_eq!(normalize_text("café"), "cafe");
    }

    #[test]
    fn test_normalize_text_punctuation() {
        assert_eq!(normalize_text("murmure!"), "murmure");
        assert_eq!(normalize_text("murmure."), "murmure");
        assert_eq!(normalize_text("\"murmure\""), "murmure");
    }

    #[test]
    fn test_normalize_text_whitespace() {
        assert_eq!(normalize_text("  murmure  "), "murmure");
        assert_eq!(normalize_text("bonjour   murmure"), "bonjour murmure");
    }

    #[test]
    fn test_normalize_text_combined() {
        assert_eq!(normalize_text("  Écoute, MURMÙRE!  "), "ecoute murmure");
    }

    #[test]
    fn test_matches_wake_word_exact_substring() {
        assert!(matches_wake_word("bonjour murmure comment", "murmure"));
    }

    #[test]
    fn test_matches_wake_word_exact_word() {
        assert!(matches_wake_word("murmure", "murmure"));
    }

    #[test]
    fn test_matches_wake_word_levenshtein_one_char() {
        // 1 edit distance: "murmur" vs "murmure" (missing 'e')
        assert!(matches_wake_word("murmur", "murmure"));
        // 1 edit distance: "murmurre" vs "murmure" (extra 'r')
        assert!(matches_wake_word("murmurre", "murmure"));
        // 1 edit distance: "nurmure" vs "murmure" (substitution)
        assert!(matches_wake_word("nurmure", "murmure"));
    }

    #[test]
    fn test_matches_wake_word_levenshtein_two_chars() {
        // 2 edit distance for 7-char word (threshold=2): should match
        assert!(matches_wake_word("mirmur", "murmure"));
    }

    #[test]
    fn test_matches_wake_word_too_distant() {
        // 3+ edit distance: should NOT match
        assert!(!matches_wake_word("miracle", "murmure"));
    }

    #[test]
    fn test_matches_wake_word_short_word() {
        // 4+ chars: threshold=2
        assert!(matches_wake_word("helo", "hello"));
        assert!(matches_wake_word("alice", "alix"));
        // <=3 chars: threshold=1
        assert!(matches_wake_word("ot", "ok"));
        assert!(!matches_wake_word("ab", "ok"));
    }

    #[test]
    fn test_matches_wake_word_in_sentence() {
        assert!(matches_wake_word(
            "bonjour nurmure comment ca va",
            "murmure"
        ));
    }

    #[test]
    fn test_matches_wake_word_no_match() {
        assert!(!matches_wake_word("bonjour comment ca va", "murmure"));
    }

    #[test]
    fn test_matches_multi_word_exact() {
        assert!(matches_wake_word("ok murmure", "ok murmure"));
    }

    #[test]
    fn test_matches_multi_word_in_sentence() {
        assert!(matches_wake_word("bonjour ok murmure merci", "ok murmure"));
    }

    #[test]
    fn test_matches_multi_word_fuzzy() {
        // "oc" is 1 edit from "ok" (<=3 chars, threshold=1)
        // "murmur" is 1 edit from "murmure" (>3 chars, threshold=2)
        assert!(matches_wake_word("oc murmur", "ok murmure"));
    }

    #[test]
    fn test_matches_multi_word_no_match() {
        assert!(!matches_wake_word("bonjour murmure", "ok murmure"));
    }

    #[test]
    fn test_matches_multi_word_too_short() {
        assert!(!matches_wake_word("ok", "ok murmure"));
    }

    #[test]
    fn test_normalize_text_okay_to_ok() {
        assert_eq!(normalize_text("okay"), "ok");
        assert_eq!(normalize_text("Okay Alix"), "ok alix");
        assert_eq!(normalize_text("okay alix"), "ok alix");
    }

    #[test]
    fn test_normalize_text_alice_to_alix() {
        assert_eq!(normalize_text("alice"), "alix");
        assert_eq!(normalize_text("Alice"), "alix");
        assert_eq!(normalize_text("Ok Alice"), "ok alix");
        assert_eq!(normalize_text("Okay Alice"), "ok alix");
        assert_eq!(normalize_text("alice command"), "alix command");
        assert_eq!(normalize_text("merci Alix"), "merci alix");
        assert_eq!(normalize_text("merci Alice"), "merci alix");
    }

    #[test]
    fn test_matches_okay_alix_wake_word() {
        let normalized = normalize_text("Okay Alix");
        assert!(matches_wake_word(&normalized, "ok alix"));
    }

    #[test]
    fn test_matches_alice_wake_word() {
        let normalized = normalize_text("Ok Alice");
        assert!(matches_wake_word(&normalized, "ok alix"));

        let normalized = normalize_text("Alice command");
        assert!(matches_wake_word(&normalized, "alix command"));

        let normalized = normalize_text("Alice cancel");
        assert!(matches_wake_word(&normalized, "alix cancel"));

        let normalized = normalize_text("merci Alix");
        assert!(matches_wake_word(&normalized, "merci alix"));

        let normalized = normalize_text("merci Alice");
        assert!(matches_wake_word(&normalized, "merci alix"));

        let normalized = normalize_text("OK, Alice.");
        assert!(matches_wake_word(&normalized, "ok alix"));

        let normalized = normalize_text("Okay, Alice.");
        assert!(matches_wake_word(&normalized, "ok alix"));
    }
}
