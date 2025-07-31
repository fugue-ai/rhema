//! Test configuration and setup for Rhema CLI tests

use std::env;
use std::path::PathBuf;

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Whether to run slow tests
    pub run_slow_tests: bool,
    /// Whether to run integration tests
    pub run_integration_tests: bool,
    /// Whether to run performance tests
    pub run_performance_tests: bool,
    /// Whether to run security tests
    pub run_security_tests: bool,
    /// Test data directory
    pub test_data_dir: PathBuf,
    /// Maximum test timeout in seconds
    pub max_test_timeout: u64,
    /// Whether to generate coverage reports
    pub generate_coverage: bool,
    /// Whether to run benchmarks
    pub run_benchmarks: bool,
    /// Whether to run property-based tests
    pub run_property_tests: bool,
    /// Number of property test cases
    pub property_test_cases: u32,
    /// Whether to run stress tests
    pub run_stress_tests: bool,
    /// Whether to run load tests
    pub run_load_tests: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            run_slow_tests: env::var("RHEMA_RUN_SLOW_TESTS").is_ok(),
            run_integration_tests: env::var("RHEMA_RUN_INTEGRATION_TESTS").is_ok(),
            run_performance_tests: env::var("RHEMA_RUN_PERFORMANCE_TESTS").is_ok(),
            run_security_tests: env::var("RHEMA_RUN_SECURITY_TESTS").is_ok(),
            test_data_dir: PathBuf::from("tests/data"),
            max_test_timeout: env::var("RHEMA_TEST_TIMEOUT")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            generate_coverage: env::var("RHEMA_GENERATE_COVERAGE").is_ok(),
            run_benchmarks: env::var("RHEMA_RUN_BENCHMARKS").is_ok(),
            run_property_tests: env::var("RHEMA_RUN_PROPERTY_TESTS").is_ok(),
            property_test_cases: env::var("RHEMA_PROPERTY_TEST_CASES")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
            run_stress_tests: env::var("RHEMA_RUN_STRESS_TESTS").is_ok(),
            run_load_tests: env::var("RHEMA_RUN_LOAD_TESTS").is_ok(),
        }
    }
}

impl TestConfig {
    /// Create a new test configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if slow tests should be run
    pub fn should_run_slow_tests(&self) -> bool {
        self.run_slow_tests
    }
    
    /// Check if integration tests should be run
    pub fn should_run_integration_tests(&self) -> bool {
        self.run_integration_tests
    }
    
    /// Check if performance tests should be run
    pub fn should_run_performance_tests(&self) -> bool {
        self.run_performance_tests
    }
    
    /// Check if security tests should be run
    pub fn should_run_security_tests(&self) -> bool {
        self.run_security_tests
    }
    
    /// Check if benchmarks should be run
    pub fn should_run_benchmarks(&self) -> bool {
        self.run_benchmarks
    }
    
    /// Check if property tests should be run
    pub fn should_run_property_tests(&self) -> bool {
        self.run_property_tests
    }
    
    /// Check if stress tests should be run
    pub fn should_run_stress_tests(&self) -> bool {
        self.run_stress_tests
    }
    
    /// Check if load tests should be run
    pub fn should_run_load_tests(&self) -> bool {
        self.run_load_tests
    }
    
    /// Get test data directory
    pub fn test_data_dir(&self) -> &PathBuf {
        &self.test_data_dir
    }
    
    /// Get maximum test timeout
    pub fn max_test_timeout(&self) -> u64 {
        self.max_test_timeout
    }
    
    /// Get number of property test cases
    pub fn property_test_cases(&self) -> u32 {
        self.property_test_cases
    }
    
    /// Check if coverage should be generated
    pub fn should_generate_coverage(&self) -> bool {
        self.generate_coverage
    }
}

lazy_static::lazy_static! {
    pub static ref TEST_CONFIG: TestConfig = TestConfig::new();
}

/// Test environment setup
pub fn setup_test_environment() {
    // Set up logging for tests
    let _ = env_logger::try_init();
    
    // Create test data directory if it doesn't exist
    let test_data_dir = TEST_CONFIG.test_data_dir();
    if !test_data_dir.exists() {
        std::fs::create_dir_all(test_data_dir).expect("Failed to create test data directory");
    }
    
    // Set up test environment variables
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "debug");
}

/// Test environment cleanup
pub fn cleanup_test_environment() {
    // Clean up test data directory
    let test_data_dir = TEST_CONFIG.test_data_dir();
    if test_data_dir.exists() {
        std::fs::remove_dir_all(test_data_dir).expect("Failed to clean up test data directory");
    }
}

/// Test utilities
pub mod utils {
    use super::*;
    use std::time::{Duration, Instant};
    use std::panic;
    
    /// Run a test with timeout
    pub fn run_test_with_timeout<F, R>(timeout: Duration, test_fn: F) -> Result<R, String>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        let start = Instant::now();
        
        let result = panic::catch_unwind(test_fn);
        
        let elapsed = start.elapsed();
        if elapsed > timeout {
            return Err(format!("Test timed out after {:?}", elapsed));
        }
        
        match result {
            Ok(value) => Ok(value),
            Err(panic_info) => {
                if let Some(s) = panic_info.downcast_ref::<String>() {
                    Err(format!("Test panicked: {}", s))
                } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                    Err(format!("Test panicked: {}", s))
                } else {
                    Err("Test panicked with unknown error".to_string())
                }
            }
        }
    }
    
    /// Run a test with default timeout
    pub fn run_test<F, R>(test_fn: F) -> Result<R, String>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        let timeout = Duration::from_secs(TEST_CONFIG.max_test_timeout());
        run_test_with_timeout(timeout, test_fn)
    }
    
    /// Skip test if condition is not met
    pub fn skip_if_not(condition: bool, reason: &str) {
        if !condition {
            panic!("Test skipped: {}", reason);
        }
    }
    
    /// Skip slow tests if not enabled
    pub fn skip_slow_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_slow_tests(),
            "Slow tests are disabled. Set RHEMA_RUN_SLOW_TESTS=1 to enable."
        );
    }
    
    /// Skip integration tests if not enabled
    pub fn skip_integration_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_integration_tests(),
            "Integration tests are disabled. Set RHEMA_RUN_INTEGRATION_TESTS=1 to enable."
        );
    }
    
    /// Skip performance tests if not enabled
    pub fn skip_performance_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_performance_tests(),
            "Performance tests are disabled. Set RHEMA_RUN_PERFORMANCE_TESTS=1 to enable."
        );
    }
    
    /// Skip security tests if not enabled
    pub fn skip_security_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_security_tests(),
            "Security tests are disabled. Set RHEMA_RUN_SECURITY_TESTS=1 to enable."
        );
    }
    
    /// Skip benchmarks if not enabled
    pub fn skip_benchmarks() {
        skip_if_not(
            TEST_CONFIG.should_run_benchmarks(),
            "Benchmarks are disabled. Set RHEMA_RUN_BENCHMARKS=1 to enable."
        );
    }
    
    /// Skip property tests if not enabled
    pub fn skip_property_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_property_tests(),
            "Property tests are disabled. Set RHEMA_RUN_PROPERTY_TESTS=1 to enable."
        );
    }
    
    /// Skip stress tests if not enabled
    pub fn skip_stress_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_stress_tests(),
            "Stress tests are disabled. Set RHEMA_RUN_STRESS_TESTS=1 to enable."
        );
    }
    
    /// Skip load tests if not enabled
    pub fn skip_load_tests() {
        skip_if_not(
            TEST_CONFIG.should_run_load_tests(),
            "Load tests are disabled. Set RHEMA_RUN_LOAD_TESTS=1 to enable."
        );
    }
}

/// Test macros
#[macro_export]
macro_rules! test_with_timeout {
    ($timeout:expr, $test_fn:expr) => {
        #[test]
        fn test_with_timeout() {
            use crate::tests::test_config::utils::run_test_with_timeout;
            use std::time::Duration;
            
            let result = run_test_with_timeout(Duration::from_secs($timeout), $test_fn);
            assert!(result.is_ok(), "Test failed: {:?}", result.err());
        }
    };
}

#[macro_export]
macro_rules! slow_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_slow_tests;
            skip_slow_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! integration_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_integration_tests;
            skip_integration_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! performance_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_performance_tests;
            skip_performance_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! security_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_security_tests;
            skip_security_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! benchmark_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_benchmarks;
            skip_benchmarks();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! property_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_property_tests;
            skip_property_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! stress_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_stress_tests;
            skip_stress_tests();
            $test_fn
        }
    };
}

#[macro_export]
macro_rules! load_test {
    ($test_name:ident, $test_fn:expr) => {
        #[test]
        fn $test_name() {
            use crate::tests::test_config::utils::skip_load_tests;
            skip_load_tests();
            $test_fn
        }
    };
} 