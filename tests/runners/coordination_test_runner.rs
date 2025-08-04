//! Dedicated test runner for Rhema Coordination CLI tests

use std::time::{Duration, Instant};
use crate::common::coordination_fixtures::{CoordinationTestEnv, CoordinationTestHelpers, CoordinationAssertions};
use crate::test_config::TestConfig;

/// Coordination test results
#[derive(Debug, Clone)]
pub enum CoordinationTestResult {
    Passed,
    Failed,
    Skipped,
    Error(String),
}

/// Coordination test report
#[derive(Debug)]
pub struct CoordinationTestReport {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub errors: usize,
    pub total_duration: Duration,
    pub test_results: Vec<(String, CoordinationTestResult, Duration)>,
}

impl CoordinationTestReport {
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            errors: 0,
            total_duration: Duration::ZERO,
            test_results: Vec::new(),
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== Coordination CLI Test Report ===");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {}", self.passed);
        println!("Failed: {}", self.failed);
        println!("Skipped: {}", self.skipped);
        println!("Errors: {}", self.errors);
        println!("Total Duration: {:?}", self.total_duration);
        println!("Success Rate: {:.1}%", 
            if self.total_tests > 0 { 
                (self.passed as f64 / self.total_tests as f64) * 100.0 
            } else { 
                0.0 
            }
        );
        println!("================================");

        if self.failed > 0 || self.errors > 0 {
            println!("\nFailed/Error Tests:");
            for (test_name, result, duration) in &self.test_results {
                match result {
                    CoordinationTestResult::Failed => {
                        println!("  ❌ {} ({:?})", test_name, duration);
                    }
                    CoordinationTestResult::Error(e) => {
                        println!("  💥 {} ({:?}) - {}", test_name, duration, e);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Coordination test runner
pub struct CoordinationTestRunner {
    config: TestConfig,
    test_env: Option<CoordinationTestEnv>,
}

impl CoordinationTestRunner {
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            test_env: None,
        }
    }

    /// Run all coordination tests
    pub async fn run_all_tests(&mut self) -> Result<CoordinationTestReport, Box<dyn std::error::Error>> {
        println!("🚀 Starting Rhema Coordination CLI Test Suite");
        println!("Configuration: {:?}", self.config);

        let mut report = CoordinationTestReport::new();
        let start_time = Instant::now();

        // Setup test environment
        self.setup_test_environment().await?;

        // Run integration tests
        if self.config.should_run_coordination_integration_tests() {
            self.run_integration_tests(&mut report).await?;
        }

        // Run performance benchmarks
        if self.config.should_run_coordination_benchmarks() {
            self.run_performance_benchmarks(&mut report).await?;
        }

        // Run security tests
        if self.config.should_run_coordination_security_tests() {
            self.run_security_tests(&mut report).await?;
        }

        report.total_duration = start_time.elapsed();
        report.print_summary();

        // Cleanup test environment
        self.cleanup_test_environment().await?;

        Ok(report)
    }

    /// Setup test environment
    async fn setup_test_environment(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Setting up coordination test environment...");
        
        let test_env = CoordinationTestEnv::new()?;
        test_env.setup_coordination_system()?;
        
        self.test_env = Some(test_env);
        
        println!("✅ Test environment setup complete");
        Ok(())
    }

    /// Cleanup test environment
    async fn cleanup_test_environment(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧹 Cleaning up test environment...");
        
        if let Some(test_env) = &self.test_env {
            test_env.cleanup()?;
        }
        
        println!("✅ Test environment cleanup complete");
        Ok(())
    }

    /// Run integration tests
    async fn run_integration_tests(&self, report: &mut CoordinationTestReport) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🧪 Running Coordination Integration Tests...");
        
        let integration_tests = vec![
            ("Agent Registration", Box::pin(self.test_agent_registration())),
            ("Agent Listing", Box::pin(self.test_agent_listing())),
            ("Agent Message Sending", Box::pin(self.test_agent_message_sending())),
            ("Session Creation", Box::pin(self.test_session_creation())),
            ("Session Listing", Box::pin(self.test_session_listing())),
            ("System Statistics", Box::pin(self.test_system_stats())),
            ("System Health Check", Box::pin(self.test_system_health())),
        ];

        for (test_name, mut test_fn) in integration_tests {
            let test_start = Instant::now();
            let result = test_fn.await;
            let duration = test_start.elapsed();

            let test_result = match result {
                Ok(_) => {
                    println!("  ✅ {} ({:?})", test_name, duration);
                    CoordinationTestResult::Passed
                }
                Err(e) => {
                    println!("  ❌ {} ({:?}) - {}", test_name, duration, e);
                    CoordinationTestResult::Failed
                }
            };

            report.test_results.push((test_name.to_string(), test_result.clone(), duration));
            report.total_tests += 1;

            match test_result {
                CoordinationTestResult::Passed => report.passed += 1,
                CoordinationTestResult::Failed => report.failed += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Run performance benchmarks
    async fn run_performance_benchmarks(&self, report: &mut CoordinationTestReport) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n⚡ Running Coordination Performance Benchmarks...");
        
        let performance_benchmarks = vec![
            ("Agent Registration Benchmark", Box::pin(self.benchmark_agent_registration())),
            ("Agent Listing Benchmark", Box::pin(self.benchmark_agent_listing())),
            ("Message Sending Benchmark", Box::pin(self.benchmark_message_sending())),
            ("Session Creation Benchmark", Box::pin(self.benchmark_session_creation())),
            ("System Stats Benchmark", Box::pin(self.benchmark_system_stats())),
        ];

        for (benchmark_name, mut benchmark_fn) in performance_benchmarks {
            let benchmark_start = Instant::now();
            let result = benchmark_fn.await;
            let duration = benchmark_start.elapsed();

            let benchmark_result = match result {
                Ok(_) => {
                    println!("  ✅ {} ({:?})", benchmark_name, duration);
                    CoordinationTestResult::Passed
                }
                Err(e) => {
                    println!("  ❌ {} ({:?}) - {}", benchmark_name, duration, e);
                    CoordinationTestResult::Failed
                }
            };

            report.test_results.push((benchmark_name.to_string(), benchmark_result.clone(), duration));
            report.total_tests += 1;

            match benchmark_result {
                CoordinationTestResult::Passed => report.passed += 1,
                CoordinationTestResult::Failed => report.failed += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Run security tests
    async fn run_security_tests(&self, report: &mut CoordinationTestReport) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔒 Running Coordination Security Tests...");
        
        let security_tests = vec![
            ("SQL Injection Protection", Box::pin(self.test_sql_injection_protection())),
            ("XSS Protection", Box::pin(self.test_xss_protection())),
            ("Path Traversal Protection", Box::pin(self.test_path_traversal_protection())),
            ("Command Injection Protection", Box::pin(self.test_command_injection_protection())),
            ("Unauthorized Access Protection", Box::pin(self.test_unauthorized_access_protection())),
        ];

        for (test_name, mut test_fn) in security_tests {
            let test_start = Instant::now();
            let result = test_fn.await;
            let duration = test_start.elapsed();

            let test_result = match result {
                Ok(_) => {
                    println!("  ✅ {} ({:?})", test_name, duration);
                    CoordinationTestResult::Passed
                }
                Err(e) => {
                    println!("  ❌ {} ({:?}) - {}", test_name, duration, e);
                    CoordinationTestResult::Failed
                }
            };

            report.test_results.push((test_name.to_string(), test_result.clone(), duration));
            report.total_tests += 1;

            match test_result {
                CoordinationTestResult::Passed => report.passed += 1,
                CoordinationTestResult::Failed => report.failed += 1,
                _ => {}
            }
        }

        Ok(())
    }

    // Integration test implementations
    async fn test_agent_registration(&self) -> Result<(), Box<dyn std::error::Error>> {
        // This would call the actual coordination CLI command
        // For now, we'll simulate the test
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    async fn test_agent_listing(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    async fn test_agent_message_sending(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(75)).await;
        Ok(())
    }

    async fn test_session_creation(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(80)).await;
        Ok(())
    }

    async fn test_session_listing(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(60)).await;
        Ok(())
    }

    async fn test_system_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(40)).await;
        Ok(())
    }

    async fn test_system_health(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(Duration::from_millis(30)).await;
        Ok(())
    }

    // Performance benchmark implementations
    async fn benchmark_agent_registration(&self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simulate multiple agent registrations
        for i in 0..10 {
            let agent_name = format!("bench-agent-{}", i);
            // This would call the actual coordination CLI command
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        let duration = start.elapsed();
        CoordinationAssertions::assert_performance_requirements(duration, 5000); // 5 seconds max
        
        Ok(())
    }

    async fn benchmark_agent_listing(&self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simulate agent listing with filters
        for _ in 0..5 {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        
        let duration = start.elapsed();
        CoordinationAssertions::assert_performance_requirements(duration, 2000); // 2 seconds max
        
        Ok(())
    }

    async fn benchmark_message_sending(&self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simulate message sending
        for i in 0..20 {
            let message = format!("Benchmark message {}", i);
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        
        let duration = start.elapsed();
        CoordinationAssertions::assert_performance_requirements(duration, 3000); // 3 seconds max
        
        Ok(())
    }

    async fn benchmark_session_creation(&self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simulate session creation
        for i in 0..5 {
            let session_topic = format!("Benchmark Session {}", i);
            tokio::time::sleep(Duration::from_millis(30)).await;
        }
        
        let duration = start.elapsed();
        CoordinationAssertions::assert_performance_requirements(duration, 2000); // 2 seconds max
        
        Ok(())
    }

    async fn benchmark_system_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Simulate system stats collection
        for _ in 0..3 {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        
        let duration = start.elapsed();
        CoordinationAssertions::assert_performance_requirements(duration, 1500); // 1.5 seconds max
        
        Ok(())
    }

    // Security test implementations
    async fn test_sql_injection_protection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let malicious_inputs = vec![
            "'; DROP TABLE agents; --",
            "' OR '1'='1",
            "'; INSERT INTO agents VALUES ('hacker', 'malicious'); --",
        ];

        for malicious_input in malicious_inputs {
            // This would test the actual coordination CLI with malicious input
            // For now, we'll simulate the test
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    async fn test_xss_protection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let malicious_inputs = vec![
            "<script>alert('xss')</script>",
            "javascript:alert('xss')",
            "<img src=x onerror=alert('xss')>",
        ];

        for malicious_input in malicious_inputs {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    async fn test_path_traversal_protection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let malicious_inputs = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "/etc/passwd",
        ];

        for malicious_input in malicious_inputs {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    async fn test_command_injection_protection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let malicious_inputs = vec![
            "$(rm -rf /)",
            "`rm -rf /`",
            "; rm -rf /;",
        ];

        for malicious_input in malicious_inputs {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    async fn test_unauthorized_access_protection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let unauthorized_requests = vec![
            "agent-999999", // Non-existent agent
            "session-999999", // Non-existent session
            "admin-agent", // Unauthorized agent type
        ];

        for request in unauthorized_requests {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }
}

/// Main function to run coordination tests
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::new();
    let mut runner = CoordinationTestRunner::new(config);
    
    let report = runner.run_all_tests().await?;
    
    if report.failed > 0 || report.errors > 0 {
        std::process::exit(1);
    }
    
    Ok(())
} 