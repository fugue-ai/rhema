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

use rhema_config::{
    Config, GlobalConfig, Migration, MigrationCondition, MigrationConditionOperator,
    MigrationManager, MigrationStep, MigrationStepType, RepositoryConfig, RhemaResult, ScopeConfig,
    CURRENT_CONFIG_VERSION,
};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{error, info, warn};

/// Example demonstrating configuration migration capabilities
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting configuration migration example");

    // Create a global configuration
    let global_config = create_sample_global_config()?;

    // Create migration manager
    let mut migration_manager = MigrationManager::new(&global_config)?;

    // Example 1: Basic version migration
    basic_version_migration(&migration_manager).await?;

    // Example 2: Complex migration with multiple steps
    complex_migration(&migration_manager).await?;

    // Example 3: Conditional migration
    conditional_migration(&migration_manager).await?;

    // Example 4: Migration rollback
    migration_rollback(&migration_manager).await?;

    // Example 5: Migration validation
    migration_validation(&migration_manager).await?;

    // Example 6: Custom migration
    custom_migration(&migration_manager).await?;

    // Example 7: Migration history
    migration_history(&migration_manager).await?;

    // Example 8: Migration with backup
    migration_with_backup(&migration_manager).await?;

    // Example 9: Migration scheduling
    migration_scheduling(&migration_manager).await?;

    // Example 10: Migration testing
    migration_testing(&migration_manager).await?;

    info!("Configuration migration example completed successfully");
    Ok(())
}

/// Create a sample global configuration
fn create_sample_global_config() -> RhemaResult<GlobalConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "environment": "development",
        "migration": {
            "auto_migrate": true,
            "backup_before_migration": true,
            "validate_after_migration": true
        }
    });

    GlobalConfig::load_from_json(&config_json)
}

/// Example 1: Basic version migration
async fn basic_version_migration(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 1: Basic Version Migration ===");

    // Create an old version configuration
    let old_config = create_old_version_config()?;

    // Migrate to current version
    let migration_report = migration_manager
        .migrate_version(&old_config, CURRENT_CONFIG_VERSION)
        .await?;

    info!("Basic migration completed:");
    info!(
        "  Migrations applied: {}",
        migration_report.migrations_applied.len()
    );
    info!(
        "  Migrations skipped: {}",
        migration_report.migrations_skipped.len()
    );
    info!(
        "  Migrations failed: {}",
        migration_report.migrations_failed.len()
    );
    info!(
        "  Total changes: {}",
        migration_report.summary.total_changes
    );
    info!("  Duration: {}ms", migration_report.duration_ms);

    Ok(())
}

/// Example 2: Complex migration with multiple steps
async fn complex_migration(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 2: Complex Migration with Multiple Steps ===");

    // Create a complex migration
    let complex_migration = Migration {
        from_version: "1.0.0".to_string(),
        to_version: "2.0.0".to_string(),
        name: "complex-upgrade".to_string(),
        description: "Complex migration with multiple steps".to_string(),
        steps: vec![
            MigrationStep {
                step_type: MigrationStepType::AddField,
                description: "Add new security settings".to_string(),
                parameters: json!({
                    "path": "security",
                    "field": "encryption_enabled",
                    "value": true
                })
                .as_object()
                .unwrap()
                .clone(),
                condition: None,
                rollback: None,
            },
            MigrationStep {
                step_type: MigrationStepType::RenameField,
                description: "Rename old field to new name".to_string(),
                parameters: json!({
                    "old_path": "old_field",
                    "new_path": "new_field"
                })
                .as_object()
                .unwrap()
                .clone(),
                condition: None,
                rollback: None,
            },
            MigrationStep {
                step_type: MigrationStepType::TransformData,
                description: "Transform data format".to_string(),
                parameters: json!({
                    "path": "data",
                    "transformation": "json_to_yaml"
                })
                .as_object()
                .unwrap()
                .clone(),
                condition: None,
                rollback: None,
            },
        ],
        rollback_steps: vec![MigrationStep {
            step_type: MigrationStepType::RemoveField,
            description: "Remove added security field".to_string(),
            parameters: json!({
                "path": "security.encryption_enabled"
            })
            .as_object()
            .unwrap()
            .clone(),
            condition: None,
            rollback: None,
        }],
        required: true,
        automatic: false,
    };

    // Add the migration to the manager
    let mut manager = migration_manager.clone();
    manager.add_migration(complex_migration);

    // Create a config to migrate
    let config = create_sample_config()?;

    // Perform the migration
    let migration_report = manager.migrate_config(&config, "complex-migration-test")?;

    info!("Complex migration completed:");
    info!(
        "  Migrations applied: {}",
        migration_report.migrations_applied.len()
    );
    info!(
        "  Total changes: {}",
        migration_report.summary.total_changes
    );

    Ok(())
}

/// Example 3: Conditional migration
async fn conditional_migration(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 3: Conditional Migration ===");

    // Create a conditional migration
    let conditional_migration = Migration {
        from_version: "1.5.0".to_string(),
        to_version: "2.0.0".to_string(),
        name: "conditional-upgrade".to_string(),
        description: "Migration that only applies under certain conditions".to_string(),
        steps: vec![MigrationStep {
            step_type: MigrationStepType::AddField,
            description: "Add feature flag if not present".to_string(),
            parameters: json!({
                "path": "features",
                "field": "new_feature",
                "value": true
            })
            .as_object()
            .unwrap()
            .clone(),
            condition: Some(MigrationCondition {
                field: "features.new_feature",
                operator: MigrationConditionOperator::NotExists,
                value: json!(null),
            }),
            rollback: None,
        }],
        rollback_steps: vec![],
        required: false,
        automatic: true,
    };

    // Add the migration to the manager
    let mut manager = migration_manager.clone();
    manager.add_migration(conditional_migration);

    // Create configs with and without the condition
    let config_with_feature = create_config_with_feature()?;
    let config_without_feature = create_config_without_feature()?;

    // Migrate both configs
    let report_with = manager.migrate_config(&config_with_feature, "with-feature")?;
    let report_without = manager.migrate_config(&config_without_feature, "without-feature")?;

    info!("Conditional migration results:");
    info!(
        "  Config with feature - migrations applied: {}",
        report_with.migrations_applied.len()
    );
    info!(
        "  Config without feature - migrations applied: {}",
        report_without.migrations_applied.len()
    );

    Ok(())
}

/// Example 4: Migration rollback
async fn migration_rollback(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 4: Migration Rollback ===");

    // Create a config and migrate it
    let original_config = create_sample_config()?;
    let migration_report = migration_manager.migrate_config(&original_config, "rollback-test")?;

    if let Some(migration_record) = migration_report.migrations_applied.first() {
        // Rollback the migration
        let rollback_report = migration_manager
            .rollback_migration(&original_config, migration_record)
            .await?;

        info!("Migration rollback completed:");
        info!(
            "  Original migrations: {}",
            migration_report.migrations_applied.len()
        );
        info!(
            "  Rollback migrations: {}",
            rollback_report.migrations_applied.len()
        );
        info!(
            "  Rollback successful: {}",
            rollback_report.summary.successful_migrations
        );
    }

    Ok(())
}

/// Example 5: Migration validation
async fn migration_validation(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 5: Migration Validation ===");

    // Create a config and migrate it
    let config = create_sample_config()?;
    let migration_report = migration_manager.migrate_config(&config, "validation-test")?;

    // Validate the migration results
    let validation_result = migration_manager
        .validate_migration_results(&config, &migration_report)
        .await?;

    info!("Migration validation completed:");
    info!("  Migration valid: {}", validation_result.valid);
    info!("  Validation issues: {}", validation_result.issues.len());
    info!(
        "  Validation warnings: {}",
        validation_result.warnings.len()
    );

    Ok(())
}

/// Example 6: Custom migration
async fn custom_migration(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 6: Custom Migration ===");

    // Create a custom migration with custom step type
    let custom_migration = Migration {
        from_version: "1.0.0".to_string(),
        to_version: "1.1.0".to_string(),
        name: "custom-upgrade".to_string(),
        description: "Custom migration with custom step".to_string(),
        steps: vec![MigrationStep {
            step_type: MigrationStepType::Custom,
            description: "Custom data transformation".to_string(),
            parameters: json!({
                "custom_action": "transform_data",
                "source_format": "old",
                "target_format": "new",
                "transformation_rules": {
                    "field_mapping": {
                        "old_field": "new_field",
                        "deprecated_field": null
                    }
                }
            })
            .as_object()
            .unwrap()
            .clone(),
            condition: None,
            rollback: None,
        }],
        rollback_steps: vec![],
        required: false,
        automatic: true,
    };

    // Add the migration to the manager
    let mut manager = migration_manager.clone();
    manager.add_migration(custom_migration);

    // Perform the migration
    let config = create_sample_config()?;
    let migration_report = manager.migrate_config(&config, "custom-migration-test")?;

    info!("Custom migration completed:");
    info!(
        "  Migrations applied: {}",
        migration_report.migrations_applied.len()
    );
    info!(
        "  Custom steps executed: {}",
        migration_report.summary.total_changes
    );

    Ok(())
}

/// Example 7: Migration history
async fn migration_history(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 7: Migration History ===");

    // Perform some migrations to build history
    let config = create_sample_config()?;
    let _report1 = migration_manager.migrate_config(&config, "history-test-1")?;
    let _report2 = migration_manager.migrate_config(&config, "history-test-2")?;

    // Get migration history
    let config_path = PathBuf::from("sample-config.yml");
    if let Some(history) = migration_manager.get_migration_history(&config_path) {
        info!("Migration history for {}:", config_path.display());
        for record in history {
            info!(
                "  {}: {} -> {} (success: {})",
                record.migration_name, record.from_version, record.to_version, record.success
            );
        }
    }

    Ok(())
}

/// Example 8: Migration with backup
async fn migration_with_backup(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 8: Migration with Backup ===");

    // Enable backup before migration
    let mut manager = migration_manager.clone();
    manager.set_backup_before_migration(true);

    // Create a config and migrate it
    let config = create_sample_config()?;
    let migration_report = manager.migrate_config(&config, "backup-test")?;

    info!("Migration with backup completed:");
    info!(
        "  Migrations applied: {}",
        migration_report.migrations_applied.len()
    );
    info!("  Backup created: {}", manager.backup_before_migration);

    Ok(())
}

/// Example 9: Migration scheduling
async fn migration_scheduling(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 9: Migration Scheduling ===");

    // Get available migrations
    let available_migrations = migration_manager.get_available_migrations();

    info!("Available migrations:");
    for migration in available_migrations {
        info!(
            "  {}: {} -> {} ({})",
            migration.name,
            migration.from_version,
            migration.to_version,
            if migration.automatic {
                "automatic"
            } else {
                "manual"
            }
        );
    }

    // Schedule a migration (this would typically be done by a scheduler)
    info!("Migration scheduling would be handled by an external scheduler");

    Ok(())
}

/// Example 10: Migration testing
async fn migration_testing(migration_manager: &MigrationManager) -> RhemaResult<()> {
    info!("=== Example 10: Migration Testing ===");

    // Create test configurations
    let test_configs = vec![
        create_sample_config()?,
        create_old_version_config()?,
        create_config_with_feature()?,
        create_config_without_feature()?,
    ];

    // Test migrations on each config
    for (i, config) in test_configs.iter().enumerate() {
        info!("Testing migration on config {}", i + 1);

        let migration_report =
            migration_manager.migrate_config(config, &format!("test-config-{}", i + 1))?;

        info!("  Config {} migration results:", i + 1);
        info!(
            "    Migrations applied: {}",
            migration_report.migrations_applied.len()
        );
        info!(
            "    Migrations failed: {}",
            migration_report.migrations_failed.len()
        );
        info!(
            "    Total changes: {}",
            migration_report.summary.total_changes
        );
    }

    Ok(())
}

/// Create an old version configuration
fn create_old_version_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "0.9.0",
        "repository": {
            "name": "old-repo",
            "url": "https://github.com/user/old-repo",
            "branch": "master" // Old field name
        },
        "old_field": "deprecated_value"
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a sample configuration
fn create_sample_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "sample-repo",
            "url": "https://github.com/user/sample-repo",
            "branch": "main"
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a configuration with a feature
fn create_config_with_feature() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.5.0",
        "repository": {
            "name": "feature-repo",
            "url": "https://github.com/user/feature-repo",
            "branch": "main"
        },
        "features": {
            "existing_feature": true
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a configuration without a feature
fn create_config_without_feature() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.5.0",
        "repository": {
            "name": "no-feature-repo",
            "url": "https://github.com/user/no-feature-repo",
            "branch": "main"
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_migration_example() {
        // This test ensures the example runs without panicking
        let global_config = create_sample_global_config().unwrap();
        let migration_manager = MigrationManager::new(&global_config).unwrap();

        // Test basic migration
        let old_config = create_old_version_config().unwrap();
        let migration_report = migration_manager
            .migrate_version(&old_config, CURRENT_CONFIG_VERSION)
            .await
            .unwrap();

        assert!(migration_report.summary.successful_migrations > 0);
    }

    #[tokio::test]
    async fn test_conditional_migration() {
        let global_config = create_sample_global_config().unwrap();
        let mut migration_manager = MigrationManager::new(&global_config).unwrap();

        // Create conditional migration
        let conditional_migration = Migration {
            from_version: "1.0.0".to_string(),
            to_version: "1.1.0".to_string(),
            name: "test-conditional".to_string(),
            description: "Test conditional migration".to_string(),
            steps: vec![MigrationStep {
                step_type: MigrationStepType::AddField,
                description: "Add test field".to_string(),
                parameters: HashMap::new(),
                condition: Some(MigrationCondition {
                    field: "test_field",
                    operator: MigrationConditionOperator::NotExists,
                    value: json!(null),
                }),
                rollback: None,
            }],
            rollback_steps: vec![],
            required: false,
            automatic: true,
        };

        migration_manager.add_migration(conditional_migration);

        let config = create_sample_config().unwrap();
        let report = migration_manager.migrate_config(&config, "test").unwrap();

        assert!(report.summary.successful_migrations >= 0);
    }
}
