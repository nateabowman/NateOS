use crate::drivers::ata::ATA;

pub const BLOCK_SIZE: usize = 512;

pub struct BlockDevice;

impl BlockDevice {
    pub fn read_block(&self, block_number: u64, buffer: &mut [u8; BLOCK_SIZE]) -> Result<(), &'static str> {
        ATA.read_sector(block_number as u32, buffer)
    }

    pub fn write_block(&self, block_number: u64, buffer: &[u8; BLOCK_SIZE]) -> Result<(), &'static str> {
        ATA.write_sector(block_number as u32, buffer)
    }
}

pub static BLOCK_DEVICE: BlockDevice = BlockDevice;

