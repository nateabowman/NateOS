use super::Driver;
use x86_64::instructions::port::Port;
use spin::Mutex;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_COMMAND_PORT: u16 = 0x64;

pub struct KeyboardDriver {
    initialized: bool,
    buffer: Mutex<heapless::Vec<u8, 256>>,
}

impl KeyboardDriver {
    pub const fn new() -> Self {
        KeyboardDriver {
            initialized: false,
            buffer: Mutex::new(heapless::Vec::new()),
        }
    }

    pub fn read_scancode(&self) -> Option<u8> {
        unsafe {
            let mut status_port = Port::new(KEYBOARD_STATUS_PORT);
            if (status_port.read() & 1) != 0 {
                let mut data_port = Port::new(KEYBOARD_DATA_PORT);
                Some(data_port.read())
            } else {
                None
            }
        }
    }

    pub fn get_key(&self) -> Option<u8> {
        if let Some(scancode) = self.read_scancode() {
            // Basic scancode to ASCII conversion (simplified)
            let key = match scancode {
                0x02 => Some(b'1'),
                0x03 => Some(b'2'),
                0x04 => Some(b'3'),
                0x05 => Some(b'4'),
                0x06 => Some(b'5'),
                0x07 => Some(b'6'),
                0x08 => Some(b'7'),
                0x09 => Some(b'8'),
                0x0A => Some(b'9'),
                0x0B => Some(b'0'),
                0x10 => Some(b'q'),
                0x11 => Some(b'w'),
                0x12 => Some(b'e'),
                0x13 => Some(b'r'),
                0x14 => Some(b't'),
                0x15 => Some(b'y'),
                0x16 => Some(b'u'),
                0x17 => Some(b'i'),
                0x18 => Some(b'o'),
                0x19 => Some(b'p'),
                0x1E => Some(b'a'),
                0x1F => Some(b's'),
                0x20 => Some(b'd'),
                0x21 => Some(b'f'),
                0x22 => Some(b'g'),
                0x23 => Some(b'h'),
                0x24 => Some(b'j'),
                0x25 => Some(b'k'),
                0x26 => Some(b'l'),
                0x2C => Some(b'z'),
                0x2D => Some(b'x'),
                0x2E => Some(b'c'),
                0x2F => Some(b'v'),
                0x30 => Some(b'b'),
                0x31 => Some(b'n'),
                0x32 => Some(b'm'),
                0x1C => Some(b'\n'),
                0x39 => Some(b' '),
                _ => None,
            };
            if let Some(k) = key {
                self.buffer.lock().push(k).ok();
            }
            key
        } else {
            None
        }
    }
}

impl Driver for KeyboardDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        // Enable keyboard interrupts
        unsafe {
            let mut cmd_port = Port::new(KEYBOARD_COMMAND_PORT);
            cmd_port.write(0xAEu8); // Enable keyboard
        }
        self.initialized = true;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "PS/2 Keyboard"
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

pub static KEYBOARD: KeyboardDriver = KeyboardDriver::new();

