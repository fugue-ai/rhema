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

//! Monitoring and Observability for gRPC Coordination Client
//!
//! This module provides comprehensive monitoring, metrics collection, and observability
//! features for the gRPC coordination client. It includes:
//!
//! - Real-time metrics collection and aggregation
//! - Health monitoring and alerting
//! - Performance monitoring and benchmarking
//! - Connection monitoring and diagnostics
//! - Structured logging and tracing
//! - Metrics export for external monitoring systems

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::coordination_client::{ClientMetrics, ConnectionStatus, CoordinationError};

/// Health status for the coordination client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Detailed health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthInfo {
    pub status: HealthStatus,
    pub timestamp: u64,
    pub message: String,
    pub details: HashMap<String, String>,
    pub last_successful_operation: Option<u64>,
    pub connection_uptime: Option<Duration>,
    pub response_time_p95: Option<Duration>,
    pub error_rate: f64,
}

/// Performance metrics for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_name: String,
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_response_time: Duration,
    pub p50_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub throughput_ops_per_second: f64,
    pub error_rate: f64,
}

/// Connection diagnostics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionDiagnostics {
    pub server_address: String,
    pub connection_status: ConnectionStatus,
    pub connection_attempts: u64,
    pub successful_connections: u64,
    pub failed_connections: u64,
    pub last_connection_attempt: Option<u64>,
    pub last_successful_connection: Option<u64>,
    pub connection_uptime: Option<Duration>,
    pub average_connection_time: Duration,
    pub connection_success_rate: f64,
    pub network_latency: Option<Duration>,
    pub tls_enabled: bool,
    pub tls_handshake_time: Option<Duration>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_collection_interval_seconds: u64,
    pub health_check_interval_seconds: u64,
    pub performance_monitoring_enabled: bool,
    pub connection_monitoring_enabled: bool,
    pub alerting_enabled: bool,
    pub metrics_export_enabled: bool,
    pub metrics_export_endpoint: Option<String>,
    pub log_level: String,
    pub enable_tracing: bool,
    pub tracing_sampling_rate: f64,
    pub performance_thresholds: PerformanceThresholds,
    pub health_thresholds: HealthThresholds,
}

/// Performance thresholds for alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_response_time_ms: u64,
    pub max_error_rate: f64,
    pub min_success_rate: f64,
    pub max_connection_time_ms: u64,
    pub max_reconnection_attempts: u32,
}

/// Health thresholds for alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthThresholds {
    pub max_consecutive_failures: u32,
    pub max_error_rate: f64,
    pub min_connection_uptime_seconds: u64,
    pub max_response_time_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_collection_interval_seconds: 30,
            health_check_interval_seconds: 60,
            performance_monitoring_enabled: true,
            connection_monitoring_enabled: true,
            alerting_enabled: true,
            metrics_export_enabled: false,
            metrics_export_endpoint: None,
            log_level: "info".to_string(),
            enable_tracing: true,
            tracing_sampling_rate: 0.1,
            performance_thresholds: PerformanceThresholds::default(),
            health_thresholds: HealthThresholds::default(),
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_response_time_ms: 5000,
            max_error_rate: 0.05,
            min_success_rate: 0.95,
            max_connection_time_ms: 10000,
            max_reconnection_attempts: 5,
        }
    }
}

impl Default for HealthThresholds {
    fn default() -> Self {
        Self {
            max_consecutive_failures: 3,
            max_error_rate: 0.1,
            min_connection_uptime_seconds: 300,
            max_response_time_ms: 10000,
        }
    }
}

/// Monitoring and observability manager
pub struct CoordinationMonitor {
    config: MonitoringConfig,
    client_metrics: Arc<ClientMetrics>,
    connection_status: Arc<RwLock<ConnectionStatus>>,
    performance_history: Arc<RwLock<Vec<PerformanceMetrics>>>,
    health_history: Arc<RwLock<Vec<HealthInfo>>>,
    connection_diagnostics: Arc<RwLock<ConnectionDiagnostics>>,
    alert_handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>>,
    start_time: Instant,
    last_metrics_collection: Arc<RwLock<Option<Instant>>>,
    consecutive_failures: Arc<RwLock<u32>>,
}

/// Alert handler trait for custom alerting
#[async_trait::async_trait]
pub trait AlertHandler: Send + Sync {
    async fn handle_alert(&self, alert: Alert) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Alert types for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighErrorRate,
    HighResponseTime,
    ConnectionFailure,
    HealthCheckFailure,
    PerformanceDegradation,
    Custom(String),
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
    pub details: HashMap<String, String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl CoordinationMonitor {
    /// Create a new monitoring instance
    pub fn new(
        config: MonitoringConfig,
        client_metrics: Arc<ClientMetrics>,
        connection_status: Arc<RwLock<ConnectionStatus>>,
    ) -> Self {
        let connection_diagnostics = ConnectionDiagnostics {
            server_address: "unknown".to_string(),
            connection_status: ConnectionStatus::Disconnected,
            connection_attempts: 0,
            successful_connections: 0,
            failed_connections: 0,
            last_connection_attempt: None,
            last_successful_connection: None,
            connection_uptime: None,
            average_connection_time: Duration::from_secs(0),
            connection_success_rate: 0.0,
            network_latency: None,
            tls_enabled: false,
            tls_handshake_time: None,
        };

        Self {
            config,
            client_metrics,
            connection_status,
            performance_history: Arc::new(RwLock::new(Vec::new())),
            health_history: Arc::new(RwLock::new(Vec::new())),
            connection_diagnostics: Arc::new(RwLock::new(connection_diagnostics)),
            alert_handlers: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
            last_metrics_collection: Arc::new(RwLock::new(None)),
            consecutive_failures: Arc::new(RwLock::new(0)),
        }
    }

    /// Start monitoring
    pub async fn start(&self) -> Result<(), CoordinationError> {
        if !self.config.enabled {
            info!("Monitoring is disabled");
            return Ok(());
        }

        info!("Starting coordination monitoring");

        // Start metrics collection
        if self.config.metrics_collection_interval_seconds > 0 {
            self.start_metrics_collection().await;
        }

        // Start health monitoring
        if self.config.health_check_interval_seconds > 0 {
            self.start_health_monitoring().await;
        }

        // Start performance monitoring
        if self.config.performance_monitoring_enabled {
            self.start_performance_monitoring().await;
        }

        // Start connection monitoring
        if self.config.connection_monitoring_enabled {
            self.start_connection_monitoring().await;
        }

        info!("✅ Coordination monitoring started successfully");
        Ok(())
    }

    /// Stop monitoring
    pub async fn stop(&self) -> Result<(), CoordinationError> {
        info!("Stopping coordination monitoring");
        // TODO: Implement graceful shutdown of monitoring tasks
        Ok(())
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> HealthInfo {
        let metrics = self.client_metrics.as_ref();
        let _connection_status = self.connection_status.read().await;
        let consecutive_failures = *self.consecutive_failures.read().await;

        let error_rate = 1.0 - metrics.get_success_rate();
        let connection_success_rate = metrics.get_connection_success_rate();

        let status = if consecutive_failures >= self.config.health_thresholds.max_consecutive_failures {
            HealthStatus::Unhealthy
        } else if error_rate > self.config.health_thresholds.max_error_rate {
            HealthStatus::Degraded
        } else if connection_success_rate < 0.9 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        let mut details = HashMap::new();
        details.insert("error_rate".to_string(), format!("{:.4}", error_rate));
        details.insert("connection_success_rate".to_string(), format!("{:.4}", connection_success_rate));
        details.insert("consecutive_failures".to_string(), consecutive_failures.to_string());
        details.insert("total_requests".to_string(), metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed).to_string());

        let message = match status {
            HealthStatus::Healthy => "All systems operational".to_string(),
            HealthStatus::Degraded => "System performance degraded".to_string(),
            HealthStatus::Unhealthy => "System unhealthy".to_string(),
            HealthStatus::Unknown => "Health status unknown".to_string(),
        };

        HealthInfo {
            status,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            message,
            details,
            last_successful_operation: None, // TODO: Track this
            connection_uptime: Some(self.start_time.elapsed()),
            response_time_p95: None, // TODO: Calculate from performance history
            error_rate,
        }
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> Vec<PerformanceMetrics> {
        self.performance_history.read().await.clone()
    }

    /// Get connection diagnostics
    pub async fn get_connection_diagnostics(&self) -> ConnectionDiagnostics {
        self.connection_diagnostics.read().await.clone()
    }

    /// Add alert handler
    pub async fn add_alert_handler(&self, handler: Box<dyn AlertHandler + Send + Sync>) {
        self.alert_handlers.write().await.push(handler);
    }

    /// Record operation result for monitoring
    pub async fn record_operation_result(
        &self,
        operation_name: &str,
        success: bool,
        response_time: Duration,
    ) {
        if !self.config.enabled {
            return;
        }

        if !success {
            let mut failures = self.consecutive_failures.write().await;
            *failures += 1;
        } else {
            *self.consecutive_failures.write().await = 0;
        }

        // Check for alerts
        if self.config.alerting_enabled {
            self.check_alerts(operation_name, success, response_time).await;
        }
    }

    /// Start metrics collection background task
    async fn start_metrics_collection(&self) {
        let monitor = self.clone();
        let interval = Duration::from_secs(self.config.metrics_collection_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = monitor.collect_metrics().await {
                    error!("Failed to collect metrics: {}", e);
                }
            }
        });
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) {
        let monitor = self.clone();
        let interval = Duration::from_secs(self.config.health_check_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                let health_info = monitor.get_health_status().await;
                monitor.record_health_info(health_info.clone()).await;

                // Check for health alerts
                if monitor.config.alerting_enabled {
                    monitor.check_health_alerts(&health_info).await;
                }
            }
        });
    }

    /// Start performance monitoring background task
    async fn start_performance_monitoring(&self) {
        let monitor = self.clone();
        let interval = Duration::from_secs(60); // Every minute

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = monitor.collect_performance_metrics().await {
                    error!("Failed to collect performance metrics: {}", e);
                }
            }
        });
    }

    /// Start connection monitoring background task
    async fn start_connection_monitoring(&self) {
        let monitor = self.clone();
        let interval = Duration::from_secs(30); // Every 30 seconds

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = monitor.update_connection_diagnostics().await {
                    error!("Failed to update connection diagnostics: {}", e);
                }
            }
        });
    }

    /// Collect and store metrics
    async fn collect_metrics(&self) -> Result<(), CoordinationError> {
        let now = Instant::now();
        *self.last_metrics_collection.write().await = Some(now);

        // Export metrics if enabled
        if self.config.metrics_export_enabled {
            if let Some(endpoint) = &self.config.metrics_export_endpoint {
                self.export_metrics(endpoint).await?;
            }
        }

        debug!("Metrics collected successfully");
        Ok(())
    }

    /// Collect performance metrics
    async fn collect_performance_metrics(&self) -> Result<(), CoordinationError> {
        let metrics = self.client_metrics.as_ref();
        let total_requests = metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed);
        let successful_requests = metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed);
        let failed_requests = metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed);

        if total_requests == 0 {
            return Ok(());
        }

        let error_rate = failed_requests as f64 / total_requests as f64;
        let throughput = total_requests as f64 / self.start_time.elapsed().as_secs_f64();

        let performance_metrics = PerformanceMetrics {
            operation_name: "overall".to_string(),
            total_operations: total_requests,
            successful_operations: successful_requests,
            failed_operations: failed_requests,
            average_response_time: Duration::from_millis(
                metrics.average_response_time.load(std::sync::atomic::Ordering::Relaxed)
            ),
            p50_response_time: Duration::from_millis(0), // TODO: Calculate from response time history
            p95_response_time: Duration::from_millis(0), // TODO: Calculate from response time history
            p99_response_time: Duration::from_millis(0), // TODO: Calculate from response time history
            min_response_time: Duration::from_millis(0), // TODO: Track min/max
            max_response_time: Duration::from_millis(0), // TODO: Track min/max
            throughput_ops_per_second: throughput,
            error_rate,
        };

        self.performance_history.write().await.push(performance_metrics);

        // Keep only last 1000 entries
        let mut history = self.performance_history.write().await;
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }

        Ok(())
    }

    /// Update connection diagnostics
    async fn update_connection_diagnostics(&self) -> Result<(), CoordinationError> {
        let mut diagnostics = self.connection_diagnostics.write().await;
        let metrics = self.client_metrics.as_ref();

        diagnostics.connection_attempts = metrics.connection_attempts.load(std::sync::atomic::Ordering::Relaxed);
        diagnostics.successful_connections = metrics.successful_connections.load(std::sync::atomic::Ordering::Relaxed);
        diagnostics.failed_connections = metrics.failed_connections.load(std::sync::atomic::Ordering::Relaxed);
        diagnostics.connection_success_rate = metrics.get_connection_success_rate();
        diagnostics.connection_status = self.connection_status.read().await.clone();

        Ok(())
    }

    /// Record health information
    async fn record_health_info(&self, health_info: HealthInfo) {
        self.health_history.write().await.push(health_info);

        // Keep only last 1000 entries
        let mut history = self.health_history.write().await;
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }
    }

    /// Check for alerts based on operation results
    async fn check_alerts(&self, operation_name: &str, _success: bool, response_time: Duration) {
        let metrics = self.client_metrics.as_ref();
        let error_rate = 1.0 - metrics.get_success_rate();

        // Check response time threshold
        if response_time.as_millis() > self.config.performance_thresholds.max_response_time_ms as u128 {
            let alert = Alert {
                alert_type: AlertType::HighResponseTime,
                severity: AlertSeverity::Warning,
                message: format!("High response time for operation '{}': {:?}", operation_name, response_time),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("operation".to_string(), operation_name.to_string());
                    details.insert("response_time_ms".to_string(), response_time.as_millis().to_string());
                    details.insert("threshold_ms".to_string(), self.config.performance_thresholds.max_response_time_ms.to_string());
                    details
                },
            };
            self.send_alert(alert).await;
        }

        // Check error rate threshold
        if error_rate > self.config.performance_thresholds.max_error_rate {
            let alert = Alert {
                alert_type: AlertType::HighErrorRate,
                severity: AlertSeverity::Error,
                message: format!("High error rate detected: {:.2}%", error_rate * 100.0),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("error_rate".to_string(), format!("{:.4}", error_rate));
                    details.insert("threshold".to_string(), format!("{:.4}", self.config.performance_thresholds.max_error_rate));
                    details
                },
            };
            self.send_alert(alert).await;
        }
    }

    /// Check for health alerts
    async fn check_health_alerts(&self, health_info: &HealthInfo) {
        match health_info.status {
            HealthStatus::Unhealthy => {
                let alert = Alert {
                    alert_type: AlertType::HealthCheckFailure,
                    severity: AlertSeverity::Critical,
                    message: "System health check failed".to_string(),
                    timestamp: health_info.timestamp,
                    details: health_info.details.clone(),
                };
                self.send_alert(alert).await;
            }
            HealthStatus::Degraded => {
                let alert = Alert {
                    alert_type: AlertType::PerformanceDegradation,
                    severity: AlertSeverity::Warning,
                    message: "System performance degraded".to_string(),
                    timestamp: health_info.timestamp,
                    details: health_info.details.clone(),
                };
                self.send_alert(alert).await;
            }
            _ => {}
        }
    }

    /// Send alert to all registered handlers
    async fn send_alert(&self, alert: Alert) {
        let handlers = self.alert_handlers.read().await;
        
        for handler in handlers.iter() {
            if let Err(e) = handler.handle_alert(alert.clone()).await {
                error!("Failed to send alert via handler: {}", e);
            }
        }
    }

    /// Export metrics to external endpoint
    async fn export_metrics(&self, endpoint: &str) -> Result<(), CoordinationError> {
        // TODO: Implement metrics export to external monitoring systems
        // This could export to Prometheus, InfluxDB, etc.
        debug!("Exporting metrics to {}", endpoint);
        Ok(())
    }
}

impl Clone for CoordinationMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            client_metrics: self.client_metrics.clone(),
            connection_status: self.connection_status.clone(),
            performance_history: self.performance_history.clone(),
            health_history: self.health_history.clone(),
            connection_diagnostics: self.connection_diagnostics.clone(),
            alert_handlers: self.alert_handlers.clone(),
            start_time: self.start_time,
            last_metrics_collection: self.last_metrics_collection.clone(),
            consecutive_failures: self.consecutive_failures.clone(),
        }
    }
}

/// Default alert handler that logs alerts
pub struct LoggingAlertHandler;

#[async_trait::async_trait]
impl AlertHandler for LoggingAlertHandler {
    async fn handle_alert(&self, alert: Alert) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match alert.severity {
            AlertSeverity::Info => info!("Alert: {}", alert.message),
            AlertSeverity::Warning => warn!("Alert: {}", alert.message),
            AlertSeverity::Error => error!("Alert: {}", alert.message),
            AlertSeverity::Critical => error!("CRITICAL ALERT: {}", alert.message),
        }

        debug!("Alert details: {:?}", alert.details);
        Ok(())
    }
}

/// Metrics exporter for Prometheus format
pub struct PrometheusExporter;

impl PrometheusExporter {
    /// Export metrics in Prometheus format
    pub fn export_prometheus_metrics(metrics: &ClientMetrics) -> String {
        let mut output = String::new();
        
        output.push_str(&format!(
            "# HELP rhema_coordination_total_requests Total number of requests\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_total_requests counter\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_total_requests {}\n",
            metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_successful_requests Total number of successful requests\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_successful_requests counter\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_successful_requests {}\n",
            metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_failed_requests Total number of failed requests\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_failed_requests counter\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_failed_requests {}\n",
            metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_average_response_time Average response time in milliseconds\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_average_response_time gauge\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_average_response_time {}\n",
            metrics.average_response_time.load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_success_rate Success rate as a percentage\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_success_rate gauge\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_success_rate {}\n",
            metrics.get_success_rate() * 100.0
        ));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU64;

    #[tokio::test]
    async fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert!(config.enabled);
        assert_eq!(config.metrics_collection_interval_seconds, 30);
        assert_eq!(config.health_check_interval_seconds, 60);
    }

    #[tokio::test]
    async fn test_health_status_healthy() {
        let metrics = Arc::new(ClientMetrics::new());
        // Add multiple successful requests to ensure healthy status
        for _ in 0..10 {
            metrics.increment_total_requests();
            metrics.increment_successful_requests();
        }
        // Set up connection metrics to ensure healthy status
        metrics.increment_connection_attempts();
        metrics.increment_successful_connections();

        let config = MonitoringConfig::default();
        let connection_status = Arc::new(RwLock::new(ConnectionStatus::Connected));
        let monitor = CoordinationMonitor::new(config, metrics, connection_status);

        let health = monitor.get_health_status().await;
        assert!(matches!(health.status, HealthStatus::Healthy));
    }

    #[tokio::test]
    async fn test_health_status_unhealthy() {
        let metrics = Arc::new(ClientMetrics::new());
        // Add multiple failed requests to trigger unhealthy status
        for _ in 0..5 {
            metrics.increment_total_requests();
            metrics.increment_failed_requests();
        }
        // Set up connection metrics to ensure unhealthy status
        metrics.increment_connection_attempts();
        // Don't increment successful connections to keep success rate low

        let config = MonitoringConfig::default();
        let connection_status = Arc::new(RwLock::new(ConnectionStatus::Failed("test".to_string())));
        let monitor = CoordinationMonitor::new(config, metrics, connection_status);
        
        // Directly set consecutive failures to trigger unhealthy status
        *monitor.consecutive_failures.write().await = 5; // Above the threshold of 3

        let health = monitor.get_health_status().await;
        assert!(matches!(health.status, HealthStatus::Unhealthy));
    }

    #[tokio::test]
    async fn test_prometheus_export() {
        let metrics = ClientMetrics::new();
        metrics.increment_total_requests();
        metrics.increment_successful_requests();

        let prometheus_output = PrometheusExporter::export_prometheus_metrics(&metrics);
        assert!(prometheus_output.contains("rhema_coordination_total_requests 1"));
        assert!(prometheus_output.contains("rhema_coordination_successful_requests 1"));
        assert!(prometheus_output.contains("rhema_coordination_success_rate 100"));
    }
}
