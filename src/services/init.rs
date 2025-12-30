use crate::services::service::SERVICE_MANAGER;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runlevel {
    Halt = 0,
    SingleUser = 1,
    MultiUser = 2,
    MultiUserNetwork = 3,
    Unused = 4,
    X11 = 5,
    Reboot = 6,
}

pub struct InitSystem {
    current_runlevel: Mutex<Runlevel>,
    target_runlevel: Mutex<Runlevel>,
}

impl InitSystem {
    pub const fn new() -> Self {
        InitSystem {
            current_runlevel: Mutex::new(Runlevel::SingleUser),
            target_runlevel: Mutex::new(Runlevel::MultiUserNetwork),
        }
    }

    pub fn init(&self) {
        crate::io::println!("Init: Starting init system");
        
        // Set target runlevel
        *self.target_runlevel.lock() = Runlevel::MultiUserNetwork;
        
        // Start essential services
        self.start_essential_services();
        
        // Transition to target runlevel
        self.transition_to_runlevel(Runlevel::MultiUserNetwork);
        
        crate::io::println!("Init: System initialized");
    }

    fn start_essential_services(&self) {
        // Start syslog
        SERVICE_MANAGER.start_service("syslog").ok();
        
        // Start network services
        SERVICE_MANAGER.start_service("network").ok();
    }

    pub fn transition_to_runlevel(&self, runlevel: Runlevel) {
        *self.current_runlevel.lock() = runlevel;
        *self.target_runlevel.lock() = runlevel;
        
        match runlevel {
            Runlevel::Halt => {
                crate::io::println!("Init: Halting system");
            }
            Runlevel::Reboot => {
                crate::io::println!("Init: Rebooting system");
            }
            _ => {
                crate::io::println!("Init: Transitioned to runlevel {}", runlevel as u8);
            }
        }
    }

    pub fn get_current_runlevel(&self) -> Runlevel {
        *self.current_runlevel.lock()
    }
}

pub static INIT_SYSTEM: InitSystem = InitSystem::new();

