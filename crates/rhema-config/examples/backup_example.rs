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
    BackupFormat, BackupFrequency, BackupManager, BackupSchedule, Config, GlobalConfig,
    RepositoryConfig, RestoredConfig, RhemaResult, ScopeConfig,
};
use serde_json::json;
use std::path::PathBuf;
use tracing::{error, info, warn};

/// Example demonstrating configuration backup and restoration capabilities
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting configuration backup example");

    // Create a global configuration
    let global_config = create_sample_global_config()?;

    // Create backup manager
    let mut backup_manager = BackupManager::new(&global_config)?;

    // Example 1: Basic backup and restore
    basic_backup_restore(&mut backup_manager).await?;

    // Example 2: Backup with compression
    backup_with_compression(&mut backup_manager).await?;

    // Example 3: Backup with encryption
    backup_with_encryption(&mut backup_manager).await?;

    // Example 4: Backup multiple configurations
    backup_multiple_configs(&mut backup_manager).await?;

    // Example 5: Automatic backup scheduling
    automatic_backup_scheduling(&mut backup_manager).await?;

    // Example 6: Backup integrity checking
    backup_integrity_checking(&mut backup_manager).await?;

    // Example 7: Backup retention management
    backup_retention_management(&mut backup_manager).await?;

    // Example 8: Backup format conversion
    backup_format_conversion(&mut backup_manager).await?;

    // Example 9: Backup statistics and monitoring
    backup_statistics_monitoring(&mut backup_manager).await?;

    // Example 10: Disaster recovery simulation
    disaster_recovery_simulation(&mut backup_manager).await?;

    info!("Configuration backup example completed successfully");
    Ok(())
}

/// Create a sample global configuration
fn create_sample_global_config() -> RhemaResult<GlobalConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "environment": "development",
        "backup": {
            "enabled": true,
            "directory": "./backups",
            "format": "YAML",
            "compression": true,
            "encryption": false,
            "max_backups": 10,
            "retention_days": 30
        }
    });

    GlobalConfig::load_from_json(&config_json)
}

/// Example 1: Basic backup and restore
async fn basic_backup_restore(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 1: Basic Backup and Restore ===");

    // Create a sample configuration
    let config = create_sample_config()?;

    // Create a backup
    let backup_record = backup_manager
        .backup_config(&config, "basic-backup-test")
        .await?;

    info!("Basic backup created:");
    info!("  Backup ID: {}", backup_record.backup_id);
    info!("  Original path: {:?}", backup_record.original_path);
    info!("  Backup path: {:?}", backup_record.backup_path);
    info!("  Size: {} bytes", backup_record.size_bytes);
    info!("  Format: {:?}", backup_record.format);
    info!("  Compression: {}", backup_record.compression_enabled);
    info!("  Encryption: {}", backup_record.encryption_enabled);

    // Restore the configuration
    let restored_config: RepositoryConfig = backup_manager
        .restore_config("repository", &backup_record.backup_id)
        .await?;

    info!("Configuration restored successfully");
    info!("  Restored version: {}", restored_config.version());
    info!("  Repository name: {}", restored_config.repository.name);

    Ok(())
}

/// Example 2: Backup with compression
async fn backup_with_compression(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 2: Backup with Compression ===");

    // Enable compression
    backup_manager.set_compression_enabled(true);

    // Create a large configuration
    let large_config = create_large_config()?;

    // Create compressed backup
    let backup_record = backup_manager
        .backup_config(&large_config, "compression-test")
        .await?;

    info!("Compressed backup created:");
    info!("  Backup ID: {}", backup_record.backup_id);
    info!("  Original size: {} bytes", backup_record.size_bytes);
    info!(
        "  Compression enabled: {}",
        backup_record.compression_enabled
    );

    // Get detailed statistics
    let detailed_stats = backup_manager.get_detailed_backup_stats().await?;
    info!("Compression statistics:");
    info!(
        "  Compression ratio: {:.2}%",
        detailed_stats.compression_ratio * 100.0
    );
    info!(
        "  Total compressed size: {} bytes",
        detailed_stats.total_compressed_size_bytes
    );

    Ok(())
}

/// Example 3: Backup with encryption
async fn backup_with_encryption(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 3: Backup with Encryption ===");

    // Enable encryption
    backup_manager.set_encryption_enabled(true);

    // Create a sensitive configuration
    let sensitive_config = create_sensitive_config()?;

    // Create encrypted backup
    let backup_record = backup_manager
        .backup_config(&sensitive_config, "encryption-test")
        .await?;

    info!("Encrypted backup created:");
    info!("  Backup ID: {}", backup_record.backup_id);
    info!("  Encryption enabled: {}", backup_record.encryption_enabled);
    info!("  Checksum: {}", backup_record.checksum);

    // Validate backup integrity
    let integrity_valid = backup_manager
        .validate_backup_integrity(&backup_record.backup_path)
        .await?;

    info!("Backup integrity validation: {}", integrity_valid);

    Ok(())
}

/// Example 4: Backup multiple configurations
async fn backup_multiple_configs(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 4: Backup Multiple Configurations ===");

    // Create multiple configurations
    let configs = vec![
        ("repo1", create_sample_config()?),
        ("repo2", create_another_config()?),
        ("scope1", create_scope_config()?),
    ];

    let mut backup_records = Vec::new();

    // Backup each configuration
    for (name, config) in configs {
        let backup_record = backup_manager
            .backup_config(&config, &format!("multi-backup-{}", name))
            .await?;

        backup_records.push(backup_record);
        info!("Backed up {}: {}", name, backup_record.backup_id);
    }

    // List all backups
    let all_backups = backup_manager.list_backups(None);
    info!("Total backups available: {}", all_backups.len());

    // Get backup statistics
    let stats = backup_manager.get_backup_stats();
    info!("Backup statistics:");
    info!("  Total backups: {}", stats.total_backups);
    info!("  Successful backups: {}", stats.successful_backups);
    info!("  Total size: {} bytes", stats.total_size_bytes);
    info!(
        "  Compression ratio: {:.2}%",
        stats.compression_ratio * 100.0
    );

    Ok(())
}

/// Example 5: Automatic backup scheduling
async fn automatic_backup_scheduling(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 5: Automatic Backup Scheduling ===");

    // Create different backup schedules
    let schedules = vec![
        BackupSchedule {
            frequency: BackupFrequency::Daily,
            time: "02:00".to_string(),
            day_of_week: None,
            day_of_month: None,
            enabled: true,
        },
        BackupSchedule {
            frequency: BackupFrequency::Weekly,
            time: "03:00".to_string(),
            day_of_week: Some("Sunday".to_string()),
            day_of_month: None,
            enabled: true,
        },
        BackupSchedule {
            frequency: BackupFrequency::Monthly,
            time: "04:00".to_string(),
            day_of_week: None,
            day_of_month: Some(1),
            enabled: true,
        },
    ];

    // Schedule automatic backups
    for (i, schedule) in schedules.iter().enumerate() {
        backup_manager.schedule_automatic_backup(schedule).await?;

        info!(
            "Scheduled automatic backup {}: {:?}",
            i + 1,
            schedule.frequency
        );
    }

    info!("Automatic backup scheduling completed");

    Ok(())
}

/// Example 6: Backup integrity checking
async fn backup_integrity_checking(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 6: Backup Integrity Checking ===");

    // Create a configuration and backup it
    let config = create_sample_config()?;
    let backup_record = backup_manager
        .backup_with_integrity_check(&config, "integrity-test")
        .await?;

    info!("Backup with integrity check created:");
    info!("  Backup ID: {}", backup_record.backup_id);
    info!("  Checksum: {}", backup_record.checksum);

    // Validate integrity
    let integrity_valid = backup_manager
        .validate_backup_integrity(&backup_record.backup_path)
        .await?;

    info!("Backup integrity validation: {}", integrity_valid);

    // Simulate corruption (in a real scenario, this would be file corruption)
    info!("Simulating backup corruption test...");

    // Restore with integrity check
    let restored_config: RepositoryConfig = backup_manager
        .restore_with_integrity_check("repository", &backup_record.backup_id)
        .await?;

    info!("Restored configuration with integrity check:");
    info!("  Version: {}", restored_config.version());
    info!("  Repository: {}", restored_config.repository.name);

    Ok(())
}

/// Example 7: Backup retention management
async fn backup_retention_management(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 7: Backup Retention Management ===");

    // Set retention policies
    backup_manager.set_max_backups(5);
    backup_manager.set_retention_days(7);

    info!("Retention policies set:");
    info!("  Max backups: 5");
    info!("  Retention days: 7");

    // Create multiple backups to test retention
    for i in 1..=10 {
        let config = create_sample_config()?;
        let backup_record = backup_manager
            .backup_config(&config, &format!("retention-test-{}", i))
            .await?;

        info!("Created backup {}: {}", i, backup_record.backup_id);
    }

    // List backups after retention cleanup
    let remaining_backups = backup_manager.list_backups(None);
    info!(
        "Backups after retention cleanup: {}",
        remaining_backups.len()
    );

    Ok(())
}

/// Example 8: Backup format conversion
async fn backup_format_conversion(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 8: Backup Format Conversion ===");

    // Test different backup formats
    let formats = vec![BackupFormat::YAML, BackupFormat::JSON, BackupFormat::TOML];

    for format in formats {
        backup_manager.set_backup_format(format.clone());

        let config = create_sample_config()?;
        let backup_record = backup_manager
            .backup_config(&config, &format!("format-test-{:?}", format))
            .await?;

        info!("Backup created in {:?} format:", format);
        info!("  Backup ID: {}", backup_record.backup_id);
        info!("  Format: {:?}", backup_record.format);
        info!("  Size: {} bytes", backup_record.size_bytes);

        // Restore from this format
        let restored_config: RepositoryConfig = backup_manager
            .restore_config("repository", &backup_record.backup_id)
            .await?;

        info!("  Successfully restored from {:?} format", format);
    }

    Ok(())
}

/// Example 9: Backup statistics and monitoring
async fn backup_statistics_monitoring(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 9: Backup Statistics and Monitoring ===");

    // Get basic statistics
    let stats = backup_manager.get_backup_stats();
    info!("Basic backup statistics:");
    info!("  Total backups: {}", stats.total_backups);
    info!("  Successful backups: {}", stats.successful_backups);
    info!("  Failed backups: {}", stats.failed_backups);
    info!("  Total size: {} bytes", stats.total_size_bytes);
    info!(
        "  Compression ratio: {:.2}%",
        stats.compression_ratio * 100.0
    );

    // Get detailed statistics
    let detailed_stats = backup_manager.get_detailed_backup_stats().await?;
    info!("Detailed backup statistics:");
    info!(
        "  Total compressed size: {} bytes",
        detailed_stats.total_compressed_size_bytes
    );
    info!(
        "  Compression ratio: {:.2}%",
        detailed_stats.compression_ratio * 100.0
    );
    info!("  Format distribution:");
    for (format, count) in &detailed_stats.format_distribution {
        info!("    {:?}: {}", format, count);
    }
    info!("  Age distribution:");
    for (age, count) in &detailed_stats.age_distribution {
        info!("    {}: {}", age, count);
    }

    Ok(())
}

/// Example 10: Disaster recovery simulation
async fn disaster_recovery_simulation(backup_manager: &mut BackupManager) -> RhemaResult<()> {
    info!("=== Example 10: Disaster Recovery Simulation ===");

    // Create a critical configuration
    let critical_config = create_critical_config()?;

    // Create multiple backups with different settings
    let backup_scenarios = vec![
        ("daily", false, false), // Daily backup, no compression, no encryption
        ("weekly", true, false), // Weekly backup, compression, no encryption
        ("monthly", true, true), // Monthly backup, compression, encryption
    ];

    let mut backup_records = Vec::new();

    for (scenario, compression, encryption) in backup_scenarios {
        backup_manager.set_compression_enabled(compression);
        backup_manager.set_encryption_enabled(encryption);

        let backup_record = backup_manager
            .backup_with_integrity_check(
                &critical_config,
                &format!("disaster-recovery-{}", scenario),
            )
            .await?;

        backup_records.push((scenario.to_string(), backup_record));
        info!("Created {} backup: {}", scenario, backup_record.backup_id);
    }

    // Simulate disaster recovery
    info!("Simulating disaster recovery...");

    // Choose the most recent backup for recovery
    if let Some((scenario, backup_record)) = backup_records.last() {
        info!(
            "Recovering from {} backup: {}",
            scenario, backup_record.backup_id
        );

        // Validate backup integrity before recovery
        let integrity_valid = backup_manager
            .validate_backup_integrity(&backup_record.backup_path)
            .await?;

        if integrity_valid {
            // Restore the configuration
            let restored_config: RepositoryConfig = backup_manager
                .restore_with_integrity_check("repository", &backup_record.backup_id)
                .await?;

            info!("Disaster recovery completed successfully:");
            info!("  Recovered version: {}", restored_config.version());
            info!("  Repository name: {}", restored_config.repository.name);
            info!("  Recovery source: {} backup", scenario);
        } else {
            error!("Backup integrity check failed, cannot proceed with recovery");
        }
    }

    Ok(())
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

/// Create another configuration
fn create_another_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "another-repo",
            "url": "https://github.com/user/another-repo",
            "branch": "develop"
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a scope configuration
fn create_scope_config() -> RhemaResult<ScopeConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "scope": {
            "include": ["src/**/*.rs"],
            "exclude": ["target/"]
        }
    });

    ScopeConfig::load_from_json(&config_json)
}

/// Create a large configuration for compression testing
fn create_large_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "large-repo",
            "url": "https://github.com/user/large-repo",
            "branch": "main"
        },
        "large_data": {
            "description": "This is a large configuration with lots of data for testing compression",
            "repeated_text": "This text is repeated many times to create a larger file for compression testing. ".repeat(1000),
            "metadata": {
                "created": "2025-01-01T00:00:00Z",
                "updated": "2025-01-01T00:00:00Z",
                "tags": vec!["large", "test", "compression"].repeat(100)
            }
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a sensitive configuration for encryption testing
fn create_sensitive_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "sensitive-repo",
            "url": "https://github.com/user/sensitive-repo",
            "branch": "main"
        },
        "sensitive_data": {
            "api_keys": {
                "production": "sk-prod-1234567890abcdef",
                "staging": "sk-staging-1234567890abcdef"
            },
            "database": {
                "host": "sensitive-db.example.com",
                "port": 5432,
                "name": "sensitive_db",
                "user": "admin",
                "password": "super-secret-password"
            }
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

/// Create a critical configuration for disaster recovery testing
fn create_critical_config() -> RhemaResult<RepositoryConfig> {
    let config_json = json!({
        "version": "1.0.0",
        "repository": {
            "name": "critical-repo",
            "url": "https://github.com/user/critical-repo",
            "branch": "main"
        },
        "critical_settings": {
            "production": true,
            "backup_frequency": "hourly",
            "recovery_time_objective": "4 hours",
            "recovery_point_objective": "1 hour",
            "monitoring": {
                "enabled": true,
                "alerts": ["email", "sms", "slack"],
                "escalation": ["oncall", "manager", "cto"]
            }
        }
    });

    RepositoryConfig::load_from_json(&config_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backup_example() {
        // This test ensures the example runs without panicking
        let global_config = create_sample_global_config().unwrap();
        let mut backup_manager = BackupManager::new(&global_config).unwrap();

        // Test basic backup
        let config = create_sample_config().unwrap();
        let backup_record = backup_manager
            .backup_config(&config, "test-backup")
            .await
            .unwrap();

        assert!(!backup_record.backup_id.is_empty());
        assert!(backup_record.size_bytes > 0);
    }

    #[tokio::test]
    async fn test_backup_integrity() {
        let global_config = create_sample_global_config().unwrap();
        let mut backup_manager = BackupManager::new(&global_config).unwrap();

        let config = create_sample_config().unwrap();
        let backup_record = backup_manager
            .backup_with_integrity_check(&config, "integrity-test")
            .await
            .unwrap();

        let integrity_valid = backup_manager
            .validate_backup_integrity(&backup_record.backup_path)
            .await
            .unwrap();

        assert!(integrity_valid);
    }
}
