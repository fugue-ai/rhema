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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Alerting system for production coordination
pub struct AlertingSystem {
    /// Alert configuration
    config: AlertingConfig,
    /// Active alerts
    active_alerts: Arc<RwLock<HashMap<String, Alert>>>,
    /// Alert history
    alert_history: Arc<RwLock<Vec<Alert>>>,
    /// Alert handlers
    handlers: Vec<Box<dyn AlertHandler + Send + Sync>>,
    /// Alert statistics
    statistics: Arc<RwLock<AlertStatistics>>,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting system
    pub enabled: bool,
    /// Alert severity levels
    pub severity_levels: Vec<AlertSeverity>,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    /// Alert thresholds
    pub thresholds: AlertThresholds,
    /// Alert grouping enabled
    pub grouping_enabled: bool,
    /// Alert deduplication enabled
    pub deduplication_enabled: bool,
    /// Alert retention days
    pub retention_days: u32,
    /// Alert cooldown period (seconds)
    pub cooldown_seconds: u64,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Emergency,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Info => write!(f, "Info"),
            AlertSeverity::Warning => write!(f, "Warning"),
            AlertSeverity::Error => write!(f, "Error"),
            AlertSeverity::Critical => write!(f, "Critical"),
            AlertSeverity::Emergency => write!(f, "Emergency"),
        }
    }
}

/// Alert channels for notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    Email {
        recipients: Vec<String>,
        smtp_config: SmtpConfig,
    },
    Slack {
        webhook_url: String,
        channel: String,
    },
    Webhook {
        url: String,
        headers: HashMap<String, String>,
    },
    Console,
    Log,
}

/// SMTP configuration for email alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Error rate threshold
    pub error_rate_threshold: f64,
    /// Latency threshold (ms)
    pub latency_threshold_ms: u64,
    /// Memory usage threshold
    pub memory_usage_threshold: f64,
    /// CPU usage threshold
    pub cpu_usage_threshold: f64,
    /// Disk usage threshold
    pub disk_usage_threshold: f64,
    /// Network error threshold
    pub network_error_threshold: u32,
    /// Agent failure threshold
    pub agent_failure_threshold: u32,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique alert ID
    pub id: String,
    /// Alert title
    pub title: String,
    /// Alert description
    pub description: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert source
    pub source: String,
    /// Alert category
    pub category: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Alert status
    pub status: AlertStatus,
    /// Alert acknowledgment
    pub acknowledged: bool,
    /// Alert acknowledgment timestamp
    pub acknowledged_at: Option<DateTime<Utc>>,
    /// Alert acknowledgment user
    pub acknowledged_by: Option<String>,
    /// Alert resolution timestamp
    pub resolved_at: Option<DateTime<Utc>>,
    /// Alert resolution user
    pub resolved_by: Option<String>,
    /// Alert resolution notes
    pub resolution_notes: Option<String>,
}

/// Alert status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Suppressed,
}

/// Alert statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStatistics {
    /// Total alerts created
    pub total_alerts: u64,
    /// Active alerts count
    pub active_alerts: u64,
    /// Acknowledged alerts count
    pub acknowledged_alerts: u64,
    /// Resolved alerts count
    pub resolved_alerts: u64,
    /// Alerts by severity
    pub alerts_by_severity: HashMap<AlertSeverity, u64>,
    /// Alerts by category
    pub alerts_by_category: HashMap<String, u64>,
    /// Average resolution time (seconds)
    pub avg_resolution_time_seconds: f64,
    /// Last alert timestamp
    pub last_alert_timestamp: Option<DateTime<Utc>>,
}

/// Alert handler trait
pub trait AlertHandler: Send + Sync {
    /// Handle alert
    fn handle_alert(&self, alert: &Alert) -> Result<(), String>;
    /// Get handler name
    fn name(&self) -> &str;
    /// Check if handler is enabled
    fn is_enabled(&self) -> bool;
}

/// Console alert handler
pub struct ConsoleAlertHandler;

impl AlertHandler for ConsoleAlertHandler {
    fn handle_alert(&self, alert: &Alert) -> Result<(), String> {
        match alert.severity {
            AlertSeverity::Info => info!("[ALERT] {}: {}", alert.title, alert.description),
            AlertSeverity::Warning => warn!("[ALERT] {}: {}", alert.title, alert.description),
            AlertSeverity::Error | AlertSeverity::Critical | AlertSeverity::Emergency => {
                error!("[ALERT] {}: {}", alert.title, alert.description)
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// Log alert handler
pub struct LogAlertHandler {
    log_file: String,
}

impl LogAlertHandler {
    pub fn new(log_file: String) -> Self {
        Self { log_file }
    }
}

impl AlertHandler for LogAlertHandler {
    fn handle_alert(&self, alert: &Alert) -> Result<(), String> {
        // In a real implementation, this would write to a log file
        // For now, we'll use tracing
        let log_entry = format!(
            "[{}] {} - {}: {}",
            alert.timestamp.format("%Y-%m-%d %H:%M:%S"),
            alert.severity,
            alert.title,
            alert.description
        );

        match alert.severity {
            AlertSeverity::Info => info!("{}", log_entry),
            AlertSeverity::Warning => warn!("{}", log_entry),
            AlertSeverity::Error | AlertSeverity::Critical | AlertSeverity::Emergency => {
                error!("{}", log_entry)
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "log"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

impl AlertingSystem {
    /// Create new alerting system
    pub fn new(config: AlertingConfig) -> Self {
        let mut handlers: Vec<Box<dyn AlertHandler + Send + Sync>> = Vec::new();

        // Add default handlers
        handlers.push(Box::new(ConsoleAlertHandler));
        handlers.push(Box::new(LogAlertHandler::new("alerts.log".to_string())));

        Self {
            config,
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            handlers,
            statistics: Arc::new(RwLock::new(AlertStatistics::default())),
        }
    }

    /// Add alert handler
    pub fn add_handler(&mut self, handler: Box<dyn AlertHandler + Send + Sync>) {
        self.handlers.push(handler);
    }

    /// Create and send alert
    pub async fn create_alert(
        &self,
        title: String,
        description: String,
        severity: AlertSeverity,
        source: String,
        category: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Ok("alerting_disabled".to_string());
        }

        let alert_id = format!("alert_{}", chrono::Utc::now().timestamp_millis());
        let alert = Alert {
            id: alert_id.clone(),
            title,
            description,
            severity,
            source,
            category,
            timestamp: Utc::now(),
            metadata: metadata.unwrap_or_default(),
            status: AlertStatus::Active,
            acknowledged: false,
            acknowledged_at: None,
            acknowledged_by: None,
            resolved_at: None,
            resolved_by: None,
            resolution_notes: None,
        };

        // Check for deduplication
        if self.config.deduplication_enabled {
            if let Some(existing_alert) = self.find_similar_alert(&alert).await {
                info!("Alert deduplicated: {}", existing_alert.id);
                return Ok(existing_alert.id);
            }
        }

        // Store alert
        {
            let mut active_alerts = self.active_alerts.write().await;
            active_alerts.insert(alert_id.clone(), alert.clone());
        }

        // Add to history
        {
            let mut alert_history = self.alert_history.write().await;
            alert_history.push(alert.clone());
        }

        // Update statistics
        self.update_statistics(&alert).await;

        // Send to handlers
        for handler in &self.handlers {
            if handler.is_enabled() {
                if let Err(e) = handler.handle_alert(&alert) {
                    error!("Failed to send alert to handler {}: {}", handler.name(), e);
                }
            }
        }

        info!("Alert created: {} - {}", alert.severity, alert.title);
        Ok(alert_id)
    }

    /// Acknowledge alert
    pub async fn acknowledge_alert(&self, alert_id: &str, user: String) -> Result<(), String> {
        let mut active_alerts = self.active_alerts.write().await;

        if let Some(alert) = active_alerts.get_mut(alert_id) {
            alert.acknowledged = true;
            alert.acknowledged_at = Some(Utc::now());
            alert.acknowledged_by = Some(user.clone());
            alert.status = AlertStatus::Acknowledged;

            info!("Alert acknowledged: {} by {}", alert_id, user);
            Ok(())
        } else {
            Err(format!("Alert not found: {}", alert_id))
        }
    }

    /// Resolve alert
    pub async fn resolve_alert(
        &self,
        alert_id: &str,
        user: String,
        notes: Option<String>,
    ) -> Result<(), String> {
        let mut active_alerts = self.active_alerts.write().await;

        if let Some(alert) = active_alerts.get_mut(alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(Utc::now());
            alert.resolved_by = Some(user.clone());
            alert.resolution_notes = notes;

            // Move to history
            let alert = active_alerts.remove(alert_id).unwrap();
            let mut alert_history = self.alert_history.write().await;
            alert_history.push(alert);

            info!("Alert resolved: {} by {}", alert_id, user);
            Ok(())
        } else {
            Err(format!("Alert not found: {}", alert_id))
        }
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let active_alerts = self.active_alerts.read().await;
        active_alerts.values().cloned().collect()
    }

    /// Get alert by ID
    pub async fn get_alert(&self, alert_id: &str) -> Option<Alert> {
        let active_alerts = self.active_alerts.read().await;
        if let Some(alert) = active_alerts.get(alert_id) {
            return Some(alert.clone());
        }

        let alert_history = self.alert_history.read().await;
        alert_history.iter().find(|a| a.id == alert_id).cloned()
    }

    /// Get alert statistics
    pub async fn get_statistics(&self) -> AlertStatistics {
        self.statistics.read().await.clone()
    }

    /// Clean up old alerts
    pub async fn cleanup_old_alerts(&self) -> Result<usize, String> {
        let retention_days = self.config.retention_days as i64;
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days);

        let mut alert_history = self.alert_history.write().await;
        let initial_count = alert_history.len();

        alert_history.retain(|alert| alert.timestamp > cutoff_date);

        let removed_count = initial_count - alert_history.len();
        info!("Cleaned up {} old alerts", removed_count);

        Ok(removed_count)
    }

    /// Find similar alert for deduplication
    async fn find_similar_alert(&self, new_alert: &Alert) -> Option<Alert> {
        let active_alerts = self.active_alerts.read().await;

        for alert in active_alerts.values() {
            if alert.title == new_alert.title
                && alert.source == new_alert.source
                && alert.category == new_alert.category
                && alert.severity == new_alert.severity
                && alert.status == AlertStatus::Active
            {
                // Check if within cooldown period
                let time_diff = (new_alert.timestamp - alert.timestamp).num_seconds() as u64;
                if time_diff < self.config.cooldown_seconds {
                    return Some(alert.clone());
                }
            }
        }

        None
    }

    /// Update alert statistics
    async fn update_statistics(&self, alert: &Alert) {
        let mut stats = self.statistics.write().await;

        stats.total_alerts += 1;
        stats.last_alert_timestamp = Some(alert.timestamp);

        // Update severity counts
        *stats
            .alerts_by_severity
            .entry(alert.severity.clone())
            .or_insert(0) += 1;

        // Update category counts
        *stats
            .alerts_by_category
            .entry(alert.category.clone())
            .or_insert(0) += 1;

        // Update active alerts count
        if alert.status == AlertStatus::Active {
            stats.active_alerts += 1;
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            severity_levels: vec![
                AlertSeverity::Info,
                AlertSeverity::Warning,
                AlertSeverity::Error,
                AlertSeverity::Critical,
                AlertSeverity::Emergency,
            ],
            channels: vec![AlertChannel::Console],
            thresholds: AlertThresholds::default(),
            grouping_enabled: true,
            deduplication_enabled: true,
            retention_days: 30,
            cooldown_seconds: 300, // 5 minutes
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_threshold: 0.05,  // 5%
            latency_threshold_ms: 1000,  // 1 second
            memory_usage_threshold: 0.8, // 80%
            cpu_usage_threshold: 0.8,    // 80%
            disk_usage_threshold: 0.9,   // 90%
            network_error_threshold: 10,
            agent_failure_threshold: 5,
        }
    }
}

impl Default for AlertStatistics {
    fn default() -> Self {
        Self {
            total_alerts: 0,
            active_alerts: 0,
            acknowledged_alerts: 0,
            resolved_alerts: 0,
            alerts_by_severity: HashMap::new(),
            alerts_by_category: HashMap::new(),
            avg_resolution_time_seconds: 0.0,
            last_alert_timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alerting_system_creation() {
        let config = AlertingConfig::default();
        let alerting_system = AlertingSystem::new(config);

        assert!(alerting_system.config.enabled);
        assert_eq!(alerting_system.config.retention_days, 30);
    }

    #[tokio::test]
    async fn test_create_alert() {
        let config = AlertingConfig::default();
        let alerting_system = AlertingSystem::new(config);

        let alert_id = alerting_system
            .create_alert(
                "Test Alert".to_string(),
                "This is a test alert".to_string(),
                AlertSeverity::Warning,
                "test_source".to_string(),
                "test_category".to_string(),
                None,
            )
            .await
            .unwrap();

        assert!(!alert_id.is_empty());

        let active_alerts = alerting_system.get_active_alerts().await;
        assert_eq!(active_alerts.len(), 1);
        assert_eq!(active_alerts[0].title, "Test Alert");
    }

    #[tokio::test]
    async fn test_acknowledge_alert() {
        let config = AlertingConfig::default();
        let alerting_system = AlertingSystem::new(config);

        let alert_id = alerting_system
            .create_alert(
                "Test Alert".to_string(),
                "This is a test alert".to_string(),
                AlertSeverity::Warning,
                "test_source".to_string(),
                "test_category".to_string(),
                None,
            )
            .await
            .unwrap();

        alerting_system
            .acknowledge_alert(&alert_id, "test_user".to_string())
            .await
            .unwrap();

        let alert = alerting_system.get_alert(&alert_id).await.unwrap();
        assert!(alert.acknowledged);
        assert_eq!(alert.acknowledged_by, Some("test_user".to_string()));
    }

    #[tokio::test]
    async fn test_resolve_alert() {
        let config = AlertingConfig::default();
        let alerting_system = AlertingSystem::new(config);

        let alert_id = alerting_system
            .create_alert(
                "Test Alert".to_string(),
                "This is a test alert".to_string(),
                AlertSeverity::Warning,
                "test_source".to_string(),
                "test_category".to_string(),
                None,
            )
            .await
            .unwrap();

        alerting_system
            .resolve_alert(
                &alert_id,
                "test_user".to_string(),
                Some("Fixed".to_string()),
            )
            .await
            .unwrap();

        let active_alerts = alerting_system.get_active_alerts().await;
        assert_eq!(active_alerts.len(), 0);

        let alert = alerting_system.get_alert(&alert_id).await.unwrap();
        assert_eq!(alert.status, AlertStatus::Resolved);
        assert_eq!(alert.resolved_by, Some("test_user".to_string()));
    }

    #[tokio::test]
    async fn test_alert_deduplication() {
        let mut config = AlertingConfig::default();
        config.deduplication_enabled = true;
        config.cooldown_seconds = 3600; // 1 hour

        let alerting_system = AlertingSystem::new(config);

        // Create first alert
        let alert_id1 = alerting_system
            .create_alert(
                "Test Alert".to_string(),
                "This is a test alert".to_string(),
                AlertSeverity::Warning,
                "test_source".to_string(),
                "test_category".to_string(),
                None,
            )
            .await
            .unwrap();

        // Create duplicate alert
        let alert_id2 = alerting_system
            .create_alert(
                "Test Alert".to_string(),
                "This is a test alert".to_string(),
                AlertSeverity::Warning,
                "test_source".to_string(),
                "test_category".to_string(),
                None,
            )
            .await
            .unwrap();

        // Should be the same ID due to deduplication
        assert_eq!(alert_id1, alert_id2);

        let active_alerts = alerting_system.get_active_alerts().await;
        assert_eq!(active_alerts.len(), 1);
    }

    #[tokio::test]
    async fn test_alert_statistics() {
        let config = AlertingConfig::default();
        let alerting_system = AlertingSystem::new(config);

        // Create multiple alerts
        for i in 0..3 {
            alerting_system
                .create_alert(
                    format!("Test Alert {}", i),
                    format!("This is test alert {}", i),
                    AlertSeverity::Warning,
                    "test_source".to_string(),
                    "test_category".to_string(),
                    None,
                )
                .await
                .unwrap();
        }

        let stats = alerting_system.get_statistics().await;
        assert_eq!(stats.total_alerts, 3);
        assert_eq!(stats.active_alerts, 3);
        assert_eq!(stats.alerts_by_severity[&AlertSeverity::Warning], 3);
    }
}
