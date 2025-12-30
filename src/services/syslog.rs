use spin::Mutex;
use alloc::collections::VecDeque;
use heapless::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub facility: String<32>,
    pub message: String<256>,
    pub timestamp: u64,
}

pub struct Syslog {
    entries: Mutex<VecDeque<LogEntry, 1024>>,
    min_level: Mutex<LogLevel>,
}

impl Syslog {
    pub const fn new() -> Self {
        Syslog {
            entries: Mutex::new(VecDeque::new()),
            min_level: Mutex::new(LogLevel::Debug),
        }
    }

    pub fn log(&self, level: LogLevel, facility: &str, message: &str) {
        if level as u8 > *self.min_level.lock() as u8 {
            return;
        }
        
        let entry = LogEntry {
            level,
            facility: String::from_str(facility).unwrap_or(String::new()),
            message: String::from_str(message).unwrap_or(String::new()),
            timestamp: crate::timer::get_time_ms(),
        };
        
        let mut entries = self.entries.lock();
        if entries.is_full() {
            entries.pop_front();
        }
        entries.push_back(entry).ok();
        
        // Also print to console
        let level_str = match level {
            LogLevel::Emergency => "EMERG",
            LogLevel::Alert => "ALERT",
            LogLevel::Critical => "CRIT",
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARN",
            LogLevel::Notice => "NOTICE",
            LogLevel::Informational => "INFO",
            LogLevel::Debug => "DEBUG",
        };
        
        crate::io::println!("[{}] {}: {}", level_str, facility, message);
    }

    pub fn set_min_level(&self, level: LogLevel) {
        *self.min_level.lock() = level;
    }

    pub fn get_entries(&self) -> VecDeque<LogEntry, 1024> {
        self.entries.lock().clone()
    }
}

pub static SYSLOG: Syslog = Syslog::new();

#[macro_export]
macro_rules! log_emerg {
    ($facility:expr, $($arg:tt)*) => {
        $crate::services::syslog::SYSLOG.log(
            $crate::services::syslog::LogLevel::Emergency,
            $facility,
            &alloc::format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! log_error {
    ($facility:expr, $($arg:tt)*) => {
        $crate::services::syslog::SYSLOG.log(
            $crate::services::syslog::LogLevel::Error,
            $facility,
            &alloc::format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! log_info {
    ($facility:expr, $($arg:tt)*) => {
        $crate::services::syslog::SYSLOG.log(
            $crate::services::syslog::LogLevel::Informational,
            $facility,
            &alloc::format!($($arg)*)
        );
    };
}

