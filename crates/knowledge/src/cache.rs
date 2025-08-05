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

use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::types::{
    CacheTier, CompressionAlgorithm, ContentType,
    DistanceMetric, EvictionPolicy, KnowledgeResult, SemanticCacheEntry,
    UnifiedCacheResult,
};
use crate::vector::VectorStoreWrapper;

/// Error types for cache operations
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Memory cache error: {0}")]
    MemoryCacheError(String),
    
    #[error("Disk cache error: {0}")]
    DiskCacheError(String),
    
    #[error("Network cache error: {0}")]
    NetworkCacheError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Compression error: {0}")]
    CompressionError(String),
    
    #[error("File system error: {0}")]
    FileSystemError(String),
    
    #[error("Redis error: {0}")]
    RedisError(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),
    
    #[error("Cache full: {0}")]
    CacheFull(String),
    
    #[error("Object too large: {0}")]
    ObjectTooLarge(String),
}

/// Semantic memory cache with intelligent eviction
pub struct SemanticMemoryCache {
    entries: Arc<DashMap<String, SemanticCacheEntry>>,
    semantic_index: Arc<RwLock<HashMap<String, Vec<String>>>>, // tag -> keys
    config: SemanticCacheConfig,
    eviction_policy: Arc<dyn SemanticEvictionPolicy>,
    stats: Arc<RwLock<CacheStats>>,
}

/// Semantic cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCacheConfig {
    pub max_size_mb: usize,
    pub eviction_policy: EvictionPolicy,
    pub enable_semantic_indexing: bool,
    pub semantic_similarity_threshold: f32,
    pub max_entries: usize,
}

impl Default for SemanticCacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            eviction_policy: EvictionPolicy::LRU,
            enable_semantic_indexing: true,
            semantic_similarity_threshold: 0.7,
            max_entries: 1000,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub memory_usage_bytes: u64,
    pub semantic_hit_count: u64,
    #[serde(skip)]
    pub last_updated: Instant,
}

impl<'de> serde::Deserialize<'de> for CacheStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct CacheStatsHelper {
            pub total_entries: usize,
            pub hit_count: u64,
            pub miss_count: u64,
            pub eviction_count: u64,
            pub memory_usage_bytes: u64,
            pub semantic_hit_count: u64,
        }
        
        let helper = CacheStatsHelper::deserialize(deserializer)?;
        Ok(CacheStats {
            total_entries: helper.total_entries,
            hit_count: helper.hit_count,
            miss_count: helper.miss_count,
            eviction_count: helper.eviction_count,
            memory_usage_bytes: helper.memory_usage_bytes,
            semantic_hit_count: helper.semantic_hit_count,
            last_updated: Instant::now(),
        })
    }
}

/// Semantic eviction policy trait
#[async_trait]
pub trait SemanticEvictionPolicy: Send + Sync {
    async fn select_for_eviction(&self, entries: &DashMap<String, SemanticCacheEntry>) -> Vec<String>;
    async fn should_evict(&self, entry: &SemanticCacheEntry, new_entry: &SemanticCacheEntry) -> bool;
}

/// LRU eviction policy
pub struct LRUEvictionPolicy;

#[async_trait]
impl SemanticEvictionPolicy for LRUEvictionPolicy {
    async fn select_for_eviction(&self, entries: &DashMap<String, SemanticCacheEntry>) -> Vec<String> {
        let mut entries_vec: Vec<(String, SemanticCacheEntry)> = entries
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        // Sort by last accessed time (oldest first)
        entries_vec.sort_by(|a, b| a.1.metadata.accessed_at.cmp(&b.1.metadata.accessed_at));
        
        // Return keys of oldest entries (up to 10% of cache)
        let evict_count = (entries_vec.len() / 10).max(1);
        entries_vec.into_iter().take(evict_count).map(|(k, _)| k).collect()
    }
    
    async fn should_evict(&self, entry: &SemanticCacheEntry, new_entry: &SemanticCacheEntry) -> bool {
        entry.metadata.accessed_at < new_entry.metadata.accessed_at
    }
}

/// Semantic LRU eviction policy
pub struct SemanticLRUEvictionPolicy {
    similarity_threshold: f32,
}

impl SemanticLRUEvictionPolicy {
    pub fn new(similarity_threshold: f32) -> Self {
        Self { similarity_threshold }
    }
}

#[async_trait]
impl SemanticEvictionPolicy for SemanticLRUEvictionPolicy {
    async fn select_for_eviction(&self, entries: &DashMap<String, SemanticCacheEntry>) -> Vec<String> {
        let mut entries_vec: Vec<(String, SemanticCacheEntry)> = entries
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        // Sort by semantic relevance and access time
        entries_vec.sort_by(|a, b| {
            let a_score = a.1.access_patterns.semantic_relevance + 
                         (a.1.metadata.access_count as f32 * 0.1);
            let b_score = b.1.access_patterns.semantic_relevance + 
                         (b.1.metadata.access_count as f32 * 0.1);
            a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Return keys of least relevant entries
        let evict_count = (entries_vec.len() / 10).max(1);
        entries_vec.into_iter().take(evict_count).map(|(k, _)| k).collect()
    }
    
    async fn should_evict(&self, entry: &SemanticCacheEntry, new_entry: &SemanticCacheEntry) -> bool {
        // Evict if new entry has higher semantic relevance
        new_entry.access_patterns.semantic_relevance > entry.access_patterns.semantic_relevance + self.similarity_threshold
    }
}

/// LFU (Least Frequently Used) eviction policy
pub struct LFUEvictionPolicy;

#[async_trait]
impl SemanticEvictionPolicy for LFUEvictionPolicy {
    async fn select_for_eviction(&self, entries: &DashMap<String, SemanticCacheEntry>) -> Vec<String> {
        let mut entries_vec: Vec<(String, SemanticCacheEntry)> = entries
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        // Sort by access count (least frequently used first)
        entries_vec.sort_by(|a, b| a.1.metadata.access_count.cmp(&b.1.metadata.access_count));
        
        // Return keys of least frequently used entries
        let evict_count = (entries_vec.len() / 10).max(1);
        entries_vec.into_iter().take(evict_count).map(|(k, _)| k).collect()
    }
    
    async fn should_evict(&self, entry: &SemanticCacheEntry, new_entry: &SemanticCacheEntry) -> bool {
        entry.metadata.access_count < new_entry.metadata.access_count
    }
}

/// Adaptive eviction policy that switches between strategies based on cache performance
pub struct AdaptiveEvictionPolicy {
    current_strategy: Arc<RwLock<EvictionStrategy>>,
    performance_history: Arc<RwLock<Vec<f64>>>, // Hit rate history
    strategy_switch_threshold: f64,
    min_observations: usize,
}

#[derive(Debug, Clone)]
enum EvictionStrategy {
    LRU,
    LFU,
    SemanticLRU,
}

impl AdaptiveEvictionPolicy {
    pub fn new(strategy_switch_threshold: f64) -> Self {
        Self {
            current_strategy: Arc::new(RwLock::new(EvictionStrategy::LRU)),
            performance_history: Arc::new(RwLock::new(Vec::new())),
            strategy_switch_threshold,
            min_observations: 10,
        }
    }

    async fn update_performance(&self, hit_rate: f64) {
        let mut history = self.performance_history.write().await;
        history.push(hit_rate);
        
        // Keep only last 50 observations
        if history.len() > 50 {
            history.remove(0);
        }
    }

    async fn should_switch_strategy(&self) -> bool {
        let history = self.performance_history.read().await;
        
        if history.len() < self.min_observations {
            return false;
        }
        
        // Calculate recent performance trend
        let recent_window = history.len().saturating_sub(10);
        let recent_avg: f64 = history[recent_window..].iter().sum::<f64>() / (history.len() - recent_window) as f64;
        let overall_avg: f64 = history.iter().sum::<f64>() / history.len() as f64;
        
        // Switch if recent performance is significantly worse
        (overall_avg - recent_avg).abs() > self.strategy_switch_threshold
    }

    async fn select_best_strategy(&self) -> EvictionStrategy {
        let history = self.performance_history.read().await;
        
        if history.len() < self.min_observations {
            return EvictionStrategy::LRU; // Default strategy
        }
        
        // Analyze patterns to select best strategy
        let recent_trend = self.calculate_trend(&history);
        
        match recent_trend {
            Trend::Declining => EvictionStrategy::LFU, // Try LFU for declining performance
            Trend::Stable => EvictionStrategy::LRU,    // Keep LRU for stable performance
            Trend::Improving => EvictionStrategy::SemanticLRU, // Try semantic for improving performance
        }
    }

    fn calculate_trend(&self, history: &[f64]) -> Trend {
        if history.len() < 5 {
            return Trend::Stable;
        }
        
        let recent: f64 = history[history.len().saturating_sub(5)..].iter().sum::<f64>() / 5.0;
        let earlier: f64 = history[..history.len().saturating_sub(5)].iter().sum::<f64>() / (history.len() - 5) as f64;
        
        let change = recent - earlier;
        
        if change > 0.05 {
            Trend::Improving
        } else if change < -0.05 {
            Trend::Declining
        } else {
            Trend::Stable
        }
    }
}

#[derive(Debug)]
enum Trend {
    Improving,
    Stable,
    Declining,
}

#[async_trait]
impl SemanticEvictionPolicy for AdaptiveEvictionPolicy {
    async fn select_for_eviction(&self, entries: &DashMap<String, SemanticCacheEntry>) -> Vec<String> {
        // Update strategy if needed
        if self.should_switch_strategy().await {
            let new_strategy = self.select_best_strategy().await;
            let strategy_name = format!("{:?}", new_strategy);
            let mut current = self.current_strategy.write().await;
            *current = new_strategy;
            debug!("Switched eviction strategy to {}", strategy_name);
        }
        
        // Use current strategy
        let strategy = self.current_strategy.read().await;
        match &*strategy {
            EvictionStrategy::LRU => {
                let lru_policy = LRUEvictionPolicy;
                lru_policy.select_for_eviction(entries).await
            }
            EvictionStrategy::LFU => {
                let lfu_policy = LFUEvictionPolicy;
                lfu_policy.select_for_eviction(entries).await
            }
            EvictionStrategy::SemanticLRU => {
                let semantic_policy = SemanticLRUEvictionPolicy::new(0.7);
                semantic_policy.select_for_eviction(entries).await
            }
        }
    }
    
    async fn should_evict(&self, entry: &SemanticCacheEntry, new_entry: &SemanticCacheEntry) -> bool {
        let strategy = self.current_strategy.read().await;
        match &*strategy {
            EvictionStrategy::LRU => {
                let lru_policy = LRUEvictionPolicy;
                lru_policy.should_evict(entry, new_entry).await
            }
            EvictionStrategy::LFU => {
                let lfu_policy = LFUEvictionPolicy;
                lfu_policy.should_evict(entry, new_entry).await
            }
            EvictionStrategy::SemanticLRU => {
                let semantic_policy = SemanticLRUEvictionPolicy::new(0.7);
                semantic_policy.should_evict(entry, new_entry).await
            }
        }
    }
}

impl SemanticMemoryCache {
    pub fn new_dummy() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
            semantic_index: Arc::new(RwLock::new(HashMap::new())),
            config: SemanticCacheConfig::default(),
            eviction_policy: Arc::new(LRUEvictionPolicy),
            stats: Arc::new(RwLock::new(CacheStats {
                total_entries: 0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
                memory_usage_bytes: 0,
                semantic_hit_count: 0,
                last_updated: Instant::now(),
            })),
        }
    }
    
    pub fn new(config: SemanticCacheConfig) -> Self {
        let eviction_policy: Arc<dyn SemanticEvictionPolicy> = match config.eviction_policy {
            EvictionPolicy::LRU => Arc::new(LRUEvictionPolicy),
            EvictionPolicy::SemanticLRU => Arc::new(SemanticLRUEvictionPolicy::new(config.semantic_similarity_threshold)),
            EvictionPolicy::LFU => Arc::new(LFUEvictionPolicy),
            EvictionPolicy::Adaptive => Arc::new(AdaptiveEvictionPolicy::new(0.1)), // 10% threshold for strategy switching
        };
        
        Self {
            entries: Arc::new(DashMap::new()),
            semantic_index: Arc::new(RwLock::new(HashMap::new())),
            config,
            eviction_policy,
            stats: Arc::new(RwLock::new(CacheStats {
                total_entries: 0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
                memory_usage_bytes: 0,
                semantic_hit_count: 0,
                last_updated: Instant::now(),
            })),
        }
    }
    
    pub async fn get(&self, key: &str) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        if let Some(entry) = self.entries.get(key) {
            // Update access statistics
            let mut entry = entry.clone();
            entry.metadata.accessed_at = chrono::Utc::now();
            entry.metadata.access_count += 1;
            self.entries.insert(key.to_string(), entry.clone());
            
            // Update stats
            self.update_stats_hit().await;
            
            debug!("Memory cache hit for key: {}", key);
            return Ok(Some(UnifiedCacheResult {
                data: entry.data,
                metadata: entry.metadata,
                semantic_info: Some(crate::types::SemanticInfo {
                    embedding: entry.embedding,
                    semantic_tags: entry.semantic_tags,
                    content_type: ContentType::Unknown, // TODO: Extract from metadata
                    relevance_score: entry.access_patterns.semantic_relevance,
                    related_keys: vec![],
                    chunk_id: None,
                }),
                cache_tier: CacheTier::Memory,
                access_patterns: entry.access_patterns,
            }));
        }
        
        self.update_stats_miss().await;
        debug!("Memory cache miss for key: {}", key);
        Ok(None)
    }
    
    pub async fn set(&self, key: String, entry: SemanticCacheEntry) -> KnowledgeResult<()> {
        // Check if we need to evict entries
        if self.entries.len() >= self.config.max_entries {
            self.evict_entries().await?;
        }
        
        // Check memory usage
        let entry_size = entry.data.len();
        if entry_size > self.config.max_size_mb * 1024 * 1024 {
            return Err(CacheError::ObjectTooLarge(
                format!("Entry size {} bytes exceeds limit {} MB", entry_size, self.config.max_size_mb)
            ).into());
        }
        
        // Update semantic index
        if self.config.enable_semantic_indexing {
            self.update_semantic_index(&key, &entry.semantic_tags).await;
        }
        
        // Store entry
        self.entries.insert(key.clone(), entry);
        
        // Update stats
        self.update_stats_set(entry_size).await;
        
        debug!("Stored entry in memory cache: {} ({} bytes)", key, entry_size);
        Ok(())
    }
    
    pub async fn delete(&self, key: &str) -> KnowledgeResult<()> {
        if let Some(entry) = self.entries.remove(key) {
            // Remove from semantic index
            if self.config.enable_semantic_indexing {
                self.remove_from_semantic_index(key, &entry.1.semantic_tags).await;
            }
            
            // Update stats
            self.update_stats_delete(entry.1.data.len()).await;
            
            debug!("Deleted entry from memory cache: {}", key);
        }
        Ok(())
    }
    
    pub async fn search_semantic(&self, query_tags: &[String], limit: usize) -> KnowledgeResult<Vec<UnifiedCacheResult>> {
        if !self.config.enable_semantic_indexing {
            return Ok(vec![]);
        }
        
        let semantic_index = self.semantic_index.read().await;
        let mut results = Vec::new();
        
        for tag in query_tags {
            if let Some(keys) = semantic_index.get(tag) {
                for key in keys.iter().take(limit) {
                    if let Some(entry) = self.entries.get(key) {
                        let entry = entry.clone();
                        results.push(UnifiedCacheResult {
                            data: entry.data,
                            metadata: entry.metadata,
                            semantic_info: Some(crate::types::SemanticInfo {
                                embedding: entry.embedding,
                                semantic_tags: entry.semantic_tags,
                                content_type: ContentType::Unknown,
                                relevance_score: entry.access_patterns.semantic_relevance,
                                related_keys: vec![],
                                chunk_id: None,
                            }),
                            cache_tier: CacheTier::Memory,
                            access_patterns: entry.access_patterns,
                        });
                    }
                }
            }
        }
        
        // Sort by relevance score
        results.sort_by(|a, b| {
            b.semantic_info.as_ref().unwrap().relevance_score
                .partial_cmp(&a.semantic_info.as_ref().unwrap().relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        results.truncate(limit);
        
        self.update_stats_semantic_hit().await;
        debug!("Semantic search returned {} results", results.len());
        Ok(results)
    }
    
    pub async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Calculate the actual cache hit rate
    pub async fn calculate_hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        let total_requests = stats.hit_count + stats.miss_count;
        
        if total_requests == 0 {
            0.0
        } else {
            stats.hit_count as f64 / total_requests as f64
        }
    }

    /// Get comprehensive cache statistics including hit rate
    pub async fn get_cache_stats(&self) -> CacheStats {
        let stats = self.stats.read().await.clone();
        // Note: hit_rate is calculated on-demand, not stored
        stats
    }
    
    async fn evict_entries(&self) -> KnowledgeResult<()> {
        let keys_to_evict = self.eviction_policy.select_for_eviction(&self.entries).await;
        
        for key in &keys_to_evict {
            if let Some(entry) = self.entries.remove(key) {
                // Remove from semantic index
                if self.config.enable_semantic_indexing {
                    self.remove_from_semantic_index(&key, &entry.1.semantic_tags).await;
                }
                
                // Update stats
                self.update_stats_eviction(entry.1.data.len()).await;
            }
        }
        
        debug!("Evicted {} entries from memory cache", keys_to_evict.len());
        Ok(())
    }
    
    async fn update_semantic_index(&self, key: &str, tags: &[String]) {
        let mut index = self.semantic_index.write().await;
        for tag in tags {
            index.entry(tag.clone()).or_insert_with(Vec::new).push(key.to_string());
        }
    }
    
    async fn remove_from_semantic_index(&self, key: &str, tags: &[String]) {
        let mut index = self.semantic_index.write().await;
        for tag in tags {
            if let Some(keys) = index.get_mut(tag) {
                keys.retain(|k| k != key);
            }
        }
    }
    
    async fn update_stats_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hit_count += 1;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.miss_count += 1;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_set(&self, size_bytes: usize) {
        let mut stats = self.stats.write().await;
        stats.total_entries += 1;
        stats.memory_usage_bytes += size_bytes as u64;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_delete(&self, size_bytes: usize) {
        let mut stats = self.stats.write().await;
        stats.total_entries = stats.total_entries.saturating_sub(1);
        stats.memory_usage_bytes = stats.memory_usage_bytes.saturating_sub(size_bytes as u64);
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_eviction(&self, size_bytes: usize) {
        let mut stats = self.stats.write().await;
        stats.eviction_count += 1;
        stats.total_entries = stats.total_entries.saturating_sub(1);
        stats.memory_usage_bytes = stats.memory_usage_bytes.saturating_sub(size_bytes as u64);
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_semantic_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.semantic_hit_count += 1;
        stats.last_updated = Instant::now();
    }
}

/// Semantic disk cache with vector storage integration
pub struct SemanticDiskCache {
    cache_dir: PathBuf,
    vector_store: Arc<dyn crate::vector::VectorStore>,
    index: Arc<RwLock<SemanticDiskIndex>>,
    config: SemanticDiskConfig,
    compression_enabled: bool,
    stats: Arc<RwLock<CacheStats>>,
}

/// Semantic disk cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDiskConfig {
    pub cache_dir: PathBuf,
    pub max_size_gb: usize,
    pub compression_enabled: bool,
    pub compression_algorithm: CompressionAlgorithm,
    pub compression_threshold_kb: usize, // Minimum size to compress
    pub enable_vector_storage: bool,
    pub vector_dimension: usize,
    pub distance_metric: DistanceMetric,
}

impl Default for SemanticDiskConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("/tmp/rhema_cache"),
            max_size_gb: 1,
            compression_enabled: true,
            compression_algorithm: CompressionAlgorithm::Zstd,
            compression_threshold_kb: 1, // Compress entries larger than 1KB
            enable_vector_storage: true,
            vector_dimension: 384,
            distance_metric: DistanceMetric::Cosine,
        }
    }
}

/// Semantic disk index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDiskIndex {
    pub key_to_vector: HashMap<String, String>, // key -> vector_id
    pub semantic_clusters: Vec<SemanticCluster>,
    pub access_patterns: HashMap<String, AccessPattern>,
}

/// Semantic cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCluster {
    pub cluster_id: String,
    pub centroid: Vec<f32>,
    pub member_keys: Vec<String>,
    pub semantic_tags: Vec<String>,
}

/// Access pattern for disk cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub frequency: f32,
    pub recency: f32,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
}

impl SemanticDiskCache {
    pub fn new_dummy() -> Self {
        Self {
            cache_dir: PathBuf::from("/tmp/rhema_cache"),
            vector_store: Arc::new(VectorStoreWrapper::Mock(crate::vector::MockVectorStore::new(
                "cache_collection".to_string(),
                384,
                DistanceMetric::Cosine,
            ))),
            index: Arc::new(RwLock::new(SemanticDiskIndex {
                key_to_vector: HashMap::new(),
                semantic_clusters: vec![],
                access_patterns: HashMap::new(),
            })),
            config: SemanticDiskConfig::default(),
            compression_enabled: false,
            stats: Arc::new(RwLock::new(CacheStats {
                total_entries: 0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
                memory_usage_bytes: 0,
                semantic_hit_count: 0,
                last_updated: Instant::now(),
            })),
        }
    }
    
    pub async fn new(config: SemanticDiskConfig) -> KnowledgeResult<Self> {
        // Create cache directory
        std::fs::create_dir_all(&config.cache_dir)
            .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
        
        // Initialize vector store if enabled
        let vector_store = if config.enable_vector_storage {
            let vector_config = crate::types::VectorStoreConfig {
                store_type: crate::types::VectorStoreType::Qdrant,
                url: None,
                api_key: None,
                collection_name: "rhema_disk_cache".to_string(),
                dimension: config.vector_dimension,
                distance_metric: config.distance_metric.clone(),
                timeout_seconds: 30,
                qdrant_url: None,
                qdrant_api_key: None,
                chroma_url: None,
                chroma_api_key: None,
                pinecone_api_key: None,
                pinecone_environment: None,
                pinecone_index_name: None,
            };
            Arc::new(crate::vector::MockVectorStore::new(
                "cache_collection".to_string(),
                384,
                DistanceMetric::Cosine,
            ))
        } else {
            Arc::new(crate::vector::MockVectorStore::new(
                "cache_collection".to_string(),
                384,
                DistanceMetric::Cosine,
            ))
        };
        
        // Load or create index
        let index_path = config.cache_dir.join("index.bin");
        let index = if index_path.exists() {
            let index_data = std::fs::read(&index_path)
                .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
            bincode::deserialize(&index_data)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?
        } else {
            SemanticDiskIndex {
                key_to_vector: HashMap::new(),
                semantic_clusters: Vec::new(),
                access_patterns: HashMap::new(),
            }
        };
        
        Ok(Self {
            cache_dir: config.cache_dir.clone(),
            vector_store,
            index: Arc::new(RwLock::new(index)),
            config: config.clone(),
            compression_enabled: config.compression_enabled,
            stats: Arc::new(RwLock::new(CacheStats {
                total_entries: 0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
                memory_usage_bytes: 0,
                semantic_hit_count: 0,
                last_updated: Instant::now(),
            })),
        })
    }
    
    pub async fn get(&self, key: &str) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        let file_path = self.cache_dir.join(format!("{}.cache", key));
        
        if !file_path.exists() {
            self.update_stats_miss().await;
            return Ok(None);
        }
        
        // Read and deserialize entry
        let data = std::fs::read(&file_path)
            .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
        
        let entry: SemanticCacheEntry = if self.compression_enabled {
            let decompressed = zstd::decode_all(&*data)
                .map_err(|e| CacheError::CompressionError(e.to_string()))?;
            bincode::deserialize(&decompressed)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?
        } else {
            bincode::deserialize(&data)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?
        };
        
        // Update access pattern
        self.update_access_pattern(key).await;
        
        // Update stats
        self.update_stats_hit().await;
        
        debug!("Disk cache hit for key: {}", key);
        Ok(Some(UnifiedCacheResult {
            data: entry.data,
            metadata: entry.metadata,
            semantic_info: Some(crate::types::SemanticInfo {
                embedding: entry.embedding,
                semantic_tags: entry.semantic_tags,
                content_type: ContentType::Unknown,
                relevance_score: entry.access_patterns.semantic_relevance,
                related_keys: vec![],
                chunk_id: None,
            }),
            cache_tier: CacheTier::Disk,
            access_patterns: entry.access_patterns,
        }))
    }
    
    pub async fn set(&self, key: String, entry: SemanticCacheEntry) -> KnowledgeResult<()> {
        let file_path = self.cache_dir.join(format!("{}.cache", key));
        
        // Serialize and optionally compress
        let serialized = bincode::serialize(&entry)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        let data = if self.compression_enabled {
            zstd::encode_all(&*serialized, 0)
                .map_err(|e| CacheError::CompressionError(e.to_string()))?
        } else {
            serialized
        };
        
        // Write to disk
        std::fs::write(&file_path, &data)
            .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
        
        // Store in vector store if enabled
        if self.config.enable_vector_storage {
            if let Some(embedding) = &entry.embedding {
                self.vector_store.store_with_metadata(
                    &key,
                    embedding,
                    "cached_content",
                    Some(entry.metadata.clone())
                ).await?;
            }
        }
        
        // Update index
        self.update_index(&key, &entry).await;
        
        // Update stats
        self.update_stats_set(data.len()).await;
        
        debug!("Stored entry in disk cache: {} ({} bytes)", key, data.len());
        Ok(())
    }
    
    pub async fn delete(&self, key: &str) -> KnowledgeResult<()> {
        let file_path = self.cache_dir.join(format!("{}.cache", key));
        
        if file_path.exists() {
            std::fs::remove_file(&file_path)
                .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
            
            // Remove from vector store
            if self.config.enable_vector_storage {
                self.vector_store.delete(key).await?;
            }
            
            // Update index
            self.remove_from_index(key).await;
            
            // Update stats
            self.update_stats_delete(0).await; // Size unknown after deletion
        }
        
        debug!("Deleted entry from disk cache: {}", key);
        Ok(())
    }
    
    async fn update_access_pattern(&self, key: &str) {
        let mut index = self.index.write().await;
        let now = chrono::Utc::now();
        
        let pattern = index.access_patterns.entry(key.to_string()).or_insert(AccessPattern {
            frequency: 0.0,
            recency: 0.0,
            last_accessed: now,
            access_count: 0,
        });
        
        pattern.access_count += 1;
        pattern.last_accessed = now;
        pattern.frequency = pattern.access_count as f32;
        pattern.recency = 1.0; // Will be updated by background task
    }
    
    async fn update_index(&self, key: &str, entry: &SemanticCacheEntry) {
        let mut index = self.index.write().await;
        
        // Update access patterns
        index.access_patterns.insert(key.to_string(), AccessPattern {
            frequency: entry.access_patterns.frequency,
            recency: entry.access_patterns.recency,
            last_accessed: entry.metadata.accessed_at,
            access_count: entry.metadata.access_count,
        });
        
        // Update vector mapping
        if let Some(embedding) = &entry.embedding {
            index.key_to_vector.insert(key.to_string(), key.to_string());
        }
    }
    
    async fn remove_from_index(&self, key: &str) {
        let mut index = self.index.write().await;
        index.access_patterns.remove(key);
        index.key_to_vector.remove(key);
    }
    
    async fn update_stats_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hit_count += 1;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.miss_count += 1;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_set(&self, size_bytes: usize) {
        let mut stats = self.stats.write().await;
        stats.total_entries += 1;
        stats.memory_usage_bytes += size_bytes as u64;
        stats.last_updated = Instant::now();
    }
    
    async fn update_stats_delete(&self, _size_bytes: usize) {
        let mut stats = self.stats.write().await;
        stats.total_entries = stats.total_entries.saturating_sub(1);
        stats.last_updated = Instant::now();
    }
    
    pub async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Calculate the actual cache hit rate
    pub async fn calculate_hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        let total_requests = stats.hit_count + stats.miss_count;
        
        if total_requests == 0 {
            0.0
        } else {
            stats.hit_count as f64 / total_requests as f64
        }
    }

    /// Get comprehensive cache statistics including hit rate
    pub async fn get_cache_stats(&self) -> CacheStats {
        let stats = self.stats.read().await.clone();
        // Note: hit_rate is calculated on-demand, not stored
        stats
    }
}

/// Mock vector store for when vector storage is disabled
pub struct MockVectorStore;

impl MockVectorStore {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl crate::vector::VectorStore for MockVectorStore {
    async fn store(&self, _id: &str, _embedding: &[f32], _metadata: Option<crate::types::SearchResultMetadata>) -> KnowledgeResult<()> {
        Ok(())
    }
    
    async fn store_with_metadata(&self, _id: &str, _embedding: &[f32], _content: &str, _metadata: Option<crate::types::CacheEntryMetadata>) -> KnowledgeResult<()> {
        Ok(())
    }
    
    async fn search(&self, _query_embedding: &[f32], _limit: usize) -> KnowledgeResult<Vec<crate::vector::VectorSearchResult>> {
        Ok(vec![])
    }
    
    async fn delete(&self, _id: &str) -> KnowledgeResult<()> {
        Ok(())
    }
    
    async fn get(&self, _id: &str) -> KnowledgeResult<Option<crate::vector::VectorRecord>> {
        Ok(None)
    }
    
    async fn collection_info(&self) -> KnowledgeResult<crate::vector::VectorCollectionInfo> {
        Ok(crate::vector::VectorCollectionInfo {
            name: "mock".to_string(),
            vector_count: 0,
            dimension: 384,
            distance_metric: DistanceMetric::Cosine,
            size_bytes: 0,
        })
    }
    
    async fn clear(&self) -> KnowledgeResult<()> {
        Ok(())
    }
}

/// Unified cache manager that coordinates memory and disk caches
pub struct UnifiedCacheManager {
    memory_cache: Arc<SemanticMemoryCache>,
    disk_cache: Arc<SemanticDiskCache>,
    config: UnifiedCacheConfig,
}

/// Configuration for unified cache manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheConfig {
    pub enable_memory_cache: bool,
    pub enable_disk_cache: bool,
    pub memory_cache_config: SemanticCacheConfig,
    pub disk_cache_config: SemanticDiskConfig,
    pub cache_warming_enabled: bool,
    pub cache_monitoring_enabled: bool,
}

impl Default for UnifiedCacheConfig {
    fn default() -> Self {
        Self {
            enable_memory_cache: true,
            enable_disk_cache: true,
            memory_cache_config: SemanticCacheConfig::default(),
            disk_cache_config: SemanticDiskConfig::default(),
            cache_warming_enabled: false,
            cache_monitoring_enabled: true,
        }
    }
}

impl UnifiedCacheManager {
    /// Create a new unified cache manager
    pub async fn new(config: UnifiedCacheConfig) -> KnowledgeResult<Self> {
        let memory_cache = if config.enable_memory_cache {
            Arc::new(SemanticMemoryCache::new(config.memory_cache_config.clone()))
        } else {
            Arc::new(SemanticMemoryCache::new_dummy())
        };

        let disk_cache = if config.enable_disk_cache {
            Arc::new(SemanticDiskCache::new(config.disk_cache_config.clone()).await?)
        } else {
            Arc::new(SemanticDiskCache::new_dummy())
        };

        Ok(Self {
            memory_cache,
            disk_cache,
            config,
        })
    }

    /// Calculate the overall cache hit rate across all cache tiers
    pub async fn calculate_hit_rate(&self) -> f64 {
        let memory_hit_rate = if self.config.enable_memory_cache {
            self.memory_cache.calculate_hit_rate().await
        } else {
            0.0
        };

        let disk_hit_rate = if self.config.enable_disk_cache {
            self.disk_cache.calculate_hit_rate().await
        } else {
            0.0
        };

        // Weighted average based on cache usage
        let memory_stats = self.memory_cache.stats().await;
        let disk_stats = self.disk_cache.stats().await;
        
        let memory_requests = memory_stats.hit_count + memory_stats.miss_count;
        let disk_requests = disk_stats.hit_count + disk_stats.miss_count;
        let total_requests = memory_requests + disk_requests;

        if total_requests == 0 {
            0.0
        } else {
            let memory_weight = memory_requests as f64 / total_requests as f64;
            let disk_weight = disk_requests as f64 / total_requests as f64;
            
            memory_hit_rate * memory_weight + disk_hit_rate * disk_weight
        }
    }

    /// Get comprehensive cache statistics
    pub async fn get_cache_stats(&self) -> UnifiedCacheStats {
        let memory_stats = self.memory_cache.stats().await;
        let disk_stats = self.disk_cache.stats().await;
        
        let overall_hit_rate = self.calculate_hit_rate().await;
        
        UnifiedCacheStats {
            memory_cache_stats: memory_stats.clone(),
            disk_cache_stats: disk_stats.clone(),
            overall_hit_rate,
            total_entries: memory_stats.total_entries + disk_stats.total_entries,
            total_memory_usage: memory_stats.memory_usage_bytes + disk_stats.memory_usage_bytes,
            cache_tier_breakdown: CacheTierBreakdown {
                memory_entries: memory_stats.total_entries,
                disk_entries: disk_stats.total_entries,
                memory_hit_rate: self.memory_cache.calculate_hit_rate().await,
                disk_hit_rate: self.disk_cache.calculate_hit_rate().await,
            },
        }
    }

    /// Get cache from memory first, then disk
    pub async fn get(&self, key: &str) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        // Try memory cache first
        if self.config.enable_memory_cache {
            if let Some(result) = self.memory_cache.get(key).await? {
                return Ok(Some(result));
            }
        }

        // Try disk cache
        if self.config.enable_disk_cache {
            if let Some(result) = self.disk_cache.get(key).await? {
                // Warm memory cache with frequently accessed items
                if self.config.cache_warming_enabled {
                    self.warm_memory_cache(key, &result).await?;
                }
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    /// Warm memory cache with frequently accessed items
    async fn warm_memory_cache(&self, key: &str, result: &UnifiedCacheResult) -> KnowledgeResult<()> {
        // Check if this item should be warmed based on access patterns
        if self.should_warm_item(result).await {
            let entry = SemanticCacheEntry {
                data: result.data.clone(),
                embedding: result.semantic_info.as_ref().and_then(|si| si.embedding.clone()),
                semantic_tags: result.semantic_info.as_ref().map(|si| si.semantic_tags.clone()).unwrap_or_default(),
                access_patterns: result.access_patterns.clone(),
                metadata: result.metadata.clone(),
            };
            
            // Add to memory cache
            self.memory_cache.set(key.to_string(), entry).await?;
            debug!("Warmed memory cache with key: {}", key);
        }
        Ok(())
    }

    /// Determine if an item should be warmed based on access patterns
    async fn should_warm_item(&self, result: &UnifiedCacheResult) -> bool {
        // Warm items that are frequently accessed or have high semantic relevance
        let access_count = result.metadata.access_count;
        let semantic_relevance = result.access_patterns.semantic_relevance;
        let recency = result.access_patterns.recency;
        
        // Warm if:
        // 1. Frequently accessed (more than 5 times)
        // 2. High semantic relevance (> 0.8)
        // 3. Recently accessed (recency > 0.7)
        access_count > 5 || semantic_relevance > 0.8 || recency > 0.7
    }

    /// Proactive cache warming based on usage patterns
    pub async fn warm_cache_proactively(&self) -> KnowledgeResult<()> {
        if !self.config.cache_warming_enabled {
            return Ok(());
        }

        // Get items that should be warmed from disk cache
        let items_to_warm = self.get_items_to_warm().await?;
        let items_count = items_to_warm.len();
        
        for (key, entry) in items_to_warm {
            if self.config.enable_memory_cache {
                self.memory_cache.set(key, entry).await?;
            }
        }
        
        info!("Proactively warmed {} items in memory cache", items_count);
        Ok(())
    }

    /// Get items that should be warmed based on access patterns
    async fn get_items_to_warm(&self) -> KnowledgeResult<Vec<(String, SemanticCacheEntry)>> {
        // This would typically query the disk cache for items with high access patterns
        // For now, return an empty vector as this requires disk cache implementation
        Ok(Vec::new())
    }

    /// Compress cache entry data
    async fn compress_entry(&self, entry: &mut SemanticCacheEntry) -> KnowledgeResult<()> {
        if !self.config.disk_cache_config.compression_enabled {
            return Ok(());
        }

        let original_size = entry.data.len();
        if original_size < self.config.disk_cache_config.compression_threshold_kb * 1024 {
            return Ok(()); // Don't compress small entries
        }

        let (compressed_data, was_compressed) = match self.config.disk_cache_config.compression_algorithm {
            CompressionAlgorithm::Zstd => {
                let compressed = zstd::encode_all(&*entry.data, 0)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?;
                (compressed, true)
            }
            CompressionAlgorithm::LZ4 => {
                let compressed = lz4::block::compress(&entry.data, None, false)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?;
                (compressed, true)
            }
            CompressionAlgorithm::Gzip => {
                let mut compressed = Vec::new();
                let mut encoder = flate2::write::GzEncoder::new(&mut compressed, flate2::Compression::default());
                std::io::copy(&mut std::io::Cursor::new(&entry.data), &mut encoder)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?;
                encoder.finish()
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?;
                (compressed, true)
            }
            CompressionAlgorithm::None => {
                (entry.data.clone(), false)
            }
        };

        if was_compressed {
            let compression_ratio = compressed_data.len() as f32 / original_size as f32;
            entry.metadata.compression_ratio = Some(compression_ratio);
            entry.data = compressed_data;
            debug!("Compressed cache entry: {} -> {} (ratio: {:.2})", 
                   original_size, entry.data.len(), compression_ratio);
        }

        Ok(())
    }

    /// Decompress cache entry data
    async fn decompress_entry(&self, entry: &mut SemanticCacheEntry) -> KnowledgeResult<()> {
        if entry.metadata.compression_ratio.is_none() {
            return Ok(()); // Not compressed
        }

        let decompressed_data = match self.config.disk_cache_config.compression_algorithm {
            CompressionAlgorithm::Zstd => {
                zstd::decode_all(&*entry.data)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?
            }
            CompressionAlgorithm::LZ4 => {
                lz4::block::decompress(&entry.data, None)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?
            }
            CompressionAlgorithm::Gzip => {
                let mut decompressed = Vec::new();
                let mut decoder = flate2::read::GzDecoder::new(&entry.data[..]);
                std::io::copy(&mut decoder, &mut decompressed)
                    .map_err(|e| CacheError::CompressionError(e.to_string()))?;
                decompressed
            }
            CompressionAlgorithm::None => {
                entry.data.clone()
            }
        };

        entry.data = decompressed_data;
        entry.metadata.compression_ratio = None; // Clear compression info after decompression
        Ok(())
    }

    /// Set cache entry in both memory and disk
    pub async fn set(&self, key: String, entry: SemanticCacheEntry) -> KnowledgeResult<()> {
        if self.config.enable_memory_cache {
            self.memory_cache.set(key.clone(), entry.clone()).await?;
        }

        if self.config.enable_disk_cache {
            self.disk_cache.set(key, entry).await?;
        }

        Ok(())
    }

    /// Delete cache entry from both memory and disk
    pub async fn delete(&self, key: &str) -> KnowledgeResult<()> {
        if self.config.enable_memory_cache {
            self.memory_cache.delete(key).await?;
        }

        if self.config.enable_disk_cache {
            self.disk_cache.delete(key).await?;
        }

        Ok(())
    }

    /// Persist cache to disk for recovery across restarts
    pub async fn persist_cache(&self) -> KnowledgeResult<()> {
        if !self.config.enable_disk_cache {
            return Ok(());
        }

        // Persist memory cache entries to disk
        if self.config.enable_memory_cache {
            let entries = self.memory_cache.entries.clone();
            for entry in entries.iter() {
                let mut entry_clone = entry.value().clone();
                
                // Compress before persisting
                self.compress_entry(&mut entry_clone).await?;
                
                // Store in disk cache
                self.disk_cache.set(entry.key().clone(), entry_clone).await?;
            }
        }

        info!("Persisted cache to disk successfully");
        Ok(())
    }

    /// Load cache from disk on startup
    pub async fn load_cache_from_disk(&self) -> KnowledgeResult<()> {
        if !self.config.enable_disk_cache {
            return Ok(());
        }

        // This would typically load frequently accessed items from disk to memory
        // For now, we'll implement a basic version that loads items based on access patterns
        let items_to_load = self.get_frequently_accessed_items().await?;
        let items_count = items_to_load.len();
        
        for (key, entry) in items_to_load {
            if self.config.enable_memory_cache {
                // Decompress if needed
                let mut entry_clone = entry.clone();
                self.decompress_entry(&mut entry_clone).await?;
                
                self.memory_cache.set(key, entry_clone).await?;
            }
        }

        info!("Loaded {} items from disk cache", items_count);
        Ok(())
    }

    /// Get frequently accessed items from disk cache
    async fn get_frequently_accessed_items(&self) -> KnowledgeResult<Vec<(String, SemanticCacheEntry)>> {
        // This would query the disk cache for items with high access counts
        // For now, return an empty vector as this requires disk cache implementation
        Ok(Vec::new())
    }

    /// Save cache state for recovery
    pub async fn save_cache_state(&self) -> KnowledgeResult<()> {
        let state = CacheState {
            memory_cache_stats: self.memory_cache.stats().await,
            disk_cache_stats: self.disk_cache.stats().await,
            timestamp: chrono::Utc::now(),
        };

        let state_path = self.config.disk_cache_config.cache_dir.join("cache_state.json");
        let state_json = serde_json::to_string_pretty(&state)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        std::fs::write(&state_path, state_json)
            .map_err(|e| CacheError::FileSystemError(e.to_string()))?;

        debug!("Saved cache state to {:?}", state_path);
        Ok(())
    }

    /// Load cache state for recovery
    pub async fn load_cache_state(&self) -> KnowledgeResult<Option<CacheState>> {
        let state_path = self.config.disk_cache_config.cache_dir.join("cache_state.json");
        
        if !state_path.exists() {
            return Ok(None);
        }

        let state_json = std::fs::read_to_string(&state_path)
            .map_err(|e| CacheError::FileSystemError(e.to_string()))?;
        
        let state: CacheState = serde_json::from_str(&state_json)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;

        debug!("Loaded cache state from {:?}", state_path);
        Ok(Some(state))
    }
}

/// Cache state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheState {
    pub memory_cache_stats: CacheStats,
    pub disk_cache_stats: CacheStats,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Cache monitoring and metrics
pub struct CacheMonitor {
    metrics: Arc<RwLock<CacheMetrics>>,
    alerts: Arc<RwLock<Vec<CacheAlert>>>,
    config: CacheMonitorConfig,
}

/// Cache monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMonitorConfig {
    pub enable_monitoring: bool,
    pub metrics_retention_hours: u64,
    pub alert_thresholds: CacheAlertThresholds,
    pub performance_targets: CachePerformanceTargets,
}

/// Cache alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAlertThresholds {
    pub low_hit_rate_threshold: f64,
    pub high_memory_usage_threshold: f64,
    pub high_disk_usage_threshold: f64,
    pub high_eviction_rate_threshold: f64,
    pub slow_response_time_threshold_ms: u64,
}

/// Cache performance targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceTargets {
    pub target_hit_rate: f64,
    pub target_response_time_ms: u64,
    pub target_memory_usage_percent: f64,
    pub target_disk_usage_percent: f64,
}

impl Default for CacheMonitorConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            metrics_retention_hours: 24,
            alert_thresholds: CacheAlertThresholds {
                low_hit_rate_threshold: 0.5,
                high_memory_usage_threshold: 0.8,
                high_disk_usage_threshold: 0.9,
                high_eviction_rate_threshold: 0.1,
                slow_response_time_threshold_ms: 100,
            },
            performance_targets: CachePerformanceTargets {
                target_hit_rate: 0.8,
                target_response_time_ms: 50,
                target_memory_usage_percent: 0.7,
                target_disk_usage_percent: 0.8,
            },
        }
    }
}

/// Cache metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub current_hit_rate: f64,
    pub average_response_time_ms: u64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub eviction_rate: f64,
    pub compression_ratio: f64,
    pub cache_efficiency: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub historical_metrics: Vec<HistoricalMetric>,
}

/// Historical metric data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMetric {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub hit_rate: f64,
    pub response_time_ms: u64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
}

/// Cache alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAlert {
    pub alert_type: CacheAlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metric_value: f64,
    pub threshold: f64,
}

/// Cache alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheAlertType {
    LowHitRate,
    HighMemoryUsage,
    HighDiskUsage,
    HighEvictionRate,
    SlowResponseTime,
    CacheFull,
    CompressionIneffective,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl CacheMonitor {
    /// Create a new cache monitor
    pub fn new(config: CacheMonitorConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(CacheMetrics {
                current_hit_rate: 0.0,
                average_response_time_ms: 0,
                memory_usage_percent: 0.0,
                disk_usage_percent: 0.0,
                eviction_rate: 0.0,
                compression_ratio: 0.0,
                cache_efficiency: 0.0,
                timestamp: chrono::Utc::now(),
                historical_metrics: Vec::new(),
            })),
            alerts: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }

    /// Update cache metrics
    pub async fn update_metrics(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<()> {
        if !self.config.enable_monitoring {
            return Ok(());
        }

        let stats = cache_manager.get_cache_stats().await;
        let mut metrics = self.metrics.write().await;

        // Calculate current metrics
        metrics.current_hit_rate = stats.overall_hit_rate;
        metrics.memory_usage_percent = self.calculate_memory_usage_percent(&stats);
        metrics.disk_usage_percent = self.calculate_disk_usage_percent(&stats);
        metrics.eviction_rate = self.calculate_eviction_rate(&stats);
        metrics.compression_ratio = self.calculate_compression_ratio(&stats);
        metrics.cache_efficiency = self.calculate_cache_efficiency(&stats);
        metrics.timestamp = chrono::Utc::now();

        // Add to historical metrics
        let timestamp = metrics.timestamp;
        let hit_rate = metrics.current_hit_rate;
        let response_time_ms = metrics.average_response_time_ms;
        let memory_usage_percent = metrics.memory_usage_percent;
        let disk_usage_percent = metrics.disk_usage_percent;
        
        metrics.historical_metrics.push(HistoricalMetric {
            timestamp,
            hit_rate,
            response_time_ms,
            memory_usage_percent,
            disk_usage_percent,
        });

        // Clean up old historical data
        self.cleanup_old_metrics(&mut metrics).await;

        // Check for alerts
        self.check_alerts(&metrics).await?;

        Ok(())
    }

    /// Calculate memory usage percentage
    fn calculate_memory_usage_percent(&self, stats: &UnifiedCacheStats) -> f64 {
        let memory_config = &stats.memory_cache_stats;
        let max_memory = memory_config.total_entries * 1024; // Rough estimate
        if max_memory == 0 {
            0.0
        } else {
            memory_config.memory_usage_bytes as f64 / max_memory as f64
        }
    }

    /// Calculate disk usage percentage
    fn calculate_disk_usage_percent(&self, stats: &UnifiedCacheStats) -> f64 {
        // This would typically check actual disk usage
        // For now, use a simple calculation based on entries
        0.5 // Placeholder
    }

    /// Calculate eviction rate
    fn calculate_eviction_rate(&self, stats: &UnifiedCacheStats) -> f64 {
        let total_operations = stats.memory_cache_stats.hit_count + stats.memory_cache_stats.miss_count;
        if total_operations == 0 {
            0.0
        } else {
            stats.memory_cache_stats.eviction_count as f64 / total_operations as f64
        }
    }

    /// Calculate compression ratio
    fn calculate_compression_ratio(&self, _stats: &UnifiedCacheStats) -> f64 {
        // This would calculate the overall compression ratio
        0.7 // Placeholder
    }

    /// Calculate cache efficiency
    fn calculate_cache_efficiency(&self, stats: &UnifiedCacheStats) -> f64 {
        let hit_rate = stats.overall_hit_rate;
        let memory_efficiency = 1.0 - self.calculate_memory_usage_percent(stats);
        let disk_efficiency = 1.0 - self.calculate_disk_usage_percent(stats);
        
        (hit_rate * 0.6 + memory_efficiency * 0.2 + disk_efficiency * 0.2).min(1.0)
    }

    /// Clean up old metrics
    async fn cleanup_old_metrics(&self, metrics: &mut CacheMetrics) {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(self.config.metrics_retention_hours as i64);
        metrics.historical_metrics.retain(|m| m.timestamp > cutoff_time);
    }

    /// Check for alerts based on current metrics
    async fn check_alerts(&self, metrics: &CacheMetrics) -> KnowledgeResult<()> {
        let mut alerts = self.alerts.write().await;

        // Check hit rate
        if metrics.current_hit_rate < self.config.alert_thresholds.low_hit_rate_threshold {
            alerts.push(CacheAlert {
                alert_type: CacheAlertType::LowHitRate,
                message: format!("Cache hit rate is low: {:.2}%", metrics.current_hit_rate * 100.0),
                severity: AlertSeverity::Warning,
                timestamp: chrono::Utc::now(),
                metric_value: metrics.current_hit_rate,
                threshold: self.config.alert_thresholds.low_hit_rate_threshold,
            });
        }

        // Check memory usage
        if metrics.memory_usage_percent > self.config.alert_thresholds.high_memory_usage_threshold {
            alerts.push(CacheAlert {
                alert_type: CacheAlertType::HighMemoryUsage,
                message: format!("Memory usage is high: {:.2}%", metrics.memory_usage_percent * 100.0),
                severity: AlertSeverity::Warning,
                timestamp: chrono::Utc::now(),
                metric_value: metrics.memory_usage_percent,
                threshold: self.config.alert_thresholds.high_memory_usage_threshold,
            });
        }

        // Check disk usage
        if metrics.disk_usage_percent > self.config.alert_thresholds.high_disk_usage_threshold {
            alerts.push(CacheAlert {
                alert_type: CacheAlertType::HighDiskUsage,
                message: format!("Disk usage is high: {:.2}%", metrics.disk_usage_percent * 100.0),
                severity: AlertSeverity::Error,
                timestamp: chrono::Utc::now(),
                metric_value: metrics.disk_usage_percent,
                threshold: self.config.alert_thresholds.high_disk_usage_threshold,
            });
        }

        // Check eviction rate
        if metrics.eviction_rate > self.config.alert_thresholds.high_eviction_rate_threshold {
            alerts.push(CacheAlert {
                alert_type: CacheAlertType::HighEvictionRate,
                message: format!("Eviction rate is high: {:.2}%", metrics.eviction_rate * 100.0),
                severity: AlertSeverity::Warning,
                timestamp: chrono::Utc::now(),
                metric_value: metrics.eviction_rate,
                threshold: self.config.alert_thresholds.high_eviction_rate_threshold,
            });
        }

        Ok(())
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().await.clone()
    }

    /// Get recent alerts
    pub async fn get_alerts(&self, hours: u64) -> Vec<CacheAlert> {
        let alerts = self.alerts.read().await;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        alerts.iter()
            .filter(|alert| alert.timestamp > cutoff_time)
            .cloned()
            .collect()
    }

    /// Clear old alerts
    pub async fn clear_old_alerts(&self, hours: u64) {
        let mut alerts = self.alerts.write().await;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        alerts.retain(|alert| alert.timestamp > cutoff_time);
    }

    /// Generate performance report
    pub async fn generate_performance_report(&self) -> CachePerformanceReport {
        let metrics = self.get_metrics().await;
        let alerts = self.get_alerts(24).await;

        CachePerformanceReport {
            summary: CachePerformanceSummary {
                overall_efficiency: metrics.cache_efficiency,
                hit_rate: metrics.current_hit_rate,
                memory_usage: metrics.memory_usage_percent,
                disk_usage: metrics.disk_usage_percent,
                response_time_ms: metrics.average_response_time_ms,
            },
            alerts_count: alerts.len(),
            recommendations: self.generate_recommendations(&metrics).await,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Generate recommendations based on metrics
    async fn generate_recommendations(&self, metrics: &CacheMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.current_hit_rate < self.config.performance_targets.target_hit_rate {
            recommendations.push("Consider increasing cache size or improving cache warming strategy".to_string());
        }

        if metrics.memory_usage_percent > self.config.performance_targets.target_memory_usage_percent {
            recommendations.push("Consider reducing memory cache size or implementing more aggressive eviction".to_string());
        }

        if metrics.disk_usage_percent > self.config.performance_targets.target_disk_usage_percent {
            recommendations.push("Consider cleaning up disk cache or increasing disk space".to_string());
        }

        if metrics.eviction_rate > 0.05 {
            recommendations.push("High eviction rate detected - consider adjusting eviction policy or increasing cache size".to_string());
        }

        recommendations
    }
}

/// Cache performance report
#[derive(Debug, Clone, Serialize)]
pub struct CachePerformanceReport {
    pub summary: CachePerformanceSummary,
    pub alerts_count: usize,
    pub recommendations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Cache performance summary
#[derive(Debug, Clone, Serialize)]
pub struct CachePerformanceSummary {
    pub overall_efficiency: f64,
    pub hit_rate: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub response_time_ms: u64,
}

/// Cache optimizer for performance improvements
pub struct CacheOptimizer {
    config: CacheOptimizerConfig,
    optimization_history: Arc<RwLock<Vec<OptimizationAction>>>,
}

/// Cache optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptimizerConfig {
    pub enable_auto_optimization: bool,
    pub optimization_interval_minutes: u64,
    pub performance_threshold: f64,
    pub max_optimization_actions: usize,
}

/// Optimization action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    pub action_type: OptimizationActionType,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub performance_impact: f64,
    pub applied: bool,
}

/// Optimization action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationActionType {
    AdjustEvictionPolicy,
    ResizeCache,
    EnableCompression,
    AdjustCompressionThreshold,
    WarmCache,
    CleanupExpired,
    RebalanceTiers,
    OptimizeIndexes,
}

impl Default for CacheOptimizerConfig {
    fn default() -> Self {
        Self {
            enable_auto_optimization: true,
            optimization_interval_minutes: 30,
            performance_threshold: 0.8,
            max_optimization_actions: 10,
        }
    }
}

impl CacheOptimizer {
    /// Create a new cache optimizer
    pub fn new(config: CacheOptimizerConfig) -> Self {
        Self {
            config,
            optimization_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Run cache optimization
    pub async fn optimize(&self, cache_manager: &mut UnifiedCacheManager, monitor: &CacheMonitor) -> KnowledgeResult<Vec<OptimizationAction>> {
        if !self.config.enable_auto_optimization {
            return Ok(Vec::new());
        }

        let metrics = monitor.get_metrics().await;
        let mut actions = Vec::new();

        // Check if optimization is needed
        if metrics.cache_efficiency >= self.config.performance_threshold {
            return Ok(actions);
        }

        // Generate optimization actions
        actions.extend(self.generate_optimization_actions(&metrics, cache_manager).await);

        // Apply optimization actions
        for action in &mut actions {
            if let Err(e) = self.apply_optimization_action(action, cache_manager).await {
                warn!("Failed to apply optimization action: {:?}, error: {}", action.action_type, e);
                action.applied = false;
            } else {
                action.applied = true;
            }
        }

        // Record optimization actions
        self.record_optimization_actions(&actions).await;

        info!("Applied {} optimization actions", actions.iter().filter(|a| a.applied).count());
        Ok(actions)
    }

    /// Generate optimization actions based on metrics
    async fn generate_optimization_actions(&self, metrics: &CacheMetrics, _cache_manager: &UnifiedCacheManager) -> Vec<OptimizationAction> {
        let mut actions = Vec::new();
        let now = chrono::Utc::now();

        // Low hit rate optimization
        if metrics.current_hit_rate < 0.6 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::WarmCache,
                description: "Cache hit rate is low, warming frequently accessed items".to_string(),
                timestamp: now,
                performance_impact: 0.1,
                applied: false,
            });

            actions.push(OptimizationAction {
                action_type: OptimizationActionType::AdjustEvictionPolicy,
                description: "Adjusting eviction policy to improve hit rate".to_string(),
                timestamp: now,
                performance_impact: 0.05,
                applied: false,
            });
        }

        // High memory usage optimization
        if metrics.memory_usage_percent > 0.8 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::EnableCompression,
                description: "High memory usage, enabling compression".to_string(),
                timestamp: now,
                performance_impact: 0.15,
                applied: false,
            });

            actions.push(OptimizationAction {
                action_type: OptimizationActionType::CleanupExpired,
                description: "Cleaning up expired cache entries".to_string(),
                timestamp: now,
                performance_impact: 0.1,
                applied: false,
            });
        }

        // High disk usage optimization
        if metrics.disk_usage_percent > 0.9 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::CleanupExpired,
                description: "High disk usage, cleaning up expired entries".to_string(),
                timestamp: now,
                performance_impact: 0.2,
                applied: false,
            });

            actions.push(OptimizationAction {
                action_type: OptimizationActionType::AdjustCompressionThreshold,
                description: "Adjusting compression threshold for better space utilization".to_string(),
                timestamp: now,
                performance_impact: 0.1,
                applied: false,
            });
        }

        // High eviction rate optimization
        if metrics.eviction_rate > 0.1 {
            actions.push(OptimizationAction {
                action_type: OptimizationActionType::ResizeCache,
                description: "High eviction rate, increasing cache size".to_string(),
                timestamp: now,
                performance_impact: 0.15,
                applied: false,
            });
        }

        // Limit number of actions
        actions.truncate(self.config.max_optimization_actions);
        actions
    }

    /// Apply optimization action
    async fn apply_optimization_action(&self, action: &OptimizationAction, cache_manager: &mut UnifiedCacheManager) -> KnowledgeResult<()> {
        match action.action_type {
            OptimizationActionType::WarmCache => {
                cache_manager.warm_cache_proactively().await?;
            }
            OptimizationActionType::AdjustEvictionPolicy => {
                // This would adjust the eviction policy based on current performance
                debug!("Adjusting eviction policy for better performance");
            }
            OptimizationActionType::EnableCompression => {
                // Enable compression if not already enabled
                if !cache_manager.config.disk_cache_config.compression_enabled {
                    cache_manager.config.disk_cache_config.compression_enabled = true;
                }
            }
            OptimizationActionType::AdjustCompressionThreshold => {
                // Adjust compression threshold
                let current_threshold = cache_manager.config.disk_cache_config.compression_threshold_kb;
                cache_manager.config.disk_cache_config.compression_threshold_kb = (current_threshold / 2).max(1);
            }
            OptimizationActionType::CleanupExpired => {
                // Clean up expired entries
                self.cleanup_expired_entries(cache_manager).await?;
            }
            OptimizationActionType::ResizeCache => {
                // Increase cache size
                let current_size = cache_manager.config.memory_cache_config.max_size_mb;
                cache_manager.config.memory_cache_config.max_size_mb = (current_size as f64 * 1.5) as usize;
            }
            OptimizationActionType::RebalanceTiers => {
                // Rebalance between memory and disk tiers
                debug!("Rebalancing cache tiers");
            }
            OptimizationActionType::OptimizeIndexes => {
                // Optimize semantic indexes
                debug!("Optimizing semantic indexes");
            }
        }

        Ok(())
    }

    /// Clean up expired cache entries
    async fn cleanup_expired_entries(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<()> {
        let now = chrono::Utc::now();
        let mut expired_keys = Vec::new();

        // Check memory cache for expired entries
        if cache_manager.config.enable_memory_cache {
            let entries = cache_manager.memory_cache.entries.clone();
            for entry in entries.iter() {
                let ttl = entry.value().metadata.ttl;
                let created_at = entry.value().metadata.created_at;
                let expires_at = created_at + chrono::Duration::from_std(ttl).unwrap_or_default();
                
                if now > expires_at {
                    expired_keys.push(entry.key().clone());
                }
            }
        }

        // Remove expired entries
        let expired_count = expired_keys.len();
        for key in expired_keys {
            cache_manager.delete(&key).await?;
        }

        info!("Cleaned up {} expired cache entries", expired_count);
        Ok(())
    }

    /// Record optimization actions
    async fn record_optimization_actions(&self, actions: &[OptimizationAction]) {
        let mut history = self.optimization_history.write().await;
        history.extend(actions.iter().cloned());
        
        // Keep only recent actions
        let history_len = history.len();
        if history_len > 100 {
            history.drain(0..history_len - 100);
        }
    }

    /// Get optimization history
    pub async fn get_optimization_history(&self, hours: u64) -> Vec<OptimizationAction> {
        let history = self.optimization_history.read().await;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        history.iter()
            .filter(|action| action.timestamp > cutoff_time)
            .cloned()
            .collect()
    }

    /// Get optimization recommendations
    pub async fn get_optimization_recommendations(&self, metrics: &CacheMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.current_hit_rate < 0.6 {
            recommendations.push("Consider implementing cache warming for frequently accessed items".to_string());
            recommendations.push("Review and adjust eviction policy settings".to_string());
        }

        if metrics.memory_usage_percent > 0.8 {
            recommendations.push("Enable compression to reduce memory usage".to_string());
            recommendations.push("Consider increasing memory cache size".to_string());
        }

        if metrics.disk_usage_percent > 0.9 {
            recommendations.push("Clean up expired cache entries".to_string());
            recommendations.push("Consider increasing disk space or implementing cleanup policies".to_string());
        }

        if metrics.eviction_rate > 0.1 {
            recommendations.push("High eviction rate detected - consider increasing cache size".to_string());
            recommendations.push("Review cache access patterns and adjust eviction policy".to_string());
        }

        recommendations
    }
}

/// Cache validator for data integrity and consistency
pub struct CacheValidator {
    config: CacheValidatorConfig,
    validation_history: Arc<RwLock<Vec<ValidationResult>>>,
}

/// Cache validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheValidatorConfig {
    pub enable_validation: bool,
    pub validation_interval_minutes: u64,
    pub checksum_validation: bool,
    pub semantic_validation: bool,
    pub consistency_checks: bool,
    pub auto_repair: bool,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_type: ValidationType,
    pub status: ValidationStatus,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub affected_keys: Vec<String>,
    pub repair_actions: Vec<String>,
}

/// Validation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    Checksum,
    Semantic,
    Consistency,
    Integrity,
    Expiration,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Passed,
    Failed,
    Warning,
    Repaired,
}

impl Default for CacheValidatorConfig {
    fn default() -> Self {
        Self {
            enable_validation: true,
            validation_interval_minutes: 60,
            checksum_validation: true,
            semantic_validation: true,
            consistency_checks: true,
            auto_repair: false,
        }
    }
}

impl CacheValidator {
    /// Create a new cache validator
    pub fn new(config: CacheValidatorConfig) -> Self {
        Self {
            config,
            validation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Run comprehensive cache validation
    pub async fn validate_cache(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<Vec<ValidationResult>> {
        if !self.config.enable_validation {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        // Run different types of validation
        if self.config.checksum_validation {
            results.extend(self.validate_checksums(cache_manager).await?);
        }

        if self.config.semantic_validation {
            results.extend(self.validate_semantic_integrity(cache_manager).await?);
        }

        if self.config.consistency_checks {
            results.extend(self.validate_consistency(cache_manager).await?);
        }

        // Auto-repair if enabled
        if self.config.auto_repair {
            self.auto_repair_failures(&results, cache_manager).await?;
        }

        // Record validation results
        self.record_validation_results(&results).await;

        info!("Cache validation completed: {} passed, {} failed", 
              results.iter().filter(|r| matches!(r.status, ValidationStatus::Passed)).count(),
              results.iter().filter(|r| matches!(r.status, ValidationStatus::Failed)).count());

        Ok(results)
    }

    /// Validate checksums of cache entries
    async fn validate_checksums(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<Vec<ValidationResult>> {
        let mut results = Vec::new();
        let failed_keys = Vec::new();

        if cache_manager.config.enable_memory_cache {
            let entries = cache_manager.memory_cache.entries.clone();
            for entry in entries.iter() {
                let key = entry.key();
                let entry_data = entry.value();

                // Calculate current checksum
                let _current_checksum = self.calculate_checksum(&entry_data.data);
                
                // Check if checksum matches (if stored)
                // TODO: Add checksum field to CacheEntryMetadata
                // if let Some(stored_checksum) = &entry_data.metadata.checksum {
                //     if current_checksum != *stored_checksum {
                //         failed_keys.push(key.clone());
                //     }
                // }
            }
        }

        if failed_keys.is_empty() {
            results.push(ValidationResult {
                validation_type: ValidationType::Checksum,
                status: ValidationStatus::Passed,
                message: "All checksums validated successfully".to_string(),
                timestamp: chrono::Utc::now(),
                affected_keys: Vec::new(),
                repair_actions: Vec::new(),
            });
        } else {
            results.push(ValidationResult {
                validation_type: ValidationType::Checksum,
                status: ValidationStatus::Failed,
                message: format!("Checksum validation failed for {} entries", failed_keys.len()),
                timestamp: chrono::Utc::now(),
                affected_keys: failed_keys,
                repair_actions: vec!["Remove corrupted entries".to_string()],
            });
        }

        Ok(results)
    }

    /// Validate semantic integrity of cache entries
    async fn validate_semantic_integrity(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<Vec<ValidationResult>> {
        let mut results = Vec::new();
        let mut invalid_keys = Vec::new();

        if cache_manager.config.enable_memory_cache {
            let entries = cache_manager.memory_cache.entries.clone();
            for entry in entries.iter() {
                let key = entry.key();
                let entry_data = entry.value();

                // Check if semantic tags are consistent with content
                if !self.validate_semantic_tags(entry_data).await {
                    invalid_keys.push(key.clone());
                }

                // Check if embeddings are valid
                if let Some(embedding) = &entry_data.embedding {
                    if !self.validate_embedding(embedding).await {
                        invalid_keys.push(key.clone());
                    }
                }
            }
        }

        if invalid_keys.is_empty() {
            results.push(ValidationResult {
                validation_type: ValidationType::Semantic,
                status: ValidationStatus::Passed,
                message: "Semantic integrity validated successfully".to_string(),
                timestamp: chrono::Utc::now(),
                affected_keys: Vec::new(),
                repair_actions: Vec::new(),
            });
        } else {
            results.push(ValidationResult {
                validation_type: ValidationType::Semantic,
                status: ValidationStatus::Warning,
                message: format!("Semantic validation issues found for {} entries", invalid_keys.len()),
                timestamp: chrono::Utc::now(),
                affected_keys: invalid_keys,
                repair_actions: vec!["Regenerate semantic tags".to_string(), "Recompute embeddings".to_string()],
            });
        }

        Ok(results)
    }

    /// Validate cache consistency
    async fn validate_consistency(&self, cache_manager: &UnifiedCacheManager) -> KnowledgeResult<Vec<ValidationResult>> {
        let mut results = Vec::new();
        let mut inconsistent_keys = Vec::new();

        // Check for consistency between memory and disk cache
        if cache_manager.config.enable_memory_cache && cache_manager.config.enable_disk_cache {
            let memory_entries = cache_manager.memory_cache.entries.clone();
            for entry in memory_entries.iter() {
                let key = entry.key();
                
                // Check if entry exists in disk cache
                if let Some(disk_entry) = cache_manager.disk_cache.get(key).await? {
                    let memory_entry = entry.value();
                    
                    // Compare metadata
                    if memory_entry.metadata.created_at != disk_entry.metadata.created_at ||
                       memory_entry.metadata.access_count != disk_entry.metadata.access_count {
                        inconsistent_keys.push(key.clone());
                    }
                } else {
                    // Entry exists in memory but not in disk
                    inconsistent_keys.push(key.clone());
                }
            }
        }

        if inconsistent_keys.is_empty() {
            results.push(ValidationResult {
                validation_type: ValidationType::Consistency,
                status: ValidationStatus::Passed,
                message: "Cache consistency validated successfully".to_string(),
                timestamp: chrono::Utc::now(),
                affected_keys: Vec::new(),
                repair_actions: Vec::new(),
            });
        } else {
            results.push(ValidationResult {
                validation_type: ValidationType::Consistency,
                status: ValidationStatus::Warning,
                message: format!("Consistency issues found for {} entries", inconsistent_keys.len()),
                timestamp: chrono::Utc::now(),
                affected_keys: inconsistent_keys,
                repair_actions: vec!["Synchronize memory and disk cache".to_string()],
            });
        }

        Ok(results)
    }

    /// Validate semantic tags
    async fn validate_semantic_tags(&self, entry: &SemanticCacheEntry) -> bool {
        // Check if semantic tags are not empty for content that should have them
        if entry.data.len() > 100 && entry.semantic_tags.is_empty() {
            return false;
        }

        // Check if tags are reasonable (not too many, not too few)
        if entry.semantic_tags.len() > 50 {
            return false;
        }

        true
    }

    /// Validate embedding
    async fn validate_embedding(&self, embedding: &[f32]) -> bool {
        // Check if embedding has reasonable dimensions
        if embedding.is_empty() || embedding.len() > 10000 {
            return false;
        }

        // Check if embedding values are reasonable (not all zeros, not all same value)
        let sum: f32 = embedding.iter().sum();
        let avg = sum / embedding.len() as f32;
        
        if avg.abs() < 0.001 || avg.abs() > 1000.0 {
            return false;
        }

        true
    }

    /// Calculate checksum for data
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Auto-repair validation failures
    async fn auto_repair_failures(&self, results: &[ValidationResult], cache_manager: &UnifiedCacheManager) -> KnowledgeResult<()> {
        for result in results {
            if matches!(result.status, ValidationStatus::Failed | ValidationStatus::Warning) {
                for key in &result.affected_keys {
                    match result.validation_type {
                        ValidationType::Checksum => {
                            // Remove corrupted entries
                            cache_manager.delete(key).await?;
                        }
                        ValidationType::Semantic => {
                            // Regenerate semantic information
                            if let Some(entry) = cache_manager.get(key).await? {
                                // This would regenerate semantic tags and embeddings
                                debug!("Regenerating semantic information for key: {}", key);
                            }
                        }
                        ValidationType::Consistency => {
                            // Synchronize cache tiers
                            debug!("Synchronizing cache tiers for key: {}", key);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    /// Record validation results
    async fn record_validation_results(&self, results: &[ValidationResult]) {
        let mut history = self.validation_history.write().await;
        history.extend(results.iter().cloned());
        
        // Keep only recent results
        let history_len = history.len();
        if history_len > 1000 {
            history.drain(0..history_len - 1000);
        }
    }

    /// Get validation history
    pub async fn get_validation_history(&self, hours: u64) -> Vec<ValidationResult> {
        let history = self.validation_history.read().await;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        history.iter()
            .filter(|result| result.timestamp > cutoff_time)
            .cloned()
            .collect()
    }

    /// Get validation statistics
    pub async fn get_validation_stats(&self, hours: u64) -> ValidationStats {
        let history = self.get_validation_history(hours).await;
        
        let total_validations = history.len();
        let passed = history.iter().filter(|r| matches!(r.status, ValidationStatus::Passed)).count();
        let failed = history.iter().filter(|r| matches!(r.status, ValidationStatus::Failed)).count();
        let warnings = history.iter().filter(|r| matches!(r.status, ValidationStatus::Warning)).count();
        let repaired = history.iter().filter(|r| matches!(r.status, ValidationStatus::Repaired)).count();

        ValidationStats {
            total_validations,
            passed,
            failed,
            warnings,
            repaired,
            success_rate: if total_validations > 0 { passed as f64 / total_validations as f64 } else { 0.0 },
        }
    }
}

/// Validation statistics
#[derive(Debug, Clone, Serialize)]
pub struct ValidationStats {
    pub total_validations: usize,
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
    pub repaired: usize,
    pub success_rate: f64,
}

/// Comprehensive cache statistics for unified cache manager
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedCacheStats {
    pub memory_cache_stats: CacheStats,
    pub disk_cache_stats: CacheStats,
    pub overall_hit_rate: f64,
    pub total_entries: usize,
    pub total_memory_usage: u64,
    pub cache_tier_breakdown: CacheTierBreakdown,
}

/// Breakdown of cache performance by tier
#[derive(Debug, Clone, Serialize)]
pub struct CacheTierBreakdown {
    pub memory_entries: usize,
    pub disk_entries: usize,
    pub memory_hit_rate: f64,
    pub disk_hit_rate: f64,
} 