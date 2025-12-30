use spin::Mutex;
use alloc::collections::BTreeMap;
use x86_64::VirtAddr;

pub struct CompressedPage {
    pub original_vaddr: VirtAddr,
    pub compressed_data: heapless::Vec<u8, 2048>,
    pub original_size: usize,
}

pub struct MemoryCompressor {
    compressed_pages: Mutex<BTreeMap<VirtAddr, CompressedPage>>,
    enabled: Mutex<bool>,
}

impl MemoryCompressor {
    pub const fn new() -> Self {
        MemoryCompressor {
            compressed_pages: Mutex::new(BTreeMap::new()),
            enabled: Mutex::new(false),
        }
    }

    pub fn compress_page(&self, vaddr: VirtAddr, data: &[u8; 4096]) -> Result<(), &'static str> {
        if !*self.enabled.lock() {
            return Err("Compression disabled");
        }

        // Simple compression: remove zero pages
        let mut has_data = false;
        for &byte in data.iter() {
            if byte != 0 {
                has_data = true;
                break;
            }
        }

        if !has_data {
            // Zero page - store minimal data
            let compressed = CompressedPage {
                original_vaddr: vaddr,
                compressed_data: heapless::Vec::new(),
                original_size: 4096,
            };
            self.compressed_pages.lock().insert(vaddr, compressed);
            return Ok(());
        }

        // TODO: Implement actual compression algorithm (LZ4, zlib, etc.)
        // For now, just store the data
        let mut compressed_data = heapless::Vec::new();
        compressed_data.extend_from_slice(data).map_err(|_| "Compression buffer full")?;

        let compressed = CompressedPage {
            original_vaddr: vaddr,
            compressed_data,
            original_size: 4096,
        };
        self.compressed_pages.lock().insert(vaddr, compressed);

        Ok(())
    }

    pub fn decompress_page(&self, vaddr: VirtAddr) -> Result<[u8; 4096], &'static str> {
        let compressed = self.compressed_pages.lock().remove(&vaddr)
            .ok_or("Page not compressed")?;

        if compressed.compressed_data.is_empty() {
            // Zero page
            return Ok([0u8; 4096]);
        }

        // TODO: Implement actual decompression
        let mut page = [0u8; 4096];
        let copy_len = core::cmp::min(compressed.compressed_data.len(), 4096);
        page[..copy_len].copy_from_slice(&compressed.compressed_data[..copy_len]);

        Ok(page)
    }

    pub fn is_compressed(&self, vaddr: VirtAddr) -> bool {
        self.compressed_pages.lock().contains_key(&vaddr)
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static MEMORY_COMPRESSOR: MemoryCompressor = MemoryCompressor::new();

