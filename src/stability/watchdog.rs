use core::sync::atomic::{AtomicU64, Ordering};
use crate::timer::get_time_ms;

static LAST_FEED_TIME: AtomicU64 = AtomicU64::new(0);
static WATCHDOG_TIMEOUT: AtomicU64 = AtomicU64::new(5000); // 5 seconds

pub struct Watchdog;

impl Watchdog {
    pub fn init() {
        LAST_FEED_TIME.store(get_time_ms(), Ordering::Relaxed);
        crate::io::println!("Watchdog initialized");
    }

    pub fn feed(&self) {
        LAST_FEED_TIME.store(get_time_ms(), Ordering::Relaxed);
    }

    pub fn check(&self) -> bool {
        let last_feed = LAST_FEED_TIME.load(Ordering::Relaxed);
        let timeout = WATCHDOG_TIMEOUT.load(Ordering::Relaxed);
        let current = get_time_ms();
        
        if current - last_feed > timeout {
            crate::io::println!("Watchdog timeout! System may be hung.");
            return false;
        }
        
        true
    }

    pub fn set_timeout(&self, timeout_ms: u64) {
        WATCHDOG_TIMEOUT.store(timeout_ms, Ordering::Relaxed);
    }
}

pub static WATCHDOG: Watchdog = Watchdog;

