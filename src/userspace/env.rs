use spin::Mutex;
use alloc::collections::BTreeMap;
use crate::process::ProcessId;
use heapless::String;

pub struct Environment {
    vars: BTreeMap<heapless::String<64>, heapless::String<256>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let key_str = heapless::String::from_str(key).map_err(|_| "Key too long")?;
        let value_str = heapless::String::from_str(value).map_err(|_| "Value too long")?;
        self.vars.insert(key_str, value_str);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&heapless::String<256>> {
        let key_str = heapless::String::from_str(key).ok()?;
        self.vars.get(&key_str)
    }

    pub fn unset(&mut self, key: &str) {
        if let Ok(key_str) = heapless::String::from_str(key) {
            self.vars.remove(&key_str);
        }
    }
}

pub struct EnvironmentManager {
    process_envs: Mutex<alloc::collections::BTreeMap<ProcessId, Environment>>,
}

impl EnvironmentManager {
    pub const fn new() -> Self {
        EnvironmentManager {
            process_envs: Mutex::new(alloc::collections::BTreeMap::new()),
        }
    }

    pub fn get_env(&self, pid: ProcessId) -> Environment {
        self.process_envs.lock()
            .get(&pid)
            .cloned()
            .unwrap_or_else(Environment::new)
    }

    pub fn set_env(&self, pid: ProcessId, env: Environment) {
        self.process_envs.lock().insert(pid, env);
    }
}

pub static ENV_MANAGER: EnvironmentManager = EnvironmentManager::new();

