use tauri::command;

/// Returns the current Linux session type as a wire string.
///
/// Wire values (`"wayland"` / `"x11"` / `"unknown"` / `null`) are produced by
/// [`LinuxSessionType::as_str`] and parsed by the frontend hook
/// `useLinuxSessionType` (`src/components/hooks/use-linux-session-type.ts`).
/// Keep the two ends in sync: any new variant added to `LinuxSessionType` must
/// also be accepted by the hook's narrowing check.
#[command]
pub fn get_linux_session_type() -> Option<String> {
    crate::utils::platform::get_linux_session_type().map(|session| session.as_str().to_string())
}
