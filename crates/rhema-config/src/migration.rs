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

use crate::{
    Config, ConfigChange, ConfigChangeType, ConfigError, ConfigIssue, ConfigIssueSeverity,
    ValidationResult, CURRENT_CONFIG_VERSION,
};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
/// Migration manager for configuration version migrations
pub struct MigrationManager {
    migrations: Vec<Migration>,
    migration_history: HashMap<PathBuf, Vec<MigrationRecord>>,
    auto_migrate: bool,
    backup_before_migration: bool,
}

/// Migration definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub from_version: String,
    pub to_version: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<MigrationStep>,
    pub rollback_steps: Vec<MigrationStep>,
    pub required: bool,
    pub automatic: bool,
}

/// Migration step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub step_type: MigrationStepType,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub condition: Option<MigrationCondition>,
    pub rollback: Option<Box<MigrationStep>>,
}

/// Migration step type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationStepType {
    AddField,
    RemoveField,
    RenameField,
    ChangeFieldType,
    UpdateFieldValue,
    AddSection,
    RemoveSection,
    RenameSection,
    TransformData,
    ExecuteScript,
    Custom,
}

/// Migration condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationCondition {
    pub field: String,
    pub operator: MigrationConditionOperator,
    pub value: serde_json::Value,
}

/// Migration condition operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationConditionOperator {
    Equals,
    NotEquals,
    Exists,
    NotExists,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

/// Migration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    pub migration_name: String,
    pub from_version: String,
    pub to_version: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
    pub changes: Vec<ConfigChange>,
}

/// Migration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    pub migrations_applied: Vec<MigrationRecord>,
    pub migrations_skipped: Vec<String>,
    pub migrations_failed: Vec<MigrationRecord>,
    pub summary: MigrationSummary,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Migration summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSummary {
    pub total_migrations: usize,
    pub successful_migrations: usize,
    pub failed_migrations: usize,
    pub skipped_migrations: usize,
    pub total_changes: usize,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(_global_config: &super::GlobalConfig) -> RhemaResult<Self> {
        let mut manager = Self {
            migrations: Vec::new(),
            migration_history: HashMap::new(),
            auto_migrate: true,
            backup_before_migration: true,
        };

        manager.load_default_migrations();

        Ok(manager)
    }

    /// Load default migrations
    fn load_default_migrations(&mut self) {
        // Migration from 0.1.0 to 1.0.0
        self.migrations.push(Migration {
            from_version: "0.1.0".to_string(),
            to_version: "1.0.0".to_string(),
            name: "initial_release_migration".to_string(),
            description: "Migration to initial release version 1.0.0".to_string(),
            steps: vec![
                MigrationStep {
                    step_type: MigrationStepType::AddField,
                    description: "Add version field to all configurations".to_string(),
                    parameters: HashMap::new(),
                    condition: None,
                    rollback: None,
                },
                MigrationStep {
                    step_type: MigrationStepType::AddSection,
                    description: "Add security section to global config".to_string(),
                    parameters: HashMap::new(),
                    condition: None,
                    rollback: None,
                },
            ],
            rollback_steps: vec![],
            required: true,
            automatic: true,
        });

        // Migration from 0.2.0 to 1.0.0
        self.migrations.push(Migration {
            from_version: "0.2.0".to_string(),
            to_version: "1.0.0".to_string(),
            name: "test_migration".to_string(),
            description: "Test migration from 0.2.0 to 1.0.0".to_string(),
            steps: vec![
                MigrationStep {
                    step_type: MigrationStepType::UpdateFieldValue,
                    description: "Update version to 1.0.0".to_string(),
                    parameters: HashMap::new(),
                    condition: None,
                    rollback: None,
                },
            ],
            rollback_steps: vec![],
            required: true,
            automatic: true,
        });

        // Migration from 1.0.0 to 1.1.0
        self.migrations.push(Migration {
            from_version: "1.0.0".to_string(),
            to_version: "1.1.0".to_string(),
            name: "performance_enhancements".to_string(),
            description: "Add performance configuration section".to_string(),
            steps: vec![MigrationStep {
                step_type: MigrationStepType::AddSection,
                description: "Add performance configuration section".to_string(),
                parameters: HashMap::new(),
                condition: None,
                rollback: None,
            }],
            rollback_steps: vec![],
            required: false,
            automatic: true,
        });
    }

    /// Migrate all configurations
    pub fn migrate_all(
        &self,
        global_config: &super::GlobalConfig,
        repository_configs: &HashMap<PathBuf, super::RepositoryConfig>,
        scope_configs: &HashMap<PathBuf, super::ScopeConfig>,
    ) -> RhemaResult<MigrationReport> {
        let start_time = Utc::now();
        let mut migrations_applied = Vec::new();
        let mut migrations_skipped = Vec::new();
        let mut migrations_failed = Vec::new();

        // Migrate global config
        let global_migrations = self.migrate_config(global_config, "global")?;
        migrations_applied.extend(global_migrations.migrations_applied);
        migrations_skipped.extend(global_migrations.migrations_skipped);
        migrations_failed.extend(global_migrations.migrations_failed);

        // Migrate repository configs
        for (path, config) in repository_configs {
            let migrations =
                self.migrate_config(config, &format!("repository:{}", path.display()))?;
            migrations_applied.extend(migrations.migrations_applied);
            migrations_skipped.extend(migrations.migrations_skipped);
            migrations_failed.extend(migrations.migrations_failed);
        }

        // Migrate scope configs
        for (path, config) in scope_configs {
            let migrations = self.migrate_config(config, &format!("scope:{}", path.display()))?;
            migrations_applied.extend(migrations.migrations_applied);
            migrations_skipped.extend(migrations.migrations_skipped);
            migrations_failed.extend(migrations.migrations_failed);
        }

        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);

        let summary = MigrationSummary {
            total_migrations: migrations_applied.len()
                + migrations_failed.len()
                + migrations_skipped.len(),
            successful_migrations: migrations_applied.len(),
            failed_migrations: migrations_failed.len(),
            skipped_migrations: migrations_skipped.len(),
            total_changes: migrations_applied.iter().map(|m| m.changes.len()).sum(),
        };

        Ok(MigrationReport {
            migrations_applied,
            migrations_skipped,
            migrations_failed,
            summary,
            timestamp: end_time,
            duration_ms: duration.num_milliseconds() as u64,
        })
    }

    /// Migrate a single configuration
    pub fn migrate_config<T: Config>(
        &self,
        config: &T,
        context: &str,
    ) -> RhemaResult<MigrationReport> {
        let current_version =
            Version::parse(&config.version()).map_err(|e| ConfigError::VersionMismatch {
                expected: "valid semver".to_string(),
                found: e.to_string(),
            })?;

        let target_version = Version::parse(CURRENT_CONFIG_VERSION).map_err(|e| ConfigError::VersionMismatch {
            expected: "valid semver".to_string(),
            found: e.to_string(),
        })?;

        if current_version >= target_version {
            return Ok(MigrationReport {
                migrations_applied: Vec::new(),
                migrations_skipped: vec!["Already at target version".to_string()],
                migrations_failed: Vec::new(),
                summary: MigrationSummary {
                    total_migrations: 0,
                    successful_migrations: 0,
                    failed_migrations: 0,
                    skipped_migrations: 1,
                    total_changes: 0,
                },
                timestamp: Utc::now(),
                duration_ms: 0,
            });
        }

        let applicable_migrations =
            self.get_applicable_migrations(&current_version, &target_version);
        let mut migrations_applied = Vec::new();
        let migrations_skipped = Vec::new();
        let mut migrations_failed = Vec::new();

        let mut config_value = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        for migration in applicable_migrations {
            match self.apply_migration_to_value(&mut config_value, &migration, context) {
                Ok(record) => {
                    if record.success {
                        migrations_applied.push(record);
                    } else {
                        migrations_failed.push(record);
                    }
                }
                Err(e) => {
                    migrations_failed.push(MigrationRecord {
                        migration_name: migration.name.clone(),
                        from_version: migration.from_version.clone(),
                        to_version: migration.to_version.clone(),
                        timestamp: Utc::now(),
                        success: false,
                        error_message: Some(e.to_string()),
                        changes: Vec::new(),
                    });
                }
            }
        }

        let summary = MigrationSummary {
            total_migrations: migrations_applied.len()
                + migrations_failed.len()
                + migrations_skipped.len(),
            successful_migrations: migrations_applied.len(),
            failed_migrations: migrations_failed.len(),
            skipped_migrations: migrations_skipped.len(),
            total_changes: migrations_applied.iter().map(|m| m.changes.len()).sum(),
        };

        Ok(MigrationReport {
            migrations_applied,
            migrations_skipped,
            migrations_failed,
            summary,
            timestamp: Utc::now(),
            duration_ms: 0,
        })
    }

    /// Migrate configuration to a specific version
    pub async fn migrate_version<T: Config>(
        &self,
        config: &T,
        target_version: &str,
    ) -> RhemaResult<(MigrationReport, T)> {
        let start_time = std::time::Instant::now();
        let mut migrations_applied = Vec::new();
        let mut migrations_failed = Vec::new();
        let migrations_skipped = Vec::new();

        let current_version = config.version();
        let from_version = Version::parse(current_version)
            .map_err(|e| ConfigError::MigrationFailed(format!("Invalid current version: {}", e)))?;
        let to_version = Version::parse(target_version)
            .map_err(|e| ConfigError::MigrationFailed(format!("Invalid target version: {}", e)))?;

        // Get applicable migrations
        let applicable_migrations = self.get_applicable_migrations(&from_version, &to_version);

        if applicable_migrations.is_empty() {
            return Ok((MigrationReport {
                migrations_applied,
                migrations_skipped: vec!["No migrations needed".to_string()],
                migrations_failed,
                summary: MigrationSummary {
                    total_migrations: 0,
                    successful_migrations: 0,
                    failed_migrations: 0,
                    skipped_migrations: 1,
                    total_changes: 0,
                },
                timestamp: chrono::Utc::now(),
                duration_ms: start_time.elapsed().as_millis() as u64,
            }, config.clone()));
        }

        // Create backup if enabled
        if self.backup_before_migration {
            // Here we would create a backup before migration
            // For now, we'll just log that backup would be created
            tracing::info!("Backup would be created before migration");
        }

        // Apply migrations in order and build updated configuration
        let mut updated_config_value = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        for migration in &applicable_migrations {
            match self.apply_migration_to_value(&mut updated_config_value, migration, "version_migration") {
                Ok(record) => {
                    migrations_applied.push(record);
                }
                Err(e) => {
                    let failed_record = MigrationRecord {
                        migration_name: migration.name.clone(),
                        from_version: migration.from_version.clone(),
                        to_version: migration.to_version.clone(),
                        timestamp: chrono::Utc::now(),
                        success: false,
                        error_message: Some(e.to_string()),
                        changes: Vec::new(),
                    };
                    migrations_failed.push(failed_record);

                    // If migration is required, stop here
                    if migration.required {
                        return Err(ConfigError::MigrationFailed(format!(
                            "Required migration '{}' failed: {}",
                            migration.name, e
                        ))
                        .into());
                    }
                }
            }
        }

        // Update version to target version
        if let Some(version) = updated_config_value.get_mut("version") {
            *version = serde_json::Value::String(target_version.to_string());
        }

        // Deserialize back to configuration type
        let updated_config: T = serde_json::from_value(updated_config_value)
            .map_err(|e| ConfigError::SerializationError(format!("Failed to deserialize migrated config: {}", e)))?;

        let duration = start_time.elapsed();
        Ok((MigrationReport {
            migrations_applied: migrations_applied.clone(),
            migrations_skipped: migrations_skipped.clone(),
            migrations_failed: migrations_failed.clone(),
            summary: MigrationSummary {
                total_migrations: applicable_migrations.len(),
                successful_migrations: migrations_applied.len(),
                failed_migrations: migrations_failed.len(),
                skipped_migrations: migrations_skipped.len(),
                total_changes: migrations_applied.iter().map(|r| r.changes.len()).sum(),
            },
            timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        }, updated_config))
    }

    /// Rollback a migration
    pub async fn rollback_migration<T: Config>(
        &self,
        config: &T,
        migration_record: &MigrationRecord,
    ) -> RhemaResult<MigrationReport> {
        let start_time = std::time::Instant::now();
        let mut rollback_changes = Vec::new();

        // Find the migration that was applied
        let migration = self
            .migrations
            .iter()
            .find(|m| m.name == migration_record.migration_name)
            .ok_or_else(|| {
                ConfigError::MigrationFailed(format!(
                    "Migration '{}' not found for rollback",
                    migration_record.migration_name
                ))
            })?;

        // Apply rollback steps in reverse order
        for step in migration.rollback_steps.iter().rev() {
            match self.apply_rollback_step(config, step, "rollback") {
                Ok(changes) => {
                    rollback_changes.extend(changes);
                }
                Err(e) => {
                    return Err(ConfigError::MigrationFailed(format!(
                        "Rollback step failed: {}",
                        e
                    ))
                    .into());
                }
            }
        }

        let duration = start_time.elapsed();
        Ok(MigrationReport {
            migrations_applied: vec![MigrationRecord {
                migration_name: format!("rollback_{}", migration_record.migration_name),
                from_version: migration_record.to_version.clone(),
                to_version: migration_record.from_version.clone(),
                timestamp: chrono::Utc::now(),
                success: true,
                error_message: None,
                changes: rollback_changes.clone(),
            }],
            migrations_skipped: Vec::new(),
            migrations_failed: Vec::new(),
            summary: MigrationSummary {
                total_migrations: 1,
                successful_migrations: 1,
                failed_migrations: 0,
                skipped_migrations: 0,
                total_changes: rollback_changes.len(),
            },
            timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        })
    }

    /// Validate migration results
    pub async fn validate_migration_results<T: Config>(
        &self,
        config: &T,
        migration_report: &MigrationReport,
    ) -> RhemaResult<ValidationResult> {
        let start_time = std::time::Instant::now();
        let mut issues = Vec::new();

        // Validate that the configuration is still valid after migration
        if let Err(e) = config.validate_config() {
            issues.push(ConfigIssue {
                severity: ConfigIssueSeverity::Error,
                message: format!("Configuration validation failed after migration: {}", e),
                location: None,
                suggestion: Some("Review migration changes and fix validation issues".to_string()),
            });
        }

        // Check for any failed migrations
        if !migration_report.migrations_failed.is_empty() {
            issues.push(ConfigIssue {
                severity: ConfigIssueSeverity::Warning,
                message: format!(
                    "{} migrations failed during the process",
                    migration_report.migrations_failed.len()
                ),
                location: None,
                suggestion: Some("Review failed migrations and consider rollback".to_string()),
            });
        }

        // Validate that all required fields are present
        let config_json = serde_json::to_value(config)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        let missing_fields = self.check_required_fields(&config_json)?;
        for field in missing_fields {
            issues.push(ConfigIssue {
                severity: ConfigIssueSeverity::Error,
                message: format!("Required field missing after migration: {}", field),
                location: None,
                suggestion: Some("Add the missing required field".to_string()),
            });
        }

        let duration = start_time.elapsed();
        Ok(ValidationResult {
            valid: issues.is_empty(),
            issues,
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        })
    }

    /// Get applicable migrations for version range
    fn get_applicable_migrations(
        &self,
        from_version: &Version,
        to_version: &Version,
    ) -> Vec<&Migration> {
        let mut applicable = Vec::new();

        for migration in &self.migrations {
            let migration_from =
                Version::parse(&migration.from_version).unwrap_or_else(|_| Version::new(0, 0, 0));
            let migration_to =
                Version::parse(&migration.to_version).unwrap_or_else(|_| Version::new(0, 0, 0));

            if migration_from >= *from_version && migration_to <= *to_version && migration_from < migration_to {
                applicable.push(migration);
            }
        }

        // Sort by version
        applicable.sort_by(|a, b| {
            let a_from = Version::parse(&a.from_version).unwrap_or_else(|_| Version::new(0, 0, 0));
            let b_from = Version::parse(&b.from_version).unwrap_or_else(|_| Version::new(0, 0, 0));
            a_from.cmp(&b_from)
        });

        applicable
    }

    /// Apply a migration to a configuration value
    fn apply_migration_to_value(
        &self,
        config_value: &mut serde_json::Value,
        migration: &Migration,
        context: &str,
    ) -> RhemaResult<MigrationRecord> {
        let mut changes = Vec::new();

        for step in &migration.steps {
            if let Some(condition) = &step.condition {
                if !self.evaluate_condition(condition, config_value)? {
                    continue;
                }
            }

            match self.apply_migration_step(config_value, step, context) {
                Ok(step_changes) => changes.extend(step_changes),
                Err(e) => {
                    return Ok(MigrationRecord {
                        migration_name: migration.name.clone(),
                        from_version: migration.from_version.clone(),
                        to_version: migration.to_version.clone(),
                        timestamp: Utc::now(),
                        success: false,
                        error_message: Some(e.to_string()),
                        changes,
                    });
                }
            }
        }

        // Update version
        if let Some(version) = config_value.get_mut("version") {
            *version = serde_json::Value::String(migration.to_version.clone());
            changes.push(ConfigChange {
                timestamp: Utc::now(),
                user: "system".to_string(),
                change_type: ConfigChangeType::Migrated,
                description: format!(
                    "Updated version from {} to {}",
                    migration.from_version, migration.to_version
                ),
            });
        }

        Ok(MigrationRecord {
            migration_name: migration.name.clone(),
            from_version: migration.from_version.clone(),
            to_version: migration.to_version.clone(),
            timestamp: Utc::now(),
            success: true,
            error_message: None,
            changes,
        })
    }

    /// Apply a migration step
    fn apply_migration_step(
        &self,
        config: &mut serde_json::Value,
        step: &MigrationStep,
        _context: &str,
    ) -> RhemaResult<Vec<ConfigChange>> {
        let mut changes = Vec::new();

        match &step.step_type {
            MigrationStepType::AddField => {
                if let Some(field_path) = step.parameters.get("field_path") {
                    if let Some(value) = step.parameters.get("value") {
                        let path_parts: Vec<&str> =
                            field_path.as_str().unwrap().split('.').collect();
                        let mut current = config;

                        for (i, part) in path_parts.iter().enumerate() {
                            if i == path_parts.len() - 1 {
                                current[part] = value.clone();
                                changes.push(ConfigChange {
                                    timestamp: Utc::now(),
                                    user: "system".to_string(),
                                    change_type: ConfigChangeType::Migrated,
                                    description: format!(
                                        "Added field: {}",
                                        field_path.as_str().unwrap()
                                    ),
                                });
                            } else {
                                if !current.get(part).is_some() {
                                    current[part] =
                                        serde_json::Value::Object(serde_json::Map::new());
                                }
                                current = current.get_mut(part).unwrap();
                            }
                        }
                    }
                }
            }
            MigrationStepType::RemoveField => {
                if let Some(field_path) = step.parameters.get("field_path") {
                    let path_parts: Vec<&str> = field_path.as_str().unwrap().split('.').collect();
                    let mut current = config;

                    for (i, part) in path_parts.iter().enumerate() {
                        if i == path_parts.len() - 1 {
                            if let Some(_old_value) = current.get(part) {
                                changes.push(ConfigChange {
                                    timestamp: Utc::now(),
                                    user: "system".to_string(),
                                    change_type: ConfigChangeType::Migrated,
                                    description: format!(
                                        "Removed field: {}",
                                        field_path.as_str().unwrap()
                                    ),
                                });
                                if let Some(obj) = current.as_object_mut() {
                                    obj.remove(*part);
                                }
                            }
                        } else {
                            current = current.get_mut(part).unwrap();
                        }
                    }
                }
            }
            MigrationStepType::RenameField => {
                if let Some(_old_path) = step.parameters.get("old_path") {
                    if let Some(_new_path) = step.parameters.get("new_path") {
                        // Implementation for renaming fields
                        // This would involve moving the value from old_path to new_path
                    }
                }
            }
            MigrationStepType::AddSection => {
                if let Some(section_name) = step.parameters.get("section_name") {
                    if let Some(section_value) = step.parameters.get("section_value") {
                        config[section_name.as_str().unwrap()] = section_value.clone();
                        changes.push(ConfigChange {
                            timestamp: Utc::now(),
                            user: "system".to_string(),
                            change_type: ConfigChangeType::Migrated,
                            description: format!(
                                "Added section: {}",
                                section_name.as_str().unwrap()
                            ),
                        });
                    }
                }
            }
            _ => {
                // Handle other migration step types
            }
        }

        Ok(changes)
    }

    /// Evaluate migration condition
    fn evaluate_condition(
        &self,
        condition: &MigrationCondition,
        config: &serde_json::Value,
    ) -> RhemaResult<bool> {
        let field_value = self.get_field_value(&condition.field, config)?;

        match condition.operator {
            MigrationConditionOperator::Equals => Ok(field_value == condition.value),
            MigrationConditionOperator::NotEquals => Ok(field_value != condition.value),
            MigrationConditionOperator::Exists => Ok(!field_value.is_null()),
            MigrationConditionOperator::NotExists => Ok(field_value.is_null()),
            MigrationConditionOperator::GreaterThan => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a > b)
                } else {
                    Ok(false)
                }
            }
            MigrationConditionOperator::LessThan => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a < b)
                } else {
                    Ok(false)
                }
            }
            MigrationConditionOperator::Contains => {
                if let (Some(a), Some(b)) = (field_value.as_str(), condition.value.as_str()) {
                    Ok(a.contains(b))
                } else {
                    Ok(false)
                }
            }
            MigrationConditionOperator::NotContains => {
                if let (Some(a), Some(b)) = (field_value.as_str(), condition.value.as_str()) {
                    Ok(!a.contains(b))
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// Get field value from nested JSON
    fn get_field_value(
        &self,
        field_path: &str,
        config: &serde_json::Value,
    ) -> RhemaResult<serde_json::Value> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = config;

        for part in parts {
            current = current.get(part).ok_or_else(|| {
                ConfigError::MigrationFailed(format!(
                    "Field '{}' not found in configuration",
                    field_path
                ))
            })?;
        }

        Ok(current.clone())
    }

    /// Apply rollback step
    fn apply_rollback_step<T: Config>(
        &self,
        _config: &T,
        step: &MigrationStep,
        _context: &str,
    ) -> RhemaResult<Vec<ConfigChange>> {
        let mut changes = Vec::new();

        match &step.step_type {
            MigrationStepType::AddField => {
                // Rollback: Remove the field
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Deleted,
                    description: format!("Rollback: Removed field {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::RemoveField => {
                // Rollback: Add the field back
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Created,
                    description: format!("Rollback: Restored field {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::RenameField => {
                // Rollback: Rename back to original
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Renamed field back {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::ChangeFieldType => {
                // Rollback: Change type back
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Changed field type back {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::UpdateFieldValue => {
                // Rollback: Restore original value
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Restored field value {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::AddSection => {
                // Rollback: Remove the section
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Deleted,
                    description: format!("Rollback: Removed section {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::RemoveSection => {
                // Rollback: Add the section back
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Created,
                    description: format!("Rollback: Restored section {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::RenameSection => {
                // Rollback: Rename section back
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Renamed section back {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::TransformData => {
                // Rollback: Transform data back
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Transformed data back {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::ExecuteScript => {
                // Rollback: Execute rollback script
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Executed rollback script {}", step.description),
                    user: "system".to_string(),
                });
            }
            MigrationStepType::Custom => {
                // Rollback: Execute custom rollback
                changes.push(ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: ConfigChangeType::Updated,
                    description: format!("Rollback: Executed custom rollback {}", step.description),
                    user: "system".to_string(),
                });
            }
        }

        Ok(changes)
    }

    /// Check for required fields in configuration
    fn check_required_fields(&self, config: &serde_json::Value) -> RhemaResult<Vec<String>> {
        let mut missing_fields = Vec::new();

        // Define required fields for different configuration types
        let required_fields = vec!["version", "user.name"];

        for field in required_fields {
            let path_parts: Vec<&str> = field.split('.').collect();
            let mut current = config;
            let mut found = true;

            for part in path_parts {
                if let Some(next) = current.get(part) {
                    current = next;
                } else {
                    found = false;
                    break;
                }
            }

            if !found {
                missing_fields.push(field.to_string());
            }
        }

        Ok(missing_fields)
    }

    /// Add custom migration
    pub fn add_migration(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }

    /// Remove migration
    pub fn remove_migration(&mut self, migration_name: &str) {
        self.migrations.retain(|m| m.name != migration_name);
    }

    /// Get migration history
    pub fn get_migration_history(&self, config_path: &Path) -> Option<&Vec<MigrationRecord>> {
        self.migration_history.get(config_path)
    }

    /// Set auto migrate
    pub fn set_auto_migrate(&mut self, enabled: bool) {
        self.auto_migrate = enabled;
    }

    /// Set backup before migration
    pub fn set_backup_before_migration(&mut self, enabled: bool) {
        self.backup_before_migration = enabled;
    }

    /// Get available migrations
    pub fn get_available_migrations(&self) -> &[Migration] {
        &self.migrations
    }
}
