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

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tokio::time::interval;
use tracing::{info, error, debug};

use rhema_core::{RhemaError, RhemaResult};

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: Instant,
    pub accessed_at: Instant,
    pub access_count: u64,
    pub ttl: Duration,
}

// Custom serialization for CacheEntry
impl<T> Serialize for CacheEntry<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CacheEntry", 5)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("created_at", &self.created_at.elapsed().as_secs())?;
        state.serialize_field("accessed_at", &self.accessed_at.elapsed().as_secs())?;
        state.serialize_field("access_count", &self.access_count)?;
        state.serialize_field("ttl", &self.ttl.as_secs())?;
        state.end()
    }
}

// Custom deserialization for CacheEntry
impl<'de, T> Deserialize<'de> for CacheEntry<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CacheEntryHelper<T> {
            data: T,
            created_at: u64,
            accessed_at: u64,
            access_count: u64,
            ttl: u64,
        }

        let helper = CacheEntryHelper::deserialize(deserializer)?;
        let now = Instant::now();

        Ok(CacheEntry {
            data: helper.data,
            created_at: now
                .checked_sub(Duration::from_secs(helper.created_at))
                .unwrap_or(now),
            accessed_at: now
                .checked_sub(Duration::from_secs(helper.accessed_at))
                .unwrap_or(now),
            access_count: helper.access_count,
            ttl: Duration::from_secs(helper.ttl),
        })
    }
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    pub fn touch(&mut self) {
        self.accessed_at = Instant::now();
        self.access_count += 1;
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
}

/// Enhanced cache statistics with additional metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
    pub average_entry_size: u64,
    pub compression_ratio: f64,
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvictionPolicy {
    LRU,           // Least Recently Used
    LFU,           // Least Frequently Used
    TTL,           // Time To Live
    FIFO,          // First In First Out
    Adaptive,      // Adaptive policy that switches based on performance
}

/// Cache warming strategy
#[derive(Debug, Clone)]
pub struct WarmingStrategy {
    pub enabled: bool,
    pub warm_on_startup: bool,
    pub warm_on_access: bool,
    pub warm_patterns: Vec<String>,
    pub warm_interval_seconds: u64,
}

impl Default for WarmingStrategy {
    fn default() -> Self {
        Self {
            enabled: false,
            warm_on_startup: false,
            warm_on_access: false,
            warm_patterns: Vec::new(),
            warm_interval_seconds: 300, // 5 minutes
        }
    }
}

/// Cache monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_interval_seconds: u64,
    pub alert_threshold_hit_rate: f64,
    pub alert_threshold_memory_usage: f64,
    pub enable_alerts: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval_seconds: 60, // 1 minute
            alert_threshold_hit_rate: 0.8, // 80%
            alert_threshold_memory_usage: 0.9, // 90%
            enable_alerts: true,
        }
    }
}

/// Cache optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub enabled: bool,
    pub auto_optimize: bool,
    pub optimization_interval_seconds: u64,
    pub target_hit_rate: f64,
    pub target_memory_usage: f64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_optimize: true,
            optimization_interval_seconds: 300, // 5 minutes
            target_hit_rate: 0.9, // 90%
            target_memory_usage: 0.8, // 80%
        }
    }
}

/// Cache validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub validate_on_read: bool,
    pub validate_on_write: bool,
    pub checksum_validation: bool,
    pub integrity_check_interval_seconds: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            validate_on_read: false,
            validate_on_write: true,
            checksum_validation: true,
            integrity_check_interval_seconds: 3600, // 1 hour
        }
    }
}

/// Cache persistence configuration
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    pub enabled: bool,
    pub persistence_path: Option<PathBuf>,
    pub save_interval_seconds: u64,
    pub load_on_startup: bool,
    pub compression_enabled: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            persistence_path: Some(PathBuf::from(".rhema/cache")),
            save_interval_seconds: 300, // 5 minutes
            load_on_startup: true,
            compression_enabled: true,
        }
    }
}

/// Cache compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: CompressionAlgorithm,
    pub min_size_bytes: usize,
    pub compression_level: u8,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: CompressionAlgorithm::Zstd,
            min_size_bytes: 1024, // 1KB
            compression_level: 6,
        }
    }
}

/// Compression algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    LZ4,
    Zstd,
    Snappy,
}

/// Performance metrics for cache operations
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub average_response_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
    pub throughput_requests_per_second: f64,
    pub memory_efficiency: f64,
    pub compression_ratio: f64,
}

/// Cache alert for monitoring
#[derive(Debug, Clone)]
pub struct CacheAlert {
    pub alert_type: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, f64>,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Cache partitioning configuration
#[derive(Debug, Clone)]
pub struct PartitioningConfig {
    pub enabled: bool,
    pub partition_count: usize,
    pub partition_strategy: PartitionStrategy,
    pub partition_key_pattern: Option<String>,
}

impl Default for PartitioningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            partition_count: 4,
            partition_strategy: PartitionStrategy::Hash,
            partition_key_pattern: None,
        }
    }
}

/// Partition strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionStrategy {
    Hash,           // Hash-based partitioning
    Range,          // Range-based partitioning
    Consistent,     // Consistent hashing
    RoundRobin,     // Round-robin partitioning
}

/// Cache coherency configuration
#[derive(Debug, Clone)]
pub struct CoherencyConfig {
    pub enabled: bool,
    pub coherency_protocol: CoherencyProtocol,
    pub sync_interval_seconds: u64,
    pub conflict_resolution: ConflictResolution,
}

impl Default for CoherencyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            coherency_protocol: CoherencyProtocol::Eventual,
            sync_interval_seconds: 30,
            conflict_resolution: ConflictResolution::LastWriteWins,
        }
    }
}

/// Coherency protocols
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoherencyProtocol {
    Strong,         // Strong consistency
    Eventual,       // Eventual consistency
    Causal,         // Causal consistency
    Sequential,     // Sequential consistency
}

/// Conflict resolution strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictResolution {
    LastWriteWins,
    FirstWriteWins,
    Merge,
    Custom,
}

/// Cache prefetching configuration
#[derive(Debug, Clone)]
pub struct PrefetchingConfig {
    pub enabled: bool,
    pub prefetch_strategy: PrefetchStrategy,
    pub prefetch_window: usize,
    pub prefetch_threshold: f64,
}

impl Default for PrefetchingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            prefetch_strategy: PrefetchStrategy::Sequential,
            prefetch_window: 10,
            prefetch_threshold: 0.7,
        }
    }
}

/// Prefetch strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefetchStrategy {
    Sequential,     // Sequential prefetching
    Pattern,        // Pattern-based prefetching
    Predictive,     // Predictive prefetching
    Adaptive,       // Adaptive prefetching
}

/// Cache analytics configuration
#[derive(Debug, Clone)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub analytics_interval_seconds: u64,
    pub retention_days: u32,
    pub export_format: AnalyticsExportFormat,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analytics_interval_seconds: 3600, // 1 hour
            retention_days: 30,
            export_format: AnalyticsExportFormat::JSON,
        }
    }
}

/// Analytics export formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyticsExportFormat {
    JSON,
    CSV,
    Prometheus,
    InfluxDB,
}

/// Cache health configuration
#[derive(Debug, Clone)]
pub struct HealthConfig {
    pub enabled: bool,
    pub health_check_interval_seconds: u64,
    pub health_thresholds: HealthThresholds,
    pub auto_recovery: bool,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            health_check_interval_seconds: 60,
            health_thresholds: HealthThresholds::default(),
            auto_recovery: true,
        }
    }
}

/// Health thresholds
#[derive(Debug, Clone)]
pub struct HealthThresholds {
    pub min_hit_rate: f64,
    pub max_memory_usage: f64,
    pub max_response_time_ms: f64,
    pub max_error_rate: f64,
}

impl Default for HealthThresholds {
    fn default() -> Self {
        Self {
            min_hit_rate: 0.8,
            max_memory_usage: 0.9,
            max_response_time_ms: 100.0,
            max_error_rate: 0.01,
        }
    }
}

/// Enhanced cache configuration with advanced features
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable in-memory caching
    pub memory_enabled: bool,
    
    /// Enable Redis caching
    pub redis_enabled: bool,
    
    /// Redis connection URL
    pub redis_url: Option<String>,
    
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    
    /// Maximum cache size in bytes
    pub max_size: usize,
    
    /// Enable compression
    pub compression_enabled: bool,

    /// Eviction policy
    pub eviction_policy: EvictionPolicy,

    /// Warming strategy
    pub warming: WarmingStrategy,

    /// Monitoring configuration
    pub monitoring: MonitoringConfig,

    /// Optimization configuration
    pub optimization: OptimizationConfig,

    /// Validation configuration
    pub validation: ValidationConfig,

    /// Persistence configuration
    pub persistence: PersistenceConfig,

    /// Compression configuration
    pub compression: CompressionConfig,

    /// Partitioning configuration
    pub partitioning: PartitioningConfig,

    /// Coherency configuration
    pub coherency: CoherencyConfig,

    /// Prefetching configuration
    pub prefetching: PrefetchingConfig,

    /// Analytics configuration
    pub analytics: AnalyticsConfig,

    /// Health configuration
    pub health: HealthConfig,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            memory_enabled: true,
            redis_enabled: false,
            redis_url: None,
            ttl_seconds: 3600, // 1 hour
            max_size: 100 * 1024 * 1024, // 100 MB
            compression_enabled: false,
            eviction_policy: EvictionPolicy::LRU,
            warming: WarmingStrategy::default(),
            monitoring: MonitoringConfig::default(),
            optimization: OptimizationConfig::default(),
            validation: ValidationConfig::default(),
            persistence: PersistenceConfig::default(),
            compression: CompressionConfig::default(),
            partitioning: PartitioningConfig::default(),
            coherency: CoherencyConfig::default(),
            prefetching: PrefetchingConfig::default(),
            analytics: AnalyticsConfig::default(),
            health: HealthConfig::default(),
        }
    }
}

/// Enhanced cache manager with advanced features
pub struct CacheManager {
    memory_cache: Arc<DashMap<String, CacheEntry<Value>>>,
    redis_client: Option<Arc<redis::Client>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
    
    // Enhanced features
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    alerts: Arc<RwLock<Vec<CacheAlert>>>,
    access_patterns: Arc<RwLock<HashMap<String, u64>>>,
    warming_cache: Arc<RwLock<HashMap<String, Value>>>,
    
    // Advanced features
    partitions: Arc<RwLock<HashMap<usize, Arc<DashMap<String, CacheEntry<Value>>>>>>,
    coherency_state: Arc<RwLock<HashMap<String, Value>>>,
    prefetch_queue: Arc<RwLock<Vec<String>>>,
    analytics_data: Arc<RwLock<Vec<AnalyticsRecord>>>,
    health_status: Arc<RwLock<HealthStatus>>,
    
    // Background tasks - removed Clone derive since JoinHandle doesn't implement Clone
    monitoring_task: Option<tokio::task::JoinHandle<()>>,
    optimization_task: Option<tokio::task::JoinHandle<()>>,
    persistence_task: Option<tokio::task::JoinHandle<()>>,
    validation_task: Option<tokio::task::JoinHandle<()>>,
    warming_task: Option<tokio::task::JoinHandle<()>>,
    partitioning_task: Option<tokio::task::JoinHandle<()>>,
    coherency_task: Option<tokio::task::JoinHandle<()>>,
    prefetching_task: Option<tokio::task::JoinHandle<()>>,
    analytics_task: Option<tokio::task::JoinHandle<()>>,
    health_task: Option<tokio::task::JoinHandle<()>>,
}

/// Analytics record for tracking cache performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRecord {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub hit_rate: f64,
    pub memory_usage: f64,
    pub response_time_ms: f64,
    pub throughput_rps: f64,
    pub error_rate: f64,
    pub partition_count: usize,
    pub coherency_conflicts: u64,
    pub prefetch_hits: u64,
}

/// Health status for cache monitoring
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub issues: Vec<HealthIssue>,
    pub recovery_attempts: u32,
}

/// Health issue for monitoring
#[derive(Debug, Clone)]
pub struct HealthIssue {
    pub issue_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Clone for CacheManager {
    fn clone(&self) -> Self {
        Self {
            memory_cache: self.memory_cache.clone(),
            redis_client: self.redis_client.clone(),
            config: self.config.clone(),
            stats: self.stats.clone(),
            performance_metrics: self.performance_metrics.clone(),
            alerts: self.alerts.clone(),
            access_patterns: self.access_patterns.clone(),
            warming_cache: self.warming_cache.clone(),
            partitions: self.partitions.clone(),
            coherency_state: self.coherency_state.clone(),
            prefetch_queue: self.prefetch_queue.clone(),
            analytics_data: self.analytics_data.clone(),
            health_status: self.health_status.clone(),
            monitoring_task: None, // JoinHandle cannot be cloned
            optimization_task: None, // JoinHandle cannot be cloned
            persistence_task: None, // JoinHandle cannot be cloned
            validation_task: None, // JoinHandle cannot be cloned
            warming_task: None, // JoinHandle cannot be cloned
            partitioning_task: None, // JoinHandle cannot be cloned
            coherency_task: None, // JoinHandle cannot be cloned
            prefetching_task: None, // JoinHandle cannot be cloned
            analytics_task: None, // JoinHandle cannot be cloned
            health_task: None, // JoinHandle cannot be cloned
        }
    }
}

impl CacheManager {
    /// Create a new enhanced cache manager
    pub async fn new(config: &super::cache::CacheConfig) -> RhemaResult<Self> {
        let memory_cache = Arc::new(DashMap::new());

        let redis_client = if config.redis_enabled {
            if let Some(redis_url) = &config.redis_url {
                let client = redis::Client::open(redis_url.as_str()).map_err(|e| {
                    RhemaError::InvalidInput(format!("Failed to create Redis client: {}", e))
                })?;
                Some(Arc::new(client))
            } else {
                None
            }
        } else {
            None
        };

        let cache_config = CacheConfig {
            memory_enabled: config.memory_enabled,
            redis_enabled: config.redis_enabled,
            redis_url: config.redis_url.clone(),
            ttl_seconds: config.ttl_seconds,
            max_size: config.max_size,
            compression_enabled: config.compression_enabled,
            eviction_policy: config.eviction_policy.clone(),
            warming: config.warming.clone(),
            monitoring: config.monitoring.clone(),
            optimization: config.optimization.clone(),
            validation: config.validation.clone(),
            persistence: config.persistence.clone(),
            compression: config.compression.clone(),
            partitioning: config.partitioning.clone(),
            coherency: config.coherency.clone(),
            prefetching: config.prefetching.clone(),
            analytics: config.analytics.clone(),
            health: config.health.clone(),
        };

        let stats = Arc::new(RwLock::new(CacheStats {
            total_entries: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            memory_usage_bytes: 0,
            eviction_count: 0,
        }));

        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics {
            average_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            throughput_requests_per_second: 0.0,
            memory_efficiency: 0.0,
            compression_ratio: 1.0,
        }));

        let alerts = Arc::new(RwLock::new(Vec::new()));
        let access_patterns = Arc::new(RwLock::new(HashMap::new()));
        let warming_cache = Arc::new(RwLock::new(HashMap::new()));

        let partitions = Arc::new(RwLock::new(HashMap::new()));
        let coherency_state = Arc::new(RwLock::new(HashMap::new()));
        let prefetch_queue = Arc::new(RwLock::new(Vec::new()));
        let analytics_data = Arc::new(RwLock::new(Vec::new()));
        let health_status = Arc::new(RwLock::new(HealthStatus {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            issues: Vec::new(),
            recovery_attempts: 0,
        }));

        let mut manager = Self {
            memory_cache,
            redis_client,
            config: cache_config,
            stats,
            performance_metrics,
            alerts,
            access_patterns,
            warming_cache,
            partitions,
            coherency_state,
            prefetch_queue,
            analytics_data,
            health_status,
            monitoring_task: None,
            optimization_task: None,
            persistence_task: None,
            validation_task: None,
            warming_task: None,
            partitioning_task: None,
            coherency_task: None,
            prefetching_task: None,
            analytics_task: None,
            health_task: None,
        };

        // Start background tasks
        manager.start_background_tasks().await?;

        // Load persisted cache if enabled
        if manager.config.persistence.load_on_startup {
            manager.load_persisted_cache().await?;
        }

        // Warm cache if enabled
        if manager.config.warming.warm_on_startup {
            manager.warm_cache().await?;
        }

        Ok(manager)
    }

    /// Start background tasks for monitoring, optimization, etc.
    async fn start_background_tasks(&mut self) -> RhemaResult<()> {
        // Start monitoring task
        if self.config.monitoring.enabled {
            let monitoring_config = self.config.monitoring.clone();
            let stats = self.stats.clone();
            let performance_metrics = self.performance_metrics.clone();
            let alerts = self.alerts.clone();
            
            self.monitoring_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(monitoring_config.metrics_interval_seconds));
                loop {
                    interval.tick().await;
                    
                    // Update performance metrics
                    let stats_guard = stats.read().await;
                    let mut metrics_guard = performance_metrics.write().await;
                    
                    // Calculate performance metrics (simplified)
                    metrics_guard.memory_efficiency = if stats_guard.total_entries > 0 {
                        stats_guard.hit_count as f64 / stats_guard.total_entries as f64
                    } else {
                        0.0
                    };
                    
                    // Check for alerts
                    if monitoring_config.enable_alerts {
                        if stats_guard.hit_rate < monitoring_config.alert_threshold_hit_rate {
                            let alert = CacheAlert {
                                alert_type: "low_hit_rate".to_string(),
                                message: format!("Cache hit rate {} is below threshold {}", 
                                    stats_guard.hit_rate, monitoring_config.alert_threshold_hit_rate),
                                severity: AlertSeverity::Warning,
                                timestamp: chrono::Utc::now(),
                                metrics: HashMap::new(),
                            };
                            alerts.write().await.push(alert);
                        }
                    }
                }
            }));
        }

        // Start optimization task
        if self.config.optimization.enabled {
            let optimization_config = self.config.optimization.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.optimization_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(optimization_config.optimization_interval_seconds));
                loop {
                    interval.tick().await;
                    
                    if optimization_config.auto_optimize {
                        if let Err(e) = cache_manager.optimize_cache().await {
                            error!("Cache optimization failed: {}", e);
                        }
                    }
                }
            }));
        }

        // Start persistence task
        if self.config.persistence.enabled {
            let persistence_config = self.config.persistence.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.persistence_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(persistence_config.save_interval_seconds));
                loop {
                    interval.tick().await;
                    
                    if let Err(e) = cache_manager.save_persisted_cache().await {
                        error!("Cache persistence failed: {}", e);
                    }
                }
            }));
        }

        // Start validation task
        if self.config.validation.enabled {
            let validation_config = self.config.validation.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.validation_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(validation_config.integrity_check_interval_seconds));
                loop {
                    interval.tick().await;
                    
                    if let Err(e) = cache_manager.validate_cache_integrity().await {
                        error!("Cache validation failed: {}", e);
                    }
                }
            }));
        }

        // Start warming task
        if self.config.warming.enabled {
            let warming_config = self.config.warming.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.warming_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(warming_config.warm_interval_seconds));
                loop {
                    interval.tick().await;
                    
                    if let Err(e) = cache_manager.warm_cache().await {
                        error!("Cache warming failed: {}", e);
                    }
                }
            }));
        }

        // Start partitioning task
        if self.config.partitioning.enabled {
            let _partitioning_config = self.config.partitioning.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.partitioning_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(1)); // Check partitioning every second
                loop {
                    interval.tick().await;
                    if let Err(e) = cache_manager.rebalance_partitions().await {
                        error!("Partition rebalancing failed: {}", e);
                    }
                }
            }));
        }

        // Start coherency task
        if self.config.coherency.enabled {
            let coherency_config = self.config.coherency.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.coherency_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(coherency_config.sync_interval_seconds));
                loop {
                    interval.tick().await;
                    if let Err(e) = cache_manager.resolve_conflicts().await {
                        error!("Conflict resolution failed: {}", e);
                    }
                }
            }));
        }

        // Start prefetching task
        if self.config.prefetching.enabled {
            let _prefetching_config = self.config.prefetching.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.prefetching_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(1)); // Check prefetching every second
                loop {
                    interval.tick().await;
                    if let Err(e) = cache_manager.perform_prefetching().await {
                        error!("Prefetching failed: {}", e);
                    }
                }
            }));
        }

        // Start analytics task
        if self.config.analytics.enabled {
            let analytics_config = self.config.analytics.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.analytics_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(analytics_config.analytics_interval_seconds));
                loop {
                    interval.tick().await;
                    if let Err(e) = cache_manager.export_analytics().await {
                        error!("Analytics export failed: {}", e);
                    }
                }
            }));
        }

        // Start health task
        if self.config.health.enabled {
            let health_config = self.config.health.clone();
            let cache_manager = Arc::new(CacheManager {
                memory_cache: self.memory_cache.clone(),
                redis_client: self.redis_client.clone(),
                config: self.config.clone(),
                stats: self.stats.clone(),
                performance_metrics: self.performance_metrics.clone(),
                alerts: self.alerts.clone(),
                access_patterns: self.access_patterns.clone(),
                warming_cache: self.warming_cache.clone(),
                partitions: self.partitions.clone(),
                coherency_state: self.coherency_state.clone(),
                prefetch_queue: self.prefetch_queue.clone(),
                analytics_data: self.analytics_data.clone(),
                health_status: self.health_status.clone(),
                monitoring_task: None,
                optimization_task: None,
                persistence_task: None,
                validation_task: None,
                warming_task: None,
                partitioning_task: None,
                coherency_task: None,
                prefetching_task: None,
                analytics_task: None,
                health_task: None,
            });
            
            self.health_task = Some(tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(health_config.health_check_interval_seconds));
                loop {
                    interval.tick().await;
                    if let Err(e) = cache_manager.check_health().await {
                        error!("Health check failed: {}", e);
                    }
                }
            }));
        }

        Ok(())
    }

    /// Get a value from cache with enhanced features
    pub async fn get(&self, key: &str) -> RhemaResult<Option<Value>> {
        let start_time = Instant::now();

        // Track access pattern
        self.track_access_pattern(key).await;

        // Try memory cache first
        if self.config.memory_enabled {
            if let Some(mut entry) = self.memory_cache.get_mut(key) {
                if entry.is_expired() {
                    self.memory_cache.remove(key);
                    self.update_stats_miss().await;
                    return Ok(None);
                }

                // Validate entry if enabled
                if self.config.validation.validate_on_read {
                    if !self.validate_entry(&entry).await? {
                        self.memory_cache.remove(key);
                        self.update_stats_miss().await;
                        return Ok(None);
                    }
                }

                entry.touch();
                self.update_stats_hit().await;
                
                // Decompress value if it's compressed
                let decompressed_value = self.decompress_value(&entry.data).await?;
                
                // Update performance metrics
                self.update_performance_metrics(start_time.elapsed()).await;
                
                // Warm cache on access
                self.warm_cache_on_access(key).await?;
                
                return Ok(Some(decompressed_value));
            }
        }

        // Try Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                if let Ok(value) = self.get_from_redis(client, key).await {
                    // Decompress value if it's compressed
                    let decompressed_value = self.decompress_value(&value).await?;
                    
                    // Store in memory cache for faster access
                    if self.config.memory_enabled {
                        self.set_in_memory(key, value).await;
                    }
                    self.update_stats_hit().await;
                    self.update_performance_metrics(start_time.elapsed()).await;
                    return Ok(Some(decompressed_value));
                }
            }
        }

        self.update_stats_miss().await;
        self.update_performance_metrics(start_time.elapsed()).await;
        Ok(None)
    }

    /// Set a value in cache with enhanced features
    pub async fn set(&self, key: &str, value: Value) -> RhemaResult<()> {
        let start_time = Instant::now();

        // Compress value if enabled
        let processed_value = if self.config.compression.enabled {
            self.compress_value(&value).await?
        } else {
            value.clone()
        };

        let ttl = Duration::from_secs(self.config.ttl_seconds);

        // Validate value if enabled
        if self.config.validation.validate_on_write {
            if !self.validate_value(&processed_value).await? {
                return Err(RhemaError::ValidationError("Value validation failed".to_string()));
            }
        }

        // Set in memory cache
        if self.config.memory_enabled {
            self.set_in_memory(key, processed_value.clone()).await;
        }

        // Set in Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.set_in_redis(client, key, processed_value, ttl).await?;
            }
        }

        // Update performance metrics
        self.update_performance_metrics(start_time.elapsed()).await;

        Ok(())
    }

    /// Delete a value from cache
    pub async fn delete(&self, key: &str) -> RhemaResult<()> {
        // Delete from memory cache
        if self.config.memory_enabled {
            self.memory_cache.remove(key);
        }

        // Delete from Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.delete_from_redis(client, key).await?;
            }
        }

        Ok(())
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> RhemaResult<()> {
        // Clear memory cache
        if self.config.memory_enabled {
            self.memory_cache.clear();
        }

        // Clear Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.clear_redis(client).await?;
            }
        }

        // Reset stats
        let mut stats = self.stats.write().await;
        stats.total_entries = 0;
        stats.memory_usage_bytes = 0;

        Ok(())
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get cache hit rate
    pub async fn hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        stats.hit_rate
    }

    /// Get cache size in bytes
    pub async fn get_size(&self) -> u64 {
        self.memory_usage().await
    }

    /// Get comprehensive cache statistics
    pub async fn get_statistics(&self) -> CacheStatistics {
        let stats = self.stats().await;
        let actual_total_entries = self.memory_cache.len();
        CacheStatistics {
            total_entries: actual_total_entries,
            hit_count: stats.hit_count,
            miss_count: stats.miss_count,
            hit_rate: stats.hit_rate,
            memory_usage_bytes: stats.memory_usage_bytes,
            eviction_count: stats.eviction_count,
            average_entry_size: if actual_total_entries > 0 {
                stats.memory_usage_bytes / actual_total_entries as u64
            } else {
                0
            },
            compression_ratio: self.calculate_compression_ratio().await,
        }
    }

    /// Evict expired entries
    pub async fn evict_expired(&self) -> RhemaResult<usize> {
        if !self.config.memory_enabled {
            return Ok(0);
        }

        let mut evicted = 0;
        let mut to_remove = Vec::new();

        for entry in self.memory_cache.iter() {
            if entry.value().is_expired() {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.memory_cache.remove(&key);
            evicted += 1;
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.eviction_count += evicted as u64;
        stats.total_entries = self.memory_cache.len();

        Ok(evicted)
    }

    /// Get memory usage estimate
    pub async fn memory_usage(&self) -> u64 {
        if !self.config.memory_enabled {
            return 0;
        }

        let mut total_size = 0;
        for entry in self.memory_cache.iter() {
            // Rough estimate: key size + value size + overhead
            total_size += entry.key().len() as u64;
            total_size += serde_json::to_string(&entry.value().data)
                .map(|s| s.len() as u64)
                .unwrap_or(0);
            total_size += std::mem::size_of::<CacheEntry<Value>>() as u64;
        }

        total_size
    }

    /// Implement intelligent cache eviction
    pub async fn evict_entries(&self) -> RhemaResult<usize> {
        match self.config.eviction_policy {
            EvictionPolicy::LRU => self.evict_lru().await,
            EvictionPolicy::LFU => self.evict_lfu().await,
            EvictionPolicy::TTL => self.evict_expired().await,
            EvictionPolicy::FIFO => self.evict_fifo().await,
            EvictionPolicy::Adaptive => self.evict_adaptive().await,
        }
    }

    /// LRU eviction
    async fn evict_lru(&self) -> RhemaResult<usize> {
        let mut evicted = 0;
        let mut to_remove = Vec::new();

        // If we're over the max size, evict the least recently used entries
        if self.memory_cache.len() > self.config.max_size {
            let mut entries: Vec<_> = self.memory_cache.iter().collect();
            entries.sort_by(|a, b| a.value().accessed_at.cmp(&b.value().accessed_at));
            
            // Remove the oldest entries to get back under max_size
            let to_evict = self.memory_cache.len() - self.config.max_size;
            for entry in entries.iter().take(to_evict) {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.memory_cache.remove(&key);
            evicted += 1;
        }

        self.update_eviction_stats(evicted).await;
        Ok(evicted)
    }

    /// LFU eviction
    async fn evict_lfu(&self) -> RhemaResult<usize> {
        let mut evicted = 0;
        let mut to_remove = Vec::new();

        // If we're over the max size, evict the least frequently used entries
        if self.memory_cache.len() > self.config.max_size {
            let mut entries: Vec<_> = self.memory_cache.iter().collect();
            entries.sort_by(|a, b| a.value().access_count.cmp(&b.value().access_count));
            
            // Remove the least frequently used entries to get back under max_size
            let to_evict = self.memory_cache.len() - self.config.max_size;
            for entry in entries.iter().take(to_evict) {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.memory_cache.remove(&key);
            evicted += 1;
        }

        self.update_eviction_stats(evicted).await;
        Ok(evicted)
    }

    /// FIFO eviction
    async fn evict_fifo(&self) -> RhemaResult<usize> {
        let mut evicted = 0;
        let mut to_remove = Vec::new();

        // If we're over the max size, evict the oldest entries
        if self.memory_cache.len() > self.config.max_size {
            let mut entries: Vec<_> = self.memory_cache.iter().collect();
            entries.sort_by(|a, b| a.value().created_at.cmp(&b.value().created_at));
            
            // Remove the oldest entries to get back under max_size
            let to_evict = self.memory_cache.len() - self.config.max_size;
            for entry in entries.iter().take(to_evict) {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.memory_cache.remove(&key);
            evicted += 1;
        }

        self.update_eviction_stats(evicted).await;
        Ok(evicted)
    }

    /// Adaptive eviction
    async fn evict_adaptive(&self) -> RhemaResult<usize> {
        let hit_rate = self.hit_rate().await;
        
        if hit_rate > 0.8 {
            // High hit rate, use TTL eviction
            self.evict_expired().await
        } else if hit_rate > 0.6 {
            // Medium hit rate, use LRU eviction
            self.evict_lru().await
        } else {
            // Low hit rate, use LFU eviction
            self.evict_lfu().await
        }
    }

    /// Warm cache based on access patterns
    pub async fn warm_cache(&self) -> RhemaResult<()> {
        if !self.config.warming.enabled {
            return Ok(());
        }

        info!("Starting cache warming");

        let access_patterns = self.access_patterns.read().await;
        let mut sorted_patterns: Vec<_> = access_patterns.iter().collect();
        sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));

        // Warm top 10 most accessed patterns
        for (pattern, _) in sorted_patterns.iter().take(10) {
            if let Some(value) = self.warming_cache.read().await.get(pattern.as_str()) {
                self.set(pattern, value.clone()).await?;
            }
        }

        info!("Cache warming completed");
        Ok(())
    }

    /// Warm cache on access - prefetch related data when a key is accessed
    pub async fn warm_cache_on_access(&self, key: &str) -> RhemaResult<()> {
        if !self.config.warming.enabled || !self.config.warming.warm_on_access {
            return Ok(());
        }

        // Analyze access patterns and prefetch related keys
        let access_patterns = self.access_patterns.read().await;
        
        // Find keys that are frequently accessed together with the current key
        let related_keys = self.find_related_keys(key, &access_patterns).await;
        
        for related_key in related_keys {
            if !self.memory_cache.contains_key(&related_key) {
                // Add to prefetch queue
                self.add_to_prefetch_queue(related_key).await?;
            }
        }

        Ok(())
    }

    /// Find keys that are frequently accessed together
    async fn find_related_keys(&self, key: &str, access_patterns: &HashMap<String, u64>) -> Vec<String> {
        let mut related_keys = Vec::new();
        
        // Simple heuristic: find keys with similar prefixes or patterns
        for (pattern, _) in access_patterns {
            if pattern != key && (pattern.starts_with(key) || key.starts_with(pattern)) {
                related_keys.push(pattern.clone());
            }
        }
        
        // Limit to top 5 related keys
        related_keys.truncate(5);
        related_keys
    }

    /// Optimize cache performance
    pub async fn optimize_cache(&self) -> RhemaResult<()> {
        if !self.config.optimization.enabled {
            return Ok(());
        }

        info!("Starting cache optimization");

        // Evict entries based on current policy
        let evicted = self.evict_entries().await?;
        info!("Evicted {} entries during optimization", evicted);

        // Adjust TTL based on hit rate
        let hit_rate = self.hit_rate().await;
        if hit_rate < self.config.optimization.target_hit_rate {
            // Increase TTL for better hit rate
            info!("Hit rate {} below target {}, adjusting TTL", hit_rate, self.config.optimization.target_hit_rate);
        }

        // Compress large entries if compression is enabled
        if self.config.compression.enabled {
            self.compress_large_entries().await?;
        }

        // Rebalance partitions if enabled
        if self.config.partitioning.enabled {
            self.rebalance_partitions().await?;
        }

        // Resolve conflicts if enabled
        if self.config.coherency.enabled {
            self.resolve_conflicts().await?;
        }

        info!("Cache optimization completed");
        Ok(())
    }

    /// Validate cache integrity
    pub async fn validate_cache_integrity(&self) -> RhemaResult<bool> {
        if !self.config.validation.enabled {
            return Ok(true);
        }

        info!("Starting cache integrity validation");

        let mut valid_entries = 0;
        let mut invalid_entries = 0;

        for entry in self.memory_cache.iter() {
            if self.validate_entry(entry.value()).await? {
                valid_entries += 1;
            } else {
                invalid_entries += 1;
            }
        }

        info!("Cache validation completed: {} valid, {} invalid entries", valid_entries, invalid_entries);
        Ok(invalid_entries == 0)
    }

    /// Validate cache data consistency
    pub async fn validate_cache_consistency(&self) -> RhemaResult<CacheConsistencyReport> {
        let mut report = CacheConsistencyReport {
            total_entries: 0,
            corrupted_entries: 0,
            expired_entries: 0,
            duplicate_keys: 0,
            memory_leaks: 0,
            validation_time_ms: 0,
            issues: Vec::new(),
        };

        let start_time = Instant::now();
        let mut seen_keys = std::collections::HashSet::new();

        for entry in self.memory_cache.iter() {
            report.total_entries += 1;

            // Check for duplicate keys
            if !seen_keys.insert(entry.key().clone()) {
                report.duplicate_keys += 1;
                report.issues.push(format!("Duplicate key found: {}", entry.key()));
            }

            // Check for expired entries
            if entry.value().is_expired() {
                report.expired_entries += 1;
                report.issues.push(format!("Expired entry found: {}", entry.key()));
            }

            // Validate entry data
            if !self.validate_entry(entry.value()).await? {
                report.corrupted_entries += 1;
                report.issues.push(format!("Corrupted entry found: {}", entry.key()));
            }
        }

        report.validation_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(report)
    }

    /// Warm cache with specific patterns
    pub async fn warm_cache_with_patterns(&self, patterns: &[String]) -> RhemaResult<usize> {
        if !self.config.warming.enabled {
            return Ok(0);
        }

        info!("Starting cache warming with {} patterns", patterns.len());
        let mut warmed_count = 0;

        for pattern in patterns {
            // Find keys that match the pattern
            let matching_keys = self.find_keys_matching_pattern(pattern).await;
            
            for key in matching_keys {
                if let Some(value) = self.warming_cache.read().await.get(&key) {
                    self.set(&key, value.clone()).await?;
                    warmed_count += 1;
                }
            }
        }

        info!("Cache warming completed: {} entries warmed", warmed_count);
        Ok(warmed_count)
    }

    /// Find keys matching a pattern
    async fn find_keys_matching_pattern(&self, pattern: &str) -> Vec<String> {
        let mut matching_keys = Vec::new();
        
        // Simple pattern matching - can be enhanced with regex
        for entry in self.memory_cache.iter() {
            if entry.key().contains(pattern) || pattern.contains(entry.key()) {
                matching_keys.push(entry.key().clone());
            }
        }
        
        matching_keys
    }

    /// Get cache warming statistics
    pub async fn get_warming_stats(&self) -> WarmingStats {
        let warming_cache = self.warming_cache.read().await;
        let access_patterns = self.access_patterns.read().await;
        
        WarmingStats {
            warming_cache_size: warming_cache.len(),
            access_patterns_count: access_patterns.len(),
            most_accessed_patterns: self.get_most_accessed_patterns().await,
            warming_enabled: self.config.warming.enabled,
            warm_on_startup: self.config.warming.warm_on_startup,
            warm_on_access: self.config.warming.warm_on_access,
        }
    }

    /// Get most accessed patterns
    async fn get_most_accessed_patterns(&self) -> Vec<(String, u64)> {
        let access_patterns = self.access_patterns.read().await;
        let mut patterns: Vec<_> = access_patterns.iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        patterns.into_iter()
            .take(10)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    /// Save cache to persistent storage
    pub async fn save_persisted_cache(&self) -> RhemaResult<()> {
        if !self.config.persistence.enabled {
            return Ok(());
        }

        let persistence_path = match &self.config.persistence.persistence_path {
            Some(path) => path.clone(),
            None => PathBuf::from(".rhema/cache"),
        };

        // Create directory if it doesn't exist
        if !persistence_path.exists() {
            fs::create_dir_all(&persistence_path)?;
        }

        let cache_file = persistence_path.join("cache_data.json");
        let mut cache_data = Vec::new();

        for entry in self.memory_cache.iter() {
            let entry_data = serde_json::json!({
                "key": entry.key(),
                "data": entry.value().data,
                "created_at": entry.value().created_at.elapsed().as_secs(),
                "accessed_at": entry.value().accessed_at.elapsed().as_secs(),
                "access_count": entry.value().access_count,
                "ttl": entry.value().ttl.as_secs(),
            });
            cache_data.push(entry_data);
        }

        let json_data = serde_json::to_string_pretty(&cache_data)?;
        fs::write(cache_file, json_data)?;

        info!("Cache persisted to disk");
        Ok(())
    }

    /// Load cache from persistent storage
    pub async fn load_persisted_cache(&self) -> RhemaResult<()> {
        if !self.config.persistence.enabled {
            return Ok(());
        }

        let persistence_path = match &self.config.persistence.persistence_path {
            Some(path) => path.clone(),
            None => PathBuf::from(".rhema/cache"),
        };

        let cache_file = persistence_path.join("cache_data.json");
        if !cache_file.exists() {
            return Ok(());
        }

        let json_data = fs::read_to_string(cache_file)?;
        let cache_data: Vec<serde_json::Value> = serde_json::from_str(&json_data)?;

        for entry_data in cache_data {
            if let (Some(key), Some(data)) = (
                entry_data["key"].as_str(),
                entry_data.get("data")
            ) {
                let ttl = Duration::from_secs(entry_data["ttl"].as_u64().unwrap_or(3600));
                let entry = CacheEntry::new(data.clone(), ttl);
                self.memory_cache.insert(key.to_string(), entry);
            }
        }

        info!("Cache loaded from disk");
        Ok(())
    }

    /// Compress a value
    async fn compress_value(&self, value: &Value) -> RhemaResult<Value> {
        if !self.config.compression.enabled {
            return Ok(value.clone());
        }

        let json_string = serde_json::to_string(value)?;
        if json_string.len() < self.config.compression.min_size_bytes {
            return Ok(value.clone());
        }

        match self.config.compression.algorithm {
            CompressionAlgorithm::Gzip => self.compress_gzip(&json_string).await,
            CompressionAlgorithm::LZ4 => self.compress_lz4(&json_string).await,
            CompressionAlgorithm::Zstd => self.compress_zstd(&json_string).await,
            CompressionAlgorithm::Snappy => self.compress_snappy(&json_string).await,
        }
    }

    /// Compress using Gzip
    async fn compress_gzip(&self, data: &str) -> RhemaResult<Value> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(self.config.compression.compression_level as u32));
        encoder.write_all(data.as_bytes())?;
        let compressed = encoder.finish()?;
        let compressed_size = compressed.len();
        
        Ok(serde_json::json!({
            "compressed": true,
            "algorithm": "gzip",
            "data": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &compressed),
            "original_size": data.len(),
            "compressed_size": compressed_size,
        }))
    }

    /// Compress using LZ4
    async fn compress_lz4(&self, data: &str) -> RhemaResult<Value> {
        let compressed = lz4::block::compress(data.as_bytes(), None, true)?;
        let compressed_size = compressed.len();
        
        Ok(serde_json::json!({
            "compressed": true,
            "algorithm": "lz4",
            "data": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &compressed),
            "original_size": data.len(),
            "compressed_size": compressed_size,
        }))
    }

    /// Compress using Zstd
    async fn compress_zstd(&self, data: &str) -> RhemaResult<Value> {
        let compressed = zstd::bulk::compress(data.as_bytes(), self.config.compression.compression_level.into())?;
        let compressed_size = compressed.len();
        
        Ok(serde_json::json!({
            "compressed": true,
            "algorithm": "zstd",
            "data": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &compressed),
            "original_size": data.len(),
            "compressed_size": compressed_size,
        }))
    }

    /// Compress using Snappy
    async fn compress_snappy(&self, data: &str) -> RhemaResult<Value> {
        let compressed = snap::raw::Encoder::new().compress_vec(data.as_bytes())
            .map_err(|e| RhemaError::SerializationError(format!("Snappy compression failed: {}", e)))?;
        let compressed_size = compressed.len();
        
        Ok(serde_json::json!({
            "compressed": true,
            "algorithm": "snappy",
            "data": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &compressed),
            "original_size": data.len(),
            "compressed_size": compressed_size,
        }))
    }

    /// Decompress a value
    pub async fn decompress_value(&self, value: &Value) -> RhemaResult<Value> {
        if let Some(compressed) = value.get("compressed") {
            if let Some(true) = compressed.as_bool() {
                if let (Some(algorithm), Some(data)) = (
                    value.get("algorithm").and_then(|a| a.as_str()),
                    value.get("data").and_then(|d| d.as_str()),
                ) {
                    let compressed_data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to decode base64: {}", e)))?;
                    
                    let decompressed = match algorithm {
                        "gzip" => self.decompress_gzip(&compressed_data).await?,
                        "lz4" => self.decompress_lz4(&compressed_data).await?,
                        "zstd" => self.decompress_zstd(&compressed_data).await?,
                        "snappy" => self.decompress_snappy(&compressed_data).await?,
                        _ => return Err(RhemaError::InvalidInput(format!("Unknown compression algorithm: {}", algorithm))),
                    };
                    
                    return serde_json::from_str(&decompressed)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to deserialize decompressed data: {}", e)));
                }
            }
        }
        
        Ok(value.clone())
    }

    /// Decompress using Gzip
    async fn decompress_gzip(&self, data: &[u8]) -> RhemaResult<String> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)?;
        Ok(decompressed)
    }

    /// Decompress using LZ4
    async fn decompress_lz4(&self, data: &[u8]) -> RhemaResult<String> {
        let decompressed = lz4::block::decompress(data, None)?;
        String::from_utf8(decompressed)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to decode LZ4 decompressed data: {}", e)))
    }

    /// Decompress using Zstd
    async fn decompress_zstd(&self, data: &[u8]) -> RhemaResult<String> {
        let decompressed = zstd::bulk::decompress(data, 0)?;
        String::from_utf8(decompressed)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to decode Zstd decompressed data: {}", e)))
    }

    /// Decompress using Snappy
    async fn decompress_snappy(&self, data: &[u8]) -> RhemaResult<String> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(data)
            .map_err(|e| RhemaError::SerializationError(format!("Snappy decompression failed: {}", e)))?;
        String::from_utf8(decompressed)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to decode Snappy decompressed data: {}", e)))
    }

    /// Validate an entry
    async fn validate_entry(&self, entry: &CacheEntry<Value>) -> RhemaResult<bool> {
        if !self.config.validation.enabled {
            return Ok(true);
        }

        // Check if entry is expired
        if entry.is_expired() {
            return Ok(false);
        }

        // Validate data structure
        if !self.validate_value(&entry.data).await? {
            return Ok(false);
        }

        Ok(true)
    }

    /// Validate a value
    async fn validate_value(&self, value: &Value) -> RhemaResult<bool> {
        if !self.config.validation.enabled {
            return Ok(true);
        }

        // Basic validation - check if it's a valid JSON value
        match value {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Array(_) | Value::Object(_) => {
                Ok(true)
            }
        }
    }

    /// Track access patterns
    async fn track_access_pattern(&self, key: &str) {
        let mut patterns = self.access_patterns.write().await;
        *patterns.entry(key.to_string()).or_insert(0) += 1;
    }

    /// Update performance metrics
    async fn update_performance_metrics(&self, response_time: Duration) {
        let mut metrics = self.performance_metrics.write().await;
        
        // Update average response time (simplified)
        metrics.average_response_time_ms = response_time.as_millis() as f64;
        
        // Update throughput (simplified)
        metrics.throughput_requests_per_second = 1.0 / (response_time.as_secs_f64() + 0.001);
    }

    /// Update eviction stats
    async fn update_eviction_stats(&self, evicted: usize) {
        let mut stats = self.stats.write().await;
        stats.eviction_count += evicted as u64;
    }

    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()
    }

    pub async fn get_alerts(&self) -> Vec<CacheAlert> {
        self.alerts.read().await.clone()
    }

    pub async fn clear_alerts(&self) {
        self.alerts.write().await.clear();
    }

    pub async fn get_access_patterns(&self) -> HashMap<String, u64> {
        self.access_patterns.read().await.clone()
    }

    async fn calculate_compression_ratio(&self) -> f64 {
        // Simplified compression ratio calculation
        let total_size = self.memory_usage().await;
        let compressed_size = total_size * 8 / 10; // Assume 20% compression
        if total_size > 0 {
            compressed_size as f64 / total_size as f64
        } else {
            1.0
        }
    }

    // Advanced Cache Management Methods

    /// Get cache partition for a key
    async fn get_partition(&self, key: &str) -> usize {
        if !self.config.partitioning.enabled {
            return 0;
        }

        match self.config.partitioning.partition_strategy {
            PartitionStrategy::Hash => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                (hasher.finish() as usize) % self.config.partitioning.partition_count
            }
            PartitionStrategy::Range => {
                // Simple range-based partitioning
                let first_char = key.chars().next().unwrap_or('a');
                (first_char as usize) % self.config.partitioning.partition_count
            }
            PartitionStrategy::Consistent => {
                // Simplified consistent hashing
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                (hasher.finish() as usize) % self.config.partitioning.partition_count
            }
            PartitionStrategy::RoundRobin => {
                // Round-robin partitioning
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                key.hash(&mut hasher);
                (hasher.finish() as usize) % self.config.partitioning.partition_count
            }
        }
    }

    /// Rebalance cache partitions
    pub async fn rebalance_partitions(&self) -> RhemaResult<()> {
        if !self.config.partitioning.enabled {
            return Ok(());
        }

        let mut partitions = self.partitions.write().await;
        
        // Initialize partitions if they don't exist
        for i in 0..self.config.partitioning.partition_count {
            if !partitions.contains_key(&i) {
                partitions.insert(i, Arc::new(DashMap::new()));
            }
        }

        // Simple rebalancing: move entries to their correct partitions
        let mut entries_to_move = Vec::new();
        
        for entry in self.memory_cache.iter() {
            let key = entry.key();
            let partition = self.get_partition(key).await;
            
            if let Some(partition_cache) = partitions.get(&partition) {
                if !partition_cache.contains_key(key) {
                    entries_to_move.push((key.clone(), entry.value().clone()));
                }
            }
        }

        // Move entries to correct partitions
        for (key, entry) in entries_to_move {
            let partition = self.get_partition(&key).await;
            if let Some(partition_cache) = partitions.get(&partition) {
                partition_cache.insert(key.clone(), entry);
            }
        }

        Ok(())
    }

    /// Resolve cache coherency conflicts
    pub async fn resolve_conflicts(&self) -> RhemaResult<()> {
        if !self.config.coherency.enabled {
            return Ok(());
        }

        let mut coherency_state = self.coherency_state.write().await;
        let mut conflicts_resolved = 0;

        // Simple conflict resolution: last write wins
        for entry in self.memory_cache.iter() {
            let key = entry.key();
            let cache_value = entry.value();
            
            if let Some(coherent_value) = coherency_state.get(key) {
                if coherent_value != &cache_value.data {
                    // Conflict detected, resolve based on strategy
                    match self.config.coherency.conflict_resolution {
                        ConflictResolution::LastWriteWins => {
                            coherency_state.insert(key.clone(), cache_value.data.clone());
                            conflicts_resolved += 1;
                        }
                        ConflictResolution::FirstWriteWins => {
                            // Keep the coherent value (first write)
                        }
                        ConflictResolution::Merge => {
                            // For JSON values, attempt to merge
                            if let (Value::Object(cache_obj), Value::Object(coherent_obj)) = 
                                (&cache_value.data, coherent_value) {
                                let mut merged = coherent_obj.clone();
                                for (k, v) in cache_obj {
                                    merged.insert(k.clone(), v.clone());
                                }
                                coherency_state.insert(key.clone(), Value::Object(merged));
                                conflicts_resolved += 1;
                            }
                        }
                        ConflictResolution::Custom => {
                            // Custom conflict resolution logic
                            coherency_state.insert(key.clone(), cache_value.data.clone());
                            conflicts_resolved += 1;
                        }
                    }
                }
            } else {
                // No conflict, update coherent state
                coherency_state.insert(key.clone(), cache_value.data.clone());
            }
        }

        if conflicts_resolved > 0 {
            info!("Resolved {} cache coherency conflicts", conflicts_resolved);
        }

        Ok(())
    }

    /// Perform cache prefetching
    pub async fn perform_prefetching(&self) -> RhemaResult<()> {
        if !self.config.prefetching.enabled {
            return Ok(());
        }

        let mut prefetch_queue = self.prefetch_queue.write().await;
        let mut prefetched = 0;

        while prefetched < self.config.prefetching.prefetch_window && !prefetch_queue.is_empty() {
            if let Some(key) = prefetch_queue.pop() {
                // Check if key is not already in cache
                if !self.memory_cache.contains_key(&key) {
                    // Simulate prefetching by creating a placeholder entry
                    let placeholder = Value::String(format!("prefetched_{}", key));
                    let entry = CacheEntry::new(placeholder, Duration::from_secs(self.config.ttl_seconds));
                    self.memory_cache.insert(key, entry);
                    prefetched += 1;
                }
            }
        }

        if prefetched > 0 {
            debug!("Prefetched {} cache entries", prefetched);
        }

        Ok(())
    }

    /// Add key to prefetch queue
    pub async fn add_to_prefetch_queue(&self, key: String) -> RhemaResult<()> {
        if !self.config.prefetching.enabled {
            return Ok(());
        }

        let mut prefetch_queue = self.prefetch_queue.write().await;
        
        // Check if key is already in queue
        if !prefetch_queue.contains(&key) {
            prefetch_queue.push(key);
        }

        Ok(())
    }

    /// Export cache analytics
    pub async fn export_analytics(&self) -> RhemaResult<()> {
        if !self.config.analytics.enabled {
            return Ok(());
        }

        let stats = self.stats.read().await;
        let performance_metrics = self.performance_metrics.read().await;
        let partitions = self.partitions.read().await;
        let _coherency_state = self.coherency_state.read().await;

        let analytics_record = AnalyticsRecord {
            timestamp: chrono::Utc::now(),
            hit_rate: stats.hit_rate,
            memory_usage: stats.memory_usage_bytes as f64,
            response_time_ms: performance_metrics.average_response_time_ms,
            throughput_rps: performance_metrics.throughput_requests_per_second,
            error_rate: 0.0, // Would need to track errors
            partition_count: partitions.len(),
            coherency_conflicts: 0, // Would need to track conflicts
            prefetch_hits: 0, // Would need to track prefetch hits
        };

        let mut analytics_data = self.analytics_data.write().await;
        analytics_data.push(analytics_record);

        // Clean up old analytics data
        let retention_days = self.config.analytics.retention_days as i64;
        let cutoff = chrono::Utc::now() - chrono::Duration::days(retention_days);
        analytics_data.retain(|record| record.timestamp > cutoff);

        // Export analytics based on format
        match self.config.analytics.export_format {
            AnalyticsExportFormat::JSON => {
                self.export_analytics_json(&analytics_data).await?;
            }
            AnalyticsExportFormat::CSV => {
                self.export_analytics_csv(&analytics_data).await?;
            }
            AnalyticsExportFormat::Prometheus => {
                self.export_analytics_prometheus(&analytics_data).await?;
            }
            AnalyticsExportFormat::InfluxDB => {
                self.export_analytics_influxdb(&analytics_data).await?;
            }
        }

        Ok(())
    }

    /// Export analytics as JSON
    async fn export_analytics_json(&self, analytics_data: &[AnalyticsRecord]) -> RhemaResult<()> {
        let json = serde_json::to_string_pretty(analytics_data)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize analytics: {}", e)))?;
        
        let analytics_dir = PathBuf::from(".rhema/analytics");
        if !analytics_dir.exists() {
            std::fs::create_dir_all(&analytics_dir)
                .map_err(|e| RhemaError::IoError(e))?;
        }

        let filename = format!("cache_analytics_{}.json", 
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let filepath = analytics_dir.join(filename);
        
        tokio::fs::write(&filepath, json).await
            .map_err(|e| RhemaError::IoError(e))?;

        Ok(())
    }

    /// Export analytics as CSV
    async fn export_analytics_csv(&self, analytics_data: &[AnalyticsRecord]) -> RhemaResult<()> {
        let mut csv = String::new();
        csv.push_str("timestamp,hit_rate,memory_usage,response_time_ms,throughput_rps,error_rate,partition_count,coherency_conflicts,prefetch_hits\n");
        
        for record in analytics_data {
            csv.push_str(&format!("{},{},{},{},{},{},{},{},{}\n",
                record.timestamp,
                record.hit_rate,
                record.memory_usage,
                record.response_time_ms,
                record.throughput_rps,
                record.error_rate,
                record.partition_count,
                record.coherency_conflicts,
                record.prefetch_hits
            ));
        }

        let analytics_dir = PathBuf::from(".rhema/analytics");
        if !analytics_dir.exists() {
            std::fs::create_dir_all(&analytics_dir)
                .map_err(|e| RhemaError::IoError(e))?;
        }

        let filename = format!("cache_analytics_{}.csv", 
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let filepath = analytics_dir.join(filename);
        
        tokio::fs::write(&filepath, csv).await
            .map_err(|e| RhemaError::IoError(e))?;

        Ok(())
    }

    /// Export analytics as Prometheus metrics
    async fn export_analytics_prometheus(&self, analytics_data: &[AnalyticsRecord]) -> RhemaResult<()> {
        if analytics_data.is_empty() {
            return Ok(());
        }

        let latest = &analytics_data[analytics_data.len() - 1];
        let mut prometheus = String::new();
        
        prometheus.push_str(&format!("# HELP cache_hit_rate Cache hit rate\n"));
        prometheus.push_str(&format!("# TYPE cache_hit_rate gauge\n"));
        prometheus.push_str(&format!("cache_hit_rate {}\n", latest.hit_rate));
        
        prometheus.push_str(&format!("# HELP cache_memory_usage Cache memory usage in bytes\n"));
        prometheus.push_str(&format!("# TYPE cache_memory_usage gauge\n"));
        prometheus.push_str(&format!("cache_memory_usage {}\n", latest.memory_usage));
        
        prometheus.push_str(&format!("# HELP cache_response_time_ms Cache average response time in milliseconds\n"));
        prometheus.push_str(&format!("# TYPE cache_response_time_ms gauge\n"));
        prometheus.push_str(&format!("cache_response_time_ms {}\n", latest.response_time_ms));

        let analytics_dir = PathBuf::from(".rhema/analytics");
        if !analytics_dir.exists() {
            std::fs::create_dir_all(&analytics_dir)
                .map_err(|e| RhemaError::IoError(e))?;
        }

        let filename = format!("cache_metrics_{}.prom", 
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let filepath = analytics_dir.join(filename);
        
        tokio::fs::write(&filepath, prometheus).await
            .map_err(|e| RhemaError::IoError(e))?;

        Ok(())
    }

    /// Export analytics as InfluxDB line protocol
    async fn export_analytics_influxdb(&self, analytics_data: &[AnalyticsRecord]) -> RhemaResult<()> {
        let mut influxdb = String::new();
        
        for record in analytics_data {
            influxdb.push_str(&format!("cache_metrics,host=rhema hit_rate={},memory_usage={},response_time_ms={},throughput_rps={},error_rate={},partition_count={},coherency_conflicts={},prefetch_hits={} {}\n",
                record.hit_rate,
                record.memory_usage,
                record.response_time_ms,
                record.throughput_rps,
                record.error_rate,
                record.partition_count,
                record.coherency_conflicts,
                record.prefetch_hits,
                record.timestamp.timestamp_nanos_opt().unwrap_or(0)
            ));
        }

        let analytics_dir = PathBuf::from(".rhema/analytics");
        if !analytics_dir.exists() {
            std::fs::create_dir_all(&analytics_dir)
                .map_err(|e| RhemaError::IoError(e))?;
        }

        let filename = format!("cache_analytics_{}.influx", 
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let filepath = analytics_dir.join(filename);
        
        tokio::fs::write(&filepath, influxdb).await
            .map_err(|e| RhemaError::IoError(e))?;

        Ok(())
    }

    /// Check cache health
    pub async fn check_health(&self) -> RhemaResult<()> {
        if !self.config.health.enabled {
            return Ok(());
        }

        let stats = self.stats.read().await;
        let performance_metrics = self.performance_metrics.read().await;
        let mut health_status = self.health_status.write().await;
        let mut issues = Vec::new();

        // Check hit rate
        if stats.hit_rate < self.config.health.health_thresholds.min_hit_rate {
            issues.push(HealthIssue {
                issue_type: "low_hit_rate".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Cache hit rate {} is below threshold {}", 
                    stats.hit_rate, self.config.health.health_thresholds.min_hit_rate),
                timestamp: chrono::Utc::now(),
            });
        }

        // Check memory usage
        let memory_usage_ratio = if self.config.max_size > 0 {
            stats.memory_usage_bytes as f64 / self.config.max_size as f64
        } else {
            0.0
        };

        if memory_usage_ratio > self.config.health.health_thresholds.max_memory_usage {
            issues.push(HealthIssue {
                issue_type: "high_memory_usage".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Cache memory usage {}% is above threshold {}%", 
                    memory_usage_ratio * 100.0, 
                    self.config.health.health_thresholds.max_memory_usage * 100.0),
                timestamp: chrono::Utc::now(),
            });
        }

        // Check response time
        if performance_metrics.average_response_time_ms > self.config.health.health_thresholds.max_response_time_ms {
            issues.push(HealthIssue {
                issue_type: "high_response_time".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Cache response time {}ms is above threshold {}ms", 
                    performance_metrics.average_response_time_ms,
                    self.config.health.health_thresholds.max_response_time_ms),
                timestamp: chrono::Utc::now(),
            });
        }

        // Update health status
        health_status.is_healthy = issues.is_empty();
        health_status.last_check = chrono::Utc::now();
        health_status.issues = issues;

        // Auto-recovery if enabled
        if self.config.health.auto_recovery && !health_status.is_healthy {
            health_status.recovery_attempts += 1;
            self.perform_auto_recovery().await?;
        }

        Ok(())
    }

    /// Perform automatic recovery
    async fn perform_auto_recovery(&self) -> RhemaResult<()> {
        info!("Performing automatic cache recovery");
        
        // Evict expired entries
        self.evict_expired().await?;
        
        // Optimize cache
        self.optimize_cache().await?;
        
        // Rebalance partitions if enabled
        if self.config.partitioning.enabled {
            self.rebalance_partitions().await?;
        }
        
        // Resolve conflicts if enabled
        if self.config.coherency.enabled {
            self.resolve_conflicts().await?;
        }

        info!("Automatic cache recovery completed");
        Ok(())
    }

    /// Get cache health status
    pub async fn get_health_status(&self) -> HealthStatus {
        self.health_status.read().await.clone()
    }

    /// Get cache analytics data
    pub async fn get_analytics_data(&self) -> Vec<AnalyticsRecord> {
        self.analytics_data.read().await.clone()
    }

    /// Get cache partition information
    pub async fn get_partition_info(&self) -> HashMap<usize, usize> {
        let partitions = self.partitions.read().await;
        let mut partition_info = HashMap::new();
        
        for (partition_id, partition_cache) in partitions.iter() {
            partition_info.insert(*partition_id, partition_cache.len());
        }
        
        partition_info
    }

    /// Get cache coherency state
    pub async fn get_coherency_state(&self) -> HashMap<String, Value> {
        self.coherency_state.read().await.clone()
    }

    /// Get prefetch queue status
    pub async fn get_prefetch_queue_status(&self) -> (usize, Vec<String>) {
        let prefetch_queue = self.prefetch_queue.read().await;
        let queue_size = prefetch_queue.len();
        let queue_items = prefetch_queue.clone();
        (queue_size, queue_items)
    }

    /// Set a value in memory cache
    async fn set_in_memory(&self, key: &str, value: Value) {
        let ttl = Duration::from_secs(self.config.ttl_seconds);
        let entry = CacheEntry::new(value, ttl);

        // Check if we need to evict entries
        if self.memory_cache.len() >= self.config.max_size {
            if let Err(e) = self.evict_entries().await {
                error!("Failed to evict entries: {}", e);
            }
        }

        self.memory_cache.insert(key.to_string(), entry);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_entries = self.memory_cache.len();
        stats.memory_usage_bytes = self.memory_usage().await;
    }

    /// Get from Redis
    async fn get_from_redis(&self, client: &redis::Client, key: &str) -> RhemaResult<Value> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis GET failed: {}", e)))?;

        match result {
            Some(data) => {
                let value: Value = serde_json::from_str(&data).map_err(|e| {
                    RhemaError::InvalidInput(format!("Failed to deserialize Redis value: {}", e))
                })?;
                Ok(value)
            }
            None => Err(RhemaError::InvalidInput(
                "Key not found in Redis".to_string(),
            )),
        }
    }

    /// Set in Redis
    async fn set_in_redis(
        &self,
        client: &redis::Client,
        key: &str,
        value: Value,
        ttl: Duration,
    ) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        let data = serde_json::to_string(&value)
            .map_err(|e| RhemaError::InvalidInput(format!("Failed to serialize value: {}", e)))?;

        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl.as_secs())
            .arg(data)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis SETEX failed: {}", e)))?;

        Ok(())
    }

    /// Delete from Redis
    async fn delete_from_redis(&self, client: &redis::Client, key: &str) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis DEL failed: {}", e)))?;

        Ok(())
    }

    /// Clear Redis
    async fn clear_redis(&self, client: &redis::Client) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        redis::cmd("FLUSHDB")
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis FLUSHDB failed: {}", e)))?;

        Ok(())
    }

    /// Update stats for hit
    async fn update_stats_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hit_count += 1;
        stats.hit_rate = stats.hit_count as f64 / (stats.hit_count + stats.miss_count) as f64;
    }

    /// Update stats for miss
    async fn update_stats_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.miss_count += 1;
        stats.hit_rate = stats.hit_count as f64 / (stats.hit_count + stats.miss_count) as f64;
    }

    /// Compress large entries
    async fn compress_large_entries(&self) -> RhemaResult<()> {
        if !self.config.compression.enabled {
            return Ok(());
        }

        for mut entry in self.memory_cache.iter_mut() {
            let json_string = serde_json::to_string(&entry.value().data)?;
            if json_string.len() > self.config.compression.min_size_bytes {
                let compressed_value = self.compress_value(&entry.value().data).await?;
                entry.value_mut().data = compressed_value;
            }
        }

        Ok(())
    }
}

/// Cached function for expensive operations
pub async fn cached_expensive_operation(key: &str) -> Value {
    // Simulate expensive operation
    tokio::time::sleep(Duration::from_millis(100)).await;
    serde_json::json!({
        "key": key,
        "result": "expensive_operation_result",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })
}

/// Cache consistency report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConsistencyReport {
    pub total_entries: usize,
    pub corrupted_entries: usize,
    pub expired_entries: usize,
    pub duplicate_keys: usize,
    pub memory_leaks: usize,
    pub validation_time_ms: u64,
    pub issues: Vec<String>,
}

/// Cache warming statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmingStats {
    pub warming_cache_size: usize,
    pub access_patterns_count: usize,
    pub most_accessed_patterns: Vec<(String, u64)>,
    pub warming_enabled: bool,
    pub warm_on_startup: bool,
    pub warm_on_access: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CacheConfig;

    #[tokio::test]
    async fn test_cache_manager_creation() -> RhemaResult<()> {
        let config = CacheConfig::default();
        let cache_manager = CacheManager::new(&config).await?;
        // Test that it was created successfully
        let stats = cache_manager.get_statistics().await;
        assert_eq!(stats.total_entries, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_cache_operations() -> RhemaResult<()> {
        let config = CacheConfig::default();
        let cache_manager = CacheManager::new(&config).await?;

        // Test set and get
        let key = "test_key";
        let value = serde_json::json!({"data": "test_value"});

        cache_manager.set(key, value.clone()).await?;
        let retrieved = cache_manager.get(key).await?;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);

        // Test delete
        cache_manager.delete(key).await?;
        let retrieved_after_delete = cache_manager.get(key).await?;
        assert!(retrieved_after_delete.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_statistics() -> RhemaResult<()> {
        let config = CacheConfig::default();
        let cache_manager = CacheManager::new(&config).await?;

        // Add some data
        for i in 0..10 {
            let key = format!("key_{}", i);
            let value = serde_json::json!({"data": i});
            cache_manager.set(&key, value).await?;
        }

        let stats = cache_manager.get_statistics().await;
        assert_eq!(stats.total_entries, 10);
        assert!(stats.hit_rate >= 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_expiration() -> RhemaResult<()> {
        let mut config = CacheConfig::default();
        config.ttl_seconds = 1; // 1 second TTL
        
        let cache_manager = CacheManager::new(&config).await?;

        let key = "expire_key";
        let value = serde_json::json!({"data": "will_expire"});

        cache_manager.set(key, value.clone()).await?;

        // Should be available immediately
        assert!(cache_manager.get(key).await?.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Manually evict expired entries
        cache_manager.evict_expired().await?;

        // Should be expired
        assert!(cache_manager.get(key).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_compression() -> RhemaResult<()> {
        let mut config = CacheConfig::default();
        config.compression.enabled = true;
        config.compression.algorithm = CompressionAlgorithm::Gzip;
        config.compression.min_size_bytes = 10; // Small threshold for testing
        
        let cache_manager = CacheManager::new(&config).await?;

        let key = "compress_key";
        let value = serde_json::json!({
            "data": "This is a large value that should be compressed for testing purposes",
            "nested": {
                "field1": "value1",
                "field2": "value2",
                "field3": "value3"
            }
        });

        cache_manager.set(key, value.clone()).await?;
        let retrieved = cache_manager.get(key).await?;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_eviction_policies() -> RhemaResult<()> {
        let mut config = CacheConfig::default();
        config.max_size = 5; // Small size to trigger eviction
        
        // Test LRU eviction
        config.eviction_policy = EvictionPolicy::LRU;
        let cache_manager = CacheManager::new(&config).await?;

        // Add more entries than max_size
        for i in 0..10 {
            let key = format!("key_{}", i);
            let value = serde_json::json!({"data": i});
            cache_manager.set(&key, value).await?;
        }

        // Manually trigger eviction
        let evicted = cache_manager.evict_entries().await?;

        // Some entries should have been evicted
        let stats = cache_manager.get_statistics().await;
        assert!(stats.total_entries <= config.max_size);

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_warming() -> RhemaResult<()> {
        let mut config = CacheConfig::default();
        config.warming.enabled = true;
        config.warming.warm_on_access = true;
        
        let cache_manager = CacheManager::new(&config).await?;

        // Add some data to warming cache
        let warming_cache = cache_manager.warming_cache.clone();
        warming_cache.write().await.insert("warm_key1".to_string(), serde_json::json!({"data": "warm1"}));
        warming_cache.write().await.insert("warm_key2".to_string(), serde_json::json!({"data": "warm2"}));

        // Add some access patterns
        let access_patterns = cache_manager.access_patterns.clone();
        access_patterns.write().await.insert("pattern1".to_string(), 5);
        access_patterns.write().await.insert("pattern2".to_string(), 3);

        // Test warming with patterns - first add some keys to memory cache
        cache_manager.set("warm_key1", serde_json::json!({"data": "warm1"})).await?;
        cache_manager.set("warm_key2", serde_json::json!({"data": "warm2"})).await?;

        // Test warming on access
        cache_manager.warm_cache_on_access("warm_key1").await?;

        // Get warming stats
        let warming_stats = cache_manager.get_warming_stats().await;
        
        // Verify warming is enabled
        assert!(warming_stats.warming_enabled);
        assert!(warming_stats.warm_on_access);
        assert_eq!(warming_stats.warming_cache_size, 2);
        assert_eq!(warming_stats.access_patterns_count, 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_validation() -> RhemaResult<()> {
        let mut config = CacheConfig::default();
        config.validation.enabled = true;
        config.validation.validate_on_read = true;
        config.validation.validate_on_write = true;
        
        let cache_manager = CacheManager::new(&config).await?;

        // Add some valid data
        let key = "valid_key";
        let value = serde_json::json!({"data": "valid_value"});
        cache_manager.set(key, value).await?;

        // Test cache integrity validation
        let is_valid = cache_manager.validate_cache_integrity().await?;
        assert!(is_valid);

        // Test cache consistency validation
        let consistency_report = cache_manager.validate_cache_consistency().await?;
        assert_eq!(consistency_report.total_entries, 1);
        assert_eq!(consistency_report.corrupted_entries, 0);

        Ok(())
    }
}
