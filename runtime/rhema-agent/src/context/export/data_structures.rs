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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context data structure for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextExport {
    /// Export metadata
    pub metadata: ExportMetadata,

    /// Scopes data
    pub scopes: Vec<ScopeExport>,

    /// Protocol information
    pub protocol_info: Option<ProtocolExport>,

    /// Knowledge base
    pub knowledge: Option<KnowledgeExport>,

    /// Todo items
    pub todos: Option<TodosExport>,

    /// Decisions
    pub decisions: Option<DecisionsExport>,

    /// Patterns
    pub patterns: Option<PatternsExport>,

    /// Conventions
    pub conventions: Option<ConventionsExport>,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export timestamp
    pub exported_at: String,

    /// Export format
    pub format: String,

    /// Scope count
    pub scope_count: usize,

    /// Export options
    pub options: HashMap<String, bool>,
}

/// Scope export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeExport {
    /// Scope name
    pub name: String,

    /// Scope type
    pub scope_type: String,

    /// Description
    pub description: Option<String>,

    /// Version
    pub version: String,

    /// Dependencies
    pub dependencies: Option<Vec<String>>,

    /// Protocol info
    pub protocol_info: Option<rhema_core::schema::ProtocolInfo>,
}

/// Protocol export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolExport {
    /// Protocol version
    pub version: String,

    /// Description
    pub description: Option<String>,

    /// Concepts
    pub concepts: Option<Vec<rhema_core::schema::ConceptDefinition>>,

    /// CQL examples
    pub cql_examples: Option<Vec<rhema_core::schema::CqlExample>>,

    /// Patterns
    pub patterns: Option<Vec<rhema_core::schema::PatternDefinition>>,

    /// Integrations
    pub integrations: Option<Vec<rhema_core::schema::IntegrationGuide>>,

    /// Troubleshooting
    pub troubleshooting: Option<Vec<rhema_core::schema::TroubleshootingItem>>,
}

/// Knowledge export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeExport {
    /// Entry count
    pub entry_count: usize,

    /// Categories
    pub categories: Option<Vec<String>>,

    /// Recent entries (summarized)
    pub recent_entries: Option<Vec<KnowledgeEntrySummary>>,
}

/// Knowledge entry summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntrySummary {
    /// Entry ID
    pub id: String,

    /// Title
    pub title: String,

    /// Category
    pub category: Option<String>,

    /// Tags
    pub tags: Option<Vec<String>>,

    /// Confidence
    pub confidence: Option<u8>,
}

/// Todos export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodosExport {
    /// Todo count
    pub todo_count: usize,

    /// By status
    pub by_status: HashMap<String, usize>,

    /// By priority
    pub by_priority: HashMap<String, usize>,

    /// Recent todos
    pub recent_todos: Option<Vec<TodoSummary>>,
}

/// Todo summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoSummary {
    /// Todo ID
    pub id: String,

    /// Title
    pub title: String,

    /// Status
    pub status: String,

    /// Priority
    pub priority: String,

    /// Assigned to
    pub assigned_to: Option<String>,
}

/// Decisions export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionsExport {
    /// Decision count
    pub decision_count: usize,

    /// By status
    pub by_status: HashMap<String, usize>,

    /// Recent decisions
    pub recent_decisions: Option<Vec<DecisionSummary>>,
}

/// Decision summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionSummary {
    /// Decision ID
    pub id: String,

    /// Title
    pub title: String,

    /// Status
    pub status: String,

    /// Decision date
    pub decided_at: String,
}

/// Patterns export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternsExport {
    /// Pattern count
    pub pattern_count: usize,

    /// By type
    pub by_type: HashMap<String, usize>,

    /// Recent patterns
    pub recent_patterns: Option<Vec<PatternSummary>>,
}

/// Pattern summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSummary {
    /// Pattern ID
    pub id: String,

    /// Name
    pub name: String,

    /// Pattern type
    pub pattern_type: String,

    /// Usage
    pub usage: String,
}

/// Conventions export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionsExport {
    /// Convention count
    pub convention_count: usize,

    /// By type
    pub by_type: HashMap<String, usize>,

    /// By enforcement level
    pub by_enforcement: HashMap<String, usize>,

    /// Recent conventions
    pub recent_conventions: Option<Vec<ConventionSummary>>,
}

/// Convention summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionSummary {
    /// Convention ID
    pub id: String,

    /// Name
    pub name: String,

    /// Convention type
    pub convention_type: String,

    /// Enforcement level
    pub enforcement: String,
}
