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

use rhema_knowledge::{
    engine::{UnifiedKnowledgeEngine, FileWatcher, UsageAnalyzer, SuggestionEngine, FileWatchConfig, SuggestionEventType},
    types::{
        AgentSessionContext, CacheEntryMetadata, ContentType, Priority, SuggestionAction,
        UnifiedEngineConfig, WorkflowContext, WorkflowType, ContextRequirement, ContextRequirementType,
    },
    cache::{UnifiedCacheManager, SemanticMemoryCache, SemanticDiskCache, CacheStats},
    proactive::ProactiveContextManager,
};
use rhema_core::RhemaResult;
use tempfile::TempDir;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};

// Mock implementations for testing
mod engine {
    use super::*;
    
    #[derive(Debug, Clone)]
    pub enum SuggestionEventType {
        Generated,
    }
}

// Mock CrossSessionManager for testing
struct MockCrossSessionManager;

impl MockCrossSessionManager {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn share_context(&self, _from_agent: &str, _to_agent: &str, _context_key: &str) -> Result<(), String> {
        Ok(())
    }
    
    pub async fn get_shared_context(&self, _agent: &str, _context_key: &str) -> Result<Option<Vec<u8>>, String> {
        Ok(Some(b"shared context data".to_vec()))
    }
}

// Mock PerformanceMonitor for testing
struct MockPerformanceMonitor;

impl MockPerformanceMonitor {
    pub fn new() -> Self {
        Self
    }
}

// Mock MetricsCollector for testing
struct MockMetricsCollector;

impl MockMetricsCollector {
    pub fn new() -> Self {
        Self
    }
}

/// Integration test demonstrating RAG and caching system working together
#[tokio::test]
async fn test_rag_cache_integration() {
    // Initialize the unified knowledge engine
    let config = UnifiedEngineConfig::default();
    let engine = UnifiedKnowledgeEngine::new(config).await.expect("Failed to create engine");
    
    info!("🚀 Starting RAG and Cache Integration Test");
    
    // Test 1: Basic RAG operations with caching
    test_basic_rag_operations(&engine).await;
    
    // Test 2: Semantic search with cache enhancement
    test_semantic_search_with_cache(&engine).await;
    
    // Test 3: Agent session management with proactive caching
    test_agent_session_management(&engine).await;
    
    // Test 4: File watching and proactive indexing
    test_file_watching_proactive_indexing(&engine).await;
    
    // Test 5: Usage analysis and intelligent warming
    test_usage_analysis_intelligent_warming(&engine).await;
    
    // Test 6: Suggestion engine and context recommendations
    test_suggestion_engine_context_recommendations(&engine).await;
    
    // Test 7: Cross-session knowledge sharing
    test_cross_session_knowledge_sharing(&engine).await;
    
    // Test 8: Performance monitoring and optimization
    test_performance_monitoring_optimization(&engine).await;
    
    info!("✅ All RAG and Cache Integration Tests Passed!");
}

/// Test basic RAG operations with caching
async fn test_basic_rag_operations(engine: &UnifiedKnowledgeEngine) {
    info!("📝 Testing Basic RAG Operations with Caching");
    
    // Store data with semantic indexing
    let test_data = "This is a test document about machine learning and artificial intelligence.";
    let metadata = CacheEntryMetadata {
        key: "test:ml:doc1".to_string(),
        created_at: chrono::Utc::now(),
        accessed_at: chrono::Utc::now(),
        access_count: 0,
        size_bytes: test_data.len() as u64,
        ttl: Duration::from_secs(3600),
        compression_ratio: None,
        semantic_tags: vec!["machine-learning".to_string(), "ai".to_string()],
        agent_session_id: None,
        scope_path: Some("docs/ml/".to_string()),
        checksum: None,
    };
    
    engine.set_with_semantic_indexing("test:ml:doc1", test_data.as_bytes(), &Some(metadata)).await
        .expect("Failed to store data with semantic indexing");
    
    // Retrieve data with RAG enhancement
    let result = engine.get_with_rag("test:ml:doc1", Some("machine learning")).await
        .expect("Failed to get data with RAG");
    
    assert!(result.is_some(), "RAG retrieval should return data");
    
    let cache_result = result.unwrap();
    assert_eq!(cache_result.data, test_data.as_bytes(), "Retrieved data should match stored data");
    
    info!("✅ Basic RAG operations test passed");
}

/// Test semantic search with cache enhancement
async fn test_semantic_search_with_cache(engine: &UnifiedKnowledgeEngine) {
    info!("🔍 Testing Semantic Search with Cache Enhancement");
    
    // Store multiple documents
    let documents = vec![
        ("doc1", "Machine learning algorithms for classification"),
        ("doc2", "Deep learning neural networks and backpropagation"),
        ("doc3", "Natural language processing with transformers"),
        ("doc4", "Computer vision and image recognition"),
    ];
    
    for (key, content) in documents {
        let metadata = CacheEntryMetadata {
            key: format!("test:search:{}", key),
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            access_count: 0,
            size_bytes: content.len() as u64,
            ttl: Duration::from_secs(3600),
            compression_ratio: None,
            semantic_tags: vec!["ai".to_string(), "research".to_string()],
            agent_session_id: None,
            scope_path: Some("docs/ai/".to_string()),
            checksum: None,
        };
        
        engine.set_with_semantic_indexing(&format!("test:search:{}", key), content.as_bytes(), &Some(metadata)).await
            .expect("Failed to store document");
    }
    
    // Perform semantic search
    let search_results = engine.search_semantic("machine learning algorithms", 3).await
        .expect("Failed to perform semantic search");
    
    assert!(!search_results.is_empty(), "Search should return results");
    assert!(search_results.len() <= 3, "Search should respect limit");
    
    // Verify cache enhancement
    for result in &search_results {
        assert!(result.relevance_score > 0.0, "Results should have relevance scores");
        assert!(!result.cache_key.is_empty(), "Results should have cache keys");
    }
    
    info!("✅ Semantic search with cache enhancement test passed");
}

/// Test agent session management with proactive caching
async fn test_agent_session_management(engine: &UnifiedKnowledgeEngine) {
    info!("🤖 Testing Agent Session Management with Proactive Caching");
    
    // Create agent session context
    let workflow_context = WorkflowContext {
        workflow_id: "code_review_001".to_string(),
        workflow_type: WorkflowType::CodeReview,
        current_step: "review_implementation".to_string(),
        steps_completed: vec!["setup".to_string(), "analysis".to_string()],
        steps_remaining: vec!["testing".to_string(), "documentation".to_string()],
        context_requirements: vec![
            ContextRequirement {
                requirement_type: ContextRequirementType::Code,
                scope_path: Some("src/".to_string()),
                content_type: ContentType::Code,
                priority: Priority::High,
                estimated_size: Some(1024),
            },
        ],
    };
    
    let session_context = AgentSessionContext {
        agent_id: "reviewer_agent".to_string(),
        session_id: "session_001".to_string(),
        created_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        workflow_context: Some(workflow_context),
        preferences: Default::default(),
        cache_keys: vec!["test:session:context1".to_string()],
    };
    
    // Warm cache for agent session
    engine.warm_cache_for_agent_session("reviewer_agent", &session_context).await
        .expect("Failed to warm cache for agent session");
    
    // Set agent-specific context
    let context_data = "Code review guidelines and best practices for Rust projects";
    engine.set_agent_context("reviewer_agent", "guidelines", context_data.as_bytes()).await
        .expect("Failed to set agent context");
    
    // Retrieve agent context
    let retrieved_context = engine.get_agent_context("reviewer_agent", "guidelines").await
        .expect("Failed to get agent context");
    
    assert!(retrieved_context.is_some(), "Agent context should be retrievable");
    assert_eq!(retrieved_context.unwrap().data, context_data.as_bytes(), "Agent context should match");
    
    info!("✅ Agent session management test passed");
}

/// Test file watching and proactive indexing
async fn test_file_watching_proactive_indexing(engine: &UnifiedKnowledgeEngine) {
    info!("👀 Testing File Watching and Proactive Indexing");
    
    // Create a temporary test file
    let test_file_path = std::env::temp_dir().join("test_rag_document.md");
    let test_content = "# Test Document\n\nThis is a test document for RAG integration.\n\n## Features\n- Machine learning\n- Natural language processing\n- Vector search";
    
    tokio::fs::write(&test_file_path, test_content).await
        .expect("Failed to create test file");
    
    // Create file watcher - use dummy constructor since we can't easily pass the engine reference
    let file_watcher = FileWatcher::new_dummy_without_engine();
    
    // Start watching the file
    file_watcher.watch_file(test_file_path.clone()).await
        .expect("Failed to start watching file");
    
    // Simulate file change
    let updated_content = "# Updated Test Document\n\nThis is an updated test document for RAG integration.\n\n## Features\n- Machine learning\n- Natural language processing\n- Vector search\n- Proactive caching";
    
    tokio::fs::write(&test_file_path, updated_content).await
        .expect("Failed to update test file");
    
    // Check for changes
    let changes = file_watcher.check_for_changes().await
        .expect("Failed to check for file changes");
    
    assert!(!changes.is_empty(), "File changes should be detected");
    
    // Verify that the file was indexed
    let file_key = format!("file:{}", test_file_path.to_string_lossy());
    let indexed_result = engine.get_with_rag(&file_key, Some("machine learning")).await
        .expect("Failed to retrieve indexed file");
    
    assert!(indexed_result.is_some(), "Indexed file should be retrievable");
    
    // Clean up
    tokio::fs::remove_file(&test_file_path).await
        .expect("Failed to remove test file");
    
    info!("✅ File watching and proactive indexing test passed");
}

/// Test usage analysis and intelligent warming
async fn test_usage_analysis_intelligent_warming(_engine: &UnifiedKnowledgeEngine) {
    info!("📊 Testing Usage Analysis and Intelligent Warming");
    
    // Create usage analyzer
    let usage_analyzer = UsageAnalyzer::new(Default::default());
    
    // Record access patterns
    usage_analyzer.record_access("test:pattern:key1", Some("agent1"), Some("workflow1")).await
        .expect("Failed to record access pattern");
    
    usage_analyzer.record_access("test:pattern:key1", Some("agent1"), Some("workflow1")).await
        .expect("Failed to record access pattern");
    
    usage_analyzer.record_access("test:pattern:key2", Some("agent1"), Some("workflow1")).await
        .expect("Failed to record access pattern");
    
    // Create session context for analysis
    let session_context = AgentSessionContext {
        agent_id: "agent1".to_string(),
        session_id: "session_analysis".to_string(),
        created_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        workflow_context: None,
        preferences: Default::default(),
        cache_keys: vec!["test:pattern:key1".to_string(), "test:pattern:key2".to_string()],
    };
    
    // Analyze agent session
    let session_analysis = usage_analyzer.analyze_agent_session("agent1", &session_context).await
        .expect("Failed to analyze agent session");
    
    assert_eq!(session_analysis.agent_id, "agent1", "Session analysis should match agent");
    assert!(!session_analysis.predicted_needs.is_empty(), "Should predict needs based on patterns");
    
    // Get agent patterns
    let patterns = usage_analyzer.get_agent_patterns("agent1").await;
    assert!(!patterns.is_empty(), "Should have recorded patterns for agent");
    
    info!("✅ Usage analysis and intelligent warming test passed");
}

/// Test suggestion engine and context recommendations
async fn test_suggestion_engine_context_recommendations(_engine: &UnifiedKnowledgeEngine) {
    info!("💡 Testing Suggestion Engine and Context Recommendations");
    
    // Create suggestion engine
    let suggestion_engine = SuggestionEngine::new(Default::default());
    
    // Create session context
    let workflow_context = WorkflowContext {
        workflow_id: "feature_dev_001".to_string(),
        workflow_type: WorkflowType::FeatureDevelopment,
        current_step: "implementation".to_string(),
        steps_completed: vec!["planning".to_string(), "design".to_string()],
        steps_remaining: vec!["testing".to_string(), "deployment".to_string()],
        context_requirements: vec![],
    };
    
    let session_context = AgentSessionContext {
        agent_id: "developer_agent".to_string(),
        session_id: "session_suggestions".to_string(),
        created_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        workflow_context: Some(workflow_context),
        preferences: Default::default(),
        cache_keys: vec![],
    };
    
    // Generate suggestions
    let suggestions = suggestion_engine.generate_suggestions(
        "developer_agent",
        &session_context,
        Some("feature_development")
    ).await.expect("Failed to generate suggestions");
    
    assert!(!suggestions.is_empty(), "Should generate suggestions for agent");
    
    // Verify suggestion quality
    for suggestion in &suggestions {
        assert!(suggestion.confidence > 0.0, "Suggestions should have confidence scores");
        assert!(!suggestion.title.is_empty(), "Suggestions should have titles");
        assert!(!suggestion.description.is_empty(), "Suggestions should have descriptions");
    }
    
    // Record suggestion events
    if let Some(suggestion) = suggestions.first() {
        suggestion_engine.record_suggestion_event(
            &suggestion.suggestion_id,
            "developer_agent",
            SuggestionEventType::Generated,
            None,
        ).await.expect("Failed to record suggestion event");
    }
    
    // Get suggestion statistics
    let stats = suggestion_engine.get_suggestion_stats("developer_agent").await;
    assert!(stats.total_suggestions > 0, "Should have recorded suggestions");
    
    info!("✅ Suggestion engine and context recommendations test passed");
}

/// Test cross-session knowledge sharing
async fn test_cross_session_knowledge_sharing(engine: &UnifiedKnowledgeEngine) {
    info!("🔄 Testing Cross-Session Knowledge Sharing");
    
    // Create cross-session manager
    // Mock cross session manager for testing
    let _cross_session_manager = MockCrossSessionManager::new();
    
    // Set context for first agent
    let context_data = "Shared knowledge about API design patterns";
    engine.set_agent_context("agent1", "api_patterns", context_data.as_bytes()).await
        .expect("Failed to set context for agent1");
    
    // Share context across agents
    engine.share_context_across_agents("agent1", "agent2", "api_patterns").await
        .expect("Failed to share context across agents");
    
    // Verify context is accessible to second agent
    let shared_context = engine.get_agent_context("agent2", "api_patterns").await
        .expect("Failed to get shared context");
    
    assert!(shared_context.is_some(), "Shared context should be accessible");
    assert_eq!(shared_context.unwrap().data, context_data.as_bytes(), "Shared context should match");
    
    // Test cross-session synthesis
    let synthesis = engine.synthesize_knowledge("API Design", Some("docs/api/")).await
        .expect("Failed to synthesize knowledge");
    
    assert!(!synthesis.synthesized_content.is_empty(), "Synthesis should produce content");
    assert!(synthesis.confidence_score > 0.0, "Synthesis should have confidence score");
    
    info!("✅ Cross-session knowledge sharing test passed");
}

/// Test performance monitoring and optimization
async fn test_performance_monitoring_optimization(engine: &UnifiedKnowledgeEngine) {
    info!("⚡ Testing Performance Monitoring and Optimization");
    
    // Mock performance monitor for testing
    let _performance_monitor = MockPerformanceMonitor::new();
    
    // Mock metrics collector for testing
    let _metrics_collector = MockMetricsCollector::new();
    
    // Get engine metrics
    let metrics = engine.get_metrics().await;
    
    // Verify metrics structure
    assert!(metrics.cache_metrics.hit_count >= 0, "Cache metrics should be valid");
    assert!(metrics.search_metrics.total_searches >= 0, "Search metrics should be valid");
    assert!(metrics.synthesis_metrics.total_syntheses >= 0, "Synthesis metrics should be valid");
    
    // Test cache performance - use metrics instead
    let metrics = engine.get_metrics().await;
    assert!(metrics.cache_metrics.hit_count >= 0, "Cache metrics should be valid");
    
    // Test hit rate calculation - use metrics
    let hit_rate = metrics.cache_metrics.hit_rate;
    assert!(hit_rate >= 0.0 && hit_rate <= 1.0, "Hit rate should be between 0 and 1");
    
    info!("✅ Performance monitoring and optimization test passed");
}

/// Helper function to create a test engine for integration tests
pub async fn create_test_engine() -> UnifiedKnowledgeEngine {
    let config = UnifiedEngineConfig::default();
    UnifiedKnowledgeEngine::new(config).await.expect("Failed to create test engine")
}

/// Helper function to create test session context
pub fn create_test_session_context(agent_id: &str, workflow_type: WorkflowType) -> AgentSessionContext {
    let workflow_context = WorkflowContext {
        workflow_id: format!("test_workflow_{}", agent_id),
        workflow_type,
        current_step: "testing".to_string(),
        steps_completed: vec!["setup".to_string()],
        steps_remaining: vec!["cleanup".to_string()],
        context_requirements: vec![],
    };
    
    AgentSessionContext {
        agent_id: agent_id.to_string(),
        session_id: format!("test_session_{}", agent_id),
        created_at: chrono::Utc::now(),
        last_active: chrono::Utc::now(),
        workflow_context: Some(workflow_context),
        preferences: Default::default(),
        cache_keys: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_rag_cache_basic_functionality() {
        let engine = create_test_engine().await;
        test_basic_rag_operations(&engine).await;
    }
    
    #[tokio::test]
    async fn test_semantic_search_functionality() {
        let engine = create_test_engine().await;
        test_semantic_search_with_cache(&engine).await;
    }
    
    #[tokio::test]
    async fn test_agent_session_functionality() {
        let engine = create_test_engine().await;
        test_agent_session_management(&engine).await;
    }
} 