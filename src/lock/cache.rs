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

use crate::{RhemaError, RhemaResult};
use crate::schema::{RhemaLock, LockedScope, LockedDependency, ResolutionStrategy};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use log::{debug, info, warn, error};

/// Cache entry with metadata for tracking and invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// Cached data
    pub data: T,
    /// When this entry was created
    pub created_at: DateTime<Utc>,
    /// When this entry was last accessed
    pub last_accessed: DateTime<Utc>,
    /// Number of times this entry has been accessed
    pub access_count: u64,
    /// Time-to-live in seconds (None = no expiration)
    pub ttl: Option<u64>,
    /// Checksum for integrity verification
    pub checksum: String,
    /// Cache entry size in bytes (approximate)
    pub size_bytes: usize,
    /// Priority level for eviction (higher = less likely to be evicted)
    pub priority: u8,
    /// Whether this entry is pinned (won't be evicted)
    pub pinned: bool,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(data: T, ttl: Option<u64>, priority: u8) -> Self {
        let now = Utc::now();
        Self {
            data,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            ttl,
            checksum: String::new(), // Will be set by cache manager
            size_bytes: 0, // Will be set by cache manager
            priority,
            pinned: false,
        }
    }

    /// Check if this entry has expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let now = Utc::now();
            let created = self.created_at.timestamp() as u64;
            (now.timestamp() as u64) > created + ttl
        } else {
            false
        }
    }

    /// Update access information
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }

    /// Calculate access frequency (accesses per second since creation)
    pub fn access_frequency(&self) -> f64 {
        let now = Utc::now();
        let duration = (now - self.created_at).num_seconds() as f64;
        if duration > 0.0 {
            self.access_count as f64 / duration
        } else {
            0.0
        }
    }
}

/// Cache statistics for monitoring and optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: u64,
    /// Total number of cache misses
    pub misses: u64,
    /// Total number of cache entries
    pub total_entries: usize,
    /// Total cache size in bytes
    pub total_size_bytes: usize,
    /// Maximum cache size in bytes
    pub max_size_bytes: usize,
    /// Number of evictions
    pub evictions: u64,
    /// Number of expired entries
    pub expired_entries: u64,
    /// Average access time in microseconds
    pub avg_access_time_us: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Last cache cleanup time
    pub last_cleanup: Option<DateTime<Utc>>,
    /// Cache efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            total_entries: 0,
            total_size_bytes: 0,
            max_size_bytes: 100 * 1024 * 1024, // 100MB default
            evictions: 0,
            expired_entries: 0,
            avg_access_time_us: 0,
            hit_rate: 0.0,
            last_cleanup: None,
            efficiency_score: 0.0,
        }
    }
}

/// Cache invalidation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidationStrategy {
    /// Time-based expiration
    TimeBased,
    /// LRU (Least Recently Used) eviction
    Lru,
    /// LFU (Least Frequently Used) eviction
    Lfu,
    /// Size-based eviction
    SizeBased,
    /// Priority-based eviction
    PriorityBased,
    /// Hybrid strategy combining multiple approaches
    Hybrid,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size in bytes
    pub max_size_bytes: usize,
    /// Default TTL in seconds
    pub default_ttl: Option<u64>,
    /// Invalidation strategy
    pub invalidation_strategy: InvalidationStrategy,
    /// Whether to enable persistent caching
    pub enable_persistent: bool,
    /// Persistent cache directory
    pub persistent_cache_dir: Option<PathBuf>,
    /// Whether to enable compression for persistent cache
    pub enable_compression: bool,
    /// Maximum number of entries in memory cache
    pub max_entries: usize,
    /// Cleanup interval in seconds
    pub cleanup_interval: u64,
    /// Whether to enable cache statistics
    pub enable_stats: bool,
    /// Whether to enable cache warming
    pub enable_warming: bool,
    /// Cache warming strategy
    pub warming_strategy: WarmingStrategy,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            default_ttl: Some(3600), // 1 hour
            invalidation_strategy: InvalidationStrategy::Hybrid,
            enable_persistent: true,
            persistent_cache_dir: None,
            enable_compression: true,
            max_entries: 10000,
            cleanup_interval: 300, // 5 minutes
            enable_stats: true,
            enable_warming: false,
            warming_strategy: WarmingStrategy::None,
        }
    }
}

/// Cache warming strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarmingStrategy {
    /// No warming
    None,
    /// Warm frequently accessed entries
    FrequentAccess,
    /// Warm based on access patterns
    AccessPattern,
    /// Warm based on dependency relationships
    DependencyBased,
    /// Warm based on time patterns
    TimeBased,
}

/// Cache key types for different lock file data
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CacheKey {
    /// Lock file by repository path
    LockFile(PathBuf),
    /// Scope by path
    Scope(PathBuf),
    /// Dependency by specification
    Dependency(String, String), // (path, version_constraint)
    /// Resolution result by dependencies
    ResolutionResult(Vec<String>),
    /// Validation result by lock file checksum
    ValidationResult(String),
    /// Performance metrics by operation
    PerformanceMetrics(String),
    /// Custom key
    Custom(String),
}

impl CacheKey {
    /// Convert cache key to string for storage
    pub fn to_string(&self) -> String {
        match self {
            CacheKey::LockFile(path) => format!("lock_file:{}", path.display()),
            CacheKey::Scope(path) => format!("scope:{}", path.display()),
            CacheKey::Dependency(path, constraint) => format!("dependency:{}:{}", path, constraint),
            CacheKey::ResolutionResult(deps) => format!("resolution:{}", deps.join(",")),
            CacheKey::ValidationResult(checksum) => format!("validation:{}", checksum),
            CacheKey::PerformanceMetrics(operation) => format!("metrics:{}", operation),
            CacheKey::Custom(key) => format!("custom:{}", key),
        }
    }

    /// Generate checksum for the key
    pub fn checksum(&self) -> String {
        let key_str = self.to_string();
        let mut hasher = Sha256::new();
        hasher.update(key_str.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// In-memory cache implementation
pub struct InMemoryCache {
    entries: HashMap<String, CacheEntry<Vec<u8>>>,
    access_order: VecDeque<String>,
    config: CacheConfig,
    stats: CacheStats,
    last_cleanup: Instant,
}

impl InMemoryCache {
    /// Create a new in-memory cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: HashMap::new(),
            access_order: VecDeque::new(),
            config,
            stats: CacheStats::default(),
            last_cleanup: Instant::now(),
        }
    }

    /// Get a value from cache
    pub fn get(&mut self, key: &CacheKey) -> Option<Vec<u8>> {
        let key_str = key.to_string();
        let start_time = Instant::now();

        // Check if entry exists and is not expired
        let is_expired = self.entries.get(&key_str).map(|entry| entry.is_expired()).unwrap_or(true);
        
        if is_expired {
            if self.entries.remove(&key_str).is_some() {
                self.stats.expired_entries += 1;
            }
            self.stats.misses += 1;
            return None;
        }

        // Now we can safely get a mutable reference
        if let Some(entry) = self.entries.get_mut(&key_str) {
            entry.touch();
            self.stats.hits += 1;
            
            let result = Some(entry.data.clone());
            
            // Update access order and avg access time after we're done with the entry
            let access_time = start_time.elapsed().as_micros() as u64;
            self.update_access_order(&key_str);
            self.update_avg_access_time(access_time);
            
            result
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Set a value in cache
    pub fn set(&mut self, key: CacheKey, value: Vec<u8>, ttl: Option<u64>, priority: u8) {
        let key_str = key.to_string();
        let checksum = key.checksum();
        let size = value.len();

        // Check if we need to evict entries
        if self.should_evict(size) {
            self.evict_entries(size);
        }

        let mut entry = CacheEntry::new(value, ttl.or(self.config.default_ttl), priority);
        entry.checksum = checksum;
        entry.size_bytes = size;

        self.entries.insert(key_str.clone(), entry);
        self.access_order.push_back(key_str);
        self.stats.total_entries = self.entries.len();
        self.stats.total_size_bytes += size;

        // Periodic cleanup
        if self.should_cleanup() {
            self.cleanup();
        }
    }

    /// Remove a value from cache
    pub fn remove(&mut self, key: &CacheKey) -> bool {
        let key_str = key.to_string();
        if let Some(entry) = self.entries.remove(&key_str) {
            self.stats.total_size_bytes -= entry.size_bytes;
            self.stats.total_entries = self.entries.len();
            true
        } else {
            false
        }
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
        self.stats.total_entries = 0;
        self.stats.total_size_bytes = 0;
        self.stats.evictions += 1;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let mut stats = self.stats.clone();
        stats.hit_rate = if stats.hits + stats.misses > 0 {
            stats.hits as f64 / (stats.hits + stats.misses) as f64
        } else {
            0.0
        };
        stats.efficiency_score = self.calculate_efficiency_score();
        stats
    }

    /// Check if cache should evict entries
    fn should_evict(&self, new_size: usize) -> bool {
        self.stats.total_size_bytes + new_size > self.config.max_size_bytes ||
        self.stats.total_entries >= self.config.max_entries
    }

    /// Evict entries based on strategy
    fn evict_entries(&mut self, required_space: usize) {
        let mut freed_space = 0;
        let mut entries_to_remove = Vec::new();

        match self.config.invalidation_strategy {
            InvalidationStrategy::Lru => {
                // Remove least recently used entries
                while freed_space < required_space && !self.access_order.is_empty() {
                    if let Some(key) = self.access_order.pop_front() {
                        if let Some(entry) = self.entries.get(&key) {
                            if !entry.pinned {
                                freed_space += entry.size_bytes;
                                entries_to_remove.push(key);
                            }
                        }
                    }
                }
            }
            InvalidationStrategy::Lfu => {
                // Remove least frequently used entries
                let mut entries: Vec<_> = self.entries.iter().collect();
                entries.sort_by_key(|(_, entry)| entry.access_count);
                
                for (key, entry) in entries {
                    if !entry.pinned && freed_space < required_space {
                        freed_space += entry.size_bytes;
                        entries_to_remove.push(key.clone());
                    }
                }
            }
            InvalidationStrategy::PriorityBased => {
                // Remove lowest priority entries
                let mut entries: Vec<_> = self.entries.iter().collect();
                entries.sort_by_key(|(_, entry)| entry.priority);
                
                for (key, entry) in entries {
                    if !entry.pinned && freed_space < required_space {
                        freed_space += entry.size_bytes;
                        entries_to_remove.push(key.clone());
                    }
                }
            }
            InvalidationStrategy::SizeBased => {
                // Remove largest entries
                let mut entries: Vec<_> = self.entries.iter().collect();
                entries.sort_by_key(|(_, entry)| entry.size_bytes);
                entries.reverse();
                
                for (key, entry) in entries {
                    if !entry.pinned && freed_space < required_space {
                        freed_space += entry.size_bytes;
                        entries_to_remove.push(key.clone());
                    }
                }
            }
            InvalidationStrategy::TimeBased => {
                // Remove expired entries
                let now = Utc::now();
                for (key, entry) in &self.entries {
                    if entry.is_expired() && !entry.pinned {
                        freed_space += entry.size_bytes;
                        entries_to_remove.push(key.clone());
                    }
                }
            }
            InvalidationStrategy::Hybrid => {
                // Combine multiple strategies
                self.evict_expired_entries();
                if freed_space < required_space {
                    self.evict_lru_entries(required_space - freed_space);
                }
            }
        }

        // Remove selected entries
        for key in entries_to_remove {
            if let Some(entry) = self.entries.remove(&key) {
                self.stats.total_size_bytes -= entry.size_bytes;
                self.stats.evictions += 1;
            }
        }
        
        self.stats.total_entries = self.entries.len();
    }

    /// Evict expired entries
    fn evict_expired_entries(&mut self) {
        let mut expired_keys = Vec::new();
        
        for (key, entry) in &self.entries {
            if entry.is_expired() && !entry.pinned {
                expired_keys.push(key.clone());
            }
        }
        
        for key in expired_keys {
            if let Some(entry) = self.entries.remove(&key) {
                self.stats.total_size_bytes -= entry.size_bytes;
                self.stats.expired_entries += 1;
            }
        }
    }

    /// Evict LRU entries
    fn evict_lru_entries(&mut self, required_space: usize) {
        let mut freed_space = 0;
        
        while freed_space < required_space && !self.access_order.is_empty() {
            if let Some(key) = self.access_order.pop_front() {
                if let Some(entry) = self.entries.get(&key) {
                    if !entry.pinned {
                        freed_space += entry.size_bytes;
                        if let Some(entry) = self.entries.remove(&key) {
                            self.stats.total_size_bytes -= entry.size_bytes;
                            self.stats.evictions += 1;
                        }
                    }
                }
            }
        }
    }

    /// Update access order for LRU tracking
    fn update_access_order(&mut self, key: &str) {
        // Remove from current position
        if let Some(pos) = self.access_order.iter().position(|k| k == key) {
            self.access_order.remove(pos);
        }
        // Add to end (most recently used)
        self.access_order.push_back(key.to_string());
    }

    /// Update average access time
    fn update_avg_access_time(&mut self, access_time: u64) {
        let total_accesses = self.stats.hits + self.stats.misses;
        if total_accesses > 0 {
            self.stats.avg_access_time_us = 
                ((self.stats.avg_access_time_us * (total_accesses - 1) as u64) + access_time) / total_accesses as u64;
        }
    }

    /// Check if cleanup is needed
    fn should_cleanup(&self) -> bool {
        self.last_cleanup.elapsed().as_secs() >= self.config.cleanup_interval
    }

    /// Perform periodic cleanup
    fn cleanup(&mut self) {
        self.evict_expired_entries();
        self.last_cleanup = Instant::now();
        self.stats.last_cleanup = Some(Utc::now());
    }

    /// Calculate cache efficiency score
    fn calculate_efficiency_score(&self) -> f64 {
        let hit_rate = self.stats.hit_rate;
        let memory_efficiency = 1.0 - (self.stats.total_size_bytes as f64 / self.stats.max_size_bytes as f64);
        let access_efficiency = if self.stats.avg_access_time_us > 0 {
            1.0 / (self.stats.avg_access_time_us as f64 / 1000.0) // Normalize to milliseconds
        } else {
            0.0
        };
        
        (hit_rate * 0.5 + memory_efficiency * 0.3 + access_efficiency * 0.2).min(1.0)
    }
}

/// Persistent cache implementation
pub struct PersistentCache {
    cache_dir: PathBuf,
    config: CacheConfig,
    compression_enabled: bool,
}

impl PersistentCache {
    /// Create a new persistent cache
    pub fn new(cache_dir: PathBuf, config: CacheConfig) -> RhemaResult<Self> {
        fs::create_dir_all(&cache_dir)?;
        
        let compression_enabled = config.enable_compression;
        
        Ok(Self {
            cache_dir,
            config,
            compression_enabled,
        })
    }

    /// Get a value from persistent cache
    pub fn get(&self, key: &CacheKey) -> RhemaResult<Option<Vec<u8>>> {
        let file_path = self.get_cache_file_path(key);
        
        if !file_path.exists() {
            return Ok(None);
        }

        let metadata_path = self.get_metadata_file_path(key);
        if let Ok(metadata) = self.load_metadata(&metadata_path) {
            if metadata.is_expired() {
                // Remove expired entry
                let _ = fs::remove_file(&file_path);
                let _ = fs::remove_file(&metadata_path);
                return Ok(None);
            }
        }

        let data = fs::read(&file_path)?;
        let data = if self.compression_enabled {
            self.decompress_data(&data)?
        } else {
            data
        };

        Ok(Some(data))
    }

    /// Set a value in persistent cache
    pub fn set(&self, key: &CacheKey, value: &[u8], ttl: Option<u64>, priority: u8) -> RhemaResult<()> {
        let file_path = self.get_cache_file_path(key);
        let metadata_path = self.get_metadata_file_path(key);
        
        // Create directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = if self.compression_enabled {
            self.compress_data(value)?
        } else {
            value.to_vec()
        };

        fs::write(&file_path, data)?;

        // Save metadata
        let metadata: CacheEntry<Vec<u8>> = CacheEntry::new(vec![], ttl.or(self.config.default_ttl), priority);
        let metadata_json = serde_json::to_string(&metadata)?;
        fs::write(&metadata_path, metadata_json)?;

        Ok(())
    }

    /// Remove a value from persistent cache
    pub fn remove(&self, key: &CacheKey) -> RhemaResult<bool> {
        let file_path = self.get_cache_file_path(key);
        let metadata_path = self.get_metadata_file_path(key);
        
        let existed = file_path.exists();
        if existed {
            let _ = fs::remove_file(&file_path);
            let _ = fs::remove_file(&metadata_path);
        }
        
        Ok(existed)
    }

    /// Clear all persistent cache entries
    pub fn clear(&self) -> RhemaResult<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    /// Get cache file path for a key
    fn get_cache_file_path(&self, key: &CacheKey) -> PathBuf {
        let key_hash = key.checksum();
        self.cache_dir.join(format!("{}.cache", key_hash))
    }

    /// Get metadata file path for a key
    fn get_metadata_file_path(&self, key: &CacheKey) -> PathBuf {
        let key_hash = key.checksum();
        self.cache_dir.join(format!("{}.meta", key_hash))
    }

    /// Load metadata from file
    fn load_metadata(&self, metadata_path: &Path) -> RhemaResult<CacheEntry<Vec<u8>>> {
        let metadata_json = fs::read_to_string(metadata_path)?;
        let metadata: CacheEntry<Vec<u8>> = serde_json::from_str(&metadata_json)?;
        Ok(metadata)
    }

    /// Compress data using gzip
    fn compress_data(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    /// Decompress data using gzip
    fn decompress_data(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        use std::io::Read;
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}

/// Main lock file cache manager
pub struct LockFileCache {
    memory_cache: Arc<RwLock<InMemoryCache>>,
    persistent_cache: Option<Arc<PersistentCache>>,
    config: CacheConfig,
    stats: Arc<Mutex<CacheStats>>,
}

impl LockFileCache {
    /// Create a new lock file cache
    pub fn new(config: CacheConfig) -> RhemaResult<Self> {
        let persistent_cache = if config.enable_persistent {
            let cache_dir = config.persistent_cache_dir.clone()
                .unwrap_or_else(|| PathBuf::from(".rhema/cache"));
            Some(Arc::new(PersistentCache::new(cache_dir, config.clone())?))
        } else {
            None
        };

        Ok(Self {
            memory_cache: Arc::new(RwLock::new(InMemoryCache::new(config.clone()))),
            persistent_cache,
            config,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        })
    }

    /// Get a lock file from cache
    pub fn get_lock_file(&self, repo_path: &Path) -> RhemaResult<Option<RhemaLock>> {
        let key = CacheKey::LockFile(repo_path.to_path_buf());
        self.get_serializable(&key)
    }

    /// Set a lock file in cache
    pub fn set_lock_file(&self, repo_path: &Path, lock_file: &RhemaLock, ttl: Option<u64>) -> RhemaResult<()> {
        let key = CacheKey::LockFile(repo_path.to_path_buf());
        self.set_serializable(&key, lock_file, ttl, 5) // High priority for lock files
    }

    /// Get a scope from cache
    pub fn get_scope(&self, scope_path: &Path) -> RhemaResult<Option<LockedScope>> {
        let key = CacheKey::Scope(scope_path.to_path_buf());
        self.get_serializable(&key)
    }

    /// Set a scope in cache
    pub fn set_scope(&self, scope_path: &Path, scope: &LockedScope, ttl: Option<u64>) -> RhemaResult<()> {
        let key = CacheKey::Scope(scope_path.to_path_buf());
        self.set_serializable(&key, scope, ttl, 3) // Medium priority for scopes
    }

    /// Get a dependency from cache
    pub fn get_dependency(&self, path: &str, constraint: &str) -> RhemaResult<Option<LockedDependency>> {
        let key = CacheKey::Dependency(path.to_string(), constraint.to_string());
        self.get_serializable(&key)
    }

    /// Set a dependency in cache
    pub fn set_dependency(&self, path: &str, constraint: &str, dependency: &LockedDependency, ttl: Option<u64>) -> RhemaResult<()> {
        let key = CacheKey::Dependency(path.to_string(), constraint.to_string());
        self.set_serializable(&key, dependency, ttl, 2) // Lower priority for dependencies
    }

    /// Get a generic serializable value from cache
    pub fn get_serializable<T>(&self, key: &CacheKey) -> RhemaResult<Option<T>>
    where
        T: for<'de> Deserialize<'de> + serde::Serialize,
    {
        let start_time = Instant::now();

        // Try memory cache first
        if let Some(data) = self.memory_cache.write().unwrap().get(key) {
            match bincode::deserialize(&data) {
                Ok(value) => {
                    self.update_stats(true, start_time.elapsed());
                    return Ok(Some(value));
                }
                Err(e) => {
                    warn!("Failed to deserialize from memory cache: {}", e);
                }
            }
        }

        // Try persistent cache
        if let Some(ref persistent_cache) = self.persistent_cache {
            if let Ok(Some(data)) = persistent_cache.get(key) {
                match bincode::deserialize(&data) {
                    Ok(value) => {
                        // Store in memory cache for future access
                        if let Ok(serialized) = bincode::serialize(&value) {
                            let _ = self.memory_cache.write().unwrap().set(
                                key.clone(),
                                serialized,
                                self.config.default_ttl,
                                1,
                            );
                        }
                        self.update_stats(true, start_time.elapsed());
                        return Ok(Some(value));
                    }
                    Err(e) => {
                        warn!("Failed to deserialize from persistent cache: {}", e);
                    }
                }
            }
        }

        self.update_stats(false, start_time.elapsed());
        Ok(None)
    }

    /// Set a generic serializable value in cache
    pub fn set_serializable<T>(&self, key: &CacheKey, value: &T, ttl: Option<u64>, priority: u8) -> RhemaResult<()>
    where
        T: serde::Serialize,
    {
        match bincode::serialize(value) {
            Ok(serialized) => {
                // Store in memory cache
                self.memory_cache.write().unwrap().set(
                    key.clone(),
                    serialized.clone(),
                    ttl,
                    priority,
                );

                // Store in persistent cache if available
                if let Some(ref persistent_cache) = self.persistent_cache {
                    persistent_cache.set(key, &serialized, ttl, priority)?;
                }

                Ok(())
            }
            Err(e) => {
                Err(RhemaError::SerializationError(format!("Failed to serialize cache value: {}", e)))
            }
        }
    }

    /// Remove a value from cache
    pub fn remove(&self, key: &CacheKey) -> RhemaResult<bool> {
        let memory_removed = self.memory_cache.write().unwrap().remove(key);
        
        let persistent_removed = if let Some(ref persistent_cache) = self.persistent_cache {
            persistent_cache.remove(key)?
        } else {
            false
        };

        Ok(memory_removed || persistent_removed)
    }

    /// Clear all cache entries
    pub fn clear(&self) -> RhemaResult<()> {
        self.memory_cache.write().unwrap().clear();
        
        if let Some(ref persistent_cache) = self.persistent_cache {
            persistent_cache.clear()?;
        }

        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let memory_stats = self.memory_cache.read().unwrap().stats();
        let mut combined_stats = self.stats.lock().unwrap().clone();
        
        // Combine memory and overall stats
        combined_stats.hits += memory_stats.hits;
        combined_stats.misses += memory_stats.misses;
        combined_stats.total_entries += memory_stats.total_entries;
        combined_stats.total_size_bytes += memory_stats.total_size_bytes;
        combined_stats.evictions += memory_stats.evictions;
        combined_stats.expired_entries += memory_stats.expired_entries;
        
        if combined_stats.hits + combined_stats.misses > 0 {
            combined_stats.hit_rate = combined_stats.hits as f64 / (combined_stats.hits + combined_stats.misses) as f64;
        }
        
        combined_stats
    }

    /// Warm the cache with frequently accessed data
    pub fn warm_cache(&self, repo_path: &Path) -> RhemaResult<()> {
        if !self.config.enable_warming {
            return Ok(());
        }

        match self.config.warming_strategy {
            WarmingStrategy::FrequentAccess => {
                // Load frequently accessed lock files
                self.warm_frequent_access(repo_path)?;
            }
            WarmingStrategy::AccessPattern => {
                // Load based on access patterns
                self.warm_access_patterns(repo_path)?;
            }
            WarmingStrategy::DependencyBased => {
                // Load based on dependency relationships
                self.warm_dependency_based(repo_path)?;
            }
            WarmingStrategy::TimeBased => {
                // Load based on time patterns
                self.warm_time_based(repo_path)?;
            }
            WarmingStrategy::None => {
                // No warming
            }
        }

        Ok(())
    }

    /// Warm cache with frequently accessed data
    fn warm_frequent_access(&self, repo_path: &Path) -> RhemaResult<()> {
        // This would analyze access logs and preload frequently accessed data
        // For now, we'll just load the main lock file
        if let Ok(Some(lock_file)) = self.get_lock_file(repo_path) {
            // Preload all scopes from the lock file
            for (scope_path, _) in &lock_file.scopes {
                let scope_key = CacheKey::Scope(PathBuf::from(scope_path));
                if let Ok(Some(scope)) = self.get_serializable::<LockedScope>(&scope_key) {
                    debug!("Warmed scope cache for: {}", scope_path);
                }
            }
        }
        Ok(())
    }

    /// Warm cache based on access patterns
    fn warm_access_patterns(&self, _repo_path: &Path) -> RhemaResult<()> {
        // This would analyze access patterns and preload related data
        // Implementation would depend on access pattern analysis
        Ok(())
    }

    /// Warm cache based on dependency relationships
    fn warm_dependency_based(&self, repo_path: &Path) -> RhemaResult<()> {
        // Load dependencies for the main lock file
        if let Ok(Some(lock_file)) = self.get_lock_file(repo_path) {
            for (_, scope) in &lock_file.scopes {
                for (dep_name, dep) in &scope.dependencies {
                    let dep_key = CacheKey::Dependency(dep_name.clone(), dep.version.clone());
                    if let Ok(Some(_)) = self.get_serializable::<LockedDependency>(&dep_key) {
                        debug!("Warmed dependency cache for: {}", dep_name);
                    }
                }
            }
        }
        Ok(())
    }

    /// Warm cache based on time patterns
    fn warm_time_based(&self, _repo_path: &Path) -> RhemaResult<()> {
        // This would analyze time-based access patterns
        // Implementation would depend on time pattern analysis
        Ok(())
    }

    /// Update cache statistics
    fn update_stats(&self, hit: bool, access_time: Duration) {
        let mut stats = self.stats.lock().unwrap();
        if hit {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        
        let access_time_us = access_time.as_micros() as u64;
        let total_accesses = stats.hits + stats.misses;
        if total_accesses > 0 {
            stats.avg_access_time_us = 
                ((stats.avg_access_time_us * (total_accesses - 1) as u64) + access_time_us) / total_accesses as u64;
        }
    }

    /// Get cache configuration
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Update cache configuration
    pub fn update_config(&mut self, config: CacheConfig) -> RhemaResult<()> {
        // Update memory cache configuration
        let mut memory_cache = self.memory_cache.write().unwrap();
        memory_cache.config = config.clone();
        
        // Update persistent cache if needed
        if config.enable_persistent && self.persistent_cache.is_none() {
            let cache_dir = config.persistent_cache_dir.clone()
                .unwrap_or_else(|| PathBuf::from(".rhema/cache"));
            self.persistent_cache = Some(Arc::new(PersistentCache::new(cache_dir, config.clone())?));
        }
        
        self.config = config;
        Ok(())
    }
}

/// Global cache instance
lazy_static::lazy_static! {
    static ref GLOBAL_CACHE: Arc<RwLock<Option<Arc<LockFileCache>>>> = Arc::new(RwLock::new(None));
}

/// Initialize the global cache
pub fn init_global_cache(config: CacheConfig) -> RhemaResult<()> {
    let cache = LockFileCache::new(config)?;
    *GLOBAL_CACHE.write().unwrap() = Some(Arc::new(cache));
    Ok(())
}

/// Get the global cache instance
pub fn get_global_cache() -> RhemaResult<Arc<LockFileCache>> {
    let cache_guard = GLOBAL_CACHE.read().unwrap();
    if let Some(cache) = &*cache_guard {
        Ok(Arc::clone(cache))
    } else {
        Err(RhemaError::CacheError("Global cache not initialized".to_string()))
    }
}

/// Cache management utilities
pub mod utils {
    use super::*;

    /// Clear expired entries from all caches
    pub fn clear_expired_entries() -> RhemaResult<()> {
        if let Ok(cache) = get_global_cache() {
            // Memory cache cleanup is handled automatically
            // Persistent cache cleanup would be implemented here
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Get cache performance report
    pub fn get_performance_report() -> RhemaResult<String> {
        if let Ok(cache) = get_global_cache() {
            let stats = cache.stats();
            let report = format!(
                "Cache Performance Report:\n\
                - Hit Rate: {:.2}%\n\
                - Total Entries: {}\n\
                - Total Size: {} MB\n\
                - Average Access Time: {} μs\n\
                - Evictions: {}\n\
                - Expired Entries: {}\n\
                - Efficiency Score: {:.2}",
                stats.hit_rate * 100.0,
                stats.total_entries,
                stats.total_size_bytes / 1024 / 1024,
                stats.avg_access_time_us,
                stats.evictions,
                stats.expired_entries,
                stats.efficiency_score
            );
            Ok(report)
        } else {
            Ok("Cache not available".to_string())
        }
    }

    /// Optimize cache based on usage patterns
    pub fn optimize_cache() -> RhemaResult<()> {
        if let Ok(cache) = get_global_cache() {
            let stats = cache.stats();
            
            // Adjust cache size based on hit rate
            if stats.hit_rate < 0.5 {
                // Low hit rate, consider increasing cache size
                // Note: Cannot update config on Arc<LockFileCache> - would need different approach
                debug!("Low hit rate detected, consider increasing cache size");
            }
            
            // Adjust TTL based on access patterns
            if stats.expired_entries > stats.evictions {
                // Many expired entries, consider increasing TTL
                // Note: Cannot update config on Arc<LockFileCache> - would need different approach
                debug!("Many expired entries detected, consider increasing TTL");
            }
            
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_entry_creation() {
        let entry = CacheEntry::new(vec![1, 2, 3], Some(3600), 5);
        assert_eq!(entry.access_count, 0);
        assert_eq!(entry.priority, 5);
        assert_eq!(entry.ttl, Some(3600));
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_expiration() {
        let mut entry = CacheEntry::new(vec![1, 2, 3], Some(1), 5);
        assert!(!entry.is_expired());
        
        // Wait for expiration
        std::thread::sleep(Duration::from_secs(2));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_key_serialization() {
        let key = CacheKey::LockFile(PathBuf::from("/test/path"));
        let key_str = key.to_string();
        assert!(key_str.starts_with("lock_file:"));
        assert!(!key.checksum().is_empty());
    }

    #[test]
    fn test_in_memory_cache_operations() {
        let config = CacheConfig::default();
        let mut cache = InMemoryCache::new(config);
        
        let key = CacheKey::Custom("test_key".to_string());
        let value = vec![1, 2, 3, 4, 5];
        
        // Test set and get
        cache.set(key.clone(), value.clone(), Some(3600), 5);
        let retrieved = cache.get(&key);
        assert_eq!(retrieved, Some(value));
        
        // Test stats
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.total_entries, 1);
    }

    #[test]
    fn test_persistent_cache_operations() -> RhemaResult<()> {
        let temp_dir = TempDir::new()?;
        let config = CacheConfig::default();
        let cache = PersistentCache::new(temp_dir.path().to_path_buf(), config)?;
        
        let key = CacheKey::Custom("test_key".to_string());
        let value = vec![1, 2, 3, 4, 5];
        
        // Test set and get
        cache.set(&key, &value, Some(3600), 5)?;
        let retrieved = cache.get(&key)?;
        assert_eq!(retrieved, Some(value));
        
        // Test remove
        let removed = cache.remove(&key)?;
        assert!(removed);
        
        let retrieved = cache.get(&key)?;
        assert_eq!(retrieved, None);
        
        Ok(())
    }

    #[test]
    fn test_lock_file_cache_integration() -> RhemaResult<()> {
        let config = CacheConfig {
            enable_persistent: false,
            ..Default::default()
        };
        let cache = LockFileCache::new(config)?;
        
        let key = CacheKey::Custom("test_key".to_string());
        let value = vec![1, 2, 3, 4, 5];
        
        // Test set and get
        cache.set_serializable(&key, &value, Some(3600), 5)?;
        let retrieved: Option<Vec<u8>> = cache.get_serializable(&key)?;
        assert_eq!(retrieved, Some(value));
        
        // Test stats
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        
        Ok(())
    }

    #[test]
    fn test_cache_eviction() {
        let mut config = CacheConfig::default();
        config.max_size_bytes = 100; // Small size to trigger eviction
        config.max_entries = 5;
        
        let mut cache = InMemoryCache::new(config);
        
        // Add entries until eviction is triggered
        for i in 0..10 {
            let key = CacheKey::Custom(format!("key_{}", i));
            let value = vec![i; 20]; // 20 bytes each
            cache.set(key, value, Some(3600), 1);
        }
        
        let stats = cache.stats();
        assert!(stats.evictions > 0);
        assert!(stats.total_entries <= 5);
    }
} 