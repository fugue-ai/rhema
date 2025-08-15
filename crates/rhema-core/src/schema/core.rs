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
