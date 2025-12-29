use core::sync::atomic::{AtomicU64, Ordering};

static MEMORY_USAGE: AtomicU64 = AtomicU64::new(0);
static MAX_MEMORY: AtomicU64 = AtomicU64::new(1024 * 1024 * 1024); // 1GB default
static PROCESS_COUNT: AtomicU64 = AtomicU64::new(0);
static MAX_PROCESSES: AtomicU64 = AtomicU64::new(1000);

pub struct ResourceMonitor;

impl ResourceMonitor {
    pub fn check_memory() -> bool {
        let usage = MEMORY_USAGE.load(Ordering::Relaxed);
        let max = MAX_MEMORY.load(Ordering::Relaxed);
        usage < max
    }

    pub fn check_processes() -> bool {
        let count = PROCESS_COUNT.load(Ordering::Relaxed);
        let max = MAX_PROCESSES.load(Ordering::Relaxed);
        count < max
    }

    pub fn increment_memory(&self, amount: u64) {
        MEMORY_USAGE.fetch_add(amount, Ordering::Relaxed);
    }

    pub fn decrement_memory(&self, amount: u64) {
        MEMORY_USAGE.fetch_sub(amount, Ordering::Relaxed);
    }

    pub fn increment_processes(&self) {
        PROCESS_COUNT.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_processes(&self) {
        PROCESS_COUNT.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get_stats() -> (u64, u64, u64, u64) {
        (
            MEMORY_USAGE.load(Ordering::Relaxed),
            MAX_MEMORY.load(Ordering::Relaxed),
            PROCESS_COUNT.load(Ordering::Relaxed),
            MAX_PROCESSES.load(Ordering::Relaxed),
        )
    }

    pub fn print_stats() {
        let (mem_usage, mem_max, proc_count, proc_max) = Self::get_stats();
        crate::io::println!("Resource Monitor:");
        crate::io::println!("  Memory: {} / {} bytes", mem_usage, mem_max);
        crate::io::println!("  Processes: {} / {}", proc_count, proc_max);
    }
}

pub static RESOURCE_MONITOR: ResourceMonitor = ResourceMonitor;

