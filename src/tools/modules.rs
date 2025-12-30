use crate::security::secure_boot::SECURE_BOOT;
use spin::Mutex;
use alloc::collections::BTreeMap;
use heapless::String;

pub struct KernelModule {
    pub name: String<64>,
    pub address: u64,
    pub size: usize,
    pub loaded: bool,
    pub signature: alloc::vec::Vec<u8>,
}

pub struct ModuleManager {
    modules: Mutex<BTreeMap<String<64>, KernelModule>>,
}

impl ModuleManager {
    pub const fn new() -> Self {
        ModuleManager {
            modules: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn load_module(&self, name: &str, data: &[u8], signature: &[u8]) -> Result<(), &'static str> {
        let name_str = String::from_str(name).map_err(|_| "Name too long")?;
        
        // Verify signature
        if SECURE_BOOT.is_enabled() {
            SECURE_BOOT.verify_module(data, signature)?;
        }
        
        // TODO: Actually load module into memory
        let module = KernelModule {
            name: name_str.clone(),
            address: 0,
            size: data.len(),
            loaded: true,
            signature: signature.to_vec(),
        };
        
        self.modules.lock().insert(name_str, module);
        crate::io::println!("Module {} loaded", name);
        Ok(())
    }

    pub fn unload_module(&self, name: &str) -> Result<(), &'static str> {
        let name_str = String::from_str(name).map_err(|_| "Name too long")?;
        let module = self.modules.lock().remove(&name_str).ok_or("Module not found")?;
        
        // TODO: Actually unload module
        crate::io::println!("Module {} unloaded", name);
        Ok(())
    }

    pub fn list_modules(&self) -> alloc::vec::Vec<String<64>> {
        self.modules.lock().keys().cloned().collect()
    }
}

pub static MODULE_MANAGER: ModuleManager = ModuleManager::new();

