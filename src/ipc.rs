use spin::Mutex;
use alloc::collections::BTreeMap;
use crate::process::ProcessId;

pub struct Message {
    pub from: ProcessId,
    pub to: ProcessId,
    pub data: [u8; 256],
    pub size: usize,
}

pub struct MessageQueue {
    messages: Mutex<Vec<Message>>,
}

impl MessageQueue {
    pub const fn new() -> Self {
        MessageQueue {
            messages: Mutex::new(Vec::new()),
        }
    }

    pub fn send(&self, message: Message) {
        self.messages.lock().push(message);
    }

    pub fn receive(&self, pid: ProcessId) -> Option<Message> {
        let mut messages = self.messages.lock();
        let pos = messages.iter().position(|m| m.to == pid);
        if let Some(pos) = pos {
            Some(messages.remove(pos))
        } else {
            None
        }
    }
}

pub static MESSAGE_QUEUE: MessageQueue = MessageQueue::new();

pub struct SharedMemory {
    pub owner: ProcessId,
    pub size: usize,
    pub address: u64,
}

pub struct SharedMemoryManager {
    regions: Mutex<BTreeMap<u64, SharedMemory>>,
    next_id: Mutex<u64>,
}

impl SharedMemoryManager {
    pub const fn new() -> Self {
        SharedMemoryManager {
            regions: Mutex::new(BTreeMap::new()),
            next_id: Mutex::new(1),
        }
    }

    pub fn create(&self, owner: ProcessId, size: usize) -> u64 {
        let id = {
            let mut next = self.next_id.lock();
            let id = *next;
            *next += 1;
            id
        };
        // TODO: Actually allocate memory
        let region = SharedMemory {
            owner,
            size,
            address: 0,
        };
        self.regions.lock().insert(id, region);
        id
    }

    pub fn attach(&self, id: u64, pid: ProcessId) -> Option<u64> {
        self.regions.lock().get(&id).map(|r| r.address)
    }
}

pub static SHARED_MEMORY: SharedMemoryManager = SharedMemoryManager::new();

