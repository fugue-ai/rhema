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

use super::*;
use crate::RhemaResult;
use crate::validation::ValidationManager;
use crate::migration::MigrationManager;
use crate::backup::BackupManager;
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_creation() -> RhemaResult<()> {
        let config = GlobalConfig::new();
        assert_eq!(config.version, CURRENT_CONFIG_VERSION);
        assert_eq!(config.user.name, "Default User");
        assert_eq!(config.application.name, "Rhema CLI");
        Ok(())
    }

    #[test]
    fn test_validation_manager_creation() -> RhemaResult<()> {
        let global_config = GlobalConfig::new();
        let manager = ValidationManager::new(&global_config)?;
        assert!(!manager.get_rules().is_empty());
        Ok(())
    }

    #[test]
    fn test_migration_manager_creation() -> RhemaResult<()> {
        let global_config = GlobalConfig::new();
        let manager = MigrationManager::new(&global_config)?;
        assert!(!manager.get_available_migrations().is_empty());
        Ok(())
    }

    #[test]
    fn test_backup_manager_creation() -> RhemaResult<()> {
        let global_config = GlobalConfig::new();
        let manager = BackupManager::new(&global_config)?;
        assert!(
            manager.get_backup_directory().exists()
                || manager.get_backup_directory().parent().unwrap().exists()
        );
        Ok(())
    }

    #[test]
    fn test_config_audit_log() {
        let mut audit_log = ConfigAuditLog::new();
        let entry = ConfigAuditEntry {
            timestamp: Utc::now(),
            action: "test_action".to_string(),
            user: "test_user".to_string(),
            details: "Test change".to_string(),
        };

        audit_log.entries.push(entry);
        assert_eq!(audit_log.entries.len(), 1);
    }

    #[test]
    fn test_config_health() {
        let health = ConfigHealth {
            status: ConfigHealthStatus::Healthy,
            issues: Vec::new(),
            last_check: Utc::now(),
        };

        assert_eq!(health.status, ConfigHealthStatus::Healthy);
        assert_eq!(health.issues.len(), 0);
    }

    #[test]
    fn test_config_stats() {
        let stats = ConfigStats::new();
        assert_eq!(stats.total_configs, 0);
        assert_eq!(stats.valid_configs, 0);
        assert_eq!(stats.invalid_configs, 0);
    }
}
