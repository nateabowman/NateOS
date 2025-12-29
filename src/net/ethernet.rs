#[repr(C, packed)]
pub struct EthernetHeader {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
}

impl EthernetHeader {
    pub const ETHERTYPE_IP: u16 = 0x0800;
    pub const ETHERTYPE_ARP: u16 = 0x0806;
}

pub struct EthernetFrame {
    pub header: EthernetHeader,
    pub payload: heapless::Vec<u8, 1500>,
}

impl EthernetFrame {
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ethertype: u16) -> Self {
        EthernetFrame {
            header: EthernetHeader {
                dst_mac,
                src_mac,
                ethertype: ethertype.to_be(),
            },
            payload: heapless::Vec::new(),
        }
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 14 {
            return None;
        }
        
        let mut dst_mac = [0u8; 6];
        let mut src_mac = [0u8; 6];
        dst_mac.copy_from_slice(&data[0..6]);
        src_mac.copy_from_slice(&data[6..12]);
        let ethertype = u16::from_be_bytes([data[12], data[13]]);
        
        let mut payload = heapless::Vec::new();
        payload.extend_from_slice(&data[14..]).ok()?;
        
        Some(EthernetFrame {
            header: EthernetHeader {
                dst_mac,
                src_mac,
                ethertype,
            },
            payload,
        })
    }
}

