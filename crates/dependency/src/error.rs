use thiserror::Error;

/// Error types for the dependency management system
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Impact analysis failed: {0}")]
    ImpactAnalysisFailed(String),

    #[error("Graph operation failed: {0}")]
    GraphOperation(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Real-time communication error: {0}")]
    RealtimeCommunication(String),

    #[error("Metrics error: {0}")]
    Metrics(String),

    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid dependency type: {0}")]
    InvalidDependencyType(String),

    #[error("Invalid health status: {0}")]
    InvalidHealthStatus(String),

    #[error("Invalid impact score: {0}")]
    InvalidImpactScore(String),

    #[error("Monitoring error: {0}")]
    Monitoring(String),

    #[error("Alert error: {0}")]
    Alert(String),

    #[error("Schema validation error: {0}")]
    SchemaValidation(String),

    #[error("Security validation error: {0}")]
    SecurityValidation(String),

    #[error("Business impact calculation error: {0}")]
    BusinessImpactCalculation(String),

    #[error("Risk assessment error: {0}")]
    RiskAssessment(String),

    #[error("Performance monitoring error: {0}")]
    PerformanceMonitoring(String),

    #[error("Dependency graph error: {0}")]
    DependencyGraph(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Prometheus metrics error: {0}")]
    PrometheusMetrics(String),

    #[error("Tracing error: {0}")]
    Tracing(String),

    #[error("Configuration file not found: {0}")]
    ConfigFileNotFound(String),

    #[error("Invalid configuration format: {0}")]
    InvalidConfigFormat(String),

    #[error("Dependency already exists: {0}")]
    DependencyAlreadyExists(String),

    #[error("Invalid version constraint: {0}")]
    InvalidVersionConstraint(String),

    #[error("No compatible version found for dependency: {0}")]
    NoCompatibleVersion(String),

    #[error("Specific version not found for dependency {0}: {1}")]
    SpecificVersionNotFound(String, semver::Version),

    #[error("Insufficient data for dependency: {0}")]
    InsufficientData(String),

    #[error("Prediction failed for dependency: {0}")]
    PredictionFailed(String),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("Invalid model parameters: {0}")]
    InvalidModelParameters(String),

    #[error("Security scan failed: {0}")]
    SecurityScanFailed(String),

    #[error("Vulnerability database error: {0}")]
    VulnerabilityDatabaseError(String),

    #[error("Compliance check failed: {0}")]
    ComplianceCheckFailed(String),

    #[error("Invalid dependency operation: {0}")]
    InvalidDependencyOperation(String),

    #[error("Health monitoring not started")]
    HealthMonitoringNotStarted,

    #[error("Impact analysis not configured")]
    ImpactAnalysisNotConfigured,

    #[error("Validation engine not initialized")]
    ValidationEngineNotInitialized,

    #[error("Real-time tracking not enabled")]
    RealtimeTrackingNotEnabled,

    #[error("Metrics collection not started")]
    MetricsCollectionNotStarted,

    #[error("Storage not initialized")]
    StorageNotInitialized,

    #[error("Graph not initialized")]
    GraphNotInitialized,

    #[error("Manager not initialized")]
    ManagerNotInitialized,

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("External error: {0}")]
    External(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Unknown(err.to_string())
    }
}

/// Result type for dependency management operations
pub type Result<T> = std::result::Result<T, Error>;

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub success: bool,
    pub message: String,
    pub duration: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Impact analysis result
#[derive(Debug, Clone)]
pub struct ImpactAnalysisResult {
    pub business_impact_score: f64,
    pub risk_level: RiskLevel,
    pub affected_services: Vec<String>,
    pub estimated_downtime: std::time::Duration,
    pub cost_impact: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Risk level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

impl From<f64> for RiskLevel {
    fn from(score: f64) -> Self {
        match score {
            s if s < 0.25 => RiskLevel::Low,
            s if s < 0.5 => RiskLevel::Medium,
            s if s < 0.75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::Low.to_string(), "Low");
        assert_eq!(RiskLevel::Medium.to_string(), "Medium");
        assert_eq!(RiskLevel::High.to_string(), "High");
        assert_eq!(RiskLevel::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_risk_level_from_f64() {
        assert_eq!(RiskLevel::from(0.1), RiskLevel::Low);
        assert_eq!(RiskLevel::from(0.3), RiskLevel::Medium);
        assert_eq!(RiskLevel::from(0.6), RiskLevel::High);
        assert_eq!(RiskLevel::from(0.9), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }
} 