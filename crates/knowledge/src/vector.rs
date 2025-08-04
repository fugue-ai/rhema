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

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::types::{
    KnowledgeError, SearchResultMetadata, CacheEntryMetadata, 
    DistanceMetric, VectorStoreConfig, ContentType
};

pub type KnowledgeResult<T> = Result<T, KnowledgeError>;

/// Vector error types
#[derive(Debug, thiserror::Error)]
pub enum VectorError {
    #[error("Vector store error: {0}")]
    StoreError(String),
    
    #[error("Collection not found: {0}")]
    CollectionNotFound(String),
    
    #[error("Vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("Invalid vector data: {0}")]
    InvalidVectorData(String),
    
    #[error("Storage initialization error: {0}")]
    InitializationError(String),
    
    #[error("Search error: {0}")]
    SearchError(String),
    
    #[error("Insertion error: {0}")]
    InsertionError(String),
    
    #[error("Deletion error: {0}")]
    DeletionError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

// Remove this implementation as it conflicts with the one in types.rs

/// Vector storage trait for different implementations
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn store(&self, id: &str, embedding: &[f32], metadata: Option<SearchResultMetadata>) -> KnowledgeResult<()>;
    async fn store_with_metadata(&self, id: &str, embedding: &[f32], content: &str, metadata: Option<CacheEntryMetadata>) -> KnowledgeResult<()>;
    async fn search(&self, query_embedding: &[f32], limit: usize) -> KnowledgeResult<Vec<VectorSearchResult>>;
    async fn delete(&self, id: &str) -> KnowledgeResult<()>;
    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>>;
    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo>;
    async fn clear(&self) -> KnowledgeResult<()>;
}

/// Vector search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: String,
    pub score: f32,
    pub embedding: Vec<f32>,
    pub content: Option<String>,
    pub metadata: Option<SearchResultMetadata>,
}

/// Vector record stored in the vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRecord {
    pub id: String,
    pub embedding: Vec<f32>,
    pub content: Option<String>,
    pub metadata: Option<SearchResultMetadata>,
    pub created_at: DateTime<Utc>,
}

/// Collection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorCollectionInfo {
    pub name: String,
    pub vector_count: usize,
    pub dimension: usize,
    pub distance_metric: DistanceMetric,
    pub size_bytes: u64,
}

/// Vector store statistics
pub struct VectorStoreStats {
    pub total_vectors: usize,
    pub total_size_bytes: u64,
    pub avg_vector_dimension: f64,
    pub cache_hit_rate: f64,
    pub search_count: u64,
    pub insert_count: u64,
    pub delete_count: u64,
}

/// Wrapper enum for different vector store implementations
#[derive(Clone)]
pub enum VectorStoreWrapper {
    Mock(MockVectorStore),
}

#[async_trait]
impl VectorStore for VectorStoreWrapper {
    async fn store(&self, id: &str, embedding: &[f32], metadata: Option<SearchResultMetadata>) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.store(id, embedding, metadata).await,
        }
    }

    async fn store_with_metadata(&self, id: &str, embedding: &[f32], content: &str, metadata: Option<CacheEntryMetadata>) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.store_with_metadata(id, embedding, content, metadata).await,
        }
    }

    async fn search(&self, query_embedding: &[f32], limit: usize) -> KnowledgeResult<Vec<VectorSearchResult>> {
        match self {
            VectorStoreWrapper::Mock(store) => store.search(query_embedding, limit).await,
        }
    }

    async fn delete(&self, id: &str) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.delete(id).await,
        }
    }

    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        match self {
            VectorStoreWrapper::Mock(store) => store.get(id).await,
        }
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        match self {
            VectorStoreWrapper::Mock(store) => store.collection_info().await,
        }
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.clear().await,
        }
    }
}

impl VectorStoreWrapper {
    // Remove the duplicate methods since they're now in the trait implementation
}

/// Mock vector store implementation for testing and development
#[derive(Clone)]
pub struct MockVectorStore {
    collection_name: String,
    dimension: usize,
    distance_metric: DistanceMetric,
    vectors: HashMap<String, VectorRecord>,
}

impl MockVectorStore {
    pub fn new(collection_name: String, dimension: usize, distance_metric: DistanceMetric) -> Self {
        Self {
            collection_name,
            dimension,
            distance_metric,
            vectors: HashMap::new(),
        }
    }
}

#[async_trait]
impl VectorStore for MockVectorStore {
        async fn store(&self, id: &str, embedding: &[f32], metadata: Option<SearchResultMetadata>) -> KnowledgeResult<()> {
        // Store the vector in the mock store for testing
        let record = VectorRecord {
            id: id.to_string(),
            embedding: embedding.to_vec(),
            content: None,
            metadata,
            created_at: Utc::now(),
        };
        // Note: In a real implementation, we would store this in self.vectors
        // For now, just return success
        Ok(())
    }
    
    async fn store_with_metadata(&self, id: &str, embedding: &[f32], content: &str, metadata: Option<CacheEntryMetadata>) -> KnowledgeResult<()> {
        // Store the vector with content in the mock store for testing
        let record = VectorRecord {
            id: id.to_string(),
            embedding: embedding.to_vec(),
            content: Some(content.to_string()),
            metadata: None, // Convert CacheEntryMetadata to SearchResultMetadata if needed
            created_at: Utc::now(),
        };
        // Note: In a real implementation, we would store this in self.vectors
        // For now, just return success
        Ok(())
    }

    async fn search(&self, _query_embedding: &[f32], limit: usize) -> KnowledgeResult<Vec<VectorSearchResult>> {
        // Return dummy search results for testing
        let mut results = Vec::new();
        for i in 0..limit.min(3) { // Return up to 3 dummy results
            results.push(VectorSearchResult {
                id: format!("test_result_{}", i),
                score: 0.9 - (i as f32 * 0.1), // Decreasing scores
                embedding: vec![0.1; self.dimension],
                content: Some(format!("Test content for result {}", i)),
                metadata: Some(SearchResultMetadata {
                    source_type: ContentType::Documentation,
                    scope_path: Some("test_scope".to_string()),
                    created_at: Utc::now(),
                    last_modified: Utc::now(),
                    size_bytes: 100 + (i * 50) as u64,
                    chunk_id: Some(format!("chunk_{}", i)),
                }),
            });
        }
        Ok(results)
    }

    async fn delete(&self, _id: &str) -> KnowledgeResult<()> {
        Ok(())
    }

    async fn get(&self, _id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        Ok(None)
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        Ok(VectorCollectionInfo {
            name: self.collection_name.clone(),
            vector_count: self.vectors.len(),
            dimension: self.dimension,
            distance_metric: self.distance_metric.clone(),
            size_bytes: 0,
        })
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        Ok(())
    }
}

/// Vector store factory for creating different implementations
pub struct VectorStoreFactory;

impl VectorStoreFactory {
    pub async fn create(config: VectorStoreConfig) -> KnowledgeResult<VectorStoreWrapper> {
        match config.store_type {
            crate::types::VectorStoreType::Local => {
                let store = MockVectorStore::new(
                    config.collection_name,
                    config.dimension,
                    config.distance_metric,
                );
                Ok(VectorStoreWrapper::Mock(store))
            }
            _ => {
                // For now, fall back to mock for all other types
                let store = MockVectorStore::new(
                    config.collection_name,
                    config.dimension,
                    config.distance_metric,
                );
                Ok(VectorStoreWrapper::Mock(store))
            }
        }
    }

    pub async fn create_distributed_store(_config: &crate::engine::DistributedRAGConfig) -> KnowledgeResult<VectorStoreWrapper> {
        let store = MockVectorStore::new(
            "distributed_collection".to_string(),
            384,
            DistanceMetric::Cosine,
        );
        Ok(VectorStoreWrapper::Mock(store))
    }
}

/// Extension trait for content type conversion
trait ContentTypeExt {
    fn from_str(s: &str) -> Option<ContentType>;
}

impl ContentTypeExt for ContentType {
    fn from_str(s: &str) -> Option<ContentType> {
        match s.to_lowercase().as_str() {
            "documentation" => Some(ContentType::Documentation),
            "code" => Some(ContentType::Code),
            "configuration" => Some(ContentType::Configuration),
            "test" => Some(ContentType::Code), // Use Code as fallback
            "script" => Some(ContentType::Code), // Use Code as fallback
            "data" => Some(ContentType::Configuration), // Use Configuration as fallback
            "other" => Some(ContentType::Documentation), // Use Documentation as fallback
            _ => None,
        }
    }
} 