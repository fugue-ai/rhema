use crate::RhemaResult;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Analytics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeLoaderAnalytics {
    /// Plugin performance metrics
    pub plugin_performance: HashMap<String, PluginPerformanceMetrics>,

    /// Scope creation success rates
    pub scope_creation_metrics: ScopeCreationMetrics,

    /// Usage statistics
    pub usage_stats: UsageStatistics,

    /// System performance metrics
    pub system_metrics: SystemMetrics,

    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,

    /// Analytics version
    pub version: String,
}

/// Plugin performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPerformanceMetrics {
    /// Plugin name
    pub plugin_name: String,

    /// Total execution count
    pub total_executions: u64,

    /// Successful executions
    pub successful_executions: u64,

    /// Failed executions
    pub failed_executions: u64,

    /// Average execution time in milliseconds
    pub avg_execution_time_ms: f64,

    /// Total execution time in milliseconds
    pub total_execution_time_ms: u64,

    /// Last execution time
    pub last_execution: Option<DateTime<Utc>>,

    /// Boundaries detected
    pub boundaries_detected: u64,

    /// Scopes created
    pub scopes_created: u64,

    /// Confidence scores (for tracking distribution)
    pub confidence_scores: Vec<f64>,

    /// Error messages (for debugging)
    pub recent_errors: Vec<String>,
}

/// Scope creation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeCreationMetrics {
    /// Total scope creation attempts
    pub total_attempts: u64,

    /// Successful scope creations
    pub successful_creations: u64,

    /// Failed scope creations
    pub failed_creations: u64,

    /// Success rate percentage
    pub success_rate: f64,

    /// Average confidence score
    pub avg_confidence: f64,

    /// Scope types created
    pub scope_types: HashMap<String, u64>,

    /// Creation times by scope type
    pub creation_times: HashMap<String, Vec<u64>>,

    /// Last creation time
    pub last_creation: Option<DateTime<Utc>>,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    /// Total commands executed
    pub total_commands: u64,

    /// Commands by type
    pub commands_by_type: HashMap<String, u64>,

    /// Total paths scanned
    pub total_paths_scanned: u64,

    /// Average scan time per path
    pub avg_scan_time_ms: f64,

    /// Cache hit rate
    pub cache_hit_rate: f64,

    /// Cache hits
    pub cache_hits: u64,

    /// Cache misses
    pub cache_misses: u64,

    /// Total cache size in bytes
    pub total_cache_size_bytes: u64,

    /// Last command execution
    pub last_command: Option<DateTime<Utc>>,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Memory usage in MB
    pub memory_usage_mb: f64,

    /// CPU usage percentage
    pub cpu_usage_percent: f64,

    /// Disk usage in MB
    pub disk_usage_mb: f64,

    /// Concurrent plugin executions
    pub concurrent_plugins: u32,

    /// Plugin queue length
    pub plugin_queue_length: u32,

    /// System uptime in seconds
    pub uptime_seconds: u64,

    /// Last metrics collection
    pub last_collection: DateTime<Utc>,
}

impl Default for ScopeLoaderAnalytics {
    fn default() -> Self {
        Self {
            plugin_performance: HashMap::new(),
            scope_creation_metrics: ScopeCreationMetrics::default(),
            usage_stats: UsageStatistics::default(),
            system_metrics: SystemMetrics::default(),
            last_updated: Utc::now(),
            version: "1.0.0".to_string(),
        }
    }
}

impl Default for ScopeCreationMetrics {
    fn default() -> Self {
        Self {
            total_attempts: 0,
            successful_creations: 0,
            failed_creations: 0,
            success_rate: 0.0,
            avg_confidence: 0.0,
            scope_types: HashMap::new(),
            creation_times: HashMap::new(),
            last_creation: None,
        }
    }
}

impl Default for UsageStatistics {
    fn default() -> Self {
        Self {
            total_commands: 0,
            commands_by_type: HashMap::new(),
            total_paths_scanned: 0,
            avg_scan_time_ms: 0.0,
            cache_hit_rate: 0.0,
            cache_hits: 0,
            cache_misses: 0,
            total_cache_size_bytes: 0,
            last_command: None,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            disk_usage_mb: 0.0,
            concurrent_plugins: 0,
            plugin_queue_length: 0,
            uptime_seconds: 0,
            last_collection: Utc::now(),
        }
    }
}

/// Analytics manager for scope loader
pub struct ScopeLoaderAnalyticsManager {
    analytics: Arc<RwLock<ScopeLoaderAnalytics>>,
    data_path: PathBuf,
    retention_period: Duration,
}

impl ScopeLoaderAnalyticsManager {
    /// Create a new analytics manager
    pub fn new(data_path: PathBuf, retention_period_days: u32) -> RhemaResult<Self> {
        let retention_period = Duration::days(retention_period_days as i64);

        // Ensure data directory exists
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| crate::RhemaError::IoError(e))?;
        }

        let analytics_path = data_path.join("analytics.json");
        let analytics = if analytics_path.exists() {
            let content =
                fs::read_to_string(&analytics_path).map_err(|e| crate::RhemaError::IoError(e))?;

            let analytics: ScopeLoaderAnalytics =
                serde_json::from_str(&content).map_err(|e| crate::RhemaError::InvalidJson {
                    message: e.to_string(),
                })?;

            analytics
        } else {
            ScopeLoaderAnalytics::default()
        };

        Ok(Self {
            analytics: Arc::new(RwLock::new(analytics)),
            data_path,
            retention_period,
        })
    }

    /// Record plugin execution
    pub async fn record_plugin_execution(
        &self,
        plugin_name: &str,
        execution_time_ms: u64,
        success: bool,
        boundaries_detected: u64,
        scopes_created: u64,
        confidence_score: Option<f64>,
        error_message: Option<String>,
    ) -> RhemaResult<()> {
        let mut analytics = self.analytics.write().await;

        let metrics = analytics
            .plugin_performance
            .entry(plugin_name.to_string())
            .or_insert_with(|| PluginPerformanceMetrics {
                plugin_name: plugin_name.to_string(),
                total_executions: 0,
                successful_executions: 0,
                failed_executions: 0,
                avg_execution_time_ms: 0.0,
                total_execution_time_ms: 0,
                last_execution: None,
                boundaries_detected: 0,
                scopes_created: 0,
                confidence_scores: Vec::new(),
                recent_errors: Vec::new(),
            });

        metrics.total_executions += 1;
        metrics.total_execution_time_ms += execution_time_ms;
        metrics.avg_execution_time_ms =
            metrics.total_execution_time_ms as f64 / metrics.total_executions as f64;
        metrics.last_execution = Some(Utc::now());
        metrics.boundaries_detected += boundaries_detected;
        metrics.scopes_created += scopes_created;

        if success {
            metrics.successful_executions += 1;
        } else {
            metrics.failed_executions += 1;
            if let Some(error) = error_message {
                metrics.recent_errors.push(error);
                // Keep only last 10 errors
                if metrics.recent_errors.len() > 10 {
                    metrics.recent_errors.remove(0);
                }
            }
        }

        if let Some(confidence) = confidence_score {
            metrics.confidence_scores.push(confidence);
            // Keep only last 100 confidence scores
            if metrics.confidence_scores.len() > 100 {
                metrics.confidence_scores.remove(0);
            }
        }

        analytics.last_updated = Utc::now();
        self.save_analytics(&analytics).await?;

        Ok(())
    }

    /// Record scope creation attempt
    pub async fn record_scope_creation(
        &self,
        success: bool,
        scope_type: &str,
        confidence: f64,
        creation_time_ms: u64,
    ) -> RhemaResult<()> {
        let mut analytics = self.analytics.write().await;

        analytics.scope_creation_metrics.total_attempts += 1;

        if success {
            analytics.scope_creation_metrics.successful_creations += 1;
        } else {
            analytics.scope_creation_metrics.failed_creations += 1;
        }

        // Update success rate
        analytics.scope_creation_metrics.success_rate =
            analytics.scope_creation_metrics.successful_creations as f64
                / analytics.scope_creation_metrics.total_attempts as f64
                * 100.0;

        // Update average confidence
        let total_confidence = analytics.scope_creation_metrics.avg_confidence
            * (analytics.scope_creation_metrics.total_attempts - 1) as f64
            + confidence;
        analytics.scope_creation_metrics.avg_confidence =
            total_confidence / analytics.scope_creation_metrics.total_attempts as f64;

        // Update scope types
        *analytics
            .scope_creation_metrics
            .scope_types
            .entry(scope_type.to_string())
            .or_insert(0) += 1;

        // Update creation times
        analytics
            .scope_creation_metrics
            .creation_times
            .entry(scope_type.to_string())
            .or_insert_with(Vec::new)
            .push(creation_time_ms);

        analytics.scope_creation_metrics.last_creation = Some(Utc::now());
        analytics.last_updated = Utc::now();

        self.save_analytics(&analytics).await?;

        Ok(())
    }

    /// Record command execution
    pub async fn record_command_execution(
        &self,
        command_type: &str,
        scan_time_ms: u64,
        cache_hit: bool,
        cache_size_bytes: u64,
    ) -> RhemaResult<()> {
        let mut analytics = self.analytics.write().await;

        analytics.usage_stats.total_commands += 1;
        *analytics
            .usage_stats
            .commands_by_type
            .entry(command_type.to_string())
            .or_insert(0) += 1;

        analytics.usage_stats.total_paths_scanned += 1;

        // Update average scan time
        let total_scan_time = analytics.usage_stats.avg_scan_time_ms
            * (analytics.usage_stats.total_paths_scanned - 1) as f64
            + scan_time_ms as f64;
        analytics.usage_stats.avg_scan_time_ms =
            total_scan_time / analytics.usage_stats.total_paths_scanned as f64;

        // Update cache statistics
        if cache_hit {
            analytics.usage_stats.cache_hits += 1;
        } else {
            analytics.usage_stats.cache_misses += 1;
        }

        let total_cache_operations =
            analytics.usage_stats.cache_hits + analytics.usage_stats.cache_misses;
        if total_cache_operations > 0 {
            analytics.usage_stats.cache_hit_rate =
                analytics.usage_stats.cache_hits as f64 / total_cache_operations as f64 * 100.0;
        }

        analytics.usage_stats.total_cache_size_bytes = cache_size_bytes;
        analytics.usage_stats.last_command = Some(Utc::now());
        analytics.last_updated = Utc::now();

        self.save_analytics(&analytics).await?;

        Ok(())
    }

    /// Update system metrics
    pub async fn update_system_metrics(
        &self,
        memory_usage_mb: f64,
        cpu_usage_percent: f64,
        disk_usage_mb: f64,
        concurrent_plugins: u32,
        plugin_queue_length: u32,
        uptime_seconds: u64,
    ) -> RhemaResult<()> {
        let mut analytics = self.analytics.write().await;

        analytics.system_metrics.memory_usage_mb = memory_usage_mb;
        analytics.system_metrics.cpu_usage_percent = cpu_usage_percent;
        analytics.system_metrics.disk_usage_mb = disk_usage_mb;
        analytics.system_metrics.concurrent_plugins = concurrent_plugins;
        analytics.system_metrics.plugin_queue_length = plugin_queue_length;
        analytics.system_metrics.uptime_seconds = uptime_seconds;
        analytics.system_metrics.last_collection = Utc::now();
        analytics.last_updated = Utc::now();

        self.save_analytics(&analytics).await?;

        Ok(())
    }

    /// Get analytics data
    pub async fn get_analytics(&self) -> ScopeLoaderAnalytics {
        self.analytics.read().await.clone()
    }

    /// Get plugin performance metrics
    pub async fn get_plugin_metrics(&self, plugin_name: &str) -> Option<PluginPerformanceMetrics> {
        let analytics = self.analytics.read().await;
        analytics.plugin_performance.get(plugin_name).cloned()
    }

    /// Get scope creation metrics
    pub async fn get_scope_creation_metrics(&self) -> ScopeCreationMetrics {
        let analytics = self.analytics.read().await;
        analytics.scope_creation_metrics.clone()
    }

    /// Get usage statistics
    pub async fn get_usage_stats(&self) -> UsageStatistics {
        let analytics = self.analytics.read().await;
        analytics.usage_stats.clone()
    }

    /// Get system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        let analytics = self.analytics.read().await;
        analytics.system_metrics.clone()
    }

    /// Clean up old analytics data
    pub async fn cleanup_old_data(&self) -> RhemaResult<()> {
        let _cutoff_date = Utc::now() - self.retention_period;
        let mut analytics = self.analytics.write().await;

        // Remove old confidence scores and errors
        for metrics in analytics.plugin_performance.values_mut() {
            // Keep only recent confidence scores (last 30 days)
            metrics.confidence_scores.retain(|_| {
                // For now, just keep the last 100 scores
                // In a real implementation, you'd track timestamps
                true
            });

            // Keep only recent errors (last 10)
            if metrics.recent_errors.len() > 10 {
                metrics.recent_errors.truncate(10);
            }
        }

        // Remove old creation times (keep only last 100 per type)
        for times in analytics.scope_creation_metrics.creation_times.values_mut() {
            if times.len() > 100 {
                times.truncate(100);
            }
        }

        analytics.last_updated = Utc::now();
        self.save_analytics(&analytics).await?;

        Ok(())
    }

    /// Generate analytics report
    pub async fn generate_report(&self) -> RhemaResult<AnalyticsReport> {
        let analytics = self.analytics.read().await;

        let mut report = AnalyticsReport {
            summary: ReportSummary::default(),
            plugin_performance: Vec::new(),
            top_plugins: Vec::new(),
            scope_creation_summary: ScopeCreationSummary::default(),
            usage_summary: UsageSummary::default(),
            system_summary: SystemSummary::default(),
            recommendations: Vec::new(),
        };

        // Generate summary
        report.summary.total_plugins = analytics.plugin_performance.len();
        report.summary.total_executions = analytics
            .plugin_performance
            .values()
            .map(|m| m.total_executions)
            .sum();
        report.summary.total_scopes_created = analytics.scope_creation_metrics.successful_creations;
        report.summary.overall_success_rate = analytics.scope_creation_metrics.success_rate;

        // Generate plugin performance data
        for metrics in analytics.plugin_performance.values() {
            report.plugin_performance.push(metrics.clone());
        }

        // Generate top plugins
        let mut top_plugins: Vec<_> = analytics.plugin_performance.values().collect();
        top_plugins.sort_by(|a, b| b.total_executions.cmp(&a.total_executions));
        report.top_plugins = top_plugins.into_iter().take(5).cloned().collect();

        // Generate scope creation summary
        report.scope_creation_summary = ScopeCreationSummary {
            total_attempts: analytics.scope_creation_metrics.total_attempts,
            successful_creations: analytics.scope_creation_metrics.successful_creations,
            failed_creations: analytics.scope_creation_metrics.failed_creations,
            success_rate: analytics.scope_creation_metrics.success_rate,
            avg_confidence: analytics.scope_creation_metrics.avg_confidence,
            most_common_type: analytics
                .scope_creation_metrics
                .scope_types
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(name, _)| name.clone())
                .unwrap_or_default(),
        };

        // Generate usage summary
        report.usage_summary = UsageSummary {
            total_commands: analytics.usage_stats.total_commands,
            most_used_command: analytics
                .usage_stats
                .commands_by_type
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(name, _)| name.clone())
                .unwrap_or_default(),
            cache_hit_rate: analytics.usage_stats.cache_hit_rate,
            avg_scan_time_ms: analytics.usage_stats.avg_scan_time_ms,
        };

        // Generate system summary
        report.system_summary = SystemSummary {
            memory_usage_mb: analytics.system_metrics.memory_usage_mb,
            cpu_usage_percent: analytics.system_metrics.cpu_usage_percent,
            disk_usage_mb: analytics.system_metrics.disk_usage_mb,
            concurrent_plugins: analytics.system_metrics.concurrent_plugins,
        };

        // Generate recommendations
        report.recommendations = self.generate_recommendations(&analytics).await;

        Ok(report)
    }

    /// Generate recommendations based on analytics
    async fn generate_recommendations(&self, analytics: &ScopeLoaderAnalytics) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Check for low success rates
        if analytics.scope_creation_metrics.success_rate < 80.0 {
            recommendations.push(
                "Consider adjusting confidence thresholds or reviewing plugin configurations to improve success rate".to_string()
            );
        }

        // Check for slow plugins
        for (name, metrics) in &analytics.plugin_performance {
            if metrics.avg_execution_time_ms > 5000.0 {
                recommendations.push(format!(
                    "Plugin '{}' is slow ({}ms avg). Consider optimization or caching",
                    name, metrics.avg_execution_time_ms as u64
                ));
            }
        }

        // Check for low cache hit rate
        if analytics.usage_stats.cache_hit_rate < 50.0 {
            recommendations.push(
                "Cache hit rate is low. Consider increasing cache duration or optimizing cache keys".to_string()
            );
        }

        // Check for high memory usage
        if analytics.system_metrics.memory_usage_mb > 512.0 {
            recommendations.push(
                "Memory usage is high. Consider reducing concurrent plugins or optimizing memory usage".to_string()
            );
        }

        recommendations
    }

    /// Save analytics to file
    async fn save_analytics(&self, analytics: &ScopeLoaderAnalytics) -> RhemaResult<()> {
        let analytics_path = self.data_path.join("analytics.json");
        let content = serde_json::to_string_pretty(analytics).map_err(|e| {
            crate::RhemaError::InvalidJson {
                message: e.to_string(),
            }
        })?;

        fs::write(&analytics_path, content).map_err(|e| crate::RhemaError::IoError(e))?;

        Ok(())
    }
}

/// Analytics report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub summary: ReportSummary,
    pub plugin_performance: Vec<PluginPerformanceMetrics>,
    pub top_plugins: Vec<PluginPerformanceMetrics>,
    pub scope_creation_summary: ScopeCreationSummary,
    pub usage_summary: UsageSummary,
    pub system_summary: SystemSummary,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_plugins: usize,
    pub total_executions: u64,
    pub total_scopes_created: u64,
    pub overall_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeCreationSummary {
    pub total_attempts: u64,
    pub successful_creations: u64,
    pub failed_creations: u64,
    pub success_rate: f64,
    pub avg_confidence: f64,
    pub most_common_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub total_commands: u64,
    pub most_used_command: String,
    pub cache_hit_rate: f64,
    pub avg_scan_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSummary {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub disk_usage_mb: f64,
    pub concurrent_plugins: u32,
}

impl Default for ReportSummary {
    fn default() -> Self {
        Self {
            total_plugins: 0,
            total_executions: 0,
            total_scopes_created: 0,
            overall_success_rate: 0.0,
        }
    }
}

impl Default for ScopeCreationSummary {
    fn default() -> Self {
        Self {
            total_attempts: 0,
            successful_creations: 0,
            failed_creations: 0,
            success_rate: 0.0,
            avg_confidence: 0.0,
            most_common_type: String::new(),
        }
    }
}

impl Default for UsageSummary {
    fn default() -> Self {
        Self {
            total_commands: 0,
            most_used_command: String::new(),
            cache_hit_rate: 0.0,
            avg_scan_time_ms: 0.0,
        }
    }
}

impl Default for SystemSummary {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            disk_usage_mb: 0.0,
            concurrent_plugins: 0,
        }
    }
}
