use crate::process::{ProcessId, PROCESS_MANAGER};
use crate::scheduler::SCHEDULER;
use spin::Mutex;
use alloc::collections::BinaryHeap;
use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OomCandidate {
    pid: ProcessId,
    score: u64,
    memory_usage: u64,
}

impl PartialOrd for OomCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OomCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher score = higher priority to kill
        other.score.cmp(&self.score)
    }
}

pub struct OomKiller {
    enabled: Mutex<bool>,
    min_free_kb: Mutex<u64>,
}

impl OomKiller {
    pub const fn new() -> Self {
        OomKiller {
            enabled: Mutex::new(true),
            min_free_kb: Mutex::new(16384), // 16MB minimum free
        }
    }

    pub fn check_oom(&self) -> bool {
        if !*self.enabled.lock() {
            return false;
        }

        // TODO: Check actual free memory
        // For now, always return false (no OOM condition)
        false
    }

    pub fn select_victim(&self) -> Option<ProcessId> {
        let mut candidates = BinaryHeap::new();

        // TODO: Iterate through all processes and calculate OOM score
        // For now, return None
        None
    }

    pub fn kill_process(&self, pid: ProcessId) -> Result<(), &'static str> {
        crate::io::println!("OOM Killer: Terminating process {}", pid.0);
        
        // TODO: Actually terminate the process
        // For now, just log it
        Ok(())
    }

    pub fn handle_oom(&self) {
        if let Some(victim) = self.select_victim() {
            if let Err(e) = self.kill_process(victim) {
                crate::io::println!("OOM Killer error: {}", e);
            }
        } else {
            crate::io::println!("OOM Killer: No suitable victim found");
        }
    }

    pub fn set_min_free(&self, kb: u64) {
        *self.min_free_kb.lock() = kb;
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static OOM_KILLER: OomKiller = OomKiller::new();

