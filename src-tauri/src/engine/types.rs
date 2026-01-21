use ndarray::Array3;
use ort::session::Session;
use serde::{Deserialize, Serialize};

pub type DecoderState = (Array3<f32>, Array3<f32>);



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


