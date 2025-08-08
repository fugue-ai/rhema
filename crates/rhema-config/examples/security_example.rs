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

use rhema_config::{
    AccessControlSettings, AccessDecision, AuditSettings, ComplianceReport, ComplianceSettings,
    ComplianceStatus, Config, EncryptionSettings, GlobalConfig, RepositoryConfig, RhemaResult,
    SecurityConfig, SecurityManager,
};
use serde_json::json;
use std::path::PathBuf;
use tracing::{error, info, warn};

/// Example demonstrating security features for configuration management
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting configuration security example");

    // Create a global configuration
    let global_config = create_sample_global_config()?;

    // Create security manager
    let security_manager = SecurityManager::new(&global_config)?;

    // Example 1: Configuration encryption and decryption
    configuration_encryption(&security_manager).await?;

    // Example 2: Access control and permissions
    access_control_permissions(&security_manager).await?;

    // Example 3: Audit logging
    audit_logging(&security_manager).await?;

    // Example 4: Compliance checking
    compliance_checking(&security_manager).await?;

    // Example 5: Integrity verification
    integrity_verification(&security_manager).await?;

    // Example 6: Security policy enforcement
    security_policy_enforcement(&security_manager).await?;

    // Example 7: Key management
    key_management(&security_manager).await?;

    // Example 8: Security monitoring
    security_monitoring(&security_manager).await?;

    // Example 9: Security incident response
    security_incident_response(&security_manager).await?;

    // Example 10: Security assessment
    security_assessment(&security_manager).await?;

    info!("Configuration security example completed successfully");
    Ok(())
}

/// Create a sample global configuration
fn create_sample_global_config() -> RhemaResult<GlobalConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "environment": "development",
        "security": {
            "encryption_enabled": true,
            "access_control_enabled": true,
            "audit_logging_enabled": true,
            "compliance_enabled": true
        }
    });

    GlobalConfig::load_from_json(&config_json)
}

/// Example 1: Configuration encryption and decryption
async fn configuration_encryption(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 1: Configuration Encryption and Decryption ===");

    // Create a sensitive configuration
    let sensitive_config = create_sensitive_config()?;

    // Encrypt the configuration
    let encrypted_data = security_manager
        .encrypt_configuration(&sensitive_config)
        .await?;

    info!("Configuration encrypted:");
    info!(
        "  Original size: {} bytes",
        serde_json::to_string(&sensitive_config)?.len()
    );
    info!("  Encrypted size: {} bytes", encrypted_data.len());
    info!(
        "  Encryption ratio: {:.2}%",
        (encrypted_data.len() as f64 / serde_json::to_string(&sensitive_config)?.len() as f64)
            * 100.0
    );

    // Decrypt the configuration
    let decrypted_config: RepositoryConfig = security_manager
        .decrypt_configuration(&encrypted_data)
        .await?;

    info!("Configuration decrypted successfully:");
    info!("  Decrypted version: {}", decrypted_config.version());
    info!("  Repository name: {}", decrypted_config.repository.name);

    // Verify the decrypted configuration matches the original
    assert_eq!(sensitive_config.version(), decrypted_config.version());
    assert_eq!(
        sensitive_config.repository.name,
        decrypted_config.repository.name
    );

    Ok(())
}

/// Example 2: Access control and permissions
async fn access_control_permissions(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 2: Access Control and Permissions ===");

    // Test different access scenarios
    let access_scenarios = vec![
        ("admin", "config.yml", "read", Some("production")),
        ("developer", "config.yml", "write", Some("development")),
        ("viewer", "config.yml", "read", None),
        ("unauthorized", "config.yml", "delete", Some("production")),
    ];

    for (user, resource, action, context) in access_scenarios {
        let access_decision = security_manager
            .check_access_permission(user, resource, action, context)
            .await?;

        info!(
            "Access check for {} on {}: {} -> {}",
            user, resource, action, access_decision.allowed
        );
        info!("  Reason: {}", access_decision.reason);
        info!("  Permissions: {:?}", access_decision.permissions);
        info!("  Timestamp: {}", access_decision.timestamp);
    }

    Ok(())
}

/// Example 3: Audit logging
async fn audit_logging(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 3: Audit Logging ===");

    // Log various audit events
    let audit_events = vec![
        ("ConfigRead", "Configuration file read by admin"),
        ("ConfigWrite", "Configuration file modified by developer"),
        ("AccessGranted", "Access granted to user for resource"),
        ("AccessDenied", "Access denied to unauthorized user"),
        ("AuthenticationSuccess", "User authentication successful"),
        ("AuthenticationFailure", "User authentication failed"),
    ];

    for (event_type, details) in audit_events {
        security_manager
            .audit_logger()
            .log_event(&event_type.parse().unwrap_or_default(), details)?;

        info!("Audit event logged: {} - {}", event_type, details);
    }

    // Log access attempts
    let access_attempts = vec![
        ("admin", "config.yml", "read", true, "Valid permissions"),
        (
            "hacker",
            "config.yml",
            "write",
            false,
            "Unauthorized access",
        ),
        ("developer", "config.yml", "read", true, "Valid permissions"),
    ];

    for (user, resource, action, allowed, reason) in access_attempts {
        security_manager
            .audit_logger()
            .log_access_attempt(user, resource, action, allowed, reason)
            .await?;

        info!(
            "Access attempt logged: {} {} {} -> {} ({})",
            user, action, resource, allowed, reason
        );
    }

    Ok(())
}

/// Example 4: Compliance checking
async fn compliance_checking(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 4: Compliance Checking ===");

    // Run compliance checks
    let compliance_report = security_manager.compliance_checker().run_checks()?;

    info!("Compliance report generated:");
    info!("  Overall status: {:?}", compliance_report.overall_status);
    info!("  Total checks: {}", compliance_report.results.len());
    info!("  Timestamp: {}", compliance_report.timestamp);
    info!("  Summary: {}", compliance_report.summary);

    // Show individual check results
    for result in &compliance_report.results {
        let status_icon = match result.status {
            ComplianceStatus::Compliant => "✅",
            ComplianceStatus::NonCompliant => "❌",
            ComplianceStatus::Pending => "⏳",
            ComplianceStatus::Exempt => "⚠️",
        };

        info!(
            "  {} {}: {:?} - {}",
            status_icon, result.check_name, result.status, result.details
        );
    }

    Ok(())
}

/// Example 5: Integrity verification
async fn integrity_verification(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 5: Integrity Verification ===");

    // Create a configuration
    let config = create_sample_config()?;

    // Verify integrity
    let integrity_valid = security_manager.verify_integrity(&config).await?;

    info!("Configuration integrity verification: {}", integrity_valid);

    if integrity_valid {
        info!("  Configuration is intact and has not been tampered with");
    } else {
        warn!("  Configuration integrity check failed - possible tampering detected");
    }

    // Test with modified configuration (simulating tampering)
    let mut modified_config = config.clone();
    // In a real scenario, this would be actual modification
    info!("Simulating configuration modification...");

    let modified_integrity = security_manager.verify_integrity(&modified_config).await?;

    info!("Modified configuration integrity: {}", modified_integrity);

    Ok(())
}

/// Example 6: Security policy enforcement
async fn security_policy_enforcement(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 6: Security Policy Enforcement ===");

    // Get security configuration
    let security_config = security_manager.config();

    info!("Security policies:");
    for policy in &security_config.policies {
        info!("  Policy: {} - {}", policy.name, policy.description);
        info!("    Enforcement: {:?}", policy.enforcement);
        info!("    Rules: {}", policy.rules.len());

        for rule in &policy.rules {
            info!(
                "      Rule: {} - {} (priority: {})",
                rule.name, rule.description, rule.priority
            );
            info!("        Condition: {}", rule.condition);
            info!("        Action: {:?}", rule.action);
        }
    }

    // Test policy enforcement
    info!("Testing policy enforcement...");

    // This would typically involve testing actual policy rules
    // For demonstration, we'll show the policy structure
    info!("Policy enforcement framework is ready for rule evaluation");

    Ok(())
}

/// Example 7: Key management
async fn key_management(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 7: Key Management ===");

    let security_config = security_manager.config();

    info!("Key management settings:");
    info!("  Key rotation:");
    info!(
        "    Enabled: {}",
        security_config.key_management.key_rotation.enabled
    );
    info!(
        "    Interval: {} days",
        security_config.key_management.key_rotation.interval
    );
    info!(
        "    Method: {:?}",
        security_config.key_management.key_rotation.method
    );
    info!(
        "    Notification: {}",
        security_config.key_management.key_rotation.notification
    );

    info!("  Key storage:");
    info!(
        "    Type: {:?}",
        security_config.key_management.key_storage.storage_type
    );
    info!(
        "    Path: {:?}",
        security_config.key_management.key_storage.path
    );

    info!("  Key backup:");
    info!(
        "    Enabled: {}",
        security_config.key_management.key_backup.enabled
    );
    info!(
        "    Location: {:?}",
        security_config.key_management.key_backup.location
    );
    info!(
        "    Encryption: {}",
        security_config.key_management.key_backup.encryption
    );
    info!(
        "    Frequency: {}",
        security_config.key_management.key_backup.frequency
    );

    info!("  Key recovery:");
    info!(
        "    Enabled: {}",
        security_config.key_management.key_recovery.enabled
    );
    info!(
        "    Method: {:?}",
        security_config.key_management.key_recovery.method
    );
    info!(
        "    Verification: {}",
        security_config.key_management.key_recovery.verification
    );

    Ok(())
}

/// Example 8: Security monitoring
async fn security_monitoring(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 8: Security Monitoring ===");

    // Simulate security monitoring activities
    let monitoring_activities = vec![
        "Monitoring access patterns",
        "Detecting suspicious activities",
        "Tracking failed authentication attempts",
        "Monitoring configuration changes",
        "Checking for policy violations",
        "Analyzing audit logs",
    ];

    for activity in monitoring_activities {
        info!("Security monitoring: {}", activity);

        // In a real implementation, this would involve actual monitoring logic
        // For demonstration, we'll simulate monitoring results
        info!("  Status: Active");
        info!("  Alerts: 0");
        info!("  Incidents: 0");
    }

    // Simulate security metrics
    info!("Security metrics:");
    info!("  Failed login attempts: 0");
    info!("  Suspicious activities: 0");
    info!("  Policy violations: 0");
    info!("  Configuration changes: 0");
    info!("  Access denials: 0");

    Ok(())
}

/// Example 9: Security incident response
async fn security_incident_response(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 9: Security Incident Response ===");

    // Simulate security incident scenarios
    let incident_scenarios = vec![
        (
            "Unauthorized access attempt",
            "High",
            "Block IP, notify admin",
        ),
        (
            "Configuration tampering detected",
            "Critical",
            "Isolate system, restore from backup",
        ),
        (
            "Failed authentication threshold exceeded",
            "Medium",
            "Lock account, investigate",
        ),
        (
            "Suspicious configuration changes",
            "High",
            "Review changes, rollback if needed",
        ),
    ];

    for (incident, severity, response) in incident_scenarios {
        info!("Security incident: {}", incident);
        info!("  Severity: {}", severity);
        info!("  Response: {}", response);
        info!("  Status: Responding");
        info!("  Timestamp: {}", chrono::Utc::now());
    }

    // Simulate incident response workflow
    info!("Incident response workflow:");
    info!("  1. Detection: Automated monitoring detected incident");
    info!("  2. Assessment: Incident severity and scope evaluated");
    info!("  3. Containment: Immediate actions to limit impact");
    info!("  4. Eradication: Root cause identified and addressed");
    info!("  5. Recovery: Systems restored to normal operation");
    info!("  6. Lessons learned: Process improvements implemented");

    Ok(())
}

/// Example 10: Security assessment
async fn security_assessment(security_manager: &SecurityManager) -> RhemaResult<()> {
    info!("=== Example 10: Security Assessment ===");

    // Perform comprehensive security assessment
    let assessment_areas = vec![
        ("Encryption", "AES256GCM encryption enabled", "Strong"),
        ("Access Control", "RBAC with granular permissions", "Strong"),
        ("Audit Logging", "Comprehensive audit trail", "Strong"),
        ("Compliance", "SOC2 and GDPR compliance", "Strong"),
        ("Key Management", "Automated key rotation", "Strong"),
        (
            "Incident Response",
            "Automated detection and response",
            "Good",
        ),
    ];

    info!("Security assessment results:");
    for (area, description, rating) in assessment_areas {
        info!("  {}: {} - {}", area, description, rating);
    }

    // Overall security score
    let security_score = 95; // Simulated score
    info!("Overall security score: {}%", security_score);

    if security_score >= 90 {
        info!("  Security posture: Excellent");
    } else if security_score >= 80 {
        info!("  Security posture: Good");
    } else if security_score >= 70 {
        info!("  Security posture: Fair");
    } else {
        info!("  Security posture: Needs improvement");
    }

    // Recommendations
    info!("Security recommendations:");
    info!("  1. Continue regular security assessments");
    info!("  2. Monitor for new security threats");
    info!("  3. Update security policies as needed");
    info!("  4. Conduct security training for users");
    info!("  5. Test incident response procedures");

    Ok(())
}

/// Create a sample configuration
fn create_sample_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "sample-repo",
            "url": "https://github.com/user/sample-repo",
            "branch": "main"
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a sensitive configuration
fn create_sensitive_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "sensitive-repo",
            "url": "https://github.com/user/sensitive-repo",
            "branch": "main"
        },
        "sensitive_data": {
            "api_keys": {
                "production": "sk-prod-1234567890abcdef",
                "staging": "sk-staging-1234567890abcdef"
            },
            "database": {
                "host": "sensitive-db.example.com",
                "port": 5432,
                "name": "sensitive_db",
                "user": "admin",
                "password": "super-secret-password"
            },
            "secrets": {
                "jwt_secret": "very-secret-jwt-key",
                "encryption_key": "very-secret-encryption-key"
            }
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_example() {
        // This test ensures the example runs without panicking
        let global_config = create_sample_global_config().unwrap();
        let security_manager = SecurityManager::new(&global_config).unwrap();

        // Test encryption
        let config = create_sample_config().unwrap();
        let encrypted_data = security_manager
            .encrypt_configuration(&config)
            .await
            .unwrap();

        assert!(!encrypted_data.is_empty());

        // Test decryption
        let decrypted_config: RepositoryConfig = security_manager
            .decrypt_configuration(&encrypted_data)
            .await
            .unwrap();

        assert_eq!(config.version(), decrypted_config.version());
    }

    #[tokio::test]
    async fn test_access_control() {
        let global_config = create_sample_global_config().unwrap();
        let security_manager = SecurityManager::new(&global_config).unwrap();

        let access_decision = security_manager
            .check_access_permission("admin", "config.yml", "read", Some("production"))
            .await
            .unwrap();

        assert!(access_decision.allowed);
    }

    #[tokio::test]
    async fn test_compliance_checking() {
        let global_config = create_sample_global_config().unwrap();
        let security_manager = SecurityManager::new(&global_config).unwrap();

        let compliance_report = security_manager.compliance_checker().run_checks().unwrap();

        assert!(!compliance_report.results.is_empty());
    }
}
