use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::HealthScoreWeights;

/// Main configuration for the dependency management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Health monitoring configuration
    pub health_monitoring: HealthMonitoringConfig,
    /// Validation configuration
    pub validation: ValidationConfig,
    /// Impact analysis configuration
    pub impact_analysis: ImpactAnalysisConfig,
    /// Real-time monitoring configuration
    pub realtime: RealtimeConfig,
    /// Alerting configuration
    pub alerting: AlertingConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database type
    pub database_type: DatabaseType,
    /// Connection string
    pub connection_string: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Query timeout in seconds
    pub query_timeout: u64,
    /// Enable migrations
    pub enable_migrations: bool,
    /// Migration directory
    pub migration_directory: Option<String>,
}

/// Database type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    MySQL,
    InMemory,
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Sqlite => write!(f, "SQLite"),
            DatabaseType::Postgres => write!(f, "PostgreSQL"),
            DatabaseType::MySQL => write!(f, "MySQL"),
            DatabaseType::InMemory => write!(f, "In-Memory"),
        }
    }
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Health check timeout in seconds
    pub health_check_timeout: u64,
    /// Enable real-time monitoring
    pub enable_realtime_monitoring: bool,
    /// Enable alerting
    pub enable_alerting: bool,
    /// Metrics retention period in hours
    pub metrics_retention_hours: u64,
    /// Health score calculation weights
    pub health_score_weights: HealthScoreWeights,
    /// Health check retry configuration
    pub retry_config: RetryConfig,
    /// Health check backoff configuration
    pub backoff_config: BackoffConfig,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
    /// Exponential backoff
    pub exponential_backoff: bool,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum retry delay in seconds
    pub max_retry_delay: u64,
}

/// Backoff configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackoffConfig {
    /// Initial delay in seconds
    pub initial_delay: u64,
    /// Maximum delay in seconds
    pub max_delay: u64,
    /// Backoff multiplier
    pub multiplier: f64,
    /// Jitter factor (0.0 to 1.0)
    pub jitter: f64,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable validation caching
    pub enable_validation_caching: bool,
    /// Validation cache TTL in seconds
    pub validation_cache_ttl: u64,
    /// Maximum validation errors to report
    pub max_validation_errors: usize,
    /// Maximum validation warnings to report
    pub max_validation_warnings: usize,
    /// Enable parallel validation
    pub enable_parallel_validation: bool,
    /// Validation timeout in seconds
    pub validation_timeout: u64,
    /// Validation rules configuration
    pub rules: HashMap<String, ValidationRuleConfig>,
}

/// Validation rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRuleConfig {
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Rule severity
    pub severity: ValidationSeverity,
    /// Rule parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Validation severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Impact analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisConfig {
    /// Business impact metrics weights
    pub business_impact_weights: BusinessImpactWeights,
    /// Risk factor weights
    pub risk_factor_weights: RiskFactorWeights,
    /// Cost calculation configuration
    pub cost_calculation: CostCalculationConfig,
    /// Historical data configuration
    pub historical_data: HistoricalDataConfig,
}

/// Business impact weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpactWeights {
    /// Revenue impact weight
    pub revenue_weight: f64,
    /// User experience impact weight
    pub user_experience_weight: f64,
    /// Operational cost impact weight
    pub operational_cost_weight: f64,
    /// Security impact weight
    pub security_weight: f64,
    /// Compliance impact weight
    pub compliance_weight: f64,
    /// Brand reputation impact weight
    pub brand_reputation_weight: f64,
}

/// Risk factor weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorWeights {
    /// Availability risk weight
    pub availability_weight: f64,
    /// Performance risk weight
    pub performance_weight: f64,
    /// Security risk weight
    pub security_weight: f64,
    /// Scalability risk weight
    pub scalability_weight: f64,
    /// Maintainability risk weight
    pub maintainability_weight: f64,
    /// Compliance risk weight
    pub compliance_weight: f64,
}

/// Cost calculation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCalculationConfig {
    /// Default cost per hour of downtime
    pub default_cost_per_hour: f64,
    /// Cost multipliers by dependency type
    pub cost_multipliers: HashMap<String, f64>,
    /// Currency
    pub currency: String,
    /// Include indirect costs
    pub include_indirect_costs: bool,
    /// Indirect cost multiplier
    pub indirect_cost_multiplier: f64,
}

/// Historical data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataConfig {
    /// Enable historical data collection
    pub enable_collection: bool,
    /// Data retention period in days
    pub retention_period_days: u64,
    /// Data aggregation interval in minutes
    pub aggregation_interval_minutes: u64,
    /// Enable trend analysis
    pub enable_trend_analysis: bool,
    /// Trend analysis window in days
    pub trend_analysis_window_days: u64,
}

/// Real-time monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeConfig {
    /// Enable real-time updates
    pub enable_realtime: bool,
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// Event streaming configuration
    pub event_streaming: EventStreamingConfig,
    /// Update frequency in seconds
    pub update_frequency: u64,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket server address
    pub server_address: String,
    /// WebSocket server port
    pub server_port: u16,
    /// Enable SSL/TLS
    pub enable_ssl: bool,
    /// SSL certificate path
    pub ssl_certificate_path: Option<String>,
    /// SSL private key path
    pub ssl_private_key_path: Option<String>,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

/// Event streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStreamingConfig {
    /// Enable event streaming
    pub enable_streaming: bool,
    /// Stream buffer size
    pub buffer_size: usize,
    /// Stream batch size
    pub batch_size: usize,
    /// Stream flush interval in seconds
    pub flush_interval: u64,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enable_alerting: bool,
    /// Alert channels
    pub channels: HashMap<String, AlertChannelConfig>,
    /// Alert rules
    pub rules: HashMap<String, AlertRuleConfig>,
    /// Alert cooldown in seconds
    pub cooldown_seconds: u64,
    /// Maximum alerts per minute
    pub max_alerts_per_minute: u32,
}

/// Alert channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannelConfig {
    /// Channel type
    pub channel_type: AlertChannelType,
    /// Channel enabled
    pub enabled: bool,
    /// Channel configuration
    pub config: HashMap<String, String>,
    /// Channel timeout in seconds
    pub timeout: u64,
    /// Channel retry configuration
    pub retry: RetryConfig,
}

/// Alert channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannelType {
    Email,
    Slack,
    Webhook,
    PagerDuty,
    Console,
    Custom(String),
}

/// Alert rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRuleConfig {
    /// Rule enabled
    pub enabled: bool,
    /// Rule severity
    pub severity: AlertSeverity,
    /// Rule conditions
    pub conditions: Vec<AlertConditionConfig>,
    /// Rule channels
    pub channels: Vec<String>,
    /// Rule cooldown in seconds
    pub cooldown: u64,
}

/// Alert condition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConditionConfig {
    /// Metric name
    pub metric: String,
    /// Operator
    pub operator: String,
    /// Threshold value
    pub threshold: f64,
    /// Duration in seconds
    pub duration: u64,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Prometheus configuration
    pub prometheus: PrometheusConfig,
    /// Metrics retention in hours
    pub retention_hours: u64,
    /// Metrics aggregation interval in minutes
    pub aggregation_interval_minutes: u64,
    /// Custom metrics
    pub custom_metrics: HashMap<String, CustomMetricConfig>,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Metrics port
    pub port: u16,
    /// Enable default metrics
    pub enable_default_metrics: bool,
    /// Metrics prefix
    pub prefix: String,
}

/// Custom metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetricConfig {
    /// Metric type
    pub metric_type: MetricType,
    /// Metric description
    pub description: String,
    /// Metric labels
    pub labels: Vec<String>,
    /// Metric enabled
    pub enabled: bool,
}

/// Metric type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub enable_authentication: bool,
    /// Authentication method
    pub authentication_method: AuthenticationMethod,
    /// API key configuration
    pub api_key: ApiKeyConfig,
    /// JWT configuration
    pub jwt: JwtConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
    /// CORS configuration
    pub cors: CorsConfig,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    None,
    ApiKey,
    Jwt,
    OAuth2,
    Custom(String),
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// API key header name
    pub header_name: String,
    /// API key query parameter name
    pub query_param_name: String,
    /// Valid API keys
    pub valid_keys: Vec<String>,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT secret
    pub secret: String,
    /// JWT issuer
    pub issuer: String,
    /// JWT audience
    pub audience: String,
    /// JWT expiration in seconds
    pub expiration: u64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Rate limit window in seconds
    pub window_seconds: u64,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub log_level: LogLevel,
    /// Log format
    pub log_format: LogFormat,
    /// Log file path
    pub log_file_path: Option<String>,
    /// Enable console logging
    pub enable_console: bool,
    /// Enable file logging
    pub enable_file: bool,
    /// Log rotation configuration
    pub rotation: LogRotationConfig,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Text,
    Json,
    Structured,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Enable log rotation
    pub enabled: bool,
    /// Maximum file size in MB
    pub max_file_size_mb: u64,
    /// Maximum number of files
    pub max_files: u32,
    /// Rotation interval
    pub rotation_interval: LogRotationInterval,
}

/// Log rotation interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogRotationInterval {
    Daily,
    Weekly,
    Monthly,
    Custom(u64), // Custom interval in seconds
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Worker thread count
    pub worker_threads: usize,
    /// Async runtime configuration
    pub async_runtime: AsyncRuntimeConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Connection pool configuration
    pub connection_pool: ConnectionPoolConfig,
}

/// Async runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncRuntimeConfig {
    /// Enable multi-threaded runtime
    pub enable_multi_threaded: bool,
    /// Worker thread count
    pub worker_threads: usize,
    /// Max blocking threads
    pub max_blocking_threads: usize,
    /// Thread stack size in KB
    pub thread_stack_size_kb: usize,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size
    pub size: usize,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Cache eviction policy
    pub eviction_policy: CacheEvictionPolicy,
}

/// Cache eviction policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Pool size
    pub size: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Idle timeout in seconds
    pub idle_timeout: u64,
    /// Max lifetime in seconds
    pub max_lifetime: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            validation: ValidationConfig::default(),
            impact_analysis: ImpactAnalysisConfig::default(),
            realtime: RealtimeConfig::default(),
            alerting: AlertingConfig::default(),
            metrics: MetricsConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: DatabaseType::Sqlite,
            connection_string: "sqlite::memory:".to_string(),
            pool_size: 10,
            connection_timeout: 30,
            query_timeout: 60,
            enable_migrations: true,
            migration_directory: None,
        }
    }
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            health_check_interval: 30,
            health_check_timeout: 10,
            enable_realtime_monitoring: true,
            enable_alerting: true,
            metrics_retention_hours: 24,
            health_score_weights: HealthScoreWeights::default(),
            retry_config: RetryConfig::default(),
            backoff_config: BackoffConfig::default(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: 1,
            exponential_backoff: true,
            backoff_multiplier: 2.0,
            max_retry_delay: 60,
        }
    }
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            initial_delay: 1,
            max_delay: 60,
            multiplier: 2.0,
            jitter: 0.1,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enable_validation_caching: true,
            validation_cache_ttl: 300,
            max_validation_errors: 100,
            max_validation_warnings: 200,
            enable_parallel_validation: true,
            validation_timeout: 30,
            rules: HashMap::new(),
        }
    }
}

impl Default for ImpactAnalysisConfig {
    fn default() -> Self {
        Self {
            business_impact_weights: BusinessImpactWeights::default(),
            risk_factor_weights: RiskFactorWeights::default(),
            cost_calculation: CostCalculationConfig::default(),
            historical_data: HistoricalDataConfig::default(),
        }
    }
}

impl Default for BusinessImpactWeights {
    fn default() -> Self {
        Self {
            revenue_weight: 0.3,
            user_experience_weight: 0.25,
            operational_cost_weight: 0.2,
            security_weight: 0.15,
            compliance_weight: 0.05,
            brand_reputation_weight: 0.05,
        }
    }
}

impl Default for RiskFactorWeights {
    fn default() -> Self {
        Self {
            availability_weight: 0.3,
            performance_weight: 0.25,
            security_weight: 0.2,
            scalability_weight: 0.15,
            maintainability_weight: 0.05,
            compliance_weight: 0.05,
        }
    }
}

impl Default for CostCalculationConfig {
    fn default() -> Self {
        Self {
            default_cost_per_hour: 1000.0,
            cost_multipliers: HashMap::new(),
            currency: "USD".to_string(),
            include_indirect_costs: true,
            indirect_cost_multiplier: 1.5,
        }
    }
}

impl Default for HistoricalDataConfig {
    fn default() -> Self {
        Self {
            enable_collection: true,
            retention_period_days: 30,
            aggregation_interval_minutes: 5,
            enable_trend_analysis: true,
            trend_analysis_window_days: 7,
        }
    }
}

impl Default for RealtimeConfig {
    fn default() -> Self {
        Self {
            enable_realtime: true,
            websocket: WebSocketConfig::default(),
            event_streaming: EventStreamingConfig::default(),
            update_frequency: 5,
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1".to_string(),
            server_port: 8080,
            enable_ssl: false,
            ssl_certificate_path: None,
            ssl_private_key_path: None,
            max_connections: 1000,
            connection_timeout: 30,
        }
    }
}

impl Default for EventStreamingConfig {
    fn default() -> Self {
        Self {
            enable_streaming: true,
            buffer_size: 1000,
            batch_size: 100,
            flush_interval: 5,
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enable_alerting: true,
            channels: HashMap::new(),
            rules: HashMap::new(),
            cooldown_seconds: 300,
            max_alerts_per_minute: 10,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            prometheus: PrometheusConfig::default(),
            retention_hours: 24,
            aggregation_interval_minutes: 5,
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            port: 9090,
            enable_default_metrics: true,
            prefix: "rhema_dependency".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_authentication: false,
            authentication_method: AuthenticationMethod::None,
            api_key: ApiKeyConfig::default(),
            jwt: JwtConfig::default(),
            rate_limiting: RateLimitingConfig::default(),
            cors: CorsConfig::default(),
        }
    }
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            header_name: "X-API-Key".to_string(),
            query_param_name: "api_key".to_string(),
            valid_keys: Vec::new(),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "your-secret-key".to_string(),
            issuer: "rhema-dependency".to_string(),
            audience: "rhema-dependency".to_string(),
            expiration: 3600,
        }
    }
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 100,
            burst_size: 20,
            window_seconds: 60,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            log_format: LogFormat::Text,
            log_file_path: None,
            enable_console: true,
            enable_file: false,
            rotation: LogRotationConfig::default(),
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_file_size_mb: 100,
            max_files: 5,
            rotation_interval: LogRotationInterval::Daily,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            async_runtime: AsyncRuntimeConfig::default(),
            cache: CacheConfig::default(),
            connection_pool: ConnectionPoolConfig::default(),
        }
    }
}

impl Default for AsyncRuntimeConfig {
    fn default() -> Self {
        Self {
            enable_multi_threaded: true,
            worker_threads: num_cpus::get(),
            max_blocking_threads: 512,
            thread_stack_size_kb: 2048,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 1000,
            ttl_seconds: 300,
            eviction_policy: CacheEvictionPolicy::Lru,
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            size: 10,
            connection_timeout: 30,
            idle_timeout: 300,
            max_lifetime: 3600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.database.database_type, DatabaseType::Sqlite);
        assert_eq!(config.health_monitoring.health_check_interval, 30);
        assert!(config.realtime.enable_realtime);
        assert!(config.alerting.enable_alerting);
    }

    #[test]
    fn test_database_config_default() {
        let db_config = DatabaseConfig::default();
        assert_eq!(db_config.database_type, DatabaseType::Sqlite);
        assert_eq!(db_config.connection_string, "sqlite::memory:");
        assert_eq!(db_config.pool_size, 10);
    }

    #[test]
    fn test_health_monitoring_config_default() {
        let health_config = HealthMonitoringConfig::default();
        assert_eq!(health_config.health_check_interval, 30);
        assert_eq!(health_config.health_check_timeout, 10);
        assert!(health_config.enable_realtime_monitoring);
        assert!(health_config.enable_alerting);
    }

    #[test]
    fn test_validation_config_default() {
        let validation_config = ValidationConfig::default();
        assert!(validation_config.enable_validation_caching);
        assert_eq!(validation_config.validation_cache_ttl, 300);
        assert_eq!(validation_config.max_validation_errors, 100);
        assert_eq!(validation_config.max_validation_warnings, 200);
    }

    #[test]
    fn test_business_impact_weights_default() {
        let weights = BusinessImpactWeights::default();
        assert_eq!(weights.revenue_weight, 0.3);
        assert_eq!(weights.user_experience_weight, 0.25);
        assert_eq!(weights.operational_cost_weight, 0.2);
    }

    #[test]
    fn test_risk_factor_weights_default() {
        let weights = RiskFactorWeights::default();
        assert_eq!(weights.availability_weight, 0.3);
        assert_eq!(weights.performance_weight, 0.25);
        assert_eq!(weights.security_weight, 0.2);
    }

    #[test]
    fn test_serialization() {
        let config = Config::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config.database.database_type, deserialized.database.database_type);
        assert_eq!(config.health_monitoring.health_check_interval, deserialized.health_monitoring.health_check_interval);
    }
} 