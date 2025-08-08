use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore};

/// Cache entry for dependency analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// Cached data
    pub data: T,
    /// When the entry was created
    pub created_at: DateTime<Utc>,
    /// When the entry expires
    pub expires_at: DateTime<Utc>,
    /// Number of times accessed
    pub access_count: u64,
    /// Last accessed time
    pub last_accessed: DateTime<Utc>,
}

/// Performance-optimized cache for dependency analysis
#[derive(Clone)]
pub struct DependencyCache {
    /// Cache storage
    cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<RwLock<CacheStatistics>>,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Default TTL in seconds
    pub default_ttl: u64,
    /// Maximum cache size
    pub max_size: usize,
    /// Enable compression
    pub enable_compression: bool,
    /// Enable statistics
    pub enable_statistics: bool,
    /// Cleanup interval in seconds
    pub cleanup_interval: u64,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStatistics {
    /// Total hits
    pub hits: u64,
    /// Total misses
    pub misses: u64,
    /// Total sets
    pub sets: u64,
    /// Total evictions
    pub evictions: u64,
    /// Current cache size
    pub current_size: usize,
    /// Hit rate percentage
    pub hit_rate: f64,
}

impl DependencyCache {
    /// Create a new cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a new cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        let cache = Arc::new(RwLock::new(HashMap::new()));
        let stats = Arc::new(RwLock::new(CacheStatistics::default()));

        // Start cleanup task
        let cache_clone = cache.clone();
        let stats_clone = stats.clone();
        let config_clone = config.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(config_clone.cleanup_interval));
            loop {
                interval.tick().await;
                Self::cleanup_expired_entries(&cache_clone, &stats_clone).await;
            }
        });

        Self {
            cache,
            config,
            stats,
        }
    }

    /// Get a value from cache
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        if let Some(entry) = cache.get(key) {
            if Utc::now() < entry.expires_at {
                // Cache hit
                stats.hits += 1;
                
                // Deserialize the data first
                let data: T = serde_json::from_value(entry.data.clone())?;
                
                // Then update access count and last accessed time
                if let Some(entry_mut) = cache.get_mut(key) {
                    entry_mut.access_count += 1;
                    entry_mut.last_accessed = Utc::now();
                }
                
                Ok(Some(data))
            } else {
                // Expired entry
                cache.remove(key);
                stats.misses += 1;
                Ok(None)
            }
        } else {
            // Cache miss
            stats.misses += 1;
            Ok(None)
        }
    }

    /// Set a value in cache
    pub async fn set<T>(&self, key: String, value: T, ttl: Option<u64>) -> Result<()>
    where
        T: Serialize,
    {
        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl as i64);
        
        let entry = CacheEntry {
            data: serde_json::to_value(value)?,
            created_at: Utc::now(),
            expires_at,
            access_count: 0,
            last_accessed: Utc::now(),
        };

        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        // Check if we need to evict entries
        if cache.len() >= self.config.max_size {
            Self::evict_least_used(&mut cache, &mut stats);
        }

        cache.insert(key, entry);
        stats.sets += 1;
        stats.current_size = cache.len();

        Ok(())
    }

    /// Remove a value from cache
    pub async fn remove(&self, key: &str) -> Result<bool> {
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        let removed = cache.remove(key).is_some();
        if removed {
            stats.current_size = cache.len();
        }

        Ok(removed)
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        cache.clear();
        stats.current_size = 0;

        Ok(())
    }

    /// Get cache statistics
    pub async fn get_statistics(&self) -> CacheStatistics {
        let stats = self.stats.read().await;
        let total_requests = stats.hits + stats.misses;
        
        let mut stats_clone = stats.clone();
        if total_requests > 0 {
            stats_clone.hit_rate = (stats.hits as f64 / total_requests as f64) * 100.0;
        }

        stats_clone
    }

    /// Cleanup expired entries
    async fn cleanup_expired_entries(
        cache: &Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
        stats: &Arc<RwLock<CacheStatistics>>,
    ) {
        let now = Utc::now();
        let mut cache = cache.write().await;
        let mut stats = stats.write().await;

        let initial_size = cache.len();
        cache.retain(|_, entry| entry.expires_at > now);
        let final_size = cache.len();
        
        stats.evictions += (initial_size - final_size) as u64;
        stats.current_size = final_size;
    }

    /// Evict least used entries
    fn evict_least_used(
        cache: &mut HashMap<String, CacheEntry<serde_json::Value>>,
        stats: &mut CacheStatistics,
    ) {
        // Find the least used entry
        let least_used = cache
            .iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(key, _)| key.clone());

        if let Some(key) = least_used {
            cache.remove(&key);
            stats.evictions += 1;
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: 3600, // 1 hour
            max_size: 1000,
            enable_compression: false,
            enable_statistics: true,
            cleanup_interval: 300, // 5 minutes
        }
    }
}

/// Parallel processing engine for dependency operations
pub struct ParallelProcessor {
    /// Maximum number of concurrent tasks
    max_concurrent: usize,
    /// Task timeout
    timeout: Duration,
}

impl ParallelProcessor {
    /// Create a new parallel processor
    pub fn new() -> Self {
        Self::with_config(ParallelConfig::default())
    }

    /// Create a new parallel processor with configuration
    pub fn with_config(config: ParallelConfig) -> Self {
        Self {
            max_concurrent: config.max_concurrent,
            timeout: config.timeout,
        }
    }

    /// Process dependencies in parallel
    pub async fn process_dependencies<F, T>(
        &self,
        dependencies: Vec<String>,
        processor: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(String) -> tokio::task::JoinHandle<Result<T>> + Send + Sync + 'static,
        T: Send + 'static,
    {
        let mut handles = Vec::new();
        let mut results = Vec::new();

        // Start processing tasks
        for dependency in dependencies {
            if handles.len() >= self.max_concurrent {
                // Wait for a task to complete
                if let Some(handle) = handles.pop() {
                    match handle.await {
                        Ok(Ok(value)) => results.push(value),
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(Error::Internal(format!("Task join error: {}", e))),
                    }
                }
            }

            let handle = processor(dependency);
            handles.push(tokio::spawn(async move {
                tokio::time::timeout(std::time::Duration::from_secs(30), handle).await
                    .map_err(|_| Error::Timeout("Task timeout".to_string()))?
                    .map_err(|e| Error::Internal(format!("Task error: {}", e)))
            }));
        }

        // Wait for remaining tasks
        for handle in handles {
            match handle.await {
                Ok(Ok(Ok(value))) => results.push(Ok(value)),
                Ok(Ok(Err(e))) => results.push(Err(e)),
                Ok(Err(e)) => results.push(Err(Error::Internal(format!("Task timeout: {}", e)))),
                Err(e) => results.push(Err(Error::Internal(format!("Task join error: {}", e)))),
            }
        }

        // Collect results, returning first error if any
        let mut final_results = Vec::new();
        for result in results {
            match result {
                Ok(value) => final_results.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(final_results)
    }

    /// Process health checks in parallel
    pub async fn process_health_checks(
        &self,
        dependencies: Vec<String>,
        health_checker: impl Fn(String) -> tokio::task::JoinHandle<Result<HealthStatus>> + Send + Sync + 'static,
    ) -> Result<Vec<(String, HealthStatus)>> {
        let mut results = Vec::new();
        for dep in dependencies {
            let handle = health_checker(dep.clone());
            match handle.await {
                Ok(Ok(status)) => results.push((dep, status)),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(Error::Internal(format!("Task join error: {}", e))),
            }
        }
        Ok(results)
    }

    /// Process impact analysis in parallel
    pub async fn process_impact_analysis(
        &self,
        dependencies: Vec<String>,
        impact_analyzer: impl Fn(String) -> tokio::task::JoinHandle<Result<ImpactScore>> + Send + Sync + 'static,
    ) -> Result<Vec<(String, ImpactScore)>> {
        let mut results = Vec::new();
        for dep in dependencies {
            let handle = impact_analyzer(dep.clone());
            match handle.await {
                Ok(Ok(score)) => results.push((dep, score)),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(Error::Internal(format!("Task join error: {}", e))),
            }
        }
        Ok(results)
    }
}

/// Parallel processing configuration
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Maximum number of concurrent tasks
    pub max_concurrent: usize,
    /// Task timeout
    pub timeout: Duration,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_concurrent: num_cpus::get(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// Memory optimization utilities
pub struct MemoryOptimizer {
    /// Memory usage threshold
    threshold: usize,
    /// Enable garbage collection
    enable_gc: bool,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new() -> Self {
        Self::with_config(MemoryConfig::default())
    }

    /// Create a new memory optimizer with configuration
    pub fn with_config(config: MemoryConfig) -> Self {
        Self {
            threshold: config.threshold,
            enable_gc: config.enable_gc,
        }
    }

    /// Check if memory usage is above threshold
    pub fn is_memory_pressure(&self) -> bool {
        if let Ok(usage) = self.get_memory_usage() {
            usage > self.threshold
        } else {
            false
        }
    }

    /// Get current memory usage in bytes
    pub fn get_memory_usage(&self) -> Result<usize> {
        // This is a simplified implementation
        // In a real implementation, you would use system-specific APIs
        Ok(0) // Placeholder
    }

    /// Optimize memory usage
    pub async fn optimize_memory(&self) -> Result<()> {
        if self.enable_gc {
            // Trigger garbage collection
            // This is a simplified implementation
        }

        Ok(())
    }

    /// Monitor memory usage
    pub async fn monitor_memory(&self) -> Result<MemoryMetrics> {
        let usage = self.get_memory_usage()?;
        let pressure = self.is_memory_pressure();

        Ok(MemoryMetrics {
            usage,
            threshold: self.threshold,
            pressure,
            timestamp: Utc::now(),
        })
    }
}

/// Memory configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Memory usage threshold in bytes
    pub threshold: usize,
    /// Enable garbage collection
    pub enable_gc: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            threshold: 1024 * 1024 * 100, // 100MB
            enable_gc: true,
        }
    }
}

/// Memory usage metrics
#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    /// Current memory usage in bytes
    pub usage: usize,
    /// Memory threshold in bytes
    pub threshold: usize,
    /// Whether memory pressure is detected
    pub pressure: bool,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Query optimization for dependency operations
pub struct QueryOptimizer {
    /// Enable query caching
    enable_caching: bool,
    /// Query timeout
    timeout: Duration,
    /// Cache for query results
    query_cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
}

impl QueryOptimizer {
    /// Create a new query optimizer
    pub fn new() -> Self {
        Self::with_config(QueryConfig::default())
    }

    /// Create a new query optimizer with configuration
    pub fn with_config(config: QueryConfig) -> Self {
        Self {
            enable_caching: config.enable_caching,
            timeout: config.timeout,
            query_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Optimize a query
    pub async fn optimize_query<F, T>(&self, query: &str, executor: F) -> Result<T>
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Serialize + for<'de> Deserialize<'de> + Send + 'static,
    {
        if self.enable_caching {
            // Check cache first
            let cache_key = format!("query:{}", query);
            if let Some(cached_result) = self.get_cached_result(&cache_key).await? {
                return Ok(cached_result);
            }

            // Execute query with timeout
            let result = tokio::time::timeout(std::time::Duration::from_secs(10), tokio::task::spawn_blocking(executor))
                .await
                .map_err(|_| Error::Timeout("Query timeout".to_string()))?
                .map_err(|e| Error::Internal(format!("Query execution error: {}", e)))??;

            // Cache the result
            self.cache_result(&cache_key, &result).await?;

            Ok(result)
        } else {
            // Execute query without caching
            tokio::time::timeout(self.timeout, tokio::task::spawn_blocking(executor))
                .await
                .map_err(|_| Error::Timeout("Query timeout".to_string()))?
                .map_err(|e| Error::Internal(format!("Query execution error: {}", e)))?
        }
    }

    /// Get cached result
    async fn get_cached_result<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let cache = self.query_cache.read().await;
        if let Some(entry) = cache.get(key) {
            if Utc::now() < entry.expires_at {
                let data: T = serde_json::from_value(entry.data.clone())?;
                Ok(Some(data))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Cache result
    async fn cache_result<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let entry = CacheEntry {
            data: serde_json::to_value(value)?,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::seconds(300), // 5 minutes
            access_count: 0,
            last_accessed: Utc::now(),
        };

        let mut cache = self.query_cache.write().await;
        cache.insert(key.to_string(), entry);

        Ok(())
    }
}

/// Query configuration
#[derive(Debug, Clone)]
pub struct QueryConfig {
    /// Enable query caching
    pub enable_caching: bool,
    /// Query timeout
    pub timeout: Duration,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            timeout: Duration::from_secs(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = DependencyCache::new();
        
        // Test set and get
        cache.set("test_key".to_string(), "test_value".to_string(), None).await.unwrap();
        let result: Option<String> = cache.get("test_key").await.unwrap();
        assert_eq!(result, Some("test_value".to_string()));
        
        // Test cache miss
        let result: Option<String> = cache.get("nonexistent_key").await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_parallel_processor() {
        let processor = ParallelProcessor::new();
        let dependencies = vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()];
        
        let results = processor.process_dependencies(dependencies, |dep| {
            tokio::spawn(async move {
                Ok::<String, Error>(format!("processed_{}", dep))
            })
        }).await.unwrap();
        
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_query_optimizer() {
        let optimizer = QueryOptimizer::new();
        
        let result: String = optimizer.optimize_query("test_query", || {
            Ok("query_result".to_string())
        }).await.unwrap();
        
        assert_eq!(result, "query_result");
    }
} 