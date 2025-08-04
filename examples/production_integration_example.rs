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

use rhema_ai::{
    ProductionAIService, ProductionConfig, PersistenceConfig, DistributedConfig,
    AdvancedFeaturesConfig, CoordinationConfig, MonitoringConfig, SecurityConfig, LoggingConfig,
    AIServiceConfig, ApiConfig, ModelConfig, RateLimitingConfig, CachingConfig, AgentManagementConfig,
    persistence::{StorageBackend, PersistenceManager},
    distributed::{NodeRole, DiscoveryMethod, LoadBalancingStrategy, RegistryBackend},
    advanced_features::{CompressionAlgorithm, EncryptionAlgorithm, KeyRotationMethod, KeyStorage},
    production_config::{LogLevel, LogFormat, LogOutput, AuthenticationMethod, MetricsStorageBackend},
};
use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Rhema AI Production Integration Example");
    println!("===========================================");

    // Create production configuration
    let config = create_production_config()?;

    // Create production AI service
    let mut service = ProductionAIService::new(config).await?;

    // Start the service
    service.start().await?;

    // Demonstrate health checking
    println!("\n📊 Health Check:");
    let health = service.health_check().await?;
    println!("Service Status: {:?}", health.status);
    for (component, component_health) in &health.components {
        println!("  {}: {:?}", component, component_health);
    }

    // Demonstrate statistics
    println!("\n📈 Service Statistics:");
    let stats = service.get_stats().await?;
    println!("Timestamp: {}", stats.timestamp);

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

    if let Some(coordination_stats) = &stats.coordination_stats {
        println!("Coordination Stats:");
        println!("  Total Messages: {}", coordination_stats.total_messages);
        println!("  Active Agents: {}", coordination_stats.active_agents);
        println!("  Active Sessions: {}", coordination_stats.active_sessions);
    }

    // Demonstrate persistence operations
    println!("\n💾 Persistence Operations:");
    if let Some(persistence_manager) = service.persistence_manager.as_ref() {
        // Perform backup
        persistence_manager.backup().await?;
        println!("✅ Backup completed");

        // Perform cleanup
        persistence_manager.cleanup().await?;
        println!("✅ Cleanup completed");

        // Validate data
        persistence_manager.validate().await?;
        println!("✅ Data validation completed");

        // Get storage statistics
        let storage_stats = persistence_manager.get_stats().await?;
        println!("Storage Statistics:");
        println!("  Total Size: {} bytes", storage_stats.total_size_bytes);
        println!("  Session Entries: {}", storage_stats.session_stats.total_entries);
        println!("  Consensus Entries: {}", storage_stats.consensus_stats.total_entries);
        println!("  State Entries: {}", storage_stats.state_stats.total_entries);
    }

    // Demonstrate distributed operations
    println!("\n🌐 Distributed Operations:");
    if let Some(distributed_manager) = service.distributed_manager.as_ref() {
        // Get cluster health
        let cluster_health = distributed_manager.get_cluster_health().await?;
        println!("Cluster Health:");
        println!("  Status: {:?}", cluster_health.status);
        println!("  Total Nodes: {}", cluster_health.total_nodes);
        println!("  Online Nodes: {}", cluster_health.online_nodes);
        println!("  Leader: {:?}", cluster_health.leader);

        // Get all nodes
        let nodes = distributed_manager.get_all_nodes().await?;
        println!("Nodes:");
        for node in nodes {
            println!("  {} ({}) - {:?}", node.name, node.node_id, node.status);
        }

        // Get all services
        let services = distributed_manager.get_all_services().await?;
        println!("Services:");
        for service_info in services {
            println!("  {} ({}) - {:?}", service_info.name, service_info.service_id, service_info.health_status);
        }
    }

    // Demonstrate advanced features
    println!("\n🔧 Advanced Features:");
    if let Some(advanced_manager) = service.advanced_features_manager.as_ref() {
        // Test message compression
        let test_data = b"This is a test message that will be compressed and encrypted for demonstration purposes.";
        println!("Original data size: {} bytes", test_data.len());

        let compressed = advanced_manager.compress_message(test_data).await?;
        println!("Compressed data size: {} bytes", compressed.len());
        println!("Compression ratio: {:.2}%", (compressed.len() as f64 / test_data.len() as f64) * 100.0);

        let decompressed = advanced_manager.decompress_message(&compressed).await?;
        println!("Decompressed data size: {} bytes", decompressed.len());
        println!("Compression successful: {}", test_data == decompressed.as_slice());

        // Test message encryption
        let encrypted = advanced_manager.encrypt_message(test_data).await?;
        println!("Encrypted data size: {} bytes", encrypted.len());
        println!("Encryption overhead: {:.2}%", ((encrypted.len() - test_data.len()) as f64 / test_data.len() as f64) * 100.0);

        let decrypted = advanced_manager.decrypt_message(&encrypted).await?;
        println!("Decrypted data size: {} bytes", decrypted.len());
        println!("Encryption successful: {}", test_data == decrypted.as_slice());

        // Get performance metrics
        let metrics = advanced_manager.get_performance_metrics();
        println!("Performance Metrics:");
        for metric in metrics {
            println!("  {}: {} {}", metric.name, metric.value, metric.unit);
        }

        // Get performance alerts
        let alerts = advanced_manager.get_performance_alerts();
        if !alerts.is_empty() {
            println!("Performance Alerts:");
            for alert in alerts {
                println!("  [{}] {}: {}", alert.severity, alert.alert_type, alert.message);
            }
        } else {
            println!("No performance alerts");
        }

        // Get key statistics
        let key_stats = advanced_manager.get_key_stats().await?;
        println!("Key Statistics:");
        println!("  Total Keys: {}", key_stats.total_keys);
        println!("  Active Keys: {}", key_stats.active_keys);
        println!("  Expired Keys: {}", key_stats.expired_keys);
    }

    // Stop the service
    service.stop().await?;
    println!("\n✅ Production AI Service stopped successfully");

    Ok(())
}

fn create_production_config() -> rhema_core::RhemaResult<ProductionConfig> {
    let mut model_overrides = HashMap::new();
    model_overrides.insert(
        "gpt-4".to_string(),
        rhema_ai::production_config::ModelOverride {
            version: "gpt-4-turbo-preview".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            cost_per_token: 0.03,
        },
    );

    let config = ProductionConfig {
        ai_service: AIServiceConfig {
            api: ApiConfig {
                base_url: "http://localhost:8080".to_string(),
                timeout_seconds: 30,
                max_request_size_bytes: 10 * 1024 * 1024, // 10MB
                enable_cors: true,
                cors_origins: vec!["*".to_string()],
                enable_request_logging: true,
                enable_response_logging: false,
            },
            models: ModelConfig {
                default_model: "gpt-4".to_string(),
                available_models: vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()],
                model_overrides,
                enable_fallback: true,
                fallback_model: "gpt-3.5-turbo".to_string(),
            },
            rate_limiting: RateLimitingConfig {
                enabled: true,
                rate_limit_per_minute: 60,
                rate_limit_per_hour: 1000,
                rate_limit_per_day: 10000,
                burst_limit: 10,
                storage_backend: rhema_ai::production_config::RateLimitStorageBackend::Memory,
            },
            caching: CachingConfig {
                enabled: true,
                ttl_seconds: 3600, // 1 hour
                max_size_bytes: 100 * 1024 * 1024, // 100MB
                storage_backend: rhema_ai::production_config::CacheStorageBackend::Memory,
                enable_warming: false,
                warming_interval_seconds: 300,
            },
            agent_management: AgentManagementConfig {
                enabled: true,
                max_concurrent_agents: 100,
                agent_timeout_seconds: 300,
                heartbeat_interval_seconds: 30,
                enable_persistence: true,
            },
        },
        persistence: PersistenceConfig {
            backend: StorageBackend::File,
            storage_path: Some(PathBuf::from("./data")),
            connection_string: None,
            enable_backups: true,
            backup_interval_hours: 24,
            backup_retention_days: 30,
            enable_compression: true,
            enable_encryption: false,
            encryption_key: None,
            max_data_size_bytes: 100 * 1024 * 1024, // 100MB
            enable_validation: true,
            enable_cleanup: true,
            cleanup_interval_hours: 168, // 1 week
            data_retention_days: 90,
        },
        distributed: Some(DistributedConfig {
            node: rhema_ai::distributed::NodeConfig {
                node_id: uuid::Uuid::new_v4().to_string(),
                name: "rhema-node-1".to_string(),
                address: "127.0.0.1:8080".parse().unwrap(),
                role: NodeRole::Worker,
                capabilities: vec!["coordination".to_string(), "ai_service".to_string()],
                metadata: HashMap::new(),
            },
            cluster: rhema_ai::distributed::ClusterConfig {
                name: "rhema-cluster".to_string(),
                cluster_id: uuid::Uuid::new_v4().to_string(),
                min_nodes: 1,
                max_nodes: 10,
                node_timeout_seconds: 30,
                heartbeat_interval_seconds: 10,
                enable_failover: true,
                enable_leader_election: true,
                leader_election_timeout_seconds: 60,
            },
            discovery: rhema_ai::distributed::DiscoveryConfig {
                method: DiscoveryMethod::Multicast,
                multicast_address: Some("224.0.0.1:8081".parse().unwrap()),
                static_nodes: Vec::new(),
                discovery_interval_seconds: 30,
                discovery_timeout_seconds: 10,
            },
            load_balancing: rhema_ai::distributed::LoadBalancingConfig {
                strategy: LoadBalancingStrategy::RoundRobin,
                enable_health_aware_routing: true,
                enable_sticky_sessions: false,
                session_timeout_seconds: 300,
                max_retry_attempts: 3,
                retry_delay_ms: 1000,
            },
            health_checking: rhema_ai::distributed::HealthCheckingConfig {
                interval_seconds: 30,
                timeout_seconds: 5,
                failure_threshold: 3,
                success_threshold: 2,
                health_check_endpoint: "/health".to_string(),
                enable_tcp_checks: true,
                enable_http_checks: true,
            },
            service_registry: rhema_ai::distributed::ServiceRegistryConfig {
                backend: RegistryBackend::Consul,
                address: "http://localhost:8500".to_string(),
                credentials: None,
                registration_interval_seconds: 30,
                deregistration_timeout_seconds: 60,
            },
        }),
        advanced_features: AdvancedFeaturesConfig {
            compression: rhema_ai::advanced_features::CompressionConfig {
                algorithm: CompressionAlgorithm::Lz4,
                level: 6,
                threshold_bytes: 1024,
                enable_adaptive: true,
                enable_metrics: true,
            },
            key_management: rhema_ai::advanced_features::KeyManagementConfig {
                rotation_policy: rhema_ai::advanced_features::KeyRotationPolicy {
                    enabled: true,
                    interval_hours: 24 * 7, // 1 week
                    method: KeyRotationMethod::Automatic,
                    notification_enabled: true,
                },
                storage: KeyStorage::File(PathBuf::from("./keys")),
                backup: rhema_ai::advanced_features::KeyBackupConfig {
                    enabled: true,
                    location: PathBuf::from("./backups/keys"),
                    encryption_enabled: true,
                    frequency_hours: 24,
                    retention_days: 30,
                },
                recovery: rhema_ai::advanced_features::KeyRecoveryConfig {
                    enabled: true,
                    method: rhema_ai::advanced_features::KeyRecoveryMethod::ShamirSecretSharing,
                    verification_enabled: true,
                    timeout_seconds: 300,
                },
            },
            encryption: rhema_ai::advanced_features::EncryptionConfig {
                algorithm: EncryptionAlgorithm::AES256,
                key_rotation_hours: 24 * 7, // 1 week
                enable_e2e_encryption: true,
                certificate_path: None,
                private_key_path: None,
            },
            performance_monitoring: rhema_ai::advanced_features::PerformanceMonitoringConfig {
                enabled: true,
                metrics_interval_seconds: 60,
                thresholds: rhema_ai::advanced_features::PerformanceThresholds {
                    max_compression_ratio: 0.8,
                    max_encryption_overhead_percent: 10.0,
                    max_key_rotation_time_seconds: 60,
                    max_message_processing_time_ms: 1000,
                },
                enable_alerts: true,
            },
        },
        coordination: CoordinationConfig {
            enabled: true,
            system: rhema_ai::agent::real_time_coordination::CoordinationConfig::default(),
            advanced: Some(rhema_ai::agent::real_time_coordination::AdvancedCoordinationConfig::default()),
            syneidesis: None,
        },
        monitoring: MonitoringConfig {
            enabled: true,
            metrics_interval_seconds: 60,
            storage_backend: MetricsStorageBackend::Memory,
            enable_health_checks: true,
            health_check_interval_seconds: 30,
            enable_alerting: true,
            alert_thresholds: rhema_ai::production_config::AlertThresholds {
                max_response_time_ms: 5000,
                max_error_rate_percent: 5.0,
                max_memory_usage_percent: 80.0,
                max_cpu_usage_percent: 80.0,
            },
        },
        security: SecurityConfig {
            enable_authentication: false,
            authentication_method: AuthenticationMethod::ApiKey,
            enable_authorization: false,
            authorization_rules: Vec::new(),
            enable_audit_logging: false,
            audit_log_retention_days: 90,
        },
        logging: LoggingConfig {
            level: LogLevel::Info,
            format: LogFormat::Json,
            output: LogOutput::Console,
            file_path: None,
            enable_structured_logging: true,
            enable_request_id_tracking: true,
        },
    };

    Ok(config)
} 