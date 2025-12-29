// Simple heap allocator for kernel use
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: Mutex<usize>,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_end: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end,
            next: Mutex::new(heap_start),
        }
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut next = self.next.lock();
        let alloc_start = (*next + layout.align() - 1) & !(layout.align() - 1);
        let alloc_end = alloc_start + layout.size();

        if alloc_end > self.heap_end {
            null_mut()
        } else {
            *next = alloc_end;
            alloc_start as *mut u8
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't deallocate
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x_4444_4444_0000, 0x_4444_4444_4000);

pub mod collections {
    pub use alloc::collections::*;
}

