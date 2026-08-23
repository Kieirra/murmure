use log::debug;
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use super::platform_linux as platform;

#[cfg(target_os = "windows")]
use super::platform_windows as platform;

#[cfg(target_os = "macos")]
use super::platform_macos as platform;

const MODIFIER_RELEASE_TIMEOUT: Duration = Duration::from_secs(2);
const MODIFIER_RELEASE_POLL: Duration = Duration::from_millis(16);

pub fn wait_for_modifiers_released() {
    let start = Instant::now();
    while platform::any_modifier_held() {
        if start.elapsed() >= MODIFIER_RELEASE_TIMEOUT {
            debug!("Timed out waiting for physical modifiers to be released");
            return;
        }
        std::thread::sleep(MODIFIER_RELEASE_POLL);
    }
}
