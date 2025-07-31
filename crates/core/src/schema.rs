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
    // TODO: Advanced features for future versions:
    // - context_rules: Vec<ContextRule> - Conditional context injection based on task type
    // - variables: HashMap<String, String> - Template variables beyond {{CONTEXT}}
    // - multi_file_context: bool - Support loading multiple context files
    // - context_priority: u8 - Priority for context injection when multiple rules match
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
