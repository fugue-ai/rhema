use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::error::{Error, Result, RiskLevel};

/// Semantic dependency types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyType {
    /// Data flow dependencies - dependencies on data sources, databases, etc.
    DataFlow,
    /// API call dependencies - external API services
    ApiCall,
    /// Infrastructure dependencies - servers, networks, cloud services
    Infrastructure,
    /// Business logic dependencies - business processes and workflows
    BusinessLogic,
    /// Security dependencies - authentication, authorization, encryption
    Security,
    /// Monitoring dependencies - logging, metrics, alerting
    Monitoring,
    /// Configuration dependencies - configuration management
    Configuration,
    /// Deployment dependencies - CI/CD, containers, orchestration
    Deployment,
}

impl std::fmt::Display for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyType::DataFlow => write!(f, "DataFlow"),
            DependencyType::ApiCall => write!(f, "ApiCall"),
            DependencyType::Infrastructure => write!(f, "Infrastructure"),
            DependencyType::BusinessLogic => write!(f, "BusinessLogic"),
            DependencyType::Security => write!(f, "Security"),
            DependencyType::Monitoring => write!(f, "Monitoring"),
            DependencyType::Configuration => write!(f, "Configuration"),
            DependencyType::Deployment => write!(f, "Deployment"),
        }
    }
}

/// Health status of a dependency
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Dependency is healthy and functioning normally
    Healthy,
    /// Dependency has minor issues but is still operational
    Degraded,
    /// Dependency is experiencing significant issues
    Unhealthy,
    /// Dependency is completely down or unreachable
    Down,
    /// Health status is unknown
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Unhealthy => write!(f, "Unhealthy"),
            HealthStatus::Down => write!(f, "Down"),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<f64> for HealthStatus {
    fn from(score: f64) -> Self {
        match score {
            s if s >= 0.9 => HealthStatus::Healthy,
            s if s >= 0.7 => HealthStatus::Degraded,
            s if s >= 0.3 => HealthStatus::Unhealthy,
            _ => HealthStatus::Down,
        }
    }
}

/// Impact score for business impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactScore {
    /// Overall business impact score (0.0 to 1.0)
    pub business_impact: f64,
    /// Revenue impact score (0.0 to 1.0)
    pub revenue_impact: f64,
    /// User experience impact score (0.0 to 1.0)
    pub user_experience_impact: f64,
    /// Operational cost impact score (0.0 to 1.0)
    pub operational_cost_impact: f64,
    /// Security impact score (0.0 to 1.0)
    pub security_impact: f64,
    /// Compliance impact score (0.0 to 1.0)
    pub compliance_impact: f64,
    /// Risk level based on impact scores
    pub risk_level: RiskLevel,
    /// Timestamp of the impact assessment
    pub timestamp: DateTime<Utc>,
}

impl ImpactScore {
    /// Create a new impact score
    pub fn new(
        business_impact: f64,
        revenue_impact: f64,
        user_experience_impact: f64,
        operational_cost_impact: f64,
        security_impact: f64,
        compliance_impact: f64,
    ) -> Result<Self> {
        // Validate scores are between 0.0 and 1.0
        let scores = [
            business_impact,
            revenue_impact,
            user_experience_impact,
            operational_cost_impact,
            security_impact,
            compliance_impact,
        ];

        for score in scores {
            if !(0.0..=1.0).contains(&score) {
                return Err(Error::InvalidImpactScore(format!(
                    "Score {} is not between 0.0 and 1.0",
                    score
                )));
            }
        }

        let risk_level = RiskLevel::from(business_impact);

        Ok(Self {
            business_impact,
            revenue_impact,
            user_experience_impact,
            operational_cost_impact,
            security_impact,
            compliance_impact,
            risk_level,
            timestamp: Utc::now(),
        })
    }

    /// Calculate the weighted average impact score
    pub fn weighted_average(&self) -> f64 {
        // Weighted average with business impact having highest weight
        (self.business_impact * 0.3
            + self.revenue_impact * 0.25
            + self.user_experience_impact * 0.2
            + self.operational_cost_impact * 0.15
            + self.security_impact * 0.05
            + self.compliance_impact * 0.05)
    }
}

/// Health metrics for a dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// Availability percentage (0.0 to 1.0)
    pub availability: f64,
    /// Error rate percentage (0.0 to 1.0)
    pub error_rate: f64,
    /// Throughput (requests per second)
    pub throughput: f64,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Network latency in milliseconds
    pub network_latency_ms: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Timestamp of the metrics
    pub timestamp: DateTime<Utc>,
}

impl HealthMetrics {
    /// Create new health metrics
    pub fn new(
        response_time_ms: f64,
        availability: f64,
        error_rate: f64,
        throughput: f64,
        cpu_usage: f64,
        memory_usage: f64,
        network_latency_ms: f64,
        disk_usage: f64,
    ) -> Result<Self> {
        // Validate percentages are between 0.0 and 1.0
        let percentages = [
            availability,
            error_rate,
            cpu_usage,
            memory_usage,
            disk_usage,
        ];
        for percentage in percentages {
            if !(0.0..=1.0).contains(&percentage) {
                return Err(Error::InvalidHealthStatus(format!(
                    "Percentage {} is not between 0.0 and 1.0",
                    percentage
                )));
            }
        }

        // Validate positive values
        if response_time_ms < 0.0 || throughput < 0.0 || network_latency_ms < 0.0 {
            return Err(Error::InvalidHealthStatus(
                "Negative values not allowed".to_string(),
            ));
        }

        Ok(Self {
            response_time_ms,
            availability,
            error_rate,
            throughput,
            cpu_usage,
            memory_usage,
            network_latency_ms,
            disk_usage,
            timestamp: Utc::now(),
        })
    }

    /// Calculate overall health score
    pub fn health_score(&self) -> f64 {
        let availability_score = self.availability;
        let error_score = 1.0 - self.error_rate;
        let performance_score = if self.response_time_ms < 100.0 {
            1.0
        } else if self.response_time_ms < 500.0 {
            0.8
        } else if self.response_time_ms < 1000.0 {
            0.6
        } else {
            0.2
        };

        (availability_score * 0.5 + error_score * 0.3 + performance_score * 0.2)
    }
}

/// Dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DependencyConfig {
    /// Unique identifier for the dependency
    #[validate(length(min = 1))]
    pub id: String,
    /// Human-readable name
    #[validate(length(min = 1))]
    pub name: String,
    /// Description of the dependency
    pub description: Option<String>,
    /// Type of dependency
    pub dependency_type: DependencyType,
    /// Target service or resource
    #[validate(length(min = 1))]
    pub target: String,
    /// Operations that can be performed on this dependency
    pub operations: Vec<String>,
    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
    /// Impact assessment configuration
    pub impact_config: Option<ImpactConfig>,
    /// Security requirements
    pub security_requirements: Option<SecurityRequirements>,
    /// Performance requirements
    pub performance_requirements: Option<PerformanceRequirements>,
    /// Metadata
    pub metadata: HashMap<String, String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Current health status
    pub health_status: HealthStatus,
}

impl DependencyConfig {
    /// Create a new dependency configuration
    pub fn new(
        id: String,
        name: String,
        dependency_type: DependencyType,
        target: String,
        operations: Vec<String>,
    ) -> Result<Self> {
        let config = Self {
            id,
            name,
            description: None,
            dependency_type,
            target,
            operations,
            health_check: None,
            impact_config: None,
            security_requirements: None,
            performance_requirements: None,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            health_status: HealthStatus::Unknown,
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        Validate::validate(self)
            .map_err(|e| Error::Validation(format!("Configuration validation failed: {}", e)))?;
        Ok(())
    }
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check URL
    pub url: String,
    /// Health check interval in seconds
    pub interval_seconds: u64,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Expected HTTP status code
    pub expected_status: u16,
    /// Health check method (GET, POST, etc.)
    pub method: String,
    /// Headers to include in health check
    pub headers: HashMap<String, String>,
    /// Body to include in health check (for POST requests)
    pub body: Option<String>,
}

/// Impact assessment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactConfig {
    /// Business impact weight (0.0 to 1.0)
    pub business_impact_weight: f64,
    /// Revenue impact weight (0.0 to 1.0)
    pub revenue_impact_weight: f64,
    /// User experience impact weight (0.0 to 1.0)
    pub user_experience_impact_weight: f64,
    /// Operational cost impact weight (0.0 to 1.0)
    pub operational_cost_impact_weight: f64,
    /// Security impact weight (0.0 to 1.0)
    pub security_impact_weight: f64,
    /// Compliance impact weight (0.0 to 1.0)
    pub compliance_impact_weight: f64,
    /// Critical business functions that depend on this
    pub critical_functions: Vec<String>,
    /// Estimated cost per hour of downtime
    pub cost_per_hour: f64,
}

/// Security requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Authentication required
    pub authentication_required: bool,
    /// Authorization required
    pub authorization_required: bool,
    /// Encryption required
    pub encryption_required: bool,
    /// Audit logging required
    pub audit_logging_required: bool,
    /// Compliance standards
    pub compliance_standards: Vec<String>,
    /// Security level (Low, Medium, High, Critical)
    pub security_level: String,
}

/// Performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Maximum response time in milliseconds
    pub max_response_time_ms: f64,
    /// Minimum availability percentage
    pub min_availability: f64,
    /// Maximum error rate
    pub max_error_rate: f64,
    /// Minimum throughput (requests per second)
    pub min_throughput: f64,
    /// SLA level
    pub sla_level: String,
}

/// Dependency operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyOperation {
    /// Operation name
    pub name: String,
    /// Operation type
    pub operation_type: String,
    /// Parameters for the operation
    pub parameters: HashMap<String, String>,
    /// Expected response format
    pub expected_response: Option<String>,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay_seconds: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum retry delay in seconds
    pub max_retry_delay_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_type_display() {
        assert_eq!(DependencyType::DataFlow.to_string(), "DataFlow");
        assert_eq!(DependencyType::ApiCall.to_string(), "ApiCall");
        assert_eq!(DependencyType::Infrastructure.to_string(), "Infrastructure");
    }

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "Healthy");
        assert_eq!(HealthStatus::Degraded.to_string(), "Degraded");
        assert_eq!(HealthStatus::Unhealthy.to_string(), "Unhealthy");
        assert_eq!(HealthStatus::Down.to_string(), "Down");
    }

    #[test]
    fn test_health_status_from_f64() {
        assert_eq!(HealthStatus::from(0.95), HealthStatus::Healthy);
        assert_eq!(HealthStatus::from(0.8), HealthStatus::Degraded);
        assert_eq!(HealthStatus::from(0.5), HealthStatus::Unhealthy);
        assert_eq!(HealthStatus::from(0.1), HealthStatus::Down);
    }

    #[test]
    fn test_impact_score_new() {
        let impact = ImpactScore::new(0.8, 0.7, 0.6, 0.5, 0.4, 0.3).unwrap();
        assert_eq!(impact.business_impact, 0.8);
        assert_eq!(impact.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_impact_score_invalid() {
        let result = ImpactScore::new(1.5, 0.7, 0.6, 0.5, 0.4, 0.3);
        assert!(result.is_err());
    }

    #[test]
    fn test_health_metrics_new() {
        let metrics = HealthMetrics::new(100.0, 0.99, 0.01, 1000.0, 0.5, 0.6, 10.0, 0.7).unwrap();
        assert_eq!(metrics.response_time_ms, 100.0);
        assert_eq!(metrics.availability, 0.99);
    }

    #[test]
    fn test_health_metrics_invalid() {
        let result = HealthMetrics::new(100.0, 1.5, 0.01, 1000.0, 0.5, 0.6, 10.0, 0.7);
        assert!(result.is_err());
    }

    #[test]
    fn test_dependency_config_new() {
        let config = DependencyConfig::new(
            "test-id".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://api.example.com".to_string(),
            vec!["GET".to_string(), "POST".to_string()],
        )
        .unwrap();
        assert_eq!(config.id, "test-id");
        assert_eq!(config.name, "Test Dependency");
    }

    #[test]
    fn test_health_metrics_health_score() {
        let metrics = HealthMetrics::new(50.0, 0.99, 0.01, 1000.0, 0.5, 0.6, 10.0, 0.7).unwrap();
        let score = metrics.health_score();
        assert!(score > 0.9); // Should be high for good metrics
    }
}
