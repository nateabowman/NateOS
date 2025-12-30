use crate::net::ip::{IPv4Packet, IPAddress};
use crate::net::tcp::TCPHeader;
use spin::Mutex;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
    Log,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub action: FirewallAction,
    pub src_addr: Option<IPAddress>,
    pub dst_addr: Option<IPAddress>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Option<u8>,
    pub state: Option<ConnectionState>,
    pub priority: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    New,
    Established,
    Related,
    Invalid,
}

pub struct StatefulFirewall {
    rules: Mutex<Vec<FirewallRule>>,
    connections: Mutex<alloc::collections::BTreeMap<(IPAddress, IPAddress, u16, u16), ConnectionState>>,
    default_action: Mutex<FirewallAction>,
}

impl StatefulFirewall {
    pub const fn new() -> Self {
        StatefulFirewall {
            rules: Mutex::new(Vec::new()),
            connections: Mutex::new(alloc::collections::BTreeMap::new()),
            default_action: Mutex::new(FirewallAction::Deny),
        }
    }

    pub fn add_rule(&self, rule: FirewallRule) {
        let mut rules = self.rules.lock();
        rules.push(rule);
        rules.sort_by_key(|r| r.priority);
    }

    pub fn check_packet(&self, packet: &IPv4Packet) -> bool {
        // Check connection state
        let conn_key = (packet.header.src_addr, packet.header.dst_addr, 0, 0);
        let state = self.connections.lock().get(&conn_key).copied();
        
        // Match against rules
        let rules = self.rules.lock();
        for rule in rules.iter() {
            if self.rule_matches(&rule, packet, state) {
                match rule.action {
                    FirewallAction::Allow => {
                        self.update_connection_state(packet, ConnectionState::Established);
                        return true;
                    }
                    FirewallAction::Deny | FirewallAction::Reject => {
                        return false;
                    }
                    FirewallAction::Log => {
                        crate::io::println!("Firewall: Logging packet");
                        continue;
                    }
                }
            }
        }
        
        // Default action
        match *self.default_action.lock() {
            FirewallAction::Allow => true,
            _ => false,
        }
    }

    fn rule_matches(&self, rule: &FirewallRule, packet: &IPv4Packet, state: Option<ConnectionState>) -> bool {
        if let Some(rule_state) = rule.state {
            if state != Some(rule_state) {
                return false;
            }
        }
        
        if let Some(rule_src) = rule.src_addr {
            if rule_src != packet.header.src_addr {
                return false;
            }
        }
        
        if let Some(rule_dst) = rule.dst_addr {
            if rule_dst != packet.header.dst_addr {
                return false;
            }
        }
        
        if let Some(rule_proto) = rule.protocol {
            if rule_proto != packet.header.protocol {
                return false;
            }
        }
        
        true
    }

    fn update_connection_state(&self, packet: &IPv4Packet, state: ConnectionState) {
        let key = (packet.header.src_addr, packet.header.dst_addr, 0, 0);
        self.connections.lock().insert(key, state);
    }

    pub fn set_default_action(&self, action: FirewallAction) {
        *self.default_action.lock() = action;
    }
}

pub static STATEFUL_FIREWALL: StatefulFirewall = StatefulFirewall::new();

