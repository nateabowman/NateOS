use bitflags::bitflags;
use crate::process::ProcessId;

bitflags! {
    pub struct AclPermissions: u16 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
        const DELETE = 1 << 3;
    }
}

#[derive(Debug, Clone)]
pub struct AclEntry {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub permissions: AclPermissions,
}

#[derive(Debug, Clone)]
pub struct AccessControlList {
    pub owner: u32,
    pub group: u32,
    pub owner_perms: AclPermissions,
    pub group_perms: AclPermissions,
    pub other_perms: AclPermissions,
    pub entries: alloc::vec::Vec<AclEntry>,
}

impl AccessControlList {
    pub fn new(owner: u32, group: u32) -> Self {
        AccessControlList {
            owner,
            group,
            owner_perms: AclPermissions::all(),
            group_perms: AclPermissions::READ | AclPermissions::EXECUTE,
            other_perms: AclPermissions::READ | AclPermissions::EXECUTE,
            entries: alloc::vec::Vec::new(),
        }
    }

    pub fn check_permission(&self, pid: ProcessId, permission: AclPermissions) -> bool {
        // TODO: Get user/group from process
        let user_id = 0; // Placeholder
        let group_id = 0; // Placeholder

        // Check owner
        if user_id == self.owner {
            return self.owner_perms.contains(permission);
        }

        // Check group
        if group_id == self.group {
            return self.group_perms.contains(permission);
        }

        // Check ACL entries
        for entry in &self.entries {
            if let Some(uid) = entry.user_id {
                if uid == user_id && entry.permissions.contains(permission) {
                    return true;
                }
            }
            if let Some(gid) = entry.group_id {
                if gid == group_id && entry.permissions.contains(permission) {
                    return true;
                }
            }
        }

        // Check other
        self.other_perms.contains(permission)
    }

    pub fn add_entry(&mut self, entry: AclEntry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, user_id: Option<u32>, group_id: Option<u32>) {
        self.entries.retain(|e| e.user_id != user_id || e.group_id != group_id);
    }
}

pub struct AclManager {
    acls: spin::Mutex<alloc::collections::BTreeMap<u64, AccessControlList>>,
}

impl AclManager {
    pub const fn new() -> Self {
        AclManager {
            acls: spin::Mutex::new(alloc::collections::BTreeMap::new()),
        }
    }

    pub fn set_acl(&self, inode: u64, acl: AccessControlList) {
        self.acls.lock().insert(inode, acl);
    }

    pub fn get_acl(&self, inode: u64) -> Option<AccessControlList> {
        self.acls.lock().get(&inode).cloned()
    }

    pub fn check_access(&self, inode: u64, pid: ProcessId, permission: AclPermissions) -> bool {
        if let Some(acl) = self.get_acl(inode) {
            acl.check_permission(pid, permission)
        } else {
            // Default permissions
            permission.contains(AclPermissions::READ | AclPermissions::EXECUTE)
        }
    }
}

pub static ACL_MANAGER: AclManager = AclManager::new();

