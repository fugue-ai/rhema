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

pub mod commands;
pub mod error;
pub mod file_ops;
pub mod git;
pub mod git_basic;
pub mod integrations;
pub mod query;
pub mod repo_analysis;
pub mod schema;
pub mod scope;
pub mod context_injection;
pub mod ai_service;
pub mod monitoring;
pub mod performance;
pub mod mcp;
pub mod config;
pub mod agent;
pub mod safety;

pub use error::{RhemaError, RhemaResult};
pub use schema::*;
pub use scope::*;

// Re-export integrations
pub use integrations::{IntegrationStatus};
pub use monitoring::{HealthStatus};

// Re-export advanced Git integration
pub use git::*;

// Re-export Git commands
pub use commands::git::GitSubcommands;

// Re-export performance commands
pub use commands::performance::PerformanceSubcommands;

// Re-export AI service and monitoring
pub use ai_service::*;
pub use monitoring::*;

// Re-export performance monitoring (excluding UsageAnalytics to avoid conflict)
pub use performance::{PerformanceMonitor, SystemMetrics, UxMetrics, PerformanceReporter, PerformanceConfig, SystemPerformanceData, UxData, UsageData, PerformanceReport};

// Re-export MCP daemon
pub use mcp::*;

// Re-export batch operations
pub use commands::batch::BatchSubcommands;

// Re-export configuration commands
pub use commands::config::ConfigSubcommands;

// Re-export agent coordination system
pub use agent::*;
pub use safety::*;

use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;


// Command enums for CLI
#[derive(clap::Subcommand)]
pub enum TodoSubcommands {
    /// Add a new todo
    Add {
        /// Todo title
        #[arg(value_name = "TITLE")]
        title: String,
        
        /// Todo description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,
        
        /// Priority level
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,
        
        /// Assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
        
        /// Due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },
    
    /// List todos
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,
        
        /// Filter by priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,
        
        /// Filter by assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
    },
    
    /// Complete a todo
    Complete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
        
        /// Completion outcome
        #[arg(long, value_name = "OUTCOME")]
        outcome: Option<String>,
    },
    
    /// Update a todo
    Update {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
        
        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,
        
        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,
        
        /// New status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,
        
        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,
        
        /// New assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
        
        /// New due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },
    
    /// Delete a todo
    Delete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(clap::Subcommand)]
pub enum InsightSubcommands {
    /// Record a new insight
    Record {
        /// Insight title
        #[arg(value_name = "TITLE")]
        title: String,
        
        /// Insight content
        #[arg(long, value_name = "CONTENT")]
        content: String,
        
        /// Confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,
        
        /// Category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,
        
        /// Tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },
    
    /// List insights
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,
        
        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,
        
        /// Filter by confidence level (minimum)
        #[arg(long, value_name = "LEVEL")]
        min_confidence: Option<u8>,
    },
    
    /// Update an insight
    Update {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,
        
        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,
        
        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,
        
        /// New confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,
        
        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,
        
        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },
    
    /// Delete an insight
    Delete {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(clap::Subcommand)]
pub enum PatternSubcommands {
    /// Add a new pattern
    Add {
        /// Pattern name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Pattern description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,
        
        /// Pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: String,
        
        /// Usage context
        #[arg(long, value_enum, default_value = "recommended")]
        usage: PatternUsage,
        
        /// Effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,
        
        /// Examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,
        
        /// Anti-patterns to avoid (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },
    
    /// List patterns
    List {
        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,
        
        /// Filter by usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,
        
        /// Filter by effectiveness rating (minimum)
        #[arg(long, value_name = "RATING")]
        min_effectiveness: Option<u8>,
    },
    
    /// Update a pattern
    Update {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
        
        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,
        
        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,
        
        /// New pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,
        
        /// New usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,
        
        /// New effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,
        
        /// New examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,
        
        /// New anti-patterns (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },
    
    /// Delete a pattern
    Delete {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(clap::Subcommand)]
pub enum DecisionSubcommands {
    /// Record a new decision
    Record {
        /// Decision title
        #[arg(value_name = "TITLE")]
        title: String,
        
        /// Decision description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,
        
        /// Decision status
        #[arg(long, value_enum, default_value = "proposed")]
        status: DecisionStatus,
        
        /// Decision context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,
        
        /// Decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,
        
        /// Alternatives considered (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,
        
        /// Rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,
        
        /// Consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },
    
    /// List decisions
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,
        
        /// Filter by decision maker
        #[arg(long, value_name = "MAKER")]
        maker: Option<String>,
    },
    
    /// Update a decision
    Update {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,
        
        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,
        
        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,
        
        /// New status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,
        
        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,
        
        /// New decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,
        
        /// New alternatives (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,
        
        /// New rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,
        
        /// New consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },
    
    /// Delete a decision
    Delete {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

/// Main Rhema context manager
#[derive(Debug, Clone)]
pub struct Rhema {
    repo_root: PathBuf,
}

impl Rhema {
    /// Create a new Rhema instance for the current repository
    pub fn new() -> Result<Self> {
        let repo_root = git_basic::find_repo_root()?;
        Ok(Self { repo_root })
    }

    /// Create a new Rhema instance for a specific repository path
    pub fn new_from_path(repo_root: PathBuf) -> Result<Self> {
        // Verify that the path contains a git repository
        if !repo_root.join(".git").exists() {
            return Err(crate::RhemaError::GitRepoNotFound(
                format!("No Git repository found at {}", repo_root.display())
            ).into());
        }
        Ok(Self { repo_root })
    }

    /// Get the repository root path
    pub fn repo_root(&self) -> &PathBuf {
        &self.repo_root
    }
    
    /// Get the repository root path (alias for repo_root)
    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Discover all scopes in the repository
    pub fn discover_scopes(&self) -> Result<Vec<Scope>> {
        Ok(scope::discover_scopes(&self.repo_root)?)
    }

    /// Get a specific scope by path
    pub fn get_scope(&self, path: &str) -> Result<Scope> {
        Ok(scope::get_scope(&self.repo_root, path)?)
    }
    
    /// Get the path for a specific scope
    pub fn scope_path(&self, scope_name: &str) -> Result<PathBuf> {
        let scope = self.get_scope(scope_name)?;
        Ok(scope.path)
    }

    /// Find scope path (alias for scope_path)
    pub fn find_scope_path(&self, scope_name: &str) -> Result<PathBuf> {
        self.scope_path(scope_name)
    }

    /// Get current scope path
    pub fn get_current_scope_path(&self) -> Result<PathBuf> {
        // For now, return the repo root as the current scope
        // This can be enhanced later to track the current working scope
        Ok(self.repo_root.clone())
    }

    /// Execute a CQL query
    pub fn query(&self, query: &str) -> Result<serde_yaml::Value> {
        Ok(query::execute_query(&self.repo_root, query)?)
    }

    /// Execute a CQL query with statistics
    pub fn query_with_stats(&self, query: &str) -> Result<(serde_yaml::Value, HashMap<String, serde_yaml::Value>)> {
        let result = query::execute_query(&self.repo_root, query)?;
        let stats = query::get_query_stats(&self.repo_root, query)?;
        Ok((result, stats))
    }

    /// Execute a CQL query with full provenance tracking
    pub fn query_with_provenance(&self, query: &str) -> Result<(serde_yaml::Value, query::QueryProvenance)> {
        Ok(query::execute_query_with_provenance(&self.repo_root, query)?)
    }

    /// Search context with regex support
    pub fn search_regex(&self, pattern: &str, file_filter: Option<&str>) -> Result<Vec<query::QueryResult>> {
        Ok(query::search_context_regex(&self.repo_root, pattern, file_filter)?)
    }

    /// Load knowledge for a specific scope
    pub fn load_knowledge(&self, scope_name: &str) -> Result<schema::Knowledge> {
        let scope = self.get_scope(scope_name)?;
        let knowledge_path = scope.path.join("knowledge.yaml");
        if knowledge_path.exists() {
            let content = std::fs::read_to_string(&knowledge_path)?;
            let knowledge: schema::Knowledge = serde_yaml::from_str(&content)?;
            Ok(knowledge)
        } else {
            Ok(schema::Knowledge {
                entries: Vec::new(),
                categories: None,
                custom: HashMap::new(),
            })
        }
    }

    /// Load todos for a specific scope
    pub fn load_todos(&self, scope_name: &str) -> Result<schema::Todos> {
        let scope = self.get_scope(scope_name)?;
        let todos_path = scope.path.join("todos.yaml");
        if todos_path.exists() {
            let content = std::fs::read_to_string(&todos_path)?;
            let todos: schema::Todos = serde_yaml::from_str(&content)?;
            Ok(todos)
        } else {
            Ok(schema::Todos {
                todos: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load decisions for a specific scope
    pub fn load_decisions(&self, scope_name: &str) -> Result<schema::Decisions> {
        let scope = self.get_scope(scope_name)?;
        let decisions_path = scope.path.join("decisions.yaml");
        if decisions_path.exists() {
            let content = std::fs::read_to_string(&decisions_path)?;
            let decisions: schema::Decisions = serde_yaml::from_str(&content)?;
            Ok(decisions)
        } else {
            Ok(schema::Decisions {
                decisions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load patterns for a specific scope
    pub fn load_patterns(&self, scope_name: &str) -> Result<schema::Patterns> {
        let scope = self.get_scope(scope_name)?;
        let patterns_path = scope.path.join("patterns.yaml");
        if patterns_path.exists() {
            let content = std::fs::read_to_string(&patterns_path)?;
            let patterns: schema::Patterns = serde_yaml::from_str(&content)?;
            Ok(patterns)
        } else {
            Ok(schema::Patterns {
                patterns: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load conventions for a specific scope
    pub fn load_conventions(&self, scope_name: &str) -> Result<schema::Conventions> {
        let scope = self.get_scope(scope_name)?;
        let conventions_path = scope.path.join("conventions.yaml");
        if conventions_path.exists() {
            let content = std::fs::read_to_string(&conventions_path)?;
            let conventions: schema::Conventions = serde_yaml::from_str(&content)?;
            Ok(conventions)
        } else {
            Ok(schema::Conventions {
                conventions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load a specific scope by name
    pub fn load_scope(&self, name: &str) -> Result<Scope> {
        self.get_scope(name)
    }

    /// List all scopes in the repository
    pub fn list_scopes(&self) -> Result<Vec<Scope>> {
        self.discover_scopes()
    }
} 