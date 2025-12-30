use spin::Mutex;
use alloc::collections::VecDeque;
use crate::fs::block::BLOCK_DEVICE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalEntryType {
    Create,
    Write,
    Delete,
    Rename,
}

pub struct JournalEntry {
    pub entry_type: JournalEntryType,
    pub inode: u64,
    pub block: u64,
    pub data: [u8; 512],
    pub sequence: u64,
}

pub struct Journal {
    entries: Mutex<VecDeque<JournalEntry, 1024>>,
    sequence: Mutex<u64>,
    journal_start_block: u64,
    enabled: Mutex<bool>,
}

impl Journal {
    pub const fn new() -> Self {
        Journal {
            entries: Mutex::new(VecDeque::new()),
            sequence: Mutex::new(0),
            journal_start_block: 0,
            enabled: Mutex::new(false),
        }
    }

    pub fn init(&self, journal_start: u64) {
        let mut journal = Journal {
            entries: Mutex::new(VecDeque::new()),
            sequence: Mutex::new(0),
            journal_start_block: journal_start,
            enabled: Mutex::new(true),
        };
        *self = journal;
    }

    pub fn log_entry(&self, entry: JournalEntry) -> Result<(), &'static str> {
        if !*self.enabled.lock() {
            return Ok(());
        }

        let seq = {
            let mut seq = self.sequence.lock();
            *seq += 1;
            *seq
        };

        let mut entry = entry;
        entry.sequence = seq;

        let mut entries = self.entries.lock();
        if entries.is_full() {
            // Flush oldest entries
            self.flush()?;
        }
        entries.push_back(entry).map_err(|_| "Journal full")?;

        Ok(())
    }

    pub fn flush(&self) -> Result<(), &'static str> {
        let mut entries = self.entries.lock();
        let mut block_offset = 0;

        for entry in entries.iter() {
            let block_num = self.journal_start_block + block_offset;
            BLOCK_DEVICE.write_block(block_num, &entry.data)?;
            block_offset += 1;
        }

        entries.clear();
        Ok(())
    }

    pub fn replay(&self) -> Result<(), &'static str> {
        // Replay journal entries after crash
        let entries = self.entries.lock();
        
        for entry in entries.iter() {
            match entry.entry_type {
                JournalEntryType::Write => {
                    BLOCK_DEVICE.write_block(entry.block, &entry.data)?;
                }
                _ => {
                    // Handle other entry types
                }
            }
        }

        Ok(())
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static JOURNAL: Journal = Journal::new();

