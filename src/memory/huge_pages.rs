use x86_64::structures::paging::{PageSize, Size2MiB, Size1GiB, PhysFrame};
use x86_64::PhysAddr;

pub enum HugePageSize {
    Size2MiB,
    Size1GiB,
}

pub struct HugePageAllocator {
    free_2mb: spin::Mutex<alloc::vec::Vec<PhysFrame<Size2MiB>>>,
    free_1gb: spin::Mutex<alloc::vec::Vec<PhysFrame<Size1GiB>>>,
}

impl HugePageAllocator {
    pub const fn new() -> Self {
        HugePageAllocator {
            free_2mb: spin::Mutex::new(alloc::vec::Vec::new()),
            free_1gb: spin::Mutex::new(alloc::vec::Vec::new()),
        }
    }

    pub fn allocate_2mb(&self) -> Option<PhysFrame<Size2MiB>> {
        self.free_2mb.lock().pop()
    }

    pub fn allocate_1gb(&self) -> Option<PhysFrame<Size1GiB>> {
        self.free_1gb.lock().pop()
    }

    pub fn free_2mb(&self, frame: PhysFrame<Size2MiB>) {
        self.free_2mb.lock().push(frame);
    }

    pub fn free_1gb(&self, frame: PhysFrame<Size1GiB>) {
        self.free_1gb.lock().push(frame);
    }

    pub fn add_2mb_frame(&self, frame: PhysFrame<Size2MiB>) {
        self.free_2mb.lock().push(frame);
    }

    pub fn add_1gb_frame(&self, frame: PhysFrame<Size1GiB>) {
        self.free_1gb.lock().push(frame);
    }
}

pub static HUGE_PAGE_ALLOCATOR: HugePageAllocator = HugePageAllocator::new();

