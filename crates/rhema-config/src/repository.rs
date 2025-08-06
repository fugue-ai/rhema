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


use crate::{Config, ConfigAuditLog, ConfigHealth, ConfigStats, CURRENT_CONFIG_VERSION};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use validator::Validate;
/// Repository configuration for Rhema CLI
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RepositoryConfig {
    /// Configuration version
    #[validate(length(min = 1))]
    pub version: String,

    /// Repository information
    pub repository: RepositoryInfo,

    /// Repository settings
    pub settings: RepositorySettings,

    /// Scope configuration
    pub scopes: ScopeConfig,

    /// Workflow configuration
    pub workflow: WorkflowConfig,

    /// Security configuration
    pub security: RepositorySecurityConfig,

    /// Integration configuration
    pub integrations: RepositoryIntegrationConfig,

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

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RepositoryInfo {
    /// Repository name
    #[validate(length(min = 1))]
    pub name: String,

    /// Repository description
    pub description: Option<String>,

    /// Repository URL
    pub url: Option<String>,

    /// Repository type
    pub repository_type: RepositoryType,

    /// Repository owner
    pub owner: String,

    /// Repository visibility
    pub visibility: RepositoryVisibility,

    /// Repository tags
    pub tags: Vec<String>,

    /// Repository metadata
    pub metadata: HashMap<String, String>,
}

/// Repository type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepositoryType {
    Git,
    SVN,
    Mercurial,
    Other(String),
}

/// Repository visibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepositoryVisibility {
    Public,
    Private,
    Internal,
}

/// Repository settings
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RepositorySettings {
    /// Default branch
    pub default_branch: String,

    /// Branch protection rules
    pub branch_protection: BranchProtectionConfig,

    /// Commit message conventions
    pub commit_conventions: CommitConventions,

    /// Code review settings
    pub code_review: CodeReviewConfig,

    /// Testing settings
    pub testing: TestingConfig,

    /// Documentation settings
    pub documentation: DocumentationConfig,

    /// Deployment settings
    pub deployment: DeploymentConfig,
}

/// Branch protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtectionConfig {
    /// Protected branches
    pub protected_branches: Vec<String>,

    /// Require pull request reviews
    pub require_reviews: bool,

    /// Required reviewers count
    pub required_reviewers: u32,

    /// Require status checks
    pub require_status_checks: bool,

    /// Required status checks
    pub required_status_checks: Vec<String>,

    /// Require up-to-date branches
    pub require_up_to_date: bool,

    /// Allow force pushes
    pub allow_force_push: bool,

    /// Allow deletions
    pub allow_deletions: bool,
}

/// Commit conventions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitConventions {
    /// Conventional commits enabled
    pub conventional_commits: bool,

    /// Commit message template
    pub message_template: Option<String>,

    /// Commit types
    pub types: Vec<String>,

    /// Commit scopes
    pub scopes: Vec<String>,

    /// Commit breaking change format
    pub breaking_change_format: String,

    /// Commit footer format
    pub footer_format: String,
}

/// Code review configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewConfig {
    /// Code review required
    pub required: bool,

    /// Auto-assign reviewers
    pub auto_assign: bool,

    /// Required reviewers
    pub required_reviewers: Vec<String>,

    /// Review guidelines
    pub guidelines: Vec<String>,

    /// Review checklist
    pub checklist: Vec<String>,

    /// Review timeout (hours)
    pub timeout_hours: u64,
}

/// Testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfig {
    /// Test framework
    pub framework: String,

    /// Test directory
    pub test_directory: PathBuf,

    /// Test patterns
    pub test_patterns: Vec<String>,

    /// Coverage requirements
    pub coverage_requirements: CoverageRequirements,

    /// Test timeout (seconds)
    pub timeout_seconds: u64,

    /// Parallel testing
    pub parallel: bool,
}

/// Coverage requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageRequirements {
    /// Minimum coverage percentage
    pub minimum_coverage: f64,

    /// Coverage by file type
    pub by_file_type: HashMap<String, f64>,

    /// Coverage exclusions
    pub exclusions: Vec<String>,
}

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Documentation directory
    pub documentation_directory: PathBuf,

    /// Documentation format
    pub format: String,

    /// Auto-generate documentation
    pub auto_generate: bool,

    /// Documentation templates
    pub templates: HashMap<String, String>,

    /// Documentation standards
    pub standards: Vec<String>,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment environments
    pub environments: Vec<DeploymentEnvironment>,

    /// Deployment strategy
    pub strategy: DeploymentStrategy,

    /// Rollback settings
    pub rollback: RollbackConfig,

    /// Health checks
    pub health_checks: Vec<HealthCheck>,
}

/// Deployment environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEnvironment {
    /// Environment name
    pub name: String,

    /// Environment URL
    pub url: Option<String>,

    /// Environment variables
    pub variables: HashMap<String, String>,

    /// Deployment triggers
    pub triggers: Vec<String>,

    /// Auto-deploy
    pub auto_deploy: bool,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    Rolling,
    BlueGreen,
    Canary,
    Recreate,
}

/// Rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Auto-rollback enabled
    pub auto_rollback: bool,

    /// Rollback triggers
    pub triggers: Vec<String>,

    /// Rollback timeout (minutes)
    pub timeout_minutes: u64,

    /// Rollback versions to keep
    pub versions_to_keep: u32,
}

/// Health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Health check name
    pub name: String,

    /// Health check URL
    pub url: String,

    /// Health check interval (seconds)
    pub interval_seconds: u64,

    /// Health check timeout (seconds)
    pub timeout_seconds: u64,

    /// Health check retries
    pub retries: u32,
}

/// Scope configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeConfig {
    /// Default scope type
    pub default_scope_type: String,

    /// Scope naming convention
    pub naming_convention: String,

    /// Scope templates
    pub templates: HashMap<String, String>,

    /// Scope inheritance
    pub inheritance: ScopeInheritanceConfig,

    /// Scope validation
    pub validation: ScopeValidationConfig,
}

/// Scope inheritance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeInheritanceConfig {
    /// Enable inheritance
    pub enabled: bool,

    /// Inheritance rules
    pub rules: Vec<InheritanceRule>,

    /// Override behavior
    pub override_behavior: OverrideBehavior,
}

/// Inheritance rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceRule {
    /// Rule name
    pub name: String,

    /// Source scope pattern
    pub source_pattern: String,

    /// Target scope pattern
    pub target_pattern: String,

    /// Inherited fields
    pub fields: Vec<String>,

    /// Priority
    pub priority: u32,
}

/// Override behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideBehavior {
    Allow,
    Deny,
    Warn,
}

/// Scope validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeValidationConfig {
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

    /// Rule pattern
    pub pattern: String,

    /// Rule message
    pub message: String,

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

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Workflow type
    pub workflow_type: String,

    /// Workflow steps
    pub steps: Vec<WorkflowStep>,

    /// Workflow triggers
    pub triggers: Vec<WorkflowTrigger>,

    /// Workflow conditions
    pub conditions: Vec<WorkflowCondition>,
}

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step name
    pub name: String,

    /// Step type
    pub step_type: String,

    /// Step configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Step dependencies
    pub dependencies: Vec<String>,

    /// Step timeout (seconds)
    pub timeout_seconds: Option<u64>,
}

/// Workflow trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    /// Trigger name
    pub name: String,

    /// Trigger type
    pub trigger_type: String,

    /// Trigger conditions
    pub conditions: HashMap<String, String>,
}

/// Workflow condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCondition {
    /// Condition name
    pub name: String,

    /// Condition expression
    pub expression: String,

    /// Condition description
    pub description: Option<String>,
}

/// Repository security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySecurityConfig {
    /// Security scanning
    pub security_scanning: SecurityScanningConfig,

    /// Access control
    pub access_control: AccessControlConfig,

    /// Secrets management
    pub secrets_management: SecretsManagementConfig,

    /// Compliance
    pub compliance: ComplianceConfig,
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

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Access control enabled
    pub enabled: bool,

    /// Access levels
    pub access_levels: Vec<AccessLevel>,

    /// Access policies
    pub policies: Vec<AccessPolicy>,
}

/// Access level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLevel {
    /// Level name
    pub name: String,

    /// Level permissions
    pub permissions: Vec<String>,

    /// Level description
    pub description: Option<String>,
}

/// Access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy name
    pub name: String,

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
}

/// Policy effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// Secrets management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsManagementConfig {
    /// Secrets management enabled
    pub enabled: bool,

    /// Secrets provider
    pub provider: String,

    /// Secrets configuration
    pub config: HashMap<String, String>,

    /// Secrets rotation
    pub rotation: SecretsRotationConfig,
}

/// Secrets rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsRotationConfig {
    /// Rotation enabled
    pub enabled: bool,

    /// Rotation interval (days)
    pub interval_days: u32,

    /// Rotation notification
    pub notification: bool,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Compliance framework
    pub framework: String,

    /// Compliance rules
    pub rules: Vec<ComplianceRule>,

    /// Compliance reporting
    pub reporting: ComplianceReportingConfig,
}

/// Compliance rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    /// Rule name
    pub name: String,

    /// Rule description
    pub description: String,

    /// Rule check
    pub check: String,

    /// Rule severity
    pub severity: ValidationSeverity,
}

/// Compliance reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportingConfig {
    /// Reporting enabled
    pub enabled: bool,

    /// Report format
    pub format: String,

    /// Report destination
    pub destination: String,

    /// Report schedule
    pub schedule: String,
}

/// Repository integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryIntegrationConfig {
    /// CI/CD integration
    pub cicd: CICDIntegrationConfig,

    /// Issue tracking integration
    pub issue_tracking: IssueTrackingIntegrationConfig,

    /// Communication integration
    pub communication: CommunicationIntegrationConfig,

    /// Monitoring integration
    pub monitoring: MonitoringIntegrationConfig,
}

/// CI/CD integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CICDIntegrationConfig {
    /// CI/CD enabled
    pub enabled: bool,

    /// CI/CD provider
    pub provider: String,

    /// CI/CD configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Pipeline configuration
    pub pipeline: PipelineConfig,
}

/// Pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline stages
    pub stages: Vec<PipelineStage>,

    /// Pipeline triggers
    pub triggers: Vec<String>,

    /// Pipeline artifacts
    pub artifacts: Vec<String>,
}

/// Pipeline stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    /// Stage name
    pub name: String,

    /// Stage commands
    pub commands: Vec<String>,

    /// Stage dependencies
    pub dependencies: Vec<String>,

    /// Stage timeout (minutes)
    pub timeout_minutes: u64,
}

/// Issue tracking integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTrackingIntegrationConfig {
    /// Issue tracking enabled
    pub enabled: bool,

    /// Issue tracking provider
    pub provider: String,

    /// Issue tracking configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Issue templates
    pub templates: HashMap<String, String>,
}

/// Communication integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationIntegrationConfig {
    /// Communication enabled
    pub enabled: bool,

    /// Communication channels
    pub channels: Vec<CommunicationChannel>,

    /// Notification settings
    pub notifications: NotificationSettings,
}

/// Communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannel {
    /// Channel name
    pub name: String,

    /// Channel type
    pub channel_type: String,

    /// Channel configuration
    pub config: HashMap<String, String>,
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// Notification events
    pub events: Vec<String>,

    /// Notification recipients
    pub recipients: Vec<String>,

    /// Notification format
    pub format: String,
}

/// Monitoring integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringIntegrationConfig {
    /// Monitoring enabled
    pub enabled: bool,

    /// Monitoring provider
    pub provider: String,

    /// Monitoring configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Monitoring metrics
    pub metrics: Vec<String>,
}

impl Config for RepositoryConfig {
    fn version(&self) -> &str {
        &self.version
    }

    fn validate_config(&self) -> rhema_core::RhemaResult<()> {
        // Basic validation
        if self.version.is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "Version cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn load_from_file(path: &Path) -> rhema_core::RhemaResult<Self> {
        Self::load(path)
    }

    fn save_to_file(&self, _path: &Path) -> rhema_core::RhemaResult<()> {
        // Implementation would save to the specified path
        Ok(())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "version": {"type": "string"},
                "repository_path": {"type": "string"},
                "branch_conventions": {"type": "object"}
            },
            "required": ["version"]
        })
    }

    fn documentation() -> &'static str {
        "Repository configuration for Rhema application"
    }
}

impl RepositoryConfig {
    /// Create a new repository configuration
    pub fn new(_repo_path: &Path) -> Self {
        let now = Utc::now();
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            repository: RepositoryInfo::default(),
            settings: RepositorySettings::default(),
            scopes: ScopeConfig::default(),
            workflow: WorkflowConfig::default(),
            security: RepositorySecurityConfig::default(),
            integrations: RepositoryIntegrationConfig::default(),
            custom: HashMap::new(),
            audit_log: ConfigAuditLog::new(),
            health: ConfigHealth::default(),
            stats: ConfigStats::new(),
            updated_at: now,
        }
    }

    /// Load repository configuration from file
    pub fn load(repo_path: &Path) -> RhemaResult<Self> {
        let config_path = repo_path.join(".rhema").join("repository.yaml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| rhema_core::RhemaError::IoError(e))?;

            let config: Self = serde_yaml::from_str(&content).map_err(|e| {
                rhema_core::RhemaError::InvalidYaml {
                    file: config_path.display().to_string(),
                    message: e.to_string(),
                }
            })?;

            config.validate_config()?;
            Ok(config)
        } else {
            // Create default configuration
            let config = Self::new(repo_path);
            config.save(repo_path)?;
            Ok(config)
        }
    }

    /// Load repository configuration from JSON string
    pub fn load_from_json(json: &str) -> RhemaResult<Self> {
        let config: Self = serde_json::from_str(json).map_err(|e| {
            rhema_core::RhemaError::InvalidJson {
                message: e.to_string(),
            }
        })?;

        config.validate_config()?;
        Ok(config)
    }

    /// Save repository configuration to file
    pub fn save(&self, repo_path: &Path) -> RhemaResult<()> {
        let config_path = repo_path.join(".rhema").join("repository.yaml");

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| rhema_core::RhemaError::IoError(e))?;
        }

        let content =
            serde_yaml::to_string(self).map_err(|e| rhema_core::RhemaError::InvalidYaml {
                file: config_path.display().to_string(),
                message: e.to_string(),
            })?;

        std::fs::write(&config_path, content).map_err(|e| rhema_core::RhemaError::IoError(e))?;

        Ok(())
    }

    /// Update configuration
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn get_value(&self, path: &str) -> Option<&serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = serde_json::to_value(self).ok()?;

        for part in parts {
            current = current.get(part)?.clone();
        }

        // Convert back to a reference - this is a simplified approach
        // In a real implementation, you'd want to return a proper reference
        None
    }

    pub fn set_value(&mut self, path: &str, value: serde_json::Value) -> RhemaResult<()> {
        let parts: Vec<&str> = path.split('.').collect();

        // This is a simplified implementation
        // In a real implementation, you'd want to properly traverse and set nested values
        match parts.as_slice() {
            ["repository", "name"] => {
                if let Some(name) = value.as_str() {
                    self.repository.name = name.to_string();
                }
            }
            ["repository", "description"] => {
                if let Some(desc) = value.as_str() {
                    self.repository.description = Some(desc.to_string());
                }
            }
            ["repository", "url"] => {
                if let Some(url) = value.as_str() {
                    self.repository.url = Some(url.to_string());
                }
            }
            ["settings", "default_branch"] => {
                if let Some(branch) = value.as_str() {
                    self.settings.default_branch = branch.to_string();
                }
            }
            _ => {
                // For custom fields, store in the custom HashMap
                self.custom.insert(path.to_string(), value);
            }
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn export(&self, format: &str) -> RhemaResult<String> {
        match format {
            "json" => {
                let json = serde_json::to_string_pretty(self)?;
                Ok(json)
            }
            "yaml" => {
                let yaml = serde_yaml::to_string(self)?;
                Ok(yaml)
            }
            _ => Err(rhema_core::RhemaError::ConfigError(format!(
                "Unsupported export format: {}. Supported formats: json, yaml",
                format
            ))),
        }
    }

    pub fn import(&mut self, content: &str, format: &str) -> RhemaResult<()> {
        let imported_config: RepositoryConfig = match format {
            "json" => serde_json::from_str(content)?,
            "yaml" => serde_yaml::from_str(content)?,
            _ => {
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Unsupported import format: {}. Supported formats: json, yaml",
                    format
                )));
            }
        };

        // Merge the imported configuration with the current one
        *self = imported_config;
        self.updated_at = Utc::now();

        Ok(())
    }
}

// Default implementations
impl Default for RepositoryInfo {
    fn default() -> Self {
        Self {
            name: "default-repository".to_string(),
            description: None,
            url: None,
            repository_type: RepositoryType::Git,
            owner: "default-owner".to_string(),
            visibility: RepositoryVisibility::Private,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for RepositorySettings {
    fn default() -> Self {
        Self {
            default_branch: "main".to_string(),
            branch_protection: BranchProtectionConfig::default(),
            commit_conventions: CommitConventions::default(),
            code_review: CodeReviewConfig::default(),
            testing: TestingConfig::default(),
            documentation: DocumentationConfig::default(),
            deployment: DeploymentConfig::default(),
        }
    }
}

impl Default for BranchProtectionConfig {
    fn default() -> Self {
        Self {
            protected_branches: vec!["main".to_string(), "develop".to_string()],
            require_reviews: true,
            required_reviewers: 1,
            require_status_checks: true,
            required_status_checks: vec!["test".to_string(), "lint".to_string()],
            require_up_to_date: true,
            allow_force_push: false,
            allow_deletions: false,
        }
    }
}

impl Default for CommitConventions {
    fn default() -> Self {
        Self {
            conventional_commits: true,
            message_template: None,
            types: vec![
                "feat".to_string(),
                "fix".to_string(),
                "docs".to_string(),
                "style".to_string(),
                "refactor".to_string(),
                "test".to_string(),
                "chore".to_string(),
            ],
            scopes: Vec::new(),
            breaking_change_format: "BREAKING CHANGE:".to_string(),
            footer_format: "Closes #".to_string(),
        }
    }
}

impl Default for CodeReviewConfig {
    fn default() -> Self {
        Self {
            required: true,
            auto_assign: false,
            required_reviewers: Vec::new(),
            guidelines: vec![
                "Check for security issues".to_string(),
                "Verify test coverage".to_string(),
            ],
            checklist: vec![
                "Code follows style guidelines".to_string(),
                "Tests pass".to_string(),
            ],
            timeout_hours: 72,
        }
    }
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            framework: "default".to_string(),
            test_directory: PathBuf::from("tests"),
            test_patterns: vec!["*_test.rs".to_string(), "test_*.rs".to_string()],
            coverage_requirements: CoverageRequirements::default(),
            timeout_seconds: 300,
            parallel: true,
        }
    }
}

impl Default for CoverageRequirements {
    fn default() -> Self {
        Self {
            minimum_coverage: 80.0,
            by_file_type: HashMap::new(),
            exclusions: vec!["tests/".to_string(), "examples/".to_string()],
        }
    }
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            documentation_directory: PathBuf::from("docs"),
            format: "markdown".to_string(),
            auto_generate: false,
            templates: HashMap::new(),
            standards: vec!["Keep documentation up to date".to_string()],
        }
    }
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environments: vec![DeploymentEnvironment::default()],
            strategy: DeploymentStrategy::Rolling,
            rollback: RollbackConfig::default(),
            health_checks: Vec::new(),
        }
    }
}

impl Default for DeploymentEnvironment {
    fn default() -> Self {
        Self {
            name: "production".to_string(),
            url: None,
            variables: HashMap::new(),
            triggers: vec!["main".to_string()],
            auto_deploy: false,
        }
    }
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            auto_rollback: true,
            triggers: vec!["health_check_failed".to_string()],
            timeout_minutes: 30,
            versions_to_keep: 5,
        }
    }
}

impl Default for ScopeConfig {
    fn default() -> Self {
        Self {
            default_scope_type: "service".to_string(),
            naming_convention: "kebab-case".to_string(),
            templates: HashMap::new(),
            inheritance: ScopeInheritanceConfig::default(),
            validation: ScopeValidationConfig::default(),
        }
    }
}

impl Default for ScopeInheritanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            override_behavior: OverrideBehavior::Allow,
        }
    }
}

impl Default for ScopeValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            severity: ValidationSeverity::Warning,
        }
    }
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            workflow_type: "gitflow".to_string(),
            steps: Vec::new(),
            triggers: Vec::new(),
            conditions: Vec::new(),
        }
    }
}

impl Default for RepositorySecurityConfig {
    fn default() -> Self {
        Self {
            security_scanning: SecurityScanningConfig::default(),
            access_control: AccessControlConfig::default(),
            secrets_management: SecretsManagementConfig::default(),
            compliance: ComplianceConfig::default(),
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

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            access_levels: vec![
                AccessLevel {
                    name: "admin".to_string(),
                    permissions: vec![
                        "read".to_string(),
                        "write".to_string(),
                        "delete".to_string(),
                    ],
                    description: Some("Full access".to_string()),
                },
                AccessLevel {
                    name: "developer".to_string(),
                    permissions: vec!["read".to_string(), "write".to_string()],
                    description: Some("Read and write access".to_string()),
                },
                AccessLevel {
                    name: "viewer".to_string(),
                    permissions: vec!["read".to_string()],
                    description: Some("Read-only access".to_string()),
                },
            ],
            policies: Vec::new(),
        }
    }
}

impl Default for SecretsManagementConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "local".to_string(),
            config: HashMap::new(),
            rotation: SecretsRotationConfig::default(),
        }
    }
}

impl Default for SecretsRotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_days: 90,
            notification: true,
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            framework: "basic".to_string(),
            rules: Vec::new(),
            reporting: ComplianceReportingConfig::default(),
        }
    }
}

impl Default for ComplianceReportingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            format: "json".to_string(),
            destination: "local".to_string(),
            schedule: "weekly".to_string(),
        }
    }
}

impl Default for RepositoryIntegrationConfig {
    fn default() -> Self {
        Self {
            cicd: CICDIntegrationConfig::default(),
            issue_tracking: IssueTrackingIntegrationConfig::default(),
            communication: CommunicationIntegrationConfig::default(),
            monitoring: MonitoringIntegrationConfig::default(),
        }
    }
}

impl Default for CICDIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "github".to_string(),
            config: HashMap::new(),
            pipeline: PipelineConfig::default(),
        }
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            stages: vec![
                PipelineStage {
                    name: "build".to_string(),
                    commands: vec!["cargo build".to_string()],
                    dependencies: Vec::new(),
                    timeout_minutes: 10,
                },
                PipelineStage {
                    name: "test".to_string(),
                    commands: vec!["cargo test".to_string()],
                    dependencies: vec!["build".to_string()],
                    timeout_minutes: 15,
                },
            ],
            triggers: vec!["push".to_string(), "pull_request".to_string()],
            artifacts: vec!["target/release".to_string()],
        }
    }
}

impl Default for IssueTrackingIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "github".to_string(),
            config: HashMap::new(),
            templates: HashMap::new(),
        }
    }
}

impl Default for CommunicationIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            channels: Vec::new(),
            notifications: NotificationSettings::default(),
        }
    }
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            events: vec!["deployment".to_string(), "security_alert".to_string()],
            recipients: Vec::new(),
            format: "text".to_string(),
        }
    }
}

impl Default for MonitoringIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "prometheus".to_string(),
            config: HashMap::new(),
            metrics: Vec::new(),
        }
    }
}

// ConfigHealth Default implementation moved to mod.rs to avoid conflicts
