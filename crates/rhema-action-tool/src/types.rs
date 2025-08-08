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

/// Safety levels for actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of actions that can be performed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Refactor,
    Feature,
    BugFix,
    Documentation,
    Test,
    Configuration,
    Dependency,
    Security,
    Performance,
    Cleanup,
    Migration,
    Custom(String),
}

/// Action intent describing what should be done
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: String,
    pub action_type: ActionType,
    pub description: String,
    pub scope: Vec<String>,
    pub safety_level: SafetyLevel,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
    // Additional fields to match the schema
    pub context_refs: Option<Vec<serde_json::Value>>,
    pub transformation: serde_json::Value,
    pub safety_checks: serde_json::Value,
    pub approval_workflow: serde_json::Value,
    pub created_by: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<String>,
    pub estimated_effort: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

impl ActionIntent {
    pub fn new(
        id: impl Into<String>,
        action_type: ActionType,
        description: impl Into<String>,
        scope: Vec<String>,
        safety_level: SafetyLevel,
    ) -> Self {
        Self {
            id: id.into(),
            action_type,
            description: description.into(),
            scope,
            safety_level,
            created_at: Utc::now(),
            metadata: serde_json::Value::Null,
            context_refs: None,
            transformation: serde_json::Value::Null,
            safety_checks: serde_json::Value::Null,
            approval_workflow: serde_json::Value::Null,
            created_by: None,
            tags: None,
            priority: None,
            estimated_effort: None,
            dependencies: None,
        }
    }
}
