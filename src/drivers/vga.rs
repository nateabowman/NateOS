use super::Driver;
use volatile::Volatile;
use spin::Mutex;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: *mut VgaBuffer = 0xB8000 as *mut VgaBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct VgaDriver {
    initialized: bool,
    row: Mutex<usize>,
    column: Mutex<usize>,
    color_code: ColorCode,
}

impl VgaDriver {
    pub const fn new() -> Self {
        VgaDriver {
            initialized: false,
            row: Mutex::new(0),
            column: Mutex::new(0),
            color_code: ColorCode::new(Color::LightGray, Color::Black),
        }
    }

    pub fn write_byte(&self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if *self.column.lock() >= VGA_WIDTH {
                    self.newline();
                }

                let row = *self.row.lock();
                let col = *self.column.lock();

                let color_code = self.color_code;
                unsafe {
                    let buffer = &mut *VGA_BUFFER;
                    buffer.chars[row][col].write(ScreenChar {
                        ascii_character: byte,
                        color_code,
                    });
                }
                *self.column.lock() += 1;
            }
        }
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn newline(&self) {
        *self.column.lock() = 0;
        let row = *self.row.lock();
        if row >= VGA_HEIGHT - 1 {
            self.clear_row(0);
            for row in 1..VGA_HEIGHT {
                self.clear_row(row);
            }
            *self.row.lock() = 0;
        } else {
            *self.row.lock() += 1;
        }
    }

    fn clear_row(&self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        unsafe {
            let buffer = &mut *VGA_BUFFER;
            for col in 0..VGA_WIDTH {
                buffer.chars[row][col].write(blank);
            }
        }
    }

    pub fn clear(&self) {
        for row in 0..VGA_HEIGHT {
            self.clear_row(row);
        }
        *self.row.lock() = 0;
        *self.column.lock() = 0;
    }
}

impl Driver for VgaDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        self.clear();
        self.initialized = true;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "VGA Text Mode"
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

pub static VGA: VgaDriver = VgaDriver::new();

