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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use validator::ValidationError;

/// Schema version for compatibility tracking
pub const CURRENT_SCHEMA_VERSION: &str = "1.0.0";

/// Core Rhema scope definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhemaScope {
    /// Scope name and identifier
    pub name: String,

    /// Scope type (service, app, library, etc.)
    pub scope_type: String,

    /// Human-readable description
    pub description: Option<String>,

    /// Version of the scope definition
    pub version: String,

    /// Schema version for compatibility
    pub schema_version: Option<String>,

    /// Dependencies on other scopes
    pub dependencies: Option<Vec<ScopeDependency>>,

    /// Protocol information for AI context bootstrapping
    pub protocol_info: Option<ProtocolInfo>,

    /// Custom fields for extensibility
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Scope dependency definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeDependency {
    /// Path to the dependent scope
    pub path: String,

    /// Type of dependency (required, optional, peer)
    pub dependency_type: String,

    /// Version constraint
    pub version: Option<String>,
}

/// Protocol information for AI context bootstrapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Protocol version
    pub version: String,

    /// Protocol description and purpose
    pub description: Option<String>,

    /// Key concepts and terminology
    pub concepts: Option<Vec<ConceptDefinition>>,

    /// CQL examples and usage patterns
    pub cql_examples: Option<Vec<CqlExample>>,

    /// Common patterns and conventions
    pub patterns: Option<Vec<PatternDefinition>>,

    /// Integration guidelines
    pub integrations: Option<Vec<IntegrationGuide>>,

    /// Troubleshooting and common issues
    pub troubleshooting: Option<Vec<TroubleshootingItem>>,

    /// Custom protocol extensions
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Concept definition for protocol documentation
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

/// CQL example for protocol documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CqlExample {
    /// Example name/title
    pub name: String,

    /// CQL query
    pub query: String,

    /// Description of what the query does
    pub description: String,

    /// Expected output format
    pub output_format: Option<String>,

    /// Use case context
    pub use_case: Option<String>,
}

/// Pattern definition for protocol documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDefinition {
    /// Pattern name
    pub name: String,

    /// Pattern description
    pub description: String,

    /// When to use this pattern
    pub when_to_use: Option<String>,

    /// Implementation examples
    pub examples: Option<Vec<String>>,
}

/// Integration guide for protocol documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationGuide {
    /// Integration name
    pub name: String,

    /// Integration description
    pub description: String,

    /// Setup instructions
    pub setup: Option<Vec<String>>,

    /// Configuration examples
    pub configuration: Option<Vec<String>>,

    /// Best practices
    pub best_practices: Option<Vec<String>>,
}

/// Troubleshooting item for protocol documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroubleshootingItem {
    /// Issue name
    pub issue: String,

    /// Problem description
    pub description: String,

    /// Solution steps
    pub solution: Vec<String>,

    /// Prevention tips
    pub prevention: Option<Vec<String>>,
}

/// Knowledge base structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    /// Knowledge entries
    pub entries: Vec<KnowledgeEntry>,

    /// Categories for organization
    pub categories: Option<HashMap<String, String>>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual knowledge entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Unique identifier
    pub id: String,

    /// Knowledge title
    pub title: String,

    /// Knowledge content
    pub content: String,

    /// Category
    pub category: Option<String>,

    /// Tags for searchability
    pub tags: Option<Vec<String>>,

    /// Confidence level (1-10)
    pub confidence: Option<u8>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: Option<DateTime<Utc>>,

    /// Source of the knowledge
    pub source: Option<String>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Todo items structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todos {
    /// Todo entries
    pub todos: Vec<TodoEntry>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual todo entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoEntry {
    /// Unique identifier
    pub id: String,

    /// Todo title
    pub title: String,

    /// Detailed description
    pub description: Option<String>,

    /// Current status
    pub status: TodoStatus,

    /// Priority level
    pub priority: Priority,

    /// Assigned to (optional)
    pub assigned_to: Option<String>,

    /// Due date (optional)
    pub due_date: Option<DateTime<Utc>>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Completion timestamp
    pub completed_at: Option<DateTime<Utc>>,

    /// Completion outcome
    pub outcome: Option<String>,

    /// Related knowledge entries
    pub related_knowledge: Option<Vec<String>>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Todo status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Decisions structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decisions {
    /// Decision entries
    pub decisions: Vec<DecisionEntry>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual decision entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEntry {
    /// Unique identifier
    pub id: String,

    /// Decision title
    pub title: String,

    /// Decision description
    pub description: String,

    /// Current status
    pub status: DecisionStatus,

    /// Decision context
    pub context: Option<String>,

    /// Alternatives considered
    pub alternatives: Option<Vec<String>>,

    /// Rationale for the decision
    pub rationale: Option<String>,

    /// Consequences and implications
    pub consequences: Option<Vec<String>>,

    /// Decision date
    pub decided_at: DateTime<Utc>,

    /// Review date (optional)
    pub review_date: Option<DateTime<Utc>>,

    /// Decision makers
    pub decision_makers: Option<Vec<String>>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Decision status
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionStatus {
    Proposed,
    UnderReview,
    Approved,
    Rejected,
    Implemented,
    Deprecated,
}

/// Patterns structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patterns {
    /// Pattern entries
    pub patterns: Vec<PatternEntry>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual pattern entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternEntry {
    /// Unique identifier
    pub id: String,

    /// Pattern name
    pub name: String,

    /// Pattern description
    pub description: String,

    /// Pattern type
    pub pattern_type: String,

    /// Usage context
    pub usage: PatternUsage,

    /// Effectiveness rating (1-10)
    pub effectiveness: Option<u8>,

    /// Implementation examples
    pub examples: Option<Vec<String>>,

    /// Anti-patterns to avoid
    pub anti_patterns: Option<Vec<String>>,

    /// Related patterns
    pub related_patterns: Option<Vec<String>>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: Option<DateTime<Utc>>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Pattern usage context
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PatternUsage {
    Required,
    Recommended,
    Optional,
    Deprecated,
}

/// Conventions structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conventions {
    /// Convention entries
    pub conventions: Vec<ConventionEntry>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual convention entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionEntry {
    /// Unique identifier
    pub id: String,

    /// Convention name
    pub name: String,

    /// Convention description
    pub description: String,

    /// Convention type
    pub convention_type: String,

    /// Enforcement level
    pub enforcement: EnforcementLevel,

    /// Examples of the convention
    pub examples: Option<Vec<String>>,

    /// Tools or linters that enforce this convention
    pub tools: Option<Vec<String>>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: Option<DateTime<Utc>>,

    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Enforcement levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementLevel {
    Required,
    Recommended,
    Optional,
    Deprecated,
}

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

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        context_files.retain(|file| seen.insert(file.clone()));

        context_files
    }

    /// Record usage with feedback
    pub fn record_usage(&mut self, successful: bool, feedback: Option<String>) {
        self.usage_analytics.record_usage(successful, feedback);
    }

    /// Get current success rate
    pub fn success_rate(&self) -> f64 {
        self.usage_analytics.success_rate()
    }

    /// Get total usage count
    pub fn total_uses(&self) -> u32 {
        self.usage_analytics.total_uses
    }

    /// Get successful usage count
    pub fn successful_uses(&self) -> u32 {
        self.usage_analytics.successful_uses
    }

    // P2: Advanced Template Features

    /// Add a composition block to the pattern
    pub fn add_composition_block(&mut self, block: CompositionBlock) {
        if self.composition_blocks.is_none() {
            self.composition_blocks = Some(Vec::new());
        }
        self.composition_blocks.as_mut().unwrap().push(block);
    }

    /// Get composition blocks by type
    pub fn get_composition_blocks(
        &self,
        block_type: Option<CompositionBlockType>,
    ) -> Vec<&CompositionBlock> {
        if let Some(blocks) = &self.composition_blocks {
            if let Some(filter_type) = block_type {
                blocks
                    .iter()
                    .filter(|b| b.block_type == filter_type)
                    .collect()
            } else {
                blocks.iter().collect()
            }
        } else {
            Vec::new()
        }
    }

    /// Add an advanced variable to the pattern
    pub fn add_advanced_variable(&mut self, variable: AdvancedVariable) {
        if self.advanced_variables.is_none() {
            self.advanced_variables = Some(Vec::new());
        }
        self.advanced_variables.as_mut().unwrap().push(variable);
    }

    /// Get advanced variable by name
    pub fn get_advanced_variable(&self, name: &str) -> Option<&AdvancedVariable> {
        if let Some(variables) = &self.advanced_variables {
            variables.iter().find(|v| v.name == name)
        } else {
            None
        }
    }

    /// Validate advanced variables against their rules
    pub fn validate_advanced_variables(&self, values: &HashMap<String, String>) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(variables) = &self.advanced_variables {
            for variable in variables {
                if let Some(value) = values.get(&variable.name) {
                    if let Some(validation) = &variable.validation {
                        // Check required
                        if variable.required && value.is_empty() {
                            errors.push(format!("Variable '{}' is required", variable.name));
                            continue;
                        }

                        // Check length constraints
                        if let Some(min_len) = validation.min_length {
                            if value.len() < min_len {
                                errors.push(format!(
                                    "Variable '{}' must be at least {} characters",
                                    variable.name, min_len
                                ));
                            }
                        }
                        if let Some(max_len) = validation.max_length {
                            if value.len() > max_len {
                                errors.push(format!(
                                    "Variable '{}' must be at most {} characters",
                                    variable.name, max_len
                                ));
                            }
                        }

                        // Check numeric constraints
                        if let Ok(num_value) = value.parse::<f64>() {
                            if let Some(min_val) = validation.min_value {
                                if num_value < min_val {
                                    errors.push(format!(
                                        "Variable '{}' must be at least {}",
                                        variable.name, min_val
                                    ));
                                }
                            }
                            if let Some(max_val) = validation.max_value {
                                if num_value > max_val {
                                    errors.push(format!(
                                        "Variable '{}' must be at most {}",
                                        variable.name, max_val
                                    ));
                                }
                            }
                        }

                        // Check pattern
                        if let Some(pattern) = &validation.pattern {
                            if let Ok(regex) = regex::Regex::new(pattern) {
                                if !regex.is_match(value) {
                                    errors.push(format!(
                                        "Variable '{}' does not match pattern {}",
                                        variable.name, pattern
                                    ));
                                }
                            }
                        }
                    }
                } else if variable.required {
                    errors.push(format!("Variable '{}' is required", variable.name));
                }
            }
        }

        errors
    }

    /// Add a template validation rule
    pub fn add_validation_rule(&mut self, rule: TemplateValidationRule) {
        if self.validation_rules.is_none() {
            self.validation_rules = Some(Vec::new());
        }
        self.validation_rules.as_mut().unwrap().push(rule);
    }

    /// Validate template against all rules
    pub fn validate_template(
        &self,
        context: &str,
        variables: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(rules) = &self.validation_rules {
            for rule in rules {
                if rule.enabled {
                    // Simple condition evaluation (in real implementation, use proper expression evaluator)
                    let condition_met =
                        self.evaluate_condition(&rule.condition, context, variables);
                    if !condition_met {
                        errors.push(rule.error_message.clone());
                    }
                }
            }
        }

        errors
    }

    /// Update performance metrics
    pub fn update_performance_metrics(&mut self, rendering_time: f64, cache_hit: bool) {
        if self.performance_metrics.is_none() {
            self.performance_metrics = Some(TemplatePerformanceMetrics {
                avg_rendering_time: rendering_time,
                max_rendering_time: rendering_time,
                min_rendering_time: rendering_time,
                total_renders: 1,
                cache_hit_rate: if cache_hit { 1.0 } else { 0.0 },
                memory_usage: 0,
                last_updated: chrono::Utc::now(),
            });
        } else {
            let metrics = self.performance_metrics.as_mut().unwrap();
            metrics.total_renders += 1;
            metrics.avg_rendering_time =
                (metrics.avg_rendering_time * (metrics.total_renders - 1) as f64 + rendering_time)
                    / metrics.total_renders as f64;
            metrics.max_rendering_time = metrics.max_rendering_time.max(rendering_time);
            metrics.min_rendering_time = metrics.min_rendering_time.min(rendering_time);
            metrics.cache_hit_rate = (metrics.cache_hit_rate * (metrics.total_renders - 1) as f64
                + if cache_hit { 1.0 } else { 0.0 })
                / metrics.total_renders as f64;
            metrics.last_updated = chrono::Utc::now();
        }
    }

    // P3: Enhanced Context Management

    /// Configure context caching
    pub fn configure_context_cache(&mut self, config: ContextCacheConfig) {
        self.context_cache = Some(config);
    }

    /// Check if context caching is enabled
    pub fn is_context_caching_enabled(&self) -> bool {
        self.context_cache
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false)
    }

    /// Configure context optimization
    pub fn configure_context_optimization(&mut self, config: ContextOptimizationConfig) {
        self.context_optimization = Some(config);
    }

    /// Check if context optimization is enabled
    pub fn is_context_optimization_enabled(&self) -> bool {
        self.context_optimization
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false)
    }

    /// Configure context learning
    pub fn configure_context_learning(&mut self, config: ContextLearningConfig) {
        self.context_learning = Some(config);
    }

    /// Check if context learning is enabled
    pub fn is_context_learning_enabled(&self) -> bool {
        self.context_learning
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false)
    }

    /// Update context quality metrics
    pub fn update_context_quality(&mut self, metrics: ContextQualityMetrics) {
        self.context_quality = Some(metrics);
    }

    /// Get context quality score
    pub fn get_context_quality_score(&self) -> Option<f64> {
        self.context_quality.as_ref().map(|q| q.overall_score)
    }

    /// Get feedback history
    pub fn get_feedback_history(&self) -> Vec<&FeedbackEntry> {
        self.usage_analytics.feedback_history.iter().collect()
    }

    /// Bump version (patch)
    pub fn bump_version(&mut self) {
        self.version.bump_patch();
    }

    /// Bump minor version
    pub fn bump_minor_version(&mut self) {
        self.version.bump_minor();
    }

    /// Bump major version
    pub fn bump_major_version(&mut self) {
        self.version.bump_major();
    }

    /// Set specific version
    pub fn set_version(&mut self, version: &str) {
        self.version = PromptVersion::new(version);
    }

    /// Render template with composition blocks
    pub fn render_with_composition(
        &self,
        context: &str,
        variables: &HashMap<String, String>,
    ) -> String {
        let mut result = self.substitute_variables(context);

        if let Some(blocks) = &self.composition_blocks {
            for block in blocks {
                match block.block_type {
                    CompositionBlockType::Conditional => {
                        if self.evaluate_condition(
                            &block.condition.as_ref().unwrap_or(&String::new()),
                            context,
                            variables,
                        ) {
                            let block_content =
                                self.substitute_variables_with_map(&block.content, variables);
                            result.push_str(&block_content);
                        }
                    }
                    CompositionBlockType::Loop => {
                        if let (Some(var_name), Some(items)) =
                            (&block.loop_variable, &block.loop_items)
                        {
                            for item in items {
                                let mut loop_vars = variables.clone();
                                loop_vars.insert(var_name.clone(), item.clone());
                                let block_content =
                                    self.substitute_variables_with_map(&block.content, &loop_vars);
                                result.push_str(&block_content);
                            }
                        }
                    }
                    CompositionBlockType::Include => {
                        // Include block content directly
                        let block_content =
                            self.substitute_variables_with_map(&block.content, variables);
                        result.push_str(&block_content);
                    }
                    CompositionBlockType::Switch => {
                        // Simple switch implementation
                        if let Some(condition) = &block.condition {
                            let block_content =
                                self.substitute_variables_with_map(&block.content, variables);
                            result.push_str(&block_content);
                        }
                    }
                    CompositionBlockType::Fallback => {
                        // Fallback block - use if no other blocks rendered content
                        if result == self.substitute_variables(context) {
                            let block_content =
                                self.substitute_variables_with_map(&block.content, variables);
                            result.push_str(&block_content);
                        }
                    }
                }
            }
        }

        result
    }

    // Helper methods

    /// Evaluate a condition expression (simplified implementation)
    fn evaluate_condition(
        &self,
        condition: &str,
        _context: &str,
        _variables: &HashMap<String, String>,
    ) -> bool {
        // In a real implementation, this would use a proper expression evaluator
        // For now, return true for non-empty conditions
        !condition.is_empty()
    }

    /// Substitute variables with a specific variable map
    fn substitute_variables_with_map(
        &self,
        template: &str,
        variables: &HashMap<String, String>,
    ) -> String {
        let mut result = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

/// Top-level structure for workflows.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflows {
    pub workflows: Vec<PromptChain>,
}

/// Prompt chain workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChain {
    /// Unique identifier
    pub id: String,
    /// Chain name
    pub name: String,
    /// Chain description
    pub description: Option<String>,
    /// Steps in the chain
    pub steps: Vec<ChainStep>,
    /// Chain metadata
    pub metadata: ChainMetadata,
    /// Optional tags for categorization
    pub tags: Option<Vec<String>>,
}

/// Individual step in a prompt chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStep {
    /// Step identifier
    pub id: String,
    /// Step name
    pub name: String,
    /// Step description
    pub description: Option<String>,
    /// Prompt pattern to use (ID or name)
    pub prompt_pattern: String,
    /// Task type for context injection
    pub task_type: Option<String>,
    /// Step order in the chain
    pub order: u32,
    /// Whether this step is required
    pub required: bool,
    /// Step dependencies (other step IDs that must complete first)
    pub dependencies: Option<Vec<String>>,
    /// Step-specific variables
    pub variables: Option<HashMap<String, String>>,
    /// Step conditions (when to execute this step)
    pub conditions: Option<Vec<String>>,
}

/// Chain metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetadata {
    /// Chain version
    pub version: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Author of the chain
    pub author: Option<String>,
    /// Usage statistics
    pub usage_stats: ChainUsageStats,
    /// Success criteria
    pub success_criteria: Option<Vec<String>>,
}

/// Chain usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainUsageStats {
    /// Total number of times this chain was executed
    pub total_executions: u32,
    /// Number of successful executions
    pub successful_executions: u32,
    /// Average execution time in seconds
    pub average_execution_time: Option<f64>,
    /// Last executed timestamp
    pub last_executed: Option<DateTime<Utc>>,
}

impl ChainUsageStats {
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }

    /// Record a new execution
    pub fn record_execution(&mut self, successful: bool, execution_time: Option<f64>) {
        self.total_executions += 1;
        if successful {
            self.successful_executions += 1;
        }
        self.last_executed = Some(Utc::now());

        if let Some(time) = execution_time {
            // Update average execution time
            let current_avg = self.average_execution_time.unwrap_or(0.0);
            let new_avg = (current_avg * (self.total_executions - 1) as f64 + time)
                / self.total_executions as f64;
            self.average_execution_time = Some(new_avg);
        }
    }

    /// Create new usage stats
    pub fn new() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            average_execution_time: None,
            last_executed: None,
        }
    }
}

/// Template library for sharing templates across teams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLibrary {
    /// Library name
    pub name: String,
    /// Library description
    pub description: Option<String>,
    /// Library owner/team
    pub owner: String,
    /// Library version
    pub version: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Templates in this library
    pub templates: Vec<SharedTemplate>,
    /// Library tags for categorization
    pub tags: Option<Vec<String>>,
    /// Access control settings
    pub access_control: Option<TemplateAccessControl>,
}

/// Shared template with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: Option<String>,
    /// Template content
    pub template: String,
    /// Template metadata
    pub metadata: TemplateMetadata,
    /// Template tags
    pub tags: Option<Vec<String>>,
    /// Usage statistics
    pub usage_stats: TemplateUsageStats,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template author
    pub author: Option<String>,
    /// Template version
    pub version: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Template category
    pub category: Option<String>,
    /// Template complexity level
    pub complexity: Option<TemplateComplexity>,
    /// Template language/framework
    pub language: Option<String>,
    /// Template dependencies
    pub dependencies: Option<Vec<String>>,
    /// Template examples
    pub examples: Option<Vec<String>>,
}

/// Template complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateComplexity {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Template access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAccessControl {
    /// Public access (true = public, false = private)
    pub public: bool,
    /// Allowed teams/organizations
    pub allowed_teams: Option<Vec<String>>,
    /// Allowed users
    pub allowed_users: Option<Vec<String>>,
    /// Read-only access (true = read-only, false = editable)
    pub read_only: bool,
}

/// Template usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateUsageStats {
    /// Total downloads
    pub total_downloads: u32,
    /// Total uses
    pub total_uses: u32,
    /// Average rating (1.0-5.0)
    pub average_rating: Option<f64>,
    /// Number of ratings
    pub rating_count: u32,
    /// Last downloaded timestamp
    pub last_downloaded: Option<DateTime<Utc>>,
    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,
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

/// Template import/export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExport {
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Exported templates
    pub templates: Vec<SharedTemplate>,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
    /// Export version
    pub export_version: String,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Source scope
    pub source_scope: String,
    /// Export description
    pub description: Option<String>,
    /// Export tags
    pub tags: Option<Vec<String>>,
    /// Export author
    pub author: Option<String>,
}

// Validation regex patterns
lazy_static::lazy_static! {
    static ref NAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    static ref ID_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    static ref VERSION_REGEX: regex::Regex = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    static ref VERSION_CONSTRAINT_REGEX: regex::Regex = regex::Regex::new(r"^[\d\.,<>=~^!]+$").unwrap();
}

// Custom validation functions
#[allow(dead_code)]
fn validate_dependency_type(dependency_type: &str) -> Result<(), ValidationError> {
    let valid_types = ["required", "optional", "peer"];
    if !valid_types.contains(&dependency_type) {
        return Err(ValidationError::new("invalid_dependency_type"));
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_tags(tags: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(tags) = tags {
        for tag in tags {
            if tag.is_empty() || tag.len() > 50 {
                return Err(ValidationError::new("invalid_tag"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_updated_at(updated_at: &Option<DateTime<Utc>>) -> Result<(), ValidationError> {
    if let Some(updated_at) = updated_at {
        if *updated_at < Utc::now() - chrono::Duration::days(365) {
            return Err(ValidationError::new("updated_at_too_old"));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_due_date(due_date: &Option<DateTime<Utc>>) -> Result<(), ValidationError> {
    if let Some(due_date) = due_date {
        if *due_date < Utc::now() {
            return Err(ValidationError::new("due_date_in_past"));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_completion_timestamp(
    completed_at: &Option<DateTime<Utc>>,
) -> Result<(), ValidationError> {
    if let Some(completed_at) = completed_at {
        if *completed_at < Utc::now() - chrono::Duration::days(365) {
            return Err(ValidationError::new("completion_too_old"));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_related_knowledge(
    related_knowledge: &Option<Vec<String>>,
) -> Result<(), ValidationError> {
    if let Some(related) = related_knowledge {
        for id in related {
            if !ID_REGEX.is_match(id) {
                return Err(ValidationError::new("invalid_related_knowledge_id"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_alternatives(alternatives: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(alternatives) = alternatives {
        for alt in alternatives {
            if alt.is_empty() || alt.len() > 200 {
                return Err(ValidationError::new("invalid_alternative"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_consequences(consequences: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(consequences) = consequences {
        for consequence in consequences {
            if consequence.is_empty() || consequence.len() > 500 {
                return Err(ValidationError::new("invalid_consequence"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_decision_makers(decision_makers: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(makers) = decision_makers {
        for maker in makers {
            if maker.is_empty() || maker.len() > 100 {
                return Err(ValidationError::new("invalid_decision_maker"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_examples(examples: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(examples) = examples {
        for example in examples {
            if example.is_empty() || example.len() > 1000 {
                return Err(ValidationError::new("invalid_example"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_anti_patterns(anti_patterns: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(anti_patterns) = anti_patterns {
        for anti_pattern in anti_patterns {
            if anti_pattern.is_empty() || anti_pattern.len() > 200 {
                return Err(ValidationError::new("invalid_anti_pattern"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_related_patterns(
    related_patterns: &Option<Vec<String>>,
) -> Result<(), ValidationError> {
    if let Some(related) = related_patterns {
        for id in related {
            if !ID_REGEX.is_match(id) {
                return Err(ValidationError::new("invalid_related_pattern_id"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_tools(tools: &Option<Vec<String>>) -> Result<(), ValidationError> {
    if let Some(tools) = tools {
        for tool in tools {
            if tool.is_empty() || tool.len() > 100 {
                return Err(ValidationError::new("invalid_tool"));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_review_date(review_date: &Option<DateTime<Utc>>) -> Result<(), ValidationError> {
    if let Some(review_date) = review_date {
        if *review_date < Utc::now() {
            return Err(ValidationError::new("review_date_in_past"));
        }
    }
    Ok(())
}

/// Schema validation trait with enhanced validation
pub trait Validatable {
    fn validate(&self) -> crate::RhemaResult<()>;
    fn validate_schema_version(&self) -> crate::RhemaResult<()>;
    fn validate_cross_fields(&self) -> crate::RhemaResult<()>;
}

impl Validatable for RhemaScope {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        if self.name.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope name cannot be empty".to_string(),
            ));
        }

        if self.scope_type.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope type cannot be empty".to_string(),
            ));
        }

        if self.version.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Version cannot be empty".to_string(),
            ));
        }

        // Validate version format (semver-like)
        if !self
            .version
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        {
            return Err(crate::RhemaError::ValidationError(
                "Invalid version format".to_string(),
            ));
        }

        // Validate dependencies if present
        if let Some(deps) = &self.dependencies {
            for _dep in deps {
                // TODO: Implement validation for ScopeDependency
                // TODO: Add lock file schema structures (RhemaLock, LockedScope, LockedDependency)
                // TODO: Integrate with lock file system for deterministic dependency resolution
                // dep.validate()?;
            }
        }

        // Validate protocol info if present
        if let Some(info) = &self.protocol_info {
            info.validate()?;
        }

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        if let Some(schema_version) = &self.schema_version {
            if schema_version != CURRENT_SCHEMA_VERSION {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Schema version mismatch. Expected {}, got {}",
                    CURRENT_SCHEMA_VERSION, schema_version
                )));
            }
        }
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that scope type is valid
        let valid_types = [
            "repository",
            "service",
            "application",
            "library",
            "component",
        ];
        if !valid_types.contains(&self.scope_type.as_str()) {
            return Err(crate::RhemaError::ValidationError(format!(
                "Invalid scope type: {}. Valid types are: {}",
                self.scope_type,
                valid_types.join(", ")
            )));
        }

        Ok(())
    }
}

impl Validatable for ProtocolInfo {
    fn validate(&self) -> crate::RhemaResult<()> {
        if self.version.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Protocol version cannot be empty".to_string(),
            ));
        }

        // Validate concepts if present
        if let Some(concepts) = &self.concepts {
            for concept in concepts {
                if concept.name.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Concept name cannot be empty".to_string(),
                    ));
                }
                if concept.description.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Concept description cannot be empty".to_string(),
                    ));
                }
            }
        }

        // Validate CQL examples if present
        if let Some(examples) = &self.cql_examples {
            for example in examples {
                if example.name.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "CQL example name cannot be empty".to_string(),
                    ));
                }
                if example.query.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "CQL query cannot be empty".to_string(),
                    ));
                }
                if example.description.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "CQL example description cannot be empty".to_string(),
                    ));
                }
            }
        }

        // Validate patterns if present
        if let Some(patterns) = &self.patterns {
            for pattern in patterns {
                if pattern.name.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Pattern name cannot be empty".to_string(),
                    ));
                }
                if pattern.description.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Pattern description cannot be empty".to_string(),
                    ));
                }
            }
        }

        // Validate integrations if present
        if let Some(integrations) = &self.integrations {
            for integration in integrations {
                if integration.name.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Integration name cannot be empty".to_string(),
                    ));
                }
                if integration.description.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Integration description cannot be empty".to_string(),
                    ));
                }
            }
        }

        // Validate troubleshooting if present
        if let Some(troubleshooting) = &self.troubleshooting {
            for item in troubleshooting {
                if item.issue.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Troubleshooting issue cannot be empty".to_string(),
                    ));
                }
                if item.description.trim().is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Troubleshooting description cannot be empty".to_string(),
                    ));
                }
                if item.solution.is_empty() {
                    return Err(crate::RhemaError::ValidationError(
                        "Troubleshooting solution cannot be empty".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // Protocol info doesn't have its own schema version
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that concept names are unique
        if let Some(concepts) = &self.concepts {
            let mut names = std::collections::HashSet::new();
            for concept in concepts {
                if !names.insert(&concept.name) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate concept name: {}",
                        concept.name
                    )));
                }
            }
        }

        // Validate that CQL example names are unique
        if let Some(examples) = &self.cql_examples {
            let mut names = std::collections::HashSet::new();
            for example in examples {
                if !names.insert(&example.name) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate CQL example name: {}",
                        example.name
                    )));
                }
            }
        }

        // Validate that pattern names are unique
        if let Some(patterns) = &self.patterns {
            let mut names = std::collections::HashSet::new();
            for pattern in patterns {
                if !names.insert(&pattern.name) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate pattern name: {}",
                        pattern.name
                    )));
                }
            }
        }

        // Validate that integration names are unique
        if let Some(integrations) = &self.integrations {
            let mut names = std::collections::HashSet::new();
            for integration in integrations {
                if !names.insert(&integration.name) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate integration name: {}",
                        integration.name
                    )));
                }
            }
        }

        // Validate that troubleshooting issue names are unique
        if let Some(troubleshooting) = &self.troubleshooting {
            let mut names = std::collections::HashSet::new();
            for item in troubleshooting {
                if !names.insert(&item.issue) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate troubleshooting issue: {}",
                        item.issue
                    )));
                }
            }
        }

        Ok(())
    }
}

impl Validatable for Knowledge {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        if self.entries.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Knowledge must contain at least one entry".to_string(),
            ));
        }

        for entry in &self.entries {
            if entry.title.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Knowledge entry title cannot be empty".to_string(),
                ));
            }
            if entry.content.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Knowledge entry content cannot be empty".to_string(),
                ));
            }
        }

        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // Knowledge doesn't have schema version field, so always valid
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for entry in &self.entries {
            if !ids.insert(&entry.id) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate knowledge entry ID: {}",
                    entry.id
                )));
            }
        }

        // Validate that categories exist for entries that reference them
        if let Some(categories) = &self.categories {
            for entry in &self.entries {
                if let Some(category) = &entry.category {
                    if !categories.contains_key(category) {
                        return Err(crate::RhemaError::ValidationError(format!(
                            "Knowledge entry references non-existent category: {}",
                            category
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Todos {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        for todo in &self.todos {
            if todo.title.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Todo title cannot be empty".to_string(),
                ));
            }
        }

        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for todo in &self.todos {
            if !ids.insert(&todo.id) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate todo ID: {}",
                    todo.id
                )));
            }
        }

        // Validate completion timestamps
        for todo in &self.todos {
            if todo.status == TodoStatus::Completed {
                if todo.completed_at.is_none() {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Completed todo {} must have completion timestamp",
                        todo.id
                    )));
                }
            } else if todo.completed_at.is_some() {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Non-completed todo {} cannot have completion timestamp",
                    todo.id
                )));
            }
        }
        Ok(())
    }
}

impl Validatable for Decisions {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        for decision in &self.decisions {
            if decision.title.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Decision title cannot be empty".to_string(),
                ));
            }
            if decision.description.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Decision description cannot be empty".to_string(),
                ));
            }
        }

        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for decision in &self.decisions {
            if !ids.insert(&decision.id) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate decision ID: {}",
                    decision.id
                )));
            }
        }

        // Validate review dates
        for decision in &self.decisions {
            if let Some(review_date) = decision.review_date {
                if review_date <= decision.decided_at {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Decision {} review date must be after decision date",
                        decision.id
                    )));
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Patterns {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        for pattern in &self.patterns {
            if pattern.name.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Pattern name cannot be empty".to_string(),
                ));
            }
            if pattern.description.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Pattern description cannot be empty".to_string(),
                ));
            }
        }

        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for pattern in &self.patterns {
            if !ids.insert(&pattern.id) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate pattern ID: {}",
                    pattern.id
                )));
            }
        }

        // Validate related patterns exist
        for pattern in &self.patterns {
            if let Some(related) = &pattern.related_patterns {
                for related_id in related {
                    if !ids.contains(related_id) {
                        return Err(crate::RhemaError::ValidationError(format!(
                            "Pattern {} references non-existent pattern: {}",
                            pattern.id, related_id
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Conventions {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Basic field validation
        for convention in &self.conventions {
            if convention.name.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Convention name cannot be empty".to_string(),
                ));
            }
            if convention.description.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Convention description cannot be empty".to_string(),
                ));
            }
        }

        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for convention in &self.conventions {
            if !ids.insert(&convention.id) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate convention ID: {}",
                    convention.id
                )));
            }
        }
        Ok(())
    }
}

/// Schema migration utilities
pub trait SchemaMigratable {
    fn migrate_to_latest(&mut self) -> crate::RhemaResult<()>;
    fn get_schema_version(&self) -> Option<String>;
}

impl SchemaMigratable for RhemaScope {
    fn migrate_to_latest(&mut self) -> crate::RhemaResult<()> {
        let current_version = self
            .get_schema_version()
            .unwrap_or_else(|| "0.1.0".to_string());

        if current_version != CURRENT_SCHEMA_VERSION {
            // Perform migrations based on version
            match current_version.as_str() {
                "0.1.0" => {
                    // Add schema_version field if missing
                    if self.schema_version.is_none() {
                        self.schema_version = Some(CURRENT_SCHEMA_VERSION.to_string());
                    }
                }
                "0.2.0" => {
                    // Future migration logic
                }
                _ => {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Cannot migrate from version {} to {}",
                        current_version, CURRENT_SCHEMA_VERSION
                    )));
                }
            }

            // Update to current version
            self.schema_version = Some(CURRENT_SCHEMA_VERSION.to_string());
        }

        Ok(())
    }

    fn get_schema_version(&self) -> Option<String> {
        self.schema_version.clone()
    }
}

/// JSON Schema generation
pub trait JsonSchema {
    fn json_schema() -> serde_json::Value;
}

impl JsonSchema for RhemaScope {
    fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "required": ["name", "scope_type", "version"],
            "properties": {
                "name": {
                    "type": "string",
                    "minLength": 1,
                    "pattern": "^[a-zA-Z0-9_-]+$"
                },
                "scope_type": {
                    "type": "string",
                    "minLength": 1
                },
                "description": {
                    "type": "string"
                },
                "version": {
                    "type": "string",
                    "pattern": "^\\d+\\.\\d+\\.\\d+$"
                },
                "schema_version": {
                    "type": "string",
                    "pattern": "^\\d+\\.\\d+\\.\\d+$"
                },
                "dependencies": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["path", "dependency_type"],
                        "properties": {
                            "path": {
                                "type": "string",
                                "minLength": 1
                            },
                            "dependency_type": {
                                "type": "string",
                                "enum": ["required", "optional", "peer"]
                            },
                            "version": {
                                "type": "string",
                                "pattern": "^[\\d\\.,<>=~^!]+$"
                            }
                        }
                    }
                }
            }
        })
    }
}

/// Dependency type enumeration for lock files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Peer,
    Development,
    Build,
}

/// Validation status for lock files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    Warning,
    Pending,
}

/// Resolution strategy for dependency conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionStrategy {
    Latest,
    Earliest,
    Pinned,
    Range,
    Compatible,
}

/// Conflict resolution method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    Manual,
    Automatic,
    Prompt,
    Skip,
    Fail,
}

/// Main lock file structure for the Rhema lock file system
///
/// This structure represents the complete state of a Rhema project's dependencies
/// and provides deterministic, reproducible builds by locking all dependency
/// versions and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhemaLock {
    /// Version of the lock file format
    pub lockfile_version: String,

    /// Timestamp when the lock file was generated
    pub generated_at: DateTime<Utc>,

    /// Information about what generated this lock file
    pub generated_by: String,

    /// Checksum of the lock file contents for integrity verification
    pub checksum: String,

    /// Locked scopes in the project
    pub scopes: HashMap<String, LockedScope>,

    /// Metadata about the lock file
    pub metadata: LockMetadata,
}

/// Individual locked scope structure
///
/// Represents a single scope with its locked dependencies and metadata.
/// Each scope is identified by its path and contains all resolved dependencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedScope {
    /// Version of the scope that was locked
    pub version: String,

    /// Path to the scope in the project
    pub path: String,

    /// All dependencies for this scope with their resolved versions
    pub dependencies: HashMap<String, LockedDependency>,

    /// Checksum of the scope's source for integrity verification
    pub source_checksum: Option<String>,

    /// Timestamp when this scope was last resolved
    pub resolved_at: DateTime<Utc>,

    /// Whether this scope has any circular dependencies
    pub has_circular_dependencies: bool,

    /// Custom metadata for the scope
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

/// Individual locked dependency structure
///
/// Represents a single dependency with its resolved version and metadata.
/// This ensures deterministic builds by locking the exact version and path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    /// Resolved version of the dependency
    pub version: String,

    /// Path to the dependency
    pub path: String,

    /// Timestamp when this dependency was resolved
    pub resolved_at: DateTime<Utc>,

    /// Checksum of the dependency for integrity verification
    pub checksum: String,

    /// Type of dependency (required, optional, peer, etc.)
    pub dependency_type: DependencyType,

    /// Original version constraint that was resolved
    pub original_constraint: Option<String>,

    /// Whether this dependency is transitive (dependency of a dependency)
    pub is_transitive: bool,

    /// Direct dependencies of this dependency
    pub dependencies: Option<Vec<String>>,

    /// Custom metadata for the dependency
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

/// Metadata structure for lock files
///
/// Contains aggregate information about the lock file and its contents,
/// useful for validation, analysis, and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockMetadata {
    /// Total number of scopes in the lock file
    pub total_scopes: u32,

    /// Total number of dependencies across all scopes
    pub total_dependencies: u32,

    /// Number of circular dependencies detected
    pub circular_dependencies: u32,

    /// Overall validation status of the lock file
    pub validation_status: ValidationStatus,

    /// Strategy used for dependency resolution
    pub resolution_strategy: ResolutionStrategy,

    /// Method used for conflict resolution
    pub conflict_resolution: ConflictResolution,

    /// Timestamp when the lock file was last validated
    pub last_validated: Option<DateTime<Utc>>,

    /// List of validation warnings or errors
    pub validation_messages: Option<Vec<String>>,

    /// Performance metrics for lock file operations
    pub performance_metrics: Option<LockPerformanceMetrics>,

    /// Custom metadata
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

/// Performance metrics for lock file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPerformanceMetrics {
    /// Time taken to generate the lock file in milliseconds
    pub generation_time_ms: u64,

    /// Time taken to validate the lock file in milliseconds
    pub validation_time_ms: Option<u64>,

    /// Memory usage during lock file operations in bytes
    pub memory_usage_bytes: Option<u64>,

    /// Number of dependency resolution attempts
    pub resolution_attempts: u32,

    /// Number of cache hits during resolution
    pub cache_hits: u32,

    /// Number of cache misses during resolution
    pub cache_misses: u32,
}

impl RhemaLock {
    /// Create a new RhemaLock with default values
    pub fn new(generated_by: &str) -> Self {
        Self {
            lockfile_version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            generated_by: generated_by.to_string(),
            checksum: String::new(), // Will be calculated later
            scopes: HashMap::new(),
            metadata: LockMetadata::new(),
        }
    }

    /// Calculate the checksum of the lock file contents
    pub fn calculate_checksum(&self) -> String {
        // TODO: Implement proper checksum calculation
        // For now, return a placeholder
        format!("checksum_{}", self.generated_at.timestamp())
    }

    /// Update the checksum field
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    /// Add a locked scope to the lock file
    pub fn add_scope(&mut self, path: String, scope: LockedScope) {
        self.scopes.insert(path, scope);
        self.update_metadata();
    }

    /// Get a locked scope by path
    pub fn get_scope(&self, path: &str) -> Option<&LockedScope> {
        self.scopes.get(path)
    }

    /// Remove a scope from the lock file
    pub fn remove_scope(&mut self, path: &str) -> Option<LockedScope> {
        let scope = self.scopes.remove(path);
        self.update_metadata();
        scope
    }

    /// Update metadata based on current scopes
    fn update_metadata(&mut self) {
        let total_scopes = self.scopes.len() as u32;
        let total_dependencies: u32 = self
            .scopes
            .values()
            .map(|scope| scope.dependencies.len() as u32)
            .sum();
        let circular_dependencies: u32 = self
            .scopes
            .values()
            .filter(|scope| scope.has_circular_dependencies)
            .count() as u32;

        self.metadata.total_scopes = total_scopes;
        self.metadata.total_dependencies = total_dependencies;
        self.metadata.circular_dependencies = circular_dependencies;
    }

    /// Validate the lock file
    pub fn validate(&self) -> crate::RhemaResult<()> {
        // Validate lock file version
        if self.lockfile_version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Lock file version cannot be empty".to_string(),
            ));
        }

        // Validate generated_by
        if self.generated_by.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Generated by field cannot be empty".to_string(),
            ));
        }

        // Validate checksum
        if self.checksum.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Checksum cannot be empty".to_string(),
            ));
        }

        // Validate scopes
        for (path, scope) in &self.scopes {
            if path.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Scope path cannot be empty".to_string(),
                ));
            }
            scope.validate()?;
        }

        // Validate metadata
        self.metadata.validate()?;

        Ok(())
    }
}

impl LockedScope {
    /// Create a new LockedScope
    pub fn new(version: &str, path: &str) -> Self {
        Self {
            version: version.to_string(),
            path: path.to_string(),
            dependencies: HashMap::new(),
            source_checksum: None,
            resolved_at: Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        }
    }

    /// Add a dependency to the scope
    pub fn add_dependency(&mut self, name: String, dependency: LockedDependency) {
        self.dependencies.insert(name, dependency);
    }

    /// Get a dependency by name
    pub fn get_dependency(&self, name: &str) -> Option<&LockedDependency> {
        self.dependencies.get(name)
    }

    /// Remove a dependency from the scope
    pub fn remove_dependency(&mut self, name: &str) -> Option<LockedDependency> {
        self.dependencies.remove(name)
    }

    /// Check if the scope has any dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Get the number of dependencies
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Validate the locked scope
    pub fn validate(&self) -> crate::RhemaResult<()> {
        if self.version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope version cannot be empty".to_string(),
            ));
        }

        if self.path.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope path cannot be empty".to_string(),
            ));
        }

        // Validate dependencies
        for (name, dependency) in &self.dependencies {
            if name.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Dependency name cannot be empty".to_string(),
                ));
            }
            dependency.validate()?;
        }

        Ok(())
    }
}

impl LockedDependency {
    /// Create a new LockedDependency
    pub fn new(version: &str, path: &str, dependency_type: DependencyType) -> Self {
        Self {
            version: version.to_string(),
            path: path.to_string(),
            resolved_at: Utc::now(),
            checksum: String::new(), // Will be calculated later
            dependency_type,
            original_constraint: None,
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        }
    }

    /// Calculate the checksum of the dependency
    pub fn calculate_checksum(&self) -> String {
        // TODO: Implement proper checksum calculation
        // For now, return a placeholder
        format!("dep_checksum_{}_{}", self.path, self.version)
    }

    /// Update the checksum field
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    /// Set the original version constraint
    pub fn set_original_constraint(&mut self, constraint: &str) {
        self.original_constraint = Some(constraint.to_string());
    }

    /// Mark the dependency as transitive
    pub fn mark_transitive(&mut self) {
        self.is_transitive = true;
    }

    /// Add a direct dependency
    pub fn add_dependency(&mut self, dependency_name: &str) {
        if let Some(ref mut deps) = self.dependencies {
            deps.push(dependency_name.to_string());
        } else {
            self.dependencies = Some(vec![dependency_name.to_string()]);
        }
    }

    /// Validate the locked dependency
    pub fn validate(&self) -> crate::RhemaResult<()> {
        if self.version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency version cannot be empty".to_string(),
            ));
        }

        if self.path.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency path cannot be empty".to_string(),
            ));
        }

        if self.checksum.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency checksum cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl LockMetadata {
    /// Create a new LockMetadata with default values
    pub fn new() -> Self {
        Self {
            total_scopes: 0,
            total_dependencies: 0,
            circular_dependencies: 0,
            validation_status: ValidationStatus::Pending,
            resolution_strategy: ResolutionStrategy::Latest,
            conflict_resolution: ConflictResolution::Automatic,
            last_validated: None,
            validation_messages: None,
            performance_metrics: None,
            custom: HashMap::new(),
        }
    }

    /// Set the validation status
    pub fn set_validation_status(&mut self, status: ValidationStatus) {
        self.validation_status = status;
        self.last_validated = Some(Utc::now());
    }

    /// Add a validation message
    pub fn add_validation_message(&mut self, message: &str) {
        if let Some(ref mut messages) = self.validation_messages {
            messages.push(message.to_string());
        } else {
            self.validation_messages = Some(vec![message.to_string()]);
        }
    }

    /// Set performance metrics
    pub fn set_performance_metrics(&mut self, metrics: LockPerformanceMetrics) {
        self.performance_metrics = Some(metrics);
    }

    /// Calculate cache hit rate
    pub fn cache_hit_rate(&self) -> Option<f64> {
        if let Some(ref metrics) = self.performance_metrics {
            let total = metrics.cache_hits + metrics.cache_misses;
            if total > 0 {
                Some(metrics.cache_hits as f64 / total as f64)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Validate the lock metadata
    pub fn validate(&self) -> crate::RhemaResult<()> {
        // Basic validation - metadata should always be valid
        // as it's generated internally
        Ok(())
    }
}

impl Default for LockMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl LockPerformanceMetrics {
    /// Create new performance metrics
    pub fn new(generation_time_ms: u64) -> Self {
        Self {
            generation_time_ms,
            validation_time_ms: None,
            memory_usage_bytes: None,
            resolution_attempts: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Set validation time
    pub fn set_validation_time(&mut self, time_ms: u64) {
        self.validation_time_ms = Some(time_ms);
    }

    /// Set memory usage
    pub fn set_memory_usage(&mut self, bytes: u64) {
        self.memory_usage_bytes = Some(bytes);
    }

    /// Increment resolution attempts
    pub fn increment_resolution_attempts(&mut self) {
        self.resolution_attempts += 1;
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
}

// Implement Validatable trait for lock file structures
impl Validatable for RhemaLock {
    fn validate(&self) -> crate::RhemaResult<()> {
        self.validate()
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // Validate lock file version format
        if !self
            .lockfile_version
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.')
        {
            return Err(crate::RhemaError::ValidationError(
                "Invalid lock file version format".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that all scope paths are unique
        let mut paths = std::collections::HashSet::new();
        for path in self.scopes.keys() {
            if !paths.insert(path) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate scope path: {}",
                    path
                )));
            }
        }

        // Validate that metadata counts match actual counts
        if self.metadata.total_scopes as usize != self.scopes.len() {
            return Err(crate::RhemaError::ValidationError(
                "Metadata total_scopes does not match actual scope count".to_string(),
            ));
        }

        let actual_deps: u32 = self
            .scopes
            .values()
            .map(|scope| scope.dependencies.len() as u32)
            .sum();

        if self.metadata.total_dependencies != actual_deps {
            return Err(crate::RhemaError::ValidationError(
                "Metadata total_dependencies does not match actual dependency count".to_string(),
            ));
        }

        Ok(())
    }
}

impl Validatable for LockedScope {
    fn validate(&self) -> crate::RhemaResult<()> {
        self.validate()
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // LockedScope doesn't have its own schema version
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that dependency names are unique
        let mut names = std::collections::HashSet::new();
        for name in self.dependencies.keys() {
            if !names.insert(name) {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Duplicate dependency name in scope {}: {}",
                    self.path, name
                )));
            }
        }

        Ok(())
    }
}

impl Validatable for LockedDependency {
    fn validate(&self) -> crate::RhemaResult<()> {
        self.validate()
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // LockedDependency doesn't have its own schema version
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that dependencies list doesn't contain duplicates
        if let Some(ref deps) = self.dependencies {
            let mut names = std::collections::HashSet::new();
            for dep in deps {
                if !names.insert(dep) {
                    return Err(crate::RhemaError::ValidationError(format!(
                        "Duplicate dependency in dependency list: {}",
                        dep
                    )));
                }
            }
        }

        Ok(())
    }
}

impl Validatable for LockMetadata {
    fn validate(&self) -> crate::RhemaResult<()> {
        self.validate()
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // LockMetadata doesn't have its own schema version
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that circular dependencies count is reasonable
        if self.circular_dependencies > self.total_scopes {
            return Err(crate::RhemaError::ValidationError(
                "Circular dependencies count cannot exceed total scopes".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhema_lock_creation() {
        let lock = RhemaLock::new("test-generator");

        assert_eq!(lock.lockfile_version, "1.0.0");
        assert_eq!(lock.generated_by, "test-generator");
        assert!(lock.scopes.is_empty());
        assert_eq!(lock.metadata.total_scopes, 0);
        assert_eq!(lock.metadata.total_dependencies, 0);
    }

    #[test]
    fn test_locked_scope_creation() {
        let scope = LockedScope::new("1.2.3", "crates/rhema-core");

        assert_eq!(scope.version, "1.2.3");
        assert_eq!(scope.path, "crates/rhema-core");
        assert!(scope.dependencies.is_empty());
        assert!(!scope.has_circular_dependencies);
    }

    #[test]
    fn test_locked_dependency_creation() {
        let dep = LockedDependency::new("2.0.0", "crates/rhema-ai", DependencyType::Required);

        assert_eq!(dep.version, "2.0.0");
        assert_eq!(dep.path, "crates/rhema-ai");
        assert_eq!(dep.dependency_type, DependencyType::Required);
        assert!(!dep.is_transitive);
        assert!(dep.original_constraint.is_none());
    }

    #[test]
    fn test_lock_metadata_creation() {
        let metadata = LockMetadata::new();

        assert_eq!(metadata.total_scopes, 0);
        assert_eq!(metadata.total_dependencies, 0);
        assert_eq!(metadata.circular_dependencies, 0);
        assert_eq!(metadata.validation_status, ValidationStatus::Pending);
        assert_eq!(metadata.resolution_strategy, ResolutionStrategy::Latest);
        assert_eq!(metadata.conflict_resolution, ConflictResolution::Automatic);
    }

    #[test]
    fn test_adding_scope_to_lock() {
        let mut lock = RhemaLock::new("test-generator");
        let scope = LockedScope::new("1.0.0", "crates/rhema-core");

        lock.add_scope("crates/rhema-core".to_string(), scope);

        assert_eq!(lock.scopes.len(), 1);
        assert!(lock.get_scope("crates/rhema-core").is_some());
        assert_eq!(lock.metadata.total_scopes, 1);
    }

    #[test]
    fn test_adding_dependency_to_scope() {
        let mut scope = LockedScope::new("1.0.0", "crates/rhema-core");
        let dep = LockedDependency::new("2.0.0", "crates/rhema-ai", DependencyType::Required);

        scope.add_dependency("ai".to_string(), dep);

        assert_eq!(scope.dependencies.len(), 1);
        assert!(scope.get_dependency("ai").is_some());
        assert!(scope.has_dependencies());
        assert_eq!(scope.dependency_count(), 1);
    }

    #[test]
    fn test_dependency_operations() {
        let mut dep = LockedDependency::new("1.0.0", "crates/test", DependencyType::Optional);

        dep.set_original_constraint("^1.0.0");
        dep.mark_transitive();
        dep.add_dependency("sub-dep");

        assert_eq!(dep.original_constraint, Some("^1.0.0".to_string()));
        assert!(dep.is_transitive);
        assert_eq!(dep.dependencies, Some(vec!["sub-dep".to_string()]));
    }

    #[test]
    fn test_metadata_operations() {
        let mut metadata = LockMetadata::new();

        metadata.set_validation_status(ValidationStatus::Valid);
        metadata.add_validation_message("Test message");

        assert_eq!(metadata.validation_status, ValidationStatus::Valid);
        assert!(metadata.last_validated.is_some());
        assert_eq!(
            metadata.validation_messages,
            Some(vec!["Test message".to_string()])
        );
    }

    #[test]
    fn test_performance_metrics() {
        let mut metrics = LockPerformanceMetrics::new(100);

        metrics.set_validation_time(50);
        metrics.set_memory_usage(1024);
        metrics.record_cache_hit();
        metrics.record_cache_miss();
        metrics.record_cache_hit();

        assert_eq!(metrics.generation_time_ms, 100);
        assert_eq!(metrics.validation_time_ms, Some(50));
        assert_eq!(metrics.memory_usage_bytes, Some(1024));
        assert_eq!(metrics.cache_hits, 2);
        assert_eq!(metrics.cache_misses, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let mut lock = RhemaLock::new("test-generator");
        let mut scope = LockedScope::new("1.0.0", "crates/rhema-core");
        let dep = LockedDependency::new("2.0.0", "crates/rhema-ai", DependencyType::Required);

        scope.add_dependency("ai".to_string(), dep);
        lock.add_scope("crates/rhema-core".to_string(), scope);

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&lock).unwrap();
        assert!(!yaml.is_empty());

        // Deserialize from YAML
        let deserialized_lock: RhemaLock = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized_lock.lockfile_version, lock.lockfile_version);
        assert_eq!(deserialized_lock.generated_by, lock.generated_by);
        assert_eq!(deserialized_lock.scopes.len(), lock.scopes.len());
    }

    #[test]
    fn test_validation() {
        let mut lock = RhemaLock::new("test-generator");
        let scope = LockedScope::new("1.0.0", "crates/rhema-core");

        lock.add_scope("crates/rhema-core".to_string(), scope);
        lock.update_checksum();

        // Should validate successfully
        assert!(lock.validate().is_ok());
    }

    #[test]
    fn test_validation_errors() {
        let mut lock = RhemaLock::new("test-generator");

        // Empty checksum should cause validation error
        lock.checksum = String::new();
        assert!(lock.validate().is_err());

        // Empty generated_by should cause validation error
        lock.generated_by = String::new();
        assert!(lock.validate().is_err());
    }

    #[test]
    fn test_enum_serialization() {
        let dep_type = DependencyType::Required;
        let yaml = serde_yaml::to_string(&dep_type).unwrap();
        assert_eq!(yaml.trim(), "required");

        let deserialized: DependencyType = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized, DependencyType::Required);
    }

    #[test]
    fn test_cache_hit_rate_calculation() {
        let mut metadata = LockMetadata::new();
        let mut metrics = LockPerformanceMetrics::new(100);

        metrics.record_cache_hit();
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        metadata.set_performance_metrics(metrics);

        let hit_rate = metadata.cache_hit_rate().unwrap();
        assert!((hit_rate - 2.0 / 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_cache_hit_rate() {
        let metadata = LockMetadata::new();
        assert_eq!(metadata.cache_hit_rate(), None);
    }

    #[test]
    fn test_prompt_pattern_creation() {
        let pattern = PromptPattern::new(
            "test-pattern",
            "Test Pattern",
            "Review this code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        assert_eq!(pattern.id, "test-pattern");
        assert_eq!(pattern.name, "Test Pattern");
        assert_eq!(pattern.template, "Review this code: {{CONTEXT}}");
        assert_eq!(pattern.total_uses(), 0);
        assert_eq!(pattern.successful_uses(), 0);
        assert_eq!(pattern.success_rate(), 0.0);
    }

    #[test]
    fn test_context_rule_creation() {
        let rule = ContextRule::new(
            "task_type == 'code_review'",
            vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        );

        assert_eq!(rule.condition, "task_type == 'code_review'");
        assert_eq!(rule.context_files.len(), 2);
        assert_eq!(rule.priority, None);
    }

    #[test]
    fn test_context_rule_with_priority() {
        let rule = ContextRule::new(
            "task_type == 'bug_fix'",
            vec!["knowledge.yaml".to_string()],
            ContextInjectionMethod::Append,
        )
        .with_priority(2);

        assert_eq!(rule.priority, Some(2));
    }

    #[test]
    fn test_context_rule_matching() {
        let rule = ContextRule::new(
            "task_type == 'code_review' && file_type == 'rust'",
            vec!["patterns.yaml".to_string()],
            ContextInjectionMethod::TemplateVariable,
        );

        // Should match
        assert!(rule.matches(Some("code_review"), Some("rust"), None));

        // Should not match
        assert!(!rule.matches(Some("bug_fix"), Some("rust"), None));
        assert!(!rule.matches(Some("code_review"), Some("python"), None));
        assert!(!rule.matches(None, Some("rust"), None));
    }

    #[test]
    fn test_prompt_pattern_context_rules() {
        let mut pattern = PromptPattern::new(
            "advanced-pattern",
            "Advanced Pattern",
            "Review this {{LANGUAGE}} code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Add context rules
        let rule1 = ContextRule::new(
            "task_type == 'code_review'",
            vec!["patterns.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        )
        .with_priority(1);

        let rule2 = ContextRule::new(
            "task_type == 'code_review' && severity == 'high'",
            vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        )
        .with_priority(2);

        pattern.add_context_rule(rule1);
        pattern.add_context_rule(rule2);

        // Test matching rules
        let matching_rules = pattern.get_matching_rules(Some("code_review"), None, Some("high"));
        assert_eq!(matching_rules.len(), 2);

        // Higher priority rule should come first
        assert_eq!(matching_rules[0].priority, Some(2));
        assert_eq!(matching_rules[1].priority, Some(1));
    }

    #[test]
    fn test_template_variable_substitution() {
        let mut pattern = PromptPattern::new(
            "variable-pattern",
            "Variable Pattern",
            "Review this {{LANGUAGE}} code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Set variables
        let mut variables = HashMap::new();
        variables.insert("LANGUAGE".to_string(), "Rust".to_string());
        pattern.set_variables(variables);

        // Test variable substitution
        let result = pattern.substitute_variables("fn main() { println!(\"Hello, world!\"); }");
        assert_eq!(
            result,
            "Review this Rust code: fn main() { println!(\"Hello, world!\"); }"
        );
    }

    #[test]
    fn test_template_inheritance() {
        let mut pattern = PromptPattern::new(
            "inherited-pattern",
            "Inherited Pattern",
            "{{BASE_TEMPLATE}}\n\nAdditional security checks: {{SECURITY_CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        pattern.set_extends("base-code-review");
        assert!(pattern.has_extends());
        assert_eq!(pattern.extends, Some("base-code-review".to_string()));
    }

    #[test]
    fn test_multi_file_context_support() {
        let mut pattern = PromptPattern::new(
            "multi-file-pattern",
            "Multi-file Pattern",
            "Review this code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        pattern.enable_multi_file_context();
        assert!(pattern.supports_multi_file_context());
    }

    #[test]
    fn test_context_files_collection() {
        let mut pattern = PromptPattern::new(
            "context-files-pattern",
            "Context Files Pattern",
            "Review this code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Add context rules with different files
        let rule1 = ContextRule::new(
            "task_type == 'code_review'",
            vec!["patterns.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        );

        let rule2 = ContextRule::new(
            "task_type == 'code_review'",
            vec!["knowledge.yaml".to_string(), "patterns.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        );

        pattern.add_context_rule(rule1);
        pattern.add_context_rule(rule2);

        // Get context files (should deduplicate)
        let context_files = pattern.get_context_files(Some("code_review"), None, None);
        assert_eq!(context_files.len(), 2);
        assert!(context_files.contains(&"patterns.yaml".to_string()));
        assert!(context_files.contains(&"knowledge.yaml".to_string()));
    }

    #[test]
    fn test_usage_tracking() {
        let mut pattern = PromptPattern::new(
            "usage-pattern",
            "Usage Pattern",
            "Review this code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Record some usage
        pattern.record_usage(true, Some("Great for code reviews".to_string()));
        pattern.record_usage(true, Some("Very helpful".to_string()));
        pattern.record_usage(false, Some("Could be more specific".to_string()));

        assert_eq!(pattern.total_uses(), 3);
        assert_eq!(pattern.successful_uses(), 2);
        assert!((pattern.success_rate() - 0.6666666666666666).abs() < f64::EPSILON);
    }

    #[test]
    fn test_variable_getter() {
        let mut pattern = PromptPattern::new(
            "variable-getter-pattern",
            "Variable Getter Pattern",
            "Review this {{LANGUAGE}} code: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        let mut variables = HashMap::new();
        variables.insert("LANGUAGE".to_string(), "Rust".to_string());
        pattern.set_variables(variables);

        assert_eq!(pattern.get_variable("LANGUAGE"), Some(&"Rust".to_string()));
        assert_eq!(pattern.get_variable("NONEXISTENT"), None);
    }
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
