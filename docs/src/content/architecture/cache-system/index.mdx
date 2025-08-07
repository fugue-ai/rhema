# Rhema Cache Directory System

The Rhema Cache Directory System provides centralized caching for implementation guides, temporary scripts, and other runtime artifacts, enabling efficient resource management and performance optimization across the Rhema ecosystem.

## Overview

The cache system implements a `.rhema/cache` directory structure that exists only in the root scope, providing:

- **Centralized Caching**: Single location for all cached resources across scopes
- **Structured Organization**: Organized subdirectories for different content types
- **Automatic Cleanup**: Intelligent lifecycle management and cleanup policies
- **Performance Optimization**: Efficient resource utilization and access patterns
- **Cross-Scope Access**: Seamless access from any scope in the project
- **CLI Integration**: Comprehensive command-line tools for cache management

## Architecture

### Core Components

The cache system consists of several key components:

```rust
// Unified cache management system
pub struct UnifiedCacheManager {
    memory_cache: Arc<SemanticMemoryCache>,
    disk_cache: Arc<SemanticDiskCache>,
    config: UnifiedCacheConfig,
}

// Cache directory structure
pub struct CacheDirectorySystem {
    root_cache_dir: PathBuf,
    subdirectories: HashMap<String, CacheSubdirectory>,
    config: CacheConfig,
    monitor: CacheMonitor,
    optimizer: CacheOptimizer,
}
```

### Directory Structure

The cache system follows a structured directory organization:

```
.rhema/
├── cache/
│   ├── implementation/     # Implementation guides and documentation
│   │   ├── guides/        # Generated implementation guides
│   │   ├── templates/     # Cached template definitions
│   │   ├── examples/      # Cached example implementations
│   │   └── metadata.json  # Cache metadata and indexing
│   ├── scripts/           # Temporary scripts and utilities
│   │   ├── generated/     # AI-generated scripts
│   │   ├── workflows/     # Workflow execution scripts
│   │   ├── migrations/    # Migration and transformation scripts
│   │   └── cleanup/       # Cleanup and maintenance scripts
│   ├── artifacts/         # Other runtime artifacts
│   │   ├── downloads/     # Downloaded resources
│   │   ├── builds/        # Build artifacts
│   │   └── temp/          # General temporary files
│   └── cache.json         # Global cache configuration and state
```

### Cache Configuration Schema

```json
{
  "version": "1.0",
  "root_scope_only": true,
  "subdirectories": {
    "implementation": {
      "description": "Implementation guides and documentation",
      "max_size": "1GB",
      "cleanup_policy": "lru",
      "retention_days": 30
    },
    "scripts": {
      "description": "Temporary scripts and utilities",
      "max_size": "500MB",
      "cleanup_policy": "age",
      "retention_days": 7
    },
    "artifacts": {
      "description": "Other runtime artifacts",
      "max_size": "2GB",
      "cleanup_policy": "lru",
      "retention_days": 14
    }
  },
  "global_settings": {
    "enable_compression": true,
    "compression_algorithm": "zstd",
    "enable_monitoring": true,
    "monitoring_interval": 300,
    "auto_cleanup": true,
    "cleanup_interval": 3600
  }
}
```

## Implementation Details

### Cache Entry Management

Cache entries include comprehensive metadata for tracking and optimization:

```rust
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub ttl: Option<u64>,
    pub checksum: String,
    pub size_bytes: usize,
    pub priority: u8,
    pub pinned: bool,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Option<u64>, priority: u8) -> Self {
        let now = Utc::now();
        Self {
            data,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            ttl,
            checksum: String::new(),
            size_bytes: 0,
            priority,
            pinned: false,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let now = Utc::now();
            let created = self.created_at.timestamp() as u64;
            (now.timestamp() as u64) > created + ttl
        } else {
            false
        }
    }
    
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}
```

### Invalidation Strategies

Multiple invalidation strategies are supported:

```rust
pub enum InvalidationStrategy {
    TimeBased,      // Time-based expiration
    Lru,           // LRU (Least Recently Used) eviction
    Lfu,           // LFU (Least Frequently Used) eviction
    SizeBased,     // Size-based eviction
    PriorityBased, // Priority-based eviction
    Hybrid,        // Hybrid strategy combining multiple approaches
}

impl CacheManager {
    pub async fn evict_entries(&mut self, strategy: InvalidationStrategy) -> RhemaResult<()> {
        match strategy {
            InvalidationStrategy::TimeBased => self.evict_expired_entries().await?,
            InvalidationStrategy::Lru => self.evict_lru_entries().await?,
            InvalidationStrategy::Lfu => self.evict_lfu_entries().await?,
            InvalidationStrategy::SizeBased => self.evict_size_based().await?,
            InvalidationStrategy::PriorityBased => self.evict_priority_based().await?,
            InvalidationStrategy::Hybrid => self.evict_hybrid().await?,
        }
        Ok(())
    }
}
```

### Semantic Caching

Advanced semantic caching with intelligent indexing:

```rust
pub struct SemanticMemoryCache {
    entries: Arc<DashMap<String, SemanticCacheEntry>>,
    semantic_index: Arc<RwLock<HashMap<String, Vec<String>>>>, // tag -> keys
    config: SemanticCacheConfig,
    eviction_policy: Arc<dyn SemanticEvictionPolicy>,
    stats: Arc<RwLock<CacheStats>>,
}

impl SemanticMemoryCache {
    pub async fn search_semantic(&self, query_tags: &[String], limit: usize) -> KnowledgeResult<Vec<UnifiedCacheResult>> {
        let mut results = Vec::new();
        
        for tag in query_tags {
            if let Some(keys) = self.semantic_index.read().await.get(tag) {
                for key in keys.iter().take(limit) {
                    if let Some(entry) = self.entries.get(key) {
                        results.push(UnifiedCacheResult {
                            key: key.clone(),
                            data: entry.data.clone(),
                            similarity_score: self.calculate_similarity(tag, &entry.tags).await?,
                        });
                    }
                }
            }
        }
        
        // Sort by similarity score
        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        results.truncate(limit);
        
        Ok(results)
    }
}
```

## Usage

### Basic Cache Operations

```rust
use rhema::cache::{UnifiedCacheManager, SemanticCacheEntry, CacheConfig};

// Create cache manager
let config = UnifiedCacheConfig::default();
let mut cache_manager = UnifiedCacheManager::new(config).await?;

// Store cache entry
let entry = SemanticCacheEntry {
    content: "Implementation guide content".to_string(),
    tags: vec!["guide".to_string(), "implementation".to_string()],
    content_type: ContentType::Text,
    compression_algorithm: CompressionAlgorithm::Zstd,
    // ... other fields
};

cache_manager.set("guide:auth-implementation".to_string(), entry).await?;

// Retrieve cache entry
if let Some(result) = cache_manager.get("guide:auth-implementation").await? {
    println!("Retrieved: {}", result.data.content);
}

// Search semantically
let results = cache_manager.search_semantic(&["guide".to_string()], 10).await?;
for result in results {
    println!("Found: {} (score: {:.2})", result.key, result.similarity_score);
}
```

### CLI Integration

```bash
# Initialize cache system
rhema cache init --config cache-config.json

# Store implementation guide
rhema cache store --key "guide:auth" --file auth-guide.md --category implementation

# Retrieve cached item
rhema cache get --key "guide:auth"

# List cached items
rhema cache list --category implementation

# Search cache semantically
rhema cache search --tags "guide,implementation" --limit 10

# Clean up expired items
rhema cache cleanup --strategy time-based

# Monitor cache performance
rhema cache monitor --metrics --alerts

# Optimize cache
rhema cache optimize --auto

# Export cache statistics
rhema cache stats --format json --output cache-stats.json
```

### Configuration

```toml
[cache]
# Directory structure
root_cache_dir = ".rhema/cache"
enable_compression = true
compression_algorithm = "zstd"

# Subdirectory configuration
[cache.subdirectories.implementation]
max_size = "1GB"
cleanup_policy = "lru"
retention_days = 30
enable_indexing = true

[cache.subdirectories.scripts]
max_size = "500MB"
cleanup_policy = "age"
retention_days = 7
enable_compression = true

[cache.subdirectories.artifacts]
max_size = "2GB"
cleanup_policy = "lru"
retention_days = 14
enable_monitoring = true

# Global settings
[cache.global]
enable_monitoring = true
monitoring_interval = 300
auto_cleanup = true
cleanup_interval = 3600
enable_semantic_indexing = true
semantic_similarity_threshold = 0.7
```

## Cache Types

### Implementation Cache

Stores implementation guides and documentation:

```rust
pub struct ImplementationCache {
    guides: HashMap<String, ImplementationGuide>,
    templates: HashMap<String, TemplateDefinition>,
    examples: HashMap<String, ExampleImplementation>,
    metadata: CacheMetadata,
}

pub struct ImplementationGuide {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub usage_count: u64,
    pub effectiveness_score: f64,
}
```

### Scripts Cache

Manages temporary scripts and utilities:

```rust
pub struct ScriptsCache {
    generated: HashMap<String, GeneratedScript>,
    workflows: HashMap<String, WorkflowScript>,
    migrations: HashMap<String, MigrationScript>,
    cleanup: HashMap<String, CleanupScript>,
}

pub struct GeneratedScript {
    pub id: String,
    pub name: String,
    pub content: String,
    pub language: String,
    pub purpose: String,
    pub generated_by: String,
    pub created_at: DateTime<Utc>,
    pub execution_count: u64,
    pub success_rate: f64,
}
```

### Artifacts Cache

Handles other runtime artifacts:

```rust
pub struct ArtifactsCache {
    downloads: HashMap<String, DownloadedResource>,
    builds: HashMap<String, BuildArtifact>,
    temp: HashMap<String, TempFile>,
}

pub struct DownloadedResource {
    pub id: String,
    pub url: String,
    pub local_path: PathBuf,
    pub content_type: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub downloaded_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}
```

## Performance Monitoring

### Cache Statistics

Comprehensive performance metrics:

```rust
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub memory_usage_bytes: u64,
    pub semantic_hit_count: u64,
    pub hit_rate: f64,
    pub avg_access_time_us: u64,
    pub efficiency_score: f64,
}

impl CacheManager {
    pub async fn get_performance_report(&self) -> CachePerformanceReport {
        let stats = self.stats().await;
        
        CachePerformanceReport {
            summary: CachePerformanceSummary {
                overall_efficiency: stats.efficiency_score,
                hit_rate: stats.hit_rate,
                memory_usage: self.calculate_memory_usage_percent(&stats),
                disk_usage: self.calculate_disk_usage_percent(&stats),
                response_time_ms: stats.avg_access_time_us / 1000,
            },
            alerts_count: self.get_alerts_count().await,
            recommendations: self.generate_recommendations(&stats).await,
            timestamp: Utc::now(),
        }
    }
}
```

### Cache Monitoring

Real-time monitoring and alerting:

```rust
pub struct CacheMonitor {
    metrics: Arc<RwLock<CacheMetrics>>,
    alerts: Arc<RwLock<Vec<CacheAlert>>>,
    config: CacheMonitorConfig,
}

impl CacheMonitor {
    pub async fn update_metrics(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<()> {
        let stats = cache_manager.get_cache_stats().await;
        
        let metrics = CacheMetrics {
            current_hit_rate: stats.overall_hit_rate,
            average_response_time_ms: self.calculate_avg_response_time(&stats),
            memory_usage_percent: self.calculate_memory_usage_percent(&stats),
            disk_usage_percent: self.calculate_disk_usage_percent(&stats),
            eviction_rate: self.calculate_eviction_rate(&stats),
            compression_ratio: self.calculate_compression_ratio(&stats),
            cache_efficiency: self.calculate_cache_efficiency(&stats),
            timestamp: Utc::now(),
            historical_metrics: self.update_historical_metrics().await,
        };
        
        *self.metrics.write().await = metrics;
        
        // Check for alerts
        self.check_alerts(&metrics).await?;
        
        Ok(())
    }
}
```

## Cache Optimization

### Automatic Optimization

Intelligent cache optimization based on performance metrics:

```rust
pub struct CacheOptimizer {
    config: CacheOptimizerConfig,
    optimization_history: Arc<RwLock<Vec<OptimizationAction>>>,
}

impl CacheOptimizer {
    pub async fn optimize(&self, cache_manager: &mut UnifiedCacheManager, monitor: &CacheMonitor) -> KnowledgeResult<Vec<OptimizationAction>> {
        let metrics = monitor.get_metrics().await;
        let actions = self.generate_optimization_actions(&metrics, cache_manager).await;
        
        for action in &actions {
            if action.applied {
                self.apply_optimization_action(action, cache_manager).await?;
            }
        }
        
        self.record_optimization_actions(&actions).await;
        
        Ok(actions)
    }
    
    async fn generate_optimization_actions(&self, metrics: &CacheMetrics, _cache_manager: &UnifiedCacheManager) -> Vec<OptimizationAction> {
        let mut actions = Vec::new();
        
        // Adjust eviction policy if hit rate is low
        if metrics.current_hit_rate < 0.7 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::AdjustEvictionPolicy,
                description: "Switch to LRU eviction policy to improve hit rate".to_string(),
                timestamp: Utc::now(),
                performance_impact: 0.1,
                applied: true,
            });
        }
        
        // Enable compression if memory usage is high
        if metrics.memory_usage_percent > 0.8 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::EnableCompression,
                description: "Enable compression to reduce memory usage".to_string(),
                timestamp: Utc::now(),
                performance_impact: 0.05,
                applied: true,
            });
        }
        
        actions
    }
}
```

## Integration

### With Rhema Components

```rust
// Integration with lock file system
impl LockFileCache {
    pub async fn get_lock_file(&self, repo_path: &Path) -> RhemaResult<Option<RhemaLock>> {
        let key = CacheKey::LockFile(repo_path.to_path_buf());
        
        if let Some(data) = self.get_serializable(&key).await? {
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
    
    pub async fn set_lock_file(&self, repo_path: &Path, lock_file: &RhemaLock, ttl: Option<u64>) -> RhemaResult<()> {
        let key = CacheKey::LockFile(repo_path.to_path_buf());
        self.set_serializable(&key, lock_file, ttl, 5).await
    }
}

// Integration with knowledge system
impl KnowledgeCache {
    pub async fn cache_knowledge(&self, scope: &str, knowledge: &Knowledge) -> RhemaResult<()> {
        let key = format!("knowledge:{}", scope);
        let entry = SemanticCacheEntry {
            content: serde_json::to_string(knowledge)?,
            tags: vec!["knowledge".to_string(), scope.to_string()],
            content_type: ContentType::Json,
            // ... other fields
        };
        
        self.cache_manager.set(key, entry).await
    }
}
```

### With External Systems

```rust
// Integration with file system
impl FileSystemCache {
    pub async fn cache_file_content(&self, path: &Path, content: &[u8]) -> RhemaResult<()> {
        let key = format!("file:{}", path.display());
        let entry = SemanticCacheEntry {
            content: String::from_utf8_lossy(content).to_string(),
            tags: vec!["file".to_string(), path.extension().unwrap_or_default().to_string_lossy().to_string()],
            content_type: ContentType::Binary,
            // ... other fields
        };
        
        self.cache_manager.set(key, entry).await
    }
}

// Integration with network resources
impl NetworkCache {
    pub async fn cache_download(&self, url: &str, content: &[u8]) -> RhemaResult<()> {
        let key = format!("download:{}", url);
        let entry = SemanticCacheEntry {
            content: String::from_utf8_lossy(content).to_string(),
            tags: vec!["download".to_string(), "network".to_string()],
            content_type: ContentType::Binary,
            // ... other fields
        };
        
        self.cache_manager.set(key, entry).await
    }
}
```

## Performance Considerations

### Optimization Features

- **Intelligent Caching**: Semantic indexing for efficient retrieval
- **Compression**: Automatic compression to reduce storage requirements
- **Eviction Policies**: Multiple eviction strategies for optimal performance
- **Cache Warming**: Proactive cache population based on access patterns
- **Monitoring**: Real-time performance monitoring and alerting

### Performance Metrics

- **Cache Hit Rate**: > 80% for typical workloads
- **Response Time**: < 10ms for cache hits
- **Storage Efficiency**: 40-60% compression ratio
- **Memory Usage**: < 100MB for typical cache sizes
- **Eviction Rate**: < 5% for well-tuned configurations

## Related Documentation

- **[Cache API](./api.md)** - Detailed API reference
- **[Cache Configuration](./configuration.md)** - Configuration options and tuning
- **[Performance Tuning](./performance.md)** - Optimization and performance guide
- **[Monitoring Guide](./monitoring.md)** - Cache monitoring and alerting
- **[Integration Guide](./integration.md)** - How to integrate with other systems 