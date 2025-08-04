# Rhema Git Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-git)](https://crates.io/crates/rhema-git)
[![Documentation](https://docs.rs/rhema-git/badge.svg)](https://docs.rs/rhema-git)

Advanced Git integration, workflow automation, security features, monitoring, and hooks for Rhema.

## Overview

The `rhema-git` crate provides comprehensive Git integration for Rhema, including workflow automation, security features, monitoring, and hooks. It enables seamless integration between Rhema's knowledge management system and Git workflows.

## Features

### 🔄 Git Workflow Automation
- **Feature Branch Management**: Automated feature branch creation, validation, and cleanup
- **Release Workflows**: Automated release branch management and versioning
- **Hotfix Management**: Streamlined hotfix branch workflows
- **Merge Automation**: Automated merge operations with conflict resolution

### 🔒 Git Security Features
- **Signature Validation**: Validate Git commit signatures and GPG keys
- **Secret Detection**: Detect and prevent secrets from being committed
- **Vulnerability Scanning**: Scan for security vulnerabilities in code
- **Malware Detection**: Detect malicious code patterns
- **File Encryption**: Encrypt sensitive files in Git repositories

### 📊 Git Monitoring and Hooks
- **Real-time Monitoring**: Monitor Git operations and repository health
- **Custom Hooks**: Pre-commit, post-commit, and other Git hooks
- **Metrics Collection**: Collect Git operation metrics and analytics
- **Health Checks**: Repository health monitoring and reporting

### 🔍 Git History Analysis
- **Change Analysis**: Analyze code changes and their impact
- **Commit Analysis**: Detailed commit history analysis
- **Blame Integration**: Integrate with Git blame for change attribution
- **Impact Assessment**: Assess the impact of changes across the codebase

### 🤖 AI Integration
- **Context Synchronization**: Keep Rhema context synchronized with Git state
- **AI-Powered Analysis**: AI-driven analysis of Git changes
- **Automated Documentation**: Automatically update documentation based on changes
- **Commit Message Generation**: Generate meaningful commit messages

## Architecture

```
rhema-git/
├── git/              # Core Git functionality
│   ├── basic.rs      # Basic Git operations
│   ├── advanced.rs   # Advanced Git features
│   ├── automation.rs # Workflow automation
│   ├── branch.rs     # Branch management
│   ├── security.rs   # Security features
│   └── hooks.rs      # Git hooks
├── git_basic.rs      # Basic Git integration
├── git.rs            # Main Git module
└── lib.rs            # Library entry point
```

## Usage

### Basic Git Operations

```rust
use rhema_git::git::basic::GitBasic;
use rhema_git::git::advanced::GitAdvanced;

let git_basic = GitBasic::new();
let git_advanced = GitAdvanced::new();

// Check repository status
let status = git_basic.get_status()?;

// Get current branch
let branch = git_basic.get_current_branch()?;

// Get repository information
let repo_info = git_advanced.get_repository_info()?;
```

### Workflow Automation

```rust
use rhema_git::git::automation::WorkflowManager;

let workflow_manager = WorkflowManager::new();

// Create feature branch
workflow_manager.create_feature_branch("user-auth")?;

// Validate feature branch
workflow_manager.validate_feature_branch("user-auth")?;

// Merge feature branch
workflow_manager.merge_feature_branch("user-auth")?;
```

### Security Features

```rust
use rhema_git::git::security::SecurityManager;

let security_manager = SecurityManager::new();

// Validate commit signatures
security_manager.validate_signatures()?;

// Scan for secrets
let secrets = security_manager.scan_for_secrets()?;

// Check for vulnerabilities
let vulnerabilities = security_manager.scan_vulnerabilities()?;
```

### Git Hooks

```rust
use rhema_git::git::hooks::HookManager;

let hook_manager = HookManager::new();

// Install pre-commit hook
hook_manager.install_pre_commit_hook()?;

// Install post-commit hook
hook_manager.install_post_commit_hook()?;

// Run custom hook
hook_manager.run_custom_hook("my-hook")?;
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
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **git2**: Git operations
- **serde**: Serialization support
- **tokio**: Async runtime
- **tracing**: Logging and tracing
- **chrono**: Date and time handling
- **uuid**: Unique identifier generation

## Development Status

### ✅ Completed Features
- Basic Git operations
- Repository detection and management
- Branch management utilities
- Basic security scanning

### 🔄 In Progress
- Workflow automation
- Advanced security features
- Git hooks implementation
- Monitoring and metrics

### 📋 Planned Features
- AI-powered analysis
- Advanced conflict resolution
- Performance optimization
- Enterprise features

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all Git operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 