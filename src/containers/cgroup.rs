use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub cpu_quota: Option<u64>,      // CPU time in microseconds
    pub cpu_period: Option<u64>,     // CPU period in microseconds
    pub memory_limit: Option<u64>,   // Memory limit in bytes
    pub memory_swap: Option<u64>,    // Swap limit in bytes
    pub pids_limit: Option<u32>,     // Maximum number of processes
}

impl ResourceLimits {
    pub fn new() -> Self {
        ResourceLimits {
            cpu_quota: None,
            cpu_period: None,
            memory_limit: None,
            memory_swap: None,
            pids_limit: None,
        }
    }
}

pub struct Cgroup {
    pub name: heapless::String<64>,
    pub limits: ResourceLimits,
    pub processes: alloc::vec::Vec<ProcessId>,
}

pub struct CgroupManager {
    cgroups: Mutex<BTreeMap<heapless::String<64>, Cgroup>>,
}

impl CgroupManager {
    pub const fn new() -> Self {
        CgroupManager {
            cgroups: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn create_cgroup(&self, name: &str) -> Result<(), &'static str> {
        let name_str = heapless::String::from_str(name).map_err(|_| "Name too long")?;
        
        if self.cgroups.lock().contains_key(&name_str) {
            return Err("Cgroup already exists");
        }
        
        let cgroup = Cgroup {
            name: name_str.clone(),
            limits: ResourceLimits::new(),
            processes: alloc::vec::Vec::new(),
        };
        
        self.cgroups.lock().insert(name_str, cgroup);
        Ok(())
    }

    pub fn add_process(&self, cgroup_name: &str, pid: ProcessId) -> Result<(), &'static str> {
        let name_str = heapless::String::from_str(cgroup_name).map_err(|_| "Name too long")?;
        let mut cgroups = self.cgroups.lock();
        let cgroup = cgroups.get_mut(&name_str).ok_or("Cgroup not found")?;
        cgroup.processes.push(pid);
        Ok(())
    }

    pub fn set_limits(&self, cgroup_name: &str, limits: ResourceLimits) -> Result<(), &'static str> {
        let name_str = heapless::String::from_str(cgroup_name).map_err(|_| "Name too long")?;
        let mut cgroups = self.cgroups.lock();
        let cgroup = cgroups.get_mut(&name_str).ok_or("Cgroup not found")?;
        cgroup.limits = limits;
        Ok(())
    }

    pub fn check_limits(&self, cgroup_name: &str, pid: ProcessId) -> bool {
        let name_str = heapless::String::from_str(cgroup_name).unwrap_or(heapless::String::new());
        if let Some(cgroup) = self.cgroups.lock().get(&name_str) {
            // TODO: Check if process exceeds limits
            true
        } else {
            true
        }
    }
}

pub static CGROUP_MANAGER: CgroupManager = CgroupManager::new();

