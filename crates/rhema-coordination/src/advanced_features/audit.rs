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
use uuid::Uuid;

/// Enhanced audit logging system for production coordination
pub struct AuditLogger {
    /// Audit configuration
    config: AuditConfig,
    /// Audit entries
    audit_entries: Arc<RwLock<Vec<AuditEntry>>>,
    /// Audit filters
    filters: Vec<Box<dyn AuditFilter + Send + Sync>>,
    /// Audit exporters
    exporters: Vec<Box<dyn AuditExporter + Send + Sync>>,
    /// Audit statistics
    statistics: Arc<RwLock<AuditStatistics>>,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit level
    pub level: AuditLevel,
    /// Audit retention days
    pub retention_days: u32,
    /// Enable real-time monitoring
    pub real_time_monitoring: bool,
    /// Enable compliance reporting
    pub compliance_reporting: bool,
    /// Enable data masking
    pub data_masking: bool,
    /// Sensitive fields to mask
    pub sensitive_fields: Vec<String>,
    /// Export formats
    pub export_formats: Vec<AuditExportFormat>,
    /// Compliance standards
    pub compliance_standards: Vec<ComplianceStandard>,
}

/// Audit levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AuditLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Audit export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditExportFormat {
    Json,
    Csv,
    Xml,
    Syslog,
    Custom(String),
}

/// Compliance standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStandard {
    SOX,
    PCI,
    HIPAA,
    GDPR,
    ISO27001,
    Custom(String),
}

/// Audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique audit ID
    pub id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Audit level
    pub level: AuditLevel,
    /// Event type
    pub event_type: String,
    /// Event description
    pub description: String,
    /// User ID
    pub user_id: Option<String>,
    /// User name
    pub user_name: Option<String>,
    /// User role
    pub user_role: Option<String>,
    /// Source IP address
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Resource accessed
    pub resource: Option<String>,
    /// Action performed
    pub action: Option<String>,
    /// Action result
    pub result: AuditResult,
    /// Request ID
    pub request_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Correlation ID
    pub correlation_id: Option<String>,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Compliance tags
    pub compliance_tags: Vec<String>,
    /// Risk score
    pub risk_score: Option<f64>,
    /// Data classification
    pub data_classification: Option<DataClassification>,
}

/// Audit result
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
    Denied,
    Timeout,
}

impl std::fmt::Display for AuditResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditResult::Success => write!(f, "Success"),
            AuditResult::Failure => write!(f, "Failure"),
            AuditResult::Partial => write!(f, "Partial"),
            AuditResult::Denied => write!(f, "Denied"),
            AuditResult::Timeout => write!(f, "Timeout"),
        }
    }
}

/// Data classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Secret,
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    /// Total audit entries
    pub total_entries: u64,
    /// Entries by level
    pub entries_by_level: HashMap<AuditLevel, u64>,
    /// Entries by event type
    pub entries_by_event_type: HashMap<String, u64>,
    /// Entries by user
    pub entries_by_user: HashMap<String, u64>,
    /// Entries by result
    pub entries_by_result: HashMap<AuditResult, u64>,
    /// High risk entries count
    pub high_risk_entries: u64,
    /// Compliance violations count
    pub compliance_violations: u64,
    /// Last audit entry timestamp
    pub last_entry_timestamp: Option<DateTime<Utc>>,
}

/// Audit filter trait
pub trait AuditFilter: Send + Sync {
    /// Check if entry should be filtered
    fn should_filter(&self, entry: &AuditEntry) -> bool;
    /// Get filter name
    fn name(&self) -> &str;
    /// Check if filter is enabled
    fn is_enabled(&self) -> bool;
}

/// Level-based audit filter
pub struct LevelAuditFilter {
    min_level: AuditLevel,
}

impl LevelAuditFilter {
    pub fn new(min_level: AuditLevel) -> Self {
        Self { min_level }
    }
}

impl AuditFilter for LevelAuditFilter {
    fn should_filter(&self, entry: &AuditEntry) -> bool {
        entry.level < self.min_level
    }

    fn name(&self) -> &str {
        "level"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// User-based audit filter
pub struct UserAuditFilter {
    excluded_users: Vec<String>,
}

impl UserAuditFilter {
    pub fn new(excluded_users: Vec<String>) -> Self {
        Self { excluded_users }
    }
}

impl AuditFilter for UserAuditFilter {
    fn should_filter(&self, entry: &AuditEntry) -> bool {
        if let Some(user_id) = &entry.user_id {
            self.excluded_users.contains(user_id)
        } else {
            false
        }
    }

    fn name(&self) -> &str {
        "user"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// Audit exporter trait
pub trait AuditExporter: Send + Sync {
    /// Export audit entry
    fn export_entry(&self, entry: &AuditEntry) -> Result<(), String>;
    /// Get exporter name
    fn name(&self) -> &str;
    /// Check if exporter is enabled
    fn is_enabled(&self) -> bool;
}

/// Console audit exporter
pub struct ConsoleAuditExporter;

impl AuditExporter for ConsoleAuditExporter {
    fn export_entry(&self, entry: &AuditEntry) -> Result<(), String> {
        match entry.level {
            AuditLevel::Debug => info!("[AUDIT] {}: {}", entry.event_type, entry.description),
            AuditLevel::Info => info!("[AUDIT] {}: {}", entry.event_type, entry.description),
            AuditLevel::Warning => warn!("[AUDIT] {}: {}", entry.event_type, entry.description),
            AuditLevel::Error | AuditLevel::Critical => {
                error!("[AUDIT] {}: {}", entry.event_type, entry.description)
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

/// JSON audit exporter
pub struct JsonAuditExporter {
    output_file: Option<String>,
}

impl JsonAuditExporter {
    pub fn new(output_file: Option<String>) -> Self {
        Self { output_file }
    }
}

impl AuditExporter for JsonAuditExporter {
    fn export_entry(&self, entry: &AuditEntry) -> Result<(), String> {
        let json = serde_json::to_string_pretty(entry)
            .map_err(|e| format!("Failed to serialize audit entry: {}", e))?;

        if let Some(file_path) = &self.output_file {
            // In a real implementation, this would write to a file
            // For now, we'll just log it
            info!("[JSON_AUDIT] Writing to {}: {}", file_path, json);
        } else {
            info!("[JSON_AUDIT] {}", json);
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "json"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

impl AuditLogger {
    /// Create new audit logger
    pub fn new(config: AuditConfig) -> Self {
        let mut filters: Vec<Box<dyn AuditFilter + Send + Sync>> = Vec::new();
        let mut exporters: Vec<Box<dyn AuditExporter + Send + Sync>> = Vec::new();

        // Add default filters
        filters.push(Box::new(LevelAuditFilter::new(AuditLevel::Info)));

        // Add default exporters
        exporters.push(Box::new(ConsoleAuditExporter));
        exporters.push(Box::new(JsonAuditExporter::new(None)));

        Self {
            config,
            audit_entries: Arc::new(RwLock::new(Vec::new())),
            filters,
            exporters,
            statistics: Arc::new(RwLock::new(AuditStatistics::default())),
        }
    }

    /// Add audit filter
    pub fn add_filter(&mut self, filter: Box<dyn AuditFilter + Send + Sync>) {
        self.filters.push(filter);
    }

    /// Add audit exporter
    pub fn add_exporter(&mut self, exporter: Box<dyn AuditExporter + Send + Sync>) {
        self.exporters.push(exporter);
    }

    /// Log audit entry
    pub async fn log(
        &self,
        level: AuditLevel,
        event_type: String,
        description: String,
        user_id: Option<String>,
        user_name: Option<String>,
        user_role: Option<String>,
        source_ip: Option<String>,
        user_agent: Option<String>,
        resource: Option<String>,
        action: Option<String>,
        result: AuditResult,
        request_id: Option<String>,
        session_id: Option<String>,
        correlation_id: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
        compliance_tags: Option<Vec<String>>,
        risk_score: Option<f64>,
        data_classification: Option<DataClassification>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Ok("audit_disabled".to_string());
        }

        let entry_id = Uuid::new_v4().to_string();
        let entry = AuditEntry {
            id: entry_id.clone(),
            timestamp: Utc::now(),
            level,
            event_type,
            description,
            user_id,
            user_name,
            user_role,
            source_ip,
            user_agent,
            resource,
            action,
            result,
            request_id,
            session_id,
            correlation_id,
            metadata: metadata.unwrap_or_default(),
            compliance_tags: compliance_tags.unwrap_or_default(),
            risk_score,
            data_classification,
        };

        // Apply data masking
        let masked_entry = if self.config.data_masking {
            self.mask_sensitive_data(entry)
        } else {
            entry
        };

        // Apply filters
        for filter in &self.filters {
            if filter.is_enabled() && filter.should_filter(&masked_entry) {
                info!(
                    "Audit entry filtered by {}: {}",
                    filter.name(),
                    masked_entry.id
                );
                return Ok("filtered".to_string());
            }
        }

        // Store entry
        {
            let mut audit_entries = self.audit_entries.write().await;
            audit_entries.push(masked_entry.clone());
        }

        // Update statistics
        self.update_statistics(&masked_entry).await;

        // Export entry
        for exporter in &self.exporters {
            if exporter.is_enabled() {
                if let Err(e) = exporter.export_entry(&masked_entry) {
                    error!("Failed to export audit entry to {}: {}", exporter.name(), e);
                }
            }
        }

        // Check for compliance violations
        if self.config.compliance_reporting {
            self.check_compliance_violations(&masked_entry).await;
        }

        // Real-time monitoring
        if self.config.real_time_monitoring {
            self.monitor_real_time(&masked_entry).await;
        }

        info!(
            "Audit entry logged: {:?} - {} ({})",
            masked_entry.level, masked_entry.event_type, entry_id
        );
        Ok(entry_id)
    }

    /// Log authentication event
    pub async fn log_authentication(
        &self,
        user_id: String,
        user_name: String,
        user_role: Option<String>,
        source_ip: String,
        user_agent: Option<String>,
        result: AuditResult,
        session_id: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        let event_type = match result {
            AuditResult::Success => "authentication_success".to_string(),
            AuditResult::Failure => "authentication_failure".to_string(),
            AuditResult::Denied => "authentication_denied".to_string(),
            _ => "authentication_attempt".to_string(),
        };

        let description = format!("User {} authentication {}", user_name, result);

        let level = match result {
            AuditResult::Success => AuditLevel::Info,
            AuditResult::Failure => AuditLevel::Warning,
            AuditResult::Denied => AuditLevel::Error,
            _ => AuditLevel::Info,
        };

        self.log(
            level,
            event_type,
            description,
            Some(user_id),
            Some(user_name),
            user_role,
            Some(source_ip),
            user_agent,
            None,
            Some("authenticate".to_string()),
            result,
            None,
            session_id,
            None,
            metadata,
            Some(vec!["authentication".to_string()]),
            None,
            None,
        )
        .await
    }

    /// Log authorization event
    pub async fn log_authorization(
        &self,
        user_id: String,
        user_name: String,
        user_role: Option<String>,
        resource: String,
        action: String,
        result: AuditResult,
        request_id: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        let event_type = match result {
            AuditResult::Success => "authorization_success".to_string(),
            AuditResult::Failure => "authorization_failure".to_string(),
            AuditResult::Denied => "authorization_denied".to_string(),
            _ => "authorization_attempt".to_string(),
        };

        let description = format!("User {} {} access to {}", user_name, result, resource);

        let level = match result {
            AuditResult::Success => AuditLevel::Info,
            AuditResult::Failure => AuditLevel::Warning,
            AuditResult::Denied => AuditLevel::Error,
            _ => AuditLevel::Info,
        };

        self.log(
            level,
            event_type,
            description,
            Some(user_id),
            Some(user_name),
            user_role,
            None,
            None,
            Some(resource),
            Some(action),
            result,
            request_id,
            None,
            None,
            metadata,
            Some(vec!["authorization".to_string()]),
            None,
            None,
        )
        .await
    }

    /// Log data access event
    pub async fn log_data_access(
        &self,
        user_id: String,
        user_name: String,
        user_role: Option<String>,
        resource: String,
        action: String,
        data_classification: DataClassification,
        result: AuditResult,
        request_id: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        let event_type = "data_access".to_string();
        let description = format!("User {} {} data from {}", user_name, action, resource);

        let level = match data_classification {
            DataClassification::Public => AuditLevel::Debug,
            DataClassification::Internal => AuditLevel::Info,
            DataClassification::Confidential => AuditLevel::Warning,
            DataClassification::Restricted | DataClassification::Secret => AuditLevel::Error,
        };

        self.log(
            level,
            event_type,
            description,
            Some(user_id),
            Some(user_name),
            user_role,
            None,
            None,
            Some(resource),
            Some(action),
            result,
            request_id,
            None,
            None,
            metadata,
            Some(vec!["data_access".to_string()]),
            None,
            Some(data_classification),
        )
        .await
    }

    /// Get audit entries
    pub async fn get_entries(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
        filters: Option<AuditFilters>,
    ) -> Vec<AuditEntry> {
        let audit_entries = self.audit_entries.read().await;
        let mut entries: Vec<AuditEntry> = audit_entries.clone();

        // Apply filters
        if let Some(filters) = filters {
            entries = self.apply_filters(entries, filters);
        }

        // Apply pagination
        if let Some(offset) = offset {
            entries = entries.into_iter().skip(offset).collect();
        }

        if let Some(limit) = limit {
            entries = entries.into_iter().take(limit).collect();
        }

        entries
    }

    /// Get audit statistics
    pub async fn get_statistics(&self) -> AuditStatistics {
        self.statistics.read().await.clone()
    }

    /// Clean up old audit entries
    pub async fn cleanup_old_entries(&self) -> Result<usize, String> {
        let retention_days = self.config.retention_days as i64;
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days);

        let mut audit_entries = self.audit_entries.write().await;
        let initial_count = audit_entries.len();

        audit_entries.retain(|entry| entry.timestamp > cutoff_date);

        let removed_count = initial_count - audit_entries.len();
        info!("Cleaned up {} old audit entries", removed_count);

        Ok(removed_count)
    }

    /// Mask sensitive data
    fn mask_sensitive_data(&self, mut entry: AuditEntry) -> AuditEntry {
        for field in &self.config.sensitive_fields {
            if let Some(value) = entry.metadata.get_mut(field) {
                *value = serde_json::Value::String("***MASKED***".to_string());
            }
        }
        entry
    }

    /// Check compliance violations
    async fn check_compliance_violations(&self, entry: &AuditEntry) {
        // Check for high-risk activities
        if let Some(risk_score) = entry.risk_score {
            if risk_score > 0.8 {
                warn!(
                    "High-risk audit entry detected: {} (score: {})",
                    entry.id, risk_score
                );
            }
        }

        // Check for compliance violations
        for standard in &self.config.compliance_standards {
            match standard {
                ComplianceStandard::SOX => {
                    if entry.event_type.contains("financial")
                        && entry.result == AuditResult::Failure
                    {
                        error!("SOX compliance violation detected: {}", entry.id);
                    }
                }
                ComplianceStandard::PCI => {
                    if entry.event_type.contains("payment") && entry.result == AuditResult::Failure
                    {
                        error!("PCI compliance violation detected: {}", entry.id);
                    }
                }
                ComplianceStandard::HIPAA => {
                    if entry.event_type.contains("health") && entry.result == AuditResult::Failure {
                        error!("HIPAA compliance violation detected: {}", entry.id);
                    }
                }
                ComplianceStandard::GDPR => {
                    if entry.event_type.contains("personal_data")
                        && entry.result == AuditResult::Failure
                    {
                        error!("GDPR compliance violation detected: {}", entry.id);
                    }
                }
                _ => {}
            }
        }
    }

    /// Monitor real-time
    async fn monitor_real_time(&self, entry: &AuditEntry) {
        // Check for suspicious patterns
        if entry.level >= AuditLevel::Error {
            warn!("Real-time alert: High severity audit entry: {}", entry.id);
        }

        // Check for rapid failures
        // This would typically involve more sophisticated pattern detection
        if entry.result == AuditResult::Failure {
            info!("Real-time monitoring: Failure detected: {}", entry.id);
        }
    }

    /// Apply filters to entries
    fn apply_filters(&self, entries: Vec<AuditEntry>, filters: AuditFilters) -> Vec<AuditEntry> {
        entries
            .into_iter()
            .filter(|entry| {
                if let Some(min_level) = &filters.min_level {
                    if entry.level < *min_level {
                        return false;
                    }
                }

                if let Some(event_types) = &filters.event_types {
                    if !event_types.contains(&entry.event_type) {
                        return false;
                    }
                }

                if let Some(user_ids) = &filters.user_ids {
                    if let Some(user_id) = &entry.user_id {
                        if !user_ids.contains(user_id) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if let Some(results) = &filters.results {
                    if !results.contains(&entry.result) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    /// Update audit statistics
    async fn update_statistics(&self, entry: &AuditEntry) {
        let mut stats = self.statistics.write().await;

        stats.total_entries += 1;
        stats.last_entry_timestamp = Some(entry.timestamp);

        // Update level counts
        *stats
            .entries_by_level
            .entry(entry.level.clone())
            .or_insert(0) += 1;

        // Update event type counts
        *stats
            .entries_by_event_type
            .entry(entry.event_type.clone())
            .or_insert(0) += 1;

        // Update user counts
        if let Some(user_id) = &entry.user_id {
            *stats.entries_by_user.entry(user_id.clone()).or_insert(0) += 1;
        }

        // Update result counts
        *stats
            .entries_by_result
            .entry(entry.result.clone())
            .or_insert(0) += 1;

        // Update high risk count
        if let Some(risk_score) = entry.risk_score {
            if risk_score > 0.8 {
                stats.high_risk_entries += 1;
            }
        }
    }
}

/// Audit filters for querying
#[derive(Debug, Clone)]
pub struct AuditFilters {
    pub min_level: Option<AuditLevel>,
    pub event_types: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
    pub results: Option<Vec<AuditResult>>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: AuditLevel::Info,
            retention_days: 90,
            real_time_monitoring: true,
            compliance_reporting: true,
            data_masking: true,
            sensitive_fields: vec![
                "password".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "key".to_string(),
            ],
            export_formats: vec![AuditExportFormat::Json],
            compliance_standards: vec![
                ComplianceStandard::SOX,
                ComplianceStandard::PCI,
                ComplianceStandard::GDPR,
            ],
        }
    }
}

impl Default for AuditStatistics {
    fn default() -> Self {
        Self {
            total_entries: 0,
            entries_by_level: HashMap::new(),
            entries_by_event_type: HashMap::new(),
            entries_by_user: HashMap::new(),
            entries_by_result: HashMap::new(),
            high_risk_entries: 0,
            compliance_violations: 0,
            last_entry_timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_logger_creation() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        assert!(audit_logger.config.enabled);
        assert_eq!(audit_logger.config.level, AuditLevel::Info);
    }

    #[tokio::test]
    async fn test_log_audit_entry() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        let entry_id = audit_logger
            .log(
                AuditLevel::Info,
                "test_event".to_string(),
                "This is a test audit entry".to_string(),
                Some("user123".to_string()),
                Some("Test User".to_string()),
                Some("admin".to_string()),
                Some("192.168.1.1".to_string()),
                Some("Mozilla/5.0".to_string()),
                Some("/api/test".to_string()),
                Some("GET".to_string()),
                AuditResult::Success,
                Some("req123".to_string()),
                Some("sess123".to_string()),
                Some("corr123".to_string()),
                None,
                Some(vec!["test".to_string()]),
                Some(0.1),
                Some(DataClassification::Internal),
            )
            .await
            .unwrap();

        assert!(!entry_id.is_empty());

        let entries = audit_logger.get_entries(None, None, None).await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].event_type, "test_event");
    }

    #[tokio::test]
    async fn test_log_authentication() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        let entry_id = audit_logger
            .log_authentication(
                "user123".to_string(),
                "Test User".to_string(),
                Some("admin".to_string()),
                "192.168.1.1".to_string(),
                Some("Mozilla/5.0".to_string()),
                AuditResult::Success,
                Some("sess123".to_string()),
                None,
            )
            .await
            .unwrap();

        assert!(!entry_id.is_empty());

        let entries = audit_logger.get_entries(None, None, None).await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].event_type, "authentication_success");
    }

    #[tokio::test]
    async fn test_log_authorization() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        let entry_id = audit_logger
            .log_authorization(
                "user123".to_string(),
                "Test User".to_string(),
                Some("admin".to_string()),
                "/api/admin".to_string(),
                "POST".to_string(),
                AuditResult::Success,
                Some("req123".to_string()),
                None,
            )
            .await
            .unwrap();

        assert!(!entry_id.is_empty());

        let entries = audit_logger.get_entries(None, None, None).await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].event_type, "authorization_success");
    }

    #[tokio::test]
    async fn test_audit_filters() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        // Log multiple entries
        for i in 0..5 {
            audit_logger
                .log(
                    if i % 2 == 0 {
                        AuditLevel::Info
                    } else {
                        AuditLevel::Error
                    },
                    format!("event_{}", i),
                    format!("Test entry {}", i),
                    Some(format!("user{}", i)),
                    Some(format!("User {}", i)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    AuditResult::Success,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .unwrap();
        }

        // Apply filters
        let filters = AuditFilters {
            min_level: Some(AuditLevel::Error),
            event_types: None,
            user_ids: None,
            results: None,
        };

        let entries = audit_logger.get_entries(None, None, Some(filters)).await;
        assert_eq!(entries.len(), 2); // Only Error level entries
    }

    #[tokio::test]
    async fn test_audit_statistics() {
        let config = AuditConfig::default();
        let audit_logger = AuditLogger::new(config);

        // Log multiple entries
        for i in 0..3 {
            audit_logger
                .log(
                    AuditLevel::Info,
                    "test_event".to_string(),
                    format!("Test entry {}", i),
                    Some("user123".to_string()),
                    Some("Test User".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    AuditResult::Success,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .unwrap();
        }

        let stats = audit_logger.get_statistics().await;
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.entries_by_level[&AuditLevel::Info], 3);
        assert_eq!(stats.entries_by_user["user123"], 3);
    }
}
