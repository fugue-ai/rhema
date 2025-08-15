# Rhema Core Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-core)](https://crates.io/crates/rhema-core)
[![Documentation](https://docs.rs/rhema-core/badge.svg)](https://docs.rs/rhema-core)

Core data structures and fundamental operations for the Rhema Protocol, providing the foundation for all Rhema functionality.

## Overview

The `rhema-core` crate provides the fundamental building blocks for Rhema, including data structures, error handling, file operations, and Git integration. It serves as the foundation upon which all other Rhema crates are built.

## Features

### 📊 Data Structures
- **Schema Definitions**: Core data structures for todos, decisions, patterns, and insights
- **Scope Management**: Hierarchical scope-based organization system
- **Type Safety**: Strongly typed data structures with validation
- **Serialization**: YAML, JSON, and binary serialization support

### 🔧 File Operations
- **CRUD Operations**: Create, read, update, and delete operations for Rhema files
- **File Watching**: Real-time file change detection and monitoring
- **Path Management**: Intelligent path resolution and management
- **Backup Operations**: File backup and recovery capabilities

### 🐙 Git Integration
- **Repository Detection**: Automatic Git repository detection
- **Git Operations**: Basic Git operations and status checking
- **Branch Management**: Branch-aware operations and context
- **Commit Integration**: Integration with Git commit workflows

### ⚠️ Error Handling
- **Comprehensive Error Types**: Detailed error types for all operations
- **Error Context**: Rich error context and debugging information
- **Error Recovery**: Graceful error recovery and fallback mechanisms
- **Error Reporting**: Structured error reporting and logging

### 🔍 Utilities
- **Validation**: Data validation and integrity checking
- **Caching**: Basic caching utilities and mechanisms
- **Monitoring**: Basic monitoring and metrics collection
- **Configuration**: Core configuration management

## Architecture

```
rhema-core/
├── schema/          # Data structure definitions
│   ├── mod.rs       # Schema module organization
│   ├── todos.rs     # Todo data structures
│   ├── decisions.rs # Decision data structures
│   ├── patterns.rs  # Pattern data structures
│   ├── insights.rs  # Insight data structures
│   └── scope.rs     # Scope data structures
├── file_ops.rs      # File operations
├── git_basic.rs     # Basic Git integration
├── error.rs         # Error handling
└── utils/           # Utility functions
```

## Usage

### Basic Data Structures

```rust
use rhema_core::schema::{Todos, Decisions, Patterns, Insights, Scope};

// Create a todo item
let todo = Todos {
    todos: vec![
        Todo {
            id: "todo-1".to_string(),
            title: "Implement user authentication".to_string(),
            description: "Add JWT-based authentication".to_string(),
            status: "pending".to_string(),
            priority: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ],
};

// Create a decision
let decision = Decisions {
    decisions: vec![
        Decision {
            id: "decision-1".to_string(),
            title: "Use JWT for authentication".to_string(),
            description: "JWT provides stateless authentication".to_string(),
            rationale: "Better scalability and performance".to_string(),
            alternatives: vec!["Session-based".to_string()],
            status: "accepted".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ],
};
```

### File Operations

```rust
use rhema_core::file_ops;

// Read a Rhema file
let todos: Todos = file_ops::read_yaml_file("todos.yaml")?;

// Write a Rhema file
file_ops::write_yaml_file("todos.yaml", &todos)?;

// Check if file exists
if file_ops::file_exists("todos.yaml") {
    // File exists
}

// Get Rhema directory
let rhema_dir = file_ops::get_rhema_dir()?;
```

### Git Integration

```rust
use rhema_core::git_basic;

// Check if current directory is a Git repository
if git_basic::is_git_repo()? {
    // Get current branch
    let branch = git_basic::get_current_branch()?;
    
    // Get repository root
    let repo_root = git_basic::get_repo_root()?;
}
```

### Error Handling

```rust
use rhema_core::RhemaError;

// Handle Rhema errors
match result {
    Ok(data) => {
        // Process data
    },
    Err(RhemaError::FileNotFound(path)) => {
        println!("File not found: {}", path);
    },
    Err(RhemaError::ValidationError(errors)) => {
        for error in errors {
            println!("Validation error: {}", error);
        }
    },
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

## Data Schemas

### Todo Schema

```yaml
todos:
  - id: "todo-1"
    title: "Implement user authentication"
    description: "Add JWT-based authentication system"
    status: "pending"
    priority: 1
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
```

### Decision Schema

```yaml
decisions:
  - id: "decision-1"
    title: "Use JWT for authentication"
    description: "Implement JWT-based authentication"
    rationale: "Better scalability and performance"
    alternatives: ["Session-based"]
    status: "accepted"
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
```

### Pattern Schema

```yaml
patterns:
  - id: "pattern-1"
    name: "Repository Pattern"
    description: "Data access abstraction pattern"
    examples: ["UserRepository", "ProductRepository"]
    benefits: ["Testability", "Maintainability"]
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
```

## Dependencies

- **serde**: Serialization support
- **git2**: Git integration
- **walkdir**: Directory traversal
- **validator**: Data validation
- **chrono**: Date and time handling
- **uuid**: Unique identifier generation
- **notify**: File watching
- **prometheus**: Metrics collection

## Development Status

### ✅ Completed Features
- Core data structures and schemas
- Basic file operations
- Git repository detection
- Error handling framework
- Serialization support
- **Advanced validation rules** - Comprehensive input validation for security
- **Async file operations** - High-performance non-blocking file operations
- **Caching strategies** - Multi-level caching with TTL and eviction policies
- **Audit logging** - Complete audit trail for security and compliance
- **Security enhancements** - Path validation, input sanitization, security monitoring
- **Performance optimizations** - Async operations, caching, memory optimization
- **Comprehensive testing** - Unit tests, integration tests, error handling tests
- **Complete documentation** - API docs, usage examples, best practices

### 🔄 In Progress
- Enhanced Git integration
- Advanced monitoring improvements
- Additional performance optimizations

### 📋 Planned Features
- File watching improvements
- Advanced error recovery mechanisms
- Additional security features

#### Performance Optimization
- [x] **Optimize file I/O operations** - Implemented async file operations with caching
- [x] **Add connection pooling** - Implemented connection pooling for async operations
- [x] **Implement async operations** - Full async support for all file operations
- [x] **Add memory optimization** - Optimized memory usage with configurable cache limits
- [x] **Implement caching strategies** - Multi-level caching with TTL and LRU eviction

#### Security
- [x] **Add input validation** - Comprehensive validation for all inputs
- [x] **Implement secure file operations** - Path validation and security checks
- [x] **Add access control** - File path validation and scope-based access control
- [x] **Implement audit logging** - Complete audit trail with configurable logging
- [x] **Add integrity checking** - Data validation and integrity verification

#### Testing and Quality
- [x] **Add comprehensive unit tests** - Complete test coverage for all modules
- [x] **Implement integration tests** - Integration tests for all components
- [x] **Add error handling tests** - Comprehensive error scenario testing

#### Documentation
- [x] **Add API documentation** - Complete API documentation with examples
- [x] **Create usage examples** - Extensive usage examples and patterns
- [x] **Add architecture documentation** - Detailed architecture documentation
- [x] **Create troubleshooting guide** - Comprehensive troubleshooting guide
- [x] **Add best practices guide** - Security and performance best practices

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all data structures are well-documented
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 