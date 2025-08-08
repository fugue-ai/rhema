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
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, info, warn};

/// Validation rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRulesConfig {
    pub rules: Vec<ValidationRule>,
    pub rule_sets: HashMap<String, RuleSet>,
    pub global_settings: GlobalValidationSettings,
    pub schema_overrides: HashMap<String, SchemaOverride>,
    pub custom_validators: HashMap<String, CustomValidatorConfig>,
}

/// Individual validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub severity: ConfigIssueSeverity,
    pub enabled: bool,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<RuleAction>,
    pub metadata: HashMap<String, Value>,
}

/// Rule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleType {
    Schema,
    Format,
    Security,
    Performance,
    Compliance,
    Business,
    Custom,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: Value,
    pub case_sensitive: Option<bool>,
}

/// Condition operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConditionOperator {
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
    In,
    NotIn,
    IsEmpty,
    IsNotEmpty,
    IsNull,
    IsNotNull,
}

/// Rule action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub action_type: ActionType,
    pub parameters: HashMap<String, Value>,
    pub enabled: bool,
}

/// Action type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    Log,
    Warn,
    Error,
    Fix,
    Skip,
    Transform,
    Custom,
}

/// Rule set for grouping related rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub name: String,
    pub description: String,
    pub rules: Vec<String>, // Rule IDs
    pub enabled: bool,
    pub priority: u32,
}

/// Global validation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalValidationSettings {
    pub strict_mode: bool,
    pub auto_fix: bool,
    pub fail_fast: bool,
    pub max_issues: Option<usize>,
    pub cache_enabled: bool,
    pub cache_ttl: u64,
    pub parallel_validation: bool,
    pub max_parallel: usize,
}

/// Schema override for customizing schema validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOverride {
    pub schema_path: String,
    pub overrides: HashMap<String, Value>,
    pub additional_properties: bool,
    pub strict_validation: bool,
}

/// Custom validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidatorConfig {
    pub name: String,
    pub description: String,
    pub validator_type: String,
    pub parameters: HashMap<String, Value>,
    pub enabled: bool,
    pub priority: u32,
}

/// Validation rules manager
pub struct ValidationRulesManager {
    config: ValidationRulesConfig,
    rule_cache: HashMap<String, ValidationRule>,
    compiled_rules: HashMap<String, CompiledRule>,
}

/// Compiled rule for efficient evaluation
pub struct CompiledRule {
    pub rule: ValidationRule,
    pub condition_tree: ConditionTree,
    pub action_handlers: Vec<ActionHandler>,
}

/// Condition tree for complex condition evaluation
#[derive(Debug, Clone)]
pub enum ConditionTree {
    Leaf(RuleCondition),
    And(Vec<ConditionTree>),
    Or(Vec<ConditionTree>),
    Not(Box<ConditionTree>),
}

/// Action handler for rule actions
pub struct ActionHandler {
    pub action: RuleAction,
    pub handler: Box<dyn Fn(&Value, &ValidationRule) -> RhemaResult<()> + Send + Sync>,
}

impl ValidationRulesConfig {
    /// Create a new validation rules configuration
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            rule_sets: HashMap::new(),
            global_settings: GlobalValidationSettings::default(),
            schema_overrides: HashMap::new(),
            custom_validators: HashMap::new(),
        }
    }

    /// Load validation rules from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> RhemaResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ValidationRulesConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save validation rules to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> RhemaResult<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add a validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }

    /// Remove a validation rule
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.retain(|rule| rule.id != rule_id);
    }

    /// Get a validation rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&ValidationRule> {
        self.rules.iter().find(|rule| rule.id == rule_id)
    }

    /// Enable or disable a rule
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> RhemaResult<()> {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
            Ok(())
        } else {
            Err(ConfigError::ValidationError(format!("Rule not found: {}", rule_id)).into())
        }
    }

    /// Add a rule set
    pub fn add_rule_set(&mut self, name: String, rule_set: RuleSet) {
        self.rule_sets.insert(name, rule_set);
    }

    /// Get rules by type
    pub fn get_rules_by_type(&self, rule_type: &RuleType) -> Vec<&ValidationRule> {
        self.rules
            .iter()
            .filter(|rule| rule.rule_type == *rule_type && rule.enabled)
            .collect()
    }

    /// Get rules by severity
    pub fn get_rules_by_severity(&self, severity: &ConfigIssueSeverity) -> Vec<&ValidationRule> {
        self.rules
            .iter()
            .filter(|rule| rule.severity == *severity && rule.enabled)
            .collect()
    }
}

impl Default for GlobalValidationSettings {
    fn default() -> Self {
        Self {
            strict_mode: false,
            auto_fix: false,
            fail_fast: false,
            max_issues: None,
            cache_enabled: true,
            cache_ttl: 300,
            parallel_validation: true,
            max_parallel: num_cpus::get(),
        }
    }
}

impl ValidationRulesManager {
    /// Create a new validation rules manager
    pub fn new(config: ValidationRulesConfig) -> RhemaResult<Self> {
        let mut manager = Self {
            config,
            rule_cache: HashMap::new(),
            compiled_rules: HashMap::new(),
        };

        manager.compile_rules()?;
        Ok(manager)
    }

    /// Load validation rules from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> RhemaResult<Self> {
        let config = ValidationRulesConfig::from_file(path)?;
        Self::new(config)
    }

    /// Compile all rules for efficient evaluation
    fn compile_rules(&mut self) -> RhemaResult<()> {
        self.compiled_rules.clear();
        self.rule_cache.clear();

        for rule in &self.config.rules {
            if rule.enabled {
                let compiled_rule = self.compile_rule(rule)?;
                self.compiled_rules.insert(rule.id.clone(), compiled_rule);
                self.rule_cache.insert(rule.id.clone(), rule.clone());
            }
        }

        info!("Compiled {} validation rules", self.compiled_rules.len());
        Ok(())
    }

    /// Compile a single rule
    fn compile_rule(&self, rule: &ValidationRule) -> RhemaResult<CompiledRule> {
        let condition_tree = self.compile_conditions(&rule.conditions)?;
        let action_handlers = self.compile_actions(&rule.actions)?;

        Ok(CompiledRule {
            rule: rule.clone(),
            condition_tree,
            action_handlers,
        })
    }

    /// Compile rule conditions into a condition tree
    fn compile_conditions(&self, conditions: &[RuleCondition]) -> RhemaResult<ConditionTree> {
        if conditions.is_empty() {
            return Ok(ConditionTree::Leaf(RuleCondition {
                field: "".to_string(),
                operator: ConditionOperator::Exists,
                value: Value::Bool(true),
                case_sensitive: None,
            }));
        }

        if conditions.len() == 1 {
            return Ok(ConditionTree::Leaf(conditions[0].clone()));
        }

        // For multiple conditions, combine with AND
        let mut condition_trees = Vec::new();
        for condition in conditions {
            condition_trees.push(ConditionTree::Leaf(condition.clone()));
        }

        Ok(ConditionTree::And(condition_trees))
    }

    /// Compile rule actions into action handlers
    fn compile_actions(&self, actions: &[RuleAction]) -> RhemaResult<Vec<ActionHandler>> {
        let mut handlers = Vec::new();

        for action in actions {
            if action.enabled {
                let handler = self.create_action_handler(action)?;
                handlers.push(handler);
            }
        }

        Ok(handlers)
    }

    /// Create an action handler for a specific action
    fn create_action_handler(&self, action: &RuleAction) -> RhemaResult<ActionHandler> {
        let action_clone = action.clone();

        let handler: Box<dyn Fn(&Value, &ValidationRule) -> RhemaResult<()> + Send + Sync> =
            match action.action_type {
                ActionType::Log => Box::new(move |value: &Value, rule: &ValidationRule| {
                    debug!("Validation rule '{}' triggered: {:?}", rule.name, value);
                    Ok(())
                }),
                ActionType::Warn => Box::new(move |value: &Value, rule: &ValidationRule| {
                    warn!("Validation warning for rule '{}': {:?}", rule.name, value);
                    Ok(())
                }),
                ActionType::Error => Box::new(move |value: &Value, rule: &ValidationRule| {
                    Err(ConfigError::ValidationError(format!(
                        "Validation error for rule '{}': {:?}",
                        rule.name, value
                    ))
                    .into())
                }),
                ActionType::Fix => Box::new(move |value: &Value, rule: &ValidationRule| {
                    debug!("Auto-fixing issue for rule '{}': {:?}", rule.name, value);
                    Ok(())
                }),
                ActionType::Skip => Box::new(move |_value: &Value, _rule: &ValidationRule| {
                    debug!("Skipping validation");
                    Ok(())
                }),
                ActionType::Transform => {
                    let parameters = action.parameters.clone();
                    Box::new(move |_value: &Value, rule: &ValidationRule| {
                        debug!(
                            "Transforming value for rule '{}': {:?}",
                            rule.name, parameters
                        );
                        Ok(())
                    })
                }
                ActionType::Custom => {
                    let parameters = action.parameters.clone();
                    Box::new(move |_value: &Value, rule: &ValidationRule| {
                        debug!("Custom action for rule '{}': {:?}", rule.name, parameters);
                        Ok(())
                    })
                }
            };

        Ok(ActionHandler {
            action: action_clone,
            handler,
        })
    }

    /// Evaluate a configuration against all rules
    pub async fn evaluate_rules(
        &self,
        config: &Value,
        context: &str,
    ) -> RhemaResult<Vec<RuleEvaluationResult>> {
        let mut results = Vec::new();

        for (_rule_id, compiled_rule) in &self.compiled_rules {
            if let Ok(evaluation_result) = self.evaluate_rule(compiled_rule, config, context).await
            {
                results.push(evaluation_result);
            }
        }

        // Sort results by priority and severity
        results.sort_by(|a, b| {
            b.rule
                .severity
                .partial_cmp(&a.rule.severity)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.rule.id.cmp(&b.rule.id))
        });

        Ok(results)
    }

    /// Evaluate a single rule
    async fn evaluate_rule(
        &self,
        compiled_rule: &CompiledRule,
        config: &Value,
        context: &str,
    ) -> RhemaResult<RuleEvaluationResult> {
        let start_time = std::time::Instant::now();

        // Evaluate conditions
        let conditions_met = self.evaluate_condition_tree(&compiled_rule.condition_tree, config)?;

        let mut actions_executed = Vec::new();
        let mut errors = Vec::new();

        if conditions_met {
            // Execute actions
            for action_handler in &compiled_rule.action_handlers {
                match (action_handler.handler)(config, &compiled_rule.rule) {
                    Ok(()) => {
                        actions_executed.push(action_handler.action.action_type.clone());
                    }
                    Err(e) => {
                        errors.push(ConfigError::ValidationError(e.to_string()));
                    }
                }
            }
        }

        let duration = start_time.elapsed();

        Ok(RuleEvaluationResult {
            rule: compiled_rule.rule.clone(),
            conditions_met,
            actions_executed,
            errors,
            context: context.to_string(),
            evaluation_time: duration,
        })
    }

    /// Evaluate a condition tree
    fn evaluate_condition_tree(&self, tree: &ConditionTree, config: &Value) -> RhemaResult<bool> {
        match tree {
            ConditionTree::Leaf(condition) => self.evaluate_condition(condition, config),
            ConditionTree::And(children) => {
                for child in children {
                    if !self.evaluate_condition_tree(child, config)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ConditionTree::Or(children) => {
                for child in children {
                    if self.evaluate_condition_tree(child, config)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            ConditionTree::Not(child) => {
                let result = self.evaluate_condition_tree(child, config)?;
                Ok(!result)
            }
        }
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, condition: &RuleCondition, config: &Value) -> RhemaResult<bool> {
        let field_value = self.get_field_value(&condition.field, config)?;

        match condition.operator {
            ConditionOperator::Equals => Ok(field_value == condition.value),
            ConditionOperator::NotEquals => Ok(field_value != condition.value),
            ConditionOperator::GreaterThan => {
                self.compare_values(&field_value, &condition.value, |a, b| a > b)
            }
            ConditionOperator::LessThan => {
                self.compare_values(&field_value, &condition.value, |a, b| a < b)
            }
            ConditionOperator::GreaterThanOrEqual => {
                self.compare_values(&field_value, &condition.value, |a, b| a >= b)
            }
            ConditionOperator::LessThanOrEqual => {
                self.compare_values(&field_value, &condition.value, |a, b| a <= b)
            }
            ConditionOperator::Contains => {
                self.string_contains(&field_value, &condition.value, condition.case_sensitive)
            }
            ConditionOperator::NotContains => {
                let contains =
                    self.string_contains(&field_value, &condition.value, condition.case_sensitive)?;
                Ok(!contains)
            }
            ConditionOperator::StartsWith => {
                self.string_starts_with(&field_value, &condition.value, condition.case_sensitive)
            }
            ConditionOperator::EndsWith => {
                self.string_ends_with(&field_value, &condition.value, condition.case_sensitive)
            }
            ConditionOperator::Regex => self.regex_match(&field_value, &condition.value),
            ConditionOperator::Exists => Ok(!field_value.is_null()),
            ConditionOperator::NotExists => Ok(field_value.is_null()),
            ConditionOperator::In => self.value_in_array(&field_value, &condition.value),
            ConditionOperator::NotIn => {
                let in_array = self.value_in_array(&field_value, &condition.value)?;
                Ok(!in_array)
            }
            ConditionOperator::IsEmpty => {
                Ok(field_value.as_str().map(|s| s.is_empty()).unwrap_or(true))
            }
            ConditionOperator::IsNotEmpty => {
                Ok(field_value.as_str().map(|s| !s.is_empty()).unwrap_or(false))
            }
            ConditionOperator::IsNull => Ok(field_value.is_null()),
            ConditionOperator::IsNotNull => Ok(!field_value.is_null()),
        }
    }

    /// Get field value from JSON path
    fn get_field_value(&self, field_path: &str, config: &Value) -> RhemaResult<Value> {
        if field_path.is_empty() {
            return Ok(config.clone());
        }

        let path_parts: Vec<&str> = field_path.split('.').collect();
        let mut current = config;

        for part in path_parts {
            match current.get(part) {
                Some(value) => current = value,
                None => return Ok(Value::Null),
            }
        }

        Ok(current.clone())
    }

    /// Compare values with a comparison function
    fn compare_values<F>(&self, a: &Value, b: &Value, compare: F) -> RhemaResult<bool>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let a_num = a.as_f64().ok_or_else(|| {
            ConfigError::ValidationError(format!("Cannot convert {:?} to number", a))
        })?;
        let b_num = b.as_f64().ok_or_else(|| {
            ConfigError::ValidationError(format!("Cannot convert {:?} to number", b))
        })?;
        Ok(compare(a_num, b_num))
    }

    /// Check if string contains substring
    fn string_contains(
        &self,
        value: &Value,
        pattern: &Value,
        case_sensitive: Option<bool>,
    ) -> RhemaResult<bool> {
        let value_str = value.as_str().unwrap_or("");
        let pattern_str = pattern.as_str().unwrap_or("");

        let case_sensitive = case_sensitive.unwrap_or(true);

        if case_sensitive {
            Ok(value_str.contains(pattern_str))
        } else {
            Ok(value_str
                .to_lowercase()
                .contains(&pattern_str.to_lowercase()))
        }
    }

    /// Check if string starts with pattern
    fn string_starts_with(
        &self,
        value: &Value,
        pattern: &Value,
        case_sensitive: Option<bool>,
    ) -> RhemaResult<bool> {
        let value_str = value.as_str().unwrap_or("");
        let pattern_str = pattern.as_str().unwrap_or("");

        let case_sensitive = case_sensitive.unwrap_or(true);

        if case_sensitive {
            Ok(value_str.starts_with(pattern_str))
        } else {
            Ok(value_str
                .to_lowercase()
                .starts_with(&pattern_str.to_lowercase()))
        }
    }

    /// Check if string ends with pattern
    fn string_ends_with(
        &self,
        value: &Value,
        pattern: &Value,
        case_sensitive: Option<bool>,
    ) -> RhemaResult<bool> {
        let value_str = value.as_str().unwrap_or("");
        let pattern_str = pattern.as_str().unwrap_or("");

        let case_sensitive = case_sensitive.unwrap_or(true);

        if case_sensitive {
            Ok(value_str.ends_with(pattern_str))
        } else {
            Ok(value_str
                .to_lowercase()
                .ends_with(&pattern_str.to_lowercase()))
        }
    }

    /// Check regex match
    fn regex_match(&self, value: &Value, pattern: &Value) -> RhemaResult<bool> {
        let value_str = value.as_str().unwrap_or("");
        let pattern_str = pattern.as_str().unwrap_or("");

        match regex::Regex::new(pattern_str) {
            Ok(regex) => Ok(regex.is_match(value_str)),
            Err(_) => Err(ConfigError::ValidationError(format!(
                "Invalid regex pattern: {}",
                pattern_str
            ))
            .into()),
        }
    }

    /// Check if value is in array
    fn value_in_array(&self, value: &Value, array: &Value) -> RhemaResult<bool> {
        if let Some(array_value) = array.as_array() {
            Ok(array_value.contains(value))
        } else {
            Err(ConfigError::ValidationError("Expected array for 'in' operator".to_string()).into())
        }
    }

    /// Reload rules from configuration
    pub fn reload_rules(&mut self, config: ValidationRulesConfig) -> RhemaResult<()> {
        self.config = config;
        self.compile_rules()?;
        Ok(())
    }

    /// Get rule statistics
    pub fn get_statistics(&self) -> ValidationRulesStatistics {
        let total_rules = self.config.rules.len();
        let enabled_rules = self.compiled_rules.len();
        let rule_sets = self.config.rule_sets.len();

        ValidationRulesStatistics {
            total_rules,
            enabled_rules,
            rule_sets,
            cache_size: self.rule_cache.len(),
        }
    }
}

/// Rule evaluation result
#[derive(Debug)]
pub struct RuleEvaluationResult {
    pub rule: ValidationRule,
    pub conditions_met: bool,
    pub actions_executed: Vec<ActionType>,
    pub errors: Vec<ConfigError>,
    pub context: String,
    pub evaluation_time: std::time::Duration,
}

/// Validation rules statistics
#[derive(Debug, Clone)]
pub struct ValidationRulesStatistics {
    pub total_rules: usize,
    pub enabled_rules: usize,
    pub rule_sets: usize,
    pub cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validation_rules_config_creation() {
        let config = ValidationRulesConfig::new();
        assert_eq!(config.rules.len(), 0);
        assert_eq!(config.rule_sets.len(), 0);
    }

    #[test]
    fn test_validation_rule_creation() {
        let rule = ValidationRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            rule_type: RuleType::Schema,
            severity: ConfigIssueSeverity::Warning,
            enabled: true,
            conditions: vec![],
            actions: vec![],
            metadata: HashMap::new(),
        };

        assert_eq!(rule.id, "test-rule");
        assert_eq!(rule.name, "Test Rule");
        assert!(rule.enabled);
    }

    #[test]
    fn test_condition_evaluation() {
        let config = ValidationRulesConfig::new();
        let manager = ValidationRulesManager::new(config).unwrap();

        let condition = RuleCondition {
            field: "test_field".to_string(),
            operator: ConditionOperator::Equals,
            value: json!("test_value"),
            case_sensitive: None,
        };

        let config_value = json!({
            "test_field": "test_value"
        });

        let result = manager
            .evaluate_condition(&condition, &config_value)
            .unwrap();
        assert!(result);
    }

    #[test]
    fn test_rule_evaluation() {
        let mut config = ValidationRulesConfig::new();

        let rule = ValidationRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            rule_type: RuleType::Schema,
            severity: ConfigIssueSeverity::Warning,
            enabled: true,
            conditions: vec![RuleCondition {
                field: "test_field".to_string(),
                operator: ConditionOperator::Equals,
                value: json!("test_value"),
                case_sensitive: None,
            }],
            actions: vec![RuleAction {
                action_type: ActionType::Log,
                parameters: HashMap::new(),
                enabled: true,
            }],
            metadata: HashMap::new(),
        };

        config.add_rule(rule);
        let manager = ValidationRulesManager::new(config).unwrap();

        let config_value = json!({
            "test_field": "test_value"
        });

        let results = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(manager.evaluate_rules(&config_value, "test"))
            .unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].conditions_met);
    }
}
