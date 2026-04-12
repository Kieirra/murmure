use crate::audio::helpers::resample_linear;
use crate::audio::types::AudioState;
use crate::dictionary::{fix_transcription_with_dictionary, get_cc_rules_path, Dictionary};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::formatting_rules;
use crate::formatting_rules::highlighter::{
    apply_formatting_with_highlights_and_original, HighlightRange,
};
use crate::overlay::overlay;
use log::{debug, error, warn};
use serde::Serialize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

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
const MONO_FONT_WIDTH_RATIO: f32 = 0.6;

#[derive(Serialize, Clone)]
pub struct StreamingTranscript {
    pub text: String,
    pub highlights: Vec<HighlightRange>,
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
        let pre_buffer_capacity =
            (sample_rate as f32 * PRE_BUFFER_DURATION_MS / 1000.0) as usize;
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
                        if start.elapsed()
                            >= std::time::Duration::from_millis(SPEECH_END_DELAY_MS)
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
}

pub fn start_streaming(
    app: &AppHandle,
    audio_state: &AudioState,
    sample_rate: u32,
) {
    let settings = crate::settings::load_settings(app);
    if !settings.streaming_preview {
        return;
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
        let _ = window.emit("streaming-transcript", &StreamingTranscript {
            text: String::new(),
            highlights: vec![],
        });
    }

    let chars_per_line = (settings.streaming_text_width as f32 / (settings.streaming_font_size as f32 * MONO_FONT_WIDTH_RATIO)) as usize;
    let max_lines = settings.streaming_max_lines;

    let buffer = audio_state.streaming_buffer.clone();
    let stop = audio_state.streaming_stop.clone();
    stop.store(false, Ordering::SeqCst);

    let app_handle = app.clone();

    let handle = std::thread::Builder::new()
        .name("streaming-vad".into())
        .spawn(move || {
            streaming_thread_loop(
                app_handle,
                buffer,
                stop,
                sample_rate,
                formatting_settings,
                dictionary,
                cc_rules_path,
                chars_per_line,
                max_lines,
            );
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

fn streaming_thread_loop(
    app: AppHandle,
    buffer: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
    sample_rate: u32,
    formatting_settings: formatting_rules::FormattingSettings,
    dictionary: HashMap<String, Vec<String>>,
    cc_rules_path: Option<PathBuf>,
    chars_per_line: usize,
    max_lines: u32,
) {
    let mut vad = StreamingVadState::new(sample_rate);
    let mut accumulated_text = String::new();
    let mut accumulated_original = String::new();

    let growing_max_samples = (sample_rate as f32 * GROWING_BUFFER_MAX_DURATION_S) as usize;
    let mut growing_audio: Vec<f32> = Vec::with_capacity(growing_max_samples);
    let mut in_growing_mode = true;
    let mut last_growing_tick = std::time::Instant::now();

    while !stop.load(Ordering::SeqCst) {
        let new_samples = drain_shared_buffer(&buffer);

        if new_samples.is_empty() {
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
                    accumulated_original = text.clone();
                    accumulated_text = correct_with_dictionary(&text, &dictionary, &cc_rules_path);
                    emit_transcript(&app, &accumulated_text, &accumulated_original, &formatting_settings, chars_per_line, max_lines);
                }
                growing_audio = Vec::with_capacity(growing_max_samples);
            } else if last_growing_tick.elapsed()
                >= std::time::Duration::from_millis(GROWING_BUFFER_TICK_MS)
            {
                last_growing_tick = std::time::Instant::now();

                if let Some(text) = transcribe_samples(&app, &growing_audio, sample_rate) {
                    accumulated_original = text.clone();
                    accumulated_text = correct_with_dictionary(&text, &dictionary, &cc_rules_path);
                    emit_transcript(&app, &accumulated_text, &accumulated_original, &formatting_settings, chars_per_line, max_lines);
                }
            }
        } else {
            // VAD mode: feed samples and wait for segments
            if let Some(segment) = vad.process_samples(&new_samples) {
                if !segment.is_empty() {
                    if let Some(text) = transcribe_samples(&app, &segment, sample_rate) {
                        if !accumulated_original.is_empty() {
                            accumulated_original.push(' ');
                        }
                        accumulated_original.push_str(&text);
                        let corrected = correct_with_dictionary(&text, &dictionary, &cc_rules_path);
                        if !accumulated_text.is_empty() {
                            accumulated_text.push(' ');
                        }
                        accumulated_text.push_str(&corrected);
                        emit_transcript(&app, &accumulated_text, &accumulated_original, &formatting_settings, chars_per_line, max_lines);
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }
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
    chars_per_line: usize,
    max_lines: u32,
) {
    let formatted = apply_formatting_with_highlights_and_original(
        text.to_string(),
        original_text.to_string(),
        formatting_settings,
    );

    let payload = StreamingTranscript {
        text: formatted.text.clone(),
        highlights: formatted.highlights,
    };

    debug!("Streaming transcript emitted");

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit("streaming-transcript", &payload);
    }

    let line_count = estimate_line_count(&formatted.text, chars_per_line, max_lines);
    overlay::resize_overlay_for_streaming(app, line_count);
}

fn estimate_line_count(text: &str, chars_per_line: usize, max_lines: u32) -> u32 {
    let cpl = chars_per_line.max(1);
    let char_count = text.chars().count();
    let lines = char_count.div_ceil(cpl);
    (lines as u32).clamp(1, max_lines)
}

pub fn stop_streaming(app: &AppHandle, audio_state: &AudioState) {
    audio_state.streaming_stop.store(true, Ordering::SeqCst);

    let handle = audio_state.streaming_handle.lock().take();
    if let Some(h) = handle {
        let _ = h.join();
        debug!("Streaming thread joined");
    }

    audio_state.streaming_buffer.lock().clear();

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit("streaming-transcript", &StreamingTranscript {
            text: String::new(),
            highlights: vec![],
        });
    }

    overlay::reset_overlay_size(app);
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
    fn estimate_line_count_short() {
        assert_eq!(estimate_line_count("hello", 45, 4), 1);
    }

    #[test]
    fn estimate_line_count_long() {
        let text = "a".repeat(100);
        assert_eq!(estimate_line_count(&text, 45, 4), 3);
    }

    #[test]
    fn estimate_line_count_capped() {
        let text = "a".repeat(500);
        assert_eq!(estimate_line_count(&text, 45, 4), 4);
    }

    #[test]
    fn estimate_line_count_custom_settings() {
        let text = "a".repeat(100);
        // text_width=350, font_size=12 => chars_per_line = 350 / (12 * 0.6) = 48
        assert_eq!(estimate_line_count(&text, 48, 8), 3);
    }

    #[test]
    fn estimate_line_count_respects_max_lines() {
        let text = "a".repeat(500);
        assert_eq!(estimate_line_count(&text, 45, 2), 2);
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
}
