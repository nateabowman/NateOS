use crate::process::ProcessId;
use crate::scheduler::SCHEDULER;
use spin::Mutex;
use alloc::collections::BinaryHeap;
use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScheduledProcess {
    pid: ProcessId,
    priority: u64,
    vruntime: u64,
}

impl PartialOrd for ScheduledProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower vruntime = higher priority
        other.vruntime.cmp(&self.vruntime)
    }
}

pub struct OptimizedScheduler {
    runqueue: Mutex<BinaryHeap<ScheduledProcess>>,
    min_vruntime: Mutex<u64>,
}

impl OptimizedScheduler {
    pub const fn new() -> Self {
        OptimizedScheduler {
            runqueue: Mutex::new(BinaryHeap::new()),
            min_vruntime: Mutex::new(0),
        }
    }

    pub fn enqueue(&self, pid: ProcessId, priority: u64) {
        let vruntime = *self.min_vruntime.lock();
        let proc = ScheduledProcess {
            pid,
            priority,
            vruntime,
        };
        self.runqueue.lock().push(proc);
    }

    pub fn schedule_next(&self) -> Option<ProcessId> {
        let mut queue = self.runqueue.lock();
        if let Some(proc) = queue.pop() {
            // Update vruntime based on execution time
            let time_slice = 10; // 10ms time slice
            let new_vruntime = proc.vruntime + (time_slice * 1000) / proc.priority.max(1);
            *self.min_vruntime.lock() = new_vruntime;
            Some(proc.pid)
        } else {
            None
        }
    }

    pub fn get_load(&self) -> usize {
        self.runqueue.lock().len()
    }
}

pub static OPTIMIZED_SCHEDULER: OptimizedScheduler = OptimizedScheduler::new();

