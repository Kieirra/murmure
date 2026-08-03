use log::{debug, warn};
use tauri_plugin_clipboard_manager::ClipboardExt;

const MAX_SNAPSHOT_BYTES: usize = 128 * 1024 * 1024;

pub(crate) enum ClipboardSnapshot {
    Text(String),
    Image(tauri::image::Image<'static>),
    #[cfg(target_os = "linux")]
    WaylandRaw {
        mime: String,
        bytes: Vec<u8>,
    },
}

impl ClipboardSnapshot {
    pub(crate) fn capture(app_handle: &tauri::AppHandle) -> Self {
        let text = app_handle.clipboard().read_text().unwrap_or_default();
        if !text.is_empty() {
            debug!("Clipboard snapshot: text ({} bytes)", text.len());
            return Self::Text(text);
        }

        #[cfg(target_os = "linux")]
        if let Some(snapshot) = capture_wayland_image() {
            return snapshot;
        }

        if let Ok(image) = app_handle.clipboard().read_image() {
            let (width, height) = (image.width(), image.height());
            let byte_len = width as usize * height as usize * 4;
            return match is_within_snapshot_limit(byte_len) {
                true => {
                    debug!("Clipboard snapshot: image {}x{}", width, height);
                    Self::Image(image.to_owned())
                }
                false => {
                    debug!(
                        "Clipboard snapshot too large ({} bytes), not preserved",
                        byte_len
                    );
                    Self::Text(String::new())
                }
            };
        }

        debug!("Clipboard snapshot: empty");
        Self::Text(String::new())
    }

    pub(crate) fn restore(self, app_handle: &tauri::AppHandle) -> Result<(), String> {
        match self {
            Self::Text(text) => super::clipboard::write_clipboard(&text, app_handle),
            Self::Image(image) => app_handle.clipboard().write_image(&image).map_err(|e| {
                let message = format!("Failed to restore clipboard image: {}", e);
                warn!("{}", message);
                message
            }),
            #[cfg(target_os = "linux")]
            Self::WaylandRaw { mime, bytes } => {
                super::clipboard::wl_copy_bytes(&bytes, Some(&mime))
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn capture_wayland_image() -> Option<ClipboardSnapshot> {
    if !crate::utils::platform::is_wayland_session() {
        return None;
    }
    if !super::clipboard::is_wl_paste_available() {
        return None;
    }

    let types = super::clipboard::wl_paste_types().ok()?;
    let mime = pick_image_mime(&types)?.to_string();
    let bytes = match super::clipboard::wl_paste_bytes(&mime) {
        Ok(bytes) => bytes,
        Err(_) => return Some(ClipboardSnapshot::Text(String::new())),
    };

    match is_within_snapshot_limit(bytes.len()) {
        true => {
            debug!("Clipboard snapshot: raw {} ({} bytes)", mime, bytes.len());
            Some(ClipboardSnapshot::WaylandRaw { mime, bytes })
        }
        false => {
            debug!(
                "Clipboard snapshot too large ({} bytes), not preserved",
                bytes.len()
            );
            Some(ClipboardSnapshot::Text(String::new()))
        }
    }
}

#[cfg(target_os = "linux")]
pub(crate) fn pick_image_mime(types: &[String]) -> Option<&str> {
    types
        .iter()
        .map(String::as_str)
        .find(|mime| mime.starts_with("image/"))
}

pub(crate) fn is_within_snapshot_limit(byte_len: usize) -> bool {
    byte_len <= MAX_SNAPSHOT_BYTES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn pick_image_mime_returns_first_image_type() {
        let types = vec![
            "text/html".to_string(),
            "image/png".to_string(),
            "image/jpeg".to_string(),
        ];
        assert_eq!(pick_image_mime(&types), Some("image/png"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pick_image_mime_ignores_non_image_types() {
        let types = vec![
            "text/plain".to_string(),
            "text/html".to_string(),
            "text/uri-list".to_string(),
        ];
        assert_eq!(pick_image_mime(&types), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pick_image_mime_handles_empty_list() {
        assert_eq!(pick_image_mime(&[]), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pick_image_mime_accepts_uncommon_image_types() {
        let types = vec!["image/svg+xml".to_string()];
        assert_eq!(pick_image_mime(&types), Some("image/svg+xml"));
    }

    #[test]
    fn snapshot_limit_accepts_a_large_screenshot() {
        assert!(is_within_snapshot_limit(7680 * 4320 * 4));
    }

    #[test]
    fn snapshot_limit_rejects_beyond_the_cap() {
        assert!(!is_within_snapshot_limit(MAX_SNAPSHOT_BYTES + 1));
    }

    #[test]
    fn snapshot_limit_accepts_the_exact_cap() {
        assert!(is_within_snapshot_limit(MAX_SNAPSHOT_BYTES));
    }
}
