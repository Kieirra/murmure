// Minimal transcription API types

use crate::audio::read_wav_samples;
use std::path::Path;

/// The result of a transcription operation.
///
/// Contains both the full transcribed text and detailed timing information
/// for individual segments within the audio.
#[derive(Debug)]
#[allow(dead_code)] // segments will be useful for UI timestamps
pub struct TranscriptionResult {
    /// The complete transcribed text from the audio
    pub text: String,
    /// Individual segments with timing information
    pub segments: Vec<TranscriptionSegment>,
}

/// A single transcribed segment with timing information.
///
/// Represents a portion of the transcribed audio with start and end timestamps
/// and the corresponding text content.
#[derive(Debug)]
#[allow(dead_code)] // fields will be useful for UI timestamps
pub struct TranscriptionSegment {
    /// Start time of the segment in seconds
    pub start: f32,
    /// End time of the segment in seconds
    pub end: f32,
    /// The transcribed text for this segment
    pub text: String,
}

/// Common interface for speech transcription engines.
///
/// This trait defines the standard operations that all transcription engines must support.
/// Each engine may have different parameter types for model loading and inference configuration.
///
/// # Examples
///
/// ## Using Whisper Engine
///
/// ```rust,no_run
/// use transcribe_rs::{TranscriptionEngine, engines::whisper::WhisperEngine};
/// use std::path::PathBuf;
///
/// let mut engine = WhisperEngine::new();
/// engine.load_model(&PathBuf::from("models/whisper-medium-q4_1.bin"))?;
///
/// let result = engine.transcribe_file(&PathBuf::from("audio.wav"), None)?;
/// println!("Transcription: {}", result.text);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Using Parakeet Engine
///
/// ```rust,no_run
/// use transcribe_rs::{TranscriptionEngine, engines::parakeet::{ParakeetEngine, ParakeetModelParams}};
/// use std::path::PathBuf;
///
/// let mut engine = ParakeetEngine::new();
/// engine.load_model_with_params(
///     &PathBuf::from("models/parakeet-v0.3"),
///     ParakeetModelParams::int8()
/// )?;
///
/// let result = engine.transcribe_file(&PathBuf::from("audio.wav"), None)?;
/// println!("Transcription: {}", result.text);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait TranscriptionEngine {
    /// Parameters for configuring inference behavior (language, timestamps, etc.)
    type InferenceParams;
    /// Parameters for configuring model loading (quantization, etc.)
    type ModelParams: Default;

    /// Load a model from the specified path with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `model_path` - Path to the model file or directory
    /// * `params` - Engine-specific model loading parameters
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the model loads successfully, or an error if loading fails.
    fn load_model_with_params(
        &mut self,
        model_path: &Path,
        params: Self::ModelParams,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Unload the currently loaded model and free associated resources.
    fn unload_model(&mut self);

    /// Transcribe audio samples directly.
    ///
    /// # Arguments
    ///
    /// * `samples` - Audio samples as f32 values (16kHz, mono)
    /// * `params` - Optional engine-specific inference parameters
    ///
    /// # Returns
    ///
    /// Returns transcription result with text and timing information.
    fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
        params: Option<Self::InferenceParams>,
    ) -> Result<TranscriptionResult, Box<dyn std::error::Error>>;

    /// Transcribe audio from a WAV file.
    ///
    /// The WAV file must meet the following requirements:
    /// - 16 kHz sample rate
    /// - 16-bit samples
    /// - Mono (single channel)
    /// - PCM format
    ///
    /// # Arguments
    ///
    /// * `wav_path` - Path to the WAV file to transcribe
    /// * `params` - Optional engine-specific inference parameters
    ///
    /// # Returns
    ///
    /// Returns transcription result with text and timing information.
    #[allow(dead_code)]
    fn transcribe_file(
        &mut self,
        wav_path: &Path,
        params: Option<Self::InferenceParams>,
    ) -> Result<TranscriptionResult, Box<dyn std::error::Error>> {
        let samples = read_wav_samples(wav_path)?;
        self.transcribe_samples(samples, params)
    }
}
