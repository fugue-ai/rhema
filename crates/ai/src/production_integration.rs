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

use crate::ai_service::{AIService, AIServiceConfig, AIRequest, AIResponse};
use crate::coordination_integration::{CoordinationIntegration, CoordinationConfig};
use crate::agent::real_time_coordination::{
    AgentInfo, AgentMessage, AgentStatus, MessageType, MessagePriority,
    RealTimeCoordinationSystem
};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error, instrument};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Production-ready integration between AI service and coordination system
pub struct ProductionIntegration {
    /// AI service instance
    ai_service: Arc<AIService>,
    /// Coordination integration
    coordination: Arc<CoordinationIntegration>,
    /// Production configuration
    config: ProductionConfig,
    /// Health monitoring
    health_monitor: Arc<RwLock<HealthMonitor>>,
    /// Performance metrics
    metrics: Arc<RwLock<ProductionMetrics>>,
    /// Circuit breaker for fault tolerance
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    /// Load balancer for distributed coordination
    load_balancer: Arc<RwLock<LoadBalancer>>,
}

/// Production configuration for AI-Coordination integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Enable production mode features
    pub production_mode: bool,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Scaling configuration
    pub scaling: ScalingConfig,
    /// Security configuration
    pub security: SecurityConfig,
}

/// Circuit breaker configuration for fault tolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold before opening circuit
    pub failure_threshold: u32,
    /// Timeout for circuit breaker (seconds)
    pub timeout_seconds: u64,
    /// Success threshold to close circuit
    pub success_threshold: u32,
    /// Enable circuit breaker
    pub enabled: bool,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    /// Maximum concurrent requests per node
    pub max_concurrent_per_node: usize,
    /// Health check enabled
    pub health_check_enabled: bool,
    /// Enable load balancing
    pub enabled: bool,
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    ConsistentHashing,
    Adaptive,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Enable tracing
    pub enable_tracing: bool,
    /// Enable logging
    pub enable_logging: bool,
    /// Metrics export interval (seconds)
    pub metrics_export_interval: u64,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// High error rate threshold
    pub error_rate_threshold: f64,
    /// High latency threshold (ms)
    pub latency_threshold_ms: u64,
    /// High memory usage threshold
    pub memory_usage_threshold: f64,
    /// High CPU usage threshold
    pub cpu_usage_threshold: f64,
}

/// Scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Auto-scaling enabled
    pub auto_scaling_enabled: bool,
    /// Minimum instances
    pub min_instances: usize,
    /// Maximum instances
    pub max_instances: usize,
    /// Scale up threshold
    pub scale_up_threshold: f64,
    /// Scale down threshold
    pub scale_down_threshold: f64,
    /// Cooldown period (seconds)
    pub cooldown_seconds: u64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub enable_auth: bool,
    /// Enable authorization
    pub enable_authorization: bool,
    /// Rate limiting enabled
    pub rate_limiting_enabled: bool,
    /// Rate limit per minute
    pub rate_limit_per_minute: u32,
    /// Enable audit logging
    pub enable_audit_logging: bool,
}

/// Health monitoring for production integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitor {
    /// Last health check time
    pub last_check: DateTime<Utc>,
    /// Health status
    pub status: HealthStatus,
    /// Component health status
    pub components: HashMap<String, ComponentHealth>,
    /// Error count
    pub error_count: u32,
    /// Success count
    pub success_count: u32,
}

/// Health status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Health status
    pub status: HealthStatus,
    /// Last check time
    pub last_check: DateTime<Utc>,
    /// Response time (ms)
    pub response_time_ms: u64,
    /// Error message if any
    pub error_message: Option<String>,
}

/// Circuit breaker for fault tolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Current state
    pub state: CircuitBreakerState,
    /// Failure count
    pub failure_count: u32,
    /// Success count
    pub success_count: u32,
    /// Last failure time
    pub last_failure: Option<DateTime<Utc>>,
    /// Last success time
    pub last_success: Option<DateTime<Utc>>,
    /// Configuration
    pub config: CircuitBreakerConfig,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Load balancer for distributed coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    /// Available nodes
    pub nodes: Vec<NodeInfo>,
    /// Current strategy
    pub strategy: LoadBalancingStrategy,
    /// Current node index (for round robin)
    pub current_index: usize,
    /// Node health status
    pub node_health: HashMap<String, NodeHealth>,
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node ID
    pub id: String,
    /// Node address
    pub address: String,
    /// Node weight (for weighted strategies)
    pub weight: f64,
    /// Current load
    pub current_load: usize,
    /// Maximum capacity
    pub max_capacity: usize,
}

/// Node health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealth {
    /// Node ID
    pub node_id: String,
    /// Health status
    pub status: HealthStatus,
    /// Last check time
    pub last_check: DateTime<Utc>,
    /// Response time (ms)
    pub response_time_ms: u64,
    /// Error rate
    pub error_rate: f64,
}

/// Production metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time (ms)
    pub avg_response_time_ms: u64,
    /// Circuit breaker trips
    pub circuit_breaker_trips: u64,
    /// Load balancer requests
    pub load_balancer_requests: u64,
    /// Health check failures
    pub health_check_failures: u64,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

impl ProductionIntegration {
    /// Create a new production integration
    #[instrument(skip(ai_service, coordination))]
    pub async fn new(
        ai_service: AIService,
        coordination: CoordinationIntegration,
        config: ProductionConfig,
    ) -> RhemaResult<Self> {
        info!("Initializing production integration with config: {:?}", config);

        let ai_service = Arc::new(ai_service);
        let coordination = Arc::new(coordination);

        let health_monitor = Arc::new(RwLock::new(HealthMonitor {
            last_check: Utc::now(),
            status: HealthStatus::Unknown,
            components: HashMap::new(),
            error_count: 0,
            success_count: 0,
        }));

        let metrics = Arc::new(RwLock::new(ProductionMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0,
            circuit_breaker_trips: 0,
            load_balancer_requests: 0,
            health_check_failures: 0,
            last_updated: Utc::now(),
        }));

        let circuit_breaker = Arc::new(RwLock::new(CircuitBreaker {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure: None,
            last_success: None,
            config: config.circuit_breaker.clone(),
        }));

        let load_balancer = Arc::new(RwLock::new(LoadBalancer {
            nodes: Vec::new(),
            strategy: config.load_balancing.strategy.clone(),
            current_index: 0,
            node_health: HashMap::new(),
        }));

        let integration = Self {
            ai_service,
            coordination,
            config,
            health_monitor,
            metrics,
            circuit_breaker,
            load_balancer,
        };

        // Start health monitoring
        if integration.config.monitoring.enable_metrics {
            integration.start_health_monitoring().await?;
        }

        info!("✅ Production integration initialized successfully");
        Ok(integration)
    }

    /// Process AI request with production features
    #[instrument(skip(self, request))]
    pub async fn process_ai_request(&self, request: AIRequest) -> RhemaResult<AIResponse> {
        let start_time = std::time::Instant::now();

        // Check circuit breaker
        if !self.check_circuit_breaker().await? {
            return Err(rhema_core::RhemaError::ServiceUnavailable(
                "Circuit breaker is open".to_string()
            ));
        }

        // Load balancing (if enabled)
        let target_node = if self.config.load_balancing.enabled {
            self.select_node().await?
        } else {
            None
        };

        // Process request
        let result = match self.ai_service.process_request(request).await {
            Ok(response) => {
                self.record_success().await;
                Ok(response)
            }
            Err(e) => {
                self.record_failure().await;
                Err(e)
            }
        };

        // Update metrics
        let duration = start_time.elapsed();
        self.update_metrics(duration.as_millis() as u64, result.is_ok()).await;

        result
    }

    /// Send coordination message with production features
    #[instrument(skip(self, message))]
    pub async fn send_coordination_message(&self, message: AgentMessage) -> RhemaResult<()> {
        // Check circuit breaker
        if !self.check_circuit_breaker().await? {
            return Err(rhema_core::RhemaError::ServiceUnavailable(
                "Circuit breaker is open".to_string()
            ));
        }

        // Load balancing (if enabled)
        if self.config.load_balancing.enabled {
            self.select_node().await?;
        }

        // Send message
        match self.coordination.send_message_with_coordination(message).await {
            Ok(()) => {
                self.record_success().await;
                Ok(())
            }
            Err(e) => {
                self.record_failure().await;
                Err(e)
            }
        }
    }

    /// Register agent with production features
    #[instrument(skip(self, agent_info))]
    pub async fn register_agent(&self, agent_info: AgentInfo) -> RhemaResult<()> {
        // Check circuit breaker
        if !self.check_circuit_breaker().await? {
            return Err(rhema_core::RhemaError::ServiceUnavailable(
                "Circuit breaker is open".to_string()
            ));
        }

        // Register with both AI service and coordination
        let ai_result = self.ai_service.register_agent(agent_info.id.clone()).await;
        let coord_result = self.coordination.register_rhema_agent(&agent_info).await;

        match (ai_result, coord_result) {
            (Ok(_), Ok(_)) => {
                self.record_success().await;
                Ok(())
            }
            (Err(e), _) | (_, Err(e)) => {
                self.record_failure().await;
                Err(e)
            }
        }
    }

    /// Get production health status
    pub async fn get_health_status(&self) -> HealthStatus {
        let monitor = self.health_monitor.read().await;
        monitor.status.clone()
    }

    /// Get production metrics
    pub async fn get_metrics(&self) -> ProductionMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Start health monitoring
    async fn start_health_monitoring(&self) -> RhemaResult<()> {
        let interval = self.config.health_check_interval;
        let health_monitor = self.health_monitor.clone();
        let ai_service = self.ai_service.clone();
        let coordination = self.coordination.clone();

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(
                std::time::Duration::from_secs(interval)
            );

            loop {
                interval_timer.tick().await;
                
                let mut monitor = health_monitor.write().await;
                monitor.last_check = Utc::now();

                // Check AI service health
                let ai_health = match ai_service.health_check().await {
                    Ok(_) => ComponentHealth {
                        name: "ai_service".to_string(),
                        status: HealthStatus::Healthy,
                        last_check: Utc::now(),
                        response_time_ms: 0,
                        error_message: None,
                    },
                    Err(e) => ComponentHealth {
                        name: "ai_service".to_string(),
                        status: HealthStatus::Unhealthy,
                        last_check: Utc::now(),
                        response_time_ms: 0,
                        error_message: Some(e.to_string()),
                    }
                };

                // Check coordination health
                let coord_health = ComponentHealth {
                    name: "coordination".to_string(),
                    status: HealthStatus::Healthy, // Simplified for now
                    last_check: Utc::now(),
                    response_time_ms: 0,
                    error_message: None,
                };

                monitor.components.insert("ai_service".to_string(), ai_health);
                monitor.components.insert("coordination".to_string(), coord_health);

                // Update overall health status
                let unhealthy_components = monitor.components.values()
                    .filter(|c| c.status == HealthStatus::Unhealthy)
                    .count();

                monitor.status = if unhealthy_components == 0 {
                    HealthStatus::Healthy
                } else if unhealthy_components < monitor.components.len() {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Unhealthy
                };
            }
        });

        Ok(())
    }

    /// Check circuit breaker status
    async fn check_circuit_breaker(&self) -> RhemaResult<bool> {
        let mut breaker = self.circuit_breaker.write().await;
        
        match breaker.state {
            CircuitBreakerState::Closed => Ok(true),
            CircuitBreakerState::Open => {
                if let Some(last_failure) = breaker.last_failure {
                    let timeout = std::time::Duration::from_secs(breaker.config.timeout_seconds);
                    if Utc::now() - last_failure > chrono::Duration::from_std(timeout).unwrap() {
                        breaker.state = CircuitBreakerState::HalfOpen;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            CircuitBreakerState::HalfOpen => Ok(true),
        }
    }

    /// Record successful operation
    async fn record_success(&self) {
        let mut breaker = self.circuit_breaker.write().await;
        breaker.success_count += 1;
        breaker.last_success = Some(Utc::now());

        if breaker.state == CircuitBreakerState::HalfOpen {
            if breaker.success_count >= breaker.config.success_threshold {
                breaker.state = CircuitBreakerState::Closed;
                breaker.failure_count = 0;
            }
        }
    }

    /// Record failed operation
    async fn record_failure(&self) {
        let mut breaker = self.circuit_breaker.write().await;
        breaker.failure_count += 1;
        breaker.last_failure = Some(Utc::now());

        if breaker.failure_count >= breaker.config.failure_threshold {
            breaker.state = CircuitBreakerState::Open;
            let mut metrics = self.metrics.write().await;
            metrics.circuit_breaker_trips += 1;
        }
    }

    /// Select node for load balancing
    async fn select_node(&self) -> RhemaResult<Option<String>> {
        let mut balancer = self.load_balancer.write().await;
        
        if balancer.nodes.is_empty() {
            return Ok(None);
        }

        let node_id = match balancer.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let node_id = balancer.nodes[balancer.current_index].id.clone();
                balancer.current_index = (balancer.current_index + 1) % balancer.nodes.len();
                node_id
            }
            LoadBalancingStrategy::LeastConnections => {
                balancer.nodes.iter()
                    .min_by_key(|node| node.current_load)
                    .map(|node| node.id.clone())
                    .unwrap_or_default()
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                // Simplified weighted round robin
                let node_id = balancer.nodes[balancer.current_index].id.clone();
                balancer.current_index = (balancer.current_index + 1) % balancer.nodes.len();
                node_id
            }
            LoadBalancingStrategy::ConsistentHashing => {
                // Simplified consistent hashing
                balancer.nodes[0].id.clone()
            }
            LoadBalancingStrategy::Adaptive => {
                // Simplified adaptive - use least connections
                balancer.nodes.iter()
                    .min_by_key(|node| node.current_load)
                    .map(|node| node.id.clone())
                    .unwrap_or_default()
            }
        };

        let mut metrics = self.metrics.write().await;
        metrics.load_balancer_requests += 1;

        Ok(Some(node_id))
    }

    /// Update production metrics
    async fn update_metrics(&self, response_time_ms: u64, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        metrics.last_updated = Utc::now();

        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }

        // Update average response time
        let total_time = metrics.avg_response_time_ms * (metrics.total_requests - 1) + response_time_ms;
        metrics.avg_response_time_ms = total_time / metrics.total_requests;
    }

    /// Add node to load balancer
    pub async fn add_node(&self, node: NodeInfo) {
        let mut balancer = self.load_balancer.write().await;
        balancer.nodes.push(node);
    }

    /// Remove node from load balancer
    pub async fn remove_node(&self, node_id: &str) {
        let mut balancer = self.load_balancer.write().await;
        balancer.nodes.retain(|node| node.id != node_id);
    }

    /// Get circuit breaker status
    pub async fn get_circuit_breaker_status(&self) -> CircuitBreakerState {
        let breaker = self.circuit_breaker.read().await;
        breaker.state.clone()
    }

    /// Reset circuit breaker
    pub async fn reset_circuit_breaker(&self) {
        let mut breaker = self.circuit_breaker.write().await;
        breaker.state = CircuitBreakerState::Closed;
        breaker.failure_count = 0;
        breaker.success_count = 0;
    }
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            production_mode: true,
            health_check_interval: 30,
            circuit_breaker: CircuitBreakerConfig {
                failure_threshold: 5,
                timeout_seconds: 60,
                success_threshold: 2,
                enabled: true,
            },
            load_balancing: LoadBalancingConfig {
                strategy: LoadBalancingStrategy::RoundRobin,
                max_concurrent_per_node: 100,
                health_check_enabled: true,
                enabled: false,
            },
            monitoring: MonitoringConfig {
                enable_metrics: true,
                enable_tracing: true,
                enable_logging: true,
                metrics_export_interval: 60,
                alert_thresholds: AlertThresholds {
                    error_rate_threshold: 0.1,
                    latency_threshold_ms: 1000,
                    memory_usage_threshold: 0.8,
                    cpu_usage_threshold: 0.8,
                },
            },
            scaling: ScalingConfig {
                auto_scaling_enabled: false,
                min_instances: 1,
                max_instances: 10,
                scale_up_threshold: 0.7,
                scale_down_threshold: 0.3,
                cooldown_seconds: 300,
            },
            security: SecurityConfig {
                enable_auth: false,
                enable_authorization: false,
                rate_limiting_enabled: true,
                rate_limit_per_minute: 1000,
                enable_audit_logging: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_service::AIServiceConfig;

    #[tokio::test]
    async fn test_production_integration_creation() {
        let ai_config = AIServiceConfig {
            api_key: "test".to_string(),
            base_url: "http://localhost:8080".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 10,
            rate_limit_per_minute: 100,
            cache_ttl_seconds: 300,
            model_version: "test".to_string(),
            enable_caching: true,
            enable_rate_limiting: true,
            enable_monitoring: true,
            enable_lock_file_awareness: false,
            lock_file_path: None,
            auto_validate_lock_file: false,
            conflict_prevention_enabled: false,
            dependency_version_consistency: false,
            enable_agent_state_management: false,
            max_concurrent_agents: 10,
            max_block_time_seconds: 30,
            agent_persistence_config: None,
            enable_coordination_integration: false,
            coordination_config: None,
            enable_advanced_conflict_prevention: false,
            advanced_conflict_prevention_config: None,
        };

        let ai_service = AIService::new(ai_config).await.unwrap();
        let coordination = CoordinationIntegration::new(
            RealTimeCoordinationSystem::new(),
            None
        ).await.unwrap();
        let config = ProductionConfig::default();

        let integration = ProductionIntegration::new(
            ai_service,
            coordination,
            config
        ).await;

        assert!(integration.is_ok());
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let ai_config = AIServiceConfig {
            api_key: "test".to_string(),
            base_url: "http://localhost:8080".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 10,
            rate_limit_per_minute: 100,
            cache_ttl_seconds: 300,
            model_version: "test".to_string(),
            enable_caching: true,
            enable_rate_limiting: true,
            enable_monitoring: true,
            enable_lock_file_awareness: false,
            lock_file_path: None,
            auto_validate_lock_file: false,
            conflict_prevention_enabled: false,
            dependency_version_consistency: false,
            enable_agent_state_management: false,
            max_concurrent_agents: 10,
            max_block_time_seconds: 30,
            agent_persistence_config: None,
            enable_coordination_integration: false,
            coordination_config: None,
            enable_advanced_conflict_prevention: false,
            advanced_conflict_prevention_config: None,
        };

        let ai_service = AIService::new(ai_config).await.unwrap();
        let coordination = CoordinationIntegration::new(
            RealTimeCoordinationSystem::new(),
            None
        ).await.unwrap();
        let mut config = ProductionConfig::default();
        config.circuit_breaker.failure_threshold = 2;

        let integration = ProductionIntegration::new(
            ai_service,
            coordination,
            config
        ).await.unwrap();

        // Initially circuit breaker should be closed
        let status = integration.get_circuit_breaker_status().await;
        assert!(matches!(status, CircuitBreakerState::Closed));

        // Reset circuit breaker
        integration.reset_circuit_breaker().await;
        let status = integration.get_circuit_breaker_status().await;
        assert!(matches!(status, CircuitBreakerState::Closed));
    }
} 