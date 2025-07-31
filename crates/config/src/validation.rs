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
use crate::{ConfigIssueSeverity, ConfigIssue, Config, ConfigError, CURRENT_CONFIG_VERSION};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Validation manager for configuration validation
pub struct ValidationManager {
    rules: Vec<ValidationRule>,
    custom_validators: HashMap<String, Box<dyn CustomValidator>>,
    validation_cache: HashMap<PathBuf, ValidationResult>,
    cache_ttl: u64,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub name: String,
    pub description: String,
    pub rule_type: ValidationRuleType,
    pub severity: ConfigIssueSeverity,
    pub enabled: bool,
    pub conditions: Vec<ValidationCondition>,
    pub actions: Vec<ValidationAction>,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Schema,
    Format,
    Security,
    Performance,
    Compliance,
    Custom,
}

/// Validation condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    pub field: String,
    pub operator: ValidationOperator,
    pub value: serde_json::Value,
}

/// Validation operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Regex,
    Exists,
    NotExists,
}

/// Validation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationAction {
    pub action_type: ValidationActionType,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Validation action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationActionType {
    Log,
    Warn,
    Error,
    Fix,
    Skip,
    Custom,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<ConfigIssue>,
    pub warnings: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub overall_valid: bool,
    pub results: HashMap<PathBuf, ValidationResult>,
    pub summary: ValidationSummary,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Validation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_configs: usize,
    pub valid_configs: usize,
    pub invalid_configs: usize,
    pub total_issues: usize,
    pub critical_issues: usize,
    pub error_issues: usize,
    pub warning_issues: usize,
    pub info_issues: usize,
}

/// Custom validator trait
pub trait CustomValidator: Send + Sync {
    fn validate(&self, config: &serde_json::Value) -> ValidationResult;
    fn name(&self) -> &str;
}

impl ValidationManager {
    /// Create a new validation manager
    pub fn new(_global_config: &super::GlobalConfig) -> RhemaResult<Self> {
        let mut manager = Self {
            rules: Vec::new(),
            custom_validators: HashMap::new(),
            validation_cache: HashMap::new(),
            cache_ttl: 3600, // 1 hour default
        };
        
        manager.load_default_rules();
        manager.load_custom_validators();
        
        Ok(manager)
    }

    /// Load default validation rules
    fn load_default_rules(&mut self) {
        // Schema validation rules
        self.rules.push(ValidationRule {
            name: "schema_version_check".to_string(),
            description: "Check if configuration version is supported".to_string(),
            rule_type: ValidationRuleType::Schema,
            severity: ConfigIssueSeverity::Error,
            enabled: true,
            conditions: vec![
                ValidationCondition {
                    field: "version".to_string(),
                    operator: ValidationOperator::Exists,
                    value: serde_json::Value::Null,
                }
            ],
            actions: vec![
                ValidationAction {
                    action_type: ValidationActionType::Error,
                    parameters: HashMap::new(),
                }
            ],
        });

        // Security validation rules
        self.rules.push(ValidationRule {
            name: "security_encryption_check".to_string(),
            description: "Check if sensitive data is encrypted".to_string(),
            rule_type: ValidationRuleType::Security,
            severity: ConfigIssueSeverity::Warning,
            enabled: true,
            conditions: vec![
                ValidationCondition {
                    field: "security.encryption.enabled".to_string(),
                    operator: ValidationOperator::Equals,
                    value: serde_json::Value::Bool(false),
                }
            ],
            actions: vec![
                ValidationAction {
                    action_type: ValidationActionType::Warn,
                    parameters: HashMap::new(),
                }
            ],
        });

        // Performance validation rules
        self.rules.push(ValidationRule {
            name: "performance_cache_check".to_string(),
            description: "Check if caching is properly configured".to_string(),
            rule_type: ValidationRuleType::Performance,
            severity: ConfigIssueSeverity::Info,
            enabled: true,
            conditions: vec![
                ValidationCondition {
                    field: "performance.cache.enabled".to_string(),
                    operator: ValidationOperator::Equals,
                    value: serde_json::Value::Bool(false),
                }
            ],
            actions: vec![
                ValidationAction {
                    action_type: ValidationActionType::Log,
                    parameters: HashMap::new(),
                }
            ],
        });
    }

    /// Load custom validators
    fn load_custom_validators(&mut self) {
        // Add custom validators here
        self.custom_validators.insert(
            "git_integration_validator".to_string(),
            Box::new(GitIntegrationValidator),
        );
        
        self.custom_validators.insert(
            "path_validator".to_string(),
            Box::new(PathValidator),
        );
    }

    /// Validate all configurations
    pub fn validate_all(
        &self,
        global_config: &super::GlobalConfig,
        repository_configs: &HashMap<PathBuf, super::RepositoryConfig>,
        scope_configs: &HashMap<PathBuf, super::ScopeConfig>,
    ) -> RhemaResult<ValidationReport> {
        let start_time = Utc::now();
        let mut results = HashMap::new();
        let mut summary = ValidationSummary {
            total_configs: 0,
            valid_configs: 0,
            invalid_configs: 0,
            total_issues: 0,
            critical_issues: 0,
            error_issues: 0,
            warning_issues: 0,
            info_issues: 0,
        };

        // Validate global config
        let global_result = self.validate_config(global_config.clone(), "global")?;
        results.insert(PathBuf::from("global"), global_result.clone());
        self.update_summary(&mut summary, &global_result);

        // Validate repository configs
        for (path, config) in repository_configs {
            let result = self.validate_config(config.clone(), &format!("repository:{}", path.display()))?;
            results.insert(path.clone(), result.clone());
            self.update_summary(&mut summary, &result);
        }

        // Validate scope configs
        for (path, config) in scope_configs {
            let result = self.validate_config(config.clone(), &format!("scope:{}", path.display()))?;
            results.insert(path.clone(), result.clone());
            self.update_summary(&mut summary, &result);
        }

        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);

        Ok(ValidationReport {
            overall_valid: summary.invalid_configs == 0,
            results,
            summary,
            timestamp: end_time,
            duration_ms: duration.num_milliseconds() as u64,
        })
    }

    /// Validate a single configuration
    pub fn validate_config<T: Config>(&self, config: T, _context: &str) -> RhemaResult<ValidationResult> {
        let start_time = Utc::now();
        let mut issues = Vec::new();

        // Basic validation using the Config trait
        if let Err(e) = config.validate_config() {
            issues.push(ConfigIssue {
                severity: ConfigIssueSeverity::Error,
                message: format!("Configuration validation failed: {}", e),
                location: None,
                suggestion: Some("Check configuration format and required fields".to_string()),
            });
        }

        // Schema validation
        let schema_issues = self.validate_schema(&config)?;
        issues.extend(schema_issues);

        // Custom validation rules
        let rule_issues = self.apply_validation_rules(&config)?;
        issues.extend(rule_issues);

        // Custom validators
        let custom_issues = self.apply_custom_validators(&config)?;
        issues.extend(custom_issues);

        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);

        let valid = issues.iter().all(|issue| {
            matches!(issue.severity, ConfigIssueSeverity::Info | ConfigIssueSeverity::Warning)
        });

        Ok(ValidationResult {
            valid,
            issues,
            warnings: Vec::new(),
            timestamp: end_time,
            duration_ms: duration.num_milliseconds() as u64,
        })
    }

    /// Validate configuration schema
    fn validate_schema<T: Config>(&self, _config: &T) -> RhemaResult<Vec<ConfigIssue>> {
        // This would implement schema validation logic
        // For now, return empty vector
        Ok(Vec::new())
    }

    /// Apply validation rules
    fn apply_validation_rules<T: Config>(&self, config: &T) -> RhemaResult<Vec<ConfigIssue>> {
        let mut issues = Vec::new();
        let config_value = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e))?;

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if self.evaluate_rule(rule, &config_value)? {
                for action in &rule.actions {
                    match action.action_type {
                        ValidationActionType::Error => {
                            issues.push(ConfigIssue {
                                severity: rule.severity.clone(),
                                message: rule.description.clone(),
                                location: None,
                                suggestion: None,
                            });
                        }
                        ValidationActionType::Warn => {
                            issues.push(ConfigIssue {
                                severity: ConfigIssueSeverity::Warning,
                                message: rule.description.clone(),
                                location: None,
                                suggestion: None,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Evaluate a validation rule
    fn evaluate_rule(&self, rule: &ValidationRule, config: &serde_json::Value) -> RhemaResult<bool> {
        for condition in &rule.conditions {
            if !self.evaluate_condition(condition, config)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Evaluate a validation condition
    fn evaluate_condition(&self, condition: &ValidationCondition, config: &serde_json::Value) -> RhemaResult<bool> {
        let field_value = self.get_field_value(&condition.field, config)?;
        
        match condition.operator {
            ValidationOperator::Equals => Ok(field_value == condition.value),
            ValidationOperator::NotEquals => Ok(field_value != condition.value),
            ValidationOperator::GreaterThan => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a > b)
                } else {
                    Ok(false)
                }
            }
            ValidationOperator::LessThan => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a < b)
                } else {
                    Ok(false)
                }
            }
            ValidationOperator::Contains => {
                if let (Some(a), Some(b)) = (field_value.as_str(), condition.value.as_str()) {
                    Ok(a.contains(b))
                } else {
                    Ok(false)
                }
            }
            ValidationOperator::Exists => Ok(!field_value.is_null()),
            ValidationOperator::NotExists => Ok(field_value.is_null()),
            _ => Ok(false), // Implement other operators as needed
        }
    }

    /// Get field value from nested JSON
    fn get_field_value(&self, field_path: &str, config: &serde_json::Value) -> RhemaResult<serde_json::Value> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = config;

        for part in parts {
            current = current.get(part)
                .ok_or_else(|| ConfigError::ValidationFailed(
                    format!("Field '{}' not found in configuration", field_path)
                ))?;
        }

        Ok(current.clone())
    }

    /// Apply custom validators
    fn apply_custom_validators<T: Config>(&self, config: &T) -> RhemaResult<Vec<ConfigIssue>> {
        let mut issues = Vec::new();
        let config_value = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e))?;

        for validator in self.custom_validators.values() {
            let result = validator.validate(&config_value);
            issues.extend(result.issues);
        }

        Ok(issues)
    }

    /// Update validation summary
    fn update_summary(&self, summary: &mut ValidationSummary, result: &ValidationResult) {
        summary.total_configs += 1;
        
        if result.valid {
            summary.valid_configs += 1;
        } else {
            summary.invalid_configs += 1;
        }

        for issue in &result.issues {
            summary.total_issues += 1;
            match issue.severity {
                ConfigIssueSeverity::Critical => summary.critical_issues += 1,
                ConfigIssueSeverity::Error => summary.error_issues += 1,
                ConfigIssueSeverity::Warning => summary.warning_issues += 1,
                ConfigIssueSeverity::Info => summary.info_issues += 1,
            }
        }
    }

    /// Add custom validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }

    /// Remove validation rule
    pub fn remove_rule(&mut self, rule_name: &str) {
        self.rules.retain(|rule| rule.name != rule_name);
    }

    /// Enable/disable validation rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) -> RhemaResult<()> {
        for rule in &mut self.rules {
            if rule.name == rule_name {
                rule.enabled = enabled;
                return Ok(());
            }
        }
        Err(ConfigError::ValidationFailed(
            format!("Rule '{}' not found", rule_name)
        ).into())
    }

    /// Get validation rules
    pub fn get_rules(&self) -> &[ValidationRule] {
        &self.rules
    }

    /// Clear validation cache
    pub fn clear_cache(&mut self) {
        self.validation_cache.clear();
    }

    /// Set cache TTL
    pub fn set_cache_ttl(&mut self, ttl: u64) {
        self.cache_ttl = ttl;
    }
}

/// Git integration validator
struct GitIntegrationValidator;

impl CustomValidator for GitIntegrationValidator {
    fn validate(&self, config: &serde_json::Value) -> ValidationResult {
        let mut issues = Vec::new();

        if let Some(integrations) = config.get("integrations") {
            if let Some(git) = integrations.get("git") {
                if let Some(enabled) = git.get("enabled") {
                    if enabled.as_bool().unwrap_or(false) {
                        // Check if git credentials are configured
                        if let Some(credentials) = git.get("credentials") {
                            let has_username = credentials.get("username").is_some();
                            let has_email = credentials.get("email").is_some();
                            let has_ssh_key = credentials.get("ssh_key_path").is_some();
                            let has_token = credentials.get("personal_access_token").is_some();

                            if !has_username && !has_email && !has_ssh_key && !has_token {
                                issues.push(ConfigIssue {
                                    severity: ConfigIssueSeverity::Warning,
                                    message: "Git integration is enabled but no credentials are configured".to_string(),
                                    location: Some("integrations.git.credentials".to_string()),
                                    suggestion: Some("Configure git username, email, SSH key, or personal access token".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        ValidationResult {
            valid: issues.is_empty(),
            issues,
            warnings: Vec::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
        }
    }

    fn name(&self) -> &str {
        "git_integration_validator"
    }
}

/// Path validator
struct PathValidator;

impl CustomValidator for PathValidator {
    fn validate(&self, config: &serde_json::Value) -> ValidationResult {
        let mut issues = Vec::new();

        // Check if paths exist and are accessible
        if let Some(environment) = config.get("environment") {
            if let Some(paths) = environment.get("paths") {
                let path_fields = ["home", "config", "data", "cache", "log", "temp"];
                
                for field in &path_fields {
                    if let Some(path_value) = paths.get(field) {
                        if let Some(path_str) = path_value.as_str() {
                            let path = Path::new(path_str);
                            if !path.exists() {
                                issues.push(ConfigIssue {
                                    severity: ConfigIssueSeverity::Warning,
                                    message: format!("Path '{}' does not exist", path_str),
                                    location: Some(format!("environment.paths.{}", field)),
                                    suggestion: Some("Create the directory or update the path".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        ValidationResult {
            valid: issues.is_empty(),
            issues,
            warnings: Vec::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
        }
    }

    fn name(&self) -> &str {
        "path_validator"
    }
} 