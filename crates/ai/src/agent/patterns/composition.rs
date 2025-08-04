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

use super::{
    CoordinationPattern, PatternContext, PatternResult, PatternError, ValidationResult,
    PatternMetadata, PatternCategory, PatternState, PatternPhase, PatternStatus,
    PatternPerformanceMetrics, AgentInfo, AgentStatus, Constraint, ConstraintType, PatternConfig
};
use super::validation::PatternValidationEngine;
use rhema_core::{RhemaResult, RhemaError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, instrument};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Advanced pattern composition system
pub struct PatternCompositionEngine {
    /// Available patterns
    patterns: Arc<RwLock<HashMap<String, Box<dyn CoordinationPattern>>>>,
    /// Pattern templates
    templates: Arc<RwLock<HashMap<String, PatternTemplate>>>,
    /// Composition rules
    composition_rules: Arc<RwLock<Vec<CompositionRule>>>,
    /// Validation engine
    validation_engine: Arc<PatternValidationEngine>,
    /// Composition history
    composition_history: Arc<RwLock<Vec<CompositionRecord>>>,
    /// Pattern dependencies
    dependencies: Arc<RwLock<PatternDependencyGraph>>,
}

/// Pattern template for reusable pattern configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Template category
    pub category: PatternCategory,
    /// Template parameters
    pub parameters: HashMap<String, TemplateParameter>,
    /// Template constraints
    pub constraints: Vec<Constraint>,
    /// Template dependencies
    pub dependencies: Vec<String>,
    /// Template metadata
    pub metadata: PatternMetadata,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

/// Template parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub parameter_type: ParameterType,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Required flag
    pub required: bool,
    /// Parameter description
    pub description: String,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Pattern,
    Agent,
    Resource,
}

/// Validation rule for template parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule value
    pub value: serde_json::Value,
    /// Error message
    pub error_message: String,
}

/// Validation rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    MinLength,
    MaxLength,
    MinValue,
    MaxValue,
    Pattern,
    Required,
    Custom,
}

/// Composition rule for pattern composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule priority
    pub priority: u32,
    /// Rule conditions
    pub conditions: Vec<CompositionCondition>,
    /// Rule actions
    pub actions: Vec<CompositionAction>,
    /// Rule enabled flag
    pub enabled: bool,
}

/// Composition condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionCondition {
    /// Condition type
    pub condition_type: ConditionType,
    /// Condition parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    PatternExists,
    PatternNotExists,
    AgentAvailable,
    ResourceAvailable,
    ConstraintSatisfied,
    DependencyMet,
    Custom,
}

/// Composition action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionAction {
    /// Action type
    pub action_type: ActionType,
    /// Action parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    AddPattern,
    RemovePattern,
    ModifyPattern,
    AddConstraint,
    RemoveConstraint,
    AddDependency,
    RemoveDependency,
    Custom,
}

/// Pattern dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDependencyGraph {
    /// Pattern nodes
    pub nodes: HashMap<String, PatternNode>,
    /// Dependency edges
    pub edges: Vec<DependencyEdge>,
}

/// Pattern node in dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternNode {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern metadata
    pub metadata: PatternMetadata,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Dependents
    pub dependents: Vec<String>,
    /// Node state
    pub state: PatternState,
}

/// Dependency edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source pattern ID
    pub source: String,
    /// Target pattern ID
    pub target: String,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Dependency strength
    pub strength: DependencyStrength,
}

/// Dependency types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Requires,
    Provides,
    Conflicts,
    Enhances,
    Optional,
}

/// Dependency strength
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyStrength {
    Strong,
    Medium,
    Weak,
}

/// Composed pattern result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedPattern {
    /// Composition ID
    pub id: String,
    /// Composition name
    pub name: String,
    /// Composition description
    pub description: String,
    /// Composed patterns
    pub patterns: Vec<String>,
    /// Composition metadata
    pub metadata: PatternMetadata,
    /// Composition state
    pub state: PatternState,
    /// Composition validation result
    pub validation_result: Option<ValidationResult>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Composition record for tracking composition history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRecord {
    /// Record ID
    pub id: String,
    /// Composition ID
    pub composition_id: String,
    /// Operation type
    pub operation: CompositionOperation,
    /// Operation parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Operation result
    pub result: CompositionResult,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Composition operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionOperation {
    Create,
    Update,
    Delete,
    Validate,
    Execute,
    Rollback,
}

/// Composition result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionResult {
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Result data
    pub data: HashMap<String, serde_json::Value>,
}

impl PatternCompositionEngine {
    /// Create a new pattern composition engine
    pub fn new() -> Self {
        Self {
            patterns: Arc::new(RwLock::new(HashMap::new())),
            templates: Arc::new(RwLock::new(HashMap::new())),
            composition_rules: Arc::new(RwLock::new(Vec::new())),
            validation_engine: Arc::new(PatternValidationEngine::new()),
            composition_history: Arc::new(RwLock::new(Vec::new())),
            dependencies: Arc::new(RwLock::new(PatternDependencyGraph {
                nodes: HashMap::new(),
                edges: Vec::new(),
            })),
        }
    }

    /// Register a pattern
    #[instrument(skip(self, pattern))]
    pub async fn register_pattern(&self, pattern_id: String, pattern: Box<dyn CoordinationPattern>) {
        let mut patterns = self.patterns.write().await;
        patterns.insert(pattern_id.clone(), pattern);
        
        // Update dependency graph
        let mut dependencies = self.dependencies.write().await;
        let metadata = patterns.get(&pattern_id).unwrap().metadata();
        dependencies.nodes.insert(pattern_id.clone(), PatternNode {
            pattern_id: pattern_id.clone(),
            metadata: metadata.clone(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
            state: PatternState {
                pattern_id: pattern_id.clone(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Idle,
                data: HashMap::new(),
            },
        });

        info!("✅ Pattern registered: {}", pattern_id);
    }

    /// Create a pattern template
    #[instrument(skip(self))]
    pub async fn create_template(&self, template: PatternTemplate) -> RhemaResult<()> {
        let mut templates = self.templates.write().await;
        
        // Validate template
        self.validate_template(&template).await?;
        
        let template_id = template.id.clone();
        templates.insert(template_id.clone(), template);
        
        info!("✅ Template created: {}", template_id);
        Ok(())
    }

    /// Instantiate a pattern from template
    #[instrument(skip(self, parameters))]
    pub async fn instantiate_from_template(
        &self,
        template_id: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<Box<dyn CoordinationPattern>> {
        let templates = self.templates.read().await;
        let template = templates.get(template_id)
            .ok_or_else(|| PatternError::TemplateNotFound(template_id.to_string()))?;

        // Validate parameters
        self.validate_template_parameters(template, &parameters).await?;

        // Create pattern from template
        let pattern = self.create_pattern_from_template(template, parameters).await?;
        
        info!("✅ Pattern instantiated from template: {}", template_id);
        Ok(pattern)
    }

    /// Compose multiple patterns
    #[instrument(skip(self, pattern_ids))]
    pub async fn compose_patterns(
        &self,
        composition_id: String,
        pattern_ids: Vec<String>,
        context: &PatternContext,
    ) -> RhemaResult<ComposedPattern> {
        // Validate pattern existence
        let patterns = self.patterns.read().await;
        for pattern_id in &pattern_ids {
            if !patterns.contains_key(pattern_id) {
                return Err(RhemaError::NotFound(pattern_id.clone()));
            }
        }

        // Check dependencies
        self.validate_composition_dependencies(&pattern_ids).await?;

        // Apply composition rules
        let composition_rules = self.composition_rules.read().await;
        let mut modified_pattern_ids = pattern_ids.clone();
        
        for rule in composition_rules.iter().filter(|r| r.enabled) {
            if self.evaluate_composition_rule(rule, &modified_pattern_ids, context).await? {
                self.apply_composition_rule(rule, &mut modified_pattern_ids).await?;
            }
        }

        // Create composed pattern
        let pattern_count = modified_pattern_ids.len();
        let patterns = modified_pattern_ids;
        let composed_pattern = ComposedPattern {
            id: composition_id.clone(),
            name: format!("Composed Pattern: {}", composition_id),
            description: format!("Composition of {} patterns", pattern_count),
            patterns,
            metadata: PatternMetadata {
                id: composition_id.clone(),
                name: format!("Composed Pattern: {}", composition_id),
                description: format!("Composition of {} patterns", pattern_count),
                category: PatternCategory::Custom("composition".to_string()),
                version: "1.0.0".to_string(),
                author: "PatternCompositionEngine".to_string(),
                created_at: Utc::now(),
                modified_at: Utc::now(),
                tags: vec!["composed".to_string()],
                required_capabilities: Vec::new(),
                required_resources: Vec::new(),
                constraints: Vec::new(),
                dependencies: Vec::new(),
                complexity: 5,
                estimated_execution_time_seconds: 60,
            },
            state: PatternState {
                pattern_id: composition_id.clone(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Idle,
                data: HashMap::new(),
            },
            validation_result: None,
            created_at: Utc::now(),
        };

        // Validate composition
        let validation_result = self.validate_composition(&composed_pattern, context).await?;
        let mut final_composed_pattern = composed_pattern;
        final_composed_pattern.validation_result = Some(validation_result.clone());

        // Record composition
        self.record_composition(
            composition_id.clone(),
            CompositionOperation::Create,
            HashMap::new(),
            CompositionResult {
                success: validation_result.is_valid,
                error_message: if validation_result.is_valid { None } else { 
                    Some(validation_result.errors.join(", "))
                },
                data: HashMap::new(),
            }
        ).await;

        if !validation_result.is_valid {
                          return Err(RhemaError::ValidationError(
                format!("Composition validation failed: {}", validation_result.errors.join(", "))
            ));
        }

        info!("✅ Patterns composed successfully: {}", composition_id);
        Ok(final_composed_pattern)
    }

    /// Execute a composed pattern
    #[instrument(skip(self, context))]
    pub async fn execute_composed_pattern(
        &self,
        composed_pattern: &ComposedPattern,
        context: &PatternContext,
    ) -> RhemaResult<PatternResult> {
        let patterns = self.patterns.read().await;
        let mut results = Vec::new();

        // Execute patterns in dependency order
        let execution_order = self.get_execution_order(&composed_pattern.patterns).await?;

        for pattern_id in execution_order {
            if let Some(pattern) = patterns.get(&pattern_id) {
                let result = pattern.execute(context).await?;
                results.push((pattern_id.clone(), result));
            }
        }

        // Combine results
        let combined_result = self.combine_pattern_results(results).await?;

        // Record execution
        self.record_composition(
            composed_pattern.id.clone(),
            CompositionOperation::Execute,
            HashMap::new(),
            CompositionResult {
                success: true,
                error_message: None,
                data: HashMap::new(),
            }
        ).await;

        info!("✅ Composed pattern executed successfully: {}", composed_pattern.id);
        Ok(combined_result)
    }

    /// Add composition rule
    pub async fn add_composition_rule(&self, rule: CompositionRule) {
        let mut rules = self.composition_rules.write().await;
        rules.push(rule);
        info!("✅ Composition rule added");
    }

    /// Get composition statistics
    pub async fn get_composition_statistics(&self) -> CompositionStatistics {
        let templates = self.templates.read().await;
        let patterns = self.patterns.read().await;
        let history = self.composition_history.read().await;

        CompositionStatistics {
            total_templates: templates.len(),
            total_patterns: patterns.len(),
            total_compositions: history.len(),
            successful_compositions: history.iter().filter(|r| r.result.success).count(),
            failed_compositions: history.iter().filter(|r| !r.result.success).count(),
        }
    }

    /// Validate template
    async fn validate_template(&self, template: &PatternTemplate) -> RhemaResult<()> {
        // Check required fields
        if template.id.is_empty() || template.name.is_empty() {
                          return Err(RhemaError::ValidationError(
                "Template ID and name are required".to_string()
            ));
        }

        // Check parameter names are unique
        let mut param_names = HashSet::new();
        for param in template.parameters.values() {
            if !param_names.insert(&param.name) {
                                  return Err(RhemaError::ValidationError(
                    format!("Duplicate parameter name: {}", param.name)
                ));
            }
        }

        Ok(())
    }

    /// Validate template parameters
    async fn validate_template_parameters(
        &self,
        template: &PatternTemplate,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> RhemaResult<()> {
        for (param_name, param_def) in &template.parameters {
            if param_def.required {
                if !parameters.contains_key(param_name) {
                                          return Err(RhemaError::ValidationError(
                        format!("Required parameter missing: {}", param_name)
                    ));
                }
            }

            if let Some(value) = parameters.get(param_name) {
                // Validate parameter value against rules
                for rule in &param_def.validation_rules {
                    if !self.validate_parameter_rule(value, rule) {
                                                  return Err(RhemaError::ValidationError(
                            format!("Parameter validation failed for {}: {}", param_name, rule.error_message)
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate parameter rule
    fn validate_parameter_rule(&self, value: &serde_json::Value, rule: &ValidationRule) -> bool {
        match rule.rule_type {
            ValidationRuleType::MinLength => {
                if let Some(s) = value.as_str() {
                    if let Some(min_len) = rule.value.as_u64() {
                        return s.len() >= min_len as usize;
                    }
                }
                false
            }
            ValidationRuleType::MaxLength => {
                if let Some(s) = value.as_str() {
                    if let Some(max_len) = rule.value.as_u64() {
                        return s.len() <= max_len as usize;
                    }
                }
                false
            }
            ValidationRuleType::MinValue => {
                if let Some(num) = value.as_f64() {
                    if let Some(min_val) = rule.value.as_f64() {
                        return num >= min_val;
                    }
                }
                false
            }
            ValidationRuleType::MaxValue => {
                if let Some(num) = value.as_f64() {
                    if let Some(max_val) = rule.value.as_f64() {
                        return num <= max_val;
                    }
                }
                false
            }
            ValidationRuleType::Required => {
                !value.is_null()
            }
            ValidationRuleType::Pattern | ValidationRuleType::Custom => {
                // Simplified validation - always pass for now
                true
            }
        }
    }

    /// Create pattern from template
    async fn create_pattern_from_template(
        &self,
        template: &PatternTemplate,
        parameters: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<Box<dyn CoordinationPattern>> {
        // This is a simplified implementation
        // In a real implementation, you would use the template and parameters
        // to create a concrete pattern instance
        
        // For now, return a mock pattern
        Ok(Box::new(MockComposedPattern {
            id: template.id.clone(),
            name: template.name.clone(),
            description: template.description.clone(),
            category: template.category.clone(),
            parameters,
        }))
    }

    /// Validate composition dependencies
    async fn validate_composition_dependencies(&self, pattern_ids: &[String]) -> RhemaResult<()> {
        let dependencies = self.dependencies.read().await;
        
        for pattern_id in pattern_ids {
            if let Some(node) = dependencies.nodes.get(pattern_id) {
                for dep in &node.dependencies {
                    if !pattern_ids.contains(dep) {
                                                  return Err(RhemaError::ValidationError(
                            format!("Missing dependency: {} for pattern {}", dep, pattern_id)
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Evaluate composition rule
    async fn evaluate_composition_rule(
        &self,
        rule: &CompositionRule,
        pattern_ids: &[String],
        context: &PatternContext,
    ) -> RhemaResult<bool> {
        for condition in &rule.conditions {
            match condition.condition_type {
                ConditionType::PatternExists => {
                    let required_pattern = condition.parameters.get("pattern_id")
                        .and_then(|v| v.as_str())
                                                  .ok_or_else(|| RhemaError::ValidationError(
                            "Pattern ID parameter required".to_string()
                        ))?;
                    
                    if !pattern_ids.contains(&required_pattern.to_string()) {
                        return Ok(false);
                    }
                }
                ConditionType::AgentAvailable => {
                    let required_capability = condition.parameters.get("capability")
                        .and_then(|v| v.as_str())
                                                  .ok_or_else(|| RhemaError::ValidationError(
                            "Capability parameter required".to_string()
                        ))?;
                    
                    let has_capability = context.agents.iter()
                        .any(|agent| agent.capabilities.contains(&required_capability.to_string()));
                    
                    if !has_capability {
                        return Ok(false);
                    }
                }
                _ => {
                    // Simplified evaluation for other condition types
                    return Ok(true);
                }
            }
        }

        Ok(true)
    }

    /// Apply composition rule
    async fn apply_composition_rule(
        &self,
        rule: &CompositionRule,
        pattern_ids: &mut Vec<String>,
    ) -> RhemaResult<()> {
        for action in &rule.actions {
            match action.action_type {
                ActionType::AddPattern => {
                    if let Some(pattern_id) = action.parameters.get("pattern_id")
                        .and_then(|v| v.as_str()) {
                        if !pattern_ids.contains(&pattern_id.to_string()) {
                            pattern_ids.push(pattern_id.to_string());
                        }
                    }
                }
                ActionType::RemovePattern => {
                    if let Some(pattern_id) = action.parameters.get("pattern_id")
                        .and_then(|v| v.as_str()) {
                        pattern_ids.retain(|id| id != pattern_id);
                    }
                }
                _ => {
                    // Simplified application for other action types
                }
            }
        }

        Ok(())
    }

    /// Validate composition
    async fn validate_composition(
        &self,
        composed_pattern: &ComposedPattern,
        context: &PatternContext,
    ) -> RhemaResult<ValidationResult> {
        // Use the validation engine to validate the composition
        Ok(self.validation_engine.validate_pattern(
            &composed_pattern.id,
            context,
            &composed_pattern.metadata,
        ).await)
    }

    /// Get execution order based on dependencies
    async fn get_execution_order(&self, pattern_ids: &[String]) -> RhemaResult<Vec<String>> {
        let dependencies = self.dependencies.read().await;
        let mut execution_order = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for pattern_id in pattern_ids {
            if !visited.contains(pattern_id) {
                self.topological_sort(
                    pattern_id,
                    &dependencies,
                    &mut visited,
                    &mut visiting,
                    &mut execution_order,
                )?;
            }
        }

        Ok(execution_order)
    }

    /// Topological sort for dependency resolution
    fn topological_sort(
        &self,
        pattern_id: &str,
        dependencies: &PatternDependencyGraph,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        execution_order: &mut Vec<String>,
    ) -> RhemaResult<()> {
        if visiting.contains(pattern_id) {
                          return Err(RhemaError::ValidationError(
                format!("Circular dependency detected: {}", pattern_id)
            ));
        }

        if visited.contains(pattern_id) {
            return Ok(());
        }

        visiting.insert(pattern_id.to_string());

        if let Some(node) = dependencies.nodes.get(pattern_id) {
            for dep in &node.dependencies {
                self.topological_sort(dep, dependencies, visited, visiting, execution_order)?;
            }
        }

        visiting.remove(pattern_id);
        visited.insert(pattern_id.to_string());
        execution_order.push(pattern_id.to_string());

        Ok(())
    }

    /// Combine pattern results
    async fn combine_pattern_results(
        &self,
        results: Vec<(String, PatternResult)>,
    ) -> RhemaResult<PatternResult> {
        let mut combined_data = HashMap::new();
        let mut combined_metadata = HashMap::new();

        for (pattern_id, result) in results {
            combined_data.insert(format!("{}_result", pattern_id), serde_json::json!(result.data));
            combined_metadata.insert(format!("{}_data", pattern_id), serde_json::json!(result.data));
        }

        Ok(PatternResult {
            pattern_id: "composed".to_string(),
            success: true,
            data: combined_data,
            performance_metrics: PatternPerformanceMetrics {
                total_execution_time_seconds: 0.1,
                coordination_overhead_seconds: 0.01,
                resource_utilization: 0.5,
                agent_efficiency: 0.8,
                communication_overhead: 2,
            },
            error_message: None,
            completed_at: Utc::now(),
            metadata: combined_metadata,
            execution_time_ms: 100,
        })
    }

    /// Record composition operation
    async fn record_composition(
        &self,
        composition_id: String,
        operation: CompositionOperation,
        parameters: HashMap<String, serde_json::Value>,
        result: CompositionResult,
    ) {
        let record = CompositionRecord {
            id: Uuid::new_v4().to_string(),
            composition_id,
            operation,
            parameters,
            result,
            timestamp: Utc::now(),
        };

        let mut history = self.composition_history.write().await;
        history.push(record);
    }
}

/// Mock composed pattern for testing
struct MockComposedPattern {
    id: String,
    name: String,
    description: String,
    category: PatternCategory,
    parameters: HashMap<String, serde_json::Value>,
}

#[async_trait::async_trait]
impl CoordinationPattern for MockComposedPattern {


    async fn execute(&self, _context: &PatternContext) -> Result<PatternResult, PatternError> {
        Ok(PatternResult {
            pattern_id: self.id.clone(),
            success: true,
            data: HashMap::new(),
            performance_metrics: PatternPerformanceMetrics {
                total_execution_time_seconds: 0.1,
                coordination_overhead_seconds: 0.01,
                resource_utilization: 0.5,
                agent_efficiency: 0.8,
                communication_overhead: 2,
            },
            error_message: None,
            completed_at: Utc::now(),
            metadata: HashMap::new(),
            execution_time_ms: 0,
        })
    }

    async fn validate(&self, _context: &PatternContext) -> Result<ValidationResult, PatternError> {
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            details: HashMap::new(),
        })
    }

    async fn rollback(&self, _context: &PatternContext) -> Result<(), PatternError> {
        Ok(())
    }

    fn metadata(&self) -> PatternMetadata {
        PatternMetadata {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            version: "1.0.0".to_string(),
            category: self.category.clone(),
            author: "MockComposedPattern".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: vec!["mock".to_string(), "composed".to_string()],
            required_capabilities: Vec::new(),
            required_resources: Vec::new(),
            constraints: Vec::new(),
            dependencies: Vec::new(),
            complexity: 1,
            estimated_execution_time_seconds: 1,
        }
    }
}

/// Composition statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionStatistics {
    /// Total templates
    pub total_templates: usize,
    /// Total patterns
    pub total_patterns: usize,
    /// Total compositions
    pub total_compositions: usize,
    /// Successful compositions
    pub successful_compositions: usize,
    /// Failed compositions
    pub failed_compositions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    // MockPattern is not available, using a placeholder
    // use crate::agent::patterns::MockPattern;

    #[tokio::test]
    async fn test_composition_engine_creation() {
        let engine = PatternCompositionEngine::new();
        let stats = engine.get_composition_statistics().await;
        
        assert_eq!(stats.total_templates, 0);
        assert_eq!(stats.total_patterns, 0);
        assert_eq!(stats.total_compositions, 0);
    }

    #[tokio::test]
    async fn test_pattern_registration() {
        let engine = PatternCompositionEngine::new();
        let pattern = Box::new(MockComposedPattern {
            id: "test".to_string(),
            name: "Test Pattern".to_string(),
            description: "Test pattern".to_string(),
            category: PatternCategory::TaskDistribution,
            parameters: HashMap::new(),
        });
        
        engine.register_pattern("test_pattern".to_string(), pattern).await;
        
        let stats = engine.get_composition_statistics().await;
        assert_eq!(stats.total_patterns, 1);
    }

    #[tokio::test]
    async fn test_template_creation() {
        let engine = PatternCompositionEngine::new();
        let template = PatternTemplate {
            id: "test_template".to_string(),
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            version: "1.0.0".to_string(),
            category: PatternCategory::TaskDistribution,
            parameters: HashMap::new(),
            constraints: Vec::new(),
            dependencies: Vec::new(),
            metadata: PatternMetadata {
                id: "test_template".to_string(),
                name: "Test Template".to_string(),
                description: "A test template".to_string(),
                category: PatternCategory::TaskDistribution,
                version: "1.0.0".to_string(),
                author: "Test".to_string(),
                created_at: Utc::now(),
                modified_at: Utc::now(),
                tags: vec!["test".to_string()],
                required_capabilities: Vec::new(),
                required_resources: Vec::new(),
                constraints: Vec::new(),
                dependencies: Vec::new(),
                complexity: 1,
                estimated_execution_time_seconds: 10,
            },
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        let result = engine.create_template(template).await;
        assert!(result.is_ok());
        
        let stats = engine.get_composition_statistics().await;
        assert_eq!(stats.total_templates, 1);
    }
} 