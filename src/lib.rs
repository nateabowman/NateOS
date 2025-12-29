#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod boot;
pub mod interrupts;
pub mod memory;
pub mod io;
pub mod process;
pub mod scheduler;
pub mod syscall;
pub mod ipc;
pub mod sync;
pub mod timer;
pub mod alloc as allocator;
pub mod drivers;
pub mod fs;
pub mod security;
pub mod net;
pub mod userspace;
pub mod performance;
pub mod testing;
pub mod stability;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

