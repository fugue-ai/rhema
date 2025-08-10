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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

/// Error types for the knowledge system
#[derive(Error, Debug)]
pub enum KnowledgeError {
    #[error("Embedding error: {0}")]
    EmbeddingError(#[from] crate::embedding::EmbeddingError),

    #[error("Vector storage error: {0}")]
    VectorError(#[from] crate::vector::VectorError),

    #[error("Search error: {0}")]
    SearchError(#[from] crate::search::SearchError),

    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),

    #[error("Synthesis error: {0}")]
    SynthesisError(#[from] crate::synthesis::SynthesisError),

    #[error("Cache error: {0}")]
    CacheError(#[from] crate::cache::CacheError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Engine error: {0}")]
    EngineError(#[from] crate::engine::EngineError),

    #[error("Indexing error: {0}")]
    IndexingError(#[from] crate::indexing::IndexingError),

    #[error("Proactive error: {0}")]
    ProactiveError(#[from] crate::proactive::ProactiveError),
    #[error("File watching error: {0}")]
    FileWatchingError(#[from] notify::Error),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

/// Result type for knowledge operations
pub type KnowledgeResult<T> = Result<T, KnowledgeError>;

/// Unified cache result with semantic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheResult {
    pub data: Vec<u8>,
    pub metadata: CacheEntryMetadata,
    pub semantic_info: Option<SemanticInfo>,
    pub cache_tier: CacheTier,
    pub access_patterns: AccessPatterns,
}

/// Cache entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntryMetadata {
    pub key: String,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u64,
    pub size_bytes: u64,
    pub ttl: Duration,
    pub compression_ratio: Option<f32>,
    pub semantic_tags: Vec<String>,
    pub agent_session_id: Option<String>,
    pub scope_path: Option<String>,
    pub checksum: Option<String>, // Add checksum for data integrity validation
}

/// Semantic information for cached content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticInfo {
    pub embedding: Option<Vec<f32>>,
    pub semantic_tags: Vec<String>,
    pub content_type: ContentType,
    pub relevance_score: f32,
    pub related_keys: Vec<String>,
    pub chunk_id: Option<String>,
}

/// Content type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum ContentType {
    Code,
    Documentation,
    Configuration,
    Knowledge,
    Decision,
    Pattern,
    Todo,
    Insight,
    #[default]
    Unknown,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Code => write!(f, "code"),
            ContentType::Documentation => write!(f, "documentation"),
            ContentType::Configuration => write!(f, "configuration"),
            ContentType::Knowledge => write!(f, "knowledge"),
            ContentType::Decision => write!(f, "decision"),
            ContentType::Pattern => write!(f, "pattern"),
            ContentType::Todo => write!(f, "todo"),
            ContentType::Insight => write!(f, "insight"),
            ContentType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Cache tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheTier {
    Memory,
    Disk,
    Network,
}

/// Access patterns for intelligent caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatterns {
    pub frequency: f32,
    pub recency: f32,
    pub semantic_relevance: f32,
    pub temporal_pattern: TemporalPattern,
    pub agent_affinity: HashMap<String, f32>,
    pub workflow_affinity: HashMap<String, f32>,
}

/// Temporal access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalPattern {
    Recent,
    Frequent,
    Periodic,
    Burst,
    Stable,
    Declining,
}

/// Semantic cache entry with enhanced metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCacheEntry {
    pub data: Vec<u8>,
    pub embedding: Option<Vec<f32>>,
    pub semantic_tags: Vec<String>,
    pub access_patterns: AccessPatterns,
    pub metadata: CacheEntryMetadata,
}

/// Semantic search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticResult {
    pub cache_key: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub relevance_score: f32,
    pub semantic_tags: Vec<String>,
    pub metadata: SearchResultMetadata,
    pub cache_info: Option<CacheInfo>,
}

/// Search result metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResultMetadata {
    pub source_type: ContentType,
    pub scope_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub size_bytes: u64,
    pub chunk_id: Option<String>,
}

/// Cache information for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub is_cached: bool,
    pub cache_tier: CacheTier,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub ttl_remaining: Duration,
}

/// Agent session context for persistent caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSessionContext {
    pub agent_id: String,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub workflow_context: Option<WorkflowContext>,
    pub preferences: AgentPreferences,
    pub cache_keys: Vec<String>,
}

/// Workflow context for proactive caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    pub workflow_id: String,
    pub workflow_type: WorkflowType,
    pub current_step: String,
    pub steps_completed: Vec<String>,
    pub steps_remaining: Vec<String>,
    pub context_requirements: Vec<ContextRequirement>,
}

/// Workflow types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowType {
    CodeReview,
    FeatureDevelopment,
    BugFixing,
    Documentation,
    Testing,
    Deployment,
    Refactoring,
    Onboarding,
    Custom(String),
}

/// Context requirements for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRequirement {
    pub requirement_type: ContextRequirementType,
    pub scope_path: Option<String>,
    pub content_type: ContentType,
    pub priority: Priority,
    pub estimated_size: Option<u64>,
}

/// Context requirement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextRequirementType {
    Knowledge,
    Code,
    Documentation,
    Decisions,
    Patterns,
    Dependencies,
    Configuration,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Agent preferences for personalized caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPreferences {
    pub preferred_content_types: Vec<ContentType>,
    pub semantic_relevance_threshold: f32,
    pub cache_retention_hours: u64,
    pub compression_preference: CompressionPreference,
    pub proactive_caching_enabled: bool,
}

impl Default for AgentPreferences {
    fn default() -> Self {
        Self {
            preferred_content_types: vec![ContentType::Code, ContentType::Documentation],
            semantic_relevance_threshold: 0.7,
            cache_retention_hours: 24,
            compression_preference: CompressionPreference::Balanced,
            proactive_caching_enabled: true,
        }
    }
}

/// Compression preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionPreference {
    None,
    Fast,
    Balanced,
    Maximum,
}

/// Context suggestion for proactive features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSuggestion {
    pub suggestion_id: String,
    pub title: String,
    pub description: String,
    pub relevance_score: f32,
    pub content_type: ContentType,
    pub cache_key: Option<String>,
    pub scope_path: Option<String>,
    pub reasoning: String,
    pub confidence: f32,
    pub action: SuggestionAction,
}

/// Suggestion actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionAction {
    Preload,
    Index,
    Synthesize,
    Share,
    Archive,
}

/// Knowledge synthesis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSynthesis {
    pub synthesis_id: String,
    pub topic: String,
    pub synthesized_content: String,
    pub source_keys: Vec<String>,
    pub confidence_score: f32,
    pub created_at: DateTime<Utc>,
    pub metadata: SynthesisMetadata,
}

/// Synthesis metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisMetadata {
    pub synthesis_method: SynthesisMethod,
    pub source_count: usize,
    pub cross_scope: bool,
    pub temporal_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub semantic_clusters: Vec<String>,
}

/// Synthesis methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynthesisMethod {
    SemanticClustering,
    TemporalAnalysis,
    CrossScopeCorrelation,
    PatternRecognition,
    DecisionTree,
    Hybrid,
}

/// Unified knowledge engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedEngineConfig {
    pub rag: RAGConfig,
    pub cache: CacheConfig,
    pub proactive: ProactiveConfig,
    pub monitoring: MonitoringConfig,
}

impl Default for UnifiedEngineConfig {
    fn default() -> Self {
        Self {
            rag: RAGConfig {
                embedding_model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
                chunk_size: 512,
                overlap_size: 50,
                vector_store: VectorStoreConfig {
                    store_type: VectorStoreType::Local,
                    url: None,
                    api_key: None,
                    collection_name: "rhema_knowledge".to_string(),
                    dimension: 384,
                    distance_metric: DistanceMetric::Cosine,
                    timeout_seconds: 30,
                    qdrant_url: None,
                    qdrant_api_key: None,
                    chroma_url: None,
                    chroma_api_key: None,
                    pinecone_api_key: None,
                    pinecone_environment: None,
                    pinecone_index_name: None,
                },
                semantic_search: SemanticSearchConfig::default(),
            },
            cache: CacheConfig {
                storage: StorageConfig {
                    memory: MemoryConfig {
                        enabled: true,
                        max_size_mb: 100,
                        eviction_policy: EvictionPolicy::LRU,
                    },
                    disk: DiskConfig {
                        enabled: true,
                        cache_dir: PathBuf::from("/tmp/rhema_cache"),
                        max_size_gb: 1,
                        compression_enabled: true,
                        compression_algorithm: CompressionAlgorithm::Zstd,
                    },
                    network: NetworkConfig {
                        enabled: false,
                        redis_url: None,
                        connection_pool_size: 5,
                    },
                },
                lifecycle: LifecycleConfig {
                    default_ttl_hours: 24,
                    max_object_size_mb: 10,
                    auto_refresh: true,
                    refresh_interval_hours: 1,
                },
                performance: PerformanceConfig {
                    compression_threshold_kb: 1,
                    parallel_operations: 4,
                    background_cleanup: true,
                    cleanup_interval_minutes: 30,
                },
            },
            proactive: ProactiveConfig {
                enabled: true,
                suggestion_threshold: 0.7,
                warm_cache_enabled: true,
                file_analysis_enabled: true,
            },
            monitoring: MonitoringConfig {
                enable_stats: true,
                stats_retention_days: 30,
                alert_on_high_memory: true,
                alert_threshold_percent: 80,
                semantic_metrics_enabled: true,
            },
        }
    }
}

/// RAG configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGConfig {
    pub embedding_model: String,
    pub chunk_size: usize,
    pub overlap_size: usize,
    pub vector_store: VectorStoreConfig,
    pub semantic_search: SemanticSearchConfig,
}

/// Vector store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreConfig {
    pub store_type: VectorStoreType,
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub collection_name: String,
    pub dimension: usize,
    pub distance_metric: DistanceMetric,
    pub timeout_seconds: u64,
    // Qdrant specific configuration
    pub qdrant_url: Option<String>,
    pub qdrant_api_key: Option<String>,
    // Chroma specific configuration
    pub chroma_url: Option<String>,
    pub chroma_api_key: Option<String>,
    // Pinecone specific configuration
    pub pinecone_api_key: Option<String>,
    pub pinecone_environment: Option<String>,
    pub pinecone_index_name: Option<String>,
}

/// Vector store types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorStoreType {
    Qdrant,
    Chroma,
    Pinecone,
    Local,
}

/// Distance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    Manhattan,
    DotProduct,
}

/// Semantic search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchConfig {
    pub similarity_threshold: f32,
    pub max_results: usize,
    pub hybrid_search_enabled: bool,
    pub reranking_enabled: bool,
}

impl Default for SemanticSearchConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.7,
            max_results: 100,
            hybrid_search_enabled: true,
            reranking_enabled: false,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub storage: StorageConfig,
    pub lifecycle: LifecycleConfig,
    pub performance: PerformanceConfig,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub memory: MemoryConfig,
    pub disk: DiskConfig,
    pub network: NetworkConfig,
}

/// Memory cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub max_size_mb: usize,
    pub eviction_policy: EvictionPolicy,
}

/// Disk cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    pub enabled: bool,
    pub cache_dir: PathBuf,
    pub max_size_gb: usize,
    pub compression_enabled: bool,
    pub compression_algorithm: CompressionAlgorithm,
}

/// Network cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub redis_url: Option<String>,
    pub connection_pool_size: usize,
}

/// Eviction policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LRU,
    LFU,
    SemanticLRU,
    Adaptive,
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Zstd,
    LZ4,
    Gzip,
    None,
}

/// Lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    pub default_ttl_hours: u64,
    pub max_object_size_mb: usize,
    pub auto_refresh: bool,
    pub refresh_interval_hours: u64,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub compression_threshold_kb: usize,
    pub parallel_operations: usize,
    pub background_cleanup: bool,
    pub cleanup_interval_minutes: u64,
}

/// Proactive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveConfig {
    pub enabled: bool,
    pub suggestion_threshold: f32,
    pub warm_cache_enabled: bool,
    pub file_analysis_enabled: bool,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_stats: bool,
    pub stats_retention_days: u64,
    pub alert_on_high_memory: bool,
    pub alert_threshold_percent: u8,
    pub semantic_metrics_enabled: bool,
}

/// Unified metrics for the knowledge system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMetrics {
    pub cache_metrics: CacheMetrics,
    pub search_metrics: SearchMetrics,
    pub synthesis_metrics: SynthesisMetrics,
    pub proactive_metrics: ProactiveMetrics,
    pub performance_metrics: PerformanceMetrics,
}

/// Cache metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub memory_usage_bytes: u64,
    pub disk_usage_bytes: u64,
    pub network_usage_bytes: u64,
    pub eviction_count: u64,
    pub semantic_hit_rate: f64,
}

/// Search metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetrics {
    pub total_searches: u64,
    pub semantic_searches: u64,
    pub hybrid_searches: u64,
    pub average_relevance_score: f32,
    pub average_response_time_ms: u64,
    pub cache_enhanced_searches: u64,
}

/// Synthesis metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisMetrics {
    pub total_syntheses: u64,
    pub average_confidence_score: f32,
    pub cross_scope_syntheses: u64,
    pub synthesis_time_ms: u64,
}

/// Proactive metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveMetrics {
    pub suggestions_generated: u64,
    pub suggestions_accepted: u64,
    pub cache_warming_events: u64,
    pub file_analysis_count: u64,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_cache_access_time_ms: u64,
    pub average_search_time_ms: u64,
    pub average_embedding_time_ms: u64,
    pub compression_ratio: f32,
    pub memory_pressure_percent: f32,
}

// Vector module types are defined in the vector module itself
// No circular dependencies should be created here
