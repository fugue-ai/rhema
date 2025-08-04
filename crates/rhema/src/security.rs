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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, instrument};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SQL_INJECTION_PATTERN: Regex = Regex::new(r#"(?i)(SELECT|INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|EXEC|UNION|SCRIPT)"#).unwrap();
    static ref XSS_PATTERN: Regex = Regex::new(r#"(?i)(<script|javascript:|vbscript:|onload=|onerror=|onclick=)"#).unwrap();
    static ref PATH_TRAVERSAL_PATTERN: Regex = Regex::new(r#"(\.\./|\.\.\\)"#).unwrap();
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable input sanitization
    pub enable_input_sanitization: bool,
    
    /// Enable access control
    pub enable_access_control: bool,
    
    /// Enable audit logging
    pub enable_audit_logging: bool,
    
    /// Allowed file extensions
    pub allowed_file_extensions: Vec<String>,
    
    /// Maximum file size in bytes
    pub max_file_size: usize,
    
    /// Allowed query patterns
    pub allowed_query_patterns: Vec<String>,
    
    /// Blocked query patterns
    pub blocked_query_patterns: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_input_sanitization: true,
            enable_access_control: true,
            enable_audit_logging: true,
            allowed_file_extensions: vec![
                "yaml".to_string(), "yml".to_string(), "json".to_string(),
                "md".to_string(), "txt".to_string()
            ],
            max_file_size: 10 * 1024 * 1024, // 10 MB
            allowed_query_patterns: vec![
                r"^SELECT\s+.*FROM\s+.*$".to_string(),
                r"^SELECT\s+.*WHERE\s+.*$".to_string(),
            ],
            blocked_query_patterns: vec![
                r"(?i)DELETE\s+FROM".to_string(),
                r"(?i)DROP\s+TABLE".to_string(),
                r"(?i)INSERT\s+INTO".to_string(),
                r"(?i)UPDATE\s+.*SET".to_string(),
            ],
        }
    }
}

/// Input sanitizer for security
#[derive(Debug, Clone)]
pub struct InputSanitizer {
    config: SecurityConfig,
}

impl InputSanitizer {
    /// Create a new input sanitizer
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// Sanitize a string input
    #[instrument(skip_all)]
    pub fn sanitize_string(&self, input: &str) -> RhemaResult<String> {
        if !self.config.enable_input_sanitization {
            return Ok(input.to_string());
        }

        let mut sanitized = input.to_string();

        // Remove SQL injection patterns
        if SQL_INJECTION_PATTERN.is_match(&sanitized) {
            return Err(RhemaError::SecurityError(
                "Potential SQL injection detected".to_string()
            ));
        }

        // Remove XSS patterns
        if XSS_PATTERN.is_match(&sanitized) {
            return Err(RhemaError::SecurityError(
                "Potential XSS attack detected".to_string()
            ));
        }

        // Remove path traversal patterns
        if PATH_TRAVERSAL_PATTERN.is_match(&sanitized) {
            return Err(RhemaError::SecurityError(
                "Path traversal attack detected".to_string()
            ));
        }

        // HTML encode special characters
        sanitized = sanitized
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;");

        Ok(sanitized)
    }

    /// Sanitize a file path
    #[instrument(skip_all)]
    pub fn sanitize_file_path(&self, path: &str) -> RhemaResult<String> {
        let sanitized = self.sanitize_string(path)?;

        // Check for path traversal
        if sanitized.contains("..") {
            return Err(RhemaError::SecurityError(
                "Path traversal not allowed".to_string()
            ));
        }

        // Check file extension
        if let Some(extension) = std::path::Path::new(&sanitized).extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            if !self.config.allowed_file_extensions.contains(&ext.to_string()) {
                return Err(RhemaError::SecurityError(
                    format!("File extension '{}' not allowed", ext)
                ));
            }
        }

        Ok(sanitized)
    }

    /// Sanitize a query
    #[instrument(skip_all)]
    pub fn sanitize_query(&self, query: &str) -> RhemaResult<String> {
        let sanitized = self.sanitize_string(query)?;

        // Check blocked patterns
        for pattern in &self.config.blocked_query_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&sanitized) {
                    return Err(RhemaError::SecurityError(
                        format!("Query pattern '{}' is blocked", pattern)
                    ));
                }
            }
        }

        Ok(sanitized)
    }
}

/// Access control for operations
#[derive(Debug, Clone)]
pub struct AccessControl {
    config: SecurityConfig,
    permissions: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl AccessControl {
    /// Create a new access control instance
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if user has permission for operation
    #[instrument(skip_all)]
    pub async fn check_permission(&self, user_id: &str, operation: &str) -> RhemaResult<bool> {
        if !self.config.enable_access_control {
            return Ok(true);
        }

        let permissions = self.permissions.read().await;
        if let Some(user_permissions) = permissions.get(user_id) {
            Ok(user_permissions.contains(&operation.to_string()))
        } else {
            Ok(false)
        }
    }

    /// Grant permission to user
    #[instrument(skip_all)]
    pub async fn grant_permission(&self, user_id: &str, operation: &str) -> RhemaResult<()> {
        let mut permissions = self.permissions.write().await;
        let user_permissions = permissions
            .entry(user_id.to_string())
            .or_insert_with(Vec::new);
        
        if !user_permissions.contains(&operation.to_string()) {
            user_permissions.push(operation.to_string());
        }

        info!("Granted permission '{}' to user '{}'", operation, user_id);
        Ok(())
    }

    /// Revoke permission from user
    #[instrument(skip_all)]
    pub async fn revoke_permission(&self, user_id: &str, operation: &str) -> RhemaResult<()> {
        let mut permissions = self.permissions.write().await;
        if let Some(user_permissions) = permissions.get_mut(user_id) {
            user_permissions.retain(|p| p != operation);
        }

        info!("Revoked permission '{}' from user '{}'", operation, user_id);
        Ok(())
    }
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// User ID
    pub user_id: String,
    
    /// Operation performed
    pub operation: String,
    
    /// Resource accessed
    pub resource: String,
    
    /// Success status
    pub success: bool,
    
    /// Error message if failed
    pub error_message: Option<String>,
    
    /// IP address
    pub ip_address: Option<String>,
    
    /// User agent
    pub user_agent: Option<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_yaml::Value>,
}

/// Audit logger for security events
#[derive(Debug, Clone)]
pub struct AuditLogger {
    config: SecurityConfig,
    log_entries: Arc<RwLock<Vec<AuditLogEntry>>>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            log_entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log an audit event
    #[instrument(skip_all)]
    pub async fn log_event(&self, entry: AuditLogEntry) -> RhemaResult<()> {
        if !self.config.enable_audit_logging {
            return Ok(());
        }

        let mut entries = self.log_entries.write().await;
        entries.push(entry.clone());

        // Keep only the last 10000 entries
        if entries.len() > 10000 {
            entries.remove(0);
        }

        info!("Audit log: {} performed {} on {} (success: {})", 
            entry.user_id, entry.operation, entry.resource, entry.success);

        Ok(())
    }

    /// Get audit log entries
    #[instrument(skip_all)]
    pub async fn get_entries(&self, user_id: Option<&str>, operation: Option<&str>) -> Vec<AuditLogEntry> {
        let entries = self.log_entries.read().await;
        
        entries.iter()
            .filter(|entry| {
                if let Some(uid) = user_id {
                    if entry.user_id != uid {
                        return false;
                    }
                }
                if let Some(op) = operation {
                    if entry.operation != op {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    /// Clear audit log
    #[instrument(skip_all)]
    pub async fn clear_log(&self) -> RhemaResult<()> {
        let mut entries = self.log_entries.write().await;
        entries.clear();
        info!("Audit log cleared");
        Ok(())
    }
}

/// Security manager that combines all security features
#[derive(Debug, Clone)]
pub struct SecurityManager {
    config: SecurityConfig,
    sanitizer: InputSanitizer,
    access_control: AccessControl,
    audit_logger: AuditLogger,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(config: SecurityConfig) -> Self {
        let sanitizer = InputSanitizer::new(config.clone());
        let access_control = AccessControl::new(config.clone());
        let audit_logger = AuditLogger::new(config.clone());

        Self {
            config,
            sanitizer,
            access_control,
            audit_logger,
        }
    }

    /// Validate and sanitize input
    #[instrument(skip_all)]
    pub async fn validate_input(&self, user_id: &str, operation: &str, input: &str) -> RhemaResult<String> {
        // Check permissions
        if !self.access_control.check_permission(user_id, operation).await? {
            return Err(RhemaError::AuthorizationError(
                format!("User '{}' not authorized for operation '{}'", user_id, operation)
            ));
        }

        // Sanitize input
        let sanitized = self.sanitizer.sanitize_string(input)?;

        // Log the event
        self.audit_logger.log_event(AuditLogEntry {
            timestamp: chrono::Utc::now(),
            user_id: user_id.to_string(),
            operation: operation.to_string(),
            resource: "input".to_string(),
            success: true,
            error_message: None,
            ip_address: None,
            user_agent: None,
            metadata: HashMap::new(),
        }).await?;

        Ok(sanitized)
    }

    /// Validate file access
    #[instrument(skip_all)]
    pub async fn validate_file_access(&self, user_id: &str, file_path: &str) -> RhemaResult<String> {
        // Check permissions
        if !self.access_control.check_permission(user_id, "file_access").await? {
            return Err(RhemaError::AuthorizationError(
                format!("User '{}' not authorized for file access", user_id)
            ));
        }

        // Sanitize file path
        let sanitized_path = self.sanitizer.sanitize_file_path(file_path)?;

        // Log the event
        self.audit_logger.log_event(AuditLogEntry {
            timestamp: chrono::Utc::now(),
            user_id: user_id.to_string(),
            operation: "file_access".to_string(),
            resource: sanitized_path.clone(),
            success: true,
            error_message: None,
            ip_address: None,
            user_agent: None,
            metadata: HashMap::new(),
        }).await?;

        Ok(sanitized_path)
    }

    /// Validate query execution
    #[instrument(skip_all)]
    pub async fn validate_query(&self, user_id: &str, query: &str) -> RhemaResult<String> {
        // Check permissions
        if !self.access_control.check_permission(user_id, "query_execution").await? {
            return Err(RhemaError::AuthorizationError(
                format!("User '{}' not authorized for query execution", user_id)
            ));
        }

        // Sanitize query
        let sanitized_query = self.sanitizer.sanitize_query(query)?;

        // Log the event
        self.audit_logger.log_event(AuditLogEntry {
            timestamp: chrono::Utc::now(),
            user_id: user_id.to_string(),
            operation: "query_execution".to_string(),
            resource: "query".to_string(),
            success: true,
            error_message: None,
            ip_address: None,
            user_agent: None,
            metadata: HashMap::from([
                ("query".to_string(), serde_yaml::Value::String(sanitized_query.clone())),
            ]),
        }).await?;

        Ok(sanitized_query)
    }
}

// Import RhemaError and RhemaResult
use crate::{RhemaError, RhemaResult}; 