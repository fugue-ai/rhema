//! Rhema Dependency Management
//!
//! A comprehensive dependency management system that provides semantic dependency types,
//! impact analysis, health monitoring, and advanced validation capabilities.

pub mod advanced_analysis;
pub mod config;
pub mod error;
pub mod graph;
pub mod health;
pub mod impact;
pub mod integrations;
pub mod manager;
pub mod metrics;
pub mod performance;
pub mod predictive;
pub mod realtime;
pub mod resolution;
pub mod security;
pub mod storage;
pub mod types;
pub mod user_experience;
pub mod validation;

pub use advanced_analysis::*;
pub use config::*;
pub use error::{
    Error, HealthCheckResult, ImpactAnalysisResult, Result, RiskLevel, ValidationResult,
};
pub use graph::*;
pub use health::*;
pub use impact::*;
pub use integrations::*;
pub use manager::*;
pub use metrics::*;
pub use performance::*;
pub use predictive::*;
pub use realtime::*;
pub use resolution::*;
pub use security::*;
pub use storage::*;
pub use types::*;
pub use user_experience::*;
pub use validation::*;

// Re-export commonly used types
pub use AdvancedAnalyzer;
pub use CiCdIntegration;
pub use DependencyAlertSystem;
pub use DependencyCache;
pub use DependencyDashboard;
pub use DependencyGraph;
pub use DependencyManager;
pub use DependencyReportGenerator;
pub use DependencyResolver;
pub use DependencySearchEngine;
pub use DependencyType;
pub use HealthMonitor;
pub use HealthStatus;
pub use IdeIntegration;
pub use ImpactAnalysis;
pub use ImpactScore;
pub use PackageManagerIntegration;
pub use ParallelProcessor;
pub use PredictiveAnalytics;
pub use ResolutionStrategy;
pub use SecurityScanner;
pub use ValidationEngine;

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
