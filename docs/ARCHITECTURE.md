# NateOS Architecture

## Overview

NateOS is a monolithic kernel operating system designed for x86_64 architecture. It is written entirely in Rust using `no_std` for kernel code.

## Kernel Structure

### Boot Process

1. Bootloader loads kernel at 0x100000
2. Kernel entry point `_start()` initializes subsystems
3. Memory map is parsed from bootloader
4. Interrupt handlers are set up
5. Main kernel loop begins

### Memory Management

#### Physical Memory
- BootInfoFrameAllocator uses bootloader memory map
- Tracks usable memory regions
- Allocates 4KB frames

#### Virtual Memory
- 4-level paging (x86_64 standard)
- Page tables managed by VirtualMemoryManager
- ASLR for address space randomization

### Process Management

- Process Control Blocks (PCB) track process state
- Each process has separate address space
- Process IDs allocated sequentially
- Process states: Running, Ready, Blocked, Terminated

### Scheduling

Two schedulers available:
1. **Basic Scheduler**: Round-robin with ready queue
2. **Optimized Scheduler**: CFS-like with virtual runtime

### System Calls

System calls use interrupt 0x80 (x86) or SYSCALL instruction (x86_64).
Syscall handler validates arguments and routes to appropriate handler.

### I/O Subsystem

#### Device Drivers
- Driver framework with trait-based interface
- Keyboard driver (PS/2)
- VGA text mode driver
- ATA/IDE storage driver

#### File System
- Inode-based filesystem
- Block device abstraction
- File operations: create, read, write, delete

### Networking

#### Protocol Stack
- Ethernet frame handling
- IPv4 protocol
- TCP and UDP
- Socket API

#### Security
- Firewall with rule-based filtering
- Packet validation

### Security

- **ASLR**: Randomizes memory layout
- **Stack Protection**: Canaries detect overflows
- **Capabilities**: Fine-grained permissions
- **Audit Logging**: Security event tracking

### User Space

- ELF program loader
- Shell with basic commands
- Multi-user support
- Environment variables

## Data Structures

### Process
```rust
struct Process {
    pid: ProcessId,
    state: ProcessState,
    stack_pointer: VirtAddr,
    instruction_pointer: VirtAddr,
}
```

### Inode
```rust
struct Inode {
    inode_number: u64,
    file_type: FileType,
    size: u64,
    blocks: Vec<u64>,
    permissions: u16,
}
```

### Socket
```rust
struct Socket {
    socket_type: SocketType,
    state: SocketState,
    local_addr: IPAddress,
    local_port: u16,
    remote_addr: IPAddress,
    remote_port: u16,
}
```

## Interrupts

- Exception handlers for CPU exceptions
- IRQ handlers for hardware interrupts
- System call interrupt (0x80)

## Synchronization

- Spinlocks for short critical sections
- Mutexes for longer operations
- Semaphores for resource counting

## Error Handling

- Panic handler for unrecoverable errors
- Error handler for recoverable errors
- Watchdog timer for system health

## Performance Monitoring

- Function-level profiler
- Memory usage tracking
- Process statistics

