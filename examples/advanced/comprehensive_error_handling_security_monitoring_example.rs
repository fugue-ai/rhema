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

use rhema_knowledge::{
    error_handling::{
        ErrorCategory, ErrorContext, ErrorHandler, ErrorHandlerConfig, ErrorLoggingConfig,
        ErrorReportingConfig, ErrorSeverity, RecoveryStrategy,
    },
    monitoring::{
        AlertSeverity, HealthState, MonitoringConfig, MonitoringManager, MetricType,
        AlertingThresholds, DashboardConfig, TracingConfig,
    },
    security::{
        AuditAction, EncryptionAlgorithm, Permission, SecurityConfig, SecurityManager,
        UserRole, ValidationRule, ValidationRuleType,
    },
    types::{KnowledgeError, KnowledgeResult},
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Comprehensive example demonstrating error handling, security, and monitoring features
#[tokio::main]
async fn main() -> KnowledgeResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("🚀 Rhema Knowledge - Comprehensive Error Handling, Security & Monitoring Example");
    println!("================================================================================\n");

    // 1. Initialize Error Handling System
    println!("1. Setting up Error Handling System...");
    let error_handler = setup_error_handling().await?;
    println!("✅ Error handling system initialized\n");

    // 2. Initialize Security System
    println!("2. Setting up Security System...");
    let security_manager = setup_security_system().await?;
    println!("✅ Security system initialized\n");

    // 3. Initialize Monitoring System
    println!("3. Setting up Monitoring System...");
    let monitoring_manager = setup_monitoring_system().await?;
    println!("✅ Monitoring system initialized\n");

    // 4. Demonstrate Error Handling
    println!("4. Demonstrating Error Handling Features...");
    demonstrate_error_handling(&error_handler).await?;
    println!("✅ Error handling demonstration completed\n");

    // 5. Demonstrate Security Features
    println!("5. Demonstrating Security Features...");
    demonstrate_security_features(&security_manager).await?;
    println!("✅ Security demonstration completed\n");

    // 6. Demonstrate Monitoring Features
    println!("6. Demonstrating Monitoring Features...");
    demonstrate_monitoring_features(&monitoring_manager).await?;
    println!("✅ Monitoring demonstration completed\n");

    // 7. Demonstrate Integration
    println!("7. Demonstrating Integrated Features...");
    demonstrate_integrated_features(&error_handler, &security_manager, &monitoring_manager).await?;
    println!("✅ Integration demonstration completed\n");

    // 8. Generate Reports
    println!("8. Generating System Reports...");
    generate_system_reports(&error_handler, &security_manager, &monitoring_manager).await?;
    println!("✅ System reports generated\n");

    println!("🎉 All demonstrations completed successfully!");
    println!("The knowledge system now has comprehensive error handling, security, and monitoring capabilities.");

    Ok(())
}

/// Setup comprehensive error handling system
async fn setup_error_handling() -> KnowledgeResult<Arc<ErrorHandler>> {
    let config = ErrorHandlerConfig {
        enable_recovery: true,
        enable_reporting: true,
        enable_logging: true,
        max_error_contexts: 1000,
        error_cleanup_interval_ms: 3600000, // 1 hour
        default_retry_attempts: 3,
        default_backoff_ms: 1000,
    };

    let logging_config = ErrorLoggingConfig {
        enabled: true,
        log_level: tracing::Level::ERROR,
        include_stack_traces: true,
        include_context: true,
        structured_logging: true,
        log_file_path: Some("logs/errors.json".to_string()),
        max_log_size_mb: 100,
        log_retention_days: 30,
    };

    let reporting_config = ErrorReportingConfig {
        enabled: true,
        reporting_endpoint: Some("https://api.monitoring.example.com/errors".to_string()),
        api_key: Some("your-api-key".to_string()),
        include_stack_traces: true,
        include_context: true,
        batch_size: 100,
        batch_timeout_ms: 5000,
        retry_on_failure: true,
    };

    let handler = ErrorHandler::new(config)
        .with_logging_config(logging_config)
        .with_reporting_config(reporting_config);

    Ok(Arc::new(handler))
}

/// Setup comprehensive security system
async fn setup_security_system() -> KnowledgeResult<Arc<SecurityManager>> {
    let mut config = SecurityConfig::default();
    config.enable_authentication = true;
    config.enable_authorization = true;
    config.enable_input_validation = true;
    config.enable_audit_logging = true;
    config.enable_encryption = true;
    config.enable_secure_communication = true;
    config.session_timeout_minutes = 60;
    config.max_login_attempts = 5;
    config.password_min_length = 12;
    config.require_strong_passwords = true;
    config.encryption_algorithm = EncryptionAlgorithm::Aes256Gcm;
    config.audit_log_retention_days = 90;

    let mut security_manager = SecurityManager::new(config);
    security_manager.initialize().await?;

    Ok(Arc::new(security_manager))
}

/// Setup comprehensive monitoring system
async fn setup_monitoring_system() -> KnowledgeResult<Arc<MonitoringManager>> {
    let alerting_thresholds = AlertingThresholds {
        error_rate_threshold: 0.05, // 5%
        response_time_threshold_ms: 1000, // 1 second
        memory_usage_threshold_percent: 80.0, // 80%
        disk_usage_threshold_percent: 85.0, // 85%
        cache_hit_rate_threshold: 0.7, // 70%
        search_latency_threshold_ms: 500, // 500ms
    };

    let dashboard_config = DashboardConfig {
        enable_real_time_updates: true,
        update_interval_ms: 1000, // 1 second
        max_data_points: 1000,
        retention_days: 30,
        export_formats: vec!["json".to_string(), "csv".to_string(), "prometheus".to_string()],
    };

    let tracing_config = TracingConfig {
        enable_distributed_tracing: true,
        sampling_rate: 0.1, // 10%
        max_trace_duration_ms: 30000, // 30 seconds
        trace_export_endpoint: Some("https://api.tracing.example.com/traces".to_string()),
        trace_export_batch_size: 100,
    };

    let config = MonitoringConfig {
        enable_metrics: true,
        enable_health_checks: true,
        enable_tracing: true,
        enable_alerting: true,
        enable_dashboards: true,
        metrics_collection_interval_ms: 5000, // 5 seconds
        health_check_interval_ms: 30000, // 30 seconds
        alerting_thresholds,
        dashboard_config,
        tracing_config,
    };

    let monitoring_manager = MonitoringManager::new(config);
    monitoring_manager.start_monitoring().await?;

    Ok(Arc::new(monitoring_manager))
}

/// Demonstrate error handling features
async fn demonstrate_error_handling(error_handler: &Arc<ErrorHandler>) -> KnowledgeResult<()> {
    info!("Demonstrating error handling features...");

    // Simulate different types of errors
    let errors = vec![
        KnowledgeError::NetworkError("Connection timeout".to_string()),
        KnowledgeError::ConfigurationError("Invalid database URL".to_string()),
        KnowledgeError::TimeoutError("Operation timed out".to_string()),
        KnowledgeError::InvalidData("Malformed JSON input".to_string()),
        KnowledgeError::FileSystemError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )),
    ];

    for (i, error) in errors.iter().enumerate() {
        info!("Handling error {}: {:?}", i + 1, error);
        
        error_handler
            .handle_error(
                error,
                &format!("test_operation_{}", i + 1),
                Some("test_user"),
                Some("session_123"),
                Some("request_456"),
            )
            .await?;
    }

    // Get error statistics
    let stats = error_handler.get_error_statistics().await;
    info!("Error statistics: {:?}", stats);

    Ok(())
}

/// Demonstrate security features
async fn demonstrate_security_features(security_manager: &Arc<SecurityManager>) -> KnowledgeResult<()> {
    info!("Demonstrating security features...");

    // 1. Input validation
    let mut test_data = HashMap::new();
    test_data.insert("username".to_string(), "testuser123".to_string());
    test_data.insert("email".to_string(), "test@example.com".to_string());
    test_data.insert("password".to_string(), "SecurePassword123!".to_string());

    let validation_result = security_manager.validate_input(&test_data, "user_registration").await;
    match validation_result {
        Ok(_) => info!("✅ Input validation passed"),
        Err(e) => warn!("❌ Input validation failed: {}", e),
    }

    // 2. Access control
    let has_permission = security_manager
        .check_permission("test_user", "/knowledge/private", &Permission::Read)
        .await?;
    info!("Access permission check: {}", has_permission);

    // 3. Password hashing
    let password = "SecurePassword123!";
    let hash = security_manager.hash_password(password).await?;
    let is_valid = security_manager.verify_password(password, &hash).await?;
    info!("Password verification: {}", is_valid);

    // 4. Data encryption
    let sensitive_data = b"This is sensitive information that needs to be encrypted";
    let encrypted = security_manager.encrypt_data(sensitive_data).await?;
    let decrypted = security_manager.decrypt_data(&encrypted).await?;
    info!("Data encryption/decryption: {}", sensitive_data == decrypted.as_slice());

    // 5. Session management
    let session_id = security_manager
        .create_session("test_user", Some("127.0.0.1"), Some("test-agent"))
        .await?;
    let user_id = security_manager.validate_session(&session_id).await?;
    info!("Session validation: {:?}", user_id);

    // 6. Audit logging
    let mut audit_details = HashMap::new();
    audit_details.insert("operation_type".to_string(), "data_access".to_string());
    audit_details.insert("resource_id".to_string(), "doc_123".to_string());

    security_manager
        .log_audit_event(
            "test_user",
            Some(&session_id),
            "read_document",
            "/knowledge/documents/doc_123",
            AuditAction::Read,
            true,
            audit_details,
            Some("127.0.0.1"),
            Some("test-agent"),
        )
        .await?;

    Ok(())
}

/// Demonstrate monitoring features
async fn demonstrate_monitoring_features(monitoring_manager: &Arc<MonitoringManager>) -> KnowledgeResult<()> {
    info!("Demonstrating monitoring features...");

    // 1. Record custom metrics
    let mut labels = HashMap::new();
    labels.insert("service".to_string(), "knowledge".to_string());
    labels.insert("version".to_string(), "1.0.0".to_string());

    monitoring_manager
        .record_metric("custom.operation.count", 42.0, MetricType::Counter, labels.clone())
        .await?;

    monitoring_manager
        .record_metric("custom.response.time", 150.0, MetricType::Histogram, labels.clone())
        .await?;

    monitoring_manager
        .record_metric("custom.memory.usage", 75.5, MetricType::Gauge, labels)
        .await?;

    // 2. Distributed tracing
    let mut trace_tags = HashMap::new();
    trace_tags.insert("operation".to_string(), "search_query".to_string());
    trace_tags.insert("user_id".to_string(), "test_user".to_string());

    let trace_id = monitoring_manager.start_trace("semantic_search", trace_tags).await?;
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    monitoring_manager.end_trace(&trace_id, monitoring::TraceStatus::Completed).await?;

    // 3. Get health status
    let health_status = monitoring_manager.get_health_status().await;
    info!("Health status: {:?}", health_status.overall_status);

    // 4. Get metrics
    let metrics = monitoring_manager
        .get_metrics(
            Some(vec!["custom.operation.count".to_string()]),
            None,
            None,
        )
        .await?;
    info!("Retrieved {} metric series", metrics.len());

    // 5. Check for alerts
    let alerts = monitoring_manager.get_alerts(None, None, None).await;
    info!("Active alerts: {}", alerts.len());

    Ok(())
}

/// Demonstrate integrated features
async fn demonstrate_integrated_features(
    error_handler: &Arc<ErrorHandler>,
    security_manager: &Arc<SecurityManager>,
    monitoring_manager: &Arc<MonitoringManager>,
) -> KnowledgeResult<()> {
    info!("Demonstrating integrated features...");

    // Simulate a secure operation with monitoring and error handling
    let operation_name = "secure_knowledge_search";
    let user_id = "test_user";
    let session_id = "session_123";
    let request_id = "request_456";

    // Start monitoring trace
    let mut trace_tags = HashMap::new();
    trace_tags.insert("user_id".to_string(), user_id.to_string());
    trace_tags.insert("operation".to_string(), operation_name.to_string());

    let trace_id = monitoring_manager.start_trace(operation_name, trace_tags).await?;

    // Check security permissions
    let has_permission = security_manager
        .check_permission(user_id, "/knowledge/search", &Permission::Read)
        .await?;

    if !has_permission {
        let error = KnowledgeError::SecurityError("Access denied".to_string());
        error_handler
            .handle_error(&error, operation_name, Some(user_id), Some(session_id), Some(request_id))
            .await?;
        
        monitoring_manager.end_trace(&trace_id, monitoring::TraceStatus::Failed).await?;
        return Err(error);
    }

    // Simulate the actual operation
    let result = perform_secure_operation().await;
    
    match result {
        Ok(_) => {
            // Log successful audit event
            let mut audit_details = HashMap::new();
            audit_details.insert("result_count".to_string(), "5".to_string());
            audit_details.insert("search_time_ms".to_string(), "150".to_string());

            security_manager
                .log_audit_event(
                    user_id,
                    Some(session_id),
                    operation_name,
                    "/knowledge/search",
                    AuditAction::Read,
                    true,
                    audit_details,
                    Some("127.0.0.1"),
                    Some("test-agent"),
                )
                .await?;

            // Record success metrics
            let mut labels = HashMap::new();
            labels.insert("operation".to_string(), operation_name.to_string());
            labels.insert("status".to_string(), "success".to_string());

            monitoring_manager
                .record_metric("secure_operations.success", 1.0, MetricType::Counter, labels)
                .await?;

            monitoring_manager.end_trace(&trace_id, monitoring::TraceStatus::Completed).await?;
        }
        Err(error) => {
            // Handle error
            error_handler
                .handle_error(&error, operation_name, Some(user_id), Some(session_id), Some(request_id))
                .await?;

            // Log failed audit event
            let mut audit_details = HashMap::new();
            audit_details.insert("error_type".to_string(), format!("{:?}", error));

            security_manager
                .log_audit_event(
                    user_id,
                    Some(session_id),
                    operation_name,
                    "/knowledge/search",
                    AuditAction::Read,
                    false,
                    audit_details,
                    Some("127.0.0.1"),
                    Some("test-agent"),
                )
                .await?;

            // Record failure metrics
            let mut labels = HashMap::new();
            labels.insert("operation".to_string(), operation_name.to_string());
            labels.insert("status".to_string(), "failure".to_string());

            monitoring_manager
                .record_metric("secure_operations.failure", 1.0, MetricType::Counter, labels)
                .await?;

            monitoring_manager.end_trace(&trace_id, monitoring::TraceStatus::Failed).await?;
        }
    }

    Ok(())
}

/// Simulate a secure operation
async fn perform_secure_operation() -> KnowledgeResult<()> {
    // Simulate some work that might fail
    if rand::random::<bool>() {
        Ok(())
    } else {
        Err(KnowledgeError::NetworkError("Simulated network error".to_string()))
    }
}

/// Generate comprehensive system reports
async fn generate_system_reports(
    error_handler: &Arc<ErrorHandler>,
    security_manager: &Arc<SecurityManager>,
    monitoring_manager: &Arc<MonitoringManager>,
) -> KnowledgeResult<()> {
    info!("Generating system reports...");

    // 1. Error handling report
    let error_stats = error_handler.get_error_statistics().await;
    println!("\n📊 Error Handling Report:");
    println!("  Total errors: {}", error_stats.total_errors);
    println!("  Errors in last 24h: {}", error_stats.errors_last_24h);
    println!("  Errors in last hour: {}", error_stats.errors_last_hour);
    println!("  Errors by category: {:?}", error_stats.errors_by_category);
    println!("  Errors by severity: {:?}", error_stats.errors_by_severity);

    // 2. Security report
    let security_stats = security_manager.get_security_statistics().await;
    println!("\n🔒 Security Report:");
    println!("  Total audit events: {}", security_stats.total_audit_events);
    println!("  Successful operations: {}", security_stats.successful_operations);
    println!("  Failed operations: {}", security_stats.failed_operations);
    println!("  Active sessions: {}", security_stats.active_sessions);
    println!("  Failed login attempts: {}", security_stats.failed_login_attempts);
    println!("  Operations by type: {:?}", security_stats.operations_by_type);

    // 3. Monitoring report
    let monitoring_stats = monitoring_manager.get_monitoring_statistics().await;
    println!("\n📈 Monitoring Report:");
    println!("  Total metrics: {}", monitoring_stats.total_metrics);
    println!("  Health status: {:?}", monitoring_stats.health_status);
    println!("  Active traces: {}", monitoring_stats.active_traces);
    println!("  Total alerts: {}", monitoring_stats.total_alerts);
    println!("  Active alerts: {}", monitoring_stats.active_alerts);

    // 4. Health status
    let health_status = monitoring_manager.get_health_status().await;
    println!("\n🏥 Health Status:");
    println!("  Overall status: {:?}", health_status.overall_status);
    println!("  Last updated: {}", health_status.last_updated);
    for (check_name, check) in &health_status.checks {
        println!("  {}: {:?} ({}ms)", check_name, check.status, check.response_time_ms);
    }

    // 5. Recent alerts
    let alerts = monitoring_manager.get_alerts(None, Some(false), Some(false)).await;
    println!("\n🚨 Recent Alerts:");
    for alert in alerts.iter().take(5) {
        println!("  [{}] {}: {} ({}%)", 
            alert.severity, 
            alert.title, 
            alert.message,
            (alert.metric_value / alert.threshold * 100.0) as i32
        );
    }

    Ok(())
}
