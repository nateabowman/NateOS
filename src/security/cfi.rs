use spin::Mutex;
use alloc::collections::BTreeSet;

pub struct ControlFlowIntegrity {
    enabled: Mutex<bool>,
    valid_targets: Mutex<BTreeSet<u64>>,
}

impl ControlFlowIntegrity {
    pub const fn new() -> Self {
        ControlFlowIntegrity {
            enabled: Mutex::new(false),
            valid_targets: Mutex::new(BTreeSet::new()),
        }
    }

    pub fn init(&self) {
        *self.enabled.lock() = true;
        crate::io::println!("CFI: Control Flow Integrity enabled");
    }

    pub fn register_target(&self, address: u64) {
        self.valid_targets.lock().insert(address);
    }

    pub fn check_indirect_call(&self, target: u64) -> bool {
        if !*self.enabled.lock() {
            return true;
        }
        
        let valid = self.valid_targets.lock().contains(&target);
        if !valid {
            crate::io::println!("CFI: Invalid indirect call target 0x{:x}", target);
            crate::security::audit::AUDIT_LOGGER.log(
                crate::security::audit::AuditEventType::SecurityViolation,
                None,
                &alloc::format!("CFI violation at 0x{:x}", target),
            );
        }
        valid
    }

    pub fn check_return(&self, return_address: u64) -> bool {
        if !*self.enabled.lock() {
            return true;
        }
        
        // TODO: Implement return address validation
        // For now, always allow
        true
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static CFI: ControlFlowIntegrity = ControlFlowIntegrity::new();

