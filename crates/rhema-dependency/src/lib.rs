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

// Re-export advanced_analysis types (excluding conflicts)
pub use advanced_analysis::{
    AdvancedAnalyzer,
    ClusteringEngine,
    CostAnalysis,
    CostAnalyzer,
    CostBreakdown,
    CostModel,
    DependencyCluster,
    DependencyScore,
    PerformanceAnalyzer,
    PerformanceImpact,
    PerformanceMetrics,
    PerformanceThresholds,
    RiskAssessment,
    RiskAssessor,
    RiskFactor,
    RiskLevel as AdvancedRiskLevel,
    ScoringEngine,
    ScoringWeights,
    SecurityAnalysis,
    SecurityAnalyzer,
    SecurityCheck,
    SecurityIssue,
    SecuritySeverity,
    TrendAnalysis as AdvancedTrendAnalysis,
    TrendAnalyzer,
    TrendDataPoint,
    TrendDirection as AdvancedTrendDirection,
    // VulnerabilityAnalysis not found in advanced_analysis
};

// Re-export config types (excluding conflicts)
pub use config::{
    AlertChannelConfig,
    AlertChannelType,
    AlertConditionConfig,
    AlertRuleConfig,
    AlertSeverity,
    CacheConfig,
    Config,
    HealthMonitoringConfig,
    PerformanceConfig,
    SecurityConfig,
    ValidationConfig,
    ValidationSeverity,
    // Exclude RetryConfig to avoid conflicts
};

pub use error::{
    Error, HealthCheckResult, ImpactAnalysisResult, Result, RiskLevel, ValidationResult,
};
pub use graph::*;
pub use health::*;

// Re-export impact types (excluding conflicts)
pub use impact::{
    BusinessImpactMetrics, ImpactAnalysis as ImpactAnalysisModule, RiskFactors,
    TrendAnalysis as ImpactTrendAnalysis, TrendDirection as ImpactTrendDirection,
};

pub use integrations::*;
pub use manager::*;
pub use metrics::*;

// Re-export performance types (excluding conflicts)
pub use performance::{
    CacheConfig as PerformanceCacheConfig, CacheEntry, CacheStatistics, DependencyCache,
    MemoryConfig, MemoryMetrics, MemoryOptimizer, ParallelConfig, ParallelProcessor, QueryConfig,
    QueryOptimizer,
};

// Re-export predictive types (excluding conflicts)
pub use predictive::{
    AnomalyResult, AnomalySeverity, AnomalyThresholds, AnomalyType, ModelConfig, PredictionModel,
    PredictionResult, PredictionStatistics, PredictiveAnalytics,
    RiskFactor as PredictiveRiskFactor, TrendAnalysis as PredictiveTrendAnalysis,
    TrendDirection as PredictiveTrendDirection,
};

pub use realtime::*;
pub use resolution::*;

// Re-export security types (excluding conflicts)
pub use security::{
    ComplianceCheck, ComplianceStandard, ImplementationEffort, RecommendationPriority,
    RiskAssessment as SecRiskAssessment, RiskFactor as SecRiskFactor, RiskLevel as SecRiskLevel,
    SecurityImpact, SecurityRecommendation, SecurityScanResult, SecurityScanner,
    SecurityScannerConfig, SecurityStatistics, SecurityStatus, Vulnerability,
    VulnerabilitySeverity, VulnerabilityType,
};

pub use storage::*;
pub use types::*;

// Re-export user_experience types (excluding conflicts)
pub use user_experience::{
    Alert, AlertAction, AlertChannel, AlertCondition, AlertConfig, AlertRule,
    AlertSeverity as UXAlertSeverity, AlertStatus, AlertThresholds, CriticalIssue, DashboardConfig,
    DashboardData, DashboardView, DependencyAlertSystem, DependencyDashboard, DependencyReport,
    DependencyReportGenerator, DependencySearchEngine, EmailConfig, HealthData, HealthStatusWidget,
    HealthTrend, IndexedDependency, IssueSeverity, ReportConfig, ReportFormat, ReportMetadata,
    ReportTemplate, SearchConfig, SearchIndex, SearchResult, SlackConfig, TicketConfig,
    VulnerabilitySeverity as UXVulnerabilitySeverity, WebhookConfig, WidgetConfig, WidgetData,
    WidgetType,
};

// Re-export validation types (excluding conflicts)
pub use validation::{
    ValidationConfig as ValConfig, ValidationEngine, ValidationIssue, ValidationRule,
    ValidationRuleType, ValidationSeverity as ValSeverity, ValidationStatistics,
};

// Note: All commonly used types are already re-exported above through the specific module imports

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
