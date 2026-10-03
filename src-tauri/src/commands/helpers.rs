use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub(super) async fn wait_until_stopped(is_running: &AtomicBool) {
    for _ in 0..20 {
        if !is_running.load(Ordering::SeqCst) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
