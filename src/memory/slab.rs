use spin::Mutex;
use alloc::collections::BTreeMap;
use core::alloc::{GlobalAlloc, Layout};

pub struct SlabCache {
    object_size: usize,
    free_list: Mutex<alloc::vec::Vec<*mut u8>>,
    total_objects: Mutex<usize>,
}

impl SlabCache {
    pub fn new(object_size: usize) -> Self {
        SlabCache {
            object_size,
            free_list: Mutex::new(alloc::vec::Vec::new()),
            total_objects: Mutex::new(0),
        }
    }

    pub fn allocate(&self) -> Option<*mut u8> {
        let mut free = self.free_list.lock();
        if let Some(ptr) = free.pop() {
            return Some(ptr);
        }

        // Allocate new object
        // TODO: Implement actual allocation from page allocator
        None
    }

    pub fn deallocate(&self, ptr: *mut u8) {
        // Zero memory for security
        unsafe {
            core::ptr::write_bytes(ptr, 0, self.object_size);
        }
        self.free_list.lock().push(ptr);
    }
}

pub struct SlabAllocator {
    caches: Mutex<BTreeMap<usize, SlabCache>>,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        SlabAllocator {
            caches: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn get_cache(&self, size: usize) -> &SlabCache {
        let mut caches = self.caches.lock();
        caches.entry(size).or_insert_with(|| SlabCache::new(size))
    }

    pub fn allocate(&self, layout: Layout) -> *mut u8 {
        let cache = self.get_cache(layout.size());
        cache.allocate().unwrap_or(core::ptr::null_mut())
    }

    pub fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }
        let cache = self.get_cache(layout.size());
        cache.deallocate(ptr);
    }
}

pub static SLAB_ALLOCATOR: SlabAllocator = SlabAllocator::new();

