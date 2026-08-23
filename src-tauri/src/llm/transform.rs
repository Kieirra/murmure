use log::{debug, error, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const MODIFIER_RELEASE_DELAY: Duration = Duration::from_millis(200);

static TRANSFORM_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn is_transform_active() -> bool {
    TRANSFORM_ACTIVE.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn is_transform_processing() -> bool {
    is_transform_active()
}

pub fn spawn_transform_selection(app: &AppHandle, index: usize) {
    if crate::shortcuts::shortcuts::ensure_llm_mode_ready(app, index, true).is_err() {
        return;
    }
    let app_for_thread = app.clone();
    std::thread::spawn(move || {
        transform_selection_with_mode(&app_for_thread, index);
    });
}

pub fn transform_selection_with_mode(app: &AppHandle, index: usize) {
    if TRANSFORM_ACTIVE
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        debug!("Transform: already running, request ignored");
        return;
    }

    crate::audio::sound::prewarm(app);
    crate::audio::sound::play_sound(app, crate::audio::sound::Sound::StartRecording);

    let _ = app.emit("transform-processing-start", ());

    if crate::settings::load_settings(app).overlay_mode.as_str() == "recording" {
        crate::overlay::overlay::clear_pending_flash(app);
        crate::overlay::overlay::show_recording_overlay(app);
    }

    #[cfg(target_os = "windows")]
    crate::shortcuts::platform_windows::wait_for_modifiers_released();

    std::thread::sleep(MODIFIER_RELEASE_DELAY);

    let selection = match crate::clipboard::get_selected_text(app) {
        Ok(text) if !text.trim().is_empty() => text,
        Ok(_) => {
            debug!("Transform: no selection captured");
            let _ = app.emit("transform-selection-empty", ());
            end_transform(app);
            return;
        }
        Err(e) => {
            error!("Transform: failed to capture selection: {}", e);
            let _ = app.emit("llm-error", e);
            end_transform(app);
            return;
        }
    };

    let settings = crate::llm::helpers::load_llm_connect_settings(app);
    if settings.modes.get(index).is_none() {
        warn!("Transform: mode {} missing", index + 1);
        end_transform(app);
        return;
    }

    crate::llm::switch_active_mode_silent(app, index);

    let result =
        tauri::async_runtime::block_on(crate::llm::post_process_with_llm(app, selection, false));

    end_transform(app);

    match result {
        Ok(text) => {
            crate::audio::sound::play_sound(app, crate::audio::sound::Sound::StopRecording);
            if let Err(e) = crate::clipboard::paste(&text, app) {
                error!("Transform: failed to paste result: {}", e);
            }
            if let Err(e) = crate::history::add_transcription(app, text) {
                error!("Transform: failed to save to history: {}", e);
            }
        }
        Err(e) => {
            warn!("Transform: LLM processing failed: {}", e);
            let _ = app.emit("llm-error", e);
        }
    }
}

fn end_transform(app: &AppHandle) {
    TRANSFORM_ACTIVE.store(false, Ordering::SeqCst);
    let _ = app.emit("transform-processing-end", ());
    hide_overlay_after_transform(app);
}

fn hide_overlay_after_transform(app: &AppHandle) {
    let s = crate::settings::load_settings(app);
    if s.overlay_mode.as_str() != "always" {
        crate::overlay::overlay::hide_recording_overlay(app);
    }
}
