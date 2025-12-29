use super::{Inode, FileType, BLOCK_DEVICE, BLOCK_SIZE};
use spin::Mutex;
use alloc::collections::BTreeMap;

pub struct FileSystem {
    inodes: Mutex<BTreeMap<u64, Inode>>,
    next_inode: Mutex<u64>,
    root_inode: u64,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = FileSystem {
            inodes: Mutex::new(BTreeMap::new()),
            next_inode: Mutex::new(1),
            root_inode: 0,
        };
        
        // Create root inode
        let root = Inode::new(0, FileType::Directory);
        fs.root_inode = 0;
        fs.inodes.lock().insert(0, root);
        *fs.next_inode.lock() = 1;
        
        fs
    }

    pub fn create_file(&self, path: &str, file_type: FileType) -> Result<u64, &'static str> {
        let inode_number = {
            let mut next = self.next_inode.lock();
            let num = *next;
            *next += 1;
            num
        };
        
        let inode = Inode::new(inode_number, file_type);
        self.inodes.lock().insert(inode_number, inode);
        Ok(inode_number)
    }

    pub fn read_file(&self, inode_number: u64, offset: u64, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let inodes = self.inodes.lock();
        let inode = inodes.get(&inode_number).ok_or("File not found")?;
        
        if offset >= inode.size {
            return Ok(0);
        }
        
        let to_read = core::cmp::min(buffer.len(), (inode.size - offset) as usize);
        let start_block = (offset / BLOCK_SIZE as u64) as usize;
        
        let mut read = 0;
        for i in 0..((to_read + BLOCK_SIZE - 1) / BLOCK_SIZE) {
            if let Some(&block_num) = inode.blocks.get(start_block + i) {
                let mut block_data = [0u8; BLOCK_SIZE];
                BLOCK_DEVICE.read_block(block_num, &mut block_data)?;
                
                let block_offset = if i == 0 { (offset % BLOCK_SIZE as u64) as usize } else { 0 };
                let copy_len = core::cmp::min(BLOCK_SIZE - block_offset, to_read - read);
                
                buffer[read..read + copy_len].copy_from_slice(&block_data[block_offset..block_offset + copy_len]);
                read += copy_len;
            }
        }
        
        Ok(read)
    }

    pub fn write_file(&self, inode_number: u64, offset: u64, data: &[u8]) -> Result<usize, &'static str> {
        let mut inodes = self.inodes.lock();
        let inode = inodes.get_mut(&inode_number).ok_or("File not found")?;
        
        let start_block = (offset / BLOCK_SIZE as u64) as usize;
        let end_block = ((offset + data.len() as u64) / BLOCK_SIZE as u64) as usize;
        
        // Allocate blocks if needed
        while inode.blocks.len() <= end_block {
            // TODO: Allocate new block from free block list
            inode.blocks.push(inode.blocks.len() as u64);
        }
        
        let mut written = 0;
        for i in start_block..=end_block {
            let block_num = inode.blocks[i];
            let mut block_data = [0u8; BLOCK_SIZE];
            
            if i == start_block || i == end_block {
                BLOCK_DEVICE.read_block(block_num, &mut block_data)?;
            }
            
            let block_offset = if i == start_block { (offset % BLOCK_SIZE as u64) as usize } else { 0 };
            let copy_len = core::cmp::min(BLOCK_SIZE - block_offset, data.len() - written);
            
            block_data[block_offset..block_offset + copy_len].copy_from_slice(&data[written..written + copy_len]);
            BLOCK_DEVICE.write_block(block_num, &block_data)?;
            
            written += copy_len;
        }
        
        inode.size = core::cmp::max(inode.size, offset + written as u64);
        Ok(written)
    }

    pub fn delete_file(&self, inode_number: u64) -> Result<(), &'static str> {
        let mut inodes = self.inodes.lock();
        inodes.remove(&inode_number).ok_or("File not found")?;
        Ok(())
    }

    pub fn get_root_inode(&self) -> u64 {
        self.root_inode
    }
}

pub static FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new());

