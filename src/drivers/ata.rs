use super::Driver;
use x86_64::instructions::port::Port;

const ATA_PRIMARY_DATA: u16 = 0x1F0;
const ATA_PRIMARY_ERROR: u16 = 0x1F1;
const ATA_PRIMARY_SECTOR_COUNT: u16 = 0x1F2;
const ATA_PRIMARY_LBA_LOW: u16 = 0x1F3;
const ATA_PRIMARY_LBA_MID: u16 = 0x1F4;
const ATA_PRIMARY_LBA_HIGH: u16 = 0x1F5;
const ATA_PRIMARY_DRIVE: u16 = 0x1F6;
const ATA_PRIMARY_COMMAND: u16 = 0x1F7;
const ATA_PRIMARY_STATUS: u16 = 0x1F7;

pub struct AtaDriver {
    initialized: bool,
}

impl AtaDriver {
    pub const fn new() -> Self {
        AtaDriver {
            initialized: false,
        }
    }

    pub fn read_sector(&self, lba: u32, buffer: &mut [u8; 512]) -> Result<(), &'static str> {
        unsafe {
            // Select drive and LBA
            let mut drive_port = Port::new(ATA_PRIMARY_DRIVE);
            drive_port.write(0xE0u8 | ((lba >> 24) & 0x0F) as u8);
            
            let mut error_port = Port::new(ATA_PRIMARY_ERROR);
            error_port.write(0u8);
            
            let mut sector_count_port = Port::new(ATA_PRIMARY_SECTOR_COUNT);
            sector_count_port.write(1u8);
            
            let mut lba_low = Port::new(ATA_PRIMARY_LBA_LOW);
            lba_low.write((lba & 0xFF) as u8);
            
            let mut lba_mid = Port::new(ATA_PRIMARY_LBA_MID);
            lba_mid.write(((lba >> 8) & 0xFF) as u8);
            
            let mut lba_high = Port::new(ATA_PRIMARY_LBA_HIGH);
            lba_high.write(((lba >> 16) & 0xFF) as u8);
            
            // Send read command
            let mut command_port = Port::new(ATA_PRIMARY_COMMAND);
            command_port.write(0x20u8); // Read sectors
            
            // Wait for ready
            let mut status_port = Port::new(ATA_PRIMARY_STATUS);
            loop {
                let status = status_port.read();
                if (status & 0x80) == 0 {
                    break;
                }
            }
            
            // Check for errors
            let status = status_port.read();
            if (status & 0x01) != 0 {
                return Err("ATA error");
            }
            
            // Read data
            let mut data_port = Port::new(ATA_PRIMARY_DATA);
            for i in 0..256 {
                let word = data_port.read() as u16;
                buffer[i * 2] = (word & 0xFF) as u8;
                buffer[i * 2 + 1] = ((word >> 8) & 0xFF) as u8;
            }
            
            Ok(())
        }
    }

    pub fn write_sector(&self, lba: u32, buffer: &[u8; 512]) -> Result<(), &'static str> {
        unsafe {
            // Select drive and LBA
            let mut drive_port = Port::new(ATA_PRIMARY_DRIVE);
            drive_port.write(0xE0u8 | ((lba >> 24) & 0x0F) as u8);
            
            let mut error_port = Port::new(ATA_PRIMARY_ERROR);
            error_port.write(0u8);
            
            let mut sector_count_port = Port::new(ATA_PRIMARY_SECTOR_COUNT);
            sector_count_port.write(1u8);
            
            let mut lba_low = Port::new(ATA_PRIMARY_LBA_LOW);
            lba_low.write((lba & 0xFF) as u8);
            
            let mut lba_mid = Port::new(ATA_PRIMARY_LBA_MID);
            lba_mid.write(((lba >> 8) & 0xFF) as u8);
            
            let mut lba_high = Port::new(ATA_PRIMARY_LBA_HIGH);
            lba_high.write(((lba >> 16) & 0xFF) as u8);
            
            // Send write command
            let mut command_port = Port::new(ATA_PRIMARY_COMMAND);
            command_port.write(0x30u8); // Write sectors
            
            // Wait for ready
            let mut status_port = Port::new(ATA_PRIMARY_STATUS);
            loop {
                let status = status_port.read();
                if (status & 0x80) == 0 {
                    break;
                }
            }
            
            // Write data
            let mut data_port = Port::new(ATA_PRIMARY_DATA);
            for i in 0..256 {
                let word = ((buffer[i * 2 + 1] as u16) << 8) | buffer[i * 2] as u16;
                data_port.write(word);
            }
            
            // Flush cache
            command_port.write(0xE7u8);
            
            Ok(())
        }
    }
}

impl Driver for AtaDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        // Check if drive is present
        unsafe {
            let mut status_port = Port::new(ATA_PRIMARY_STATUS);
            let status = status_port.read();
            if status == 0xFF {
                return Err("No ATA drive detected");
            }
        }
        self.initialized = true;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "ATA/IDE"
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

pub static ATA: AtaDriver = AtaDriver::new();

