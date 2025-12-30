use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceClass {
    MassStorage,
    HID,
    Network,
    Audio,
    Video,
    Unknown,
}

pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_class: UsbDeviceClass,
    pub bus: u8,
    pub address: u8,
}

pub struct UsbManager {
    devices: Mutex<BTreeMap<u8, UsbDevice>>,
    next_address: Mutex<u8>,
}

impl UsbManager {
    pub const fn new() -> Self {
        UsbManager {
            devices: Mutex::new(BTreeMap::new()),
            next_address: Mutex::new(1),
        }
    }

    pub fn init(&self) -> Result<(), &'static str> {
        // TODO: Initialize USB controller
        crate::io::println!("USB: Initializing USB subsystem");
        Ok(())
    }

    pub fn enumerate_devices(&self) -> Result<(), &'static str> {
        // TODO: Enumerate USB devices
        Ok(())
    }

    pub fn register_device(&self, device: UsbDevice) -> u8 {
        let address = {
            let mut next = self.next_address.lock();
            let addr = *next;
            *next += 1;
            addr
        };
        
        let mut device = device;
        device.address = address;
        self.devices.lock().insert(address, device);
        address
    }

    pub fn get_device(&self, address: u8) -> Option<UsbDevice> {
        self.devices.lock().get(&address).copied()
    }
}

pub static USB_MANAGER: UsbManager = UsbManager::new();

