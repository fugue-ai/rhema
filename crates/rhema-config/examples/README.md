# Configuration Examples

This directory contains comprehensive examples demonstrating the various features of the Rhema configuration system.

## Overview

The configuration system provides a robust, secure, and flexible way to manage configuration files with features including:

- **Comprehensive Validation**: Schema validation, business logic validation, and cross-reference checking
- **Migration Management**: Version migrations with rollback capabilities
- **Backup and Recovery**: Automatic backups with compression and encryption
- **Security Features**: Encryption, access control, audit logging, and compliance checking

## Examples

### 1. Comprehensive Validation Example (`comprehensive_validation_example.rs`)

Demonstrates the full validation system including:

- **Single Configuration Validation**: Validate individual configuration files
- **Directory Validation**: Validate multiple configuration files in a directory
- **Cross-Reference Validation**: Validate references between configuration sections
- **Custom Rules Validation**: Validate with custom business rules
- **Performance Validation**: Validate performance-related configurations
- **Security Validation**: Validate security configurations
- **Compliance Validation**: Validate compliance configurations
- **Dependency Validation**: Validate configuration dependencies
- **Auto-Fix Demonstration**: Show automatic issue fixing capabilities
- **Validation Statistics**: Display validation performance metrics

**Usage:**
```bash
cargo run --example comprehensive_validation_example
```

### 2. Migration Example (`migration_example.rs`)

Demonstrates configuration migration capabilities including:

- **Basic Version Migration**: Simple version upgrades
- **Complex Migration**: Multi-step migrations with conditions
- **Conditional Migration**: Migrations that only apply under certain conditions
- **Migration Rollback**: Rollback failed or unwanted migrations
- **Migration Validation**: Validate migration results
- **Custom Migration**: Custom migration steps and transformations
- **Migration History**: Track migration history
- **Migration with Backup**: Automatic backups before migration
- **Migration Scheduling**: Schedule migrations for later execution
- **Migration Testing**: Test migrations on various configurations

**Usage:**
```bash
cargo run --example migration_example
```

### 3. Backup Example (`backup_example.rs`)

Demonstrates backup and recovery features including:

- **Basic Backup and Restore**: Simple backup and restoration
- **Backup with Compression**: Compressed backups to save space
- **Backup with Encryption**: Encrypted backups for security
- **Multiple Configurations**: Backup multiple configuration files
- **Automatic Backup Scheduling**: Schedule automatic backups
- **Backup Integrity Checking**: Verify backup integrity
- **Backup Retention Management**: Manage backup retention policies
- **Backup Format Conversion**: Convert between backup formats
- **Backup Statistics and Monitoring**: Monitor backup performance
- **Disaster Recovery Simulation**: Simulate disaster recovery scenarios

**Usage:**
```bash
cargo run --example backup_example
```

### 4. Security Example (`security_example.rs`)

Demonstrates security features including:

- **Configuration Encryption**: Encrypt and decrypt sensitive configurations
- **Access Control and Permissions**: Control access to configuration files
- **Audit Logging**: Log configuration access and changes
- **Compliance Checking**: Check compliance with security standards
- **Integrity Verification**: Verify configuration integrity
- **Security Policy Enforcement**: Enforce security policies
- **Key Management**: Manage encryption keys
- **Security Monitoring**: Monitor security events
- **Security Incident Response**: Respond to security incidents
- **Security Assessment**: Assess overall security posture

**Usage:**
```bash
cargo run --example security_example
```

## Running the Examples

### Prerequisites

1. **Rust Toolchain**: Ensure you have Rust installed (version 1.70 or later)
2. **Dependencies**: The examples use various dependencies that should be available in the workspace
3. **Configuration Files**: Some examples create temporary configuration files for demonstration

### Basic Usage

To run any example:

```bash
# Navigate to the config crate directory
cd crates/rhema-config

# Run a specific example
cargo run --example <example_name>

# For example:
cargo run --example comprehensive_validation_example
cargo run --example migration_example
cargo run --example backup_example
cargo run --example security_example
```

### Running All Examples

To run all examples in sequence:

```bash
# Run all examples
cargo run --example comprehensive_validation_example
cargo run --example migration_example
cargo run --example backup_example
cargo run --example security_example
```

### Testing the Examples

Each example includes comprehensive tests:

```bash
# Run tests for all examples
cargo test --examples

# Run tests for a specific example
cargo test --example comprehensive_validation_example
```

## Example Output

### Comprehensive Validation Example

```
Starting comprehensive configuration validation example
=== Example 1: Single Configuration Validation ===
Single Configuration Validation Result:
  Valid: true
  Schema valid: true
  Business valid: true
  Duration: 15ms
  Issues: 0
  Warnings: 0
```

### Migration Example

```
Starting configuration migration example
=== Example 1: Basic Version Migration ===
Basic migration completed:
  Migrations applied: 2
  Migrations skipped: 0
  Migrations failed: 0
  Total changes: 5
  Duration: 45ms
```

### Backup Example

```
Starting configuration backup example
=== Example 1: Basic Backup and Restore ===
Basic backup created:
  Backup ID: backup_20250101_120000_001
  Original path: "config.yml"
  Backup path: "./backups/backup_20250101_120000_001.yml"
  Size: 1024 bytes
  Format: YAML
  Compression: true
  Encryption: false
```

### Security Example

```
Starting configuration security example
=== Example 1: Configuration Encryption and Decryption ===
Configuration encrypted:
  Original size: 512 bytes
  Encrypted size: 768 bytes
  Encryption ratio: 150.00%
Configuration decrypted successfully:
  Decrypted version: 1.0.0
  Repository name: sensitive-repo
```

## Configuration Examples

### Sample Configuration Files

The examples create various sample configurations:

#### Basic Repository Configuration
```yaml
version: "1.0.0"
repository:
  name: "sample-repo"
  url: "https://github.com/user/sample-repo"
  branch: "main"
```

#### Sensitive Configuration (for encryption testing)
```yaml
version: "1.0.0"
repository:
  name: "sensitive-repo"
  url: "https://github.com/user/sensitive-repo"
  branch: "main"
sensitive_data:
  api_keys:
    production: "sk-prod-1234567890abcdef"
    staging: "sk-staging-1234567890abcdef"
  database:
    host: "sensitive-db.example.com"
    port: 5432
    name: "sensitive_db"
    user: "admin"
    password: "super-secret-password"
```

#### Large Configuration (for compression testing)
```yaml
version: "1.0.0"
repository:
  name: "large-repo"
  url: "https://github.com/user/large-repo"
  branch: "main"
large_data:
  description: "This is a large configuration with lots of data for testing compression"
  repeated_text: "This text is repeated many times to create a larger file..."
  metadata:
    created: "2025-01-01T00:00:00Z"
    updated: "2025-01-01T00:00:00Z"
    tags: ["large", "test", "compression"]
```

## Key Features Demonstrated

### Validation Features
- **Schema Validation**: Validate against JSON schemas
- **Business Logic Validation**: Custom validation rules
- **Cross-Reference Validation**: Validate references between sections
- **Performance Validation**: Validate performance configurations
- **Security Validation**: Validate security settings
- **Compliance Validation**: Validate compliance requirements
- **Auto-Fix**: Automatic issue resolution
- **Validation Statistics**: Performance metrics

### Migration Features
- **Version Migration**: Upgrade between versions
- **Conditional Migration**: Apply migrations based on conditions
- **Rollback**: Undo migrations
- **Validation**: Validate migration results
- **History Tracking**: Track migration history
- **Scheduling**: Schedule migrations
- **Testing**: Test migrations

### Backup Features
- **Compression**: Compress backup files
- **Encryption**: Encrypt sensitive backups
- **Retention**: Manage backup retention
- **Integrity Checking**: Verify backup integrity
- **Format Conversion**: Convert between formats
- **Statistics**: Monitor backup performance
- **Disaster Recovery**: Simulate recovery scenarios

### Security Features
- **Encryption**: Encrypt sensitive data
- **Access Control**: Control file access
- **Audit Logging**: Log all activities
- **Compliance**: Check compliance standards
- **Integrity**: Verify data integrity
- **Policy Enforcement**: Enforce security policies
- **Key Management**: Manage encryption keys
- **Monitoring**: Monitor security events

## Troubleshooting

### Common Issues

1. **Permission Errors**: Ensure you have write permissions in the current directory
2. **Missing Dependencies**: Run `cargo build` to ensure all dependencies are available
3. **Configuration Errors**: Check that sample configurations are properly formatted
4. **Backup Directory**: Ensure backup directories exist and are writable

### Debug Mode

To run examples with debug output:

```bash
RUST_LOG=debug cargo run --example comprehensive_validation_example
```

### Verbose Output

To see detailed output:

```bash
RUST_LOG=info cargo run --example migration_example
```

## Contributing

When adding new examples:

1. **Follow the Pattern**: Use the existing example structure
2. **Include Tests**: Add comprehensive tests for each example
3. **Documentation**: Update this README with new example details
4. **Error Handling**: Include proper error handling and logging
5. **Cleanup**: Ensure examples clean up after themselves

## License

These examples are licensed under the Apache License, Version 2.0. See the LICENSE file for details. 