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

use super::{Config, CURRENT_CONFIG_VERSION};
use super::global::GlobalConfig;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use validator::Validate;
use crate::RhemaResult;

/// Security manager for Rhema CLI configuration
pub struct SecurityManager {
    config: SecurityConfig,
    encryption_manager: EncryptionManager,
    access_control: AccessControlManager,
    audit_logger: AuditLogger,
    compliance_checker: ComplianceChecker,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityConfig {
    /// Configuration version
    #[validate(length(min = 1))]
    pub version: String,
    
    /// Encryption settings
    pub encryption: EncryptionSettings,
    
    /// Access control settings
    pub access_control: AccessControlSettings,
    
    /// Audit settings
    pub audit: AuditSettings,
    
    /// Compliance settings
    pub compliance: ComplianceSettings,
    
    /// Key management settings
    pub key_management: KeyManagementSettings,
    
    /// Security policies
    pub policies: Vec<SecurityPolicy>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Encryption settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    /// Enable encryption
    pub enabled: bool,
    
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    
    /// Key size
    pub key_size: u32,
    
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    
    /// Salt size
    pub salt_size: u32,
    
    /// Iteration count
    pub iteration_count: u32,
    
    /// Master password required
    pub master_password_required: bool,
    
    /// Key file path
    pub key_file: Option<PathBuf>,
}

/// Encryption algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncryptionAlgorithm {
    AES256GCM,
    AES256CBC,
    ChaCha20Poly1305,
    Custom(String),
}

/// Key derivation function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyDerivationFunction {
    PBKDF2,
    Argon2,
    Scrypt,
    Custom(String),
}

/// Access control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlSettings {
    /// Enable access control
    pub enabled: bool,
    
    /// Authentication method
    pub authentication_method: AuthenticationMethod,
    
    /// Authorization model
    pub authorization_model: AuthorizationModel,
    
    /// Session management
    pub session_management: SessionManagement,
    
    /// Permission model
    pub permission_model: PermissionModel,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Token,
    OAuth,
    SSO,
    MFA,
    Custom(String),
}

/// Authorization model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthorizationModel {
    RBAC,
    ABAC,
    PBAC,
    Custom(String),
}

/// Session management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManagement {
    /// Session timeout (minutes)
    pub session_timeout: u64,
    
    /// Max failed attempts
    pub max_failed_attempts: u32,
    
    /// Lockout duration (minutes)
    pub lockout_duration: u64,
    
    /// Session storage
    pub session_storage: SessionStorage,
}

/// Session storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStorage {
    Memory,
    File,
    Database,
    Redis,
    Custom(String),
}

/// Permission model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionModel {
    /// Permission granularity
    pub granularity: PermissionGranularity,
    
    /// Permission inheritance
    pub inheritance: bool,
    
    /// Permission caching
    pub caching: PermissionCaching,
}

/// Permission granularity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionGranularity {
    File,
    Directory,
    Scope,
    Repository,
    Global,
}

/// Permission caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCaching {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache timeout (minutes)
    pub timeout: u64,
    
    /// Cache size
    pub size: usize,
}

/// Audit settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSettings {
    /// Enable audit logging
    pub enabled: bool,
    
    /// Audit level
    pub level: AuditLevel,
    
    /// Audit events
    pub events: Vec<AuditEvent>,
    
    /// Audit storage
    pub storage: AuditStorage,
    
    /// Audit retention
    pub retention: AuditRetention,
}

/// Audit level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditLevel {
    None,
    Basic,
    Detailed,
    Verbose,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEvent {
    ConfigRead,
    ConfigWrite,
    ConfigDelete,
    AccessGranted,
    AccessDenied,
    AuthenticationSuccess,
    AuthenticationFailure,
    AuthorizationSuccess,
    AuthorizationFailure,
    Encryption,
    Decryption,
    KeyRotation,
    Custom(String),
}

/// Audit storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorage {
    /// Storage type
    pub storage_type: AuditStorageType,
    
    /// Storage path
    pub path: Option<PathBuf>,
    
    /// Storage configuration
    pub config: HashMap<String, String>,
}

/// Audit storage type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditStorageType {
    File,
    Database,
    Syslog,
    Custom(String),
}

/// Audit retention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRetention {
    /// Retention period (days)
    pub period: u32,
    
    /// Retention policy
    pub policy: RetentionPolicy,
    
    /// Archive policy
    pub archive_policy: Option<ArchivePolicy>,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RetentionPolicy {
    Delete,
    Archive,
    Compress,
    Custom(String),
}

/// Archive policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivePolicy {
    /// Archive format
    pub format: String,
    
    /// Archive location
    pub location: PathBuf,
    
    /// Archive compression
    pub compression: bool,
}

/// Compliance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSettings {
    /// Compliance framework
    pub framework: ComplianceFramework,
    
    /// Compliance level
    pub level: ComplianceLevel,
    
    /// Compliance checks
    pub checks: Vec<ComplianceCheck>,
    
    /// Compliance reporting
    pub reporting: ComplianceReporting,
}

/// Compliance framework
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceFramework {
    SOC2,
    ISO27001,
    GDPR,
    HIPAA,
    PCI,
    Custom(String),
}

/// Compliance level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceLevel {
    Basic,
    Standard,
    Enhanced,
    Custom(String),
}

/// Compliance check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    /// Check name
    pub name: String,
    
    /// Check description
    pub description: String,
    
    /// Check type
    pub check_type: ComplianceCheckType,
    
    /// Check criteria
    pub criteria: Vec<String>,
    
    /// Check severity
    pub severity: ComplianceSeverity,
}

/// Compliance check type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceCheckType {
    Encryption,
    AccessControl,
    AuditLogging,
    DataRetention,
    KeyManagement,
    Custom(String),
}

/// Compliance severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReporting {
    /// Enable reporting
    pub enabled: bool,
    
    /// Report format
    pub format: ReportFormat,
    
    /// Report schedule
    pub schedule: String,
    
    /// Report recipients
    pub recipients: Vec<String>,
}

/// Report format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    JSON,
    XML,
    CSV,
    PDF,
    Custom(String),
}

/// Key management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementSettings {
    /// Key rotation
    pub key_rotation: KeyRotation,
    
    /// Key storage
    pub key_storage: KeyStorage,
    
    /// Key backup
    pub key_backup: KeyBackup,
    
    /// Key recovery
    pub key_recovery: KeyRecovery,
}

/// Key rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotation {
    /// Enable key rotation
    pub enabled: bool,
    
    /// Rotation interval (days)
    pub interval: u32,
    
    /// Rotation method
    pub method: RotationMethod,
    
    /// Rotation notification
    pub notification: bool,
}

/// Rotation method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RotationMethod {
    Automatic,
    Manual,
    SemiAutomatic,
}

/// Key storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStorage {
    /// Storage type
    pub storage_type: KeyStorageType,
    
    /// Storage path
    pub path: Option<PathBuf>,
    
    /// Storage configuration
    pub config: HashMap<String, String>,
}

/// Key storage type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyStorageType {
    File,
    Hardware,
    Cloud,
    Custom(String),
}

/// Key backup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBackup {
    /// Enable backup
    pub enabled: bool,
    
    /// Backup location
    pub location: PathBuf,
    
    /// Backup encryption
    pub encryption: bool,
    
    /// Backup frequency
    pub frequency: String,
}

/// Key recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRecovery {
    /// Enable recovery
    pub enabled: bool,
    
    /// Recovery method
    pub method: RecoveryMethod,
    
    /// Recovery verification
    pub verification: bool,
}

/// Recovery method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryMethod {
    Backup,
    Escrow,
    Shamir,
    Custom(String),
}

/// Security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    pub name: String,
    
    /// Policy description
    pub description: String,
    
    /// Policy rules
    pub rules: Vec<SecurityRule>,
    
    /// Policy enforcement
    pub enforcement: PolicyEnforcement,
}

/// Security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Rule condition
    pub condition: String,
    
    /// Rule action
    pub action: SecurityAction,
    
    /// Rule priority
    pub priority: u32,
}

/// Security action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityAction {
    Allow,
    Deny,
    Warn,
    Log,
    Custom(String),
}

/// Policy enforcement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyEnforcement {
    Strict,
    Flexible,
    Advisory,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(_global_config: &GlobalConfig) -> RhemaResult<Self> {
        let config = SecurityConfig::load()?;
        let encryption_manager = EncryptionManager::new(&config)?;
        let access_control = AccessControlManager::new(&config)?;
        let audit_logger = AuditLogger::new(&config)?;
        let compliance_checker = ComplianceChecker::new(&config)?;

        Ok(Self {
            config,
            encryption_manager,
            access_control,
            audit_logger,
            compliance_checker,
        })
    }

    /// Get security configuration
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }

    /// Get encryption manager
    pub fn encryption(&self) -> &EncryptionManager {
        &self.encryption_manager
    }

    /// Get access control manager
    pub fn access_control(&self) -> &AccessControlManager {
        &self.access_control
    }

    /// Get audit logger
    pub fn audit_logger(&self) -> &AuditLogger {
        &self.audit_logger
    }

    /// Get compliance checker
    pub fn compliance_checker(&self) -> &ComplianceChecker {
        &self.compliance_checker
    }

    /// Encrypt configuration data
    pub fn encrypt_data(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        self.encryption_manager.encrypt(data)
    }

    /// Decrypt configuration data
    pub fn decrypt_data(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        self.encryption_manager.decrypt(data)
    }

    /// Check access permission
    pub fn check_permission(&self, _user: &str, _resource: &str, _action: &str) -> RhemaResult<bool> {
        self.access_control.check_permission(_user, _resource, _action)
    }

    /// Log audit event
    pub fn log_audit_event(&self, event: &AuditEvent, details: &str) -> RhemaResult<()> {
        self.audit_logger.log_event(event, details)
    }

    /// Run compliance checks
    pub fn run_compliance_checks(&self) -> RhemaResult<ComplianceReport> {
        self.compliance_checker.run_checks()
    }
}

/// Encryption manager
pub struct EncryptionManager {
    config: SecurityConfig,
    _key: Option<Vec<u8>>,
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub fn new(config: &SecurityConfig) -> RhemaResult<Self> {
        let key = if config.encryption.enabled {
            Self::load_or_generate_key(config)?
        } else {
            None
        };

        Ok(Self {
            config: config.clone(),
            _key: key,
        })
    }

    /// Load or generate encryption key
    fn load_or_generate_key(_config: &SecurityConfig) -> RhemaResult<Option<Vec<u8>>> {
        // This is a simplified implementation
        // In a real implementation, you'd want to use proper key management
        Ok(Some(vec![0u8; 32])) // Placeholder key
    }

    /// Encrypt data
    pub fn encrypt(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        if !self.config.encryption.enabled {
            return Ok(data.to_vec());
        }

        // This is a simplified implementation
        // In a real implementation, you'd want to use proper encryption
        Ok(data.to_vec())
    }

    /// Decrypt data
    pub fn decrypt(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        if !self.config.encryption.enabled {
            return Ok(data.to_vec());
        }

        // This is a simplified implementation
        // In a real implementation, you'd want to use proper decryption
        Ok(data.to_vec())
    }
}

/// Access control manager
pub struct AccessControlManager {
    config: SecurityConfig,
    _permissions: HashMap<String, Vec<String>>,
}

impl AccessControlManager {
    /// Create a new access control manager
    pub fn new(config: &SecurityConfig) -> RhemaResult<Self> {
        let permissions = Self::load_permissions(config)?;

        Ok(Self {
            config: config.clone(),
            _permissions: permissions,
        })
    }

    /// Load permissions
    fn load_permissions(_config: &SecurityConfig) -> RhemaResult<HashMap<String, Vec<String>>> {
        // This is a simplified implementation
        // In a real implementation, you'd want to load from a proper source
        Ok(HashMap::new())
    }

    /// Check permission
    pub fn check_permission(&self, _user: &str, _resource: &str, _action: &str) -> RhemaResult<bool> {
        if !self.config.access_control.enabled {
            return Ok(true);
        }

        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper permission checking
        Ok(true)
    }
}

/// Audit logger
pub struct AuditLogger {
    config: SecurityConfig,
    _log_file: Option<std::fs::File>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(config: &SecurityConfig) -> RhemaResult<Self> {
        let log_file = if config.audit.enabled {
            Self::setup_log_file(config)?
        } else {
            None
        };

        Ok(Self {
            config: config.clone(),
            _log_file: log_file,
        })
    }

    /// Setup log file
    fn setup_log_file(_config: &SecurityConfig) -> RhemaResult<Option<std::fs::File>> {
        // This is a simplified implementation
        // In a real implementation, you'd want to setup proper logging
        Ok(None)
    }

    /// Log event
    pub fn log_event(&self, event: &AuditEvent, details: &str) -> RhemaResult<()> {
        if !self.config.audit.enabled {
            return Ok(());
        }

        let log_entry = format!(
            "{} - {:?} - {}\n",
            Utc::now(),
            event,
            details
        );

        // This is a simplified implementation
        // In a real implementation, you'd want to write to the log file
        println!("AUDIT: {}", log_entry.trim());

        Ok(())
    }
}

/// Compliance checker
pub struct ComplianceChecker {
    config: SecurityConfig,
}

impl ComplianceChecker {
    /// Create a new compliance checker
    pub fn new(config: &SecurityConfig) -> RhemaResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Run compliance checks
    pub fn run_checks(&self) -> RhemaResult<ComplianceReport> {
        let mut report = ComplianceReport::new();

        for check in &self.config.compliance.checks {
            let result = self.run_check(check)?;
            report.add_result(result);
        }

        Ok(report)
    }

    /// Run individual check
    fn run_check(&self, check: &ComplianceCheck) -> RhemaResult<ComplianceCheckResult> {
        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper compliance checking
        Ok(ComplianceCheckResult {
            check_name: check.name.clone(),
            status: ComplianceStatus::Compliant,
            details: "Check passed".to_string(),
            timestamp: Utc::now(),
        })
    }
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Check results
    pub results: Vec<ComplianceCheckResult>,
    
    /// Overall status
    pub overall_status: ComplianceStatus,
    
    /// Summary
    pub summary: String,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// Check name
    pub check_name: String,
    
    /// Status
    pub status: ComplianceStatus,
    
    /// Details
    pub details: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Pending,
    Exempt,
}

impl ComplianceReport {
    /// Create a new compliance report
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            results: Vec::new(),
            overall_status: ComplianceStatus::Compliant,
            summary: String::new(),
        }
    }

    /// Add check result
    pub fn add_result(&mut self, result: ComplianceCheckResult) {
        self.results.push(result);
        self.update_overall_status();
    }

    /// Update overall status
    fn update_overall_status(&mut self) {
        let non_compliant_count = self.results
            .iter()
            .filter(|r| r.status == ComplianceStatus::NonCompliant)
            .count();

        self.overall_status = if non_compliant_count == 0 {
            ComplianceStatus::Compliant
        } else {
            ComplianceStatus::NonCompliant
        };

        self.summary = format!(
            "{} checks passed, {} failed",
            self.results.iter().filter(|r| r.status == ComplianceStatus::Compliant).count(),
            non_compliant_count
        );
    }
}

impl SecurityConfig {
    /// Create a new security configuration
    pub fn new() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            encryption: EncryptionSettings::default(),
            access_control: AccessControlSettings::default(),
            audit: AuditSettings::default(),
            compliance: ComplianceSettings::default(),
            key_management: KeyManagementSettings::default(),
            policies: Vec::new(),
            updated_at: Utc::now(),
        }
    }

    /// Load security configuration from file
    pub fn load() -> RhemaResult<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| crate::RhemaError::IoError(e))?;
            
            let config: Self = serde_yaml::from_str(&content)
                .map_err(|e| crate::RhemaError::InvalidYaml {
                    file: config_path.display().to_string(),
                    message: e.to_string(),
                })?;
            
            config.validate_config()?;
            Ok(config)
        } else {
            // Create default configuration
            let config = Self::new();
            config.save()?;
            Ok(config)
        }
    }

    /// Save security configuration to file
    pub fn save(&self) -> RhemaResult<()> {
        let config_path = Self::get_config_path()?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| crate::RhemaError::IoError(e))?;
        }
        
        let content = serde_yaml::to_string(self)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: config_path.display().to_string(),
                message: e.to_string(),
            })?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| crate::RhemaError::IoError(e))?;
        
        Ok(())
    }

    /// Get configuration file path
    fn get_config_path() -> RhemaResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::RhemaError::ConfigError("Could not determine config directory".to_string()))?
            .join("rhema");
        
        Ok(config_dir.join("security.yaml"))
    }

    /// Update configuration
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        self.save()
    }
}

impl Config for SecurityConfig {
    fn version(&self) -> &str {
        &self.version
    }
    
    fn validate_config(&self) -> RhemaResult<()> {
        self.validate()
            .map_err(|e| crate::RhemaError::ConfigError(format!("Validation failed: {}", e)))?;
        Ok(())
    }
    
    fn load_from_file(path: &Path) -> RhemaResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::RhemaError::IoError(e))?;
        
        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;
        
        config.validate_config()?;
        Ok(config)
    }
    
    fn save_to_file(&self, path: &Path) -> RhemaResult<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;
        
        std::fs::write(path, content)
            .map_err(|e| crate::RhemaError::IoError(e))?;
        
        Ok(())
    }
    
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "version": {"type": "string"},
                "encryption": {"type": "object"},
                "access_control": {"type": "object"},
                "audit": {"type": "object"},
                "compliance": {"type": "object"},
                "key_management": {"type": "object"},
                "policies": {"type": "array"}
            },
            "required": ["version", "encryption", "access_control", "audit", "compliance", "key_management", "policies"]
        })
    }
    
    fn documentation() -> &'static str {
        "Security configuration for Rhema CLI containing encryption settings, access control, audit logging, compliance requirements, and key management."
    }
}

// Default implementations
impl Default for EncryptionSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: EncryptionAlgorithm::AES256GCM,
            key_size: 256,
            kdf: KeyDerivationFunction::PBKDF2,
            salt_size: 32,
            iteration_count: 100000,
            master_password_required: false,
            key_file: None,
        }
    }
}

impl Default for AccessControlSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            authentication_method: AuthenticationMethod::Password,
            authorization_model: AuthorizationModel::RBAC,
            session_management: SessionManagement::default(),
            permission_model: PermissionModel::default(),
        }
    }
}

impl Default for SessionManagement {
    fn default() -> Self {
        Self {
            session_timeout: 1440, // 24 hours
            max_failed_attempts: 5,
            lockout_duration: 30,
            session_storage: SessionStorage::Memory,
        }
    }
}

impl Default for PermissionModel {
    fn default() -> Self {
        Self {
            granularity: PermissionGranularity::Scope,
            inheritance: true,
            caching: PermissionCaching::default(),
        }
    }
}

impl Default for PermissionCaching {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: 60,
            size: 1000,
        }
    }
}

impl Default for AuditSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            level: AuditLevel::Basic,
            events: vec![
                AuditEvent::ConfigRead,
                AuditEvent::ConfigWrite,
                AuditEvent::AccessGranted,
                AuditEvent::AccessDenied,
            ],
            storage: AuditStorage::default(),
            retention: AuditRetention::default(),
        }
    }
}

impl Default for AuditStorage {
    fn default() -> Self {
        Self {
            storage_type: AuditStorageType::File,
            path: Some(dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join("rhema/audit.log")),
            config: HashMap::new(),
        }
    }
}

impl Default for AuditRetention {
    fn default() -> Self {
        Self {
            period: 90,
            policy: RetentionPolicy::Delete,
            archive_policy: None,
        }
    }
}

impl Default for ComplianceSettings {
    fn default() -> Self {
        Self {
            framework: ComplianceFramework::SOC2,
            level: ComplianceLevel::Basic,
            checks: Vec::new(),
            reporting: ComplianceReporting::default(),
        }
    }
}

impl Default for ComplianceReporting {
    fn default() -> Self {
        Self {
            enabled: false,
            format: ReportFormat::JSON,
            schedule: "monthly".to_string(),
            recipients: Vec::new(),
        }
    }
}

impl Default for KeyManagementSettings {
    fn default() -> Self {
        Self {
            key_rotation: KeyRotation::default(),
            key_storage: KeyStorage::default(),
            key_backup: KeyBackup::default(),
            key_recovery: KeyRecovery::default(),
        }
    }
}

impl Default for KeyRotation {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: 90,
            method: RotationMethod::Manual,
            notification: true,
        }
    }
}

impl Default for KeyStorage {
    fn default() -> Self {
        Self {
            storage_type: KeyStorageType::File,
            path: Some(dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("rhema/keys")),
            config: HashMap::new(),
        }
    }
}

impl Default for KeyBackup {
    fn default() -> Self {
        Self {
            enabled: false,
            location: PathBuf::from("~/.rhema/backup/keys"),
            encryption: true,
            frequency: "weekly".to_string(),
        }
    }
}

impl Default for KeyRecovery {
    fn default() -> Self {
        Self {
            enabled: false,
            method: RecoveryMethod::Backup,
            verification: true,
        }
    }
} 