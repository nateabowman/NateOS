use spin::Mutex;
use alloc::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOSchedulerType {
    Noop,
    Deadline,
    CFQ,
}

#[derive(Debug, Clone)]
pub struct IORequest {
    pub block_number: u64,
    pub write: bool,
    pub priority: u64,
}

pub struct IOScheduler {
    scheduler_type: Mutex<IOSchedulerType>,
    request_queue: Mutex<VecDeque<IORequest, 256>>,
}

impl IOScheduler {
    pub const fn new() -> Self {
        IOScheduler {
            scheduler_type: Mutex::new(IOSchedulerType::Deadline),
            request_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn set_scheduler_type(&self, scheduler_type: IOSchedulerType) {
        *self.scheduler_type.lock() = scheduler_type;
    }

    pub fn enqueue_request(&self, request: IORequest) -> Result<(), &'static str> {
        self.request_queue.lock().push_back(request).map_err(|_| "Queue full")
    }

    pub fn get_next_request(&self) -> Option<IORequest> {
        let mut queue = self.request_queue.lock();
        let scheduler_type = *self.scheduler_type.lock();
        
        match scheduler_type {
            IOSchedulerType::Noop => {
                queue.pop_front()
            }
            IOSchedulerType::Deadline => {
                // Find request with highest priority
                let mut best_idx = 0;
                let mut best_priority = 0;
                for (i, req) in queue.iter().enumerate() {
                    if req.priority > best_priority {
                        best_priority = req.priority;
                        best_idx = i;
                    }
                }
                queue.remove(best_idx)
            }
            IOSchedulerType::CFQ => {
                // Round-robin with fairness
                queue.pop_front()
            }
        }
    }

    pub fn get_queue_length(&self) -> usize {
        self.request_queue.lock().len()
    }
}

pub static IO_SCHEDULER: IOScheduler = IOScheduler::new();

