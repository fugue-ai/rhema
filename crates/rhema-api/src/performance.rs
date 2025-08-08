/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, instrument};

/// Performance metrics for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operation name
    pub operation_name: String,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Memory usage in bytes
    pub memory_usage_bytes: Option<usize>,

    /// CPU usage percentage
    pub cpu_usage_percent: Option<f64>,

    /// Number of files processed
    pub files_processed: Option<usize>,

    /// Cache hit rate
    pub cache_hit_rate: Option<f64>,

    /// Error count
    pub error_count: u32,

    /// Success count
    pub success_count: u32,

    /// Additional custom metrics
    pub custom_metrics: HashMap<String, serde_yaml::Value>,

    /// Timestamp when metrics were recorded
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Performance monitor for tracking operation metrics
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<HashMap<String, Vec<PerformanceMetrics>>>>,
    enabled: bool,
    max_metrics_per_operation: usize,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
            max_metrics_per_operation: 1000,
        }
    }

    /// Create a new performance monitor with custom settings
    pub fn new_with_settings(enabled: bool, max_metrics_per_operation: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            enabled,
            max_metrics_per_operation,
        }
    }

    /// Record performance metrics for an operation
    #[instrument(skip_all)]
    pub async fn record_metrics(&self, metrics: PerformanceMetrics) {
        if !self.enabled {
            return;
        }

        let mut metrics_store = self.metrics.write().await;
        let operation_metrics = metrics_store
            .entry(metrics.operation_name.clone())
            .or_insert_with(Vec::new);

        let operation_name = metrics.operation_name.clone();
        operation_metrics.push(metrics);

        // Keep only the most recent metrics
        if operation_metrics.len() > self.max_metrics_per_operation {
            operation_metrics.remove(0);
        }

        info!(
            "Recorded performance metrics for operation: {}",
            operation_name
        );
    }

    /// Get performance metrics for an operation
    #[instrument(skip_all)]
    pub async fn get_metrics(&self, operation_name: &str) -> Vec<PerformanceMetrics> {
        let metrics_store = self.metrics.read().await;
        metrics_store
            .get(operation_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Get aggregated performance metrics for an operation
    #[instrument(skip_all)]
    pub async fn get_aggregated_metrics(&self, operation_name: &str) -> Option<AggregatedMetrics> {
        let metrics = self.get_metrics(operation_name).await;
        if metrics.is_empty() {
            return None;
        }

        let mut aggregated = AggregatedMetrics {
            operation_name: operation_name.to_string(),
            total_executions: metrics.len(),
            avg_execution_time_ms: 0.0,
            min_execution_time_ms: u64::MAX,
            max_execution_time_ms: 0,
            total_errors: 0,
            total_successes: 0,
            avg_memory_usage_bytes: None,
            avg_cpu_usage_percent: None,
            avg_cache_hit_rate: None,
            custom_metrics: HashMap::new(),
        };

        let mut total_execution_time = 0u64;
        let mut total_memory_usage = 0usize;
        let mut memory_count = 0usize;
        let mut total_cpu_usage = 0.0;
        let mut cpu_count = 0usize;
        let mut total_cache_hit_rate = 0.0;
        let mut cache_count = 0usize;

        for metric in &metrics {
            total_execution_time += metric.execution_time_ms;
            aggregated.min_execution_time_ms = aggregated
                .min_execution_time_ms
                .min(metric.execution_time_ms);
            aggregated.max_execution_time_ms = aggregated
                .max_execution_time_ms
                .max(metric.execution_time_ms);
            aggregated.total_errors += metric.error_count;
            aggregated.total_successes += metric.success_count;

            if let Some(memory_usage) = metric.memory_usage_bytes {
                total_memory_usage += memory_usage;
                memory_count += 1;
            }

            if let Some(cpu_usage) = metric.cpu_usage_percent {
                total_cpu_usage += cpu_usage;
                cpu_count += 1;
            }

            if let Some(cache_hit_rate) = metric.cache_hit_rate {
                total_cache_hit_rate += cache_hit_rate;
                cache_count += 1;
            }

            // Merge custom metrics
            for (key, value) in &metric.custom_metrics {
                aggregated.custom_metrics.insert(key.clone(), value.clone());
            }
        }

        aggregated.avg_execution_time_ms = total_execution_time as f64 / metrics.len() as f64;

        if memory_count > 0 {
            aggregated.avg_memory_usage_bytes = Some(total_memory_usage / memory_count);
        }

        if cpu_count > 0 {
            aggregated.avg_cpu_usage_percent = Some(total_cpu_usage / cpu_count as f64);
        }

        if cache_count > 0 {
            aggregated.avg_cache_hit_rate = Some(total_cache_hit_rate / cache_count as f64);
        }

        Some(aggregated)
    }

    /// Get all aggregated metrics
    #[instrument(skip_all)]
    pub async fn get_all_aggregated_metrics(&self) -> HashMap<String, AggregatedMetrics> {
        let metrics_store = self.metrics.read().await;
        let mut result = HashMap::new();

        for operation_name in metrics_store.keys() {
            if let Some(aggregated) = self.get_aggregated_metrics(operation_name).await {
                result.insert(operation_name.clone(), aggregated);
            }
        }

        result
    }

    /// Clear all metrics
    #[instrument(skip_all)]
    pub async fn clear_metrics(&self) {
        let mut metrics_store = self.metrics.write().await;
        metrics_store.clear();
        info!("Cleared all performance metrics");
    }

    /// Check if performance is within acceptable limits
    #[instrument(skip_all)]
    pub async fn check_performance_limits(
        &self,
        operation_name: &str,
        limits: &PerformanceLimits,
    ) -> PerformanceCheckResult {
        if let Some(aggregated) = self.get_aggregated_metrics(operation_name).await {
            let mut violations = Vec::new();

            if aggregated.avg_execution_time_ms > limits.max_avg_execution_time_ms as f64 {
                violations.push(format!(
                    "Average execution time {}ms exceeds limit {}ms",
                    aggregated.avg_execution_time_ms, limits.max_avg_execution_time_ms
                ));
            }

            if aggregated.max_execution_time_ms > limits.max_execution_time_ms {
                violations.push(format!(
                    "Maximum execution time {}ms exceeds limit {}ms",
                    aggregated.max_execution_time_ms, limits.max_execution_time_ms
                ));
            }

            if let Some(avg_memory) = aggregated.avg_memory_usage_bytes {
                if avg_memory > limits.max_memory_usage_bytes {
                    violations.push(format!(
                        "Average memory usage {} bytes exceeds limit {} bytes",
                        avg_memory, limits.max_memory_usage_bytes
                    ));
                }
            }

            if let Some(avg_cpu) = aggregated.avg_cpu_usage_percent {
                if avg_cpu > limits.max_cpu_usage_percent {
                    violations.push(format!(
                        "Average CPU usage {}% exceeds limit {}%",
                        avg_cpu, limits.max_cpu_usage_percent
                    ));
                }
            }

            let error_rate = if aggregated.total_executions > 0 {
                aggregated.total_errors as f64 / aggregated.total_executions as f64
            } else {
                0.0
            };

            if error_rate > limits.max_error_rate {
                violations.push(format!(
                    "Error rate {} exceeds limit {}",
                    error_rate, limits.max_error_rate
                ));
            }

            PerformanceCheckResult {
                operation_name: operation_name.to_string(),
                passed: violations.is_empty(),
                violations,
                metrics: aggregated,
            }
        } else {
            PerformanceCheckResult {
                operation_name: operation_name.to_string(),
                passed: true,
                violations: vec!["No metrics available".to_string()],
                metrics: AggregatedMetrics::default(),
            }
        }
    }
}

/// Aggregated performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Operation name
    pub operation_name: String,

    /// Total number of executions
    pub total_executions: usize,

    /// Average execution time in milliseconds
    pub avg_execution_time_ms: f64,

    /// Minimum execution time in milliseconds
    pub min_execution_time_ms: u64,

    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,

    /// Total number of errors
    pub total_errors: u32,

    /// Total number of successes
    pub total_successes: u32,

    /// Average memory usage in bytes
    pub avg_memory_usage_bytes: Option<usize>,

    /// Average CPU usage percentage
    pub avg_cpu_usage_percent: Option<f64>,

    /// Average cache hit rate
    pub avg_cache_hit_rate: Option<f64>,

    /// Custom metrics
    pub custom_metrics: HashMap<String, serde_yaml::Value>,
}

impl Default for AggregatedMetrics {
    fn default() -> Self {
        Self {
            operation_name: String::new(),
            total_executions: 0,
            avg_execution_time_ms: 0.0,
            min_execution_time_ms: 0,
            max_execution_time_ms: 0,
            total_errors: 0,
            total_successes: 0,
            avg_memory_usage_bytes: None,
            avg_cpu_usage_percent: None,
            avg_cache_hit_rate: None,
            custom_metrics: HashMap::new(),
        }
    }
}

/// Performance limits for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceLimits {
    /// Maximum average execution time in milliseconds
    pub max_avg_execution_time_ms: u64,

    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,

    /// Maximum memory usage in bytes
    pub max_memory_usage_bytes: usize,

    /// Maximum CPU usage percentage
    pub max_cpu_usage_percent: f64,

    /// Maximum error rate (0.0 to 1.0)
    pub max_error_rate: f64,
}

impl Default for PerformanceLimits {
    fn default() -> Self {
        Self {
            max_avg_execution_time_ms: 1000,           // 1 second
            max_execution_time_ms: 5000,               // 5 seconds
            max_memory_usage_bytes: 100 * 1024 * 1024, // 100 MB
            max_cpu_usage_percent: 80.0,
            max_error_rate: 0.1, // 10%
        }
    }
}

/// Performance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCheckResult {
    /// Operation name
    pub operation_name: String,

    /// Whether performance is within limits
    pub passed: bool,

    /// List of violations
    pub violations: Vec<String>,

    /// Aggregated metrics
    pub metrics: AggregatedMetrics,
}

/// Performance measurement guard for automatic timing
pub struct PerformanceGuard {
    operation_name: String,
    start_time: Instant,
    monitor: Arc<PerformanceMonitor>,
    success: bool,
}

impl PerformanceGuard {
    /// Create a new performance guard
    pub fn new(operation_name: String, monitor: Arc<PerformanceMonitor>) -> Self {
        Self {
            operation_name,
            start_time: Instant::now(),
            monitor,
            success: true,
        }
    }

    /// Mark the operation as failed
    pub fn mark_failed(&mut self) {
        self.success = false;
    }

    /// Get the elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Drop for PerformanceGuard {
    fn drop(&mut self) {
        let execution_time = self.start_time.elapsed();
        let metrics = PerformanceMetrics {
            operation_name: self.operation_name.clone(),
            execution_time_ms: execution_time.as_millis() as u64,
            memory_usage_bytes: None, // Could be enhanced to measure actual memory usage
            cpu_usage_percent: None,  // Could be enhanced to measure actual CPU usage
            files_processed: None,
            cache_hit_rate: None,
            error_count: if self.success { 0 } else { 1 },
            success_count: if self.success { 1 } else { 0 },
            custom_metrics: HashMap::new(),
            timestamp: chrono::Utc::now(),
        };

        // Spawn async task to record metrics
        let monitor = self.monitor.clone();
        tokio::spawn(async move {
            monitor.record_metrics(metrics).await;
        });
    }
}

/// Resource manager for efficient resource usage
#[derive(Debug, Clone)]
pub struct ResourceManager {
    memory_limit: usize,
    cpu_limit: f64,
    connection_pool_size: usize,
    cache_size: usize,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            memory_limit: 100 * 1024 * 1024, // 100 MB
            cpu_limit: 80.0,                 // 80%
            connection_pool_size: 10,
            cache_size: 1000,
        }
    }

    /// Create a new resource manager with custom limits
    pub fn new_with_limits(
        memory_limit: usize,
        cpu_limit: f64,
        connection_pool_size: usize,
        cache_size: usize,
    ) -> Self {
        Self {
            memory_limit,
            cpu_limit,
            connection_pool_size,
            cache_size,
        }
    }

    /// Check if current resource usage is within limits
    pub fn check_resource_usage(&self) -> ResourceUsageStatus {
        // This is a simplified implementation
        // In a real implementation, you would measure actual system resources
        ResourceUsageStatus {
            memory_usage_bytes: 0,  // Placeholder
            cpu_usage_percent: 0.0, // Placeholder
            within_limits: true,
            warnings: Vec::new(),
        }
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> usize {
        // This is a simplified implementation
        // In a real implementation, you would measure actual memory usage
        0
    }

    /// Get CPU usage
    pub fn get_cpu_usage(&self) -> f64 {
        // This is a simplified implementation
        // In a real implementation, you would measure actual CPU usage
        0.0
    }
}

/// Resource usage status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageStatus {
    /// Current memory usage in bytes
    pub memory_usage_bytes: usize,

    /// Current CPU usage percentage
    pub cpu_usage_percent: f64,

    /// Whether usage is within limits
    pub within_limits: bool,

    /// List of warnings
    pub warnings: Vec<String>,
}

/// Performance optimization utilities
pub struct PerformanceOptimizer;

impl PerformanceOptimizer {
    /// Optimize query execution
    pub fn optimize_query(query: &str) -> String {
        // This is a placeholder for query optimization logic
        // In a real implementation, you would analyze and optimize the query
        query.to_string()
    }

    /// Optimize file operations
    pub fn optimize_file_operations(files: &[String]) -> Vec<String> {
        // This is a placeholder for file operation optimization
        // In a real implementation, you would optimize file access patterns
        files.to_vec()
    }

    /// Optimize memory usage
    pub fn optimize_memory_usage(data: &serde_yaml::Value) -> serde_yaml::Value {
        // This is a placeholder for memory optimization
        // In a real implementation, you would optimize data structures
        data.clone()
    }
}
