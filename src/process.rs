use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;
use x86_64::VirtAddr;

static NEXT_PID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(usize);

impl ProcessId {
    pub fn new() -> Self {
        ProcessId(NEXT_PID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

pub struct Process {
    pub pid: ProcessId,
    pub state: ProcessState,
    pub stack_pointer: VirtAddr,
    pub instruction_pointer: VirtAddr,
}

impl Process {
    pub fn new(entry_point: VirtAddr, stack_top: VirtAddr) -> Self {
        Process {
            pid: ProcessId::new(),
            state: ProcessState::Ready,
            stack_pointer: stack_top,
            instruction_pointer: entry_point,
        }
    }
}

pub struct ProcessManager {
    processes: Mutex<Vec<Process>>,
    current_pid: Mutex<Option<ProcessId>>,
}

impl ProcessManager {
    pub const fn new() -> Self {
        ProcessManager {
            processes: Mutex::new(Vec::new()),
            current_pid: Mutex::new(None),
        }
    }

    pub fn create_process(&self, entry_point: VirtAddr, stack_top: VirtAddr) -> ProcessId {
        let process = Process::new(entry_point, stack_top);
        let pid = process.pid;
        self.processes.lock().push(process);
        pid
    }

    pub fn get_current_process(&self) -> Option<ProcessId> {
        *self.current_pid.lock()
    }

    pub fn set_current_process(&self, pid: ProcessId) {
        *self.current_pid.lock() = Some(pid);
    }
}

pub static PROCESS_MANAGER: ProcessManager = ProcessManager::new();

