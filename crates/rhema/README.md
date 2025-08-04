# Rhema Main Crate

[![Crates.io](https://img.shields.io/crates/v/rhema)](https://crates.io/crates/rhema)
[![Documentation](https://docs.rs/rhema/badge.svg)](https://docs.rs/rhema)

Main Rhema crate providing the core API, scope management, and integration point for all Rhema functionality.

## Overview

The `rhema` crate serves as the main entry point and integration layer for the Rhema system. It provides the core API, scope management, and orchestrates all other Rhema crates to deliver a unified knowledge management experience.

## Features

### 🏗️ Core API
- **Unified API**: Single entry point for all Rhema functionality
- **Scope Management**: Hierarchical scope-based organization
- **Integration Layer**: Orchestrates all Rhema crates
- **Error Handling**: Comprehensive error handling and recovery

### 📊 Scope Management
- **Scope Creation**: Create and manage project scopes
- **Scope Hierarchy**: Manage hierarchical scope relationships
- **Scope Validation**: Validate scope configurations and dependencies
- **Scope Migration**: Migrate scopes between versions

### 🔧 Core Operations
- **Knowledge Operations**: CRUD operations for all knowledge types
- **Query Operations**: Execute queries across scopes
- **Git Integration**: Seamless Git workflow integration
- **AI Integration**: AI agent coordination and context injection

### 🚀 Performance and Optimization
- **Caching**: Intelligent caching across all operations
- **Optimization**: Performance optimization and monitoring
- **Resource Management**: Efficient resource management
- **Scalability**: Scalable architecture for large projects

### 🔄 Integration and Extensibility
- **Plugin System**: Extensible plugin architecture
- **API Extensions**: Extensible API for custom functionality
- **Event System**: Event-driven architecture for extensibility
- **Custom Integrations**: Support for custom integrations

## Architecture

```
rhema/
├── lib.rs            # Main library entry point
├── api.rs            # Core API implementation
├── scope.rs          # Scope management
├── operations.rs     # Core operations
├── integration.rs    # Integration layer
├── performance.rs    # Performance optimization
└── extensions.rs     # Extensibility and plugins
```

## Usage

### Basic Usage

```rust
use rhema::Rhema;

// Initialize Rhema
let rhema = Rhema::new()?;

// Create a scope
let scope = rhema.create_scope("user-service", ScopeConfig {
    name: "User Service".to_string(),
    description: "User authentication and management service".to_string(),
    parent: None,
    children: vec![],
})?;

// Add knowledge to scope
rhema.add_todo(&scope, Todo {
    title: "Implement JWT authentication".to_string(),
    description: "Add JWT-based authentication system".to_string(),
    status: "pending".to_string(),
    priority: 1,
})?;

// Query knowledge
let todos = rhema.query_todos(&scope, "status = 'pending'")?;
```

### Scope Management

```rust
use rhema::scope::ScopeManager;

let scope_manager = ScopeManager::new();

// Create hierarchical scopes
let parent_scope = scope_manager.create_scope("backend")?;
let child_scope = scope_manager.create_child_scope(&parent_scope, "user-service")?;

// Validate scope hierarchy
scope_manager.validate_hierarchy()?;

// Migrate scope
scope_manager.migrate_scope(&child_scope, "2.0.0")?;
```

### Knowledge Operations

```rust
use rhema::operations::KnowledgeOperations;

let operations = KnowledgeOperations::new();

// Add decision
operations.add_decision(&scope, Decision {
    title: "Use JWT for authentication".to_string(),
    description: "JWT provides stateless authentication".to_string(),
    rationale: "Better scalability and performance".to_string(),
    status: "accepted".to_string(),
})?;

// Add pattern
operations.add_pattern(&scope, Pattern {
    name: "Repository Pattern".to_string(),
    description: "Data access abstraction pattern".to_string(),
    examples: vec!["UserRepository".to_string()],
})?;

// Add insight
operations.add_insight(&scope, Insight {
    title: "Authentication performance".to_string(),
    description: "JWT reduces database queries".to_string(),
    impact: "High".to_string(),
})?;
```

### Query Operations

```rust
use rhema::operations::QueryOperations;

let query_ops = QueryOperations::new();

// Execute CQL query
let results = query_ops.execute_cql("
    SELECT todos, decisions 
    FROM scope('user-service') 
    WHERE status = 'pending'
")?;

// Semantic search
let search_results = query_ops.semantic_search("authentication patterns")?;

// Repository analysis
let analysis = query_ops.analyze_repository("src/")?;
```

### AI Integration

```rust
use rhema::integration::AIIntegration;

let ai_integration = AIIntegration::new();

// Inject context for AI
let context = ai_integration.inject_context("implement user auth", &scope)?;

// Coordinate AI agents
ai_integration.coordinate_agents(&["agent-1", "agent-2"], "shared-task")?;

// Get AI recommendations
let recommendations = ai_integration.get_recommendations(&scope)?;
```

## Configuration

### Main Configuration

```yaml
# .rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    name: "my-project"
    type: "service"
    description: "Main project scope"
    
  api:
    enabled: true
    port: 8080
    cors:
      enabled: true
      origins: ["http://localhost:3000"]
    
  performance:
    caching:
      enabled: true
      max_size: "1GB"
    optimization:
      enabled: true
      parallel_processing: true
```

### Scope Configuration

```yaml
rhema:
  scopes:
    - name: "backend"
      type: "service"
      children:
        - name: "user-service"
          type: "service"
          dependencies:
            - "auth-service"
            - "database"
    
    - name: "frontend"
      type: "application"
      children:
        - name: "user-interface"
          type: "component"
```

## Dependencies

- **rhema-core**: Core data structures and operations
- **rhema-cli**: Command line interface
- **rhema-query**: Query engine and search
- **rhema-git**: Git integration
- **rhema-ai**: AI service integration
- **rhema-mcp**: MCP protocol support
- **rhema-config**: Configuration management
- **rhema-knowledge**: Knowledge management
- **rhema-monitoring**: Monitoring and observability
- **rhema-integrations**: External integrations

## Development Status

### ✅ Completed Features
- Core API framework
- Basic scope management
- Knowledge operations
- Integration layer

### 🔄 In Progress
- Advanced scope management
- Performance optimization
- Plugin system
- API extensions

### 📋 Planned Features
- Advanced API features
- Enterprise integrations
- Performance monitoring
- Advanced extensibility

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all API operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 