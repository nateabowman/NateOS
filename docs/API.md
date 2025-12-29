# NateOS API Documentation

## System Calls

### Process Management

#### `exit(status: i32) -> !`
Terminate the current process with the given exit status.

#### `fork() -> Result<ProcessId, Error>`
Create a new process by duplicating the current process.

#### `exec(path: &str, args: &[&str]) -> Result<!, Error>`
Replace the current process with a new program.

#### `wait(pid: ProcessId) -> Result<i32, Error>`
Wait for a child process to terminate.

#### `kill(pid: ProcessId, signal: i32) -> Result<(), Error>`
Send a signal to a process.

#### `getpid() -> ProcessId`
Get the current process ID.

### File Operations

#### `open(path: &str, flags: u32) -> Result<u64, Error>`
Open a file and return a file descriptor.

#### `close(fd: u64) -> Result<(), Error>`
Close a file descriptor.

#### `read(fd: u64, buffer: &mut [u8]) -> Result<usize, Error>`
Read data from a file descriptor.

#### `write(fd: u64, buffer: &[u8]) -> Result<usize, Error>`
Write data to a file descriptor.

### Memory Management

#### `mmap(addr: Option<VirtAddr>, length: usize, prot: u32, flags: u32) -> Result<VirtAddr, Error>`
Map memory into the process address space.

#### `munmap(addr: VirtAddr, length: usize) -> Result<(), Error>`
Unmap memory from the process address space.

#### `brk(addr: VirtAddr) -> Result<VirtAddr, Error>`
Change the data segment size.

### Networking

#### `socket(domain: u32, socket_type: u32, protocol: u32) -> Result<u64, Error>`
Create a new socket.

#### `bind(fd: u64, addr: &SocketAddr) -> Result<(), Error>`
Bind a socket to an address.

#### `listen(fd: u64, backlog: u32) -> Result<(), Error>`
Listen for connections on a socket.

#### `connect(fd: u64, addr: &SocketAddr) -> Result<(), Error>`
Connect to a remote address.

#### `send(fd: u64, buffer: &[u8], flags: u32) -> Result<usize, Error>`
Send data on a socket.

#### `recv(fd: u64, buffer: &mut [u8], flags: u32) -> Result<usize, Error>`
Receive data from a socket.

## Kernel APIs

### Process Manager

```rust
pub static PROCESS_MANAGER: ProcessManager;

impl ProcessManager {
    pub fn create_process(entry_point: VirtAddr, stack_top: VirtAddr) -> ProcessId;
    pub fn get_current_process() -> Option<ProcessId>;
}
```

### Scheduler

```rust
pub static SCHEDULER: Scheduler;

impl Scheduler {
    pub fn enqueue(pid: ProcessId);
    pub fn schedule_next() -> Option<ProcessId>;
}
```

### File System

```rust
pub static FILESYSTEM: Mutex<FileSystem>;

impl FileSystem {
    pub fn create_file(path: &str, file_type: FileType) -> Result<u64, Error>;
    pub fn read_file(inode: u64, offset: u64, buffer: &mut [u8]) -> Result<usize, Error>;
    pub fn write_file(inode: u64, offset: u64, data: &[u8]) -> Result<usize, Error>;
}
```

### Networking

```rust
pub static SOCKET_MANAGER: SocketManager;

impl SocketManager {
    pub fn create_socket(socket_type: SocketType, owner: ProcessId) -> u64;
    pub fn get_socket(fd: u64) -> Option<Socket>;
}
```

## Security APIs

### Capabilities

```rust
pub static CAPABILITY_MANAGER: CapabilityManager;

impl CapabilityManager {
    pub fn has_capability(pid: ProcessId, cap: Capabilities) -> bool;
    pub fn set_capabilities(pid: ProcessId, caps: Capabilities);
}
```

### Audit Logging

```rust
pub static AUDIT_LOGGER: AuditLogger;

impl AuditLogger {
    pub fn log(event_type: AuditEventType, pid: Option<ProcessId>, message: &str);
}
```

