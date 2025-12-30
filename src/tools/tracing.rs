use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub event_type: heapless::String<32>,
    pub function: heapless::String<64>,
    pub timestamp: u64,
    pub data: heapless::Vec<u8, 256>,
}

pub struct Tracing {
    events: Mutex<alloc::collections::VecDeque<TraceEvent, 2048>>,
    enabled: Mutex<bool>,
    trace_functions: Mutex<alloc::collections::BTreeSet<heapless::String<64>>>,
}

impl Tracing {
    pub const fn new() -> Self {
        Tracing {
            events: Mutex::new(alloc::collections::VecDeque::new()),
            enabled: Mutex::new(false),
            trace_functions: Mutex::new(alloc::collections::BTreeSet::new()),
        }
    }

    pub fn enable(&self) {
        *self.enabled.lock() = true;
    }

    pub fn disable(&self) {
        *self.enabled.lock() = false;
    }

    pub fn trace_function(&self, function: &str) {
        let func_str = heapless::String::from_str(function).unwrap_or(heapless::String::new());
        self.trace_functions.lock().insert(func_str);
    }

    pub fn record_event(&self, event_type: &str, function: &str, data: &[u8]) {
        if !*self.enabled.lock() {
            return;
        }
        
        let func_str = heapless::String::from_str(function).unwrap_or(heapless::String::new());
        if !self.trace_functions.lock().contains(&func_str) {
            return;
        }
        
        let mut event_data = heapless::Vec::new();
        event_data.extend_from_slice(data).ok();
        
        let event = TraceEvent {
            event_type: heapless::String::from_str(event_type).unwrap_or(heapless::String::new()),
            function: func_str,
            timestamp: crate::timer::get_time_ms(),
            data: event_data,
        };
        
        let mut events = self.events.lock();
        if events.is_full() {
            events.pop_front();
        }
        events.push_back(event).ok();
    }

    pub fn get_events(&self) -> alloc::collections::VecDeque<TraceEvent, 2048> {
        self.events.lock().clone()
    }
}

pub static TRACING: Tracing = Tracing::new();

