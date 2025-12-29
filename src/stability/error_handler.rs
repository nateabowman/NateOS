use spin::Mutex;
use alloc::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ErrorRecord {
    pub error_type: heapless::String<64>,
    pub message: heapless::String<256>,
    pub timestamp: u64,
}

pub struct ErrorHandler {
    errors: Mutex<VecDeque<ErrorRecord, 1024>>,
    panic_on_error: Mutex<bool>,
}

impl ErrorHandler {
    pub const fn new() -> Self {
        ErrorHandler {
            errors: Mutex::new(VecDeque::new()),
            panic_on_error: Mutex::new(false),
        }
    }

    pub fn handle_error(&self, error_type: &str, message: &str) {
        let record = ErrorRecord {
            error_type: heapless::String::from_str(error_type).unwrap_or(heapless::String::new()),
            message: heapless::String::from_str(message).unwrap_or(heapless::String::new()),
            timestamp: crate::timer::get_time_ms(),
        };
        
        let mut errors = self.errors.lock();
        if errors.is_full() {
            errors.pop_front();
        }
        errors.push_back(record).ok();
        
        if *self.panic_on_error.lock() {
            panic!("Error: {} - {}", error_type, message);
        }
    }

    pub fn get_errors(&self) -> VecDeque<ErrorRecord, 1024> {
        self.errors.lock().clone()
    }

    pub fn clear_errors(&self) {
        self.errors.lock().clear();
    }
}

pub static ERROR_HANDLER: ErrorHandler = ErrorHandler::new();

