# Action Agents

This directory contains individual agent implementations that have been refactored from the main `rhema-agent` crate into dedicated crates for better modularity and maintainability.

## Overview

The action agents provide specialized functionality for different aspects of software development and operations:

- **Code Review Agent** - Security analysis and code review
- **Test Runner Agent** - Test generation and execution
- **Deployment Agent** - Application deployment and CI/CD
- **Documentation Agent** - Documentation generation and maintenance
- **Monitoring Agent** - System performance monitoring and alerting

## Crates

### rhema-code-review-agent

Performs comprehensive code analysis with focus on security vulnerabilities, code quality issues, and performance problems.

```rust
use rhema_code_review_agent::CodeReviewAgent;

let mut agent = CodeReviewAgent::new("code-review-1".to_string());
agent.initialize().await?;
```

### rhema-test-runner-agent

Automatically generates and executes tests for codebases with support for multiple testing frameworks.

```rust
use rhema_test_runner_agent::TestRunnerAgent;

let mut agent = TestRunnerAgent::new("test-runner-1".to_string());
agent.initialize().await?;
```

### rhema-deployment-agent

Manages application deployment and CI/CD processes with support for various deployment strategies.

```rust
use rhema_deployment_agent::DeploymentAgent;

let mut agent = DeploymentAgent::new("deployment-1".to_string());
agent.initialize().await?;
```

### rhema-documentation-agent

Generates and maintains documentation for codebases, APIs, and user guides.

```rust
use rhema_documentation_agent::DocumentationAgent;

let mut agent = DocumentationAgent::new("documentation-1".to_string());
agent.initialize().await?;
```

### rhema-monitoring-agent

Monitors system performance and provides alerting capabilities for various metrics.

```rust
use rhema_monitoring_agent::MonitoringAgent;

let mut agent = MonitoringAgent::new("monitoring-1".to_string());
agent.initialize().await?;
```

## Migration from rhema-agent

The agent implementations have been moved from `crates/rhema-agent/src/agents/` to individual crates in `crates/action-agents/`. 

### Before (old structure):
```rust
use rhema_agent::agents::{CodeReviewAgent, TestRunnerAgent};
```

### After (new structure):
```rust
use rhema_code_review_agent::CodeReviewAgent;
use rhema_test_runner_agent::TestRunnerAgent;
```

## Dependencies

Each agent crate depends on:
- `rhema-agent` - Core agent framework
- `rhema-core` - Core functionality
- `rhema-ai` - AI service integration
- `rhema-config` - Configuration management
- `rhema-knowledge` - Knowledge base integration

## Building

To build all action agents:

```bash
cargo build -p rhema-code-review-agent
cargo build -p rhema-test-runner-agent
cargo build -p rhema-deployment-agent
cargo build -p rhema-documentation-agent
cargo build -p rhema-monitoring-agent
```

Or build all workspace members:

```bash
cargo build
```

## Testing

Each agent crate includes its own test suite:

```bash
cargo test -p rhema-code-review-agent
cargo test -p rhema-test-runner-agent
cargo test -p rhema-deployment-agent
cargo test -p rhema-documentation-agent
cargo test -p rhema-monitoring-agent
```

## ✅ Refactoring Complete

All agent implementations have been successfully moved to dedicated crates and all tests are passing. The refactoring provides:

- **Better modularity**: Each agent is now in its own crate with focused dependencies
- **Improved maintainability**: Changes to one agent don't affect others
- **Selective dependencies**: Consumers can depend only on the agents they need
- **Cleaner architecture**: The main `rhema-agent` crate is now focused on core agent infrastructure

## Contributing

When adding new agents or modifying existing ones:

1. Create a new crate in `crates/action-agents/<agent-name>/`
2. Follow the established structure with `src/lib.rs` and `Cargo.toml`
3. Update the workspace `Cargo.toml` to include the new crate
4. Add appropriate documentation and tests
5. Update this README.md with information about the new agent 