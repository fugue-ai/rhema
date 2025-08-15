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

use rhema_coordination::{
    advanced_features::{
        AlertingConfig, AlertingSystem, AuditConfig, AuditLogger, TracingConfig, TracingSystem,
    },
    testing::{
        ChaosTestingConfig, ChaosTestingSystem, SecurityTestingConfig, SecurityTestingSystem,
    },
    AdvancedFeaturesConfig, ProductionConfig, ProductionIntegration,
};
use std::collections::HashMap;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Comprehensive Monitoring Example");

    // Create comprehensive monitoring system
    let monitoring_system = create_monitoring_system().await?;

    // Demonstrate alerting system
    demonstrate_alerting_system(&monitoring_system.alerting_system).await?;

    // Demonstrate tracing system
    demonstrate_tracing_system(&monitoring_system.tracing_system).await?;

    // Demonstrate audit logging
    demonstrate_audit_logging(&monitoring_system.audit_logger).await?;

    // Demonstrate chaos testing
    demonstrate_chaos_testing(&monitoring_system.chaos_testing_system).await?;

    // Demonstrate security testing
    demonstrate_security_testing(&monitoring_system.security_testing_system).await?;

    // Demonstrate production integration
    demonstrate_production_integration(&monitoring_system.production_integration).await?;

    info!("✅ Comprehensive monitoring example completed successfully!");
    Ok(())
}

struct ComprehensiveMonitoringSystem {
    alerting_system: AlertingSystem,
    tracing_system: TracingSystem,
    audit_logger: AuditLogger,
    chaos_testing_system: ChaosTestingSystem,
    security_testing_system: SecurityTestingSystem,
    production_integration: ProductionIntegration,
}

async fn create_monitoring_system(
) -> Result<ComprehensiveMonitoringSystem, Box<dyn std::error::Error>> {
    info!("📊 Creating comprehensive monitoring system...");

    // Create alerting system
    let alerting_config = AlertingConfig::default();
    let alerting_system = AlertingSystem::new(alerting_config);

    // Create tracing system
    let tracing_config = TracingConfig::default();
    let tracing_system = TracingSystem::new(tracing_config);

    // Create audit logger
    let audit_config = AuditConfig::default();
    let audit_logger = AuditLogger::new(audit_config);

    // Create chaos testing system
    let mut chaos_config = ChaosTestingConfig::default();
    chaos_config.enabled = true; // Enable for demonstration
    let chaos_testing_system = ChaosTestingSystem::new(chaos_config);

    // Create security testing system
    let security_config = SecurityTestingConfig::default();
    let security_testing_system = SecurityTestingSystem::new(security_config);

    // Create production integration
    let production_config = ProductionConfig::default();
    let production_integration = ProductionIntegration::new(production_config).await?;

    Ok(ComprehensiveMonitoringSystem {
        alerting_system,
        tracing_system,
        audit_logger,
        chaos_testing_system,
        security_testing_system,
        production_integration,
    })
}

async fn demonstrate_alerting_system(
    alerting_system: &AlertingSystem,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔔 Demonstrating Alerting System...");

    // Create various types of alerts
    let alert_id1 = alerting_system
        .create_alert(
            "High CPU Usage".to_string(),
            "CPU usage has exceeded 80% threshold".to_string(),
            rhema_coordination::advanced_features::AlertSeverity::Warning,
            "system_monitor".to_string(),
            "performance".to_string(),
            Some(HashMap::from([
                ("cpu_percentage".to_string(), serde_json::json!(85.5)),
                ("threshold".to_string(), serde_json::json!(80.0)),
            ])),
        )
        .await?;

    let alert_id2 = alerting_system
        .create_alert(
            "Database Connection Failed".to_string(),
            "Failed to connect to primary database".to_string(),
            rhema_coordination::advanced_features::AlertSeverity::Error,
            "database_monitor".to_string(),
            "connectivity".to_string(),
            Some(HashMap::from([
                ("retry_count".to_string(), serde_json::json!(3)),
                ("error_code".to_string(), serde_json::json!("CONN_REFUSED")),
            ])),
        )
        .await?;

    // Acknowledge an alert
    alerting_system
        .acknowledge_alert(&alert_id1, "admin".to_string())
        .await?;

    // Resolve an alert
    alerting_system
        .resolve_alert(
            &alert_id2,
            "admin".to_string(),
            Some("Database connection restored".to_string()),
        )
        .await?;

    // Get alert statistics
    let stats = alerting_system.get_statistics().await;
    info!(
        "📊 Alert Statistics: {} total alerts, {} active",
        stats.total_alerts, stats.active_alerts
    );

    Ok(())
}

async fn demonstrate_tracing_system(
    tracing_system: &TracingSystem,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 Demonstrating Tracing System...");

    // Create trace context
    let context =
        rhema_coordination::advanced_features::TraceContext::new("api_service".to_string());

    // Start a trace
    let trace_id = tracing_system
        .start_trace(
            "API Request Processing".to_string(),
            Some("Processing user API request".to_string()),
            context,
            Some(HashMap::from([
                ("user_id".to_string(), serde_json::json!("user123")),
                ("endpoint".to_string(), serde_json::json!("/api/users")),
            ])),
            Some(HashMap::from([
                ("environment".to_string(), "production".to_string()),
                ("version".to_string(), "1.0.0".to_string()),
            ])),
        )
        .await?;

    // Add spans to the trace
    let span_id1 = tracing_system
        .add_span(
            &trace_id,
            "Database Query".to_string(),
            Some("Querying user data from database".to_string()),
            None,
            Some(HashMap::from([
                ("table".to_string(), serde_json::json!("users")),
                ("query_type".to_string(), serde_json::json!("SELECT")),
            ])),
            Some(HashMap::from([(
                "database".to_string(),
                "postgres".to_string(),
            )])),
        )
        .await?;

    // Add logs to span
    tracing_system
        .add_span_log(
            &trace_id,
            &span_id1,
            rhema_coordination::advanced_features::LogLevel::Info,
            "Executing database query".to_string(),
            Some(HashMap::from([(
                "sql".to_string(),
                serde_json::json!("SELECT * FROM users WHERE id = ?"),
            )])),
        )
        .await?;

    // End span
    tracing_system
        .end_span(
            &trace_id,
            &span_id1,
            rhema_coordination::advanced_features::SpanStatus::Completed,
        )
        .await?;

    // End trace
    tracing_system
        .end_trace(
            &trace_id,
            rhema_coordination::advanced_features::TraceStatus::Completed,
        )
        .await?;

    // Get trace statistics
    let stats = tracing_system.get_statistics().await;
    info!(
        "📊 Trace Statistics: {} total traces, {} active",
        stats.total_traces, stats.active_traces
    );

    Ok(())
}

async fn demonstrate_audit_logging(
    audit_logger: &AuditLogger,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("📝 Demonstrating Audit Logging...");

    // Log authentication events
    audit_logger
        .log_authentication(
            "user123".to_string(),
            "John Doe".to_string(),
            Some("admin".to_string()),
            "192.168.1.100".to_string(),
            Some("Mozilla/5.0".to_string()),
            rhema_coordination::advanced_features::AuditResult::Success,
            Some("sess_abc123".to_string()),
            Some(HashMap::from([
                ("auth_method".to_string(), serde_json::json!("password")),
                ("mfa_enabled".to_string(), serde_json::json!(true)),
            ])),
        )
        .await?;

    // Log authorization events
    audit_logger
        .log_authorization(
            "user123".to_string(),
            "John Doe".to_string(),
            Some("admin".to_string()),
            "/api/admin/users".to_string(),
            "GET".to_string(),
            rhema_coordination::advanced_features::AuditResult::Success,
            Some("req_xyz789".to_string()),
            Some(HashMap::from([
                (
                    "resource_type".to_string(),
                    serde_json::json!("user_management"),
                ),
                ("permission".to_string(), serde_json::json!("read")),
            ])),
        )
        .await?;

    // Log data access events
    audit_logger
        .log_data_access(
            "user123".to_string(),
            "John Doe".to_string(),
            Some("admin".to_string()),
            "/api/users/personal-data".to_string(),
            "READ".to_string(),
            rhema_coordination::advanced_features::DataClassification::Confidential,
            rhema_coordination::advanced_features::AuditResult::Success,
            Some("req_def456".to_string()),
            Some(HashMap::from([
                (
                    "data_type".to_string(),
                    serde_json::json!("personal_information"),
                ),
                ("records_accessed".to_string(), serde_json::json!(25)),
            ])),
        )
        .await?;

    // Get audit statistics
    let stats = audit_logger.get_statistics().await;
    info!(
        "📊 Audit Statistics: {} total entries, {} high risk",
        stats.total_entries, stats.high_risk_entries
    );

    Ok(())
}

async fn demonstrate_chaos_testing(
    chaos_system: &ChaosTestingSystem,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🎲 Demonstrating Chaos Testing...");

    // Start a network latency experiment
    let experiment_id1 = chaos_system
        .start_experiment(
            "Network Latency Test".to_string(),
            "Injecting network latency to test system resilience".to_string(),
            rhema_coordination::testing::ChaosExperimentType::NetworkLatency {
                latency_ms: 100,
                jitter_ms: 50,
            },
            vec!["api_gateway".to_string(), "database".to_string()],
            Some(HashMap::from([
                ("test_duration".to_string(), serde_json::json!(300)),
                ("monitoring_enabled".to_string(), serde_json::json!(true)),
            ])),
        )
        .await?;

    // Start a CPU stress experiment
    let experiment_id2 = chaos_system
        .start_experiment(
            "CPU Stress Test".to_string(),
            "Stressing CPU to test performance under load".to_string(),
            rhema_coordination::testing::ChaosExperimentType::CpuStress {
                cpu_percentage: 70.0,
                duration_seconds: 60,
            },
            vec!["application_server".to_string()],
            None,
        )
        .await?;

    // Wait a bit to simulate experiment running
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Stop experiments
    chaos_system.stop_experiment(&experiment_id1).await?;
    chaos_system.stop_experiment(&experiment_id2).await?;

    // Get chaos statistics
    let stats = chaos_system.get_statistics().await;
    info!(
        "📊 Chaos Statistics: {} total experiments, resilience score: {}",
        stats.total_experiments, stats.resilience_score
    );

    Ok(())
}

async fn demonstrate_security_testing(
    security_system: &SecurityTestingSystem,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔒 Demonstrating Security Testing...");

    // Start a vulnerability scan
    let test_id1 = security_system
        .start_test(
            "API Vulnerability Scan".to_string(),
            "Scanning API endpoints for security vulnerabilities".to_string(),
            rhema_coordination::testing::SecurityTestType::VulnerabilityScan {
                scan_depth: rhema_coordination::testing::ScanDepth::Standard,
            },
            vec!["api_gateway".to_string(), "user_service".to_string()],
            Some(HashMap::from([
                ("scan_type".to_string(), serde_json::json!("automated")),
                ("include_owasp_top_10".to_string(), serde_json::json!(true)),
            ])),
        )
        .await?;

    // Start a code security analysis
    let test_id2 = security_system
        .start_test(
            "Code Security Analysis".to_string(),
            "Analyzing source code for security issues".to_string(),
            rhema_coordination::testing::SecurityTestType::CodeSecurityAnalysis {
                languages: vec!["rust".to_string(), "javascript".to_string()],
            },
            vec!["src".to_string(), "frontend".to_string()],
            None,
        )
        .await?;

    // Wait a bit for tests to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Stop tests
    security_system.stop_test(&test_id1).await?;
    security_system.stop_test(&test_id2).await?;

    // Get security statistics
    let stats = security_system.get_statistics().await;
    info!(
        "📊 Security Statistics: {} total tests, {} vulnerabilities found, avg risk score: {}",
        stats.total_tests, stats.total_vulnerabilities, stats.avg_risk_score
    );

    Ok(())
}

async fn demonstrate_production_integration(
    production_integration: &ProductionIntegration,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🏭 Demonstrating Production Integration...");

    // Simulate production operations
    info!("📈 Production system is running with comprehensive monitoring");
    info!("🔧 All monitoring systems are integrated and operational");
    info!("📊 Real-time metrics are being collected");
    info!("🚨 Alerts are being generated for critical issues");
    info!("🔍 Distributed tracing is tracking requests across services");
    info!("📝 Audit logs are recording all security-relevant events");
    info!("🎲 Chaos testing is validating system resilience");
    info!("🔒 Security testing is ensuring compliance and security");

    // Simulate some time passing
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    info!("✅ Production integration demonstration completed");

    Ok(())
}
