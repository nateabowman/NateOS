use crate::process::PROCESS_MANAGER;
use crate::scheduler::SCHEDULER;
use crate::fs::FILESYSTEM;
use x86_64::VirtAddr;

pub struct StressTester;

impl StressTester {
    pub fn run_memory_stress(&self) {
        crate::io::println!("Running memory stress test...");
        
        // Create many processes
        for i in 0..100 {
            let entry = VirtAddr::new(0x400000 + (i * 0x1000));
            let stack = VirtAddr::new(0x800000 + (i * 0x1000));
            PROCESS_MANAGER.create_process(entry, stack);
        }
        
        crate::io::println!("Created 100 processes");
    }

    pub fn run_scheduler_stress(&self) {
        crate::io::println!("Running scheduler stress test...");
        
        // Create processes and add to scheduler
        for i in 0..50 {
            let entry = VirtAddr::new(0x400000 + (i * 0x1000));
            let stack = VirtAddr::new(0x800000 + (i * 0x1000));
            let pid = PROCESS_MANAGER.create_process(entry, stack);
            SCHEDULER.enqueue(pid);
        }
        
        // Run scheduler
        for _ in 0..1000 {
            SCHEDULER.schedule_next();
        }
        
        crate::io::println!("Scheduler stress test completed");
    }

    pub fn run_filesystem_stress(&self) {
        crate::io::println!("Running filesystem stress test...");
        
        let fs = FILESYSTEM.lock();
        
        // Create many files
        for i in 0..100 {
            let filename = heapless::String::<64>::from_str(&alloc::format!("test{}", i)).unwrap_or(heapless::String::new());
            fs.create_file(filename.as_str(), crate::fs::FileType::Regular).ok();
        }
        
        crate::io::println!("Created 100 files");
    }

    pub fn run_all(&self) {
        self.run_memory_stress();
        self.run_scheduler_stress();
        self.run_filesystem_stress();
        crate::io::println!("All stress tests completed");
    }
}

pub static STRESS_TESTER: StressTester = StressTester;

