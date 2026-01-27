use crate::settings;
use crate::shortcuts::types::ShortcutState;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::shortcuts::{
    keys_to_string, parse_binding_keys, LLMRecordShortcutKeys, LastTranscriptShortcutKeys,
    RecordShortcutKeys, CommandShortcutKeys, LLMMode1ShortcutKeys, LLMMode2ShortcutKeys,
    LLMMode3ShortcutKeys, LLMMode4ShortcutKeys,
};
#[cfg(target_os = "macos")]
use crate::shortcuts::{
    register_last_transcript_shortcut, register_llm_record_shortcut, register_record_shortcut,
    register_command_shortcut, register_mode_switch_shortcut,
};
use log::{error, info};
use tauri::{command, AppHandle, Manager};
#[cfg(target_os = "macos")]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[command]
pub fn get_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.record_shortcut)
}

#[command]
pub fn set_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return set_record_shortcut_linux_windows(app, binding);
    #[cfg(target_os = "macos")]
    return set_record_shortcut_macos(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_record_shortcut_macos(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set record shortcut to: {}", binding);

    let mut s = settings::load_settings(&app);

    match binding.parse::<Shortcut>() {
        Ok(new_shortcut) => {
            // Step 1: Unregister the old shortcut handler
            if let Ok(old_shortcut) = s.record_shortcut.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(old_shortcut);
            }

            // Step 2: Register the new shortcut with its handler
            register_record_shortcut(&app, new_shortcut)?;

            // Step 3: Save the new binding to settings
            s.record_shortcut = binding.clone();
            settings::save_settings(&app, &s)?;

            info!("Successfully set record shortcut to: {}", binding);
            Ok(binding)
        }
        Err(parse_error) => {
            let error_msg = format!(
                "Invalid shortcut format '{}'. Expected format: 'Cmd+Shift+R' or similar. Parse error: {}",
                binding, parse_error
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_record_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    info!("Attempting to set record shortcut to: {}", binding);

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+r' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.record_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<RecordShortcutKeys>().set(keys);

    info!("Successfully set record shortcut to: {}", normalized);
    Ok(normalized)
}

#[command]
pub fn get_last_transcript_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.last_transcript_shortcut)
}

#[command]
pub fn set_last_transcript_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return set_last_transcript_shortcut_linux_windows(app, binding);
    #[cfg(target_os = "macos")]
    return set_last_transcript_shortcut_macos(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_last_transcript_shortcut_macos(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set last transcript shortcut to: {}", binding);

    let mut s = settings::load_settings(&app);

    match binding.parse::<Shortcut>() {
        Ok(new_shortcut) => {
            // Step 1: Unregister the old shortcut handler
            if let Ok(old_shortcut) = s.last_transcript_shortcut.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(old_shortcut);
            }

            // Step 2: Register the new shortcut with its handler
            register_last_transcript_shortcut(&app, new_shortcut)?;

            // Step 3: Save the new binding to settings
            s.last_transcript_shortcut = binding.clone();
            settings::save_settings(&app, &s)?;

            info!("Successfully set last transcript shortcut to: {}", binding);
            Ok(binding)
        }
        Err(parse_error) => {
            let error_msg = format!(
                "Invalid shortcut format '{}'. Expected format: 'Cmd+Shift+T' or similar. Parse error: {}",
                binding, parse_error
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_last_transcript_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    info!("Attempting to set last transcript shortcut to: {}", binding);

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+t' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.last_transcript_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<LastTranscriptShortcutKeys>().set(keys);

    info!("Successfully set last transcript shortcut to: {}", normalized);
    Ok(normalized)
}

// LLM Record Shortcut Commands
#[command]
pub fn get_llm_record_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_record_shortcut)
}

#[command]
pub fn set_llm_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_llm_record_shortcut_macos(app, binding);
    }
    #[cfg(not(target_os = "macos"))]
    return set_llm_record_shortcut_linux_windows(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_llm_record_shortcut_macos(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set LLM record shortcut to: {}", binding);

    let mut s = settings::load_settings(&app);

    match binding.parse::<Shortcut>() {
        Ok(new_shortcut) => {
            if let Ok(old_shortcut) = s.llm_record_shortcut.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(old_shortcut);
            }

            register_llm_record_shortcut(&app, new_shortcut)?;

            s.llm_record_shortcut = binding.clone();
            settings::save_settings(&app, &s)?;

            info!("Successfully set LLM record shortcut to: {}", binding);
            Ok(binding)
        }
        Err(parse_error) => {
            let error_msg = format!(
                "Invalid shortcut format '{}'. Expected format: 'Cmd+Shift+L' or similar. Parse error: {}",
                binding, parse_error
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_llm_record_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set LLM record shortcut to: {}", binding);

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+l' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.llm_record_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<LLMRecordShortcutKeys>().set(keys);

    info!("Successfully set LLM record shortcut to: {}", normalized);
    Ok(normalized)
}


// LLM Record Shortcut Commands
#[command]
pub fn get_command_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.command_shortcut)
}

#[command]
pub fn set_command_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_command_shortcut_macos(app, binding);
    }
    #[cfg(not(target_os = "macos"))]
    return set_command_shortcut_linux_windows(app, binding);
}

#[cfg(target_os = "macos")]
pub fn set_command_shortcut_macos(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set command shortcut to: {}", binding);

    let mut s = settings::load_settings(&app);

    match binding.parse::<Shortcut>() {
        Ok(new_shortcut) => {
            if let Ok(old_shortcut) = s.command_shortcut.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(old_shortcut);
            }

            register_command_shortcut(&app, new_shortcut)?;

            s.command_shortcut = binding.clone();
            settings::save_settings(&app, &s)?;

            info!("Successfully set command shortcut to: {}", binding);
            Ok(binding)
        }
        Err(parse_error) => {
            let error_msg = format!(
                "Invalid shortcut format '{}'. Expected format: 'Cmd+Shift+C' or similar. Parse error: {}",
                binding, parse_error
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_command_shortcut_linux_windows(
    app: AppHandle,
    binding: String,
) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set command shortcut to: {}", binding);

    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+c' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);

    let mut s = settings::load_settings(&app);
    s.command_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;

    app.state::<CommandShortcutKeys>().set(keys);

    info!("Successfully set command shortcut to: {}", normalized);
    Ok(normalized)
}

#[command]
pub fn suspend_transcription(app_handle: AppHandle) {
    let state = app_handle.state::<ShortcutState>();
    state.set_suspended(true);
}

#[command]
pub fn resume_transcription(app_handle: AppHandle) {
    let state = app_handle.state::<ShortcutState>();
    state.set_suspended(false);
}

#[command]
pub fn get_llm_mode_1_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_1_shortcut)
}

#[command]
pub fn set_llm_mode_1_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_llm_mode_shortcut_macos(app, binding, 0, |s| &mut s.llm_mode_1_shortcut);
    }
    #[cfg(not(target_os = "macos"))]
    return set_llm_mode_1_shortcut_linux_windows(app, binding);
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_llm_mode_1_shortcut_linux_windows(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }
    info!("Attempting to set LLM mode 1 shortcut to: {}", binding);
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+1' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);
    let mut s = settings::load_settings(&app);
    s.llm_mode_1_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;
    app.state::<LLMMode1ShortcutKeys>().set(keys);
    info!("Successfully set LLM mode 1 shortcut to: {}", normalized);
    Ok(normalized)
}

#[command]
pub fn get_llm_mode_2_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_2_shortcut)
}

#[command]
pub fn set_llm_mode_2_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_llm_mode_shortcut_macos(app, binding, 1, |s| &mut s.llm_mode_2_shortcut);
    }
    #[cfg(not(target_os = "macos"))]
    return set_llm_mode_2_shortcut_linux_windows(app, binding);
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_llm_mode_2_shortcut_linux_windows(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }
    info!("Attempting to set LLM mode 2 shortcut to: {}", binding);
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+2' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);
    let mut s = settings::load_settings(&app);
    s.llm_mode_2_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;
    app.state::<LLMMode2ShortcutKeys>().set(keys);
    info!("Successfully set LLM mode 2 shortcut to: {}", normalized);
    Ok(normalized)
}

#[command]
pub fn get_llm_mode_3_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_3_shortcut)
}

#[command]
pub fn set_llm_mode_3_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_llm_mode_shortcut_macos(app, binding, 2, |s| &mut s.llm_mode_3_shortcut);
    }
    #[cfg(not(target_os = "macos"))]
    return set_llm_mode_3_shortcut_linux_windows(app, binding);
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_llm_mode_3_shortcut_linux_windows(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }
    info!("Attempting to set LLM mode 3 shortcut to: {}", binding);
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+3' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);
    let mut s = settings::load_settings(&app);
    s.llm_mode_3_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;
    app.state::<LLMMode3ShortcutKeys>().set(keys);
    info!("Successfully set LLM mode 3 shortcut to: {}", normalized);
    Ok(normalized)
}

#[command]
pub fn get_llm_mode_4_shortcut(app: AppHandle) -> Result<String, String> {
    let s = settings::load_settings(&app);
    Ok(s.llm_mode_4_shortcut)
}

#[command]
pub fn set_llm_mode_4_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        return set_llm_mode_shortcut_macos(app, binding, 3, |s| &mut s.llm_mode_4_shortcut);
    }
    #[cfg(not(target_os = "macos"))]
    return set_llm_mode_4_shortcut_linux_windows(app, binding);
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn set_llm_mode_4_shortcut_linux_windows(app: AppHandle, binding: String) -> Result<String, String> {
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }
    info!("Attempting to set LLM mode 4 shortcut to: {}", binding);
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        let error_msg = format!(
            "Invalid shortcut format '{}'. Expected format: 'ctrl+shift+4' or similar. No valid keys could be parsed.",
            binding
        );
        error!("{}", error_msg);
        return Err(error_msg);
    }
    let normalized = keys_to_string(&keys);
    let mut s = settings::load_settings(&app);
    s.llm_mode_4_shortcut = normalized.clone();
    settings::save_settings(&app, &s)?;
    app.state::<LLMMode4ShortcutKeys>().set(keys);
    info!("Successfully set LLM mode 4 shortcut to: {}", normalized);
    Ok(normalized)
}

#[cfg(target_os = "macos")]
pub fn set_llm_mode_shortcut_macos<F>(
    app: AppHandle,
    binding: String,
    mode_index: usize,
    get_field: F,
) -> Result<String, String>
where
    F: Fn(&mut crate::settings::types::AppSettings) -> &mut String,
{
    if binding.is_empty() {
        return Err("Shortcut binding cannot be empty".to_string());
    }

    info!("Attempting to set LLM mode {} shortcut to: {}", mode_index + 1, binding);

    let mut s = settings::load_settings(&app);

    match binding.parse::<Shortcut>() {
        Ok(new_shortcut) => {
            let old_binding = get_field(&mut s).clone();
            if let Ok(old_shortcut) = old_binding.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(old_shortcut);
            }

            register_mode_switch_shortcut(&app, new_shortcut, mode_index)?;

            *get_field(&mut s) = binding.clone();
            settings::save_settings(&app, &s)?;

            info!("Successfully set LLM mode {} shortcut to: {}", mode_index + 1, binding);
            Ok(binding)
        }
        Err(parse_error) => {
            let error_msg = format!(
                "Invalid shortcut format '{}'. Expected format: 'Cmd+Shift+1' or similar. Parse error: {}",
                binding, parse_error
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}
