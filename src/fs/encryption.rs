use crate::security::random::SecureRandom;
use spin::Mutex;

pub struct EncryptionKey {
    key: [u8; 32],
}

impl EncryptionKey {
    pub fn new() -> Self {
        let mut rng = SecureRandom::new();
        let mut key = [0u8; 32];
        rng.next_bytes(&mut key);
        EncryptionKey { key }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        EncryptionKey { key: *bytes }
    }

    pub fn encrypt_block(&self, plaintext: &[u8; 512], ciphertext: &mut [u8; 512]) {
        // Simple XOR encryption (should be replaced with AES in production)
        let mut rng = SecureRandom::new();
        let iv = rng.next_u64();
        
        for i in 0..512 {
            let key_byte = self.key[i % 32];
            ciphertext[i] = plaintext[i] ^ key_byte ^ ((iv >> (i % 8)) as u8);
        }
    }

    pub fn decrypt_block(&self, ciphertext: &[u8; 512], plaintext: &mut [u8; 512]) {
        // XOR is symmetric
        self.encrypt_block(ciphertext, plaintext);
    }
}

pub struct FileSystemEncryption {
    key: Mutex<Option<EncryptionKey>>,
    enabled: Mutex<bool>,
}

impl FileSystemEncryption {
    pub const fn new() -> Self {
        FileSystemEncryption {
            key: Mutex::new(None),
            enabled: Mutex::new(false),
        }
    }

    pub fn set_key(&self, key: EncryptionKey) {
        *self.key.lock() = Some(key);
    }

    pub fn encrypt_block(&self, block: &[u8; 512], encrypted: &mut [u8; 512]) -> Result<(), &'static str> {
        if !*self.enabled.lock() {
            encrypted.copy_from_slice(block);
            return Ok(());
        }

        let key = self.key.lock().as_ref().ok_or("Encryption key not set")?;
        key.encrypt_block(block, encrypted);
        Ok(())
    }

    pub fn decrypt_block(&self, encrypted: &[u8; 512], block: &mut [u8; 512]) -> Result<(), &'static str> {
        if !*self.enabled.lock() {
            block.copy_from_slice(encrypted);
            return Ok(());
        }

        let key = self.key.lock().as_ref().ok_or("Encryption key not set")?;
        key.decrypt_block(encrypted, block);
        Ok(())
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }
}

pub static FS_ENCRYPTION: FileSystemEncryption = FileSystemEncryption::new();

