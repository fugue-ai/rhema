use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, sleep};
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, HealthStatus, HealthMetrics, HealthCheckConfig};
use crate::graph::DependencyGraph;

/// Health check result with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Whether the health check was successful
    pub success: bool,
    /// Health check message
    pub message: String,
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// HTTP status code (if applicable)
    pub status_code: Option<u16>,
    /// Health check duration
    pub duration: Duration,
    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Health status with metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatusWithMetrics {
    /// Current health status
    pub status: HealthStatus,
    /// Health metrics
    pub metrics: Option<HealthMetrics>,
    /// Last health check result
    pub last_check: Option<HealthCheckResult>,
    /// Health score (0.0 to 1.0)
    pub health_score: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Alert name
    pub name: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert conditions
    pub conditions: Vec<AlertCondition>,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    /// Cooldown period in seconds
    pub cooldown_seconds: u64,
    /// Last triggered timestamp
    pub last_triggered: Option<DateTime<Utc>>,
}

/// Alert severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    /// Metric name
    pub metric: String,
    /// Operator (>, <, >=, <=, ==, !=)
    pub operator: String,
    /// Threshold value
    pub threshold: f64,
    /// Duration in seconds for condition to be true
    pub duration_seconds: u64,
}

/// Alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    /// Channel type
    pub channel_type: AlertChannelType,
    /// Channel configuration
    pub config: HashMap<String, String>,
}

/// Alert channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannelType {
    Email,
    Slack,
    Webhook,
    PagerDuty,
    Console,
}

/// Health monitoring system
pub struct HealthMonitor {
    /// Dependency graph
    graph: Arc<RwLock<DependencyGraph>>,
    /// Health check configurations
    health_configs: HashMap<String, HealthCheckConfig>,
    /// Alert configurations
    alert_configs: HashMap<String, AlertConfig>,
    /// Health status cache
    health_cache: Arc<RwLock<HashMap<String, HealthStatusWithMetrics>>>,
    /// Monitoring task handle
    monitoring_task: Option<tokio::task::JoinHandle<()>>,
    /// Alert sender
    alert_sender: mpsc::Sender<Alert>,
    /// Alert receiver
    alert_receiver: Option<mpsc::Receiver<Alert>>,
    /// Monitoring configuration
    config: HealthMonitorConfig,
}

/// Health monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    /// Default health check interval in seconds
    pub default_check_interval: u64,
    /// Default health check timeout in seconds
    pub default_timeout: u64,
    /// Enable real-time monitoring
    pub enable_realtime: bool,
    /// Enable alerting
    pub enable_alerting: bool,
    /// Metrics retention period in hours
    pub metrics_retention_hours: u64,
    /// Health score calculation weights
    pub health_score_weights: HealthScoreWeights,
}

/// Health score calculation weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScoreWeights {
    /// Availability weight
    pub availability_weight: f64,
    /// Response time weight
    pub response_time_weight: f64,
    /// Error rate weight
    pub error_rate_weight: f64,
    /// Throughput weight
    pub throughput_weight: f64,
}

impl Default for HealthScoreWeights {
    fn default() -> Self {
        Self {
            availability_weight: 0.4,
            response_time_weight: 0.3,
            error_rate_weight: 0.2,
            throughput_weight: 0.1,
        }
    }
}

/// Alert message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert name
    pub name: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Dependency ID
    pub dependency_id: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert metadata
    pub metadata: HashMap<String, String>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(graph: Arc<RwLock<DependencyGraph>>) -> Self {
        let (alert_sender, alert_receiver) = mpsc::channel(100);
        
        Self {
            graph,
            health_configs: HashMap::new(),
            alert_configs: HashMap::new(),
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            monitoring_task: None,
            alert_sender,
            alert_receiver: Some(alert_receiver),
            config: HealthMonitorConfig::default(),
        }
    }

    /// Create a new health monitor with configuration
    pub fn with_config(graph: Arc<RwLock<DependencyGraph>>, config: HealthMonitorConfig) -> Self {
        let (alert_sender, alert_receiver) = mpsc::channel(100);
        
        Self {
            graph,
            health_configs: HashMap::new(),
            alert_configs: HashMap::new(),
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            monitoring_task: None,
            alert_sender,
            alert_receiver: Some(alert_receiver),
            config,
        }
    }

    /// Add health check configuration
    pub fn add_health_check(&mut self, dependency_id: String, config: HealthCheckConfig) {
        self.health_configs.insert(dependency_id, config);
    }

    /// Add alert configuration
    pub fn add_alert(&mut self, alert_config: AlertConfig) {
        self.alert_configs.insert(alert_config.name.clone(), alert_config);
    }

    /// Start health monitoring
    pub async fn start(&mut self) -> Result<()> {
        if self.monitoring_task.is_some() {
            return Err(Error::HealthMonitoringNotStarted);
        }

        let graph = self.graph.clone();
        let health_cache = self.health_cache.clone();
        let health_configs = self.health_configs.clone();
        let alert_configs = self.alert_configs.clone();
        let config = self.config.clone();
        let alert_sender = self.alert_sender.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.default_check_interval));
            
            loop {
                interval.tick().await;
                
                // Perform health checks for all configured dependencies
                for (dependency_id, health_config) in &health_configs {
                    if let Err(e) = Self::perform_health_check(
                        dependency_id,
                        health_config,
                        &graph,
                        &health_cache,
                        &alert_configs,
                        &alert_sender,
                    ).await {
                        tracing::error!("Health check failed for {}: {}", dependency_id, e);
                    }
                }
            }
        });

        self.monitoring_task = Some(task);
        Ok(())
    }

    /// Stop health monitoring
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(task) = self.monitoring_task.take() {
            task.abort();
        }
        Ok(())
    }

    /// Get health status for a dependency
    pub async fn get_health_status(&self, dependency_id: &str) -> Result<HealthStatusWithMetrics> {
        let cache = self.health_cache.read().await;
        cache
            .get(dependency_id)
            .cloned()
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))
    }

    /// Get health status for all dependencies
    pub async fn get_all_health_statuses(&self) -> HashMap<String, HealthStatusWithMetrics> {
        let cache = self.health_cache.read().await;
        cache.clone()
    }

    /// Perform a manual health check
    pub async fn perform_manual_health_check(&self, dependency_id: &str) -> Result<HealthCheckResult> {
        let health_config = self.health_configs
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        Self::perform_health_check_internal(dependency_id, health_config).await
    }

    /// Perform health check for a dependency
    async fn perform_health_check(
        dependency_id: &str,
        health_config: &HealthCheckConfig,
        graph: &Arc<RwLock<DependencyGraph>>,
        health_cache: &Arc<RwLock<HashMap<String, HealthStatusWithMetrics>>>,
        alert_configs: &HashMap<String, AlertConfig>,
        alert_sender: &mpsc::Sender<Alert>,
    ) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        // Perform the health check
        let health_result = Self::perform_health_check_internal(dependency_id, health_config).await?;
        
        // Calculate health metrics
        let health_metrics = Self::calculate_health_metrics(&health_result);
        
        // Determine health status
        let health_status = Self::determine_health_status(&health_result, &health_metrics);
        
        // Calculate health score
        let health_score = Self::calculate_health_score(&health_metrics);
        
        // Update health cache
        let health_status_with_metrics = HealthStatusWithMetrics {
            status: health_status.clone(),
            metrics: Some(health_metrics.clone()),
            last_check: Some(health_result.clone()),
            health_score,
            last_updated: Utc::now(),
        };

        {
            let mut cache = health_cache.write().await;
            cache.insert(dependency_id.to_string(), health_status_with_metrics);
        }

        // Update graph health status
        {
            let mut graph = graph.write().await;
            graph.update_health_status(dependency_id, health_status)?;
        }

        // Check for alerts
        Self::check_alerts(
            dependency_id,
            &health_result,
            &health_metrics,
            alert_configs,
            alert_sender,
        ).await?;

        Ok(())
    }

    /// Perform internal health check
    async fn perform_health_check_internal(
        dependency_id: &str,
        health_config: &HealthCheckConfig,
    ) -> Result<HealthCheckResult> {
        let start_time = std::time::Instant::now();
        
        // Create HTTP client
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(health_config.timeout_seconds))
            .build()
            .map_err(|e| Error::HealthCheckFailed(e.to_string()))?;

        // Build request
        let mut request = match health_config.method.to_uppercase().as_str() {
            "GET" => client.get(&health_config.url),
            "POST" => client.post(&health_config.url),
            "PUT" => client.put(&health_config.url),
            "DELETE" => client.delete(&health_config.url),
            _ => return Err(Error::HealthCheckFailed("Unsupported HTTP method".to_string())),
        };

        // Add headers
        for (key, value) in &health_config.headers {
            request = request.header(key, value);
        }

        // Add body for POST/PUT requests
        if let Some(body) = &health_config.body {
            request = request.body(body.clone());
        }

        // Execute request
        let response = request.send().await
            .map_err(|e| Error::HealthCheckFailed(e.to_string()))?;

        let status_code = response.status().as_u16();
        let response_time = start_time.elapsed();
        let success = status_code == health_config.expected_status;

        let message = if success {
            "Health check passed".to_string()
        } else {
            format!("Health check failed: expected status {}, got {}", 
                health_config.expected_status, status_code)
        };

        Ok(HealthCheckResult {
            success,
            message,
            response_time_ms: response_time.as_millis() as f64,
            status_code: Some(status_code),
            duration: response_time,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        })
    }

    /// Calculate health metrics from health check result
    fn calculate_health_metrics(health_result: &HealthCheckResult) -> HealthMetrics {
        let availability = if health_result.success { 1.0 } else { 0.0 };
        let error_rate = if health_result.success { 0.0 } else { 1.0 };
        
        HealthMetrics::new(
            health_result.response_time_ms,
            availability,
            error_rate,
            1.0, // Default throughput
            0.5, // Default CPU usage
            0.5, // Default memory usage
            10.0, // Default network latency
            0.5, // Default disk usage
        ).unwrap_or_else(|_| {
            // Fallback metrics if validation fails
            HealthMetrics {
                response_time_ms: health_result.response_time_ms,
                availability,
                error_rate,
                throughput: 1.0,
                cpu_usage: 0.5,
                memory_usage: 0.5,
                network_latency_ms: 10.0,
                disk_usage: 0.5,
                timestamp: Utc::now(),
            }
        })
    }

    /// Determine health status from health check result and metrics
    fn determine_health_status(
        health_result: &HealthCheckResult,
        metrics: &HealthMetrics,
    ) -> HealthStatus {
        if !health_result.success {
            return HealthStatus::Down;
        }

        let health_score = metrics.health_score();
        HealthStatus::from(health_score)
    }

    /// Calculate health score from metrics
    fn calculate_health_score(metrics: &HealthMetrics) -> f64 {
        metrics.health_score()
    }

    /// Check for alerts
    async fn check_alerts(
        dependency_id: &str,
        health_result: &HealthCheckResult,
        health_metrics: &HealthMetrics,
        alert_configs: &HashMap<String, AlertConfig>,
        alert_sender: &mpsc::Sender<Alert>,
    ) -> Result<()> {
        for (alert_name, alert_config) in alert_configs {
            // Check if alert should be triggered
            if Self::should_trigger_alert(alert_config, health_result, health_metrics) {
                // Check cooldown
                if let Some(last_triggered) = alert_config.last_triggered {
                    let cooldown_duration = Duration::from_secs(alert_config.cooldown_seconds);
                    let time_since_last = Utc::now() - last_triggered;
                    if time_since_last < chrono::Duration::from_std(cooldown_duration).unwrap() {
                        continue; // Still in cooldown
                    }
                }

                // Send alert
                let alert = Alert {
                    name: alert_name.clone(),
                    severity: alert_config.severity.clone(),
                    message: format!("Alert triggered for {}: {}", dependency_id, alert_name),
                    dependency_id: dependency_id.to_string(),
                    timestamp: Utc::now(),
                    metadata: HashMap::new(),
                };

                if let Err(e) = alert_sender.send(alert).await {
                    tracing::error!("Failed to send alert: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Check if alert should be triggered
    fn should_trigger_alert(
        alert_config: &AlertConfig,
        health_result: &HealthCheckResult,
        health_metrics: &HealthMetrics,
    ) -> bool {
        for condition in &alert_config.conditions {
            let metric_value = match condition.metric.as_str() {
                "response_time" => health_metrics.response_time_ms,
                "availability" => health_metrics.availability,
                "error_rate" => health_metrics.error_rate,
                "throughput" => health_metrics.throughput,
                "cpu_usage" => health_metrics.cpu_usage,
                "memory_usage" => health_metrics.memory_usage,
                "network_latency" => health_metrics.network_latency_ms,
                "disk_usage" => health_metrics.disk_usage,
                _ => continue,
            };

            let should_trigger = match condition.operator.as_str() {
                ">" => metric_value > condition.threshold,
                "<" => metric_value < condition.threshold,
                ">=" => metric_value >= condition.threshold,
                "<=" => metric_value <= condition.threshold,
                "==" => (metric_value - condition.threshold).abs() < f64::EPSILON,
                "!=" => (metric_value - condition.threshold).abs() >= f64::EPSILON,
                _ => false,
            };

            if should_trigger {
                return true;
            }
        }

        false
    }

    /// Get monitoring statistics
    pub async fn get_statistics(&self) -> HealthMonitorStatistics {
        let cache = self.health_cache.read().await;
        
        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;
        let mut down_count = 0;
        let mut unknown_count = 0;
        let mut total_health_score = 0.0;

        for health_status in cache.values() {
            match health_status.status {
                HealthStatus::Healthy => healthy_count += 1,
                HealthStatus::Degraded => degraded_count += 1,
                HealthStatus::Unhealthy => unhealthy_count += 1,
                HealthStatus::Down => down_count += 1,
                HealthStatus::Unknown => unknown_count += 1,
            }
            total_health_score += health_status.health_score;
        }

        let total_count = cache.len();
        let average_health_score = if total_count > 0 {
            total_health_score / total_count as f64
        } else {
            0.0
        };

        HealthMonitorStatistics {
            total_dependencies: total_count,
            healthy_count,
            degraded_count,
            unhealthy_count,
            down_count,
            unknown_count,
            average_health_score,
            last_updated: Utc::now(),
        }
    }
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            default_check_interval: 30,
            default_timeout: 10,
            enable_realtime: true,
            enable_alerting: true,
            metrics_retention_hours: 24,
            health_score_weights: HealthScoreWeights::default(),
        }
    }
}

/// Health monitor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorStatistics {
    /// Total number of dependencies
    pub total_dependencies: usize,
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
    /// Average health score
    pub average_health_score: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    fn create_test_config(id: &str, name: &str) -> DependencyConfig {
        DependencyConfig::new(
            id.to_string(),
            name.to_string(),
            crate::types::DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).unwrap()
    }

    #[test]
    fn test_health_monitor_new() {
        let graph = Arc::new(RwLock::new(crate::graph::DependencyGraph::new()));
        let monitor = HealthMonitor::new(graph);
        assert_eq!(monitor.health_configs.len(), 0);
        assert_eq!(monitor.alert_configs.len(), 0);
    }

    #[test]
    fn test_health_monitor_config_default() {
        let config = HealthMonitorConfig::default();
        assert_eq!(config.default_check_interval, 30);
        assert_eq!(config.default_timeout, 10);
        assert!(config.enable_realtime);
        assert!(config.enable_alerting);
    }

    #[test]
    fn test_health_score_weights_default() {
        let weights = HealthScoreWeights::default();
        assert_eq!(weights.availability_weight, 0.4);
        assert_eq!(weights.response_time_weight, 0.3);
        assert_eq!(weights.error_rate_weight, 0.2);
        assert_eq!(weights.throughput_weight, 0.1);
    }

    #[test]
    fn test_alert_severity() {
        assert_eq!(AlertSeverity::Info, AlertSeverity::Info);
        assert_ne!(AlertSeverity::Info, AlertSeverity::Critical);
    }

    #[test]
    fn test_should_trigger_alert() {
        let health_result = HealthCheckResult {
            success: true,
            message: "OK".to_string(),
            response_time_ms: 150.0,
            status_code: Some(200),
            duration: Duration::from_millis(150),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };

        let health_metrics = HealthMetrics::new(
            150.0, 1.0, 0.0, 100.0, 0.5, 0.5, 10.0, 0.5
        ).unwrap();

        let alert_config = AlertConfig {
            name: "high_latency".to_string(),
            severity: AlertSeverity::Warning,
            conditions: vec![
                AlertCondition {
                    metric: "response_time".to_string(),
                    operator: ">".to_string(),
                    threshold: 100.0,
                    duration_seconds: 60,
                }
            ],
            channels: vec![],
            cooldown_seconds: 300,
            last_triggered: None,
        };

        assert!(HealthMonitor::should_trigger_alert(&alert_config, &health_result, &health_metrics));
    }
} 