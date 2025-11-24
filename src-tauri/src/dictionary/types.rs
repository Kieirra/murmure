use std::sync::{Arc, Mutex};

pub struct Dictionary(pub Arc<Mutex<Vec<String>>>);

impl Dictionary {
    pub fn new(dictionary: Vec<String>) -> Self {
        Self(Arc::new(Mutex::new(dictionary)))
    }
    pub fn get(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
    pub fn set(&self, dictionary: Vec<String>) {
        *self.0.lock().unwrap() = dictionary;
    }
}
