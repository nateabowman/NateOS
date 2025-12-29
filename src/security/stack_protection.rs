use core::sync::atomic::{AtomicU64, Ordering};

static STACK_CANARY: AtomicU64 = AtomicU64::new(0xDEADBEEFCAFEBABE);
static STACK_PROTECTION_ENABLED: AtomicU64 = AtomicU64::new(0);

pub struct StackProtection;

impl StackProtection {
    pub fn init() {
        // Generate random stack canary
        let canary = Self::generate_canary();
        STACK_CANARY.store(canary, Ordering::Relaxed);
        STACK_PROTECTION_ENABLED.store(1, Ordering::Relaxed);
        crate::io::println!("Stack protection initialized");
    }

    pub fn is_enabled() -> bool {
        STACK_PROTECTION_ENABLED.load(Ordering::Relaxed) != 0
    }

    pub fn get_canary() -> u64 {
        STACK_CANARY.load(Ordering::Relaxed)
    }

    pub fn check_canary(canary: u64) -> bool {
        if Self::is_enabled() {
            canary == STACK_CANARY.load(Ordering::Relaxed)
        } else {
            true
        }
    }

    fn generate_canary() -> u64 {
        // Use a combination of values for canary
        use crate::random::SecureRandom;
        let mut rng = SecureRandom::new();
        rng.next_u64()
    }
}

#[macro_export]
macro_rules! stack_guard {
    () => {
        {
            let canary = $crate::security::StackProtection::get_canary();
            let _guard = StackGuard::new(canary);
            _guard
        }
    };
}

pub struct StackGuard {
    canary: u64,
}

impl StackGuard {
    pub fn new(canary: u64) -> Self {
        StackGuard { canary }
    }
}

impl Drop for StackGuard {
    fn drop(&mut self) {
        if !StackProtection::check_canary(self.canary) {
            crate::io::println!("Stack overflow detected!");
            panic!("Stack canary mismatch - possible buffer overflow");
        }
    }
}

