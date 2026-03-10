use crate::dictionary::{self, Dictionary};
use crate::settings;
use std::collections::HashMap;
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
        settings::save_settings(&app, &s)?;
    }

    let mut words = HashMap::new();
    for word in dictionary {
        words
            .entry(word)
            .or_insert(vec!["english".to_string(), "french".to_string()]);
    }
    dictionary::save(&app, &words)?;
    app.state::<Dictionary>().set(words.clone());

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    let dictionary = dictionary::load(&app)?;
    let words = dictionary.into_keys().collect();
    Ok(words)
}

#[command]
pub fn get_dictionary_with_languages(
    app: AppHandle,
) -> Result<HashMap<String, Vec<String>>, String> {
    dictionary::load(&app)
}

#[command]
pub fn set_dictionary_with_languages(
    app: AppHandle,
    dictionary: HashMap<String, Vec<String>>,
) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
        settings::save_settings(&app, &s)?;
    }

    dictionary::save(&app, &dictionary)?;
    app.state::<Dictionary>().set(dictionary);

    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[command]
pub fn export_dictionary(app: AppHandle, file_path: String) -> Result<(), String> {
    dictionary::export_dictionary(&app, file_path)?;
    Ok(())
}

#[command]
pub fn import_dictionary(app: AppHandle, file_path: String) -> Result<(), String> {
    dictionary::import_dictionary(&app, file_path)?;

    let _ = app.emit("dictionary:updated", ());
    Ok(())
}
