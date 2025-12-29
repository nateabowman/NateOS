use spin::Mutex;
use alloc::collections::VecDeque;
use crate::process::ProcessId;

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub pid: Option<ProcessId>,
    pub event_type: AuditEventType,
    pub message: heapless::String<256>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    SystemCall,
    FileAccess,
    ProcessCreation,
    ProcessTermination,
    PermissionDenied,
    SecurityViolation,
    CapabilityChange,
}

pub struct AuditLogger {
    events: Mutex<VecDeque<AuditEvent, 1024>>,
    enabled: Mutex<bool>,
}

impl AuditLogger {
    pub const fn new() -> Self {
        AuditLogger {
            events: Mutex::new(VecDeque::new()),
            enabled: Mutex::new(true),
        }
    }

    pub fn log(&self, event_type: AuditEventType, pid: Option<ProcessId>, message: &str) {
        if !*self.enabled.lock() {
            return;
        }

        let event = AuditEvent {
            timestamp: crate::timer::get_time_ms(),
            pid,
            event_type,
            message: heapless::String::from_str(message).unwrap_or(heapless::String::new()),
        };

        let mut events = self.events.lock();
        if events.is_full() {
            events.pop_front();
        }
        events.push_back(event).ok();
    }

    pub fn get_events(&self) -> VecDeque<AuditEvent, 1024> {
        self.events.lock().clone()
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static AUDIT_LOGGER: AuditLogger = AuditLogger::new();

