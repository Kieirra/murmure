use crate::dictionary::Dictionary;
use crate::settings;
use tauri::{AppHandle, Emitter, Manager, command};

#[command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    let mut saved_dictionary = settings::load_dictionary(&app)?;
    saved_dictionary.extend(dictionary.clone());
    if !s.onboarding.added_dictionary_word && !saved_dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
    }
    settings::save_settings(&app, &s)?;
    settings::save_dictionary(&app, &saved_dictionary)?;

    app.state::<Dictionary>().set(saved_dictionary.clone());

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ()); 

    Ok(())
}

#[command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    let dictionary = settings::load_dictionary(&app)?;
    Ok(dictionary)
}
