use tauri::{AppHandle, Emitter, Manager};

// Enter, Escape (dialog controls), left click (dialog interactions).
const CAPTURE_EXCLUDED_VKS: &[i32] = &[0x0D, 0x1B, 0x01];

fn is_fallback_name(name: &str) -> bool {
    name.strip_prefix("key").is_some_and(|rest| {
        !rest.is_empty() && rest.chars().all(|character| character.is_ascii_digit())
    })
}

/// Accumulates a native key press and emits the complete canonical binding.
pub fn handle_capture_key(app: &AppHandle, vk: i32) {
    if CAPTURE_EXCLUDED_VKS.contains(&vk) {
        return;
    }
    if is_fallback_name(&crate::shortcuts::helpers::vk_to_key_name(vk)) {
        return;
    }

    let state = app.state::<crate::shortcuts::ShortcutState>();
    let mut keys = state.capture_keys.lock();
    if keys.contains(&vk) {
        return;
    }

    keys.push(vk);
    let _ = app.emit(
        "shortcut-capture-update",
        serde_json::json!({ "keys": crate::shortcuts::keys_to_string(&keys) }),
    );
}

#[cfg(test)]
mod tests {
    use super::{is_fallback_name, CAPTURE_EXCLUDED_VKS};

    #[test]
    fn identifies_only_numeric_fallback_names() {
        assert!(is_fallback_name("key20"));
        assert!(!is_fallback_name("f13"));
        assert!(!is_fallback_name("key"));
        assert!(!is_fallback_name("keyboard"));
    }

    #[test]
    fn excludes_dialog_control_inputs() {
        assert!(CAPTURE_EXCLUDED_VKS.contains(&0x0D));
        assert!(CAPTURE_EXCLUDED_VKS.contains(&0x1B));
        assert!(CAPTURE_EXCLUDED_VKS.contains(&0x01));
    }
}
