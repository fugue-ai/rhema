/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::query::{CqlQuery, QueryResult};

/// Cache manager for query results
#[derive(Debug, Clone)]
pub struct CacheManager {
    /// Cache configuration
    config: CacheConfig,
    /// Cache storage
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache statistics
    stats: Arc<Mutex<CacheStats>>,
    /// Cache eviction policy
    eviction_policy: EvictionPolicy,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Maximum cache size in bytes
    pub max_size_bytes: usize,
    /// Default TTL in seconds
    pub default_ttl_secs: u64,
    /// Maximum TTL in seconds
    pub max_ttl_secs: u64,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u8,
    /// Enable cache persistence
    pub enable_persistence: bool,
    /// Persistence file path
    pub persistence_path: Option<String>,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicyType,
    /// Maximum number of cache entries
    pub max_entries: usize,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache key
    pub key: String,
    /// Cached data
    pub data: Value,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last accessed timestamp
    pub last_accessed: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Access count
    pub access_count: u64,
    /// Data size in bytes
    pub size_bytes: usize,
    /// Cache metadata
    pub metadata: HashMap<String, Value>,
    /// Compression flag
    pub compressed: bool,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Cache hit rate
    pub hit_rate: f64,
    /// Total cache size in bytes
    pub total_size_bytes: usize,
    /// Number of cache entries
    pub entry_count: usize,
    /// Cache evictions
    pub evictions: u64,
    /// Cache compressions
    pub compressions: u64,
    /// Cache decompressions
    pub decompressions: u64,
    /// Last cache cleanup
    pub last_cleanup: Option<DateTime<Utc>>,
}

/// Eviction policy
#[derive(Debug, Clone)]
pub struct EvictionPolicy {
    /// Policy type
    pub policy_type: EvictionPolicyType,
    /// Policy parameters
    pub parameters: HashMap<String, Value>,
}

/// Eviction policy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicyType {
    LRU,    // Least Recently Used
    LFU,    // Least Frequently Used
    FIFO,   // First In, First Out
    TTL,    // Time To Live
    Size,   // Size-based
    Hybrid, // Combination of policies
}

/// Cache operation result
#[derive(Debug, Clone)]
pub struct CacheResult<T> {
    /// Whether the operation was successful
    pub success: bool,
    /// Cached data (if found)
    pub data: Option<T>,
    /// Cache metadata
    pub metadata: HashMap<String, Value>,
    /// Operation duration in milliseconds
    pub duration_ms: u64,
    /// Cache hit/miss
    pub cache_hit: bool,
}

/// Cache key generator
#[derive(Debug, Clone)]
pub struct CacheKeyGenerator {
    /// Include query hash in key
    pub include_query_hash: bool,
    /// Include timestamp in key
    pub include_timestamp: bool,
    /// Include scope in key
    pub include_scope: bool,
    /// Custom key prefix
    pub key_prefix: Option<String>,
}

/// Cache compression utilities
#[derive(Debug, Clone)]
pub struct CacheCompression {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level
    pub level: u8,
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Lz4,
    Zstd,
    None,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            default_ttl_secs: 3600, // 1 hour
            max_ttl_secs: 86400, // 24 hours
            enable_compression: true,
            compression_level: 6,
            enable_persistence: false,
            persistence_path: None,
            eviction_policy: EvictionPolicyType::LRU,
            max_entries: 10000,
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
            total_size_bytes: 0,
            entry_count: 0,
            evictions: 0,
            compressions: 0,
            decompressions: 0,
            last_cleanup: None,
        }
    }
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self {
            policy_type: EvictionPolicyType::LRU,
            parameters: HashMap::new(),
        }
    }
}

impl Default for CacheKeyGenerator {
    fn default() -> Self {
        Self {
            include_query_hash: true,
            include_timestamp: false,
            include_scope: true,
            key_prefix: None,
        }
    }
}

impl Default for CacheCompression {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: CompressionAlgorithm::Gzip,
            level: 6,
        }
    }
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new() -> Self {
        Self {
            config: CacheConfig::default(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(CacheStats::default())),
            eviction_policy: EvictionPolicy::default(),
        }
    }

    /// Create cache manager with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        let eviction_policy = EvictionPolicy {
            policy_type: config.eviction_policy.clone(),
            parameters: HashMap::new(),
        };

        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(CacheStats::default())),
            eviction_policy,
        }
    }

    /// Get cached data
    pub async fn get(&self, key: &str) -> RhemaResult<CacheResult<Value>> {
        let start_time = Instant::now();
        let mut cache_hit = false;
        let mut data = None;
        let mut metadata = HashMap::new();

        if self.config.enabled {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(key) {
                // Check if entry is expired
                if Utc::now() < entry.expires_at {
                    cache_hit = true;
                    data = Some(entry.data.clone());
                    metadata = entry.metadata.clone();

                    // Update access statistics
                    self.update_access_stats(key, true).await;
                } else {
                    // Entry is expired, remove it
                    drop(cache);
                    self.remove(key).await?;
                }
            } else {
                self.update_access_stats(key, false).await;
            }
        }

        let duration = start_time.elapsed().as_millis() as u64;

        Ok(CacheResult {
            success: true,
            data,
            metadata,
            duration_ms: duration,
            cache_hit,
        })
    }

    /// Store data in cache
    pub async fn set(&self, key: &str, data: Value, ttl_secs: Option<u64>) -> RhemaResult<CacheResult<()>> {
        let start_time = Instant::now();

        if !self.config.enabled {
            return Ok(CacheResult {
                success: true,
                data: None,
                metadata: HashMap::new(),
                duration_ms: start_time.elapsed().as_millis() as u64,
                cache_hit: false,
            });
        }

        let ttl = ttl_secs.unwrap_or(self.config.default_ttl_secs).min(self.config.max_ttl_secs);
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl as i64);
        let size_bytes = self.calculate_data_size(&data);

        // Check if we need to evict entries
        self.check_eviction(size_bytes).await?;

        let mut compressed_data = data.clone();
        let mut compressed = false;

        // Apply compression if enabled
        if self.config.enable_compression && size_bytes > 1024 {
            compressed_data = self.compress_data(&data).await?;
            compressed = true;
        }

        let entry = CacheEntry {
            key: key.to_string(),
            data: compressed_data,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            expires_at,
            access_count: 0,
            size_bytes,
            metadata: HashMap::new(),
            compressed,
        };

        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), entry);

        // Update statistics
        self.update_set_stats(size_bytes, compressed).await;

        let duration = start_time.elapsed().as_millis() as u64;

        Ok(CacheResult {
            success: true,
            data: None,
            metadata: HashMap::new(),
            duration_ms: duration,
            cache_hit: false,
        })
    }

    /// Remove data from cache
    pub async fn remove(&self, key: &str) -> RhemaResult<CacheResult<()>> {
        let start_time = Instant::now();

        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.remove(key) {
            // Update statistics
            self.update_remove_stats(entry.size_bytes).await;
        }

        let duration = start_time.elapsed().as_millis() as u64;

        Ok(CacheResult {
            success: true,
            data: None,
            metadata: HashMap::new(),
            duration_ms: duration,
            cache_hit: false,
        })
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> RhemaResult<CacheResult<()>> {
        let start_time = Instant::now();

        let mut cache = self.cache.write().await;
        let entry_count = cache.len();
        cache.clear();

        // Reset statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.entry_count = 0;
            stats.total_size_bytes = 0;
        }

        let duration = start_time.elapsed().as_millis() as u64;

        Ok(CacheResult {
            success: true,
            data: None,
            metadata: HashMap::new(),
            duration_ms: duration,
            cache_hit: false,
        })
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> RhemaResult<CacheStats> {
        self.stats.lock()
            .map(|guard| guard.clone())
            .map_err(|_| RhemaError::Internal("Failed to lock cache stats".to_string()))
    }

    /// Clean up expired entries
    pub async fn cleanup(&self) -> RhemaResult<usize> {
        let mut removed_count = 0;
        let now = Utc::now();

        let mut cache = self.cache.write().await;
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, entry)| entry.expires_at < now)
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            if let Some(entry) = cache.remove(&key) {
                removed_count += 1;
                self.update_remove_stats(entry.size_bytes).await;
            }
        }

        // Update last cleanup time
        if let Ok(mut stats) = self.stats.lock() {
            stats.last_cleanup = Some(now);
        }

        Ok(removed_count)
    }

    /// Generate cache key for query
    pub fn generate_query_key(&self, query: &CqlQuery, scope: Option<&str>) -> String {
        let mut key_parts = Vec::new();

        // Add custom prefix if configured
        if let Some(prefix) = &self.config.persistence_path {
            key_parts.push(prefix.clone());
        }

        // Add scope if specified
        if let Some(scope_name) = scope {
            key_parts.push(format!("scope:{}", scope_name));
        }

        // Add query target
        key_parts.push(format!("target:{}", query.target));

        // Add YAML path if specified
        if let Some(yaml_path) = &query.yaml_path {
            key_parts.push(format!("path:{}", yaml_path));
        }

        // Add conditions hash
        if !query.conditions.is_empty() {
            let conditions_hash = self.hash_conditions(&query.conditions);
            key_parts.push(format!("conditions:{}", conditions_hash));
        }

        // Add ordering
        if let Some(order_by) = &query.order_by {
            let order_hash = self.hash_order_by(order_by);
            key_parts.push(format!("order:{}", order_hash));
        }

        // Add limit/offset
        if let Some(limit) = query.limit {
            key_parts.push(format!("limit:{}", limit));
        }
        if let Some(offset) = query.offset {
            key_parts.push(format!("offset:{}", offset));
        }

        // Join parts with separator
        key_parts.join("::")
    }

    /// Check if cache needs eviction
    async fn check_eviction(&self, new_entry_size: usize) -> RhemaResult<()> {
        let cache = self.cache.read().await;
        let current_size = cache.values().map(|entry| entry.size_bytes).sum::<usize>();
        let current_count = cache.len();

        if current_size + new_entry_size > self.config.max_size_bytes || 
           current_count >= self.config.max_entries {
            drop(cache);
            self.evict_entries(new_entry_size).await?;
        }

        Ok(())
    }

    /// Evict entries based on policy
    async fn evict_entries(&self, required_space: usize) -> RhemaResult<()> {
        let mut cache = self.cache.write().await;
        let mut entries: Vec<(String, CacheEntry)> = cache.drain().collect();

        match self.eviction_policy.policy_type {
            EvictionPolicyType::LRU => {
                // Sort by last accessed time (oldest first)
                entries.sort_by(|a, b| a.1.last_accessed.cmp(&b.1.last_accessed));
            }
            EvictionPolicyType::LFU => {
                // Sort by access count (lowest first)
                entries.sort_by(|a, b| a.1.access_count.cmp(&b.1.access_count));
            }
            EvictionPolicyType::FIFO => {
                // Sort by creation time (oldest first)
                entries.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
            }
            EvictionPolicyType::TTL => {
                // Sort by expiration time (expiring first)
                entries.sort_by(|a, b| a.1.expires_at.cmp(&b.1.expires_at));
            }
            EvictionPolicyType::Size => {
                // Sort by size (largest first)
                entries.sort_by(|a, b| b.1.size_bytes.cmp(&a.1.size_bytes));
            }
            EvictionPolicyType::Hybrid => {
                // Combine LRU and size
                entries.sort_by(|a, b| {
                    let a_score = a.1.access_count as f64 / a.1.size_bytes as f64;
                    let b_score = b.1.access_count as f64 / b.1.size_bytes as f64;
                    a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        // Calculate current size
        let mut current_size: usize = 0;
        let mut evicted_count = 0;

        // Evict entries until we have enough space
        for (key, entry) in entries {
            if current_size + required_space <= self.config.max_size_bytes &&
               cache.len() - evicted_count < self.config.max_entries {
                cache.insert(key, entry);
                current_size += entry.size_bytes;
            } else {
                evicted_count += 1;
                self.update_remove_stats(entry.size_bytes).await;
            }
        }

        // Update eviction statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.evictions += evicted_count as u64;
        }

        Ok(())
    }

    /// Update access statistics
    async fn update_access_stats(&self, key: &str, hit: bool) {
        if let Ok(mut stats) = self.stats.lock() {
            if hit {
                stats.hits += 1;
            } else {
                stats.misses += 1;
            }

            let total = stats.hits + stats.misses;
            if total > 0 {
                stats.hit_rate = stats.hits as f64 / total as f64;
            }
        }

        // Update entry access count and timestamp
        if hit {
            let mut cache = self.cache.write().await;
            if let Some(entry) = cache.get_mut(key) {
                entry.access_count += 1;
                entry.last_accessed = Utc::now();
            }
        }
    }

    /// Update set statistics
    async fn update_set_stats(&self, size_bytes: usize, compressed: bool) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_size_bytes += size_bytes;
            stats.entry_count += 1;
            if compressed {
                stats.compressions += 1;
            }
        }
    }

    /// Update remove statistics
    async fn update_remove_stats(&self, size_bytes: usize) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_size_bytes = stats.total_size_bytes.saturating_sub(size_bytes);
            stats.entry_count = stats.entry_count.saturating_sub(1);
        }
    }

    /// Calculate data size in bytes
    fn calculate_data_size(&self, data: &Value) -> usize {
        // Simple size estimation
        match data {
            Value::String(s) => s.len(),
            Value::Number(n) => n.to_string().len(),
            Value::Bool(_) => 1,
            Value::Null => 0,
            Value::Sequence(seq) => seq.iter().map(|v| self.calculate_data_size(v)).sum(),
            Value::Mapping(map) => map.iter().map(|(k, v)| k.len() + self.calculate_data_size(v)).sum(),
        }
    }

    /// Compress data
    async fn compress_data(&self, data: &Value) -> RhemaResult<Value> {
        // For now, return the original data
        // TODO: Implement actual compression
        Ok(data.clone())
    }

    /// Hash conditions for cache key
    fn hash_conditions(&self, conditions: &[crate::query::Condition]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        for condition in conditions {
            condition.field.hash(&mut hasher);
            format!("{:?}", condition.operator).hash(&mut hasher);
            format!("{:?}", condition.value).hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }

    /// Hash order by for cache key
    fn hash_order_by(&self, order_by: &[crate::query::OrderBy]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        for order in order_by {
            order.field.hash(&mut hasher);
            format!("{:?}", order.direction).hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
} 