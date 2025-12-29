use crate::net::ethernet::{EthernetFrame, EthernetHeader};

#[repr(C, packed)]
pub struct IPv4Header {
    pub version_ihl: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_addr: [u8; 4],
    pub dst_addr: [u8; 4],
}

impl IPv4Header {
    pub fn version(&self) -> u8 {
        (self.version_ihl >> 4) & 0x0F
    }

    pub fn ihl(&self) -> u8 {
        self.version_ihl & 0x0F
    }

    pub fn protocol_icmp() -> u8 { 1 }
    pub fn protocol_tcp() -> u8 { 6 }
    pub fn protocol_udp() -> u8 { 17 }
}

pub struct IPv4Packet {
    pub header: IPv4Header,
    pub payload: heapless::Vec<u8, 1500>,
}

impl IPv4Packet {
    pub fn new(src_addr: [u8; 4], dst_addr: [u8; 4], protocol: u8) -> Self {
        IPv4Packet {
            header: IPv4Header {
                version_ihl: 0x45,
                tos: 0,
                total_length: 0,
                identification: 0,
                flags_fragment_offset: 0,
                ttl: 64,
                protocol,
                checksum: 0,
                src_addr,
                dst_addr,
            },
            payload: heapless::Vec::new(),
        }
    }

    pub fn from_ethernet(frame: &EthernetFrame) -> Option<Self> {
        if frame.header.ethertype != EthernetHeader::ETHERTYPE_IP.to_be() {
            return None;
        }
        
        if frame.payload.len() < 20 {
            return None;
        }
        
        let mut header = IPv4Header {
            version_ihl: frame.payload[0],
            tos: frame.payload[1],
            total_length: u16::from_be_bytes([frame.payload[2], frame.payload[3]]),
            identification: u16::from_be_bytes([frame.payload[4], frame.payload[5]]),
            flags_fragment_offset: u16::from_be_bytes([frame.payload[6], frame.payload[7]]),
            ttl: frame.payload[8],
            protocol: frame.payload[9],
            checksum: u16::from_be_bytes([frame.payload[10], frame.payload[11]]),
            src_addr: [0; 4],
            dst_addr: [0; 4],
        };
        
        header.src_addr.copy_from_slice(&frame.payload[12..16]);
        header.dst_addr.copy_from_slice(&frame.payload[16..20]);
        
        let mut payload = heapless::Vec::new();
        let ihl = (header.ihl() * 4) as usize;
        payload.extend_from_slice(&frame.payload[ihl..]).ok()?;
        
        Some(IPv4Packet { header, payload })
    }

    pub fn to_bytes(&self) -> heapless::Vec<u8, 1520> {
        let mut bytes = heapless::Vec::new();
        bytes.push(self.header.version_ihl).ok();
        bytes.push(self.header.tos).ok();
        bytes.extend_from_slice(&self.header.total_length.to_be_bytes()).ok();
        bytes.extend_from_slice(&self.header.identification.to_be_bytes()).ok();
        bytes.extend_from_slice(&self.header.flags_fragment_offset.to_be_bytes()).ok();
        bytes.push(self.header.ttl).ok();
        bytes.push(self.header.protocol).ok();
        bytes.extend_from_slice(&self.header.checksum.to_be_bytes()).ok();
        bytes.extend_from_slice(&self.header.src_addr).ok();
        bytes.extend_from_slice(&self.header.dst_addr).ok();
        bytes.extend_from_slice(&self.payload).ok();
        bytes
    }
}

pub type IPAddress = [u8; 4];

pub const IP_LOCALHOST: IPAddress = [127, 0, 0, 1];
pub const IP_ANY: IPAddress = [0, 0, 0, 0];

