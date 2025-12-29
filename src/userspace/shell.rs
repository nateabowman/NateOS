use crate::drivers::keyboard::KEYBOARD;
use crate::process::ProcessId;
use crate::fs::FILESYSTEM;
use heapless::String;

pub struct Shell {
    current_directory: String<256>,
    prompt: String<64>,
}

impl Shell {
    pub const fn new() -> Self {
        Shell {
            current_directory: String::new(),
            prompt: String::from_str("nateos$ "),
        }
    }

    pub fn run(&mut self) {
        crate::io::println!("NateOS Shell v0.1.0");
        crate::io::println!("Type 'help' for available commands.");
        
        loop {
            crate::io::print!("{}", self.prompt.as_str());
            
            let mut command = String::<256>::new();
            let mut input_buffer = [0u8; 256];
            let mut pos = 0;
            
            // Read command line
            loop {
                if let Some(key) = KEYBOARD.get_key() {
                    if key == b'\n' {
                        break;
                    } else if key == 0x08 || key == 0x7F { // Backspace
                        if pos > 0 {
                            pos -= 1;
                            command.pop();
                        }
                    } else if key >= 0x20 && key <= 0x7E {
                        if pos < 255 {
                            input_buffer[pos] = key;
                            command.push(key as char).ok();
                            pos += 1;
                        }
                    }
                }
            }
            
            // Execute command
            self.execute_command(&command);
            command.clear();
        }
    }

    fn execute_command(&mut self, command: &str) {
        let parts: heapless::Vec<&str, 16> = command.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        match parts[0] {
            "help" => {
                crate::io::println!("Available commands:");
                crate::io::println!("  help - Show this help message");
                crate::io::println!("  ls - List files");
                crate::io::println!("  cat <file> - Display file contents");
                crate::io::println!("  echo <text> - Echo text");
                crate::io::println!("  exit - Exit shell");
            }
            "ls" => {
                crate::io::println!("Files in current directory:");
                // TODO: List files from filesystem
            }
            "cat" => {
                if parts.len() > 1 {
                    // TODO: Read and display file
                    crate::io::println!("cat: File reading not yet implemented");
                } else {
                    crate::io::println!("cat: missing file argument");
                }
            }
            "echo" => {
                if parts.len() > 1 {
                    for part in parts.iter().skip(1) {
                        crate::io::print!("{} ", part);
                    }
                    crate::io::println!("");
                }
            }
            "exit" => {
                crate::io::println!("Exiting shell...");
                // TODO: Properly exit
            }
            _ => {
                crate::io::println!("Unknown command: {}", parts[0]);
            }
        }
    }
}

pub static SHELL: Shell = Shell::new();

