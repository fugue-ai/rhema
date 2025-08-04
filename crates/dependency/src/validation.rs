use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use jsonschema::JSONSchema;
use serde_json::Value;

use crate::error::{Error, Result, ValidationResult};
use crate::types::{DependencyConfig, DependencyType, SecurityRequirements, PerformanceRequirements};
use crate::graph::DependencyGraph;

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule severity
    pub severity: ValidationSeverity,
    /// Rule configuration
    pub config: HashMap<String, Value>,
    /// Whether the rule is enabled
    pub enabled: bool,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    /// Circular dependency detection
    CircularDependency,
    /// Schema validation
    SchemaValidation,
    /// Security validation
    SecurityValidation,
    /// Performance validation
    PerformanceValidation,
    /// Naming convention validation
    NamingConvention,
    /// Configuration validation
    ConfigurationValidation,
    /// Dependency depth validation
    DependencyDepth,
    /// Health check validation
    HealthCheckValidation,
    /// Custom validation rule
    Custom(String),
}

/// Validation severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Validation engine
pub struct ValidationEngine {
    /// Validation rules
    rules: HashMap<String, ValidationRule>,
    /// JSON schemas for validation
    schemas: HashMap<String, JSONSchema>,
    /// Custom validators
    custom_validators: HashMap<String, Box<dyn CustomValidator + Send + Sync>>,
    /// Validation cache
    validation_cache: HashMap<String, ValidationResult>,
    /// Validation configuration
    config: ValidationConfig,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable validation caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Maximum validation errors to report
    pub max_errors: usize,
    /// Maximum validation warnings to report
    pub max_warnings: usize,
    /// Enable parallel validation
    pub enable_parallel: bool,
    /// Validation timeout in seconds
    pub timeout_seconds: u64,
}

/// Custom validator trait
pub trait CustomValidator {
    /// Validate a dependency configuration
    fn validate(&self, config: &DependencyConfig, graph: &DependencyGraph) -> Result<Vec<ValidationIssue>>;
}

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Issue type
    pub issue_type: String,
    /// Issue message
    pub message: String,
    /// Issue severity
    pub severity: ValidationSeverity,
    /// Affected dependency ID
    pub dependency_id: Option<String>,
    /// Issue location
    pub location: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Issue metadata
    pub metadata: HashMap<String, String>,
}

impl ValidationEngine {
    /// Create a new validation engine
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            schemas: HashMap::new(),
            custom_validators: HashMap::new(),
            validation_cache: HashMap::new(),
            config: ValidationConfig::default(),
        }
    }

    /// Create a new validation engine with configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self {
            rules: HashMap::new(),
            schemas: HashMap::new(),
            custom_validators: HashMap::new(),
            validation_cache: HashMap::new(),
            config,
        }
    }

    /// Add a validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.insert(rule.name.clone(), rule);
    }

    /// Remove a validation rule
    pub fn remove_rule(&mut self, rule_name: &str) {
        self.rules.remove(rule_name);
    }

    /// Add a JSON schema for validation
    pub fn add_schema(&mut self, name: String, schema: JSONSchema) {
        self.schemas.insert(name, schema);
    }

    /// Add a custom validator
    pub fn add_custom_validator(&mut self, name: String, validator: Box<dyn CustomValidator + Send + Sync>) {
        self.custom_validators.insert(name, validator);
    }

    /// Validate a dependency configuration
    pub fn validate_dependency(&self, config: &DependencyConfig, graph: &DependencyGraph) -> Result<ValidationResult> {
        let cache_key = format!("dependency_{}", config.id);
        
        // Check cache if enabled
        if self.config.enable_caching {
            if let Some(cached_result) = self.validation_cache.get(&cache_key) {
                return Ok(cached_result.clone());
            }
        }

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Run all enabled validation rules
        for rule in self.rules.values() {
            if !rule.enabled {
                continue;
            }

            match rule.rule_type {
                ValidationRuleType::CircularDependency => {
                    self.validate_circular_dependencies(config, graph, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::SchemaValidation => {
                    self.validate_schema(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::SecurityValidation => {
                    self.validate_security(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::PerformanceValidation => {
                    self.validate_performance(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::NamingConvention => {
                    self.validate_naming_convention(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::ConfigurationValidation => {
                    self.validate_configuration(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::DependencyDepth => {
                    self.validate_dependency_depth(config, graph, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::HealthCheckValidation => {
                    self.validate_health_check(config, rule, &mut errors, &mut warnings)?;
                }
                ValidationRuleType::Custom(ref custom_type) => {
                    self.validate_custom(config, graph, custom_type, rule, &mut errors, &mut warnings)?;
                }
            }
        }

        // Limit the number of errors and warnings
        errors.truncate(self.config.max_errors);
        warnings.truncate(self.config.max_warnings);

        let result = ValidationResult {
            valid: errors.is_empty(),
            errors: errors.into_iter().map(|issue| issue.message).collect(),
            warnings: warnings.into_iter().map(|issue| issue.message).collect(),
            timestamp: Utc::now(),
        };

        // Cache the result if enabled
        if self.config.enable_caching {
            // In a real implementation, you'd want to use a proper cache with TTL
            // For now, we'll just store it in the HashMap
            // self.validation_cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    /// Validate the entire dependency graph
    pub fn validate_graph(&self, graph: &DependencyGraph) -> Result<ValidationResult> {
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        // Validate each dependency
        for config in graph.get_all_dependency_configs() {
            let result = self.validate_dependency(config, graph)?;
            all_errors.extend(result.errors);
            all_warnings.extend(result.warnings);
        }

        // Validate graph-level rules
        // Note: We can't call validate_graph_level_rules here because it expects ValidationIssue
        // but we have strings. This would need to be refactored to use consistent types.

        // Limit the number of errors and warnings
        all_errors.truncate(self.config.max_errors);
        all_warnings.truncate(self.config.max_warnings);

        Ok(ValidationResult {
            valid: all_errors.is_empty(),
            errors: all_errors,
            warnings: all_warnings,
            timestamp: Utc::now(),
        })
    }

    /// Validate circular dependencies
    fn validate_circular_dependencies(
        &self,
        config: &DependencyConfig,
        graph: &DependencyGraph,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if graph.has_circular_dependencies()? {
            let issue = ValidationIssue {
                issue_type: "circular_dependency".to_string(),
                message: format!("Circular dependency detected involving {}", config.id),
                severity: rule.severity.clone(),
                dependency_id: Some(config.id.clone()),
                location: None,
                suggested_fix: Some("Review and remove circular dependencies".to_string()),
                metadata: HashMap::new(),
            };

            match rule.severity {
                ValidationSeverity::Error | ValidationSeverity::Critical => {
                    errors.push(issue);
                }
                _ => {
                    warnings.push(issue);
                }
            }
        }

        Ok(())
    }

    /// Validate schema
    fn validate_schema(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if let Some(schema_name) = rule.config.get("schema") {
            if let Some(schema) = self.schemas.get(schema_name.as_str().unwrap()) {
                let config_value = serde_json::to_value(config)
                    .map_err(|e| Error::SchemaValidation(e.to_string()))?;

                let validation_result = schema.validate(&config_value);
                if let Err(validation_errors) = validation_result {
                    for error in validation_errors {
                        let issue = ValidationIssue {
                            issue_type: "schema_validation".to_string(),
                            message: format!("Schema validation failed: {}", error),
                            severity: rule.severity.clone(),
                            dependency_id: Some(config.id.clone()),
                            location: Some(error.instance_path.to_string()),
                            suggested_fix: None,
                            metadata: HashMap::new(),
                        };

                        match rule.severity {
                            ValidationSeverity::Error | ValidationSeverity::Critical => {
                                errors.push(issue);
                            }
                            _ => {
                                warnings.push(issue);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate security requirements
    fn validate_security(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if let Some(security_req) = &config.security_requirements {
            // Check authentication requirement
            if security_req.authentication_required {
                if !config.operations.iter().any(|op| op.contains("auth") || op.contains("token")) {
                    let issue = ValidationIssue {
                        issue_type: "security_validation".to_string(),
                        message: "Authentication required but no auth operations found".to_string(),
                        severity: rule.severity.clone(),
                        dependency_id: Some(config.id.clone()),
                        location: Some("security_requirements".to_string()),
                        suggested_fix: Some("Add authentication operations".to_string()),
                        metadata: HashMap::new(),
                    };

                    match rule.severity {
                        ValidationSeverity::Error | ValidationSeverity::Critical => {
                            errors.push(issue);
                        }
                        _ => {
                            warnings.push(issue);
                        }
                    }
                }
            }

            // Check encryption requirement
            if security_req.encryption_required {
                if !config.target.starts_with("https://") && !config.target.starts_with("wss://") {
                    let issue = ValidationIssue {
                        issue_type: "security_validation".to_string(),
                        message: "Encryption required but target does not use secure protocol".to_string(),
                        severity: rule.severity.clone(),
                        dependency_id: Some(config.id.clone()),
                        location: Some("target".to_string()),
                        suggested_fix: Some("Use HTTPS or WSS protocol".to_string()),
                        metadata: HashMap::new(),
                    };

                    match rule.severity {
                        ValidationSeverity::Error | ValidationSeverity::Critical => {
                            errors.push(issue);
                        }
                        _ => {
                            warnings.push(issue);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate performance requirements
    fn validate_performance(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if let Some(perf_req) = &config.performance_requirements {
            // Check if performance requirements are reasonable
            if perf_req.max_response_time_ms > 5000.0 {
                let issue = ValidationIssue {
                    issue_type: "performance_validation".to_string(),
                    message: "Maximum response time is very high".to_string(),
                    severity: rule.severity.clone(),
                    dependency_id: Some(config.id.clone()),
                    location: Some("performance_requirements.max_response_time_ms".to_string()),
                    suggested_fix: Some("Consider reducing maximum response time".to_string()),
                    metadata: HashMap::new(),
                };

                match rule.severity {
                    ValidationSeverity::Error | ValidationSeverity::Critical => {
                        errors.push(issue);
                    }
                    _ => {
                        warnings.push(issue);
                    }
                }
            }

            if perf_req.min_availability < 0.95 {
                let issue = ValidationIssue {
                    issue_type: "performance_validation".to_string(),
                    message: "Minimum availability is below recommended threshold".to_string(),
                    severity: rule.severity.clone(),
                    dependency_id: Some(config.id.clone()),
                    location: Some("performance_requirements.min_availability".to_string()),
                    suggested_fix: Some("Consider increasing minimum availability".to_string()),
                    metadata: HashMap::new(),
                };

                match rule.severity {
                    ValidationSeverity::Error | ValidationSeverity::Critical => {
                        errors.push(issue);
                    }
                    _ => {
                        warnings.push(issue);
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate naming convention
    fn validate_naming_convention(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        // Check if ID follows naming convention
        if let Some(pattern) = rule.config.get("id_pattern") {
            if let Some(pattern_str) = pattern.as_str() {
                let regex = regex::Regex::new(pattern_str)
                    .map_err(|e| Error::Validation(format!("Invalid regex pattern: {}", e)))?;

                if !regex.is_match(&config.id) {
                    let issue = ValidationIssue {
                        issue_type: "naming_convention".to_string(),
                        message: format!("ID '{}' does not match pattern '{}'", config.id, pattern_str),
                        severity: rule.severity.clone(),
                        dependency_id: Some(config.id.clone()),
                        location: Some("id".to_string()),
                        suggested_fix: Some("Update ID to match naming convention".to_string()),
                        metadata: HashMap::new(),
                    };

                    match rule.severity {
                        ValidationSeverity::Error | ValidationSeverity::Critical => {
                            errors.push(issue);
                        }
                        _ => {
                            warnings.push(issue);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate configuration
    fn validate_configuration(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        // Validate required fields
        if config.id.is_empty() {
            let issue = ValidationIssue {
                issue_type: "configuration_validation".to_string(),
                message: "Dependency ID cannot be empty".to_string(),
                severity: rule.severity.clone(),
                dependency_id: None,
                location: Some("id".to_string()),
                suggested_fix: Some("Provide a valid dependency ID".to_string()),
                metadata: HashMap::new(),
            };

            match rule.severity {
                ValidationSeverity::Error | ValidationSeverity::Critical => {
                    errors.push(issue);
                }
                _ => {
                    warnings.push(issue);
                }
            }
        }

        if config.target.is_empty() {
            let issue = ValidationIssue {
                issue_type: "configuration_validation".to_string(),
                message: "Dependency target cannot be empty".to_string(),
                severity: rule.severity.clone(),
                dependency_id: Some(config.id.clone()),
                location: Some("target".to_string()),
                suggested_fix: Some("Provide a valid dependency target".to_string()),
                metadata: HashMap::new(),
            };

            match rule.severity {
                ValidationSeverity::Error | ValidationSeverity::Critical => {
                    errors.push(issue);
                }
                _ => {
                    warnings.push(issue);
                }
            }
        }

        if config.operations.is_empty() {
            let issue = ValidationIssue {
                issue_type: "configuration_validation".to_string(),
                message: "Dependency must have at least one operation".to_string(),
                severity: rule.severity.clone(),
                dependency_id: Some(config.id.clone()),
                location: Some("operations".to_string()),
                suggested_fix: Some("Add at least one operation".to_string()),
                metadata: HashMap::new(),
            };

            match rule.severity {
                ValidationSeverity::Error | ValidationSeverity::Critical => {
                    errors.push(issue);
                }
                _ => {
                    warnings.push(issue);
                }
            }
        }

        Ok(())
    }

    /// Validate dependency depth
    fn validate_dependency_depth(
        &self,
        config: &DependencyConfig,
        graph: &DependencyGraph,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        let max_depth = rule.config
            .get("max_depth")
            .and_then(|v| v.as_u64())
            .unwrap_or(5);

        let dependents = graph.get_dependents(&config.id)?;
        if dependents.len() > max_depth as usize {
            let issue = ValidationIssue {
                issue_type: "dependency_depth".to_string(),
                message: format!("Dependency has too many dependents: {} (max: {})", dependents.len(), max_depth),
                severity: rule.severity.clone(),
                dependency_id: Some(config.id.clone()),
                location: None,
                suggested_fix: Some("Consider breaking down the dependency".to_string()),
                metadata: HashMap::new(),
            };

            match rule.severity {
                ValidationSeverity::Error | ValidationSeverity::Critical => {
                    errors.push(issue);
                }
                _ => {
                    warnings.push(issue);
                }
            }
        }

        Ok(())
    }

    /// Validate health check configuration
    fn validate_health_check(
        &self,
        config: &DependencyConfig,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if let Some(health_check) = &config.health_check {
            // Validate health check URL
            if health_check.url.is_empty() {
                let issue = ValidationIssue {
                    issue_type: "health_check_validation".to_string(),
                    message: "Health check URL cannot be empty".to_string(),
                    severity: rule.severity.clone(),
                    dependency_id: Some(config.id.clone()),
                    location: Some("health_check.url".to_string()),
                    suggested_fix: Some("Provide a valid health check URL".to_string()),
                    metadata: HashMap::new(),
                };

                match rule.severity {
                    ValidationSeverity::Error | ValidationSeverity::Critical => {
                        errors.push(issue);
                    }
                    _ => {
                        warnings.push(issue);
                    }
                }
            }

            // Validate health check interval
            if health_check.interval_seconds == 0 {
                let issue = ValidationIssue {
                    issue_type: "health_check_validation".to_string(),
                    message: "Health check interval cannot be zero".to_string(),
                    severity: rule.severity.clone(),
                    dependency_id: Some(config.id.clone()),
                    location: Some("health_check.interval_seconds".to_string()),
                    suggested_fix: Some("Set a positive health check interval".to_string()),
                    metadata: HashMap::new(),
                };

                match rule.severity {
                    ValidationSeverity::Error | ValidationSeverity::Critical => {
                        errors.push(issue);
                    }
                    _ => {
                        warnings.push(issue);
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate custom rules
    fn validate_custom(
        &self,
        config: &DependencyConfig,
        graph: &DependencyGraph,
        custom_type: &str,
        rule: &ValidationRule,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        if let Some(validator) = self.custom_validators.get(custom_type) {
            match validator.validate(config, graph) {
                Ok(issues) => {
                    for issue in issues {
                        match rule.severity {
                            ValidationSeverity::Error | ValidationSeverity::Critical => {
                                errors.push(issue);
                            }
                            _ => {
                                warnings.push(issue);
                            }
                        }
                    }
                }
                Err(e) => {
                    let issue = ValidationIssue {
                        issue_type: "custom_validation".to_string(),
                        message: format!("Custom validation failed: {}", e),
                        severity: rule.severity.clone(),
                        dependency_id: Some(config.id.clone()),
                        location: None,
                        suggested_fix: None,
                        metadata: HashMap::new(),
                    };

                    match rule.severity {
                        ValidationSeverity::Error | ValidationSeverity::Critical => {
                            errors.push(issue);
                        }
                        _ => {
                            warnings.push(issue);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate graph-level rules
    fn validate_graph_level_rules(
        &self,
        graph: &DependencyGraph,
        errors: &mut Vec<ValidationIssue>,
        warnings: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        // Check for orphaned dependencies (no dependents and no dependencies)
        for config in graph.get_all_dependency_configs() {
            let dependents = graph.get_dependents(&config.id)?;
            let dependencies = graph.get_dependencies(&config.id)?;

            if dependents.is_empty() && dependencies.is_empty() {
                let issue = ValidationIssue {
                    issue_type: "orphaned_dependency".to_string(),
                    message: format!("Dependency '{}' is orphaned (no connections)", config.id),
                    severity: ValidationSeverity::Warning,
                    dependency_id: Some(config.id.clone()),
                    location: None,
                    suggested_fix: Some("Connect the dependency or remove it if not needed".to_string()),
                    metadata: HashMap::new(),
                };
                warnings.push(issue);
            }
        }

        Ok(())
    }

    /// Get validation statistics
    pub fn get_statistics(&self) -> ValidationStatistics {
        let mut total_validations = 0;
        let mut successful_validations = 0;
        let mut failed_validations = 0;
        let mut total_errors = 0;
        let mut total_warnings = 0;

        for result in self.validation_cache.values() {
            total_validations += 1;
            if result.valid {
                successful_validations += 1;
            } else {
                failed_validations += 1;
            }
            total_errors += result.errors.len();
            total_warnings += result.warnings.len();
        }

        ValidationStatistics {
            total_validations,
            successful_validations,
            failed_validations,
            total_errors,
            total_warnings,
            success_rate: if total_validations > 0 {
                successful_validations as f64 / total_validations as f64
            } else {
                0.0
            },
            last_updated: Utc::now(),
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_ttl_seconds: 300,
            max_errors: 100,
            max_warnings: 200,
            enable_parallel: true,
            timeout_seconds: 30,
        }
    }
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatistics {
    /// Total number of validations
    pub total_validations: usize,
    /// Number of successful validations
    pub successful_validations: usize,
    /// Number of failed validations
    pub failed_validations: usize,
    /// Total number of errors
    pub total_errors: usize,
    /// Total number of warnings
    pub total_warnings: usize,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    fn create_test_config(id: &str, name: &str) -> DependencyConfig {
        DependencyConfig::new(
            id.to_string(),
            name.to_string(),
            crate::types::DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).unwrap()
    }

    #[test]
    fn test_validation_engine_new() {
        let engine = ValidationEngine::new();
        assert_eq!(engine.rules.len(), 0);
        assert_eq!(engine.schemas.len(), 0);
        assert_eq!(engine.custom_validators.len(), 0);
    }

    #[test]
    fn test_validation_config_default() {
        let config = ValidationConfig::default();
        assert!(config.enable_caching);
        assert_eq!(config.cache_ttl_seconds, 300);
        assert_eq!(config.max_errors, 100);
        assert_eq!(config.max_warnings, 200);
        assert!(config.enable_parallel);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_validation_severity() {
        assert_eq!(ValidationSeverity::Info, ValidationSeverity::Info);
        assert_ne!(ValidationSeverity::Info, ValidationSeverity::Critical);
    }

    #[test]
    fn test_validation_rule() {
        let rule = ValidationRule {
            name: "test_rule".to_string(),
            description: "Test rule".to_string(),
            rule_type: ValidationRuleType::SchemaValidation,
            severity: ValidationSeverity::Error,
            config: HashMap::new(),
            enabled: true,
        };

        assert_eq!(rule.name, "test_rule");
        assert_eq!(rule.severity, ValidationSeverity::Error);
        assert!(rule.enabled);
    }

    #[test]
    fn test_validation_issue() {
        let issue = ValidationIssue {
            issue_type: "test_issue".to_string(),
            message: "Test message".to_string(),
            severity: ValidationSeverity::Warning,
            dependency_id: Some("test_id".to_string()),
            location: Some("test_location".to_string()),
            suggested_fix: Some("Test fix".to_string()),
            metadata: HashMap::new(),
        };

        assert_eq!(issue.issue_type, "test_issue");
        assert_eq!(issue.severity, ValidationSeverity::Warning);
        assert_eq!(issue.dependency_id, Some("test_id".to_string()));
    }
} 