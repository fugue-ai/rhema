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

pub mod cluster_manager;
pub mod node_discovery;
pub mod load_balancer;
pub mod health_checker;
pub mod service_registry;

pub use cluster_manager::ClusterManager;
pub use node_discovery::NodeDiscovery;
pub use load_balancer::DistributedLoadBalancer;
pub use health_checker::HealthChecker;
pub use service_registry::ServiceRegistry;

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use chrono::{DateTime, Utc};

/// Distributed deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    /// Node configuration
    pub node: NodeConfig,
    /// Cluster configuration
    pub cluster: ClusterConfig,
    /// Service discovery configuration
    pub discovery: DiscoveryConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Health checking configuration
    pub health_checking: HealthCheckingConfig,
    /// Service registry configuration
    pub service_registry: ServiceRegistryConfig,
}

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node ID
    pub node_id: String,
    /// Node name
    pub name: String,
    /// Node address
    pub address: SocketAddr,
    /// Node role
    pub role: NodeRole,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Node metadata
    pub metadata: HashMap<String, String>,
}

/// Node roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeRole {
    /// Coordinator node
    Coordinator,
    /// Worker node
    Worker,
    /// Observer node
    Observer,
    /// Backup node
    Backup,
}

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Cluster name
    pub name: String,
    /// Cluster ID
    pub cluster_id: String,
    /// Minimum number of nodes required
    pub min_nodes: usize,
    /// Maximum number of nodes allowed
    pub max_nodes: usize,
    /// Node timeout in seconds
    pub node_timeout_seconds: u64,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Enable leader election
    pub enable_leader_election: bool,
    /// Leader election timeout in seconds
    pub leader_election_timeout_seconds: u64,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Discovery method
    pub method: DiscoveryMethod,
    /// Multicast address (for multicast discovery)
    pub multicast_address: Option<SocketAddr>,
    /// Static node list (for static discovery)
    pub static_nodes: Vec<SocketAddr>,
    /// Discovery interval in seconds
    pub discovery_interval_seconds: u64,
    /// Discovery timeout in seconds
    pub discovery_timeout_seconds: u64,
}

/// Discovery methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Multicast discovery
    Multicast,
    /// Static node list
    Static,
    /// DNS-based discovery
    Dns,
    /// Consul-based discovery
    Consul,
    /// Etcd-based discovery
    Etcd,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    /// Enable health-aware routing
    pub enable_health_aware_routing: bool,
    /// Enable sticky sessions
    pub enable_sticky_sessions: bool,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Least response time
    LeastResponseTime,
    /// Random
    Random,
    /// IP hash
    IpHash,
}

/// Health checking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckingConfig {
    /// Health check interval in seconds
    pub interval_seconds: u64,
    /// Health check timeout in seconds
    pub timeout_seconds: u64,
    /// Number of consecutive failures before marking unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before marking healthy
    pub success_threshold: u32,
    /// Health check endpoint
    pub health_check_endpoint: String,
    /// Enable TCP health checks
    pub enable_tcp_checks: bool,
    /// Enable HTTP health checks
    pub enable_http_checks: bool,
}

/// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Registry backend
    pub backend: RegistryBackend,
    /// Registry address
    pub address: String,
    /// Registry credentials
    pub credentials: Option<RegistryCredentials>,
    /// Service registration interval in seconds
    pub registration_interval_seconds: u64,
    /// Service deregistration timeout in seconds
    pub deregistration_timeout_seconds: u64,
}

/// Registry backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryBackend {
    /// Consul
    Consul,
    /// Etcd
    Etcd,
    /// Zookeeper
    Zookeeper,
    /// Custom
    Custom(String),
}

/// Registry credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryCredentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Token
    pub token: Option<String>,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            node: NodeConfig {
                node_id: uuid::Uuid::new_v4().to_string(),
                name: "rhema-node".to_string(),
                address: "127.0.0.1:8080".parse().unwrap(),
                role: NodeRole::Worker,
                capabilities: vec!["coordination".to_string(), "ai_service".to_string()],
                metadata: HashMap::new(),
            },
            cluster: ClusterConfig {
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
            discovery: DiscoveryConfig {
                method: DiscoveryMethod::Multicast,
                multicast_address: Some("224.0.0.1:8081".parse().unwrap()),
                static_nodes: Vec::new(),
                discovery_interval_seconds: 30,
                discovery_timeout_seconds: 10,
            },
            load_balancing: LoadBalancingConfig {
                strategy: LoadBalancingStrategy::RoundRobin,
                enable_health_aware_routing: true,
                enable_sticky_sessions: false,
                session_timeout_seconds: 300,
                max_retry_attempts: 3,
                retry_delay_ms: 1000,
            },
            health_checking: HealthCheckingConfig {
                interval_seconds: 30,
                timeout_seconds: 5,
                failure_threshold: 3,
                success_threshold: 2,
                health_check_endpoint: "/health".to_string(),
                enable_tcp_checks: true,
                enable_http_checks: true,
            },
            service_registry: ServiceRegistryConfig {
                backend: RegistryBackend::Consul,
                address: "http://localhost:8500".to_string(),
                credentials: None,
                registration_interval_seconds: 30,
                deregistration_timeout_seconds: 60,
            },
        }
    }
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node ID
    pub node_id: String,
    /// Node name
    pub name: String,
    /// Node address
    pub address: SocketAddr,
    /// Node role
    pub role: NodeRole,
    /// Node status
    pub status: NodeStatus,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Node metadata
    pub metadata: HashMap<String, String>,
    /// Last heartbeat
    pub last_heartbeat: DateTime<Utc>,
    /// Node uptime in seconds
    pub uptime_seconds: u64,
}

/// Node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is online and healthy
    Online,
    /// Node is offline
    Offline,
    /// Node is unhealthy
    Unhealthy,
    /// Node is joining the cluster
    Joining,
    /// Node is leaving the cluster
    Leaving,
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    /// Service ID
    pub service_id: String,
    /// Service address
    pub address: SocketAddr,
    /// Service health status
    pub health_status: ServiceHealthStatus,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service registration time
    pub registered_at: DateTime<Utc>,
    /// Last health check
    pub last_health_check: DateTime<Utc>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceHealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is unhealthy
    Unhealthy,
    /// Service health is unknown
    Unknown,
}

/// Cluster health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    /// Cluster status
    pub status: ClusterStatus,
    /// Total nodes
    pub total_nodes: usize,
    /// Online nodes
    pub online_nodes: usize,
    /// Unhealthy nodes
    pub unhealthy_nodes: usize,
    /// Offline nodes
    pub offline_nodes: usize,
    /// Cluster leader
    pub leader: Option<String>,
    /// Last health check
    pub last_health_check: DateTime<Utc>,
}

/// Cluster status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClusterStatus {
    /// Cluster is healthy
    Healthy,
    /// Cluster is degraded
    Degraded,
    /// Cluster is unhealthy
    Unhealthy,
    /// Cluster is forming
    Forming,
}

/// Distributed deployment manager
pub struct DistributedManager {
    config: DistributedConfig,
    cluster_manager: ClusterManager,
    node_discovery: NodeDiscovery,
    load_balancer: DistributedLoadBalancer,
    health_checker: HealthChecker,
    service_registry: ServiceRegistry,
}

impl DistributedManager {
    /// Create a new distributed manager
    pub async fn new(config: DistributedConfig) -> RhemaResult<Self> {
        let cluster_manager = ClusterManager::new(config.cluster.clone()).await?;
        let node_discovery = NodeDiscovery::new(config.discovery.clone()).await?;
        let load_balancer = DistributedLoadBalancer::new(config.load_balancing.clone()).await?;
        let health_checker = HealthChecker::new(config.health_checking.clone()).await?;
        let service_registry = ServiceRegistry::new(config.service_registry.clone()).await?;

        Ok(Self {
            config,
            cluster_manager,
            node_discovery,
            load_balancer,
            health_checker,
            service_registry,
        })
    }

    /// Start the distributed manager
    pub async fn start(&self) -> RhemaResult<()> {
        // Start all components
        self.cluster_manager.start().await?;
        self.node_discovery.start().await?;
        self.load_balancer.start().await?;
        self.health_checker.start().await?;
        self.service_registry.start().await?;

        Ok(())
    }

    /// Stop the distributed manager
    pub async fn stop(&self) -> RhemaResult<()> {
        // Stop all components
        self.service_registry.stop().await?;
        self.health_checker.stop().await?;
        self.load_balancer.stop().await?;
        self.node_discovery.stop().await?;
        self.cluster_manager.stop().await?;

        Ok(())
    }

    /// Get cluster health
    pub async fn get_cluster_health(&self) -> RhemaResult<ClusterHealth> {
        self.cluster_manager.get_health().await
    }

    /// Get node information
    pub async fn get_node_info(&self) -> RhemaResult<NodeInfo> {
        self.cluster_manager.get_node_info().await
    }

    /// Get all nodes
    pub async fn get_all_nodes(&self) -> RhemaResult<Vec<NodeInfo>> {
        self.cluster_manager.get_all_nodes().await
    }

    /// Register service
    pub async fn register_service(&self, service_info: ServiceInfo) -> RhemaResult<()> {
        self.service_registry.register_service(service_info).await
    }

    /// Deregister service
    pub async fn deregister_service(&self, service_id: &str) -> RhemaResult<()> {
        self.service_registry.deregister_service(service_id).await
    }

    /// Get service information
    pub async fn get_service_info(&self, service_id: &str) -> RhemaResult<Option<ServiceInfo>> {
        self.service_registry.get_service_info(service_id).await
    }

    /// Get all services
    pub async fn get_all_services(&self) -> RhemaResult<Vec<ServiceInfo>> {
        self.service_registry.get_all_services().await
    }

    /// Select node for service
    pub async fn select_node_for_service(&self, service_name: &str) -> RhemaResult<Option<NodeInfo>> {
        self.load_balancer.select_node(service_name).await
    }
} 