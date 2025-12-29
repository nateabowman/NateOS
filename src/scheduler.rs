use crate::process::{ProcessId, ProcessState, PROCESS_MANAGER};
use spin::Mutex;
use alloc::collections::VecDeque;

pub struct Scheduler {
    ready_queue: Mutex<VecDeque<ProcessId>>,
    current_process: Mutex<Option<ProcessId>>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            ready_queue: Mutex::new(VecDeque::new()),
            current_process: Mutex::new(None),
        }
    }

    pub fn enqueue(&self, pid: ProcessId) {
        self.ready_queue.lock().push_back(pid);
    }

    pub fn schedule_next(&self) -> Option<ProcessId> {
        let mut queue = self.ready_queue.lock();
        let next = queue.pop_front();
        if let Some(pid) = next {
            *self.current_process.lock() = Some(pid);
        }
        next
    }

    pub fn get_current(&self) -> Option<ProcessId> {
        *self.current_process.lock()
    }

    pub fn yield_cpu(&self) {
        if let Some(current) = self.get_current() {
            self.enqueue(current);
            *self.current_process.lock() = None;
        }
    }
}

pub static SCHEDULER: Scheduler = Scheduler::new();

