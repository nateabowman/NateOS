use crate::net::ip::{IPv4Packet, IPAddress};
use crate::process::ProcessId;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ThreatEvent {
    pub threat_type: heapless::String<64>,
    pub level: ThreatLevel,
    pub source: IPAddress,
    pub timestamp: u64,
    pub details: heapless::String<256>,
}

pub struct IntrusionDetectionSystem {
    events: Mutex<alloc::collections::VecDeque<ThreatEvent, 1024>>,
    enabled: Mutex<bool>,
    thresholds: Mutex<BTreeMap<ThreatLevel, u64>>,
}

impl IntrusionDetectionSystem {
    pub const fn new() -> Self {
        IntrusionDetectionSystem {
            events: Mutex::new(alloc::collections::VecDeque::new()),
            enabled: Mutex::new(true),
            thresholds: Mutex::new(BTreeMap::new()),
        }
    }
    
    pub fn init(&self) {
        let mut thresholds = BTreeMap::new();
        thresholds.insert(ThreatLevel::Low, 100);
        thresholds.insert(ThreatLevel::Medium, 50);
        thresholds.insert(ThreatLevel::High, 10);
        thresholds.insert(ThreatLevel::Critical, 1);
        *self.thresholds.lock() = thresholds;
    }

    pub fn analyze_packet(&self, packet: &IPv4Packet) {
        if !*self.enabled.lock() {
            return;
        }
        
        // Check for suspicious patterns
        if self.detect_port_scan(packet) {
            self.record_threat(
                ThreatLevel::Medium,
                packet.header.src_addr,
                "Port scan detected",
            );
        }
        
        if self.detect_syn_flood(packet) {
            self.record_threat(
                ThreatLevel::High,
                packet.header.src_addr,
                "SYN flood detected",
            );
        }
    }

    pub fn analyze_syscall(&self, pid: ProcessId, syscall_num: u64) {
        if !*self.enabled.lock() {
            return;
        }
        
        // Check for suspicious syscall patterns
        if syscall_num > 100 {
            self.record_threat(
                ThreatLevel::Medium,
                [0, 0, 0, 0],
                &alloc::format!("Invalid syscall {} from PID {}", syscall_num, pid.0),
            );
        }
    }

    fn detect_port_scan(&self, _packet: &IPv4Packet) -> bool {
        // TODO: Implement port scan detection
        false
    }

    fn detect_syn_flood(&self, _packet: &IPv4Packet) -> bool {
        // TODO: Implement SYN flood detection
        false
    }

    fn record_threat(&self, level: ThreatLevel, source: IPAddress, details: &str) {
        let event = ThreatEvent {
            threat_type: heapless::String::from_str("Network Attack").unwrap_or(heapless::String::new()),
            level,
            source,
            timestamp: crate::timer::get_time_ms(),
            details: heapless::String::from_str(details).unwrap_or(heapless::String::new()),
        };
        
        let mut events = self.events.lock();
        if events.is_full() {
            events.pop_front();
        }
        events.push_back(event).ok();
        
        crate::io::println!("IDS Alert [{}]: {}", 
            match level {
                ThreatLevel::Low => "LOW",
                ThreatLevel::Medium => "MEDIUM",
                ThreatLevel::High => "HIGH",
                ThreatLevel::Critical => "CRITICAL",
            },
            details
        );
    }

    pub fn get_events(&self) -> alloc::collections::VecDeque<ThreatEvent, 1024> {
        self.events.lock().clone()
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static IDS: IntrusionDetectionSystem = IntrusionDetectionSystem::new();

