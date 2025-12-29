use spin::Mutex;
use alloc::collections::BTreeMap;
use heapless::String;

#[derive(Debug, Clone)]
pub struct User {
    pub uid: u32,
    pub gid: u32,
    pub username: String<64>,
    pub home_directory: String<256>,
}

pub struct UserManager {
    users: Mutex<BTreeMap<u32, User>>,
    current_user: Mutex<Option<u32>>,
    next_uid: Mutex<u32>,
}

impl UserManager {
    pub const fn new() -> Self {
        UserManager {
            users: Mutex::new(BTreeMap::new()),
            current_user: Mutex::new(None),
            next_uid: Mutex::new(1000),
        }
    }

    pub fn create_user(&self, username: &str) -> Result<u32, &'static str> {
        let uid = {
            let mut next = self.next_uid.lock();
            let uid = *next;
            *next += 1;
            uid
        };
        
        let user = User {
            uid,
            gid: uid,
            username: String::from_str(username).map_err(|_| "Username too long")?,
            home_directory: String::from_str("/home/").map_err(|_| "Path too long")?,
        };
        
        self.users.lock().insert(uid, user);
        Ok(uid)
    }

    pub fn get_user(&self, uid: u32) -> Option<User> {
        self.users.lock().get(&uid).cloned()
    }

    pub fn get_current_user(&self) -> Option<u32> {
        *self.current_user.lock()
    }

    pub fn set_current_user(&self, uid: u32) -> Result<(), &'static str> {
        if self.users.lock().contains_key(&uid) {
            *self.current_user.lock() = Some(uid);
            Ok(())
        } else {
            Err("User not found")
        }
    }

    pub fn authenticate(&self, username: &str, _password: &str) -> Result<u32, &'static str> {
        // TODO: Implement proper password authentication
        let users = self.users.lock();
        for (uid, user) in users.iter() {
            if user.username.as_str() == username {
                return Ok(*uid);
            }
        }
        Err("Authentication failed")
    }
}

pub static USER_MANAGER: UserManager = UserManager::new();

