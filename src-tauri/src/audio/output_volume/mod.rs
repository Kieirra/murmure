pub mod helpers;
pub mod output_volume;
pub mod types;

#[cfg(target_os = "linux")]
mod platform_linux;
#[cfg(target_os = "linux")]
use platform_linux as platform;

#[cfg(target_os = "macos")]
mod platform_macos;
#[cfg(target_os = "macos")]
use platform_macos as platform;

#[cfg(target_os = "windows")]
mod platform_windows;
#[cfg(target_os = "windows")]
use platform_windows as platform;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod platform_unsupported;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
use platform_unsupported as platform;

pub use helpers::MAX_LOWERED_PERCENT;
pub use output_volume::{
    lower_and_persist, restore_and_clear, restore_pending, unsupported_reason,
};
pub use types::LoweredOutput;
