use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Dictionary(pub Arc<Mutex<HashMap<String, Vec<String>>>>);

impl Dictionary {
    pub fn new(dictionary: HashMap<String, Vec<String>>) -> Self {
        Self(Arc::new(Mutex::new(dictionary)))
    }
    pub fn get(&self) -> HashMap<String, Vec<String>> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, dictionary: HashMap<String, Vec<String>>) {
        *self.0.lock().unwrap() = dictionary;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DictionaryError {
    #[error("Invalid word format: {0}. Words must contain only letters (a-z, A-Z)")]
    InvalidWordFormat(String),
    #[error("Dictionary import must contain at least one valid word")]
    EmptyDictionary,
}
