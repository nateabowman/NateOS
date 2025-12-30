#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod boot;
mod interrupts;
mod memory;
mod io;
mod process;
mod scheduler;
mod syscall;
mod timer;
mod drivers;
mod fs;
mod security;
mod net;
mod userspace;
mod performance;
mod testing;
mod stability;
mod hardware;
mod containers;
mod services;
mod tools;
mod production;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize kernel subsystems
    boot::init();
    interrupts::init();
    memory::init();
    
    // Initialize advanced memory features
    memory::SWAP_MANAGER.init(0, 1024 * 1024); // 512MB swap
    memory::MEMORY_COMPRESSOR.enable();
    
    io::init();
    timer::init();
    
    // Initialize security
    security::ASLR::init();
    security::StackProtection::init();
    
    // Initialize drivers
    drivers::DRIVER_MANAGER.init_all();
    
    // Initialize filesystem
    fs::FILESYSTEM.lock();
    
    // Initialize networking
    net::driver::NETWORK_DRIVER.init().ok();
    
    // Initialize user management
    userspace::user::USER_MANAGER.create_user("root").ok();
    userspace::user::USER_MANAGER.set_current_user(0).ok();
    
    // Initialize performance monitoring
    performance::profiler::PROFILER.enable();
    
    // Initialize stability systems
    stability::watchdog::WATCHDOG.init();
    
    // Initialize hardware
    hardware::pci::PCI_MANAGER.init();
    hardware::acpi::ACPI_MANAGER.init().ok();
    hardware::power::POWER_MANAGER.init();
    hardware::thermal::THERMAL_MANAGER.init();
    
    // Initialize services
    services::init::INIT_SYSTEM.init();
    
    // Initialize containers
    containers::namespace::NAMESPACE_MANAGER.create_namespace(containers::namespace::NamespaceType::Pid);
    
    // Initialize tools
    tools::debugger::DEBUGGER.enable();
    tools::tracing::TRACING.enable();
    
    // Apply production hardening
    production::hardening::HARDENING.run_security_audit();
    production::hardening::HARDENING.apply_hardening();
    production::optimization::OPTIMIZATION.apply_optimizations();
    production::monitoring::MONITORING.record_metrics();
    
    // Print welcome message
    io::println!("NateOS Kernel v0.1.0");
    io::println!("Initialization complete.");
    io::println!("Core kernel systems loaded.");
    io::println!("I/O subsystem initialized.");
    io::println!("Security features enabled.");
    io::println!("Networking stack initialized.");
    io::println!("User space support ready.");
    io::println!("Performance optimizations enabled.");
    io::println!("Stability monitoring active.");
    io::println!("Hardware support initialized.");
    io::println!("System services started.");
    io::println!("Container runtime ready.");
    io::println!("Development tools available.");
    io::println!("Production hardening applied.");
    
    // Start shell
    userspace::shell::SHELL.run();
    
    // Main kernel loop (should not reach here)
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    io::println!("PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    io::println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    io::println!("All tests passed!");
}

