use crate::formatting_rules;
use tauri::{command, AppHandle};

#[command]
pub fn get_formatting_settings(
    app: AppHandle,
) -> Result<formatting_rules::FormattingSettings, String> {
    formatting_rules::load(&app)
}

#[command]
pub fn set_formatting_settings(
    app: AppHandle,
    settings: formatting_rules::FormattingSettings,
) -> Result<(), String> {
    formatting_rules::save(&app, &settings)
}

#[command]
pub fn validate_regex(pattern: String) -> Result<(), String> {
    formatting_rules::compile_user_regex(&pattern).map(|_| ())
}
