use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct PhysicalVolume {
    pub device_id: u64,
    pub size_blocks: u64,
    pub free_blocks: u64,
}

#[derive(Debug, Clone)]
pub struct VolumeGroup {
    pub name: heapless::String<64>,
    pub physical_volumes: Vec<PhysicalVolume>,
    pub logical_volumes: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct LogicalVolume {
    pub lv_id: u64,
    pub name: heapless::String<64>,
    pub size_blocks: u64,
    pub extents: Vec<u64>, // Physical extents
}

pub struct LvmManager {
    volume_groups: Mutex<BTreeMap<heapless::String<64>, VolumeGroup>>,
    logical_volumes: Mutex<BTreeMap<u64, LogicalVolume>>,
    next_lv_id: Mutex<u64>,
}

impl LvmManager {
    pub const fn new() -> Self {
        LvmManager {
            volume_groups: Mutex::new(BTreeMap::new()),
            logical_volumes: Mutex::new(BTreeMap::new()),
            next_lv_id: Mutex::new(1),
        }
    }

    pub fn create_volume_group(&self, name: &str, pvs: Vec<PhysicalVolume>) -> Result<(), &'static str> {
        let name_str = heapless::String::from_str(name).map_err(|_| "Name too long")?;
        
        let vg = VolumeGroup {
            name: name_str.clone(),
            physical_volumes: pvs,
            logical_volumes: Vec::new(),
        };
        
        self.volume_groups.lock().insert(name_str, vg);
        Ok(())
    }

    pub fn create_logical_volume(&self, vg_name: &str, lv_name: &str, size_blocks: u64) -> Result<u64, &'static str> {
        let vg_name_str = heapless::String::from_str(vg_name).map_err(|_| "VG name too long")?;
        let lv_name_str = heapless::String::from_str(lv_name).map_err(|_| "LV name too long")?;
        
        let mut vgs = self.volume_groups.lock();
        let vg = vgs.get_mut(&vg_name_str).ok_or("Volume group not found")?;
        
        // Check available space
        let total_free: u64 = vg.physical_volumes.iter().map(|pv| pv.free_blocks).sum();
        if size_blocks > total_free {
            return Err("Insufficient space in volume group");
        }
        
        let lv_id = {
            let mut next = self.next_lv_id.lock();
            let id = *next;
            *next += 1;
            id
        };
        
        // Allocate extents (simplified - just track size)
        let extents = Vec::new(); // TODO: Actually allocate extents
        
        let lv = LogicalVolume {
            lv_id,
            name: lv_name_str,
            size_blocks,
            extents,
        };
        
        vg.logical_volumes.push(lv_id);
        self.logical_volumes.lock().insert(lv_id, lv);
        
        Ok(lv_id)
    }

    pub fn get_logical_volume(&self, lv_id: u64) -> Option<LogicalVolume> {
        self.logical_volumes.lock().get(&lv_id).cloned()
    }

    pub fn map_block(&self, lv_id: u64, logical_block: u64) -> Result<(u64, u64), &'static str> {
        let lv = self.logical_volumes.lock().get(&lv_id).ok_or("Logical volume not found")?;
        
        if logical_block >= lv.size_blocks {
            return Err("Block out of range");
        }
        
        // TODO: Map to physical extent
        // For now, return dummy mapping
        Ok((0, logical_block))
    }
}

pub static LVM_MANAGER: LvmManager = LvmManager::new();

