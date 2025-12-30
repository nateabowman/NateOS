use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType {
    Pid,
    Network,
    Mount,
    Uts,
    Ipc,
    User,
}

pub struct Namespace {
    pub ns_type: NamespaceType,
    pub id: u64,
    pub processes: alloc::vec::Vec<ProcessId>,
}

pub struct NamespaceManager {
    namespaces: Mutex<BTreeMap<(NamespaceType, u64), Namespace>>,
    process_namespaces: Mutex<BTreeMap<ProcessId, BTreeMap<NamespaceType, u64>>>,
    next_ns_id: Mutex<u64>,
}

impl NamespaceManager {
    pub const fn new() -> Self {
        NamespaceManager {
            namespaces: Mutex::new(BTreeMap::new()),
            process_namespaces: Mutex::new(BTreeMap::new()),
            next_ns_id: Mutex::new(1),
        }
    }

    pub fn create_namespace(&self, ns_type: NamespaceType) -> u64 {
        let ns_id = {
            let mut next = self.next_ns_id.lock();
            let id = *next;
            *next += 1;
            id
        };
        
        let namespace = Namespace {
            ns_type,
            id: ns_id,
            processes: alloc::vec::Vec::new(),
        };
        
        self.namespaces.lock().insert((ns_type, ns_id), namespace);
        ns_id
    }

    pub fn join_namespace(&self, pid: ProcessId, ns_type: NamespaceType, ns_id: u64) -> Result<(), &'static str> {
        let mut namespaces = self.namespaces.lock();
        let namespace = namespaces.get_mut(&(ns_type, ns_id)).ok_or("Namespace not found")?;
        namespace.processes.push(pid);
        
        let mut proc_ns = self.process_namespaces.lock();
        proc_ns.entry(pid)
            .or_insert_with(BTreeMap::new)
            .insert(ns_type, ns_id);
        
        Ok(())
    }

    pub fn get_namespace(&self, pid: ProcessId, ns_type: NamespaceType) -> Option<u64> {
        self.process_namespaces.lock()
            .get(&pid)
            .and_then(|ns_map| ns_map.get(&ns_type).copied())
    }

    pub fn isolate_process(&self, pid: ProcessId) -> Result<(), &'static str> {
        // Create all namespaces for process isolation
        for ns_type in [NamespaceType::Pid, NamespaceType::Network, NamespaceType::Mount,
                        NamespaceType::Uts, NamespaceType::Ipc, NamespaceType::User] {
            let ns_id = self.create_namespace(ns_type);
            self.join_namespace(pid, ns_type, ns_id)?;
        }
        Ok(())
    }
}

pub static NAMESPACE_MANAGER: NamespaceManager = NamespaceManager::new();

