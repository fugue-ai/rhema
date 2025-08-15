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
use tokio::sync::{broadcast, RwLock};
use tokio::task::JoinHandle;
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
    response_times: Arc<RwLock<Vec<Duration>>>,
    last_successful_operation: Arc<RwLock<Option<u64>>>,
    shutdown_tx: Option<broadcast::Sender<()>>,
    monitoring_tasks: Vec<JoinHandle<()>>,
}

/// Alert handler trait for custom alerting
#[async_trait::async_trait]
pub trait AlertHandler: Send + Sync {
    async fn handle_alert(
        &self,
        alert: Alert,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
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
            response_times: Arc::new(RwLock::new(Vec::new())),
            last_successful_operation: Arc::new(RwLock::new(None)),
            shutdown_tx: None,
            monitoring_tasks: Vec::new(),
        }
    }

    /// Start monitoring
    pub async fn start(&mut self) -> Result<(), CoordinationError> {
        if !self.config.enabled {
            info!("Monitoring is disabled");
            return Ok(());
        }

        info!("Starting coordination monitoring");

        // Create shutdown channel
        let (shutdown_tx, _) = broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx.clone());

        // Start metrics collection
        if self.config.metrics_collection_interval_seconds > 0 {
            let task = self.start_metrics_collection(shutdown_tx.subscribe()).await;
            self.monitoring_tasks.push(task);
        }

        // Start health monitoring
        if self.config.health_check_interval_seconds > 0 {
            let task = self.start_health_monitoring(shutdown_tx.subscribe()).await;
            self.monitoring_tasks.push(task);
        }

        // Start performance monitoring
        if self.config.performance_monitoring_enabled {
            let task = self
                .start_performance_monitoring(shutdown_tx.subscribe())
                .await;
            self.monitoring_tasks.push(task);
        }

        // Start connection monitoring
        if self.config.connection_monitoring_enabled {
            let task = self
                .start_connection_monitoring(shutdown_tx.subscribe())
                .await;
            self.monitoring_tasks.push(task);
        }

        info!("✅ Coordination monitoring started successfully");
        Ok(())
    }

    /// Stop monitoring
    pub async fn stop(&self) -> Result<(), CoordinationError> {
        info!("Stopping coordination monitoring");

        // Send shutdown signal to all monitoring tasks
        if let Some(shutdown_tx) = &self.shutdown_tx {
            if let Err(e) = shutdown_tx.send(()) {
                warn!("Failed to send shutdown signal: {}", e);
            }
        }

        // Wait for all monitoring tasks to complete
        for task in &self.monitoring_tasks {
            if !task.is_finished() {
                // Just check if the task is finished, don't await it
                warn!("Monitoring task still running during shutdown");
            }
        }

        info!("✅ Coordination monitoring stopped successfully");
        Ok(())
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> HealthInfo {
        let metrics = self.client_metrics.as_ref();
        let _connection_status = self.connection_status.read().await;
        let consecutive_failures = *self.consecutive_failures.read().await;
        let last_successful = *self.last_successful_operation.read().await;

        let error_rate = 1.0 - metrics.get_success_rate();
        let connection_success_rate = metrics.get_connection_success_rate();

        let status =
            if consecutive_failures >= self.config.health_thresholds.max_consecutive_failures {
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
        details.insert(
            "connection_success_rate".to_string(),
            format!("{:.4}", connection_success_rate),
        );
        details.insert(
            "consecutive_failures".to_string(),
            consecutive_failures.to_string(),
        );
        details.insert(
            "total_requests".to_string(),
            metrics
                .total_requests
                .load(std::sync::atomic::Ordering::Relaxed)
                .to_string(),
        );

        let message = match status {
            HealthStatus::Healthy => "All systems operational".to_string(),
            HealthStatus::Degraded => "System performance degraded".to_string(),
            HealthStatus::Unhealthy => "System unhealthy".to_string(),
            HealthStatus::Unknown => "Health status unknown".to_string(),
        };

        // Calculate p95 response time from performance history
        let response_time_p95 = self.calculate_response_time_percentile(0.95).await;

        HealthInfo {
            status,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            message,
            details,
            last_successful_operation: last_successful,
            connection_uptime: Some(self.start_time.elapsed()),
            response_time_p95,
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

        // Track response times for percentile calculations
        self.response_times.write().await.push(response_time);

        // Keep only last 10000 response times to prevent memory bloat
        let mut times = self.response_times.write().await;
        let current_len = times.len();
        if current_len > 10000 {
            times.drain(0..current_len - 10000);
        }

        if success {
            *self.last_successful_operation.write().await = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            *self.consecutive_failures.write().await = 0;
        } else {
            let mut failures = self.consecutive_failures.write().await;
            *failures += 1;
        }

        // Check for alerts
        if self.config.alerting_enabled {
            self.check_alerts(operation_name, success, response_time)
                .await;
        }
    }

    /// Start metrics collection background task
    async fn start_metrics_collection(
        &self,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> JoinHandle<()> {
        let monitor = self.clone();
        let interval = Duration::from_secs(self.config.metrics_collection_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = interval_timer.tick() => {
                        if let Err(e) = monitor.collect_metrics().await {
                            error!("Failed to collect metrics: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Shutting down metrics collection task");
                        break;
                    }
                }
            }
        })
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(
        &self,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> JoinHandle<()> {
        let monitor = self.clone();
        let interval = Duration::from_secs(self.config.health_check_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = interval_timer.tick() => {
                        let health_info = monitor.get_health_status().await;
                        monitor.record_health_info(health_info.clone()).await;

                        // Check for health alerts
                        if monitor.config.alerting_enabled {
                            monitor.check_health_alerts(&health_info).await;
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Shutting down health monitoring task");
                        break;
                    }
                }
            }
        })
    }

    /// Start performance monitoring background task
    async fn start_performance_monitoring(
        &self,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> JoinHandle<()> {
        let monitor = self.clone();
        let interval = Duration::from_secs(60); // Every minute

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = interval_timer.tick() => {
                        if let Err(e) = monitor.collect_performance_metrics().await {
                            error!("Failed to collect performance metrics: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Shutting down performance monitoring task");
                        break;
                    }
                }
            }
        })
    }

    /// Start connection monitoring background task
    async fn start_connection_monitoring(
        &self,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> JoinHandle<()> {
        let monitor = self.clone();
        let interval = Duration::from_secs(30); // Every 30 seconds

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = interval_timer.tick() => {
                        if let Err(e) = monitor.update_connection_diagnostics().await {
                            error!("Failed to update connection diagnostics: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Shutting down connection monitoring task");
                        break;
                    }
                }
            }
        })
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
        let total_requests = metrics
            .total_requests
            .load(std::sync::atomic::Ordering::Relaxed);
        let successful_requests = metrics
            .successful_requests
            .load(std::sync::atomic::Ordering::Relaxed);
        let failed_requests = metrics
            .failed_requests
            .load(std::sync::atomic::Ordering::Relaxed);

        if total_requests == 0 {
            return Ok(());
        }

        let error_rate = failed_requests as f64 / total_requests as f64;
        let throughput = total_requests as f64 / self.start_time.elapsed().as_secs_f64();

        // Calculate percentiles and min/max from response times
        let response_times = self.response_times.read().await;
        let (p50, p95, p99, min_time, max_time) = if !response_times.is_empty() {
            let mut sorted_times = response_times.clone();
            sorted_times.sort_by(|a, b| a.cmp(b));

            let len = sorted_times.len();
            let p50_idx = (0.5 * len as f64).floor() as usize;
            let p95_idx = (0.95 * len as f64).floor() as usize;
            let p99_idx = (0.99 * len as f64).floor() as usize;

            let p50 = if p50_idx < len {
                sorted_times[p50_idx]
            } else {
                Duration::from_millis(0)
            };
            let p95 = if p95_idx < len {
                sorted_times[p95_idx]
            } else {
                Duration::from_millis(0)
            };
            let p99 = if p99_idx < len {
                sorted_times[p99_idx]
            } else {
                Duration::from_millis(0)
            };
            let min_time = sorted_times[0];
            let max_time = sorted_times[len - 1];

            (p50, p95, p99, min_time, max_time)
        } else {
            (
                Duration::from_millis(0),
                Duration::from_millis(0),
                Duration::from_millis(0),
                Duration::from_millis(0),
                Duration::from_millis(0),
            )
        };

        let performance_metrics = PerformanceMetrics {
            operation_name: "overall".to_string(),
            total_operations: total_requests,
            successful_operations: successful_requests,
            failed_operations: failed_requests,
            average_response_time: Duration::from_millis(
                metrics
                    .average_response_time
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            min_response_time: min_time,
            max_response_time: max_time,
            throughput_ops_per_second: throughput,
            error_rate,
        };

        self.performance_history
            .write()
            .await
            .push(performance_metrics);

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

        diagnostics.connection_attempts = metrics
            .connection_attempts
            .load(std::sync::atomic::Ordering::Relaxed);
        diagnostics.successful_connections = metrics
            .successful_connections
            .load(std::sync::atomic::Ordering::Relaxed);
        diagnostics.failed_connections = metrics
            .failed_connections
            .load(std::sync::atomic::Ordering::Relaxed);
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
        if response_time.as_millis()
            > self.config.performance_thresholds.max_response_time_ms as u128
        {
            let alert = Alert {
                alert_type: AlertType::HighResponseTime,
                severity: AlertSeverity::Warning,
                message: format!(
                    "High response time for operation '{}': {:?}",
                    operation_name, response_time
                ),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("operation".to_string(), operation_name.to_string());
                    details.insert(
                        "response_time_ms".to_string(),
                        response_time.as_millis().to_string(),
                    );
                    details.insert(
                        "threshold_ms".to_string(),
                        self.config
                            .performance_thresholds
                            .max_response_time_ms
                            .to_string(),
                    );
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
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("error_rate".to_string(), format!("{:.4}", error_rate));
                    details.insert(
                        "threshold".to_string(),
                        format!("{:.4}", self.config.performance_thresholds.max_error_rate),
                    );
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
        let metrics = self.client_metrics.as_ref();

        // Determine export format based on endpoint
        let export_data = if endpoint.contains("prometheus") || endpoint.ends_with("/metrics") {
            // Export in Prometheus format
            PrometheusExporter::export_prometheus_metrics(metrics)
        } else if endpoint.contains("influxdb") {
            // Export in InfluxDB line protocol format
            self.export_influxdb_metrics(metrics).await?
        } else if endpoint.contains("json") || endpoint.ends_with("/json") {
            // Export as JSON
            serde_json::to_string(&self.create_metrics_json(metrics).await)
                .map_err(CoordinationError::SerializationError)?
        } else {
            // Default to Prometheus format
            PrometheusExporter::export_prometheus_metrics(metrics)
        };

        // Send metrics to endpoint
        self.send_metrics_to_endpoint(endpoint, &export_data)
            .await?;

        debug!("Exported metrics to {}", endpoint);
        Ok(())
    }

    /// Export metrics in InfluxDB line protocol format
    async fn export_influxdb_metrics(
        &self,
        metrics: &ClientMetrics,
    ) -> Result<String, CoordinationError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut lines = Vec::new();

        // Total requests
        lines.push(format!(
            "rhema_coordination,metric=total_requests value={} {}",
            metrics
                .total_requests
                .load(std::sync::atomic::Ordering::Relaxed),
            timestamp
        ));

        // Successful requests
        lines.push(format!(
            "rhema_coordination,metric=successful_requests value={} {}",
            metrics
                .successful_requests
                .load(std::sync::atomic::Ordering::Relaxed),
            timestamp
        ));

        // Failed requests
        lines.push(format!(
            "rhema_coordination,metric=failed_requests value={} {}",
            metrics
                .failed_requests
                .load(std::sync::atomic::Ordering::Relaxed),
            timestamp
        ));

        // Average response time
        lines.push(format!(
            "rhema_coordination,metric=avg_response_time value={} {}",
            metrics
                .average_response_time
                .load(std::sync::atomic::Ordering::Relaxed),
            timestamp
        ));

        // Success rate
        lines.push(format!(
            "rhema_coordination,metric=success_rate value={} {}",
            metrics.get_success_rate(),
            timestamp
        ));

        Ok(lines.join("\n"))
    }

    /// Create JSON metrics structure
    async fn create_metrics_json(&self, metrics: &ClientMetrics) -> serde_json::Value {
        let health_info = self.get_health_status().await;
        let performance_metrics = self.get_performance_metrics().await;
        let connection_diagnostics = self.get_connection_diagnostics().await;

        serde_json::json!({
            "timestamp": SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "metrics": {
                "total_requests": metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed),
                "successful_requests": metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed),
                "failed_requests": metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed),
                "average_response_time_ms": metrics.average_response_time.load(std::sync::atomic::Ordering::Relaxed),
                "success_rate": metrics.get_success_rate(),
                "connection_success_rate": metrics.get_connection_success_rate()
            },
            "health": {
                "status": format!("{:?}", health_info.status),
                "message": health_info.message,
                "error_rate": health_info.error_rate,
                "details": health_info.details
            },
            "performance": performance_metrics,
            "connection": {
                "status": format!("{:?}", connection_diagnostics.connection_status),
                "success_rate": connection_diagnostics.connection_success_rate,
                "uptime_seconds": connection_diagnostics.connection_uptime.map(|d| d.as_secs())
            }
        })
    }

    /// Send metrics data to external endpoint
    async fn send_metrics_to_endpoint(
        &self,
        endpoint: &str,
        data: &str,
    ) -> Result<(), CoordinationError> {
        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", self.get_content_type_for_endpoint(endpoint))
            .body(data.to_string())
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| {
                CoordinationError::NetworkError(format!("Failed to send metrics: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(CoordinationError::NetworkError(format!(
                "Metrics export failed with status: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Get appropriate content type for endpoint
    fn get_content_type_for_endpoint(&self, endpoint: &str) -> &'static str {
        if endpoint.contains("prometheus") || endpoint.ends_with("/metrics") {
            "text/plain; version=0.0.4; charset=utf-8"
        } else if endpoint.contains("influxdb") {
            "text/plain"
        } else if endpoint.contains("json") || endpoint.ends_with("/json") {
            "application/json"
        } else {
            "text/plain"
        }
    }

    /// Calculate a percentile of response times from the performance history
    async fn calculate_response_time_percentile(&self, percentile: f64) -> Option<Duration> {
        let history = self.performance_history.read().await;
        if history.is_empty() {
            return None;
        }

        let mut sorted_times: Vec<Duration> =
            history.iter().map(|p| p.average_response_time).collect();
        sorted_times.sort_by(|a, b| a.cmp(b));

        let index = (percentile * sorted_times.len() as f64).floor() as usize;
        if index < sorted_times.len() {
            Some(sorted_times[index])
        } else {
            None
        }
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
            response_times: self.response_times.clone(),
            last_successful_operation: self.last_successful_operation.clone(),
            shutdown_tx: self.shutdown_tx.clone(),
            monitoring_tasks: Vec::new(), // JoinHandle doesn't implement Clone
        }
    }
}

/// Default alert handler that logs alerts
pub struct LoggingAlertHandler;

#[async_trait::async_trait]
impl AlertHandler for LoggingAlertHandler {
    async fn handle_alert(
        &self,
        alert: Alert,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
            metrics
                .total_requests
                .load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_successful_requests Total number of successful requests\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_successful_requests counter\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_successful_requests {}\n",
            metrics
                .successful_requests
                .load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_failed_requests Total number of failed requests\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_failed_requests counter\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_failed_requests {}\n",
            metrics
                .failed_requests
                .load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_average_response_time Average response time in milliseconds\n"
        ));
        output.push_str(&format!(
            "# TYPE rhema_coordination_average_response_time gauge\n"
        ));
        output.push_str(&format!(
            "rhema_coordination_average_response_time {}\n",
            metrics
                .average_response_time
                .load(std::sync::atomic::Ordering::Relaxed)
        ));

        output.push_str(&format!(
            "# HELP rhema_coordination_success_rate Success rate as a percentage\n"
        ));
        output.push_str(&format!("# TYPE rhema_coordination_success_rate gauge\n"));
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
        let mut monitor = CoordinationMonitor::new(config, metrics, connection_status);

        monitor.start().await.unwrap();

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
        let mut monitor = CoordinationMonitor::new(config, metrics, connection_status);

        monitor.start().await.unwrap();

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
