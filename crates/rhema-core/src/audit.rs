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

use crate::{RhemaError, RhemaResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};
use uuid::Uuid;

/// Audit event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Security,
}

/// Types of audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    FileRead,
    FileWrite,
    FileDelete,
    FileCreate,
    ScopeAccess,
    ScopeModify,
    GitOperation,
    ValidationError,
    SecurityViolation,
    ConfigurationChange,
    UserAction,
    SystemEvent,
}

/// Audit event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub resource_path: Option<String>,
    pub action: String,
    pub details: HashMap<String, String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
}

impl AuditEvent {
    pub fn new(
        event_type: AuditEventType,
        severity: AuditSeverity,
        action: String,
        success: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            severity,
            user_id: None,
            session_id: None,
            resource_path: None,
            action,
            details: HashMap::new(),
            ip_address: None,
            user_agent: None,
            success,
            error_message: None,
        }
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_resource(mut self, resource_path: String) -> Self {
        self.resource_path = Some(resource_path);
        self
    }

    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }

    pub fn with_ip(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn with_error(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }
}

/// Audit logger configuration
#[derive(Debug, Clone)]
pub struct AuditLoggerConfig {
    pub enabled: bool,
    pub log_to_file: bool,
    pub log_to_stdout: bool,
    pub log_file_path: Option<String>,
    pub max_file_size: usize,
    pub retention_days: u32,
    pub include_sensitive_data: bool,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_to_file: true,
            log_to_stdout: false,
            log_file_path: None,
            max_file_size: 10 * 1024 * 1024, // 10MB
            retention_days: 90,
            include_sensitive_data: false,
        }
    }
}

/// Thread-safe audit logger
pub struct AuditLogger {
    config: AuditLoggerConfig,
    events: Arc<Mutex<Vec<AuditEvent>>>,
    file_writer: Option<Arc<Mutex<std::fs::File>>>,
}

impl AuditLogger {
    /// Create a new audit logger with the specified configuration
    pub fn new(config: AuditLoggerConfig) -> RhemaResult<Self> {
        let file_writer = if config.log_to_file {
            let log_path = config.log_file_path.clone().unwrap_or_else(|| {
                let mut path = std::env::temp_dir();
                path.push("rhema_audit.log");
                path.to_string_lossy().to_string()
            });

            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .map_err(|e| RhemaError::IoError(e))?;

            Some(Arc::new(Mutex::new(file)))
        } else {
            None
        };

        Ok(Self {
            config,
            events: Arc::new(Mutex::new(Vec::new())),
            file_writer,
        })
    }

    /// Log an audit event
    pub fn log_event(&self, event: AuditEvent) -> RhemaResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Store in memory
        {
            let mut events = self.events.lock().unwrap();
            events.push(event.clone());

            // Keep only recent events in memory
            if events.len() > 10000 {
                events.drain(0..1000);
            }
        }

        // Log to stdout if configured
        if self.config.log_to_stdout {
            self.log_to_stdout(&event);
        }

        // Log to file if configured
        if let Some(ref file_writer) = self.file_writer {
            self.log_to_file(file_writer, &event)?;
        }

        Ok(())
    }

    /// Log a file operation
    pub fn log_file_operation(
        &self,
        operation: &str,
        file_path: &Path,
        success: bool,
        error_message: Option<String>,
    ) -> RhemaResult<()> {
        let event_type = match operation {
            "read" => AuditEventType::FileRead,
            "write" => AuditEventType::FileWrite,
            "delete" => AuditEventType::FileDelete,
            "create" => AuditEventType::FileCreate,
            _ => AuditEventType::FileRead,
        };

        let severity = if success {
            AuditSeverity::Info
        } else {
            AuditSeverity::Error
        };

        let mut event = AuditEvent::new(event_type, severity, operation.to_string(), success)
            .with_resource(file_path.to_string_lossy().to_string());

        if let Some(error) = error_message {
            event = event.with_error(error);
        }

        self.log_event(event)
    }

    /// Log a scope operation
    pub fn log_scope_operation(
        &self,
        operation: &str,
        scope_path: &Path,
        success: bool,
        error_message: Option<String>,
    ) -> RhemaResult<()> {
        let event_type = match operation {
            "access" => AuditEventType::ScopeAccess,
            "modify" => AuditEventType::ScopeModify,
            _ => AuditEventType::ScopeAccess,
        };

        let severity = if success {
            AuditSeverity::Info
        } else {
            AuditSeverity::Error
        };

        let mut event = AuditEvent::new(event_type, severity, operation.to_string(), success)
            .with_resource(scope_path.to_string_lossy().to_string());

        if let Some(error) = error_message {
            event = event.with_error(error);
        }

        self.log_event(event)
    }

    /// Log a security violation
    pub fn log_security_violation(
        &self,
        action: &str,
        details: HashMap<String, String>,
        error_message: Option<String>,
    ) -> RhemaResult<()> {
        let mut event = AuditEvent::new(
            AuditEventType::SecurityViolation,
            AuditSeverity::Security,
            action.to_string(),
            false,
        );

        for (key, value) in details {
            event = event.with_detail(key, value);
        }

        if let Some(error) = error_message {
            event = event.with_error(error);
        }

        self.log_event(event)
    }

    /// Log a validation error
    pub fn log_validation_error(
        &self,
        field: &str,
        value: &str,
        error_message: &str,
    ) -> RhemaResult<()> {
        let mut details = HashMap::new();
        details.insert("field".to_string(), field.to_string());
        details.insert("value".to_string(), value.to_string());

        let event = AuditEvent::new(
            AuditEventType::ValidationError,
            AuditSeverity::Warning,
            "validation_error".to_string(),
            false,
        )
        .with_detail("field".to_string(), field.to_string())
        .with_detail("value".to_string(), value.to_string())
        .with_error(error_message.to_string());

        self.log_event(event)
    }

    /// Get recent audit events
    pub fn get_recent_events(&self, limit: usize) -> Vec<AuditEvent> {
        let events = self.events.lock().unwrap();
        let start = if events.len() > limit {
            events.len() - limit
        } else {
            0
        };
        events[start..].to_vec()
    }

    /// Get events by severity
    pub fn get_events_by_severity(&self, severity: AuditSeverity) -> Vec<AuditEvent> {
        let events = self.events.lock().unwrap();
        events
            .iter()
            .filter(|event| event.severity == severity)
            .cloned()
            .collect()
    }

    /// Get events by type
    pub fn get_events_by_type(&self, event_type: AuditEventType) -> Vec<AuditEvent> {
        let events = self.events.lock().unwrap();
        events
            .iter()
            .filter(|event| event.event_type == event_type)
            .cloned()
            .collect()
    }

    /// Clear all events from memory
    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    /// Log to stdout
    fn log_to_stdout(&self, event: &AuditEvent) {
        let log_message = format!(
            "[AUDIT] {} {} {} {}",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.severity.as_str(),
            event.event_type.as_str(),
            event.action
        );

        match event.severity {
            AuditSeverity::Info => info!("{}", log_message),
            AuditSeverity::Warning => warn!("{}", log_message),
            AuditSeverity::Error | AuditSeverity::Security => error!("{}", log_message),
        }
    }

    /// Log to file
    fn log_to_file(
        &self,
        file_writer: &Arc<Mutex<std::fs::File>>,
        event: &AuditEvent,
    ) -> RhemaResult<()> {
        let json = serde_json::to_string(event).map_err(|e| RhemaError::JsonError(e))?;

        let mut file = file_writer.lock().unwrap();
        use std::io::Write;
        writeln!(file, "{}", json).map_err(|e| RhemaError::IoError(e))?;

        Ok(())
    }
}

impl AuditSeverity {
    fn as_str(&self) -> &'static str {
        match self {
            AuditSeverity::Info => "INFO",
            AuditSeverity::Warning => "WARN",
            AuditSeverity::Error => "ERROR",
            AuditSeverity::Security => "SECURITY",
        }
    }
}

impl AuditEventType {
    fn as_str(&self) -> &'static str {
        match self {
            AuditEventType::FileRead => "FILE_READ",
            AuditEventType::FileWrite => "FILE_WRITE",
            AuditEventType::FileDelete => "FILE_DELETE",
            AuditEventType::FileCreate => "FILE_CREATE",
            AuditEventType::ScopeAccess => "SCOPE_ACCESS",
            AuditEventType::ScopeModify => "SCOPE_MODIFY",
            AuditEventType::GitOperation => "GIT_OPERATION",
            AuditEventType::ValidationError => "VALIDATION_ERROR",
            AuditEventType::SecurityViolation => "SECURITY_VIOLATION",
            AuditEventType::ConfigurationChange => "CONFIG_CHANGE",
            AuditEventType::UserAction => "USER_ACTION",
            AuditEventType::SystemEvent => "SYSTEM_EVENT",
        }
    }
}

/// Global audit logger instance
lazy_static::lazy_static! {
    static ref GLOBAL_AUDIT_LOGGER: Arc<AuditLogger> = {
        let config = AuditLoggerConfig::default();
        Arc::new(AuditLogger::new(config).unwrap_or_else(|_| {
            // Fallback to disabled logger if initialization fails
            let mut fallback_config = AuditLoggerConfig::default();
            fallback_config.enabled = false;
            AuditLogger::new(fallback_config).unwrap()
        }))
    };
}

/// Convenience functions for global audit logging
pub fn log_audit_event(event: AuditEvent) -> RhemaResult<()> {
    GLOBAL_AUDIT_LOGGER.log_event(event)
}

pub fn log_file_operation(
    operation: &str,
    file_path: &Path,
    success: bool,
    error_message: Option<String>,
) -> RhemaResult<()> {
    GLOBAL_AUDIT_LOGGER.log_file_operation(operation, file_path, success, error_message)
}

pub fn log_security_violation(
    action: &str,
    details: HashMap<String, String>,
    error_message: Option<String>,
) -> RhemaResult<()> {
    GLOBAL_AUDIT_LOGGER.log_security_violation(action, details, error_message)
}

pub fn log_validation_error(field: &str, value: &str, error_message: &str) -> RhemaResult<()> {
    GLOBAL_AUDIT_LOGGER.log_validation_error(field, value, error_message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(
            AuditEventType::FileRead,
            AuditSeverity::Info,
            "read_file".to_string(),
            true,
        );

        assert_eq!(event.event_type, AuditEventType::FileRead);
        assert_eq!(event.severity, AuditSeverity::Info);
        assert_eq!(event.action, "read_file");
        assert!(event.success);
    }

    #[test]
    fn test_audit_event_builder() {
        let event = AuditEvent::new(
            AuditEventType::FileWrite,
            AuditSeverity::Warning,
            "write_file".to_string(),
            false,
        )
        .with_user("user123".to_string())
        .with_resource("/path/to/file".to_string())
        .with_detail("size".to_string(), "1024".to_string())
        .with_error("Permission denied".to_string());

        assert_eq!(event.user_id, Some("user123".to_string()));
        assert_eq!(event.resource_path, Some("/path/to/file".to_string()));
        assert_eq!(event.details.get("size"), Some(&"1024".to_string()));
        assert_eq!(event.error_message, Some("Permission denied".to_string()));
    }

    #[test]
    fn test_audit_logger_creation() {
        let config = AuditLoggerConfig::default();
        let logger = AuditLogger::new(config).unwrap();

        assert!(logger.config.enabled);
        assert!(logger.config.log_to_file);
    }

    #[test]
    fn test_audit_logger_logging() {
        let mut config = AuditLoggerConfig::default();
        config.log_to_stdout = true;
        config.log_to_file = false;

        let logger = AuditLogger::new(config).unwrap();

        let event = AuditEvent::new(
            AuditEventType::FileRead,
            AuditSeverity::Info,
            "test_read".to_string(),
            true,
        );

        assert!(logger.log_event(event).is_ok());

        let events = logger.get_recent_events(10);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].action, "test_read");
    }

    #[test]
    fn test_file_operation_logging() {
        let mut config = AuditLoggerConfig::default();
        config.log_to_stdout = true;
        config.log_to_file = false;

        let logger = AuditLogger::new(config).unwrap();

        let file_path = Path::new("test.txt");

        // Test successful operation
        assert!(logger
            .log_file_operation("read", file_path, true, None)
            .is_ok());

        // Test failed operation
        assert!(logger
            .log_file_operation(
                "write",
                file_path,
                false,
                Some("Permission denied".to_string())
            )
            .is_ok());

        let events = logger.get_recent_events(10);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, AuditEventType::FileRead);
        assert_eq!(events[1].event_type, AuditEventType::FileWrite);
        assert_eq!(events[1].severity, AuditSeverity::Error);
    }

    #[test]
    fn test_security_violation_logging() {
        let mut config = AuditLoggerConfig::default();
        config.log_to_stdout = true;
        config.log_to_file = false;

        let logger = AuditLogger::new(config).unwrap();

        let mut details = HashMap::new();
        details.insert("attempted_path".to_string(), "/etc/passwd".to_string());
        details.insert("user_ip".to_string(), "192.168.1.1".to_string());

        assert!(logger
            .log_security_violation(
                "path_traversal_attempt",
                details,
                Some("Path traversal detected".to_string())
            )
            .is_ok());

        let events = logger.get_events_by_severity(AuditSeverity::Security);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, AuditEventType::SecurityViolation);
    }
}
