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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, error, info, instrument, warn};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use tokio::fs;

use crate::types::{
    ContentType, KnowledgeResult, CacheEntryMetadata, SemanticInfo,
    CompressionAlgorithm, DistanceMetric, CacheTier, SearchResultMetadata,
};

use super::{
    embedding::EmbeddingManager,
    vector::VectorStore,
};

/// Error types for indexing operations
#[derive(Error, Debug)]
pub enum IndexingError {
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    
    #[error("Vector storage error: {0}")]
    VectorStorageError(String),
    
    #[error("File processing error: {0}")]
    FileProcessingError(String),
    
    #[error("Chunking error: {0}")]
    ChunkingError(String),
    
    #[error("Metadata extraction error: {0}")]
    MetadataExtractionError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Invalid content: {0}")]
    InvalidContent(String),
}

/// Semantic indexer for processing and indexing content
pub struct SemanticIndexer {
    embedding_manager: Arc<EmbeddingManager>,
    vector_store: crate::vector::VectorStoreWrapper,
    chunking_strategy: Arc<dyn ChunkingStrategy>,
    metadata_extractor: Arc<dyn MetadataExtractor>,
    config: IndexingConfig,
}

/// Indexing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    pub chunk_size: usize,
    pub overlap_size: usize,
    pub max_chunks_per_document: usize,
    pub enable_metadata_extraction: bool,
    pub enable_semantic_tagging: bool,
    pub content_type_detection: bool,
    pub parallel_processing: bool,
    pub batch_size: usize,
}

impl Default for IndexingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            overlap_size: 50,
            max_chunks_per_document: 100,
            enable_metadata_extraction: true,
            enable_semantic_tagging: true,
            content_type_detection: true,
            parallel_processing: true,
            batch_size: 32,
        }
    }
}

/// Chunking strategy for breaking content into manageable pieces
#[async_trait]
pub trait ChunkingStrategy: Send + Sync {
    async fn chunk(&self, content: &str) -> KnowledgeResult<Vec<ContentChunk>>;
    async fn chunk_with_metadata(&self, content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<Vec<ContentChunk>>;
}

/// Content chunk with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub id: String,
    pub content: String,
    pub start_position: usize,
    pub end_position: usize,
    pub chunk_index: usize,
    pub metadata: ChunkMetadata,
}

/// Chunk metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub source_id: String,
    pub content_type: ContentType,
    pub semantic_tags: Vec<String>,
    pub chunk_type: ChunkType,
    pub importance_score: f32,
}

/// Chunk types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkType {
    Header,
    Code,
    Documentation,
    Configuration,
    Comment,
    Text,
    Mixed,
}

/// Indexing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingMetadata {
    pub source_path: Option<PathBuf>,
    pub content_type: ContentType,
    pub scope_path: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub language: Option<String>,
    pub tags: Vec<String>,
}

/// Metadata extractor for content analysis
#[async_trait]
pub trait MetadataExtractor: Send + Sync {
    async fn extract(&self, _content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<SearchResultMetadata>;
    async fn extract_semantic_info(&self, content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<SemanticInfo>;
}

/// Fixed-size chunking strategy
pub struct FixedSizeChunkingStrategy {
    chunk_size: usize,
    overlap_size: usize,
}

impl FixedSizeChunkingStrategy {
    pub fn new(chunk_size: usize, overlap_size: usize) -> Self {
        Self {
            chunk_size,
            overlap_size,
        }
    }
}

#[async_trait]
impl ChunkingStrategy for FixedSizeChunkingStrategy {
    async fn chunk(&self, content: &str) -> KnowledgeResult<Vec<ContentChunk>> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut chunk_index = 0;
        
        while start < content.len() {
            let end = (start + self.chunk_size).min(content.len());
            let chunk_content = content[start..end].to_string();
            
            // Find a good break point (end of sentence or word)
            let actual_end = if end < content.len() {
                self.find_break_point(&chunk_content, end)
            } else {
                end
            };
            
            let chunk_content = content[start..actual_end].to_string();
            
            let chunk = ContentChunk {
                id: format!("chunk_{}", chunk_index),
                content: chunk_content,
                start_position: start,
                end_position: actual_end,
                chunk_index,
                metadata: ChunkMetadata {
                    source_id: "unknown".to_string(),
                    content_type: ContentType::Unknown,
                    semantic_tags: vec![],
                    chunk_type: ChunkType::Text,
                    importance_score: 1.0,
                },
            };
            
            chunks.push(chunk);
            
            // Move to next chunk with overlap
            start = if actual_end > start + self.overlap_size {
                actual_end - self.overlap_size
            } else {
                actual_end
            };
            
            chunk_index += 1;
        }
        
        Ok(chunks)
    }
    
    async fn chunk_with_metadata(&self, content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<Vec<ContentChunk>> {
        let mut chunks = self.chunk(content).await?;
        
        // Update chunk metadata
        for chunk in &mut chunks {
            chunk.metadata.source_id = metadata.source_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            chunk.metadata.content_type = metadata.content_type.clone();
            chunk.metadata.semantic_tags = metadata.tags.clone();
            chunk.metadata.chunk_type = self.detect_chunk_type(&chunk.content);
            chunk.metadata.importance_score = self.calculate_importance_score(&chunk.content);
        }
        
        Ok(chunks)
    }
}

impl FixedSizeChunkingStrategy {
    fn find_break_point(&self, content: &str, max_end: usize) -> usize {
        // Try to find sentence or word boundaries
        let content_slice = &content[..max_end.min(content.len())];
        
        // Look for sentence endings
        if let Some(pos) = content_slice.rfind('.') {
            return pos + 1;
        }
        
        // Look for line breaks
        if let Some(pos) = content_slice.rfind('\n') {
            return pos + 1;
        }
        
        // Look for word boundaries
        if let Some(pos) = content_slice.rfind(' ') {
            return pos + 1;
        }
        
        max_end
    }
    
    fn detect_chunk_type(&self, content: &str) -> ChunkType {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("function") || content_lower.contains("class") || content_lower.contains("pub fn") {
            ChunkType::Code
        } else if content_lower.contains("#") || content_lower.contains("##") {
            ChunkType::Header
        } else if content_lower.contains("//") || content_lower.contains("/*") {
            ChunkType::Comment
        } else if content_lower.contains("config") || content_lower.contains("setting") {
            ChunkType::Configuration
        } else if content_lower.contains("documentation") || content_lower.contains("guide") {
            ChunkType::Documentation
        } else {
            ChunkType::Text
        }
    }
    
    fn calculate_importance_score(&self, content: &str) -> f32 {
        let mut score: f32 = 1.0;
        
        // Boost score for headers
        if content.starts_with('#') {
            score += 0.5;
        }
        
        // Boost score for code blocks
        if content.contains("```") {
            score += 0.3;
        }
        
        // Boost score for important keywords
        let important_keywords = ["important", "note", "warning", "error", "TODO", "FIXME"];
        for keyword in &important_keywords {
            if content.to_lowercase().contains(keyword) {
                score += 0.2;
            }
        }
        
        score.min(2.0f32)
    }
}

/// Basic metadata extractor
pub struct BasicMetadataExtractor;

#[async_trait]
impl MetadataExtractor for BasicMetadataExtractor {
    async fn extract(&self, _content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<SearchResultMetadata> {
        Ok(SearchResultMetadata {
            source_type: metadata.content_type.clone(),
            scope_path: metadata.scope_path.clone(),
            created_at: metadata.created_at,
            last_modified: metadata.last_modified,
            size_bytes: metadata.size_bytes,
            chunk_id: None,
        })
    }
    
    async fn extract_semantic_info(&self, content: &str, metadata: &IndexingMetadata) -> KnowledgeResult<SemanticInfo> {
        let semantic_tags = self.extract_semantic_tags(content).await?;
        
        Ok(SemanticInfo {
            embedding: None,
            semantic_tags,
            content_type: metadata.content_type.clone(),
            relevance_score: 1.0,
            related_keys: vec![],
            chunk_id: None,
        })
    }
}

impl BasicMetadataExtractor {
    async fn extract_semantic_tags(&self, content: &str) -> KnowledgeResult<Vec<String>> {
        let mut tags = Vec::new();
        
        // Extract common programming keywords
        let code_keywords = [
            "function", "class", "struct", "enum", "trait", "impl", "pub", "fn", "let", "const",
            "async", "await", "match", "if", "else", "for", "while", "loop", "return", "use",
        ];
        
        for keyword in &code_keywords {
            if content.to_lowercase().contains(keyword) {
                tags.push(keyword.to_string());
            }
        }
        
        // Extract common documentation keywords
        let doc_keywords = [
            "api", "usage", "example", "guide", "tutorial", "reference", "documentation",
            "note", "warning", "important", "deprecated", "experimental",
        ];
        
        for keyword in &doc_keywords {
            if content.to_lowercase().contains(keyword) {
                tags.push(keyword.to_string());
            }
        }
        
        // Extract file extensions and languages
        let language_patterns = [
            (".rs", "rust"), (".py", "python"), (".js", "javascript"), (".ts", "typescript"),
            (".go", "golang"), (".java", "java"), (".cpp", "cpp"), (".c", "c"),
        ];
        
        for (ext, lang) in &language_patterns {
            if content.contains(ext) {
                tags.push(lang.to_string());
            }
        }
        
        Ok(tags)
    }
}

impl SemanticIndexer {
    pub async fn new(
        embedding_manager: Arc<EmbeddingManager>,
        vector_store: crate::vector::VectorStoreWrapper,
        config: IndexingConfig,
    ) -> KnowledgeResult<Self> {
        let chunking_strategy = Arc::new(FixedSizeChunkingStrategy::new(
            config.chunk_size,
            config.overlap_size,
        ));
        
        let metadata_extractor = Arc::new(BasicMetadataExtractor);
        
        Ok(Self {
            embedding_manager,
            vector_store,
            chunking_strategy,
            metadata_extractor,
            config,
        })
    }
    
    /// Index content with semantic processing
    #[instrument(skip(self, content))]
    pub async fn index_content(&self, content: &str, metadata: IndexingMetadata) -> KnowledgeResult<Vec<String>> {
        info!("Indexing content: {} bytes", content.len());
        
        // Chunk the content
        let chunks = self.chunking_strategy.chunk_with_metadata(content, &metadata).await?;
        
        if chunks.len() > self.config.max_chunks_per_document {
            warn!("Content has {} chunks, limiting to {}", chunks.len(), self.config.max_chunks_per_document);
        }
        
        let chunks_to_process = chunks.iter().take(self.config.max_chunks_per_document);
        let mut chunk_ids = Vec::new();
        
        // Process chunks in batches
        let chunk_batch: Vec<_> = chunks_to_process.collect();
        for batch in chunk_batch.chunks(self.config.batch_size) {
            let batch_ids = self.process_chunk_batch(batch, &metadata).await?;
            chunk_ids.extend(batch_ids);
        }
        
        debug!("Indexed {} chunks for content", chunk_ids.len());
        Ok(chunk_ids)
    }
    
    /// Index a single file
    #[instrument(skip(self, file_path))]
    pub async fn index_file(&self, file_path: &PathBuf, scope_path: Option<&str>) -> KnowledgeResult<Vec<String>> {
        if !file_path.exists() {
            return Err(IndexingError::FileProcessingError(format!("File not found: {}", file_path.display())).into());
        }
        
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| IndexingError::FileProcessingError(format!("Failed to read file {}: {}", file_path.display(), e)))?;
        
        let content_type = self.detect_content_type(file_path, &content);
        
        // Extract actual file metadata
        let mut metadata = self.extract_file_metadata(file_path, &content).await?;
        metadata.scope_path = scope_path.map(|s| s.to_string());
        
        self.index_content(&content, metadata).await
    }
    
    /// Extract actual file metadata including creation and modification times
    async fn extract_file_metadata(&self, file_path: &PathBuf, content: &str) -> KnowledgeResult<IndexingMetadata> {
        // Get file system metadata
        let file_metadata = std::fs::metadata(file_path)
            .map_err(|e| IndexingError::FileProcessingError(format!("Failed to get metadata for {}: {}", file_path.display(), e)))?;
        
        // Extract creation and modification times
        let created_at = file_metadata.created()
            .map(|time| chrono::DateTime::from(time))
            .unwrap_or(chrono::Utc::now());
            
        let last_modified = file_metadata.modified()
            .map(|time| chrono::DateTime::from(time))
            .unwrap_or(chrono::Utc::now());
        
        let metadata = IndexingMetadata {
            source_path: Some(file_path.clone()),
            content_type: self.detect_content_type(file_path, content),
            scope_path: None, // Will be set by caller
            created_at,
            last_modified,
            size_bytes: content.len() as u64,
            language: self.detect_language(file_path, content),
            tags: self.extract_file_tags(file_path, content).await?,
        };
        
        Ok(metadata)
    }
    
    /// Index multiple files in parallel
    #[instrument(skip(self, file_paths))]
    pub async fn index_files(&self, file_paths: &[PathBuf], scope_path: Option<&str>) -> KnowledgeResult<HashMap<PathBuf, Vec<String>>> {
        let mut results = HashMap::new();
        
        if self.config.parallel_processing {
            // Process files in parallel
            let tasks: Vec<_> = file_paths
                .iter()
                .map(|path| {
                    let indexer = self.clone();
                    let path = path.clone();
                    let scope_path = scope_path.map(|s| s.to_string());
                    
                    tokio::spawn(async move {
                        indexer.index_file(&path, scope_path.as_deref()).await
                    })
                })
                .collect();
            
            for (path, task) in file_paths.iter().zip(tasks) {
                match task.await {
                    Ok(Ok(chunk_ids)) => {
                        results.insert(path.clone(), chunk_ids);
                    }
                    Ok(Err(e)) => {
                        warn!("Failed to index file {}: {}", path.display(), e);
                    }
                    Err(e) => {
                        warn!("Task failed for file {}: {}", path.display(), e);
                    }
                }
            }
        } else {
            // Process files sequentially
            for path in file_paths {
                match self.index_file(path, scope_path).await {
                    Ok(chunk_ids) => {
                        results.insert(path.clone(), chunk_ids);
                    }
                    Err(e) => {
                        warn!("Failed to index file {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Process a batch of chunks
    async fn process_chunk_batch(
        &self,
        chunks: &[&ContentChunk],
        metadata: &IndexingMetadata,
    ) -> KnowledgeResult<Vec<String>> {
        let mut chunk_ids = Vec::new();
        
        for chunk in chunks {
            let chunk_id = self.process_single_chunk(chunk, metadata).await?;
            chunk_ids.push(chunk_id);
        }
        
        Ok(chunk_ids)
    }
    
    /// Process a single chunk
    async fn process_single_chunk(
        &self,
        chunk: &ContentChunk,
        metadata: &IndexingMetadata,
    ) -> KnowledgeResult<String> {
        // Generate embedding for the chunk
        let embedding = self.embedding_manager.embed(&chunk.content, None).await?;
        
        // Store in vector store
        let search_metadata = self.metadata_extractor.extract(&chunk.content, metadata).await?;
        self.vector_store.store(&chunk.id, &embedding, Some(search_metadata)).await?;
        
        Ok(chunk.id.clone())
    }
    
    /// Detect content type from file path and content
    fn detect_content_type(&self, file_path: &PathBuf, content: &str) -> ContentType {
        // Check file extension first
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "rs" => return ContentType::Code,
                    "py" | "js" | "ts" | "go" | "java" | "cpp" | "c" => return ContentType::Code,
                    "md" | "txt" | "rst" => return ContentType::Documentation,
                    "yaml" | "yml" | "toml" | "json" | "ini" => return ContentType::Configuration,
                    _ => {}
                }
            }
        }
        
        // Check content patterns
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("function") || content_lower.contains("class") || content_lower.contains("pub fn") {
            ContentType::Code
        } else if content_lower.contains("config") || content_lower.contains("setting") {
            ContentType::Configuration
        } else if content_lower.contains("documentation") || content_lower.contains("guide") {
            ContentType::Documentation
        } else {
            ContentType::Unknown
        }
    }
    
    /// Detect programming language
    fn detect_language(&self, file_path: &PathBuf, content: &str) -> Option<String> {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return match ext_str.to_lowercase().as_str() {
                    "rs" => Some("rust".to_string()),
                    "py" => Some("python".to_string()),
                    "js" => Some("javascript".to_string()),
                    "ts" => Some("typescript".to_string()),
                    "go" => Some("golang".to_string()),
                    "java" => Some("java".to_string()),
                    "cpp" => Some("cpp".to_string()),
                    "c" => Some("c".to_string()),
                    _ => None,
                };
            }
        }
        
        // Try to detect from content
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("fn ") && content_lower.contains("let ") {
            Some("rust".to_string())
        } else if content_lower.contains("def ") && content_lower.contains("import ") {
            Some("python".to_string())
        } else if content_lower.contains("function ") && content_lower.contains("const ") {
            Some("javascript".to_string())
        } else {
            None
        }
    }
    
    /// Extract tags from file
    async fn extract_file_tags(&self, file_path: &PathBuf, content: &str) -> KnowledgeResult<Vec<String>> {
        let mut tags = Vec::new();
        
        // Add file extension as tag
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                tags.push(format!(".{}", ext_str));
            }
        }
        
        // Add language tag
        if let Some(language) = self.detect_language(file_path, content) {
            tags.push(language);
        }
        
        // Add content type tag
        let content_type = self.detect_content_type(file_path, content);
        tags.push(format!("{:?}", content_type).to_lowercase());
        
        Ok(tags)
    }
    
    /// Perform incremental indexing of changed content
    pub async fn incremental_index(&self, content: &str, metadata: IndexingMetadata, previous_hash: &str) -> KnowledgeResult<Vec<String>> {
        let current_hash = self.calculate_content_hash(content).await?;
        
        // If content hasn't changed, return empty result
        if current_hash == previous_hash {
            return Ok(vec![]);
        }
        
        // Remove old chunks for this content
        self.remove_old_chunks(&metadata.source_path).await?;
        
        // Index new content
        self.index_content(content, metadata).await
    }
    
    /// Validate indexed content integrity
    pub async fn validate_index(&self, source_path: &PathBuf) -> KnowledgeResult<IndexValidationResult> {
        let mut validation_result = IndexValidationResult {
            is_valid: true,
            total_chunks: 0,
            valid_chunks: 0,
            invalid_chunks: 0,
            missing_embeddings: 0,
            duplicate_chunks: 0,
            issues: Vec::new(),
        };
        
        // Get all chunks for this source
        let chunks = self.get_chunks_for_source(source_path).await?;
        validation_result.total_chunks = chunks.len();
        
        for chunk in chunks {
            // Validate chunk content
            if chunk.content.is_empty() {
                validation_result.invalid_chunks += 1;
                validation_result.issues.push(format!("Empty chunk: {}", chunk.id));
                continue;
            }
            
            // Check for duplicate content
            if self.is_duplicate_chunk(&chunk).await? {
                validation_result.duplicate_chunks += 1;
                validation_result.issues.push(format!("Duplicate chunk: {}", chunk.id));
                continue;
            }
            
            // Validate embedding exists
            if !self.has_embedding(&chunk.id).await? {
                validation_result.missing_embeddings += 1;
                validation_result.issues.push(format!("Missing embedding: {}", chunk.id));
                continue;
            }
            
            validation_result.valid_chunks += 1;
        }
        
        validation_result.is_valid = validation_result.invalid_chunks == 0 && validation_result.missing_embeddings == 0;
        
        Ok(validation_result)
    }
    
    /// Monitor indexing progress and performance
    pub async fn get_indexing_stats(&self) -> KnowledgeResult<IndexingStats> {
        let stats = IndexingStats {
            total_indexed_files: self.get_total_indexed_files().await?,
            total_chunks: self.get_total_chunks().await?,
            average_chunks_per_file: self.get_average_chunks_per_file().await?,
            indexing_speed_chunks_per_second: self.get_indexing_speed().await?,
            last_indexing_time: self.get_last_indexing_time().await?,
            storage_usage_bytes: self.get_storage_usage().await?,
            cache_hit_rate: self.get_cache_hit_rate().await?,
        };
        
        Ok(stats)
    }
    
    /// Recover from indexing failures
    pub async fn recover_from_failure(&self, source_path: &PathBuf) -> KnowledgeResult<IndexRecoveryResult> {
        let mut recovery_result = IndexRecoveryResult {
            recovered_chunks: 0,
            failed_chunks: 0,
            reindexed_files: 0,
            errors: Vec::new(),
        };
        
        // Check for incomplete indexing
        let incomplete_chunks = self.get_incomplete_chunks(source_path).await?;
        
        for chunk in incomplete_chunks {
            match self.recover_chunk(&chunk).await {
                Ok(_) => recovery_result.recovered_chunks += 1,
                Err(e) => {
                    recovery_result.failed_chunks += 1;
                    recovery_result.errors.push(format!("Failed to recover chunk {}: {}", chunk.id, e));
                }
            }
        }
        
        // Reindex files with failures
        let failed_files = self.get_failed_files(source_path).await?;
        for file_path in failed_files {
            match self.index_file(&file_path, None).await {
                Ok(_) => recovery_result.reindexed_files += 1,
                Err(e) => {
                    recovery_result.errors.push(format!("Failed to reindex file {:?}: {}", file_path, e));
                }
            }
        }
        
        Ok(recovery_result)
    }
    
    /// Schedule indexing operations
    pub async fn schedule_indexing(&self, schedule: IndexingSchedule) -> KnowledgeResult<()> {
        // Store the schedule for background processing
        self.store_indexing_schedule(schedule).await?;
        
        // Start background indexing if not already running
        self.start_background_indexing().await?;
        
        Ok(())
    }
    
    /// Prioritize indexing based on importance
    pub async fn prioritize_indexing(&self, priorities: Vec<IndexingPriority>) -> KnowledgeResult<()> {
        for priority in priorities {
            match priority {
                IndexingPriority::File(path, importance) => {
                    self.set_file_priority(&path, importance).await?;
                }
                IndexingPriority::Directory(path, importance) => {
                    self.set_directory_priority(&path, importance).await?;
                }
                IndexingPriority::ContentType(content_type, importance) => {
                    self.set_content_type_priority(content_type, importance).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Clean up old or invalid indexes
    pub async fn cleanup_indexes(&self, cleanup_config: IndexCleanupConfig) -> KnowledgeResult<IndexCleanupResult> {
        let mut cleanup_result = IndexCleanupResult {
            removed_chunks: 0,
            removed_files: 0,
            freed_storage_bytes: 0,
            errors: Vec::new(),
        };
        
        // Remove old chunks based on age
        if let Some(max_age_days) = cleanup_config.max_age_days {
            let old_chunks = self.get_old_chunks(max_age_days).await?;
            for chunk in old_chunks {
                match self.remove_chunk(&chunk.id).await {
                    Ok(_) => {
                        cleanup_result.removed_chunks += 1;
                        cleanup_result.freed_storage_bytes += chunk.content.len() as u64;
                    }
                    Err(e) => {
                        cleanup_result.errors.push(format!("Failed to remove chunk {}: {}", chunk.id, e));
                    }
                }
            }
        }
        
        // Remove invalid chunks
        if cleanup_config.remove_invalid {
            let invalid_chunks = self.get_invalid_chunks().await?;
            for chunk in invalid_chunks {
                match self.remove_chunk(&chunk.id).await {
                    Ok(_) => {
                        cleanup_result.removed_chunks += 1;
                        cleanup_result.freed_storage_bytes += chunk.content.len() as u64;
                    }
                    Err(e) => {
                        cleanup_result.errors.push(format!("Failed to remove invalid chunk {}: {}", chunk.id, e));
                    }
                }
            }
        }
        
        // Remove orphaned files
        if cleanup_config.remove_orphaned {
            let orphaned_files = self.get_orphaned_files().await?;
            for file_path in orphaned_files {
                match self.remove_file_index(&file_path).await {
                    Ok(_) => {
                        cleanup_result.removed_files += 1;
                    }
                    Err(e) => {
                        cleanup_result.errors.push(format!("Failed to remove orphaned file {:?}: {}", file_path, e));
                    }
                }
            }
        }
        
        Ok(cleanup_result)
    }
    
    // Helper methods for the above functionality
    
    async fn calculate_content_hash(&self, content: &str) -> KnowledgeResult<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }
    
    async fn remove_old_chunks(&self, _source_path: &Option<PathBuf>) -> KnowledgeResult<()> {
        // Implementation would remove chunks for the given source
        // For now, just return success
        Ok(())
    }
    
    async fn get_chunks_for_source(&self, _source_path: &PathBuf) -> KnowledgeResult<Vec<ContentChunk>> {
        // Implementation would retrieve chunks for the source
        // For now, return empty vector
        Ok(vec![])
    }
    
    async fn is_duplicate_chunk(&self, _chunk: &ContentChunk) -> KnowledgeResult<bool> {
        // Implementation would check for duplicates
        // For now, return false
        Ok(false)
    }
    
    async fn has_embedding(&self, _chunk_id: &str) -> KnowledgeResult<bool> {
        // Implementation would check if embedding exists
        // For now, return true
        Ok(true)
    }
    
    async fn get_total_indexed_files(&self) -> KnowledgeResult<usize> {
        // Implementation would count total indexed files
        Ok(0)
    }
    
    async fn get_total_chunks(&self) -> KnowledgeResult<usize> {
        // Implementation would count total chunks
        Ok(0)
    }
    
    async fn get_average_chunks_per_file(&self) -> KnowledgeResult<f32> {
        // Implementation would calculate average
        Ok(0.0)
    }
    
    async fn get_indexing_speed(&self) -> KnowledgeResult<f32> {
        // Implementation would calculate speed
        Ok(0.0)
    }
    
    async fn get_last_indexing_time(&self) -> KnowledgeResult<Option<chrono::DateTime<chrono::Utc>>> {
        // Implementation would get last indexing time
        Ok(None)
    }
    
    async fn get_storage_usage(&self) -> KnowledgeResult<u64> {
        // Implementation would calculate storage usage
        Ok(0)
    }
    
    async fn get_cache_hit_rate(&self) -> KnowledgeResult<f32> {
        // Implementation would calculate cache hit rate
        Ok(0.0)
    }
    
    async fn get_incomplete_chunks(&self, _source_path: &PathBuf) -> KnowledgeResult<Vec<ContentChunk>> {
        // Implementation would get incomplete chunks
        Ok(vec![])
    }
    
    async fn recover_chunk(&self, _chunk: &ContentChunk) -> KnowledgeResult<()> {
        // Implementation would recover chunk
        Ok(())
    }
    
    async fn get_failed_files(&self, _source_path: &PathBuf) -> KnowledgeResult<Vec<PathBuf>> {
        // Implementation would get failed files
        Ok(vec![])
    }
    
    async fn store_indexing_schedule(&self, _schedule: IndexingSchedule) -> KnowledgeResult<()> {
        // Implementation would store schedule
        Ok(())
    }
    
    async fn start_background_indexing(&self) -> KnowledgeResult<()> {
        // Implementation would start background indexing
        Ok(())
    }
    
    async fn set_file_priority(&self, _path: &PathBuf, _importance: f32) -> KnowledgeResult<()> {
        // Implementation would set file priority
        Ok(())
    }
    
    async fn set_directory_priority(&self, _path: &PathBuf, _importance: f32) -> KnowledgeResult<()> {
        // Implementation would set directory priority
        Ok(())
    }
    
    async fn set_content_type_priority(&self, _content_type: ContentType, _importance: f32) -> KnowledgeResult<()> {
        // Implementation would set content type priority
        Ok(())
    }
    
    async fn get_old_chunks(&self, _max_age_days: u64) -> KnowledgeResult<Vec<ContentChunk>> {
        // Implementation would get old chunks
        Ok(vec![])
    }
    
    async fn remove_chunk(&self, _chunk_id: &str) -> KnowledgeResult<()> {
        // Implementation would remove chunk
        Ok(())
    }
    
    async fn get_invalid_chunks(&self) -> KnowledgeResult<Vec<ContentChunk>> {
        // Implementation would get invalid chunks
        Ok(vec![])
    }
    
    async fn get_orphaned_files(&self) -> KnowledgeResult<Vec<PathBuf>> {
        // Implementation would get orphaned files
        Ok(vec![])
    }
    
    async fn remove_file_index(&self, _file_path: &PathBuf) -> KnowledgeResult<()> {
        // Implementation would remove file index
        Ok(())
    }
}

impl Clone for SemanticIndexer {
    fn clone(&self) -> Self {
        Self {
            embedding_manager: self.embedding_manager.clone(),
            vector_store: self.vector_store.clone(),
            chunking_strategy: self.chunking_strategy.clone(),
            metadata_extractor: self.metadata_extractor.clone(),
            config: self.config.clone(),
        }
    }
} 

/// Index validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexValidationResult {
    pub is_valid: bool,
    pub total_chunks: usize,
    pub valid_chunks: usize,
    pub invalid_chunks: usize,
    pub missing_embeddings: usize,
    pub duplicate_chunks: usize,
    pub issues: Vec<String>,
}

/// Indexing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingStats {
    pub total_indexed_files: usize,
    pub total_chunks: usize,
    pub average_chunks_per_file: f32,
    pub indexing_speed_chunks_per_second: f32,
    pub last_indexing_time: Option<chrono::DateTime<chrono::Utc>>,
    pub storage_usage_bytes: u64,
    pub cache_hit_rate: f32,
}

/// Index recovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRecoveryResult {
    pub recovered_chunks: usize,
    pub failed_chunks: usize,
    pub reindexed_files: usize,
    pub errors: Vec<String>,
}

/// Indexing schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingSchedule {
    pub schedule_type: ScheduleType,
    pub interval_minutes: Option<u64>,
    pub cron_expression: Option<String>,
    pub enabled: bool,
    pub priority: SchedulePriority,
}

/// Schedule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    Continuous,
    Periodic,
    Cron,
    Manual,
}

/// Schedule priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Indexing priority configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexingPriority {
    File(PathBuf, f32), // path, importance (0.0-1.0)
    Directory(PathBuf, f32), // path, importance (0.0-1.0)
    ContentType(ContentType, f32), // content type, importance (0.0-1.0)
}

/// Index cleanup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexCleanupConfig {
    pub max_age_days: Option<u64>,
    pub remove_invalid: bool,
    pub remove_orphaned: bool,
    pub dry_run: bool,
}

/// Index cleanup result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexCleanupResult {
    pub removed_chunks: usize,
    pub removed_files: usize,
    pub freed_storage_bytes: u64,
    pub errors: Vec<String>,
} 