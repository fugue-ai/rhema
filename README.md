# GACP CLI - Git-Based Agent Context Protocol

A Rust CLI tool for managing AI agent context using distributed YAML files in Git repositories.

## Overview

GACP (Git-Based Agent Context Protocol) is a Git-native system for AI agent context management. It stores context as YAML files in distributed `.gacp/` directories throughout repositories, enabling deterministic AI agent behavior and persistent knowledge accumulation.

## Features

- **Git-native**: Context travels with repository, no external dependencies
- **Human-readable**: YAML with inline documentation
- **Distributed ownership**: Teams manage their scope's context independently
- **Cross-scope relationships**: Explicit dependencies between scopes
- **Version-controlled**: Context evolves with code through Git

## Installation

### From Source

```bash
git clone <repository-url>
cd gacp
cargo build --release
```

The binary will be available at `target/release/gacp`.

### From Binary (Future)

Download the latest release binary for your platform from the releases page.

## Quick Start

### 1. Initialize a GACP Scope

```bash
# Initialize in current directory
gacp init

# Initialize with specific type and name
gacp init --scope-type service --scope-name user-service
```

This creates a `.gacp/` directory with template files:
- `gacp.yaml` - Scope definition
- `knowledge.yaml` - Knowledge base
- `todos.yaml` - Todo items
- `decisions.yaml` - Decisions
- `patterns.yaml` - Design patterns
- `conventions.yaml` - Coding conventions

### 2. Discover Scopes

```bash
# List all scopes in repository
gacp scopes

# Show scope hierarchy
gacp tree

# Show specific scope details
gacp scope path/to/scope
```

### 3. Query Context

```bash
# Query todos across all scopes
gacp query "todos WHERE status='in_progress'"

# Query specific scope
gacp query "../backend/.gacp/todos.active"

# Query with filtering
gacp query "knowledge WHERE category='architecture'"
```

### 4. Manage Content

```bash
# Add a todo
gacp todo add "Implement user authentication" --priority high

# Record an insight
gacp insight record "Database connection pooling improves performance by 40%" --confidence 8

# Add a pattern
gacp pattern add "Repository Pattern" --description "Separate data access logic" --pattern-type architectural

# Record a decision
gacp decision record "Use PostgreSQL" --description "Chosen for ACID compliance" --status approved
```

## Core Concepts

### Scopes

A scope represents a logical boundary in your codebase (service, app, library, etc.). Each scope has its own `.gacp/` directory containing context files.

### Context Files

- **`gacp.yaml`** - Scope definition and metadata
- **`knowledge.yaml`** - Insights, learnings, and domain knowledge
- **`todos.yaml`** - Work items and tasks
- **`decisions.yaml`** - Important decisions and rationale
- **`patterns.yaml`** - Design patterns and architectural patterns
- **`conventions.yaml`** - Coding conventions and standards

### CQL (Context Query Language)

Simple YAML path-based query syntax:

```bash
# Basic queries
gacp query "todos"
gacp query "knowledge.entries"

# Filtering
gacp query "todos WHERE status='pending'"
gacp query "knowledge WHERE confidence>7"

# Cross-scope queries
gacp query "../backend/.gacp/todos"
gacp query "*/patterns WHERE usage='required'"
```

## Command Reference

### Initialization and Discovery

```bash
gacp init [--scope-type TYPE] [--scope-name NAME]  # Initialize new scope
gacp scopes                                         # List all scopes
gacp scope [PATH]                                   # Show scope details
gacp tree                                           # Show scope hierarchy
```

### Content Management

```bash
gacp show FILE [--scope SCOPE]                      # Display YAML file content
gacp query "CQL_QUERY"                              # Execute context query
gacp search "TERM" [--in FILE]                      # Search across context files
```

### Validation and Health

```bash
gacp validate [--recursive]                         # Validate YAML files
gacp health [--scope SCOPE]                         # Check scope completeness
gacp stats                                          # Show context metrics
```

### Work Item Management

```bash
gacp todo add "TITLE" [--priority LEVEL]            # Add todo
gacp todo list [--status STATUS]                    # List todos
gacp todo complete ID [--outcome "DESCRIPTION"]     # Complete todo
```

### Knowledge Management

```bash
gacp insight record "INSIGHT" [--confidence LEVEL]  # Record insight
gacp pattern add "NAME" [--effectiveness LEVEL]     # Add pattern
gacp decision record "TITLE" [--status STATUS]      # Record decision
```

### Cross-Scope Operations

```bash
gacp dependencies                                   # Show scope relationships
gacp impact FILE                                    # Show affected scopes
gacp sync-knowledge                                 # Update cross-scope references
```

## Schema Examples

### gacp.yaml

```yaml
name: user-service
scope_type: service
description: User management and authentication service
version: "1.0.0"
dependencies:
  - path: "../shared-lib"
    dependency_type: required
    version: ">=1.0.0"
```

### knowledge.yaml

```yaml
entries:
  - id: "auth-pattern-001"
    title: "JWT Token Refresh Pattern"
    content: "Implement automatic token refresh before expiration..."
    category: "security"
    tags: ["jwt", "authentication", "security"]
    confidence: 9
    created_at: "2024-01-15T10:30:00Z"
    source: "production-incident"
```

### todos.yaml

```yaml
todos:
  - id: "todo-001"
    title: "Implement rate limiting"
    description: "Add rate limiting to prevent abuse"
    status: in_progress
    priority: high
    assigned_to: "alice"
    due_date: "2024-02-01T00:00:00Z"
    created_at: "2024-01-15T09:00:00Z"
```

## Development

### Building

```bash
cargo build
cargo test
cargo clippy
```

### Project Structure

```
gacp-cli/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── commands/            # CLI command implementations
│   ├── schema/              # YAML schema definitions
│   ├── query/               # Context query engine (CQL)
│   ├── git/                 # Git integration utilities
│   ├── scope/               # Scope discovery and management
│   └── error.rs             # Error types and handling
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Roadmap

- [ ] Enhanced CQL with joins and aggregations
- [ ] Git hooks for validation
- [ ] Web UI for context visualization
- [ ] Integration with popular IDEs
- [ ] Context export/import formats
- [ ] Advanced search and filtering
- [ ] Context analytics and metrics 