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

use crate::config::{SafetyValidator, SafetyViolation};
use crate::{Config, CURRENT_CONFIG_VERSION};
use super::global::GlobalConfig;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use validator::Validate;
use rhema_core::RhemaResult;
/// Tools manager for Rhema CLI configuration
pub struct ToolsManager {
    config: ToolsConfig,
    editor: ConfigEditor,
    validator: ConfigValidator,
    migrator: ConfigMigrator,
    backup_tool: ConfigBackupTool,
    documentation_tool: ConfigDocumentationTool,
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ToolsConfig {
    /// Configuration version
    #[validate(length(min = 1))]
    pub version: String,
    
    /// Editor settings
    pub editor: EditorSettings,
    
    /// Validation settings
    pub validation: ValidationSettings,
    
    /// Migration settings
    pub migration: MigrationSettings,
    
    /// Backup settings
    pub backup: BackupSettings,
    
    /// Documentation settings
    pub documentation: DocumentationSettings,
    
    /// Tool integrations
    pub integrations: ToolIntegrations,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Editor settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Default editor
    pub default_editor: EditorType,
    
    /// Editor configuration
    pub editor_config: HashMap<String, String>,
    
    /// Auto-save enabled
    pub auto_save: bool,
    
    /// Auto-save interval (seconds)
    pub auto_save_interval: u64,
    
    /// Syntax highlighting
    pub syntax_highlighting: bool,
    
    /// Line numbers
    pub line_numbers: bool,
    
    /// Word wrap
    pub word_wrap: bool,
    
    /// Tab size
    pub tab_size: u32,
}

/// Editor type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditorType {
    Vim,
    Emacs,
    Nano,
    VSCode,
    Sublime,
    Custom(String),
}

/// Validation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSettings {
    /// Enable validation
    pub enabled: bool,
    
    /// Validation level
    pub level: ValidationLevel,
    
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    
    /// Auto-validation
    pub auto_validation: bool,
    
    /// Validation timeout (seconds)
    pub timeout: u64,
    
    /// Validation cache
    pub cache: ValidationCache,
}

/// Validation level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationLevel {
    Basic,
    Standard,
    Strict,
    Custom(String),
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
    
    /// Rule enabled
    pub enabled: bool,
}

/// Validation severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Validation cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCache {
    /// Enable cache
    pub enabled: bool,
    
    /// Cache size
    pub size: usize,
    
    /// Cache timeout (minutes)
    pub timeout: u64,
}

/// Migration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSettings {
    /// Enable migration
    pub enabled: bool,
    
    /// Auto-migration
    pub auto_migration: bool,
    
    /// Migration strategy
    pub strategy: MigrationStrategy,
    
    /// Migration backup
    pub backup: bool,
    
    /// Migration rollback
    pub rollback: bool,
    
    /// Migration validation
    pub validation: bool,
}

/// Migration strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStrategy {
    InPlace,
    SideBySide,
    Gradual,
    Custom(String),
}

/// Backup settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettings {
    /// Enable backup
    pub enabled: bool,
    
    /// Auto-backup
    pub auto_backup: bool,
    
    /// Backup location
    pub location: PathBuf,
    
    /// Backup format
    pub format: BackupFormat,
    
    /// Backup compression
    pub compression: bool,
    
    /// Backup encryption
    pub encryption: bool,
    
    /// Backup retention
    pub retention: BackupRetention,
}

/// Backup format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupFormat {
    Tar,
    Zip,
    Custom(String),
}

/// Backup retention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRetention {
    /// Retention period (days)
    pub period: u32,
    
    /// Max backups
    pub max_backups: u32,
    
    /// Retention policy
    pub policy: RetentionPolicy,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RetentionPolicy {
    Delete,
    Archive,
    Compress,
    Custom(String),
}

/// Documentation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSettings {
    /// Enable documentation
    pub enabled: bool,
    
    /// Documentation format
    pub format: DocumentationFormat,
    
    /// Documentation location
    pub location: PathBuf,
    
    /// Auto-generation
    pub auto_generation: bool,
    
    /// Documentation templates
    pub templates: HashMap<String, String>,
    
    /// Documentation style
    pub style: DocumentationStyle,
}

/// Documentation format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentationFormat {
    Markdown,
    HTML,
    PDF,
    Custom(String),
}

/// Documentation style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationStyle {
    /// Theme
    pub theme: String,
    
    /// Font size
    pub font_size: u32,
    
    /// Line spacing
    pub line_spacing: f64,
    
    /// Code highlighting
    pub code_highlighting: bool,
}

/// Tool integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolIntegrations {
    /// Git integration
    pub git: GitIntegration,
    
    /// IDE integration
    pub ide: IDEIntegration,
    
    /// CI/CD integration
    pub cicd: CICDIntegration,
    
    /// External tools
    pub external_tools: HashMap<String, ExternalTool>,
}

/// Git integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegration {
    /// Enable git integration
    pub enabled: bool,
    
    /// Auto-commit
    pub auto_commit: bool,
    
    /// Commit message template
    pub commit_message_template: String,
    
    /// Branch protection
    pub branch_protection: bool,
}

/// IDE integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEIntegration {
    /// Enable IDE integration
    pub enabled: bool,
    
    /// Supported IDEs
    pub supported_ides: Vec<String>,
    
    /// Auto-sync
    pub auto_sync: bool,
    
    /// Sync interval (seconds)
    pub sync_interval: u64,
}

/// CI/CD integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CICDIntegration {
    /// Enable CI/CD integration
    pub enabled: bool,
    
    /// CI/CD provider
    pub provider: String,
    
    /// Pipeline configuration
    pub pipeline: HashMap<String, String>,
}

/// External tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalTool {
    /// Tool name
    pub name: String,
    
    /// Tool command
    pub command: String,
    
    /// Tool arguments
    pub arguments: Vec<String>,
    
    /// Tool enabled
    pub enabled: bool,
}

impl ToolsManager {
    /// Create a new tools manager
    pub fn new(_global_config: &GlobalConfig) -> RhemaResult<Self> {
        let config = ToolsConfig::load()?;
        let editor = ConfigEditor::new(&config)?;
        let validator = ConfigValidator::new(&config)?;
        let migrator = ConfigMigrator::new(&config)?;
        let backup_tool = ConfigBackupTool::new(&config)?;
        let documentation_tool = ConfigDocumentationTool::new(&config)?;

        Ok(Self {
            config,
            editor,
            validator,
            migrator,
            backup_tool,
            documentation_tool,
        })
    }

    /// Get tools configuration
    pub fn config(&self) -> &ToolsConfig {
        &self.config
    }

    /// Get config editor
    pub fn editor(&self) -> &ConfigEditor {
        &self.editor
    }

    /// Get config validator
    pub fn validator(&self) -> &ConfigValidator {
        &self.validator
    }

    /// Get config migrator
    pub fn migrator(&self) -> &ConfigMigrator {
        &self.migrator
    }

    /// Get config backup tool
    pub fn backup_tool(&self) -> &ConfigBackupTool {
        &self.backup_tool
    }

    /// Get config documentation tool
    pub fn documentation_tool(&self) -> &ConfigDocumentationTool {
        &self.documentation_tool
    }

    /// Edit configuration
    pub fn edit_config(&self, path: &Path) -> RhemaResult<()> {
        self.editor.edit(path)
    }

    /// Validate configuration
    pub fn validate_config(&self, path: &Path) -> RhemaResult<ValidationReport> {
        self.validator.validate(path)
    }

    /// Migrate configuration
    pub fn migrate_config(&self, path: &Path) -> RhemaResult<MigrationReport> {
        self.migrator.migrate(path)
    }

    /// Backup configuration
    pub fn backup_config(&self, path: &Path) -> RhemaResult<BackupReport> {
        self.backup_tool.backup(path)
    }

    /// Generate documentation
    pub fn generate_documentation(&self, path: &Path) -> RhemaResult<DocumentationReport> {
        self.documentation_tool.generate(path)
    }
}

/// Configuration editor
pub struct ConfigEditor {
    config: ToolsConfig,
}

impl ConfigEditor {
    /// Create a new config editor
    pub fn new(config: &ToolsConfig) -> RhemaResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Edit configuration file
    pub fn edit(&self, path: &Path) -> RhemaResult<()> {
        let editor = match &self.config.editor.default_editor {
            EditorType::Vim => "vim",
            EditorType::Emacs => "emacs",
            EditorType::Nano => "nano",
            EditorType::VSCode => "code",
            EditorType::Sublime => "subl",
            EditorType::Custom(cmd) => cmd,
        };

        // This is a simplified implementation
        // In a real implementation, you'd want to spawn the editor process
        println!("Opening {} with {}", path.display(), editor);

        Ok(())
    }
}

/// Configuration validator
pub struct ConfigValidator {
    _config: ToolsConfig,
    _cache: HashMap<PathBuf, ValidationResult>,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Is valid
    pub is_valid: bool,
    
    /// Errors
    pub errors: Vec<ValidationError>,
    
    /// Warnings
    pub warnings: Vec<ValidationWarning>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    
    /// Error location
    pub location: Option<String>,
    
    /// Error severity
    pub severity: ValidationSeverity,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning message
    pub message: String,
    
    /// Warning location
    pub location: Option<String>,
    
    /// Warning severity
    pub severity: ValidationSeverity,
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// File path
    pub file_path: PathBuf,
    
    /// Validation results
    pub results: Vec<ValidationResult>,
    
    /// Overall status
    pub overall_status: ValidationStatus,
    
    /// Summary
    pub summary: String,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Valid,
    Invalid,
    Warning,
}

impl ConfigValidator {
    /// Create a new config validator
    pub fn new(config: &ToolsConfig) -> RhemaResult<Self> {
        Ok(Self {
            _config: config.clone(),
            _cache: HashMap::new(),
        })
    }

    /// Validate configuration file
    pub fn validate(&self, path: &Path) -> RhemaResult<ValidationReport> {
        let mut report = ValidationReport {
            file_path: path.to_path_buf(),
            results: Vec::new(),
            overall_status: ValidationStatus::Valid,
            summary: String::new(),
        };

        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper validation
        let result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            timestamp: Utc::now(),
        };

        report.results.push(result);
        report.update_status();

        Ok(report)
    }
}

impl ValidationReport {
    /// Update overall status
    pub fn update_status(&mut self) {
        let has_errors = self.results.iter().any(|r| !r.errors.is_empty());
        let has_warnings = self.results.iter().any(|r| !r.warnings.is_empty());

        self.overall_status = if has_errors {
            ValidationStatus::Invalid
        } else if has_warnings {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Valid
        };

        let error_count: usize = self.results.iter().map(|r| r.errors.len()).sum();
        let warning_count: usize = self.results.iter().map(|r| r.warnings.len()).sum();

        self.summary = format!(
            "Validation completed: {} errors, {} warnings",
            error_count,
            warning_count
        );
    }
}

/// Configuration migrator
pub struct ConfigMigrator {
    _config: ToolsConfig,
}

/// Migration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    /// File path
    pub file_path: PathBuf,
    
    /// Migration status
    pub status: MigrationStatus,
    
    /// Migration details
    pub details: String,
    
    /// Backup path
    pub backup_path: Option<PathBuf>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Migration status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    Success,
    Failed,
    Skipped,
    RolledBack,
}

impl ConfigMigrator {
    /// Create a new config migrator
    pub fn new(config: &ToolsConfig) -> RhemaResult<Self> {
        Ok(Self {
            _config: config.clone(),
        })
    }

    /// Migrate configuration file
    pub fn migrate(&self, path: &Path) -> RhemaResult<MigrationReport> {
        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper migration
        Ok(MigrationReport {
            file_path: path.to_path_buf(),
            status: MigrationStatus::Success,
            details: "Migration completed successfully".to_string(),
            backup_path: None,
            timestamp: Utc::now(),
        })
    }
}

/// Configuration backup tool
pub struct ConfigBackupTool {
    config: ToolsConfig,
}

/// Backup report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupReport {
    /// Original path
    pub original_path: PathBuf,
    
    /// Backup path
    pub backup_path: PathBuf,
    
    /// Backup size
    pub size: u64,
    
    /// Backup format
    pub format: String,
    
    /// Backup status
    pub status: BackupStatus,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Backup status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupStatus {
    Success,
    Failed,
    Partial,
}

impl ConfigBackupTool {
    /// Create a new config backup tool
    pub fn new(config: &ToolsConfig) -> RhemaResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Backup configuration file
    pub fn backup(&self, path: &Path) -> RhemaResult<BackupReport> {
        let backup_path = self.config.backup.location.join(format!(
            "{}.backup.{}",
            path.file_name().unwrap().to_string_lossy(),
            Utc::now().format("%Y%m%d_%H%M%S")
        ));

        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper backup
        std::fs::copy(path, &backup_path)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;

        let metadata = std::fs::metadata(&backup_path)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;

        Ok(BackupReport {
            original_path: path.to_path_buf(),
            backup_path,
            size: metadata.len(),
            format: format!("{:?}", self.config.backup.format),
            status: BackupStatus::Success,
            timestamp: Utc::now(),
        })
    }
}

/// Configuration documentation tool
pub struct ConfigDocumentationTool {
    config: ToolsConfig,
}

/// Documentation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationReport {
    /// Source path
    pub source_path: PathBuf,
    
    /// Documentation path
    pub documentation_path: PathBuf,
    
    /// Documentation format
    pub format: String,
    
    /// Generation status
    pub status: DocumentationStatus,
    
    /// Generation details
    pub details: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Documentation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentationStatus {
    Success,
    Failed,
    Partial,
}

impl ConfigDocumentationTool {
    /// Create a new config documentation tool
    pub fn new(config: &ToolsConfig) -> RhemaResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Generate documentation
    pub fn generate(&self, path: &Path) -> RhemaResult<DocumentationReport> {
        let doc_path = self.config.documentation.location.join(format!(
            "{}.md",
            path.file_name().unwrap().to_string_lossy()
        ));

        // This is a simplified implementation
        // In a real implementation, you'd want to implement proper documentation generation
        let content = format!(
            "# Configuration Documentation\n\nGenerated from: {}\n\nGenerated at: {}\n",
            path.display(),
            Utc::now()
        );

        std::fs::write(&doc_path, content)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;

        Ok(DocumentationReport {
            source_path: path.to_path_buf(),
            documentation_path: doc_path,
            format: format!("{:?}", self.config.documentation.format),
            status: DocumentationStatus::Success,
            details: "Documentation generated successfully".to_string(),
            timestamp: Utc::now(),
        })
    }
}

impl ToolsConfig {
    /// Create a new tools configuration
    pub fn new() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            editor: EditorSettings::default(),
            validation: ValidationSettings::default(),
            migration: MigrationSettings::default(),
            backup: BackupSettings::default(),
            documentation: DocumentationSettings::default(),
            integrations: ToolIntegrations::default(),
            updated_at: Utc::now(),
        }
    }

    /// Load tools configuration from file
    pub fn load() -> RhemaResult<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| rhema_core::RhemaError::IoError(e))?;
            
            let config: Self = serde_yaml::from_str(&content)
                .map_err(|e| rhema_core::RhemaError::InvalidYaml {
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

    /// Save tools configuration to file
    pub fn save(&self) -> RhemaResult<()> {
        let config_path = Self::get_config_path()?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| rhema_core::RhemaError::IoError(e))?;
        }
        
        let content = serde_yaml::to_string(self)
            .map_err(|e| rhema_core::RhemaError::InvalidYaml {
                file: config_path.display().to_string(),
                message: e.to_string(),
            })?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;
        
        Ok(())
    }

    /// Get configuration file path
    fn get_config_path() -> RhemaResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| rhema_core::RhemaError::ConfigError("Could not determine config directory".to_string()))?
            .join("rhema");
        
        Ok(config_dir.join("tools.yaml"))
    }

    /// Update configuration
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        self.save()
    }
}

impl Config for ToolsConfig {
    fn version(&self) -> &str {
        &self.version
    }
    
    fn validate_config(&self) -> RhemaResult<()> {
        self.validate()
            .map_err(|e| rhema_core::RhemaError::ConfigError(format!("Validation failed: {}", e)))?;
        Ok(())
    }
    
    fn load_from_file(path: &Path) -> RhemaResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;
        
        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| rhema_core::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;
        
        config.validate_config()?;
        Ok(config)
    }
    
    fn save_to_file(&self, path: &Path) -> RhemaResult<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| rhema_core::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;
        
        std::fs::write(path, content)
            .map_err(|e| rhema_core::RhemaError::IoError(e))?;
        
        Ok(())
    }
    
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "version": {"type": "string"},
                "editor": {"type": "object"},
                "validation": {"type": "object"},
                "migration": {"type": "object"},
                "backup": {"type": "object"},
                "documentation": {"type": "object"},
                "integrations": {"type": "object"}
            },
            "required": ["version", "editor", "validation", "migration", "backup", "documentation", "integrations"]
        })
    }
    
    fn documentation() -> &'static str {
        "Tools configuration for Rhema CLI containing editor settings, validation rules, migration strategies, backup policies, and documentation generation."
    }
}

// Default implementations
impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            default_editor: EditorType::Vim,
            editor_config: HashMap::new(),
            auto_save: true,
            auto_save_interval: 300,
            syntax_highlighting: true,
            line_numbers: true,
            word_wrap: false,
            tab_size: 2,
        }
    }
}

impl Default for ValidationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            level: ValidationLevel::Standard,
            rules: Vec::new(),
            auto_validation: true,
            timeout: 30,
            cache: ValidationCache::default(),
        }
    }
}

impl Default for ValidationCache {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 1000,
            timeout: 60,
        }
    }
}

impl Default for MigrationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_migration: false,
            strategy: MigrationStrategy::InPlace,
            backup: true,
            rollback: true,
            validation: true,
        }
    }
}

impl Default for BackupSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_backup: true,
            location: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join("rhema/backups"),
            format: BackupFormat::Tar,
            compression: true,
            encryption: false,
            retention: BackupRetention::default(),
        }
    }
}

impl Default for BackupRetention {
    fn default() -> Self {
        Self {
            period: 30,
            max_backups: 10,
            policy: RetentionPolicy::Delete,
        }
    }
}

impl Default for DocumentationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format: DocumentationFormat::Markdown,
            location: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join("rhema/docs"),
            auto_generation: false,
            templates: HashMap::new(),
            style: DocumentationStyle::default(),
        }
    }
}

impl Default for DocumentationStyle {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            font_size: 12,
            line_spacing: 1.5,
            code_highlighting: true,
        }
    }
}

impl Default for ToolIntegrations {
    fn default() -> Self {
        Self {
            git: GitIntegration::default(),
            ide: IDEIntegration::default(),
            cicd: CICDIntegration::default(),
            external_tools: HashMap::new(),
        }
    }
}

impl Default for GitIntegration {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_commit: false,
            commit_message_template: "Update configuration".to_string(),
            branch_protection: true,
        }
    }
}

impl Default for IDEIntegration {
    fn default() -> Self {
        Self {
            enabled: true,
            supported_ides: vec!["vscode".to_string(), "intellij".to_string(), "vim".to_string()],
            auto_sync: true,
            sync_interval: 300,
        }
    }
}

impl Default for CICDIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "github".to_string(),
            pipeline: HashMap::new(),
        }
    }
} 