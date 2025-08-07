# Lock Configuration System

The Rhema Lock Configuration System provides comprehensive configuration management for dependency resolution, conflict resolution, update policies, validation rules, and performance tuning. This system integrates seamlessly with the existing Rhema configuration infrastructure and provides sensible defaults while allowing extensive customization.

## Table of Contents

1. [Overview](#overview)
2. [Configuration Structure](#configuration-structure)
3. [Resolution Strategies](#resolution-strategies)
4. [Conflict Resolution](#conflict-resolution)
5. [Update Policies](#update-policies)
6. [Validation Rules](#validation-rules)
7. [Performance Tuning](#performance-tuning)
8. [Environment-Specific Configuration](#environment-specific-configuration)
9. [Usage Examples](#usage-examples)
10. [Integration with Existing Systems](#integration-with-existing-systems)
11. [Best Practices](#best-practices)

## Overview

The lock configuration system is designed to provide fine-grained control over all aspects of dependency management in Rhema. It supports:

- **Multiple resolution strategies** for different dependency types and scenarios
- **Advanced conflict resolution** with configurable strategies and thresholds
- **Flexible update policies** with scheduling, notifications, and rollback capabilities
- **Comprehensive validation rules** with custom rule support
- **Performance optimization** with caching, parallel processing, and monitoring
- **Environment-specific overrides** for different deployment scenarios

## Configuration Structure

The main configuration structure is defined in `LockConfig`:

```rust
pub struct LockConfig {
    pub version: String,
    pub resolution: ResolutionConfig,
    pub conflict_resolution: ConflictResolutionConfig,
    pub update_policies: UpdatePoliciesConfig,
    pub validation: ValidationConfig,
    pub performance: PerformanceConfig,
    pub environments: HashMap<ConfigEnvironment, EnvironmentLockConfig>,
    pub custom: HashMap<String, serde_json::Value>,
    pub audit_log: ConfigAuditLog,
    pub health: ConfigHealth,
    pub stats: ConfigStats,
    pub updated_at: DateTime<Utc>,
}
```

### Default Configuration

The system provides sensible defaults that work well for most projects:

```rust
let config = LockConfig::new();
// Creates a configuration with:
// - Latest resolution strategy
// - LatestCompatible conflict resolution
// - On-demand updates
// - Standard validation
// - Optimized performance settings
```

## Resolution Strategies

The resolution system supports multiple strategies for determining which versions of dependencies to use:

### Available Strategies

- **Latest**: Always use the latest available version
- **Earliest**: Use the earliest compatible version
- **Pinned**: Use pinned versions from lock files
- **Range**: Use versions within specified ranges
- **Compatible**: Use versions that satisfy all constraints
- **Conservative**: Prefer stable, well-tested versions
- **Aggressive**: Prefer latest features and improvements
- **Smart**: Use compatibility scoring for intelligent selection
- **Hybrid**: Combine multiple strategies in sequence

### Configuration Example

```yaml
resolution:
  default_strategy: "Latest"
  smart_resolution_enabled: true
  prefer_stable: true
  allow_prereleases: false
  allow_development: false
  max_depth: 10
  resolve_transitive: true
  pin_by_default: false
  strategy_overrides:
    "production-deps": "Conservative"
    "development-deps": "Aggressive"
  scope_strategies:
    "core": "Pinned"
    "experimental": "Latest"
  version_constraints:
    default_type: "Caret"
    use_caret_ranges: true
    use_tilde_ranges: false
    allow_wildcards: false
    allow_exact: true
    allow_ranges: true
```

## Conflict Resolution

The conflict resolution system provides advanced strategies for handling dependency conflicts:

### Available Strategies

- **LatestCompatible**: Select the latest version that satisfies all requirements
- **PinnedVersion**: Enforce pinned versions when available
- **ManualResolution**: Require manual intervention for conflicts
- **AutomaticDetection**: Detect and report conflicts automatically
- **HistoryTracking**: Use historical resolution decisions
- **SmartSelection**: Use compatibility scoring for selection
- **Conservative**: Prefer stable versions
- **Aggressive**: Prefer latest versions
- **Hybrid**: Try multiple strategies in sequence

### Configuration Example

```yaml
conflict_resolution:
  primary_strategy: "LatestCompatible"
  fallback_strategies:
    - "SmartSelection"
    - "Conservative"
  enable_auto_detection: true
  track_history: true
  max_attempts: 3
  allow_user_prompts: true
  prefer_stable: true
  strict_pinning: false
  compatibility_threshold: 0.8
  parallel_resolution: true
  max_parallel_threads: 4
  timeout_seconds: 300
  auto_resolve_low_severity: true
  auto_resolve_medium_severity: false
  fail_on_high_severity: false
  fail_on_critical_severity: true
  dependency_type_preferences:
    "production": "Conservative"
    "development": "Aggressive"
```

## Update Policies

The update policy system controls when and how dependencies are updated:

### Update Frequency

- **Never**: Disable automatic updates
- **Daily**: Update once per day
- **Weekly**: Update once per week
- **Monthly**: Update once per month
- **OnDemand**: Update only when explicitly requested
- **Scheduled**: Use custom cron expressions

### Configuration Example

```yaml
update_policies:
  auto_update_enabled: false
  update_frequency: "OnDemand"
  update_on_dependency_changes: true
  update_on_scope_changes: true
  update_on_constraint_changes: true
  preserve_pinned_versions: true
  allow_breaking_changes: false
  require_approval_major_updates: true
  require_approval_minor_updates: false
  require_approval_patch_updates: false
  notifications:
    notify_on_available: true
    notify_on_success: false
    notify_on_failure: true
    notify_on_breaking_changes: true
    channels:
      - "Email"
      - "Slack"
    recipients:
      - "team@example.com"
  rollback:
    auto_rollback_enabled: true
    rollback_on_validation_failure: true
    rollback_on_test_failure: true
    rollback_on_build_failure: true
    max_rollback_attempts: 3
    rollback_timeout_seconds: 300
    preserve_rollback_history: true
    max_rollback_history: 10
  scheduling:
    scheduled_updates_enabled: false
    schedule: "0 2 * * 0"  # Weekly on Sunday at 2 AM
    timezone: "UTC"
    business_hours_only: false
    business_hours_start: "09:00"
    business_hours_end: "17:00"
    skip_weekends: false
    skip_holidays: false
```

## Validation Rules

The validation system ensures lock files meet quality and security standards:

### Validation Levels

- **Minimal**: Basic validation only
- **Standard**: Comprehensive validation (default)
- **Strict**: Maximum validation with strict rules
- **Custom**: User-defined validation level

### Configuration Example

```yaml
validation:
  validation_enabled: true
  validation_level: "Standard"
  validate_on_generation: true
  validate_on_loading: true
  validate_on_updates: true
  validate_on_conflict_resolution: true
  timeout_seconds: 60
  fail_fast: false
  generate_reports: false
  report_output_path: "./validation-reports"
  include_warnings: true
  include_suggestions: true
  rules:
    validate_version_constraints: true
    validate_dependency_types: true
    validate_scope_references: true
    validate_circular_dependencies: true
    validate_dependency_depth: true
    validate_checksums: true
    validate_timestamps: true
    validate_metadata: true
    validate_performance_metrics: false
    validate_security: false
    validate_licenses: false
    validate_architecture: false
    validate_platform: false
    max_dependency_depth: 10
    max_circular_dependencies: 0
    max_validation_time: 30
  custom_rules:
    security_scan:
      name: "Security Vulnerability Scan"
      description: "Scan for known security vulnerabilities"
      severity: "Error"
      enabled: true
      expression: "vulnerability_scan(dependency)"
      parameters:
        scan_level: "comprehensive"
      error_message: "Security vulnerability detected in {dependency}"
      fail_on_violation: true
```

## Performance Tuning

The performance system optimizes lock file operations for speed and efficiency:

### Configuration Example

```yaml
performance:
  cache:
    enabled: true
    cache_type: "Memory"
    size_limit_mb: 100
    ttl_seconds: 3600
    directory: "./cache"
    compression_enabled: false
    encryption_enabled: false
    key_prefix: "rhema_lock"
    statistics_enabled: true
    eviction_policy: "LRU"
  parallel:
    enabled: true
    max_threads: 4
    thread_pool_size: 8
    async_enabled: true
    async_runtime_threads: 2
    work_stealing_enabled: true
    task_queue_size: 1000
    load_balancing_enabled: true
  memory:
    max_memory_mb: 512
    memory_limit_mb: 1024
    gc_enabled: true
    gc_interval_seconds: 300
    monitoring_enabled: true
    warning_threshold_percent: 80
    critical_threshold_percent: 95
  network:
    connection_timeout_seconds: 30
    request_timeout_seconds: 60
    max_connections: 100
    keep_alive_enabled: true
    keep_alive_timeout_seconds: 60
    connection_pooling_enabled: true
    connection_pool_size: 10
    retry_enabled: true
    max_retry_attempts: 3
    retry_delay_seconds: 5
  optimization:
    graph_optimization_enabled: true
    path_optimization_enabled: true
    constraint_optimization_enabled: true
    version_selection_optimization_enabled: true
    conflict_resolution_optimization_enabled: true
    validation_optimization_enabled: true
    caching_optimization_enabled: true
    parallel_optimization_enabled: true
    optimization_level: "Standard"
    profile_guided_optimization_enabled: false
  monitoring:
    performance_monitoring_enabled: true
    memory_monitoring_enabled: true
    network_monitoring_enabled: false
    cache_monitoring_enabled: true
    error_monitoring_enabled: true
    monitoring_interval_seconds: 60
    metrics_collection_enabled: true
    metrics_format: "JSON"
    metrics_export_path: "./metrics"
    alerting_enabled: false
    alert_thresholds:
      memory_usage_threshold_percent: 80
      cpu_usage_threshold_percent: 80
      response_time_threshold_ms: 5000
      error_rate_threshold_percent: 5
      cache_hit_rate_threshold_percent: 80
```

## Environment-Specific Configuration

The system supports environment-specific overrides for different deployment scenarios:

### Configuration Example

```yaml
environments:
  development:
    name: "Development"
    description: "Development environment"
    resolution_strategy: "Latest"
    conflict_resolution_strategy: "Aggressive"
    update_policies:
      auto_update_enabled: true
      update_frequency: "Daily"
      allow_breaking_changes: true
    validation_rules:
      validation_level: "Minimal"
      validate_security: false
    performance_tuning:
      cache:
        size_limit_mb: 50
      parallel:
        max_threads: 2
  testing:
    name: "Testing"
    description: "Testing environment"
    resolution_strategy: "Conservative"
    conflict_resolution_strategy: "Conservative"
    update_policies:
      auto_update_enabled: false
      update_frequency: "OnDemand"
    validation_rules:
      validation_level: "Standard"
      validate_security: true
  staging:
    name: "Staging"
    description: "Staging environment"
    resolution_strategy: "Pinned"
    conflict_resolution_strategy: "ManualResolution"
    update_policies:
      auto_update_enabled: false
      update_frequency: "OnDemand"
      require_approval_major_updates: true
    validation_rules:
      validation_level: "Strict"
      validate_security: true
      validate_licenses: true
  production:
    name: "Production"
    description: "Production environment"
    resolution_strategy: "Pinned"
    conflict_resolution_strategy: "Conservative"
    update_policies:
      auto_update_enabled: false
      update_frequency: "Never"
      require_approval_major_updates: true
      require_approval_minor_updates: true
    validation_rules:
      validation_level: "Strict"
      validate_security: true
      validate_licenses: true
      validate_architecture: true
    performance_tuning:
      cache:
        size_limit_mb: 200
      parallel:
        max_threads: 8
```

## Usage Examples

### Basic Configuration

```rust
use rhema_config::LockConfig;

// Create default configuration
let mut config = LockConfig::new();

// Customize resolution strategy
config.resolution.default_strategy = ResolutionStrategy::Conservative;

// Enable automatic conflict resolution
config.conflict_resolution.enable_auto_detection = true;

// Save configuration
config.save().unwrap();
```

### Loading and Modifying Configuration

```rust
// Load existing configuration
let mut config = LockConfig::load().unwrap();

// Set configuration values by path
config.set_value(
    "resolution.smart_resolution_enabled", 
    serde_json::json!(false)
).unwrap();

config.set_value(
    "conflict_resolution.compatibility_threshold", 
    serde_json::json!(0.9)
).unwrap();

// Get configuration values
let smart_resolution = config.get_value("resolution.smart_resolution_enabled");
let threshold = config.get_value("conflict_resolution.compatibility_threshold");

// Save changes
config.save().unwrap();
```

### Environment-Specific Configuration

```rust
// Get environment-specific configuration
let env_config = config.get_environment_config(&ConfigEnvironment::Production);

if let Some(env_config) = env_config {
    if let Some(strategy) = &env_config.resolution_strategy {
        println!("Production resolution strategy: {:?}", strategy);
    }
}

// Set environment-specific configuration
let production_config = EnvironmentLockConfig {
    name: "Production".to_string(),
    description: Some("Production environment".to_string()),
    resolution_strategy: Some(ResolutionStrategy::Pinned),
    conflict_resolution_strategy: Some(ConflictResolutionStrategy::Conservative),
    update_policies: None,
    validation_rules: None,
    performance_tuning: None,
    settings: HashMap::new(),
};

config.set_environment_config(ConfigEnvironment::Production, production_config);
```

### Configuration Export and Import

```rust
// Export configuration to different formats
let yaml = config.export("yaml").unwrap();
let json = config.export("json").unwrap();

// Import configuration from different formats
let mut new_config = LockConfig::new();
new_config.import(&yaml, "yaml").unwrap();

// Merge configurations
config.merge(&new_config).unwrap();
```

### Custom Validation Rules

```rust
use rhema_config::{ValidationSeverity, CustomValidationRule};

let custom_rule = CustomValidationRule {
    name: "security_scan".to_string(),
    description: Some("Security vulnerability scan".to_string()),
    severity: ValidationSeverity::Error,
    enabled: true,
    expression: "vulnerability_scan(dependency)".to_string(),
    parameters: HashMap::new(),
    error_message: Some("Security vulnerability detected in {dependency}".to_string()),
    fail_on_violation: true,
};

config.validation.custom_rules.insert("security_scan".to_string(), custom_rule);
```

## Integration with Existing Systems

The lock configuration system integrates seamlessly with existing Rhema systems:

### Integration with Global Configuration

```rust
use rhema_config::{GlobalConfig, LockConfig};

// Load global configuration
let global_config = GlobalConfig::load().unwrap();

// Load lock configuration
let lock_config = LockConfig::load().unwrap();

// Use both configurations together
if global_config.application.features.advanced_features {
    // Enable advanced lock features
    lock_config.resolution.smart_resolution_enabled = true;
    lock_config.conflict_resolution.enable_auto_detection = true;
}
```

### Integration with Conflict Resolver

```rust
use rhema_config::LockConfig;
use rhema_lock::ConflictResolver;

// Load configuration
let config = LockConfig::load().unwrap();

// Create conflict resolver with configuration
let conflict_config = config.conflict_resolution.clone();
let resolver = ConflictResolver::with_config(conflict_config);

// Use resolver with configured settings
let result = resolver.resolve_conflicts(&dependencies, &repo_path);
```

### Integration with Lock System

```rust
use rhema_config::LockConfig;
use rhema_lock::LockSystem;

// Load configuration
let config = LockConfig::load().unwrap();

// Apply configuration to lock system
let lock_system = LockSystem::new();
lock_system.apply_configuration(&config).unwrap();

// Use lock system with configured settings
let lock_file = lock_system.generate_lock_file(&dependencies).unwrap();
```

## Best Practices

### Configuration Management

1. **Use Environment-Specific Configurations**: Configure different settings for development, testing, staging, and production environments.

2. **Version Control Configuration**: Store configuration files in version control to ensure consistency across team members and environments.

3. **Incremental Configuration**: Start with defaults and gradually customize based on project needs.

4. **Documentation**: Document custom configurations and the reasoning behind specific settings.

### Security Considerations

1. **Validation Rules**: Enable security validation in production environments.

2. **Update Policies**: Require approval for major updates in production.

3. **Rollback Configuration**: Enable automatic rollback for failed updates.

4. **Access Control**: Restrict configuration modification in production environments.

### Performance Optimization

1. **Caching**: Enable caching for frequently accessed dependencies.

2. **Parallel Processing**: Use parallel resolution for large dependency graphs.

3. **Memory Management**: Monitor memory usage and adjust limits as needed.

4. **Network Optimization**: Configure connection pooling and retry logic.

### Monitoring and Alerting

1. **Metrics Collection**: Enable metrics collection for performance monitoring.

2. **Alert Thresholds**: Set appropriate thresholds for memory, CPU, and error rates.

3. **Logging**: Configure comprehensive logging for troubleshooting.

4. **Health Checks**: Implement health checks for the lock system.

### Testing

1. **Configuration Testing**: Test configuration changes in non-production environments.

2. **Validation Testing**: Verify that validation rules work as expected.

3. **Performance Testing**: Test performance with realistic dependency graphs.

4. **Integration Testing**: Test integration with other Rhema systems.

## Conclusion

The Rhema Lock Configuration System provides comprehensive control over dependency management while maintaining ease of use through sensible defaults. The system supports complex scenarios through environment-specific configurations, custom validation rules, and extensive performance tuning options.

By following the best practices outlined in this documentation, teams can effectively manage dependencies across different environments while maintaining security, performance, and reliability standards. 