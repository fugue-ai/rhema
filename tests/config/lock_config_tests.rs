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

use rhema_config::{
    lock::{LockConfig, ValidationLevel as LockValidationLevel}, CacheConfig, CacheEvictionPolicy, CacheType, Config, ConfigEnvironment,
    ConflictResolutionConfig, ConflictResolutionStrategy, ConstraintType, EnvironmentLockConfig, MetricsFormat, MonitoringConfig, NotificationChannel,
    OptimizationConfig, OptimizationLevel, PerformanceConfig, ResolutionConfig,
    ResolutionStrategy, UpdateFrequency, UpdateNotificationConfig, UpdatePoliciesConfig, ValidationConfig,
    ValidationSeverity, VersionConstraintConfig, CURRENT_CONFIG_VERSION,
};
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_lock_config_creation_and_defaults() {
    let config = LockConfig::new();

    // Test basic structure
    assert_eq!(config.version, CURRENT_CONFIG_VERSION);
    assert!(config.resolution.smart_resolution_enabled);
    assert!(config.conflict_resolution.enable_auto_detection);
    assert!(!config.update_policies.auto_update_enabled);
    assert!(config.validation.validation_enabled);
    assert!(config.performance.cache.enabled);

    // Test resolution config defaults
    assert_eq!(
        config.resolution.default_strategy,
        ResolutionStrategy::Latest
    );
    assert!(config.resolution.prefer_stable);
    assert!(!config.resolution.allow_prereleases);
    assert_eq!(config.resolution.max_depth, 10);

    // Test conflict resolution defaults
    assert_eq!(
        config.conflict_resolution.primary_strategy,
        ConflictResolutionStrategy::LatestCompatible
    );
    assert_eq!(config.conflict_resolution.compatibility_threshold, 0.8);
    assert_eq!(config.conflict_resolution.max_attempts, 3);
    assert!(config.conflict_resolution.auto_resolve_low_severity);
    assert!(!config.conflict_resolution.auto_resolve_medium_severity);

    // Test update policies defaults
    assert_eq!(
        config.update_policies.update_frequency,
        UpdateFrequency::OnDemand
    );
    assert!(config.update_policies.preserve_pinned_versions);
    assert!(!config.update_policies.allow_breaking_changes);
    assert!(config.update_policies.require_approval_major_updates);

    // Test validation defaults
    assert_eq!(
        config.validation.validation_level,
        LockValidationLevel::Standard
    );
    assert!(config.validation.validate_on_generation);
    assert!(config.validation.validate_on_loading);
    assert_eq!(config.validation.timeout_seconds, 60);

    // Test performance defaults
    assert_eq!(config.performance.cache.cache_type, CacheType::Memory);
    assert_eq!(config.performance.cache.size_limit_mb, 100);
    assert_eq!(config.performance.parallel.max_threads, 4);
    assert_eq!(config.performance.memory.max_memory_mb, 512);
}

#[test]
fn test_resolution_config() {
    let mut config = ResolutionConfig::default();

    // Test default values
    assert_eq!(config.default_strategy, ResolutionStrategy::Latest);
    assert!(config.smart_resolution_enabled);
    assert!(config.prefer_stable);
    assert!(!config.allow_prereleases);
    assert_eq!(config.max_depth, 10);

    // Test strategy overrides
    config
        .strategy_overrides
        .insert("test-dep".to_string(), ResolutionStrategy::Conservative);
    assert_eq!(
        config.strategy_overrides.get("test-dep"),
        Some(&ResolutionStrategy::Conservative)
    );

    // Test version constraint config
    assert_eq!(
        config.version_constraints.default_type,
        ConstraintType::Caret
    );
    assert!(config.version_constraints.use_caret_ranges);
    assert!(!config.version_constraints.use_tilde_ranges);
    assert!(!config.version_constraints.allow_wildcards);
}

#[test]
fn test_conflict_resolution_config() {
    let mut config = ConflictResolutionConfig::default();

    // Test default values
    assert_eq!(
        config.primary_strategy,
        ConflictResolutionStrategy::LatestCompatible
    );
    assert!(config.enable_auto_detection);
    assert!(config.track_history);
    assert_eq!(config.max_attempts, 3);
    assert_eq!(config.compatibility_threshold, 0.8);
    assert!(config.auto_resolve_low_severity);
    assert!(!config.auto_resolve_medium_severity);
    assert!(!config.fail_on_high_severity);
    assert!(config.fail_on_critical_severity);

    // Test fallback strategies
    assert_eq!(config.fallback_strategies.len(), 2);
    assert!(config
        .fallback_strategies
        .contains(&ConflictResolutionStrategy::SmartSelection));
    assert!(config
        .fallback_strategies
        .contains(&ConflictResolutionStrategy::Conservative));

    // Test dependency type preferences
    config.dependency_type_preferences.insert(
        "production".to_string(),
        ConflictResolutionStrategy::Conservative,
    );
    assert_eq!(
        config.dependency_type_preferences.get("production"),
        Some(&ConflictResolutionStrategy::Conservative)
    );
}

#[test]
fn test_update_policies_config() {
    let config = UpdatePoliciesConfig::default();

    // Test default values
    assert!(!config.auto_update_enabled);
    assert_eq!(config.update_frequency, UpdateFrequency::OnDemand);
    assert!(config.update_on_dependency_changes);
    assert!(config.update_on_scope_changes);
    assert!(config.preserve_pinned_versions);
    assert!(!config.allow_breaking_changes);
    assert!(config.require_approval_major_updates);
    assert!(!config.require_approval_minor_updates);
    assert!(!config.require_approval_patch_updates);

    // Test notification config
    assert!(config.notifications.notify_on_available);
    assert!(!config.notifications.notify_on_success);
    assert!(config.notifications.notify_on_failure);
    assert!(config.notifications.notify_on_breaking_changes);
    assert_eq!(config.notifications.channels.len(), 1);
    assert!(matches!(
        config.notifications.channels[0],
        NotificationChannel::Console
    ));

    // Test rollback config
    assert!(config.rollback.auto_rollback_enabled);
    assert!(config.rollback.rollback_on_validation_failure);
    assert_eq!(config.rollback.max_rollback_attempts, 3);
    assert_eq!(config.rollback.rollback_timeout_seconds, 300);

    // Test scheduling config
    assert!(!config.scheduling.scheduled_updates_enabled);
    assert!(config.scheduling.schedule.is_none());
    assert!(!config.scheduling.business_hours_only);
    assert!(!config.scheduling.skip_weekends);
}

#[test]
fn test_validation_config() {
    let config = ValidationConfig::default();

    // Test default values
    assert!(config.validation_enabled);
    assert_eq!(config.validation_level, LockValidationLevel::Standard);
    assert!(config.validate_on_generation);
    assert!(config.validate_on_loading);
    assert!(config.validate_on_updates);
    assert!(config.validate_on_conflict_resolution);
    assert_eq!(config.timeout_seconds, 60);
    assert!(!config.fail_fast);
    assert!(!config.generate_reports);
    assert!(config.include_warnings);
    assert!(config.include_suggestions);

    // Test validation rules
    assert!(config.rules.validate_version_constraints);
    assert!(config.rules.validate_dependency_types);
    assert!(config.rules.validate_circular_dependencies);
    assert!(config.rules.validate_checksums);
    assert!(!config.rules.validate_security);
    assert!(!config.rules.validate_licenses);
    assert_eq!(config.rules.max_dependency_depth, 10);
    assert_eq!(config.rules.max_circular_dependencies, 0);
    assert_eq!(config.rules.max_validation_time, 30);
}

#[test]
fn test_performance_config() {
    let config = PerformanceConfig::default();

    // Test cache config
    assert!(config.cache.enabled);
    assert_eq!(config.cache.cache_type, CacheType::Memory);
    assert_eq!(config.cache.size_limit_mb, 100);
    assert_eq!(config.cache.ttl_seconds, 3600);
    assert!(!config.cache.compression_enabled);
    assert!(!config.cache.encryption_enabled);
    assert_eq!(config.cache.eviction_policy, CacheEvictionPolicy::LRU);

    // Test parallel config
    assert!(config.parallel.enabled);
    assert_eq!(config.parallel.max_threads, 4);
    assert_eq!(config.parallel.thread_pool_size, 8);
    assert!(config.parallel.async_enabled);
    assert_eq!(config.parallel.async_runtime_threads, 2);
    assert!(config.parallel.work_stealing_enabled);
    assert_eq!(config.parallel.task_queue_size, 1000);
    assert!(config.parallel.load_balancing_enabled);

    // Test memory config
    assert_eq!(config.memory.max_memory_mb, 512);
    assert_eq!(config.memory.memory_limit_mb, 1024);
    assert!(config.memory.gc_enabled);
    assert_eq!(config.memory.gc_interval_seconds, 300);
    assert!(config.memory.monitoring_enabled);
    assert_eq!(config.memory.warning_threshold_percent, 80);
    assert_eq!(config.memory.critical_threshold_percent, 95);

    // Test network config
    assert_eq!(config.network.connection_timeout_seconds, 30);
    assert_eq!(config.network.request_timeout_seconds, 60);
    assert_eq!(config.network.max_connections, 100);
    assert!(config.network.keep_alive_enabled);
    assert_eq!(config.network.keep_alive_timeout_seconds, 60);
    assert!(config.network.connection_pooling_enabled);
    assert_eq!(config.network.connection_pool_size, 10);
    assert!(config.network.retry_enabled);
    assert_eq!(config.network.max_retry_attempts, 3);
    assert_eq!(config.network.retry_delay_seconds, 5);

    // Test optimization config
    assert!(config.optimization.graph_optimization_enabled);
    assert!(config.optimization.path_optimization_enabled);
    assert!(config.optimization.constraint_optimization_enabled);
    assert!(config.optimization.version_selection_optimization_enabled);
    assert!(config.optimization.conflict_resolution_optimization_enabled);
    assert!(config.optimization.validation_optimization_enabled);
    assert!(config.optimization.caching_optimization_enabled);
    assert!(config.optimization.parallel_optimization_enabled);
    assert_eq!(
        config.optimization.optimization_level,
        OptimizationLevel::Standard
    );
    assert!(!config.optimization.profile_guided_optimization_enabled);

    // Test monitoring config
    assert!(config.monitoring.performance_monitoring_enabled);
    assert!(config.monitoring.memory_monitoring_enabled);
    assert!(!config.monitoring.network_monitoring_enabled);
    assert!(config.monitoring.cache_monitoring_enabled);
    assert!(config.monitoring.error_monitoring_enabled);
    assert_eq!(config.monitoring.monitoring_interval_seconds, 60);
    assert!(config.monitoring.metrics_collection_enabled);
    assert_eq!(config.monitoring.metrics_format, MetricsFormat::JSON);
    assert!(!config.monitoring.alerting_enabled);

    // Test alert thresholds
    assert_eq!(
        config
            .monitoring
            .alert_thresholds
            .memory_usage_threshold_percent,
        80
    );
    assert_eq!(
        config
            .monitoring
            .alert_thresholds
            .cpu_usage_threshold_percent,
        80
    );
    assert_eq!(
        config
            .monitoring
            .alert_thresholds
            .response_time_threshold_ms,
        5000
    );
    assert_eq!(
        config
            .monitoring
            .alert_thresholds
            .error_rate_threshold_percent,
        5
    );
    assert_eq!(
        config
            .monitoring
            .alert_thresholds
            .cache_hit_rate_threshold_percent,
        80
    );
}

#[test]
fn test_config_validation() {
    let mut config = LockConfig::new();

    // Test valid configuration
    assert!(config.validate_config().is_ok());

    // Test invalid compatibility threshold
    config.conflict_resolution.compatibility_threshold = 1.5;
    assert!(config.validate_config().is_err());

    // Reset to valid value
    config.conflict_resolution.compatibility_threshold = 0.8;
    assert!(config.validate_config().is_ok());

    // Test invalid timeout values
    config.validation.timeout_seconds = 0;
    assert!(config.validate_config().is_err());

    config.validation.timeout_seconds = 4000; // Too high
    assert!(config.validate_config().is_err());

    // Reset to valid value
    config.validation.timeout_seconds = 60;
    assert!(config.validate_config().is_ok());
}

#[test]
fn test_config_serialization() {
    let config = LockConfig::new();

    // Test YAML serialization
    let yaml = serde_yaml::to_string(&config).unwrap();
    assert!(yaml.contains("version:"));
    assert!(yaml.contains("resolution:"));
    assert!(yaml.contains("conflict_resolution:"));
    assert!(yaml.contains("update_policies:"));
    assert!(yaml.contains("validation:"));
    assert!(yaml.contains("performance:"));

    // Test YAML deserialization
    let deserialized: LockConfig = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(config.version, deserialized.version);
    assert_eq!(
        config.resolution.default_strategy,
        deserialized.resolution.default_strategy
    );
    assert_eq!(
        config.conflict_resolution.primary_strategy,
        deserialized.conflict_resolution.primary_strategy
    );

    // Test JSON serialization
    let json = serde_json::to_string_pretty(&config).unwrap();
    assert!(json.contains("\"version\""));
    assert!(json.contains("\"resolution\""));
    assert!(json.contains("\"conflict_resolution\""));

    // Test JSON deserialization
    let deserialized_json: LockConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.version, deserialized_json.version);
}

#[test]
fn test_config_merge() {
    let mut config1 = LockConfig::new();
    let mut config2 = LockConfig::new();

    // Modify config2
    config2.resolution.default_strategy = ResolutionStrategy::Conservative;
    config2.conflict_resolution.compatibility_threshold = 0.9;
    config2.update_policies.auto_update_enabled = true;
    config2.validation.validation_level = LockValidationLevel::Strict;
    config2.performance.cache.size_limit_mb = 200;

    // Merge config2 into config1
    config1.merge(&config2).unwrap();

    // Verify merged values
    assert_eq!(
        config1.resolution.default_strategy,
        ResolutionStrategy::Conservative
    );
    assert_eq!(config1.conflict_resolution.compatibility_threshold, 0.9);
    assert!(config1.update_policies.auto_update_enabled);
    assert_eq!(
        config1.validation.validation_level,
        LockValidationLevel::Strict
    );
    assert_eq!(config1.performance.cache.size_limit_mb, 200);

    // Verify original values that weren't changed
    assert!(config1.resolution.smart_resolution_enabled);
    assert!(config1.conflict_resolution.enable_auto_detection);
    assert_eq!(config1.resolution.max_depth, 10);
}

#[test]
fn test_environment_config() {
    let mut config = LockConfig::new();

    // Create environment-specific config
    let env_config = EnvironmentLockConfig {
        name: "production".to_string(),
        description: Some("Production environment".to_string()),
        resolution_strategy: Some(ResolutionStrategy::Conservative),
        conflict_resolution_strategy: Some(ConflictResolutionStrategy::Conservative),
        update_policies: None,
        validation_rules: None,
        performance_tuning: None,
        settings: HashMap::new(),
    };

    // Set environment config
    config.set_environment_config(ConfigEnvironment::Production, env_config);

    // Retrieve and verify
    let retrieved = config.get_environment_config(&ConfigEnvironment::Production);
    assert!(retrieved.is_some());
    let retrieved_config = retrieved.unwrap();
    assert_eq!(retrieved_config.name, "production");
    assert_eq!(
        retrieved_config.description,
        Some("Production environment".to_string())
    );
    assert_eq!(
        retrieved_config.resolution_strategy,
        Some(ResolutionStrategy::Conservative)
    );
    assert_eq!(
        retrieved_config.conflict_resolution_strategy,
        Some(ConflictResolutionStrategy::Conservative)
    );

    // Test non-existent environment
    let non_existent = config.get_environment_config(&ConfigEnvironment::Testing);
    assert!(non_existent.is_none());
}

#[test]
fn test_config_value_access() {
    let config = LockConfig::new();

    // Test getting values by path
    let smart_resolution = config.get_value("resolution.smart_resolution_enabled");
    assert!(smart_resolution.is_some());
    assert_eq!(smart_resolution.unwrap().as_bool().unwrap(), true);

    let version = config.get_value("version");
    assert!(version.is_some());
    assert_eq!(version.unwrap().as_str().unwrap(), CURRENT_CONFIG_VERSION);

    let default_strategy = config.get_value("resolution.default_strategy");
    assert!(default_strategy.is_some());
    assert_eq!(default_strategy.unwrap().as_str().unwrap(), "Latest");

    let compatibility_threshold = config.get_value("conflict_resolution.compatibility_threshold");
    assert!(compatibility_threshold.is_some());
    assert_eq!(compatibility_threshold.unwrap().as_f64().unwrap(), 0.8);

    // Test non-existent paths
    let non_existent = config.get_value("non.existent.path");
    assert!(non_existent.is_none());
}

#[test]
fn test_config_value_setting() {
    let mut config = LockConfig::new();

    // Test setting boolean values
    config
        .set_value(
            "resolution.smart_resolution_enabled",
            serde_json::json!(false),
        )
        .unwrap();
    assert!(!config.resolution.smart_resolution_enabled);

    config
        .set_value("resolution.prefer_stable", serde_json::json!(false))
        .unwrap();
    assert!(!config.resolution.prefer_stable);

    // Test setting numeric values
    config
        .set_value(
            "conflict_resolution.compatibility_threshold",
            serde_json::json!(0.95),
        )
        .unwrap();
    assert_eq!(config.conflict_resolution.compatibility_threshold, 0.95);

    config
        .set_value("resolution.max_depth", serde_json::json!(15))
        .unwrap();
    assert_eq!(config.resolution.max_depth, 15);

    // Test setting enum values
    config
        .set_value(
            "resolution.default_strategy",
            serde_json::json!("Conservative"),
        )
        .unwrap();
    assert_eq!(
        config.resolution.default_strategy,
        ResolutionStrategy::Conservative
    );

    config
        .set_value(
            "conflict_resolution.primary_strategy",
            serde_json::json!("SmartSelection"),
        )
        .unwrap();
    assert_eq!(
        config.conflict_resolution.primary_strategy,
        ConflictResolutionStrategy::SmartSelection
    );

    // Test setting update policy values
    config
        .set_value(
            "update_policies.auto_update_enabled",
            serde_json::json!(true),
        )
        .unwrap();
    assert!(config.update_policies.auto_update_enabled);

    config
        .set_value(
            "update_policies.update_frequency",
            serde_json::json!("Daily"),
        )
        .unwrap();
    assert_eq!(
        config.update_policies.update_frequency,
        UpdateFrequency::Daily
    );

    // Test setting validation values
    config
        .set_value("validation.validation_enabled", serde_json::json!(false))
        .unwrap();
    assert!(!config.validation.validation_enabled);

    config
        .set_value("validation.validation_level", serde_json::json!("Strict"))
        .unwrap();
    assert_eq!(
        config.validation.validation_level,
        LockValidationLevel::Strict
    );

    // Test setting performance values
    config
        .set_value("performance.cache.enabled", serde_json::json!(false))
        .unwrap();
    assert!(!config.performance.cache.enabled);

    config
        .set_value("performance.cache.size_limit_mb", serde_json::json!(500))
        .unwrap();
    assert_eq!(config.performance.cache.size_limit_mb, 500);
}

#[test]
fn test_config_export_import() {
    let config = LockConfig::new();

    // Test YAML export/import
    let yaml = config.export("yaml").unwrap();
    assert!(yaml.contains("version:"));
    assert!(yaml.contains("resolution:"));

    let mut new_config = LockConfig::new();
    new_config.import(&yaml, "yaml").unwrap();
    assert_eq!(config.version, new_config.version);
    assert_eq!(
        config.resolution.default_strategy,
        new_config.resolution.default_strategy
    );

    // Test JSON export/import
    let json = config.export("json").unwrap();
    assert!(json.contains("\"version\""));
    assert!(json.contains("\"resolution\""));

    let mut new_config2 = LockConfig::new();
    new_config2.import(&json, "json").unwrap();
    assert_eq!(config.version, new_config2.version);
    assert_eq!(
        config.conflict_resolution.primary_strategy,
        new_config2.conflict_resolution.primary_strategy
    );

    // Test unsupported format
    let result = config.export("unsupported");
    assert!(result.is_err());

    let result = new_config.import("invalid", "unsupported");
    assert!(result.is_err());
}

#[test]
fn test_config_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_lock_config.yaml");

    let mut config = LockConfig::new();
    config.resolution.default_strategy = ResolutionStrategy::Conservative;
    config.conflict_resolution.compatibility_threshold = 0.9;

    // Test saving to file
    config.save_to_file(&config_path).unwrap();
    assert!(config_path.exists());

    // Test loading from file
    let loaded_config = LockConfig::load_from_file(&config_path).unwrap();
    assert_eq!(loaded_config.version, config.version);
    assert_eq!(
        loaded_config.resolution.default_strategy,
        ResolutionStrategy::Conservative
    );
    assert_eq!(
        loaded_config.conflict_resolution.compatibility_threshold,
        0.9
    );

    // Test loading non-existent file
    let non_existent_path = temp_dir.path().join("non_existent.yaml");
    let result = LockConfig::load_from_file(&non_existent_path);
    assert!(result.is_err());
}

#[test]
fn test_enum_serialization() {
    // Test ResolutionStrategy serialization
    let strategies = vec![
        ResolutionStrategy::Latest,
        ResolutionStrategy::Earliest,
        ResolutionStrategy::Pinned,
        ResolutionStrategy::Range,
        ResolutionStrategy::Compatible,
        ResolutionStrategy::Conservative,
        ResolutionStrategy::Aggressive,
        ResolutionStrategy::Smart,
        ResolutionStrategy::Hybrid,
    ];

    for strategy in strategies {
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: ResolutionStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(strategy, deserialized);
    }

    // Test ConflictResolutionStrategy serialization
    let conflict_strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::PinnedVersion,
        ConflictResolutionStrategy::ManualResolution,
        ConflictResolutionStrategy::AutomaticDetection,
        ConflictResolutionStrategy::HistoryTracking,
        ConflictResolutionStrategy::SmartSelection,
        ConflictResolutionStrategy::Conservative,
        ConflictResolutionStrategy::Aggressive,
        ConflictResolutionStrategy::Hybrid,
    ];

    for strategy in conflict_strategies {
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: ConflictResolutionStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(strategy, deserialized);
    }

    // Test UpdateFrequency serialization
    let frequencies = vec![
        UpdateFrequency::Never,
        UpdateFrequency::Daily,
        UpdateFrequency::Weekly,
        UpdateFrequency::Monthly,
        UpdateFrequency::OnDemand,
        UpdateFrequency::Scheduled("0 0 * * *".to_string()),
    ];

    for frequency in frequencies {
        let json = serde_json::to_string(&frequency).unwrap();
        let deserialized: UpdateFrequency = serde_json::from_str(&json).unwrap();
        assert_eq!(frequency, deserialized);
    }
}

#[test]
fn test_custom_validation_rules() {
    let mut config = LockConfig::new();

    // Add custom validation rule
    let custom_rule = rhema_config::lock::CustomValidationRule {
        name: "test_rule".to_string(),
        description: Some("Test validation rule".to_string()),
        severity: ValidationSeverity::Warning,
        enabled: true,
        expression: "dependency.version >= '1.0.0'".to_string(),
        parameters: HashMap::new(),
        error_message: Some("Dependency version must be at least 1.0.0".to_string()),
        fail_on_violation: false,
    };

    config
        .validation
        .custom_rules
        .insert("test_rule".to_string(), custom_rule);

    // Verify custom rule was added
    assert!(config.validation.custom_rules.contains_key("test_rule"));
    let rule = &config.validation.custom_rules["test_rule"];
    assert_eq!(rule.name, "test_rule");
    assert_eq!(rule.severity, ValidationSeverity::Warning);
    assert!(rule.enabled);
    assert_eq!(rule.expression, "dependency.version >= '1.0.0'");
}

#[test]
fn test_notification_channels() {
    let mut config = UpdateNotificationConfig::default();

    // Test different notification channels
    config.channels = vec![
        NotificationChannel::Email,
        NotificationChannel::Slack,
        NotificationChannel::Discord,
        NotificationChannel::Webhook("https://example.com/webhook".to_string()),
        NotificationChannel::Console,
        NotificationChannel::LogFile,
    ];

    assert_eq!(config.channels.len(), 6);
    assert!(matches!(config.channels[0], NotificationChannel::Email));
    assert!(matches!(config.channels[1], NotificationChannel::Slack));
    assert!(matches!(config.channels[2], NotificationChannel::Discord));
    assert!(matches!(
        config.channels[3],
        NotificationChannel::Webhook(_)
    ));
    assert!(matches!(config.channels[4], NotificationChannel::Console));
    assert!(matches!(config.channels[5], NotificationChannel::LogFile));

    // Test webhook URL extraction
    if let NotificationChannel::Webhook(url) = &config.channels[3] {
        assert_eq!(url, "https://example.com/webhook");
    } else {
        panic!("Expected Webhook channel");
    }
}

#[test]
fn test_cache_configurations() {
    // Test different cache types
    let cache_types = vec![
        CacheType::Memory,
        CacheType::File,
        CacheType::Redis,
        CacheType::Custom("custom_cache".to_string()),
    ];

    for cache_type in cache_types {
        let mut config = CacheConfig::default();
        config.cache_type = cache_type.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CacheConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.cache_type, cache_type);
    }

    // Test eviction policies
    let eviction_policies = vec![
        CacheEvictionPolicy::LRU,
        CacheEvictionPolicy::LFU,
        CacheEvictionPolicy::FIFO,
        CacheEvictionPolicy::TimeBased,
        CacheEvictionPolicy::SizeBased,
    ];

    for policy in eviction_policies {
        let mut config = CacheConfig::default();
        config.eviction_policy = policy.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CacheConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.eviction_policy, policy);
    }
}

#[test]
fn test_optimization_levels() {
    let optimization_levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Basic,
        OptimizationLevel::Standard,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Maximum,
    ];

    for level in optimization_levels {
        let mut config = OptimizationConfig::default();
        config.optimization_level = level.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OptimizationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.optimization_level, level);
    }
}

#[test]
fn test_validation_levels() {
    let validation_levels = vec![
        LockValidationLevel::Minimal,
        LockValidationLevel::Standard,
        LockValidationLevel::Strict,
        LockValidationLevel::Custom("custom_level".to_string()),
    ];

    for level in validation_levels {
        let mut config = ValidationConfig::default();
        config.validation_level = level.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ValidationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.validation_level, level);
    }
}

#[test]
fn test_metrics_formats() {
    let metrics_formats = vec![
        MetricsFormat::JSON,
        MetricsFormat::Prometheus,
        MetricsFormat::Graphite,
        MetricsFormat::Custom("custom_format".to_string()),
    ];

    for format in metrics_formats {
        let mut config = MonitoringConfig::default();
        config.metrics_format = format.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: MonitoringConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.metrics_format, format);
    }
}

#[test]
fn test_constraint_types() {
    let constraint_types = vec![
        ConstraintType::Exact,
        ConstraintType::Range,
        ConstraintType::Caret,
        ConstraintType::Tilde,
        ConstraintType::Wildcard,
        ConstraintType::Latest,
        ConstraintType::Earliest,
    ];

    for constraint_type in constraint_types {
        let mut config = VersionConstraintConfig::default();
        config.default_type = constraint_type.clone();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: VersionConstraintConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.default_type, constraint_type);
    }
}

#[test]
fn test_validation_severities() {
    let severities = vec![
        ValidationSeverity::Info,
        ValidationSeverity::Warning,
        ValidationSeverity::Error,
        ValidationSeverity::Critical,
    ];

    for severity in severities {
        let rule = rhema_config::lock::CustomValidationRule {
            name: "test".to_string(),
            description: None,
            severity: severity.clone(),
            enabled: true,
            expression: "test".to_string(),
            parameters: HashMap::new(),
            error_message: None,
            fail_on_violation: false,
        };

        let json = serde_json::to_string(&rule).unwrap();
        let deserialized: rhema_config::lock::CustomValidationRule =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.severity, severity);
    }
}
