use crate::net::ip::IPv4Packet;

#[repr(C, packed)]
pub struct UDPHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

pub struct UDPPacket {
    pub header: UDPHeader,
    pub payload: heapless::Vec<u8, 1500>,
}

impl UDPPacket {
    pub fn from_ip(packet: &IPv4Packet) -> Option<Self> {
        if packet.header.protocol != IPv4Header::protocol_udp() {
            return None;
        }
        
        if packet.payload.len() < 8 {
            return None;
        }
        
        let header = UDPHeader {
            src_port: u16::from_be_bytes([packet.payload[0], packet.payload[1]]),
            dst_port: u16::from_be_bytes([packet.payload[2], packet.payload[3]]),
            length: u16::from_be_bytes([packet.payload[4], packet.payload[5]]),
            checksum: u16::from_be_bytes([packet.payload[6], packet.payload[7]]),
        };
        
        let mut payload = heapless::Vec::new();
        payload.extend_from_slice(&packet.payload[8..]).ok()?;
        
        Some(UDPPacket { header, payload })
    }

    pub fn new(src_port: u16, dst_port: u16) -> Self {
        UDPPacket {
            header: UDPHeader {
                src_port,
                dst_port,
                length: 0,
                checksum: 0,
            },
            payload: heapless::Vec::new(),
        }
    }
}

