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

use clap::Subcommand;
use rhema_core::{DecisionStatus, PatternUsage, Priority, TodoStatus};
use std::path::PathBuf;

// Command enums for CLI
#[derive(Subcommand)]
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

#[derive(Subcommand)]
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

#[derive(Subcommand)]
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

#[derive(Subcommand)]
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

#[derive(Subcommand)]
pub enum ContextRulesSubcommands {
    /// Add a new context injection rule
    Add {
        /// Rule name
        #[arg(value_name = "NAME")]
        name: String,

        /// Rule description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Rule pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: String,

        /// Context to inject
        #[arg(long, value_name = "CONTEXT")]
        context: String,

        /// Priority level
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,
    },

    /// List context injection rules
    List {
        /// Filter by pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,
    },

    /// Update a context injection rule
    Update {
        /// Rule ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,

        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,
    },

    /// Delete a context injection rule
    Delete {
        /// Rule ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum WorkflowSubcommands {
    /// Add a new workflow
    Add {
        /// Workflow name
        #[arg(value_name = "NAME")]
        name: String,

        /// Workflow description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Workflow steps (comma-separated)
        #[arg(long, value_name = "STEPS")]
        steps: String,

        /// Workflow triggers (comma-separated)
        #[arg(long, value_name = "TRIGGERS")]
        triggers: Option<String>,
    },

    /// List workflows
    List {
        /// Filter by trigger
        #[arg(long, value_name = "TRIGGER")]
        trigger: Option<String>,
    },

    /// Update a workflow
    Update {
        /// Workflow ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New steps (comma-separated)
        #[arg(long, value_name = "STEPS")]
        steps: Option<String>,

        /// New triggers (comma-separated)
        #[arg(long, value_name = "TRIGGERS")]
        triggers: Option<String>,
    },

    /// Delete a workflow
    Delete {
        /// Workflow ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TemplateSubcommands {
    /// Add a new template
    Add {
        /// Template name
        #[arg(value_name = "NAME")]
        name: String,

        /// Template description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Template content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Template category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Template tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// List templates
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,
    },

    /// Update a template
    Update {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// Delete a template
    Delete {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PromptSubcommands {
    /// Add a new prompt
    Add {
        /// Prompt name
        #[arg(value_name = "NAME")]
        name: String,

        /// Prompt description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Prompt content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Prompt category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Prompt tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// List prompts
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,
    },

    /// Update a prompt
    Update {
        /// Prompt ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// Delete a prompt
    Delete {
        /// Prompt ID
        #[arg(value_name = "ID")]
        id: String,
    },
}
