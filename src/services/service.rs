use spin::Mutex;
use alloc::collections::BTreeMap;
use heapless::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String<64>,
    pub state: ServiceState,
    pub pid: Option<crate::process::ProcessId>,
    pub dependencies: alloc::vec::Vec<String<64>>,
    pub restart_policy: RestartPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    Always,
    OnFailure,
}

pub struct ServiceManager {
    services: Mutex<BTreeMap<String<64>, Service>>,
}

impl ServiceManager {
    pub const fn new() -> Self {
        ServiceManager {
            services: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn register_service(&self, service: Service) -> Result<(), &'static str> {
        let name = service.name.clone();
        self.services.lock().insert(name, service);
        Ok(())
    }

    pub fn start_service(&self, name: &str) -> Result<(), &'static str> {
        let name_str = String::from_str(name).map_err(|_| "Name too long")?;
        let mut services = self.services.lock();
        let service = services.get_mut(&name_str).ok_or("Service not found")?;
        
        if service.state == ServiceState::Running {
            return Ok(());
        }
        
        service.state = ServiceState::Starting;
        
        // Start dependencies first
        for dep in &service.dependencies {
            self.start_service(dep.as_str())?;
        }
        
        // TODO: Actually start the service process
        service.state = ServiceState::Running;
        crate::io::println!("Service {} started", name);
        
        Ok(())
    }

    pub fn stop_service(&self, name: &str) -> Result<(), &'static str> {
        let name_str = String::from_str(name).map_err(|_| "Name too long")?;
        let mut services = self.services.lock();
        let service = services.get_mut(&name_str).ok_or("Service not found")?;
        
        service.state = ServiceState::Stopping;
        // TODO: Actually stop the service process
        service.state = ServiceState::Stopped;
        crate::io::println!("Service {} stopped", name);
        
        Ok(())
    }

    pub fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        let name_str = String::from_str(name).ok()?;
        self.services.lock().get(&name_str).map(|s| s.state)
    }
}

pub static SERVICE_MANAGER: ServiceManager = ServiceManager::new();

