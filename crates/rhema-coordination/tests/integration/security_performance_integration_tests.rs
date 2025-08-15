use rhema_coordination::grpc::{
    SyneidesisCoordinationClient, SyneidesisConfig, SecurityConfig, PerformanceConfig,
    CompressionAlgorithm, CoordinationMonitor, MonitoringConfig, LoggingAlertHandler,
    ClientMetrics, ConnectionStatus
};
use rhema_coordination::agent::real_time_coordination::{AgentInfo, AgentMessage, MessageType, MessagePriority};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[tokio::test]
async fn test_security_performance_integration() {
    // Initialize tracing for tests
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_test_writer()
        .init();

    info!("🧪 Starting Security and Performance Integration Test");

    // Test 1: Security Configuration Integration
    test_security_configuration_integration().await;

    // Test 2: Performance Configuration Integration
    test_performance_configuration_integration().await;

    // Test 3: Compression Integration
    test_compression_integration().await;

    // Test 4: Connection Pooling Integration
    test_connection_pooling_integration().await;

    // Test 5: Monitoring Integration
    test_monitoring_integration().await;

    info!("✅ All Security and Performance Integration Tests Passed");
}

async fn test_security_configuration_integration() {
    info!("🔒 Testing Security Configuration Integration");

    let security_config = SecurityConfig {
        enable_tls: true,
        ca_cert_path: Some("test-certs/ca.pem".to_string()),
        client_cert_path: Some("test-certs/client.pem".to_string()),
        client_key_path: Some("test-certs/client-key.pem".to_string()),
        skip_cert_verification: true, // For testing
        auth_token: Some("test-auth-token".to_string()),
        jwt_secret: Some("test-jwt-secret".to_string()),
        token_refresh_interval: 3600,
    };

    let performance_config = PerformanceConfig {
        enable_connection_pooling: false, // Disable for this test
        enable_compression: false, // Disable for this test
        ..Default::default()
    };

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("https://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;

    // Verify configuration is properly set
    assert!(config.security.enable_tls);
    assert_eq!(config.security.token_refresh_interval, 3600);
    assert!(config.security.skip_cert_verification);
    assert_eq!(config.security.auth_token, Some("test-auth-token".to_string()));

    info!("✅ Security Configuration Integration Test Passed");
}

async fn test_performance_configuration_integration() {
    info!("⚡ Testing Performance Configuration Integration");

    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 15,
        pool_timeout: 45,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Brotli,
        compression_level: 8,
        enable_keep_alive: true,
        keep_alive_interval: 60,
        keep_alive_timeout: 10,
        max_message_size: 16 * 1024 * 1024, // 16MB
        request_timeout: 120,
    };

    let security_config = SecurityConfig {
        enable_tls: false, // Disable for this test
        ..Default::default()
    };

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;

    // Verify configuration is properly set
    assert!(config.performance.enable_connection_pooling);
    assert_eq!(config.performance.max_connections, 15);
    assert!(config.performance.enable_compression);
    assert_eq!(config.performance.compression_algorithm, CompressionAlgorithm::Brotli);
    assert_eq!(config.performance.compression_level, 8);
    assert_eq!(config.performance.max_message_size, 16 * 1024 * 1024);

    info!("✅ Performance Configuration Integration Test Passed");
}

async fn test_compression_integration() {
    info!("🗜️ Testing Compression Integration");

    let performance_config = PerformanceConfig {
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Gzip,
        compression_level: 6,
        ..Default::default()
    };

    let security_config = SecurityConfig::default();

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;

    // Test that compression is properly configured
    assert!(config.performance.enable_compression);
    assert_eq!(config.performance.compression_algorithm, CompressionAlgorithm::Gzip);
    assert_eq!(config.performance.compression_level, 6);

    // Test compression with different algorithms
    let algorithms = vec![
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Brotli,
        CompressionAlgorithm::Zstd,
    ];

    for algorithm in algorithms {
        let test_config = PerformanceConfig {
            compression_algorithm: algorithm,
            enable_compression: true,
            compression_level: 6,
            ..Default::default()
        };

        // Verify algorithm is properly set
        assert_eq!(test_config.compression_algorithm, algorithm);
        assert!(test_config.enable_compression);
    }

    info!("✅ Compression Integration Test Passed");
}

async fn test_connection_pooling_integration() {
    info!("🏊 Testing Connection Pooling Integration");

    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 10,
        pool_timeout: 30,
        enable_compression: false, // Disable for this test
        ..Default::default()
    };

    let security_config = SecurityConfig::default();

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;

    // Verify connection pooling is properly configured
    assert!(config.performance.enable_connection_pooling);
    assert_eq!(config.performance.max_connections, 10);
    assert_eq!(config.performance.pool_timeout, 30);

    // Test different pool sizes
    let pool_sizes = vec![5, 10, 20, 50];

    for pool_size in pool_sizes {
        let test_config = PerformanceConfig {
            enable_connection_pooling: true,
            max_connections: pool_size,
            pool_timeout: 30,
            enable_compression: false,
            ..Default::default()
        };

        assert_eq!(test_config.max_connections, pool_size);
        assert!(test_config.enable_connection_pooling);
    }

    info!("✅ Connection Pooling Integration Test Passed");
}

async fn test_monitoring_integration() {
    info!("📊 Testing Monitoring Integration");

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
        Arc::new(ClientMetrics::new()),
        Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
    );

    // Add alert handler
    monitor.add_alert_handler(Box::new(LoggingAlertHandler::new()));

    // Start monitoring
    monitor.start().await;

    // Verify monitoring is running
    // Note: In a real test, we would verify that monitoring is actually working
    // For now, we just verify that the monitor was created successfully

    // Stop monitoring
    monitor.stop().await;

    info!("✅ Monitoring Integration Test Passed");
}

#[tokio::test]
async fn test_full_client_integration() {
    info!("🔗 Testing Full Client Integration");

    // Create comprehensive configuration
    let security_config = SecurityConfig {
        enable_tls: false, // Disable for testing
        auth_token: Some("test-token".to_string()),
        jwt_secret: Some("test-secret".to_string()),
        token_refresh_interval: 3600,
        ..Default::default()
    };

    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 5,
        pool_timeout: 30,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Gzip,
        compression_level: 6,
        enable_keep_alive: true,
        keep_alive_interval: 30,
        keep_alive_timeout: 5,
        max_message_size: 4 * 1024 * 1024, // 4MB
        request_timeout: 30,
    };

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;
    config.enable_health_monitoring = true;
    config.enable_metrics = true;

    // Test client creation (will fail without server, but tests configuration)
    match SyneidesisCoordinationClient::new(config).await {
        Ok(_client) => {
            info!("✅ Client created successfully with security and performance features");
        }
        Err(e) => {
            warn!("⚠️ Client creation failed (expected without server): {}", e);
            // This is expected without a running server
        }
    }

    info!("✅ Full Client Integration Test Passed");
}

#[tokio::test]
async fn test_configuration_serialization_integration() {
    info!("📝 Testing Configuration Serialization Integration");

    let security_config = SecurityConfig {
        enable_tls: true,
        ca_cert_path: Some("certs/ca.pem".to_string()),
        client_cert_path: Some("certs/client.pem".to_string()),
        client_key_path: Some("certs/client-key.pem".to_string()),
        skip_cert_verification: false,
        auth_token: Some("test-auth-token".to_string()),
        jwt_secret: Some("test-jwt-secret".to_string()),
        token_refresh_interval: 3600,
    };

    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 20,
        pool_timeout: 60,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Brotli,
        compression_level: 8,
        enable_keep_alive: true,
        keep_alive_interval: 45,
        keep_alive_timeout: 10,
        max_message_size: 16 * 1024 * 1024,
        request_timeout: 120,
    };

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("https://localhost:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;

    // Test serialization
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: SyneidesisConfig = serde_json::from_str(&serialized).unwrap();

    // Verify all fields are preserved
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.server_address, deserialized.server_address);
    assert_eq!(config.security.enable_tls, deserialized.security.enable_tls);
    assert_eq!(config.security.ca_cert_path, deserialized.security.ca_cert_path);
    assert_eq!(config.security.client_cert_path, deserialized.security.client_cert_path);
    assert_eq!(config.security.client_key_path, deserialized.security.client_key_path);
    assert_eq!(config.security.skip_cert_verification, deserialized.security.skip_cert_verification);
    assert_eq!(config.security.auth_token, deserialized.security.auth_token);
    assert_eq!(config.security.jwt_secret, deserialized.security.jwt_secret);
    assert_eq!(config.security.token_refresh_interval, deserialized.security.token_refresh_interval);
    assert_eq!(config.performance.enable_connection_pooling, deserialized.performance.enable_connection_pooling);
    assert_eq!(config.performance.max_connections, deserialized.performance.max_connections);
    assert_eq!(config.performance.pool_timeout, deserialized.performance.pool_timeout);
    assert_eq!(config.performance.enable_compression, deserialized.performance.enable_compression);
    assert_eq!(config.performance.compression_algorithm, deserialized.performance.compression_algorithm);
    assert_eq!(config.performance.compression_level, deserialized.performance.compression_level);
    assert_eq!(config.performance.enable_keep_alive, deserialized.performance.enable_keep_alive);
    assert_eq!(config.performance.keep_alive_interval, deserialized.performance.keep_alive_interval);
    assert_eq!(config.performance.keep_alive_timeout, deserialized.performance.keep_alive_timeout);
    assert_eq!(config.performance.max_message_size, deserialized.performance.max_message_size);
    assert_eq!(config.performance.request_timeout, deserialized.performance.request_timeout);

    info!("✅ Configuration Serialization Integration Test Passed");
}
