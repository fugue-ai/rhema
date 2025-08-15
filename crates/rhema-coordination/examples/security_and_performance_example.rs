use rhema_coordination::grpc::{
    SyneidesisCoordinationClient, SyneidesisConfig, SecurityConfig, PerformanceConfig,
    CompressionAlgorithm, CoordinationMonitor, MonitoringConfig, LoggingAlertHandler,
    WebhookAlertHandler, SlackAlertHandler
};
use rhema_coordination::agent::real_time_coordination::{AgentInfo, AgentMessage, MessageType, MessagePriority};
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🚀 Starting Security and Performance Example");

    // Configure security settings
    let security_config = SecurityConfig {
        enable_tls: true,
        ca_cert_path: Some("certs/ca.pem".to_string()),
        client_cert_path: Some("certs/client.pem".to_string()),
        client_key_path: Some("certs/client-key.pem".to_string()),
        skip_cert_verification: false, // Set to true for development
        auth_token: Some("your-auth-token".to_string()),
        jwt_secret: Some("your-jwt-secret-key".to_string()),
        token_refresh_interval: 3600,
    };

    // Configure performance settings
    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 20,
        pool_timeout: 30,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Gzip,
        compression_level: 6,
        enable_keep_alive: true,
        keep_alive_interval: 30,
        keep_alive_timeout: 5,
        max_message_size: 8 * 1024 * 1024, // 8MB
        request_timeout: 60,
    };

    // Create comprehensive client configuration
    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("https://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;
    config.enable_health_monitoring = true;
    config.enable_metrics = true;

    info!("📋 Client Configuration:");
    info!("  - TLS Enabled: {}", config.security.enable_tls);
    info!("  - Connection Pooling: {}", config.performance.enable_connection_pooling);
    info!("  - Compression: {:?}", config.performance.compression_algorithm);
    info!("  - Max Connections: {}", config.performance.max_connections);
    info!("  - Max Message Size: {}MB", config.performance.max_message_size / 1024 / 1024);

    // Create monitoring configuration
    let monitoring_config = MonitoringConfig {
        enable_health_monitoring: true,
        health_check_interval: 30,
        enable_performance_monitoring: true,
        performance_check_interval: 60,
        enable_connection_monitoring: true,
        connection_check_interval: 15,
        enable_alerting: true,
        performance_thresholds: rhema_coordination::grpc::PerformanceThresholds {
            max_response_time_ms: 1000,
            max_error_rate: 0.05,
            max_connection_failures: 3,
            min_throughput_ops_per_sec: 10,
        },
        health_thresholds: rhema_coordination::grpc::HealthThresholds {
            max_latency_ms: 500,
            min_success_rate: 0.95,
            max_consecutive_failures: 2,
        },
    };

    // Create monitoring instance
    let monitor = CoordinationMonitor::new(
        monitoring_config,
        std::sync::Arc::new(rhema_coordination::grpc::ClientMetrics::new()),
        std::sync::Arc::new(tokio::sync::RwLock::new(rhema_coordination::grpc::ConnectionStatus::Disconnected)),
    );

    // Add alert handlers
    monitor.add_alert_handler(Box::new(LoggingAlertHandler::new()));
    
    // Add webhook alert handler (for production)
    let webhook_handler = WebhookAlertHandler::new(
        "https://your-webhook-url.com/alerts".to_string(),
        "your-webhook-secret".to_string(),
    );
    monitor.add_alert_handler(Box::new(webhook_handler));

    // Add Slack alert handler (for production)
    let slack_handler = SlackAlertHandler::new(
        "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK".to_string(),
        "#alerts".to_string(),
    );
    monitor.add_alert_handler(Box::new(slack_handler));

    // Start monitoring
    monitor.start().await;

    info!("🔒 Creating secure gRPC client...");
    
    // Create the client (this will fail without a running server, but demonstrates the setup)
    match SyneidesisCoordinationClient::new(config).await {
        Ok(client) => {
            info!("✅ Secure gRPC client created successfully");
            
            // Demonstrate agent registration with security
            let agent_info = AgentInfo {
                id: "secure-agent-001".to_string(),
                name: "Secure Test Agent".to_string(),
                agent_type: "test".to_string(),
                status: rhema_coordination::agent::real_time_coordination::AgentStatus::Idle,
                current_task_id: None,
                assigned_scope: "test-scope".to_string(),
                capabilities: vec!["secure-communication".to_string()],
                last_heartbeat: chrono::Utc::now(),
                is_online: true,
                performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
            };

            info!("📝 Registering agent with security...");
            match client.register_agent(agent_info).await {
                Ok(()) => info!("✅ Agent registered successfully"),
                Err(e) => warn!("⚠️ Agent registration failed (expected without server): {}", e),
            }

            // Demonstrate secure message sending with compression
            let message = AgentMessage {
                id: "secure-msg-001".to_string(),
                message_type: MessageType::Custom("secure-data".to_string()),
                priority: MessagePriority::High,
                sender_id: "secure-agent-001".to_string(),
                recipient_ids: vec!["other-agent".to_string()],
                content: "This is a secure message with compression".to_string(),
                payload: Some(serde_json::json!({
                    "encrypted_data": "base64-encoded-encrypted-content",
                    "compression": "gzip",
                    "security_level": "high"
                })),
                timestamp: chrono::Utc::now(),
                requires_ack: true,
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
                metadata: std::collections::HashMap::new(),
            };

            info!("📤 Sending secure message with compression...");
            match client.send_message(message).await {
                Ok(()) => info!("✅ Secure message sent successfully"),
                Err(e) => warn!("⚠️ Message sending failed (expected without server): {}", e),
            }

            // Demonstrate performance monitoring
            info!("📊 Performance Metrics:");
            let metrics = client.get_metrics();
            info!("  - Total Requests: {}", metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed));
            info!("  - Successful Requests: {}", metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed));
            info!("  - Failed Requests: {}", metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed));
            info!("  - Connection Attempts: {}", metrics.connection_attempts.load(std::sync::atomic::Ordering::Relaxed));
            info!("  - Average Response Time: {}ms", metrics.average_response_time_ms.load(std::sync::atomic::Ordering::Relaxed));

            // Demonstrate health checking
            info!("🏥 Performing health check...");
            match client.health_check().await {
                Ok(health) => info!("✅ Health check passed: {:?}", health),
                Err(e) => warn!("⚠️ Health check failed (expected without server): {}", e),
            }

            // Demonstrate graceful shutdown
            info!("🛑 Shutting down client gracefully...");
            client.shutdown().await;
            info!("✅ Client shutdown completed");

        }
        Err(e) => {
            error!("❌ Failed to create secure gRPC client: {}", e);
            warn!("This is expected without a running gRPC server");
        }
    }

    // Stop monitoring
    monitor.stop().await;
    info!("✅ Monitoring stopped");

    info!("🎉 Security and Performance Example completed");
    info!("");
    info!("📋 Summary of Security Features:");
    info!("  ✅ TLS encryption support");
    info!("  ✅ Client certificate authentication");
    info!("  ✅ JWT token authentication");
    info!("  ✅ Certificate validation");
    info!("  ✅ Token refresh mechanism");
    info!("");
    info!("📋 Summary of Performance Features:");
    info!("  ✅ Connection pooling");
    info!("  ✅ Message compression (Gzip/Brotli/Zstd)");
    info!("  ✅ Keep-alive connections");
    info!("  ✅ Configurable timeouts");
    info!("  ✅ Performance monitoring");
    info!("  ✅ Real-time metrics collection");
    info!("  ✅ Automated alerting");

    Ok(())
}
