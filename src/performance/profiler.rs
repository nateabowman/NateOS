use spin::Mutex;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub function_name: heapless::String<64>,
    pub call_count: u64,
    pub total_time: u64,
    pub min_time: u64,
    pub max_time: u64,
}

pub struct Profiler {
    entries: Mutex<BTreeMap<heapless::String<64>, ProfileEntry>>,
    enabled: AtomicU64,
}

impl Profiler {
    pub const fn new() -> Self {
        Profiler {
            entries: Mutex::new(BTreeMap::new()),
            enabled: AtomicU64::new(0),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(1, Ordering::Relaxed);
    }

    pub fn disable(&self) {
        self.enabled.store(0, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed) != 0
    }

    pub fn record_call(&self, function_name: &str, duration: u64) {
        if !self.is_enabled() {
            return;
        }

        let name = heapless::String::from_str(function_name).unwrap_or(heapless::String::new());
        let mut entries = self.entries.lock();
        
        let entry = entries.entry(name).or_insert_with(|| ProfileEntry {
            function_name: heapless::String::from_str(function_name).unwrap_or(heapless::String::new()),
            call_count: 0,
            total_time: 0,
            min_time: u64::MAX,
            max_time: 0,
        });

        entry.call_count += 1;
        entry.total_time += duration;
        entry.min_time = entry.min_time.min(duration);
        entry.max_time = entry.max_time.max(duration);
    }

    pub fn get_report(&self) -> alloc::vec::Vec<ProfileEntry> {
        self.entries.lock().values().cloned().collect()
    }

    pub fn reset(&self) {
        self.entries.lock().clear();
    }
}

pub static PROFILER: Profiler = Profiler::new();

#[macro_export]
macro_rules! profile {
    ($name:expr, $block:block) => {
        {
            let start = crate::timer::get_time_ms();
            let result = $block;
            let duration = crate::timer::get_time_ms() - start;
            crate::performance::profiler::PROFILER.record_call($name, duration);
            result
        }
    };
}

