use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::instructions::port::Port;

const PIT_FREQUENCY: u64 = 1193182;
const TARGET_FREQUENCY: u64 = 100; // 100 Hz

static TICKS: AtomicU64 = AtomicU64::new(0);
static TIME_MS: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let divisor = (PIT_FREQUENCY / TARGET_FREQUENCY) as u16;
    
    unsafe {
        // Channel 0, both bytes, mode 3 (square wave), binary
        let mut cmd: Port<u8> = Port::new(0x43);
        cmd.write(0x36);
        
        // Set divisor
        let mut data: Port<u8> = Port::new(0x40);
        data.write((divisor & 0xFF) as u8);
        data.write((divisor >> 8) as u8);
    }
}

pub fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    TIME_MS.fetch_add(10, Ordering::Relaxed); // 100 Hz = 10ms per tick
}

pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn get_time_ms() -> u64 {
    TIME_MS.load(Ordering::Relaxed)
}

pub fn sleep_ms(ms: u64) {
    let start = get_time_ms();
    while get_time_ms() - start < ms {
        x86_64::instructions::hlt();
    }
}

