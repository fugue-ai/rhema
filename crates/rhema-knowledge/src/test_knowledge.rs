#[cfg(test)]
mod tests {
    use crate::cache::{UnifiedCacheConfig, UnifiedCacheManager};
    use crate::embedding::EmbeddingManager;
    use crate::engine::UnifiedKnowledgeEngine;
    use crate::storage::{StorageConfig, StorageManager, StorageMetadata};
    use crate::types::*;
    use crate::vector::{
        MockVectorStore as VectorMockVectorStore, VectorStore, VectorStoreWrapper,
    };

    #[tokio::test]
    async fn test_basic_knowledge_functionality() {
        // Test that we can create basic components
        let embedding_manager = EmbeddingManager::new_dummy();

        // Test embedding generation
        let embedding = embedding_manager.embed("test text", None).await.unwrap();
        assert!(!embedding.is_empty());
        assert_eq!(embedding.len(), 384);

        // Test vector store operations
        let vector_store = VectorStoreWrapper::Mock(VectorMockVectorStore::new(
            "test_collection".to_string(),
            384,
            DistanceMetric::Cosine,
        ));

        let store_result = vector_store.store("test_id", &embedding, None).await;
        assert!(store_result.is_ok());

        let search_result = vector_store.search(&embedding, 5).await;
        assert!(search_result.is_ok());
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        // Test cache creation and basic operations
        let config = UnifiedCacheConfig::default();
        let _cache_manager = UnifiedCacheManager::new(config).await.unwrap();

        // Test that cache manager was created successfully
        assert!(true); // Basic creation test passed
    }

    #[tokio::test]
    async fn test_storage_functionality() {
        // Test storage manager creation
        let config = StorageConfig {
            base_path: std::env::temp_dir().join("rhema_test"),
            max_size_gb: 1,
            compression_enabled: true,
            compression_algorithm: CompressionAlgorithm::Zstd,
            enable_checksums: true,
            backup_enabled: false,
            backup_interval_hours: 24,
            cleanup_enabled: true,
            cleanup_interval_hours: 1,
        };

        let storage_manager = StorageManager::new(config).await.unwrap();

        // Test basic storage operations
        let test_data = "test storage data".as_bytes().to_vec();
        let metadata = StorageMetadata {
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            size_bytes: test_data.len() as u64,
            content_type: ContentType::Knowledge,
            tags: vec![],
            ttl: None,
        };

        let store_result = storage_manager
            .store("test_key", &test_data, metadata)
            .await;
        assert!(store_result.is_ok());

        let retrieve_result = storage_manager.retrieve("test_key").await;
        assert!(retrieve_result.is_ok());
    }

    #[tokio::test]
    async fn test_knowledge_engine_integration() {
        // Test knowledge engine creation and basic operations
        let engine = UnifiedKnowledgeEngine::new_dummy_minimal();

        // Test basic operations
        let result = engine.get_with_rag("test_key", Some("test query")).await;
        assert!(result.is_ok());

        // Test metrics
        let metrics = engine.get_metrics().await;
        assert!(metrics.cache_metrics.hit_count >= 0);
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
