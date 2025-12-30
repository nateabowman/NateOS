use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub address: u64,
    pub original_byte: u8,
    pub enabled: bool,
}

pub struct KernelDebugger {
    breakpoints: Mutex<BTreeMap<u64, Breakpoint>>,
    attached_processes: Mutex<alloc::vec::Vec<ProcessId>>,
    enabled: Mutex<bool>,
}

impl KernelDebugger {
    pub const fn new() -> Self {
        KernelDebugger {
            breakpoints: Mutex::new(BTreeMap::new()),
            attached_processes: Mutex::new(alloc::vec::Vec::new()),
            enabled: Mutex::new(false),
        }
    }

    pub fn set_breakpoint(&self, address: u64) -> Result<(), &'static str> {
        // TODO: Actually set breakpoint (INT3 instruction)
        let breakpoint = Breakpoint {
            address,
            original_byte: 0,
            enabled: true,
        };
        self.breakpoints.lock().insert(address, breakpoint);
        Ok(())
    }

    pub fn remove_breakpoint(&self, address: u64) -> Result<(), &'static str> {
        self.breakpoints.lock().remove(&address).ok_or("Breakpoint not found")?;
        Ok(())
    }

    pub fn attach(&self, pid: ProcessId) -> Result<(), &'static str> {
        self.attached_processes.lock().push(pid);
        Ok(())
    }

    pub fn detach(&self, pid: ProcessId) -> Result<(), &'static str> {
        let mut processes = self.attached_processes.lock();
        processes.retain(|&p| p != pid);
        Ok(())
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static DEBUGGER: KernelDebugger = KernelDebugger::new();

