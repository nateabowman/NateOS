use crate::net::ethernet::EthernetFrame;

#[repr(C, packed)]
pub struct IPv6Header {
    pub version_traffic_flow: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_addr: [u8; 16],
    pub dst_addr: [u8; 16],
}

impl IPv6Header {
    pub fn version(&self) -> u8 {
        ((self.version_traffic_flow >> 28) & 0x0F) as u8
    }

    pub fn traffic_class(&self) -> u8 {
        ((self.version_traffic_flow >> 20) & 0xFF) as u8
    }

    pub fn flow_label(&self) -> u32 {
        self.version_traffic_flow & 0x000FFFFF
    }
}

pub struct IPv6Packet {
    pub header: IPv6Header,
    pub payload: heapless::Vec<u8, 1500>,
}

impl IPv6Packet {
    pub fn new(src_addr: [u8; 16], dst_addr: [u8; 16], next_header: u8) -> Self {
        IPv6Packet {
            header: IPv6Header {
                version_traffic_flow: (6u32 << 28), // Version 6
                payload_length: 0,
                next_header,
                hop_limit: 64,
                src_addr,
                dst_addr,
            },
            payload: heapless::Vec::new(),
        }
    }

    pub fn from_ethernet(frame: &EthernetFrame) -> Option<Self> {
        const ETHERTYPE_IPV6: u16 = 0x86DD;
        if frame.header.ethertype != ETHERTYPE_IPV6.to_be() {
            return None;
        }
        
        if frame.payload.len() < 40 {
            return None;
        }
        
        let mut header = IPv6Header {
            version_traffic_flow: u32::from_be_bytes([
                frame.payload[0], frame.payload[1],
                frame.payload[2], frame.payload[3],
            ]),
            payload_length: u16::from_be_bytes([frame.payload[4], frame.payload[5]]),
            next_header: frame.payload[6],
            hop_limit: frame.payload[7],
            src_addr: [0; 16],
            dst_addr: [0; 16],
        };
        
        header.src_addr.copy_from_slice(&frame.payload[8..24]);
        header.dst_addr.copy_from_slice(&frame.payload[24..40]);
        
        let mut payload = heapless::Vec::new();
        payload.extend_from_slice(&frame.payload[40..]).ok()?;
        
        Some(IPv6Packet { header, payload })
    }
}

pub type IPv6Address = [u8; 16];

pub const IPV6_LOCALHOST: IPv6Address = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
pub const IPV6_UNSPECIFIED: IPv6Address = [0; 16];

