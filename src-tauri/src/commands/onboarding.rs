use crate::settings::{self, OnboardingState};
use tauri::{command, AppHandle};

#[command]
pub fn get_onboarding_state(app: AppHandle) -> Result<OnboardingState, String> {
    let s = settings::load_settings(&app);
    Ok(s.onboarding)
}

#[command]
pub fn set_onboarding_used_home_shortcut(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.used_home_shortcut {
        s.onboarding.used_home_shortcut = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[command]
pub fn set_onboarding_transcribed_outside_app(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.transcribed_outside_app {
        s.onboarding.transcribed_outside_app = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[command]
pub fn set_onboarding_added_dictionary_word(app: AppHandle) -> Result<OnboardingState, String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.added_dictionary_word {
        s.onboarding.added_dictionary_word = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(s.onboarding)
}

#[command]
pub fn set_onboarding_congrats_dismissed(app: AppHandle) -> Result<(), String> {
    let mut s = settings::load_settings(&app);
    if !s.onboarding.congrats_dismissed {
        s.onboarding.congrats_dismissed = true;
        settings::save_settings(&app, &s)?;
    }
    Ok(())
}
