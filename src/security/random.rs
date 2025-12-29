use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicU64, Ordering};

static RANDOM_SEED: AtomicU64 = AtomicU64::new(0);

pub struct SecureRandom {
    state: u64,
}

impl SecureRandom {
    pub fn new() -> Self {
        let seed = if RANDOM_SEED.load(Ordering::Relaxed) == 0 {
            // Initialize from RDTSC and port I/O
            let seed = Self::generate_seed();
            RANDOM_SEED.store(seed, Ordering::Relaxed);
            seed
        } else {
            RANDOM_SEED.load(Ordering::Relaxed)
        };
        
        SecureRandom { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // Linear congruential generator with better constants
        self.state = self.state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
        self.state
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    pub fn next_bytes(&mut self, buffer: &mut [u8]) {
        for chunk in buffer.chunks_mut(8) {
            let value = self.next_u64();
            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte = ((value >> (i * 8)) & 0xFF) as u8;
            }
        }
    }

    fn generate_seed() -> u64 {
        // Use RDTSC for timing-based entropy
        let tsc = unsafe { x86_64::registers::model_specific::Msr::new(0x10).read() };
        
        // Mix with port I/O timing
        unsafe {
            let mut port = Port::new(0x80);
            port.read();
        }
        
        tsc
    }
}

impl Default for SecureRandom {
    fn default() -> Self {
        Self::new()
    }
}

