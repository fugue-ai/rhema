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

use chrono::Utc;
use git2::{Commit, Repository, Signature};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::time::Instant;
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce, KeyInit as ChaChaKeyInit};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use rand::{Rng, RngCore};
use keyring::Keyring;
use tracing::{info, warn, error, debug};
use walkdir::WalkDir;

/// Security configuration for Git integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable security features
    pub enabled: bool,

    /// Access control settings
    pub access_control: AccessControlConfig,

    /// Audit logging settings
    pub audit_logging: AuditLoggingConfig,

    /// Security validation settings
    pub validation: SecurityValidationConfig,

    /// Encryption settings
    pub encryption: EncryptionConfig,

    /// Threat detection settings
    pub threat_detection: ThreatDetectionConfig,
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Require authentication for all operations
    pub require_authentication: bool,

    /// Role-based access control
    pub rbac_enabled: bool,

    /// User roles and permissions
    pub roles: HashMap<String, RolePermissions>,

    /// Branch protection rules
    pub branch_protection: HashMap<String, BranchProtectionRule>,

    /// File access control
    pub file_access_control: FileAccessControl,
}

/// Role permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermissions {
    /// Role name
    pub name: String,

    /// Allowed operations
    pub allowed_operations: Vec<Operation>,

    /// Allowed branches
    pub allowed_branches: Vec<String>,

    /// Allowed file patterns
    pub allowed_files: Vec<String>,

    /// Denied file patterns
    pub denied_files: Vec<String>,
}

/// Git operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Operation {
    Read,
    Write,
    Commit,
    Push,
    Pull,
    Merge,
    Rebase,
    Delete,
    Admin,
    Execute,
}

/// Branch protection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtectionRule {
    /// Branch name pattern
    pub pattern: String,

    /// Require code review
    pub require_review: bool,

    /// Require status checks
    pub require_status_checks: bool,

    /// Require up-to-date branches
    pub require_up_to_date: bool,

    /// Restrict pushes
    pub restrict_pushes: bool,

    /// Allowed users
    pub allowed_users: Vec<String>,

    /// Allowed teams
    pub allowed_teams: Vec<String>,
}

/// File access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessControl {
    /// Sensitive file patterns
    pub sensitive_files: Vec<String>,

    /// Read-only file patterns
    pub read_only_files: Vec<String>,

    /// Admin-only file patterns
    pub admin_only_files: Vec<String>,

    /// Encryption required patterns
    pub encryption_required: Vec<String>,
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggingConfig {
    /// Enable audit logging
    pub enabled: bool,

    /// Log file path
    pub log_file: PathBuf,

    /// Log level
    pub log_level: LogLevel,

    /// Events to log
    pub events: Vec<AuditEvent>,

    /// Retention policy
    pub retention: RetentionPolicy,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditEvent {
    Commit,
    Push,
    Pull,
    Merge,
    Rebase,
    BranchCreate,
    BranchDelete,
    TagCreate,
    TagDelete,
    FileAccess,
    SecurityViolation,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Keep logs for N days
    pub retention_days: u32,

    /// Maximum log file size (MB)
    pub max_file_size_mb: u64,

    /// Archive old logs
    pub archive_old_logs: bool,

    /// Archive directory
    pub archive_directory: Option<PathBuf>,
}

/// Security validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationConfig {
    /// Validate commit signatures
    pub validate_signatures: bool,

    /// Check for suspicious patterns
    pub check_suspicious_patterns: bool,

    /// Validate file permissions
    pub validate_permissions: bool,

    /// Check for secrets in code
    pub check_secrets: bool,

    /// Validate dependencies
    pub validate_dependencies: bool,

    /// Security scanning
    pub security_scanning: SecurityScanningConfig,
}

/// Security scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanningConfig {
    /// Enable security scanning
    pub enabled: bool,

    /// Scan for common vulnerabilities
    pub scan_vulnerabilities: bool,

    /// Scan for malware
    pub scan_malware: bool,

    /// Scan for secrets
    pub scan_secrets: bool,

    /// Custom security rules
    pub custom_rules: Vec<SecurityRule>,
}

/// Security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule name
    pub name: String,

    /// Rule pattern
    pub pattern: String,

    /// Rule severity
    pub severity: SecuritySeverity,

    /// Rule description
    pub description: String,

    /// Rule action
    pub action: SecurityAction,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    Warn,
    Block,
    Log,
    Notify,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable encryption
    pub enabled: bool,

    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,

    /// Key management
    pub key_management: KeyManagementConfig,

    /// Encrypt sensitive files
    pub encrypt_sensitive_files: bool,

    /// Encrypt context files
    pub encrypt_context_files: bool,
}

/// Encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    Custom(String),
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key storage location
    pub key_storage: KeyStorage,

    /// Key rotation policy
    pub key_rotation: KeyRotationPolicy,

    /// Backup keys
    pub backup_keys: bool,
}

/// Key storage options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyStorage {
    File(PathBuf),
    Environment,
    Keyring,
    Custom(String),
    Database,
    Cloud,
}

/// Key rotation policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationPolicy {
    /// Rotate keys every N days
    pub rotation_days: u32,

    /// Auto-rotate keys
    pub auto_rotate: bool,

    /// Notify before rotation
    pub notify_before_rotation: bool,
}

/// Threat detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable threat detection
    pub enabled: bool,

    /// Detection rules
    pub rules: Vec<ThreatDetectionRule>,

    /// Machine learning models
    pub ml_models: Vec<MLModel>,

    /// Alerting
    pub alerting: AlertingConfig,
}

/// Threat detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    /// Rule name
    pub name: String,

    /// Rule type
    pub rule_type: ThreatRuleType,

    /// Rule pattern
    pub pattern: String,

    /// Threshold
    pub threshold: f64,

    /// Action
    pub action: ThreatAction,
}

/// Threat rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatRuleType {
    Pattern,
    Frequency,
    Anomaly,
    ML,
}

/// Threat actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatAction {
    Alert,
    Block,
    Quarantine,
    Log,
}

/// Machine learning model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModel {
    /// Model name
    pub name: String,

    /// Model type
    pub model_type: MLModelType,

    /// Model path
    pub model_path: PathBuf,

    /// Confidence threshold
    pub confidence_threshold: f64,
}

/// ML model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLModelType {
    AnomalyDetection,
    Classification,
    Regression,
    Custom(String),
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Email alerts
    pub email: Option<EmailAlertConfig>,

    /// Slack alerts
    pub slack: Option<SlackAlertConfig>,

    /// Webhook alerts
    pub webhook: Option<WebhookAlertConfig>,

    /// Alert severity levels
    pub severity_levels: Vec<SecuritySeverity>,
}

/// Email alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAlertConfig {
    pub recipients: Vec<String>,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
}

/// Slack alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAlertConfig {
    pub webhook_url: String,
    pub channel: String,
    pub username: String,
}

/// Webhook alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAlertConfig {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub timeout: u64,
}

/// Security manager for Git integration
pub struct SecurityManager {
    #[allow(dead_code)]
    repo: Repository,
    config: SecurityConfig,
    audit_logger: Option<AuditLogger>,
}

/// Audit logger
pub struct AuditLogger {
    #[allow(dead_code)]
    log_file: PathBuf,
    #[allow(dead_code)]
    log_level: LogLevel,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(repo: Repository, config: SecurityConfig) -> RhemaResult<Self> {
        let audit_logger = if config.audit_logging.enabled {
            Some(AuditLogger::new(
                config.audit_logging.log_file.clone(),
                config.audit_logging.log_level.clone(),
            )?)
        } else {
            None
        };

        Ok(Self {
            repo,
            config,
            audit_logger,
        })
    }

    /// Validate access for a user
    pub fn validate_access(
        &self,
        user: &str,
        operation: &Operation,
        resource: &str,
    ) -> RhemaResult<bool> {
        if !self.config.enabled {
            return Ok(true);
        }

        // Check if user has role
        let user_role = self.get_user_role(user)?;

        // Check role permissions
        if let Some(permissions) = self.config.access_control.roles.get(&user_role) {
            if !permissions.allowed_operations.contains(operation) {
                self.log_audit_event(
                    AuditEvent::SecurityViolation,
                    &format!(
                        "User {} attempted unauthorized operation {:?} on {}",
                        user, operation, resource
                    ),
                    LogLevel::Warn,
                )?;
                return Ok(false);
            }
        }

        self.log_audit_event(
            AuditEvent::FileAccess,
            &format!(
                "User {} performed operation {:?} on {}",
                user, operation, resource
            ),
            LogLevel::Info,
        )?;

        Ok(true)
    }

    /// Validate commit security
    pub fn validate_commit_security(
        &self,
        commit: &Commit,
    ) -> RhemaResult<SecurityValidationResult> {
        let mut result = SecurityValidationResult::new();

        if self.config.validation.validate_signatures {
            self.validate_commit_signature(commit, &mut result)?;
        }

        if self.config.validation.check_suspicious_patterns {
            self.check_suspicious_patterns(commit, &mut result)?;
        }

        if self.config.validation.check_secrets {
            self.check_for_secrets(commit, &mut result)?;
        }

        self.log_audit_event(
            AuditEvent::Commit,
            &format!("Security validation for commit {}", commit.id()),
            LogLevel::Info,
        )?;

        Ok(result)
    }

    /// Validate commit signature
    fn validate_commit_signature(
        &self,
        commit: &Commit,
        result: &mut SecurityValidationResult,
    ) -> RhemaResult<()> {
        // Get commit author (signature)
        let author = commit.author();
        
        // For now, we'll just validate the author signature
        // In a real implementation, you would verify GPG signatures
        self.validate_signature_metadata(author, result)?;
        
        // Check if commit has a GPG signature (this would require additional git2 features)
        // For now, we'll just log that we're checking
        result.add_info("Commit signature validation completed".to_string());
        
        Ok(())
    }

    /// Validate signature metadata
    fn validate_signature_metadata(
        &self,
        signature: Signature,
        result: &mut SecurityValidationResult,
    ) -> RhemaResult<()> {
        // Check signature timestamp
        let now = Utc::now().timestamp();
        let sig_time = signature.when().seconds();
        let time_diff = (now - sig_time).abs();

        if time_diff > 86400 * 365 { // 1 year
            result.add_warning("Commit signature is very old".to_string());
        }

        // Validate email format
        if let Some(email) = signature.email() {
            if !self.is_valid_email(email) {
                result.add_warning("Invalid email format in signature".to_string());
            }
        }

        Ok(())
    }

    /// Validate email format
    fn is_valid_email(&self, email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    /// Check for suspicious patterns
    fn check_suspicious_patterns(
        &self,
        commit: &Commit,
        result: &mut SecurityValidationResult,
    ) -> RhemaResult<()> {
        let message = commit.message().unwrap_or("");
        let author = commit.author();
        let author_name = author.name().unwrap_or("");
        let author_email = author.email().unwrap_or("");

        // Check for suspicious patterns in commit message
        let suspicious_patterns = vec![
            (r"(?i)password|secret|key|token|credential", "Potential credential exposure"),
            (r"(?i)fix|patch|hotfix|urgent", "Urgent fix pattern"),
            (r"(?i)debug|test|temp|tmp", "Debug/test code"),
            (r"(?i)backdoor|exploit|hack", "Suspicious security terms"),
            (r"(?i)admin|root|sudo", "Administrative operations"),
        ];

        for (pattern, description) in suspicious_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(message) {
                    result.add_warning(format!("Suspicious pattern in commit message: {}", description));
                    result.add_issue(SecurityIssue {
                        severity: "Medium".to_string(),
                        category: "Pattern".to_string(),
                        description: description.to_string(),
                        file_path: None,
                        line_number: None,
                    });
                }
            }
        }

        // Check for suspicious author patterns
        let suspicious_authors = vec![
            (r"(?i)test|temp|admin|root", "Suspicious author name"),
            (r"(?i)example\.com|test\.com|localhost", "Suspicious email domain"),
        ];

        for (pattern, description) in suspicious_authors {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(author_name) || regex.is_match(author_email) {
                    result.add_warning(format!("Suspicious author pattern: {}", description));
                }
            }
        }

        // Check for suspicious patterns in tree structure
        if let Ok(tree) = commit.tree() {
            // Use tree.iter() instead of tree.count()
            let entry_count = tree.iter().count();
            if entry_count > 100 {
                result.add_warning("Large commit detected (potential bulk changes)".to_string());
            }
            if entry_count > 1000 {
                result.add_warning("Large number of files in commit".to_string());
            }
            
            // Check for suspicious file patterns
            for entry in tree.iter() {
                let name = entry.name().unwrap_or("");
                if name.contains("password") || name.contains("secret") || name.contains("key") {
                    result.add_warning(format!("Suspicious file name: {}", name));
                }
            }
        }

        Ok(())
    }

    /// Check for secrets in commit
    fn check_for_secrets(
        &self,
        commit: &Commit,
        result: &mut SecurityValidationResult,
    ) -> RhemaResult<()> {
        let message = commit.message().unwrap_or("");
        
        // Common secret patterns
        let secret_patterns = vec![
            // API Keys
            ("sk-[a-zA-Z0-9]{20,}".to_string(), "Stripe API Key".to_string()),
            ("pk_[a-zA-Z0-9]{24}".to_string(), "Stripe Publishable Key".to_string()),
            ("AKIA[0-9A-Z]{16}".to_string(), "AWS Access Key ID".to_string()),
            ("[0-9a-zA-Z/+]{40}".to_string(), "AWS Secret Access Key".to_string()),
            ("AIza[0-9A-Za-z\\-_]{35}".to_string(), "Google API Key".to_string()),
            ("[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com".to_string(), "Google OAuth Client ID".to_string()),
            ("ya29\\.[0-9A-Za-z\\-_]+".to_string(), "Google OAuth Access Token".to_string()),
            
            // Database credentials
            ("mongodb://[a-zA-Z0-9:._%+-]+@[a-zA-Z0-9.-]+:[0-9]+/[a-zA-Z0-9._%+-]+".to_string(), "MongoDB Connection String".to_string()),
            ("postgresql://[a-zA-Z0-9:._%+-]+@[a-zA-Z0-9.-]+:[0-9]+/[a-zA-Z0-9._%+-]+".to_string(), "PostgreSQL Connection String".to_string()),
            ("mysql://[a-zA-Z0-9:._%+-]+@[a-zA-Z0-9.-]+:[0-9]+/[a-zA-Z0-9._%+-]+".to_string(), "MySQL Connection String".to_string()),
            
            // SSH Keys
            ("ssh-rsa [A-Za-z0-9+/]+[=]{0,3} [^@]+@[^@]+".to_string(), "SSH Public Key".to_string()),
            ("-----BEGIN OPENSSH PRIVATE KEY-----".to_string(), "SSH Private Key".to_string()),
            ("-----BEGIN RSA PRIVATE KEY-----".to_string(), "RSA Private Key".to_string()),
            ("-----BEGIN DSA PRIVATE KEY-----".to_string(), "DSA Private Key".to_string()),
            ("-----BEGIN EC PRIVATE KEY-----".to_string(), "EC Private Key".to_string()),
            
            // Passwords and tokens
            ("password[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Password in plain text".to_string()),
            ("token[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9]{20,}['\"]?".to_string(), "Token in plain text".to_string()),
            ("secret[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Secret in plain text".to_string()),
            
            // JWT Tokens
            ("eyJ[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*".to_string(), "JWT Token".to_string()),
            
            // Private keys
            ("-----BEGIN PRIVATE KEY-----".to_string(), "Private Key".to_string()),
            ("-----BEGIN PGP PRIVATE KEY BLOCK-----".to_string(), "PGP Private Key".to_string()),
            
            // Access tokens
            ("[a-zA-Z0-9]{32,}".to_string(), "Potential access token".to_string()),
        ];

        for (pattern, description) in secret_patterns {
            if let Ok(regex) = Regex::new(pattern.as_str()) {
                if regex.is_match(message) {
                    result.add_error(format!("Secret detected in commit message: {}", description));
                    result.add_issue(SecurityIssue {
                        severity: "Critical".to_string(),
                        category: "Secret".to_string(),
                        description: description.to_string(),
                        file_path: None,
                        line_number: None,
                    });
                }
            }
        }

        // Check commit diff for secrets
        if let Ok(parent) = commit.parent(0) {
            if let Ok(diff) = self.repo.diff_tree_to_tree(
                Some(&parent.tree()?),
                Some(&commit.tree()?),
                None,
            ) {
                for delta in diff.deltas() {
                    if let Some(new_file) = delta.new_file().path() {
                        if let Ok(content) = fs::read_to_string(new_file) {
                            self.scan_content_for_secrets(&content, new_file, result)?;
                        }
                    }
                }
            }
        }

        // Scan tree for sensitive files
        if let Ok(tree) = commit.tree() {
            for entry in tree.iter() {
                let entry_path = entry.name().unwrap_or("");
                
                // Check if file matches sensitive patterns
                for pattern in &self.config.validation.security_scanning.custom_rules {
                    if let Ok(regex) = Regex::new(&pattern.pattern) {
                        if regex.is_match(entry_path) {
                            result.add_issue(SecurityIssue {
                                severity: format!("{:?}", pattern.severity),
                                category: "Sensitive File".to_string(),
                                description: format!("Sensitive file detected: {}", entry_path),
                                file_path: Some(PathBuf::from(entry_path)),
                                line_number: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Scan content for secrets
    fn scan_content_for_secrets(
        &self,
        content: &str,
        file_path: &Path,
        result: &mut SecurityValidationResult,
    ) -> RhemaResult<()> {
        let secret_patterns = vec![
            ("password[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Password in plain text".to_string()),
            ("api_key[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9]{20,}['\"]?".to_string(), "API Key in plain text".to_string()),
            ("secret[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Secret in plain text".to_string()),
            ("token[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9]{20,}['\"]?".to_string(), "Token in plain text".to_string()),
        ];

        for (line_num, line) in content.lines().enumerate() {
            for (pattern, description) in &secret_patterns {
                if let Ok(regex) = Regex::new(pattern.as_str()) {
                    if regex.is_match(line) {
                        result.add_error(format!("Secret detected in {}:{}: {}", 
                            file_path.display(), line_num + 1, description));
                        result.add_issue(SecurityIssue {
                            severity: "Critical".to_string(),
                            category: "Secret".to_string(),
                            description: description.to_string(),
                            file_path: Some(file_path.to_path_buf()),
                            line_number: Some(line_num as u32 + 1),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Get user role
    fn get_user_role(&self, user: &str) -> RhemaResult<String> {
        // Check if user exists in role configuration
        if self.config.access_control.roles.contains_key(user) {
            return Ok(user.to_string());
        }

        // Check for role patterns (e.g., admin-*, dev-*)
        for (role_name, _) in &self.config.access_control.roles {
            if role_name.ends_with("*") {
                let pattern = &role_name[..role_name.len() - 1];
                if user.starts_with(pattern) {
                    return Ok(role_name.clone());
                }
            }
        }

        // Check environment variables for role mapping
        if let Ok(role) = std::env::var(format!("RHEMA_USER_ROLE_{}", user.to_uppercase())) {
            return Ok(role);
        }

        // Default role
        Ok("default".to_string())
    }

    /// Log audit event
    fn log_audit_event(
        &self,
        event: AuditEvent,
        message: &str,
        level: LogLevel,
    ) -> RhemaResult<()> {
        if let Some(logger) = &self.audit_logger {
            logger.log(event, message, level)?;
        }
        Ok(())
    }

    /// Run security scan
    pub fn run_security_scan(&self, path: &Path) -> RhemaResult<SecurityScanResult> {
        let mut result = SecurityScanResult::new();

        if self.config.validation.security_scanning.enabled {
            if self
                .config
                .validation
                .security_scanning
                .scan_vulnerabilities
            {
                self.scan_vulnerabilities(path, &mut result)?;
            }

            if self.config.validation.security_scanning.scan_malware {
                self.scan_malware(path, &mut result)?;
            }

            if self.config.validation.security_scanning.scan_secrets {
                self.scan_secrets(path, &mut result)?;
            }
        }

        Ok(result)
    }

    /// Scan for vulnerabilities
    fn scan_vulnerabilities(
        &self,
        path: &Path,
        result: &mut SecurityScanResult,
    ) -> RhemaResult<()> {
        // Common vulnerability patterns
        let vulnerability_patterns = vec![
            // SQL Injection
            ("(?i)SELECT.*FROM.*WHERE.*\\$\\{.*\\}".to_string(), "Potential SQL injection".to_string()),
            ("(?i)INSERT.*INTO.*VALUES.*\\$\\{.*\\}".to_string(), "Potential SQL injection".to_string()),
            ("(?i)UPDATE.*SET.*WHERE.*\\$\\{.*\\}".to_string(), "Potential SQL injection".to_string()),
            ("(?i)DELETE.*FROM.*WHERE.*\\$\\{.*\\}".to_string(), "Potential SQL injection".to_string()),
            
            // XSS
            ("(?i)innerHTML.*\\$\\{.*\\}".to_string(), "Potential XSS vulnerability".to_string()),
            ("(?i)outerHTML.*\\$\\{.*\\}".to_string(), "Potential XSS vulnerability".to_string()),
            ("(?i)document\\.write.*\\$\\{.*\\}".to_string(), "Potential XSS vulnerability".to_string()),
            
            // Command Injection
            ("(?i)exec.*\\$\\{.*\\}".to_string(), "Potential command injection".to_string()),
            ("(?i)system.*\\$\\{.*\\}".to_string(), "Potential command injection".to_string()),
            ("(?i)shell_exec.*\\$\\{.*\\}".to_string(), "Potential command injection".to_string()),
            
            // Path Traversal
            ("(?i)\\.\\./".to_string(), "Potential path traversal".to_string()),
            ("(?i)\\.\\.\\\\".to_string(), "Potential path traversal".to_string()),
            
            // Hardcoded credentials
            ("(?i)password[\\s]*=[\\s]*['\"][^'\"]{8,}['\"]".to_string(), "Hardcoded password".to_string()),
            ("(?i)secret[\\s]*=[\\s]*['\"][^'\"]{8,}['\"]".to_string(), "Hardcoded secret".to_string()),
            
            // Insecure random
            ("(?i)Math\\.random\\(\\)".to_string(), "Insecure random number generation".to_string()),
            ("(?i)rand\\(\\)".to_string(), "Insecure random number generation".to_string()),
            
            // Weak encryption
            ("(?i)md5\\(".to_string(), "Weak hash function (MD5)".to_string()),
            ("(?i)sha1\\(".to_string(), "Weak hash function (SHA1)".to_string()),
            
            // Debug code
            ("(?i)console\\.log".to_string(), "Debug code in production".to_string()),
            ("(?i)debugger;".to_string(), "Debug code in production".to_string()),
            ("(?i)print\\(".to_string(), "Debug code in production".to_string()),
        ];

        self.scan_directory_for_patterns(path, &vulnerability_patterns, "Vulnerability", result)?;
        Ok(())
    }

    /// Scan directory for patterns
    fn scan_directory_for_patterns(
        &self,
        path: &Path,
        patterns: &[(String, String)],
        category: &str,
        result: &mut SecurityScanResult,
    ) -> RhemaResult<()> {
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(path) {
                for (line_num, line) in content.lines().enumerate() {
                    for (pattern, description) in patterns {
                        if let Ok(regex) = Regex::new(pattern) {
                            if regex.is_match(line) {
                                result.add_vulnerability(format!("{} in {}:{}: {}", 
                                    category, path.display(), line_num + 1, description));
                                result.add_issue(SecurityIssue {
                                    severity: "Medium".to_string(),
                                    category: category.to_string(),
                                    description: description.to_string(),
                                    file_path: Some(path.to_path_buf()),
                                    line_number: Some(line_num as u32 + 1),
                                });
                            }
                        }
                    }
                }
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let file_path = entry.path();
                if let Ok(content) = fs::read_to_string(file_path) {
                    for (line_num, line) in content.lines().enumerate() {
                        for (pattern, description) in patterns {
                            if let Ok(regex) = Regex::new(pattern) {
                                if regex.is_match(line) {
                                    result.add_vulnerability(format!("{} in {}:{}: {}", 
                                        category, file_path.display(), line_num + 1, description));
                                    result.add_issue(SecurityIssue {
                                        severity: "Medium".to_string(),
                                        category: category.to_string(),
                                        description: description.to_string(),
                                        file_path: Some(file_path.to_path_buf()),
                                        line_number: Some(line_num as u32 + 1),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Scan for malware
    fn scan_malware(&self, path: &Path, result: &mut SecurityScanResult) -> RhemaResult<()> {
        // Malware patterns
        let malware_patterns = vec![
            ("(?i)eval\\(".to_string(), "Potential code injection".to_string()),
            ("(?i)exec\\(".to_string(), "Potential code execution".to_string()),
            ("(?i)system\\(".to_string(), "Potential system command execution".to_string()),
            ("(?i)shell_exec\\(".to_string(), "Potential shell command execution".to_string()),
            ("(?i)passthru\\(".to_string(), "Potential command passthrough".to_string()),
            ("(?i)base64_decode\\(".to_string(), "Potential encoded payload".to_string()),
            ("(?i)gzinflate\\(".to_string(), "Potential compressed payload".to_string()),
            ("(?i)str_rot13\\(".to_string(), "Potential obfuscated code".to_string()),
            ("(?i)create_function\\(".to_string(), "Potential dynamic code creation".to_string()),
            ("(?i)assert\\(".to_string(), "Potential code execution".to_string()),
        ];

        self.scan_directory_for_patterns(path, &malware_patterns, "Malware", result)?;
        Ok(())
    }

    /// Scan for secrets
    fn scan_secrets(&self, path: &Path, result: &mut SecurityScanResult) -> RhemaResult<()> {
        // Secret patterns
        let secret_patterns = vec![
            ("sk-[a-zA-Z0-9]{20,}".to_string(), "Stripe API Key".to_string()),
            ("pk_[a-zA-Z0-9]{24}".to_string(), "Stripe Publishable Key".to_string()),
            ("AKIA[0-9A-Z]{16}".to_string(), "AWS Access Key ID".to_string()),
            ("[0-9a-zA-Z/+]{40}".to_string(), "AWS Secret Access Key".to_string()),
            ("AIza[0-9A-Za-z\\-_]{35}".to_string(), "Google API Key".to_string()),
            ("[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com".to_string(), "Google OAuth Client ID".to_string()),
            ("ya29\\.[0-9A-Za-z\\-_]+".to_string(), "Google OAuth Access Token".to_string()),
            ("ssh-rsa [A-Za-z0-9+/]+[=]{0,3} [^@]+@[^@]+".to_string(), "SSH Public Key".to_string()),
            ("-----BEGIN OPENSSH PRIVATE KEY-----".to_string(), "SSH Private Key".to_string()),
            ("-----BEGIN RSA PRIVATE KEY-----".to_string(), "RSA Private Key".to_string()),
            ("-----BEGIN DSA PRIVATE KEY-----".to_string(), "DSA Private Key".to_string()),
            ("-----BEGIN EC PRIVATE KEY-----".to_string(), "EC Private Key".to_string()),
            ("password[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Password in plain text".to_string()),
            ("token[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9]{20,}['\"]?".to_string(), "Token in plain text".to_string()),
            ("secret[\\s]*[=:][\\s]*['\"]?[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?]{8,}['\"]?".to_string(), "Secret in plain text".to_string()),
            ("eyJ[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*".to_string(), "JWT Token".to_string()),
            ("-----BEGIN PRIVATE KEY-----".to_string(), "Private Key".to_string()),
            ("-----BEGIN PGP PRIVATE KEY BLOCK-----".to_string(), "PGP Private Key".to_string()),
        ];

        self.scan_directory_for_patterns(path, &secret_patterns, "Secret", result)?;
        Ok(())
    }

    /// Encrypt file
    pub fn encrypt_file(&self, file_path: &Path) -> RhemaResult<()> {
        if !self.config.encryption.enabled {
            return Ok(());
        }

        let key = self.get_encryption_key()?;
        let content = fs::read(file_path)?;
        
        let encrypted_content = match &self.config.encryption.algorithm {
            EncryptionAlgorithm::AES256 => {
                let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
                let nonce = self.generate_nonce();
                cipher.encrypt((&nonce).into(), content.as_ref())
                    .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Encryption failed: {}", e))))?
            }
            EncryptionAlgorithm::ChaCha20 => {
                let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key));
                let nonce = self.generate_chacha_nonce();
                cipher.encrypt(&nonce, content.as_ref())
                    .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Encryption failed: {}", e))))?
            }
            EncryptionAlgorithm::Custom(algorithm_name) => {
                // Implement custom encryption based on algorithm name
                let key_bytes = key.as_slice();
                match algorithm_name.as_str() {
                    "xor" => {
                        // Simple XOR encryption (for demonstration)
                        let mut encrypted = Vec::new();
                        for (i, &byte) in content.iter().enumerate() {
                            encrypted.push(byte ^ key_bytes[i % key_bytes.len()]);
                        }
                        encrypted
                    }
                    "caesar" => {
                        // Caesar cipher encryption (for demonstration)
                        let shift = key_bytes[0] as u8 % 26;
                        let mut encrypted = Vec::new();
                        for &byte in content.as_slice() {
                            let byte: u8 = byte;
                            if byte.is_ascii_alphabetic() {
                                let base = if byte.is_ascii_uppercase() { b'A' } else { b'a' };
                                encrypted.push(((byte - base + shift) % 26) + base);
                            } else {
                                encrypted.push(byte);
                            }
                        }
                        encrypted
                    }
                    _ => {
                        return Err(RhemaError::GitError(git2::Error::from_str("Unknown custom encryption algorithm")));
                    }
                }
            }
        };

        // Write encrypted content with .encrypted extension
        let encrypted_path = file_path.with_extension("encrypted");
        fs::write(&encrypted_path, encrypted_content)?;
        
        // Remove original file
        fs::remove_file(file_path)?;
        
        info!("File encrypted: {} -> {}", file_path.display(), encrypted_path.display());
        Ok(())
    }

    /// Decrypt file
    pub fn decrypt_file(&self, file_path: &Path) -> RhemaResult<()> {
        if !self.config.encryption.enabled {
            return Ok(());
        }

        let key = self.get_encryption_key()?;
        let encrypted_content = fs::read(file_path)?;
        
        let decrypted_content = match &self.config.encryption.algorithm {
            EncryptionAlgorithm::AES256 => {
                let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
                let nonce = self.generate_nonce();
                cipher.decrypt((&nonce).into(), encrypted_content.as_ref())
                    .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Decryption failed: {}", e))))?
            }
            EncryptionAlgorithm::ChaCha20 => {
                let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key));
                let nonce = self.generate_chacha_nonce();
                cipher.decrypt(&nonce, encrypted_content.as_ref())
                    .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Decryption failed: {}", e))))?
            }
            EncryptionAlgorithm::Custom(algorithm_name) => {
                // Implement custom decryption based on algorithm name
                let key_bytes = key.as_slice();
                match algorithm_name.as_str() {
                    "xor" => {
                        // Simple XOR decryption (same as encryption for XOR)
                        let mut decrypted = Vec::new();
                        for (i, &byte) in encrypted_content.iter().enumerate() {
                            decrypted.push(byte ^ key_bytes[i % key_bytes.len()]);
                        }
                        decrypted
                    }
                    "caesar" => {
                        // Caesar cipher decryption
                        let shift = key_bytes[0] as u8 % 26;
                        let mut decrypted = Vec::new();
                        for &byte in encrypted_content.as_slice() {
                            let byte: u8 = byte;
                            if byte.is_ascii_alphabetic() {
                                let base = if byte.is_ascii_uppercase() { b'A' } else { b'a' };
                                decrypted.push(((byte - base + 26 - shift) % 26) + base);
                            } else {
                                decrypted.push(byte);
                            }
                        }
                        decrypted
                    }
                    _ => {
                        return Err(RhemaError::GitError(git2::Error::from_str("Unknown custom decryption algorithm")));
                    }
                }
            }
        };

        // Write decrypted content without .encrypted extension
        let decrypted_path = file_path.with_extension("");
        fs::write(&decrypted_path, decrypted_content)?;
        
        // Remove encrypted file
        fs::remove_file(file_path)?;
        
        info!("File decrypted: {} -> {}", file_path.display(), decrypted_path.display());
        Ok(())
    }

    /// Get encryption key
    fn get_encryption_key(&self) -> RhemaResult<Vec<u8>> {
        match &self.config.encryption.key_management.key_storage {
            KeyStorage::File(path) => {
                if path.exists() {
                    let key_content = fs::read_to_string(path)?;
                    Ok(general_purpose::STANDARD.decode(key_content.trim())
                        .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Base64 decode error: {}", e))))?)
                } else {
                    // Generate new key
                    let key = self.generate_encryption_key()?;
                    fs::write(path, general_purpose::STANDARD.encode(&key))?;
                    Ok(key)
                }
            }
            KeyStorage::Environment => {
                let key_var = std::env::var("RHEMA_ENCRYPTION_KEY")
                    .map_err(|_| RhemaError::GitError(git2::Error::from_str("RHEMA_ENCRYPTION_KEY not set")))?;
                Ok(general_purpose::STANDARD.decode(key_var)
                    .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Base64 decode error: {}", e))))?)
            }
            KeyStorage::Keyring => {
                        let keyring = Keyring::new("rhema", "encryption_key");
        match keyring.get_password() {
                    Ok(password) => Ok(general_purpose::STANDARD.decode(password)
                        .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Base64 decode error: {}", e))))?),
                    Err(_) => {
                        // Generate new key
                        let key = self.generate_encryption_key()?;
                        let key_b64 = general_purpose::STANDARD.encode(&key);
                        keyring.set_password(&key_b64)
                            .map_err(|e| RhemaError::GitError(git2::Error::from_str(&format!("Failed to store key: {}", e))))?;
                        Ok(key)
                    }
                }
            }
            KeyStorage::Database => {
                // TODO: Implement database key storage
                Err(RhemaError::GitError(git2::Error::from_str("Database key storage not implemented")))
            }
            KeyStorage::Cloud => {
                // TODO: Implement cloud key storage
                Err(RhemaError::GitError(git2::Error::from_str("Cloud key storage not implemented")))
            }
            KeyStorage::Custom(storage_type) => {
                Err(RhemaError::GitError(git2::Error::from_str(&format!("Unknown custom key storage type: {}", storage_type))))
            }
        }
    }

    /// Generate encryption key
    fn generate_encryption_key(&self) -> RhemaResult<Vec<u8>> {
        let mut key = vec![0u8; 32]; // 256-bit key
        rand::thread_rng().fill_bytes(&mut key);
        Ok(key)
    }

    /// Generate AES nonce
    fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        nonce_bytes
    }

    /// Generate ChaCha20 nonce
    fn generate_chacha_nonce(&self) -> ChaChaNonce {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        *ChaChaNonce::from_slice(&nonce_bytes)
    }

    /// Enhanced signature validation with detailed verification
    pub async fn validate_signature_enhanced(&self, commit_id: &str) -> RhemaResult<SignatureValidationResult> {
        let mut result = SignatureValidationResult::new();
        
        // Find the commit
        let oid = git2::Oid::from_str(commit_id)?;
        let commit = self.repo.find_commit(oid)?;
        
        // Get signature - use author() instead of signature()
        let signature = commit.author();
        
        // Note: GPG signature verification not available in git2 0.18
        result.add_warning("GPG signature verification not available in this git2 version".to_string());
        result.signature_valid = false;
        
        // Validate signature metadata
        self.validate_signature_metadata_enhanced(&signature, &mut result)?;
        
        // Check signature against known keys
        self.validate_signature_against_known_keys(&signature, &mut result).await?;
        
        // Validate signature timestamp
        self.validate_signature_timestamp(&signature, &mut result)?;
        
        // Check for signature replay attacks
        self.check_for_signature_replay(&signature, &mut result)?;
        
        Ok(result)
    }

    /// Validate signature metadata with enhanced checks
    fn validate_signature_metadata_enhanced(
        &self,
        signature: &Signature,
        result: &mut SignatureValidationResult,
    ) -> RhemaResult<()> {
        // Validate email format
        if let Some(email) = signature.email() {
            if !self.is_valid_email(email) {
                result.add_error(format!("Invalid email format in signature: {}", email));
                result.metadata_valid = false;
            } else {
                result.add_info(format!("Valid email format: {}", email));
            }
        }

        // Validate name format
        if let Some(name) = signature.name() {
            if name.trim().is_empty() {
                result.add_error("Empty name in signature".to_string());
                result.metadata_valid = false;
            } else if name.len() > 100 {
                result.add_warning("Name in signature is unusually long".to_string());
            }
        }

        // Check for suspicious patterns in name/email
        let suspicious_patterns = vec![
            (r"(?i)test|example|demo|fake", "Test or example signature"),
            (r"(?i)admin|root|system", "System-level signature"),
            (r"(?i)bot|automated|ci", "Automated signature"),
        ];

        let name = signature.name().unwrap_or("");
        let email = signature.email().unwrap_or("");

        for (pattern, description) in suspicious_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(name) || regex.is_match(email) {
                    result.add_warning(format!("Suspicious pattern in signature: {}", description));
                }
            }
        }

        Ok(())
    }

    /// Validate signature against known keys
    async fn validate_signature_against_known_keys(
        &self,
        signature: &Signature<'_>,
        result: &mut SignatureValidationResult,
    ) -> RhemaResult<()> {
        // Load known keys from configuration
        let known_keys = self.load_known_keys().await?;
        
        if let Some(email) = signature.email() {
            if let Some(key_info) = known_keys.get(email) {
                result.add_info(format!("Signature from known key: {}", key_info.name));
                result.known_key = true;
                
                // Check if key is still valid
                if let Some(expiry) = key_info.expiry {
                    let now = chrono::Utc::now();
                    if now > expiry {
                        result.add_error("Known key has expired".to_string());
                        result.key_valid = false;
                    } else {
                        result.add_info("Known key is still valid".to_string());
                        result.key_valid = true;
                    }
                }
            } else {
                result.add_warning("Signature from unknown key".to_string());
                result.known_key = false;
            }
        }

        Ok(())
    }

    /// Validate signature timestamp
    fn validate_signature_timestamp(
        &self,
        signature: &Signature<'_>,
        result: &mut SignatureValidationResult,
    ) -> RhemaResult<()> {
        let now = chrono::Utc::now().timestamp();
        let sig_time = signature.when().seconds();
        let time_diff = (now - sig_time).abs();

        // Check for future timestamps (clock skew or malicious)
        if sig_time > now {
            result.add_error("Signature timestamp is in the future".to_string());
            result.timestamp_valid = false;
        }
        // Check for very old signatures
        else if time_diff > 86400 * 365 * 2 { // 2 years
            result.add_warning("Signature is very old (over 2 years)".to_string());
            result.timestamp_valid = true; // Still valid, just old
        }
        // Check for very recent signatures (potential replay)
        else if time_diff < 60 { // 1 minute
            result.add_warning("Signature is very recent (potential replay attack)".to_string());
            result.timestamp_valid = true;
        }
        else {
            result.add_info("Signature timestamp is reasonable".to_string());
            result.timestamp_valid = true;
        }

        result.signature_timestamp = sig_time;
        Ok(())
    }

    /// Check for signature replay attacks
    fn check_for_signature_replay(
        &self,
        signature: &Signature<'_>,
        result: &mut SignatureValidationResult,
    ) -> RhemaResult<()> {
        // Check if this signature has been used before
        let signature_hash = self.calculate_signature_hash(signature);
        
        if let Ok(replay_cache) = self.load_replay_cache() {
            if replay_cache.contains(&signature_hash) {
                result.add_error("Potential signature replay attack detected".to_string());
                result.replay_safe = false;
            } else {
                result.add_info("No replay attack detected".to_string());
                result.replay_safe = true;
                
                // Add to replay cache
                self.add_to_replay_cache(&signature_hash)?;
            }
        }

        Ok(())
    }

    /// Calculate signature hash for replay detection
    fn calculate_signature_hash(&self, signature: &Signature<'_>) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(signature.name().unwrap_or("").as_bytes());
        hasher.update(signature.email().unwrap_or("").as_bytes());
        hasher.update(signature.when().seconds().to_string().as_bytes());
        
        format!("{:x}", hasher.finalize())
    }

    /// Load known keys from configuration
    async fn load_known_keys(&self) -> RhemaResult<HashMap<String, KnownKey>> {
        let mut keys = HashMap::new();
        
        // Load from configuration file
        let config_path = self.repo.path().join(".rhema").join("security").join("known-keys.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let key_list: Vec<KnownKey> = serde_json::from_str(&content)?;
            
            for key in key_list {
                keys.insert(key.email.clone(), key);
            }
        }
        
        Ok(keys)
    }

    /// Load replay cache
    fn load_replay_cache(&self) -> RhemaResult<Vec<String>> {
        let cache_path = self.repo.path().join(".rhema").join("security").join("replay-cache.json");
        if cache_path.exists() {
            let content = std::fs::read_to_string(&cache_path)?;
            let cache: Vec<String> = serde_json::from_str(&content)?;
            Ok(cache)
        } else {
            Ok(Vec::new())
        }
    }

    /// Add signature to replay cache
    fn add_to_replay_cache(&self, signature_hash: &str) -> RhemaResult<()> {
        let cache_path = self.repo.path().join(".rhema").join("security").join("replay-cache.json");
        let mut cache = self.load_replay_cache().unwrap_or_default();
        
        cache.push(signature_hash.to_string());
        
        // Keep only last 1000 signatures to prevent cache bloat
        if cache.len() > 1000 {
            cache = cache.into_iter().rev().take(1000).collect();
            cache.reverse();
        }
        
        let content = serde_json::to_string_pretty(&cache)?;
        self.write_file_with_rotation(&cache_path, &content)?;
        
        Ok(())
    }

    /// Enhanced role lookup with caching and validation
    pub async fn lookup_user_role_enhanced(&self, user: &str) -> RhemaResult<UserRole> {
        // Check cache first
        if let Some(cached_role) = self.get_cached_role(user).await? {
            return Ok(cached_role);
        }
        
        // Load roles from configuration
        let roles = self.load_user_roles().await?;
        
        if let Some(role_config) = roles.get(user) {
            let user_role = UserRole {
                username: user.to_string(),
                role: role_config.role.clone(),
                permissions: role_config.permissions.clone(),
                groups: role_config.groups.clone(),
                last_updated: chrono::Utc::now(),
                expires_at: role_config.expires_at,
            };
            
            // Cache the role
            self.cache_user_role(&user_role).await?;
            
            Ok(user_role)
        } else {
            // Return default role
            Ok(UserRole {
                username: user.to_string(),
                role: "user".to_string(),
                permissions: vec!["read".to_string()],
                groups: Vec::new(),
                last_updated: chrono::Utc::now(),
                expires_at: None,
            })
        }
    }

    /// Get cached role
    async fn get_cached_role(&self, user: &str) -> RhemaResult<Option<UserRole>> {
        let cache_path = self.repo.path().join(".rhema").join("security").join("role-cache.json");
        if cache_path.exists() {
            let content = std::fs::read_to_string(&cache_path)?;
            let cache: HashMap<String, CachedRole> = serde_json::from_str(&content)?;
            
            if let Some(cached) = cache.get(user) {
                // Check if cache is still valid
                let now = chrono::Utc::now();
                if now < cached.expires_at {
                    return Ok(Some(cached.role.clone()));
                }
            }
        }
        
        Ok(None)
    }

    /// Cache user role
    async fn cache_user_role(&self, role: &UserRole) -> RhemaResult<()> {
        let cache_path = self.repo.path().join(".rhema").join("security").join("role-cache.json");
        let mut cache = self.load_role_cache().await?;
        
        let cached_role = CachedRole {
            role: role.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1), // Cache for 1 hour
        };
        
        cache.insert(role.username.clone(), cached_role);
        
        // Keep cache size manageable
        if cache.len() > 100 {
            let mut entries: Vec<_> = cache.into_iter().collect();
            entries.sort_by(|a, b| a.1.expires_at.cmp(&b.1.expires_at));
            cache = entries.into_iter().take(50).collect();
        }
        
        let content = serde_json::to_string_pretty(&cache)?;
        self.write_file_with_rotation(&cache_path, &content)?;
        
        Ok(())
    }

    /// Load role cache
    async fn load_role_cache(&self) -> RhemaResult<HashMap<String, CachedRole>> {
        let cache_path = self.repo.path().join(".rhema").join("security").join("role-cache.json");
        if cache_path.exists() {
            let content = std::fs::read_to_string(&cache_path)?;
            let cache: HashMap<String, CachedRole> = serde_json::from_str(&content)?;
            Ok(cache)
        } else {
            Ok(HashMap::new())
        }
    }

    /// Load user roles from configuration
    async fn load_user_roles(&self) -> RhemaResult<HashMap<String, RoleConfig>> {
        let mut roles = HashMap::new();
        
        // Load from configuration file
        let config_path = self.repo.path().join(".rhema").join("security").join("user-roles.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let role_list: Vec<RoleConfig> = serde_json::from_str(&content)?;
            
            for role in role_list {
                roles.insert(role.username.clone(), role);
            }
        }
        
        Ok(roles)
    }

    /// Write file with rotation to prevent corruption
    pub fn write_file_with_rotation(&self, file_path: &Path, content: &str) -> RhemaResult<()> {
        // Create backup directory
        let backup_dir = file_path.parent().unwrap().join("backups");
        std::fs::create_dir_all(&backup_dir)?;
        
        // Create backup filename with timestamp
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let file_name = file_path.file_name().unwrap().to_string_lossy();
        let backup_path = backup_dir.join(format!("{}.{}", file_name, timestamp));
        
        // Write to temporary file first
        let temp_path = file_path.with_extension("tmp");
        std::fs::write(&temp_path, content)?;
        
        // Create backup of existing file if it exists
        if file_path.exists() {
            std::fs::copy(file_path, &backup_path)?;
        }
        
        // Atomic move from temp to final location
        std::fs::rename(&temp_path, file_path)?;
        
        // Clean up old backups (keep last 10)
        self.cleanup_old_backups(&backup_dir, 10)?;
        
        Ok(())
    }

    /// Clean up old backup files
    fn cleanup_old_backups(&self, backup_dir: &Path, keep_count: usize) -> RhemaResult<()> {
        let mut entries: Vec<_> = std::fs::read_dir(backup_dir)?
            .filter_map(|entry| entry.ok())
            .collect();
        
        // Sort by modification time (oldest first)
        entries.sort_by(|a, b| {
            let a_time = a.metadata().unwrap().modified().unwrap();
            let b_time = b.metadata().unwrap().modified().unwrap();
            a_time.cmp(&b_time)
        });
        
        // Remove old files
        if entries.len() > keep_count {
            let entries_len = entries.len();
            for entry in entries.into_iter().take(entries_len - keep_count) {
                std::fs::remove_file(entry.path())?;
            }
        }
        
        Ok(())
    }

    /// Enhanced secret detection with machine learning
    pub async fn detect_secrets_enhanced(&self, content: &str, file_path: &Path) -> RhemaResult<SecretDetectionResult> {
        let mut result = SecretDetectionResult::new();
        
        // Pattern-based detection
        let patterns = self.load_secret_patterns().await?;
        for pattern in &patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                for cap in regex.find_iter(content) {
                    result.add_secret(Secret {
                        pattern: pattern.name.clone(),
                        value: cap.as_str().to_string(),
                        line_number: self.find_line_number(content, cap.start()),
                        confidence: pattern.confidence,
                        severity: pattern.severity.clone(),
                    });
                }
            }
        }
        
        // ML-based detection (if enabled)
        if self.config.threat_detection.enabled {
            self.detect_secrets_ml(content, file_path, &mut result).await;
        }
        
        // Context-aware detection
        self.detect_secrets_context_aware(content, file_path, &mut result)?;
        
        // Validate detected secrets
        self.validate_detected_secrets(&mut result).await?;
        
        Ok(result)
    }

    /// Load secret patterns from configuration
    async fn load_secret_patterns(&self) -> RhemaResult<Vec<SecretPattern>> {
        let mut patterns = Vec::new();
        
        // Default patterns
        patterns.push(SecretPattern {
            name: "api_key".to_string(),
            pattern: r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*['"][a-zA-Z0-9]{20,}['"]"#.to_string(),
            confidence: 0.9,
            severity: "high".to_string(),
        });
        
        patterns.push(SecretPattern {
            name: "password".to_string(),
            pattern: r#"(?i)(password|passwd|pwd)\s*[:=]\s*['"][^'"]{8,}['"]"#.to_string(),
            confidence: 0.8,
            severity: "high".to_string(),
        });
        
        patterns.push(SecretPattern {
            name: "token".to_string(),
            pattern: r#"(?i)(token|access_token|bearer)\s*[:=]\s*['"][a-zA-Z0-9]{20,}['"]"#.to_string(),
            confidence: 0.9,
            severity: "high".to_string(),
        });
        
        // Load custom patterns from configuration
        let config_path = self.repo.path().join(".rhema").join("security").join("secret-patterns.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let custom_patterns: Vec<SecretPattern> = serde_json::from_str(&content)?;
            patterns.extend(custom_patterns);
        }
        
        Ok(patterns)
    }

    /// Find line number for a character position
    fn find_line_number(&self, content: &str, position: usize) -> usize {
        content[..position].lines().count() + 1
    }

    /// ML-based secret detection
    async fn detect_secrets_ml(&self, content: &str, file_path: &Path, result: &mut SecretDetectionResult) {
        // This would integrate with a machine learning model
        // For now, we'll use a simple heuristic approach
        
        let words: Vec<&str> = content.split_whitespace().collect();
        let suspicious_words = vec!["secret", "key", "token", "password", "credential"];
        
        for (i, word) in words.iter().enumerate() {
            let lower_word = word.to_lowercase();
            for suspicious in &suspicious_words {
                if lower_word.contains(suspicious) && word.len() > 20 {
                    result.add_secret(Secret {
                        pattern: "ml_detected".to_string(),
                        value: word.to_string(),
                        line_number: self.find_line_number(content, content.find(word).unwrap_or(0)),
                        confidence: 0.7,
                        severity: "medium".to_string(),
                    });
                }
            }
        }
    }

    /// Context-aware secret detection
    fn detect_secrets_context_aware(&self, content: &str, file_path: &Path, result: &mut SecretDetectionResult) -> RhemaResult<()> {
        let file_extension = file_path.extension().unwrap_or_else(|| std::ffi::OsStr::new(""));
        let file_name = file_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("")).to_string_lossy();
        
        // Adjust confidence based on file context
        for secret in &mut result.secrets {
            // Higher confidence for config files
            if file_extension == "env" || file_extension == "config" || file_extension == "conf" {
                secret.confidence = (secret.confidence * 1.2).min(1.0);
            }
            
            // Lower confidence for test files
            if file_name.contains("test") || file_name.contains("spec") || file_name.contains("mock") {
                secret.confidence = secret.confidence * 0.8;
            }
            
            // Higher confidence for files with sensitive names
            if file_name.contains("secret") || file_name.contains("key") || file_name.contains("credential") {
                secret.confidence = (secret.confidence * 1.3).min(1.0);
            }
        }
        
        Ok(())
    }

    /// Validate detected secrets
    async fn validate_detected_secrets(&self, result: &mut SecretDetectionResult) -> RhemaResult<()> {
        // Remove low-confidence detections
        result.secrets.retain(|secret| secret.confidence > 0.5);
        
        // Remove known false positives
        let false_positives = self.load_false_positives().await?;
        result.secrets.retain(|secret| {
            !false_positives.iter().any(|fp| {
                fp.pattern == secret.pattern && fp.value == secret.value
            })
        });
        
        // Update severity based on confidence
        for secret in &mut result.secrets {
            if secret.confidence > 0.9 {
                secret.severity = "critical".to_string();
            } else if secret.confidence > 0.7 {
                secret.severity = "high".to_string();
            } else if secret.confidence > 0.5 {
                secret.severity = "medium".to_string();
            }
        }
        
        Ok(())
    }

    /// Load false positives
    async fn load_false_positives(&self) -> RhemaResult<Vec<FalsePositive>> {
        let config_path = self.repo.path().join(".rhema").join("security").join("false-positives.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let false_positives: Vec<FalsePositive> = serde_json::from_str(&content)?;
            Ok(false_positives)
        } else {
            Ok(Vec::new())
        }
    }
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(log_file: PathBuf, log_level: LogLevel) -> RhemaResult<Self> {
        // Ensure log directory exists
        if let Some(parent) = log_file.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self {
            log_file,
            log_level,
        })
    }

    /// Log an audit event
    pub fn log(&self, event: AuditEvent, message: &str, level: LogLevel) -> RhemaResult<()> {
        let timestamp = Utc::now();
        let log_entry = format!(
            "[{}] {:?} - {} - {}\n",
            timestamp.format("%Y-%m-%d %H:%M:%S"),
            level,
            format!("{:?}", event),
            message
        );

        // Write to log file
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;
        
        file.write_all(log_entry.as_bytes())?;

        Ok(())
    }
}

/// Security validation result
#[derive(Debug, Clone)]
pub struct SecurityValidationResult {
    pub is_valid: bool,
    pub risk_level: String,
    pub issues: Vec<SecurityIssue>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub info: Vec<String>,
}

impl SecurityValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            risk_level: "low".to_string(),
            issues: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }

    pub fn add_issue(&mut self, issue: SecurityIssue) {
        let severity = issue.severity.clone();
        self.issues.push(issue);
        if severity == "high" || severity == "critical" {
            self.is_valid = false;
        }
    }

    pub fn set_risk_level(&mut self, risk_level: String) {
        self.risk_level = risk_level;
    }
}

#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: String,
    pub category: String,
    pub description: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<u32>,
}

/// Security scan result
#[derive(Debug, Clone)]
pub struct SecurityScanResult {
    pub clean: bool,
    pub risk_level: String,
    pub scan_duration: std::time::Duration,
    pub issues: Vec<SecurityIssue>,
    pub vulnerabilities: Vec<String>,
    pub malware: Vec<String>,
    pub secrets: Vec<String>,
    pub info: Vec<String>,
}

impl SecurityScanResult {
    pub fn new() -> Self {
        Self {
            clean: true,
            risk_level: "low".to_string(),
            scan_duration: std::time::Duration::from_secs(0),
            issues: Vec::new(),
            vulnerabilities: Vec::new(),
            malware: Vec::new(),
            secrets: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn add_vulnerability(&mut self, vulnerability: String) {
        self.vulnerabilities.push(vulnerability);
        self.clean = false;
    }

    pub fn add_malware(&mut self, malware: String) {
        self.malware.push(malware);
        self.clean = false;
    }

    pub fn add_secret(&mut self, secret: String) {
        self.secrets.push(secret);
        self.clean = false;
    }

    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }

    pub fn add_issue(&mut self, issue: SecurityIssue) {
        self.issues.push(issue);
        self.clean = false;
    }

    pub fn set_risk_level(&mut self, risk_level: String) {
        self.risk_level = risk_level;
    }

    pub fn set_scan_duration(&mut self, duration: std::time::Duration) {
        self.scan_duration = duration;
    }
}

/// Signature validation result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureValidationResult {
    pub signature_valid: bool,
    pub metadata_valid: bool,
    pub key_valid: bool,
    pub timestamp_valid: bool,
    pub replay_safe: bool,
    pub known_key: bool,
    pub signature_timestamp: i64,
    pub info: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl SignatureValidationResult {
    pub fn new() -> Self {
        Self {
            signature_valid: false,
            metadata_valid: true,
            key_valid: true,
            timestamp_valid: true,
            replay_safe: true,
            known_key: false,
            signature_timestamp: 0,
            info: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

/// Known key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownKey {
    pub name: String,
    pub email: String,
    pub key_id: String,
    pub expiry: Option<chrono::DateTime<chrono::Utc>>,
    pub trust_level: String,
}

/// User role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub groups: Vec<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Cached role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRole {
    pub role: UserRole,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Role configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub groups: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Secret detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretDetectionResult {
    pub secrets: Vec<Secret>,
    pub total_secrets: usize,
    pub high_confidence_secrets: usize,
    pub medium_confidence_secrets: usize,
    pub low_confidence_secrets: usize,
}

impl SecretDetectionResult {
    pub fn new() -> Self {
        Self {
            secrets: Vec::new(),
            total_secrets: 0,
            high_confidence_secrets: 0,
            medium_confidence_secrets: 0,
            low_confidence_secrets: 0,
        }
    }

    pub fn add_secret(&mut self, secret: Secret) {
        self.secrets.push(secret);
        self.total_secrets = self.secrets.len();
        
        // Update confidence counts
        self.high_confidence_secrets = self.secrets.iter().filter(|s| s.confidence > 0.8).count();
        self.medium_confidence_secrets = self.secrets.iter().filter(|s| s.confidence > 0.5 && s.confidence <= 0.8).count();
        self.low_confidence_secrets = self.secrets.iter().filter(|s| s.confidence <= 0.5).count();
    }
}

/// Detected secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub pattern: String,
    pub value: String,
    pub line_number: usize,
    pub confidence: f64,
    pub severity: String,
}

/// Secret pattern for detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretPattern {
    pub name: String,
    pub pattern: String,
    pub confidence: f64,
    pub severity: String,
}

/// False positive entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalsePositive {
    pub pattern: String,
    pub value: String,
    pub reason: String,
    pub added_by: String,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Default security configuration
pub fn default_security_config() -> SecurityConfig {
    SecurityConfig {
        enabled: true,
        access_control: AccessControlConfig {
            require_authentication: true,
            rbac_enabled: true,
            roles: HashMap::new(),
            branch_protection: HashMap::new(),
            file_access_control: FileAccessControl {
                sensitive_files: vec![
                    "*.key".to_string(),
                    "*.pem".to_string(),
                    "secrets/*".to_string(),
                ],
                read_only_files: vec!["*.md".to_string(), "docs/*".to_string()],
                admin_only_files: vec!["admin/*".to_string()],
                encryption_required: vec!["secrets/*".to_string()],
            },
        },
        audit_logging: AuditLoggingConfig {
            enabled: true,
            log_file: PathBuf::from(".rhema/security/audit.log"),
            log_level: LogLevel::Info,
            events: vec![
                AuditEvent::Commit,
                AuditEvent::Push,
                AuditEvent::SecurityViolation,
            ],
            retention: RetentionPolicy {
                retention_days: 90,
                max_file_size_mb: 100,
                archive_old_logs: true,
                archive_directory: Some(PathBuf::from(".rhema/security/archive")),
            },
        },
        validation: SecurityValidationConfig {
            validate_signatures: true,
            check_suspicious_patterns: true,
            validate_permissions: true,
            check_secrets: true,
            validate_dependencies: true,
            security_scanning: SecurityScanningConfig {
                enabled: true,
                scan_vulnerabilities: true,
                scan_malware: true,
                scan_secrets: true,
                custom_rules: Vec::new(),
            },
        },
        encryption: EncryptionConfig {
            enabled: false,
            algorithm: EncryptionAlgorithm::AES256,
            key_management: KeyManagementConfig {
                key_storage: KeyStorage::File(PathBuf::from(".rhema/security/keys")),
                key_rotation: KeyRotationPolicy {
                    rotation_days: 365,
                    auto_rotate: true,
                    notify_before_rotation: true,
                },
                backup_keys: true,
            },
            encrypt_sensitive_files: true,
            encrypt_context_files: false,
        },
        threat_detection: ThreatDetectionConfig {
            enabled: false,
            rules: Vec::new(),
            ml_models: Vec::new(),
            alerting: AlertingConfig {
                email: None,
                slack: None,
                webhook: None,
                severity_levels: vec![SecuritySeverity::High, SecuritySeverity::Critical],
            },
        },
    }
}
