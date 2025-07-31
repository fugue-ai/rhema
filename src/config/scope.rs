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

use super::{Config, ConfigAuditLog, ConfigHealth, ConfigStats, CURRENT_CONFIG_VERSION};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use validator::Validate;
use crate::RhemaResult;
use std::path::PathBuf;

/// Scope configuration for Rhema CLI
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ScopeConfig {
    /// Configuration version
    #[validate(length(min = 1))]
    pub version: String,
    
    /// Scope information
    pub scope: ScopeInfo,
    
    /// Scope settings
    pub settings: ScopeSettings,
    
    /// Dependencies configuration
    pub dependencies: DependenciesConfig,
    
    /// Protocol configuration
    pub protocol: ProtocolConfig,
    
    /// Content configuration
    pub content: ContentConfig,
    
    /// Security configuration
    pub security: ScopeSecurityConfig,
    
    /// Custom settings
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
    
    /// Audit log
    pub audit_log: ConfigAuditLog,
    
    /// Health status
    pub health: ConfigHealth,
    
    /// Statistics
    pub stats: ConfigStats,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Scope information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ScopeInfo {
    /// Scope name
    #[validate(length(min = 1))]
    pub name: String,
    
    /// Scope type
    #[validate(length(min = 1))]
    pub scope_type: String,
    
    /// Scope description
    pub description: Option<String>,
    
    /// Scope version
    pub version: String,
    
    /// Scope owner
    pub owner: String,
    
    /// Scope maintainers
    pub maintainers: Vec<String>,
    
    /// Scope tags
    pub tags: Vec<String>,
    
    /// Scope metadata
    pub metadata: HashMap<String, String>,
}

/// Scope settings
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ScopeSettings {
    /// Scope visibility
    pub visibility: ScopeVisibility,
    
    /// Scope access control
    pub access_control: ScopeAccessControl,
    
    /// Scope lifecycle
    pub lifecycle: ScopeLifecycle,
    
    /// Scope governance
    pub governance: ScopeGovernance,
    
    /// Scope quality
    pub quality: ScopeQuality,
}

/// Scope visibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScopeVisibility {
    Public,
    Private,
    Internal,
    Restricted,
}

/// Scope access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeAccessControl {
    /// Read access
    pub read_access: Vec<String>,
    
    /// Write access
    pub write_access: Vec<String>,
    
    /// Admin access
    pub admin_access: Vec<String>,
    
    /// Access policies
    pub policies: Vec<AccessPolicy>,
}

/// Access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy name
    pub name: String,
    
    /// Policy description
    pub description: Option<String>,
    
    /// Policy rules
    pub rules: Vec<AccessRule>,
    
    /// Policy effect
    pub effect: PolicyEffect,
}

/// Access rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRule {
    /// Rule name
    pub name: String,
    
    /// Rule pattern
    pub pattern: String,
    
    /// Rule permissions
    pub permissions: Vec<String>,
    
    /// Rule conditions
    pub conditions: HashMap<String, String>,
}

/// Policy effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
    Warn,
}

/// Scope lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeLifecycle {
    /// Lifecycle stage
    pub stage: LifecycleStage,
    
    /// Lifecycle transitions
    pub transitions: Vec<LifecycleTransition>,
    
    /// Lifecycle rules
    pub rules: Vec<LifecycleRule>,
}

/// Lifecycle stage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifecycleStage {
    Planning,
    Development,
    Testing,
    Staging,
    Production,
    Maintenance,
    Deprecated,
    Archived,
}

/// Lifecycle transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTransition {
    /// Transition name
    pub name: String,
    
    /// From stage
    pub from: LifecycleStage,
    
    /// To stage
    pub to: LifecycleStage,
    
    /// Transition conditions
    pub conditions: Vec<String>,
    
    /// Transition actions
    pub actions: Vec<String>,
}

/// Lifecycle rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Rule conditions
    pub conditions: Vec<String>,
    
    /// Rule actions
    pub actions: Vec<String>,
}

/// Scope governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeGovernance {
    /// Governance model
    pub model: GovernanceModel,
    
    /// Decision making process
    pub decision_making: DecisionMakingProcess,
    
    /// Review process
    pub review_process: ReviewProcess,
    
    /// Compliance requirements
    pub compliance: Vec<ComplianceRequirement>,
}

/// Governance model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceModel {
    Centralized,
    Decentralized,
    Hybrid,
    Consensus,
}

/// Decision making process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMakingProcess {
    /// Decision makers
    pub decision_makers: Vec<String>,
    
    /// Decision criteria
    pub criteria: Vec<String>,
    
    /// Decision timeline
    pub timeline: String,
    
    /// Appeal process
    pub appeal_process: Option<String>,
}

/// Review process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewProcess {
    /// Reviewers
    pub reviewers: Vec<String>,
    
    /// Review criteria
    pub criteria: Vec<String>,
    
    /// Review timeline
    pub timeline: String,
    
    /// Review outcomes
    pub outcomes: Vec<String>,
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    /// Requirement name
    pub name: String,
    
    /// Requirement description
    pub description: String,
    
    /// Requirement type
    pub requirement_type: String,
    
    /// Requirement status
    pub status: ComplianceStatus,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Pending,
    Exempt,
}

/// Scope quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeQuality {
    /// Quality metrics
    pub metrics: Vec<QualityMetric>,
    
    /// Quality gates
    pub gates: Vec<QualityGate>,
    
    /// Quality standards
    pub standards: Vec<String>,
}

/// Quality metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetric {
    /// Metric name
    pub name: String,
    
    /// Metric description
    pub description: String,
    
    /// Metric value
    pub value: f64,
    
    /// Metric threshold
    pub threshold: f64,
    
    /// Metric unit
    pub unit: String,
}

/// Quality gate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    /// Gate name
    pub name: String,
    
    /// Gate description
    pub description: String,
    
    /// Gate conditions
    pub conditions: Vec<String>,
    
    /// Gate status
    pub status: GateStatus,
}

/// Gate status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GateStatus {
    Passed,
    Failed,
    Pending,
    Skipped,
}

/// Dependencies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependenciesConfig {
    /// Dependencies
    pub dependencies: Vec<ScopeDependency>,
    
    /// Dependency resolution
    pub resolution: DependencyResolution,
    
    /// Dependency validation
    pub validation: DependencyValidation,
}

/// Scope dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeDependency {
    /// Dependency path
    pub path: String,
    
    /// Dependency type
    pub dependency_type: DependencyType,
    
    /// Dependency version
    pub version: Option<String>,
    
    /// Dependency description
    pub description: Option<String>,
    
    /// Dependency metadata
    pub metadata: HashMap<String, String>,
}

/// Dependency type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyType {
    Required,
    Optional,
    Peer,
    Development,
    Test,
}

/// Dependency resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyResolution {
    /// Resolution strategy
    pub strategy: ResolutionStrategy,
    
    /// Resolution rules
    pub rules: Vec<ResolutionRule>,
    
    /// Conflict resolution
    pub conflict_resolution: ConflictResolution,
}

/// Resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    Latest,
    Pinned,
    Range,
    Semantic,
}

/// Resolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionRule {
    /// Rule name
    pub name: String,
    
    /// Rule pattern
    pub pattern: String,
    
    /// Rule strategy
    pub strategy: ResolutionStrategy,
    
    /// Rule priority
    pub priority: u32,
}

/// Conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// Resolution method
    pub method: ConflictResolutionMethod,
    
    /// Resolution rules
    pub rules: Vec<ConflictResolutionRule>,
}

/// Conflict resolution method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionMethod {
    Manual,
    Automatic,
    SemiAutomatic,
}

/// Conflict resolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionRule {
    /// Rule name
    pub name: String,
    
    /// Rule pattern
    pub pattern: String,
    
    /// Resolution action
    pub action: String,
}

/// Dependency validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyValidation {
    /// Validation enabled
    pub enabled: bool,
    
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    
    /// Validation severity
    pub severity: ValidationSeverity,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Rule pattern
    pub pattern: String,
    
    /// Rule severity
    pub severity: ValidationSeverity,
}

/// Validation severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    /// Protocol version
    pub version: String,
    
    /// Protocol description
    pub description: Option<String>,
    
    /// Protocol concepts
    pub concepts: Vec<ConceptDefinition>,
    
    /// Protocol examples
    pub examples: Vec<ProtocolExample>,
    
    /// Protocol patterns
    pub patterns: Vec<ProtocolPattern>,
    
    /// Protocol integrations
    pub integrations: Vec<ProtocolIntegration>,
}

/// Concept definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptDefinition {
    /// Concept name
    pub name: String,
    
    /// Concept description
    pub description: String,
    
    /// Related concepts
    pub related: Option<Vec<String>>,
    
    /// Usage examples
    pub examples: Option<Vec<String>>,
}

/// Protocol example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolExample {
    /// Example name
    pub name: String,
    
    /// Example description
    pub description: String,
    
    /// Example query
    pub query: String,
    
    /// Example output
    pub output: Option<String>,
}

/// Protocol pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolPattern {
    /// Pattern name
    pub name: String,
    
    /// Pattern description
    pub description: String,
    
    /// Pattern usage
    pub usage: String,
    
    /// Pattern examples
    pub examples: Vec<String>,
}

/// Protocol integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolIntegration {
    /// Integration name
    pub name: String,
    
    /// Integration description
    pub description: String,
    
    /// Integration setup
    pub setup: Vec<String>,
    
    /// Integration configuration
    pub configuration: HashMap<String, String>,
}

/// Content configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    /// Knowledge configuration
    pub knowledge: KnowledgeConfig,
    
    /// Todos configuration
    pub todos: TodosConfig,
    
    /// Decisions configuration
    pub decisions: DecisionsConfig,
    
    /// Patterns configuration
    pub patterns: PatternsConfig,
}

/// Knowledge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConfig {
    /// Knowledge categories
    pub categories: Vec<String>,
    
    /// Knowledge tags
    pub tags: Vec<String>,
    
    /// Knowledge confidence levels
    pub confidence_levels: Vec<u8>,
    
    /// Knowledge sources
    pub sources: Vec<String>,
    
    /// Knowledge validation
    pub validation: ContentValidation,
}

/// Todos configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodosConfig {
    /// Todo priorities
    pub priorities: Vec<String>,
    
    /// Todo statuses
    pub statuses: Vec<String>,
    
    /// Todo assignees
    pub assignees: Vec<String>,
    
    /// Todo categories
    pub categories: Vec<String>,
    
    /// Todo validation
    pub validation: ContentValidation,
}

/// Decisions configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionsConfig {
    /// Decision statuses
    pub statuses: Vec<String>,
    
    /// Decision makers
    pub decision_makers: Vec<String>,
    
    /// Decision types
    pub types: Vec<String>,
    
    /// Decision validation
    pub validation: ContentValidation,
}

/// Patterns configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternsConfig {
    /// Pattern types
    pub types: Vec<String>,
    
    /// Pattern usage contexts
    pub usage_contexts: Vec<String>,
    
    /// Pattern effectiveness levels
    pub effectiveness_levels: Vec<u8>,
    
    /// Pattern validation
    pub validation: ContentValidation,
}

/// Content validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentValidation {
    /// Validation enabled
    pub enabled: bool,
    
    /// Required fields
    pub required_fields: Vec<String>,
    
    /// Field validation rules
    pub field_rules: HashMap<String, String>,
    
    /// Content validation rules
    pub content_rules: Vec<String>,
}

/// Scope security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeSecurityConfig {
    /// Security scanning
    pub security_scanning: SecurityScanningConfig,
    
    /// Access control
    pub access_control: ScopeAccessControl,
    
    /// Data protection
    pub data_protection: DataProtectionConfig,
    
    /// Audit logging
    pub audit_logging: AuditLoggingConfig,
}

/// Security scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanningConfig {
    /// Scanning enabled
    pub enabled: bool,
    
    /// Scanning tools
    pub tools: Vec<String>,
    
    /// Scanning schedule
    pub schedule: String,
    
    /// Vulnerability thresholds
    pub vulnerability_thresholds: VulnerabilityThresholds,
}

/// Vulnerability thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityThresholds {
    /// Critical threshold
    pub critical: u32,
    
    /// High threshold
    pub high: u32,
    
    /// Medium threshold
    pub medium: u32,
    
    /// Low threshold
    pub low: u32,
}

/// Data protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionConfig {
    /// Data classification
    pub data_classification: DataClassification,
    
    /// Data retention
    pub data_retention: DataRetentionConfig,
    
    /// Data encryption
    pub data_encryption: DataEncryptionConfig,
}

/// Data classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

/// Data retention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    /// Retention period (days)
    pub retention_period: u32,
    
    /// Retention policy
    pub retention_policy: String,
    
    /// Archive policy
    pub archive_policy: Option<String>,
}

/// Data encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEncryptionConfig {
    /// Encryption enabled
    pub enabled: bool,
    
    /// Encryption algorithm
    pub algorithm: String,
    
    /// Key management
    pub key_management: KeyManagementConfig,
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key rotation
    pub key_rotation: bool,
    
    /// Key rotation period (days)
    pub rotation_period: u32,
    
    /// Key storage
    pub key_storage: String,
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggingConfig {
    /// Audit logging enabled
    pub enabled: bool,
    
    /// Audit events
    pub events: Vec<String>,
    
    /// Audit retention
    pub retention: u32,
    
    /// Audit format
    pub format: String,
}

impl ScopeConfig {
    /// Create a new scope configuration
    pub fn new(_scope_path: &Path) -> Self {
        let now = Utc::now();
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            scope: ScopeInfo::default(),
            settings: ScopeSettings::default(),
            dependencies: DependenciesConfig::default(),
            protocol: ProtocolConfig::default(),
            content: ContentConfig::default(),
            security: ScopeSecurityConfig::default(),
            custom: HashMap::new(),
            audit_log: ConfigAuditLog::new(),
            health: ConfigHealth::default(),
            stats: ConfigStats::new(),
            updated_at: now,
        }
    }

    /// Find the scope configuration file in the given directory, checking multiple possible locations
    fn find_config_file(scope_path: &Path) -> Result<PathBuf, crate::RhemaError> {
        // Define the possible locations in order of preference
        let mut all_locations = Vec::new();
        
        // First priority: files in the scope directory itself
        all_locations.push(scope_path.join("scope.yaml"));
        all_locations.push(scope_path.join("rhema.yaml"));
        
        // Second priority: files in parent directory (if we're in a .rhema directory)
        if scope_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
            let parent = scope_path.parent().unwrap_or(scope_path);
            all_locations.push(parent.join("scope.yaml"));
            all_locations.push(parent.join("rhema.yaml"));
        }
        
        // Find the first existing file
        for location in &all_locations {
            if location.exists() {
                return Ok(location.clone());
            }
        }
        
        // If no file found, return error with all checked locations
        let checked_locations = all_locations.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        Err(crate::RhemaError::FileNotFound(
            format!("No scope configuration file found in {} (checked: {})", scope_path.display(), checked_locations)
        ))
    }

    /// Load scope configuration from file
    pub fn load(scope_path: &Path) -> RhemaResult<Self> {
        match Self::find_config_file(scope_path) {
            Ok(config_path) => {
                let content = std::fs::read_to_string(&config_path)
                    .map_err(|e| crate::RhemaError::IoError(e))?;
                
                let config: Self = serde_yaml::from_str(&content)
                    .map_err(|e| crate::RhemaError::InvalidYaml {
                        file: config_path.display().to_string(),
                        message: e.to_string(),
                    })?;
                
                config.validate_config()?;
                Ok(config)
            }
            Err(_) => {
                // Create default configuration
                let config = Self::new(scope_path);
                config.save(scope_path)?;
                Ok(config)
            }
        }
    }

    /// Save scope configuration to file
    pub fn save(&self, scope_path: &Path) -> RhemaResult<()> {
        let config_path = scope_path.join("scope.yaml");
        
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

    /// Update configuration
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get configuration value by path
    pub fn get_value(&self, path: &str) -> Option<serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let value = serde_json::to_value(self).ok()?;
        let mut current = value;
        
        for part in parts {
            current = current.get(part)?.clone();
        }
        
        Some(current)
    }

    /// Set configuration value by path
    pub fn set_value(&mut self, path: &str, value: serde_json::Value) -> RhemaResult<()> {
        if path.starts_with("custom.") {
            let key = path.trim_start_matches("custom.");
            self.custom.insert(key.to_string(), value);
        } else {
            return Err(crate::RhemaError::ConfigError(
                format!("Cannot set value for path: {}", path)
            ));
        }
        
        self.update()
    }

    /// Export configuration
    pub fn export(&self, format: &str) -> RhemaResult<String> {
        match format.to_lowercase().as_str() {
            "yaml" => {
                serde_yaml::to_string(self)
                    .map_err(|e| crate::RhemaError::InvalidYaml {
                        file: "export".to_string(),
                        message: e.to_string(),
                    })
            }
            "json" => {
                serde_json::to_string_pretty(self)
                    .map_err(|e| crate::RhemaError::ConfigError(e.to_string()))
            }
            _ => Err(crate::RhemaError::ConfigError(
                format!("Unsupported export format: {}", format)
            ))
        }
    }

    /// Import configuration
    pub fn import(&mut self, content: &str, format: &str) -> RhemaResult<()> {
        let imported: Self = match format.to_lowercase().as_str() {
            "yaml" => {
                serde_yaml::from_str(content)
                    .map_err(|e| crate::RhemaError::InvalidYaml {
                        file: "import".to_string(),
                        message: e.to_string(),
                    })?
            }
            "json" => {
                serde_json::from_str(content)
                    .map_err(|e| crate::RhemaError::ConfigError(e.to_string()))?
            }
            _ => {
                return Err(crate::RhemaError::ConfigError(
                    format!("Unsupported import format: {}", format)
                ));
            }
        };
        
        imported.validate_config()?;
        *self = imported;
        self.update()
    }
}

impl Config for ScopeConfig {
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
                "scope": {"type": "object"},
                "settings": {"type": "object"},
                "dependencies": {"type": "object"},
                "protocol": {"type": "object"},
                "content": {"type": "object"},
                "security": {"type": "object"}
            },
            "required": ["version", "scope", "settings", "dependencies", "protocol", "content", "security"]
        })
    }
    
    fn documentation() -> &'static str {
        "Scope configuration for Rhema CLI containing scope-specific settings, dependencies, protocol configuration, content settings, and security configuration."
    }
}

// Default implementations
impl Default for ScopeInfo {
    fn default() -> Self {
        Self {
            name: "default-scope".to_string(),
            scope_type: "service".to_string(),
            description: None,
            version: "1.0.0".to_string(),
            owner: "default-owner".to_string(),
            maintainers: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for ScopeSettings {
    fn default() -> Self {
        Self {
            visibility: ScopeVisibility::Private,
            access_control: ScopeAccessControl::default(),
            lifecycle: ScopeLifecycle::default(),
            governance: ScopeGovernance::default(),
            quality: ScopeQuality::default(),
        }
    }
}

impl Default for ScopeAccessControl {
    fn default() -> Self {
        Self {
            read_access: vec!["owner".to_string()],
            write_access: vec!["owner".to_string()],
            admin_access: vec!["owner".to_string()],
            policies: Vec::new(),
        }
    }
}

impl Default for ScopeLifecycle {
    fn default() -> Self {
        Self {
            stage: LifecycleStage::Development,
            transitions: Vec::new(),
            rules: Vec::new(),
        }
    }
}

impl Default for ScopeGovernance {
    fn default() -> Self {
        Self {
            model: GovernanceModel::Centralized,
            decision_making: DecisionMakingProcess::default(),
            review_process: ReviewProcess::default(),
            compliance: Vec::new(),
        }
    }
}

impl Default for DecisionMakingProcess {
    fn default() -> Self {
        Self {
            decision_makers: vec!["owner".to_string()],
            criteria: vec!["Technical feasibility".to_string(), "Business value".to_string()],
            timeline: "1 week".to_string(),
            appeal_process: None,
        }
    }
}

impl Default for ReviewProcess {
    fn default() -> Self {
        Self {
            reviewers: vec!["owner".to_string()],
            criteria: vec!["Code quality".to_string(), "Security".to_string()],
            timeline: "3 days".to_string(),
            outcomes: vec!["Approved".to_string(), "Rejected".to_string(), "Changes requested".to_string()],
        }
    }
}

impl Default for ScopeQuality {
    fn default() -> Self {
        Self {
            metrics: Vec::new(),
            gates: Vec::new(),
            standards: vec!["Code quality standards".to_string()],
        }
    }
}

impl Default for DependenciesConfig {
    fn default() -> Self {
        Self {
            dependencies: Vec::new(),
            resolution: DependencyResolution::default(),
            validation: DependencyValidation::default(),
        }
    }
}

impl Default for DependencyResolution {
    fn default() -> Self {
        Self {
            strategy: ResolutionStrategy::Latest,
            rules: Vec::new(),
            conflict_resolution: ConflictResolution::default(),
        }
    }
}

impl Default for ConflictResolution {
    fn default() -> Self {
        Self {
            method: ConflictResolutionMethod::Manual,
            rules: Vec::new(),
        }
    }
}

impl Default for DependencyValidation {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            severity: ValidationSeverity::Warning,
        }
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            description: None,
            concepts: Vec::new(),
            examples: Vec::new(),
            patterns: Vec::new(),
            integrations: Vec::new(),
        }
    }
}

impl Default for ContentConfig {
    fn default() -> Self {
        Self {
            knowledge: KnowledgeConfig::default(),
            todos: TodosConfig::default(),
            decisions: DecisionsConfig::default(),
            patterns: PatternsConfig::default(),
        }
    }
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            categories: vec!["architecture".to_string(), "design".to_string(), "implementation".to_string()],
            tags: Vec::new(),
            confidence_levels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            sources: vec!["documentation".to_string(), "code".to_string(), "discussion".to_string()],
            validation: ContentValidation::default(),
        }
    }
}

impl Default for TodosConfig {
    fn default() -> Self {
        Self {
            priorities: vec!["low".to_string(), "medium".to_string(), "high".to_string(), "critical".to_string()],
            statuses: vec!["pending".to_string(), "in_progress".to_string(), "completed".to_string(), "cancelled".to_string()],
            assignees: Vec::new(),
            categories: vec!["feature".to_string(), "bug".to_string(), "improvement".to_string()],
            validation: ContentValidation::default(),
        }
    }
}

impl Default for DecisionsConfig {
    fn default() -> Self {
        Self {
            statuses: vec!["proposed".to_string(), "approved".to_string(), "rejected".to_string(), "implemented".to_string()],
            decision_makers: vec!["owner".to_string()],
            types: vec!["architectural".to_string(), "technical".to_string(), "business".to_string()],
            validation: ContentValidation::default(),
        }
    }
}

impl Default for PatternsConfig {
    fn default() -> Self {
        Self {
            types: vec!["architectural".to_string(), "design".to_string(), "implementation".to_string()],
            usage_contexts: vec!["recommended".to_string(), "optional".to_string(), "deprecated".to_string()],
            effectiveness_levels: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            validation: ContentValidation::default(),
        }
    }
}

impl Default for ContentValidation {
    fn default() -> Self {
        Self {
            enabled: true,
            required_fields: vec!["title".to_string(), "description".to_string()],
            field_rules: HashMap::new(),
            content_rules: vec!["Content must be clear and concise".to_string()],
        }
    }
}

impl Default for ScopeSecurityConfig {
    fn default() -> Self {
        Self {
            security_scanning: SecurityScanningConfig::default(),
            access_control: ScopeAccessControl::default(),
            data_protection: DataProtectionConfig::default(),
            audit_logging: AuditLoggingConfig::default(),
        }
    }
}

impl Default for SecurityScanningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tools: vec!["cargo-audit".to_string()],
            schedule: "daily".to_string(),
            vulnerability_thresholds: VulnerabilityThresholds::default(),
        }
    }
}

impl Default for VulnerabilityThresholds {
    fn default() -> Self {
        Self {
            critical: 0,
            high: 0,
            medium: 5,
            low: 10,
        }
    }
}

impl Default for DataProtectionConfig {
    fn default() -> Self {
        Self {
            data_classification: DataClassification::Internal,
            data_retention: DataRetentionConfig::default(),
            data_encryption: DataEncryptionConfig::default(),
        }
    }
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            retention_period: 365,
            retention_policy: "Keep for 1 year".to_string(),
            archive_policy: None,
        }
    }
}

impl Default for DataEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".to_string(),
            key_management: KeyManagementConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            key_rotation: false,
            rotation_period: 90,
            key_storage: "local".to_string(),
        }
    }
}

impl Default for AuditLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            events: vec!["config_change".to_string(), "access".to_string()],
            retention: 90,
            format: "json".to_string(),
        }
    }
}

// ConfigHealth Default implementation moved to mod.rs to avoid conflicts 