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

use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;

use crate::query::{CqlQuery, QueryResult};

/// Performance monitor for query operations
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    /// Monitor configuration
    config: MonitorConfig,
    /// Performance metrics storage
    metrics: Arc<Mutex<PerformanceMetrics>>,
    /// Query performance history
    history: Arc<Mutex<Vec<QueryPerformanceRecord>>>,
    /// Performance alerts
    alerts: Arc<Mutex<Vec<PerformanceAlert>>>,
    /// Metrics sender for async processing
    metrics_sender: Option<mpsc::UnboundedSender<MetricEvent>>,
}

/// Monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub collection_interval_secs: u64,
    /// Performance threshold for alerts (milliseconds)
    pub performance_threshold_ms: u64,
    /// Maximum history size
    pub max_history_size: usize,
    /// Enable real-time monitoring
    pub enable_realtime: bool,
    /// Enable performance alerts
    pub enable_alerts: bool,
    /// Alert threshold for slow queries (milliseconds)
    pub slow_query_threshold_ms: u64,
    /// Alert threshold for memory usage (bytes)
    pub memory_threshold_bytes: usize,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total queries executed
    pub total_queries: u64,
    /// Successful queries
    pub successful_queries: u64,
    /// Failed queries
    pub failed_queries: u64,
    /// Average query execution time (milliseconds)
    pub avg_execution_time_ms: f64,
    /// Median query execution time (milliseconds)
    pub median_execution_time_ms: f64,
    /// 95th percentile execution time (milliseconds)
    pub p95_execution_time_ms: f64,
    /// 99th percentile execution time (milliseconds)
    pub p99_execution_time_ms: f64,
    /// Total execution time (milliseconds)
    pub total_execution_time_ms: u64,
    /// Current memory usage (bytes)
    pub current_memory_bytes: usize,
    /// Peak memory usage (bytes)
    pub peak_memory_bytes: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Cache misses
    pub cache_misses: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Query performance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPerformanceRecord {
    /// Query ID
    pub query_id: String,
    /// Query hash
    pub query_hash: String,
    /// Query string
    pub query_string: String,
    /// Execution start time
    pub start_time: DateTime<Utc>,
    /// Execution end time
    pub end_time: DateTime<Utc>,
    /// Execution duration (milliseconds)
    pub duration_ms: u64,
    /// Result count
    pub result_count: usize,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Memory usage (bytes)
    pub memory_usage_bytes: usize,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Cache hit/miss
    pub cache_hit: bool,
    /// Optimization applied
    pub optimization_applied: Option<String>,
    /// Performance tags
    pub tags: HashMap<String, String>,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Related query ID
    pub query_id: Option<String>,
    /// Alert metadata
    pub metadata: HashMap<String, Value>,
    /// Alert acknowledged
    pub acknowledged: bool,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    SlowQuery,
    HighMemoryUsage,
    HighCPUUsage,
    CacheMissRate,
    ErrorRate,
    PerformanceDegradation,
    Custom(String),
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Metric event for async processing
#[derive(Debug, Clone)]
pub enum MetricEvent {
    QueryStart(String, DateTime<Utc>),
    QueryEnd(String, DateTime<Utc>, bool, usize, u64),
    MemoryUsage(usize),
    CacheHit,
    CacheMiss,
    Error(String),
}

/// Performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Report period
    pub period: ReportPeriod,
    /// Summary metrics
    pub summary: PerformanceMetrics,
    /// Top slow queries
    pub slow_queries: Vec<QueryPerformanceRecord>,
    /// Top memory consumers
    pub memory_consumers: Vec<QueryPerformanceRecord>,
    /// Error analysis
    pub error_analysis: ErrorAnalysis,
    /// Performance trends
    pub trends: PerformanceTrends,
    /// Recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
}

/// Report period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPeriod {
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: DateTime<Utc>,
    /// Duration in seconds
    pub duration_secs: u64,
}

/// Error analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalysis {
    /// Total errors
    pub total_errors: u64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Most common errors
    pub common_errors: Vec<ErrorSummary>,
    /// Error trends
    pub error_trends: Vec<ErrorTrend>,
}

/// Error summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSummary {
    /// Error message
    pub error_message: String,
    /// Error count
    pub count: u64,
    /// Error percentage
    pub percentage: f64,
}

/// Error trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTrend {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Error count
    pub error_count: u64,
    /// Error rate
    pub error_rate: f64,
}

/// Performance trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// Execution time trend
    pub execution_time_trend: Vec<TimePoint>,
    /// Memory usage trend
    pub memory_usage_trend: Vec<TimePoint>,
    /// Query volume trend
    pub query_volume_trend: Vec<TimePoint>,
    /// Cache hit rate trend
    pub cache_hit_rate_trend: Vec<TimePoint>,
}

/// Time point for trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Value
    pub value: f64,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Recommendation description
    pub description: String,
    /// Expected impact
    pub expected_impact: f64,
    /// Implementation difficulty (1-5)
    pub difficulty: u8,
    /// Priority (1-5)
    pub priority: u8,
    /// Related metrics
    pub related_metrics: Vec<String>,
}

/// Recommendation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    OptimizeQuery,
    AddCaching,
    IncreaseMemory,
    ReduceConcurrency,
    AddIndexing,
    UpdateConfiguration,
    MonitorClosely,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_secs: 60,
            performance_threshold_ms: 1000,
            max_history_size: 10000,
            enable_realtime: true,
            enable_alerts: true,
            slow_query_threshold_ms: 5000,
            memory_threshold_bytes: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_queries: 0,
            successful_queries: 0,
            failed_queries: 0,
            avg_execution_time_ms: 0.0,
            median_execution_time_ms: 0.0,
            p95_execution_time_ms: 0.0,
            p99_execution_time_ms: 0.0,
            total_execution_time_ms: 0,
            current_memory_bytes: 0,
            peak_memory_bytes: 0,
            cache_hit_rate: 0.0,
            cache_misses: 0,
            cache_hits: 0,
            last_updated: Utc::now(),
        }
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            config: MonitorConfig::default(),
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            history: Arc::new(Mutex::new(Vec::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            metrics_sender: None,
        }
    }

    /// Create monitor with custom configuration
    pub fn with_config(config: MonitorConfig) -> Self {
        Self {
            config,
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            history: Arc::new(Mutex::new(Vec::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            metrics_sender: None,
        }
    }

    /// Start monitoring a query
    pub async fn start_query_monitoring(&self, query: &CqlQuery) -> String {
        let query_id = self.generate_query_id();
        let start_time = Utc::now();

        if let Some(sender) = &self.metrics_sender {
            let _ = sender.send(MetricEvent::QueryStart(query_id.clone(), start_time));
        }

        // Store query start information
        let query_hash = self.hash_query(query);
        let record = QueryPerformanceRecord {
            query_id: query_id.clone(),
            query_hash,
            query_string: format!("{:?}", query),
            start_time,
            end_time: start_time, // Will be updated when query ends
            duration_ms: 0,
            result_count: 0,
            success: false,
            error_message: None,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            cache_hit: false,
            optimization_applied: None,
            tags: HashMap::new(),
        };

        if let Ok(mut history) = self.history.lock() {
            history.push(record);
        }

        query_id
    }

    /// End monitoring a query
    pub async fn end_query_monitoring(
        &self,
        query_id: &str,
        success: bool,
        result_count: usize,
        error_message: Option<String>,
        optimization_applied: Option<String>,
    ) -> RhemaResult<()> {
        let end_time = Utc::now();

        if let Ok(mut history) = self.history.lock() {
            if let Some(record) = history.iter_mut().find(|r| r.query_id == query_id) {
                record.end_time = end_time;
                record.duration_ms = (end_time - record.start_time).num_milliseconds() as u64;
                record.success = success;
                record.result_count = result_count;
                record.error_message = error_message;
                record.optimization_applied = optimization_applied;

                // Update metrics
                self.update_metrics(record).await?;

                // Check for alerts
                self.check_alerts(record).await?;
            }
        }

        if let Some(sender) = &self.metrics_sender {
            let _ = sender.send(MetricEvent::QueryEnd(
                query_id.to_string(),
                end_time,
                success,
                result_count,
                (end_time - Utc::now()).num_milliseconds() as u64,
            ));
        }

        Ok(())
    }

    /// Update performance metrics
    async fn update_metrics(&self, record: &QueryPerformanceRecord) -> RhemaResult<()> {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.total_queries += 1;
            metrics.total_execution_time_ms += record.duration_ms;

            if record.success {
                metrics.successful_queries += 1;
            } else {
                metrics.failed_queries += 1;
            }

            // Update average execution time
            metrics.avg_execution_time_ms = metrics.total_execution_time_ms as f64 / metrics.total_queries as f64;

            // Update memory usage
            metrics.current_memory_bytes = record.memory_usage_bytes;
            if record.memory_usage_bytes > metrics.peak_memory_bytes {
                metrics.peak_memory_bytes = record.memory_usage_bytes;
            }

            // Update cache statistics
            if record.cache_hit {
                metrics.cache_hits += 1;
            } else {
                metrics.cache_misses += 1;
            }

            let total_cache_ops = metrics.cache_hits + metrics.cache_misses;
            if total_cache_ops > 0 {
                metrics.cache_hit_rate = metrics.cache_hits as f64 / total_cache_ops as f64;
            }

            metrics.last_updated = Utc::now();
        }

        Ok(())
    }

    /// Check for performance alerts
    async fn check_alerts(&self, record: &QueryPerformanceRecord) -> RhemaResult<()> {
        if !self.config.enable_alerts {
            return Ok(());
        }

        let mut alerts = Vec::new();

        // Check for slow queries
        if record.duration_ms > self.config.slow_query_threshold_ms {
            alerts.push(PerformanceAlert {
                alert_id: format!("slow_query_{}", record.query_id),
                alert_type: AlertType::SlowQuery,
                severity: AlertSeverity::Warning,
                message: format!("Slow query detected: {}ms", record.duration_ms),
                timestamp: Utc::now(),
                query_id: Some(record.query_id.clone()),
                metadata: HashMap::new(),
                acknowledged: false,
            });
        }

        // Check for high memory usage
        if record.memory_usage_bytes > self.config.memory_threshold_bytes {
            alerts.push(PerformanceAlert {
                alert_id: format!("high_memory_{}", record.query_id),
                alert_type: AlertType::HighMemoryUsage,
                severity: AlertSeverity::Warning,
                message: format!("High memory usage: {} bytes", record.memory_usage_bytes),
                timestamp: Utc::now(),
                query_id: Some(record.query_id.clone()),
                metadata: HashMap::new(),
                acknowledged: false,
            });
        }

        // Check for failed queries
        if !record.success {
            alerts.push(PerformanceAlert {
                alert_id: format!("query_error_{}", record.query_id),
                alert_type: AlertType::ErrorRate,
                severity: AlertSeverity::Error,
                message: format!("Query failed: {}", record.error_message.as_deref().unwrap_or("Unknown error")),
                timestamp: Utc::now(),
                query_id: Some(record.query_id.clone()),
                metadata: HashMap::new(),
                acknowledged: false,
            });
        }

        // Add alerts to storage
        if let Ok(mut alert_storage) = self.alerts.lock() {
            alert_storage.extend(alerts);
        }

        Ok(())
    }

    /// Generate performance report
    pub async fn generate_report(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> RhemaResult<PerformanceReport> {
        let period = ReportPeriod {
            start_time,
            end_time,
            duration_secs: (end_time - start_time).num_seconds() as u64,
        };

        let history = self.history.lock().map_err(|_| RhemaError::Internal("Failed to lock history".to_string()))?;
        let metrics = self.metrics.lock().map_err(|_| RhemaError::Internal("Failed to lock metrics".to_string()))?;

        // Filter records for the period
        let period_records: Vec<QueryPerformanceRecord> = history
            .iter()
            .filter(|record| record.start_time >= start_time && record.start_time <= end_time)
            .cloned()
            .collect();

        // Generate summary metrics
        let summary = self.calculate_period_metrics(&period_records, &metrics);

        // Get slow queries
        let mut slow_queries = period_records.clone();
        slow_queries.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms));
        slow_queries.truncate(10);

        // Get memory consumers
        let mut memory_consumers = period_records.clone();
        memory_consumers.sort_by(|a, b| b.memory_usage_bytes.cmp(&a.memory_usage_bytes));
        memory_consumers.truncate(10);

        // Generate error analysis
        let error_analysis = self.analyze_errors(&period_records);

        // Generate trends
        let trends = self.generate_trends(&period_records, start_time, end_time);

        // Generate recommendations
        let recommendations = self.generate_recommendations(&summary, &error_analysis, &trends);

        Ok(PerformanceReport {
            period,
            summary,
            slow_queries,
            memory_consumers,
            error_analysis,
            trends,
            recommendations,
        })
    }

    /// Calculate metrics for a specific period
    fn calculate_period_metrics(&self, records: &[QueryPerformanceRecord], global_metrics: &PerformanceMetrics) -> PerformanceMetrics {
        let total_queries = records.len() as u64;
        let successful_queries = records.iter().filter(|r| r.success).count() as u64;
        let failed_queries = total_queries - successful_queries;
        let total_execution_time = records.iter().map(|r| r.duration_ms).sum::<u64>();
        let avg_execution_time = if total_queries > 0 {
            total_execution_time as f64 / total_queries as f64
        } else {
            0.0
        };

        // Calculate percentiles
        let mut durations: Vec<u64> = records.iter().map(|r| r.duration_ms).collect();
        durations.sort();
        let median = if !durations.is_empty() {
            durations[durations.len() / 2]
        } else {
            0
        };
        let p95 = if durations.len() > 0 {
            let index = (durations.len() as f64 * 0.95) as usize;
            durations.get(index).copied().unwrap_or(0)
        } else {
            0
        };
        let p99 = if durations.len() > 0 {
            let index = (durations.len() as f64 * 0.99) as usize;
            durations.get(index).copied().unwrap_or(0)
        } else {
            0
        };

        PerformanceMetrics {
            total_queries,
            successful_queries,
            failed_queries,
            avg_execution_time_ms: avg_execution_time,
            median_execution_time_ms: median as f64,
            p95_execution_time_ms: p95 as f64,
            p99_execution_time_ms: p99 as f64,
            total_execution_time_ms: total_execution_time,
            current_memory_bytes: global_metrics.current_memory_bytes,
            peak_memory_bytes: global_metrics.peak_memory_bytes,
            cache_hit_rate: global_metrics.cache_hit_rate,
            cache_misses: global_metrics.cache_misses,
            cache_hits: global_metrics.cache_hits,
            last_updated: Utc::now(),
        }
    }

    /// Analyze errors in the period
    fn analyze_errors(&self, records: &[QueryPerformanceRecord]) -> ErrorAnalysis {
        let total_errors = records.iter().filter(|r| !r.success).count() as u64;
        let total_queries = records.len() as u64;
        let error_rate = if total_queries > 0 {
            total_errors as f64 / total_queries as f64 * 100.0
        } else {
            0.0
        };

        // Group errors by message
        let mut error_counts: HashMap<String, u64> = HashMap::new();
        for record in records.iter().filter(|r| !r.success) {
            let error_msg = record.error_message.as_deref().unwrap_or("Unknown error");
            *error_counts.entry(error_msg.to_string()).or_insert(0) += 1;
        }

        let mut common_errors: Vec<ErrorSummary> = error_counts
            .into_iter()
            .map(|(msg, count)| ErrorSummary {
                error_message: msg,
                count,
                percentage: count as f64 / total_errors as f64 * 100.0,
            })
            .collect();

        common_errors.sort_by(|a, b| b.count.cmp(&a.count));
        common_errors.truncate(10);

        ErrorAnalysis {
            total_errors,
            error_rate_percent: error_rate,
            common_errors,
            error_trends: Vec::new(), // TODO: Implement error trends
        }
    }

    /// Generate performance trends
    fn generate_trends(&self, records: &[QueryPerformanceRecord], start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> PerformanceTrends {
        // TODO: Implement trend generation
        PerformanceTrends {
            execution_time_trend: Vec::new(),
            memory_usage_trend: Vec::new(),
            query_volume_trend: Vec::new(),
            cache_hit_rate_trend: Vec::new(),
        }
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, metrics: &PerformanceMetrics, error_analysis: &ErrorAnalysis, _trends: &PerformanceTrends) -> Vec<PerformanceRecommendation> {
        let mut recommendations = Vec::new();

        // Check for high error rate
        if error_analysis.error_rate_percent > 5.0 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::MonitorClosely,
                description: "High error rate detected. Monitor system closely.".to_string(),
                expected_impact: 0.8,
                difficulty: 1,
                priority: 5,
                related_metrics: vec!["error_rate".to_string()],
            });
        }

        // Check for slow average execution time
        if metrics.avg_execution_time_ms > 1000.0 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::OptimizeQuery,
                description: "Average query execution time is high. Consider query optimization.".to_string(),
                expected_impact: 0.6,
                difficulty: 3,
                priority: 4,
                related_metrics: vec!["avg_execution_time".to_string()],
            });
        }

        // Check for low cache hit rate
        if metrics.cache_hit_rate < 0.5 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::AddCaching,
                description: "Low cache hit rate. Consider adding more caching.".to_string(),
                expected_impact: 0.4,
                difficulty: 2,
                priority: 3,
                related_metrics: vec!["cache_hit_rate".to_string()],
            });
        }

        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> RhemaResult<PerformanceMetrics> {
        self.metrics.lock()
            .map(|guard| guard.clone())
            .map_err(|_| RhemaError::Internal("Failed to lock metrics".to_string()))
    }

    /// Get performance alerts
    pub fn get_alerts(&self) -> RhemaResult<Vec<PerformanceAlert>> {
        self.alerts.lock()
            .map(|guard| guard.clone())
            .map_err(|_| RhemaError::Internal("Failed to lock alerts".to_string()))
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&self, alert_id: &str) -> RhemaResult<()> {
        if let Ok(mut alerts) = self.alerts.lock() {
            if let Some(alert) = alerts.iter_mut().find(|a| a.alert_id == alert_id) {
                alert.acknowledged = true;
            }
        }
        Ok(())
    }

    /// Clear old history
    pub fn clear_old_history(&self) -> RhemaResult<()> {
        if let Ok(mut history) = self.history.lock() {
            if history.len() > self.config.max_history_size {
                let to_remove = history.len() - self.config.max_history_size;
                history.drain(0..to_remove);
            }
        }
        Ok(())
    }

    /// Generate query ID
    fn generate_query_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        Utc::now().timestamp_nanos().hash(&mut hasher);
        format!("query_{:x}", hasher.finish())
    }

    /// Hash query for identification
    fn hash_query(&self, query: &CqlQuery) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.target.hash(&mut hasher);
        query.yaml_path.hash(&mut hasher);
        query.conditions.len().hash(&mut hasher);
        query.order_by.as_ref().map(|ob| ob.len()).hash(&mut hasher);
        query.limit.hash(&mut hasher);
        query.offset.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
} 