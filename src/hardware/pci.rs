use x86_64::instructions::port::Port;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
}

pub struct PciManager {
    devices: Mutex<BTreeMap<(u8, u8, u8), PciDevice>>,
}

impl PciManager {
    pub const fn new() -> Self {
        PciManager {
            devices: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn init(&self) {
        crate::io::println!("PCI: Scanning PCI bus");
        self.scan_bus();
    }

    fn scan_bus(&self) {
        for bus in 0..256 {
            for device in 0..32 {
                for function in 0..8 {
                    if let Some(pci_dev) = self.read_device(bus, device, function) {
                        let key = (bus, device, function);
                        self.devices.lock().insert(key, pci_dev);
                    }
                }
            }
        }
    }

    fn read_device(&self, bus: u8, device: u8, function: u8) -> Option<PciDevice> {
        unsafe {
            let config_addr = 0x80000000u32
                | ((bus as u32) << 16)
                | ((device as u32) << 11)
                | ((function as u32) << 8);
            
            let mut config_port = Port::<u32>::new(0xCF8);
            config_port.write(config_addr);
            
            let mut data_port = Port::<u32>::new(0xCFC);
            let vendor_device = data_port.read();
            
            let vendor_id = (vendor_device & 0xFFFF) as u16;
            if vendor_id == 0xFFFF {
                return None;
            }
            
            let device_id = ((vendor_device >> 16) & 0xFFFF) as u16;
            
            // Read class code
            let class_reg = config_addr | 0x08;
            config_port.write(class_reg);
            let class_data = data_port.read();
            let class_code = ((class_data >> 24) & 0xFF) as u8;
            let subclass = ((class_data >> 16) & 0xFF) as u8;
            
            Some(PciDevice {
                bus,
                device,
                function,
                vendor_id,
                device_id,
                class_code,
                subclass,
            })
        }
    }

    pub fn get_devices(&self) -> alloc::vec::Vec<PciDevice> {
        self.devices.lock().values().cloned().collect()
    }
}

pub static PCI_MANAGER: PciManager = PciManager::new();

