//! macOS Accessibility permission handling
//!
//! Global keyboard shortcuts on macOS require Accessibility permissions.
//! This module provides utilities to check and request these permissions.

use log::{info, warn};

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

/// Check if the application has Accessibility permission
pub fn is_accessibility_enabled() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Open System Settings > Privacy & Security > Accessibility panel
pub fn open_accessibility_settings() {
    info!("Opening Accessibility settings panel");
    let _ = std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
}

/// Check permission and log the status
pub fn check_and_log_permission() -> bool {
    let trusted = is_accessibility_enabled();
    if trusted {
        info!("Accessibility permission: GRANTED");
    } else {
        warn!("Accessibility permission: DENIED - shortcuts will not work");
    }
    trusted
}
