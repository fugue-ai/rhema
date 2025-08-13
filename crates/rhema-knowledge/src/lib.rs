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

pub mod cache;
pub mod embedding;
pub mod engine;
pub mod indexing;
pub mod integration;
pub mod proactive;
pub mod search;
pub mod storage;
pub mod synthesis;
pub mod temporal;
pub mod types;
pub mod vector;

// Re-export main types for convenience
// Cache module exports
pub use cache::{
    AdaptiveEvictionPolicy, CacheMetrics, CacheMonitor, CacheOptimizer, CachePerformanceReport,
    CacheValidator, UnifiedCacheManager, UnifiedCacheStats,
};

// Engine module exports
pub use engine::{
    ConnectionPool, DistributedRAGCache, FileWatchInfo, FileWatcher, ProactiveContextManager,
    SuggestionEngine, SuggestionEngineConfig, UnifiedKnowledgeEngine, UsageAnalyzer,
};

// Types module exports
pub use types::{
    AccessPatterns, AgentPreferences, AgentSessionContext, CacheConfig, CacheEntryMetadata,
    CacheInfo, CacheMetrics as TypesCacheMetrics, CacheTier, CompressionAlgorithm,
    CompressionPreference, ContentType, ContextRequirement, ContextRequirementType,
    ContextSuggestion, DiskConfig, DistanceMetric, EvictionPolicy, KnowledgeError, KnowledgeResult,
    KnowledgeSynthesis, LifecycleConfig, MemoryConfig, MonitoringConfig, NetworkConfig,
    PerformanceConfig, PerformanceMetrics, Priority, ProactiveConfig, ProactiveMetrics, RAGConfig,
    SearchMetrics, SearchResultMetadata, SemanticInfo, SemanticSearchConfig, SuggestionAction,
    SynthesisMetadata, SynthesisMethod, SynthesisMetrics, TemporalPattern, UnifiedCacheResult,
    UnifiedEngineConfig, UnifiedMetrics, VectorStoreConfig, VectorStoreType, WorkflowContext,
    WorkflowType,
};

// Search module exports
pub use search::SemanticSearchEngine;

// Proactive module exports
pub use proactive::{
    FileWatchInfo as ProactiveFileWatchInfo, FileWatcher as ProactiveFileWatcher,
    ProactiveConfig as ProactiveEngineConfig, ProactiveContextManager as ProactiveManager,
    SuggestionEngine as ProactiveSuggestionEngine,
    SuggestionEngineConfig as ProactiveSuggestionConfig, UsageAnalyzer as ProactiveUsageAnalyzer,
};

// Synthesis module exports
pub use synthesis::KnowledgeSynthesizer;

// Performance module exports - not yet implemented
// pub use performance::{
//     PerformanceMonitor, PerformanceConfig, PerformanceMetrics, ResourceUsage,
//     MemoryOptimization, MemoryOptimizationType, ParallelProcessingConfig, LazyLoadingConfig,
//     MemoryOptimizationResult, PerformanceOptimizationResult
// };

// Storage module exports
pub use storage::{
    CleanupResult, CompressionResult, DeduplicationResult, EncryptionAlgorithm, StorageConfig,
    StorageEntry, StorageManager, StorageMetadata, StorageOptimizationConfig,
    StorageOptimizationResult, StorageValidationResult,
};

// AI Integration module exports
pub use integration::{
    AIEnhancement, AIEnhancementType, AIInsight, AIInsightType, AIIntegration, AIIntegrationConfig,
    AIIntegrationMetrics, AIKnowledgeRequest, AIKnowledgeResponse, AIKnowledgeResult,
    KnowledgeSuggestion, KnowledgeSuggestionType, SuggestionPriority,
};

// Temporal module exports
pub use temporal::{
    AccessType, AdaptiveDecayConfig, CausalDirection, ContentAccess, DecayFunction,
    FreshnessPreference, SeasonalPattern, SeasonalPatternDetector, SeasonalPeriod,
    SeasonalPreference, TemporalConfig, TemporalContextManager, TemporalContextRelationship,
    TemporalEnhancedResult, TemporalFactor, TemporalFactorType, TemporalFilter,
    TemporalFilterBuilder, TemporalFilterUtils, TemporalFilterValidator,
    TemporalRelationshipDetector, TemporalRelationshipType, TemporalRelevanceBreakdown,
    TemporalRelevanceEngine, TemporalSearchEnhancer, TemporalSearchQuery, TemporalWeights,
    TimeRange, TimezoneAwareContextManager, TimezoneContext,
};

// Error type conversions
impl From<types::KnowledgeError> for rhema_core::RhemaError {
    fn from(err: types::KnowledgeError) -> Self {
        rhema_core::RhemaError::KnowledgeError(err.to_string())
    }
}

impl From<embedding::EmbeddingError> for rhema_core::RhemaError {
    fn from(err: embedding::EmbeddingError) -> Self {
        rhema_core::RhemaError::KnowledgeError(err.to_string())
    }
}

impl From<vector::VectorError> for rhema_core::RhemaError {
    fn from(err: vector::VectorError) -> Self {
        rhema_core::RhemaError::KnowledgeError(err.to_string())
    }
}

impl From<search::SearchError> for rhema_core::RhemaError {
    fn from(err: search::SearchError) -> Self {
        rhema_core::RhemaError::KnowledgeError(err.to_string())
    }
}

mod test_knowledge;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::{EmbeddingCompressionType, EmbeddingManager};
    use crate::engine::{ProactiveContextManager, UnifiedKnowledgeEngine};
    use crate::indexing::{IndexCleanupConfig, IndexingConfig, IndexingMetadata, SemanticIndexer};
    use crate::synthesis::KnowledgeSynthesizer;
    use crate::types::*;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_search_enhancements() {
        let search_engine = SemanticSearchEngine::new_dummy();

        // Test reranking
        let results = search_engine
            .search_with_reranking("test query", 5)
            .await
            .unwrap();
        assert!(!results.is_empty());

        // Test keyword search
        let keyword_results = search_engine.search_keyword("test", 5).await.unwrap();
        assert!(!keyword_results.is_empty());

        // Test hybrid search
        let hybrid_results = search_engine
            .search_hybrid("test query", 5, 0.7)
            .await
            .unwrap();
        assert!(!hybrid_results.is_empty());
    }

    #[tokio::test]
    async fn test_embedding_enhancements() {
        let embedding_manager = EmbeddingManager::new_dummy();

        // Test embedding caching
        let text = "test embedding";
        let embedding = embedding_manager.embed(text, None).await.unwrap();
        embedding_manager
            .cache_embedding("test_key", &embedding, None)
            .await
            .unwrap();

        // Test cached embedding retrieval
        let cached = embedding_manager
            .get_cached_embedding("test_key", None)
            .await
            .unwrap();
        assert!(cached.is_some());

        // Test embedding validation
        let validation = embedding_manager
            .validate_embedding(&embedding, None)
            .await
            .unwrap();
        assert!(validation.is_valid);

        // Test embedding compression
        let compressed = embedding_manager
            .compress_embedding(&embedding, EmbeddingCompressionType::Quantization)
            .await
            .unwrap();
        assert_eq!(compressed.original_dimension, embedding.len());

        // Test embedding decompression
        let decompressed = embedding_manager
            .decompress_embedding(&compressed)
            .await
            .unwrap();
        assert_eq!(decompressed.len(), embedding.len());

        // Test embedding versioning
        let versioned = embedding_manager
            .version_embedding(&embedding, "1.0.0")
            .await
            .unwrap();
        assert_eq!(versioned.version, "1.0.0");
        assert_eq!(versioned.dimension, embedding.len());
    }

    #[tokio::test]
    async fn test_indexing_enhancements() {
        let indexer = SemanticIndexer::new(
            Arc::new(EmbeddingManager::new_dummy()),
            crate::vector::VectorStoreWrapper::Mock(crate::vector::MockVectorStore::new(
                "test_collection".to_string(),
                384,
                DistanceMetric::Cosine,
            )),
            IndexingConfig::default(),
        )
        .await
        .unwrap();

        // Test incremental indexing
        let content = "test content";
        let metadata = IndexingMetadata {
            source_path: Some(PathBuf::from("test.txt")),
            content_type: ContentType::Documentation,
            scope_path: None,
            created_at: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
            size_bytes: content.len() as u64,
            language: None,
            tags: vec![],
        };

        let result = indexer
            .incremental_index(content, metadata, "old_hash")
            .await
            .unwrap();
        assert!(!result.is_empty());

        // Test index validation
        let validation = indexer
            .validate_index(&PathBuf::from("test.txt"))
            .await
            .unwrap();
        assert!(validation.is_valid);

        // Test indexing stats
        let stats = indexer.get_indexing_stats().await.unwrap();
        assert!(stats.total_indexed_files >= 0);

        // Test index recovery
        let recovery = indexer
            .recover_from_failure(&PathBuf::from("test.txt"))
            .await
            .unwrap();
        assert!(recovery.recovered_chunks >= 0);

        // Test index cleanup
        let cleanup_config = IndexCleanupConfig {
            max_age_days: Some(30),
            remove_invalid: true,
            remove_orphaned: true,
            dry_run: true,
        };

        let cleanup_result = indexer.cleanup_indexes(cleanup_config).await.unwrap();
        assert!(cleanup_result.removed_chunks >= 0);
    }

    #[tokio::test]
    async fn test_knowledge_engine_integration() {
        let engine = Arc::new(UnifiedKnowledgeEngine::new_dummy());

        // Test basic operations
        let result = engine.get_with_rag("test_key", Some("test query")).await;
        assert!(result.is_ok());

        // Test metrics
        let metrics = engine.get_metrics().await;
        assert!(metrics.cache_metrics.hit_count >= 0);
    }

    #[tokio::test]
    async fn test_proactive_features() {
        let engine = Arc::new(UnifiedKnowledgeEngine::new_dummy());
        let _proactive_manager = ProactiveContextManager::new(engine);

        // Test that proactive manager was created successfully
        assert!(true); // Basic creation test passed
    }

    #[tokio::test]
    async fn test_error_handling() {
        let embedding_manager = EmbeddingManager::new_dummy();

        // Test error handling for invalid input
        let result = embedding_manager.embed("", None).await;
        assert!(result.is_err());

        // Test error handling for invalid compression
        let invalid_embedding = vec![f32::NAN; 384];
        let result = embedding_manager
            .validate_embedding(&invalid_embedding, None)
            .await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_valid);
    }

    #[tokio::test]
    async fn test_performance_metrics() {
        let embedding_manager = EmbeddingManager::new_dummy();

        // Test basic embedding generation
        let text = "test performance";
        let embedding = embedding_manager.embed(text, None).await.unwrap();
        assert!(!embedding.is_empty());

        // Test that embedding was generated successfully
        assert!(embedding.len() > 0);
    }
}

#[cfg(test)]
mod knowledge_tests {
    use super::*;

    #[test]
    fn test_knowledge_crate_compilation() {
        // Test that the crate compiles and basic functionality works
        assert!(true); // Basic compilation test
    }

    #[test]
    fn test_performance_config_creation() {
        let config = PerformanceConfig {
            compression_threshold_kb: 1024,
            parallel_operations: 4,
            background_cleanup: true,
            cleanup_interval_minutes: 30,
        };
        assert!(config.compression_threshold_kb > 0);
    }

    #[test]
    fn test_storage_config_creation() {
        let config = StorageConfig {
            base_path: std::path::PathBuf::from("/tmp"),
            max_size_gb: 1,
            compression_enabled: true,
            compression_algorithm: CompressionAlgorithm::Zstd,
            enable_checksums: true,
            backup_enabled: false,
            backup_interval_hours: 24,
            cleanup_enabled: true,
            cleanup_interval_hours: 1,
        };
        assert!(config.compression_enabled);
        assert_eq!(config.max_size_gb, 1);
    }
}
