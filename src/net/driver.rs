use crate::net::ethernet::EthernetFrame;
use crate::net::ip::IPv4Packet;
use crate::net::firewall::FIREWALL;

pub struct NetworkDriver {
    mac_address: [u8; 6],
    initialized: bool,
}

impl NetworkDriver {
    pub const fn new() -> Self {
        NetworkDriver {
            mac_address: [0x02, 0x00, 0x00, 0x00, 0x00, 0x01],
            initialized: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // TODO: Initialize network hardware (e.g., E1000, RTL8139, virtio-net)
        self.initialized = true;
        Ok(())
    }

    pub fn receive_packet(&self) -> Option<EthernetFrame> {
        // TODO: Receive packet from hardware
        None
    }

    pub fn send_packet(&self, frame: &EthernetFrame) -> Result<(), &'static str> {
        // TODO: Send packet to hardware
        Ok(())
    }

    pub fn process_packet(&self, frame: EthernetFrame) {
        if let Some(ip_packet) = IPv4Packet::from_ethernet(&frame) {
            // Check firewall
            if !FIREWALL.check_packet(&ip_packet) {
                crate::io::println!("Firewall blocked packet");
                return;
            }
            
            // Process IP packet
            match ip_packet.header.protocol {
                p if p == IPv4Header::protocol_tcp() => {
                    // Handle TCP
                }
                p if p == IPv4Header::protocol_udp() => {
                    // Handle UDP
                }
                _ => {}
            }
        }
    }
}

pub static NETWORK_DRIVER: NetworkDriver = NetworkDriver::new();

