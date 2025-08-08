# Syneidesis Configuration Management

A centralized configuration management system for the Syneidesis ecosystem. This crate provides comprehensive configuration loading, validation, and management capabilities across all Syneidesis components.

## Features

- **Multi-format Support**: YAML, JSON, TOML, and environment variables
- **Validation**: Schema-based configuration validation with custom rules
- **Environment Integration**: Seamless environment variable support
- **Hot Reloading**: Dynamic configuration updates
- **Type Safety**: Strongly typed configuration structures
- **Default Values**: Comprehensive default configurations
- **Builder Pattern**: Fluent API for configuration setup
- **Error Handling**: Comprehensive error types and handling
- **Statistics**: Configuration loading and validation statistics
- **Caching**: Optional configuration caching
- **Monitoring**: Configuration change monitoring
- **Backup**: Configuration backup and restore
- **Migration**: Configuration schema migration support

## Quick Start

### Basic Usage

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration manager with defaults
    let config_manager = ConfigBuilder::new()
        .with_defaults()
        .build()
        .await?;

    // Load configuration
    let config = config_manager.load().await?;
    println!("Configuration loaded: {:?}", config);
    Ok(())
}
```

### Loading from Files

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration manager with file
    let config_manager = ConfigBuilder::new()
        .with_file("config.yaml")
        .with_defaults()
        .build()
        .await?;

    let config = config_manager.load().await?;
    println!("Configuration loaded from file: {:?}", config);
    Ok(())
}
```

### Environment Variables

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set environment variables
    std::env::set_var("SYNEIDESIS_SYSTEM_NAME", "my-system");
    std::env::set_var("SYNEIDESIS_GRPC_ADDR", "0.0.0.0:50051");

    // Create configuration manager with environment variables
    let config_manager = ConfigBuilder::new()
        .with_env_prefix("SYNEIDESIS")
        .with_defaults()
        .build()
        .await?;

    let config = config_manager.load().await?;
    println!("Configuration loaded from environment: {:?}", config);
    Ok(())
}
```

### Custom Configuration

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder, SyneidesisConfig};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let mut custom_config = SyneidesisConfig::default();
    if let Some(system) = &mut custom_config.system {
        system.name = "custom-system".to_string();
        system.version = "2.0.0".to_string();
    }

    // Create configuration manager with custom config
    let config_manager = ConfigBuilder::new()
        .with_default_config(custom_config)
        .with_custom_value("custom.field", Value::String("custom-value".to_string()))
        .build()
        .await?;

    let config = config_manager.load().await?;
    println!("Custom configuration loaded: {:?}", config);
    Ok(())
}
```

## Configuration Structure

The configuration system supports a comprehensive structure covering all aspects of the Syneidesis ecosystem:

### System Configuration

```yaml
system:
  name: "syneidesis"
  version: "1.0.0"
  environment: "development"  # development, staging, production, test
  timezone: "UTC"
  locale: "en_US"
  max_concurrent_operations: 1000
  operation_timeout: 30s
  debug: false
  profiling: false
  metrics_enabled: true
  metrics_interval: 60s
  health_check_interval: 30s
  shutdown_timeout: 30s
```

### Agent Configuration

```yaml
agent:
  max_agents: 1000
  heartbeat_interval: 30s
  agent_timeout: 120s
  registration_timeout: 60s
  auto_discovery: true
  discovery_interval: 300s
  cleanup_interval: 600s
  load_balancing: true
  load_balancing_strategy: "round_robin"
  priority_levels: 10
  failover: true
  failover_timeout: 30s
  resource_limits:
    max_cpu_usage: 80.0
    max_memory_usage: 1073741824  # 1GB
    max_disk_usage: 10737418240   # 10GB
    max_network_bandwidth: 104857600  # 100MB/s
    max_concurrent_tasks: 100
```

### Coordination Configuration

```yaml
coordination:
  max_agents: 1000
  heartbeat_interval: 30
  agent_timeout: 120
  enable_metrics: true
  enable_conflict_resolution: true
  conflict_resolution_strategy: "priority"
  enable_state_sync: true
  state_sync_interval: 60
  enable_message_persistence: false
  message_retention_period: 86400
  max_message_queue_size: 10000
  enable_message_encryption: false
  enable_message_compression: true
```

### gRPC Configuration

```yaml
grpc:
  addr: "127.0.0.1:50051"
  max_message_size: 10485760  # 10MB
  connection_timeout: 30
  keep_alive_interval: 30
  keep_alive_timeout: 10
  max_concurrent_streams: 1000
  enable_reflection: true
  enable_health_checks: true
  enable_metrics: true
  enable_tracing: false
  tls:
    cert_file: "/path/to/cert.pem"
    key_file: "/path/to/key.pem"
    ca_file: "/path/to/ca.pem"
    client_auth: false
    min_tls_version: "1.2"
    max_tls_version: "1.3"
```

### HTTP Configuration

```yaml
http:
  addr: "127.0.0.1"
  port: 8080
  max_request_size: 10485760  # 10MB
  request_timeout: 30
  enable_cors: true
  cors_origins: ["*"]
  enable_rate_limiting: true
  rate_limit: 1000
  enable_compression: true
  enable_websocket: true
  websocket_ping_interval: 30
  websocket_pong_timeout: 10
  enable_static_files: true
  static_files_dir: "static"
  enable_api_docs: true
  api_docs_path: "/docs"
```

### Network Configuration

```yaml
network:
  bind_addr: "0.0.0.0"
  external_addr: null
  interface: null
  enable_ipv6: false
  tcp_keep_alive: true
  tcp_keep_alive_interval: 30
  tcp_keep_alive_probes: 3
  tcp_keep_alive_time: 60
  tcp_no_delay: true
  tcp_reuse_addr: true
  tcp_linger: null
  socket_buffer_size: 65536  # 64KB
  enable_multicast: false
  multicast_addr: null
  multicast_interface: null
  multicast_ttl: 1
```

### Security Configuration

```yaml
security:
  enable_auth: false
  auth_method: "jwt"
  jwt_secret: null
  jwt_expiration: 3600
  jwt_refresh_expiration: 86400
  enable_authorization: false
  authorization_policy: null
  enable_encryption: false
  encryption_algorithm: "AES-256-GCM"
  encryption_key: null
  enable_rate_limiting: true
  rate_limit_window: 60
  rate_limit_max_requests: 100
  enable_ip_whitelist: false
  ip_whitelist: []
  enable_ip_blacklist: false
  ip_blacklist: []
  enable_audit_logging: false
  audit_log_file: "audit.log"
  enable_session_management: false
  session_timeout: 3600
  session_cleanup_interval: 300
```

### Logging Configuration

```yaml
logging:
  level: "info"
  format: "text"
  file: null
  enable_console: true
  enable_file: false
  enable_syslog: false
  syslog_facility: "daemon"
  enable_structured: false
  enable_json: false
  enable_rotation: true
  rotation_size: 104857600  # 100MB
  rotation_count: 5
  enable_compression: true
  timestamp_format: "%Y-%m-%d %H:%M:%S"
  enable_color: true
  enable_thread_ids: false
  enable_target_filtering: false
  include_targets: []
  exclude_targets: []
```

### Validation Configuration

```yaml
validation:
  enable_validation: true
  mode: "strict"
  strict: true
  enable_schema: true
  schema_file: null
  enable_custom: false
  custom_rules_file: null
  enable_cross_field: true
  enable_dependency: true
  enable_conditional: true
  error_mode: "collect"
  max_errors: 100
  enable_caching: true
  cache_size: 1000
  cache_ttl: 3600
```

## Environment Variables

The configuration system supports environment variable overrides using the `SYNEIDESIS_` prefix:

```bash
# System configuration
export SYNEIDESIS_SYSTEM_NAME="my-system"
export SYNEIDESIS_SYSTEM_VERSION="2.0.0"
export SYNEIDESIS_SYSTEM_ENVIRONMENT="production"

# Agent configuration
export SYNEIDESIS_AGENT_MAX_AGENTS="500"
export SYNEIDESIS_AGENT_HEARTBEAT_INTERVAL="60"

# gRPC configuration
export SYNEIDESIS_GRPC_ADDR="0.0.0.0:50051"
export SYNEIDESIS_GRPC_MAX_MESSAGE_SIZE="20971520"

# HTTP configuration
export SYNEIDESIS_HTTP_ADDR="0.0.0.0"
export SYNEIDESIS_HTTP_PORT="8080"

# Logging configuration
export SYNEIDESIS_LOGGING_LEVEL="debug"
export SYNEIDESIS_LOGGING_ENABLE_JSON="true"
```

## Validation

The configuration system includes comprehensive validation:

### Built-in Validation Rules

```rust
use syneidesis_config::{ConfigValidator, validation::rules};

let mut validator = ConfigValidator::new();

// Add built-in rules
validator.add_rule(rules::system_config_required());
validator.add_rule(rules::agent_config_required());
validator.add_rule(rules::grpc_config_required());
validator.add_rule(rules::production_security_required());
validator.add_rule(rules::production_grpc_tls_required());

// Validate configuration
let result = validator.validate(&config);
```

### Custom Validation

```rust
use syneidesis_config::{ConfigValidator, ValidationError};

let mut validator = ConfigValidator::new();

// Add custom validator
validator.add_custom_validator("custom_rule", |config| {
    if let Some(system) = &config.system {
        if system.name == "forbidden-name" {
            return Err(ValidationError::CustomValidation {
                field: "system.name".to_string(),
                message: "This system name is not allowed".to_string(),
            });
        }
    }
    Ok(())
});

let result = validator.validate(&config);
```

## Advanced Features

### Hot Reloading

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

let config_manager = ConfigBuilder::new()
    .with_hot_reload(true)
    .with_file("config.yaml")
    .with_defaults()
    .build()
    .await?;

// Configuration will automatically reload when the file changes
```

### Caching

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

let config_manager = ConfigBuilder::new()
    .with_caching(true)
    .with_cache_dir("/tmp/config-cache")
    .with_cache_ttl(1800)  // 30 minutes
    .with_defaults()
    .build()
    .await?;
```

### Monitoring

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

let config_manager = ConfigBuilder::new()
    .with_monitoring(true)
    .with_monitoring_interval(60)  // Check every minute
    .with_defaults()
    .build()
    .await?;
```

### Statistics

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

let config_manager = ConfigBuilder::new()
    .with_statistics(true)
    .with_statistics_interval(300)  // Collect every 5 minutes
    .with_defaults()
    .build()
    .await?;

// Get statistics
let stats = config_manager.get_statistics().await;
println!("Load count: {}", stats.load_count);
println!("Average load time: {}ms", stats.avg_load_time_ms);
```

### Backup and Migration

```rust
use syneidesis_config::{ConfigManager, ConfigBuilder};

let config_manager = ConfigBuilder::new()
    .with_backup(true)
    .with_backup_dir("/backup/config")
    .with_migration(true)
    .with_migration_dir("/migrations")
    .with_defaults()
    .build()
    .await?;
```

## Error Handling

The configuration system provides comprehensive error handling:

```rust
use syneidesis_config::{ConfigError, ValidationError};

match config_manager.load().await {
    Ok(config) => {
        println!("Configuration loaded successfully");
    }
    Err(ConfigError::FileNotFound { path }) => {
        eprintln!("Configuration file not found: {:?}", path);
    }
    Err(ConfigError::ParseError { format, path, source }) => {
        eprintln!("Failed to parse {} file {:?}: {}", format, path, source);
    }
    Err(ConfigError::ValidationError { message }) => {
        eprintln!("Configuration validation failed: {}", message);
    }
    Err(e) => {
        eprintln!("Configuration error: {}", e);
    }
}
```

## Testing

The configuration system includes comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_loading() {
        let config_manager = ConfigBuilder::new()
            .with_defaults()
            .build()
            .await
            .unwrap();

        let config = config_manager.load().await.unwrap();
        assert!(config.system.is_some());
    }

    #[tokio::test]
    async fn test_config_validation() {
        let mut validator = ConfigValidator::new();
        let config = SyneidesisConfig::default();
        
        assert!(validator.validate(&config).is_ok());
    }
}
```

## Contributing

When contributing to the configuration system:

1. Add comprehensive tests for new features
2. Update documentation for new configuration options
3. Follow the existing error handling patterns
4. Ensure backward compatibility when possible
5. Add validation rules for new configuration fields

## License

This project is licensed under the Apache License, Version 2.0. 