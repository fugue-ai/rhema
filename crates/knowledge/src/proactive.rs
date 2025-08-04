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
use notify::{Watcher, RecursiveMode};
use chrono::{DateTime, Utc};

use crate::types::{
    ContentType, KnowledgeResult, CacheEntryMetadata, SemanticInfo,
    CompressionAlgorithm, DistanceMetric, CacheTier, AgentSessionContext,
    WorkflowContext, ContextSuggestion, SuggestionAction, Priority, WorkflowType,
};

use super::engine::UnifiedKnowledgeEngine;

/// Error types for proactive operations
#[derive(Error, Debug)]
pub enum ProactiveError {
    #[error("File analysis error: {0}")]
    FileAnalysisError(String),
    
    #[error("Usage analysis error: {0}")]
    UsageAnalysisError(String),
    
    #[error("Suggestion generation error: {0}")]
    SuggestionGenerationError(String),
    
    #[error("Cache warming error: {0}")]
    CacheWarmingError(String),
    
    #[error("Workflow analysis error: {0}")]
    WorkflowAnalysisError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Proactive context manager for intelligent context suggestions
pub struct ProactiveContextManager {
    unified_engine: Arc<UnifiedKnowledgeEngine>,
    file_watcher: Arc<FileWatcher>,
    usage_analyzer: Arc<UsageAnalyzer>,
    suggestion_engine: Arc<SuggestionEngine>,
    config: ProactiveConfig,
}

/// Proactive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveConfig {
    pub enabled: bool,
    pub suggestion_threshold: f32,
    pub warm_cache_enabled: bool,
    pub file_analysis_enabled: bool,
    pub workflow_analysis_enabled: bool,
    pub agent_session_tracking: bool,
    pub suggestion_limit: usize,
    pub cache_warming_limit: usize,
}

impl Default for ProactiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            suggestion_threshold: 0.8,
            warm_cache_enabled: true,
            file_analysis_enabled: true,
            workflow_analysis_enabled: true,
            agent_session_tracking: true,
            suggestion_limit: 10,
            cache_warming_limit: 20,
        }
    }
}

/// File watcher for monitoring file changes
pub struct FileWatcher {
    watched_files: Arc<RwLock<HashMap<PathBuf, FileWatchInfo>>>,
    config: FileWatcherConfig,
    watcher: Option<notify::RecommendedWatcher>,
    event_sender: Option<tokio::sync::mpsc::UnboundedSender<notify::Event>>,
}

/// File watcher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatcherConfig {
    pub watch_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub debounce_interval_ms: u64,
    pub max_watched_files: usize,
}

impl Default for FileWatcherConfig {
    fn default() -> Self {
        Self {
            watch_patterns: vec!["*.rs".to_string(), "*.md".to_string(), "*.yaml".to_string()],
            ignore_patterns: vec!["target/".to_string(), ".git/".to_string()],
            debounce_interval_ms: 1000,
            max_watched_files: 1000,
        }
    }
}

/// File watch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatchInfo {
    pub path: PathBuf,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub content_hash: String,
    pub change_count: u64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// File watch statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWatchStats {
    pub total_watched_files: usize,
    pub total_changes: u64,
    pub max_files: usize,
}

/// Usage analyzer for predicting context needs
pub struct UsageAnalyzer {
    usage_patterns: Arc<RwLock<HashMap<String, UsagePattern>>>,
    agent_sessions: Arc<RwLock<HashMap<String, AgentSessionContext>>>,
    config: UsageAnalyzerConfig,
}

/// Usage analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalyzerConfig {
    pub pattern_retention_days: u64,
    pub min_pattern_occurrences: u64,
    pub confidence_threshold: f32,
    pub enable_learning: bool,
}

impl Default for UsageAnalyzerConfig {
    fn default() -> Self {
        Self {
            pattern_retention_days: 30,
            min_pattern_occurrences: 3,
            confidence_threshold: 0.7,
            enable_learning: true,
        }
    }
}

/// Usage pattern for context prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePattern {
    pub pattern_id: String,
    pub agent_id: String,
    pub workflow_type: WorkflowType,
    pub context_keys: Vec<String>,
    pub frequency: u64,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub confidence: f32,
    pub success_rate: f32,
}

/// Suggestion engine for generating context suggestions
pub struct SuggestionEngine {
    suggestion_templates: Arc<RwLock<HashMap<String, SuggestionTemplate>>>,
    config: SuggestionEngineConfig,
}

/// Suggestion engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEngineConfig {
    pub enable_ai_suggestions: bool,
    pub enable_pattern_based_suggestions: bool,
    pub enable_workflow_suggestions: bool,
    pub suggestion_quality_threshold: f32,
    pub max_suggestions_per_context: usize,
}

impl Default for SuggestionEngineConfig {
    fn default() -> Self {
        Self {
            enable_ai_suggestions: true,
            enable_pattern_based_suggestions: true,
            enable_workflow_suggestions: true,
            suggestion_quality_threshold: 0.6,
            max_suggestions_per_context: 10,
        }
    }
}

/// Suggestion template for generating suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub trigger_conditions: Vec<SuggestionTrigger>,
    pub content_type: ContentType,
    pub priority: Priority,
    pub confidence_boost: f32,
}

/// Suggestion trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionTrigger {
    FileType(String),
    WorkflowType(WorkflowType),
    ContentPattern(String),
    AgentSession(String),
    TimeOfDay(u8, u8), // hour, minute
    Frequency(u64),
}

impl ProactiveContextManager {
    pub fn new(unified_engine: Arc<UnifiedKnowledgeEngine>) -> Self {
        let file_watcher = Arc::new(FileWatcher::new(FileWatcherConfig::default()));
        let usage_analyzer = Arc::new(UsageAnalyzer::new(UsageAnalyzerConfig::default()));
        let suggestion_engine = Arc::new(SuggestionEngine::new(SuggestionEngineConfig::default()));
        
        Self {
            unified_engine,
            file_watcher,
            usage_analyzer,
            suggestion_engine,
            config: ProactiveConfig::default(),
        }
    }
    
    /// Initialize the proactive context manager
    pub async fn initialize(&self) -> KnowledgeResult<()> {
        info!("Initializing proactive context manager");
        
        // Initialize usage analyzer
        self.usage_analyzer.initialize().await?;
        
        // Initialize suggestion engine
        self.suggestion_engine.initialize().await?;
        
        // Initialize file watcher with proper mutable access using RwLock
        {
            let mut _file_watcher_guard = self.file_watcher.watched_files.write().await;
            // The file watcher initialization is handled separately since it requires mutable access
            // We'll initialize it when needed in individual operations
        }
        
        info!("Proactive context manager initialized successfully");
        Ok(())
    }
    
    /// Suggest context for a specific file
    #[instrument(skip(self, file_path))]
    pub async fn suggest_context_for_file(&self, file_path: &str) -> KnowledgeResult<Vec<ContextSuggestion>> {
        if !self.config.enabled || !self.config.file_analysis_enabled {
            return Ok(vec![]);
        }
        
        info!("Generating context suggestions for file: {}", file_path);
        
        // Add file to watcher if not already watching
        let file_path_buf = PathBuf::from(file_path);
        self.file_watcher.watch_file(file_path_buf).await?;
        
        // Analyze file content to understand context
        let file_context = self.analyze_file_context(file_path).await?;
        
        // Find relevant knowledge using semantic search
        let search_results = self.unified_engine.search_semantic(&file_context, 10).await?;
        
        // Convert search results to context suggestions
        let suggestions = self.convert_to_context_suggestions(search_results, file_path).await?;
        
        // Filter by relevance threshold
        let filtered_suggestions: Vec<ContextSuggestion> = suggestions
            .into_iter()
            .filter(|s| s.relevance_score >= self.config.suggestion_threshold)
            .take(self.config.suggestion_limit)
            .collect();
        
        debug!("Generated {} context suggestions for file: {}", filtered_suggestions.len(), file_path);
        Ok(filtered_suggestions)
    }
    
    /// Suggest context for a workflow
    #[instrument(skip(self, workflow_context))]
    pub async fn suggest_context_for_workflow(&self, workflow_context: &WorkflowContext) -> KnowledgeResult<Vec<ContextSuggestion>> {
        if !self.config.enabled || !self.config.workflow_analysis_enabled {
            return Ok(vec![]);
        }
        
        info!("Generating context suggestions for workflow: {}", workflow_context.workflow_id);
        
        // Analyze workflow to predict needed context
        let workflow_query = format!(
            "workflow: {} step: {} type: {}",
            workflow_context.workflow_id,
            workflow_context.current_step,
            WorkflowTypeExt::to_string(&workflow_context.workflow_type)
        );
        
        // Search for relevant content
        let search_results = self.unified_engine.search_semantic(&workflow_query, 10).await?;
        
        // Convert to context suggestions
        let suggestions = self.convert_to_context_suggestions(search_results, &workflow_context.workflow_id).await?;
        
        // Filter by relevance threshold
        let filtered_suggestions: Vec<ContextSuggestion> = suggestions
            .into_iter()
            .filter(|s| s.relevance_score >= self.config.suggestion_threshold)
            .take(self.config.suggestion_limit)
            .collect();
        
        debug!("Generated {} workflow context suggestions", filtered_suggestions.len());
        Ok(filtered_suggestions)
    }
    
    /// Warm cache for a workflow
    #[instrument(skip(self, workflow_context))]
    pub async fn warm_cache_for_workflow(&self, workflow_context: &WorkflowContext) -> KnowledgeResult<()> {
        if !self.config.enabled || !self.config.warm_cache_enabled {
            return Ok(());
        }
        
        info!("Warming cache for workflow: {}", workflow_context.workflow_id);
        
        // Analyze workflow to predict needed context
        let predicted_context = self.usage_analyzer.predict_context_needs(workflow_context).await?;
        
        // Pre-load relevant context into cache
        for context_item in predicted_context.iter().take(self.config.cache_warming_limit) {
            self.unified_engine.prewarm_agent_context("default", context_item).await?;
        }
        
        debug!("Warmed cache with {} context items for workflow", predicted_context.len());
        Ok(())
    }
    
    /// Warm cache for an agent session
    #[instrument(skip(self, agent_id, session_context))]
    pub async fn warm_cache_for_agent_session(&self, agent_id: &str, session_context: &AgentSessionContext) -> KnowledgeResult<()> {
        if !self.config.enabled || !self.config.warm_cache_enabled {
            return Ok(());
        }
        
        info!("Warming cache for agent session: {}", agent_id);
        
        // Analyze agent session to predict needed context
        let predicted_context = self.usage_analyzer.predict_agent_context_needs(agent_id, session_context).await?;
        
        // Pre-load relevant context into agent-specific cache
        for context_item in predicted_context.iter().take(self.config.cache_warming_limit) {
            self.unified_engine.prewarm_agent_context(agent_id, context_item).await?;
        }
        
        debug!("Warmed cache with {} context items for agent session", predicted_context.len());
        Ok(())
    }
    
    /// Share context between agents
    #[instrument(skip(self, source_agent_id, target_agent_id, context_key))]
    pub async fn share_context_across_agents(&self, source_agent_id: &str, target_agent_id: &str, context_key: &str) -> KnowledgeResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        info!("Sharing context from agent {} to agent {}: {}", source_agent_id, target_agent_id, context_key);
        
        // Share cached context from one agent session to another
        if let Some(cached_context) = self.unified_engine.get_agent_context(source_agent_id, context_key).await? {
            self.unified_engine.set_agent_context(target_agent_id, context_key, &cached_context.data).await?;
        }
        
        Ok(())
    }
    
    /// Track agent session activity
    #[instrument(skip(self, agent_id, session_context))]
    pub async fn track_agent_session(&self, agent_id: &str, session_context: &AgentSessionContext) -> KnowledgeResult<()> {
        if !self.config.enabled || !self.config.agent_session_tracking {
            return Ok(());
        }
        
        // Update agent session tracking
        self.usage_analyzer.update_agent_session(agent_id, session_context).await?;
        
        Ok(())
    }
    
    /// Watch a directory for changes
    #[instrument(skip(self, dir_path))]
    pub async fn watch_directory(&self, dir_path: &str) -> KnowledgeResult<()> {
        if !self.config.enabled || !self.config.file_analysis_enabled {
            return Ok(());
        }
        
        info!("Watching directory for changes: {}", dir_path);
        
        let dir_path_buf = PathBuf::from(dir_path);
        self.file_watcher.watch_directory(dir_path_buf).await?;
        
        Ok(())
    }
    
    /// Get file watch statistics
    pub async fn get_file_watch_stats(&self) -> KnowledgeResult<FileWatchStats> {
        Ok(self.file_watcher.get_stats().await)
    }
    
    /// Get changed files
    pub async fn get_changed_files(&self) -> KnowledgeResult<Vec<PathBuf>> {
        self.file_watcher.check_for_changes().await
    }
    
    /// Get files matching a pattern
    pub async fn get_matching_files(&self, pattern: &str) -> KnowledgeResult<Vec<PathBuf>> {
        self.file_watcher.get_matching_files(pattern).await
    }
    
    /// Stop watching a file
    pub async fn unwatch_file(&self, file_path: &str) -> KnowledgeResult<()> {
        let file_path_buf = PathBuf::from(file_path);
        self.file_watcher.unwatch_file(&file_path_buf).await
    }
    
    /// Analyze file context for suggestions
    async fn analyze_file_context(&self, file_path: &str) -> KnowledgeResult<String> {
        // Simple file context analysis
        let path = PathBuf::from(file_path);
        
        // Extract file extension and name
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");
        
        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        // Create context string for search
        let context = format!("file:{} extension:{} name:{}", file_path, extension, file_name);
        
        Ok(context)
    }
    
    /// Convert search results to context suggestions
    async fn convert_to_context_suggestions(
        &self,
        search_results: Vec<crate::types::SemanticResult>,
        file_path: &str,
    ) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();
        
        for result in search_results {
            let suggestion = ContextSuggestion {
                suggestion_id: uuid::Uuid::new_v4().to_string(),
                title: format!("Relevant context for {}", file_path),
                description: result.content.lines().next().unwrap_or("").to_string(),
                relevance_score: result.relevance_score,
                content_type: result.metadata.source_type,
                cache_key: Some(result.cache_key.clone()),
                scope_path: result.metadata.scope_path.clone(),
                reasoning: format!("Semantic similarity score: {:.2}", result.relevance_score),
                confidence: result.relevance_score,
                action: SuggestionAction::Preload,
            };
            suggestions.push(suggestion);
        }
        
        Ok(suggestions)
    }
}

impl FileWatcher {
    pub fn new(config: FileWatcherConfig) -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            config,
            watcher: None,
            event_sender: None,
        }
    }
    
    /// Initialize the file watcher with real-time monitoring
    pub async fn initialize(&mut self) -> KnowledgeResult<()> {
        let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel::<notify::Event>();
        
        // Create the file system watcher
        let event_sender_clone = event_sender.clone();
        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
            if let Ok(event) = res {
                let _ = event_sender_clone.send(event);
            }
        })?;
        
        // Start watching the configured directories
        for pattern in &self.config.watch_patterns {
            if let Some(dir) = self.extract_directory_from_pattern(pattern) {
                if let Err(e) = watcher.watch(&dir, RecursiveMode::Recursive) {
                    warn!("Failed to watch directory {}: {}", dir.display(), e);
                }
            }
        }
        
        self.watcher = Some(watcher);
        self.event_sender = Some(event_sender);
        
        // Spawn event processing task
        let watched_files = self.watched_files.clone();
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                Self::process_file_event(&watched_files, event).await;
            }
        });
        
        Ok(())
    }
    
    /// Add a file to watch
    pub async fn watch_file(&self, file_path: PathBuf) -> KnowledgeResult<()> {
        let mut watched_files = self.watched_files.write().await;
        
        if watched_files.len() >= self.config.max_watched_files {
            return Err(crate::types::KnowledgeError::ProactiveError(ProactiveError::ConfigurationError(
                format!("Maximum watched files limit reached: {}", self.config.max_watched_files)
            )));
        }
        
        // Calculate content hash
        let content_hash = self.calculate_file_hash(&file_path).await?;
        
        let file_info = FileWatchInfo {
            path: file_path.clone(),
            last_modified: chrono::Utc::now(),
            content_hash,
            change_count: 0,
            last_accessed: chrono::Utc::now(),
        };
        
        watched_files.insert(file_path.clone(), file_info);
        
        // Add to file system watcher if available
        // TODO: Implement file system watching when we have proper mutable access
        // if let Some(watcher) = &mut self.watcher {
        //     if let Some(parent) = file_path.parent() {
        //         if let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive) {
        //             warn!("Failed to watch file {}: {}", file_path.display(), e);
        //         }
        //     }
        // }
        
        Ok(())
    }
    
    /// Watch an entire directory recursively
    pub async fn watch_directory(&self, dir_path: PathBuf) -> KnowledgeResult<()> {
        // TODO: Implement directory watching when we have proper mutable access
        // For now, just scan directory for existing files
        if let Ok(entries) = std::fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if self.should_watch_file(&path) {
                    // TODO: Add file to watched_files
                }
            }
        }
        
        Ok(())
    }
    
    /// Check for file changes (polling fallback)
    pub async fn check_for_changes(&self) -> KnowledgeResult<Vec<PathBuf>> {
        let mut changed_files = Vec::new();
        let mut watched_files = self.watched_files.write().await;
        
        for (path, info) in watched_files.iter_mut() {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    let modified_time = chrono::DateTime::from(modified);
                    if modified_time > info.last_modified {
                        // Check if content actually changed
                        let new_hash = self.calculate_file_hash(path).await?;
                        if new_hash != info.content_hash {
                            info.last_modified = modified_time;
                            info.content_hash = new_hash;
                            info.change_count += 1;
                            changed_files.push(path.clone());
                        }
                    }
                }
            }
        }
        
        Ok(changed_files)
    }
    
    /// Get files that match a pattern
    pub async fn get_matching_files(&self, pattern: &str) -> KnowledgeResult<Vec<PathBuf>> {
        let watched_files = self.watched_files.read().await;
        let mut matching_files = Vec::new();
        
        for path in watched_files.keys() {
            if self.matches_pattern(path, pattern) {
                matching_files.push(path.clone());
            }
        }
        
        Ok(matching_files)
    }
    
    /// Remove a file from watching
    pub async fn unwatch_file(&self, file_path: &PathBuf) -> KnowledgeResult<()> {
        let mut watched_files = self.watched_files.write().await;
        watched_files.remove(file_path);
        Ok(())
    }
    
    /// Get file watch statistics
    pub async fn get_stats(&self) -> FileWatchStats {
        let watched_files = self.watched_files.read().await;
        let total_files = watched_files.len();
        let total_changes = watched_files.values().map(|info| info.change_count).sum();
        
        FileWatchStats {
            total_watched_files: total_files,
            total_changes: total_changes,
            max_files: self.config.max_watched_files,
        }
    }
    
    /// Process file system events
    async fn process_file_event(watched_files: &Arc<RwLock<HashMap<PathBuf, FileWatchInfo>>>, event: notify::Event) {
        let mut files = watched_files.write().await;
        
        for path in event.paths {
            if let Some(info) = files.get_mut(&path) {
                info.last_modified = chrono::Utc::now();
                info.change_count += 1;
                debug!("File changed: {}", path.display());
            }
        }
    }
    
    /// Calculate file content hash
    async fn calculate_file_hash(&self, file_path: &PathBuf) -> KnowledgeResult<String> {
        use sha2::{Sha256, Digest};
        
        match tokio::fs::read(file_path).await {
            Ok(content) => {
                let mut hasher = Sha256::new();
                hasher.update(&content);
                Ok(format!("{:x}", hasher.finalize()))
            }
            Err(_) => Ok("".to_string()), // Return empty hash for non-existent files
        }
    }
    
    /// Check if a file should be watched based on patterns
    fn should_watch_file(&self, path: &PathBuf) -> bool {
        // Check ignore patterns first
        for ignore_pattern in &self.config.ignore_patterns {
            if self.matches_pattern(path, ignore_pattern) {
                return false;
            }
        }
        
        // Check watch patterns
        for watch_pattern in &self.config.watch_patterns {
            if self.matches_pattern(path, watch_pattern) {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a path matches a pattern
    fn matches_pattern(&self, path: &PathBuf, pattern: &str) -> bool {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if pattern.starts_with("*.") {
                let extension = pattern[1..].to_string();
                return file_name.ends_with(&extension);
            } else if pattern.ends_with("/") {
                return path.is_dir();
            } else {
                return file_name.contains(pattern);
            }
        }
        false
    }
    
    /// Extract directory from a file pattern
    fn extract_directory_from_pattern(&self, pattern: &str) -> Option<PathBuf> {
        if pattern.starts_with("*.") {
            // For file extensions, watch current directory
            Some(PathBuf::from("."))
        } else if pattern.ends_with("/") {
            // For directory patterns
            Some(PathBuf::from(pattern.trim_end_matches('/')))
        } else {
            // For other patterns, watch current directory
            Some(PathBuf::from("."))
        }
    }
}

impl UsageAnalyzer {
    pub fn new(config: UsageAnalyzerConfig) -> Self {
        Self {
            usage_patterns: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    pub async fn initialize(&self) -> KnowledgeResult<()> {
        info!("Initializing usage analyzer");
        // Initialize any required resources
        Ok(())
    }
    
    /// Predict context needs for a workflow
    pub async fn predict_context_needs(&self, workflow_context: &WorkflowContext) -> KnowledgeResult<Vec<String>> {
        let mut predicted_context = Vec::new();
        
        // Look for patterns based on workflow type
        let patterns = self.usage_patterns.read().await;
        for pattern in patterns.values() {
            if pattern.workflow_type == workflow_context.workflow_type {
                predicted_context.extend(pattern.context_keys.clone());
            }
        }
        
        // Add workflow-specific context requirements
        for requirement in &workflow_context.context_requirements {
            predicted_context.push(format!("requirement:{}", requirement.requirement_type.to_string()));
        }
        
        // Remove duplicates and limit results
        predicted_context.sort();
        predicted_context.dedup();
        predicted_context.truncate(10);
        
        Ok(predicted_context)
    }
    
    /// Predict context needs for an agent session
    pub async fn predict_agent_context_needs(&self, agent_id: &str, session_context: &AgentSessionContext) -> KnowledgeResult<Vec<String>> {
        let mut predicted_context = Vec::new();
        
        // Look for patterns based on agent preferences
        let patterns = self.usage_patterns.read().await;
        for pattern in patterns.values() {
            if pattern.agent_id == agent_id {
                predicted_context.extend(pattern.context_keys.clone());
            }
        }
        
        // Add agent-specific cached keys
        predicted_context.extend(session_context.cache_keys.clone());
        
        // Remove duplicates and limit results
        predicted_context.sort();
        predicted_context.dedup();
        predicted_context.truncate(10);
        
        Ok(predicted_context)
    }
    
    /// Update agent session tracking
    pub async fn update_agent_session(&self, agent_id: &str, session_context: &AgentSessionContext) -> KnowledgeResult<()> {
        let mut agent_sessions = self.agent_sessions.write().await;
        agent_sessions.insert(agent_id.to_string(), session_context.clone());
        Ok(())
    }
    
    /// Learn from usage patterns
    pub async fn learn_pattern(&self, agent_id: &str, workflow_type: WorkflowType, context_keys: Vec<String>) -> KnowledgeResult<()> {
        if !self.config.enable_learning {
            return Ok(());
        }
        
        let pattern_id = format!("{}:{}", agent_id, workflow_type.to_string());
        let mut patterns = self.usage_patterns.write().await;
        
        if let Some(pattern) = patterns.get_mut(&pattern_id) {
            pattern.frequency += 1;
            pattern.last_used = chrono::Utc::now();
            pattern.context_keys.extend(context_keys);
            pattern.context_keys.sort();
            pattern.context_keys.dedup();
        } else {
            let new_pattern = UsagePattern {
                pattern_id: pattern_id.clone(),
                agent_id: agent_id.to_string(),
                workflow_type,
                context_keys,
                frequency: 1,
                last_used: chrono::Utc::now(),
                confidence: 0.5,
                success_rate: 1.0,
            };
            patterns.insert(pattern_id, new_pattern);
        }
        
        Ok(())
    }
}

impl SuggestionEngine {
    pub fn new(config: SuggestionEngineConfig) -> Self {
        Self {
            suggestion_templates: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    pub async fn initialize(&self) -> KnowledgeResult<()> {
        info!("Initializing suggestion engine");
        // Initialize any required resources
        Ok(())
    }
    
    /// Generate suggestions based on templates
    pub async fn generate_suggestions(&self, context: &str, content_type: ContentType) -> KnowledgeResult<Vec<ContextSuggestion>> {
        let mut suggestions = Vec::new();
        let templates = self.suggestion_templates.read().await;
        
        for template in templates.values() {
            if template.content_type == content_type && self.matches_trigger_conditions(template, context).await? {
                let suggestion = ContextSuggestion {
                    suggestion_id: uuid::Uuid::new_v4().to_string(),
                    title: template.name.clone(),
                    description: template.description.clone(),
                    relevance_score: template.confidence_boost,
                    content_type: template.content_type.clone(),
                    cache_key: None,
                    scope_path: None,
                    reasoning: format!("Based on template: {}", template.template_id),
                    confidence: template.confidence_boost,
                    action: SuggestionAction::Preload,
                };
                suggestions.push(suggestion);
            }
        }
        
        Ok(suggestions)
    }
    
    /// Check if context matches trigger conditions
    async fn matches_trigger_conditions(&self, template: &SuggestionTemplate, context: &str) -> KnowledgeResult<bool> {
        for trigger in &template.trigger_conditions {
            match trigger {
                SuggestionTrigger::FileType(file_type) => {
                    if context.contains(file_type) {
                        return Ok(true);
                    }
                }
                SuggestionTrigger::ContentPattern(pattern) => {
                    if context.contains(pattern) {
                        return Ok(true);
                    }
                }
                _ => {
                    // TODO: Implement other trigger conditions
                }
            }
        }
        Ok(false)
    }
}

// Helper trait for WorkflowType
trait WorkflowTypeExt {
    fn to_string(&self) -> String;
}

impl WorkflowTypeExt for WorkflowType {
    fn to_string(&self) -> String {
        match self {
            WorkflowType::CodeReview => "code_review".to_string(),
            WorkflowType::FeatureDevelopment => "feature_development".to_string(),
            WorkflowType::BugFixing => "bug_fixing".to_string(),
            WorkflowType::Documentation => "documentation".to_string(),
            WorkflowType::Testing => "testing".to_string(),
            WorkflowType::Deployment => "deployment".to_string(),
            WorkflowType::Refactoring => "refactoring".to_string(),
            WorkflowType::Onboarding => "onboarding".to_string(),
            WorkflowType::Custom(name) => name.clone(),
        }
    }
}

// Helper trait for ContextRequirementType
trait ContextRequirementTypeExt {
    fn to_string(&self) -> String;
}

impl ContextRequirementTypeExt for crate::types::ContextRequirementType {
    fn to_string(&self) -> String {
        match self {
            crate::types::ContextRequirementType::Knowledge => "knowledge".to_string(),
            crate::types::ContextRequirementType::Code => "code".to_string(),
            crate::types::ContextRequirementType::Documentation => "documentation".to_string(),
            crate::types::ContextRequirementType::Decisions => "decisions".to_string(),
            crate::types::ContextRequirementType::Patterns => "patterns".to_string(),
            crate::types::ContextRequirementType::Dependencies => "dependencies".to_string(),
            crate::types::ContextRequirementType::Configuration => "configuration".to_string(),
        }
    }
} 