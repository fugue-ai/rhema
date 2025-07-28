use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use validator::ValidationError;

/// Schema version for compatibility tracking
pub const CURRENT_SCHEMA_VERSION: &str = "1.0.0";

/// Core GACP scope definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GacpScope {
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
fn validate_completion_timestamp(completed_at: &Option<DateTime<Utc>>) -> Result<(), ValidationError> {
    if let Some(completed_at) = completed_at {
        if *completed_at < Utc::now() - chrono::Duration::days(365) {
            return Err(ValidationError::new("completion_too_old"));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_related_knowledge(related_knowledge: &Option<Vec<String>>) -> Result<(), ValidationError> {
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
fn validate_related_patterns(related_patterns: &Option<Vec<String>>) -> Result<(), ValidationError> {
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
    fn validate(&self) -> crate::GacpResult<()>;
    fn validate_schema_version(&self) -> crate::GacpResult<()>;
    fn validate_cross_fields(&self) -> crate::GacpResult<()>;
}

impl Validatable for GacpScope {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        if self.name.is_empty() {
            return Err(crate::GacpError::SchemaValidation("Scope name cannot be empty".to_string()));
        }
        if !NAME_REGEX.is_match(&self.name) {
            return Err(crate::GacpError::SchemaValidation("Scope name must be alphanumeric with hyphens/underscores".to_string()));
        }
        if self.scope_type.is_empty() {
            return Err(crate::GacpError::SchemaValidation("Scope type cannot be empty".to_string()));
        }
        if self.version.is_empty() {
            return Err(crate::GacpError::SchemaValidation("Version cannot be empty".to_string()));
        }
        if !VERSION_REGEX.is_match(&self.version) {
            return Err(crate::GacpError::SchemaValidation("Version must follow semantic versioning".to_string()));
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        if let Some(schema_version) = &self.schema_version {
            if !VERSION_REGEX.is_match(schema_version) {
                return Err(crate::GacpError::SchemaValidation(
                    "Invalid schema version format".to_string()
                ));
            }
            
            // Check for major version compatibility
            let current_major = CURRENT_SCHEMA_VERSION.split('.').next().unwrap_or("1");
            let schema_major = schema_version.split('.').next().unwrap_or("1");
            
            if current_major != schema_major {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Incompatible schema version: {} (current: {})", schema_version, CURRENT_SCHEMA_VERSION)
                ));
            }
        }
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate that dependencies don't reference the same scope
        if let Some(dependencies) = &self.dependencies {
            let mut paths = std::collections::HashSet::new();
            for dep in dependencies {
                if !paths.insert(&dep.path) {
                    return Err(crate::GacpError::SchemaValidation(
                        format!("Duplicate dependency path: {}", dep.path)
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Knowledge {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        if self.entries.is_empty() {
            return Err(crate::GacpError::SchemaValidation("Knowledge must contain at least one entry".to_string()));
        }
        
        for entry in &self.entries {
            if entry.title.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Knowledge entry title cannot be empty".to_string()));
            }
            if entry.content.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Knowledge entry content cannot be empty".to_string()));
            }
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        // Knowledge doesn't have schema version field, so always valid
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for entry in &self.entries {
            if !ids.insert(&entry.id) {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Duplicate knowledge entry ID: {}", entry.id)
                ));
            }
        }

        // Validate that categories exist for entries that reference them
        if let Some(categories) = &self.categories {
            for entry in &self.entries {
                if let Some(category) = &entry.category {
                    if !categories.contains_key(category) {
                        return Err(crate::GacpError::SchemaValidation(
                            format!("Knowledge entry references non-existent category: {}", category)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Todos {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        for todo in &self.todos {
            if todo.title.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Todo title cannot be empty".to_string()));
            }
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for todo in &self.todos {
            if !ids.insert(&todo.id) {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Duplicate todo ID: {}", todo.id)
                ));
            }
        }

        // Validate completion timestamps
        for todo in &self.todos {
            if todo.status == TodoStatus::Completed {
                if todo.completed_at.is_none() {
                    return Err(crate::GacpError::SchemaValidation(
                        format!("Completed todo {} must have completion timestamp", todo.id)
                    ));
                }
            } else if todo.completed_at.is_some() {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Non-completed todo {} cannot have completion timestamp", todo.id)
                ));
            }
        }
        Ok(())
    }
}

impl Validatable for Decisions {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        for decision in &self.decisions {
            if decision.title.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Decision title cannot be empty".to_string()));
            }
            if decision.description.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Decision description cannot be empty".to_string()));
            }
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for decision in &self.decisions {
            if !ids.insert(&decision.id) {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Duplicate decision ID: {}", decision.id)
                ));
            }
        }

        // Validate review dates
        for decision in &self.decisions {
            if let Some(review_date) = decision.review_date {
                if review_date <= decision.decided_at {
                    return Err(crate::GacpError::SchemaValidation(
                        format!("Decision {} review date must be after decision date", decision.id)
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Patterns {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        for pattern in &self.patterns {
            if pattern.name.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Pattern name cannot be empty".to_string()));
            }
            if pattern.description.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Pattern description cannot be empty".to_string()));
            }
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for pattern in &self.patterns {
            if !ids.insert(&pattern.id) {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Duplicate pattern ID: {}", pattern.id)
                ));
            }
        }

        // Validate related patterns exist
        for pattern in &self.patterns {
            if let Some(related) = &pattern.related_patterns {
                for related_id in related {
                    if !ids.contains(related_id) {
                        return Err(crate::GacpError::SchemaValidation(
                            format!("Pattern {} references non-existent pattern: {}", pattern.id, related_id)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Validatable for Conventions {
    fn validate(&self) -> crate::GacpResult<()> {
        // Basic field validation
        for convention in &self.conventions {
            if convention.name.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Convention name cannot be empty".to_string()));
            }
            if convention.description.is_empty() {
                return Err(crate::GacpError::SchemaValidation("Convention description cannot be empty".to_string()));
            }
        }
        
        self.validate_schema_version()?;
        self.validate_cross_fields()?;
        Ok(())
    }

    fn validate_schema_version(&self) -> crate::GacpResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::GacpResult<()> {
        // Validate unique IDs
        let mut ids = std::collections::HashSet::new();
        for convention in &self.conventions {
            if !ids.insert(&convention.id) {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Duplicate convention ID: {}", convention.id)
                ));
            }
        }
        Ok(())
    }
}

/// Schema migration utilities
pub trait SchemaMigratable {
    fn migrate_to_latest(&mut self) -> crate::GacpResult<()>;
    fn get_schema_version(&self) -> Option<String>;
}

impl SchemaMigratable for GacpScope {
    fn migrate_to_latest(&mut self) -> crate::GacpResult<()> {
        let current_version = self.get_schema_version().unwrap_or_else(|| "0.1.0".to_string());
        
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
                    return Err(crate::GacpError::SchemaValidation(
                        format!("Cannot migrate from version {} to {}", current_version, CURRENT_SCHEMA_VERSION)
                    ));
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

impl JsonSchema for GacpScope {
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