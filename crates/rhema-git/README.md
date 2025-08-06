# Rhema Git Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-git)](https://crates.io/crates/rhema-git)
[![Documentation](https://docs.rs/rhema-git/badge.svg)](https://docs.rs/rhema-git)

Advanced Git integration, workflow automation, security features, monitoring, and hooks for Rhema.

## Overview

The `rhema-git` crate provides comprehensive Git integration for Rhema, including workflow automation, security features, monitoring, and hooks. It enables seamless integration between Rhema's knowledge management system and Git workflows with extensive automation capabilities.

## Features

### 🔄 Git Workflow Automation
- **Feature Branch Management**: Automated feature branch creation, validation, and cleanup
- **Release Workflows**: Automated release branch management and versioning
- **Hotfix Management**: Streamlined hotfix branch workflows
- **Merge Automation**: Automated merge operations with conflict resolution
- **Workflow Templates**: Pre-configured templates for GitFlow, GitHub Flow, GitLab Flow, and more
- **Context-Aware Automation**: AI-driven workflow suggestions and smart branch naming

### 🔒 Git Security Features
- **Signature Validation**: Validate Git commit signatures and GPG keys with enhanced metadata checking
- **Secret Detection**: Advanced secret detection with ML-based validation and false positive filtering
- **Vulnerability Scanning**: Scan for security vulnerabilities in code and dependencies
- **Access Control**: Role-based access control with branch protection rules
- **File Encryption**: Encrypt sensitive files using AES-256-GCM and ChaCha20-Poly1305
- **Audit Logging**: Comprehensive audit logging with configurable retention policies
- **Threat Detection**: ML-based threat detection with configurable rules and alerting

### 📊 Git Monitoring and Hooks
- **Real-time Monitoring**: Monitor Git operations and repository health
- **Custom Hooks**: Pre-commit, post-commit, and other Git hooks with validation
- **Metrics Collection**: Collect Git operation metrics and analytics
- **Health Checks**: Repository health monitoring and reporting
- **Automation Status Tracking**: Track automation tasks and their execution status

### 🔍 Git History Analysis
- **Change Analysis**: Analyze code changes and their impact
- **Commit Analysis**: Detailed commit history analysis with context evolution tracking
- **Blame Integration**: Integrate with Git blame for change attribution
- **Impact Assessment**: Assess the impact of changes across the codebase
- **Context Evolution**: Track how context files evolve over time

### 🤖 AI Integration
- **Context Synchronization**: Keep Rhema context synchronized with Git state
- **AI-Powered Analysis**: AI-driven analysis of Git changes and workflow optimization
- **Automated Documentation**: Automatically update documentation based on changes
- **Commit Message Generation**: Generate meaningful commit messages using AI
- **Smart Branch Naming**: AI-enhanced branch naming with context awareness

## Architecture

```
rhema-git/
├── src/
│   ├── lib.rs                    # Library entry point and re-exports
│   ├── git_basic.rs              # Basic Git operations and data structures
│   ├── git_hooks.rs              # Git hooks management
│   ├── workflow_templates.rs     # Workflow template definitions
│   └── git/                      # Core Git functionality
│       ├── mod.rs                # Module exports
│       ├── advanced.rs           # Advanced Git features
│       ├── automation.rs         # Workflow automation (3619 lines)
│       ├── branch.rs             # Branch management (1181 lines)
│       ├── feature_automation.rs # Feature branch automation (1855 lines)
│       ├── history.rs            # Git history analysis (1724 lines)
│       ├── hooks.rs              # Git hooks implementation (2617 lines)
│       ├── monitoring.rs         # Monitoring and metrics (2266 lines)
│       ├── security.rs           # Security features (2207 lines)
│       ├── version_management.rs # Version management (1082 lines)
│       └── workflow.rs           # Workflow management (3187 lines)
```

## Usage

### Basic Git Operations

```rust
use rhema_git::{get_repo, get_current_branch, get_changed_files};
use std::path::Path;

// Get repository instance
let repo = get_repo(Path::new("."))?;

// Get current branch
let branch = get_current_branch(&repo)?;

// Get changed files
let changed_files = get_changed_files(&repo)?;
```

### Advanced Git Integration

```rust
use rhema_git::{create_advanced_git_integration, AdvancedGitIntegration};

// Create advanced integration
let mut git_integration = create_advanced_git_integration(Path::new("."))?;

// Create feature branch
let feature_branch = git_integration.create_feature_branch("user-auth", "develop")?;

// Finish feature branch
let result = git_integration.finish_feature_branch("user-auth")?;
```

### Workflow Automation

```rust
use rhema_git::git::automation::{GitAutomationManager, default_automation_config};

let config = default_automation_config();
let automation_manager = GitAutomationManager::new(repo, config);

// Start automation
automation_manager.start_automation()?;

// Trigger feature automation
automation_manager.trigger_feature_automation("user-auth", "create")?;
```

### Security Features

```rust
use rhema_git::git::security::{SecurityManager, default_security_config};

let security_config = default_security_config();
let security_manager = SecurityManager::new(repo, security_config);

// Run security scan
let scan_result = security_manager.run_security_scan(Path::new("."))?;

// Validate commit security
let validation = security_manager.validate_commit_security(&commit)?;
```

### Workflow Templates

```rust
use rhema_git::git::workflow_templates::{WorkflowTemplateManager, WorkflowTemplateType};

// Get available templates
let templates = WorkflowTemplateManager::get_available_templates();

// Get GitFlow template
let gitflow_template = WorkflowTemplateManager::get_template(&WorkflowTemplateType::GitFlow)?;

// Apply customizations
let mut customizations = HashMap::new();
customizations.insert("main_branch".to_string(), serde_json::Value::String("master".to_string()));
let config = WorkflowTemplateManager::apply_customization(&gitflow_template, &customizations)?;
```

## Configuration

### Git Workflow Configuration

```yaml
# .rhema/git.yaml
git:
  workflows:
    feature:
      auto_create: true
      validation:
        required_tests: true
        code_review: true
      cleanup:
        auto_delete: true
        after_merge: true
    
    release:
      auto_version: true
      changelog:
        auto_generate: true
        template: "CHANGELOG.md"
    
    hotfix:
      auto_create: true
      validation:
        critical_tests: true
```

### Security Configuration

```yaml
git:
  security:
    enabled: true
    signature_validation: true
    secret_detection:
      enabled: true
      patterns:
        - "api_key"
        - "password"
        - "secret"
    vulnerability_scanning:
      enabled: true
      tools:
        - "snyk"
        - "bandit"
    encryption:
      enabled: true
      algorithm: "AES256"
```

### Automation Configuration

```yaml
git:
  automation:
    auto_context_updates: true
    auto_synchronization: true
    auto_notifications: true
    intervals:
      context_update_interval: 300
      sync_interval: 600
      backup_interval: 3600
    notifications:
      email:
        smtp_server: "smtp.example.com"
        recipients: ["team@example.com"]
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **git2**: Git operations
- **serde**: Serialization support
- **tokio**: Async runtime
- **tracing**: Logging and tracing
- **chrono**: Date and time handling
- **uuid**: Unique identifier generation
- **aes-gcm**: AES encryption
- **chacha20poly1305**: ChaCha20 encryption
- **argon2**: Password hashing
- **keyring**: Secure key storage
- **regex**: Pattern matching
- **semver**: Semantic versioning

## Development Status

### ✅ Completed Features
- **Core Git Operations**: Repository management, branch operations, commit handling
- **Workflow Automation**: Feature, release, and hotfix branch automation
- **Security Features**: Signature validation, secret detection, vulnerability scanning, file encryption
- **Git Hooks**: Pre-commit, post-commit, and custom hook management
- **Monitoring**: Real-time monitoring and metrics collection
- **Workflow Templates**: GitFlow, GitHub Flow, GitLab Flow, and custom templates
- **Version Management**: Automated version bumping and tagging
- **History Analysis**: Git history analysis and context evolution tracking
- **AI Integration**: AI-powered workflow suggestions and smart automation

### 🔄 In Progress
- **Performance Optimization**: Large repository handling and caching improvements
- **Enterprise Features**: Advanced access control and compliance features
- **Integration Testing**: Comprehensive test coverage for all features

### 📋 Planned Features
- **Distributed Workflows**: Multi-repository workflow coordination
- **Advanced ML Models**: Enhanced AI-driven analysis and predictions
- **Cloud Integration**: Native integration with GitHub, GitLab, and Azure DevOps
- **Real-time Collaboration**: Live collaboration features for team workflows

## Examples

The crate includes several comprehensive examples:

- `workflow_example.rs`: Basic workflow demonstration
- `real_workflow_example.rs`: Real-world workflow scenarios
- `git_hooks_example.rs`: Git hooks implementation
- `conflict_resolution_example.rs`: Conflict resolution strategies

Run examples with:
```bash
cargo run --example workflow_example
cargo run --example real_workflow_example
```

## Testing

Run the test suite:
```bash
cargo test
```

The module includes comprehensive unit tests for:
- Workflow template validation
- Security feature testing
- Automation workflow testing
- Git operations testing

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all Git operations are properly tested
4. Run the test suite: `cargo test`
5. Add tests for new features

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 