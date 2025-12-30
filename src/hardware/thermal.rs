use spin::Mutex;
use core::sync::atomic::{AtomicU8, Ordering};

static CPU_TEMPERATURE: AtomicU8 = AtomicU8::new(50); // 50°C default
static THROTTLE_THRESHOLD: AtomicU8 = AtomicU8::new(90); // 90°C
static CRITICAL_THRESHOLD: AtomicU8 = AtomicU8::new(100); // 100°C

pub struct ThermalManager {
    enabled: Mutex<bool>,
}

impl ThermalManager {
    pub const fn new() -> Self {
        ThermalManager {
            enabled: Mutex::new(true),
        }
    }

    pub fn init(&self) {
        crate::io::println!("Thermal: Thermal management initialized");
    }

    pub fn get_cpu_temperature(&self) -> u8 {
        CPU_TEMPERATURE.load(Ordering::Relaxed)
    }

    pub fn set_cpu_temperature(&self, temp: u8) {
        CPU_TEMPERATURE.store(temp, Ordering::Relaxed);
        self.check_throttle();
    }

    fn check_throttle(&self) {
        let temp = self.get_cpu_temperature();
        let throttle = THROTTLE_THRESHOLD.load(Ordering::Relaxed);
        let critical = CRITICAL_THRESHOLD.load(Ordering::Relaxed);
        
        if temp >= critical {
            crate::io::println!("Thermal: CRITICAL temperature! {}", temp);
            // TODO: Emergency shutdown or heavy throttling
        } else if temp >= throttle {
            crate::io::println!("Thermal: High temperature, throttling CPU");
            // TODO: Throttle CPU
            if let Err(e) = crate::hardware::power::POWER_MANAGER.set_cpu_frequency(1000) {
                crate::io::println!("Thermal: Failed to throttle: {}", e);
            }
        }
    }

    pub fn set_throttle_threshold(&self, threshold: u8) {
        THROTTLE_THRESHOLD.store(threshold, Ordering::Relaxed);
    }

    pub fn set_critical_threshold(&self, threshold: u8) {
        CRITICAL_THRESHOLD.store(threshold, Ordering::Relaxed);
    }
}

pub static THERMAL_MANAGER: ThermalManager = ThermalManager::new();

