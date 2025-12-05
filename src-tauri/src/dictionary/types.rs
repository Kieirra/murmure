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
