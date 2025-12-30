use crate::containers::namespace::{NamespaceManager, NamespaceType};
use crate::containers::cgroup::{CgroupManager, ResourceLimits};
use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

pub struct Container {
    pub container_id: u64,
    pub name: heapless::String<64>,
    pub pid: ProcessId,
    pub namespace_ids: BTreeMap<NamespaceType, u64>,
    pub cgroup_name: heapless::String<64>,
    pub image: heapless::String<256>,
}

pub struct ContainerRuntime {
    containers: Mutex<BTreeMap<u64, Container>>,
    next_container_id: Mutex<u64>,
}

impl ContainerRuntime {
    pub const fn new() -> Self {
        ContainerRuntime {
            containers: Mutex::new(BTreeMap::new()),
            next_container_id: Mutex::new(1),
        }
    }

    pub fn create_container(&self, name: &str, image: &str) -> Result<u64, &'static str> {
        let container_id = {
            let mut next = self.next_container_id.lock();
            let id = *next;
            *next += 1;
            id
        };
        
        let name_str = heapless::String::from_str(name).map_err(|_| "Name too long")?;
        let image_str = heapless::String::from_str(image).map_err(|_| "Image name too long")?;
        let cgroup_name = heapless::String::from_str(name).map_err(|_| "Cgroup name too long")?;
        
        // Create namespaces
        let mut namespace_ids = BTreeMap::new();
        for ns_type in [NamespaceType::Pid, NamespaceType::Network, NamespaceType::Mount,
                        NamespaceType::Uts, NamespaceType::Ipc, NamespaceType::User] {
            let ns_id = NAMESPACE_MANAGER.create_namespace(ns_type);
            namespace_ids.insert(ns_type, ns_id);
        }
        
        // Create cgroup
        CGROUP_MANAGER.create_cgroup(name)?;
        
        // Create dummy process (would create actual process in real implementation)
        let pid = crate::process::PROCESS_MANAGER.create_process(
            x86_64::VirtAddr::new(0x400000),
            x86_64::VirtAddr::new(0x800000),
        );
        
        // Join namespaces
        for (ns_type, ns_id) in namespace_ids.iter() {
            NAMESPACE_MANAGER.join_namespace(pid, *ns_type, *ns_id)?;
        }
        
        // Add to cgroup
        CGROUP_MANAGER.add_process(name, pid)?;
        
        let container = Container {
            container_id,
            name: name_str,
            pid,
            namespace_ids,
            cgroup_name,
            image: image_str,
        };
        
        self.containers.lock().insert(container_id, container);
        Ok(container_id)
    }

    pub fn start_container(&self, container_id: u64) -> Result<(), &'static str> {
        let container = self.containers.lock().get(&container_id).ok_or("Container not found")?;
        // TODO: Actually start the container process
        crate::io::println!("Container {} started", container.name.as_str());
        Ok(())
    }

    pub fn stop_container(&self, container_id: u64) -> Result<(), &'static str> {
        let container = self.containers.lock().get(&container_id).ok_or("Container not found")?;
        // TODO: Stop the container process
        crate::io::println!("Container {} stopped", container.name.as_str());
        Ok(())
    }

    pub fn get_container(&self, container_id: u64) -> Option<Container> {
        self.containers.lock().get(&container_id).cloned()
    }
}

pub static CONTAINER_RUNTIME: ContainerRuntime = ContainerRuntime::new();

