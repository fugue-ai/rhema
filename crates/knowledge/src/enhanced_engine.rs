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
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::types::{
    AgentSessionContext, CacheEntryMetadata, ContentType, KnowledgeResult, SemanticResult,
    UnifiedCacheResult, UnifiedEngineConfig, UnifiedMetrics, WorkflowContext,
};

use super::{
    cache::{SemanticDiskCache, SemanticDiskConfig, SemanticMemoryCache, SemanticCacheConfig},
    embedding::EmbeddingManager,
    search::SemanticSearchEngine,
    synthesis::KnowledgeSynthesizer,
    vector::VectorStoreFactory,
    cross_session::CrossSessionManager,
    enhanced_cache::EnhancedCacheManager,
    proactive::ProactiveContextManager,
    performance::PerformanceMonitor,
    metrics::MetricsCollector,
};

/// Error types for the enhanced unified knowledge engine
#[derive(Error, Debug)]
pub enum EnhancedEngineError {
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Search error: {0}")]
    SearchError(String),
    
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    
    #[error("Vector store error: {0}")]
    VectorError(String),
    
    #[error("Synthesis error: {0}")]
    SynthesisError(String),
    
    #[error("Cross-session error: {0}")]
    CrossSessionError(String),
    
    #[error("Proactive error: {0}")]
    ProactiveError(String),
    
    #[error("Performance error: {0}")]
    PerformanceError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Agent session error: {0}")]
    AgentSessionError(String),
    
    #[error("Workflow error: {0}")]
    WorkflowError(String),
}

/// Enhanced unified knowledge engine with cross-session management
pub struct EnhancedUnifiedKnowledgeEngine {
    // Core components
    embedding_manager: Arc<EmbeddingManager>,
    vector_store: Arc<dyn crate::vector::VectorStore>,
    semantic_search: Arc<SemanticSearchEngine>,
    
    // Enhanced cache components
    enhanced_cache_manager: Arc<EnhancedCacheManager>,
    
    // Cross-session management
    cross_session_manager: Arc<CrossSessionManager>,
    
    // Proactive features
    proactive_manager: Arc<ProactiveContextManager>,
    
    // Knowledge synthesis
    knowledge_synthesizer: Arc<KnowledgeSynthesizer>,
    
    // Agent session management
    agent_sessions: Arc<RwLock<HashMap<String, AgentSessionContext>>>,
    
    // Performance and monitoring
    performance_monitor: Arc<PerformanceMonitor>,
    metrics_collector: Arc<MetricsCollector>,
    
    // Configuration and monitoring
    config: EnhancedEngineConfig,
    metrics: Arc<RwLock<UnifiedMetrics>>,
}

/// Enhanced engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedEngineConfig {
    pub base_config: UnifiedEngineConfig,
    pub cross_session: CrossSessionConfig,
    pub enhanced_caching: EnhancedCachingConfig,
    pub proactive: EnhancedProactiveConfig,
    pub performance: PerformanceConfig,
}

/// Cross-session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSessionConfig {
    pub enabled: bool,
    pub context_sharing_enabled: bool,
    pub session_persistence_enabled: bool,
    pub semantic_clustering_enabled: bool,
    pub max_shared_contexts: usize,
    pub context_ttl_hours: u64,
    pub auto_synthesis_enabled: bool,
}

impl Default for CrossSessionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            context_sharing_enabled: true,
            session_persistence_enabled: true,
            semantic_clustering_enabled: true,
            max_shared_contexts: 100,
            context_ttl_hours: 168, // 1 week
            auto_synthesis_enabled: true,
        }
    }
}

/// Enhanced caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCachingConfig {
    pub semantic_aware_caching: bool,
    pub adaptive_eviction: bool,
    pub intelligent_warming: bool,
    pub cross_tier_optimization: bool,
    pub compression_optimization: bool,
    pub cache_synchronization: bool,
}

impl Default for EnhancedCachingConfig {
    fn default() -> Self {
        Self {
            semantic_aware_caching: true,
            adaptive_eviction: true,
            intelligent_warming: true,
            cross_tier_optimization: true,
            compression_optimization: true,
            cache_synchronization: true,
        }
    }
}

/// Enhanced proactive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedProactiveConfig {
    pub ai_driven_suggestions: bool,
    pub workflow_prediction: bool,
    pub agent_profiling: bool,
    pub real_time_monitoring: bool,
    pub context_impact_analysis: bool,
    pub suggestion_threshold: f32,
    pub prediction_confidence_threshold: f32,
}

impl Default for EnhancedProactiveConfig {
    fn default() -> Self {
        Self {
            ai_driven_suggestions: true,
            workflow_prediction: true,
            agent_profiling: true,
            real_time_monitoring: true,
            context_impact_analysis: true,
            suggestion_threshold: 0.7,
            prediction_confidence_threshold: 0.8,
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub monitoring_enabled: bool,
    pub metrics_collection_enabled: bool,
    pub performance_alerting: bool,
    pub auto_optimization: bool,
    pub resource_monitoring: bool,
    pub optimization_threshold: f32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            metrics_collection_enabled: true,
            performance_alerting: true,
            auto_optimization: true,
            resource_monitoring: true,
            optimization_threshold: 0.8,
        }
    }
}

impl EnhancedUnifiedKnowledgeEngine {
    pub async fn new(config: EnhancedEngineConfig) -> KnowledgeResult<Self> {
        info!("Initializing enhanced unified knowledge engine");
        
        // Initialize embedding manager
        let embedding_config = crate::embedding::default_embedding_manager_config();
        let embedding_manager = Arc::new(EmbeddingManager::new(embedding_config).await?);
        
        // Initialize vector store
        let vector_store = VectorStoreFactory::create(config.base_config.rag.vector_store.clone()).await?;
        
        // Initialize semantic search engine
        let semantic_search = Arc::new(SemanticSearchEngine::new(
            embedding_manager.clone(),
            vector_store.clone(),
            config.base_config.rag.semantic_search.clone(),
        ).await?);
        
        // Initialize enhanced cache manager
        let enhanced_cache_manager = Arc::new(EnhancedCacheManager::new(
            config.enhanced_caching.clone(),
            embedding_manager.clone(),
            vector_store.clone(),
        ).await?);
        
        // Initialize cross-session manager
        let cross_session_manager = Arc::new(CrossSessionManager::new(
            config.cross_session.clone(),
            enhanced_cache_manager.clone(),
            embedding_manager.clone(),
        ).await?);
        
        // Initialize knowledge synthesizer
        let knowledge_synthesizer = Arc::new(KnowledgeSynthesizer::new(
            embedding_manager.clone(),
            vector_store.clone(),
        ).await?);
        
        // Initialize proactive manager
        let proactive_manager = Arc::new(ProactiveContextManager::new(
            config.proactive.clone(),
            enhanced_cache_manager.clone(),
            cross_session_manager.clone(),
            embedding_manager.clone(),
        ).await?);
        
        // Initialize performance monitor
        let performance_monitor = Arc::new(PerformanceMonitor::new(
            config.performance.clone(),
        ));
        
        // Initialize metrics collector
        let metrics_collector = Arc::new(MetricsCollector::new(
            config.performance.metrics_collection_enabled,
        ));
        
        // Initialize agent sessions
        let agent_sessions = Arc::new(RwLock::new(HashMap::new()));
        
        // Initialize metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        
        let engine = Self {
            embedding_manager,
            vector_store,
            semantic_search,
            enhanced_cache_manager,
            cross_session_manager,
            proactive_manager,
            knowledge_synthesizer,
            agent_sessions,
            performance_monitor,
            metrics_collector,
            config,
            metrics,
        };
        
        info!("Enhanced unified knowledge engine initialized successfully");
        Ok(engine)
    }
    
    /// Get data with enhanced RAG and cross-session awareness
    pub async fn get_with_enhanced_rag(
        &self,
        key: &str,
        query: Option<&str>,
        agent_id: Option<&str>,
    ) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        let start_time = Instant::now();
        
        // Try enhanced cache with semantic fallback
        if let Some(result) = self.enhanced_cache_manager
            .get_with_semantic_fallback(key, query)
            .await? {
            self.update_metrics_cache_hit(start_time).await;
            return Ok(Some(result));
        }
        
        // If query provided, try semantic search with cross-session enhancement
        if let Some(query) = query {
            let semantic_results = self.semantic_search.search_semantic(query, 5).await?;
            
            // Enhance with cross-session context
            let enhanced_results = self.cross_session_manager
                .enhance_search_results_with_cross_session_context(
                    &semantic_results,
                    agent_id,
                ).await?;
            
            // Check if any enhanced results match the key or are highly relevant
            for result in enhanced_results {
                if result.cache_key == key || result.relevance_score > 0.8 {
                    // Promote to cache and return
                    let cached_result = self.promote_to_enhanced_cache(&result, agent_id).await?;
                    self.update_metrics_semantic_hit(start_time).await;
                    return Ok(Some(cached_result));
                }
            }
        }
        
        self.update_metrics_cache_miss(start_time).await;
        Ok(None)
    }
    
    /// Set data with enhanced semantic indexing and cross-session awareness
    pub async fn set_with_enhanced_indexing(
        &self,
        key: &str,
        data: &[u8],
        metadata: &Option<CacheEntryMetadata>,
        agent_id: Option<&str>,
    ) -> KnowledgeResult<()> {
        let start_time = Instant::now();
        
        // Store in enhanced cache
        self.enhanced_cache_manager
            .set_with_semantic_indexing(key, data, metadata.clone(), agent_id)
            .await?;
        
        // Update cross-session context if agent_id provided
        if let Some(agent_id) = agent_id {
            self.cross_session_manager
                .update_agent_context(agent_id, key, data, metadata)
                .await?;
        }
        
        // Generate proactive suggestions
        if self.config.proactive.ai_driven_suggestions {
            let suggestions = self.proactive_manager
                .generate_context_suggestions_for_data(key, data, agent_id)
                .await?;
            
            if !suggestions.is_empty() {
                debug!("Generated {} proactive suggestions for key: {}", suggestions.len(), key);
            }
        }
        
        self.update_metrics_set(start_time).await;
        Ok(())
    }
    
    /// Enhanced semantic search with cross-session context
    pub async fn search_with_cross_session_context(
        &self,
        query: &str,
        limit: usize,
        agent_id: Option<&str>,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let start_time = Instant::now();
        
        // Perform semantic search
        let search_results = self.semantic_search.search_semantic(query, limit).await?;
        
        // Enhance with cross-session context
        let enhanced_results = self.cross_session_manager
            .enhance_search_results_with_cross_session_context(
                &search_results,
                agent_id,
            ).await?;
        
        // Enhance with cache information
        let final_results = self.enhanced_cache_manager
            .enhance_with_cache_info(&enhanced_results)
            .await?;
        
        self.update_metrics_search(start_time).await;
        Ok(final_results)
    }
    
    /// Warm cache for agent session with enhanced intelligence
    pub async fn warm_cache_for_agent_session(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<()> {
        info!("Warming enhanced cache for agent session: {}", agent_id);
        
        // Analyze session context to predict needed data
        let predicted_keys = self.cross_session_manager
            .predict_agent_context_needs(agent_id, session_context)
            .await?;
        
        // Pre-load predicted context into agent-specific cache
        for key in predicted_keys {
            self.enhanced_cache_manager
                .prewarm_agent_context(agent_id, &key)
                .await?;
        }
        
        // Generate proactive suggestions
        if self.config.proactive.ai_driven_suggestions {
            let suggestions = self.proactive_manager
                .generate_context_suggestions_for_session(agent_id, session_context)
                .await?;
            
            if !suggestions.is_empty() {
                debug!("Generated {} proactive suggestions for agent session: {}", suggestions.len(), agent_id);
            }
        }
        
        Ok(())
    }
    
    /// Get agent-specific context with cross-session enhancement
    pub async fn get_agent_context(
        &self,
        agent_id: &str,
        key: &str,
    ) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        // Try agent-specific cache first
        let agent_key = format!("agent:{}:{}", agent_id, key);
        if let Some(result) = self.enhanced_cache_manager.get(&agent_key).await? {
            return Ok(Some(result));
        }
        
        // Try cross-session context sharing
        if self.config.cross_session.context_sharing_enabled {
            if let Some(shared_result) = self.cross_session_manager
                .get_shared_context(agent_id, key)
                .await? {
                return Ok(Some(shared_result));
            }
        }
        
        // Fall back to global cache
        self.enhanced_cache_manager.get(key).await
    }
    
    /// Set agent-specific context with cross-session awareness
    pub async fn set_agent_context(
        &self,
        agent_id: &str,
        key: &str,
        data: &[u8],
    ) -> KnowledgeResult<()> {
        let agent_key = format!("agent:{}:{}", agent_id, key);
        self.enhanced_cache_manager
            .set(&agent_key, data, None, Some(agent_id))
            .await?;
        
        // Update cross-session context
        if self.config.cross_session.context_sharing_enabled {
            self.cross_session_manager
                .update_agent_context(agent_id, key, data, None)
                .await?;
        }
        
        Ok(())
    }
    
    /// Share context between agents with enhanced intelligence
    pub async fn share_context_across_agents(
        &self,
        source_agent_id: &str,
        target_agent_id: &str,
        context_key: &str,
    ) -> KnowledgeResult<()> {
        if let Some(cached_context) = self.get_agent_context(source_agent_id, context_key).await? {
            self.set_agent_context(target_agent_id, context_key, &cached_context.data).await?;
            
            // Update cross-session relationship
            self.cross_session_manager
                .record_context_sharing(source_agent_id, target_agent_id, context_key)
                .await?;
        }
        Ok(())
    }
    
    /// Synthesize knowledge with cross-session awareness
    pub async fn synthesize_knowledge_with_cross_session(
        &self,
        topic: &str,
        scope_path: Option<&str>,
        agent_id: Option<&str>,
    ) -> KnowledgeResult<crate::types::KnowledgeSynthesis> {
        let synthesis = self.knowledge_synthesizer.synthesize(topic, scope_path).await?;
        
        // Enhance with cross-session context if agent_id provided
        if let Some(agent_id) = agent_id {
            let enhanced_synthesis = self.cross_session_manager
                .enhance_synthesis_with_cross_session_context(
                    &synthesis,
                    agent_id,
                ).await?;
            return Ok(enhanced_synthesis);
        }
        
        Ok(synthesis)
    }
    
    /// Get proactive context suggestions
    pub async fn get_context_suggestions(
        &self,
        agent_id: &str,
        current_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<crate::types::ContextSuggestion>> {
        self.proactive_manager
            .generate_context_suggestions_for_session(agent_id, current_context)
            .await
    }
    
    /// Get cross-session context synthesis
    pub async fn get_cross_session_synthesis(
        &self,
        session_ids: &[String],
        topic: &str,
    ) -> KnowledgeResult<Option<crate::types::CrossSessionContext>> {
        self.cross_session_manager
            .synthesize_cross_session_context(session_ids, topic)
            .await
    }
    
    /// Get enhanced metrics
    pub async fn get_enhanced_metrics(&self) -> EnhancedMetrics {
        let base_metrics = self.metrics.read().await.clone();
        let cache_metrics = self.enhanced_cache_manager.get_metrics().await;
        let cross_session_metrics = self.cross_session_manager.get_metrics().await;
        let proactive_metrics = self.proactive_manager.get_metrics().await;
        let performance_metrics = self.performance_monitor.get_metrics().await;
        
        EnhancedMetrics {
            base_metrics,
            cache_metrics,
            cross_session_metrics,
            proactive_metrics,
            performance_metrics,
        }
    }
    
    // Private helper methods
    
    async fn promote_to_enhanced_cache(
        &self,
        result: &SemanticResult,
        agent_id: Option<&str>,
    ) -> KnowledgeResult<UnifiedCacheResult> {
        // Create cache entry from search result
        let cache_entry = self.create_cache_entry_from_search_result(result).await?;
        
        // Store in enhanced cache
        self.enhanced_cache_manager
            .set(&result.cache_key, &cache_entry.data, Some(cache_entry.metadata.clone()), agent_id)
            .await?;
        
        Ok(UnifiedCacheResult {
            data: cache_entry.data,
            metadata: cache_entry.metadata,
            semantic_info: Some(crate::types::SemanticInfo {
                embedding: Some(result.embedding.clone()),
                semantic_tags: result.semantic_tags.clone(),
                content_type: result.metadata.source_type.clone(),
                relevance_score: result.relevance_score,
                related_keys: vec![],
                chunk_id: result.metadata.chunk_id.clone(),
            }),
            cache_tier: crate::types::CacheTier::Memory,
            access_patterns: crate::types::AccessPatterns {
                frequency: 1.0,
                recency: 1.0,
                semantic_relevance: result.relevance_score,
                temporal_pattern: crate::types::TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
            },
        })
    }
    
    async fn create_cache_entry_from_search_result(&self, result: &SemanticResult) -> KnowledgeResult<UnifiedCacheResult> {
        // Convert search result to cache entry
        let content_bytes = result.content.as_bytes().to_vec();
        
        Ok(UnifiedCacheResult {
            data: content_bytes,
            metadata: CacheEntryMetadata {
                key: result.cache_key.clone(),
                created_at: result.metadata.created_at,
                accessed_at: chrono::Utc::now(),
                access_count: 1,
                size_bytes: result.content.len() as u64,
                ttl: Duration::from_secs(3600),
                compression_ratio: None,
                semantic_tags: result.semantic_tags.clone(),
                agent_session_id: None,
                scope_path: result.metadata.scope_path.clone(),
            },
            semantic_info: Some(crate::types::SemanticInfo {
                embedding: Some(result.embedding.clone()),
                semantic_tags: result.semantic_tags.clone(),
                content_type: result.metadata.source_type.clone(),
                relevance_score: result.relevance_score,
                related_keys: vec![],
                chunk_id: result.metadata.chunk_id.clone(),
            }),
            cache_tier: crate::types::CacheTier::Memory,
            access_patterns: crate::types::AccessPatterns {
                frequency: 1.0,
                recency: 1.0,
                semantic_relevance: result.relevance_score,
                temporal_pattern: crate::types::TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
            },
        })
    }
    
    async fn update_metrics_cache_hit(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.hit_count += 1;
        metrics.performance_metrics.average_cache_access_time_ms = start_time.elapsed().as_millis() as u64;
    }
    
    async fn update_metrics_cache_miss(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.miss_count += 1;
        metrics.performance_metrics.average_cache_access_time_ms = start_time.elapsed().as_millis() as u64;
    }
    
    async fn update_metrics_semantic_hit(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.semantic_hit_rate += 1.0;
        metrics.search_metrics.cache_enhanced_searches += 1;
    }
    
    async fn update_metrics_set(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.total_entries += 1;
    }
    
    async fn update_metrics_search(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.search_metrics.total_searches += 1;
        metrics.search_metrics.semantic_searches += 1;
        metrics.search_metrics.average_response_time_ms = start_time.elapsed().as_millis() as u64;
    }
}

/// Enhanced metrics for the unified knowledge system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMetrics {
    pub base_metrics: UnifiedMetrics,
    pub cache_metrics: crate::enhanced_cache::EnhancedCacheMetrics,
    pub cross_session_metrics: crate::cross_session::CrossSessionMetrics,
    pub proactive_metrics: crate::proactive::ProactiveMetrics,
    pub performance_metrics: crate::performance::PerformanceMetrics,
} 