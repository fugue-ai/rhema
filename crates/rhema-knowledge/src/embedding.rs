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
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{error};

use crate::types::{
    ContentType, KnowledgeResult, SemanticInfo,
};

/// Error types for embedding operations
#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Model loading error: {0}")]
    ModelLoadingError(String),
    
    #[error("Embedding generation error: {0}")]
    EmbeddingGenerationError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Model not initialized: {0}")]
    ModelNotInitialized(String),
    
    #[error("Batch processing error: {0}")]
    BatchProcessingError(String),
    
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

/// Embedding model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModelConfig {
    pub model_name: String,
    pub model_type: EmbeddingModelType,
    pub max_length: usize,
    pub dimension: usize,
    pub device: EmbeddingDevice,
    pub batch_size: usize,
    pub enable_caching: bool,
    pub cache_size: usize,
}

/// Embedding model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingModelType {
    SimpleHash,
    SentenceTransformers,
    BERT,
    RoBERTa,
    DistilBERT,
    Custom(String),
}

/// Device for embedding computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingDevice {
    CPU,
    CUDA,
    MPS,
}

/// Embedding model trait for different implementations
#[async_trait]
pub trait EmbeddingModel: Send + Sync {
    async fn embed(&self, text: &str) -> KnowledgeResult<Vec<f32>>;
    async fn embed_batch(&self, texts: &[String]) -> KnowledgeResult<Vec<Vec<f32>>>;
    async fn similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> KnowledgeResult<f32>;
    async fn dimension(&self) -> usize;
    async fn model_info(&self) -> EmbeddingModelInfo;
    
    /// Get type information for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Embedding model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModelInfo {
    pub name: String,
    pub version: String,
    pub dimension: usize,
    pub max_length: usize,
    pub model_type: EmbeddingModelType,
    pub device: EmbeddingDevice,
}

/// Simple hash-based embedding model for development/testing
pub struct SimpleHashEmbeddingModel {
    config: EmbeddingModelConfig,
    cache: Arc<RwLock<lru::LruCache<String, Vec<f32>>>>,
}

impl SimpleHashEmbeddingModel {
    pub fn new(config: EmbeddingModelConfig) -> Self {
        let cache_size = if config.enable_caching { config.cache_size } else { 0 };
        let cache = if cache_size > 0 {
            Arc::new(RwLock::new(lru::LruCache::new(std::num::NonZeroUsize::new(cache_size).unwrap())))
        } else {
            Arc::new(RwLock::new(lru::LruCache::new(std::num::NonZeroUsize::new(1).unwrap())))
        };
        Self {
            config,
            cache,
        }
    }

    fn simple_hash_embed(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate a vector based on the hash
        let mut embedding = vec![0.0; self.config.dimension];
        for i in 0..self.config.dimension {
            let shift = (i * 7) % 64;
            let bit = (hash >> shift) & 1;
            embedding[i] = if bit == 1 { 1.0 } else { -1.0 };
        }
        
        embedding
    }
}

#[async_trait]
impl EmbeddingModel for SimpleHashEmbeddingModel {
    async fn embed(&self, text: &str) -> KnowledgeResult<Vec<f32>> {
        if text.trim().is_empty() {
            return Err(EmbeddingError::InvalidInput("Text cannot be empty".to_string()).into());
        }
        
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached_embedding) = cache.peek(text) {
                return Ok(cached_embedding.clone());
            }
        }
        
        // Generate embedding
        let embedding = self.simple_hash_embed(text);
        
        // Cache the result
        {
            let mut cache = self.cache.write().await;
            cache.put(text.to_string(), embedding.clone());
        }
        
        Ok(embedding)
    }
    
    async fn embed_batch(&self, texts: &[String]) -> KnowledgeResult<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Err(EmbeddingError::InvalidInput("Texts cannot be empty".to_string()).into());
        }
        
        let mut embeddings = Vec::with_capacity(texts.len());
        
        for text in texts {
            let embedding = self.embed(text).await?;
            embeddings.push(embedding);
        }
        
        Ok(embeddings)
    }
    
    async fn similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> KnowledgeResult<f32> {
        if embedding1.len() != embedding2.len() {
            return Err(EmbeddingError::DimensionMismatch {
                expected: embedding1.len(),
                actual: embedding2.len(),
            }.into());
        }
        
        // Calculate cosine similarity
        let dot_product: f32 = embedding1.iter().zip(embedding2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            return Ok(0.0);
        }
        
        Ok(dot_product / (norm1 * norm2))
    }
    
    async fn dimension(&self) -> usize {
        self.config.dimension
    }
    
    async fn model_info(&self) -> EmbeddingModelInfo {
        EmbeddingModelInfo {
            name: self.config.model_name.clone(),
            version: "1.0.0".to_string(),
            dimension: self.config.dimension,
            max_length: self.config.max_length,
            model_type: self.config.model_type.clone(),
            device: self.config.device.clone(),
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Placeholder for future SentenceTransformersModel
pub struct SentenceTransformersModel {
    config: EmbeddingModelConfig,
    cache: Arc<RwLock<lru::LruCache<String, Vec<f32>>>>,
}

impl SentenceTransformersModel {
    pub async fn new(config: EmbeddingModelConfig) -> KnowledgeResult<Self> {
        // For now, create a dummy model
        // In a real implementation, this would load the actual sentence-transformers model
        Ok(Self {
            config: config.clone(),
            cache: Arc::new(RwLock::new(lru::LruCache::new(
                config.cache_size.try_into().unwrap_or(std::num::NonZeroUsize::new(1000).unwrap())
            ))),
        })
    }
    
    fn simple_hash_embed(&self, text: &str) -> Vec<f32> {
        // Simple hash-based embedding for testing
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        let mut embedding = vec![0.0; self.config.dimension];
        for i in 0..self.config.dimension {
            embedding[i] = ((hash >> (i * 8)) & 0xFF) as f32 / 255.0 * 2.0 - 1.0;
        }
        embedding
    }
}

#[async_trait]
impl EmbeddingModel for SentenceTransformersModel {
    async fn embed(&self, text: &str) -> KnowledgeResult<Vec<f32>> {
        // For now, return a dummy embedding
        // In a real implementation, this would use the actual sentence-transformers model
        Ok(self.simple_hash_embed(text))
    }
    
    async fn embed_batch(&self, texts: &[String]) -> KnowledgeResult<Vec<Vec<f32>>> {
        let mut embeddings = Vec::with_capacity(texts.len());
        for text in texts {
            embeddings.push(self.simple_hash_embed(text));
        }
        Ok(embeddings)
    }
    
    async fn similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> KnowledgeResult<f32> {
        // Calculate cosine similarity
        let dot_product: f32 = embedding1.iter().zip(embedding2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            return Ok(0.0);
        }
        
        Ok(dot_product / (norm1 * norm2))
    }
    
    async fn dimension(&self) -> usize {
        self.config.dimension
    }
    
    async fn model_info(&self) -> EmbeddingModelInfo {
        EmbeddingModelInfo {
            name: self.config.model_name.clone(),
            version: "1.0.0".to_string(),
            dimension: self.config.dimension,
            max_length: self.config.max_length,
            model_type: self.config.model_type.clone(),
            device: self.config.device.clone(),
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Embedding manager for handling multiple models and caching
pub struct EmbeddingManager {
    models: Arc<RwLock<std::collections::HashMap<String, Arc<dyn EmbeddingModel>>>>,
    default_model: String,
    config: EmbeddingManagerConfig,
}

/// Embedding manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingManagerConfig {
    pub default_model: String,
    pub enable_caching: bool,
    pub cache_size: usize,
    pub batch_size: usize,
    pub max_concurrent_requests: usize,
}

impl Default for EmbeddingManagerConfig {
    fn default() -> Self {
        Self {
            default_model: "simple-hash".to_string(),
            enable_caching: true,
            cache_size: 1000,
            batch_size: 32,
            max_concurrent_requests: 10,
        }
    }
}

impl EmbeddingManager {
    pub fn new_dummy() -> Self {
        let mut models = std::collections::HashMap::new();
        
        // Initialize default model for testing
        let default_model_config = EmbeddingModelConfig {
            model_name: "simple-hash".to_string(),
            model_type: EmbeddingModelType::SimpleHash,
            max_length: 512,
            dimension: 384,
            device: EmbeddingDevice::CPU,
            batch_size: 32,
            enable_caching: true,
            cache_size: 1000,
        };
        
        let default_model = Arc::new(SimpleHashEmbeddingModel::new(default_model_config)) as Arc<dyn EmbeddingModel>;
        models.insert("simple-hash".to_string(), default_model);
        
        Self {
            models: Arc::new(RwLock::new(models)),
            default_model: "simple-hash".to_string(),
            config: EmbeddingManagerConfig::default(),
        }
    }
    
    pub async fn new(config: EmbeddingManagerConfig) -> KnowledgeResult<Self> {
        let models = Arc::new(RwLock::new(std::collections::HashMap::new()));
        
        // Initialize default model (use SimpleHash for now)
        let default_model_config = EmbeddingModelConfig {
            model_name: "simple-hash".to_string(),
            model_type: EmbeddingModelType::SimpleHash,
            max_length: 512,
            dimension: 384,
            device: EmbeddingDevice::CPU,
            batch_size: config.batch_size,
            enable_caching: config.enable_caching,
            cache_size: config.cache_size,
        };
        
        let default_model = Arc::new(SimpleHashEmbeddingModel::new(default_model_config)) as Arc<dyn EmbeddingModel>;
        {
            let mut models_guard = models.write().await;
            models_guard.insert("simple-hash".to_string(), default_model);
        }
        
        Ok(Self {
            models,
            default_model: "simple-hash".to_string(),
            config,
        })
    }
    
    pub async fn add_model(&self, name: String, model: Arc<dyn EmbeddingModel>) {
        let mut models = self.models.write().await;
        models.insert(name, model);
    }
    
    pub async fn get_model(&self, name: Option<&str>) -> KnowledgeResult<Arc<dyn EmbeddingModel>> {
        let model_name = name.unwrap_or(&self.default_model);
        let models = self.models.read().await;
        models.get(model_name)
            .cloned()
            .ok_or_else(|| EmbeddingError::ModelNotInitialized(
                format!("Model not found: {}", model_name)
            ).into())
    }
    
    pub async fn embed(&self, text: &str, model_name: Option<&str>) -> KnowledgeResult<Vec<f32>> {
        let model = self.get_model(model_name).await?;
        model.embed(text).await
    }

    pub async fn generate_embedding(&self, text: &str) -> KnowledgeResult<Vec<f32>> {
        self.embed(text, None).await
    }
    
    pub async fn embed_batch(&self, texts: &[String], model_name: Option<&str>) -> KnowledgeResult<Vec<Vec<f32>>> {
        let model = self.get_model(model_name).await?;
        model.embed_batch(texts).await
    }
    
    pub async fn similarity(&self, embedding1: &[f32], embedding2: &[f32], model_name: Option<&str>) -> KnowledgeResult<f32> {
        let model = self.get_model(model_name).await?;
        model.similarity(embedding1, embedding2).await
    }
    
    pub async fn generate_semantic_info(&self, content: &str, content_type: ContentType) -> KnowledgeResult<SemanticInfo> {
        let embedding = self.embed(content, None).await?;
        let semantic_tags = self.extract_semantic_tags(content, &content_type).await?;
        
        Ok(SemanticInfo {
            embedding: Some(embedding),
            semantic_tags,
            content_type,
            relevance_score: 1.0, // Default relevance
            related_keys: vec![],
            chunk_id: None,
        })
    }
    
    async fn extract_semantic_tags(&self, content: &str, content_type: &ContentType) -> KnowledgeResult<Vec<String>> {
        // Simple keyword extraction based on content type
        let mut tags = vec![];
        
        match content_type {
            ContentType::Code => {
                // Extract programming language keywords
                let code_keywords = ["function", "class", "struct", "enum", "trait", "impl", "pub", "fn", "let", "const"];
                for keyword in &code_keywords {
                    if content.to_lowercase().contains(keyword) {
                        tags.push(keyword.to_string());
                    }
                }
            }
            ContentType::Documentation => {
                // Extract documentation keywords
                let doc_keywords = ["api", "usage", "example", "guide", "tutorial", "reference"];
                for keyword in &doc_keywords {
                    if content.to_lowercase().contains(keyword) {
                        tags.push(keyword.to_string());
                    }
                }
            }
            ContentType::Configuration => {
                // Extract configuration keywords
                let config_keywords = ["config", "setting", "option", "parameter", "environment"];
                for keyword in &config_keywords {
                    if content.to_lowercase().contains(keyword) {
                        tags.push(keyword.to_string());
                    }
                }
            }
            _ => {
                // Generic keyword extraction
                let words: Vec<&str> = content.split_whitespace().collect();
                for word in words.iter().take(10) {
                    if word.len() > 3 {
                        tags.push(word.to_lowercase());
                    }
                }
            }
        }
        
        Ok(tags)
    }
    
    /// Cache embedding for reuse
    pub async fn cache_embedding(&self, key: &str, embedding: &[f32], model_name: Option<&str>) -> KnowledgeResult<()> {
        let model = self.get_model(model_name).await?;
        
        // For now, we'll use the model's internal cache
        // In a full implementation, this would use a separate cache manager
        if let Some(simple_hash_model) = model.as_any().downcast_ref::<SimpleHashEmbeddingModel>() {
            simple_hash_model.cache_embedding(key, embedding).await?;
        }
        
        Ok(())
    }
    
    /// Get cached embedding if available
    pub async fn get_cached_embedding(&self, key: &str, model_name: Option<&str>) -> KnowledgeResult<Option<Vec<f32>>> {
        let model = self.get_model(model_name).await?;
        
        if let Some(simple_hash_model) = model.as_any().downcast_ref::<SimpleHashEmbeddingModel>() {
            return simple_hash_model.get_cached_embedding(key).await;
        }
        
        Ok(None)
    }
    
    /// Validate embedding quality
    pub async fn validate_embedding(&self, embedding: &[f32], model_name: Option<&str>) -> KnowledgeResult<EmbeddingValidationResult> {
        let model = self.get_model(model_name).await?;
        let expected_dimension = model.dimension().await;
        
        let mut validation_result = EmbeddingValidationResult {
            is_valid: true,
            dimension_match: embedding.len() == expected_dimension,
            has_nan_values: false,
            has_infinite_values: false,
            magnitude_check: true,
            quality_score: 1.0,
            issues: Vec::new(),
        };
        
        // Check for NaN values
        for &value in embedding {
            if value.is_nan() {
                validation_result.has_nan_values = true;
                validation_result.is_valid = false;
                validation_result.issues.push("Contains NaN values".to_string());
            }
            
            if value.is_infinite() {
                validation_result.has_infinite_values = true;
                validation_result.is_valid = false;
                validation_result.issues.push("Contains infinite values".to_string());
            }
        }
        
        // Check dimension match
        if !validation_result.dimension_match {
            validation_result.is_valid = false;
            validation_result.issues.push(format!(
                "Dimension mismatch: expected {}, got {}",
                expected_dimension,
                embedding.len()
            ));
        }
        
        // Check magnitude (embeddings should typically be normalized)
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude < 0.1 || magnitude > 10.0 {
            validation_result.magnitude_check = false;
            validation_result.quality_score *= 0.8;
            validation_result.issues.push(format!("Unusual magnitude: {}", magnitude));
        }
        
        // Calculate quality score based on various factors
        validation_result.quality_score = self.calculate_quality_score(embedding).await;
        
        Ok(validation_result)
    }
    
    /// Compress embedding for storage efficiency
    pub async fn compress_embedding(&self, embedding: &[f32], compression_type: EmbeddingCompressionType) -> KnowledgeResult<CompressedEmbedding> {
        match compression_type {
            EmbeddingCompressionType::Quantization => {
                self.quantize_embedding(embedding).await
            }
            EmbeddingCompressionType::DimensionalityReduction => {
                self.reduce_dimensionality(embedding).await
            }
            EmbeddingCompressionType::Sparse => {
                self.sparsify_embedding(embedding).await
            }
        }
    }
    
    /// Decompress embedding
    pub async fn decompress_embedding(&self, compressed: &CompressedEmbedding) -> KnowledgeResult<Vec<f32>> {
        match &compressed.compression_type {
            EmbeddingCompressionType::Quantization => {
                self.dequantize_embedding(compressed).await
            }
            EmbeddingCompressionType::DimensionalityReduction => {
                self.expand_dimensionality(compressed).await
            }
            EmbeddingCompressionType::Sparse => {
                self.desparsify_embedding(compressed).await
            }
        }
    }
    
    /// Version embeddings for compatibility
    pub async fn version_embedding(&self, embedding: &[f32], version: &str) -> KnowledgeResult<VersionedEmbedding> {
        let model = self.get_model(None).await?;
        let model_info = model.model_info().await;
        
        Ok(VersionedEmbedding {
            embedding: embedding.to_vec(),
            version: version.to_string(),
            model_name: model_info.name,
            model_version: model_info.version,
            dimension: embedding.len(),
            created_at: chrono::Utc::now(),
        })
    }
    
    /// Migrate embeddings between versions
    pub async fn migrate_embedding(&self, versioned_embedding: &VersionedEmbedding, target_version: &str) -> KnowledgeResult<VersionedEmbedding> {
        // For now, we'll just update the version
        // In a full implementation, this would handle actual migration logic
        let mut migrated = versioned_embedding.clone();
        migrated.version = target_version.to_string();
        migrated.created_at = chrono::Utc::now();
        
        Ok(migrated)
    }
    
    /// Calculate embedding quality score
    async fn calculate_quality_score(&self, embedding: &[f32]) -> f32 {
        let mut score: f32 = 1.0;
        
        // Check for zero vectors
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude < 0.001 {
            score *= 0.1; // Very low score for zero vectors
        }
        
        // Check for uniform distributions (might indicate poor quality)
        let mean = embedding.iter().sum::<f32>() / embedding.len() as f32;
        let variance = embedding.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / embedding.len() as f32;
        
        if variance < 0.01 {
            score *= 0.7; // Penalize very uniform embeddings
        }
        
        // Check for extreme values
        let max_abs = embedding.iter().map(|x| x.abs()).fold(0.0, f32::max);
        if max_abs > 10.0 {
            score *= 0.8; // Penalize extreme values
        }
        
        score.max(0.0).min(1.0)
    }
    
    /// Quantize embedding to reduce precision
    async fn quantize_embedding(&self, embedding: &[f32]) -> KnowledgeResult<CompressedEmbedding> {
        let quantized: Vec<u8> = embedding
            .iter()
            .map(|&x| {
                // Quantize to 8-bit (0-255 range)
                let normalized = (x + 1.0) / 2.0; // Map [-1, 1] to [0, 1]
                (normalized * 255.0).round() as u8
            })
            .collect();
        
        Ok(CompressedEmbedding {
            data: quantized,
            compression_type: EmbeddingCompressionType::Quantization,
            original_dimension: embedding.len(),
            metadata: CompressedEmbeddingMetadata {
                compression_ratio: 4.0, // 32-bit to 8-bit = 4x compression
                quality_loss: 0.1, // Estimated quality loss
            },
        })
    }
    
    /// Dequantize embedding
    async fn dequantize_embedding(&self, compressed: &CompressedEmbedding) -> KnowledgeResult<Vec<f32>> {
        let dequantized: Vec<f32> = compressed
            .data
            .iter()
            .map(|&x| {
                // Dequantize from 8-bit back to float
                let normalized = x as f32 / 255.0;
                normalized * 2.0 - 1.0 // Map [0, 1] back to [-1, 1]
            })
            .collect();
        
        Ok(dequantized)
    }
    
    /// Reduce dimensionality using PCA-like approach
    async fn reduce_dimensionality(&self, embedding: &[f32]) -> KnowledgeResult<CompressedEmbedding> {
        // Simple dimensionality reduction: keep top 50% of dimensions by magnitude
        let mut indexed: Vec<(usize, f32)> = embedding
            .iter()
            .enumerate()
            .map(|(i, &x)| (i, x.abs()))
            .collect();
        
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let keep_count = embedding.len() / 2;
        let mut reduced = vec![0.0; embedding.len()];
        
        for (i, _) in indexed.iter().take(keep_count) {
            reduced[*i] = embedding[*i];
        }
        
        let compressed_data: Vec<u8> = reduced
            .into_iter()
            .map(|x| ((x * 1000.0) as i16) as u8)
            .collect();
        
        Ok(CompressedEmbedding {
            data: compressed_data,
            compression_type: EmbeddingCompressionType::DimensionalityReduction,
            original_dimension: embedding.len(),
            metadata: CompressedEmbeddingMetadata {
                compression_ratio: 2.0,
                quality_loss: 0.2,
            },
        })
    }
    
    /// Expand dimensionality back
    async fn expand_dimensionality(&self, compressed: &CompressedEmbedding) -> KnowledgeResult<Vec<f32>> {
        let expanded: Vec<f32> = compressed
            .data
            .iter()
            .map(|&x| x as f32 / 1000.0)
            .collect();
        
        Ok(expanded)
    }
    
    /// Sparsify embedding (keep only top values)
    async fn sparsify_embedding(&self, embedding: &[f32]) -> KnowledgeResult<CompressedEmbedding> {
        let mut indexed: Vec<(usize, f32)> = embedding
            .iter()
            .enumerate()
            .map(|(i, &x)| (i, x.abs()))
            .collect();
        
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let keep_count = embedding.len() / 4; // Keep top 25%
        let mut sparse_data = Vec::new();
        
        for (i, _) in indexed.iter().take(keep_count) {
            sparse_data.push(*i as u8);
            sparse_data.push((embedding[*i] * 1000.0) as u8);
        }
        
        Ok(CompressedEmbedding {
            data: sparse_data,
            compression_type: EmbeddingCompressionType::Sparse,
            original_dimension: embedding.len(),
            metadata: CompressedEmbeddingMetadata {
                compression_ratio: 4.0,
                quality_loss: 0.15,
            },
        })
    }
    
    /// Desparsify embedding
    async fn desparsify_embedding(&self, compressed: &CompressedEmbedding) -> KnowledgeResult<Vec<f32>> {
        let mut result = vec![0.0; compressed.original_dimension];
        
        let mut i = 0;
        while i < compressed.data.len() {
            if i + 1 < compressed.data.len() {
                let index = compressed.data[i] as usize;
                let value = compressed.data[i + 1] as f32 / 1000.0;
                if index < result.len() {
                    result[index] = value;
                }
                i += 2;
            } else {
                break;
            }
        }
        
        Ok(result)
    }
}

impl SimpleHashEmbeddingModel {
    /// Cache embedding for reuse
    pub async fn cache_embedding(&self, key: &str, embedding: &[f32]) -> KnowledgeResult<()> {
        let mut cache = self.cache.write().await;
        cache.put(key.to_string(), embedding.to_vec());
        Ok(())
    }
    
    /// Get cached embedding if available
    pub async fn get_cached_embedding(&self, key: &str) -> KnowledgeResult<Option<Vec<f32>>> {
        let cache = self.cache.read().await;
        Ok(cache.peek(key).cloned())
    }
}

/// Embedding compression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingCompressionType {
    Quantization,
    DimensionalityReduction,
    Sparse,
}

/// Compressed embedding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedEmbedding {
    pub data: Vec<u8>,
    pub compression_type: EmbeddingCompressionType,
    pub original_dimension: usize,
    pub metadata: CompressedEmbeddingMetadata,
}

/// Compressed embedding metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedEmbeddingMetadata {
    pub compression_ratio: f32,
    pub quality_loss: f32,
}

/// Embedding validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingValidationResult {
    pub is_valid: bool,
    pub dimension_match: bool,
    pub has_nan_values: bool,
    pub has_infinite_values: bool,
    pub magnitude_check: bool,
    pub quality_score: f32,
    pub issues: Vec<String>,
}

/// Versioned embedding for compatibility tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedEmbedding {
    pub embedding: Vec<f32>,
    pub version: String,
    pub model_name: String,
    pub model_version: String,
    pub dimension: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Default embedding model configuration
pub fn default_embedding_config() -> EmbeddingModelConfig {
    EmbeddingModelConfig {
        model_name: "simple-hash".to_string(),
        model_type: EmbeddingModelType::SimpleHash,
        max_length: 512,
        dimension: 384,
        device: EmbeddingDevice::CPU,
        batch_size: 32,
        enable_caching: true,
        cache_size: 10000,
    }
}

/// Default embedding manager configuration
pub fn default_embedding_manager_config() -> EmbeddingManagerConfig {
    EmbeddingManagerConfig {
        default_model: "simple-hash".to_string(),
        enable_caching: true,
        cache_size: 10000,
        batch_size: 32,
        max_concurrent_requests: 100,
    }
} 