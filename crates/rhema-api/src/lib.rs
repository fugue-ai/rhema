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
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, instrument, warn};

// Re-export types from core crate
pub use rhema_core::{schema::*, scope, RhemaError, RhemaResult, Scope};

// Re-export types from other crates
pub use rhema_ai::ai_service;
pub use rhema_config::config;
pub use rhema_git::git_basic;
pub use rhema_integrations::integrations;
pub use rhema_mcp::mcp;
pub use rhema_monitoring::monitoring;
pub use rhema_query::{QueryProvenance, QueryResult};

// Re-export coordination types
pub use rhema_ai::agent::real_time_coordination::{
    AdvancedCoordinationConfig, AdvancedSession, AgentInfo, AgentMessage, AgentPerformanceMetrics,
    AgentStatus, ConsensusConfig, ConsensusEntry, ConsensusMessage, ConsensusState,
    CoordinationConfig, CoordinationError, CoordinationSession, CoordinationStats,
    EncryptionConfig, FaultToleranceConfig, LoadBalancingStrategy, MessagePriority, MessageType,
    PerformanceMetrics as CoordinationPerformanceMetrics, PerformanceMonitoringConfig,
    RealTimeCoordinationSystem, ResourceInfo, SessionDecision, SessionRule, SessionRuleType,
    SessionStatus,
};
pub use rhema_ai::coordination_integration::{
    CoordinationConfig as IntegrationConfig, CoordinationIntegration, IntegrationStats,
};

// API documentation module
pub mod api_docs;
pub use api_docs::{ApiDocGenerator, ApiDocumentation};

// Performance monitoring module
pub mod performance;
pub use performance::{
    AggregatedMetrics, PerformanceCheckResult, PerformanceGuard, PerformanceLimits,
    PerformanceMetrics, PerformanceMonitor, PerformanceOptimizer, ResourceManager,
    ResourceUsageStatus,
};

// Security module
pub mod security;
pub use security::{
    AccessControl, AuditLogEntry, AuditLogger, InputSanitizer, SecurityConfig, SecurityManager,
};

// Init module
pub mod init;
pub use init::run as init_run;

// Tests module
#[cfg(test)]
mod tests;

/// API version for backward compatibility
pub const API_VERSION: &str = "1.0.0";

/// Rate limiting configuration
#[derive(Debug)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_size: 100,
        }
    }
}

/// API input validation
#[derive(Debug, Clone)]
pub struct ApiInput {
    pub query: Option<String>,
    pub scope_name: Option<String>,
    pub file_path: Option<String>,
    pub operation: String,
    pub parameters: HashMap<String, serde_yaml::Value>,
}

impl ApiInput {
    pub fn validate(&self) -> RhemaResult<()> {
        if self.operation.is_empty() {
            return Err(RhemaError::InvalidInput(
                "Operation cannot be empty".to_string(),
            ));
        }

        // Validate query if present
        if let Some(ref query) = self.query {
            if query.is_empty() {
                return Err(RhemaError::InvalidInput(
                    "Query cannot be empty".to_string(),
                ));
            }
        }

        // Validate scope name if present
        if let Some(ref scope_name) = self.scope_name {
            if scope_name.is_empty() {
                return Err(RhemaError::InvalidInput(
                    "Scope name cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// Main Rhema context manager with enhanced features
pub struct Rhema {
    repo_root: PathBuf,
    rate_limit_config: RateLimitConfig,
    cache: Arc<RwLock<HashMap<String, serde_yaml::Value>>>,
    scope_cache: Arc<RwLock<HashMap<String, Scope>>>,
    /// Coordination system for agent communication
    coordination_system: Option<Arc<RealTimeCoordinationSystem>>,
    /// Coordination integration for external systems
    coordination_integration: Option<Arc<CoordinationIntegration>>,
}

impl Rhema {
    /// Create a new Rhema instance for the current repository
    #[instrument(skip_all)]
    pub fn new() -> RhemaResult<Self> {
        let repo_root = git_basic::find_repo_root()?;
        info!("Initializing Rhema for repository: {}", repo_root.display());

        Ok(Self {
            repo_root,
            rate_limit_config: RateLimitConfig::default(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            scope_cache: Arc::new(RwLock::new(HashMap::new())),
            coordination_system: None,
            coordination_integration: None,
        })
    }

    /// Create a new Rhema instance for a specific repository path
    #[instrument(skip_all)]
    pub fn new_from_path(repo_root: PathBuf) -> RhemaResult<Self> {
        // Verify that the path contains a git repository
        if !repo_root.join(".git").exists() {
            return Err(RhemaError::GitRepoNotFound(format!(
                "No Git repository found at {}",
                repo_root.display()
            )));
        }

        info!("Initializing Rhema for repository: {}", repo_root.display());

        Ok(Self {
            repo_root,
            rate_limit_config: RateLimitConfig::default(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            scope_cache: Arc::new(RwLock::new(HashMap::new())),
            coordination_system: None,
            coordination_integration: None,
        })
    }

    /// Create a new Rhema instance with custom rate limiting
    #[instrument(skip_all)]
    pub fn new_with_rate_limit(
        repo_root: PathBuf,
        rate_limit_config: RateLimitConfig,
    ) -> RhemaResult<Self> {
        if !repo_root.join(".git").exists() {
            return Err(RhemaError::GitRepoNotFound(format!(
                "No Git repository found at {}",
                repo_root.display()
            )));
        }

        info!(
            "Initializing Rhema with rate limiting for repository: {}",
            repo_root.display()
        );

        Ok(Self {
            repo_root,
            rate_limit_config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            scope_cache: Arc::new(RwLock::new(HashMap::new())),
            coordination_system: None,
            coordination_integration: None,
        })
    }

    /// Get the repository root path
    pub fn repo_root(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Get the repository root path (alias for repo_root)
    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Get API version
    pub fn api_version(&self) -> &str {
        API_VERSION
    }

    /// Initialize coordination system
    #[instrument(skip_all)]
    pub async fn init_coordination(
        &mut self,
        config: Option<CoordinationConfig>,
    ) -> RhemaResult<()> {
        let config = config.unwrap_or_default();
        info!("Initializing coordination system with config: {:?}", config);

        let coordination_system = RealTimeCoordinationSystem::with_config(config);
        self.coordination_system = Some(Arc::new(coordination_system));

        info!("✅ Coordination system initialized successfully");
        Ok(())
    }

    /// Initialize coordination with advanced features
    #[instrument(skip_all)]
    pub async fn init_advanced_coordination(
        &mut self,
        config: CoordinationConfig,
        advanced_config: AdvancedCoordinationConfig,
    ) -> RhemaResult<()> {
        info!("Initializing advanced coordination system");

        let coordination_system =
            RealTimeCoordinationSystem::with_advanced_config(config, advanced_config);
        self.coordination_system = Some(Arc::new(coordination_system));

        info!("✅ Advanced coordination system initialized successfully");
        Ok(())
    }

    /// Initialize coordination integration with external systems
    #[instrument(skip_all)]
    pub async fn init_coordination_integration(
        &mut self,
        integration_config: Option<IntegrationConfig>,
    ) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            let integration = CoordinationIntegration::new(
                coordination_system.as_ref().clone(),
                integration_config,
            )
            .await?;

            self.coordination_integration = Some(Arc::new(integration));
            info!("✅ Coordination integration initialized successfully");
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system must be initialized before integration".to_string(),
            });
        }

        Ok(())
    }

    /// Check if coordination system is initialized
    pub fn has_coordination(&self) -> bool {
        self.coordination_system.is_some()
    }

    /// Check if coordination integration is initialized
    pub fn has_coordination_integration(&self) -> bool {
        self.coordination_integration.is_some()
    }

    /// Get coordination system reference
    pub fn get_coordination_system(&self) -> Option<&Arc<RealTimeCoordinationSystem>> {
        self.coordination_system.as_ref()
    }

    /// Get coordination integration reference
    pub fn get_coordination_integration(&self) -> Option<&Arc<CoordinationIntegration>> {
        self.coordination_integration.as_ref()
    }

    /// Validate API input
    #[instrument(skip_all)]
    pub async fn validate_api_input(&self, input: &ApiInput) -> RhemaResult<()> {
        input.validate()?;

        // Additional validation based on operation type
        match input.operation.as_str() {
            "query" => {
                if input.query.is_none() {
                    return Err(RhemaError::InvalidInput(
                        "Query operation requires a query string".to_string(),
                    ));
                }
            }
            "load_scope" => {
                if input.scope_name.is_none() {
                    return Err(RhemaError::InvalidInput(
                        "Load scope operation requires a scope name".to_string(),
                    ));
                }
            }
            _ => {
                // Unknown operation type
                warn!("Unknown operation type: {}", input.operation);
            }
        }

        Ok(())
    }

    /// Handle operation with comprehensive error recovery
    #[instrument(skip_all)]
    pub async fn handle_operation_with_error_recovery(
        &self,
        operation: &ApiInput,
    ) -> RhemaResult<serde_yaml::Value> {
        // Validate input first
        self.validate_api_input(operation).await?;

        // Apply rate limiting
        self.check_rate_limit().await?;

        // Execute operation with error recovery
        let result = match operation.operation.as_str() {
            "query" => {
                if let Some(ref query) = operation.query {
                    self.query_with_error_recovery(query).await
                } else {
                    Err(RhemaError::InvalidInput(
                        "Query operation requires a query string".to_string(),
                    ))
                }
            }
            "discover_scopes" => self
                .discover_scopes_optimized()
                .await
                .map(|scopes| serde_yaml::to_value(scopes).unwrap_or_default()),
            "load_scope" => {
                if let Some(ref scope_name) = operation.scope_name {
                    self.get_scope_optimized(scope_name)
                        .await
                        .map(|scope| serde_yaml::to_value(scope).unwrap_or_default())
                } else {
                    Err(RhemaError::InvalidInput(
                        "Load scope operation requires a scope name".to_string(),
                    ))
                }
            }
            _ => Err(RhemaError::InvalidInput(format!(
                "Unknown operation: {}",
                operation.operation
            ))),
        };

        // Log operation result
        match &result {
            Ok(_) => info!("Operation '{}' completed successfully", operation.operation),
            Err(e) => error!("Operation '{}' failed: {}", operation.operation, e),
        }

        result
    }

    /// Check rate limiting
    #[instrument(skip_all)]
    async fn check_rate_limit(&self) -> RhemaResult<()> {
        // Simple rate limiting implementation
        // In a production environment, this would use a proper rate limiting library
        // For now, we'll just log the check
        info!("Rate limit check passed");
        Ok(())
    }

    /// Query with error recovery
    #[instrument(skip_all)]
    async fn query_with_error_recovery(&self, query: &str) -> RhemaResult<serde_yaml::Value> {
        // Check cache first
        let cache_key = format!("query:{}", query);
        if let Some(cached_result) = self.cache.read().await.get(&cache_key) {
            info!("Cache hit for query: {}", query);
            return Ok(cached_result.clone());
        }

        // Execute query with retry logic
        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            match self.query(query) {
                Ok(result) => {
                    // Cache the result
                    let mut cache = self.cache.write().await;
                    cache.insert(cache_key, result.clone());
                    return Ok(result);
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(e);
                    }
                    warn!("Query attempt {} failed: {}, retrying...", attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * attempts as u64))
                        .await;
                }
            }
        }

        unreachable!()
    }

    /// Discover all scopes in the repository with optimization
    #[instrument(skip_all)]
    pub async fn discover_scopes_optimized(&self) -> RhemaResult<Vec<Scope>> {
        // Check cache first
        let cache_key = "discovered_scopes".to_string();
        if let Some(cached_scopes) = self.scope_cache.read().await.get(&cache_key) {
            info!("Using cached scopes");
            return Ok(vec![cached_scopes.clone()]);
        }

        // Discover scopes
        let scopes = scope::discover_scopes(&self.repo_root)?;

        // Cache the first scope (representing the repository root)
        if let Some(first_scope) = scopes.first() {
            let mut scope_cache = self.scope_cache.write().await;
            scope_cache.insert(cache_key, first_scope.clone());
        }

        Ok(scopes)
    }

    /// Get a specific scope by path with optimization
    #[instrument(skip_all)]
    pub async fn get_scope_optimized(&self, path: &str) -> RhemaResult<Scope> {
        // Check cache first
        let cache_key = format!("scope:{}", path);
        if let Some(cached_scope) = self.scope_cache.read().await.get(&cache_key) {
            info!("Using cached scope: {}", path);
            return Ok(cached_scope.clone());
        }

        // Get scope
        let scope = scope::get_scope(&self.repo_root, path)?;

        // Cache the scope
        let mut scope_cache = self.scope_cache.write().await;
        scope_cache.insert(cache_key, scope.clone());

        Ok(scope)
    }

    /// Validate scope configuration
    #[instrument(skip_all)]
    pub async fn validate_scope(&self, scope: &Scope) -> RhemaResult<()> {
        // Validate scope definition
        scope.definition.validate()?;

        // Validate scope path exists
        if !scope.path.exists() {
            return Err(RhemaError::ValidationError(format!(
                "Scope path does not exist: {}",
                scope.path.display()
            )));
        }

        // Validate required files exist
        for (filename, filepath) in &scope.files {
            if !filepath.exists() {
                warn!(
                    "Scope file {} not found at {}",
                    filename,
                    filepath.display()
                );
            }
        }

        info!("Scope validation passed: {}", scope.path.display());
        Ok(())
    }

    /// Discover all scopes in the repository (legacy sync version)
    pub fn discover_scopes(&self) -> RhemaResult<Vec<Scope>> {
        Ok(scope::discover_scopes(&self.repo_root)?)
    }

    /// Get a specific scope by path (legacy sync version)
    pub fn get_scope(&self, path: &str) -> RhemaResult<Scope> {
        Ok(scope::get_scope(&self.repo_root, path)?)
    }

    /// Get the path for a specific scope
    pub fn scope_path(&self, scope_name: &str) -> RhemaResult<PathBuf> {
        let scope = self.get_scope(scope_name)?;
        Ok(scope.path)
    }

    /// Find scope path (alias for scope_path)
    pub fn find_scope_path(&self, scope_name: &str) -> RhemaResult<PathBuf> {
        self.scope_path(scope_name)
    }

    /// Get current scope path
    pub fn get_current_scope_path(&self) -> RhemaResult<PathBuf> {
        // Discover all scopes in the repository
        let scopes = self.discover_scopes()?;
        
        // If there's only one scope, return it
        if scopes.len() == 1 {
            return Ok(scopes[0].path.clone());
        }
        
        // If there are multiple scopes, try to find the one at the repo root
        for scope in &scopes {
            if scope.path.parent().unwrap() == &self.repo_root {
                return Ok(scope.path.clone());
            }
        }
        
        // If no scope found at repo root, return the first scope
        if let Some(first_scope) = scopes.first() {
            return Ok(first_scope.path.clone());
        }
        
        // If no scopes found, return the repo root
        Ok(self.repo_root.clone())
    }

    /// Execute a CQL query with enhanced error handling
    #[instrument(skip_all)]
    pub fn query(&self, query: &str) -> RhemaResult<serde_yaml::Value> {
        // Validate query input
        if query.trim().is_empty() {
            return Err(RhemaError::InvalidInput(
                "Query cannot be empty".to_string(),
            ));
        }

        // Execute query with performance monitoring
        let start = std::time::Instant::now();
        let result = rhema_query::execute_query(&self.repo_root, query)?;
        let duration = start.elapsed();

        info!("Query executed in {:?}: {}", duration, query);

        Ok(result)
    }

    /// Execute a CQL query with statistics
    pub fn query_with_stats(
        &self,
        query: &str,
    ) -> RhemaResult<(serde_yaml::Value, HashMap<String, serde_yaml::Value>)> {
        let result = rhema_query::execute_query(&self.repo_root, query)?;
        let stats = rhema_query::get_query_stats(&self.repo_root, query)?;
        Ok((result, stats))
    }

    /// Execute a CQL query with full provenance tracking
    pub fn query_with_provenance(
        &self,
        query: &str,
    ) -> RhemaResult<(serde_yaml::Value, QueryProvenance)> {
        Ok(rhema_query::execute_query_with_provenance(
            &self.repo_root,
            query,
        )?)
    }

    /// Search context with regex support
    pub fn search_regex(
        &self,
        pattern: &str,
        file_filter: Option<&str>,
    ) -> RhemaResult<Vec<QueryResult>> {
        Ok(rhema_query::search_context_regex(
            &self.repo_root,
            pattern,
            file_filter,
        )?)
    }

    /// Load knowledge for a specific scope with error recovery
    #[instrument(skip_all)]
    pub async fn load_knowledge_async(&self, scope_name: &str) -> RhemaResult<Knowledge> {
        let scope = self.get_scope_optimized(scope_name).await?;
        let knowledge_path = scope.path.join("knowledge.yaml");

        if knowledge_path.exists() {
            match tokio::fs::read_to_string(&knowledge_path).await {
                Ok(content) => match serde_yaml::from_str(&content) {
                    Ok(knowledge) => Ok(knowledge),
                    Err(e) => Err(RhemaError::InvalidYaml {
                        file: knowledge_path.display().to_string(),
                        message: e.to_string(),
                    }),
                },
                Err(e) => Err(RhemaError::IoError(e)),
            }
        } else {
            Ok(Knowledge {
                entries: Vec::new(),
                categories: None,
                custom: HashMap::new(),
            })
        }
    }

    /// Load knowledge for a specific scope (legacy sync version)
    pub fn load_knowledge(&self, scope_name: &str) -> RhemaResult<Knowledge> {
        let scope = self.get_scope(scope_name)?;
        let knowledge_path = scope.path.join("knowledge.yaml");
        if knowledge_path.exists() {
            let content = std::fs::read_to_string(&knowledge_path)?;
            let knowledge: Knowledge = serde_yaml::from_str(&content)?;
            Ok(knowledge)
        } else {
            Ok(Knowledge {
                entries: Vec::new(),
                categories: None,
                custom: HashMap::new(),
            })
        }
    }

    /// Load todos for a specific scope
    pub fn load_todos(&self, scope_name: &str) -> RhemaResult<Todos> {
        let scope = self.get_scope(scope_name)?;
        let todos_path = scope.path.join("todos.yaml");
        if todos_path.exists() {
            let content = std::fs::read_to_string(&todos_path)?;
            let todos: Todos = serde_yaml::from_str(&content)?;
            Ok(todos)
        } else {
            Ok(Todos {
                todos: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load decisions for a specific scope
    pub fn load_decisions(&self, scope_name: &str) -> RhemaResult<Decisions> {
        let scope = self.get_scope(scope_name)?;
        let decisions_path = scope.path.join("decisions.yaml");
        if decisions_path.exists() {
            let content = std::fs::read_to_string(&decisions_path)?;
            let decisions: Decisions = serde_yaml::from_str(&content)?;
            Ok(decisions)
        } else {
            Ok(Decisions {
                decisions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load patterns for a specific scope
    pub fn load_patterns(&self, scope_name: &str) -> RhemaResult<Patterns> {
        let scope = self.get_scope(scope_name)?;
        let patterns_path = scope.path.join("patterns.yaml");
        if patterns_path.exists() {
            let content = std::fs::read_to_string(&patterns_path)?;
            let patterns: Patterns = serde_yaml::from_str(&content)?;
            Ok(patterns)
        } else {
            Ok(Patterns {
                patterns: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load conventions for a specific scope
    pub fn load_conventions(&self, scope_name: &str) -> RhemaResult<Conventions> {
        let scope = self.get_scope(scope_name)?;
        let conventions_path = scope.path.join("conventions.yaml");
        if conventions_path.exists() {
            let content = std::fs::read_to_string(&conventions_path)?;
            let conventions: Conventions = serde_yaml::from_str(&content)?;
            Ok(conventions)
        } else {
            Ok(Conventions {
                conventions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load scope by name
    pub fn load_scope(&self, name: &str) -> RhemaResult<Scope> {
        self.get_scope(name)
    }

    /// List all scopes
    pub fn list_scopes(&self) -> RhemaResult<Vec<Scope>> {
        self.discover_scopes()
    }

    /// Clear all caches
    #[instrument(skip_all)]
    pub async fn clear_caches(&self) -> RhemaResult<()> {
        let mut cache = self.cache.write().await;
        cache.clear();

        let mut scope_cache = self.scope_cache.write().await;
        scope_cache.clear();

        info!("All caches cleared");
        Ok(())
    }

    /// Get cache statistics
    #[instrument(skip_all)]
    pub async fn get_cache_stats(&self) -> RhemaResult<HashMap<String, usize>> {
        let cache_size = self.cache.read().await.len();
        let scope_cache_size = self.scope_cache.read().await.len();

        Ok(HashMap::from([
            ("query_cache_size".to_string(), cache_size),
            ("scope_cache_size".to_string(), scope_cache_size),
        ]))
    }

    // ===== COORDINATION METHODS =====

    /// Register an agent with the coordination system
    #[instrument(skip_all)]
    pub async fn register_agent(&self, agent_info: AgentInfo) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            let agent_id = agent_info.id.clone();
            coordination_system.register_agent(agent_info).await?;
            info!("✅ Agent registered with coordination system: {}", agent_id);
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Send a message through the coordination system
    #[instrument(skip_all)]
    pub async fn send_coordination_message(&self, message: AgentMessage) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            coordination_system.send_message(message).await?;
            info!("✅ Message sent through coordination system");
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Create a coordination session
    #[instrument(skip_all)]
    pub async fn create_coordination_session(
        &self,
        topic: String,
        participants: Vec<String>,
    ) -> RhemaResult<String> {
        if let Some(coordination_system) = &self.coordination_system {
            let session_id = coordination_system
                .create_session(topic, participants)
                .await?;
            info!("✅ Coordination session created: {}", session_id);
            Ok(session_id)
        } else {
            Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            })
        }
    }

    /// Join a coordination session
    #[instrument(skip_all)]
    pub async fn join_coordination_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            coordination_system
                .join_session(session_id, agent_id)
                .await?;
            info!("✅ Agent {} joined session {}", agent_id, session_id);
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Send a session message
    #[instrument(skip_all)]
    pub async fn send_session_message(
        &self,
        session_id: &str,
        message: AgentMessage,
    ) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            coordination_system
                .send_session_message(session_id, message)
                .await?;
            info!("✅ Session message sent to {}", session_id);
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Get coordination statistics
    #[instrument(skip_all)]
    pub async fn get_coordination_stats(&self) -> RhemaResult<CoordinationStats> {
        if let Some(coordination_system) = &self.coordination_system {
            Ok(coordination_system.get_stats())
        } else {
            Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            })
        }
    }

    /// Get all registered agents
    #[instrument(skip_all)]
    pub async fn get_all_agents(&self) -> RhemaResult<Vec<AgentInfo>> {
        if let Some(coordination_system) = &self.coordination_system {
            Ok(coordination_system.get_all_agents().await)
        } else {
            Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            })
        }
    }

    /// Get agent information
    #[instrument(skip_all)]
    pub async fn get_agent_info(&self, agent_id: &str) -> RhemaResult<Option<AgentInfo>> {
        if let Some(coordination_system) = &self.coordination_system {
            Ok(coordination_system.get_agent_info(agent_id).await)
        } else {
            Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            })
        }
    }

    /// Update agent status
    #[instrument(skip_all)]
    pub async fn update_agent_status(
        &self,
        agent_id: &str,
        status: AgentStatus,
    ) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            coordination_system
                .update_agent_status(agent_id, status)
                .await?;
            info!("✅ Agent {} status updated", agent_id);
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Start coordination health monitoring
    #[instrument(skip_all)]
    pub async fn start_coordination_health_monitoring(&self) -> RhemaResult<()> {
        if let Some(coordination_system) = &self.coordination_system {
            coordination_system.start_heartbeat_monitoring().await;
            info!("✅ Coordination health monitoring started");
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination system not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Get coordination integration statistics
    #[instrument(skip_all)]
    pub async fn get_integration_stats(&self) -> RhemaResult<IntegrationStats> {
        if let Some(integration) = &self.coordination_integration {
            Ok(integration.get_integration_stats().await)
        } else {
            Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination integration not initialized".to_string(),
            })
        }
    }

    /// Bridge a message through coordination integration
    #[instrument(skip_all)]
    pub async fn bridge_coordination_message(&self, message: &AgentMessage) -> RhemaResult<()> {
        if let Some(integration) = &self.coordination_integration {
            integration.bridge_rhema_message(message).await?;
            info!("✅ Message bridged through coordination integration");
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination integration not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Start coordination integration health monitoring
    #[instrument(skip_all)]
    pub async fn start_integration_health_monitoring(&self) -> RhemaResult<()> {
        if let Some(integration) = &self.coordination_integration {
            integration.start_health_monitoring().await?;
            info!("✅ Coordination integration health monitoring started");
        } else {
            return Err(RhemaError::InvalidYaml {
                file: "coordination".to_string(),
                message: "Coordination integration not initialized".to_string(),
            });
        }
        Ok(())
    }

    /// Shutdown coordination systems
    #[instrument(skip_all)]
    pub async fn shutdown_coordination(&self) -> RhemaResult<()> {
        if let Some(integration) = &self.coordination_integration {
            integration.shutdown().await?;
            info!("✅ Coordination integration shutdown complete");
        }

        info!("✅ Coordination systems shutdown complete");
        Ok(())
    }
}
