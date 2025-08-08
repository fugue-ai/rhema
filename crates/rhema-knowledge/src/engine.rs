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

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::types::DistanceMetric;
use crate::types::{
    AccessPatterns, AgentSessionContext, CacheEntryMetadata, CacheTier, ContentType,
    ContextRequirement, ContextSuggestion, KnowledgeResult, Priority, SearchResultMetadata,
    SemanticResult, SuggestionAction, TemporalPattern, UnifiedCacheResult, UnifiedEngineConfig,
    UnifiedMetrics,
};

use super::{
    cache::{SemanticCacheConfig, SemanticDiskCache, SemanticDiskConfig, SemanticMemoryCache},
    embedding::EmbeddingManager,
    search::SemanticSearchEngine,
    synthesis::KnowledgeSynthesizer,
    vector::VectorStoreFactory,
};

/// Error types for the unified knowledge engine
#[derive(Error, Debug)]
pub enum EngineError {
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

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Agent session error: {0}")]
    AgentSessionError(String),

    #[error("Workflow error: {0}")]
    WorkflowError(String),
}

/// Unified knowledge engine that combines RAG and caching
pub struct UnifiedKnowledgeEngine {
    // RAG components
    embedding_manager: Arc<EmbeddingManager>,
    vector_store: Arc<dyn crate::vector::VectorStore>,
    semantic_search: Arc<SemanticSearchEngine>,

    // Cache components
    memory_cache: Arc<SemanticMemoryCache>,
    disk_cache: Arc<SemanticDiskCache>,
    network_cache: Option<Arc<DistributedRAGCache>>,

    // Knowledge synthesis
    knowledge_synthesizer: Arc<KnowledgeSynthesizer>,

    // Proactive features
    proactive_manager: Option<Arc<ProactiveContextManager>>,

    // Agent session management
    agent_sessions: Arc<RwLock<HashMap<String, AgentSessionContext>>>,

    // Configuration and monitoring
    config: UnifiedEngineConfig,
    metrics: Arc<RwLock<UnifiedMetrics>>,
}

/// Distributed RAG cache for network-level caching
pub struct DistributedRAGCache {
    redis_client: Arc<redis::Client>,
    distributed_vector_store: Arc<dyn crate::vector::VectorStore>,
    config: DistributedRAGConfig,
    connection_pool: Arc<ConnectionPool>,
}

/// Distributed RAG configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedRAGConfig {
    pub redis_url: String,
    pub connection_pool_size: usize,
    pub enable_replication: bool,
    pub replication_factor: usize,
    pub auto_failover: bool,
}

/// Connection pool for distributed operations
pub struct ConnectionPool {
    pool: Arc<redis::aio::ConnectionManager>,
}

/// Proactive context manager
pub struct ProactiveContextManager {
    unified_engine: Option<Arc<UnifiedKnowledgeEngine>>,
    file_watcher: Arc<FileWatcher>,
    usage_analyzer: Arc<UsageAnalyzer>,
    suggestion_engine: Arc<SuggestionEngine>,
}

/// File watcher for proactive caching
pub struct FileWatcher {
    watched_files: Arc<RwLock<HashMap<PathBuf, FileWatchInfo>>>,
    unified_engine: Arc<UnifiedKnowledgeEngine>,
    config: FileWatchConfig,
}

/// File watch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatchConfig {
    pub enabled: bool,
    pub watch_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub auto_index: bool,
    pub cache_on_change: bool,
    pub semantic_indexing: bool,
}

impl Default for FileWatchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_patterns: vec!["*.rs".to_string(), "*.md".to_string(), "*.toml".to_string()],
            ignore_patterns: vec![
                "target/".to_string(),
                ".git/".to_string(),
                "node_modules/".to_string(),
            ],
            auto_index: true,
            cache_on_change: true,
            semantic_indexing: true,
        }
    }
}

/// File watch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatchInfo {
    pub path: PathBuf,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub file_size: u64,
    pub hash: String,
    pub indexed: bool,
    pub cached: bool,
}

impl FileWatcher {
    pub fn new(unified_engine: Arc<UnifiedKnowledgeEngine>, config: FileWatchConfig) -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            unified_engine,
            config,
        }
    }

    /// Start watching a file
    pub async fn watch_file(&self, file_path: PathBuf) -> KnowledgeResult<()> {
        if !self.should_watch_file(&file_path) {
            return Ok(());
        }

        let file_info = self.get_file_info(&file_path).await?;
        let mut watched = self.watched_files.write().await;
        watched.insert(file_path.clone(), file_info);

        info!("Started watching file: {:?}", file_path);
        Ok(())
    }

    /// Stop watching a file
    pub async fn unwatch_file(&self, file_path: &PathBuf) -> KnowledgeResult<()> {
        let mut watched = self.watched_files.write().await;
        watched.remove(file_path);

        info!("Stopped watching file: {:?}", file_path);
        Ok(())
    }

    /// Check for file changes and trigger proactive actions
    pub async fn check_for_changes(&self) -> KnowledgeResult<Vec<FileChangeEvent>> {
        let changes = Vec::new();
        let mut watched = self.watched_files.write().await;

        for (path, info) in watched.iter_mut() {
            if let Some(new_info) = self.get_file_info(path).await.ok() {
                if new_info.hash != info.hash {
                    info.hash = new_info.hash.clone();
                    info.last_modified = new_info.last_modified;
                    info.file_size = new_info.file_size;
                    info.indexed = false;
                    info.cached = false;

                    self.handle_file_change(path, &new_info).await?;
                }
            }
        }

        Ok(changes)
    }

    /// Handle file change event
    async fn handle_file_change(
        &self,
        path: &PathBuf,
        file_info: &FileWatchInfo,
    ) -> KnowledgeResult<()> {
        info!("File changed: {:?}", path);

        // Read file content
        let content = tokio::fs::read(path).await?;
        let key = format!("file:{}", path.to_string_lossy());

        // Create metadata
        let metadata = CacheEntryMetadata {
            key: format!("file:{}", path.to_string_lossy()),
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            access_count: 0,
            size_bytes: file_info.file_size,
            ttl: std::time::Duration::from_secs(3600), // 1 hour
            compression_ratio: None,
            semantic_tags: vec!["file-watch".to_string(), "auto-indexed".to_string()],
            agent_session_id: None,
            scope_path: Some(path.to_string_lossy().to_string()),
            checksum: None,
        };

        // Cache the file content
        if self.config.cache_on_change {
            self.unified_engine
                .set_with_semantic_indexing(&key, &content, &Some(metadata))
                .await?;
        }

        // Index semantically if enabled
        if self.config.semantic_indexing {
            // Extract text content and create semantic entry
            if let Some(text_content) = String::from_utf8(content.clone()).ok() {
                let semantic_key = format!("semantic:{}", key);
                let semantic_metadata = CacheEntryMetadata {
                    key: format!("semantic:{}", key),
                    created_at: chrono::Utc::now(),
                    accessed_at: chrono::Utc::now(),
                    access_count: 0,
                    size_bytes: text_content.len() as u64,
                    ttl: std::time::Duration::from_secs(3600), // 1 hour
                    compression_ratio: None,
                    semantic_tags: vec!["semantic".to_string(), "file-watch".to_string()],
                    agent_session_id: None,
                    scope_path: Some(path.to_string_lossy().to_string()),
                    checksum: None,
                };

                self.unified_engine
                    .set_with_semantic_indexing(
                        &semantic_key,
                        text_content.as_bytes(),
                        &Some(semantic_metadata),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    /// Check if file should be watched based on patterns
    fn should_watch_file(&self, path: &PathBuf) -> bool {
        let path_str = path.to_string_lossy();

        // Check ignore patterns first
        for pattern in &self.config.ignore_patterns {
            if path_str.contains(pattern) {
                return false;
            }
        }

        // Check watch patterns
        for pattern in &self.config.watch_patterns {
            if path_str.ends_with(pattern) {
                return true;
            }
        }

        false
    }

    /// Get file information
    async fn get_file_info(&self, path: &PathBuf) -> KnowledgeResult<FileWatchInfo> {
        let metadata = tokio::fs::metadata(path).await?;
        let content = tokio::fs::read(path).await?;
        let hash = self.calculate_hash(&content);

        Ok(FileWatchInfo {
            path: path.clone(),
            last_modified: chrono::DateTime::from(metadata.modified()?),
            file_size: metadata.len(),
            hash,
            indexed: false,
            cached: false,
        })
    }

    /// Calculate file hash
    fn calculate_hash(&self, content: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Get list of watched files
    pub async fn get_watched_files(&self) -> Vec<FileWatchInfo> {
        let watched = self.watched_files.read().await;
        watched.values().cloned().collect()
    }
}

/// File change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub old_hash: String,
    pub new_hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Usage analyzer for intelligent cache warming
pub struct UsageAnalyzer {
    access_patterns: Arc<RwLock<HashMap<String, AccessPattern>>>,
    agent_sessions: Arc<RwLock<HashMap<String, AgentSessionAnalysis>>>,
    workflow_patterns: Arc<RwLock<HashMap<String, WorkflowPattern>>>,
    config: UsageAnalysisConfig,
}

/// Usage analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalysisConfig {
    pub enabled: bool,
    pub pattern_detection_enabled: bool,
    pub prediction_enabled: bool,
    pub analysis_window_hours: u64,
    pub min_pattern_confidence: f32,
    pub max_patterns_per_agent: usize,
}

impl Default for UsageAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pattern_detection_enabled: true,
            prediction_enabled: true,
            analysis_window_hours: 24,
            min_pattern_confidence: 0.7,
            max_patterns_per_agent: 10,
        }
    }
}

/// Access pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub key: String,
    pub frequency: f32,
    pub recency: f32,
    pub semantic_relevance: f32,
    pub temporal_pattern: TemporalPattern,
    pub agent_affinity: HashMap<String, f32>,
    pub workflow_affinity: HashMap<String, f32>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub confidence: f32,
}

/// Agent session analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSessionAnalysis {
    pub agent_id: String,
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub access_patterns: Vec<String>,
    pub workflow_context: Option<String>,
    pub predicted_needs: Vec<PredictedNeed>,
    pub session_duration: std::time::Duration,
}

/// Workflow pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPattern {
    pub workflow_id: String,
    pub workflow_type: String,
    pub common_access_patterns: Vec<String>,
    pub typical_duration: std::time::Duration,
    pub success_rate: f32,
    pub context_requirements: Vec<ContextRequirement>,
}

/// Predicted need for cache warming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedNeed {
    pub key: String,
    pub confidence: f32,
    pub reasoning: String,
    pub priority: Priority,
    pub estimated_access_time: chrono::DateTime<chrono::Utc>,
}

impl UsageAnalyzer {
    pub fn new(config: UsageAnalysisConfig) -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            workflow_patterns: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Record access pattern
    pub async fn record_access(
        &self,
        key: &str,
        agent_id: Option<&str>,
        workflow_id: Option<&str>,
    ) -> KnowledgeResult<()> {
        let mut patterns = self.access_patterns.write().await;
        let now = chrono::Utc::now();

        let pattern = patterns
            .entry(key.to_string())
            .or_insert_with(|| AccessPattern {
                key: key.to_string(),
                frequency: 0.0,
                recency: 0.0,
                semantic_relevance: 0.0,
                temporal_pattern: TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
                last_accessed: now,
                access_count: 0,
                confidence: 0.0,
            });

        // Update access count and timing
        pattern.access_count += 1;
        pattern.last_accessed = now;

        // Update frequency (exponential moving average)
        pattern.frequency = 0.9 * pattern.frequency + 0.1;

        // Update recency (time-based decay)
        let hours_since_last = (now - pattern.last_accessed).num_hours() as f32;
        pattern.recency = (-hours_since_last / 24.0).exp();

        // Update agent affinity
        if let Some(agent_id) = agent_id {
            let affinity = pattern
                .agent_affinity
                .entry(agent_id.to_string())
                .or_insert(0.0);
            *affinity = 0.9 * *affinity + 0.1;
        }

        // Update workflow affinity
        if let Some(workflow_id) = workflow_id {
            let affinity = pattern
                .workflow_affinity
                .entry(workflow_id.to_string())
                .or_insert(0.0);
            *affinity = 0.9 * *affinity + 0.1;
        }

        // Update temporal pattern
        pattern.temporal_pattern = self.detect_temporal_pattern(pattern).await;

        // Update confidence based on consistency
        pattern.confidence = self.calculate_pattern_confidence(pattern).await;

        Ok(())
    }

    /// Analyze agent session for patterns
    pub async fn analyze_agent_session(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<AgentSessionAnalysis> {
        let mut sessions = self.agent_sessions.write().await;
        let now = chrono::Utc::now();

        let session_key = format!("{}:{}", agent_id, session_context.session_id);
        let session_analysis =
            sessions
                .entry(session_key.clone())
                .or_insert_with(|| AgentSessionAnalysis {
                    agent_id: agent_id.to_string(),
                    session_id: session_context.session_id.clone(),
                    created_at: session_context.created_at,
                    last_active: session_context.last_active,
                    access_patterns: vec![],
                    workflow_context: session_context
                        .workflow_context
                        .as_ref()
                        .map(|w| w.workflow_id.clone()),
                    predicted_needs: vec![],
                    session_duration: (now - session_context.created_at)
                        .to_std()
                        .unwrap_or_default(),
                });

        // Update last active time
        session_analysis.last_active = now;
        session_analysis.session_duration = (now - session_context.created_at)
            .to_std()
            .unwrap_or_default();

        // Analyze access patterns for this session
        let patterns = self.access_patterns.read().await;
        let session_patterns: Vec<String> = patterns
            .iter()
            .filter(|(_, pattern)| {
                *pattern.agent_affinity.get(agent_id).unwrap_or(&0.0)
                    > self.config.min_pattern_confidence
            })
            .map(|(key, _)| key.clone())
            .collect();

        session_analysis.access_patterns = session_patterns;

        // Generate predictions for this session
        session_analysis.predicted_needs = self
            .predict_session_needs(agent_id, session_context)
            .await?;

        Ok(session_analysis.clone())
    }

    /// Predict what an agent will need based on session context
    pub async fn predict_session_needs(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<PredictedNeed>> {
        let mut predictions = Vec::new();
        let patterns = self.access_patterns.read().await;

        // Get high-confidence patterns for this agent
        let agent_patterns: Vec<&AccessPattern> = patterns
            .values()
            .filter(|pattern| {
                *pattern.agent_affinity.get(agent_id).unwrap_or(&0.0)
                    > self.config.min_pattern_confidence
            })
            .collect();

        // Predict based on workflow context
        if let Some(workflow) = &session_context.workflow_context {
            let workflow_patterns = self.workflow_patterns.read().await;
            if let Some(workflow_pattern) = workflow_patterns.get(&workflow.workflow_id) {
                for pattern_key in &workflow_pattern.common_access_patterns {
                    if let Some(pattern) = patterns.get(pattern_key) {
                        predictions.push(PredictedNeed {
                            key: pattern_key.clone(),
                            confidence: pattern.confidence * 0.8, // Slightly lower for workflow-based prediction
                            reasoning: format!(
                                "Common pattern for workflow {}",
                                workflow.workflow_id
                            ),
                            priority: Priority::High,
                            estimated_access_time: chrono::Utc::now()
                                + chrono::Duration::minutes(30),
                        });
                    }
                }
            }
        }

        // Predict based on agent's historical patterns
        for pattern in agent_patterns
            .iter()
            .take(self.config.max_patterns_per_agent)
        {
            predictions.push(PredictedNeed {
                key: pattern.key.clone(),
                confidence: pattern.confidence,
                reasoning: format!("Historical pattern for agent {}", agent_id),
                priority: if pattern.frequency > 0.8 {
                    Priority::Critical
                } else {
                    Priority::Medium
                },
                estimated_access_time: chrono::Utc::now() + chrono::Duration::minutes(15),
            });
        }

        // Sort by confidence and priority
        predictions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(predictions)
    }

    /// Detect temporal pattern from access history
    async fn detect_temporal_pattern(&self, pattern: &AccessPattern) -> TemporalPattern {
        // Simple heuristic based on frequency and recency
        if pattern.frequency > 0.8 {
            TemporalPattern::Frequent
        } else if pattern.recency > 0.8 {
            TemporalPattern::Recent
        } else if pattern.frequency > 0.5 {
            TemporalPattern::Stable
        } else {
            TemporalPattern::Declining
        }
    }

    /// Calculate pattern confidence based on consistency
    async fn calculate_pattern_confidence(&self, pattern: &AccessPattern) -> f32 {
        let frequency_weight = 0.4;
        let recency_weight = 0.3;
        let affinity_weight = 0.3;

        let frequency_score = pattern.frequency;
        let recency_score = pattern.recency;
        let affinity_score = pattern
            .agent_affinity
            .values()
            .fold(0.0f32, |max, &val| max.max(val));

        frequency_weight * frequency_score
            + recency_weight * recency_score
            + affinity_weight * affinity_score
    }

    /// Get access patterns for a specific agent
    pub async fn get_agent_patterns(&self, agent_id: &str) -> Vec<AccessPattern> {
        let patterns = self.access_patterns.read().await;
        patterns
            .values()
            .filter(|pattern| *pattern.agent_affinity.get(agent_id).unwrap_or(&0.0) > 0.0)
            .cloned()
            .collect()
    }

    /// Get workflow patterns
    pub async fn get_workflow_patterns(&self) -> Vec<WorkflowPattern> {
        let patterns = self.workflow_patterns.read().await;
        patterns.values().cloned().collect()
    }

    /// Clean up old patterns
    pub async fn cleanup_old_patterns(&self) -> KnowledgeResult<()> {
        let cutoff_time =
            chrono::Utc::now() - chrono::Duration::hours(self.config.analysis_window_hours as i64);

        let mut patterns = self.access_patterns.write().await;
        patterns.retain(|_, pattern| pattern.last_accessed > cutoff_time);

        let mut sessions = self.agent_sessions.write().await;
        sessions.retain(|_, session| session.last_active > cutoff_time);

        Ok(())
    }
}

/// Suggestion engine for context recommendations
pub struct SuggestionEngine {
    suggestions: Arc<RwLock<HashMap<String, ContextSuggestion>>>,
    suggestion_history: Arc<RwLock<Vec<SuggestionEvent>>>,
    config: SuggestionEngineConfig,
}

/// Suggestion engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEngineConfig {
    pub enabled: bool,
    pub suggestion_threshold: f32,
    pub max_suggestions_per_agent: usize,
    pub suggestion_ttl_hours: u64,
    pub enable_learning: bool,
    pub confidence_threshold: f32,
}

impl Default for SuggestionEngineConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            suggestion_threshold: 0.8,
            max_suggestions_per_agent: 10,
            suggestion_ttl_hours: 24,
            enable_learning: true,
            confidence_threshold: 0.7,
        }
    }
}

/// Suggestion event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEvent {
    pub suggestion_id: String,
    pub agent_id: String,
    pub event_type: SuggestionEventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub feedback: Option<SuggestionFeedback>,
}

/// Suggestion event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionEventType {
    Generated,
    Presented,
    Accepted,
    Rejected,
    Ignored,
    Expired,
}

/// Suggestion feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionFeedback {
    pub rating: u8, // 1-5 scale
    pub comment: Option<String>,
    pub was_helpful: bool,
    pub action_taken: Option<String>,
}

impl SuggestionEngine {
    pub fn new(config: SuggestionEngineConfig) -> Self {
        Self {
            suggestions: Arc::new(RwLock::new(HashMap::new())),
            suggestion_history: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }

    /// Generate context suggestions for an agent
    pub async fn generate_suggestions(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
        current_workflow: Option<&str>,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();

        // Generate workflow-based suggestions
        if let Some(workflow) = current_workflow {
            suggestions.extend(
                self.generate_workflow_suggestions(agent_id, workflow, session_context)
                    .await?,
            );
        }

        // Generate pattern-based suggestions
        suggestions.extend(
            self.generate_pattern_suggestions(agent_id, session_context)
                .await?,
        );

        // Generate semantic suggestions
        suggestions.extend(
            self.generate_semantic_suggestions(agent_id, session_context)
                .await?,
        );

        // Filter by threshold and limit
        suggestions.retain(|s| s.confidence >= self.config.confidence_threshold);
        suggestions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(self.config.max_suggestions_per_agent);

        // Store suggestions
        self.store_suggestions(agent_id, &suggestions).await?;

        Ok(suggestions)
    }

    /// Generate workflow-based suggestions
    async fn generate_workflow_suggestions(
        &self,
        _agent_id: &str,
        workflow: &str,
        _session_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();

        // Common workflow patterns
        let workflow_suggestions = match workflow {
            "code_review" => vec![
                (
                    "Review Guidelines",
                    "Common code review patterns and best practices",
                    0.8,
                ),
                (
                    "Recent Changes",
                    "Recently modified files that may need review",
                    0.7,
                ),
                (
                    "Test Coverage",
                    "Test files related to the code being reviewed",
                    0.6,
                ),
            ],
            "feature_development" => vec![
                (
                    "Feature Requirements",
                    "Requirements and specifications for the feature",
                    0.9,
                ),
                (
                    "Related Components",
                    "Existing components that may be affected",
                    0.7,
                ),
                (
                    "Testing Strategy",
                    "Testing approaches for the new feature",
                    0.6,
                ),
            ],
            "bug_fixing" => vec![
                ("Bug Reports", "Related bug reports and issue tracking", 0.8),
                (
                    "Error Logs",
                    "Recent error logs and debugging information",
                    0.7,
                ),
                ("Fix History", "Previous fixes for similar issues", 0.6),
            ],
            "documentation" => vec![
                (
                    "Style Guide",
                    "Documentation style and formatting guidelines",
                    0.8,
                ),
                (
                    "Related Docs",
                    "Related documentation that may need updates",
                    0.7,
                ),
                ("Examples", "Code examples and usage patterns", 0.6),
            ],
            _ => vec![(
                "Workflow Context",
                "General context for the current workflow",
                0.6,
            )],
        };

        for (title, description, confidence) in workflow_suggestions {
            suggestions.push(ContextSuggestion {
                suggestion_id: format!(
                    "workflow:{}:{}",
                    workflow,
                    title.to_lowercase().replace(" ", "_")
                ),
                title: title.to_string(),
                description: description.to_string(),
                relevance_score: confidence,
                content_type: ContentType::Knowledge,
                cache_key: None,
                scope_path: None,
                reasoning: format!("Based on current workflow: {}", workflow),
                confidence,
                action: SuggestionAction::Preload,
            });
        }

        Ok(suggestions)
    }

    /// Generate pattern-based suggestions
    async fn generate_pattern_suggestions(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();

        // Analyze session context for patterns
        let session_duration = chrono::Utc::now() - session_context.created_at;
        let is_long_session = session_duration.num_hours() > 2;

        if is_long_session {
            suggestions.push(ContextSuggestion {
                suggestion_id: format!("pattern:long_session:{}", agent_id),
                title: "Session Summary".to_string(),
                description: "Generate a summary of the current session for future reference"
                    .to_string(),
                relevance_score: 0.7,
                content_type: ContentType::Knowledge,
                cache_key: None,
                scope_path: None,
                reasoning: "Long session detected - summary would be helpful".to_string(),
                confidence: 0.7,
                action: SuggestionAction::Synthesize,
            });
        }

        // Check for frequently accessed patterns
        if session_context.cache_keys.len() > 5 {
            suggestions.push(ContextSuggestion {
                suggestion_id: format!("pattern:frequent_access:{}", agent_id),
                title: "Frequently Accessed Context".to_string(),
                description: "Context that has been accessed multiple times in this session"
                    .to_string(),
                relevance_score: 0.8,
                content_type: ContentType::Knowledge,
                cache_key: None,
                scope_path: None,
                reasoning: "Multiple context accesses detected - consolidation may be helpful"
                    .to_string(),
                confidence: 0.8,
                action: SuggestionAction::Preload,
            });
        }

        Ok(suggestions)
    }

    /// Generate semantic suggestions
    async fn generate_semantic_suggestions(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();

        // Analyze current context for semantic relationships
        if let Some(workflow) = &session_context.workflow_context {
            let current_step = &workflow.current_step;

            // Suggest next steps based on current workflow
            suggestions.push(ContextSuggestion {
                suggestion_id: format!("semantic:next_step:{}", agent_id),
                title: "Next Steps".to_string(),
                description: format!("Suggested next steps after completing: {}", current_step),
                relevance_score: 0.8,
                content_type: ContentType::Knowledge,
                cache_key: None,
                scope_path: None,
                reasoning: format!("Based on workflow step: {}", current_step),
                confidence: 0.8,
                action: SuggestionAction::Preload,
            });
        }

        // Suggest related knowledge based on session context
        suggestions.push(ContextSuggestion {
            suggestion_id: format!("semantic:related_knowledge:{}", agent_id),
            title: "Related Knowledge".to_string(),
            description: "Knowledge that may be relevant to the current session".to_string(),
            relevance_score: 0.6,
            content_type: ContentType::Knowledge,
            cache_key: None,
            scope_path: None,
            reasoning: "Based on semantic analysis of session context".to_string(),
            confidence: 0.6,
            action: SuggestionAction::Index,
        });

        Ok(suggestions)
    }

    /// Store suggestions for tracking
    async fn store_suggestions(
        &self,
        agent_id: &str,
        suggestions: &[ContextSuggestion],
    ) -> KnowledgeResult<()> {
        let mut stored = self.suggestions.write().await;

        for suggestion in suggestions {
            stored.insert(suggestion.suggestion_id.clone(), suggestion.clone());

            // Record generation event
            self.record_suggestion_event(
                &suggestion.suggestion_id,
                agent_id,
                SuggestionEventType::Generated,
                None,
            )
            .await?;
        }

        Ok(())
    }

    /// Record suggestion event
    pub async fn record_suggestion_event(
        &self,
        suggestion_id: &str,
        agent_id: &str,
        event_type: SuggestionEventType,
        feedback: Option<SuggestionFeedback>,
    ) -> KnowledgeResult<()> {
        let event = SuggestionEvent {
            suggestion_id: suggestion_id.to_string(),
            agent_id: agent_id.to_string(),
            event_type,
            timestamp: chrono::Utc::now(),
            feedback,
        };

        let mut history = self.suggestion_history.write().await;
        history.push(event);

        Ok(())
    }

    /// Get suggestion history for an agent
    pub async fn get_suggestion_history(&self, agent_id: &str, hours: u64) -> Vec<SuggestionEvent> {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        let history = self.suggestion_history.read().await;

        history
            .iter()
            .filter(|event| event.agent_id == agent_id && event.timestamp > cutoff_time)
            .cloned()
            .collect()
    }

    /// Get suggestion statistics
    pub async fn get_suggestion_stats(&self, agent_id: &str) -> SuggestionStats {
        let history = self.suggestion_history.read().await;
        let agent_events: Vec<&SuggestionEvent> = history
            .iter()
            .filter(|event| event.agent_id == agent_id)
            .collect();

        let total_suggestions = agent_events.len();
        let accepted = agent_events
            .iter()
            .filter(|e| matches!(e.event_type, SuggestionEventType::Accepted))
            .count();
        let rejected = agent_events
            .iter()
            .filter(|e| matches!(e.event_type, SuggestionEventType::Rejected))
            .count();
        let ignored = agent_events
            .iter()
            .filter(|e| matches!(e.event_type, SuggestionEventType::Ignored))
            .count();

        let acceptance_rate = if total_suggestions > 0 {
            accepted as f32 / total_suggestions as f32
        } else {
            0.0
        };

        SuggestionStats {
            total_suggestions,
            accepted,
            rejected,
            ignored,
            acceptance_rate,
        }
    }

    /// Clean up expired suggestions
    pub async fn cleanup_expired_suggestions(&self) -> KnowledgeResult<()> {
        let cutoff_time =
            chrono::Utc::now() - chrono::Duration::hours(self.config.suggestion_ttl_hours as i64);

        let mut suggestions = self.suggestions.write().await;
        suggestions.retain(|_, suggestion| {
            // Keep suggestions that are still relevant
            suggestion.confidence > self.config.suggestion_threshold
        });

        let mut history = self.suggestion_history.write().await;
        history.retain(|event| event.timestamp > cutoff_time);

        Ok(())
    }
}

/// Suggestion statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionStats {
    pub total_suggestions: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub ignored: usize,
    pub acceptance_rate: f32,
}

impl UnifiedKnowledgeEngine {
    pub async fn new(config: UnifiedEngineConfig) -> KnowledgeResult<Self> {
        info!("Initializing unified knowledge engine");

        // Initialize embedding manager
        let embedding_config = crate::embedding::default_embedding_manager_config();
        let embedding_manager = Arc::new(EmbeddingManager::new(embedding_config).await?);

        // Initialize vector store
        let vector_store =
            Arc::new(VectorStoreFactory::create(config.rag.vector_store.clone()).await?);

        // Initialize semantic search engine
        let semantic_search = Arc::new(
            SemanticSearchEngine::new(
                embedding_manager.clone(),
                vector_store.clone(),
                config.rag.semantic_search.clone(),
            )
            .await?,
        );

        // Initialize memory cache
        let memory_cache_config = SemanticCacheConfig {
            max_size_mb: config.cache.storage.memory.max_size_mb,
            eviction_policy: config.cache.storage.memory.eviction_policy.clone(),
            enable_semantic_indexing: true,
            semantic_similarity_threshold: 0.7,
            max_entries: 10000,
        };
        let memory_cache = Arc::new(SemanticMemoryCache::new(memory_cache_config));

        // Initialize disk cache
        let disk_cache_config = SemanticDiskConfig {
            cache_dir: config.cache.storage.disk.cache_dir.clone(),
            max_size_gb: config.cache.storage.disk.max_size_gb,
            compression_enabled: config.cache.storage.disk.compression_enabled,
            compression_algorithm: config.cache.storage.disk.compression_algorithm.clone(),
            compression_threshold_kb: config.cache.performance.compression_threshold_kb,
            enable_vector_storage: true,
            vector_dimension: config.rag.vector_store.dimension,
            distance_metric: config.rag.vector_store.distance_metric.clone(),
        };
        let disk_cache = Arc::new(SemanticDiskCache::new(disk_cache_config).await?);

        // Initialize network cache if enabled
        let network_cache = if config.cache.storage.network.enabled {
            let network_config = DistributedRAGConfig {
                redis_url: config
                    .cache
                    .storage
                    .network
                    .redis_url
                    .clone()
                    .unwrap_or_default(),
                connection_pool_size: config.cache.storage.network.connection_pool_size,
                enable_replication: false,
                replication_factor: 1,
                auto_failover: false,
            };
            Some(Arc::new(DistributedRAGCache::new(network_config).await?))
        } else {
            None
        };

        // Initialize knowledge synthesizer
        let knowledge_synthesizer = Arc::new(
            KnowledgeSynthesizer::new(embedding_manager.clone(), vector_store.clone()).await?,
        );

        // Initialize proactive manager
        let proactive_manager = if config.proactive.enabled {
            Some(Arc::new(ProactiveContextManager::new(Arc::new(Self {
                embedding_manager: embedding_manager.clone(),
                vector_store: vector_store.clone(),
                semantic_search: semantic_search.clone(),
                memory_cache: memory_cache.clone(),
                disk_cache: disk_cache.clone(),
                network_cache: network_cache.clone(),
                knowledge_synthesizer: knowledge_synthesizer.clone(),
                proactive_manager: None, // This will be set by the ProactiveContextManager constructor
                agent_sessions: Arc::new(RwLock::new(HashMap::new())),
                config: config.clone(),
                metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
            }))))
        } else {
            None
        };

        // Initialize agent sessions
        let agent_sessions = Arc::new(RwLock::new(HashMap::new()));

        // Initialize metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));

        let engine = Self {
            embedding_manager,
            vector_store,
            semantic_search,
            memory_cache,
            disk_cache,
            network_cache,
            knowledge_synthesizer,
            proactive_manager,
            agent_sessions,
            config,
            metrics,
        };

        info!("Unified knowledge engine initialized successfully");
        Ok(engine)
    }

    /// Get data with RAG-enhanced caching
    pub async fn get_with_rag(
        &self,
        key: &str,
        query: Option<&str>,
    ) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        let start_time = Instant::now();

        // Try direct cache lookup first
        if let Some(result) = self.get_direct(key).await? {
            self.update_metrics_cache_hit(start_time).await;
            return Ok(Some(result));
        }

        // If query provided, try semantic search
        if let Some(query) = query {
            let semantic_results = self.semantic_search.search_semantic(query, 5).await?;

            // Check if any semantic results match the key or are highly relevant
            for result in semantic_results {
                if result.cache_key == key || result.relevance_score > 0.8 {
                    // Promote to cache and return
                    let cached_result = self.promote_to_cache(&result).await?;
                    self.update_metrics_semantic_hit(start_time).await;
                    return Ok(Some(cached_result));
                }
            }
        }

        self.update_metrics_cache_miss(start_time).await;
        Ok(None)
    }

    /// Set data with semantic indexing
    pub async fn set_with_semantic_indexing(
        &self,
        key: &str,
        data: &[u8],
        metadata: &Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        let start_time = Instant::now();

        // Store in cache
        self.set_direct(key, data, metadata.clone()).await?;

        // Generate semantic embedding and index
        if let Some(content) = self.extract_content(data).await {
            let embedding = self.embedding_manager.embed(&content, None).await?;

            // Store in vector store
            self.vector_store
                .store_with_metadata(key, &embedding, &content, metadata.clone())
                .await?;

            // Update semantic cache
            let semantic_entry = self
                .create_semantic_cache_entry(key, data, &content, &embedding, metadata.clone())
                .await?;
            self.memory_cache
                .set(key.to_string(), semantic_entry)
                .await?;
        }

        self.update_metrics_set(start_time).await;
        Ok(())
    }

    /// Semantic search across all cached and indexed content
    pub async fn search_semantic(
        &self,
        query: &str,
        limit: usize,
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let start_time = Instant::now();

        // Search in vector store
        let query_embedding = self.embedding_manager.embed(query, None).await?;
        let vector_results = self.vector_store.search(&query_embedding, limit).await?;

        // Enhance with cache information
        let enhanced_results = self.enhance_with_cache_info(&vector_results).await?;

        self.update_metrics_search(start_time).await;
        Ok(enhanced_results)
    }

    /// Warm cache for agent session
    pub async fn warm_cache_for_agent_session(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<()> {
        info!("Warming cache for agent session: {}", agent_id);

        // Analyze session context to predict needed data
        let predicted_keys = self
            .predict_agent_context_needs(agent_id, session_context)
            .await?;

        // Pre-load predicted context into agent-specific cache
        for key in predicted_keys {
            self.prewarm_agent_context(agent_id, &key).await?;
        }

        Ok(())
    }

    /// Get agent-specific context
    pub async fn get_agent_context(
        &self,
        agent_id: &str,
        key: &str,
    ) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        // Try agent-specific cache first
        let agent_key = format!("agent:{}:{}", agent_id, key);
        if let Some(result) = self.get_direct(&agent_key).await? {
            return Ok(Some(result));
        }

        // Fall back to global cache
        self.get_direct(key).await
    }

    /// Set agent-specific context
    pub async fn set_agent_context(
        &self,
        agent_id: &str,
        key: &str,
        data: &[u8],
    ) -> KnowledgeResult<()> {
        let agent_key = format!("agent:{}:{}", agent_id, key);
        self.set_direct(&agent_key, data, None).await
    }

    /// Share context between agents
    pub async fn share_context_across_agents(
        &self,
        source_agent_id: &str,
        target_agent_id: &str,
        context_key: &str,
    ) -> KnowledgeResult<()> {
        if let Some(cached_context) = self.get_agent_context(source_agent_id, context_key).await? {
            self.set_agent_context(target_agent_id, context_key, &cached_context.data)
                .await?;
        }
        Ok(())
    }

    /// Synthesize knowledge on a topic
    pub async fn synthesize_knowledge(
        &self,
        topic: &str,
        scope_path: Option<&str>,
    ) -> KnowledgeResult<crate::types::KnowledgeSynthesis> {
        self.knowledge_synthesizer
            .synthesize(topic, scope_path)
            .await
    }

    /// Get unified metrics
    pub async fn get_metrics(&self) -> UnifiedMetrics {
        self.metrics.read().await.clone()
    }

    // Private helper methods

    async fn get_direct(&self, key: &str) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        // Try memory cache first
        if let Some(result) = self.memory_cache.get(key).await? {
            return Ok(Some(result));
        }

        // Try disk cache
        if let Some(result) = self.disk_cache.get(key).await? {
            // Promote to memory cache
            if let Some(semantic_entry) = self.create_semantic_entry_from_result(&result).await? {
                self.memory_cache
                    .set(key.to_string(), semantic_entry)
                    .await?;
            }
            return Ok(Some(result));
        }

        // Try network cache if available
        if let Some(network_cache) = &self.network_cache {
            if let Some(result) = network_cache.get_with_rag(key, None).await? {
                // Update metrics for network cache hit
                self.update_metrics_cache_hit(Instant::now()).await;
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    async fn set_direct(
        &self,
        key: &str,
        data: &[u8],
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        // Store in memory cache first
        let semantic_entry = self
            .create_semantic_cache_entry(key, data, "", &[], metadata.clone())
            .await?;
        self.memory_cache
            .set(key.to_string(), semantic_entry)
            .await?;

        // Store in disk cache
        let disk_entry = self
            .create_semantic_cache_entry(key, data, "", &[], metadata)
            .await?;
        self.disk_cache.set(key.to_string(), disk_entry).await?;

        Ok(())
    }

    async fn promote_to_cache(
        &self,
        result: &SemanticResult,
    ) -> KnowledgeResult<UnifiedCacheResult> {
        // Create cache entry from search result
        let cache_entry = self.create_cache_entry_from_search_result(result).await?;

        // Store in cache
        self.set_direct(
            &result.cache_key,
            &cache_entry.data,
            Some(cache_entry.metadata.clone()),
        )
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

    async fn enhance_with_cache_info(
        &self,
        vector_results: &[crate::vector::VectorSearchResult],
    ) -> KnowledgeResult<Vec<SemanticResult>> {
        let mut enhanced_results = Vec::new();

        for result in vector_results {
            let cache_info = if let Some(cached) = self.get_direct(&result.id).await? {
                Some(crate::types::CacheInfo {
                    is_cached: true,
                    cache_tier: cached.cache_tier,
                    access_count: cached.metadata.access_count,
                    last_accessed: cached.metadata.accessed_at,
                    ttl_remaining: cached.metadata.ttl,
                })
            } else {
                None
            };

            enhanced_results.push(SemanticResult {
                cache_key: result.id.clone(),
                content: result.content.clone().unwrap_or_default(),
                embedding: result.embedding.clone(),
                relevance_score: result.score,
                semantic_tags: vec![], // TODO: Extract from metadata
                metadata: result.metadata.clone().unwrap_or_default(),
                cache_info,
            });
        }

        Ok(enhanced_results)
    }

    async fn extract_content(&self, data: &[u8]) -> Option<String> {
        // Try to extract text content from binary data
        String::from_utf8(data.to_vec()).ok()
    }

    async fn create_semantic_cache_entry(
        &self,
        key: &str,
        data: &[u8],
        content: &str,
        embedding: &[f32],
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<crate::types::SemanticCacheEntry> {
        let now = chrono::Utc::now();
        let metadata = metadata.unwrap_or_else(|| CacheEntryMetadata {
            key: key.to_string(),
            created_at: now,
            accessed_at: now,
            access_count: 0,
            size_bytes: data.len() as u64,
            ttl: Duration::from_secs(3600), // 1 hour default
            compression_ratio: None,
            semantic_tags: vec![],
            agent_session_id: None,
            scope_path: None,
            checksum: None,
        });

        Ok(crate::types::SemanticCacheEntry {
            data: data.to_vec(),
            embedding: Some(embedding.to_vec()),
            semantic_tags: metadata.semantic_tags.clone(),
            access_patterns: crate::types::AccessPatterns {
                frequency: 1.0,
                recency: 1.0,
                semantic_relevance: 1.0,
                temporal_pattern: crate::types::TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
            },
            metadata,
        })
    }

    async fn create_semantic_entry_from_result(
        &self,
        result: &UnifiedCacheResult,
    ) -> KnowledgeResult<Option<crate::types::SemanticCacheEntry>> {
        if let Some(semantic_info) = &result.semantic_info {
            Ok(Some(crate::types::SemanticCacheEntry {
                data: result.data.clone(),
                embedding: semantic_info.embedding.clone(),
                semantic_tags: semantic_info.semantic_tags.clone(),
                access_patterns: result.access_patterns.clone(),
                metadata: result.metadata.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn create_cache_entry_from_search_result(
        &self,
        result: &SemanticResult,
    ) -> KnowledgeResult<UnifiedCacheResult> {
        // Convert search result to cache entry
        let content_bytes = result.content.as_bytes().to_vec();

        Ok(UnifiedCacheResult {
            data: content_bytes,
            metadata: CacheEntryMetadata {
                key: result.cache_key.clone(),
                created_at: result.metadata.created_at,
                accessed_at: chrono::Utc::now(),
                access_count: 1,
                size_bytes: 0, // Temporarily hardcoded to avoid type inference issues
                ttl: Duration::from_secs(3600),
                compression_ratio: None,
                semantic_tags: result.semantic_tags.clone(),
                agent_session_id: None,
                scope_path: result.metadata.scope_path.clone(),
                checksum: None,
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

    async fn predict_agent_context_needs(
        &self,
        agent_id: &str,
        session_context: &AgentSessionContext,
    ) -> KnowledgeResult<Vec<String>> {
        // Use the usage analyzer to predict context needs
        if let Some(proactive_manager) = &self.proactive_manager {
            let predicted_needs = proactive_manager
                .usage_analyzer
                .predict_session_needs(agent_id, session_context)
                .await?;
            Ok(predicted_needs.into_iter().map(|need| need.key).collect())
        } else {
            // Fallback to basic prediction based on session context
            let mut predicted_keys = Vec::new();

            // Add workflow-specific context
            if let Some(workflow) = &session_context.workflow_context {
                predicted_keys.push(format!("workflow:{}:context", workflow.workflow_id));
                predicted_keys.push(format!("workflow:{}:requirements", workflow.workflow_id));
            }

            // Add agent-specific context
            predicted_keys.push(format!("agent:{}:preferences", agent_id));
            predicted_keys.push(format!("agent:{}:history", agent_id));

            Ok(predicted_keys)
        }
    }

    pub async fn prewarm_agent_context(&self, agent_id: &str, key: &str) -> KnowledgeResult<()> {
        // Try to prewarm the context by checking if it exists in any cache tier
        let agent_key = format!("agent:{}:{}", agent_id, key);

        // Check memory cache first
        if let Some(_) = self.memory_cache.get(&agent_key).await? {
            return Ok(());
        }

        // Check disk cache
        if let Some(_) = self.disk_cache.get(&agent_key).await? {
            return Ok(());
        }

        // Check network cache if available
        if let Some(network_cache) = &self.network_cache {
            if let Some(_) = network_cache.get_with_rag(&agent_key, None).await? {
                return Ok(());
            }
        }

        // If not found, create a placeholder entry for future use
        let placeholder_data =
            format!("Prewarmed context for agent {}: {}", agent_id, key).into_bytes();
        let metadata = CacheEntryMetadata {
            key: agent_key.clone(),
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            access_count: 0,
            size_bytes: placeholder_data.len() as u64,
            ttl: std::time::Duration::from_secs(3600), // 1 hour
            compression_ratio: None,
            semantic_tags: vec!["prewarmed".to_string(), "agent-context".to_string()],
            agent_session_id: Some(agent_id.to_string()),
            scope_path: None,
            checksum: None,
        };

        self.set_direct(&agent_key, &placeholder_data, Some(metadata))
            .await?;

        info!("Prewarmed context for agent {}: {}", agent_id, key);
        Ok(())
    }

    async fn update_metrics_cache_hit(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.hit_count += 1;
        metrics.performance_metrics.average_cache_access_time_ms =
            start_time.elapsed().as_millis() as u64;
    }

    async fn update_metrics_cache_miss(&self, start_time: Instant) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_metrics.miss_count += 1;
        metrics.performance_metrics.average_cache_access_time_ms =
            start_time.elapsed().as_millis() as u64;
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

impl DistributedRAGCache {
    pub async fn new(config: DistributedRAGConfig) -> KnowledgeResult<Self> {
        let redis_client = Arc::new(redis::Client::open(config.redis_url.clone())?);

        let connection_pool = Arc::new(ConnectionPool {
            pool: Arc::new(
                redis::aio::ConnectionManager::new(redis_client.as_ref().clone()).await?,
            ),
        });

        let distributed_vector_store = Arc::new(crate::vector::MockVectorStore::new(
            "distributed_mock_collection".to_string(),
            1536,
            DistanceMetric::Cosine,
        ));

        Ok(Self {
            redis_client,
            distributed_vector_store,
            config,
            connection_pool,
        })
    }

    pub async fn get_with_rag(
        &self,
        key: &str,
        query: Option<&str>,
    ) -> KnowledgeResult<Option<UnifiedCacheResult>> {
        // Simplified implementation without Redis for now
        if let Some(query) = query {
            let embedding = self.generate_embedding(query).await?;
            let vector_results = self.distributed_vector_store.search(&embedding, 5).await?;

            if let Some(result) = vector_results.first() {
                let cache_result = self.convert_to_cache_result(result).await?;
                return Ok(Some(cache_result));
            }
        }

        Ok(None)
    }

    pub async fn set_with_semantic_indexing(
        &self,
        key: &str,
        data: &[u8],
        metadata: Option<CacheEntryMetadata>,
    ) -> KnowledgeResult<()> {
        let result = UnifiedCacheResult {
            data: data.to_vec(),
            metadata: metadata.unwrap_or_else(|| CacheEntryMetadata {
                key: key.to_string(),
                created_at: chrono::Utc::now(),
                accessed_at: chrono::Utc::now(),
                access_count: 0,
                size_bytes: data.len() as u64,
                ttl: std::time::Duration::from_secs(3600),
                compression_ratio: None,
                semantic_tags: vec![],
                agent_session_id: None,
                scope_path: None,
                checksum: None,
            }),
            semantic_info: None,
            cache_tier: CacheTier::Network,
            access_patterns: AccessPatterns {
                frequency: 0.0,
                recency: 0.0,
                semantic_relevance: 0.0,
                temporal_pattern: TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
            },
        };

        self.store_in_redis(key, &result).await?;

        // Also store in vector store if we have content
        if let Some(content) = self.extract_content(data) {
            let embedding = self.generate_embedding(&content).await?;
            let metadata = SearchResultMetadata {
                source_type: ContentType::Unknown,
                scope_path: None,
                created_at: chrono::Utc::now(),
                last_modified: chrono::Utc::now(),
                size_bytes: data.len() as u64,
                chunk_id: None,
            };
            self.distributed_vector_store
                .store(key, &embedding, Some(metadata))
                .await?;
        }

        Ok(())
    }

    async fn generate_embedding(&self, text: &str) -> KnowledgeResult<Vec<f32>> {
        // Simple hash-based embedding for testing
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut embedding = vec![0.0; 384]; // Default dimension
        for i in 0..384 {
            embedding[i] = ((hash >> (i * 8)) & 0xFF) as f32 / 255.0 * 2.0 - 1.0;
        }
        Ok(embedding)
    }

    fn extract_content(&self, data: &[u8]) -> Option<String> {
        String::from_utf8(data.to_vec()).ok()
    }

    // Redis functionality temporarily disabled
    async fn store_in_redis(
        &self,
        _key: &str,
        _result: &UnifiedCacheResult,
    ) -> KnowledgeResult<()> {
        Ok(())
    }

    async fn convert_to_cache_result(
        &self,
        result: &crate::vector::VectorSearchResult,
    ) -> KnowledgeResult<UnifiedCacheResult> {
        Ok(UnifiedCacheResult {
            data: result
                .content
                .as_ref()
                .map(|c| c.as_bytes().to_vec())
                .unwrap_or_default(),
            metadata: CacheEntryMetadata {
                key: result.id.clone(),
                created_at: chrono::Utc::now(),
                accessed_at: chrono::Utc::now(),
                access_count: 0,
                size_bytes: result.content.as_ref().map(|c| c.len() as u64).unwrap_or(0),
                ttl: std::time::Duration::from_secs(3600),
                compression_ratio: None,
                semantic_tags: vec![],
                agent_session_id: None,
                scope_path: None,
                checksum: None,
            },
            semantic_info: None,
            cache_tier: CacheTier::Network,
            access_patterns: AccessPatterns {
                frequency: 0.0,
                recency: 0.0,
                semantic_relevance: result.score,
                temporal_pattern: TemporalPattern::Recent,
                agent_affinity: HashMap::new(),
                workflow_affinity: HashMap::new(),
            },
        })
    }
}

impl ProactiveContextManager {
    pub fn new(unified_engine: Arc<UnifiedKnowledgeEngine>) -> Self {
        Self {
            unified_engine: Some(unified_engine.clone()),
            file_watcher: Arc::new(FileWatcher::new(unified_engine, FileWatchConfig::default())),
            usage_analyzer: Arc::new(UsageAnalyzer::new(UsageAnalysisConfig::default())),
            suggestion_engine: Arc::new(SuggestionEngine {
                suggestions: Arc::new(RwLock::new(HashMap::new())),
                suggestion_history: Arc::new(RwLock::new(Vec::new())),
                config: SuggestionEngineConfig::default(),
            }),
        }
    }

    /// Dummy without engine to break circular dependency
    pub fn new_dummy_without_engine() -> Self {
        Self {
            unified_engine: None,
            file_watcher: Arc::new(FileWatcher::new_dummy_without_engine()),
            usage_analyzer: Arc::new(UsageAnalyzer::new_dummy_without_engine()),
            suggestion_engine: Arc::new(SuggestionEngine {
                suggestions: Arc::new(RwLock::new(HashMap::new())),
                suggestion_history: Arc::new(RwLock::new(Vec::new())),
                config: SuggestionEngineConfig::default(),
            }),
        }
    }

    pub fn new_dummy_minimal() -> Self {
        // Create a minimal dummy implementation without circular references
        Self {
            unified_engine: None,
            file_watcher: Arc::new(FileWatcher::new_dummy_minimal()),
            usage_analyzer: Arc::new(UsageAnalyzer::new_dummy_minimal()),
            suggestion_engine: Arc::new(SuggestionEngine {
                suggestions: Arc::new(RwLock::new(HashMap::new())),
                suggestion_history: Arc::new(RwLock::new(Vec::new())),
                config: SuggestionEngineConfig::default(),
            }),
        }
    }

    pub fn new_dummy_simple() -> Self {
        // Create a simple dummy implementation without circular references
        Self {
            unified_engine: None,
            file_watcher: Arc::new(FileWatcher::new_dummy_simple()),
            usage_analyzer: Arc::new(UsageAnalyzer::new_dummy_simple()),
            suggestion_engine: Arc::new(SuggestionEngine {
                suggestions: Arc::new(RwLock::new(HashMap::new())),
                suggestion_history: Arc::new(RwLock::new(Vec::new())),
                config: SuggestionEngineConfig::default(),
            }),
        }
    }

    pub fn new_dummy_null() -> Self {
        // Create a dummy implementation with a null engine to break circular dependency
        Self {
            unified_engine: None,
            file_watcher: Arc::new(FileWatcher::new_dummy_null()),
            usage_analyzer: Arc::new(UsageAnalyzer::new_dummy_null()),
            suggestion_engine: Arc::new(SuggestionEngine {
                suggestions: Arc::new(RwLock::new(HashMap::new())),
                suggestion_history: Arc::new(RwLock::new(Vec::new())),
                config: SuggestionEngineConfig::default(),
            }),
        }
    }
}

impl FileWatcher {
    pub fn new_dummy_without_engine() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            unified_engine: Arc::new(UnifiedKnowledgeEngine::new_dummy()),
            config: FileWatchConfig::default(),
        }
    }

    pub fn new_dummy_minimal() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            unified_engine: Arc::new(UnifiedKnowledgeEngine::new_dummy_minimal()),
            config: FileWatchConfig::default(),
        }
    }

    pub fn new_dummy_simple() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            unified_engine: Arc::new(UnifiedKnowledgeEngine::new_dummy_simple()),
            config: FileWatchConfig::default(),
        }
    }

    pub fn new_dummy_null() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            unified_engine: Arc::new(UnifiedKnowledgeEngine::new_dummy()),
            config: FileWatchConfig::default(),
        }
    }
}

impl UsageAnalyzer {
    pub fn new_dummy_without_engine() -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            workflow_patterns: Arc::new(RwLock::new(HashMap::new())),
            config: UsageAnalysisConfig::default(),
        }
    }

    pub fn new_dummy_minimal() -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            workflow_patterns: Arc::new(RwLock::new(HashMap::new())),
            config: UsageAnalysisConfig::default(),
        }
    }

    pub fn new_dummy_simple() -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            workflow_patterns: Arc::new(RwLock::new(HashMap::new())),
            config: UsageAnalysisConfig::default(),
        }
    }

    pub fn new_dummy_null() -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            workflow_patterns: Arc::new(RwLock::new(HashMap::new())),
            config: UsageAnalysisConfig::default(),
        }
    }
}

impl UnifiedKnowledgeEngine {
    pub fn empty_dummy() -> Self {
        // Minimal dummy with no proactive_manager
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "mock_collection".to_string(),
                1536,
                DistanceMetric::Cosine,
            )),
            semantic_search: Arc::new(SemanticSearchEngine::new_dummy()),
            memory_cache: Arc::new(SemanticMemoryCache::new_dummy()),
            disk_cache: Arc::new(SemanticDiskCache::new_dummy()),
            network_cache: None,
            knowledge_synthesizer: Arc::new(KnowledgeSynthesizer::new_dummy()),
            proactive_manager: None,
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config: UnifiedEngineConfig::default(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
        }
    }
    pub fn new_dummy() -> Self {
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "mock_collection".to_string(),
                1536,
                DistanceMetric::Cosine,
            )),
            semantic_search: Arc::new(SemanticSearchEngine::new_dummy()),
            memory_cache: Arc::new(SemanticMemoryCache::new_dummy()),
            disk_cache: Arc::new(SemanticDiskCache::new_dummy()),
            network_cache: None,
            knowledge_synthesizer: Arc::new(KnowledgeSynthesizer::new_dummy()),
            proactive_manager: None,
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config: UnifiedEngineConfig::default(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
        }
    }

    pub fn new_dummy_without_proactive() -> Self {
        // This is a temporary implementation for circular dependency resolution
        // It will be replaced by the actual implementation
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "mock_collection".to_string(),
                1536,
                DistanceMetric::Cosine,
            )),
            semantic_search: Arc::new(SemanticSearchEngine::new_dummy()),
            memory_cache: Arc::new(SemanticMemoryCache::new_dummy()),
            disk_cache: Arc::new(SemanticDiskCache::new_dummy()),
            network_cache: None,
            knowledge_synthesizer: Arc::new(KnowledgeSynthesizer::new_dummy()),
            proactive_manager: None,
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config: UnifiedEngineConfig::default(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
        }
    }

    pub fn new_dummy_minimal() -> Self {
        // This is a minimal dummy implementation without circular references
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "mock_collection".to_string(),
                1536,
                DistanceMetric::Cosine,
            )),
            semantic_search: Arc::new(SemanticSearchEngine::new_dummy()),
            memory_cache: Arc::new(SemanticMemoryCache::new_dummy()),
            disk_cache: Arc::new(SemanticDiskCache::new_dummy()),
            network_cache: None,
            knowledge_synthesizer: Arc::new(KnowledgeSynthesizer::new_dummy()),
            proactive_manager: None,
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config: UnifiedEngineConfig::default(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
        }
    }

    pub fn new_dummy_simple() -> Self {
        // This is a simple dummy implementation without circular references
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "mock_collection".to_string(),
                1536,
                DistanceMetric::Cosine,
            )),
            semantic_search: Arc::new(SemanticSearchEngine::new_dummy()),
            memory_cache: Arc::new(SemanticMemoryCache::new_dummy()),
            disk_cache: Arc::new(SemanticDiskCache::new_dummy()),
            network_cache: None,
            knowledge_synthesizer: Arc::new(KnowledgeSynthesizer::new_dummy()),
            proactive_manager: None,
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config: UnifiedEngineConfig::default(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics::default())),
        }
    }
}

// Default implementations for metrics
impl Default for UnifiedMetrics {
    fn default() -> Self {
        Self {
            cache_metrics: crate::types::CacheMetrics {
                total_entries: 0,
                hit_count: 0,
                miss_count: 0,
                hit_rate: 0.0,
                memory_usage_bytes: 0,
                disk_usage_bytes: 0,
                network_usage_bytes: 0,
                eviction_count: 0,
                semantic_hit_rate: 0.0,
            },
            search_metrics: crate::types::SearchMetrics {
                total_searches: 0,
                semantic_searches: 0,
                hybrid_searches: 0,
                average_relevance_score: 0.0,
                average_response_time_ms: 0,
                cache_enhanced_searches: 0,
            },
            synthesis_metrics: crate::types::SynthesisMetrics {
                total_syntheses: 0,
                average_confidence_score: 0.0,
                cross_scope_syntheses: 0,
                synthesis_time_ms: 0,
            },
            proactive_metrics: crate::types::ProactiveMetrics {
                suggestions_generated: 0,
                suggestions_accepted: 0,
                cache_warming_events: 0,
                file_analysis_count: 0,
            },
            performance_metrics: crate::types::PerformanceMetrics {
                average_cache_access_time_ms: 0,
                average_search_time_ms: 0,
                average_embedding_time_ms: 0,
                compression_ratio: 0.0,
                memory_pressure_percent: 0.0,
            },
        }
    }
}
