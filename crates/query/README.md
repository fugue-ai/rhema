# Rhema Query Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-query)](https://crates.io/crates/rhema-query)
[![Documentation](https://docs.rs/rhema-query/badge.svg)](https://docs.rs/rhema-query)

CQL (Context Query Language) query engine, repository analysis, and advanced search capabilities for Rhema.

## Top Priorities

| Priority | Component | Task | Status | Effort | Dependencies | Impact |
|----------|-----------|------|--------|--------|--------------|---------|
| 🔴 **Critical** | AI Crate | Real-time Coordination System | 🚫 **BLOCKED** | 2-3 weeks | External shared library team | High |
| 🔴 **Critical** | AI Crate | Advanced Conflict Prevention | 🚫 **BLOCKED** | 2-3 weeks | External shared library team | High |
| 🔴 **Critical** | Knowledge Crate | Performance Optimization | 🟡 High Priority | 1-2 weeks | Core system ✅ | High |
| 🔴 **Critical** | Knowledge Crate | Storage & Persistence | 🟡 High Priority | 1-2 weeks | Core system ✅ | High |
| 🟡 **High** | Editor Plugins | IntelliJ Plugin Enhancement | ✅ 85% Complete | 1-2 weeks | Core system ✅ | Medium |
| 🟡 **High** | Editor Plugins | Vim Plugin Development | 📋 Planned | 2-3 weeks | Core system ✅ | Medium |
| 🟡 **High** | CLI Crate | Interactive Mode Enhancements | ✅ Complete | - | Core system ✅ | Medium |
| 🟡 **High** | Query Crate | CQL Query Engine | ✅ Complete | - | Core system ✅ | Medium |
| 🟢 **Medium** | AI Crate | Context Injection Enhancement | ✅ Complete | - | Core system ✅ | Medium |
| 🟢 **Medium** | Editor Plugins | Emacs Plugin Development | 📋 Planned | 3-4 weeks | Core system ✅ | Low |
| 🟢 **Medium** | All Crates | Documentation & Testing | Ongoing | 1-2 weeks | Core system ✅ | Medium |
| 🟢 **Medium** | All Crates | Security Enhancements | Ready | 2-3 weeks | Core system ✅ | High |
| 🔵 **Low** | All Crates | Monitoring & Observability | Ready | 1-2 weeks | Core system ✅ | Medium |

### Priority Legend
- 🔴 **Critical**: Blocking issues or core functionality
- 🟡 **High**: Important features for user experience
- 🟢 **Medium**: Nice-to-have features and improvements
- 🔵 **Low**: Future enhancements and optimizations

### Status Legend
- ✅ **Complete**: Fully implemented and functional
- 🟡 **High Priority**: Currently being worked on
- 📋 **Planned**: Scheduled for implementation
- 🚫 **BLOCKED**: Blocked by external dependencies
- Ready: Ready to start implementation

## Overview

The `rhema-query` crate provides advanced querying and analysis capabilities for Rhema, including CQL (Context Query Language), repository analysis, and intelligent search. It enables powerful knowledge discovery and analysis across Rhema-managed projects.

## Features

### 🔍 CQL Query Engine
- **Context Query Language**: Powerful query language for knowledge discovery
- **Semantic Queries**: AI-powered semantic query capabilities
- **Structured Queries**: Structured query support for complex searches
- **Query Optimization**: Intelligent query optimization and caching

### 📊 Repository Analysis
- **Code Analysis**: Analyze code structure, patterns, and dependencies
- **Architecture Analysis**: Analyze system architecture and design patterns
- **Dependency Analysis**: Analyze project dependencies and relationships
- **Impact Analysis**: Analyze the impact of changes across the codebase

### 🔎 Advanced Search
- **Full-Text Search**: Comprehensive full-text search across all content
- **Semantic Search**: AI-powered semantic search capabilities
- **Hybrid Search**: Combine multiple search strategies for optimal results
- **Search Suggestions**: Intelligent search query suggestions

### 📈 Query Performance
- **Query Optimization**: Optimize query performance and execution
- **Result Caching**: Cache query results for improved performance
- **Parallel Execution**: Parallel query execution for large datasets
- **Performance Monitoring**: Monitor and analyze query performance

### 🔄 Query Integration
- **CLI Integration**: Seamless integration with Rhema CLI
- **API Integration**: RESTful API for query operations
- **SDK Integration**: SDK for programmatic query access
- **Plugin System**: Extensible plugin system for custom queries

## Architecture

```
rhema-query/
├── query.rs          # CQL query engine
├── repo_analysis.rs  # Repository analysis
├── search.rs         # Search functionality
├── optimization.rs   # Query optimization
├── caching.rs        # Result caching
└── lib.rs            # Library entry point
```

## Usage

### CQL Queries

```rust
use rhema_query::query::QueryEngine;

let query_engine = QueryEngine::new();

// Execute a CQL query
let results = query_engine.execute("
    SELECT todos, decisions 
    FROM scope('user-service') 
    WHERE status = 'pending' 
    AND priority > 5
")?;

// Semantic query
let semantic_results = query_engine.semantic_query(
    "authentication patterns and security best practices"
)?;

// Structured query
let structured_results = query_engine.structured_query(Query {
    scope: "user-service".to_string(),
    types: vec!["todos", "decisions", "patterns"],
    filters: vec![
        Filter::Status("pending".to_string()),
        Filter::Priority(GreaterThan(5)),
    ],
    limit: Some(10),
})?;
```

### Repository Analysis

```rust
use rhema_query::repo_analysis::RepositoryAnalyzer;

let analyzer = RepositoryAnalyzer::new();

// Analyze code structure
let structure = analyzer.analyze_structure("src/")?;

// Analyze dependencies
let dependencies = analyzer.analyze_dependencies()?;

// Analyze architecture
let architecture = analyzer.analyze_architecture()?;

// Analyze impact of changes
let impact = analyzer.analyze_impact("src/auth/mod.rs")?;
```

### Search Operations

```rust
use rhema_query::search::SearchEngine;

let search_engine = SearchEngine::new();

// Full-text search
let text_results = search_engine.full_text_search("JWT authentication")?;

// Semantic search
let semantic_results = search_engine.semantic_search("user authentication")?;

// Hybrid search
let hybrid_results = search_engine.hybrid_search(
    "authentication",
    SearchOptions {
        semantic_weight: 0.7,
        keyword_weight: 0.3,
        limit: 20,
    }
)?;

// Get search suggestions
let suggestions = search_engine.get_suggestions("auth")?;
```

### Query Optimization

```rust
use rhema_query::optimization::QueryOptimizer;

let optimizer = QueryOptimizer::new();

// Optimize query
let optimized_query = optimizer.optimize(&original_query)?;

// Get query plan
let plan = optimizer.get_query_plan(&query)?;

// Analyze query performance
let performance = optimizer.analyze_performance(&query)?;
```

## CQL Language Reference

### Basic Queries

```sql
-- Select all todos from a scope
SELECT todos FROM scope('user-service')

-- Select specific fields
SELECT title, status, priority FROM todos WHERE priority > 5

-- Join multiple types
SELECT todos, decisions FROM scope('auth-module') WHERE status = 'pending'
```

### Advanced Queries

```sql
-- Semantic search
SELECT * FROM scope('user-service') WHERE content CONTAINS "authentication patterns"

-- Time-based queries
SELECT * FROM todos WHERE created_at > '2024-01-01' AND status = 'pending'

-- Aggregation queries
SELECT COUNT(*) as total_todos, AVG(priority) as avg_priority 
FROM todos 
GROUP BY status
```

### Repository Analysis Queries

```sql
-- Analyze code structure
ANALYZE STRUCTURE FROM scope('src/')

-- Find dependencies
FIND DEPENDENCIES FOR 'user-service'

-- Analyze impact
ANALYZE IMPACT OF 'src/auth/mod.rs'
```

## Configuration

### Query Engine Configuration

```yaml
# .rhema/query.yaml
query:
  engine:
    cql:
      enabled: true
      max_query_time: 30s
      max_results: 1000
    
    semantic:
      enabled: true
      model: "sentence-transformers"
      similarity_threshold: 0.7
    
    optimization:
      enabled: true
      cache_size: "1GB"
      parallel_execution: true
```

### Repository Analysis Configuration

```yaml
query:
  analysis:
    code_analysis:
      enabled: true
      languages: ["rust", "python", "javascript"]
      patterns: ["design_patterns", "anti_patterns"]
    
    dependency_analysis:
      enabled: true
      depth: 3
      include_dev: false
    
    impact_analysis:
      enabled: true
      max_depth: 5
      include_tests: true
```

### Search Configuration

```yaml
query:
  search:
    full_text:
      enabled: true
      index_type: "inverted"
      stemming: true
    
    semantic:
      enabled: true
      model: "sentence-transformers"
      embedding_cache: true
    
    hybrid:
      enabled: true
      semantic_weight: 0.7
      keyword_weight: 0.3
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **rhema-knowledge**: Knowledge management
- **serde**: Serialization support
- **tantivy**: Full-text search
- **sentence-transformers**: Semantic search
- **tokio**: Async runtime

## Development Status

### ✅ Completed Features
- Basic CQL query engine
- Repository analysis framework
- Query optimization framework
- Semantic query capabilities

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all query operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 