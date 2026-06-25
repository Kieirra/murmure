use crate::audio::clean_recording::strip_fillers_and_repeats;
use crate::audio::helpers::resample;
use crate::audio::types::{AudioState, PreviewSnapshot};
use crate::dictionary::{correct_transcription, sync_boost_words, Dictionary};
use crate::engine::transcription_engine::TranscriptionEngine;
use crate::formatting_rules;
use crate::formatting_rules::highlighter::{
    apply_formatting_with_highlights_and_original, HighlightRange,
};
use log::{debug, error, warn};
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

const LOOP_SLEEP_MS: u64 = 30;

#[derive(Serialize, Clone)]
pub struct PreviewProvisional {
    pub seq: u64,
    pub text: String,
    pub highlights: Vec<HighlightRange>,
}

pub fn start_streaming(app: &AppHandle, audio_state: &AudioState, sample_rate: u32) {
    let settings = crate::settings::load_settings(app);
    if !settings.streaming_preview {
        return;
    }

    // Stop any previous thread before spawning, otherwise stop.store(false)
    // below would silently revive it.
    if audio_state.streaming_handle.lock().is_some() {
        warn!("start_streaming called with a streaming thread still tracked");
        stop_streaming(app, audio_state);
    }

    let formatting_settings = match formatting_rules::load(app) {
        Ok(s) => s,
        Err(e) => {
            warn!("Failed to load formatting settings for streaming: {}", e);
            formatting_rules::FormattingSettings::default()
        }
    };

    let dictionary = app.state::<Dictionary>().get();

    reset_overlay_preview(app);

    {
        let mut snapshot = audio_state.preview_snapshot.lock();
        *snapshot = PreviewSnapshot::default();
    }

    let snapshot = audio_state.preview_snapshot.clone();
    let inference_active = audio_state.chunk_inference_active.clone();
    let stop = audio_state.streaming_stop.clone();
    stop.store(false, Ordering::SeqCst);

    let app_handle = app.clone();

    let handle = std::thread::Builder::new()
        .name("streaming-preview".into())
        .spawn(move || {
            streaming_thread_loop(StreamingLoopParams {
                app: app_handle,
                snapshot,
                inference_active,
                stop,
                sample_rate,
                formatting_settings,
                dictionary,
            });
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
    snapshot: Arc<Mutex<PreviewSnapshot>>,
    inference_active: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    sample_rate: u32,
    formatting_settings: formatting_rules::FormattingSettings,
    dictionary: Vec<String>,
}

fn streaming_thread_loop(params: StreamingLoopParams) {
    let StreamingLoopParams {
        app,
        snapshot,
        inference_active,
        stop,
        sample_rate,
        formatting_settings,
        dictionary,
    } = params;

    let mut last_revision: u64 = 0;

    while !stop.load(Ordering::SeqCst) {
        let pending = {
            let snap = snapshot.lock();
            if snap.revision != last_revision && !inference_active.load(Ordering::SeqCst) {
                Some((snap.queue.clone(), snap.generation, snap.revision))
            } else {
                None
            }
        };

        if let Some((queue, generation, revision)) = pending {
            last_revision = revision;
            if let Some((text, corrected)) =
                transcribe_samples(&app, &queue, sample_rate, &dictionary)
            {
                emit_provisional(&app, generation, &corrected, &text, &formatting_settings);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }
}

/// Transcribe a chunk and return it raw and dictionary-corrected.
fn transcribe_samples(
    app: &AppHandle,
    samples: &[f32],
    sample_rate: u32,
    dictionary: &[String],
) -> Option<(String, String)> {
    let resampled = if sample_rate != 16000 {
        resample(samples, sample_rate as usize, 16000)
    } else {
        samples.to_vec()
    };

    let state = app.state::<AudioState>();
    let mut engine_guard = state.engine.lock();
    let Some(engine) = engine_guard.as_mut() else {
        debug!("Engine not loaded for streaming transcription");
        return None;
    };
    sync_boost_words(engine, dictionary);
    match engine.transcribe_samples(resampled, None) {
        Ok(result) => {
            let cleaned = strip_fillers_and_repeats(result.text.trim());
            if cleaned.is_empty() {
                None
            } else {
                let corrected =
                    correct_transcription(&cleaned, dictionary, &result.word_confidences);
                Some((cleaned, corrected))
            }
        }
        Err(e) => {
            debug!("Streaming transcription error: {}", e);
            None
        }
    }
}

fn emit_provisional(
    app: &AppHandle,
    seq: u64,
    text: &str,
    original_text: &str,
    formatting_settings: &formatting_rules::FormattingSettings,
) {
    let formatted = apply_formatting_with_highlights_and_original(
        text.to_string(),
        original_text.to_string(),
        formatting_settings,
    );

    let payload = PreviewProvisional {
        seq,
        text: formatted.text,
        highlights: formatted.highlights,
    };

    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit("preview-provisional", &payload);
    }
}

fn reset_overlay_preview(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("recording_overlay") {
        let _ = window.emit(
            "preview-provisional",
            &PreviewProvisional {
                seq: 0,
                text: String::new(),
                highlights: vec![],
            },
        );
    }
}

pub fn stop_streaming(app: &AppHandle, audio_state: &AudioState) {
    audio_state.streaming_stop.store(true, Ordering::SeqCst);

    let handle = audio_state.streaming_handle.lock().take();
    if let Some(h) = handle {
        let _ = h.join();
        debug!("Streaming thread joined");
    }

    *audio_state.preview_snapshot.lock() = PreviewSnapshot::default();
    audio_state
        .chunk_inference_active
        .store(false, Ordering::SeqCst);

    reset_overlay_preview(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_provisional_serialization() {
        let t = PreviewProvisional {
            seq: 0,
            text: "Bonjour, je voudrais réserver".to_string(),
            highlights: vec![],
        };
        let json = serde_json::to_string(&t).expect("serialize");
        assert!(json.contains("Bonjour"));
        assert!(json.contains("\"seq\":0"));
    }

    #[test]
    fn preview_provisional_with_highlights() {
        let t = PreviewProvisional {
            seq: 3,
            text: "Bonjour Monsieur Dupont".to_string(),
            highlights: vec![HighlightRange { start: 8, end: 23 }],
        };
        let json = serde_json::to_string(&t).expect("serialize");
        assert!(json.contains("\"start\":8"));
        assert!(json.contains("\"end\":23"));
        assert!(json.contains("\"seq\":3"));
    }
}
