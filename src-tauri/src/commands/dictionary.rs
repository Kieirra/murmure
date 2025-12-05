use crate::dictionary::{self, Dictionary};
use crate::settings;
use std::collections::HashMap;
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    let mut words = dictionary::load(&app)?;

    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
    }

    for word in dictionary {
        words
            .entry(word)
            .or_insert(vec!["english".to_string(), "french".to_string()]);
    }

    settings::save_settings(&app, &s)?;
    dictionary::save(&app, &words)?;
    app.state::<Dictionary>().set(words.clone());

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    let dictionary = dictionary::load(&app)?;
    let words = HashMap::keys(&dictionary).cloned().collect();
    Ok(words)
}
