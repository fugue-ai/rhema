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

use super::templates::TemplateUsageStats;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

/// Prompt pattern entry for prompts.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptPattern {
    /// Unique identifier
    pub id: String,
    /// Prompt name
    pub name: String,
    /// Prompt description
    pub description: Option<String>,
    /// Current prompt template (with optional {{CONTEXT}} variable)
    pub template: String,
    /// Context injection method: prepend, append, or template_variable
    pub injection: PromptInjectionMethod,
    /// Usage analytics and effectiveness tracking
    pub usage_analytics: UsageAnalytics,
    /// Version information
    pub version: PromptVersion,
    /// Optional tags for categorization
    pub tags: Option<Vec<String>>,
    /// Conditional context injection rules based on task type, file type, etc.
    pub context_rules: Option<Vec<ContextRule>>,
    /// Template variables beyond {{CONTEXT}}
    pub variables: Option<HashMap<String, String>>,
    /// Base template to extend from (for template inheritance)
    pub extends: Option<String>,
    /// Multi-file context support flag
    pub multi_file_context: Option<bool>,
    /// Template composition blocks for complex templates
    pub composition_blocks: Option<Vec<CompositionBlock>>,
    /// Advanced variable types and validation
    pub advanced_variables: Option<Vec<AdvancedVariable>>,
    /// Template validation rules
    pub validation_rules: Option<Vec<TemplateValidationRule>>,
    /// Template performance metrics
    pub performance_metrics: Option<TemplatePerformanceMetrics>,
    /// Context caching configuration
    pub context_cache: Option<ContextCacheConfig>,
    /// Context optimization settings
    pub context_optimization: Option<ContextOptimizationConfig>,
    /// Context learning configuration
    pub context_learning: Option<ContextLearningConfig>,
    /// Context quality metrics
    pub context_quality: Option<ContextQualityMetrics>,
}

/// Conditional context injection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRule {
    /// Condition expression (e.g., "task_type == 'code_review'")
    pub condition: String,
    /// Context files to inject when condition matches
    pub context_files: Vec<String>,
    /// How to inject the context
    pub injection_method: ContextInjectionMethod,
    /// Priority when multiple rules match (higher number = higher priority)
    pub priority: Option<u8>,
    /// Additional variables for this context rule
    pub variables: Option<HashMap<String, String>>,
}

/// Context injection method for conditional rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextInjectionMethod {
    Prepend,
    Append,
    TemplateVariable,
}

/// Template composition block for complex templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionBlock {
    /// Block identifier
    pub id: String,
    /// Block type: conditional, loop, or include
    pub block_type: CompositionBlockType,
    /// Condition expression for conditional blocks
    pub condition: Option<String>,
    /// Loop variable for loop blocks
    pub loop_variable: Option<String>,
    /// Loop items for loop blocks
    pub loop_items: Option<Vec<String>>,
    /// Block content template
    pub content: String,
    /// Block priority for ordering
    pub priority: Option<u8>,
    /// Block-specific variables
    pub variables: Option<HashMap<String, String>>,
}

/// Composition block type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompositionBlockType {
    Conditional,
    Loop,
    Include,
    Switch,
    Fallback,
}

/// Advanced variable with type and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedVariable {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: VariableType,
    /// Default value
    pub default_value: Option<String>,
    /// Validation rules
    pub validation: Option<VariableValidation>,
    /// Variable description
    pub description: Option<String>,
    /// Whether variable is required
    pub required: bool,
    /// Variable constraints
    pub constraints: Option<VariableConstraints>,
}

/// Variable type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Enum(Vec<String>),
    Date,
    Email,
    Url,
    FilePath,
    Json,
}

/// Variable validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableValidation {
    /// Minimum length for strings/arrays
    pub min_length: Option<usize>,
    /// Maximum length for strings/arrays
    pub max_length: Option<usize>,
    /// Minimum value for numbers
    pub min_value: Option<f64>,
    /// Maximum value for numbers
    pub max_value: Option<f64>,
    /// Regular expression pattern
    pub pattern: Option<String>,
    /// Custom validation function name
    pub custom_validator: Option<String>,
}

/// Variable constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableConstraints {
    /// Allowed values for enum types
    pub allowed_values: Option<Vec<String>>,
    /// Forbidden values
    pub forbidden_values: Option<Vec<String>>,
    /// Dependency on other variables
    pub depends_on: Option<String>,
    /// Conditional visibility
    pub visible_when: Option<String>,
}

/// Template validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateValidationRule {
    /// Rule identifier
    pub id: String,
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule condition expression
    pub condition: String,
    /// Error message for failed validation
    pub error_message: String,
    /// Rule severity
    pub severity: ValidationSeverity,
    /// Whether rule is enabled
    pub enabled: bool,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationRuleType {
    Required,
    Format,
    Length,
    Range,
    Custom,
    Dependency,
    Consistency,
}

/// Validation severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Context caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Maximum cache size in bytes
    pub max_size_bytes: u64,
    /// Cache invalidation strategy
    pub invalidation_strategy: CacheInvalidationStrategy,
    /// Cache compression enabled
    pub compression_enabled: bool,
    /// Cache persistence enabled
    pub persistence_enabled: bool,
}

/// Cache invalidation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheInvalidationStrategy {
    TimeBased,
    EventBased,
    Manual,
    Adaptive,
}

/// Context optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextOptimizationConfig {
    /// Whether optimization is enabled
    pub enabled: bool,
    /// Maximum context size in tokens
    pub max_tokens: usize,
    /// Minimum relevance score
    pub min_relevance_score: f64,
    /// Semantic compression enabled
    pub semantic_compression: bool,
    /// Structure optimization enabled
    pub structure_optimization: bool,
    /// Relevance filtering enabled
    pub relevance_filtering: bool,
    /// Optimization algorithm
    pub algorithm: OptimizationAlgorithm,
}

/// Optimization algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationAlgorithm {
    Greedy,
    DynamicProgramming,
    MachineLearning,
    Hybrid,
}

/// Context learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLearningConfig {
    /// Whether learning is enabled
    pub enabled: bool,
    /// Learning rate
    pub learning_rate: f64,
    /// Minimum sample size for learning
    pub min_sample_size: usize,
    /// Learning window size
    pub window_size: usize,
    /// Feedback weight
    pub feedback_weight: f64,
    /// Success threshold
    pub success_threshold: f64,
    /// Learning algorithm
    pub algorithm: LearningAlgorithm,
}

/// Learning algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningAlgorithm {
    Reinforcement,
    Supervised,
    Unsupervised,
    Online,
}

/// Context quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextQualityMetrics {
    /// Relevance score (0.0 to 1.0)
    pub relevance_score: f64,
    /// Completeness score (0.0 to 1.0)
    pub completeness_score: f64,
    /// Accuracy score (0.0 to 1.0)
    pub accuracy_score: f64,
    /// Timeliness score (0.0 to 1.0)
    pub timeliness_score: f64,
    /// Overall quality score (0.0 to 1.0)
    pub overall_score: f64,
    /// Quality assessment timestamp
    pub assessed_at: chrono::DateTime<chrono::Utc>,
    /// Quality improvement suggestions
    pub improvement_suggestions: Vec<String>,
}

/// Version information for prompt patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptVersion {
    /// Current version string (e.g., "1.2.3")
    pub current: String,
    /// Version history
    pub history: Vec<VersionEntry>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Individual version entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    /// Version string
    pub version: String,
    /// Template at this version
    pub template: String,
    /// Description of this version
    pub description: String,
    /// When this version was created
    pub timestamp: DateTime<Utc>,
    /// Author of this version (optional)
    pub author: Option<String>,
    /// List of changes in this version
    pub changes: Vec<String>,
}

/// Usage analytics for prompt patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalytics {
    /// Total number of times this prompt was used
    pub total_uses: u32,
    /// Number of successful uses (user marked as successful)
    pub successful_uses: u32,
    /// Last time this prompt was used
    pub last_used: Option<DateTime<Utc>>,
    /// Feedback history for this prompt
    pub feedback_history: Vec<FeedbackEntry>,
}

/// Individual feedback entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
    /// When this feedback was recorded
    pub timestamp: DateTime<Utc>,
    /// Whether the prompt was successful
    pub successful: bool,
    /// User feedback text
    pub feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptInjectionMethod {
    Prepend,
    Append,
    TemplateVariable,
}

/// Top-level structure for prompts.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompts {
    pub prompts: Vec<PromptPattern>,
}

/// Workflows structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflows {
    pub workflows: Vec<PromptChain>,
}

/// Prompt chain for workflow orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChain {
    /// Chain identifier
    pub id: String,
    /// Chain name
    pub name: String,
    /// Chain description
    pub description: Option<String>,
    /// Chain steps
    pub steps: Vec<ChainStep>,
    /// Chain metadata
    pub metadata: ChainMetadata,
    /// Usage statistics
    pub usage_stats: ChainUsageStats,
}

/// Individual chain step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStep {
    /// Step identifier
    pub id: String,
    /// Step name
    pub name: String,
    /// Prompt pattern to use
    pub prompt_pattern: String,
    /// Step order
    pub order: u32,
    /// Whether step is required
    pub required: bool,
    /// Step dependencies (other step IDs)
    pub dependencies: Option<Vec<String>>,
    /// Step timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Step retry configuration
    pub retry_config: Option<RetryConfig>,
}

/// Retry configuration for chain steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub delay_seconds: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

/// Chain metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetadata {
    /// Chain version
    pub version: String,
    /// Chain author
    pub author: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Chain tags
    pub tags: Option<Vec<String>>,
}

/// Chain usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainUsageStats {
    /// Total executions
    pub total_executions: u32,
    /// Successful executions
    pub successful_executions: u32,
    /// Failed executions
    pub failed_executions: u32,
    /// Average execution time in milliseconds
    pub avg_execution_time_ms: u64,
    /// Last execution timestamp
    pub last_executed: Option<DateTime<Utc>>,
    /// Step success rates
    pub step_success_rates: HashMap<String, f64>,
}

/// Template performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePerformanceMetrics {
    /// Average rendering time in milliseconds
    pub avg_rendering_time: f64,
    /// Maximum rendering time in milliseconds
    pub max_rendering_time: f64,
    /// Minimum rendering time in milliseconds
    pub min_rendering_time: f64,
    /// Total render count
    pub total_renders: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Last performance update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl ContextRule {
    /// Create a new context rule
    pub fn new(
        condition: &str,
        context_files: Vec<String>,
        injection_method: ContextInjectionMethod,
    ) -> Self {
        Self {
            condition: condition.to_string(),
            context_files,
            injection_method,
            priority: None,
            variables: None,
        }
    }

    /// Set priority for this rule
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Set variables for this rule
    pub fn with_variables(mut self, variables: HashMap<String, String>) -> Self {
        self.variables = Some(variables);
        self
    }

    /// Check if this rule matches the given context
    pub fn matches(
        &self,
        task_type: Option<&str>,
        file_type: Option<&str>,
        severity: Option<&str>,
    ) -> bool {
        // Simple condition evaluation - in a real implementation, this would use a proper expression evaluator
        let condition = self.condition.to_lowercase();

        if condition.contains("task_type") {
            if let Some(task) = task_type {
                if !condition.contains(&format!("task_type == '{}'", task)) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if condition.contains("file_type") {
            if let Some(file) = file_type {
                if !condition.contains(&format!("file_type == '{}'", file)) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if condition.contains("severity") {
            if let Some(sev) = severity {
                if !condition.contains(&format!("severity == '{}'", sev)) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl PromptVersion {
    /// Create a new version with initial version
    pub fn new(initial_version: &str) -> Self {
        let now = Utc::now();
        Self {
            current: initial_version.to_string(),
            history: vec![VersionEntry {
                version: initial_version.to_string(),
                template: String::new(), // Will be set by caller
                description: "Initial version".to_string(),
                timestamp: now,
                author: None,
                changes: vec!["Initial creation".to_string()],
            }],
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new version
    pub fn create_version(
        &mut self,
        new_version: &str,
        template: &str,
        description: &str,
        changes: Vec<String>,
        author: Option<String>,
    ) {
        let now = Utc::now();

        // Add current version to history
        self.history.push(VersionEntry {
            version: self.current.clone(),
            template: template.to_string(),
            description: description.to_string(),
            timestamp: now,
            author,
            changes,
        });

        // Update current version
        self.current = new_version.to_string();
        self.updated_at = now;
    }

    /// Get version history
    pub fn get_history(&self) -> &Vec<VersionEntry> {
        &self.history
    }

    /// Get a specific version entry
    pub fn get_version(&self, version: &str) -> Option<&VersionEntry> {
        self.history.iter().find(|entry| entry.version == version)
    }

    /// Get the latest version entry
    pub fn get_latest(&self) -> Option<&VersionEntry> {
        self.history.last()
    }

    /// Bump version (patch)
    pub fn bump_patch(&mut self) {
        // Simple implementation - in real code, use semver crate
        let parts: Vec<&str> = self.current.split('.').collect();
        if parts.len() >= 3 {
            if let Ok(patch) = parts[2].parse::<u32>() {
                self.current = format!("{}.{}.{}", parts[0], parts[1], patch + 1);
            }
        }
    }

    /// Bump minor version
    pub fn bump_minor(&mut self) {
        // Simple implementation - in real code, use semver crate
        let parts: Vec<&str> = self.current.split('.').collect();
        if parts.len() >= 2 {
            if let Ok(minor) = parts[1].parse::<u32>() {
                self.current = format!("{}.{}.0", parts[0], minor + 1);
            }
        }
    }

    /// Bump major version
    pub fn bump_major(&mut self) {
        // Simple implementation - in real code, use semver crate
        let parts: Vec<&str> = self.current.split('.').collect();
        if parts.len() >= 1 {
            if let Ok(major) = parts[0].parse::<u32>() {
                self.current = format!("{}.0.0", major + 1);
            }
        }
    }

    /// Set specific version
    pub fn set_version(&mut self, version: &str) {
        self.current = version.to_string();
    }
}

impl UsageAnalytics {
    /// Calculate success rate based on usage data
    pub fn success_rate(&self) -> f64 {
        if self.total_uses == 0 {
            0.0
        } else {
            self.successful_uses as f64 / self.total_uses as f64
        }
    }

    /// Record a new usage of this prompt
    pub fn record_usage(&mut self, successful: bool, feedback: Option<String>) {
        self.total_uses += 1;
        if successful {
            self.successful_uses += 1;
        }
        self.last_used = Some(Utc::now());

        if let Some(feedback_text) = feedback {
            self.feedback_history.push(FeedbackEntry {
                timestamp: Utc::now(),
                successful,
                feedback: feedback_text,
            });
        }
    }

    /// Create new usage analytics
    pub fn new() -> Self {
        Self {
            total_uses: 0,
            successful_uses: 0,
            last_used: None,
            feedback_history: Vec::new(),
        }
    }

    /// Get feedback history
    pub fn get_feedback_history(&self) -> Vec<&FeedbackEntry> {
        self.feedback_history.iter().collect()
    }
}

impl PromptPattern {
    /// Create a new prompt pattern
    pub fn new(id: &str, name: &str, template: &str, injection: PromptInjectionMethod) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            template: template.to_string(),
            injection,
            usage_analytics: UsageAnalytics::new(),
            version: PromptVersion::new("1.0.0"),
            tags: None,
            context_rules: None,
            variables: None,
            extends: None,
            multi_file_context: None,
            composition_blocks: None,
            advanced_variables: None,
            validation_rules: None,
            performance_metrics: None,
            context_cache: None,
            context_optimization: None,
            context_learning: None,
            context_quality: None,
        }
    }

    /// Add a context rule to this prompt pattern
    pub fn add_context_rule(&mut self, rule: ContextRule) {
        if self.context_rules.is_none() {
            self.context_rules = Some(Vec::new());
        }
        self.context_rules.as_mut().unwrap().push(rule);
    }

    /// Get matching context rules for the given context
    pub fn get_matching_rules(
        &self,
        task_type: Option<&str>,
        file_type: Option<&str>,
        severity: Option<&str>,
    ) -> Vec<&ContextRule> {
        let mut matching_rules = Vec::new();

        if let Some(rules) = &self.context_rules {
            for rule in rules {
                if rule.matches(task_type, file_type, severity) {
                    matching_rules.push(rule);
                }
            }
        }

        // Sort by priority (higher priority first)
        matching_rules.sort_by(|a, b| {
            let a_priority = a.priority.unwrap_or(0);
            let b_priority = b.priority.unwrap_or(0);
            b_priority.cmp(&a_priority)
        });

        matching_rules
    }

    /// Set template variables
    pub fn set_variables(&mut self, variables: HashMap<String, String>) {
        self.variables = Some(variables);
    }

    /// Get a template variable value
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.as_ref()?.get(key)
    }

    /// Substitute variables in the template
    pub fn substitute_variables(&self, context: &str) -> String {
        let mut result = self.template.clone();

        // Substitute {{CONTEXT}} variable
        result = result.replace("{{CONTEXT}}", context);

        // Substitute custom variables
        if let Some(vars) = &self.variables {
            for (key, value) in vars {
                let placeholder = format!("{{{{{}}}}}", key.to_uppercase());
                result = result.replace(&placeholder, value);
            }
        }

        result
    }

    /// Set the base template to extend from
    pub fn set_extends(&mut self, base_template: &str) {
        self.extends = Some(base_template.to_string());
    }

    /// Check if this pattern extends another template
    pub fn has_extends(&self) -> bool {
        self.extends.is_some()
    }

    /// Enable multi-file context support
    pub fn enable_multi_file_context(&mut self) {
        self.multi_file_context = Some(true);
    }

    /// Check if multi-file context is enabled
    pub fn supports_multi_file_context(&self) -> bool {
        self.multi_file_context.unwrap_or(false)
    }

    /// Get all context files that should be loaded for this pattern
    pub fn get_context_files(
        &self,
        task_type: Option<&str>,
        file_type: Option<&str>,
        severity: Option<&str>,
    ) -> Vec<String> {
        let mut context_files = Vec::new();

        // Get context files from matching rules
        let matching_rules = self.get_matching_rules(task_type, file_type, severity);
        for rule in matching_rules {
            context_files.extend(rule.context_files.clone());
        }

        context_files
    }
}

impl ChainUsageStats {
    /// Create new chain usage stats
    pub fn new() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            avg_execution_time_ms: 0,
            last_executed: None,
            step_success_rates: HashMap::new(),
        }
    }

    /// Record an execution
    pub fn record_execution(&mut self, successful: bool, execution_time_ms: u64) {
        self.total_executions += 1;
        if successful {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }

        // Update average execution time
        let total_time =
            self.avg_execution_time_ms * (self.total_executions - 1) as u64 + execution_time_ms;
        self.avg_execution_time_ms = total_time / self.total_executions as u64;

        self.last_executed = Some(Utc::now());
    }

    /// Update step success rate
    pub fn update_step_success_rate(&mut self, step_id: &str, success_rate: f64) {
        self.step_success_rates
            .insert(step_id.to_string(), success_rate);
    }

    /// Get overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
}

impl TemplateUsageStats {
    /// Create new usage stats
    pub fn new() -> Self {
        Self {
            total_downloads: 0,
            total_uses: 0,
            average_rating: None,
            rating_count: 0,
            last_downloaded: None,
            last_used: None,
        }
    }

    /// Record a download
    pub fn record_download(&mut self) {
        self.total_downloads += 1;
        self.last_downloaded = Some(Utc::now());
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.total_uses += 1;
        self.last_used = Some(Utc::now());
    }

    /// Add a rating
    pub fn add_rating(&mut self, rating: f64) {
        let current_avg = self.average_rating.unwrap_or(0.0);
        let new_avg =
            (current_avg * self.rating_count as f64 + rating) / (self.rating_count + 1) as f64;
        self.average_rating = Some(new_avg);
        self.rating_count += 1;
    }
}
