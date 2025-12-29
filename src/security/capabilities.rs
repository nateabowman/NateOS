use bitflags::bitflags;
use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

bitflags! {
    pub struct Capabilities: u64 {
        const CAP_CHOWN = 1 << 0;
        const CAP_DAC_OVERRIDE = 1 << 1;
        const CAP_DAC_READ_SEARCH = 1 << 2;
        const CAP_FOWNER = 1 << 3;
        const CAP_FSETID = 1 << 4;
        const CAP_KILL = 1 << 5;
        const CAP_SETGID = 1 << 6;
        const CAP_SETUID = 1 << 7;
        const CAP_SETPCAP = 1 << 8;
        const CAP_LINUX_IMMUTABLE = 1 << 9;
        const CAP_NET_BIND_SERVICE = 1 << 10;
        const CAP_NET_BROADCAST = 1 << 11;
        const CAP_NET_ADMIN = 1 << 12;
        const CAP_NET_RAW = 1 << 13;
        const CAP_IPC_LOCK = 1 << 14;
        const CAP_IPC_OWNER = 1 << 15;
        const CAP_SYS_MODULE = 1 << 16;
        const CAP_SYS_RAWIO = 1 << 17;
        const CAP_SYS_CHROOT = 1 << 18;
        const CAP_SYS_PTRACE = 1 << 19;
        const CAP_SYS_PACCT = 1 << 20;
        const CAP_SYS_ADMIN = 1 << 21;
        const CAP_SYS_BOOT = 1 << 22;
        const CAP_SYS_NICE = 1 << 23;
        const CAP_SYS_RESOURCE = 1 << 24;
        const CAP_SYS_TIME = 1 << 25;
        const CAP_SYS_TTY_CONFIG = 1 << 26;
        const CAP_MKNOD = 1 << 27;
        const CAP_LEASE = 1 << 28;
        const CAP_AUDIT_WRITE = 1 << 29;
        const CAP_AUDIT_CONTROL = 1 << 30;
        const CAP_SETFCAP = 1 << 31;
    }
}

pub struct CapabilityManager {
    process_caps: Mutex<BTreeMap<ProcessId, Capabilities>>,
}

impl CapabilityManager {
    pub const fn new() -> Self {
        CapabilityManager {
            process_caps: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn set_capabilities(&self, pid: ProcessId, caps: Capabilities) {
        self.process_caps.lock().insert(pid, caps);
    }

    pub fn get_capabilities(&self, pid: ProcessId) -> Capabilities {
        self.process_caps.lock().get(&pid).copied().unwrap_or(Capabilities::empty())
    }

    pub fn has_capability(&self, pid: ProcessId, cap: Capabilities) -> bool {
        let caps = self.get_capabilities(pid);
        caps.contains(cap)
    }

    pub fn drop_capability(&self, pid: ProcessId, cap: Capabilities) {
        let mut caps = self.process_caps.lock();
        if let Some(process_caps) = caps.get_mut(&pid) {
            *process_caps &= !cap;
        }
    }
}

pub static CAPABILITY_MANAGER: CapabilityManager = CapabilityManager::new();

