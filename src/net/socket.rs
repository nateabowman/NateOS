use spin::Mutex;
use alloc::collections::BTreeMap;
use crate::process::ProcessId;
use crate::net::tcp::{TCPConnection, TCPState};
use crate::net::udp::UDPPacket;
use crate::net::ip::IPAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    TCP,
    UDP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Unbound,
    Bound,
    Listening,
    Connected,
    Closed,
}

pub struct Socket {
    pub socket_type: SocketType,
    pub state: SocketState,
    pub local_addr: IPAddress,
    pub local_port: u16,
    pub remote_addr: IPAddress,
    pub remote_port: u16,
    pub owner: ProcessId,
    pub tcp_conn: Option<TCPConnection>,
}

impl Socket {
    pub fn new(socket_type: SocketType, owner: ProcessId) -> Self {
        Socket {
            socket_type,
            state: SocketState::Unbound,
            local_addr: [0, 0, 0, 0],
            local_port: 0,
            remote_addr: [0, 0, 0, 0],
            remote_port: 0,
            owner,
            tcp_conn: None,
        }
    }

    pub fn bind(&mut self, addr: IPAddress, port: u16) -> Result<(), &'static str> {
        if self.state != SocketState::Unbound {
            return Err("Socket already bound");
        }
        self.local_addr = addr;
        self.local_port = port;
        self.state = SocketState::Bound;
        Ok(())
    }

    pub fn listen(&mut self) -> Result<(), &'static str> {
        if self.state != SocketState::Bound {
            return Err("Socket must be bound before listening");
        }
        if self.socket_type != SocketType::TCP {
            return Err("Only TCP sockets can listen");
        }
        self.state = SocketState::Listening;
        self.tcp_conn = Some(TCPConnection::new(self.local_port));
        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        match self.socket_type {
            SocketType::UDP => {
                // TODO: Send UDP packet
                Ok(data.len())
            }
            SocketType::TCP => {
                if self.state != SocketState::Connected {
                    return Err("Socket not connected");
                }
                // TODO: Send TCP data
                Ok(data.len())
            }
        }
    }

    pub fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        match self.socket_type {
            SocketType::UDP => {
                // TODO: Receive UDP packet
                Ok(0)
            }
            SocketType::TCP => {
                if self.state != SocketState::Connected {
                    return Err("Socket not connected");
                }
                // TODO: Receive TCP data
                Ok(0)
            }
        }
    }
}

pub struct SocketManager {
    sockets: Mutex<BTreeMap<u64, Socket>>,
    next_fd: Mutex<u64>,
}

impl SocketManager {
    pub const fn new() -> Self {
        SocketManager {
            sockets: Mutex::new(BTreeMap::new()),
            next_fd: Mutex::new(3), // Start after stdin, stdout, stderr
        }
    }

    pub fn create_socket(&self, socket_type: SocketType, owner: ProcessId) -> u64 {
        let fd = {
            let mut next = self.next_fd.lock();
            let fd = *next;
            *next += 1;
            fd
        };
        
        let socket = Socket::new(socket_type, owner);
        self.sockets.lock().insert(fd, socket);
        fd
    }

    pub fn get_socket(&self, fd: u64) -> Option<Socket> {
        self.sockets.lock().get(&fd).cloned()
    }

    pub fn get_socket_mut(&self, fd: u64) -> Option<spin::MutexGuard<Socket>> {
        // This is a simplified version - in reality we'd need a different structure
        None
    }
}

pub static SOCKET_MANAGER: SocketManager = SocketManager::new();

