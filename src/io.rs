use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

const SERIAL_PORT: u16 = 0x3F8; // COM1

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = Mutex::new(SerialPort::new(SERIAL_PORT));
}

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        SerialPort { port }
    }

    fn write_byte(&mut self, byte: u8) {
        unsafe {
            let mut line_status = Port::new(self.port + 5);
            // Wait until transmit buffer is empty
            while (line_status.read() & 0x20) == 0 {}
            let mut data = Port::new(self.port);
            data.write(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

/// Initialize I/O subsystem
pub fn init() {
    // Initialize serial port
    unsafe {
        let mut port1 = Port::new(SERIAL_PORT + 1);
        port1.write(0x00u8); // Disable interrupts
        let mut port3 = Port::new(SERIAL_PORT + 3);
        port3.write(0x80u8); // Enable DLAB
        let mut port0 = Port::new(SERIAL_PORT + 0);
        port0.write(0x03u8); // Set divisor to 3 (lo byte)
        let mut port1_hi = Port::new(SERIAL_PORT + 1);
        port1_hi.write(0x00u8); // Set divisor to 3 (hi byte)
        let mut port3_config = Port::new(SERIAL_PORT + 3);
        port3_config.write(0x03u8); // 8 bits, no parity, one stop bit
        let mut port2 = Port::new(SERIAL_PORT + 2);
        port2.write(0xC7u8); // Enable FIFO, clear them, 14-byte threshold
        let mut port4 = Port::new(SERIAL_PORT + 4);
        port4.write(0x0Bu8); // IRQs enabled, RTS/DSR set
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}

