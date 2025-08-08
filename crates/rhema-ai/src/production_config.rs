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

use crate::persistence::{PersistenceConfig, PersistenceManager};
use crate::distributed::{DistributedConfig, DistributedManager};
use crate::advanced_features::{AdvancedFeaturesConfig, AdvancedFeaturesManager};
use crate::agent::real_time_coordination::RealTimeCoordinationSystem;
use crate::coordination_integration::CoordinationIntegration;
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Production configuration for the complete AI service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// AI Service configuration
    pub ai_service: AIServiceConfig,
    /// Persistence configuration
    pub persistence: PersistenceConfig,
    /// Distributed deployment configuration
    pub distributed: Option<DistributedConfig>,
    /// Advanced features configuration
    pub advanced_features: AdvancedFeaturesConfig,
    /// Coordination configuration
    pub coordination: CoordinationConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// AI Service configuration for production
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    /// API configuration
    pub api: ApiConfig,
    /// Model configuration
    pub models: ModelConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
    /// Caching configuration
    pub caching: CachingConfig,
    /// Agent management configuration
    pub agent_management: AgentManagementConfig,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// API base URL
    pub base_url: String,
    /// API timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum request size in bytes
    pub max_request_size_bytes: u64,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS origins
    pub cors_origins: Vec<String>,
    /// Enable request logging
    pub enable_request_logging: bool,
    /// Enable response logging
    pub enable_response_logging: bool,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Default model
    pub default_model: String,
    /// Available models
    pub available_models: Vec<String>,
    /// Model configuration overrides
    pub model_overrides: HashMap<String, ModelOverride>,
    /// Enable model fallback
    pub enable_fallback: bool,
    /// Fallback model
    pub fallback_model: String,
}

/// Model override configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOverride {
    /// Model version
    pub version: String,
    /// Maximum tokens
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
    /// Cost per token
    pub cost_per_token: f64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Rate limit per minute
    pub rate_limit_per_minute: u32,
    /// Rate limit per hour
    pub rate_limit_per_hour: u32,
    /// Rate limit per day
    pub rate_limit_per_day: u32,
    /// Burst limit
    pub burst_limit: u32,
    /// Rate limit storage backend
    pub storage_backend: RateLimitStorageBackend,
}

/// Rate limit storage backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitStorageBackend {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis,
    /// Database storage
    Database,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Maximum cache size in bytes
    pub max_size_bytes: u64,
    /// Cache storage backend
    pub storage_backend: CacheStorageBackend,
    /// Enable cache warming
    pub enable_warming: bool,
    /// Cache warming interval in seconds
    pub warming_interval_seconds: u64,
}

/// Cache storage backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStorageBackend {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis,
    /// File-based storage
    File,
}

/// Agent management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentManagementConfig {
    /// Enable agent state management
    pub enabled: bool,
    /// Maximum concurrent agents
    pub max_concurrent_agents: usize,
    /// Agent timeout in seconds
    pub agent_timeout_seconds: u64,
    /// Agent heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,
    /// Enable agent persistence
    pub enable_persistence: bool,
}

/// Coordination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Enable coordination
    pub enabled: bool,
    /// Coordination system configuration
    pub system: crate::agent::real_time_coordination::CoordinationConfig,
    /// Advanced coordination configuration
    pub advanced: Option<crate::agent::real_time_coordination::AdvancedCoordinationConfig>,
    /// Syneidesis integration configuration
    pub syneidesis: Option<crate::grpc::coordination_client::SyneidesisConfig>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
    /// Metrics storage backend
    pub storage_backend: MetricsStorageBackend,
    /// Enable health checks
    pub enable_health_checks: bool,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
    /// Enable alerting
    pub enable_alerting: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Metrics storage backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsStorageBackend {
    /// In-memory storage
    Memory,
    /// Prometheus
    Prometheus,
    /// InfluxDB
    InfluxDb,
    /// Custom
    Custom(String),
}

/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Maximum response time in milliseconds
    pub max_response_time_ms: u64,
    /// Maximum error rate percentage
    pub max_error_rate_percent: f64,
    /// Maximum memory usage percentage
    pub max_memory_usage_percent: f64,
    /// Maximum CPU usage percentage
    pub max_cpu_usage_percent: f64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub enable_authentication: bool,
    /// Authentication method
    pub authentication_method: AuthenticationMethod,
    /// Enable authorization
    pub enable_authorization: bool,
    /// Authorization rules
    pub authorization_rules: Vec<AuthorizationRule>,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention days
    pub audit_log_retention_days: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_authentication: false,
            authentication_method: AuthenticationMethod::ApiKey,
            enable_authorization: false,
            authorization_rules: Vec::new(),
            enable_audit_logging: false,
            audit_log_retention_days: 30,
        }
    }
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// API key authentication
    ApiKey,
    /// JWT authentication
    Jwt,
    /// OAuth2 authentication
    OAuth2,
    /// Custom authentication
    Custom(String),
}

/// Authorization rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRule {
    /// Rule name
    pub name: String,
    /// Resource pattern
    pub resource_pattern: String,
    /// Allowed actions
    pub allowed_actions: Vec<String>,
    /// Allowed roles
    pub allowed_roles: Vec<String>,
    /// Allowed users
    pub allowed_users: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log format
    pub format: LogFormat,
    /// Log output
    pub output: LogOutput,
    /// Log file path (for file output)
    pub file_path: Option<PathBuf>,
    /// Enable structured logging
    pub enable_structured_logging: bool,
    /// Enable request ID tracking
    pub enable_request_id_tracking: bool,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Log formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    /// JSON format
    Json,
    /// Text format
    Text,
    /// Compact format
    Compact,
}

/// Log outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    /// Console output
    Console,
    /// File output
    File,
    /// Syslog output
    Syslog,
    /// Custom output
    Custom(String),
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
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
                    model_overrides: HashMap::new(),
                    enable_fallback: true,
                    fallback_model: "gpt-3.5-turbo".to_string(),
                },
                rate_limiting: RateLimitingConfig {
                    enabled: true,
                    rate_limit_per_minute: 60,
                    rate_limit_per_hour: 1000,
                    rate_limit_per_day: 10000,
                    burst_limit: 10,
                    storage_backend: RateLimitStorageBackend::Memory,
                },
                caching: CachingConfig {
                    enabled: true,
                    ttl_seconds: 3600, // 1 hour
                    max_size_bytes: 100 * 1024 * 1024, // 100MB
                    storage_backend: CacheStorageBackend::Memory,
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
            persistence: PersistenceConfig::default(),
            distributed: None,
            advanced_features: AdvancedFeaturesConfig::default(),
            coordination: CoordinationConfig {
                enabled: true,
                system: crate::agent::real_time_coordination::CoordinationConfig::default(),
                advanced: None,
                syneidesis: None,
            },
            monitoring: MonitoringConfig {
                enabled: true,
                metrics_interval_seconds: 60,
                storage_backend: MetricsStorageBackend::Memory,
                enable_health_checks: true,
                health_check_interval_seconds: 30,
                enable_alerting: true,
                alert_thresholds: AlertThresholds {
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
        }
    }
}

/// Production AI Service with all components integrated
pub struct ProductionAIService {
    config: ProductionConfig,
    persistence_manager: Option<PersistenceManager>,
    distributed_manager: Option<DistributedManager>,
    advanced_features_manager: Option<AdvancedFeaturesManager>,
    coordination_system: Option<Arc<RealTimeCoordinationSystem>>,
    coordination_integration: Option<Arc<CoordinationIntegration>>,
    // TODO: Add other components as they are implemented
}

impl ProductionAIService {
    /// Create a new production AI service
    pub async fn new(config: ProductionConfig) -> RhemaResult<Self> {
        info!("Initializing Production AI Service with configuration: {:?}", config);

        // Initialize persistence manager
        let persistence_manager = if config.persistence.backend != crate::persistence::StorageBackend::Memory {
            Some(PersistenceManager::new(config.persistence.clone()).await?)
        } else {
            None
        };

        // Initialize distributed manager
        let distributed_manager = if let Some(distributed_config) = &config.distributed {
            Some(DistributedManager::new(distributed_config.clone()).await?)
        } else {
            None
        };

        // Initialize advanced features manager
        let advanced_features_manager = Some(AdvancedFeaturesManager::new(config.advanced_features.clone()).await?);

        // Initialize coordination system
        let coordination_system = if config.coordination.enabled {
            let mut system = RealTimeCoordinationSystem::new();
            
            if let Some(advanced_config) = &config.coordination.advanced {
                system.enable_advanced_features(advanced_config.clone()).await?;
            }
            
            Some(Arc::new(system))
        } else {
            None
        };

        // Initialize coordination integration
        let coordination_integration = if let Some(system) = &coordination_system {
            let integration_config = crate::coordination_integration::CoordinationConfig {
                run_local_server: true,
                server_address: None,
                auto_register_agents: true,
                sync_messages: true,
                sync_tasks: true,
                enable_health_monitoring: true,
                syneidesis: config.coordination.syneidesis.clone(),
            };
            
            Some(Arc::new(CoordinationIntegration::new(
                system.as_ref().clone(),
                Some(integration_config),
            ).await?))
        } else {
            None
        };

        Ok(Self {
            config,
            persistence_manager,
            distributed_manager,
            advanced_features_manager,
            coordination_system,
            coordination_integration,
        })
    }

    /// Start the production AI service
    pub async fn start(&self) -> RhemaResult<()> {
        info!("Starting Production AI Service");

        // Start distributed manager if available
        if let Some(distributed_manager) = &self.distributed_manager {
            distributed_manager.start().await?;
        }

        // Start coordination system if available
        if let Some(coordination_system) = &self.coordination_system {
            // Start heartbeat monitoring
            coordination_system.start_heartbeat_monitoring().await;
        }

        info!("Production AI Service started successfully");
        Ok(())
    }

    /// Stop the production AI service
    pub async fn stop(&self) -> RhemaResult<()> {
        info!("Stopping Production AI Service");

        // Stop distributed manager if available
        if let Some(distributed_manager) = &self.distributed_manager {
            distributed_manager.stop().await?;
        }

        // Stop coordination system if available
        if let Some(coordination_system) = &self.coordination_system {
            // Stop heartbeat monitoring
            // TODO: Implement stop method for coordination system
        }

        info!("Production AI Service stopped successfully");
        Ok(())
    }

    /// Get service health
    pub async fn health_check(&self) -> RhemaResult<ServiceHealth> {
        let mut health = ServiceHealth {
            status: ServiceStatus::Healthy,
            components: HashMap::new(),
            timestamp: chrono::Utc::now(),
        };

        // Check persistence health
        if let Some(persistence_manager) = &self.persistence_manager {
            match persistence_manager.validate().await {
                Ok(_) => {
                    health.components.insert("persistence".to_string(), ComponentHealth::Healthy);
                }
                Err(e) => {
                    health.components.insert("persistence".to_string(), ComponentHealth::Unhealthy(e.to_string()));
                    health.status = ServiceStatus::Degraded;
                }
            }
        }

        // Check distributed manager health
        if let Some(distributed_manager) = &self.distributed_manager {
            match distributed_manager.get_cluster_health().await {
                Ok(cluster_health) => {
                    let component_health = match cluster_health.status {
                        crate::distributed::ClusterStatus::Healthy => ComponentHealth::Healthy,
                        crate::distributed::ClusterStatus::Degraded => ComponentHealth::Degraded,
                        crate::distributed::ClusterStatus::Unhealthy => ComponentHealth::Unhealthy("Cluster unhealthy".to_string()),
                        crate::distributed::ClusterStatus::Forming => ComponentHealth::Degraded,
                    };
                    health.components.insert("distributed".to_string(), component_health);
                    
                    if cluster_health.status == crate::distributed::ClusterStatus::Unhealthy {
                        health.status = ServiceStatus::Unhealthy;
                    }
                }
                Err(e) => {
                    health.components.insert("distributed".to_string(), ComponentHealth::Unhealthy(e.to_string()));
                    health.status = ServiceStatus::Degraded;
                }
            }
        }

        // Check advanced features health
        if let Some(advanced_features_manager) = &self.advanced_features_manager {
            let alerts = advanced_features_manager.get_performance_alerts();
            let critical_alerts = alerts.iter().filter(|a| matches!(a.severity, crate::advanced_features::AlertSeverity::Critical)).count();
            
            if critical_alerts > 0 {
                health.components.insert("advanced_features".to_string(), ComponentHealth::Unhealthy(format!("{} critical alerts", critical_alerts)));
                health.status = ServiceStatus::Degraded;
            } else {
                health.components.insert("advanced_features".to_string(), ComponentHealth::Healthy);
            }
        }

        Ok(health)
    }

    /// Get service statistics
    pub async fn get_stats(&self) -> RhemaResult<ServiceStats> {
        let mut stats = ServiceStats {
            persistence_stats: None,
            distributed_stats: None,
            advanced_features_stats: None,
            coordination_stats: None,
            timestamp: chrono::Utc::now(),
        };

        // Get persistence statistics
        if let Some(persistence_manager) = &self.persistence_manager {
            stats.persistence_stats = Some(persistence_manager.get_stats().await?);
        }

        // Get distributed statistics
        if let Some(distributed_manager) = &self.distributed_manager {
            stats.distributed_stats = Some(DistributedStats {
                cluster_health: distributed_manager.get_cluster_health().await?,
                node_count: distributed_manager.get_all_nodes().await?.len(),
                service_count: distributed_manager.get_all_services().await?.len(),
            });
        }

        // Get advanced features statistics
        if let Some(advanced_features_manager) = &self.advanced_features_manager {
            stats.advanced_features_stats = Some(AdvancedFeaturesStats {
                performance_metrics: advanced_features_manager.get_performance_metrics(),
                performance_alerts: advanced_features_manager.get_performance_alerts(),
                key_stats: advanced_features_manager.get_key_stats().await?,
            });
        }

        // Get coordination statistics
        if let Some(coordination_system) = &self.coordination_system {
            stats.coordination_stats = Some(coordination_system.get_stats());
        }

        Ok(stats)
    }
}

/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: ServiceStatus,
    pub components: HashMap<String, ComponentHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentHealth {
    Healthy,
    Degraded,
    Unhealthy(String),
}

/// Service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    pub persistence_stats: Option<crate::persistence::StorageStats>,
    pub distributed_stats: Option<DistributedStats>,
    pub advanced_features_stats: Option<AdvancedFeaturesStats>,
    pub coordination_stats: Option<crate::agent::real_time_coordination::CoordinationStats>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Distributed statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStats {
    pub cluster_health: crate::distributed::ClusterHealth,
    pub node_count: usize,
    pub service_count: usize,
}

/// Advanced features statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeaturesStats {
    pub performance_metrics: Vec<crate::advanced_features::PerformanceMetric>,
    pub performance_alerts: Vec<crate::advanced_features::PerformanceAlert>,
    pub key_stats: crate::advanced_features::key_management::KeyStats,
} 