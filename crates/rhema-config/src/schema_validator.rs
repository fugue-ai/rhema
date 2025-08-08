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

use crate::{ConfigError, ConfigIssueSeverity};
use jsonschema::{Draft, JSONSchema};
use rhema_core::RhemaResult;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Schema validator for Rhema configuration files
pub struct SchemaValidator {
    schemas: Arc<RwLock<HashMap<String, JSONSchema>>>,
    schema_cache: Arc<RwLock<HashMap<PathBuf, SchemaValidationResult>>>,
    cache_ttl: u64,
    strict_mode: bool,
}

/// Schema validation result
#[derive(Debug, Clone)]
pub struct SchemaValidationResult {
    pub valid: bool,
    pub issues: Vec<SchemaValidationIssue>,
    pub warnings: Vec<String>,
    pub schema_version: Option<String>,
    pub validation_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Schema validation issue
#[derive(Debug, Clone)]
pub struct SchemaValidationIssue {
    pub severity: ConfigIssueSeverity,
    pub path: String,
    pub message: String,
    pub code: String,
    pub details: Option<Value>,
}

/// Schema type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaType {
    Rhema,
    Scope,
    Knowledge,
    Todos,
    Decisions,
    Patterns,
    Conventions,
    Lock,
    Action,
    Global,
    Repository,
    Project,
    Performance,
    Security,
    Compliance,
    Dependencies,
    Custom(String),
}

impl SchemaType {
    pub fn as_str(&self) -> &str {
        match self {
            SchemaType::Rhema => "rhema",
            SchemaType::Scope => "scope",
            SchemaType::Knowledge => "knowledge",
            SchemaType::Todos => "todos",
            SchemaType::Decisions => "decisions",
            SchemaType::Patterns => "patterns",
            SchemaType::Conventions => "conventions",
            SchemaType::Lock => "lock",
            SchemaType::Action => "action",
            SchemaType::Global => "global",
            SchemaType::Repository => "repository",
            SchemaType::Project => "project",
            SchemaType::Performance => "performance",
            SchemaType::Security => "security",
            SchemaType::Compliance => "compliance",
            SchemaType::Dependencies => "dependencies",
            SchemaType::Custom(name) => name,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "rhema" => SchemaType::Rhema,
            "scope" => SchemaType::Scope,
            "knowledge" => SchemaType::Knowledge,
            "todos" => SchemaType::Todos,
            "decisions" => SchemaType::Decisions,
            "patterns" => SchemaType::Patterns,
            "conventions" => SchemaType::Conventions,
            "lock" => SchemaType::Lock,
            "action" => SchemaType::Action,
            "global" => SchemaType::Global,
            "repository" => SchemaType::Repository,
            "project" => SchemaType::Project,
            "performance" => SchemaType::Performance,
            "security" => SchemaType::Security,
            "compliance" => SchemaType::Compliance,
            "dependencies" => SchemaType::Dependencies,
            custom => SchemaType::Custom(custom.to_string()),
        }
    }
}

impl SchemaValidator {
    /// Create a new schema validator
    pub fn new() -> RhemaResult<Self> {
        let mut validator = Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
            schema_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: 300, // 5 minutes default
            strict_mode: false,
        };

        // Load built-in schemas
        validator.load_builtin_schemas()?;

        Ok(validator)
    }

    /// Create a new schema validator with custom settings
    pub fn with_settings(cache_ttl: u64, strict_mode: bool) -> RhemaResult<Self> {
        let mut validator = Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
            schema_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
            strict_mode,
        };

        // Load built-in schemas
        validator.load_builtin_schemas()?;

        Ok(validator)
    }

    /// Load built-in schemas from the schemas directory
    fn load_builtin_schemas(&mut self) -> RhemaResult<()> {
        // Try multiple possible paths for schema files
        let possible_paths = [
            "schemas",
            "../../schemas",
            "../../../schemas",
            "crates/rhema-config/schemas",
        ];

        let schema_files = [
            ("rhema", "rhema.json"),
            ("scope", "scope.json"),
            ("knowledge", "knowledge.json"),
            ("todos", "todos.json"),
            ("decisions", "decisions.json"),
            ("patterns", "patterns.json"),
            ("conventions", "conventions.json"),
            ("lock", "lock.json"),
            ("action", "action.json"),
        ];

        for (name, filename) in schema_files {
            let mut loaded = false;
            for base_path in &possible_paths {
                let full_path = format!("{}/{}", base_path, filename);
                if let Ok(schema) = self.load_schema_from_file(&full_path) {
                    let mut schemas = self.schemas.try_write().map_err(|_| {
                        ConfigError::ValidationError(
                            "Failed to acquire write lock for schemas".to_string(),
                        )
                    })?;
                    schemas.insert(name.to_string(), schema);
                    info!("Loaded schema: {} from {}", name, full_path);
                    loaded = true;
                    break;
                }
            }
            if !loaded {
                warn!("Failed to load schema: {} from any path", name);
            }
        }

        Ok(())
    }

    /// Load a schema from a file
    pub fn load_schema_from_file<P: AsRef<Path>>(&self, path: P) -> RhemaResult<JSONSchema> {
        let path = path.as_ref();
        debug!("Attempting to load schema from: {}", path.display());

        if !path.exists() {
            return Err(ConfigError::ValidationError(format!(
                "Schema file not found: {}",
                path.display()
            ))
            .into());
        }

        let content = std::fs::read_to_string(path)?;
        let schema_value: Value = serde_json::from_str(&content)?;
        let schema = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema_value)
            .map_err(|e| {
                ConfigError::ValidationError(format!("Schema compilation error: {}", e))
            })?;

        debug!("Successfully loaded schema from: {}", path.display());
        Ok(schema)
    }

    /// Load a custom schema
    pub async fn load_custom_schema(&self, name: &str, schema: JSONSchema) -> RhemaResult<()> {
        let mut schemas = self.schemas.write().await;
        schemas.insert(name.to_string(), schema);
        info!("Loaded custom schema: {}", name);
        Ok(())
    }

    /// Validate a configuration against a specific schema
    pub async fn validate_against_schema(
        &self,
        config: &Value,
        schema_type: &SchemaType,
    ) -> RhemaResult<SchemaValidationResult> {
        let schema_name = schema_type.as_str();
        let schemas = self.schemas.read().await;

        if let Some(schema) = schemas.get(schema_name) {
            self.validate_with_schema(config, schema, schema_name).await
        } else {
            Err(ConfigError::ValidationError(format!("Schema not found: {}", schema_name)).into())
        }
    }

    /// Validate a configuration with a specific schema
    async fn validate_with_schema(
        &self,
        config: &Value,
        schema: &JSONSchema,
        schema_name: &str,
    ) -> RhemaResult<SchemaValidationResult> {
        let start_time = std::time::Instant::now();
        let validation_result = schema.validate(config);

        let mut issues = Vec::new();
        let warnings = Vec::new();

        match validation_result {
            Ok(_) => {
                debug!("Schema validation passed for {}", schema_name);
            }
            Err(errors) => {
                for error in errors {
                    let severity = if self.strict_mode {
                        ConfigIssueSeverity::Error
                    } else {
                        ConfigIssueSeverity::Warning
                    };

                    let issue = SchemaValidationIssue {
                        severity,
                        path: error.instance_path.to_string(),
                        message: error.to_string(),
                        code: "SCHEMA_VALIDATION_ERROR".to_string(),
                        details: Some(serde_json::json!({
                            "error": error.to_string(),
                            "instance_path": error.instance_path.to_string(),
                            "schema_path": error.schema_path.to_string()
                        })),
                    };

                    issues.push(issue);
                }
            }
        }

        // Additional semantic validations
        let semantic_issues = self.validate_semantic_rules(config, schema_name).await?;
        issues.extend(semantic_issues);

        let _duration = start_time.elapsed();
        let valid = issues
            .iter()
            .all(|issue| issue.severity != ConfigIssueSeverity::Error);

        Ok(SchemaValidationResult {
            valid,
            issues,
            warnings,
            schema_version: self.extract_schema_version(schema),
            validation_timestamp: chrono::Utc::now(),
        })
    }

    /// Validate semantic rules beyond JSON Schema
    async fn validate_semantic_rules(
        &self,
        config: &Value,
        schema_name: &str,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        match schema_name {
            "rhema" => {
                issues.extend(self.validate_rhema_semantics(config).await?);
            }
            "scope" => {
                issues.extend(self.validate_scope_semantics(config).await?);
            }
            "knowledge" => {
                issues.extend(self.validate_knowledge_semantics(config).await?);
            }
            "todos" => {
                issues.extend(self.validate_todos_semantics(config).await?);
            }
            "decisions" => {
                issues.extend(self.validate_decisions_semantics(config).await?);
            }
            "patterns" => {
                issues.extend(self.validate_patterns_semantics(config).await?);
            }
            "conventions" => {
                issues.extend(self.validate_conventions_semantics(config).await?);
            }
            "lock" => {
                issues.extend(self.validate_lock_semantics(config).await?);
            }
            "action" => {
                issues.extend(self.validate_action_semantics(config).await?);
            }
            _ => {
                // Custom schema - no additional semantic validation
            }
        }

        Ok(issues)
    }

    /// Validate Rhema-specific semantic rules
    async fn validate_rhema_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check version compatibility
        if let Some(version) = config.get("rhema").and_then(|r| r.get("version")) {
            if let Some(version_str) = version.as_str() {
                if !self.is_valid_rhema_version(version_str) {
                    issues.push(SchemaValidationIssue {
                        severity: ConfigIssueSeverity::Warning,
                        path: "rhema.version".to_string(),
                        message: format!("Unsupported Rhema version: {}", version_str),
                        code: "UNSUPPORTED_VERSION".to_string(),
                        details: None,
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Validate scope-specific semantic rules
    async fn validate_scope_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check scope type validity
        if let Some(scope_type) = config
            .get("rhema")
            .and_then(|r| r.get("scope"))
            .and_then(|s| s.get("type"))
        {
            if let Some(type_str) = scope_type.as_str() {
                let valid_types = [
                    "repository",
                    "service",
                    "application",
                    "library",
                    "component",
                ];
                if !valid_types.contains(&type_str) {
                    issues.push(SchemaValidationIssue {
                        severity: ConfigIssueSeverity::Error,
                        path: "rhema.scope.type".to_string(),
                        message: format!("Invalid scope type: {}", type_str),
                        code: "INVALID_SCOPE_TYPE".to_string(),
                        details: None,
                    });
                }
            }
        }

        // Check for required dependencies
        if let Some(dependencies) = config.get("dependencies") {
            if let Some(deps_array) = dependencies.as_array() {
                for dep in deps_array {
                    if let Some(dep_obj) = dep.as_object() {
                        if !dep_obj.contains_key("name") || !dep_obj.contains_key("version") {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: "dependencies".to_string(),
                                message: "Dependency missing required fields: name or version"
                                    .to_string(),
                                code: "INCOMPLETE_DEPENDENCY".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate knowledge-specific semantic rules
    async fn validate_knowledge_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for duplicate knowledge entries
        if let Some(entries) = config.get("knowledge") {
            if let Some(entries_array) = entries.as_array() {
                let mut seen_titles = std::collections::HashSet::new();
                for (index, entry) in entries_array.iter().enumerate() {
                    if let Some(title) = entry.get("title").and_then(|t| t.as_str()) {
                        if !seen_titles.insert(title) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: format!("knowledge[{}]", index),
                                message: format!("Duplicate knowledge title: {}", title),
                                code: "DUPLICATE_KNOWLEDGE".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate todos-specific semantic rules
    async fn validate_todos_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for circular dependencies in todos
        if let Some(todos) = config.get("todos") {
            if let Some(todos_array) = todos.as_array() {
                let mut dependencies = std::collections::HashMap::new();

                for (_index, todo) in todos_array.iter().enumerate() {
                    if let Some(id) = todo.get("id").and_then(|i| i.as_str()) {
                        if let Some(deps) = todo.get("dependencies").and_then(|d| d.as_array()) {
                            let deps_vec: Vec<String> = deps
                                .iter()
                                .filter_map(|d| d.as_str().map(|s| s.to_string()))
                                .collect();
                            dependencies.insert(id.to_string(), deps_vec);
                        }
                    }
                }

                // Check for circular dependencies
                for (id, _deps) in &dependencies {
                    if self.has_circular_dependency(
                        id,
                        &dependencies,
                        &mut std::collections::HashSet::new(),
                    ) {
                        issues.push(SchemaValidationIssue {
                            severity: ConfigIssueSeverity::Error,
                            path: format!("todos.{}", id),
                            message: format!("Circular dependency detected for todo: {}", id),
                            code: "CIRCULAR_DEPENDENCY".to_string(),
                            details: None,
                        });
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate decisions-specific semantic rules
    async fn validate_decisions_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for decision status consistency
        if let Some(decisions) = config.get("decisions") {
            if let Some(decisions_array) = decisions.as_array() {
                for (index, decision) in decisions_array.iter().enumerate() {
                    if let Some(status) = decision.get("status").and_then(|s| s.as_str()) {
                        let valid_statuses = [
                            "proposed",
                            "accepted",
                            "rejected",
                            "deprecated",
                            "superseded",
                        ];
                        if !valid_statuses.contains(&status) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: format!("decisions[{}].status", index),
                                message: format!("Invalid decision status: {}", status),
                                code: "INVALID_DECISION_STATUS".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate patterns-specific semantic rules
    async fn validate_patterns_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for pattern category consistency
        if let Some(patterns) = config.get("patterns") {
            if let Some(patterns_array) = patterns.as_array() {
                for (index, pattern) in patterns_array.iter().enumerate() {
                    if let Some(category) = pattern.get("category").and_then(|c| c.as_str()) {
                        let valid_categories =
                            ["architectural", "design", "coding", "testing", "deployment"];
                        if !valid_categories.contains(&category) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: format!("patterns[{}].category", index),
                                message: format!("Invalid pattern category: {}", category),
                                code: "INVALID_PATTERN_CATEGORY".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate conventions-specific semantic rules
    async fn validate_conventions_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for convention scope consistency
        if let Some(conventions) = config.get("conventions") {
            if let Some(conventions_array) = conventions.as_array() {
                for (index, convention) in conventions_array.iter().enumerate() {
                    if let Some(scope) = convention.get("scope").and_then(|s| s.as_str()) {
                        let valid_scopes = ["global", "repository", "service", "component"];
                        if !valid_scopes.contains(&scope) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: format!("conventions[{}].scope", index),
                                message: format!("Invalid convention scope: {}", scope),
                                code: "INVALID_CONVENTION_SCOPE".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate lock-specific semantic rules
    async fn validate_lock_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for lock consistency
        if let Some(locks) = config.get("locks") {
            if let Some(locks_obj) = locks.as_object() {
                let mut agent_locks = std::collections::HashMap::new();

                for (resource, lock_info) in locks_obj {
                    if let Some(agent) = lock_info.get("agent").and_then(|a| a.as_str()) {
                        if agent_locks.contains_key(agent) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Error,
                                path: format!("locks.{}", resource),
                                message: format!("Agent {} already holds a lock", agent),
                                code: "AGENT_MULTIPLE_LOCKS".to_string(),
                                details: None,
                            });
                        } else {
                            agent_locks.insert(agent.to_string(), resource.clone());
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate action-specific semantic rules
    async fn validate_action_semantics(
        &self,
        config: &Value,
    ) -> RhemaResult<Vec<SchemaValidationIssue>> {
        let mut issues = Vec::new();

        // Check for action type consistency
        if let Some(actions) = config.get("actions") {
            if let Some(actions_array) = actions.as_array() {
                for (index, action) in actions_array.iter().enumerate() {
                    if let Some(action_type) = action.get("type").and_then(|t| t.as_str()) {
                        let valid_types = ["create", "update", "delete", "move", "copy", "custom"];
                        if !valid_types.contains(&action_type) {
                            issues.push(SchemaValidationIssue {
                                severity: ConfigIssueSeverity::Warning,
                                path: format!("actions[{}].type", index),
                                message: format!("Invalid action type: {}", action_type),
                                code: "INVALID_ACTION_TYPE".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Check if a Rhema version is valid
    fn is_valid_rhema_version(&self, version: &str) -> bool {
        // Basic version validation - can be enhanced with more sophisticated logic
        version.starts_with("1.") || version.starts_with("0.")
    }

    /// Check for circular dependencies
    fn has_circular_dependency(
        &self,
        id: &str,
        dependencies: &std::collections::HashMap<String, Vec<String>>,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if visited.contains(id) {
            return true;
        }

        visited.insert(id.to_string());

        if let Some(deps) = dependencies.get(id) {
            for dep in deps {
                if self.has_circular_dependency(dep, dependencies, visited) {
                    return true;
                }
            }
        }

        visited.remove(id);
        false
    }

    /// Extract schema version from schema
    fn extract_schema_version(&self, _schema: &JSONSchema) -> Option<String> {
        // This would need to be implemented based on how version is stored in the schema
        Some("1.0.0".to_string())
    }

    /// Set strict mode
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Set cache TTL
    pub fn set_cache_ttl(&mut self, ttl: u64) {
        self.cache_ttl = ttl;
    }

    /// Clear validation cache
    pub async fn clear_cache(&self) {
        let mut cache = self.schema_cache.write().await;
        cache.clear();
    }

    /// Get validation statistics
    pub async fn get_statistics(&self) -> SchemaValidationStatistics {
        let schemas = self.schemas.read().await;
        let cache = self.schema_cache.read().await;

        SchemaValidationStatistics {
            loaded_schemas: schemas.len(),
            cached_results: cache.len(),
            cache_ttl: self.cache_ttl,
            strict_mode: self.strict_mode,
        }
    }
}

/// Schema validation statistics
#[derive(Debug, Clone)]
pub struct SchemaValidationStatistics {
    pub loaded_schemas: usize,
    pub cached_results: usize,
    pub cache_ttl: u64,
    pub strict_mode: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_schema_validator_creation() {
        let validator = SchemaValidator::new();
        assert!(validator.is_ok());
    }

    #[tokio::test]
    async fn test_rhema_schema_validation() {
        let validator = SchemaValidator::new().unwrap();

        let valid_config = json!({
            "scope": {
                "type": "repository",
                "name": "test-repo"
            }
        });

        let result = validator
            .validate_against_schema(&valid_config, &SchemaType::Scope)
            .await;
        if let Err(e) = &result {
            eprintln!("Validation error: {:?}", e);
        }
        assert!(result.is_ok());

        let validation_result = result.unwrap();
        assert!(validation_result.valid);
    }

    #[tokio::test]
    async fn test_invalid_schema_validation() {
        let validator = SchemaValidator::new().unwrap();

        let invalid_config = json!({
            "scope": {
                "type": "invalid-type",
                "name": ""
            }
        });

        let result = validator
            .validate_against_schema(&invalid_config, &SchemaType::Scope)
            .await;
        if let Err(e) = &result {
            eprintln!("Validation error: {:?}", e);
        }
        assert!(result.is_ok());

        let validation_result = result.unwrap();
        // Should have issues due to invalid scope type and empty name
        assert!(!validation_result.issues.is_empty());
    }

    #[tokio::test]
    async fn test_schema_type_conversion() {
        assert_eq!(SchemaType::Rhema.as_str(), "rhema");
        assert_eq!(SchemaType::from_str("rhema"), SchemaType::Rhema);
        assert_eq!(
            SchemaType::from_str("custom"),
            SchemaType::Custom("custom".to_string())
        );
    }
}
