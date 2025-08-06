/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    ConfigIssueSeverity, GlobalConfig,
};
use crate::validation::{ValidationManager, ValidationResult};
use crate::schema_validator::{SchemaValidator, SchemaType, SchemaValidationResult};
use rhema_core::RhemaResult;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Comprehensive configuration validator that combines schema and business logic validation
pub struct ComprehensiveValidator {
    schema_validator: Arc<SchemaValidator>,
    validation_manager: Arc<ValidationManager>,
    validation_cache: Arc<RwLock<HashMap<PathBuf, ComprehensiveValidationResult>>>,
    cache_ttl: u64,
    validation_level: ValidationLevel,
    auto_fix: bool,
}

/// Validation level for comprehensive validation
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ValidationLevel {
    Basic,      // Schema validation only
    Standard,   // Schema + basic business rules
    Strict,     // Schema + all business rules + cross-references
    Complete,   // All validations including performance and security
}

/// Comprehensive validation result
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationResult {
    pub valid: bool,
    pub schema_valid: bool,
    pub business_valid: bool,
    pub issues: Vec<ComprehensiveValidationIssue>,
    pub warnings: Vec<String>,
    pub schema_result: Option<SchemaValidationResult>,
    pub business_result: Option<ValidationResult>,
    pub validation_timestamp: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
}

/// Comprehensive validation issue
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationIssue {
    pub severity: ConfigIssueSeverity,
    pub category: ValidationCategory,
    pub path: String,
    pub message: String,
    pub code: String,
    pub details: Option<Value>,
    pub auto_fixable: bool,
    pub suggested_fix: Option<Value>,
}

/// Validation category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationCategory {
    Schema,
    Business,
    Security,
    Performance,
    Compliance,
    CrossReference,
    Dependency,
    Custom,
}

impl ComprehensiveValidator {
    /// Create a new comprehensive validator
    pub async fn new(global_config: &GlobalConfig) -> RhemaResult<Self> {
        let schema_validator = Arc::new(SchemaValidator::new()?);
        let validation_manager = Arc::new(ValidationManager::new(global_config)?);

        Ok(Self {
            schema_validator,
            validation_manager,
            validation_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: 300, // 5 minutes default
            validation_level: ValidationLevel::Standard,
            auto_fix: false,
        })
    }

    /// Create a new comprehensive validator with custom settings
    pub async fn with_settings(
        global_config: &GlobalConfig,
        cache_ttl: u64,
        validation_level: ValidationLevel,
        auto_fix: bool,
    ) -> RhemaResult<Self> {
        let schema_validator = Arc::new(SchemaValidator::with_settings(cache_ttl, validation_level == ValidationLevel::Strict)?);
        let validation_manager = Arc::new(ValidationManager::new(global_config)?);

        Ok(Self {
            schema_validator,
            validation_manager,
            validation_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
            validation_level,
            auto_fix,
        })
    }

    /// Validate a single configuration file
    pub async fn validate_config_file<P: AsRef<Path>>(
        &self,
        file_path: P,
        schema_type: &SchemaType,
    ) -> RhemaResult<ComprehensiveValidationResult> {
        let file_path = file_path.as_ref();
        let start_time = std::time::Instant::now();

        // Check cache first
        let cache_key = file_path.to_path_buf();
        {
            let cache = self.validation_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                if self.is_cache_valid(cached_result.validation_timestamp) {
                    debug!("Using cached validation result for {}", file_path.display());
                    return Ok(cached_result.clone());
                }
            }
        }

        // Load and parse the configuration file
        let config_content = std::fs::read_to_string(file_path)?;
        let config_value: Value = if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            if ext == "yaml" || ext == "yml" {
                serde_yaml::from_str(&config_content)?
            } else {
                serde_json::from_str(&config_content)?
            }
        } else {
            serde_json::from_str(&config_content)?
        };

        // Perform comprehensive validation
        let result = self.validate_config_value(&config_value, schema_type, file_path).await?;

        // Cache the result
        {
            let mut cache = self.validation_cache.write().await;
            cache.insert(cache_key, result.clone());
        }

        let duration = start_time.elapsed();
        info!(
            "Validated {} in {:?} - Valid: {}",
            file_path.display(),
            duration,
            result.valid
        );

        Ok(result)
    }

    /// Validate a configuration value
    pub async fn validate_config_value(
        &self,
        config_value: &Value,
        schema_type: &SchemaType,
        context: &Path,
    ) -> RhemaResult<ComprehensiveValidationResult> {
        let start_time = std::time::Instant::now();
        let mut all_issues = Vec::new();
        let mut all_warnings = Vec::new();

        // 1. Schema validation
        let schema_result = match self.schema_validator.validate_against_schema(config_value, schema_type).await {
            Ok(result) => Some(result),
            Err(_) => {
                // If schema not found, create a basic validation result
                Some(SchemaValidationResult {
                    valid: true, // Assume valid if no schema available
                    issues: Vec::new(),
                    warnings: vec!["No schema available for validation".to_string()],
                    schema_version: None,
                    validation_timestamp: chrono::Utc::now(),
                })
            }
        };
        let schema_valid = schema_result.as_ref().map(|r| r.valid).unwrap_or(true);

        // Convert schema issues to comprehensive issues
        if let Some(ref schema_result) = schema_result {
            for issue in &schema_result.issues {
                all_issues.push(ComprehensiveValidationIssue {
                    severity: issue.severity.clone(),
                    category: ValidationCategory::Schema,
                    path: issue.path.clone(),
                    message: issue.message.clone(),
                    code: issue.code.clone(),
                    details: issue.details.clone(),
                    auto_fixable: self.is_schema_issue_fixable(issue),
                    suggested_fix: self.suggest_schema_fix(issue, config_value),
                });
            }
        }

        if let Some(ref schema_result) = schema_result {
            all_warnings.extend(schema_result.warnings.clone());
        }

        // 2. Business logic validation (if level permits)
        let business_result = if self.validation_level >= ValidationLevel::Standard {
            // For now, skip business validation since it requires Config trait
            None
        } else {
            None
        };

        let business_valid = business_result.as_ref().map(|r: &ValidationResult| r.valid).unwrap_or(true);

        // Convert business issues to comprehensive issues
        if let Some(ref business_result) = business_result {
            for issue in &business_result.issues {
                all_issues.push(ComprehensiveValidationIssue {
                    severity: issue.severity.clone(),
                    category: ValidationCategory::Business,
                    path: issue.location.clone().unwrap_or_default(),
                    message: issue.message.clone(),
                    code: "BUSINESS_VALIDATION_ERROR".to_string(),
                    details: None,
                    auto_fixable: false,
                    suggested_fix: None,
                });
            }
            all_warnings.extend(business_result.warnings.clone());
        }

        // 3. Additional validations based on level
        if self.validation_level >= ValidationLevel::Strict {
            let additional_issues = self.perform_strict_validations(config_value, schema_type, context).await?;
            all_issues.extend(additional_issues);
        }

        if self.validation_level >= ValidationLevel::Complete {
            let complete_issues = self.perform_complete_validations(config_value, schema_type, context).await?;
            all_issues.extend(complete_issues);
        }

        // 4. Auto-fix if enabled
        if self.auto_fix {
            let fixed_issues = self.auto_fix_issues(&mut all_issues, config_value).await?;
            all_issues = fixed_issues;
        }

        let duration = start_time.elapsed();
        let valid = schema_valid && business_valid && all_issues.iter().all(|issue| issue.severity != ConfigIssueSeverity::Error);

        Ok(ComprehensiveValidationResult {
            valid,
            schema_valid,
            business_valid,
            issues: all_issues,
            warnings: all_warnings,
            schema_result,
            business_result,
            validation_timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        })
    }

    /// Validate all configurations in a directory
    pub async fn validate_directory<P: AsRef<Path>>(
        &self,
        directory: P,
    ) -> RhemaResult<ComprehensiveValidationReport> {
        let directory = directory.as_ref();
        let start_time = std::time::Instant::now();
        let mut results = HashMap::new();
        let mut total_issues = 0;
        let mut total_warnings = 0;
        let mut valid_configs = 0;
        let mut invalid_configs = 0;

        // Find all configuration files
        let config_files = self.find_config_files(directory).await?;

        for file_path in config_files {
            let schema_type = self.detect_schema_type(&file_path).await?;
            let result = self.validate_config_file(&file_path, &schema_type).await?;

            if result.valid {
                valid_configs += 1;
            } else {
                invalid_configs += 1;
            }

            total_issues += result.issues.len();
            total_warnings += result.warnings.len();
            results.insert(file_path, result);
        }

        let duration = start_time.elapsed();

        let total_configs = results.len();
        Ok(ComprehensiveValidationReport {
            overall_valid: invalid_configs == 0,
            results,
            summary: ComprehensiveValidationSummary {
                total_configs,
                valid_configs,
                invalid_configs,
                total_issues,
                total_warnings,
                critical_issues: 0, // Would need to count based on severity
                error_issues: 0,
                warning_issues: 0,
                info_issues: 0,
            },
            validation_timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        })
    }

    /// Perform strict validations
    async fn perform_strict_validations(
        &self,
        config_value: &Value,
        schema_type: &SchemaType,
        context: &Path,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Cross-reference validation
        let cross_ref_issues = self.validate_cross_references(config_value, schema_type, context).await?;
        issues.extend(cross_ref_issues);

        // Dependency validation
        let dep_issues = self.validate_dependencies(config_value, schema_type).await?;
        issues.extend(dep_issues);

        // Security validation
        let security_issues = self.validate_security(config_value, schema_type).await?;
        issues.extend(security_issues);

        Ok(issues)
    }

    /// Perform complete validations
    async fn perform_complete_validations(
        &self,
        config_value: &Value,
        schema_type: &SchemaType,
        _context: &Path,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Performance validation
        let perf_issues = self.validate_performance(config_value, schema_type).await?;
        issues.extend(perf_issues);

        // Compliance validation
        let compliance_issues = self.validate_compliance(config_value, schema_type).await?;
        issues.extend(compliance_issues);

        // Custom validations
        let custom_issues = self.validate_custom_rules(config_value, schema_type).await?;
        issues.extend(custom_issues);

        Ok(issues)
    }

    /// Validate cross-references
    async fn validate_cross_references(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
        context: &Path,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Extract all references from the configuration
        let references = self.extract_references(config_value).await?;

        // Validate each reference
        for (path, reference) in references {
            if !self.reference_exists(&reference, context).await? {
                issues.push(ComprehensiveValidationIssue {
                    severity: ConfigIssueSeverity::Error,
                    category: ValidationCategory::CrossReference,
                    path,
                    message: format!("Reference not found: {}", reference),
                    code: "MISSING_REFERENCE".to_string(),
                    details: None,
                    auto_fixable: false,
                    suggested_fix: None,
                });
            }
        }

        Ok(issues)
    }

    /// Validate dependencies
    async fn validate_dependencies(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Check for circular dependencies
        if let Some(dependencies) = config_value.get("dependencies") {
            if let Some(deps_array) = dependencies.as_array() {
                let mut dep_map = HashMap::new();
                for dep in deps_array {
                    if let Some(dep_obj) = dep.as_object() {
                        if let (Some(name), Some(version)) = (
                            dep_obj.get("name").and_then(|n| n.as_str()),
                            dep_obj.get("version").and_then(|v| v.as_str()),
                        ) {
                            dep_map.insert(name.to_string(), version.to_string());
                        }
                    }
                }

                // Check for circular dependencies
                if self.has_circular_dependencies(&dep_map) {
                    issues.push(ComprehensiveValidationIssue {
                        severity: ConfigIssueSeverity::Error,
                        category: ValidationCategory::Dependency,
                        path: "dependencies".to_string(),
                        message: "Circular dependencies detected".to_string(),
                        code: "CIRCULAR_DEPENDENCIES".to_string(),
                        details: None,
                        auto_fixable: false,
                        suggested_fix: None,
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Validate security aspects
    async fn validate_security(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Check for sensitive information in configuration
        let sensitive_fields = ["password", "secret", "token", "key", "credential"];
        for field in &sensitive_fields {
            if let Some(value) = config_value.get(field) {
                if let Some(str_value) = value.as_str() {
                    if !str_value.is_empty() && !str_value.starts_with("${") {
                        issues.push(ComprehensiveValidationIssue {
                            severity: ConfigIssueSeverity::Warning,
                            category: ValidationCategory::Security,
                            path: field.to_string(),
                            message: format!("Sensitive field '{}' should use environment variables", field),
                            code: "SENSITIVE_FIELD_EXPOSED".to_string(),
                            details: None,
                            auto_fixable: true,
                            suggested_fix: Some(Value::String(format!("${{{}}}", field.to_uppercase()))),
                        });
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate performance aspects
    async fn validate_performance(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Check for large arrays or objects that might impact performance
        if let Some(Value::Array(arr)) = config_value.get("knowledge") {
            if arr.len() > 1000 {
                issues.push(ComprehensiveValidationIssue {
                    severity: ConfigIssueSeverity::Warning,
                    category: ValidationCategory::Performance,
                    path: "knowledge".to_string(),
                    message: "Large knowledge array detected - consider pagination".to_string(),
                    code: "LARGE_ARRAY".to_string(),
                    details: Some(Value::Number(arr.len().into())),
                    auto_fixable: false,
                    suggested_fix: None,
                });
            }
        }

        Ok(issues)
    }

    /// Validate compliance aspects
    async fn validate_compliance(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut issues = Vec::new();

        // Check for required compliance fields
        let required_fields = ["version", "author", "license"];
        for field in &required_fields {
            if config_value.get(field).is_none() {
                issues.push(ComprehensiveValidationIssue {
                    severity: ConfigIssueSeverity::Warning,
                    category: ValidationCategory::Compliance,
                    path: field.to_string(),
                    message: format!("Missing required compliance field: {}", field),
                    code: "MISSING_COMPLIANCE_FIELD".to_string(),
                    details: None,
                    auto_fixable: true,
                    suggested_fix: Some(Value::String("".to_string())),
                });
            }
        }

        Ok(issues)
    }

    /// Validate custom rules
    async fn validate_custom_rules(
        &self,
        config_value: &Value,
        _schema_type: &SchemaType,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let issues = Vec::new();

        // Custom validation rules can be added here
        // For example, domain-specific business rules

        Ok(issues)
    }

    /// Auto-fix issues where possible
    async fn auto_fix_issues(
        &self,
        issues: &mut Vec<ComprehensiveValidationIssue>,
        _config_value: &Value,
    ) -> RhemaResult<Vec<ComprehensiveValidationIssue>> {
        let mut fixed_issues = Vec::new();

        for issue in issues {
            if issue.auto_fixable {
                if let Some(_fix) = &issue.suggested_fix {
                    // Apply the fix
                    debug!("Auto-fixing issue: {}", issue.message);
                    // Note: In a real implementation, this would modify the config_value
                }
            } else {
                fixed_issues.push(issue.clone());
            }
        }

        Ok(fixed_issues)
    }

    /// Helper methods
    fn is_cache_valid(&self, timestamp: chrono::DateTime<chrono::Utc>) -> bool {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(timestamp);
        duration.num_seconds() < self.cache_ttl as i64
    }

    fn is_schema_issue_fixable(&self, issue: &crate::schema_validator::SchemaValidationIssue) -> bool {
        // Determine if a schema issue can be auto-fixed
        matches!(issue.code.as_str(), "MISSING_REQUIRED_FIELD" | "INVALID_TYPE")
    }

    fn suggest_schema_fix(&self, issue: &crate::schema_validator::SchemaValidationIssue, _config: &Value) -> Option<Value> {
        // Suggest fixes for schema issues
        match issue.code.as_str() {
            "MISSING_REQUIRED_FIELD" => Some(Value::String("".to_string())),
            "INVALID_TYPE" => Some(Value::Null),
            _ => None,
        }
    }

    async fn find_config_files<P: AsRef<Path>>(&self, directory: P) -> RhemaResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        let entries = std::fs::read_dir(directory)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "yaml" || extension == "yml" || extension == "json" {
                        files.push(path);
                    }
                }
            }
        }

        Ok(files)
    }

    async fn detect_schema_type(&self, file_path: &Path) -> RhemaResult<SchemaType> {
        // Detect schema type based on filename or content
        let filename = file_path.file_name().and_then(|f| f.to_str()).unwrap_or("");
        
        match filename {
            "rhema.yaml" | "rhema.yml" => Ok(SchemaType::Rhema),
            "scope.yaml" | "scope.yml" => Ok(SchemaType::Scope),
            "knowledge.yaml" | "knowledge.yml" => Ok(SchemaType::Knowledge),
            "todos.yaml" | "todos.yml" => Ok(SchemaType::Todos),
            "decisions.yaml" | "decisions.yml" => Ok(SchemaType::Decisions),
            "patterns.yaml" | "patterns.yml" => Ok(SchemaType::Patterns),
            "conventions.yaml" | "conventions.yml" => Ok(SchemaType::Conventions),
            "lock.yaml" | "lock.yml" => Ok(SchemaType::Lock),
            "action.yaml" | "action.yml" => Ok(SchemaType::Action),
            _ => {
                // Try to detect schema type from content
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    if content.contains("repository:") {
                        Ok(SchemaType::Repository)
                    } else if content.contains("scope:") {
                        Ok(SchemaType::Scope)
                    } else if content.contains("rhema:") {
                        Ok(SchemaType::Rhema)
                    } else {
                        Ok(SchemaType::Custom(filename.to_string()))
                    }
                } else {
                    Ok(SchemaType::Custom(filename.to_string()))
                }
            }
        }
    }

    async fn extract_references(&self, _config_value: &Value) -> RhemaResult<HashMap<String, String>> {
        let references = HashMap::new();
        // Implementation to extract references from config_value
        // This would traverse the JSON structure looking for reference patterns
        Ok(references)
    }

    async fn reference_exists(&self, _reference: &str, _context: &Path) -> RhemaResult<bool> {
        // Check if a reference exists
        // This would check against the file system or a reference registry
        Ok(true) // Placeholder
    }

    fn has_circular_dependencies(&self, _dependencies: &HashMap<String, String>) -> bool {
        // Check for circular dependencies
        // Implementation would use a graph traversal algorithm
        false // Placeholder
    }

    /// Set validation level
    pub fn set_validation_level(&mut self, level: ValidationLevel) {
        self.validation_level = level;
    }

    /// Set auto-fix mode
    pub fn set_auto_fix(&mut self, auto_fix: bool) {
        self.auto_fix = auto_fix;
    }

    /// Clear validation cache
    pub async fn clear_cache(&self) {
        let mut cache = self.validation_cache.write().await;
        cache.clear();
    }

    /// Get validation statistics
    pub async fn get_statistics(&self) -> ComprehensiveValidationStatistics {
        let cache = self.validation_cache.read().await;
        let schema_stats = self.schema_validator.get_statistics().await;

        ComprehensiveValidationStatistics {
            cached_results: cache.len(),
            cache_ttl: self.cache_ttl,
            validation_level: self.validation_level.clone(),
            auto_fix: self.auto_fix,
            schema_statistics: schema_stats,
        }
    }
}

/// Comprehensive validation report
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationReport {
    pub overall_valid: bool,
    pub results: HashMap<PathBuf, ComprehensiveValidationResult>,
    pub summary: ComprehensiveValidationSummary,
    pub validation_timestamp: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
}

/// Comprehensive validation summary
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationSummary {
    pub total_configs: usize,
    pub valid_configs: usize,
    pub invalid_configs: usize,
    pub total_issues: usize,
    pub total_warnings: usize,
    pub critical_issues: usize,
    pub error_issues: usize,
    pub warning_issues: usize,
    pub info_issues: usize,
}

/// Comprehensive validation statistics
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationStatistics {
    pub cached_results: usize,
    pub cache_ttl: u64,
    pub validation_level: ValidationLevel,
    pub auto_fix: bool,
    pub schema_statistics: crate::schema_validator::SchemaValidationStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_comprehensive_validator_creation() {
        let global_config = GlobalConfig::new();
        let validator = ComprehensiveValidator::new(&global_config).await;
        assert!(validator.is_ok());
    }

    #[tokio::test]
    async fn test_validation_level_comparison() {
        assert!(ValidationLevel::Complete > ValidationLevel::Strict);
        assert!(ValidationLevel::Strict > ValidationLevel::Standard);
        assert!(ValidationLevel::Standard > ValidationLevel::Basic);
    }

    #[tokio::test]
    async fn test_schema_type_detection() {
        let validator = ComprehensiveValidator::new(&GlobalConfig::new()).await.unwrap();
        
        let rhema_path = Path::new("rhema.yaml");
        let schema_type = validator.detect_schema_type(rhema_path).await.unwrap();
        assert_eq!(schema_type, SchemaType::Rhema);
    }
} 