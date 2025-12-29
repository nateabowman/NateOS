pub trait Driver {
    fn init(&mut self) -> Result<(), &'static str>;
    fn name(&self) -> &'static str;
    fn is_initialized(&self) -> bool;
}

pub struct DriverManager {
    drivers: spin::Mutex<Vec<Box<dyn Driver>>>,
}

impl DriverManager {
    pub const fn new() -> Self {
        DriverManager {
            drivers: spin::Mutex::new(Vec::new()),
        }
    }

    pub fn register(&self, driver: Box<dyn Driver>) {
        self.drivers.lock().push(driver);
    }

    pub fn init_all(&self) {
        let mut drivers = self.drivers.lock();
        for driver in drivers.iter_mut() {
            if let Err(e) = driver.init() {
                crate::io::println!("Failed to initialize driver {}: {}", driver.name(), e);
            }
        }
    }
}

pub static DRIVER_MANAGER: DriverManager = DriverManager::new();

