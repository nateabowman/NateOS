use spin::Mutex;
use alloc::collections::BTreeMap;
use x86_64::PhysAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId(pub u32);

pub struct NumaNode {
    pub node_id: NodeId,
    pub memory_start: PhysAddr,
    pub memory_end: PhysAddr,
    pub cpu_mask: u64,
}

pub struct NumaManager {
    nodes: Mutex<BTreeMap<NodeId, NumaNode>>,
    current_node: Mutex<Option<NodeId>>,
}

impl NumaManager {
    pub const fn new() -> Self {
        NumaManager {
            nodes: Mutex::new(BTreeMap::new()),
            current_node: Mutex::new(None),
        }
    }

    pub fn add_node(&self, node: NumaNode) {
        self.nodes.lock().insert(node.node_id, node);
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<NumaNode> {
        self.nodes.lock().get(&node_id).copied()
    }

    pub fn get_nearest_node(&self, addr: PhysAddr) -> Option<NodeId> {
        let nodes = self.nodes.lock();
        let mut nearest = None;
        let mut min_distance = u64::MAX;

        for (node_id, node) in nodes.iter() {
            if addr >= node.memory_start && addr <= node.memory_end {
                return Some(*node_id);
            }
            let distance = if addr < node.memory_start {
                node.memory_start.as_u64() - addr.as_u64()
            } else {
                addr.as_u64() - node.memory_end.as_u64()
            };
            if distance < min_distance {
                min_distance = distance;
                nearest = Some(*node_id);
            }
        }
        nearest
    }

    pub fn allocate_on_node(&self, node_id: NodeId, size: usize) -> Option<PhysAddr> {
        if let Some(node) = self.get_node(node_id) {
            // TODO: Implement actual allocation on specific node
            Some(node.memory_start)
        } else {
            None
        }
    }

    pub fn get_current_node(&self) -> Option<NodeId> {
        *self.current_node.lock()
    }

    pub fn set_current_node(&self, node_id: NodeId) {
        *self.current_node.lock() = Some(node_id);
    }
}

pub static NUMA_MANAGER: NumaManager = NumaManager::new();

