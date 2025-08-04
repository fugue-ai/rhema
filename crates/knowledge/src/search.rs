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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};
use chrono::{DateTime, Utc};

use crate::types::{
    ContentType, KnowledgeResult, CacheEntryMetadata, SemanticInfo,
    CompressionAlgorithm, DistanceMetric, CacheTier, SearchResultMetadata,
    SemanticResult, CacheInfo, SemanticSearchConfig,
};

use super::{
    embedding::EmbeddingManager,
    vector::{VectorSearchResult, VectorStore},
};

/// Error types for search operations
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    
    #[error("Vector search error: {0}")]
    VectorSearchError(String),
    
    #[error("Query processing error: {0}")]
    QueryProcessingError(String),
    
    #[error("Result processing error: {0}")]
    ResultProcessingError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}

/// Semantic search engine
pub struct SemanticSearchEngine {
    embedding_manager: Arc<EmbeddingManager>,
    vector_store: Arc<dyn VectorStore>,
    config: SemanticSearchConfig,
}

impl SemanticSearchEngine {
    pub fn new_dummy() -> Self {
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
                    vector_store: Arc::new(crate::vector::MockVectorStore::new(
            "search_mock_collection".to_string(),
            1536,
            crate::types::DistanceMetric::Cosine,
        )),
            config: SemanticSearchConfig::default(),
        }
    }
    
    pub async fn new(
        embedding_manager: Arc<EmbeddingManager>,
        vector_store: Arc<dyn VectorStore>,
        config: SemanticSearchConfig,
    ) -> KnowledgeResult<Self> {
        info!("Initializing semantic search engine");
        
        Ok(Self {
            embedding_manager,
            vector_store,
            config,
        })
    }
    
    /// Perform semantic search
    #[instrument(skip(self, query))]
    pub async fn search_semantic(&self, query: &str, limit: usize) -> KnowledgeResult<Vec<SemanticResult>> {
        if query.trim().is_empty() {
            return Err(SearchError::InvalidQuery("Query cannot be empty".to_string()).into());
        }
        
        // Generate query embedding
        let query_embedding = self.embedding_manager.embed(query, None).await?;
        
        // Search in vector store
        let vector_results = self.vector_store.search(&query_embedding, limit).await?;
        
        // Convert to semantic results
        let semantic_results = self.convert_to_semantic_results(vector_results, query).await?;
        
        // Filter by similarity threshold
        let filtered_results: Vec<_> = semantic_results
            .into_iter()
            .filter(|result| result.relevance_score >= self.config.similarity_threshold)
            .collect();
        
        debug!("Semantic search returned {} results for query: {}", filtered_results.len(), query);
        Ok(filtered_results)
    }
    
    /// Perform search with reranking for better result quality
    pub async fn search_with_reranking(&self, query: &str, limit: usize) -> KnowledgeResult<Vec<SemanticResult>> {
        if query.trim().is_empty() {
            return Err(SearchError::InvalidQuery("Query cannot be empty".to_string()).into());
        }
        
        // Get initial semantic results
        let initial_results = self.search_semantic(query, limit * 2).await?;
        
        // Apply reranking
        let reranked_results = self.rerank_results(&initial_results, query).await?;
        
        // Return top results
        Ok(reranked_results.into_iter().take(limit).collect())
    }
    
    /// Perform keyword search for exact matches
    pub async fn search_keyword(&self, query: &str, limit: usize) -> KnowledgeResult<Vec<SemanticResult>> {
        if query.trim().is_empty() {
            return Err(SearchError::InvalidQuery("Query cannot be empty".to_string()).into());
        }
        
        // Extract keywords from query
        let keywords = self.extract_keywords(query);
        
        // Search for exact keyword matches
        let mut keyword_results = Vec::new();
        for keyword in &keywords {
            let results = self.search_exact_keyword(keyword, limit).await?;
            keyword_results.extend(results);
        }
        
        // Deduplicate and rank by keyword frequency
        let ranked_results = self.rank_by_keyword_frequency(keyword_results, &keywords).await?;
        
        Ok(ranked_results.into_iter().take(limit).collect())
    }
    
    /// Perform hybrid search combining semantic and keyword search
    pub async fn search_hybrid(&self, query: &str, limit: usize, semantic_weight: f32) -> KnowledgeResult<Vec<SemanticResult>> {
        if query.trim().is_empty() {
            return Err(SearchError::InvalidQuery("Query cannot be empty".to_string()).into());
        }
        
        let keyword_weight = 1.0 - semantic_weight;
        
        // Perform both searches
        let semantic_results = self.search_semantic(query, limit).await?;
        let keyword_results = self.search_keyword(query, limit).await?;
        
        // Combine and rank results
        let combined_results = self.combine_search_results(
            &semantic_results,
            &keyword_results,
            semantic_weight,
            keyword_weight,
        ).await?;
        
        Ok(combined_results.into_iter().take(limit).collect())
    }
    
    /// Search by content type
    #[instrument(skip(self, query, content_type))]
    pub async fn search_by_content_type(
        &self,
        query: &str,
        content_type: ContentType,
        limit: usize,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let all_results = self.search_semantic(query, limit * 2).await?;
        
        let filtered_results = all_results
            .into_iter()
            .filter(|result| result.metadata.source_type == content_type)
            .take(limit)
            .collect();
        
        Ok(filtered_results)
    }
    
    /// Search by scope
    #[instrument(skip(self, query, scope_path))]
    pub async fn search_by_scope(
        &self,
        query: &str,
        scope_path: &str,
        limit: usize,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let all_results = self.search_semantic(query, limit * 2).await?;
        
        let filtered_results = all_results
            .into_iter()
            .filter(|result| {
                result.metadata.scope_path.as_ref().map_or(false, |path| path == scope_path)
            })
            .take(limit)
            .collect();
        
        Ok(filtered_results)
    }
    
    /// Convert vector search results to semantic results
    async fn convert_to_semantic_results(
        &self,
        vector_results: Vec<VectorSearchResult>,
        query: &str,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let mut semantic_results = Vec::new();
        
        for result in vector_results {
            let content = result.content.clone().unwrap_or_default();
            let semantic_result = SemanticResult {
                cache_key: result.id.clone(),
                content: content.clone(),
                embedding: result.embedding,
                relevance_score: result.score,
                semantic_tags: self.extract_semantic_tags(&content).await?,
                metadata: result.metadata.unwrap_or_else(|| SearchResultMetadata {
                    source_type: ContentType::Unknown,
                    scope_path: None,
                    created_at: chrono::Utc::now(),
                    last_modified: chrono::Utc::now(),
                    size_bytes: 0,
                    chunk_id: None,
                }),
                cache_info: None, // Will be populated by the engine
            };
            
            semantic_results.push(semantic_result);
        }
        
        Ok(semantic_results)
    }
    
    /// Extract semantic tags from content
    async fn extract_semantic_tags(&self, content: &str) -> KnowledgeResult<Vec<String>> {
        // Simple keyword extraction
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut tags = Vec::new();
        
        for word in words.iter().take(10) {
            if word.len() > 3 {
                tags.push(word.to_lowercase());
            }
        }
        
        Ok(tags)
    }
    
    /// Rerank search results for better relevance
    async fn rerank_results(&self, results: &[SemanticResult], query: &str) -> KnowledgeResult<Vec<SemanticResult>> {
        let mut reranked_results = results.to_vec();
        
        // Apply multiple reranking factors
        for result in &mut reranked_results {
            let mut new_score = result.relevance_score;
            
            // Boost by recency
            new_score *= self.calculate_recency_boost(&result.metadata).await?;
            
            // Boost by content type relevance
            new_score *= self.calculate_content_type_boost(result, query).await?;
            
            // Boost by semantic tag relevance
            new_score *= self.calculate_semantic_tag_boost(&result.semantic_tags, query).await?;
            
            // Penalize by content length (prefer concise results)
            new_score *= self.calculate_length_penalty(&result.content).await?;
            
            result.relevance_score = new_score.min(1.0);
        }
        
        // Sort by new scores
        reranked_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(reranked_results)
    }
    
    /// Extract keywords from query
    fn extract_keywords(&self, query: &str) -> Vec<String> {
        query
            .split_whitespace()
            .filter(|word| word.len() > 2) // Filter out short words
            .map(|word| word.to_lowercase())
            .collect()
    }
    
    /// Search for exact keyword matches
    async fn search_exact_keyword(&self, keyword: &str, limit: usize) -> KnowledgeResult<Vec<SemanticResult>> {
        // This would typically search through an inverted index
        // For now, we'll use a simplified approach
        let query_embedding = self.embedding_manager.embed(keyword, None).await?;
        let vector_results = self.vector_store.search(&query_embedding, limit).await?;
        
        self.convert_to_semantic_results(vector_results, keyword).await
    }
    
    /// Rank results by keyword frequency
    async fn rank_by_keyword_frequency(&self, results: Vec<SemanticResult>, keywords: &[String]) -> KnowledgeResult<Vec<SemanticResult>> {
        let mut ranked_results = results;
        
        for result in &mut ranked_results {
            let mut keyword_score = 0.0;
            let content_lower = result.content.to_lowercase();
            
            for keyword in keywords {
                let count = content_lower.matches(keyword).count();
                keyword_score += count as f32;
            }
            
            // Normalize by content length
            keyword_score /= result.content.len() as f32;
            result.relevance_score = keyword_score.min(1.0);
        }
        
        ranked_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(ranked_results)
    }
    
    /// Combine semantic and keyword search results
    async fn combine_search_results(
        &self,
        semantic_results: &[SemanticResult],
        keyword_results: &[SemanticResult],
        semantic_weight: f32,
        keyword_weight: f32,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let mut combined = std::collections::HashMap::new();
        
        // Add semantic results
        for result in semantic_results {
            combined.insert(result.cache_key.clone(), (result.clone(), semantic_weight));
        }
        
        // Add keyword results with weighted scoring
        for result in keyword_results {
            let entry = combined.entry(result.cache_key.clone()).or_insert((result.clone(), 0.0));
            entry.1 += keyword_weight;
        }
        
        // Normalize and create final results
        let mut final_results = Vec::new();
        for (result, weight) in combined.values() {
            let mut final_result = result.clone();
            final_result.relevance_score = (final_result.relevance_score * weight).min(1.0);
            final_results.push(final_result);
        }
        
        // Sort by final scores
        final_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(final_results)
    }
    
    /// Calculate recency boost for search results
    async fn calculate_recency_boost(&self, metadata: &SearchResultMetadata) -> KnowledgeResult<f32> {
        let now = chrono::Utc::now();
        let age_hours = (now - metadata.last_modified).num_hours() as f32;
        
        // Boost newer content (exponential decay)
        let boost = (-age_hours / 168.0).exp(); // 1 week half-life
        Ok(boost.max(0.1)) // Minimum 10% boost
    }
    
    /// Calculate content type boost based on query
    async fn calculate_content_type_boost(&self, result: &SemanticResult, query: &str) -> KnowledgeResult<f32> {
        let query_lower = query.to_lowercase();
        
        match result.metadata.source_type {
            ContentType::Code => {
                if query_lower.contains("code") || query_lower.contains("implementation") {
                    Ok(1.2)
                } else {
                    Ok(1.0)
                }
            }
            ContentType::Documentation => {
                if query_lower.contains("doc") || query_lower.contains("guide") || query_lower.contains("how") {
                    Ok(1.3)
                } else {
                    Ok(1.0)
                }
            }
            ContentType::Decision => {
                if query_lower.contains("decision") || query_lower.contains("why") || query_lower.contains("rationale") {
                    Ok(1.4)
                } else {
                    Ok(1.0)
                }
            }
            _ => Ok(1.0),
        }
    }
    
    /// Calculate semantic tag boost
    async fn calculate_semantic_tag_boost(&self, tags: &[String], query: &str) -> KnowledgeResult<f32> {
        let query_lower = query.to_lowercase();
        let mut boost: f32 = 1.0;
        
        for tag in tags {
            if query_lower.contains(&tag.to_lowercase()) {
                boost += 0.2; // 20% boost per matching tag
            }
        }
        
        Ok(boost.min(2.0)) // Cap at 200% boost
    }
    
    /// Calculate length penalty (prefer concise results)
    async fn calculate_length_penalty(&self, content: &str) -> KnowledgeResult<f32> {
        let length = content.len() as f32;
        
        // Prefer content between 100-1000 characters
        if length < 100.0 {
            Ok(0.8) // Penalize very short content
        } else if length > 1000.0 {
            Ok(0.9) // Slight penalty for very long content
        } else {
            Ok(1.0) // Optimal length
        }
    }
}

/// Search query builder for complex queries
pub struct SearchQueryBuilder {
    query: String,
    content_types: Vec<ContentType>,
    scope_paths: Vec<String>,
    similarity_threshold: Option<f32>,
    limit: Option<usize>,
    enable_hybrid: bool,
    enable_reranking: bool,
}

impl SearchQueryBuilder {
    pub fn new(query: String) -> Self {
        Self {
            query,
            content_types: Vec::new(),
            scope_paths: Vec::new(),
            similarity_threshold: None,
            limit: None,
            enable_hybrid: false,
            enable_reranking: false,
        }
    }
    
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_types.push(content_type);
        self
    }
    
    pub fn with_scope_path(mut self, scope_path: String) -> Self {
        self.scope_paths.push(scope_path);
        self
    }
    
    pub fn with_similarity_threshold(mut self, threshold: f32) -> Self {
        self.similarity_threshold = Some(threshold);
        self
    }
    
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn with_hybrid_search(mut self, enable: bool) -> Self {
        self.enable_hybrid = enable;
        self
    }
    
    pub fn with_reranking(mut self, enable: bool) -> Self {
        self.enable_reranking = enable;
        self
    }
    
    pub fn build(self) -> SearchQuery {
        SearchQuery {
            query: self.query,
            content_types: self.content_types,
            scope_paths: self.scope_paths,
            similarity_threshold: self.similarity_threshold,
            limit: self.limit,
            enable_hybrid: self.enable_hybrid,
            enable_reranking: self.enable_reranking,
        }
    }
}

/// Search query with all parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub content_types: Vec<ContentType>,
    pub scope_paths: Vec<String>,
    pub similarity_threshold: Option<f32>,
    pub limit: Option<usize>,
    pub enable_hybrid: bool,
    pub enable_reranking: bool,
}

impl SearchQuery {
    pub fn new(query: String) -> SearchQueryBuilder {
        SearchQueryBuilder::new(query)
    }
    
    pub fn execute<'a>(&'a self, engine: &'a SemanticSearchEngine) -> impl std::future::Future<Output = KnowledgeResult<Vec<SemanticResult>>> + 'a {
        async move {
            let limit = self.limit.unwrap_or(10);
            
            let mut results = if self.enable_hybrid {
                engine.search_hybrid(&self.query, limit, 0.7).await? // Default semantic weight
            } else {
                engine.search_semantic(&self.query, limit).await?
            };
            
            // Filter by content types
            if !self.content_types.is_empty() {
                results.retain(|result| self.content_types.contains(&result.metadata.source_type));
            }
            
            // Filter by scope paths
            if !self.scope_paths.is_empty() {
                results.retain(|result| {
                    result.metadata.scope_path.as_ref().map_or(false, |path| {
                        self.scope_paths.contains(path)
                    })
                });
            }
            
            // Apply similarity threshold
            if let Some(threshold) = self.similarity_threshold {
                results.retain(|result| result.relevance_score >= threshold);
            }
            
            // Apply reranking
            if self.enable_reranking {
                results.sort_by(|a, b| {
                    b.relevance_score
                        .partial_cmp(&a.relevance_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            
            results.truncate(limit);
            Ok(results)
        }
    }
} 