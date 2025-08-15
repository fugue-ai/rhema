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

/// Bootstrap content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapContent {
    /// Bootstrap metadata
    pub metadata: BootstrapMetadata,

    /// Use case specific content
    pub use_case: UseCaseContent,

    /// Scopes summary
    pub scopes: Vec<ScopeSummary>,

    /// Context summary
    pub context_summary: ContextSummary,

    /// AI agent instructions
    pub ai_instructions: Option<AiInstructions>,

    /// Quick reference
    pub quick_reference: QuickReference,
}

/// Bootstrap metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapMetadata {
    /// Bootstrap version
    pub version: String,

    /// Generation timestamp
    pub generated_at: String,

    /// Use case
    pub use_case: String,

    /// Output format
    pub output_format: String,

    /// Scope count
    pub scope_count: usize,

    /// Optimization level
    pub optimization: String,
}

/// Use case specific content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCaseContent {
    /// Use case name
    pub name: String,

    /// Description
    pub description: String,

    /// Key objectives
    pub objectives: Vec<String>,

    /// Context requirements
    pub context_requirements: Vec<String>,

    /// Success criteria
    pub success_criteria: Vec<String>,
}

/// Scope summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeSummary {
    /// Scope name
    pub name: String,

    /// Scope type
    pub scope_type: String,

    /// Description
    pub description: Option<String>,

    /// Key responsibilities
    pub responsibilities: Vec<String>,

    /// Dependencies
    pub dependencies: Option<Vec<String>>,

    /// Context relevance
    pub context_relevance: String,
}

/// Context summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSummary {
    /// Knowledge entries
    pub knowledge_entries: usize,

    /// Todo items
    pub todo_items: usize,

    /// Decisions
    pub decisions: usize,

    /// Patterns
    pub patterns: usize,

    /// Conventions
    pub conventions: usize,

    /// Key insights
    pub key_insights: Vec<String>,

    /// Context gaps
    pub context_gaps: Vec<String>,
}

/// AI agent instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInstructions {
    /// Context understanding
    pub context_understanding: String,

    /// Key concepts
    pub key_concepts: Vec<String>,

    /// Query patterns
    pub query_patterns: Vec<String>,

    /// Decision making guidelines
    pub decision_guidelines: Vec<String>,

    /// Context limitations
    pub context_limitations: Vec<String>,
}

/// Quick reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReference {
    /// Essential commands
    pub essential_commands: Vec<CommandReference>,

    /// Common queries
    pub common_queries: Vec<QueryReference>,

    /// Context patterns
    pub context_patterns: Vec<PatternReference>,

    /// Troubleshooting
    pub troubleshooting: Vec<TroubleshootingReference>,
}

/// Command reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandReference {
    /// Command
    pub command: String,

    /// Description
    pub description: String,

    /// Use case
    pub use_case: String,
}

/// Query reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReference {
    /// Query
    pub query: String,

    /// Description
    pub description: String,

    /// Expected output
    pub expected_output: String,
}

/// Pattern reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternReference {
    /// Pattern name
    pub name: String,

    /// Description
    pub description: String,

    /// Implementation
    pub implementation: String,
}

/// Troubleshooting reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroubleshootingReference {
    /// Issue
    pub issue: String,

    /// Solution
    pub solution: String,

    /// Prevention
    pub prevention: Option<String>,
}
