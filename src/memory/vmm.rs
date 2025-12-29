use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    VirtAddr, PhysAddr,
};
use crate::memory::BootInfoFrameAllocator;

pub struct VirtualMemoryManager {
    mapper: Option<&'static mut PageTable>,
    frame_allocator: Option<&'static mut BootInfoFrameAllocator>,
}

impl VirtualMemoryManager {
    pub const fn new() -> Self {
        VirtualMemoryManager {
            mapper: None,
            frame_allocator: None,
        }
    }

    pub fn init(&mut self, mapper: &'static mut PageTable, frame_allocator: &'static mut BootInfoFrameAllocator) {
        self.mapper = Some(mapper);
        self.frame_allocator = Some(frame_allocator);
    }

    pub fn map_page(&mut self, page: Page, flags: PageTableFlags) -> Result<(), ()> {
        // TODO: Implement page mapping
        Ok(())
    }

    pub fn unmap_page(&mut self, page: Page) -> Result<(), ()> {
        // TODO: Implement page unmapping
        Ok(())
    }
}

