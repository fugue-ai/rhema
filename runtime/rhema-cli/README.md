# Rhema Protocol CLI

The Rhema Protocol CLI is a comprehensive command-line interface for managing AI agent context through distributed YAML files in Git repositories. It provides a standardized protocol for organizing, querying, and maintaining contextual information that AI agents can use to understand and work with codebases effectively.

## Overview

Rhema CLI serves as the main entry point for the Rhema Protocol ecosystem, providing tools to:

- **Initialize and manage** Rhema repositories with structured context
- **Query and search** through contextual information using CQL (Context Query Language)
- **Manage knowledge artifacts** including todos, insights, patterns, and decisions
- **Validate and maintain** repository structure and data integrity
- **Coordinate** with AI services and external integrations
- **Monitor** performance and health of the context system

## Features

### Core Functionality

- **Repository Management**: Initialize, configure, and maintain Rhema repositories
- **Scope Discovery**: Automatically discover and manage different scopes within a repository
- **Context Querying**: Execute CQL queries to extract specific information from context files
- **Search Capabilities**: Full-text and regex search across all contextual data
- **Validation**: Comprehensive validation of repository structure and data integrity

### Knowledge Management

- **Todos**: Track tasks, priorities, assignments, and completion status
- **Insights**: Record and categorize knowledge gained during development
- **Patterns**: Document effective patterns and anti-patterns with usage contexts
- **Decisions**: Track architectural and design decisions with rationale and consequences

### Advanced Features

- **Lock System**: Dependency resolution and conflict prevention for context files
- **AI Integration**: Seamless integration with AI services for context injection
- **MCP Support**: Model Context Protocol daemon for external tool integration
- **Performance Monitoring**: Built-in performance tracking and health monitoring
- **Export/Import**: Context export and import capabilities for sharing knowledge

## Installation

The Rhema CLI is part of the larger Rhema Protocol workspace. To build and install:

```bash
# From the workspace root
cargo build --release -p rhema

# Install globally
cargo install --path apps/rhemad
```

## Usage

### Basic Commands

```bash
# Initialize a new Rhema repository
rhema init

# List all scopes in the repository
rhema scopes

# Show scope tree structure
rhema tree

# Execute a CQL query
rhema query "SELECT * FROM todos WHERE status = 'pending'"

# Search for content
rhema search "authentication" --in-file "auth.md"
```

### Knowledge Management

```bash
# Add a new todo
rhema todo add "Implement user authentication" --description "Add OAuth2 support" --priority high

# List todos
rhema todo list --status pending --priority high

# Record an insight
rhema insight record "Database connection pooling" --content "Connection pooling significantly improves performance" --confidence 8

# Add a pattern
rhema pattern add "Circuit Breaker" --description "Prevent cascade failures" --pattern-type "resilience" --usage recommended

# Record a decision
rhema decision record "Use PostgreSQL" --description "Database technology choice" --status accepted --rationale "ACID compliance and JSON support"
```

### Advanced Operations

```bash
# Validate repository structure
rhema validate --recursive

# Check health status
rhema health

# Show performance statistics
rhema stats

# Export context for sharing
rhema export-context --format json

# Start MCP daemon
rhema daemon start
```

## Architecture

### Core Components

- **`lib.rs`**: Main library interface providing the `Rhema` struct and core functionality
- **`main.rs`**: CLI application with comprehensive command structure
- **`commands/`**: Command implementations and subcommand handling
- **`lock/`**: Dependency resolution and conflict prevention system
- **`config/`**: Configuration management and validation

### Key Dependencies

- **`rhema-api`**: API layer and command implementations
- **`rhema-core`**: Core types, schemas, and error handling
- **`rhema-query`**: Query engine and repository analysis
- **`rhema-ai`**: AI service integration and context injection
- **`rhema-git`**: Git integration and repository operations
- **`rhema-config`**: Configuration management
- **`rhema-monitoring`**: Performance monitoring and health checks

### Lock System

The lock system provides:

- **Dependency Resolution**: Automatic resolution of context file dependencies
- **Conflict Prevention**: Detection and resolution of conflicting changes
- **Cache Management**: Efficient caching of resolved dependencies
- **Validation**: Comprehensive validation of lock file integrity

## Configuration

Rhema CLI supports both global and repository-specific configuration:

- **Global Config**: User-wide settings and preferences
- **Repository Config**: Project-specific configuration and rules
- **Scope Config**: Scope-level settings and conventions

Configuration files are automatically discovered and merged based on scope hierarchy.

## Integration

### AI Services

Rhema CLI integrates with AI services to provide:

- **Context Injection**: Automatic injection of relevant context into AI interactions
- **Query Assistance**: AI-powered query suggestions and optimization
- **Content Generation**: Automated generation of documentation and summaries

### External Tools

- **MCP Daemon**: Model Context Protocol support for external tool integration
- **Editor Plugins**: Integration with VSCode, Vim, Emacs, and IntelliJ
- **CI/CD**: Automated validation and context updates in CI/CD pipelines

## Development

### Building

```bash
# Build the crate
cargo build

# Run tests
cargo test

# Build with all features
cargo build --all-features
```

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'

# Run specific test suite
cargo test --test coordination_test_runner
```

### Contributing

1. Follow the existing code style and patterns
2. Add comprehensive tests for new functionality
3. Update documentation for any API changes
4. Ensure all tests pass before submitting changes

## License

Licensed under the Apache License, Version 2.0. See the LICENSE file for details.

## Related Documentation

- [Rhema Protocol Overview](../../ARCHITECTURE.md)
- [API Documentation](../rhema-api/README.md)
- [Core Library](../core/README.md)
- [Configuration Guide](../config/README.md)
- [Query Language Reference](../query/README.md) 