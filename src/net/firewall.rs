use crate::net::ip::{IPv4Packet, IPAddress};
use spin::Mutex;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub src_addr: Option<IPAddress>,
    pub dst_addr: Option<IPAddress>,
    pub action: FirewallAction,
}

pub struct Firewall {
    rules: Mutex<Vec<FirewallRule>>,
    default_action: Mutex<FirewallAction>,
}

impl Firewall {
    pub const fn new() -> Self {
        Firewall {
            rules: Mutex::new(Vec::new()),
            default_action: Mutex::new(FirewallAction::Allow),
        }
    }

    pub fn add_rule(&self, rule: FirewallRule) {
        self.rules.lock().push(rule);
    }

    pub fn check_packet(&self, packet: &IPv4Packet) -> bool {
        let rules = self.rules.lock();
        
        for rule in rules.iter() {
            let matches = (rule.src_addr.is_none() || rule.src_addr == Some(packet.header.src_addr))
                && (rule.dst_addr.is_none() || rule.dst_addr == Some(packet.header.dst_addr));
            
            if matches {
                return match rule.action {
                    FirewallAction::Allow => true,
                    FirewallAction::Deny => false,
                };
            }
        }
        
        match *self.default_action.lock() {
            FirewallAction::Allow => true,
            FirewallAction::Deny => false,
        }
    }

    pub fn set_default_action(&self, action: FirewallAction) {
        *self.default_action.lock() = action;
    }
}

pub static FIREWALL: Firewall = Firewall::new();

