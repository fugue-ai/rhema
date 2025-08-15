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

    /// Pattern category (architectural, performance, security, etc.)
    pub category: Option<String>,

    /// Pattern maturity level
    pub maturity: Option<PatternMaturity>,

    /// Implementation complexity (1-10)
    pub complexity: Option<u8>,

    /// Maintenance effort (1-10)
    pub maintenance_effort: Option<u8>,

    /// Performance impact rating (1-10)
    pub performance_impact: Option<u8>,

    /// Security implications
    pub security_implications: Option<Vec<String>>,

    /// Testing requirements
    pub testing_requirements: Option<Vec<String>>,

    /// Documentation requirements
    pub documentation_requirements: Option<Vec<String>>,

    /// Dependencies and prerequisites
    pub dependencies: Option<Vec<String>>,

    /// Applicable contexts or domains
    pub applicable_contexts: Option<Vec<String>>,

    /// Pattern version
    pub version: Option<String>,

    /// Pattern author
    pub author: Option<String>,

    /// Review status
    pub review_status: Option<ReviewStatus>,

    /// Last reviewed date
    pub last_reviewed: Option<DateTime<Utc>>,

    /// Reviewers
    pub reviewers: Option<Vec<String>>,

    /// Usage statistics
    pub usage_stats: Option<PatternUsageStats>,

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

/// Pattern maturity levels
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PatternMaturity {
    Experimental,
    Emerging,
    Established,
    Mature,
    Legacy,
}

/// Pattern review status
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
    NeedsRevision,
}

/// Pattern usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternUsageStats {
    /// Number of times this pattern has been used
    pub usage_count: u64,

    /// Number of successful implementations
    pub success_count: u64,

    /// Success rate percentage
    pub success_rate: f32,

    /// Average implementation time in hours
    pub avg_implementation_time: Option<f32>,

    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,

    /// Most common use cases
    pub common_use_cases: Option<Vec<String>>,

    /// Reported issues or challenges
    pub reported_issues: Option<Vec<String>>,

    /// User satisfaction rating (1-10)
    pub satisfaction_rating: Option<f32>,
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

/// Pattern template for reusable pattern definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTemplate {
    /// Template ID
    pub id: String,

    /// Template name
    pub name: String,

    /// Template description
    pub description: String,

    /// Template category
    pub category: String,

    /// Template variables/placeholders
    pub variables: Vec<TemplateVariable>,

    /// Template content (pattern definition with placeholders)
    pub content: TemplateContent,

    /// Template metadata
    pub metadata: TemplateMetadata,

    /// Template version
    pub version: String,

    /// Template author
    pub author: Option<String>,

    /// Template tags
    pub tags: Option<Vec<String>>,

    /// Template usage count
    pub usage_count: u64,

    /// Template rating
    pub rating: Option<f32>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: Option<DateTime<Utc>>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name
    pub name: String,

    /// Variable description
    pub description: String,

    /// Variable type
    pub var_type: VariableType,

    /// Default value (optional)
    pub default_value: Option<String>,

    /// Required flag
    pub required: bool,

    /// Validation rules
    pub validation: Option<Vec<String>>,

    /// Variable examples
    pub examples: Option<Vec<String>>,
}

/// Variable types
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Enum,
    FilePath,
    Url,
    Email,
    Custom,
}

/// Template content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContent {
    /// Pattern definition template
    pub pattern_definition: String,

    /// Implementation examples
    pub examples: Option<Vec<String>>,

    /// Code snippets
    pub code_snippets: Option<Vec<CodeSnippet>>,

    /// Configuration templates
    pub configurations: Option<Vec<ConfigurationTemplate>>,

    /// Documentation template
    pub documentation: Option<String>,

    /// Test templates
    pub tests: Option<Vec<TestTemplate>>,
}

/// Code snippet template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    /// Snippet name
    pub name: String,

    /// Snippet description
    pub description: String,

    /// Programming language
    pub language: String,

    /// Code content with placeholders
    pub content: String,

    /// Snippet type
    pub snippet_type: SnippetType,
}

/// Snippet types
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum SnippetType {
    Implementation,
    Interface,
    Configuration,
    Test,
    Documentation,
    Example,
}

/// Configuration template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTemplate {
    /// Configuration name
    pub name: String,

    /// Configuration description
    pub description: String,

    /// Configuration format
    pub format: ConfigFormat,

    /// Configuration content with placeholders
    pub content: String,

    /// Target file path
    pub target_path: Option<String>,
}

/// Configuration formats
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Ini,
    Properties,
    Custom,
}

/// Test template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTemplate {
    /// Test name
    pub name: String,

    /// Test description
    pub description: String,

    /// Test framework
    pub framework: String,

    /// Test content with placeholders
    pub content: String,

    /// Test type
    pub test_type: TestType,
}

/// Test types
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum TestType {
    Unit,
    Integration,
    E2e,
    Performance,
    Security,
    Custom,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template maturity level
    pub maturity: TemplateMaturity,

    /// Template complexity (1-10)
    pub complexity: u8,

    /// Estimated implementation time (hours)
    pub estimated_time: Option<f32>,

    /// Prerequisites
    pub prerequisites: Option<Vec<String>>,

    /// Dependencies
    pub dependencies: Option<Vec<String>>,

    /// Supported languages
    pub supported_languages: Option<Vec<String>>,

    /// Supported frameworks
    pub supported_frameworks: Option<Vec<String>>,

    /// License
    pub license: Option<String>,

    /// Repository URL
    pub repository_url: Option<String>,

    /// Documentation URL
    pub documentation_url: Option<String>,
}

/// Template maturity levels
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMaturity {
    Experimental,
    Beta,
    Stable,
    Deprecated,
}

/// Pattern templates collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTemplates {
    /// Templates collection
    pub templates: Vec<PatternTemplate>,

    /// Template categories
    pub categories: Option<Vec<String>>,

    /// Template tags
    pub tags: Option<Vec<String>>,
}
