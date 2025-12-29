use x86_64::registers::rflags::RFlags;

#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    Exit = 1,
    Read = 2,
    Write = 3,
    Open = 4,
    Close = 5,
    Fork = 6,
    Exec = 7,
    Wait = 8,
    Kill = 9,
    GetPid = 10,
    Sleep = 11,
    Mmap = 12,
    Munmap = 13,
    Brk = 14,
    Ioctl = 15,
}

pub struct SyscallContext {
    pub syscall_number: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
    pub arg6: u64,
}

pub fn handle_syscall(context: SyscallContext) -> u64 {
    match context.syscall_number {
        1 => sys_exit(context.arg1 as i32),
        2 => sys_read(context.arg1, context.arg2 as *mut u8, context.arg3),
        3 => sys_write(context.arg1, context.arg2 as *const u8, context.arg3),
        10 => sys_getpid(),
        _ => {
            crate::io::println!("Unknown syscall: {}", context.syscall_number);
            !0u64
        }
    }
}

fn sys_exit(status: i32) -> u64 {
    crate::io::println!("Process exiting with status: {}", status);
    // TODO: Clean up process resources
    0
}

fn sys_read(fd: u64, buf: *mut u8, count: u64) -> u64 {
    // TODO: Implement read syscall
    crate::io::println!("sys_read: fd={}, count={}", fd, count);
    0
}

fn sys_write(fd: u64, buf: *const u8, count: u64) -> u64 {
    // TODO: Implement write syscall
    crate::io::println!("sys_write: fd={}, count={}", fd, count);
    count
}

fn sys_getpid() -> u64 {
    if let Some(pid) = crate::process::PROCESS_MANAGER.get_current_process() {
        pid.0 as u64
    } else {
        0
    }
}

