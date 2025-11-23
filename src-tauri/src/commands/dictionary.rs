use crate::dictionary::Dictionary;
use crate::settings;
use tauri::{AppHandle, Emitter, Manager, command};

#[command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    s.dictionary = dictionary.clone();
    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
    }
    settings::save_settings(&app, &s)?;

    app.state::<Dictionary>().set(dictionary.clone());

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    let s = settings::load_settings(&app);
    Ok(s.dictionary)
}
