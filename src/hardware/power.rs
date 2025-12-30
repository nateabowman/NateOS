use spin::Mutex;
use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    S0,  // Working
    S1,  // Standby
    S2,  // Standby
    S3,  // Suspend to RAM
    S4,  // Suspend to disk
    S5,  // Shutdown
}

static CURRENT_POWER_STATE: AtomicU8 = AtomicU8::new(0); // S0

pub struct PowerManager {
    cpu_freq_mhz: Mutex<u32>,
    enabled: Mutex<bool>,
}

impl PowerManager {
    pub const fn new() -> Self {
        PowerManager {
            cpu_freq_mhz: Mutex::new(0),
            enabled: Mutex::new(true),
        }
    }

    pub fn init(&self) {
        // TODO: Initialize power management
        crate::io::println!("Power: Power management initialized");
    }

    pub fn set_cpu_frequency(&self, freq_mhz: u32) -> Result<(), &'static str> {
        if !*self.enabled.lock() {
            return Err("Power management disabled");
        }
        
        // TODO: Actually set CPU frequency
        *self.cpu_freq_mhz.lock() = freq_mhz;
        Ok(())
    }

    pub fn get_cpu_frequency(&self) -> u32 {
        *self.cpu_freq_mhz.lock()
    }

    pub fn enter_sleep_state(&self, state: PowerState) -> Result<(), &'static str> {
        CURRENT_POWER_STATE.store(state as u8, Ordering::Relaxed);
        
        match state {
            PowerState::S3 => {
                // TODO: Suspend to RAM
                crate::io::println!("Power: Entering S3 (Suspend to RAM)");
            }
            PowerState::S4 => {
                // TODO: Suspend to disk
                crate::io::println!("Power: Entering S4 (Suspend to disk)");
            }
            PowerState::S5 => {
                // TODO: Shutdown
                crate::io::println!("Power: Shutting down");
            }
            _ => {}
        }
        
        Ok(())
    }

    pub fn get_current_state(&self) -> PowerState {
        match CURRENT_POWER_STATE.load(Ordering::Relaxed) {
            0 => PowerState::S0,
            1 => PowerState::S1,
            2 => PowerState::S2,
            3 => PowerState::S3,
            4 => PowerState::S4,
            5 => PowerState::S5,
            _ => PowerState::S0,
        }
    }
}

pub static POWER_MANAGER: PowerManager = PowerManager::new();

