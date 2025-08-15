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

//! Enhanced Monitoring and Error Handling Example
//!
//! This example demonstrates the comprehensive error handling, resilience,
//! monitoring, and observability features of the gRPC coordination client.
//!
//! Features demonstrated:
//! - Error handling and retry mechanisms
//! - Connection recovery and resilience
//! - Real-time monitoring and metrics collection
//! - Health checking and alerting
//! - Performance monitoring and diagnostics
//! - Custom alert handlers
//! - Prometheus metrics export

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn, error, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rhema_coordination::grpc::{
    SyneidesisCoordinationClient, SyneidesisConfig, CoordinationError,
    ClientMetrics, ConnectionStatus, CoordinationMonitor, MonitoringConfig,
    AlertHandler, Alert, AlertSeverity, AlertType, LoggingAlertHandler,
    PrometheusExporter, HealthStatus
};
use rhema_coordination::agent::real_time_coordination::{AgentInfo, AgentMessage, AgentStatus, MessageType, MessagePriority};

/// Custom alert handler that sends alerts to a webhook
struct WebhookAlertHandler {
    webhook_url: String,
}

#[async_trait::async_trait]
impl AlertHandler for WebhookAlertHandler {
    async fn handle_alert(&self, alert: Alert) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would send the alert to a webhook
        info!("Webhook Alert - Severity: {:?}, Type: {:?}, Message: {}", 
              alert.severity, alert.alert_type, alert.message);
        
        // Simulate webhook call
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        Ok(())
    }
}

/// Custom alert handler for Slack notifications
struct SlackAlertHandler {
    channel: String,
}

#[async_trait::async_trait]
impl AlertHandler for SlackAlertHandler {
    async fn handle_alert(&self, alert: Alert) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would send the alert to Slack
        let emoji = match alert.severity {
            AlertSeverity::Info => "ℹ️",
            AlertSeverity::Warning => "⚠️",
            AlertSeverity::Error => "❌",
            AlertSeverity::Critical => "🚨",
        };
        
        info!("Slack Alert {} - Channel: {}, Message: {}", 
              emoji, self.channel, alert.message);
        
        // Simulate Slack API call
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing with structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting Enhanced Monitoring and Error Handling Example");

    // Create enhanced configuration with monitoring
    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://127.0.0.1:50051".to_string());
    config.enable_health_monitoring = true;
    config.connection_recovery_enabled = true;
    config.max_retries = 5;
    config.retry_backoff_ms = 2000;
    config.health_check_interval_seconds = 30;
    config.max_reconnection_attempts = 10;
    config.reconnection_backoff_ms = 5000;

    info!("📋 Configuration: {:?}", config);

    // Create monitoring configuration
    let mut monitoring_config = MonitoringConfig::default();
    monitoring_config.enabled = true;
    monitoring_config.alerting_enabled = true;
    monitoring_config.metrics_collection_interval_seconds = 15;
    monitoring_config.health_check_interval_seconds = 30;
    monitoring_config.performance_monitoring_enabled = true;
    monitoring_config.connection_monitoring_enabled = true;

    // Configure performance thresholds
    monitoring_config.performance_thresholds.max_response_time_ms = 3000;
    monitoring_config.performance_thresholds.max_error_rate = 0.05;
    monitoring_config.performance_thresholds.min_success_rate = 0.95;

    // Configure health thresholds
    monitoring_config.health_thresholds.max_consecutive_failures = 3;
    monitoring_config.health_thresholds.max_error_rate = 0.1;

    info!("📊 Monitoring Configuration: {:?}", monitoring_config);

    // Create client metrics
    let client_metrics = Arc::new(ClientMetrics::new());
    let connection_status = Arc::new(RwLock::new(ConnectionStatus::Disconnected));

    // Create coordination client with enhanced error handling
    info!("🔌 Creating Syneidesis coordination client...");
    let client = match SyneidesisCoordinationClient::new(config).await {
        Ok(client) => {
            info!("✅ Client created successfully");
            client
        }
        Err(e) => {
            error!("❌ Failed to create client: {}", e);
            return Err(e.into());
        }
    };

    // Create monitoring instance
    info!("📈 Setting up monitoring and observability...");
    let monitor = CoordinationMonitor::new(
        monitoring_config,
        client_metrics.clone(),
        connection_status.clone(),
    );

    // Add custom alert handlers
    monitor.add_alert_handler(Box::new(LoggingAlertHandler)).await;
    monitor.add_alert_handler(Box::new(WebhookAlertHandler {
        webhook_url: "https://api.example.com/webhook".to_string(),
    })).await;
    monitor.add_alert_handler(Box::new(SlackAlertHandler {
        channel: "#alerts".to_string(),
    })).await;

    // Start monitoring
    monitor.start().await?;
    info!("✅ Monitoring started successfully");

    // Create test agent
    let agent_info = AgentInfo {
        id: "test-agent-001".to_string(),
        name: "Test Agent".to_string(),
        agent_type: "example".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "test".to_string(),
        capabilities: vec!["monitoring".to_string(), "testing".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: Default::default(),
    };

    // Demonstrate error handling and resilience
    info!("🔄 Demonstrating error handling and resilience...");
    
    // Attempt to register agent (this will fail since server is not running)
    match client.register_agent(agent_info.clone()).await {
        Ok(()) => {
            info!("✅ Agent registered successfully");
        }
        Err(e) => {
            warn!("⚠️ Agent registration failed (expected): {}", e);
            
            // Demonstrate error type handling
            match e {
                CoordinationError::ConnectionFailed(msg) => {
                    info!("🔍 Connection failed: {}", msg);
                }
                CoordinationError::NotConnected => {
                    info!("🔍 Not connected to server");
                }
                CoordinationError::RetryLimitExceeded { operation, attempts } => {
                    info!("🔍 Retry limit exceeded for {} after {} attempts", operation, attempts);
                }
                _ => {
                    info!("🔍 Other error: {}", e);
                }
            }
        }
    }

    // Demonstrate metrics collection
    info!("📊 Demonstrating metrics collection...");
    
    // Simulate some operations to generate metrics
    for i in 0..10 {
        let message = AgentMessage {
            id: format!("msg-{}", i),
            message_type: MessageType::Custom("test".to_string()),
            priority: MessagePriority::Normal,
            sender_id: "test-sender".to_string(),
            recipient_ids: vec!["test-recipient".to_string()],
            content: format!("Test message {}", i),
            payload: None,
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        // Record operation result for monitoring
        let start_time = std::time::Instant::now();
        let success = i % 3 != 0; // Simulate some failures
        let response_time = Duration::from_millis(100 + (i * 50) as u64);
        
        monitor.record_operation_result("send_message", success, response_time).await;
        
        if success {
            info!("✅ Simulated successful message send {}", i);
        } else {
            warn!("⚠️ Simulated failed message send {}", i);
        }
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Get and display current metrics
    info!("📈 Current Metrics:");
    let metrics = client_metrics.as_ref();
    info!("  Total Requests: {}", metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed));
    info!("  Successful Requests: {}", metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed));
    info!("  Failed Requests: {}", metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed));
    info!("  Success Rate: {:.2}%", metrics.get_success_rate() * 100.0);
    info!("  Average Response Time: {}ms", metrics.average_response_time.load(std::sync::atomic::Ordering::Relaxed));
    info!("  Connection Success Rate: {:.2}%", metrics.get_connection_success_rate() * 100.0);

    // Get health status
    info!("🏥 Health Status:");
    let health = monitor.get_health_status().await;
    info!("  Status: {:?}", health.status);
    info!("  Message: {}", health.message);
    info!("  Error Rate: {:.4}", health.error_rate);
    info!("  Details: {:?}", health.details);

    // Export Prometheus metrics
    info!("📊 Prometheus Metrics Export:");
    let prometheus_metrics = PrometheusExporter::export_prometheus_metrics(metrics);
    println!("{}", prometheus_metrics);

    // Get connection diagnostics
    info!("🔍 Connection Diagnostics:");
    let diagnostics = monitor.get_connection_diagnostics().await;
    info!("  Server Address: {}", diagnostics.server_address);
    info!("  Connection Status: {:?}", diagnostics.connection_status);
    info!("  Connection Attempts: {}", diagnostics.connection_attempts);
    info!("  Successful Connections: {}", diagnostics.successful_connections);
    info!("  Failed Connections: {}", diagnostics.failed_connections);
    info!("  Connection Success Rate: {:.2}%", diagnostics.connection_success_rate * 100.0);

    // Get performance metrics
    info!("⚡ Performance Metrics:");
    let performance_metrics = monitor.get_performance_metrics().await;
    for metric in performance_metrics.iter().take(3) {
        info!("  Operation: {}", metric.operation_name);
        info!("    Total Operations: {}", metric.total_operations);
        info!("    Success Rate: {:.2}%", (metric.successful_operations as f64 / metric.total_operations as f64) * 100.0);
        info!("    Average Response Time: {:?}", metric.average_response_time);
        info!("    Throughput: {:.2} ops/sec", metric.throughput_ops_per_second);
    }

    // Demonstrate health monitoring
    info!("🏥 Demonstrating health monitoring...");
    
    // Perform health check
    match client.health_check().await {
        Ok(()) => {
            info!("✅ Health check passed");
        }
        Err(e) => {
            warn!("⚠️ Health check failed: {}", e);
        }
    }

    // Wait for some monitoring data to be collected
    info!("⏳ Waiting for monitoring data collection...");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Demonstrate alerting
    info!("🚨 Demonstrating alerting system...");
    
    // Simulate a high response time alert
    monitor.record_operation_result(
        "slow_operation",
        true,
        Duration::from_millis(5000), // Above threshold
    ).await;

    // Simulate a high error rate alert
    for _ in 0..5 {
        monitor.record_operation_result("failing_operation", false, Duration::from_millis(100)).await;
    }

    // Wait for alerts to be processed
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Demonstrate graceful shutdown
    info!("🛑 Demonstrating graceful shutdown...");
    
    // Shutdown client
    client.shutdown().await?;
    info!("✅ Client shutdown completed");

    // Stop monitoring
    monitor.stop().await?;
    info!("✅ Monitoring stopped");

    info!("🎉 Enhanced Monitoring and Error Handling Example completed successfully!");
    info!("");
    info!("📋 Summary of demonstrated features:");
    info!("  ✅ Comprehensive error handling with custom error types");
    info!("  ✅ Retry mechanisms with exponential backoff");
    info!("  ✅ Connection recovery and resilience");
    info!("  ✅ Real-time metrics collection and monitoring");
    info!("  ✅ Health status monitoring and alerting");
    info!("  ✅ Performance monitoring and diagnostics");
    info!("  ✅ Custom alert handlers (Logging, Webhook, Slack)");
    info!("  ✅ Prometheus metrics export");
    info!("  ✅ Structured logging and tracing");
    info!("  ✅ Graceful shutdown and cleanup");

    Ok(())
}
