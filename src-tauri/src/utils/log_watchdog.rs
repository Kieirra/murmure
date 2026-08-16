use log::{info, warn};
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime};

pub const MAX_LOG_FILE_SIZE: u64 = 1024 * 1024;

const CHECK_INTERVAL: Duration = Duration::from_secs(60);

fn reset_log_file(path: &Path) -> std::io::Result<()> {
    OpenOptions::new().write(true).open(path)?.set_len(0)
}

pub fn spawn<R: Runtime>(app: &AppHandle<R>) {
    let dir = match app.path().app_log_dir() {
        Ok(dir) => dir,
        Err(e) => {
            warn!("Log watchdog: cannot resolve the log directory: {}", e);
            return;
        }
    };
    let path = dir.join(format!("{}.log", app.package_info().name));

    std::thread::spawn(move || loop {
        std::thread::sleep(CHECK_INTERVAL);

        let size = match fs::metadata(&path) {
            Ok(meta) => meta.len(),
            Err(_) => continue,
        };
        if size <= MAX_LOG_FILE_SIZE {
            continue;
        }

        match reset_log_file(&path) {
            Ok(()) => info!("Log file reset after reaching {} bytes", size),
            Err(e) => warn!("Log watchdog: cannot reset the log file: {}", e),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn reset_keeps_append_writes_at_offset_zero() {
        let path = std::env::temp_dir().join(format!(
            "murmure-log-watchdog-{}-{:?}.log",
            std::process::id(),
            std::thread::current().id()
        ));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open the test log file");

        let chunk = vec![b'x'; 64 * 1024];
        for _ in 0..33 {
            file.write_all(&chunk).expect("write the test payload");
        }
        file.flush().expect("flush the test payload");
        assert!(fs::metadata(&path).expect("read the metadata").len() > 2 * 1024 * 1024);

        reset_log_file(&path).expect("reset the test log file");

        let line = b"log file reset\n";
        file.write_all(line).expect("write after the reset");
        file.flush().expect("flush after the reset");

        let content = fs::read(&path).expect("read the test log file");
        assert_eq!(content.len(), line.len());
        assert!(!content.contains(&0));

        let _ = fs::remove_file(&path);
    }
}
