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

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use prometheus::{Counter, Gauge, Histogram, HistogramOpts, Registry};
use rhema_core::{RhemaError, RhemaResult};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, instrument};

/// Monitoring service for Rhema
pub struct MonitoringService {
    registry: Registry,
    metrics: Arc<Metrics>,
    health_status: Arc<RwLock<HealthStatus>>,
    performance_monitor: Option<Arc<crate::performance::PerformanceMonitor>>,
}

/// Application metrics
#[derive(Clone)]
pub struct Metrics {
    // Request metrics
    pub requests_total: Counter,
    pub requests_duration: Histogram,
    pub requests_in_flight: Gauge,

    // AI service metrics
    pub ai_requests_total: Counter,
    pub ai_requests_duration: Histogram,
    pub ai_cache_hits: Counter,
    pub ai_cache_misses: Counter,
    pub ai_model_memory_usage: Gauge,

    // Git operations metrics
    pub git_operations_total: Counter,
    pub git_operations_duration: Histogram,
    pub git_errors_total: Counter,

    // File operations metrics
    pub file_operations_total: Counter,
    pub file_operations_duration: Histogram,
    pub file_errors_total: Counter,

    // System metrics
    pub memory_usage_bytes: Gauge,
    pub cpu_usage_percent: Gauge,
    pub active_connections: Gauge,

    // Performance monitoring metrics
    pub performance_monitoring_enabled: Gauge,
    pub performance_reports_generated: Counter,
    pub performance_alerts_triggered: Counter,
}

/// Health status
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub uptime: Duration,
    pub checks: HashMap<String, CheckResult>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CheckResult {
    pub status: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new() -> RhemaResult<Self> {
        let registry = Registry::new();

        let metrics = Arc::new(Metrics {
            requests_total: Counter::new("rhema_requests_total", "Total number of requests")
                .unwrap(),
            requests_duration: Histogram::with_opts(HistogramOpts::new(
                "rhema_requests_duration_seconds",
                "Request duration in seconds",
            ))
            .unwrap(),
            requests_in_flight: Gauge::new(
                "rhema_requests_in_flight",
                "Number of requests currently being processed",
            )
            .unwrap(),

            ai_requests_total: Counter::new(
                "rhema_coordination_requests_total",
                "Total number of AI service requests",
            )
            .unwrap(),
            ai_requests_duration: Histogram::with_opts(HistogramOpts::new(
                "rhema_coordination_requests_duration_seconds",
                "AI request duration in seconds",
            ))
            .unwrap(),
            ai_cache_hits: Counter::new(
                "rhema_coordination_cache_hits_total",
                "Total number of AI cache hits",
            )
            .unwrap(),
            ai_cache_misses: Counter::new(
                "rhema_coordination_cache_misses_total",
                "Total number of AI cache misses",
            )
            .unwrap(),
            ai_model_memory_usage: Gauge::new(
                "rhema_coordination_model_memory_bytes",
                "AI model memory usage in bytes",
            )
            .unwrap(),

            git_operations_total: Counter::new(
                "rhema_git_operations_total",
                "Total number of git operations",
            )
            .unwrap(),
            git_operations_duration: Histogram::with_opts(HistogramOpts::new(
                "rhema_git_operations_duration_seconds",
                "Git operation duration in seconds",
            ))
            .unwrap(),
            git_errors_total: Counter::new(
                "rhema_git_errors_total",
                "Total number of git operation errors",
            )
            .unwrap(),

            file_operations_total: Counter::new(
                "rhema_file_operations_total",
                "Total number of file operations",
            )
            .unwrap(),
            file_operations_duration: Histogram::with_opts(HistogramOpts::new(
                "rhema_file_operations_duration_seconds",
                "File operation duration in seconds",
            ))
            .unwrap(),
            file_errors_total: Counter::new(
                "rhema_file_errors_total",
                "Total number of file operation errors",
            )
            .unwrap(),

            memory_usage_bytes: Gauge::new("rhema_memory_usage_bytes", "Memory usage in bytes")
                .unwrap(),
            cpu_usage_percent: Gauge::new("rhema_cpu_usage_percent", "CPU usage percentage")
                .unwrap(),
            active_connections: Gauge::new(
                "rhema_active_connections",
                "Number of active connections",
            )
            .unwrap(),

            performance_monitoring_enabled: Gauge::new(
                "rhema_performance_monitoring_enabled",
                "Indicates if performance monitoring is enabled",
            )
            .unwrap(),
            performance_reports_generated: Counter::new(
                "rhema_performance_reports_generated_total",
                "Total number of performance reports generated",
            )
            .unwrap(),
            performance_alerts_triggered: Counter::new(
                "rhema_performance_alerts_triggered_total",
                "Total number of performance alerts triggered",
            )
            .unwrap(),
        });

        // Register metrics
        registry
            .register(Box::new(metrics.requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.requests_duration.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.requests_in_flight.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.ai_requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.ai_requests_duration.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.ai_cache_hits.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.ai_cache_misses.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.ai_model_memory_usage.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.git_operations_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.git_operations_duration.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.git_errors_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.file_operations_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.file_operations_duration.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.file_errors_total.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.memory_usage_bytes.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.cpu_usage_percent.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.active_connections.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.performance_monitoring_enabled.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.performance_reports_generated.clone()))
            .unwrap();
        registry
            .register(Box::new(metrics.performance_alerts_triggered.clone()))
            .unwrap();

        let health_status = Arc::new(RwLock::new(HealthStatus {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: Duration::from_secs(0),
            checks: HashMap::new(),
        }));

        Ok(Self {
            registry,
            metrics,
            health_status,
            performance_monitor: None,
        })
    }

    /// Set performance monitor
    pub fn set_performance_monitor(
        &mut self,
        monitor: Arc<crate::performance::PerformanceMonitor>,
    ) {
        self.performance_monitor = Some(monitor.clone());
        self.metrics.performance_monitoring_enabled.set(1.0);
    }

    /// Get performance monitor reference
    pub fn performance_monitor(&self) -> Option<Arc<crate::performance::PerformanceMonitor>> {
        self.performance_monitor.clone()
    }

    /// Start the monitoring HTTP server
    #[instrument(skip(self))]
    pub async fn start_server(&self, addr: &str) -> RhemaResult<()> {
        let metrics = self.metrics.clone();
        let health_status = self.health_status.clone();
        let registry = self.registry.clone();

        info!("Starting monitoring server on {}", addr);

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .app_data(web::Data::new(metrics.clone()))
                .app_data(web::Data::new(health_status.clone()))
                .app_data(web::Data::new(registry.clone()))
                .route("/metrics", web::get().to(metrics_handler))
                .route("/health", web::get().to(health_handler))
                .route("/ready", web::get().to(ready_handler))
                .route("/live", web::get().to(live_handler))
        })
        .bind(addr)?
        .run()
        .await?;

        Ok(())
    }

    /// Get metrics reference
    pub fn metrics(&self) -> Arc<Metrics> {
        self.metrics.clone()
    }

    /// Update health status
    pub async fn update_health(&self, status: HealthStatus) {
        let mut health = self.health_status.write().await;
        *health = status;
    }

    /// Record request metrics
    pub fn record_request(&self, duration: Duration) {
        self.metrics.requests_total.inc();
        self.metrics
            .requests_duration
            .observe(duration.as_secs_f64());
    }

    /// Record AI request metrics
    pub fn record_ai_request(&self, duration: Duration, cache_hit: bool) {
        self.metrics.ai_requests_total.inc();
        self.metrics
            .ai_requests_duration
            .observe(duration.as_secs_f64());

        if cache_hit {
            self.metrics.ai_cache_hits.inc();
        } else {
            self.metrics.ai_cache_misses.inc();
        }
    }

    /// Record git operation metrics
    pub fn record_git_operation(&self, duration: Duration, success: bool) {
        self.metrics.git_operations_total.inc();
        self.metrics
            .git_operations_duration
            .observe(duration.as_secs_f64());

        if !success {
            self.metrics.git_errors_total.inc();
        }
    }

    /// Record file operation metrics
    pub fn record_file_operation(&self, duration: Duration, success: bool) {
        self.metrics.file_operations_total.inc();
        self.metrics
            .file_operations_duration
            .observe(duration.as_secs_f64());

        if !success {
            self.metrics.file_errors_total.inc();
        }
    }

    /// Update system metrics
    pub fn update_system_metrics(&self, memory_bytes: u64, cpu_percent: f64, connections: u64) {
        self.metrics.memory_usage_bytes.set(memory_bytes as f64);
        self.metrics.cpu_usage_percent.set(cpu_percent);
        self.metrics.active_connections.set(connections as f64);

        // Also record in performance monitor if available
        if let Some(monitor) = &self.performance_monitor {
            let data = crate::performance::SystemPerformanceData {
                timestamp: chrono::Utc::now(),
                cpu_usage_percent: cpu_percent,
                memory_usage_bytes: memory_bytes,
                memory_usage_percent: 0.0, // Would need to calculate from total memory
                disk_io_ops: 0,
                disk_io_bytes: 0,
                network_io_bytes: 0,
                network_latency_ms: 0.0,
                fs_operations: 0,
                fs_latency_ms: 0.0,
                process_count: 0,
                thread_count: 0,
                open_file_descriptors: connections,
            };

            // Spawn async task to record metrics
            let monitor_clone = monitor.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor_clone.record_system_metrics(data).await {
                    error!(
                        "Failed to record system metrics in performance monitor: {}",
                        e
                    );
                }
            });
        }
    }

    /// Record command execution metrics
    pub fn record_command_execution(
        &self,
        command_name: &str,
        duration: Duration,
        success: bool,
        error_message: Option<&str>,
    ) {
        // Record in basic metrics
        self.record_request(duration);

        // Record in performance monitor if available
        if let Some(monitor) = &self.performance_monitor {
            let ux_data = crate::performance::UxData {
                timestamp: chrono::Utc::now(),
                command_name: command_name.to_string(),
                execution_time_ms: duration.as_millis() as u64,
                success,
                interaction_time_ms: 0, // Would need to track separately
                response_time_ms: duration.as_millis() as u64,
                error_message: error_message.map(|s| s.to_string()),
                satisfaction_score: None, // Would need user input
            };

            // Spawn async task to record metrics
            let monitor_clone = monitor.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor_clone.record_ux_metrics(ux_data).await {
                    error!("Failed to record UX metrics in performance monitor: {}", e);
                }
            });
        }
    }

    /// Record usage analytics
    pub fn record_usage_analytics(
        &self,
        user_id: &str,
        command_name: &str,
        feature_name: &str,
        session_duration_seconds: u64,
        workflow_completed: bool,
        usage_pattern: &str,
        user_behavior: &str,
    ) {
        // Record in performance monitor if available
        if let Some(monitor) = &self.performance_monitor {
            let usage_data = crate::performance::UsageData {
                timestamp: chrono::Utc::now(),
                user_id: user_id.to_string(),
                command_name: command_name.to_string(),
                feature_name: feature_name.to_string(),
                session_duration_seconds,
                workflow_completed,
                usage_pattern: usage_pattern.to_string(),
                user_behavior: user_behavior.to_string(),
            };

            // Spawn async task to record metrics
            let monitor_clone = monitor.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor_clone.record_usage_analytics(usage_data).await {
                    error!(
                        "Failed to record usage analytics in performance monitor: {}",
                        e
                    );
                }
            });
        }
    }

    /// Generate performance report
    pub async fn generate_performance_report(
        &self,
        hours: Option<u64>,
    ) -> RhemaResult<crate::performance::PerformanceReport> {
        if let Some(monitor) = &self.performance_monitor {
            let hours = hours.unwrap_or(24);
            let period = crate::performance::ReportPeriod {
                start: chrono::Utc::now() - chrono::Duration::hours(hours as i64),
                end: chrono::Utc::now(),
                duration_seconds: hours * 3600,
            };

            let report = monitor.generate_performance_report(period).await?;
            self.metrics.performance_reports_generated.inc();

            Ok(report)
        } else {
            Err(RhemaError::ConfigError(
                "Performance monitoring is not enabled".to_string(),
            ))
        }
    }
}

/// Metrics handler for Prometheus
async fn metrics_handler(registry: web::Data<Registry>) -> HttpResponse {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&registry.gather(), &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(buffer)
}

/// Health check handler
async fn health_handler(health: web::Data<Arc<RwLock<HealthStatus>>>) -> HttpResponse {
    let health_status = health.read().await;

    let response = serde_json::json!({
        "status": health_status.status,
        "timestamp": health_status.timestamp,
        "version": health_status.version,
        "uptime_seconds": health_status.uptime.as_secs(),
        "checks": health_status.checks
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

/// Readiness probe handler
async fn ready_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready"
    }))
}

/// Liveness probe handler
async fn live_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "alive"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_service_creation() {
        let service = MonitoringService::new().unwrap();
        assert!(service.metrics().requests_total.get() == 0.0);
    }

    #[tokio::test]
    async fn test_metrics_recording() {
        let service = MonitoringService::new().unwrap();

        service.record_request(Duration::from_millis(100));
        service.record_ai_request(Duration::from_millis(50), true);
        service.record_git_operation(Duration::from_millis(200), true);
        service.record_file_operation(Duration::from_millis(75), false);

        assert!(service.metrics().requests_total.get() == 1.0);
        assert!(service.metrics().ai_requests_total.get() == 1.0);
        assert!(service.metrics().ai_cache_hits.get() == 1.0);
        assert!(service.metrics().git_operations_total.get() == 1.0);
        assert!(service.metrics().file_operations_total.get() == 1.0);
        assert!(service.metrics().file_errors_total.get() == 1.0);
    }
}
