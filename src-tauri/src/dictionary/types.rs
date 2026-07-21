use std::sync::{Arc, Mutex};

pub struct Dictionary(pub Arc<Mutex<Vec<String>>>);

impl Dictionary {
    pub fn new(words: Vec<String>) -> Self {
        Self(Arc::new(Mutex::new(words)))
    }
    pub fn get(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, words: Vec<String>) {
        *self.0.lock().unwrap() = words;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DictionaryError {
    #[error("Invalid word format: {0}. Words must not contain digits and may contain at most one space")]
    InvalidWordFormat(String),
    #[error("Dictionary import must contain at least one valid word")]
    EmptyDictionary,
}
