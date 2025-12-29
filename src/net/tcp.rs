use crate::net::ip::IPv4Packet;

#[repr(C, packed)]
pub struct TCPHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub data_offset_flags: u16,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
}

impl TCPHeader {
    pub fn flags_syn() -> u16 { 1 << 1 }
    pub fn flags_ack() -> u16 { 1 << 4 }
    pub fn flags_fin() -> u16 { 1 << 0 }
    pub fn flags_rst() -> u16 { 1 << 2 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TCPState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    TimeWait,
}

pub struct TCPConnection {
    pub local_port: u16,
    pub remote_addr: [u8; 4],
    pub remote_port: u16,
    pub state: TCPState,
    pub send_seq: u32,
    pub recv_seq: u32,
}

impl TCPConnection {
    pub fn new(local_port: u16) -> Self {
        TCPConnection {
            local_port,
            remote_addr: [0; 4],
            remote_port: 0,
            state: TCPState::Closed,
            send_seq: 0,
            recv_seq: 0,
        }
    }

    pub fn handle_packet(&mut self, packet: &IPv4Packet) {
        if packet.payload.len() < 20 {
            return;
        }
        
        let header = TCPHeader {
            src_port: u16::from_be_bytes([packet.payload[0], packet.payload[1]]),
            dst_port: u16::from_be_bytes([packet.payload[2], packet.payload[3]]),
            sequence: u32::from_be_bytes([
                packet.payload[4], packet.payload[5],
                packet.payload[6], packet.payload[7],
            ]),
            ack_sequence: u32::from_be_bytes([
                packet.payload[8], packet.payload[9],
                packet.payload[10], packet.payload[11],
            ]),
            data_offset_flags: u16::from_be_bytes([packet.payload[12], packet.payload[13]]),
            window_size: u16::from_be_bytes([packet.payload[14], packet.payload[15]]),
            checksum: u16::from_be_bytes([packet.payload[16], packet.payload[17]]),
            urgent_pointer: u16::from_be_bytes([packet.payload[18], packet.payload[19]]),
        };
        
        // Handle TCP state machine
        match self.state {
            TCPState::Listen => {
                if (header.data_offset_flags & TCPHeader::flags_syn()) != 0 {
                    self.state = TCPState::SynReceived;
                    self.remote_addr = packet.header.src_addr;
                    self.remote_port = header.src_port;
                }
            }
            TCPState::Established => {
                if (header.data_offset_flags & TCPHeader::flags_fin()) != 0 {
                    self.state = TCPState::CloseWait;
                }
            }
            _ => {}
        }
    }
}

