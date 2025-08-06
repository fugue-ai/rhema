# Rhema Query Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-query)](https://crates.io/crates/rhema-query)
[![Documentation](https://docs.rs/rhema-query/badge.svg)](https://docs.rs/rhema-query)

Advanced query engine, search capabilities, and repository analysis for the Rhema Protocol. This crate provides powerful CQL (Context Query Language) querying, full-text search, semantic search, and intelligent repository analysis.

## Overview

The `rhema-query` crate is a comprehensive query and search solution for Rhema-managed projects. It provides:

- **CQL Query Engine**: Powerful query language for knowledge discovery and data extraction
- **Advanced Search**: Full-text, semantic, regex, and hybrid search capabilities
- **Repository Analysis**: Intelligent analysis of project structure, technologies, and dependencies
- **LOCOMO Integration**: Specialized queries for performance analysis and optimization tracking
- **Query Provenance**: Detailed tracking of query execution and data lineage
- **Performance Optimization**: Caching, parallel processing, and query optimization

## Features

### 🔍 CQL Query Engine
- **Context Query Language**: SQL-like query language for YAML-based knowledge discovery
- **YAML Path Support**: Direct querying of nested YAML structures using path expressions
- **Conditional Filtering**: Complex WHERE clauses with multiple operators and logical combinations
- **Ordering and Pagination**: ORDER BY, LIMIT, and OFFSET support
- **Query Provenance**: Detailed execution tracking and data lineage information
- **Scope-based Queries**: Query across multiple scopes and repositories

### 🔎 Advanced Search Capabilities
- **Full-Text Search**: Comprehensive text search with relevance scoring
- **Semantic Search**: AI-powered semantic search with similarity matching
- **Regex Search**: Pattern-based search with regex support
- **Hybrid Search**: Combine multiple search strategies for optimal results
- **Fuzzy Matching**: Typo-tolerant search with configurable distance thresholds
- **Search Indexing**: Automatic indexing of documents for fast retrieval
- **Search Suggestions**: Intelligent query completion and suggestions

### 📊 Repository Analysis
- **Technology Detection**: Automatic detection of programming languages, frameworks, and tools
- **Project Classification**: Identify project types (monorepo, microservice, library, etc.)
- **Dependency Analysis**: Analyze project dependencies and relationships
- **Code Quality Assessment**: Evaluate code quality and identify improvement areas
- **Security Analysis**: Detect security patterns and potential vulnerabilities
- **Scope Recommendations**: Generate intelligent scope suggestions

### 🚀 LOCOMO Integration
- **Performance Analysis**: Track and analyze context retrieval performance
- **Quality Assessment**: Evaluate context quality and relevance
- **Optimization Tracking**: Monitor optimization metrics and improvements
- **Trend Analysis**: Analyze performance and quality trends over time
- **Benchmark Comparison**: Compare performance against baselines
- **Validation Reports**: Generate comprehensive validation reports

### ⚡ Performance Features
- **Query Optimization**: Intelligent query planning and optimization
- **Result Caching**: Configurable caching for improved performance
- **Parallel Processing**: Parallel execution for large datasets
- **Performance Monitoring**: Detailed performance metrics and analytics
- **Memory Management**: Efficient memory usage and garbage collection

## Architecture

```
rhema-query/
├── query.rs              # CQL query engine and execution
├── search.rs             # Search engine and indexing
├── repo_analysis.rs      # Repository analysis and technology detection
├── locomo_queries.rs     # LOCOMO-specific query extensions
├── caching.rs            # Result caching and optimization
├── performance.rs        # Performance monitoring and metrics
├── optimization.rs       # Query optimization and planning
└── lib.rs                # Library entry point
```

## Usage

### Basic CQL Queries

```rust
use rhema_query::{execute_query, execute_query_with_provenance};

// Simple query to extract todos from a scope
let results = execute_query(
    repo_path,
    "SELECT todos FROM scope('user-service') WHERE status = 'pending'"
)?;

// Query with provenance tracking
let (results, provenance) = execute_query_with_provenance(
    repo_path,
    "SELECT todos, decisions FROM scope('auth-module') WHERE priority > 5 ORDER BY created_at DESC LIMIT 10"
)?;

// Query with YAML path
let results = execute_query(
    repo_path,
    "SELECT todos.title, todos.description FROM scope('user-service').todos WHERE priority > 3"
)?;
```

### Advanced Search

```rust
use rhema_query::search::{SearchEngine, SearchOptions, SearchType};

let mut search_engine = SearchEngine::new();

// Build search index
search_engine.build_index(repo_path, &scopes).await?;

// Full-text search
let results = search_engine.full_text_search(
    "JWT authentication",
    Some(SearchOptions {
        limit: Some(20),
        case_sensitive: false,
        fuzzy_matching: true,
        ..Default::default()
    })
).await?;

// Regex search
let results = search_engine.regex_search(
    r"auth.*token",
    Some(SearchOptions {
        limit: Some(10),
        case_sensitive: false,
        ..Default::default()
    })
).await?;

// Get search suggestions
let suggestions = search_engine.get_suggestions("auth").await?;
```

### Repository Analysis

```rust
use rhema_query::repo_analysis::RepoAnalysis;

// Analyze repository
let analysis = RepoAnalysis::analyze(repo_path)?;

// Get project type
match analysis.project_type {
    ProjectType::Monorepo => println!("Detected monorepo structure"),
    ProjectType::Microservice => println!("Detected microservice architecture"),
    ProjectType::Library => println!("Detected library project"),
    _ => println!("Other project type"),
}

// Get detected technologies
println!("Languages: {:?}", analysis.languages);
println!("Frameworks: {:?}", analysis.frameworks);
println!("Databases: {:?}", analysis.databases);

// Generate Rhema scope
let scope = analysis.generate_rhema_scope();
```

### LOCOMO Performance Analysis

```rust
use rhema_query::locomo_queries::LocomoQueryExtensions;

let locomo = LocomoQueryExtensions::new();

// Analyze performance
let performance_result = locomo.analyze_performance("user-service", "last_7_days").await?;
println!("Performance score: {}", performance_result.metrics.context_retrieval.average_latency_ms);

// Assess quality
let quality_result = locomo.assess_quality("user-service", "last_30_days").await?;
println!("Quality score: {}", quality_result.metrics.quality_assessment.overall_quality_score);

// Track optimization
let optimization_result = locomo.track_optimization("user-service", "last_week").await?;
println!("Optimization score: {}", optimization_result.metrics.ai_optimization.token_reduction_percentage);
```

## CQL Language Reference

### Basic Query Syntax

```sql
-- Select all todos from a scope
SELECT todos FROM scope('user-service')

-- Select specific fields with YAML path
SELECT todos.title, todos.description FROM scope('user-service')

-- Filter with WHERE clause
SELECT todos FROM scope('user-service') WHERE status = 'pending' AND priority > 5

-- Order results
SELECT todos FROM scope('user-service') ORDER BY created_at DESC

-- Limit results
SELECT todos FROM scope('user-service') LIMIT 10 OFFSET 20
```

### Advanced Queries

```sql
-- Complex conditions
SELECT todos FROM scope('user-service') 
WHERE (status = 'pending' OR status = 'in_progress') 
AND priority >= 3 
AND created_at > '2024-01-01'

-- Multiple scopes
SELECT todos FROM scope('user-service', 'auth-service') 
WHERE priority > 5

-- Nested YAML path queries
SELECT todos.metadata.tags FROM scope('user-service') 
WHERE todos.metadata.priority > 3
```

### Repository Analysis Queries

```sql
-- Analyze project structure
SELECT analysis FROM scope('.')

-- Find dependencies
SELECT dependencies FROM scope('user-service')

-- Get technology stack
SELECT technologies FROM scope('.')
```

## Configuration

### Search Configuration

```yaml
# .rhema/query.yaml
search:
  full_text_enabled: true
  semantic_enabled: true
  hybrid_enabled: true
  regex_enabled: true
  default_limit: 20
  timeout_seconds: 30
  min_similarity_threshold: 0.7
  parallel_processing: true
  max_file_size: 10485760  # 10MB
  included_file_types: ["yaml", "yml", "json", "md", "txt"]
  excluded_file_types: ["log", "tmp"]
  enable_caching: true
  cache_ttl_seconds: 3600
```

### Query Engine Configuration

```yaml
query:
  engine:
    max_query_time: 30s
    max_results: 1000
    enable_provenance: true
    parallel_execution: true
    cache_size: "1GB"
```

## Dependencies

- **rhema-core**: Core Rhema functionality and error handling
- **serde**: Serialization and deserialization
- **serde_yaml**: YAML parsing and manipulation
- **regex**: Regular expression support
- **walkdir**: Directory traversal
- **rayon**: Parallel processing
- **tokio**: Async runtime
- **chrono**: Date and time handling
- **glob**: File pattern matching
- **tracing**: Logging and diagnostics

## Development Status

### ✅ Implemented Features
- Complete CQL query engine with YAML path support
- Full-text search with indexing and relevance scoring
- Regex search with pattern matching
- Repository analysis with technology detection
- LOCOMO integration for performance analysis
- Query provenance and execution tracking
- Performance monitoring and optimization
- Parallel processing and caching
- Comprehensive error handling

### 🔄 Current Capabilities
- **Query Engine**: Fully functional CQL implementation
- **Search**: Full-text, regex, and hybrid search
- **Analysis**: Repository structure and technology analysis
- **Performance**: Caching, optimization, and monitoring
- **Integration**: LOCOMO performance tracking
- **Provenance**: Complete query execution tracking

### 📋 Future Enhancements
- Semantic search with embedding models
- Advanced query optimization
- Distributed search capabilities
- Real-time indexing
- Advanced analytics and reporting

## Contributing

1. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
2. Ensure all query operations are properly tested
3. Run the test suite: `cargo test`
4. Check code quality: `cargo clippy`
5. Format code: `cargo fmt`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 