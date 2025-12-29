use spin::Mutex;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: heapless::String<64>,
    pub passed: bool,
    pub message: heapless::String<256>,
}

pub struct TestFramework {
    tests: Mutex<Vec<TestResult>>,
}

impl TestFramework {
    pub const fn new() -> Self {
        TestFramework {
            tests: Mutex::new(Vec::new()),
        }
    }

    pub fn run_test<F>(&self, name: &str, test_fn: F) -> bool
    where
        F: FnOnce() -> Result<(), &'static str>,
    {
        let result = test_fn();
        let passed = result.is_ok();
        let message = result.err().unwrap_or("Test passed");
        
        let test_result = TestResult {
            name: heapless::String::from_str(name).unwrap_or(heapless::String::new()),
            passed,
            message: heapless::String::from_str(message).unwrap_or(heapless::String::new()),
        };
        
        self.tests.lock().push(test_result.clone());
        
        if passed {
            crate::io::println!("[PASS] {}", name);
        } else {
            crate::io::println!("[FAIL] {}: {}", name, message);
        }
        
        passed
    }

    pub fn get_results(&self) -> Vec<TestResult> {
        self.tests.lock().clone()
    }

    pub fn print_summary(&self) {
        let results = self.get_results();
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        
        crate::io::println!("Test Summary: {}/{} tests passed", passed, total);
    }
}

pub static TEST_FRAMEWORK: TestFramework = TestFramework::new();

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(concat!("Assertion failed: ", stringify!($left), " != ", stringify!($right)));
        }
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        if $left == $right {
            return Err(concat!("Assertion failed: ", stringify!($left), " == ", stringify!($right)));
        }
    };
}

