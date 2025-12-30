use crate::process::ProcessId;
use crate::fs::inode::Inode;
use spin::Mutex;
use alloc::collections::BTreeMap;
use heapless::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacAction {
    Allow,
    Deny,
    Audit,
}

#[derive(Debug, Clone)]
pub struct MacPolicy {
    pub subject: String<64>,  // Process label
    pub object: String<64>,    // File/resource label
    pub action: MacAction,
}

pub struct MandatoryAccessControl {
    policies: Mutex<BTreeMap<(String<64>, String<64>), MacPolicy>>,
    process_labels: Mutex<BTreeMap<ProcessId, String<64>>>,
    object_labels: Mutex<BTreeMap<u64, String<64>>>, // inode -> label
}

impl MandatoryAccessControl {
    pub const fn new() -> Self {
        MandatoryAccessControl {
            policies: Mutex::new(BTreeMap::new()),
            process_labels: Mutex::new(BTreeMap::new()),
            object_labels: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn set_process_label(&self, pid: ProcessId, label: &str) -> Result<(), &'static str> {
        let label_str = String::from_str(label).map_err(|_| "Label too long")?;
        self.process_labels.lock().insert(pid, label_str);
        Ok(())
    }

    pub fn set_object_label(&self, inode: u64, label: &str) -> Result<(), &'static str> {
        let label_str = String::from_str(label).map_err(|_| "Label too long")?;
        self.object_labels.lock().insert(inode, label_str);
        Ok(())
    }

    pub fn add_policy(&self, subject: &str, object: &str, action: MacAction) -> Result<(), &'static str> {
        let subject_str = String::from_str(subject).map_err(|_| "Subject too long")?;
        let object_str = String::from_str(object).map_err(|_| "Object too long")?;
        
        let policy = MacPolicy {
            subject: subject_str.clone(),
            object: object_str.clone(),
            action,
        };
        
        self.policies.lock().insert((subject_str, object_str), policy);
        Ok(())
    }

    pub fn check_access(&self, pid: ProcessId, inode: u64, operation: &str) -> bool {
        let process_label = self.process_labels.lock().get(&pid).cloned();
        let object_label = self.object_labels.lock().get(&inode).cloned();
        
        if let (Some(subject), Some(object)) = (process_label, object_label) {
            if let Some(policy) = self.policies.lock().get(&(subject, object)) {
                match policy.action {
                    MacAction::Allow => return true,
                    MacAction::Deny => return false,
                    MacAction::Audit => {
                        crate::security::audit::AUDIT_LOGGER.log(
                            crate::security::audit::AuditEventType::SecurityViolation,
                            Some(pid),
                            &alloc::format!("MAC audit: {} on {}", operation, object.as_str()),
                        );
                        return true;
                    }
                }
            }
        }
        
        // Default deny
        false
    }
}

pub static MAC: MandatoryAccessControl = MandatoryAccessControl::new();

