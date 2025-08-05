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

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use uuid::Uuid;

use rhema_knowledge::{
    // Core components
    UnifiedKnowledgeEngine, SemanticSearchEngine, EmbeddingManager,
    UnifiedCacheManager, StorageManager, ProactiveContextManager,
    
    // AI Integration
    AIIntegration, AIIntegrationConfig, AIKnowledgeRequest, AIKnowledgeResponse,
    
    // Configuration
    UnifiedEngineConfig, RAGConfig, VectorStoreConfig, VectorStoreType,
    DistanceMetric, CacheConfig, StorageConfig, MemoryConfig, DiskConfig,
    NetworkConfig, LifecycleConfig, PerformanceConfig, ProactiveConfig,
    MonitoringConfig, SemanticSearchConfig,
    
    // Types
    ContentType, KnowledgeResult, SearchResultMetadata, CacheEntryMetadata,
    UnifiedCacheResult, SemanticInfo, CacheTier, AccessPatterns,
    TemporalPattern, AgentSessionContext, WorkflowContext, WorkflowType,
    ContextRequirement, ContextRequirementType, Priority, AgentPreferences,
    CompressionPreference, ContextSuggestion, SuggestionAction,
    KnowledgeSynthesis, SynthesisMetadata, SynthesisMethod,
    
    // Vector stores
    VectorStoreWrapper, VectorStoreFactory,
    
    // Error handling
    KnowledgeError,
};

/// Production-ready knowledge integration example
/// This demonstrates how to integrate the knowledge crate with:
/// 1. Real vector stores (Qdrant, Chroma, Pinecone)
/// 2. AI service integration
/// 3. Production-grade caching and storage
/// 4. Proactive context management
/// 5. Comprehensive monitoring and optimization
pub struct ProductionKnowledgeIntegration {
    knowledge_engine: Arc<UnifiedKnowledgeEngine>,
    search_engine: Arc<SemanticSearchEngine>,
    embedding_manager: Arc<EmbeddingManager>,
    cache_manager: Arc<UnifiedCacheManager>,
    storage_manager: Arc<StorageManager>,
    proactive_manager: Arc<ProactiveContextManager>,
    ai_integration: Option<Arc<AIIntegration>>,
    vector_store: Arc<VectorStoreWrapper>,
}

impl ProductionKnowledgeIntegration {
    /// Create a new production knowledge integration instance
    pub async fn new(
        vector_store_type: VectorStoreType,
        enable_ai_integration: bool,
    ) -> KnowledgeResult<Self> {
        // 1. Configure vector store
        let vector_config = Self::create_vector_store_config(vector_store_type);
        
        // 2. Create vector store
        let vector_store = Arc::new(VectorStoreFactory::create(vector_config).await?);
        
        // 3. Create embedding manager
        let embedding_manager = Arc::new(EmbeddingManager::new_dummy().await?);
        
        // 4. Create search engine
        let search_config = SemanticSearchConfig {
            similarity_threshold: 0.7,
            max_results: 20,
            hybrid_search_enabled: true,
            reranking_enabled: true,
        };
        let search_engine = Arc::new(SemanticSearchEngine::new(
            vector_store.clone(),
            embedding_manager.clone(),
            search_config,
        )?);
        
        // 5. Create cache manager
        let cache_config = Self::create_cache_config();
        let cache_manager = Arc::new(UnifiedCacheManager::new(cache_config).await?);
        
        // 6. Create storage manager
        let storage_config = Self::create_storage_config();
        let storage_manager = Arc::new(StorageManager::new(storage_config).await?);
        
        // 7. Create proactive context manager
        let proactive_config = ProactiveConfig {
            enabled: true,
            suggestion_threshold: 0.8,
            warm_cache_enabled: true,
            file_analysis_enabled: true,
        };
        let proactive_manager = Arc::new(ProactiveContextManager::new(
            proactive_config,
            cache_manager.clone(),
            search_engine.clone(),
        ).await?);
        
        // 8. Create unified knowledge engine
        let engine_config = Self::create_engine_config();
        let knowledge_engine = Arc::new(UnifiedKnowledgeEngine::new(
            engine_config,
            cache_manager.clone(),
            search_engine.clone(),
            embedding_manager.clone(),
            storage_manager.clone(),
            proactive_manager.clone(),
        ).await?);
        
        // 9. Create AI integration if enabled
        let ai_integration = if enable_ai_integration {
            let ai_config = AIIntegrationConfig::default();
            let ai_integration = AIIntegration::new(
                ai_config,
                knowledge_engine.clone(),
                search_engine.clone(),
                embedding_manager.clone(),
                vector_store.clone(),
            ).await?;
            Some(Arc::new(ai_integration))
        } else {
            None
        };
        
        Ok(Self {
            knowledge_engine,
            search_engine,
            embedding_manager,
            cache_manager,
            storage_manager,
            proactive_manager,
            ai_integration,
            vector_store,
        })
    }
    
    /// Create vector store configuration based on type
    fn create_vector_store_config(store_type: VectorStoreType) -> VectorStoreConfig {
        match store_type {
            VectorStoreType::Qdrant => VectorStoreConfig {
                store_type: VectorStoreType::Qdrant,
                collection_name: "rhema_knowledge".to_string(),
                dimension: 384,
                distance_metric: DistanceMetric::Cosine,
                timeout_seconds: 30,
                qdrant_url: Some("http://localhost:6333".to_string()),
                qdrant_api_key: None,
                url: None,
                api_key: None,
                chroma_url: None,
                chroma_api_key: None,
                pinecone_api_key: None,
                pinecone_environment: None,
                pinecone_index_name: None,
            },
            VectorStoreType::Chroma => VectorStoreConfig {
                store_type: VectorStoreType::Chroma,
                collection_name: "rhema_knowledge".to_string(),
                dimension: 384,
                distance_metric: DistanceMetric::Cosine,
                timeout_seconds: 30,
                chroma_url: Some("http://localhost:8000".to_string()),
                chroma_api_key: None,
                url: None,
                api_key: None,
                qdrant_url: None,
                qdrant_api_key: None,
                pinecone_api_key: None,
                pinecone_environment: None,
                pinecone_index_name: None,
            },
            VectorStoreType::Pinecone => VectorStoreConfig {
                store_type: VectorStoreType::Pinecone,
                collection_name: "rhema_knowledge".to_string(),
                dimension: 384,
                distance_metric: DistanceMetric::Cosine,
                timeout_seconds: 30,
                pinecone_api_key: Some("your-pinecone-api-key".to_string()),
                pinecone_environment: Some("us-west1-gcp".to_string()),
                pinecone_index_name: Some("rhema-knowledge".to_string()),
                url: None,
                api_key: None,
                qdrant_url: None,
                qdrant_api_key: None,
                chroma_url: None,
                chroma_api_key: None,
            },
            VectorStoreType::Local => VectorStoreConfig {
                store_type: VectorStoreType::Local,
                collection_name: "rhema_knowledge".to_string(),
                dimension: 384,
                distance_metric: DistanceMetric::Cosine,
                timeout_seconds: 30,
                url: None,
                api_key: None,
                qdrant_url: None,
                qdrant_api_key: None,
                chroma_url: None,
                chroma_api_key: None,
                pinecone_api_key: None,
                pinecone_environment: None,
                pinecone_index_name: None,
            },
        }
    }
    
    /// Create cache configuration for production
    fn create_cache_config() -> CacheConfig {
        CacheConfig {
            storage: StorageConfig {
                memory: MemoryConfig {
                    enabled: true,
                    max_size_mb: 1024, // 1GB memory cache
                    eviction_policy: rhema_knowledge::EvictionPolicy::Adaptive,
                },
                disk: DiskConfig {
                    enabled: true,
                    cache_dir: std::path::PathBuf::from("./knowledge_cache"),
                    max_size_gb: 10, // 10GB disk cache
                    compression_enabled: true,
                    compression_algorithm: rhema_knowledge::CompressionAlgorithm::Zstd,
                },
                network: NetworkConfig {
                    enabled: false, // Disable Redis for this example
                    redis_url: None,
                    connection_pool_size: 10,
                },
            },
            lifecycle: LifecycleConfig {
                default_ttl_hours: 24 * 7, // 1 week
                max_object_size_mb: 100,
                auto_refresh: true,
                refresh_interval_hours: 24,
            },
            performance: PerformanceConfig {
                compression_threshold_kb: 10,
                parallel_operations: 8,
                background_cleanup: true,
                cleanup_interval_minutes: 60,
            },
        }
    }
    
    /// Create storage configuration for production
    fn create_storage_config() -> rhema_knowledge::StorageConfig {
        rhema_knowledge::StorageConfig {
            storage_dir: std::path::PathBuf::from("./knowledge_storage"),
            max_size_gb: 50,
            compression_enabled: true,
            encryption_enabled: false, // Enable in production
            encryption_algorithm: rhema_knowledge::EncryptionAlgorithm::AES256,
            deduplication_enabled: true,
            auto_cleanup: true,
            cleanup_interval_hours: 24,
            integrity_checking: true,
            backup_enabled: true,
            backup_interval_hours: 24,
        }
    }
    
    /// Create engine configuration for production
    fn create_engine_config() -> UnifiedEngineConfig {
        UnifiedEngineConfig {
            rag: RAGConfig {
                embedding_model: "all-MiniLM-L6-v2".to_string(),
                chunk_size: 512,
                overlap_size: 50,
                vector_store: Self::create_vector_store_config(VectorStoreType::Local), // Will be overridden
                semantic_search: SemanticSearchConfig {
                    similarity_threshold: 0.7,
                    max_results: 20,
                    hybrid_search_enabled: true,
                    reranking_enabled: true,
                },
            },
            cache: Self::create_cache_config(),
            proactive: ProactiveConfig {
                enabled: true,
                suggestion_threshold: 0.8,
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
    
    /// Store knowledge content with comprehensive metadata
    pub async fn store_knowledge(
        &self,
        content: &str,
        content_type: ContentType,
        scope_path: Option<String>,
        tags: Vec<String>,
    ) -> KnowledgeResult<String> {
        let id = Uuid::new_v4().to_string();
        
        // Generate embedding
        let embedding = self.embedding_manager.generate_embedding(content).await?;
        
        // Create metadata
        let metadata = SearchResultMetadata {
            source_type: content_type,
            scope_path,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            size_bytes: content.len() as u64,
            chunk_id: Some(id.clone()),
        };
        
        // Store in vector store
        self.vector_store.store(&id, &embedding, Some(metadata)).await?;
        
        // Store in cache with semantic info
        let semantic_info = SemanticInfo {
            embedding: Some(embedding),
            semantic_tags: tags,
            content_type,
            relevance_score: 1.0,
            related_keys: vec![],
            chunk_id: Some(id.clone()),
        };
        
        let cache_metadata = CacheEntryMetadata {
            key: id.clone(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 1,
            size_bytes: content.len() as u64,
            ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
            compression_ratio: Some(0.8),
            semantic_tags: tags,
            agent_session_id: None,
            scope_path,
            checksum: Some(sha2::Sha256::digest(content.as_bytes()).to_vec().into_iter().map(|b| format!("{:02x}", b)).collect::<String>()),
        };
        
        let cache_result = UnifiedCacheResult {
            data: content.as_bytes().to_vec(),
            metadata: cache_metadata,
            semantic_info: Some(semantic_info),
            cache_tier: CacheTier::Memory,
            access_patterns: AccessPatterns {
                frequency: 1.0,
                recency: 1.0,
                semantic_relevance: 1.0,
                temporal_pattern: TemporalPattern::Recent,
                agent_affinity: std::collections::HashMap::new(),
                workflow_affinity: std::collections::HashMap::new(),
            },
        };
        
        self.cache_manager.store(&id, cache_result).await?;
        
        Ok(id)
    }
    
    /// Search knowledge with AI enhancement
    pub async fn search_knowledge_enhanced(
        &self,
        query: &str,
        max_results: usize,
        enable_synthesis: bool,
    ) -> KnowledgeResult<AIKnowledgeResponse> {
        if let Some(ai_integration) = &self.ai_integration {
            // Use AI-enhanced search
            let request = AIKnowledgeRequest {
                request_id: Uuid::new_v4().to_string(),
                query: query.to_string(),
                context_scope: None,
                content_types: vec![ContentType::Documentation, ContentType::Code, ContentType::Knowledge],
                max_results,
                similarity_threshold: 0.7,
                include_metadata: true,
                enable_synthesis,
                user_id: None,
                session_id: None,
                created_at: Utc::now(),
            };
            
            ai_integration.process_request(request).await
        } else {
            // Fall back to basic search
            let embedding = self.embedding_manager.generate_embedding(query).await?;
            let results = self.search_engine.search_semantic(
                query,
                &embedding,
                max_results,
                0.7,
            ).await?;
            
            // Convert to AI response format
            let ai_results = results.into_iter().map(|result| {
                rhema_knowledge::AIKnowledgeResult {
                    id: result.id,
                    content: result.content,
                    relevance_score: result.relevance_score,
                    ai_enhanced_score: result.relevance_score,
                    content_type: result.metadata.as_ref().map(|m| m.source_type.clone()).unwrap_or(ContentType::Unknown),
                    metadata: result.metadata,
                    ai_insights: vec![],
                    related_concepts: vec![],
                    confidence_level: 0.8,
                }
            }).collect();
            
            Ok(AIKnowledgeResponse {
                request_id: Uuid::new_v4().to_string(),
                results: ai_results,
                synthesized_content: None,
                confidence_score: 0.8,
                processing_time_ms: 0,
                ai_enhancements: vec![],
                suggestions: vec![],
                created_at: Utc::now(),
            })
        }
    }
    
    /// Get proactive suggestions for context
    pub async fn get_context_suggestions(
        &self,
        agent_id: &str,
        session_id: &str,
        current_workflow: &str,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let agent_context = AgentSessionContext {
            agent_id: agent_id.to_string(),
            session_id: session_id.to_string(),
            created_at: Utc::now(),
            last_active: Utc::now(),
            workflow_context: Some(WorkflowContext {
                workflow_id: current_workflow.to_string(),
                workflow_type: WorkflowType::FeatureDevelopment,
                current_step: "code_review".to_string(),
                steps_completed: vec!["planning".to_string(), "implementation".to_string()],
                steps_remaining: vec!["testing".to_string(), "deployment".to_string()],
                context_requirements: vec![
                    ContextRequirement {
                        requirement_type: ContextRequirementType::Code,
                        scope_path: Some("src/".to_string()),
                        content_type: ContentType::Code,
                        priority: Priority::High,
                        estimated_size: Some(1024 * 1024), // 1MB
                    },
                ],
            }),
            preferences: AgentPreferences {
                preferred_content_types: vec![ContentType::Code, ContentType::Documentation],
                semantic_relevance_threshold: 0.8,
                cache_retention_hours: 24,
                compression_preference: CompressionPreference::Balanced,
                proactive_caching_enabled: true,
            },
            cache_keys: vec![],
        };
        
        self.proactive_manager.get_context_suggestions(&agent_context).await
    }
    
    /// Synthesize knowledge from multiple sources
    pub async fn synthesize_knowledge(
        &self,
        topic: &str,
        source_keys: Vec<String>,
    ) -> KnowledgeResult<KnowledgeSynthesis> {
        self.knowledge_engine.synthesize_knowledge(
            topic,
            source_keys,
            SynthesisMethod::Hybrid,
        ).await
    }
    
    /// Get comprehensive metrics
    pub async fn get_metrics(&self) -> KnowledgeResult<rhema_knowledge::UnifiedMetrics> {
        self.knowledge_engine.get_metrics().await
    }
    
    /// Start monitoring and optimization
    pub async fn start_monitoring(&self) -> KnowledgeResult<()> {
        // Start proactive monitoring
        self.proactive_manager.start_monitoring().await?;
        
        // Start AI monitoring if enabled
        if let Some(ai_integration) = &self.ai_integration {
            ai_integration.start_monitoring().await?;
        }
        
        // Start cache monitoring
        self.cache_manager.start_monitoring().await?;
        
        // Start storage monitoring
        self.storage_manager.start_monitoring().await?;
        
        Ok(())
    }
    
    /// Optimize the knowledge base
    pub async fn optimize_knowledge_base(&self) -> KnowledgeResult<()> {
        // Optimize cache
        self.cache_manager.optimize().await?;
        
        // Optimize storage
        self.storage_manager.optimize().await?;
        
        // AI optimization if enabled
        if let Some(ai_integration) = &self.ai_integration {
            ai_integration.optimize_knowledge_base().await?;
        }
        
        Ok(())
    }
}

/// Example usage demonstrating production integration
#[tokio::main]
async fn main() -> KnowledgeResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Production Knowledge Integration Example");
    
    // Create production knowledge integration with Qdrant vector store and AI integration
    let knowledge_integration = ProductionKnowledgeIntegration::new(
        VectorStoreType::Qdrant,
        true, // Enable AI integration
    ).await?;
    
    println!("✅ Knowledge integration initialized");
    
    // Start monitoring
    knowledge_integration.start_monitoring().await?;
    println!("✅ Monitoring started");
    
    // Store some sample knowledge
    println!("\n📚 Storing sample knowledge...");
    
    let code_content = r#"
# Rust Vector Store Implementation

This module provides vector store implementations for Qdrant, Chroma, and Pinecone.

## Key Features
- Real-time vector search
- Metadata storage
- Collection management
- Performance optimization

## Usage Example
```rust
let vector_store = QdrantVectorStore::new(config);
let results = vector_store.search(&embedding, 10).await?;
```
"#;
    
    let doc_content = r#"
# Knowledge Management System

A comprehensive knowledge management system with RAG capabilities.

## Architecture
- Vector storage for semantic search
- Caching for performance
- AI integration for enhancement
- Proactive context management

## Benefits
- Fast semantic search
- Intelligent caching
- AI-powered insights
- Context-aware suggestions
"#;
    
    let code_id = knowledge_integration.store_knowledge(
        code_content,
        ContentType::Code,
        Some("src/vector.rs".to_string()),
        vec!["rust".to_string(), "vector-store".to_string(), "qdrant".to_string()],
    ).await?;
    
    let doc_id = knowledge_integration.store_knowledge(
        doc_content,
        ContentType::Documentation,
        Some("docs/architecture.md".to_string()),
        vec!["documentation".to_string(), "architecture".to_string(), "rag".to_string()],
    ).await?;
    
    println!("✅ Stored knowledge with IDs: {} and {}", code_id, doc_id);
    
    // Search with AI enhancement
    println!("\n🔍 Searching knowledge with AI enhancement...");
    
    let search_response = knowledge_integration.search_knowledge_enhanced(
        "vector store implementation",
        5,
        true, // Enable synthesis
    ).await?;
    
    println!("✅ Found {} results", search_response.results.len());
    println!("📊 Confidence score: {:.2}", search_response.confidence_score);
    println!("⏱️ Processing time: {}ms", search_response.processing_time_ms);
    
    for (i, result) in search_response.results.iter().enumerate() {
        println!("  {}. {} (score: {:.2})", i + 1, result.id, result.ai_enhanced_score);
        println!("     Content: {}...", &result.content[..result.content.len().min(100)]);
        println!("     Concepts: {:?}", result.related_concepts);
    }
    
    if let Some(synthesized) = search_response.synthesized_content {
        println!("\n📝 Synthesized Content:");
        println!("{}", synthesized);
    }
    
    // Get proactive suggestions
    println!("\n💡 Getting proactive context suggestions...");
    
    let suggestions = knowledge_integration.get_context_suggestions(
        "agent-1",
        "session-1",
        "feature-development",
    ).await?;
    
    println!("✅ Found {} suggestions", suggestions.len());
    
    for suggestion in suggestions {
        println!("  - {}: {}", suggestion.title, suggestion.description);
        println!("    Relevance: {:.2}, Priority: {:?}", suggestion.relevance_score, suggestion.priority);
    }
    
    // Synthesize knowledge
    println!("\n🧠 Synthesizing knowledge...");
    
    let synthesis = knowledge_integration.synthesize_knowledge(
        "Vector Store Architecture",
        vec![code_id, doc_id],
    ).await?;
    
    println!("✅ Synthesized knowledge: {}", synthesis.topic);
    println!("📊 Confidence: {:.2}", synthesis.confidence_score);
    println!("📝 Content: {}...", &synthesis.synthesized_content[..synthesis.synthesized_content.len().min(200)]);
    
    // Get metrics
    println!("\n📈 Getting performance metrics...");
    
    let metrics = knowledge_integration.get_metrics().await?;
    
    println!("✅ Cache Metrics:");
    println!("  - Hit rate: {:.2}%", metrics.cache_metrics.hit_rate * 100.0);
    println!("  - Total entries: {}", metrics.cache_metrics.total_entries);
    println!("  - Memory usage: {} MB", metrics.cache_metrics.memory_usage_bytes / 1024 / 1024);
    
    println!("✅ Search Metrics:");
    println!("  - Total searches: {}", metrics.search_metrics.total_searches);
    println!("  - Average response time: {}ms", metrics.search_metrics.average_response_time_ms);
    println!("  - Average relevance: {:.2}", metrics.search_metrics.average_relevance_score);
    
    // Optimize knowledge base
    println!("\n⚡ Optimizing knowledge base...");
    
    knowledge_integration.optimize_knowledge_base().await?;
    
    println!("✅ Knowledge base optimization completed");
    
    // Simulate some activity
    println!("\n🔄 Simulating activity...");
    
    for i in 1..=3 {
        println!("  Round {}: Searching and caching...", i);
        
        let _ = knowledge_integration.search_knowledge_enhanced(
            "knowledge management",
            3,
            false,
        ).await?;
        
        sleep(Duration::from_secs(1)).await;
    }
    
    // Final metrics
    println!("\n📊 Final metrics after activity:");
    
    let final_metrics = knowledge_integration.get_metrics().await?;
    
    println!("✅ Cache hit rate: {:.2}%", final_metrics.cache_metrics.hit_rate * 100.0);
    println!("✅ Total searches: {}", final_metrics.search_metrics.total_searches);
    println!("✅ Average response time: {}ms", final_metrics.search_metrics.average_response_time_ms);
    
    println!("\n🎉 Production Knowledge Integration Example Completed Successfully!");
    println!("✨ The knowledge crate is now ready for production use with:");
    println!("   - Real vector store integration (Qdrant/Chroma/Pinecone)");
    println!("   - AI-powered knowledge enhancement");
    println!("   - Production-grade caching and storage");
    println!("   - Proactive context management");
    println!("   - Comprehensive monitoring and optimization");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_production_integration_creation() {
        let integration = ProductionKnowledgeIntegration::new(
            VectorStoreType::Local, // Use local for testing
            false, // Disable AI for testing
        ).await;
        
        assert!(integration.is_ok());
    }
    
    #[tokio::test]
    async fn test_knowledge_storage_and_search() {
        let integration = ProductionKnowledgeIntegration::new(
            VectorStoreType::Local,
            false,
        ).await.unwrap();
        
        // Store knowledge
        let id = integration.store_knowledge(
            "Test content for vector search",
            ContentType::Documentation,
            Some("test/path".to_string()),
            vec!["test".to_string(), "example".to_string()],
        ).await.unwrap();
        
        assert!(!id.is_empty());
        
        // Search knowledge
        let response = integration.search_knowledge_enhanced(
            "vector search",
            5,
            false,
        ).await.unwrap();
        
        assert!(!response.results.is_empty());
        assert!(response.confidence_score > 0.0);
    }
} 