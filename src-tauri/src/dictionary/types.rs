use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type EncodedDict = Vec<(String, String)>;

#[derive(Clone)]
pub struct Dictionary {
    pub words: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub encoded_cache: Arc<Mutex<Option<EncodedDict>>>,
}

impl Dictionary {
    pub fn new(dictionary: HashMap<String, Vec<String>>) -> Self {
        Self {
            words: Arc::new(Mutex::new(dictionary)),
            encoded_cache: Arc::new(Mutex::new(None)),
        }
    }
    pub fn set(&self, dictionary: HashMap<String, Vec<String>>) {
        *self.words.lock().unwrap() = dictionary;
        *self.encoded_cache.lock().unwrap() = None;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DictionaryError {
    #[error("Invalid word format: {0}. Words must contain only letters (a-z, A-Z)")]
    InvalidWordFormat(String),
    #[error("Dictionary import must contain at least one valid word")]
    EmptyDictionary,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(pairs: &[(&str, &[&str])]) -> HashMap<String, Vec<String>> {
        pairs
            .iter()
            .map(|(k, langs)| {
                (
                    k.to_string(),
                    langs.iter().map(|s| s.to_string()).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn new_starts_with_empty_cache() {
        let dict = Dictionary::new(HashMap::new());
        assert!(dict.encoded_cache.lock().unwrap().is_none());
    }

    #[test]
    fn new_stores_words_unchanged() {
        let initial = words(&[("hello", &["english"]), ("bonjour", &["french"])]);
        let dict = Dictionary::new(initial.clone());
        assert_eq!(*dict.words.lock().unwrap(), initial);
    }

    #[test]
    fn set_replaces_words() {
        let dict = Dictionary::new(words(&[("old", &["english"])]));
        let new_words = words(&[("new", &["french"])]);
        dict.set(new_words.clone());
        assert_eq!(*dict.words.lock().unwrap(), new_words);
    }

    #[test]
    fn set_invalidates_populated_cache() {
        let dict = Dictionary::new(words(&[("hello", &["english"])]));
        *dict.encoded_cache.lock().unwrap() =
            Some(vec![("hello".to_string(), "HL".to_string())]);
        assert!(dict.encoded_cache.lock().unwrap().is_some());

        dict.set(words(&[("world", &["english"])]));
        assert!(dict.encoded_cache.lock().unwrap().is_none());
    }

    #[test]
    fn set_to_same_words_still_invalidates_cache() {
        let initial = words(&[("hello", &["english"])]);
        let dict = Dictionary::new(initial.clone());
        *dict.encoded_cache.lock().unwrap() =
            Some(vec![("hello".to_string(), "HL".to_string())]);

        dict.set(initial);
        assert!(dict.encoded_cache.lock().unwrap().is_none());
    }

    #[test]
    fn clone_shares_words_storage() {
        let dict = Dictionary::new(HashMap::new());
        let dict2 = dict.clone();

        let new_words = words(&[("shared", &["english"])]);
        dict.set(new_words.clone());

        assert_eq!(*dict2.words.lock().unwrap(), new_words);
    }

    #[test]
    fn clone_shares_cache_storage() {
        let dict = Dictionary::new(HashMap::new());
        let dict2 = dict.clone();

        let cached = vec![("a".to_string(), "A".to_string())];
        *dict.encoded_cache.lock().unwrap() = Some(cached.clone());

        assert_eq!(*dict2.encoded_cache.lock().unwrap(), Some(cached));
    }

    #[test]
    fn set_via_one_handle_invalidates_cache_on_clone() {
        let dict = Dictionary::new(words(&[("hello", &["english"])]));
        let dict2 = dict.clone();

        *dict.encoded_cache.lock().unwrap() =
            Some(vec![("hello".to_string(), "HL".to_string())]);
        assert!(dict2.encoded_cache.lock().unwrap().is_some());

        dict.set(words(&[("world", &["english"])]));
        assert!(dict2.encoded_cache.lock().unwrap().is_none());
    }
}
