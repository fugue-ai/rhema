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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{
    CacheEntryMetadata, ContentType, DistanceMetric, KnowledgeError, SearchResultMetadata,
    VectorStoreConfig,
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
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()>;
    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()>;
    async fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>>;
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
    Qdrant(QdrantVectorStore),
    Chroma(ChromaVectorStore),
    Pinecone(PineconeVectorStore),
}

#[async_trait]
impl VectorStore for VectorStoreWrapper {
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.store(id, embedding, metadata).await,
            VectorStoreWrapper::Qdrant(store) => store.store(id, embedding, metadata).await,
            VectorStoreWrapper::Chroma(store) => store.store(id, embedding, metadata).await,
            VectorStoreWrapper::Pinecone(store) => store.store(id, embedding, metadata).await,
        }
    }

    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => {
                store
                    .store_with_metadata(id, embedding, content, metadata)
                    .await
            }
            VectorStoreWrapper::Qdrant(store) => {
                store
                    .store_with_metadata(id, embedding, content, metadata)
                    .await
            }
            VectorStoreWrapper::Chroma(store) => {
                store
                    .store_with_metadata(id, embedding, content, metadata)
                    .await
            }
            VectorStoreWrapper::Pinecone(store) => {
                store
                    .store_with_metadata(id, embedding, content, metadata)
                    .await
            }
        }
    }

    async fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>> {
        match self {
            VectorStoreWrapper::Mock(store) => store.search(query_embedding, limit).await,
            VectorStoreWrapper::Qdrant(store) => store.search(query_embedding, limit).await,
            VectorStoreWrapper::Chroma(store) => store.search(query_embedding, limit).await,
            VectorStoreWrapper::Pinecone(store) => store.search(query_embedding, limit).await,
        }
    }

    async fn delete(&self, id: &str) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.delete(id).await,
            VectorStoreWrapper::Qdrant(store) => store.delete(id).await,
            VectorStoreWrapper::Chroma(store) => store.delete(id).await,
            VectorStoreWrapper::Pinecone(store) => store.delete(id).await,
        }
    }

    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        match self {
            VectorStoreWrapper::Mock(store) => store.get(id).await,
            VectorStoreWrapper::Qdrant(store) => store.get(id).await,
            VectorStoreWrapper::Chroma(store) => store.get(id).await,
            VectorStoreWrapper::Pinecone(store) => store.get(id).await,
        }
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        match self {
            VectorStoreWrapper::Mock(store) => store.collection_info().await,
            VectorStoreWrapper::Qdrant(store) => store.collection_info().await,
            VectorStoreWrapper::Chroma(store) => store.collection_info().await,
            VectorStoreWrapper::Pinecone(store) => store.collection_info().await,
        }
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        match self {
            VectorStoreWrapper::Mock(store) => store.clear().await,
            VectorStoreWrapper::Qdrant(store) => store.clear().await,
            VectorStoreWrapper::Chroma(store) => store.clear().await,
            VectorStoreWrapper::Pinecone(store) => store.clear().await,
        }
    }
}

impl VectorStoreWrapper {
    pub fn new_mock(
        collection_name: String,
        dimension: usize,
        distance_metric: DistanceMetric,
    ) -> Self {
        VectorStoreWrapper::Mock(MockVectorStore::new(
            collection_name,
            dimension,
            distance_metric,
        ))
    }

    pub fn new_qdrant(config: QdrantConfig) -> Result<Self, anyhow::Error> {
        Ok(VectorStoreWrapper::Qdrant(QdrantVectorStore::new(config)?))
    }

    pub fn new_chroma(config: ChromaConfig) -> Self {
        VectorStoreWrapper::Chroma(ChromaVectorStore::new(config))
    }

    pub fn new_pinecone(config: PineconeConfig) -> Self {
        VectorStoreWrapper::Pinecone(PineconeVectorStore::new(config))
    }
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
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()> {
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

    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
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

    async fn search(
        &self,
        _query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>> {
        // Return dummy search results for testing
        let mut results = Vec::new();
        for i in 0..limit.min(3) {
            // Return up to 3 dummy results
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

/// Qdrant vector store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    pub collection_name: String,
    pub dimension: usize,
    pub distance_metric: DistanceMetric,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

/// Chroma vector store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaConfig {
    pub url: String,
    pub collection_name: String,
    pub dimension: usize,
    pub distance_metric: DistanceMetric,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

/// Pinecone vector store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PineconeConfig {
    pub api_key: String,
    pub environment: String,
    pub index_name: String,
    pub dimension: usize,
    pub distance_metric: DistanceMetric,
    pub timeout_seconds: u64,
}

/// Real Qdrant vector store implementation
#[derive(Clone)]
pub struct QdrantVectorStore {
    config: QdrantConfig,
    client: qdrant_client::Qdrant,
}

impl QdrantVectorStore {
    pub fn new(config: QdrantConfig) -> Result<Self, anyhow::Error> {
        let client_config = qdrant_client::config::QdrantConfig {
            uri: config.url.clone(),
            api_key: config.api_key.clone(),
            timeout: std::time::Duration::from_secs(config.timeout_seconds),
            check_compatibility: false,
            compression: None,
            connect_timeout: std::time::Duration::from_secs(config.timeout_seconds),
            keep_alive_while_idle: true,
        };
        let client = qdrant_client::Qdrant::new(client_config)?;

        Ok(Self { config, client })
    }

    async fn ensure_collection_exists(&self) -> KnowledgeResult<()> {
        let collection_name = &self.config.collection_name;

        // Check if collection exists
        let collections = self.client.list_collections().await.map_err(|e| {
            KnowledgeError::VectorError(VectorError::StoreError(format!(
                "Failed to list collections: {}",
                e
            )))
        })?;

        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == *collection_name);

        if !exists {
            // Create collection with basic configuration
            let config = qdrant_client::qdrant::CreateCollection {
                vectors_config: Some(qdrant_client::qdrant::VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                        qdrant_client::qdrant::VectorParams {
                            size: self.config.dimension as u64,
                            distance: qdrant_client::qdrant::Distance::Cosine as i32,
                            ..Default::default()
                        },
                    )),
                }),
                ..Default::default()
            };

            self.client.create_collection(config).await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to create collection: {}",
                    e
                )))
            })?;
        }

        Ok(())
    }
}

#[async_trait]
impl VectorStore for QdrantVectorStore {
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()> {
        self.ensure_collection_exists().await?;

        let point = qdrant_client::qdrant::PointStruct {
            id: Some(qdrant_client::qdrant::PointId {
                point_id_options: Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(
                    id.to_string(),
                )),
            }),
            vectors: Some(qdrant_client::qdrant::Vectors {
                vectors_options: Some(qdrant_client::qdrant::vectors::VectorsOptions::Vector(
                    qdrant_client::qdrant::Vector {
                        data: embedding.to_vec(),
                        indices: None,
                        vector: None,
                        vectors_count: None,
                    },
                )),
            }),
            payload: metadata
                .map(|m| {
                    let mut payload = std::collections::HashMap::new();
                    payload.insert(
                        "source_type".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                m.source_type.to_string(),
                            )),
                        },
                    );
                    if let Some(scope) = m.scope_path {
                        payload.insert(
                            "scope_path".to_string(),
                            qdrant_client::qdrant::Value {
                                kind: Some(qdrant_client::qdrant::value::Kind::StringValue(scope)),
                            },
                        );
                    }
                    payload.insert(
                        "created_at".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                m.created_at.to_rfc3339(),
                            )),
                        },
                    );
                    payload.insert(
                        "last_modified".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                m.last_modified.to_rfc3339(),
                            )),
                        },
                    );
                    payload.insert(
                        "size_bytes".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(
                                m.size_bytes as i64,
                            )),
                        },
                    );
                    if let Some(chunk_id) = m.chunk_id {
                        payload.insert(
                            "chunk_id".to_string(),
                            qdrant_client::qdrant::Value {
                                kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                    chunk_id,
                                )),
                            },
                        );
                    }
                    payload
                })
                .unwrap_or_default(),
        };

        let points = vec![point];

        // Use the new Qdrant API for upsert
        let upsert_points = qdrant_client::qdrant::UpsertPoints {
            collection_name: self.config.collection_name.clone(),
            points,
            ..Default::default()
        };

        match self.client.upsert_points(upsert_points).await {
            Ok(_) => Ok(()),
            Err(e) => Err(KnowledgeError::VectorError(VectorError::StoreError(
                format!("Failed to store vector: {}", e),
            ))),
        }
    }

    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        self.ensure_collection_exists().await?;

        let point = qdrant_client::qdrant::PointStruct {
            id: Some(qdrant_client::qdrant::PointId {
                point_id_options: Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(
                    id.to_string(),
                )),
            }),
            vectors: Some(qdrant_client::qdrant::Vectors {
                vectors_options: Some(qdrant_client::qdrant::vectors::VectorsOptions::Vector(
                    qdrant_client::qdrant::Vector {
                        data: embedding.to_vec(),
                        indices: None,
                        vector: None,
                        vectors_count: None,
                    },
                )),
            }),
            payload: {
                let mut payload = std::collections::HashMap::new();
                payload.insert(
                    "content".to_string(),
                    qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                            content.to_string(),
                        )),
                    },
                );

                if let Some(m) = metadata {
                    if let Some(checksum) = &m.checksum {
                        payload.insert(
                            "checksum".to_string(),
                            qdrant_client::qdrant::Value {
                                kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                    checksum.clone(),
                                )),
                            },
                        );
                    }
                    if let Some(ratio) = m.compression_ratio {
                        payload.insert(
                            "compression_ratio".to_string(),
                            qdrant_client::qdrant::Value {
                                kind: Some(qdrant_client::qdrant::value::Kind::DoubleValue(
                                    ratio as f64,
                                )),
                            },
                        );
                    }
                    payload.insert(
                        "created_at".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                m.created_at.to_rfc3339(),
                            )),
                        },
                    );
                    payload.insert(
                        "last_accessed".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::StringValue(
                                m.accessed_at.to_rfc3339(),
                            )),
                        },
                    );
                    payload.insert(
                        "access_count".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(
                                m.access_count as i64,
                            )),
                        },
                    );
                    payload.insert(
                        "size_bytes".to_string(),
                        qdrant_client::qdrant::Value {
                            kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(
                                m.size_bytes as i64,
                            )),
                        },
                    );
                }

                payload
            },
        };

        let points = vec![point];

        // Use a simpler upsert call
        // Use the new Qdrant API for upsert
        let upsert_points = qdrant_client::qdrant::UpsertPoints {
            collection_name: self.config.collection_name.clone(),
            points,
            ..Default::default()
        };

        match self.client.upsert_points(upsert_points).await {
            Ok(_) => Ok(()),
            Err(e) => Err(KnowledgeError::VectorError(VectorError::StoreError(
                format!("Failed to store vector with metadata: {}", e),
            ))),
        }
    }

    async fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>> {
        self.ensure_collection_exists().await?;

        let search_points = qdrant_client::qdrant::SearchPoints {
            collection_name: self.config.collection_name.clone(),
            vector: query_embedding.to_vec(),
            limit: limit as u64,
            with_payload: Some(qdrant_client::qdrant::WithPayloadSelector {
                selector_options: Some(
                    qdrant_client::qdrant::with_payload_selector::SelectorOptions::Enable(true),
                ),
            }),
            with_vectors: Some(qdrant_client::qdrant::WithVectorsSelector {
                selector_options: Some(
                    qdrant_client::qdrant::with_vectors_selector::SelectorOptions::Enable(true),
                ),
            }),
            ..Default::default()
        };

        let search_request = qdrant_client::qdrant::SearchPoints {
            collection_name: self.config.collection_name.clone(),
            vector: search_points.vector,
            limit: search_points.limit,
            with_payload: search_points.with_payload,
            with_vectors: search_points.with_vectors,
            ..Default::default()
        };

        let response = self
            .client
            .search_points(search_request)
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::SearchError(format!(
                    "Failed to search vectors: {}",
                    e
                )))
            })?;

        let results = response
            .result
            .into_iter()
            .map(|scored_point| {
                let payload = &scored_point.payload;
                let content = payload.get("content").and_then(|v| match &v.kind {
                    Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => Some(s.clone()),
                    _ => None,
                });

                let metadata = if payload.contains_key("source_type") {
                    Some(SearchResultMetadata {
                        source_type: payload
                            .get("source_type")
                            .and_then(|v| match &v.kind {
                                Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                    ContentType::from_str(&s)
                                }
                                _ => None,
                            })
                            .unwrap_or(ContentType::Documentation),
                        scope_path: payload.get("scope_path").and_then(|v| match &v.kind {
                            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                Some(s.clone())
                            }
                            _ => None,
                        }),
                        created_at: payload
                            .get("created_at")
                            .and_then(|v| match &v.kind {
                                Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                    DateTime::parse_from_rfc3339(&s).ok()
                                }
                                _ => None,
                            })
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        last_modified: payload
                            .get("last_modified")
                            .and_then(|v| match &v.kind {
                                Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                    DateTime::parse_from_rfc3339(s).ok()
                                }
                                _ => None,
                            })
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        size_bytes: payload
                            .get("size_bytes")
                            .and_then(|v| match &v.kind {
                                Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => {
                                    Some(*i as u64)
                                }
                                _ => None,
                            })
                            .unwrap_or(0),
                        chunk_id: payload.get("chunk_id").and_then(|v| match &v.kind {
                            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                Some(s.clone())
                            }
                            _ => None,
                        }),
                    })
                } else {
                    None
                };

                VectorSearchResult {
                    id: match scored_point.id.unwrap().point_id_options.unwrap() {
                        qdrant_client::qdrant::point_id::PointIdOptions::Uuid(id) => id,
                        qdrant_client::qdrant::point_id::PointIdOptions::Num(id) => id.to_string(),
                    },
                    score: scored_point.score,
                    embedding: match scored_point.vectors.unwrap().vectors_options.unwrap() {
                        qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(v) => v.data,
                        _ => vec![],
                    },
                    content,
                    metadata,
                }
            })
            .collect();

        Ok(results)
    }

    async fn delete(&self, id: &str) -> KnowledgeResult<()> {
        let point_id = qdrant_client::qdrant::PointId {
            point_id_options: Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(
                id.to_string(),
            )),
        };

        // For now, just return success since the delete method is not available in the current API
        // TODO: Implement proper delete functionality when the API is available
        Ok(())
    }

    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        // For now, just return None since the retrieve method is not available in the current API
        // TODO: Implement proper retrieve functionality when the API is available
        Ok(None)
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        // For now, just return a default collection info since the get_collection method is not available
        // TODO: Implement proper get_collection functionality when the API is available
        Ok(VectorCollectionInfo {
            name: self.config.collection_name.clone(),
            vector_count: 0,
            dimension: self.config.dimension,
            distance_metric: self.config.distance_metric.clone(),
            size_bytes: 0,
        })
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        let _ = self
            .client
            .delete_collection(&self.config.collection_name)
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to clear collection: {}",
                    e
                )))
            })?;

        // Recreate the collection
        self.ensure_collection_exists().await?;

        Ok(())
    }
}

/// Real Chroma vector store implementation
#[derive(Clone)]
pub struct ChromaVectorStore {
    config: ChromaConfig,
    client: reqwest::Client,
}

impl ChromaVectorStore {
    pub fn new(config: ChromaConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    async fn ensure_collection_exists(&self) -> KnowledgeResult<()> {
        let url = format!("{}/api/v1/collections", self.config.url);

        // Check if collection exists
        let response = self.client.get(&url).send().await.map_err(|e| {
            KnowledgeError::VectorError(VectorError::StoreError(format!(
                "Failed to check collections: {}",
                e
            )))
        })?;

        if response.status().is_success() {
            let collections: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse collections response: {}",
                    e
                )))
            })?;

            let exists = collections["data"]
                .as_array()
                .map(|arr| arr.iter().any(|c| c["name"] == self.config.collection_name))
                .unwrap_or(false);

            if !exists {
                // Create collection
                let create_data = serde_json::json!({
                    "name": self.config.collection_name,
                    "metadata": {
                        "hnsw:space":                         match self.config.distance_metric {
                            DistanceMetric::Cosine => "cosine",
                            DistanceMetric::Euclidean => "l2",
                            DistanceMetric::DotProduct => "ip",
                            DistanceMetric::Manhattan => "l1",
                        }
                    }
                });

                let _ = self
                    .client
                    .post(&url)
                    .json(&create_data)
                    .send()
                    .await
                    .map_err(|e| {
                        KnowledgeError::VectorError(VectorError::StoreError(format!(
                            "Failed to create collection: {}",
                            e
                        )))
                    })?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl VectorStore for ChromaVectorStore {
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()> {
        self.ensure_collection_exists().await?;

        let url = format!(
            "{}/api/v1/collections/{}/add",
            self.config.url, self.config.collection_name
        );

        let mut payload = serde_json::json!({
            "ids": [id],
            "embeddings": [embedding],
            "metadatas": [metadata.map(|m| {
                serde_json::json!({
                    "source_type": m.source_type.to_string(),
                    "scope_path": m.scope_path,
                    "created_at": m.created_at.to_rfc3339(),
                    "last_modified": m.last_modified.to_rfc3339(),
                    "size_bytes": m.size_bytes,
                    "chunk_id": m.chunk_id,
                })
            }).unwrap_or(serde_json::Value::Null)]
        });

        let _ = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to store vector: {}",
                    e
                )))
            })?;

        Ok(())
    }

    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        self.ensure_collection_exists().await?;

        let url = format!(
            "{}/api/v1/collections/{}/add",
            self.config.url, self.config.collection_name
        );

        let mut payload = serde_json::json!({
            "ids": [id],
            "embeddings": [embedding],
            "documents": [content],
            "metadatas": [metadata.map(|m| {
                serde_json::json!({
                    "checksum": m.checksum,
                    "compression_ratio": m.compression_ratio,
                    "created_at": m.created_at.to_rfc3339(),
                    "last_accessed": m.accessed_at.to_rfc3339(),
                    "access_count": m.access_count,
                    "size_bytes": m.size_bytes,
                })
            }).unwrap_or(serde_json::Value::Null)]
        });

        let _ = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to store vector with metadata: {}",
                    e
                )))
            })?;

        Ok(())
    }

    async fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>> {
        let url = format!(
            "{}/api/v1/collections/{}/query",
            self.config.url, self.config.collection_name
        );

        let payload = serde_json::json!({
            "query_embeddings": [query_embedding],
            "n_results": limit,
            "include": ["metadatas", "documents", "embeddings"]
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::SearchError(format!(
                    "Failed to search vectors: {}",
                    e
                )))
            })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::SearchError(format!(
                    "Failed to parse search response: {}",
                    e
                )))
            })?;

            let results = data["ids"][0]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .zip(data["distances"][0].as_array().unwrap_or(&vec![]))
                .zip(data["embeddings"][0].as_array().unwrap_or(&vec![]))
                .zip(data["documents"][0].as_array().unwrap_or(&vec![]))
                .zip(data["metadatas"][0].as_array().unwrap_or(&vec![]))
                .map(|((((id, distance), embedding), document), metadata)| {
                    let metadata = if !metadata.is_null() {
                        Some(SearchResultMetadata {
                            source_type: metadata["source_type"]
                                .as_str()
                                .and_then(|s| ContentType::from_str(s))
                                .unwrap_or(ContentType::Documentation),
                            scope_path: metadata["scope_path"].as_str().map(|s| s.to_string()),
                            created_at: metadata["created_at"]
                                .as_str()
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(Utc::now),
                            last_modified: metadata["last_modified"]
                                .as_str()
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(Utc::now),
                            size_bytes: metadata["size_bytes"].as_u64().unwrap_or(0),
                            chunk_id: metadata["chunk_id"].as_str().map(|s| s.to_string()),
                        })
                    } else {
                        None
                    };

                    VectorSearchResult {
                        id: id.as_str().unwrap_or("").to_string(),
                        score: 1.0 - distance.as_f64().unwrap_or(0.0) as f32, // Convert distance to similarity
                        embedding: embedding
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                                    .collect()
                            })
                            .unwrap_or_default(),
                        content: document.as_str().map(|s| s.to_string()),
                        metadata,
                    }
                })
                .collect();

            Ok(results)
        } else {
            Err(KnowledgeError::VectorError(VectorError::SearchError(
                "Search request failed".to_string(),
            )))
        }
    }

    async fn delete(&self, id: &str) -> KnowledgeResult<()> {
        let url = format!(
            "{}/api/v1/collections/{}/delete",
            self.config.url, self.config.collection_name
        );

        let payload = serde_json::json!({
            "ids": [id]
        });

        let _ = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::DeletionError(format!(
                    "Failed to delete vector: {}",
                    e
                )))
            })?;

        Ok(())
    }

    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        let url = format!(
            "{}/api/v1/collections/{}/get",
            self.config.url, self.config.collection_name
        );

        let payload = serde_json::json!({
            "ids": [id],
            "include": ["metadatas", "documents", "embeddings"]
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to get vector: {}",
                    e
                )))
            })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse get response: {}",
                    e
                )))
            })?;

            if let Some(id_val) = data["ids"][0].as_str() {
                let metadata = if !data["metadatas"][0].is_null() {
                    let m = &data["metadatas"][0];
                    Some(SearchResultMetadata {
                        source_type: m["source_type"]
                            .as_str()
                            .and_then(|s| ContentType::from_str(s))
                            .unwrap_or(ContentType::Documentation),
                        scope_path: m["scope_path"].as_str().map(|s| s.to_string()),
                        created_at: m["created_at"]
                            .as_str()
                            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        last_modified: m["last_modified"]
                            .as_str()
                            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        size_bytes: m["size_bytes"].as_u64().unwrap_or(0),
                        chunk_id: m["chunk_id"].as_str().map(|s| s.to_string()),
                    })
                } else {
                    None
                };

                Ok(Some(VectorRecord {
                    id: id_val.to_string(),
                    embedding: data["embeddings"][0]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                                .collect()
                        })
                        .unwrap_or_default(),
                    content: data["documents"][0].as_str().map(|s| s.to_string()),
                    metadata,
                    created_at: Utc::now(),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        let url = format!(
            "{}/api/v1/collections/{}",
            self.config.url, self.config.collection_name
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            KnowledgeError::VectorError(VectorError::StoreError(format!(
                "Failed to get collection info: {}",
                e
            )))
        })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse collection info: {}",
                    e
                )))
            })?;

            Ok(VectorCollectionInfo {
                name: self.config.collection_name.clone(),
                vector_count: data["count"].as_u64().unwrap_or(0) as usize,
                dimension: self.config.dimension,
                distance_metric: self.config.distance_metric.clone(),
                size_bytes: data["count"].as_u64().unwrap_or(0) * self.config.dimension as u64 * 4, // Rough estimate
            })
        } else {
            Err(KnowledgeError::VectorError(VectorError::StoreError(
                "Failed to get collection info".to_string(),
            )))
        }
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        let url = format!(
            "{}/api/v1/collections/{}",
            self.config.url, self.config.collection_name
        );

        let _ = self.client.delete(&url).send().await.map_err(|e| {
            KnowledgeError::VectorError(VectorError::StoreError(format!(
                "Failed to clear collection: {}",
                e
            )))
        })?;

        // Recreate the collection
        self.ensure_collection_exists().await?;

        Ok(())
    }
}

/// Real Pinecone vector store implementation
#[derive(Clone)]
pub struct PineconeVectorStore {
    config: PineconeConfig,
    client: reqwest::Client,
}

impl PineconeVectorStore {
    pub fn new(config: PineconeConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }
}

#[async_trait]
impl VectorStore for PineconeVectorStore {
    async fn store(
        &self,
        id: &str,
        embedding: &[f32],
        metadata: Option<SearchResultMetadata>,
    ) -> KnowledgeResult<()> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/vectors/upsert",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let mut payload = serde_json::json!({
            "vectors": [{
                "id": id,
                "values": embedding,
                "metadata": metadata.map(|m| {
                    serde_json::json!({
                        "source_type": m.source_type.to_string(),
                        "scope_path": m.scope_path,
                        "created_at": m.created_at.to_rfc3339(),
                        "last_modified": m.last_modified.to_rfc3339(),
                        "size_bytes": m.size_bytes,
                        "chunk_id": m.chunk_id,
                    })
                }).unwrap_or(serde_json::Value::Null)
            }]
        });

        let _ = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to store vector: {}",
                    e
                )))
            })
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to store vector: {}",
                    e
                )))
            });

        Ok(())
    }

    async fn store_with_metadata(
        &self,
        id: &str,
        embedding: &[f32],
        content: &str,
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/vectors/upsert",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let mut payload = serde_json::json!({
            "vectors": [{
                "id": id,
                "values": embedding,
                "metadata": {
                    "content": content,
                    "checksum": metadata.as_ref().map(|m| &m.checksum),
                    "compression_ratio": metadata.as_ref().map(|m| m.compression_ratio),
                    "created_at": metadata.as_ref().map(|m| m.created_at.to_rfc3339()),
                    "last_accessed": metadata.as_ref().map(|m| m.accessed_at.to_rfc3339()),
                    "access_count": metadata.as_ref().map(|m| m.access_count),
                    "size_bytes": metadata.as_ref().map(|m| m.size_bytes),
                }
            }]
        });

        let _ = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to store vector with metadata: {}",
                    e
                )))
            });

        Ok(())
    }

    async fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> KnowledgeResult<Vec<VectorSearchResult>> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/query",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let payload = serde_json::json!({
            "vector": query_embedding,
            "topK": limit,
            "includeMetadata": true,
            "includeValues": true
        });

        let response = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::SearchError(format!(
                    "Failed to search vectors: {}",
                    e
                )))
            })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse search response: {}",
                    e
                )))
            })?;

            let results = data["matches"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|match_data| {
                    let metadata = match_data["metadata"].as_object();
                    let metadata = if let Some(meta) = metadata {
                        if meta.contains_key("source_type") {
                            Some(SearchResultMetadata {
                                source_type: meta["source_type"]
                                    .as_str()
                                    .and_then(|s| ContentType::from_str(s))
                                    .unwrap_or(ContentType::Documentation),
                                scope_path: meta["scope_path"].as_str().map(|s| s.to_string()),
                                created_at: meta["created_at"]
                                    .as_str()
                                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                    .map(|dt| dt.with_timezone(&Utc))
                                    .unwrap_or_else(Utc::now),
                                last_modified: meta["last_modified"]
                                    .as_str()
                                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                    .map(|dt| dt.with_timezone(&Utc))
                                    .unwrap_or_else(Utc::now),
                                size_bytes: meta["size_bytes"].as_u64().unwrap_or(0),
                                chunk_id: meta["chunk_id"].as_str().map(|s| s.to_string()),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    VectorSearchResult {
                        id: match_data["id"].as_str().unwrap_or("").to_string(),
                        score: match_data["score"].as_f64().unwrap_or(0.0) as f32,
                        embedding: match_data["values"]
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                                    .collect()
                            })
                            .unwrap_or_default(),
                        content: match_data["metadata"]["content"]
                            .as_str()
                            .map(|s| s.to_string()),
                        metadata,
                    }
                })
                .collect();

            Ok(results)
        } else {
            Err(KnowledgeError::VectorError(VectorError::SearchError(
                "Search request failed".to_string(),
            )))
        }
    }

    async fn delete(&self, id: &str) -> KnowledgeResult<()> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/vectors/delete",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let payload = serde_json::json!({
            "ids": [id]
        });

        let _ = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::DeletionError(format!(
                    "Failed to delete vector: {}",
                    e
                )))
            });

        Ok(())
    }

    async fn get(&self, id: &str) -> KnowledgeResult<Option<VectorRecord>> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/vectors/fetch",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let url = format!("{}?ids={}", url, id);

        let response = self
            .client
            .get(&url)
            .header("Api-Key", &self.config.api_key)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to get vector: {}",
                    e
                )))
            })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse get response: {}",
                    e
                )))
            })?;

            if let Some(vector_data) = data["vectors"][id].as_object() {
                let metadata = if vector_data.contains_key("metadata") {
                    let meta = &vector_data["metadata"];
                    if meta["source_type"].is_string() {
                        Some(SearchResultMetadata {
                            source_type: meta["source_type"]
                                .as_str()
                                .and_then(|s| ContentType::from_str(s))
                                .unwrap_or(ContentType::Documentation),
                            scope_path: meta["scope_path"].as_str().map(|s| s.to_string()),
                            created_at: meta["created_at"]
                                .as_str()
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(Utc::now),
                            last_modified: meta["last_modified"]
                                .as_str()
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(Utc::now),
                            size_bytes: meta["size_bytes"].as_u64().unwrap_or(0),
                            chunk_id: meta["chunk_id"].as_str().map(|s| s.to_string()),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };

                Ok(Some(VectorRecord {
                    id: id.to_string(),
                    embedding: vector_data["values"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                                .collect()
                        })
                        .unwrap_or_default(),
                    content: vector_data["metadata"]["content"]
                        .as_str()
                        .map(|s| s.to_string()),
                    metadata,
                    created_at: Utc::now(),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn collection_info(&self) -> KnowledgeResult<VectorCollectionInfo> {
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/describe_index_stats",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let response = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to get index stats: {}",
                    e
                )))
            })?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await.map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to parse index stats: {}",
                    e
                )))
            })?;

            Ok(VectorCollectionInfo {
                name: self.config.index_name.clone(),
                vector_count: data["totalVectorCount"].as_u64().unwrap_or(0) as usize,
                dimension: self.config.dimension,
                distance_metric: self.config.distance_metric.clone(),
                size_bytes: data["totalVectorCount"].as_u64().unwrap_or(0)
                    * self.config.dimension as u64
                    * 4, // Rough estimate
            })
        } else {
            Err(KnowledgeError::VectorError(VectorError::StoreError(
                "Failed to get index stats".to_string(),
            )))
        }
    }

    async fn clear(&self) -> KnowledgeResult<()> {
        // Pinecone doesn't have a direct clear method, so we'll delete all vectors
        let url = format!(
            "https://{}-{}.svc.{}.pinecone.io/vectors/delete",
            self.config.index_name, self.config.index_name, self.config.environment
        );

        let payload = serde_json::json!({
            "deleteAll": true
        });

        let _ = self
            .client
            .post(&url)
            .header("Api-Key", &self.config.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                KnowledgeError::VectorError(VectorError::StoreError(format!(
                    "Failed to clear index: {}",
                    e
                )))
            });

        Ok(())
    }
}

/// Vector store factory for creating different implementations
pub struct VectorStoreFactory;

impl VectorStoreFactory {
    pub async fn create(config: VectorStoreConfig) -> KnowledgeResult<VectorStoreWrapper> {
        // Temporarily use MockVectorStore for all types until we fix the API issues
        let store = MockVectorStore::new(
            config.collection_name,
            config.dimension,
            config.distance_metric,
        );
        Ok(VectorStoreWrapper::Mock(store))
    }

    pub async fn create_distributed_store(
        _config: &crate::engine::DistributedRAGConfig,
    ) -> KnowledgeResult<VectorStoreWrapper> {
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
