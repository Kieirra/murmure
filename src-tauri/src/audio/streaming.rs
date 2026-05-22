use crate::audio::helpers::resample_linear;
use crate::audio::types::{AudioState, TranscriptionFinalizationStrategy};
use crate::dictionary::{fix_transcription_with_dictionary, get_cc_rules_path, Dictionary};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::formatting_rules;
use crate::formatting_rules::highlighter::{
    apply_formatting_with_highlights_and_original, HighlightRange,
};
use log::{debug, error, warn};
use parking_lot::Mutex;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

const SPEECH_THRESHOLD: f32 = 0.015;
const SILENCE_THRESHOLD: f32 = 0.01;
const SPEECH_START_DELAY_MS: u64 = 120;
const SPEECH_END_DELAY_MS: u64 = 400;
const MAX_SEGMENT_DURATION_S: f32 = 5.0;
const PRE_BUFFER_DURATION_MS: f32 = 400.0;
const EMA_ALPHA: f32 = 0.3;
const LOOP_SLEEP_MS: u64 = 30;
const GROWING_BUFFER_MAX_DURATION_S: f32 = 30.0;
const GROWING_BUFFER_TICK_MS: u64 = 1250;
const CORRECTED_CHUNK_DURATION_S: f32 = 8.0;
const CORRECTED_OVERLAP_DURATION_S: f32 = 1.5;
const CORRECTED_FINAL_TAIL_DURATION_S: f32 = 12.0;
const MAX_MERGE_OVERLAP_WORDS: usize = 18;
const MIN_MERGE_OVERLAP_WORDS: usize = 2;

#[derive(Serialize, Clone)]
pub struct StreamingTranscript {
    pub text: String,
    pub highlights: Vec<HighlightRange>,
}

#[derive(Default)]
struct StreamingTranscriptionState {
    raw_text: String,
    corrected_text: String,
    last_growing_sample_count: usize,
}

impl StreamingTranscriptionState {
    fn set_raw_text(
        &mut self,
        text: String,
        dictionary: &HashMap<String, Vec<String>>,
        cc_rules_path: &Option<PathBuf>,
    ) {
        self.raw_text = text.trim().to_string();
        self.corrected_text = correct_with_dictionary(&self.raw_text, dictionary, cc_rules_path);
    }

    fn replace_growing_transcript(
        &mut self,
        text: String,
        sample_count: usize,
        dictionary: &HashMap<String, Vec<String>>,
        cc_rules_path: &Option<PathBuf>,
    ) {
        self.set_raw_text(text, dictionary, cc_rules_path);
        self.last_growing_sample_count = sample_count;
    }

    fn append_segment(
        &mut self,
        text: String,
        dictionary: &HashMap<String, Vec<String>>,
        cc_rules_path: &Option<PathBuf>,
    ) {
        let merged = merge_transcripts(&self.raw_text, &text, true);
        self.set_raw_text(merged, dictionary, cc_rules_path);
    }

    fn replace_tail(
        &mut self,
        text: String,
        dictionary: &HashMap<String, Vec<String>>,
        cc_rules_path: &Option<PathBuf>,
    ) {
        if let Some(merged) = replace_transcript_tail(&self.raw_text, &text) {
            self.set_raw_text(merged, dictionary, cc_rules_path);
        }
    }

    fn final_raw_text(self) -> Option<String> {
        let text = self.raw_text.trim().to_string();
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

struct StreamingVadState {
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
}

impl StreamingVadState {
    fn new(sample_rate: u32) -> Self {
        let max_samples = (sample_rate as f32 * MAX_SEGMENT_DURATION_S) as usize;
        let pre_buffer_capacity = (sample_rate as f32 * PRE_BUFFER_DURATION_MS / 1000.0) as usize;
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
        }
    }

    fn process_samples(&mut self, samples: &[f32]) -> Option<Vec<f32>> {
        for &sample in samples {
            self.acc_sum_squares += sample * sample;
            self.acc_count += 1;

            if self.speech_active {
                if self.buffer.len() < self.max_samples {
                    self.buffer.push(sample);
                }
            } else {
                if self.pre_buffer.len() >= self.pre_buffer_capacity {
                    self.pre_buffer.pop_front();
                }
                self.pre_buffer.push_back(sample);
            }
        }

        if self.last_check.elapsed() < std::time::Duration::from_millis(33) {
            return self.check_max_duration();
        }
        self.last_check = std::time::Instant::now();

        if self.acc_count == 0 {
            return self.check_max_duration();
        }

        let rms = (self.acc_sum_squares / self.acc_count as f32).sqrt();
        self.acc_sum_squares = 0.0;
        self.acc_count = 0;

        self.smoothed_rms = EMA_ALPHA * rms + (1.0 - EMA_ALPHA) * self.smoothed_rms;

        if !self.speech_active {
            if self.smoothed_rms > SPEECH_THRESHOLD {
                match self.speech_start_time {
                    Some(start) => {
                        if start.elapsed()
                            >= std::time::Duration::from_millis(SPEECH_START_DELAY_MS)
                        {
                            self.speech_active = true;
                            self.silence_start_time = None;
                            self.buffer.clear();
                            self.buffer.extend(self.pre_buffer.drain(..));
                        }
                    }
                    None => {
                        self.speech_start_time = Some(std::time::Instant::now());
                    }
                }
            } else {
                self.speech_start_time = None;
            }
            None
        } else {
            if rms < SILENCE_THRESHOLD {
                match self.silence_start_time {
                    Some(start) => {
                        if start.elapsed() >= std::time::Duration::from_millis(SPEECH_END_DELAY_MS)
                        {
                            return Some(self.take_segment());
                        }
                    }
                    None => {
                        self.silence_start_time = Some(std::time::Instant::now());
                    }
                }
            } else {
                self.silence_start_time = None;
            }

            self.check_max_duration()
        }
    }

    fn take_segment(&mut self) -> Vec<f32> {
        let segment = std::mem::take(&mut self.buffer);
        self.speech_active = false;
        self.silence_start_time = None;
        self.speech_start_time = None;
        segment
    }

    fn check_max_duration(&mut self) -> Option<Vec<f32>> {
        if self.speech_active && self.buffer.len() >= self.max_samples {
            Some(self.take_segment())
        } else {
            None
        }
    }

    fn finish_segment(&mut self) -> Option<Vec<f32>> {
        if self.speech_active && !self.buffer.is_empty() {
            Some(self.take_segment())
        } else {
            None
        }
    }
}

pub fn start_streaming(app: &AppHandle, audio_state: &AudioState, sample_rate: u32) {
    let settings = crate::settings::load_settings(app);
    let strategy = TranscriptionFinalizationStrategy::from_settings_value(
        &settings.transcription_finalization_strategy,
    );
    if !settings.streaming_preview && strategy == TranscriptionFinalizationStrategy::Wav {
        return;
    }

    // Stop any previous thread before spawning, otherwise stop.store(false)
    // below would silently revive it.
    if audio_state.streaming_handle.lock().is_some() {
        warn!("start_streaming called with a streaming thread still tracked");
        discard_streaming(app, audio_state);
    }

    let formatting_settings = match formatting_rules::load(app) {
        Ok(s) => s,
        Err(e) => {
            warn!("Failed to load formatting settings for streaming: {}", e);
            formatting_rules::FormattingSettings::default()
        }
    };

    let dictionary = app.state::<Dictionary>().get();
    let cc_rules_path = get_cc_rules_path(app).ok();

    // Reset the overlay text immediately before starting a new streaming session
    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit(
            "streaming-transcript",
            &StreamingTranscript {
                text: String::new(),
                highlights: vec![],
            },
        );
    }

    let buffer = audio_state.streaming_buffer.clone();
    let stop = audio_state.streaming_stop.clone();
    let stop_strategy = audio_state.streaming_stop_strategy.clone();
    stop.store(false, Ordering::SeqCst);
    stop_strategy.store(
        TranscriptionFinalizationStrategy::Wav as u8,
        Ordering::SeqCst,
    );

    let app_handle = app.clone();

    let handle = std::thread::Builder::new()
        .name("streaming-vad".into())
        .spawn(move || {
            streaming_thread_loop(StreamingLoopParams {
                app: app_handle,
                buffer,
                stop,
                stop_strategy,
                sample_rate,
                strategy,
                formatting_settings,
                dictionary,
                cc_rules_path,
            })
        });

    match handle {
        Ok(h) => {
            *audio_state.streaming_handle.lock() = Some(h);
            debug!("Streaming thread started");
        }
        Err(e) => {
            error!("Failed to spawn streaming thread: {}", e);
        }
    }
}

struct StreamingLoopParams {
    app: AppHandle,
    buffer: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
    stop_strategy: Arc<AtomicU8>,
    sample_rate: u32,
    strategy: TranscriptionFinalizationStrategy,
    formatting_settings: formatting_rules::FormattingSettings,
    dictionary: HashMap<String, Vec<String>>,
    cc_rules_path: Option<PathBuf>,
}

fn streaming_thread_loop(params: StreamingLoopParams) -> Option<String> {
    let StreamingLoopParams {
        app,
        buffer,
        stop,
        stop_strategy,
        sample_rate,
        strategy,
        formatting_settings,
        dictionary,
        cc_rules_path,
    } = params;

    if strategy == TranscriptionFinalizationStrategy::StreamingCorrected {
        return corrected_streaming_thread_loop(CorrectedStreamingLoopParams {
            app,
            buffer,
            stop,
            stop_strategy,
            sample_rate,
            formatting_settings,
            dictionary,
            cc_rules_path,
        });
    }

    let mut vad = StreamingVadState::new(sample_rate);
    let mut transcript_state = StreamingTranscriptionState::default();

    let growing_max_samples = (sample_rate as f32 * GROWING_BUFFER_MAX_DURATION_S) as usize;
    let mut growing_audio: Vec<f32> = Vec::with_capacity(growing_max_samples);
    let mut in_growing_mode = true;
    let mut first_tick = true;
    let mut last_growing_tick = std::time::Instant::now();

    loop {
        let should_stop = stop.load(Ordering::SeqCst);
        let new_samples = drain_shared_buffer(&buffer);

        if new_samples.is_empty() {
            if should_stop {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
            continue;
        }

        if in_growing_mode {
            growing_audio.extend_from_slice(&new_samples);

            // Switch to VAD mode when we exceed the growing buffer limit
            if growing_audio.len() >= growing_max_samples {
                in_growing_mode = false;
                debug!(
                    "Streaming: switching from growing buffer to VAD mode after {}s",
                    GROWING_BUFFER_MAX_DURATION_S
                );
                if let Some(text) = transcribe_samples(&app, &growing_audio, sample_rate) {
                    transcript_state.replace_growing_transcript(
                        text,
                        growing_audio.len(),
                        &dictionary,
                        &cc_rules_path,
                    );
                    emit_transcript(
                        &app,
                        &transcript_state.corrected_text,
                        &transcript_state.raw_text,
                        &formatting_settings,
                    );
                }
                growing_audio = Vec::with_capacity(growing_max_samples);
            } else if last_growing_tick.elapsed()
                >= std::time::Duration::from_millis(if first_tick {
                    900
                } else {
                    GROWING_BUFFER_TICK_MS
                })
            {
                last_growing_tick = std::time::Instant::now();
                first_tick = false;

                if let Some(text) = transcribe_samples(&app, &growing_audio, sample_rate) {
                    transcript_state.replace_growing_transcript(
                        text,
                        growing_audio.len(),
                        &dictionary,
                        &cc_rules_path,
                    );
                    emit_transcript(
                        &app,
                        &transcript_state.corrected_text,
                        &transcript_state.raw_text,
                        &formatting_settings,
                    );
                }
            }
        } else {
            // VAD mode: feed samples and wait for segments
            if let Some(segment) = vad.process_samples(&new_samples) {
                if !segment.is_empty() {
                    if let Some(text) = transcribe_samples(&app, &segment, sample_rate) {
                        transcript_state.append_segment(text, &dictionary, &cc_rules_path);
                        emit_transcript(
                            &app,
                            &transcript_state.corrected_text,
                            &transcript_state.raw_text,
                            &formatting_settings,
                        );
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }

    if TranscriptionFinalizationStrategy::from(stop_strategy.load(Ordering::SeqCst))
        == TranscriptionFinalizationStrategy::Wav
    {
        return None;
    }

    if in_growing_mode {
        if !growing_audio.is_empty()
            && growing_audio.len() != transcript_state.last_growing_sample_count
        {
            if let Some(text) = transcribe_samples(&app, &growing_audio, sample_rate) {
                transcript_state.replace_growing_transcript(
                    text,
                    growing_audio.len(),
                    &dictionary,
                    &cc_rules_path,
                );
            }
        }
    } else if let Some(segment) = vad.finish_segment() {
        if !segment.is_empty() {
            if let Some(text) = transcribe_samples(&app, &segment, sample_rate) {
                transcript_state.append_segment(text, &dictionary, &cc_rules_path);
            }
        }
    }

    transcript_state.final_raw_text()
}

struct CorrectedStreamingLoopParams {
    app: AppHandle,
    buffer: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
    stop_strategy: Arc<AtomicU8>,
    sample_rate: u32,
    formatting_settings: formatting_rules::FormattingSettings,
    dictionary: HashMap<String, Vec<String>>,
    cc_rules_path: Option<PathBuf>,
}

fn corrected_streaming_thread_loop(params: CorrectedStreamingLoopParams) -> Option<String> {
    let CorrectedStreamingLoopParams {
        app,
        buffer,
        stop,
        stop_strategy,
        sample_rate,
        formatting_settings,
        dictionary,
        cc_rules_path,
    } = params;

    let chunk_samples = (sample_rate as f32 * CORRECTED_CHUNK_DURATION_S) as usize;
    let overlap_samples = (sample_rate as f32 * CORRECTED_OVERLAP_DURATION_S) as usize;
    let tail_samples = (sample_rate as f32 * CORRECTED_FINAL_TAIL_DURATION_S) as usize;

    let mut transcript_state = StreamingTranscriptionState::default();
    let mut chunk_audio: Vec<f32> = Vec::with_capacity(chunk_samples + overlap_samples);
    let mut tail_audio: VecDeque<f32> = VecDeque::with_capacity(tail_samples);

    loop {
        let should_stop = stop.load(Ordering::SeqCst);
        let new_samples = drain_shared_buffer(&buffer);

        if new_samples.is_empty() {
            if should_stop {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
            continue;
        }

        append_tail_samples(&mut tail_audio, &new_samples, tail_samples);
        chunk_audio.extend_from_slice(&new_samples);

        if chunk_audio.len() >= chunk_samples {
            if let Some(text) = transcribe_samples(&app, &chunk_audio, sample_rate) {
                transcript_state.append_segment(text, &dictionary, &cc_rules_path);
                emit_transcript(
                    &app,
                    &transcript_state.corrected_text,
                    &transcript_state.raw_text,
                    &formatting_settings,
                );
            }

            let keep_from = chunk_audio.len().saturating_sub(overlap_samples);
            chunk_audio = chunk_audio[keep_from..].to_vec();
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }

    let final_strategy =
        TranscriptionFinalizationStrategy::from(stop_strategy.load(Ordering::SeqCst));
    if final_strategy == TranscriptionFinalizationStrategy::Wav {
        return None;
    }

    if final_strategy == TranscriptionFinalizationStrategy::StreamingCorrected {
        let tail: Vec<f32> = tail_audio.into_iter().collect();
        if !tail.is_empty() {
            if let Some(text) = transcribe_samples(&app, &tail, sample_rate) {
                transcript_state.replace_tail(text, &dictionary, &cc_rules_path);
            }
        }
    } else if chunk_audio.len() > overlap_samples {
        if let Some(text) = transcribe_samples(&app, &chunk_audio, sample_rate) {
            transcript_state.append_segment(text, &dictionary, &cc_rules_path);
        }
    }

    transcript_state.final_raw_text()
}

fn append_tail_samples(tail_audio: &mut VecDeque<f32>, samples: &[f32], capacity: usize) {
    if capacity == 0 {
        return;
    }

    for &sample in samples {
        if tail_audio.len() >= capacity {
            tail_audio.pop_front();
        }
        tail_audio.push_back(sample);
    }
}

fn merge_transcripts(existing: &str, next: &str, append_on_no_overlap: bool) -> String {
    let existing_words: Vec<&str> = existing.split_whitespace().collect();
    let next_words: Vec<&str> = next.split_whitespace().collect();

    if existing_words.is_empty() {
        return next.trim().to_string();
    }
    if next_words.is_empty() {
        return existing.trim().to_string();
    }

    if let Some(overlap) = best_overlap(&existing_words, &next_words) {
        let mut merged = existing_words[..existing_words.len() - overlap].to_vec();
        merged.extend(next_words);
        return merged.join(" ");
    }

    if append_on_no_overlap {
        format!("{} {}", existing.trim(), next.trim())
    } else {
        existing.trim().to_string()
    }
}

fn replace_transcript_tail(existing: &str, tail: &str) -> Option<String> {
    let existing_words: Vec<&str> = existing.split_whitespace().collect();
    let tail_words: Vec<&str> = tail.split_whitespace().collect();

    if tail_words.is_empty() {
        return None;
    }
    if existing_words.is_empty() {
        return Some(tail.trim().to_string());
    }

    best_overlap(&existing_words, &tail_words).map(|overlap| {
        let mut merged = existing_words[..existing_words.len() - overlap].to_vec();
        merged.extend(tail_words);
        merged.join(" ")
    })
}

fn best_overlap(existing_words: &[&str], next_words: &[&str]) -> Option<usize> {
    let max_overlap = MAX_MERGE_OVERLAP_WORDS
        .min(existing_words.len())
        .min(next_words.len());

    for overlap in (MIN_MERGE_OVERLAP_WORDS..=max_overlap).rev() {
        let existing_tail = &existing_words[existing_words.len() - overlap..];
        let next_head = &next_words[..overlap];
        if token_windows_match(existing_tail, next_head) {
            return Some(overlap);
        }
    }

    None
}

fn token_windows_match(left: &[&str], right: &[&str]) -> bool {
    if left.len() != right.len() || left.is_empty() {
        return false;
    }

    let matches = left
        .iter()
        .zip(right.iter())
        .filter(|(l, r)| tokens_match(l, r))
        .count();
    matches * 4 >= left.len() * 3
}

fn tokens_match(left: &str, right: &str) -> bool {
    let left = normalize_token(left);
    let right = normalize_token(right);

    if left.is_empty() || right.is_empty() {
        return false;
    }
    if left == right {
        return true;
    }

    let max_distance = if left.len().min(right.len()) <= 4 {
        1
    } else {
        2
    };
    strsim::levenshtein(&left, &right) <= max_distance
}

fn normalize_token(token: &str) -> String {
    token
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn correct_with_dictionary(
    text: &str,
    dictionary: &HashMap<String, Vec<String>>,
    cc_rules_path: &Option<PathBuf>,
) -> String {
    match cc_rules_path {
        Some(path) if !dictionary.is_empty() => {
            fix_transcription_with_dictionary(text.to_string(), dictionary, path)
        }
        _ => text.to_string(),
    }
}

fn drain_shared_buffer(buffer: &Arc<Mutex<Vec<f32>>>) -> Vec<f32> {
    std::mem::take(&mut *buffer.lock())
}

fn transcribe_samples(app: &AppHandle, samples: &[f32], sample_rate: u32) -> Option<String> {
    let resampled = if sample_rate != 16000 {
        resample_linear(samples, sample_rate as usize, 16000)
    } else {
        samples.to_vec()
    };

    let state = app.state::<AudioState>();
    let mut engine_guard = state.engine.lock();
    match engine_guard.as_mut() {
        Some(e) => match e.transcribe_samples(resampled, None) {
            Ok(result) => {
                let trimmed = result.text.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            }
            Err(e) => {
                debug!("Streaming transcription error: {}", e);
                None
            }
        },
        None => {
            debug!("Engine not loaded for streaming transcription");
            None
        }
    }
}

fn emit_transcript(
    app: &AppHandle,
    text: &str,
    original_text: &str,
    formatting_settings: &formatting_rules::FormattingSettings,
) {
    let formatted = apply_formatting_with_highlights_and_original(
        text.to_string(),
        original_text.to_string(),
        formatting_settings,
    );

    let payload = StreamingTranscript {
        text: formatted.text,
        highlights: formatted.highlights,
    };

    debug!("Streaming transcript emitted");

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit("streaming-transcript", &payload);
    }
}

pub fn stop_streaming(
    app: &AppHandle,
    audio_state: &AudioState,
    strategy: TranscriptionFinalizationStrategy,
) -> Option<String> {
    audio_state
        .streaming_stop_strategy
        .store(strategy as u8, Ordering::SeqCst);
    stop_streaming_thread(app, audio_state)
}

pub fn discard_streaming(app: &AppHandle, audio_state: &AudioState) {
    audio_state.streaming_stop_strategy.store(
        TranscriptionFinalizationStrategy::Wav as u8,
        Ordering::SeqCst,
    );
    let _ = stop_streaming_thread(app, audio_state);
}

fn stop_streaming_thread(app: &AppHandle, audio_state: &AudioState) -> Option<String> {
    audio_state.streaming_stop.store(true, Ordering::SeqCst);

    let handle = audio_state.streaming_handle.lock().take();
    let mut final_transcript = None;
    if let Some(h) = handle {
        match h.join() {
            Ok(transcript) => {
                final_transcript = transcript;
                debug!("Streaming thread joined");
            }
            Err(_) => {
                warn!("Streaming thread panicked during shutdown");
            }
        }
    }

    audio_state.streaming_buffer.lock().clear();

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit(
            "streaming-transcript",
            &StreamingTranscript {
                text: String::new(),
                highlights: vec![],
            },
        );
    }

    final_transcript
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_transcript_serialization() {
        let t = StreamingTranscript {
            text: "Bonjour, je voudrais réserver".to_string(),
            highlights: vec![],
        };
        let json = serde_json::to_string(&t).expect("serialize");
        assert!(json.contains("Bonjour"));
    }

    #[test]
    fn streaming_transcript_with_highlights() {
        let t = StreamingTranscript {
            text: "Bonjour Monsieur Dupont".to_string(),
            highlights: vec![HighlightRange { start: 8, end: 23 }],
        };
        let json = serde_json::to_string(&t).expect("serialize");
        assert!(json.contains("\"start\":8"));
        assert!(json.contains("\"end\":23"));
    }

    #[test]
    fn transcription_state_replaces_growing_text() {
        let mut state = StreamingTranscriptionState::default();
        let dictionary = HashMap::new();
        let cc_rules_path = None;

        state.replace_growing_transcript("bonjour".to_string(), 1_600, &dictionary, &cc_rules_path);
        state.replace_growing_transcript(
            "bonjour le monde".to_string(),
            3_200,
            &dictionary,
            &cc_rules_path,
        );

        assert_eq!(state.raw_text, "bonjour le monde");
        assert_eq!(state.corrected_text, "bonjour le monde");
        assert_eq!(state.last_growing_sample_count, 3_200);
    }

    #[test]
    fn transcription_state_appends_segment_text() {
        let mut state = StreamingTranscriptionState::default();
        let dictionary = HashMap::new();
        let cc_rules_path = None;

        state.append_segment("bonjour".to_string(), &dictionary, &cc_rules_path);
        state.append_segment("le monde".to_string(), &dictionary, &cc_rules_path);

        assert_eq!(state.final_raw_text(), Some("bonjour le monde".to_string()));
    }

    #[test]
    fn merge_transcripts_removes_boundary_overlap() {
        let merged = merge_transcripts(
            "je voudrais prendre rendez vous demain",
            "rendez vous demain matin a neuf heures",
            true,
        );

        assert_eq!(
            merged,
            "je voudrais prendre rendez vous demain matin a neuf heures"
        );
    }

    #[test]
    fn merge_transcripts_appends_when_overlap_is_unclear() {
        let merged = merge_transcripts("bonjour docteur", "je voudrais un rendez vous", true);

        assert_eq!(merged, "bonjour docteur je voudrais un rendez vous");
    }

    #[test]
    fn replace_tail_uses_clear_overlap() {
        let replaced = replace_transcript_tail(
            "je voudrais prendre rendez vous demain matin",
            "demain matin a neuf heures",
        );

        assert_eq!(
            replaced,
            Some("je voudrais prendre rendez vous demain matin a neuf heures".to_string())
        );
    }

    #[test]
    fn replace_tail_keeps_existing_when_overlap_is_unclear() {
        let replaced = replace_transcript_tail(
            "je voudrais prendre rendez vous",
            "le patient arrive demain",
        );

        assert!(replaced.is_none());
    }

    #[test]
    fn vad_state_initial() {
        let vad = StreamingVadState::new(16000);
        assert!(!vad.speech_active);
        assert_eq!(vad.max_samples, (16000.0 * MAX_SEGMENT_DURATION_S) as usize);
    }

    #[test]
    fn vad_state_silence_returns_none() {
        let mut vad = StreamingVadState::new(16000);
        let silence = vec![0.0f32; 1600];
        let result = vad.process_samples(&silence);
        assert!(result.is_none());
    }

    #[test]
    fn vad_finish_returns_active_segment() {
        let mut vad = StreamingVadState::new(16000);
        vad.speech_active = true;
        vad.buffer.extend_from_slice(&[0.2, 0.1, 0.0]);

        let segment = vad.finish_segment();

        assert_eq!(segment, Some(vec![0.2, 0.1, 0.0]));
        assert!(!vad.speech_active);
    }
}
