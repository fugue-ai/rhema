//! Rhema Dependency Management
//! 
//! A comprehensive dependency management system that provides semantic dependency types,
//! impact analysis, health monitoring, and advanced validation capabilities.

pub mod error;
pub mod types;
pub mod graph;
pub mod impact;
pub mod health;
pub mod validation;
pub mod manager;
pub mod config;
pub mod metrics;
pub mod storage;
pub mod realtime;
pub mod resolution;
pub mod predictive;
pub mod security;
pub mod performance;
pub mod advanced_analysis;
pub mod integrations;
pub mod user_experience;

pub use error::{Error, Result, RiskLevel, ValidationResult, ImpactAnalysisResult, HealthCheckResult};
pub use types::*;
pub use graph::*;
pub use impact::*;
pub use health::*;
pub use validation::*;
pub use manager::*;
pub use config::*;
pub use metrics::*;
pub use storage::*;
pub use realtime::*;
pub use resolution::*;
pub use predictive::*;
pub use security::*;
pub use performance::*;
pub use advanced_analysis::*;
pub use integrations::*;
pub use user_experience::*;

// Re-export commonly used types
pub use DependencyManager;
pub use DependencyType;
pub use ImpactAnalysis;
pub use HealthMonitor;
pub use ValidationEngine;
pub use DependencyGraph;
pub use HealthStatus;
pub use ImpactScore;
pub use DependencyResolver;
pub use ResolutionStrategy;
pub use PredictiveAnalytics;
pub use SecurityScanner;
pub use DependencyCache;
pub use ParallelProcessor;
pub use AdvancedAnalyzer;
pub use PackageManagerIntegration;
pub use CiCdIntegration;
pub use IdeIntegration;
pub use DependencyDashboard;
pub use DependencyReportGenerator;
pub use DependencyAlertSystem;
pub use DependencySearchEngine;

/// Initialize the dependency management system
pub async fn init() -> Result<DependencyManager> {
    DependencyManager::new().await
}

/// Create a new dependency manager with default configuration
pub async fn new_manager() -> Result<DependencyManager> {
    DependencyManager::new().await
}

/// Create a new dependency manager with custom configuration
pub async fn new_manager_with_config(config: Config) -> Result<DependencyManager> {
    DependencyManager::with_config(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let manager = init().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_new_manager() {
        let manager = new_manager().await;
        assert!(manager.is_ok());
    }
} 