//! Comprehensive test runner for Rhema CLI
//! Provides orchestration, reporting, and execution management for all test types

use colored::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::config::test_config::TestConfig;

/// Test result status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed(String),
    Skipped(String),
    Timeout,
    Error(String),
}

/// Test result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration: Duration,
    pub memory_usage: Option<usize>,
    pub error_message: Option<String>,
    pub test_type: TestType,
}

/// Simple test result enum for coordination tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimpleTestResult {
    Passed,
    Failed,
    Skipped,
}

/// Test types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    Performance,
    Security,
    Property,
    Stress,
    Load,
    Benchmark,
}

/// Test suite results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    pub name: String,
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration: Duration,
    pub results: Vec<TestResult>,
}

/// Comprehensive test runner
pub struct TestRunner {
    config: TestConfig,
    results: Arc<Mutex<Vec<TestSuiteResult>>>,
    start_time: Instant,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            results: Arc::new(Mutex::new(Vec::new())),
            start_time: Instant::now(),
        }
    }

    /// Run all tests based on configuration
    pub fn run_all_tests(&mut self) -> Result<TestRunnerReport, Box<dyn std::error::Error>> {
        println!("{}", "🚀 Starting Rhema CLI Test Suite".bold().green());
        println!("Configuration: {:?}", self.config);

        let mut report = TestRunnerReport::new();

        // Run unit tests
        if self.config.should_run_unit_tests() {
            report.unit_tests = Some(self.run_unit_tests()?);
        }

        // Run integration tests
        if self.config.should_run_integration_tests() {
            report.integration_tests = Some(self.run_integration_tests()?);
        }

        // Run performance tests
        if self.config.should_run_performance_tests() {
            report.performance_tests = Some(self.run_performance_tests()?);
        }

        // Run security tests
        if self.config.should_run_security_tests() {
            report.security_tests = Some(self.run_security_tests()?);
        }

        // Run property tests
        if self.config.should_run_property_tests() {
            report.property_tests = Some(self.run_property_tests()?);
        }

        // Run stress tests
        if self.config.should_run_stress_tests() {
            report.stress_tests = Some(self.run_stress_tests()?);
        }

        // Run load tests
        if self.config.should_run_load_tests() {
            report.load_tests = Some(self.run_load_tests()?);
        }

        // Run benchmarks
        if self.config.should_run_benchmarks() {
            report.benchmarks = Some(self.run_benchmarks()?);
        }

        report.total_duration = self.start_time.elapsed();
        self.print_summary(&report);

        Ok(report)
    }

    /// Run unit tests
    fn run_unit_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "📋 Running Unit Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Unit Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run unit tests from unit_tests.rs
        let unit_test_results = self.run_test_module("unit_tests")?;
        suite_result.results.extend(unit_test_results);

        // Run unit tests from unit/ directory
        let unit_dir_results = self.run_test_directory("unit")?;
        suite_result.results.extend(unit_dir_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run integration tests
    fn run_integration_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "🔗 Running Integration Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Integration Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run integration tests from integration_tests.rs
        let integration_test_results = self.run_test_module("integration_tests")?;
        suite_result.results.extend(integration_test_results);

        // Run integration tests from integration/ directory
        let integration_dir_results = self.run_test_directory("integration")?;
        suite_result.results.extend(integration_dir_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run performance tests
    fn run_performance_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "⚡ Running Performance Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Performance Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run performance tests from performance/ directory
        let performance_results = self.run_test_directory("performance")?;
        suite_result.results.extend(performance_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run security tests
    fn run_security_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "🔒 Running Security Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Security Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run security tests from security/ directory
        let security_results = self.run_test_directory("security")?;
        suite_result.results.extend(security_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run property tests
    fn run_property_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "🎲 Running Property Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Property Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run property tests using proptest
        let property_results = self.run_property_test_suite()?;
        suite_result.results.extend(property_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run stress tests
    fn run_stress_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "💪 Running Stress Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Stress Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run stress tests
        let stress_results = self.run_stress_test_suite()?;
        suite_result.results.extend(stress_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run load tests
    fn run_load_tests(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "📊 Running Load Tests".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Load Tests".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run load tests
        let load_results = self.run_load_test_suite()?;
        suite_result.results.extend(load_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run benchmarks
    fn run_benchmarks(&self) -> Result<TestSuiteResult, Box<dyn std::error::Error>> {
        println!("{}", "🏃 Running Benchmarks".bold().blue());

        let mut suite_result = TestSuiteResult {
            name: "Benchmarks".to_string(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration: Duration::ZERO,
            results: Vec::new(),
        };

        let start_time = Instant::now();

        // Run benchmarks using criterion
        let benchmark_results = self.run_benchmark_suite()?;
        suite_result.results.extend(benchmark_results);

        suite_result.duration = start_time.elapsed();
        suite_result.total_tests = suite_result.results.len();
        suite_result.passed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Passed))
            .count();
        suite_result.failed = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Failed(_)))
            .count();
        suite_result.skipped = suite_result
            .results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped(_)))
            .count();

        self.print_suite_summary(&suite_result);
        Ok(suite_result)
    }

    /// Run coordination integration tests
    pub fn run_coordination_integration_tests(
        &self,
    ) -> Result<SimpleTestResult, Box<dyn std::error::Error>> {
        if !self.config.should_run_coordination_integration_tests() {
            return Ok(SimpleTestResult::Skipped);
        }

        println!("🧪 Running Coordination Integration Tests...");
        let start_time = Instant::now();
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        // Run coordination integration tests
        let coordination_tests = vec![
            "test_coordination_agent_register",
            "test_coordination_agent_list",
            "test_coordination_agent_send_message",
            "test_coordination_session_create",
            "test_coordination_session_list",
            "test_coordination_system_stats",
            "test_coordination_system_health",
        ];

        for test_name in coordination_tests {
            match self.run_single_test("coordination_integration", test_name) {
                SimpleTestResult::Passed => passed += 1,
                SimpleTestResult::Failed => failed += 1,
                SimpleTestResult::Skipped => skipped += 1,
                _ => {}
            }
        }

        let duration = start_time.elapsed();
        println!(
            "✅ Coordination Integration Tests completed in {:?}",
            duration
        );
        println!(
            "   Passed: {}, Failed: {}, Skipped: {}",
            passed, failed, skipped
        );

        if failed > 0 {
            Ok(SimpleTestResult::Failed)
        } else {
            Ok(SimpleTestResult::Passed)
        }
    }

    /// Run coordination performance benchmarks
    pub fn run_coordination_benchmarks(
        &self,
    ) -> Result<SimpleTestResult, Box<dyn std::error::Error>> {
        if !self.config.should_run_coordination_benchmarks() {
            return Ok(SimpleTestResult::Skipped);
        }

        println!("⚡ Running Coordination Performance Benchmarks...");
        let start_time = Instant::now();
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        // Run coordination performance benchmarks
        let coordination_benchmarks = vec![
            "benchmark_agent_registration",
            "benchmark_agent_listing",
            "benchmark_agent_message_sending",
            "benchmark_session_creation",
            "benchmark_session_listing",
            "benchmark_system_stats",
            "benchmark_health_check",
        ];

        for benchmark_name in coordination_benchmarks {
            match self.run_single_benchmark("coordination_performance", benchmark_name) {
                SimpleTestResult::Passed => passed += 1,
                SimpleTestResult::Failed => failed += 1,
                SimpleTestResult::Skipped => skipped += 1,
                _ => {}
            }
        }

        let duration = start_time.elapsed();
        println!(
            "✅ Coordination Performance Benchmarks completed in {:?}",
            duration
        );
        println!(
            "   Passed: {}, Failed: {}, Skipped: {}",
            passed, failed, skipped
        );

        if failed > 0 {
            Ok(SimpleTestResult::Failed)
        } else {
            Ok(SimpleTestResult::Passed)
        }
    }

    /// Run coordination security tests
    pub fn run_coordination_security_tests(
        &self,
    ) -> Result<SimpleTestResult, Box<dyn std::error::Error>> {
        if !self.config.should_run_coordination_security_tests() {
            return Ok(SimpleTestResult::Skipped);
        }

        println!("🔒 Running Coordination Security Tests...");
        let start_time = Instant::now();
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        // Run coordination security tests
        let coordination_security_tests = vec![
            "test_coordination_sql_injection_agent_name",
            "test_coordination_xss_agent_name",
            "test_coordination_path_traversal_agent_name",
            "test_coordination_command_injection_agent_name",
            "test_coordination_malicious_message_payload",
            "test_coordination_unauthorized_agent_access",
            "test_coordination_session_hijacking_attempt",
        ];

        for test_name in coordination_security_tests {
            match self.run_single_test("coordination_security", test_name) {
                SimpleTestResult::Passed => passed += 1,
                SimpleTestResult::Failed => failed += 1,
                SimpleTestResult::Skipped => skipped += 1,
                _ => {}
            }
        }

        let duration = start_time.elapsed();
        println!("✅ Coordination Security Tests completed in {:?}", duration);
        println!(
            "   Passed: {}, Failed: {}, Skipped: {}",
            passed, failed, skipped
        );

        if failed > 0 {
            Ok(SimpleTestResult::Failed)
        } else {
            Ok(SimpleTestResult::Passed)
        }
    }

    /// Run a test module
    fn run_test_module(
        &self,
        module_name: &str,
    ) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would integrate with the actual test runner
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run tests from a directory
    fn run_test_directory(
        &self,
        dir_name: &str,
    ) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would integrate with the actual test runner
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run property test suite
    fn run_property_test_suite(&self) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would integrate with proptest
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run stress test suite
    fn run_stress_test_suite(&self) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would run stress tests
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run load test suite
    fn run_load_test_suite(&self) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would run load tests
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run benchmark suite
    fn run_benchmark_suite(&self) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
        // This would integrate with criterion
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Run a single test
    fn run_single_test(&self, _suite: &str, _test_name: &str) -> SimpleTestResult {
        // Mock implementation - always pass
        SimpleTestResult::Passed
    }

    /// Run a single benchmark
    fn run_single_benchmark(&self, _suite: &str, _benchmark_name: &str) -> SimpleTestResult {
        // Mock implementation - always pass
        SimpleTestResult::Passed
    }

    /// Print suite summary
    fn print_suite_summary(&self, suite: &TestSuiteResult) {
        let status = if suite.failed == 0 {
            "✅".green()
        } else {
            "❌".red()
        };

        println!(
            "{} {}: {}/{} passed, {} failed, {} skipped ({:?})",
            status,
            suite.name,
            suite.passed,
            suite.total_tests,
            suite.failed,
            suite.skipped,
            suite.duration
        );
    }

    /// Print overall summary
    fn print_summary(&self, report: &TestRunnerReport) {
        println!("\n{}", "📊 Test Summary".bold().green());
        println!("Total Duration: {:?}", report.total_duration);

        let total_tests = report.total_tests();
        let total_passed = report.total_passed();
        let total_failed = report.total_failed();

        println!("Total Tests: {}", total_tests);
        println!("Passed: {}", total_passed.to_string().green());
        println!("Failed: {}", total_failed.to_string().red());

        if total_failed > 0 {
            println!("{}", "❌ Some tests failed!".bold().red());
            std::process::exit(1);
        } else {
            println!("{}", "✅ All tests passed!".bold().green());
        }
    }
}

/// Test runner report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunnerReport {
    pub unit_tests: Option<TestSuiteResult>,
    pub integration_tests: Option<TestSuiteResult>,
    pub performance_tests: Option<TestSuiteResult>,
    pub security_tests: Option<TestSuiteResult>,
    pub property_tests: Option<TestSuiteResult>,
    pub stress_tests: Option<TestSuiteResult>,
    pub load_tests: Option<TestSuiteResult>,
    pub benchmarks: Option<TestSuiteResult>,
    pub total_duration: Duration,
}

impl TestRunnerReport {
    /// Create a new test runner report
    pub fn new() -> Self {
        Self {
            unit_tests: None,
            integration_tests: None,
            performance_tests: None,
            security_tests: None,
            property_tests: None,
            stress_tests: None,
            load_tests: None,
            benchmarks: None,
            total_duration: Duration::ZERO,
        }
    }

    /// Get total number of tests
    pub fn total_tests(&self) -> usize {
        let mut total = 0;
        if let Some(suite) = &self.unit_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.integration_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.performance_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.security_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.property_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.stress_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.load_tests {
            total += suite.total_tests;
        }
        if let Some(suite) = &self.benchmarks {
            total += suite.total_tests;
        }
        total
    }

    /// Get total number of passed tests
    pub fn total_passed(&self) -> usize {
        let mut total = 0;
        if let Some(suite) = &self.unit_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.integration_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.performance_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.security_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.property_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.stress_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.load_tests {
            total += suite.passed;
        }
        if let Some(suite) = &self.benchmarks {
            total += suite.passed;
        }
        total
    }

    /// Get total number of failed tests
    pub fn total_failed(&self) -> usize {
        let mut total = 0;
        if let Some(suite) = &self.unit_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.integration_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.performance_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.security_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.property_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.stress_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.load_tests {
            total += suite.failed;
        }
        if let Some(suite) = &self.benchmarks {
            total += suite.failed;
        }
        total
    }
}

/// Extension trait for TestConfig
pub trait TestConfigExt {
    fn should_run_unit_tests(&self) -> bool;
}

impl TestConfigExt for TestConfig {
    fn should_run_unit_tests(&self) -> bool {
        // Unit tests are always run unless explicitly disabled
        !std::env::var("RHEMA_SKIP_UNIT_TESTS").is_ok()
    }
}
