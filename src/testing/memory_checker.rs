use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;
use alloc::collections::BTreeMap;

static ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
static DEALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
static ALLOCATIONS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());

pub struct MemoryChecker;

impl MemoryChecker {
    pub fn record_allocation(ptr: usize, size: usize) {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATIONS.lock().insert(ptr, size);
    }

    pub fn record_deallocation(ptr: usize) {
        DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATIONS.lock().remove(&ptr);
    }

    pub fn check_leaks() -> usize {
        ALLOCATIONS.lock().len()
    }

    pub fn get_stats() -> (u64, u64, usize) {
        (
            ALLOC_COUNT.load(Ordering::Relaxed),
            DEALLOC_COUNT.load(Ordering::Relaxed),
            Self::check_leaks(),
        )
    }

    pub fn print_stats() {
        let (alloc, dealloc, leaks) = Self::get_stats();
        crate::io::println!("Memory Stats:");
        crate::io::println!("  Allocations: {}", alloc);
        crate::io::println!("  Deallocations: {}", dealloc);
        crate::io::println!("  Potential leaks: {}", leaks);
    }
}

pub static MEMORY_CHECKER: MemoryChecker = MemoryChecker;

