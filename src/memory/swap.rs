use spin::Mutex;
use alloc::collections::BTreeMap;
use x86_64::{PhysAddr, VirtAddr};
use crate::fs::block::BLOCK_DEVICE;

pub struct SwapEntry {
    pub page_vaddr: VirtAddr,
    pub swap_slot: u64,
    pub dirty: bool,
}

pub struct SwapManager {
    entries: Mutex<BTreeMap<VirtAddr, SwapEntry>>,
    swap_device_start: u64,
    swap_size_blocks: u64,
    next_slot: Mutex<u64>,
}

impl SwapManager {
    pub const fn new() -> Self {
        SwapManager {
            entries: Mutex::new(BTreeMap::new()),
            swap_device_start: 0,
            swap_size_blocks: 0,
            next_slot: Mutex::new(0),
        }
    }

    pub fn init(&self, swap_start: u64, swap_size_blocks: u64) {
        let mut manager = SwapManager {
            entries: Mutex::new(BTreeMap::new()),
            swap_device_start: swap_start,
            swap_size_blocks,
            next_slot: Mutex::new(0),
        };
        *self = manager;
    }

    pub fn swap_out(&self, vaddr: VirtAddr, page_data: &[u8; 4096]) -> Result<u64, &'static str> {
        let slot = {
            let mut next = self.next_slot.lock();
            if *next >= self.swap_size_blocks {
                return Err("Swap space full");
            }
            let slot = *next;
            *next += 1;
            slot
        };

        let block_num = self.swap_device_start + slot;
        BLOCK_DEVICE.write_block(block_num, page_data)?;

        let entry = SwapEntry {
            page_vaddr: vaddr,
            swap_slot: slot,
            dirty: true,
        };
        self.entries.lock().insert(vaddr, entry);

        Ok(slot)
    }

    pub fn swap_in(&self, vaddr: VirtAddr) -> Result<[u8; 4096], &'static str> {
        let entry = self.entries.lock().remove(&vaddr)
            .ok_or("Page not in swap")?;

        let block_num = self.swap_device_start + entry.swap_slot;
        let mut page_data = [0u8; 4096];
        BLOCK_DEVICE.read_block(block_num, &mut page_data)?;

        Ok(page_data)
    }

    pub fn is_swapped(&self, vaddr: VirtAddr) -> bool {
        self.entries.lock().contains_key(&vaddr)
    }

    pub fn get_swap_usage(&self) -> (u64, u64) {
        let used = self.entries.lock().len() as u64;
        (used, self.swap_size_blocks)
    }
}

pub static SWAP_MANAGER: SwapManager = SwapManager::new();

