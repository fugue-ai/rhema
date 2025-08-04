use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::error::{Error, Result, ImpactAnalysisResult, ValidationResult};
use crate::types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore, HealthMetrics};
use crate::graph::DependencyGraph;
use crate::impact::ImpactAnalysis;
use crate::health::{HealthMonitor, HealthMonitorConfig, HealthStatusWithMetrics};
use crate::validation::{ValidationEngine, ValidationConfig};
use crate::config::Config;

/// Main dependency manager that orchestrates all components
pub struct DependencyManager {
    /// Dependency graph
    graph: Arc<RwLock<DependencyGraph>>,
    /// Impact analysis engine
    impact_analysis: ImpactAnalysis,
    /// Health monitor
    health_monitor: HealthMonitor,
    /// Validation engine
    validation_engine: ValidationEngine,
    /// Configuration
    config: Config,
    /// Created timestamp
    created_at: DateTime<Utc>,
    /// Last updated timestamp
    last_updated: DateTime<Utc>,
}

impl DependencyManager {
    /// Create a new dependency manager with default configuration
    pub async fn new() -> Result<Self> {
        let config = Config::default();
        Self::with_config(config).await
    }

    /// Create a new dependency manager with custom configuration
    pub async fn with_config(config: Config) -> Result<Self> {
        let graph = Arc::new(RwLock::new(DependencyGraph::new()));
        
        let health_config = HealthMonitorConfig {
            default_check_interval: config.health_monitoring.health_check_interval,
            default_timeout: config.health_monitoring.health_check_timeout,
            enable_realtime: config.health_monitoring.enable_realtime_monitoring,
            enable_alerting: config.health_monitoring.enable_alerting,
            metrics_retention_hours: config.health_monitoring.metrics_retention_hours,
            health_score_weights: config.health_monitoring.health_score_weights.clone(),
        };

        let validation_config = ValidationConfig {
            enable_caching: config.validation.enable_validation_caching,
            cache_ttl_seconds: config.validation.validation_cache_ttl,
            max_errors: config.validation.max_validation_errors,
            max_warnings: config.validation.max_validation_warnings,
            enable_parallel: config.validation.enable_parallel_validation,
            timeout_seconds: config.validation.validation_timeout,
        };

        let health_monitor = HealthMonitor::with_config(graph.clone(), health_config);
        let validation_engine = ValidationEngine::with_config(validation_config);

        Ok(Self {
            graph,
            impact_analysis: ImpactAnalysis::new(),
            health_monitor,
            validation_engine,
            config,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        })
    }

    /// Add a dependency to the manager
    pub async fn add_dependency(
        &mut self,
        id: String,
        name: String,
        dependency_type: DependencyType,
        target: String,
        operations: Vec<String>,
    ) -> Result<()> {
        // Create dependency configuration
        let config = DependencyConfig::new(id.clone(), name, dependency_type, target, operations)?;

        // Validate the configuration
        let graph_guard = self.graph.read().await;
        let validation_result = self.validation_engine.validate_dependency(&config, &*graph_guard)?;
        if !validation_result.valid {
            return Err(Error::Validation(format!("Dependency validation failed: {:?}", validation_result.errors)));
        }

        // Add to graph
        {
            let mut graph = self.graph.write().await;
            graph.add_node(config.clone())?;
        }

        // Update health monitor if health check is configured
        if let Some(health_check) = &config.health_check {
            self.health_monitor.add_health_check(id.clone(), health_check.clone());
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Remove a dependency from the manager
    pub async fn remove_dependency(&mut self, dependency_id: &str) -> Result<()> {
        {
            let mut graph = self.graph.write().await;
            graph.remove_node(dependency_id)?;
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Get dependency configuration
    pub async fn get_dependency_config(&self, dependency_id: &str) -> Result<DependencyConfig> {
        let graph = self.graph.read().await;
        let config = graph.get_dependency_config(dependency_id)?;
        Ok(config.clone())
    }

    /// List all dependencies
    pub async fn list_dependencies(&self) -> Result<Vec<DependencyConfig>> {
        let graph = self.graph.read().await;
        Ok(graph.get_all_dependency_configs().into_iter().cloned().collect())
    }

    /// Add a dependency relationship
    pub async fn add_dependency_relationship(
        &mut self,
        source_id: &str,
        target_id: &str,
        relationship_type: String,
        strength: f64,
        operations: Vec<String>,
    ) -> Result<()> {
        {
            let mut graph = self.graph.write().await;
            graph.add_edge(source_id, target_id, relationship_type, strength, operations)?;
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Remove a dependency relationship
    pub async fn remove_dependency_relationship(
        &mut self,
        source_id: &str,
        target_id: &str,
    ) -> Result<()> {
        {
            let mut graph = self.graph.write().await;
            graph.remove_edge(source_id, target_id)?;
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Analyze the impact of a dependency
    pub async fn analyze_impact(&self, dependency_id: &str) -> Result<ImpactAnalysisResult> {
        let graph = self.graph.read().await;
        self.impact_analysis.analyze_dependency_impact(dependency_id, &graph)
    }

    /// Analyze the impact of a change
    pub async fn analyze_change_impact(
        &self,
        change_description: &str,
        affected_dependencies: &[String],
    ) -> Result<ImpactAnalysisResult> {
        let graph = self.graph.read().await;
        self.impact_analysis.analyze_change_impact(change_description, affected_dependencies, &graph)
    }

    /// Get health status of a dependency
    pub async fn get_health(&self, dependency_id: &str) -> Result<HealthStatusWithMetrics> {
        self.health_monitor.get_health_status(dependency_id).await
    }

    /// Get health status of all dependencies
    pub async fn get_all_health_statuses(&self) -> std::collections::HashMap<String, HealthStatusWithMetrics> {
        self.health_monitor.get_all_health_statuses().await
    }

    /// Perform a manual health check
    pub async fn perform_health_check(&self, dependency_id: &str) -> Result<crate::health::HealthCheckResult> {
        self.health_monitor.perform_manual_health_check(dependency_id).await
    }

    /// Start health monitoring
    pub async fn start_health_monitoring(&mut self) -> Result<()> {
        self.health_monitor.start().await
    }

    /// Stop health monitoring
    pub async fn stop_health_monitoring(&mut self) -> Result<()> {
        self.health_monitor.stop().await
    }

    /// Validate a dependency
    pub async fn validate_dependency(&self, dependency_id: &str) -> Result<ValidationResult> {
        let graph = self.graph.read().await;
        let config = graph.get_dependency_config(dependency_id)?;
        self.validation_engine.validate_dependency(&config, &graph)
    }

    /// Validate the entire dependency graph
    pub async fn validate_graph(&self) -> Result<ValidationResult> {
        let graph = self.graph.read().await;
        self.validation_engine.validate_graph(&graph)
    }

    /// Get dependents of a dependency
    pub async fn get_dependents(&self, dependency_id: &str) -> Result<Vec<String>> {
        let graph = self.graph.read().await;
        graph.get_dependents(dependency_id)
    }

    /// Get dependencies that a service depends on
    pub async fn get_dependencies(&self, dependency_id: &str) -> Result<Vec<String>> {
        let graph = self.graph.read().await;
        graph.get_dependencies(dependency_id)
    }

    /// Get dependencies by type
    pub async fn get_dependencies_by_type(&self, dependency_type: DependencyType) -> Result<Vec<String>> {
        let graph = self.graph.read().await;
        graph.get_dependencies_by_type(dependency_type)
    }

    /// Get dependencies by health status
    pub async fn get_dependencies_by_health(&self, health_status: HealthStatus) -> Result<Vec<String>> {
        let graph = self.graph.read().await;
        graph.get_dependencies_by_health(health_status)
    }

    /// Update health status of a dependency
    pub async fn update_health_status(&mut self, dependency_id: &str, health_status: HealthStatus) -> Result<()> {
        {
            let mut graph = self.graph.write().await;
            graph.update_health_status(dependency_id, health_status)?;
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Update health metrics of a dependency
    pub async fn update_health_metrics(&mut self, dependency_id: &str, health_metrics: ImpactScore) -> Result<()> {
        {
            let mut graph = self.graph.write().await;
            graph.update_health_metrics(dependency_id, health_metrics)?;
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Check for circular dependencies
    pub async fn has_circular_dependencies(&self) -> Result<bool> {
        let graph = self.graph.read().await;
        graph.has_circular_dependencies()
    }

    /// Find circular dependencies
    pub async fn find_circular_dependencies(&self) -> Result<Vec<Vec<String>>> {
        let graph = self.graph.read().await;
        graph.find_circular_dependencies()
    }

    /// Get dependency graph statistics
    pub async fn get_graph_statistics(&self) -> GraphStatistics {
        let graph = self.graph.read().await;
        let health_statuses = self.health_monitor.get_all_health_statuses().await;

        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;
        let mut down_count = 0;
        let mut unknown_count = 0;

        for health_status in health_statuses.values() {
            match health_status.status {
                HealthStatus::Healthy => healthy_count += 1,
                HealthStatus::Degraded => degraded_count += 1,
                HealthStatus::Unhealthy => unhealthy_count += 1,
                HealthStatus::Down => down_count += 1,
                HealthStatus::Unknown => unknown_count += 1,
            }
        }

        GraphStatistics {
            total_nodes: graph.node_count(),
            total_edges: graph.edge_count(),
            healthy_count,
            degraded_count,
            unhealthy_count,
            down_count,
            unknown_count,
            last_updated: Utc::now(),
        }
    }

    /// Get health monitoring statistics
    pub async fn get_health_statistics(&self) -> crate::health::HealthMonitorStatistics {
        self.health_monitor.get_statistics().await
    }

    /// Get validation statistics
    pub async fn get_validation_statistics(&self) -> Result<crate::validation::ValidationStatistics> {
        Ok(self.validation_engine.get_statistics())
    }

    /// Export dependency graph as DOT format
    pub async fn export_graph_dot(&self) -> Result<String> {
        let graph = self.graph.read().await;
        Ok(graph.to_dot())
    }

    /// Import dependencies from configuration
    pub async fn import_from_config(&mut self, configs: Vec<DependencyConfig>) -> Result<()> {
        for config in configs {
                    // Validate the configuration
        let graph_guard = self.graph.read().await;
        let validation_result = self.validation_engine.validate_dependency(&config, &*graph_guard)?;
            if !validation_result.valid {
                return Err(Error::Validation(format!("Dependency validation failed for {}: {:?}", config.id, validation_result.errors)));
            }

            // Add to graph
            {
                let mut graph = self.graph.write().await;
                graph.add_node(config.clone())?;
            }

            // Add health check if configured
            if let Some(health_check) = &config.health_check {
                self.health_monitor.add_health_check(config.id.clone(), health_check.clone());
            }
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Export dependencies to configuration
    pub async fn export_to_config(&self) -> Result<Vec<DependencyConfig>> {
        self.list_dependencies().await
    }

    /// Get manager information
    pub fn get_info(&self) -> ManagerInfo {
        ManagerInfo {
            created_at: self.created_at,
            last_updated: self.last_updated,
            config: self.config.clone(),
        }
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: Config) -> Result<()> {
        self.config = config;
        self.last_updated = Utc::now();
        Ok(())
    }

    /// Get configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Check if the manager is healthy
    pub async fn is_healthy(&self) -> Result<bool> {
        // Check if graph is accessible
        let graph = self.graph.read().await;
        if graph.is_empty() {
            return Ok(true); // Empty graph is considered healthy
        }

        // Check for circular dependencies
        if graph.has_circular_dependencies()? {
            return Ok(false);
        }

        // Check health statuses
        let health_statuses = self.health_monitor.get_all_health_statuses().await;
        let critical_dependencies = health_statuses.values()
            .filter(|status| status.status == HealthStatus::Down)
            .count();

        // Consider unhealthy if more than 10% of dependencies are down
        let total_dependencies = health_statuses.len();
        if total_dependencies > 0 {
            let down_percentage = critical_dependencies as f64 / total_dependencies as f64;
            Ok(down_percentage < 0.1)
        } else {
            Ok(true)
        }
    }

    /// Get health report
    pub async fn get_health_report(&self) -> Result<HealthReport> {
        let health_statuses = self.health_monitor.get_all_health_statuses().await;
        let graph_stats = self.get_graph_statistics().await;
        let health_stats = self.get_health_statistics().await;
        let validation_stats = self.get_validation_statistics().await;
        let is_healthy = self.is_healthy().await?;

        Ok(HealthReport {
            is_healthy,
            graph_statistics: graph_stats,
            health_statistics: health_stats,
            validation_statistics: validation_stats?,
            critical_dependencies: health_statuses.values()
                .filter(|status| status.status == HealthStatus::Down)
                .map(|status| status.clone())
                .collect(),
            timestamp: Utc::now(),
        })
    }
}

/// Graph statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphStatistics {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Total number of edges
    pub total_edges: usize,
    /// Number of healthy dependencies
    pub healthy_count: usize,
    /// Number of degraded dependencies
    pub degraded_count: usize,
    /// Number of unhealthy dependencies
    pub unhealthy_count: usize,
    /// Number of down dependencies
    pub down_count: usize,
    /// Number of unknown dependencies
    pub unknown_count: usize,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Manager information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManagerInfo {
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    /// Configuration
    pub config: Config,
}

/// Health report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthReport {
    /// Whether the system is healthy
    pub is_healthy: bool,
    /// Graph statistics
    pub graph_statistics: GraphStatistics,
    /// Health statistics
    pub health_statistics: crate::health::HealthMonitorStatistics,
    /// Validation statistics
    pub validation_statistics: crate::validation::ValidationStatistics,
    /// Critical dependencies (down)
    pub critical_dependencies: Vec<crate::health::HealthStatusWithMetrics>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyType;

    #[tokio::test]
    async fn test_new_manager() {
        let manager = DependencyManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_add_dependency() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        let result = manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_dependencies() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).await.unwrap();

        let dependencies = manager.list_dependencies().await.unwrap();
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].id, "test-1");
    }

    #[tokio::test]
    async fn test_get_dependency_config() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).await.unwrap();

        let config = manager.get_dependency_config("test-1").await.unwrap();
        assert_eq!(config.id, "test-1");
        assert_eq!(config.name, "Test Dependency");
    }

    #[tokio::test]
    async fn test_remove_dependency() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).await.unwrap();

        let result = manager.remove_dependency("test-1").await;
        assert!(result.is_ok());

        let dependencies = manager.list_dependencies().await.unwrap();
        assert_eq!(dependencies.len(), 0);
    }

    #[tokio::test]
    async fn test_add_dependency_relationship() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency 1".to_string(),
            DependencyType::ApiCall,
            "http://test1.example.com".to_string(),
            vec!["GET".to_string()],
        ).await.unwrap();

        manager.add_dependency(
            "test-2".to_string(),
            "Test Dependency 2".to_string(),
            DependencyType::DataFlow,
            "http://test2.example.com".to_string(),
            vec!["POST".to_string()],
        ).await.unwrap();

        let result = manager.add_dependency_relationship(
            "test-1",
            "test-2",
            "depends_on".to_string(),
            0.8,
            vec!["read".to_string()],
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_graph_statistics() {
        let mut manager = DependencyManager::new().await.unwrap();
        
        manager.add_dependency(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).await.unwrap();

        let stats = manager.get_graph_statistics().await;
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.total_edges, 0);
    }

    #[tokio::test]
    async fn test_is_healthy() {
        let manager = DependencyManager::new().await.unwrap();
        let is_healthy = manager.is_healthy().await.unwrap();
        assert!(is_healthy); // Empty manager should be healthy
    }
} 