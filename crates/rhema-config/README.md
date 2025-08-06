# Rhema Config Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-config)](https://crates.io/crates/rhema-config)
[![Documentation](https://docs.rs/rhema-config/badge.svg)](https://docs.rs/rhema-config)

Configuration management for Rhema, providing robust configuration validation, migration, backup, and security features.

## Overview

The `rhema-config` crate provides comprehensive configuration management capabilities for Rhema, including validation, migration, backup, and security features. It ensures that all Rhema components have consistent, validated, and secure configuration management.

## Features

### ⚙️ Configuration Management
- **Multi-Format Support**: YAML, JSON, TOML, and binary configuration formats
- **Hierarchical Configuration**: Global, project, and scope-level configuration
- **Environment Overrides**: Environment variable-based configuration overrides
- **Default Values**: Sensible defaults with override capabilities

### ✅ Validation and Schema
- **Schema Validation**: JSON Schema-based configuration validation
- **Type Safety**: Strongly typed configuration structures
- **Custom Validators**: Extensible validation rules
- **Error Reporting**: Detailed validation error messages

### 🔄 Migration and Versioning
- **Configuration Migration**: Automatic migration between configuration versions
- **Version Compatibility**: Backward and forward compatibility support
- **Migration Scripts**: Custom migration logic for complex changes
- **Rollback Support**: Ability to rollback configuration changes

### 💾 Backup and Recovery
- **Automatic Backups**: Automatic backup of configuration changes
- **Backup Rotation**: Configurable backup retention policies
- **Recovery Tools**: Tools for recovering from configuration issues
- **Backup Validation**: Validation of backup integrity

### 🔒 Security Features
- **Encryption**: Encryption of sensitive configuration data
- **Access Control**: Role-based access to configuration
- **Audit Logging**: Comprehensive audit trail for configuration changes
- **Secret Management**: Secure handling of secrets and credentials

## Architecture

```
rhema-config/
├── config.rs        # Main configuration management
├── validation.rs    # Configuration validation
├── migration.rs     # Configuration migration
├── backup.rs        # Backup and recovery
├── security.rs      # Security features
├── schema.rs        # Configuration schemas
├── global.rs        # Global configuration
└── utils/           # Utility functions
```

## Usage

### Basic Configuration

```rust
use rhema_config::ConfigManager;
use rhema_config::schema::RhemaConfig;

// Initialize configuration manager
let config_manager = ConfigManager::new();

// Load configuration
let config: RhemaConfig = config_manager.load_config("config.yaml")?;

// Validate configuration
config_manager.validate_config(&config)?;
```

### Configuration Validation

```rust
use rhema_config::validation::ConfigValidator;

let validator = ConfigValidator::new();

// Validate configuration against schema
let validation_result = validator.validate(&config)?;

if !validation_result.is_valid() {
    for error in validation_result.errors() {
        println!("Validation error: {}", error);
    }
}
```

### Configuration Migration

```rust
use rhema_config::migration::ConfigMigrator;

let migrator = ConfigMigrator::new();

// Migrate configuration to latest version
let migrated_config = migrator.migrate_config(&old_config)?;

// Check migration status
let migration_status = migrator.get_migration_status(&config)?;
```

### Backup and Recovery

```rust
use rhema_config::backup::BackupManager;

let backup_manager = BackupManager::new();

// Create backup
backup_manager.create_backup(&config, "config-backup")?;

// Restore from backup
let restored_config = backup_manager.restore_backup("config-backup")?;
```

## Configuration Schema

### Global Configuration

```yaml
# ~/.rhema/config.yaml
global:
  editor: vscode
  theme: dark
  ai_provider: openai
  cache_dir: ~/.rhema/cache
  security:
    encryption_enabled: true
    audit_logging: true
```

### Project Configuration

```yaml
# .rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    name: "my-project"
    type: "service"
    dependencies:
      parent: "../shared"
      children: ["../api", "../ui"]
  config:
    validation:
      strict: true
    backup:
      auto_backup: true
      retention_days: 30
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **serde**: Serialization support
- **validator**: Configuration validation
- **toml**: TOML format support
- **bincode**: Binary serialization
- **semver**: Version management
- **flate2**: Compression support
- **sha2**: Cryptographic hashing

## Development Status

### ✅ Completed Features
- Basic configuration loading and saving
- Multi-format support (YAML, JSON, TOML)
- Schema validation framework
- Configuration migration infrastructure

### 🔄 In Progress
- Advanced validation rules
- Migration script system
- Backup and recovery features
- Security enhancements

### 📋 Planned Features
- Configuration encryption
- Role-based access control
- Advanced audit logging
- Configuration templates

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all configuration schemas are well-documented
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 