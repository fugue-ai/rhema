//! Enhanced performance testing and benchmarking utilities
//! Provides comprehensive performance testing capabilities for Rhema CLI

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rhema::{Rhema, RhemaResult};

use crate::common::{TestEnv, TestFixtures, enhanced_fixtures::EnhancedFixtures};

/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub warmup_iterations: usize,
    pub measurement_iterations: usize,
    pub confidence_level: f64,
    pub max_duration: Duration,
    pub memory_tracking: bool,
    pub cpu_tracking: bool,
    pub io_tracking: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            measurement_iterations: 100,
            confidence_level: 0.95,
            max_duration: Duration::from_secs(60),
            memory_tracking: true,
            cpu_tracking: true,
            io_tracking: true,
        }
    }
}

/// Performance test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResult {
    pub test_name: String,
    pub operation: String,
    pub dataset_size: usize,
    pub mean_duration: Duration,
    pub median_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub standard_deviation: Duration,
    pub memory_usage: Option<MemoryUsage>,
    pub cpu_usage: Option<CpuUsage>,
    pub io_stats: Option<IoStats>,
    pub throughput: f64,
    pub latency_p95: Duration,
    pub latency_p99: Duration,
    pub success_rate: f64,
    pub error_count: usize,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub peak_memory: usize,
    pub average_memory: usize,
    pub memory_leak_detected: bool,
    pub memory_growth_rate: f64,
}

/// CPU usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuUsage {
    pub peak_cpu: f64,
    pub average_cpu: f64,
    pub cpu_utilization: f64,
    pub context_switches: u64,
}

/// I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStats {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub io_wait_time: Duration,
}

/// Performance test suite
pub struct PerformanceTestSuite {
    config: PerformanceConfig,
    results: Arc<Mutex<Vec<PerformanceResult>>>,
    fixtures: EnhancedFixtures,
}

impl PerformanceTestSuite {
    /// Create a new performance test suite
    pub fn new(config: PerformanceConfig) -> RhemaResult<Self> {
        let fixtures = EnhancedFixtures::with_scenario("performance_tests")?;
        
        Ok(Self {
            config,
            results: Arc::new(Mutex::new(Vec::new())),
            fixtures,
        })
    }

    /// Run all performance tests
    pub fn run_all_tests(&mut self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("🚀 Starting Performance Test Suite");
        
        let mut all_results = Vec::new();
        
        // Run query performance tests
        all_results.extend(self.run_query_performance_tests()?);
        
        // Run scope discovery performance tests
        all_results.extend(self.run_scope_discovery_performance_tests()?);
        
        // Run search performance tests
        all_results.extend(self.run_search_performance_tests()?);
        
        // Run file operation performance tests
        all_results.extend(self.run_file_operation_performance_tests()?);
        
        // Run memory usage tests
        all_results.extend(self.run_memory_usage_tests()?);
        
        // Run concurrent operation tests
        all_results.extend(self.run_concurrent_operation_tests()?);
        
        // Run large dataset tests
        all_results.extend(self.run_large_dataset_tests()?);
        
        // Run stress tests
        all_results.extend(self.run_stress_tests()?);
        
        self.print_summary(&all_results);
        
        Ok(all_results)
    }

    /// Run query performance tests
    fn run_query_performance_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("📊 Running Query Performance Tests");
        
        let mut results = Vec::new();
        
        // Test simple queries
        results.push(self.benchmark_query("simple_query", "todos", 100)?);
        results.push(self.benchmark_query("filtered_query", "todos WHERE status=pending", 100)?);
        results.push(self.benchmark_query("complex_query", "todos WHERE status=pending AND priority=high", 100)?);
        
        // Test queries with different dataset sizes
        for size in [100, 1000, 10000] {
            results.push(self.benchmark_query_with_size(&format!("query_size_{}", size), "todos", size)?);
        }
        
        Ok(results)
    }

    /// Run scope discovery performance tests
    fn run_scope_discovery_performance_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("🔍 Running Scope Discovery Performance Tests");
        
        let mut results = Vec::new();
        
        // Test scope discovery
        results.push(self.benchmark_operation("scope_discovery", || {
            self.fixtures.rhema.discover_scopes()
        })?);
        
        // Test get scope
        results.push(self.benchmark_operation("get_scope", || {
            self.fixtures.rhema.get_scope(".rhema")
        })?);
        
        Ok(results)
    }

    /// Run search performance tests
    fn run_search_performance_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("🔎 Running Search Performance Tests");
        
        let mut results = Vec::new();
        
        // Test regex search
        results.push(self.benchmark_operation("regex_search", || {
            self.fixtures.rhema.search_regex("todo", None)
        })?);
        
        // Test regex search with file filter
        results.push(self.benchmark_operation("regex_search_with_filter", || {
            self.fixtures.rhema.search_regex("todo", Some("*.yaml"))
        })?);
        
        Ok(results)
    }

    /// Run file operation performance tests
    fn run_file_operation_performance_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("📁 Running File Operation Performance Tests");
        
        let mut results = Vec::new();
        
        // Test file reading
        results.push(self.benchmark_file_operation("file_read", |path| {
            std::fs::read_to_string(path)
        })?);
        
        // Test file writing
        results.push(self.benchmark_file_operation("file_write", |path| {
            std::fs::write(path, "test content")
        })?);
        
        // Test YAML parsing
        results.push(self.benchmark_yaml_operation("yaml_parse", |content| {
            serde_yaml::from_str::<serde_yaml::Value>(content)
        })?);
        
        Ok(results)
    }

    /// Run memory usage tests
    fn run_memory_usage_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("🧠 Running Memory Usage Tests");
        
        let mut results = Vec::new();
        
        // Test memory usage during query execution
        results.push(self.benchmark_memory_usage("query_memory_usage", || {
            self.fixtures.rhema.query("todos")
        })?);
        
        // Test memory usage during large dataset processing
        results.push(self.benchmark_memory_usage("large_dataset_memory", || {
            self.fixtures.rhema.query("large_todos")
        })?);
        
        Ok(results)
    }

    /// Run concurrent operation tests
    fn run_concurrent_operation_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("🔄 Running Concurrent Operation Tests");
        
        let mut results = Vec::new();
        
        // Test concurrent queries
        results.push(self.benchmark_concurrent_operations("concurrent_queries", 10, || {
            self.fixtures.rhema.query("todos")
        })?);
        
        // Test concurrent searches
        results.push(self.benchmark_concurrent_operations("concurrent_searches", 10, || {
            self.fixtures.rhema.search_regex("todo", None)
        })?);
        
        Ok(results)
    }

    /// Run large dataset tests
    fn run_large_dataset_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("📈 Running Large Dataset Tests");
        
        let mut results = Vec::new();
        
        // Test different dataset sizes
        for size in [1000, 10000, 100000] {
            results.push(self.benchmark_large_dataset(&format!("dataset_size_{}", size), size)?);
        }
        
        Ok(results)
    }

    /// Run stress tests
    fn run_stress_tests(&self) -> RhemaResult<Vec<PerformanceResult>> {
        println!("💪 Running Stress Tests");
        
        let mut results = Vec::new();
        
        // Test continuous operation for extended period
        results.push(self.benchmark_stress_test("continuous_queries", Duration::from_secs(30), || {
            self.fixtures.rhema.query("todos")
        })?);
        
        // Test memory pressure
        results.push(self.benchmark_memory_pressure_test("memory_pressure", 1000)?);
        
        Ok(results)
    }

    /// Benchmark a query operation
    fn benchmark_query(&self, test_name: &str, query: &str, iterations: usize) -> RhemaResult<PerformanceResult> {
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = self.fixtures.rhema.query(query);
        }
        
        // Measurement
        for _ in 0..iterations {
            let start = Instant::now();
            match self.fixtures.rhema.query(query) {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "query", 0, durations, errors, iterations)
    }

    /// Benchmark a query operation with specific dataset size
    fn benchmark_query_with_size(&self, test_name: &str, query: &str, dataset_size: usize) -> RhemaResult<PerformanceResult> {
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = self.fixtures.rhema.query(query);
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            match self.fixtures.rhema.query(query) {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "query", dataset_size, durations, errors, self.config.measurement_iterations)
    }

    /// Benchmark a generic operation
    fn benchmark_operation<F, T>(&self, test_name: &str, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn() -> RhemaResult<T>,
    {
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = operation();
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            match operation() {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "operation", 0, durations, errors, self.config.measurement_iterations)
    }

    /// Benchmark file operations
    fn benchmark_file_operation<F>(&self, test_name: &str, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn(&std::path::Path) -> std::io::Result<()>,
    {
        let test_file = self.fixtures.repo_path.join("test_file.txt");
        std::fs::write(&test_file, "test content")?;
        
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = operation(&test_file);
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            match operation(&test_file) {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        // Cleanup
        let _ = std::fs::remove_file(&test_file);
        
        self.calculate_performance_result(test_name, "file_operation", 0, durations, errors, self.config.measurement_iterations)
    }

    /// Benchmark YAML operations
    fn benchmark_yaml_operation<F, T>(&self, test_name: &str, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn(&str) -> Result<T, serde_yaml::Error>,
    {
        let test_yaml = r#"
items:
  - id: "test-001"
    name: "Test Item"
    status: "active"
"#;
        
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = operation(test_yaml);
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            match operation(test_yaml) {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "yaml_operation", 0, durations, errors, self.config.measurement_iterations)
    }

    /// Benchmark memory usage
    fn benchmark_memory_usage<F, T>(&self, test_name: &str, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn() -> RhemaResult<T>,
    {
        let mut durations = Vec::new();
        let mut errors = 0;
        let mut memory_usage = Vec::new();
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = operation();
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            let memory_before = self.get_memory_usage()?;
            
            match operation() {
                Ok(_) => {
                    let duration = start.elapsed();
                    let memory_after = self.get_memory_usage()?;
                    
                    durations.push(duration);
                    memory_usage.push(memory_after - memory_before);
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        let mut result = self.calculate_performance_result(test_name, "memory_usage", 0, durations, errors, self.config.measurement_iterations)?;
        
        if self.config.memory_tracking {
            result.memory_usage = Some(MemoryUsage {
                peak_memory: memory_usage.iter().max().copied().unwrap_or(0),
                average_memory: memory_usage.iter().sum::<usize>() / memory_usage.len(),
                memory_leak_detected: self.detect_memory_leak(&memory_usage),
                memory_growth_rate: self.calculate_memory_growth_rate(&memory_usage),
            });
        }
        
        Ok(result)
    }

    /// Benchmark concurrent operations
    fn benchmark_concurrent_operations<F, T>(&self, test_name: &str, concurrency: usize, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn() -> RhemaResult<T> + Send + Sync,
    {
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = operation();
        }
        
        // Measurement with concurrent execution
        let start = Instant::now();
        let handles: Vec<_> = (0..concurrency)
            .map(|_| {
                let operation = &operation;
                thread::spawn(move || {
                    let op_start = Instant::now();
                    let result = operation();
                    (op_start.elapsed(), result.is_ok())
                })
            })
            .collect();
        
        for handle in handles {
            match handle.join() {
                Ok((duration, success)) => {
                    durations.push(duration);
                    if !success {
                        errors += 1;
                    }
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "concurrent", concurrency, durations, errors, concurrency)
    }

    /// Benchmark large dataset operations
    fn benchmark_large_dataset(&self, test_name: &str, dataset_size: usize) -> RhemaResult<PerformanceResult> {
        let mut durations = Vec::new();
        let mut errors = 0;
        
        // Generate large dataset
        let large_dataset = self.generate_large_dataset(dataset_size)?;
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let _ = self.process_large_dataset(&large_dataset);
        }
        
        // Measurement
        for _ in 0..self.config.measurement_iterations {
            let start = Instant::now();
            match self.process_large_dataset(&large_dataset) {
                Ok(_) => {
                    durations.push(start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "large_dataset", dataset_size, durations, errors, self.config.measurement_iterations)
    }

    /// Benchmark stress test
    fn benchmark_stress_test<F, T>(&self, test_name: &str, duration: Duration, operation: F) -> RhemaResult<PerformanceResult>
    where
        F: Fn() -> RhemaResult<T>,
    {
        let mut durations = Vec::new();
        let mut errors = 0;
        let start_time = Instant::now();
        
        // Run operations continuously for the specified duration
        while start_time.elapsed() < duration {
            let op_start = Instant::now();
            match operation() {
                Ok(_) => {
                    durations.push(op_start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        self.calculate_performance_result(test_name, "stress_test", 0, durations, errors, durations.len())
    }

    /// Benchmark memory pressure test
    fn benchmark_memory_pressure_test(&self, test_name: &str, iterations: usize) -> RhemaResult<PerformanceResult> {
        let mut durations = Vec::new();
        let mut errors = 0;
        let mut memory_usage = Vec::new();
        
        // Create memory pressure by allocating large amounts of data
        for i in 0..iterations {
            let start = Instant::now();
            let memory_before = self.get_memory_usage()?;
            
            // Allocate memory
            let _large_data = vec![0u8; 1024 * 1024]; // 1MB
            
            match self.fixtures.rhema.query("todos") {
                Ok(_) => {
                    let duration = start.elapsed();
                    let memory_after = self.get_memory_usage()?;
                    
                    durations.push(duration);
                    memory_usage.push(memory_after - memory_before);
                }
                Err(_) => {
                    errors += 1;
                }
            }
            
            // Force garbage collection if possible
            if i % 100 == 0 {
                // In Rust, we can't force GC, but we can drop references
                drop(_large_data);
            }
        }
        
        let mut result = self.calculate_performance_result(test_name, "memory_pressure", 0, durations, errors, iterations)?;
        
        if self.config.memory_tracking {
            result.memory_usage = Some(MemoryUsage {
                peak_memory: memory_usage.iter().max().copied().unwrap_or(0),
                average_memory: memory_usage.iter().sum::<usize>() / memory_usage.len(),
                memory_leak_detected: self.detect_memory_leak(&memory_usage),
                memory_growth_rate: self.calculate_memory_growth_rate(&memory_usage),
            });
        }
        
        Ok(result)
    }

    /// Calculate performance result from collected data
    fn calculate_performance_result(
        &self,
        test_name: &str,
        operation: &str,
        dataset_size: usize,
        durations: Vec<Duration>,
        errors: usize,
        total_iterations: usize,
    ) -> RhemaResult<PerformanceResult> {
        if durations.is_empty() {
            return Err(rhema::RhemaError::TestError("No valid measurements collected".to_string()));
        }
        
        let mut sorted_durations = durations.clone();
        sorted_durations.sort();
        
        let mean_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
        let median_duration = sorted_durations[sorted_durations.len() / 2];
        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();
        
        // Calculate standard deviation
        let variance = durations.iter()
            .map(|d| {
                let diff = if *d > mean_duration {
                    *d - mean_duration
                } else {
                    mean_duration - *d
                };
                diff.as_nanos() as f64
            })
            .map(|d| d * d)
            .sum::<f64>() / durations.len() as f64;
        let standard_deviation = Duration::from_nanos(variance.sqrt() as u64);
        
        // Calculate percentiles
        let p95_index = (durations.len() as f64 * 0.95) as usize;
        let p99_index = (durations.len() as f64 * 0.99) as usize;
        let latency_p95 = sorted_durations[p95_index.min(durations.len() - 1)];
        let latency_p99 = sorted_durations[p99_index.min(durations.len() - 1)];
        
        // Calculate throughput (operations per second)
        let total_duration = durations.iter().sum::<Duration>();
        let throughput = if total_duration.as_secs() > 0 {
            durations.len() as f64 / total_duration.as_secs() as f64
        } else {
            durations.len() as f64 / total_duration.as_millis() as f64 * 1000.0
        };
        
        let success_rate = (total_iterations - errors) as f64 / total_iterations as f64;
        
        Ok(PerformanceResult {
            test_name: test_name.to_string(),
            operation: operation.to_string(),
            dataset_size,
            mean_duration,
            median_duration,
            min_duration,
            max_duration,
            standard_deviation,
            memory_usage: None,
            cpu_usage: None,
            io_stats: None,
            throughput,
            latency_p95,
            latency_p99,
            success_rate,
            error_count: errors,
        })
    }

    /// Get current memory usage
    fn get_memory_usage(&self) -> RhemaResult<usize> {
        // This is a simplified implementation
        // In a real implementation, you would use platform-specific APIs
        Ok(0) // Placeholder
    }

    /// Detect memory leak
    fn detect_memory_leak(&self, memory_usage: &[usize]) -> bool {
        if memory_usage.len() < 10 {
            return false;
        }
        
        // Simple linear regression to detect increasing trend
        let n = memory_usage.len() as f64;
        let sum_x = (0..memory_usage.len()).map(|i| i as f64).sum::<f64>();
        let sum_y = memory_usage.iter().map(|&m| m as f64).sum::<f64>();
        let sum_xy = (0..memory_usage.len())
            .zip(memory_usage.iter())
            .map(|(i, &m)| i as f64 * m as f64)
            .sum::<f64>();
        let sum_x2 = (0..memory_usage.len()).map(|i| (i as f64).powi(2)).sum::<f64>();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        
        // Consider it a leak if slope is positive and significant
        slope > 100.0
    }

    /// Calculate memory growth rate
    fn calculate_memory_growth_rate(&self, memory_usage: &[usize]) -> f64 {
        if memory_usage.len() < 2 {
            return 0.0;
        }
        
        let first = memory_usage[0] as f64;
        let last = memory_usage[memory_usage.len() - 1] as f64;
        
        if first == 0.0 {
            return 0.0;
        }
        
        (last - first) / first
    }

    /// Generate large dataset
    fn generate_large_dataset(&self, size: usize) -> RhemaResult<Vec<serde_yaml::Value>> {
        let mut dataset = Vec::new();
        
        for i in 0..size {
            let item = serde_yaml::from_str(&format!(
                r#"
id: "item-{:06}"
name: "Large Dataset Item {}"
description: "This is a large dataset item for performance testing"
status: "active"
priority: "medium"
created_at: "2024-01-15T10:00:00Z"
tags: ["performance", "test", "large"]
"#,
                i, i
            ))?;
            dataset.push(item);
        }
        
        Ok(dataset)
    }

    /// Process large dataset
    fn process_large_dataset(&self, dataset: &[serde_yaml::Value]) -> RhemaResult<()> {
        // Simulate processing the dataset
        for item in dataset {
            let _ = serde_yaml::to_string(item)?;
        }
        Ok(())
    }

    /// Print performance test summary
    fn print_summary(&self, results: &[PerformanceResult]) {
        println!("\n📊 Performance Test Summary");
        println!("==========================");
        
        for result in results {
            println!(
                "{}: {:.2?} (mean), {:.2?} (p95), {:.2?} (p99), {:.2} ops/sec, {:.1}% success",
                result.test_name,
                result.mean_duration,
                result.latency_p95,
                result.latency_p99,
                result.throughput,
                result.success_rate * 100.0
            );
            
            if let Some(ref memory) = result.memory_usage {
                println!(
                    "  Memory: {}MB peak, {}MB avg, leak: {}",
                    memory.peak_memory / 1024 / 1024,
                    memory.average_memory / 1024 / 1024,
                    memory.memory_leak_detected
                );
            }
        }
    }
}

/// Criterion benchmark functions
pub fn criterion_benchmark(c: &mut Criterion) {
    let config = PerformanceConfig::default();
    let mut suite = PerformanceTestSuite::new(config).unwrap();
    
    // Add benchmark groups
    let mut query_group = c.benchmark_group("query_performance");
    let mut search_group = c.benchmark_group("search_performance");
    let mut file_group = c.benchmark_group("file_performance");
    
    // Query benchmarks
    query_group.bench_function("simple_query", |b| {
        b.iter(|| {
            suite.fixtures.rhema.query("todos").unwrap();
        });
    });
    
    query_group.bench_function("filtered_query", |b| {
        b.iter(|| {
            suite.fixtures.rhema.query("todos WHERE status=pending").unwrap();
        });
    });
    
    query_group.bench_function("complex_query", |b| {
        b.iter(|| {
            suite.fixtures.rhema.query("todos WHERE status=pending AND priority=high").unwrap();
        });
    });
    
    // Search benchmarks
    search_group.bench_function("regex_search", |b| {
        b.iter(|| {
            suite.fixtures.rhema.search_regex("todo", None).unwrap();
        });
    });
    
    search_group.bench_function("regex_search_with_filter", |b| {
        b.iter(|| {
            suite.fixtures.rhema.search_regex("todo", Some("*.yaml")).unwrap();
        });
    });
    
    // File operation benchmarks
    file_group.bench_function("yaml_parse", |b| {
        let test_yaml = r#"
items:
  - id: "test-001"
    name: "Test Item"
    status: "active"
"#;
        b.iter(|| {
            serde_yaml::from_str::<serde_yaml::Value>(test_yaml).unwrap();
        });
    });
    
    query_group.finish();
    search_group.finish();
    file_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 