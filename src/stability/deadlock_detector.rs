use spin::Mutex;
use alloc::collections::BTreeMap;
use crate::process::ProcessId;

#[derive(Debug, Clone)]
pub struct LockAcquisition {
    pub process: ProcessId,
    pub lock_id: usize,
    pub timestamp: u64,
}

pub struct DeadlockDetector {
    lock_holders: Mutex<BTreeMap<usize, LockAcquisition>>,
    process_locks: Mutex<BTreeMap<ProcessId, alloc::vec::Vec<usize>>>,
}

impl DeadlockDetector {
    pub const fn new() -> Self {
        DeadlockDetector {
            lock_holders: Mutex::new(BTreeMap::new()),
            process_locks: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn acquire_lock(&self, process: ProcessId, lock_id: usize) -> Result<(), &'static str> {
        // Check for potential deadlock
        if let Some(holder) = self.lock_holders.lock().get(&lock_id) {
            // Check if holder is waiting for a lock held by this process
            let process_locks = self.process_locks.lock();
            if let Some(held_locks) = process_locks.get(&holder.process) {
                for &held_lock in held_locks {
                    if let Some(held_holder) = self.lock_holders.lock().get(&held_lock) {
                        if held_holder.process == process {
                            return Err("Potential deadlock detected");
                        }
                    }
                }
            }
        }
        
        // Record lock acquisition
        self.lock_holders.lock().insert(lock_id, LockAcquisition {
            process,
            lock_id,
            timestamp: crate::timer::get_time_ms(),
        });
        
        self.process_locks.lock()
            .entry(process)
            .or_insert_with(alloc::vec::Vec::new)
            .push(lock_id);
        
        Ok(())
    }

    pub fn release_lock(&self, process: ProcessId, lock_id: usize) {
        self.lock_holders.lock().remove(&lock_id);
        if let Some(locks) = self.process_locks.lock().get_mut(&process) {
            locks.retain(|&id| id != lock_id);
        }
    }
}

pub static DEADLOCK_DETECTOR: DeadlockDetector = DeadlockDetector::new();

