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

use crate::{
    Config, ConfigAuditLog, ConfigEnvironment, ConfigHealth, ConfigStats, CURRENT_CONFIG_VERSION,
};
use chrono::{DateTime, Utc};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};


/// Lock file configuration for Rhema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockConfig {
    /// Configuration version
    pub version: String,

    /// Resolution strategies configuration
    pub resolution: ResolutionConfig,

    /// Conflict resolution preferences
    pub conflict_resolution: ConflictResolutionConfig,

    /// Lock file update policies
    pub update_policies: UpdatePoliciesConfig,

    /// Validation rules configuration
    pub validation: ValidationConfig,

    /// Performance tuning options
    pub performance: PerformanceConfig,

    /// Environment-specific overrides
    pub environments: HashMap<ConfigEnvironment, EnvironmentLockConfig>,

    /// Custom settings
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,

    /// Audit log
    pub audit_log: ConfigAuditLog,

    /// Health status
    pub health: ConfigHealth,

    /// Statistics
    pub stats: ConfigStats,

    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Resolution strategies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionConfig {
    /// Default resolution strategy
    pub default_strategy: ResolutionStrategy,

    /// Strategy overrides for specific dependency types
    pub strategy_overrides: HashMap<String, ResolutionStrategy>,

    /// Strategy overrides for specific scopes
    pub scope_strategies: HashMap<String, ResolutionStrategy>,

    /// Whether to use smart resolution
    pub smart_resolution_enabled: bool,

    /// Whether to prefer stable versions
    pub prefer_stable: bool,

    /// Whether to allow pre-release versions
    pub allow_prereleases: bool,

    /// Whether to allow development versions
    pub allow_development: bool,

    /// Maximum dependency depth
    pub max_depth: u32,

    /// Whether to resolve transitive dependencies
    pub resolve_transitive: bool,

    /// Whether to pin versions by default
    pub pin_by_default: bool,

    /// Version constraint preferences
    pub version_constraints: VersionConstraintConfig,
}

/// Resolution strategy types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    /// Latest version
    Latest,
    /// Earliest compatible version
    Earliest,
    /// Pinned version (from lock file)
    Pinned,
    /// Version range
    Range,
    /// Compatible version
    Compatible,
    /// Conservative (prefer stable, tested versions)
    Conservative,
    /// Aggressive (prefer latest features)
    Aggressive,
    /// Smart selection based on compatibility scores
    Smart,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Version constraint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraintConfig {
    /// Default constraint type
    pub default_type: ConstraintType,

    /// Whether to use caret ranges by default
    pub use_caret_ranges: bool,

    /// Whether to use tilde ranges by default
    pub use_tilde_ranges: bool,

    /// Whether to allow wildcard versions
    pub allow_wildcards: bool,

    /// Whether to allow exact versions
    pub allow_exact: bool,

    /// Whether to allow version ranges
    pub allow_ranges: bool,

    /// Minimum version constraint
    pub min_version: Option<String>,

    /// Maximum version constraint
    pub max_version: Option<String>,
}

/// Version constraint types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Exact version
    Exact,
    /// Version range
    Range,
    /// Caret range
    Caret,
    /// Tilde range
    Tilde,
    /// Wildcard
    Wildcard,
    /// Latest
    Latest,
    /// Earliest
    Earliest,
}

/// Conflict resolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionConfig {
    /// Primary conflict resolution strategy
    pub primary_strategy: ConflictResolutionStrategy,

    /// Fallback strategies in order of preference
    pub fallback_strategies: Vec<ConflictResolutionStrategy>,

    /// Whether to enable automatic conflict detection
    pub enable_auto_detection: bool,

    /// Whether to track resolution history
    pub track_history: bool,

    /// Maximum resolution attempts
    pub max_attempts: usize,

    /// Whether to allow user prompts for manual resolution
    pub allow_user_prompts: bool,

    /// Whether to prefer stable versions
    pub prefer_stable: bool,

    /// Whether to enforce pinned versions strictly
    pub strict_pinning: bool,

    /// Compatibility threshold for smart selection (0.0-1.0)
    pub compatibility_threshold: f64,

    /// Whether to enable parallel resolution
    pub parallel_resolution: bool,

    /// Maximum parallel resolution threads
    pub max_parallel_threads: usize,

    /// Timeout for resolution operations (in seconds)
    pub timeout_seconds: u64,

    /// Whether to auto-resolve low severity conflicts
    pub auto_resolve_low_severity: bool,

    /// Whether to auto-resolve medium severity conflicts
    pub auto_resolve_medium_severity: bool,

    /// Whether to fail on high severity conflicts
    pub fail_on_high_severity: bool,

    /// Whether to fail on critical severity conflicts
    pub fail_on_critical_severity: bool,

    /// Conflict resolution preferences by dependency type
    pub dependency_type_preferences: HashMap<String, ConflictResolutionStrategy>,

    /// Conflict resolution preferences by scope
    pub scope_preferences: HashMap<String, ConflictResolutionStrategy>,
}

/// Conflict resolution strategy types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Latest compatible version resolution
    LatestCompatible,
    /// Pinned version enforcement
    PinnedVersion,
    /// Manual conflict resolution workflows
    ManualResolution,
    /// Automatic conflict detection and reporting
    AutomaticDetection,
    /// Conflict resolution history tracking
    HistoryTracking,
    /// Smart version selection based on compatibility scores
    SmartSelection,
    /// Conservative version selection (prefer stable, tested versions)
    Conservative,
    /// Aggressive version selection (prefer latest features)
    Aggressive,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Lock file update policies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoliciesConfig {
    /// Whether to enable automatic updates
    pub auto_update_enabled: bool,

    /// Update frequency
    pub update_frequency: UpdateFrequency,

    /// Whether to update on dependency changes
    pub update_on_dependency_changes: bool,

    /// Whether to update on scope changes
    pub update_on_scope_changes: bool,

    /// Whether to update on version constraint changes
    pub update_on_constraint_changes: bool,

    /// Whether to preserve pinned versions during updates
    pub preserve_pinned_versions: bool,

    /// Whether to allow breaking changes during updates
    pub allow_breaking_changes: bool,

    /// Whether to require manual approval for major version updates
    pub require_approval_major_updates: bool,

    /// Whether to require manual approval for minor version updates
    pub require_approval_minor_updates: bool,

    /// Whether to require manual approval for patch version updates
    pub require_approval_patch_updates: bool,

    /// Update notification preferences
    pub notifications: UpdateNotificationConfig,

    /// Update rollback configuration
    pub rollback: UpdateRollbackConfig,

    /// Update scheduling configuration
    pub scheduling: UpdateSchedulingConfig,
}

/// Update frequency types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateFrequency {
    /// Never update automatically
    Never,
    /// Update daily
    Daily,
    /// Update weekly
    Weekly,
    /// Update monthly
    Monthly,
    /// Update on demand only
    OnDemand,
    /// Update on specific schedule
    Scheduled(String),
}

/// Update notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotificationConfig {
    /// Whether to notify on available updates
    pub notify_on_available: bool,

    /// Whether to notify on successful updates
    pub notify_on_success: bool,

    /// Whether to notify on failed updates
    pub notify_on_failure: bool,

    /// Whether to notify on breaking changes
    pub notify_on_breaking_changes: bool,

    /// Notification channels
    pub channels: Vec<NotificationChannel>,

    /// Notification recipients
    pub recipients: Vec<String>,
}

/// Notification channel types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notifications
    Email,
    /// Slack notifications
    Slack,
    /// Discord notifications
    Discord,
    /// Webhook notifications
    Webhook(String),
    /// Console notifications
    Console,
    /// Log file notifications
    LogFile,
}

/// Update rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRollbackConfig {
    /// Whether to enable automatic rollback
    pub auto_rollback_enabled: bool,

    /// Whether to rollback on validation failures
    pub rollback_on_validation_failure: bool,

    /// Whether to rollback on test failures
    pub rollback_on_test_failure: bool,

    /// Whether to rollback on build failures
    pub rollback_on_build_failure: bool,

    /// Maximum rollback attempts
    pub max_rollback_attempts: u32,

    /// Rollback timeout (in seconds)
    pub rollback_timeout_seconds: u64,

    /// Whether to preserve rollback history
    pub preserve_rollback_history: bool,

    /// Maximum rollback history entries
    pub max_rollback_history: u32,
}

/// Update scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSchedulingConfig {
    /// Whether to enable scheduled updates
    pub scheduled_updates_enabled: bool,

    /// Update schedule (cron expression)
    pub schedule: Option<String>,

    /// Timezone for scheduled updates
    pub timezone: Option<String>,

    /// Whether to run updates during business hours only
    pub business_hours_only: bool,

    /// Business hours start (HH:MM)
    pub business_hours_start: Option<String>,

    /// Business hours end (HH:MM)
    pub business_hours_end: Option<String>,

    /// Whether to skip updates on weekends
    pub skip_weekends: bool,

    /// Whether to skip updates on holidays
    pub skip_holidays: bool,

    /// Holiday calendar file path
    pub holiday_calendar_path: Option<PathBuf>,
}

/// Validation rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Whether to enable validation
    pub validation_enabled: bool,

    /// Validation level
    pub validation_level: ValidationLevel,

    /// Whether to validate on lock file generation
    pub validate_on_generation: bool,

    /// Whether to validate on lock file loading
    pub validate_on_loading: bool,

    /// Whether to validate on dependency updates
    pub validate_on_updates: bool,

    /// Whether to validate on conflict resolution
    pub validate_on_conflict_resolution: bool,

    /// Validation rules
    pub rules: ValidationRulesConfig,

    /// Validation timeout (in seconds)
    pub timeout_seconds: u64,

    /// Whether to fail fast on validation errors
    pub fail_fast: bool,

    /// Whether to generate validation reports
    pub generate_reports: bool,

    /// Validation report output path
    pub report_output_path: Option<PathBuf>,

    /// Whether to include warnings in validation
    pub include_warnings: bool,

    /// Whether to include suggestions in validation
    pub include_suggestions: bool,

    /// Custom validation rules
    pub custom_rules: HashMap<String, CustomValidationRule>,
}

/// Validation level types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationLevel {
    /// Minimal validation
    Minimal,
    /// Standard validation
    Standard,
    /// Strict validation
    Strict,
    /// Custom validation level
    Custom(String),
}

/// Validation rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRulesConfig {
    /// Whether to validate version constraints
    pub validate_version_constraints: bool,

    /// Whether to validate dependency types
    pub validate_dependency_types: bool,

    /// Whether to validate scope references
    pub validate_scope_references: bool,

    /// Whether to validate circular dependencies
    pub validate_circular_dependencies: bool,

    /// Whether to validate dependency depth
    pub validate_dependency_depth: bool,

    /// Whether to validate checksums
    pub validate_checksums: bool,

    /// Whether to validate timestamps
    pub validate_timestamps: bool,

    /// Whether to validate metadata
    pub validate_metadata: bool,

    /// Whether to validate performance metrics
    pub validate_performance_metrics: bool,

    /// Whether to validate security vulnerabilities
    pub validate_security: bool,

    /// Whether to validate license compatibility
    pub validate_licenses: bool,

    /// Whether to validate architecture compatibility
    pub validate_architecture: bool,

    /// Whether to validate platform compatibility
    pub validate_platform: bool,

    /// Maximum allowed dependency depth
    pub max_dependency_depth: u32,

    /// Maximum allowed circular dependencies
    pub max_circular_dependencies: u32,

    /// Maximum allowed validation time (in seconds)
    pub max_validation_time: u64,
}

/// Custom validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidationRule {
    /// Rule name
    pub name: String,

    /// Rule description
    pub description: Option<String>,

    /// Rule severity
    pub severity: ValidationSeverity,

    /// Whether the rule is enabled
    pub enabled: bool,

    /// Rule expression or pattern
    pub expression: String,

    /// Rule parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Error message template
    pub error_message: Option<String>,

    /// Whether to fail on rule violation
    pub fail_on_violation: bool,
}

/// Validation severity types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Information level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

/// Performance tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Cache configuration
    pub cache: CacheConfig,

    /// Parallel processing configuration
    pub parallel: ParallelConfig,

    /// Memory management configuration
    pub memory: MemoryConfig,

    /// Network configuration
    pub network: NetworkConfig,

    /// Optimization settings
    pub optimization: OptimizationConfig,

    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether to enable caching
    pub enabled: bool,

    /// Cache type
    pub cache_type: CacheType,

    /// Cache size limit (in MB)
    pub size_limit_mb: u64,

    /// Cache TTL (time to live) in seconds
    pub ttl_seconds: u64,

    /// Cache directory
    pub directory: Option<PathBuf>,

    /// Whether to enable cache compression
    pub compression_enabled: bool,

    /// Whether to enable cache encryption
    pub encryption_enabled: bool,

    /// Cache key prefix
    pub key_prefix: Option<String>,

    /// Whether to enable cache statistics
    pub statistics_enabled: bool,

    /// Cache eviction policy
    pub eviction_policy: CacheEvictionPolicy,
}

/// Cache type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheType {
    /// In-memory cache
    Memory,
    /// File-based cache
    File,
    /// Redis cache
    Redis,
    /// Custom cache implementation
    Custom(String),
}

/// Cache eviction policy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
    /// Least recently used
    LRU,
    /// Least frequently used
    LFU,
    /// First in, first out
    FIFO,
    /// Time-based eviction
    TimeBased,
    /// Size-based eviction
    SizeBased,
}

/// Parallel processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    /// Whether to enable parallel processing
    pub enabled: bool,

    /// Maximum parallel threads
    pub max_threads: u32,

    /// Thread pool size
    pub thread_pool_size: u32,

    /// Whether to enable async processing
    pub async_enabled: bool,

    /// Async runtime threads
    pub async_runtime_threads: u32,

    /// Whether to enable work stealing
    pub work_stealing_enabled: bool,

    /// Task queue size
    pub task_queue_size: u32,

    /// Whether to enable load balancing
    pub load_balancing_enabled: bool,
}

/// Memory management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum memory usage (in MB)
    pub max_memory_mb: u64,

    /// Memory limit (in MB)
    pub memory_limit_mb: u64,

    /// Whether to enable garbage collection
    pub gc_enabled: bool,

    /// GC interval (in seconds)
    pub gc_interval_seconds: u64,

    /// Whether to enable memory monitoring
    pub monitoring_enabled: bool,

    /// Memory warning threshold (percentage)
    pub warning_threshold_percent: u8,

    /// Memory critical threshold (percentage)
    pub critical_threshold_percent: u8,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Connection timeout (in seconds)
    pub connection_timeout_seconds: u64,

    /// Request timeout (in seconds)
    pub request_timeout_seconds: u64,

    /// Maximum connections
    pub max_connections: u32,

    /// Whether to enable keep-alive
    pub keep_alive_enabled: bool,

    /// Keep-alive timeout (in seconds)
    pub keep_alive_timeout_seconds: u64,

    /// Whether to enable connection pooling
    pub connection_pooling_enabled: bool,

    /// Connection pool size
    pub connection_pool_size: u32,

    /// Whether to enable retry logic
    pub retry_enabled: bool,

    /// Maximum retry attempts
    pub max_retry_attempts: u32,

    /// Retry delay (in seconds)
    pub retry_delay_seconds: u64,
}

/// Optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Whether to enable dependency graph optimization
    pub graph_optimization_enabled: bool,

    /// Whether to enable resolution path optimization
    pub path_optimization_enabled: bool,

    /// Whether to enable constraint optimization
    pub constraint_optimization_enabled: bool,

    /// Whether to enable version selection optimization
    pub version_selection_optimization_enabled: bool,

    /// Whether to enable conflict resolution optimization
    pub conflict_resolution_optimization_enabled: bool,

    /// Whether to enable validation optimization
    pub validation_optimization_enabled: bool,

    /// Whether to enable caching optimization
    pub caching_optimization_enabled: bool,

    /// Whether to enable parallel processing optimization
    pub parallel_optimization_enabled: bool,

    /// Optimization level
    pub optimization_level: OptimizationLevel,

    /// Whether to enable profile-guided optimization
    pub profile_guided_optimization_enabled: bool,

    /// Profile data path
    pub profile_data_path: Option<PathBuf>,
}

/// Optimization level
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization
    None,
    /// Basic optimization
    Basic,
    /// Standard optimization
    Standard,
    /// Aggressive optimization
    Aggressive,
    /// Maximum optimization
    Maximum,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether to enable performance monitoring
    pub performance_monitoring_enabled: bool,

    /// Whether to enable memory monitoring
    pub memory_monitoring_enabled: bool,

    /// Whether to enable network monitoring
    pub network_monitoring_enabled: bool,

    /// Whether to enable cache monitoring
    pub cache_monitoring_enabled: bool,

    /// Whether to enable error monitoring
    pub error_monitoring_enabled: bool,

    /// Monitoring interval (in seconds)
    pub monitoring_interval_seconds: u64,

    /// Whether to enable metrics collection
    pub metrics_collection_enabled: bool,

    /// Metrics export format
    pub metrics_format: MetricsFormat,

    /// Metrics export path
    pub metrics_export_path: Option<PathBuf>,

    /// Whether to enable alerting
    pub alerting_enabled: bool,

    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Metrics format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricsFormat {
    /// JSON format
    JSON,
    /// Prometheus format
    Prometheus,
    /// Graphite format
    Graphite,
    /// Custom format
    Custom(String),
}

/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Memory usage threshold (percentage)
    pub memory_usage_threshold_percent: u8,

    /// CPU usage threshold (percentage)
    pub cpu_usage_threshold_percent: u8,

    /// Response time threshold (in milliseconds)
    pub response_time_threshold_ms: u64,

    /// Error rate threshold (percentage)
    pub error_rate_threshold_percent: u8,

    /// Cache hit rate threshold (percentage)
    pub cache_hit_rate_threshold_percent: u8,
}

/// Environment-specific lock configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentLockConfig {
    /// Environment name
    pub name: String,

    /// Environment description
    pub description: Option<String>,

    /// Resolution strategy override
    pub resolution_strategy: Option<ResolutionStrategy>,

    /// Conflict resolution strategy override
    pub conflict_resolution_strategy: Option<ConflictResolutionStrategy>,

    /// Update policy overrides
    pub update_policies: Option<UpdatePoliciesConfig>,

    /// Validation rule overrides
    pub validation_rules: Option<ValidationRulesConfig>,

    /// Performance tuning overrides
    pub performance_tuning: Option<PerformanceConfig>,

    /// Environment-specific settings
    pub settings: HashMap<String, serde_json::Value>,
} 

impl Config for LockConfig {
    fn version(&self) -> &str {
        &self.version
    }

    fn validate_config(&self) -> RhemaResult<()> {
        // Basic validation
        if self.version.is_empty() {
            return Err(RhemaError::ConfigError("Version cannot be empty".to_string()));
        }
        
        if self.conflict_resolution.compatibility_threshold < 0.0 || self.conflict_resolution.compatibility_threshold > 1.0 {
            return Err(RhemaError::ConfigError("Compatibility threshold must be between 0.0 and 1.0".to_string()));
        }
        
        if self.resolution.max_depth == 0 {
            return Err(RhemaError::ConfigError("Max depth must be greater than 0".to_string()));
        }
        
        Ok(())
    }

    fn load_from_file(path: &Path) -> RhemaResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to read lock config file: {}", e)))?;
        
        let config: LockConfig = serde_yaml::from_str(&content)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to parse lock config YAML: {}", e)))?;
        
        config.validate_config()?;
        Ok(config)
    }

    fn save_to_file(&self, path: &Path) -> RhemaResult<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to serialize lock config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to write lock config file: {}", e)))?;
        
        Ok(())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "version": { "type": "string" },
                "resolution": { "type": "object" },
                "conflict_resolution": { "type": "object" },
                "update_policies": { "type": "object" },
                "validation": { "type": "object" },
                "performance": { "type": "object" },
                "environments": { "type": "object" },
                "custom": { "type": "object" }
            },
            "required": ["version", "resolution", "conflict_resolution", "update_policies", "validation", "performance"]
        })
    }

    fn documentation() -> &'static str {
        "Lock file configuration for Rhema dependency management system"
    }
}

impl LockConfig {
    /// Create a new lock configuration with default values
    pub fn new() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            resolution: ResolutionConfig::default(),
            conflict_resolution: ConflictResolutionConfig::default(),
            update_policies: UpdatePoliciesConfig::default(),
            validation: ValidationConfig::default(),
            performance: PerformanceConfig::default(),
            environments: HashMap::new(),
            custom: HashMap::new(),
            audit_log: ConfigAuditLog::new(),
            health: ConfigHealth::default(),
            stats: ConfigStats::new(),
            updated_at: Utc::now(),
        }
    }

    /// Load lock configuration from the default location
    pub fn load() -> RhemaResult<Self> {
        let config_path = Self::get_config_path()?;
        Self::load_from_file(&config_path)
    }

    /// Save lock configuration to the default location
    pub fn save(&self) -> RhemaResult<()> {
        let config_path = Self::get_config_path()?;
        self.save_to_file(&config_path)
    }

    /// Get the default configuration path
    fn get_config_path() -> RhemaResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| RhemaError::ConfigError("Could not determine config directory".to_string()))?
            .join("rhema");
        
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        
        Ok(config_dir.join("lock.yaml"))
    }

    /// Update the configuration timestamp
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get a configuration value by path
    pub fn get_value(&self, path: &str) -> Option<serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let value = serde_json::to_value(self).ok()?;
        let mut current = &value;
        
        for part in parts {
            current = current.get(part)?;
        }
        
        Some(current.clone())
    }

    /// Set a configuration value by path
    pub fn set_value(&mut self, path: &str, value: serde_json::Value) -> RhemaResult<()> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty configuration path".to_string()));
        }

        match parts[0] {
            "resolution" => self.set_resolution_value(&parts[1..], value)?,
            "conflict_resolution" => self.set_conflict_resolution_value(&parts[1..], value)?,
            "update_policies" => self.set_update_policies_value(&parts[1..], value)?,
            "validation" => self.set_validation_value(&parts[1..], value)?,
            "performance" => self.set_performance_value(&parts[1..], value)?,
            "environments" => self.set_environments_value(&parts[1..], value)?,
            "custom" => self.set_custom_value(&parts[1..], value)?,
            _ => return Err(RhemaError::ConfigError(format!("Unknown configuration section: {}", parts[0]))),
        }

        self.update()?;
        Ok(())
    }

    /// Get environment-specific configuration
    pub fn get_environment_config(&self, environment: &ConfigEnvironment) -> Option<&EnvironmentLockConfig> {
        self.environments.get(environment)
    }

    /// Set environment-specific configuration
    pub fn set_environment_config(&mut self, environment: ConfigEnvironment, config: EnvironmentLockConfig) {
        self.environments.insert(environment, config);
        self.update().ok();
    }

    /// Merge configuration from another source
    pub fn merge(&mut self, other: &LockConfig) -> RhemaResult<()> {
        // Merge resolution config
        self.resolution.merge(&other.resolution);
        
        // Merge conflict resolution config
        self.conflict_resolution.merge(&other.conflict_resolution);
        
        // Merge update policies
        self.update_policies.merge(&other.update_policies);
        
        // Merge validation config
        self.validation.merge(&other.validation);
        
        // Merge performance config
        self.performance.merge(&other.performance);
        
        // Merge environments
        for (env, config) in &other.environments {
            self.environments.insert(env.clone(), config.clone());
        }
        
        // Merge custom settings
        for (key, value) in &other.custom {
            self.custom.insert(key.clone(), value.clone());
        }
        
        self.update()?;
        Ok(())
    }

    /// Export configuration to different formats
    pub fn export(&self, format: &str) -> RhemaResult<String> {
        match format.to_lowercase().as_str() {
            "yaml" | "yml" => {
                serde_yaml::to_string(self)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to export to YAML: {}", e)))
            }
            "json" => {
                serde_json::to_string_pretty(self)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to export to JSON: {}", e)))
            }
            "toml" => {
                toml::to_string_pretty(self)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to export to TOML: {}", e)))
            }
            _ => Err(RhemaError::ConfigError(format!("Unsupported export format: {}", format))),
        }
    }

    /// Import configuration from different formats
    pub fn import(&mut self, content: &str, format: &str) -> RhemaResult<()> {
        let imported_config: LockConfig = match format.to_lowercase().as_str() {
            "yaml" | "yml" => {
                serde_yaml::from_str(content)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to parse YAML: {}", e)))?
            }
            "json" => {
                serde_json::from_str(content)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to parse JSON: {}", e)))?
            }
            "toml" => {
                toml::from_str(content)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to parse TOML: {}", e)))?
            }
            _ => return Err(RhemaError::ConfigError(format!("Unsupported import format: {}", format))),
        };

        self.merge(&imported_config)?;
        Ok(())
    }

    // Helper methods for setting nested values
    fn set_resolution_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty resolution path".to_string()));
        }

        match parts[0] {
            "default_strategy" => {
                let strategy: ResolutionStrategy = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid resolution strategy: {}", e)))?;
                self.resolution.default_strategy = strategy;
            }
            "smart_resolution_enabled" => {
                let enabled: bool = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                self.resolution.smart_resolution_enabled = enabled;
            }
            "prefer_stable" => {
                let prefer: bool = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                self.resolution.prefer_stable = prefer;
            }
            _ => return Err(RhemaError::ConfigError(format!("Unknown resolution setting: {}", parts[0]))),
        }

        Ok(())
    }

    fn set_conflict_resolution_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty conflict resolution path".to_string()));
        }

        match parts[0] {
            "primary_strategy" => {
                let strategy: ConflictResolutionStrategy = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid conflict resolution strategy: {}", e)))?;
                self.conflict_resolution.primary_strategy = strategy;
            }
            "enable_auto_detection" => {
                let enabled: bool = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                self.conflict_resolution.enable_auto_detection = enabled;
            }
            "compatibility_threshold" => {
                let threshold: f64 = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid threshold value: {}", e)))?;
                if !(0.0..=1.0).contains(&threshold) {
                    return Err(RhemaError::ConfigError("Compatibility threshold must be between 0.0 and 1.0".to_string()));
                }
                self.conflict_resolution.compatibility_threshold = threshold;
            }
            _ => return Err(RhemaError::ConfigError(format!("Unknown conflict resolution setting: {}", parts[0]))),
        }

        Ok(())
    }

    fn set_update_policies_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty update policies path".to_string()));
        }

        match parts[0] {
            "auto_update_enabled" => {
                let enabled: bool = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                self.update_policies.auto_update_enabled = enabled;
            }
            "update_frequency" => {
                let frequency: UpdateFrequency = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid update frequency: {}", e)))?;
                self.update_policies.update_frequency = frequency;
            }
            _ => return Err(RhemaError::ConfigError(format!("Unknown update policy setting: {}", parts[0]))),
        }

        Ok(())
    }

    fn set_validation_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty validation path".to_string()));
        }

        match parts[0] {
            "validation_enabled" => {
                let enabled: bool = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                self.validation.validation_enabled = enabled;
            }
            "validation_level" => {
                let level: ValidationLevel = serde_json::from_value(value)
                    .map_err(|e| RhemaError::ConfigError(format!("Invalid validation level: {}", e)))?;
                self.validation.validation_level = level;
            }
            _ => return Err(RhemaError::ConfigError(format!("Unknown validation setting: {}", parts[0]))),
        }

        Ok(())
    }

    fn set_performance_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty performance path".to_string()));
        }

        match parts[0] {
            "cache" => {
                // Handle cache configuration
                if parts.len() > 1 {
                    match parts[1] {
                        "enabled" => {
                            let enabled: bool = serde_json::from_value(value)
                                .map_err(|e| RhemaError::ConfigError(format!("Invalid boolean value: {}", e)))?;
                            self.performance.cache.enabled = enabled;
                        }
                        "size_limit_mb" => {
                            let size: u64 = serde_json::from_value(value)
                                .map_err(|e| RhemaError::ConfigError(format!("Invalid size value: {}", e)))?;
                            self.performance.cache.size_limit_mb = size;
                        }
                        _ => return Err(RhemaError::ConfigError(format!("Unknown cache setting: {}", parts[1]))),
                    }
                }
            }
            _ => return Err(RhemaError::ConfigError(format!("Unknown performance setting: {}", parts[0]))),
        }

        Ok(())
    }

    fn set_environments_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.len() < 2 {
            return Err(RhemaError::ConfigError("Environment path must include environment name".to_string()));
        }

        let env_name = parts[0];
        let env: ConfigEnvironment = match env_name {
            "development" => ConfigEnvironment::Development,
            "testing" => ConfigEnvironment::Testing,
            "staging" => ConfigEnvironment::Staging,
            "production" => ConfigEnvironment::Production,
            _ => ConfigEnvironment::Custom(env_name.to_string()),
        };

        let env_config: EnvironmentLockConfig = serde_json::from_value(value)
            .map_err(|e| RhemaError::ConfigError(format!("Invalid environment config: {}", e)))?;

        self.environments.insert(env, env_config);
        Ok(())
    }

    fn set_custom_value(&mut self, parts: &[&str], value: serde_json::Value) -> RhemaResult<()> {
        if parts.is_empty() {
            return Err(RhemaError::ConfigError("Empty custom path".to_string()));
        }

        let key = parts[0];
        self.custom.insert(key.to_string(), value);
        Ok(())
    }
}

// Default implementations
impl Default for LockConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            default_strategy: ResolutionStrategy::Latest,
            strategy_overrides: HashMap::new(),
            scope_strategies: HashMap::new(),
            smart_resolution_enabled: true,
            prefer_stable: true,
            allow_prereleases: false,
            allow_development: false,
            max_depth: 10,
            resolve_transitive: true,
            pin_by_default: false,
            version_constraints: VersionConstraintConfig::default(),
        }
    }
}

impl Default for VersionConstraintConfig {
    fn default() -> Self {
        Self {
            default_type: ConstraintType::Caret,
            use_caret_ranges: true,
            use_tilde_ranges: false,
            allow_wildcards: false,
            allow_exact: true,
            allow_ranges: true,
            min_version: None,
            max_version: None,
        }
    }
}

impl Default for ConflictResolutionConfig {
    fn default() -> Self {
        Self {
            primary_strategy: ConflictResolutionStrategy::LatestCompatible,
            fallback_strategies: vec![
                ConflictResolutionStrategy::SmartSelection,
                ConflictResolutionStrategy::Conservative,
            ],
            enable_auto_detection: true,
            track_history: true,
            max_attempts: 3,
            allow_user_prompts: true,
            prefer_stable: true,
            strict_pinning: false,
            compatibility_threshold: 0.8,
            parallel_resolution: true,
            max_parallel_threads: 4,
            timeout_seconds: 300,
            auto_resolve_low_severity: true,
            auto_resolve_medium_severity: false,
            fail_on_high_severity: false,
            fail_on_critical_severity: true,
            dependency_type_preferences: HashMap::new(),
            scope_preferences: HashMap::new(),
        }
    }
}

impl Default for UpdatePoliciesConfig {
    fn default() -> Self {
        Self {
            auto_update_enabled: false,
            update_frequency: UpdateFrequency::OnDemand,
            update_on_dependency_changes: true,
            update_on_scope_changes: true,
            update_on_constraint_changes: true,
            preserve_pinned_versions: true,
            allow_breaking_changes: false,
            require_approval_major_updates: true,
            require_approval_minor_updates: false,
            require_approval_patch_updates: false,
            notifications: UpdateNotificationConfig::default(),
            rollback: UpdateRollbackConfig::default(),
            scheduling: UpdateSchedulingConfig::default(),
        }
    }
}

impl Default for UpdateNotificationConfig {
    fn default() -> Self {
        Self {
            notify_on_available: true,
            notify_on_success: false,
            notify_on_failure: true,
            notify_on_breaking_changes: true,
            channels: vec![NotificationChannel::Console],
            recipients: Vec::new(),
        }
    }
}

impl Default for UpdateRollbackConfig {
    fn default() -> Self {
        Self {
            auto_rollback_enabled: true,
            rollback_on_validation_failure: true,
            rollback_on_test_failure: true,
            rollback_on_build_failure: true,
            max_rollback_attempts: 3,
            rollback_timeout_seconds: 300,
            preserve_rollback_history: true,
            max_rollback_history: 10,
        }
    }
}

impl Default for UpdateSchedulingConfig {
    fn default() -> Self {
        Self {
            scheduled_updates_enabled: false,
            schedule: None,
            timezone: None,
            business_hours_only: false,
            business_hours_start: None,
            business_hours_end: None,
            skip_weekends: false,
            skip_holidays: false,
            holiday_calendar_path: None,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            validation_enabled: true,
            validation_level: ValidationLevel::Standard,
            validate_on_generation: true,
            validate_on_loading: true,
            validate_on_updates: true,
            validate_on_conflict_resolution: true,
            rules: ValidationRulesConfig::default(),
            timeout_seconds: 60,
            fail_fast: false,
            generate_reports: false,
            report_output_path: None,
            include_warnings: true,
            include_suggestions: true,
            custom_rules: HashMap::new(),
        }
    }
}

impl Default for ValidationRulesConfig {
    fn default() -> Self {
        Self {
            validate_version_constraints: true,
            validate_dependency_types: true,
            validate_scope_references: true,
            validate_circular_dependencies: true,
            validate_dependency_depth: true,
            validate_checksums: true,
            validate_timestamps: true,
            validate_metadata: true,
            validate_performance_metrics: false,
            validate_security: false,
            validate_licenses: false,
            validate_architecture: false,
            validate_platform: false,
            max_dependency_depth: 10,
            max_circular_dependencies: 0,
            max_validation_time: 30,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            parallel: ParallelConfig::default(),
            memory: MemoryConfig::default(),
            network: NetworkConfig::default(),
            optimization: OptimizationConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: CacheType::Memory,
            size_limit_mb: 100,
            ttl_seconds: 3600,
            directory: None,
            compression_enabled: false,
            encryption_enabled: false,
            key_prefix: Some("rhema_lock".to_string()),
            statistics_enabled: true,
            eviction_policy: CacheEvictionPolicy::LRU,
        }
    }
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_threads: 4,
            thread_pool_size: 8,
            async_enabled: true,
            async_runtime_threads: 2,
            work_stealing_enabled: true,
            task_queue_size: 1000,
            load_balancing_enabled: true,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            memory_limit_mb: 1024,
            gc_enabled: true,
            gc_interval_seconds: 300,
            monitoring_enabled: true,
            warning_threshold_percent: 80,
            critical_threshold_percent: 95,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connection_timeout_seconds: 30,
            request_timeout_seconds: 60,
            max_connections: 100,
            keep_alive_enabled: true,
            keep_alive_timeout_seconds: 60,
            connection_pooling_enabled: true,
            connection_pool_size: 10,
            retry_enabled: true,
            max_retry_attempts: 3,
            retry_delay_seconds: 5,
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            graph_optimization_enabled: true,
            path_optimization_enabled: true,
            constraint_optimization_enabled: true,
            version_selection_optimization_enabled: true,
            conflict_resolution_optimization_enabled: true,
            validation_optimization_enabled: true,
            caching_optimization_enabled: true,
            parallel_optimization_enabled: true,
            optimization_level: OptimizationLevel::Standard,
            profile_guided_optimization_enabled: false,
            profile_data_path: None,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            performance_monitoring_enabled: true,
            memory_monitoring_enabled: true,
            network_monitoring_enabled: false,
            cache_monitoring_enabled: true,
            error_monitoring_enabled: true,
            monitoring_interval_seconds: 60,
            metrics_collection_enabled: true,
            metrics_format: MetricsFormat::JSON,
            metrics_export_path: None,
            alerting_enabled: false,
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            memory_usage_threshold_percent: 80,
            cpu_usage_threshold_percent: 80,
            response_time_threshold_ms: 5000,
            error_rate_threshold_percent: 5,
            cache_hit_rate_threshold_percent: 80,
        }
    }
}

// Merge implementations for nested configurations
impl ResolutionConfig {
    pub fn merge(&mut self, other: &ResolutionConfig) {
        if other.default_strategy != ResolutionStrategy::Latest {
            self.default_strategy = other.default_strategy.clone();
        }
        self.strategy_overrides.extend(other.strategy_overrides.clone());
        self.scope_strategies.extend(other.scope_strategies.clone());
        self.smart_resolution_enabled = other.smart_resolution_enabled;
        self.prefer_stable = other.prefer_stable;
        self.allow_prereleases = other.allow_prereleases;
        self.allow_development = other.allow_development;
        self.max_depth = other.max_depth;
        self.resolve_transitive = other.resolve_transitive;
        self.pin_by_default = other.pin_by_default;
        self.version_constraints.merge(&other.version_constraints);
    }
}

impl VersionConstraintConfig {
    pub fn merge(&mut self, other: &VersionConstraintConfig) {
        self.default_type = other.default_type.clone();
        self.use_caret_ranges = other.use_caret_ranges;
        self.use_tilde_ranges = other.use_tilde_ranges;
        self.allow_wildcards = other.allow_wildcards;
        self.allow_exact = other.allow_exact;
        self.allow_ranges = other.allow_ranges;
        self.min_version = other.min_version.clone();
        self.max_version = other.max_version.clone();
    }
}

impl ConflictResolutionConfig {
    pub fn merge(&mut self, other: &ConflictResolutionConfig) {
        self.primary_strategy = other.primary_strategy.clone();
        self.fallback_strategies = other.fallback_strategies.clone();
        self.enable_auto_detection = other.enable_auto_detection;
        self.track_history = other.track_history;
        self.max_attempts = other.max_attempts;
        self.allow_user_prompts = other.allow_user_prompts;
        self.prefer_stable = other.prefer_stable;
        self.strict_pinning = other.strict_pinning;
        self.compatibility_threshold = other.compatibility_threshold;
        self.parallel_resolution = other.parallel_resolution;
        self.max_parallel_threads = other.max_parallel_threads;
        self.timeout_seconds = other.timeout_seconds;
        self.auto_resolve_low_severity = other.auto_resolve_low_severity;
        self.auto_resolve_medium_severity = other.auto_resolve_medium_severity;
        self.fail_on_high_severity = other.fail_on_high_severity;
        self.fail_on_critical_severity = other.fail_on_critical_severity;
        self.dependency_type_preferences.extend(other.dependency_type_preferences.clone());
        self.scope_preferences.extend(other.scope_preferences.clone());
    }
}

impl UpdatePoliciesConfig {
    pub fn merge(&mut self, other: &UpdatePoliciesConfig) {
        self.auto_update_enabled = other.auto_update_enabled;
        self.update_frequency = other.update_frequency.clone();
        self.update_on_dependency_changes = other.update_on_dependency_changes;
        self.update_on_scope_changes = other.update_on_scope_changes;
        self.update_on_constraint_changes = other.update_on_constraint_changes;
        self.preserve_pinned_versions = other.preserve_pinned_versions;
        self.allow_breaking_changes = other.allow_breaking_changes;
        self.require_approval_major_updates = other.require_approval_major_updates;
        self.require_approval_minor_updates = other.require_approval_minor_updates;
        self.require_approval_patch_updates = other.require_approval_patch_updates;
        self.notifications.merge(&other.notifications);
        self.rollback.merge(&other.rollback);
        self.scheduling.merge(&other.scheduling);
    }
}

impl UpdateNotificationConfig {
    pub fn merge(&mut self, other: &UpdateNotificationConfig) {
        self.notify_on_available = other.notify_on_available;
        self.notify_on_success = other.notify_on_success;
        self.notify_on_failure = other.notify_on_failure;
        self.notify_on_breaking_changes = other.notify_on_breaking_changes;
        self.channels = other.channels.clone();
        self.recipients = other.recipients.clone();
    }
}

impl UpdateRollbackConfig {
    pub fn merge(&mut self, other: &UpdateRollbackConfig) {
        self.auto_rollback_enabled = other.auto_rollback_enabled;
        self.rollback_on_validation_failure = other.rollback_on_validation_failure;
        self.rollback_on_test_failure = other.rollback_on_test_failure;
        self.rollback_on_build_failure = other.rollback_on_build_failure;
        self.max_rollback_attempts = other.max_rollback_attempts;
        self.rollback_timeout_seconds = other.rollback_timeout_seconds;
        self.preserve_rollback_history = other.preserve_rollback_history;
        self.max_rollback_history = other.max_rollback_history;
    }
}

impl UpdateSchedulingConfig {
    pub fn merge(&mut self, other: &UpdateSchedulingConfig) {
        self.scheduled_updates_enabled = other.scheduled_updates_enabled;
        self.schedule = other.schedule.clone();
        self.timezone = other.timezone.clone();
        self.business_hours_only = other.business_hours_only;
        self.business_hours_start = other.business_hours_start.clone();
        self.business_hours_end = other.business_hours_end.clone();
        self.skip_weekends = other.skip_weekends;
        self.skip_holidays = other.skip_holidays;
        self.holiday_calendar_path = other.holiday_calendar_path.clone();
    }
}

impl ValidationConfig {
    pub fn merge(&mut self, other: &ValidationConfig) {
        self.validation_enabled = other.validation_enabled;
        self.validation_level = other.validation_level.clone();
        self.validate_on_generation = other.validate_on_generation;
        self.validate_on_loading = other.validate_on_loading;
        self.validate_on_updates = other.validate_on_updates;
        self.validate_on_conflict_resolution = other.validate_on_conflict_resolution;
        self.rules.merge(&other.rules);
        self.timeout_seconds = other.timeout_seconds;
        self.fail_fast = other.fail_fast;
        self.generate_reports = other.generate_reports;
        self.report_output_path = other.report_output_path.clone();
        self.include_warnings = other.include_warnings;
        self.include_suggestions = other.include_suggestions;
        self.custom_rules.extend(other.custom_rules.clone());
    }
}

impl ValidationRulesConfig {
    pub fn merge(&mut self, other: &ValidationRulesConfig) {
        self.validate_version_constraints = other.validate_version_constraints;
        self.validate_dependency_types = other.validate_dependency_types;
        self.validate_scope_references = other.validate_scope_references;
        self.validate_circular_dependencies = other.validate_circular_dependencies;
        self.validate_dependency_depth = other.validate_dependency_depth;
        self.validate_checksums = other.validate_checksums;
        self.validate_timestamps = other.validate_timestamps;
        self.validate_metadata = other.validate_metadata;
        self.validate_performance_metrics = other.validate_performance_metrics;
        self.validate_security = other.validate_security;
        self.validate_licenses = other.validate_licenses;
        self.validate_architecture = other.validate_architecture;
        self.validate_platform = other.validate_platform;
        self.max_dependency_depth = other.max_dependency_depth;
        self.max_circular_dependencies = other.max_circular_dependencies;
        self.max_validation_time = other.max_validation_time;
    }
}

impl PerformanceConfig {
    pub fn merge(&mut self, other: &PerformanceConfig) {
        self.cache.merge(&other.cache);
        self.parallel.merge(&other.parallel);
        self.memory.merge(&other.memory);
        self.network.merge(&other.network);
        self.optimization.merge(&other.optimization);
        self.monitoring.merge(&other.monitoring);
    }
}

impl CacheConfig {
    pub fn merge(&mut self, other: &CacheConfig) {
        self.enabled = other.enabled;
        self.cache_type = other.cache_type.clone();
        self.size_limit_mb = other.size_limit_mb;
        self.ttl_seconds = other.ttl_seconds;
        self.directory = other.directory.clone();
        self.compression_enabled = other.compression_enabled;
        self.encryption_enabled = other.encryption_enabled;
        self.key_prefix = other.key_prefix.clone();
        self.statistics_enabled = other.statistics_enabled;
        self.eviction_policy = other.eviction_policy.clone();
    }
}

impl ParallelConfig {
    pub fn merge(&mut self, other: &ParallelConfig) {
        self.enabled = other.enabled;
        self.max_threads = other.max_threads;
        self.thread_pool_size = other.thread_pool_size;
        self.async_enabled = other.async_enabled;
        self.async_runtime_threads = other.async_runtime_threads;
        self.work_stealing_enabled = other.work_stealing_enabled;
        self.task_queue_size = other.task_queue_size;
        self.load_balancing_enabled = other.load_balancing_enabled;
    }
}

impl MemoryConfig {
    pub fn merge(&mut self, other: &MemoryConfig) {
        self.max_memory_mb = other.max_memory_mb;
        self.memory_limit_mb = other.memory_limit_mb;
        self.gc_enabled = other.gc_enabled;
        self.gc_interval_seconds = other.gc_interval_seconds;
        self.monitoring_enabled = other.monitoring_enabled;
        self.warning_threshold_percent = other.warning_threshold_percent;
        self.critical_threshold_percent = other.critical_threshold_percent;
    }
}

impl NetworkConfig {
    pub fn merge(&mut self, other: &NetworkConfig) {
        self.connection_timeout_seconds = other.connection_timeout_seconds;
        self.request_timeout_seconds = other.request_timeout_seconds;
        self.max_connections = other.max_connections;
        self.keep_alive_enabled = other.keep_alive_enabled;
        self.keep_alive_timeout_seconds = other.keep_alive_timeout_seconds;
        self.connection_pooling_enabled = other.connection_pooling_enabled;
        self.connection_pool_size = other.connection_pool_size;
        self.retry_enabled = other.retry_enabled;
        self.max_retry_attempts = other.max_retry_attempts;
        self.retry_delay_seconds = other.retry_delay_seconds;
    }
}

impl OptimizationConfig {
    pub fn merge(&mut self, other: &OptimizationConfig) {
        self.graph_optimization_enabled = other.graph_optimization_enabled;
        self.path_optimization_enabled = other.path_optimization_enabled;
        self.constraint_optimization_enabled = other.constraint_optimization_enabled;
        self.version_selection_optimization_enabled = other.version_selection_optimization_enabled;
        self.conflict_resolution_optimization_enabled = other.conflict_resolution_optimization_enabled;
        self.validation_optimization_enabled = other.validation_optimization_enabled;
        self.caching_optimization_enabled = other.caching_optimization_enabled;
        self.parallel_optimization_enabled = other.parallel_optimization_enabled;
        self.optimization_level = other.optimization_level.clone();
        self.profile_guided_optimization_enabled = other.profile_guided_optimization_enabled;
        self.profile_data_path = other.profile_data_path.clone();
    }
}

impl MonitoringConfig {
    pub fn merge(&mut self, other: &MonitoringConfig) {
        self.performance_monitoring_enabled = other.performance_monitoring_enabled;
        self.memory_monitoring_enabled = other.memory_monitoring_enabled;
        self.network_monitoring_enabled = other.network_monitoring_enabled;
        self.cache_monitoring_enabled = other.cache_monitoring_enabled;
        self.error_monitoring_enabled = other.error_monitoring_enabled;
        self.monitoring_interval_seconds = other.monitoring_interval_seconds;
        self.metrics_collection_enabled = other.metrics_collection_enabled;
        self.metrics_format = other.metrics_format.clone();
        self.metrics_export_path = other.metrics_export_path.clone();
        self.alerting_enabled = other.alerting_enabled;
        self.alert_thresholds.merge(&other.alert_thresholds);
    }
}

impl AlertThresholds {
    pub fn merge(&mut self, other: &AlertThresholds) {
        self.memory_usage_threshold_percent = other.memory_usage_threshold_percent;
        self.cpu_usage_threshold_percent = other.cpu_usage_threshold_percent;
        self.response_time_threshold_ms = other.response_time_threshold_ms;
        self.error_rate_threshold_percent = other.error_rate_threshold_percent;
        self.cache_hit_rate_threshold_percent = other.cache_hit_rate_threshold_percent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_config_creation() {
        let config = LockConfig::new();
        assert_eq!(config.version, CURRENT_CONFIG_VERSION);
        assert!(config.resolution.smart_resolution_enabled);
        assert!(config.conflict_resolution.enable_auto_detection);
        assert!(!config.update_policies.auto_update_enabled);
        assert!(config.validation.validation_enabled);
    }

    #[test]
    fn test_lock_config_validation() {
        let mut config = LockConfig::new();
        assert!(config.validate_config().is_ok());

        // Test invalid configuration
        config.conflict_resolution.compatibility_threshold = 1.5; // Invalid value
        assert!(config.validate_config().is_err());
    }

    #[test]
    fn test_lock_config_serialization() {
        let config = LockConfig::new();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: LockConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    fn test_lock_config_merge() {
        let mut config1 = LockConfig::new();
        let mut config2 = LockConfig::new();
        
        config2.resolution.default_strategy = ResolutionStrategy::Conservative;
        config2.conflict_resolution.compatibility_threshold = 0.9;
        
        config1.merge(&config2).unwrap();
        
        assert_eq!(config1.resolution.default_strategy, ResolutionStrategy::Conservative);
        assert_eq!(config1.conflict_resolution.compatibility_threshold, 0.9);
    }

    #[test]
    fn test_environment_config() {
        let mut config = LockConfig::new();
        let env_config = EnvironmentLockConfig {
            name: "test".to_string(),
            description: Some("Test environment".to_string()),
            resolution_strategy: Some(ResolutionStrategy::Conservative),
            conflict_resolution_strategy: None,
            update_policies: None,
            validation_rules: None,
            performance_tuning: None,
            settings: HashMap::new(),
        };
        
        config.set_environment_config(ConfigEnvironment::Testing, env_config);
        let retrieved = config.get_environment_config(&ConfigEnvironment::Testing);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test");
    }

    #[test]
    fn test_config_value_access() {
        let config = LockConfig::new();
        
        // Test getting values
        let smart_resolution = config.get_value("resolution.smart_resolution_enabled");
        assert!(smart_resolution.is_some());
        assert_eq!(smart_resolution.unwrap().as_bool().unwrap(), true);
        
        let version = config.get_value("version");
        assert!(version.is_some());
        assert_eq!(version.unwrap().as_str().unwrap(), CURRENT_CONFIG_VERSION);
    }

    #[test]
    fn test_config_value_setting() {
        let mut config = LockConfig::new();
        
        // Test setting values
        config.set_value("resolution.smart_resolution_enabled", serde_json::json!(false)).unwrap();
        assert!(!config.resolution.smart_resolution_enabled);
        
        config.set_value("conflict_resolution.compatibility_threshold", serde_json::json!(0.95)).unwrap();
        assert_eq!(config.conflict_resolution.compatibility_threshold, 0.95);
    }

    #[test]
    fn test_config_export_import() {
        let config = LockConfig::new();
        
        // Test YAML export/import
        let yaml = config.export("yaml").unwrap();
        let mut new_config = LockConfig::new();
        new_config.import(&yaml, "yaml").unwrap();
        assert_eq!(config.version, new_config.version);
        
        // Test JSON export/import
        let json = config.export("json").unwrap();
        let mut new_config2 = LockConfig::new();
        new_config2.import(&json, "json").unwrap();
        assert_eq!(config.version, new_config2.version);
    }
} 