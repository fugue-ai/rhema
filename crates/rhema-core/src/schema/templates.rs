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
