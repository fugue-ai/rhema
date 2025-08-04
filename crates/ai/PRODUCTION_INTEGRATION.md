# Production Integration

This document describes the production integration features implemented for the Rhema AI service, providing enterprise-ready capabilities for deployment in production environments.

## Overview

The production integration system provides comprehensive support for deploying Rhema AI services in production environments with the following key features:

- **Persistence Layer**: Session and consensus state persistence with multiple storage backends
- **Distributed Deployment**: Multi-node cluster support with service discovery and load balancing
- **Advanced Features**: Message compression, encryption, and key management
- **Production Configuration**: Comprehensive configuration system for all components
- **Monitoring & Health**: Built-in health checking and performance monitoring

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ProductionAIService                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │Persistence  │  │Distributed  │  │Advanced     │        │
│  │Manager      │  │Manager      │  │Features     │        │
│  └─────────────┘  └─────────────┘  │Manager      │        │
│                                    └─────────────┘        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │SessionStore │  │ClusterMgr   │  │Compression  │        │
│  │ConsensusStore│  │NodeDiscovery│  │Encryption   │        │
│  │StateManager │  │LoadBalancer │  │KeyManagement│        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Persistence Layer

The persistence layer provides durable storage for sessions, consensus state, and general system state.

#### PersistenceManager

Central manager for all persistence operations:

```rust
use rhema_ai::persistence::{PersistenceManager, PersistenceConfig};

let config = PersistenceConfig {
    backend: StorageBackend::File,
    storage_path: Some(PathBuf::from("./data")),
    enable_backups: true,
    enable_compression: true,
    // ... other configuration
};

let persistence_manager = PersistenceManager::new(config).await?;
```

#### Storage Backends

- **File**: JSON/YAML file-based storage
- **SQLite**: Lightweight database storage
- **PostgreSQL**: Enterprise database storage
- **Redis**: High-performance in-memory storage
- **Memory**: In-memory storage (for testing)

#### SessionStore

Manages coordination session persistence:

```rust
// Store a session
persistence_manager.session_store().store_session(session).await?;

// Retrieve a session
let session = persistence_manager.session_store().get_session("session-id").await?;

// List sessions by status
let active_sessions = persistence_manager.session_store()
    .get_sessions_by_status(SessionStatus::Active).await?;
```

#### ConsensusStore

Manages consensus state persistence:

```rust
// Store consensus state
persistence_manager.consensus_store()
    .store_consensus_state("node-id".to_string(), state).await?;

// Store consensus entry
persistence_manager.consensus_store()
    .store_consensus_entry("node-id".to_string(), entry).await?;

// Mark entry as committed
persistence_manager.consensus_store()
    .mark_entry_committed("node-id", index).await?;
```

#### StateManager

Manages general system state:

```rust
// Store agent state
persistence_manager.state_manager()
    .store_agent_state(agent_info).await?;

// Update system metrics
persistence_manager.state_manager()
    .update_system_metrics(metrics).await?;

// Get agent performance statistics
let stats = persistence_manager.state_manager()
    .get_agent_performance_stats().await?;
```

### 2. Distributed Deployment

The distributed deployment system provides multi-node cluster support with service discovery and load balancing.

#### DistributedManager

Central manager for distributed operations:

```rust
use rhema_ai::distributed::{DistributedManager, DistributedConfig};

let config = DistributedConfig {
    node: NodeConfig {
        node_id: "node-1".to_string(),
        name: "rhema-node-1".to_string(),
        address: "127.0.0.1:8080".parse().unwrap(),
        role: NodeRole::Worker,
        // ... other configuration
    },
    cluster: ClusterConfig {
        name: "rhema-cluster".to_string(),
        min_nodes: 1,
        max_nodes: 10,
        enable_failover: true,
        // ... other configuration
    },
    // ... other configuration
};

let distributed_manager = DistributedManager::new(config).await?;
```

#### Cluster Management

- **Node Registration**: Automatic node registration and health monitoring
- **Leader Election**: Automatic leader election with failover support
- **Health Checking**: Comprehensive health checking for all nodes
- **Service Discovery**: Automatic service discovery and registration

#### Load Balancing

Multiple load balancing strategies:

- **Round Robin**: Simple round-robin distribution
- **Least Connections**: Route to node with fewest connections
- **Weighted Round Robin**: Round-robin with node weights
- **Least Response Time**: Route to fastest responding node
- **Random**: Random node selection
- **IP Hash**: Consistent hashing based on client IP

#### Service Registry

Service registration and discovery:

```rust
// Register a service
let service_info = ServiceInfo {
    name: "ai-service".to_string(),
    service_id: "ai-service-1".to_string(),
    address: "127.0.0.1:8080".parse().unwrap(),
    health_status: ServiceHealthStatus::Healthy,
    // ... other fields
};

distributed_manager.register_service(service_info).await?;

// Get service information
let service = distributed_manager.get_service_info("ai-service-1").await?;

// Select node for service
let node = distributed_manager.select_node_for_service("ai-service").await?;
```

### 3. Advanced Features

Advanced features provide message compression, encryption, and key management.

#### AdvancedFeaturesManager

Central manager for advanced features:

```rust
use rhema_ai::advanced_features::{AdvancedFeaturesManager, AdvancedFeaturesConfig};

let config = AdvancedFeaturesConfig {
    compression: CompressionConfig {
        algorithm: CompressionAlgorithm::Lz4,
        level: 6,
        threshold_bytes: 1024,
        enable_adaptive: true,
        // ... other configuration
    },
    key_management: KeyManagementConfig {
        rotation_policy: KeyRotationPolicy {
            enabled: true,
            interval_hours: 24 * 7, // 1 week
            method: KeyRotationMethod::Automatic,
            // ... other configuration
        },
        // ... other configuration
    },
    encryption: EncryptionConfig {
        algorithm: EncryptionAlgorithm::AES256,
        key_rotation_hours: 24 * 7, // 1 week
        enable_e2e_encryption: true,
        // ... other configuration
    },
    // ... other configuration
};

let advanced_manager = AdvancedFeaturesManager::new(config).await?;
```

#### Message Compression

Support for multiple compression algorithms:

- **LZ4**: Fast compression with good ratio
- **Gzip**: Standard compression with good ratio
- **Zstd**: High compression ratio with good speed
- **Snappy**: Very fast compression

```rust
// Compress message
let compressed = advanced_manager.compress_message(data).await?;

// Decompress message
let decompressed = advanced_manager.decompress_message(&compressed).await?;
```

#### Message Encryption

Support for multiple encryption algorithms:

- **AES-256**: Industry-standard encryption
- **ChaCha20**: High-performance encryption
- **XChaCha20**: Extended nonce ChaCha20

```rust
// Encrypt message
let encrypted = advanced_manager.encrypt_message(data).await?;

// Decrypt message
let decrypted = advanced_manager.decrypt_message(&encrypted).await?;
```

#### Key Management

Comprehensive key management with:

- **Automatic Key Rotation**: Configurable key rotation intervals
- **Key Backup**: Automatic key backup with encryption
- **Key Recovery**: Multiple recovery methods (Shamir's Secret Sharing, HSM, Cloud KMS)
- **Key Storage**: Multiple storage backends (File, Database, HSM, Cloud KMS)

```rust
// Rotate keys
advanced_manager.rotate_keys().await?;

// Backup keys
advanced_manager.backup_keys().await?;

// Restore keys from backup
advanced_manager.restore_keys(&backup_path).await?;

// Get key statistics
let key_stats = advanced_manager.get_key_stats().await?;
```

### 4. Production Configuration

The production configuration system provides comprehensive configuration for all components.

#### ProductionConfig

Complete configuration structure:

```rust
use rhema_ai::production_config::{ProductionConfig, ProductionAIService};

let config = ProductionConfig {
    ai_service: AIServiceConfig {
        api: ApiConfig {
            base_url: "http://localhost:8080".to_string(),
            timeout_seconds: 30,
            max_request_size_bytes: 10 * 1024 * 1024, // 10MB
            enable_cors: true,
            // ... other configuration
        },
        models: ModelConfig {
            default_model: "gpt-4".to_string(),
            available_models: vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()],
            enable_fallback: true,
            // ... other configuration
        },
        rate_limiting: RateLimitingConfig {
            enabled: true,
            rate_limit_per_minute: 60,
            rate_limit_per_hour: 1000,
            // ... other configuration
        },
        caching: CachingConfig {
            enabled: true,
            ttl_seconds: 3600, // 1 hour
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            // ... other configuration
        },
        agent_management: AgentManagementConfig {
            enabled: true,
            max_concurrent_agents: 100,
            agent_timeout_seconds: 300,
            // ... other configuration
        },
    },
    persistence: PersistenceConfig::default(),
    distributed: Some(DistributedConfig::default()),
    advanced_features: AdvancedFeaturesConfig::default(),
    coordination: CoordinationConfig {
        enabled: true,
        system: CoordinationConfig::default(),
        advanced: Some(AdvancedCoordinationConfig::default()),
        syneidesis: None,
    },
    monitoring: MonitoringConfig {
        enabled: true,
        metrics_interval_seconds: 60,
        storage_backend: MetricsStorageBackend::Memory,
        enable_health_checks: true,
        // ... other configuration
    },
    security: SecurityConfig {
        enable_authentication: false,
        authentication_method: AuthenticationMethod::ApiKey,
        enable_authorization: false,
        // ... other configuration
    },
    logging: LoggingConfig {
        level: LogLevel::Info,
        format: LogFormat::Json,
        output: LogOutput::Console,
        enable_structured_logging: true,
        // ... other configuration
    },
};

let service = ProductionAIService::new(config).await?;
```

#### Configuration Features

- **AI Service Configuration**: API settings, model configuration, rate limiting, caching, agent management
- **Persistence Configuration**: Storage backend selection, backup settings, data retention
- **Distributed Configuration**: Node settings, cluster configuration, service discovery, load balancing
- **Advanced Features Configuration**: Compression, encryption, key management settings
- **Coordination Configuration**: Real-time coordination system settings
- **Monitoring Configuration**: Metrics collection, health checking, alerting
- **Security Configuration**: Authentication, authorization, audit logging
- **Logging Configuration**: Log levels, formats, outputs

### 5. Health Checking and Monitoring

Comprehensive health checking and monitoring capabilities.

#### Health Checking

```rust
// Get service health
let health = service.health_check().await?;

println!("Service Status: {:?}", health.status);
for (component, component_health) in &health.components {
    println!("  {}: {:?}", component, component_health);
}
```

#### Service Statistics

```rust
// Get service statistics
let stats = service.get_stats().await?;

if let Some(persistence_stats) = &stats.persistence_stats {
    println!("Persistence Stats:");
    println!("  Total Entries: {}", persistence_stats.total_entries);
    println!("  Size: {} bytes", persistence_stats.total_size_bytes);
}

if let Some(distributed_stats) = &stats.distributed_stats {
    println!("Distributed Stats:");
    println!("  Nodes: {}", distributed_stats.node_count);
    println!("  Services: {}", distributed_stats.service_count);
    println!("  Cluster Status: {:?}", distributed_stats.cluster_health.status);
}

if let Some(advanced_stats) = &stats.advanced_features_stats {
    println!("Advanced Features Stats:");
    println!("  Performance Metrics: {}", advanced_stats.performance_metrics.len());
    println!("  Performance Alerts: {}", advanced_stats.performance_alerts.len());
    println!("  Active Keys: {}", advanced_stats.key_stats.active_keys);
}
```

#### Performance Monitoring

- **Compression Metrics**: Compression ratios, processing times
- **Encryption Metrics**: Encryption overhead, processing times
- **Key Management Metrics**: Key rotation times, key statistics
- **Performance Alerts**: Configurable thresholds and alerting

## Usage Example

See `examples/production_integration_example.rs` for a complete example demonstrating all production integration features.

## Deployment

### Single Node Deployment

```rust
let config = ProductionConfig::default();
let service = ProductionAIService::new(config).await?;
service.start().await?;
```

### Multi-Node Deployment

```rust
let config = ProductionConfig {
    distributed: Some(DistributedConfig {
        node: NodeConfig {
            node_id: format!("node-{}", node_id),
            name: format!("rhema-node-{}", node_id),
            address: format!("127.0.0.1:{}", 8080 + node_id).parse().unwrap(),
            role: NodeRole::Worker,
            // ... other configuration
        },
        // ... other configuration
    }),
    // ... other configuration
};

let service = ProductionAIService::new(config).await?;
service.start().await?;
```

### Configuration Files

Production configurations can be loaded from files:

```rust
use std::fs;

let config_content = fs::read_to_string("config.json")?;
let config: ProductionConfig = serde_json::from_str(&config_content)?;
let service = ProductionAIService::new(config).await?;
```

## Best Practices

1. **Persistence**: Use appropriate storage backends for your environment (PostgreSQL for production, SQLite for development)
2. **Security**: Enable authentication and authorization for production deployments
3. **Monitoring**: Configure comprehensive monitoring and alerting
4. **Backup**: Enable automatic backups for persistence data
5. **Key Management**: Use secure key storage (HSM or Cloud KMS) for production
6. **Load Balancing**: Configure appropriate load balancing strategies for your workload
7. **Health Checking**: Set up external health checking for production deployments

## Troubleshooting

### Common Issues

1. **Persistence Errors**: Check storage permissions and disk space
2. **Distributed Issues**: Verify network connectivity and cluster configuration
3. **Performance Issues**: Monitor compression and encryption overhead
4. **Key Management Issues**: Verify key storage configuration and permissions

### Debugging

Enable debug logging for troubleshooting:

```rust
let config = ProductionConfig {
    logging: LoggingConfig {
        level: LogLevel::Debug,
        // ... other configuration
    },
    // ... other configuration
};
```

## Future Enhancements

- **Database Backends**: Additional database support (MongoDB, Cassandra)
- **Cloud Integration**: Native cloud service integration (AWS, Azure, GCP)
- **Advanced Monitoring**: Integration with monitoring systems (Prometheus, Grafana)
- **Security Enhancements**: Additional authentication methods and security features
- **Performance Optimization**: Advanced performance tuning and optimization features 