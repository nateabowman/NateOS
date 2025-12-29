use crate::process::{ProcessId, PROCESS_MANAGER};
use crate::memory::vmm::VirtualMemoryManager;
use x86_64::VirtAddr;

#[repr(C, packed)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub _padding: [u8; 7],
    pub file_type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

pub struct ProgramLoader;

impl ProgramLoader {
    pub fn load_elf(&self, elf_data: &[u8]) -> Result<ProcessId, &'static str> {
        if elf_data.len() < 64 {
            return Err("ELF file too small");
        }
        
        // Parse ELF header
        let magic = [elf_data[0], elf_data[1], elf_data[2], elf_data[3]];
        if magic != [0x7F, b'E', b'L', b'F'] {
            return Err("Invalid ELF magic");
        }
        
        // TODO: Parse program headers and load segments
        // For now, create a dummy process
        let entry_point = VirtAddr::new(0x400000);
        let stack_top = VirtAddr::new(0x800000);
        
        let pid = PROCESS_MANAGER.create_process(entry_point, stack_top);
        Ok(pid)
    }

    pub fn load_program(&self, program_data: &[u8]) -> Result<ProcessId, &'static str> {
        // Try to load as ELF first
        if program_data.len() >= 4 && program_data[0..4] == [0x7F, b'E', b'L', b'F'] {
            self.load_elf(program_data)
        } else {
            Err("Unsupported executable format")
        }
    }
}

pub static PROGRAM_LOADER: ProgramLoader = ProgramLoader;

