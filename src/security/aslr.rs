use x86_64::VirtAddr;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::random::SecureRandom;

static ASLR_ENABLED: AtomicU64 = AtomicU64::new(0);
static BASE_OFFSET: AtomicU64 = AtomicU64::new(0);

pub struct ASLR;

impl ASLR {
    pub fn init() {
        // Generate random base offset for ASLR
        let mut rng = SecureRandom::new();
        let offset = rng.next_u64() & 0x0000_FFFF_FFFF_0000; // Align to 64KB
        BASE_OFFSET.store(offset, Ordering::Relaxed);
        ASLR_ENABLED.store(1, Ordering::Relaxed);
        crate::io::println!("ASLR initialized with offset: 0x{:x}", offset);
    }

    pub fn is_enabled() -> bool {
        ASLR_ENABLED.load(Ordering::Relaxed) != 0
    }

    pub fn randomize_address(base: VirtAddr) -> VirtAddr {
        if Self::is_enabled() {
            let offset = BASE_OFFSET.load(Ordering::Relaxed);
            let mut rng = SecureRandom::new();
            let random_offset = (rng.next_u64() % 0x10000) & 0xFFF; // 4KB alignment
            VirtAddr::new(base.as_u64() + offset + random_offset)
        } else {
            base
        }
    }

    pub fn get_base_offset() -> u64 {
        BASE_OFFSET.load(Ordering::Relaxed)
    }
}

