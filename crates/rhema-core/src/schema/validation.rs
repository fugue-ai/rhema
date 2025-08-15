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

use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

use super::core::*;
use super::knowledge::*;
use super::lock::*;
use super::prompts::*;
use super::templates::*;

/// Schema validation trait with enhanced validation
pub trait Validatable {
    fn validate(&self) -> crate::RhemaResult<()>;
    fn validate_schema_version(&self) -> crate::RhemaResult<()>;
    fn validate_cross_fields(&self) -> crate::RhemaResult<()>;
}

/// Schema migration trait for version compatibility
pub trait SchemaMigratable {
    fn migrate_to_version(&mut self, target_version: &str) -> crate::RhemaResult<()>;
    fn get_current_version(&self) -> &str;
    fn is_compatible_with(&self, version: &str) -> bool;
}

/// JSON Schema generation trait
pub trait JsonSchema {
    fn generate_schema() -> serde_json::Value;
    fn validate_against_schema(&self, schema: &serde_json::Value) -> crate::RhemaResult<()>;
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
            for dep in deps {
                dep.validate()?;
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
            if schema_version != super::core::CURRENT_SCHEMA_VERSION {
                return Err(crate::RhemaError::ValidationError(format!(
                    "Schema version mismatch. Expected {}, got {}",
                    super::core::CURRENT_SCHEMA_VERSION,
                    schema_version
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

impl Validatable for ScopeDependency {
    fn validate(&self) -> crate::RhemaResult<()> {
        if self.path.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency path cannot be empty".to_string(),
            ));
        }

        if self.dependency_type.trim().is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency type cannot be empty".to_string(),
            ));
        }

        // Validate dependency type
        let valid_types = ["required", "optional", "peer"];
        if !valid_types.contains(&self.dependency_type.as_str()) {
            return Err(crate::RhemaError::ValidationError(format!(
                "Invalid dependency type: {}. Valid types are: {}",
                self.dependency_type,
                valid_types.join(", ")
            )));
        }

        // Validate version constraint if present
        if let Some(version) = &self.version {
            if !version.chars().all(|c| {
                c.is_alphanumeric()
                    || c == '.'
                    || c == '-'
                    || c == '<'
                    || c == '>'
                    || c == '='
                    || c == '~'
                    || c == '^'
                    || c == '!'
            }) {
                return Err(crate::RhemaError::ValidationError(
                    "Invalid version constraint format".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        // ScopeDependency doesn't have schema version
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate path security (no directory traversal)
        if self.path.contains("..") || self.path.contains("~") {
            return Err(crate::RhemaError::ValidationError(
                "Dependency path contains security issues".to_string(),
            ));
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

impl Validatable for RhemaLock {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Validate lock file version
        if self.lockfile_version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Lock file version cannot be empty".to_string(),
            ));
        }

        // Validate generated_by
        if self.generated_by.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Generated by field cannot be empty".to_string(),
            ));
        }

        // Validate checksum
        if self.checksum.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Checksum cannot be empty".to_string(),
            ));
        }

        // Validate scopes
        for (path, scope) in &self.scopes {
            if path.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Scope path cannot be empty".to_string(),
                ));
            }
            scope.validate()?;
        }

        // Validate metadata
        self.metadata.validate()?;

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that checksum matches calculated checksum
        let calculated_checksum = self.calculate_checksum();
        if self.checksum != calculated_checksum {
            return Err(crate::RhemaError::ValidationError(
                "Lock file checksum mismatch".to_string(),
            ));
        }

        Ok(())
    }
}

impl Validatable for LockedScope {
    fn validate(&self) -> crate::RhemaResult<()> {
        if self.version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope version cannot be empty".to_string(),
            ));
        }

        if self.path.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Scope path cannot be empty".to_string(),
            ));
        }

        // Validate dependencies
        for (name, dependency) in &self.dependencies {
            if name.is_empty() {
                return Err(crate::RhemaError::ValidationError(
                    "Dependency name cannot be empty".to_string(),
                ));
            }
            dependency.validate()?;
        }

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        Ok(())
    }
}

impl Validatable for LockedDependency {
    fn validate(&self) -> crate::RhemaResult<()> {
        if self.version.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency version cannot be empty".to_string(),
            ));
        }

        if self.path.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency path cannot be empty".to_string(),
            ));
        }

        if self.checksum.is_empty() {
            return Err(crate::RhemaError::ValidationError(
                "Dependency checksum cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        // Validate that checksum matches calculated checksum
        let calculated_checksum = self.calculate_checksum();
        if self.checksum != calculated_checksum {
            return Err(crate::RhemaError::ValidationError(
                "Dependency checksum mismatch".to_string(),
            ));
        }

        Ok(())
    }
}

impl Validatable for LockMetadata {
    fn validate(&self) -> crate::RhemaResult<()> {
        // Validate that totals match actual counts
        let actual_scopes = self.total_scopes;
        let actual_dependencies = self.total_dependencies;

        // Note: We can't validate against actual counts here since we don't have access to the lock file
        // This validation would need to be done at a higher level

        Ok(())
    }

    fn validate_schema_version(&self) -> crate::RhemaResult<()> {
        Ok(())
    }

    fn validate_cross_fields(&self) -> crate::RhemaResult<()> {
        Ok(())
    }
}

// Implementation stubs for SchemaMigratable and JsonSchema traits
impl SchemaMigratable for RhemaScope {
    fn migrate_to_version(&mut self, target_version: &str) -> crate::RhemaResult<()> {
        // Implementation would handle schema migrations
        Ok(())
    }

    fn get_current_version(&self) -> &str {
        self.schema_version
            .as_deref()
            .unwrap_or(super::core::CURRENT_SCHEMA_VERSION)
    }

    fn is_compatible_with(&self, version: &str) -> bool {
        self.get_current_version() == version
    }
}

impl JsonSchema for RhemaScope {
    fn generate_schema() -> serde_json::Value {
        // Implementation would generate JSON Schema
        serde_json::json!({})
    }

    fn validate_against_schema(&self, schema: &serde_json::Value) -> crate::RhemaResult<()> {
        // Implementation would validate against JSON Schema
        Ok(())
    }
}
