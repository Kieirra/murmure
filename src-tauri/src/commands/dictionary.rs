use crate::dictionary::{self, Dictionary};
use crate::settings;
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
pub fn set_dictionary(app: AppHandle, dictionary: Vec<String>) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.added_dictionary_word && !dictionary.is_empty() {
        s.onboarding.added_dictionary_word = true;
        settings::save_settings(&app, &s)?;
    }

    let mut words: Vec<String> = Vec::new();
    for word in dictionary {
        if !words.contains(&word) {
            words.push(word);
        }
    }
    dictionary::save(&app, &words)?;
    app.state::<Dictionary>().set(words);

    // Emit event so frontend can react (onboarding, UI refresh)
    let _ = app.emit("dictionary:updated", ());

    Ok(())
}

#[command]
pub fn get_dictionary(app: AppHandle) -> Result<Vec<String>, String> {
    dictionary::load(&app)
}

#[command]
pub fn export_dictionary(app: AppHandle, file_path: String) -> Result<(), String> {
    dictionary::export_dictionary(&app, file_path)?;
    Ok(())
}

#[command]
pub fn import_dictionary(app: AppHandle, file_path: String) -> Result<(), String> {
    dictionary::import_dictionary(&app, file_path)?;
    let words = dictionary::load(&app)?;
    app.state::<Dictionary>().set(words);

    let _ = app.emit("dictionary:updated", ());
    Ok(())
}
