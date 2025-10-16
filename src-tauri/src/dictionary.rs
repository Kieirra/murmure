use std::{path::PathBuf, sync::{Arc, Mutex}};
use rphonetic::{BeiderMorseBuilder, ConfigFiles, LanguageSet};
use tauri::{AppHandle, Manager};

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

/**
 * Use phonetic algorithm to fix the transcription
 */
pub fn fix_transcription_with_dictionary(transcription: String, dictionary: Vec<String>, cc_rules_path: PathBuf) -> String {
    if dictionary.is_empty() {
        return transcription;
    }

    let config_files = ConfigFiles::new(&cc_rules_path).unwrap();
    let builder = BeiderMorseBuilder::new(&config_files);
    let beider_morse = builder.build();

    // TODO: Make user able to choose the languages for each word
    let langs = LanguageSet::from(vec!["french", "english"]);

    // Prepare dictionary words to be encoded phonetically
    let mut encoded_dict = Vec::new();
    for word in &dictionary {
        let code = beider_morse.encode_with_languages(word, &langs);
        encoded_dict.push((word, code));
    }

    // Split transcription into words
    let mut corrected_transcription = transcription.clone();
    let words: Vec<&str> = transcription.split_whitespace().collect();

    for word in words {
        let candidate = beider_morse.encode_with_languages(word, &langs);
        let candidate_codes: Vec<&str> = candidate.split('|').collect();
        for (dict_word, dict_code) in &encoded_dict {
            let dict_codes: Vec<&str> = dict_code.split('|').collect();
            println!("Dict word: {:?}, Dict code: {:?}, Candidate: {:?}", dict_word, dict_code, candidate);
            if dict_codes.iter().any(|dc| candidate_codes.contains(dc)) {
                corrected_transcription = corrected_transcription.replace(word, dict_word);
            }
        }
    }

    corrected_transcription
}

// Downloaded from https://github.com/apache/commons-codec/tree/rel/commons-codec-1.15/src/main/resources/org/apache/commons/codec/language/bm
pub fn get_cc_rules_path(app_handle: &AppHandle) -> anyhow::Result<PathBuf> {
    let possible_paths = vec![
        // 1. Chemin pour la production (bundle)
        app_handle.path().resolve(
            "../resources/cc-rules/",
            tauri::path::BaseDirectory::Resource,
        ),
        app_handle.path().resolve(
            "_up_/resources/cc-rules/",
            tauri::path::BaseDirectory::Resource,
        ),
        app_handle.path().resolve(
            "resources/cc-rules/",
            tauri::path::BaseDirectory::Resource,
        ),
    ];

    // Essayer chaque chemin
    for path_result in possible_paths {
        match path_result {
            Ok(ref ccrules_path) if ccrules_path.exists() => {
                println!("Model found at: {}", ccrules_path.display());
                return Ok(ccrules_path.clone());
            }
            Ok(ref ccrules_path) => {
                println!("Model not found at: {}", ccrules_path.display());
            }
            Err(e) => {
                println!("Error resolving path: {:?}", e);
            }
        }
    }

    anyhow::bail!("Bundled cc_rules not found in any known location");
}