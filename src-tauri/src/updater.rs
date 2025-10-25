use std::env;

/// Represents the Linux installation format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinuxFormat {
    /// AppImage format (default for generic Linux)
    AppImage,
    /// Debian package format (.deb)
    Deb,
}

impl LinuxFormat {
    /// Determine the Linux format based on environment or build configuration
    pub fn detect() -> Self {
        // Check environment variable first (useful for testing and CI/CD)
        if let Ok(format) = env::var("MURMURE_LINUX_FORMAT") {
            match format.to_lowercase().as_str() {
                "deb" => return LinuxFormat::Deb,
                "appimage" => return LinuxFormat::AppImage,
                _ => {}
            }
        }

        // Check if running on a Debian-based system and was installed via deb
        #[cfg(target_os = "linux")]
        {
            // Check if the app was installed via dpkg (typical for .deb installations)
            if Self::is_deb_installed() {
                return LinuxFormat::Deb;
            }
        }

        // Default to AppImage
        LinuxFormat::AppImage
    }

    /// Check if the application was installed via Debian package manager
    #[cfg(target_os = "linux")]
    #[allow(dead_code)]
    fn is_deb_installed() -> bool {
        // Check if the executable path contains /opt/ or /usr/ (typical deb installation paths)
        if let Ok(exe_path) = env::current_exe() {
            let path_str = exe_path.to_string_lossy();
            if path_str.contains("/opt/") || path_str.contains("/usr/") {
                return true;
            }
        }

        // Alternative: check if installed via apt/dpkg
        // This is more reliable but requires system calls
        std::process::Command::new("dpkg")
            .args(&["-l", "murmure"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "linux"))]
    #[allow(dead_code)]
    fn is_deb_installed() -> bool {
        false
    }

    /// Get the custom target string for the updater plugin
    pub fn to_target_string(&self) -> String {
        match self {
            LinuxFormat::AppImage => "linux-x86_64".to_string(),
            LinuxFormat::Deb => "linux-deb-x86_64".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_format_to_target_string() {
        assert_eq!(LinuxFormat::AppImage.to_target_string(), "linux-x86_64");
        assert_eq!(LinuxFormat::Deb.to_target_string(), "linux-deb-x86_64");
    }
}
