#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Device,
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub inode_number: u64,
    pub file_type: FileType,
    pub size: u64,
    pub blocks: Vec<u64>,
    pub permissions: u16,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

impl Inode {
    pub fn new(inode_number: u64, file_type: FileType) -> Self {
        Inode {
            inode_number,
            file_type,
            size: 0,
            blocks: Vec::new(),
            permissions: 0o644,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }
}

