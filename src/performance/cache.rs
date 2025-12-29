use spin::Mutex;
use alloc::collections::BTreeMap;

pub struct CacheEntry<T> {
    data: T,
    last_accessed: u64,
    access_count: u64,
}

pub struct LRUCache<K, V> {
    entries: Mutex<BTreeMap<K, CacheEntry<V>>>,
    max_size: usize,
}

impl<K: core::cmp::Ord, V> LRUCache<K, V> {
    pub fn new(max_size: usize) -> Self {
        LRUCache {
            entries: Mutex::new(BTreeMap::new()),
            max_size,
        }
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let mut entries = self.entries.lock();
        if let Some(entry) = entries.get_mut(key) {
            entry.last_accessed = crate::timer::get_time_ms();
            entry.access_count += 1;
            Some(entry.data.clone())
        } else {
            None
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut entries = self.entries.lock();
        
        // Evict if necessary
        if entries.len() >= self.max_size && !entries.contains_key(&key) {
            // Find LRU entry
            let mut lru_key = None;
            let mut lru_time = u64::MAX;
            
            for (k, entry) in entries.iter() {
                if entry.last_accessed < lru_time {
                    lru_time = entry.last_accessed;
                    lru_key = Some(k.clone());
                }
            }
            
            if let Some(key_to_remove) = lru_key {
                entries.remove(&key_to_remove);
            }
        }
        
        entries.insert(key, CacheEntry {
            data: value,
            last_accessed: crate::timer::get_time_ms(),
            access_count: 0,
        });
    }

    pub fn clear(&self) {
        self.entries.lock().clear();
    }
}

pub struct BlockCache {
    cache: LRUCache<u64, [u8; 512]>,
}

impl BlockCache {
    pub const fn new() -> Self {
        BlockCache {
            cache: LRUCache::new(1024), // Cache 1024 blocks
        }
    }

    pub fn get_block(&self, block_number: u64) -> Option<[u8; 512]> {
        self.cache.get(&block_number)
    }

    pub fn cache_block(&self, block_number: u64, data: [u8; 512]) {
        self.cache.insert(block_number, data);
    }

    pub fn invalidate_block(&self, block_number: u64) {
        // TODO: Implement block invalidation
    }
}

pub static BLOCK_CACHE: BlockCache = BlockCache::new();

