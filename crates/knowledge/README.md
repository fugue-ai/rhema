# Rhema Knowledge Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-knowledge)](https://crates.io/crates/rhema-knowledge)
[![Documentation](https://docs.rs/rhema-knowledge/badge.svg)](https://docs.rs/rhema-knowledge)

RAG (Retrieval-Augmented Generation), caching, embedding, indexing, search, synthesis, and proactive features for Rhema.

## Overview

The `rhema-knowledge` crate provides advanced knowledge management capabilities for Rhema, including RAG (Retrieval-Augmented Generation), intelligent caching, semantic search, and proactive knowledge synthesis. It serves as the brain of Rhema's knowledge system.

## Features

### 🧠 Knowledge Engine
- **RAG Implementation**: Retrieval-Augmented Generation for AI context enhancement
- **Knowledge Synthesis**: Intelligent synthesis of knowledge from multiple sources
- **Proactive Management**: Anticipate and prepare knowledge based on usage patterns
- **Usage Analysis**: Analyze knowledge usage for optimization

### 🔍 Search and Discovery
- **Semantic Search**: AI-powered semantic search across knowledge artifacts
- **Full-Text Search**: Traditional full-text search capabilities
- **Hybrid Search**: Combine semantic and keyword search for optimal results
- **Search Suggestions**: Intelligent search query suggestions

### 💾 Caching System ✅ COMPLETED
- **Multi-Tier Caching**: Memory and disk-based caching with intelligent eviction
- **Cache Warming**: Proactive cache warming based on access patterns
- **Compression**: Multiple compression algorithms (Zstd, LZ4, Gzip)
- **Persistence**: Cross-restart persistence with state management
- **Monitoring**: Real-time cache performance monitoring and optimization

### 🔤 Embedding and Vector Operations
- **Embedding Generation**: Generate embeddings for text and code
- **Vector Similarity**: Efficient vector similarity search
- **Embedding Caching**: Cache embeddings for reuse and performance
- **Embedding Versioning**: Version embeddings for compatibility

### 📚 Indexing System
- **Incremental Indexing**: Index only changed content for efficiency
- **Index Optimization**: Optimize indexing performance and storage
- **Index Validation**: Validate indexed content integrity
- **Index Monitoring**: Monitor indexing progress and performance

### 🔄 Proactive Features
- **Content Recommendation**: Recommend relevant content based on context
- **Usage Pattern Analysis**: Analyze usage patterns for optimization
- **Predictive Caching**: Predict and cache likely needed content
- **Automatic Indexing**: Automatically index new content

## Architecture

```
rhema-knowledge/
├── engine.rs         # Knowledge engine core
├── cache.rs          # Caching system ✅ COMPLETED
├── embedding.rs      # Embedding operations
├── indexing.rs       # Indexing system
├── search.rs         # Search functionality
├── synthesis.rs      # Knowledge synthesis
├── proactive.rs      # Proactive features
└── utils/            # Utility functions
```

## Usage

### Knowledge Engine

```rust
use rhema_knowledge::engine::KnowledgeEngine;

let engine = KnowledgeEngine::new();

// Process knowledge query
let results = engine.query("user authentication patterns")?;

// Synthesize knowledge
let synthesis = engine.synthesize(&["auth", "security", "patterns"])?;

// Analyze usage patterns
let patterns = engine.analyze_usage_patterns()?;
```

### Caching System ✅ COMPLETED

```rust
use rhema_knowledge::cache::UnifiedCacheManager;

let cache_manager = UnifiedCacheManager::new();

// Cache knowledge item
cache_manager.set("user-auth-patterns", &knowledge_data)?;

// Retrieve from cache
let cached_data = cache_manager.get("user-auth-patterns")?;

// Get cache statistics
let stats = cache_manager.get_statistics()?;
println!("Cache hit rate: {:.2}%", stats.hit_rate * 100.0);
```

### Search Operations

```rust
use rhema_knowledge::search::SearchEngine;

let search_engine = SearchEngine::new();

// Semantic search
let semantic_results = search_engine.semantic_search("authentication")?;

// Full-text search
let text_results = search_engine.full_text_search("JWT")?;

// Hybrid search
let hybrid_results = search_engine.hybrid_search("user auth", 0.7)?;
```

### Embedding Operations

```rust
use rhema_knowledge::embedding::EmbeddingManager;

let embedding_manager = EmbeddingManager::new();

// Generate embedding
let embedding = embedding_manager.generate_embedding("user authentication")?;

// Find similar content
let similar = embedding_manager.find_similar(&embedding, 5)?;

// Cache embedding
embedding_manager.cache_embedding("auth-key", &embedding)?;
```

### Indexing Operations

```rust
use rhema_knowledge::indexing::IndexManager;

let index_manager = IndexManager::new();

// Index content
index_manager.index_content("user-service", &content)?;

// Incremental indexing
index_manager.incremental_index("user-service", &changes)?;

// Get index statistics
let stats = index_manager.get_statistics()?;
```

## Configuration

### Knowledge Engine Configuration

```yaml
# .rhema/knowledge.yaml
knowledge:
  engine:
    rag:
      enabled: true
      model: "gpt-4"
      max_tokens: 1000
    
    synthesis:
      enabled: true
      min_confidence: 0.8
    
    proactive:
      enabled: true
      prediction_window: 24h
```

### Cache Configuration

```yaml
knowledge:
  cache:
    memory:
      max_size: "1GB"
      eviction_policy: "LRU"
    
    disk:
      max_size: "10GB"
      compression: "zstd"
      compression_threshold: "1KB"
    
    warming:
      enabled: true
      patterns: ["recent", "frequent", "semantic"]
```

### Search Configuration

```yaml
knowledge:
  search:
    semantic:
      enabled: true
      model: "sentence-transformers"
      similarity_threshold: 0.7
    
    full_text:
      enabled: true
      index_type: "inverted"
    
    hybrid:
      enabled: true
      semantic_weight: 0.7
      keyword_weight: 0.3
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **serde**: Serialization support
- **tokio**: Async runtime
- **reqwest**: HTTP client for AI services
- **sentence-transformers**: Embedding generation
- **tantivy**: Full-text search
- **faiss**: Vector similarity search

## Development Status

### ✅ Completed Features
- **Cache System**: Complete multi-tier caching with monitoring and optimization
- Basic knowledge engine framework
- Embedding generation and caching
- Search infrastructure

### 🔄 In Progress
- RAG implementation
- Knowledge synthesis
- Proactive features
- Advanced search algorithms

### 📋 Planned Features
- Advanced RAG capabilities
- Knowledge graph integration
- Multi-modal knowledge support
- Performance optimization

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all knowledge operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 