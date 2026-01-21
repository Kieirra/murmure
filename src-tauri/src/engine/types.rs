use ndarray::Array3;
use ort::session::Session;
use serde::{Deserialize, Serialize};

pub type DecoderState = (Array3<f32>, Array3<f32>);

/// Configuration for mel spectrogram extraction
#[derive(Debug, Clone)]
pub struct MelConfig {
    pub sample_rate: usize,
    pub n_fft: usize,
    pub hop_length: usize,
    pub win_length: usize,
    pub n_mels: usize,
}

impl Default for MelConfig {
    fn default() -> Self {
        // MedASR default parameters from processor_config.json
        Self {
            sample_rate: 16000,
            n_fft: 512,
            hop_length: 160,
            win_length: 400,
            n_mels: 128,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimestampedResult {
    pub text: String,
    pub timestamps: Vec<f32>,
    pub tokens: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParakeetError {
    #[error("ORT error")]
    Ort(#[from] ort::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("ndarray shape error")]
    Shape(#[from] ndarray::ShapeError),
    #[error("Model input not found: {0}")]
    InputNotFound(String),
    #[error("Model output not found: {0}")]
    OutputNotFound(String),
    #[error("Failed to get tensor shape for input: {0}")]
    TensorShape(String),
}

pub struct ParakeetModel {
    pub encoder: Session,
    pub decoder_joint: Session,
    pub preprocessor: Session,
    pub vocab: Vec<String>,
    pub blank_idx: i32,
    pub vocab_size: usize,
}

pub struct MedAsrModel {
    pub session: Session,
    pub vocab: Vec<String>,
    pub blank_idx: i32,
    pub mel_config: MelConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub text: String,
    pub token_id: Option<usize>,
    pub t_start: f32,
    pub t_end: f32,
    pub is_blank: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub t_start: f32,
    pub t_end: f32,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub text: String,
    pub t_start: f32,
    pub t_end: f32,
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Utterance {
    pub text: String,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimestampGranularity {
    Token,
    Word,
    Segment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantizationType {
    FP32,
    Int8,
}
/// Parameters for configuring Parakeet model loading.
#[derive(Debug, Clone)]
pub struct ParakeetModelParams {
    pub quantization: QuantizationType,
}

impl Default for ParakeetModelParams {
    fn default() -> Self {
        Self {
            quantization: QuantizationType::FP32,
        }
    }
}

impl ParakeetModelParams {
    pub fn int8() -> Self {
        Self {
            quantization: QuantizationType::Int8,
        }
    }
}

/// Parameters for configuring Parakeet inference behavior.
#[derive(Debug, Clone)]
pub struct ParakeetInferenceParams {
    pub timestamp_granularity: TimestampGranularity,
}

impl Default for ParakeetInferenceParams {
    fn default() -> Self {
        Self {
            timestamp_granularity: TimestampGranularity::Token,
        }
    }
}

/// Parakeet speech recognition engine wrapper.
pub struct ParakeetEngine {
    pub model: Option<ParakeetModel>,
    pub loaded_model_path: Option<std::path::PathBuf>,
}

impl ParakeetEngine {
    pub fn new() -> Self {
        Self {
            model: None,
            loaded_model_path: None,
        }
    }
}

/// Parameters for configuring MedAsr model loading.
#[derive(Debug, Clone, Default)]
pub struct MedAsrModelParams {
    // Add any specific parameters here if needed
}

/// Parameters for configuring MedAsr inference behavior.
#[derive(Debug, Clone)]
pub struct MedAsrInferenceParams {
    pub timestamp_granularity: TimestampGranularity,
    /// Beam width for CTC beam search decoding. Use 1 for greedy.
    pub beam_width: usize,
}

impl Default for MedAsrInferenceParams {
    fn default() -> Self {
        Self {
            timestamp_granularity: TimestampGranularity::Token,
            beam_width: 8,  // Default to beam search with width 8
        }
    }
}

/// MedAsr speech recognition engine wrapper.
pub struct MedAsrEngine {
    pub model: Option<MedAsrModel>,
    pub loaded_model_path: Option<std::path::PathBuf>,
}

impl MedAsrEngine {
    pub fn new() -> Self {
        Self {
            model: None,
            loaded_model_path: None,
        }
    }
}
