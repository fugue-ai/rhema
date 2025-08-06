use rhema_core::schema::{PromptInjectionMethod, PromptPattern};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Task type for context injection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    CodeReview,
    BugFix,
    FeatureDevelopment,
    Testing,
    Documentation,
    Refactoring,
    SecurityReview,
    PerformanceOptimization,
    DependencyUpdate,
    Deployment,
    LockFileManagement,
    DependencyResolution,
    ConflictResolution,
    Custom(String),
}

/// Context injection rule based on task type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextInjectionRule {
    /// Task type this rule applies to
    pub task_type: TaskType,
    /// Context files to load for this task
    pub context_files: Vec<String>,
    /// Injection method for this task
    pub injection_method: PromptInjectionMethod,
    /// Priority (higher = more specific)
    pub priority: u8,
    /// Additional context to include
    pub additional_context: Option<String>,
    /// Lock file context requirements
    pub lock_file_context: Option<LockFileContextRequirement>,
}

/// Lock file context requirements for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileContextRequirement {
    /// Whether to include dependency version information
    pub include_dependency_versions: bool,
    /// Whether to include conflict prevention information
    pub include_conflict_prevention: bool,
    /// Whether to include lock file health information
    pub include_health_info: bool,
    /// Whether to include dependency recommendations
    pub include_recommendations: bool,
    /// Specific scopes to include lock file context for
    pub target_scopes: Option<Vec<String>>,
    /// Whether to include transitive dependencies
    pub include_transitive_deps: bool,
}

/// Context cache entry for performance optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheEntry {
    /// Cached context content
    pub content: String,
    /// When this entry was created (timestamp in nanoseconds)
    #[serde(with = "timestamp_serde")]
    pub created_at: Instant,
    /// When this entry was last accessed (timestamp in nanoseconds)
    #[serde(with = "timestamp_serde")]
    pub last_accessed: Instant,
    /// Number of times this entry was accessed
    pub access_count: u32,
    /// Hash of the source files for cache invalidation
    pub source_hash: u64,
    /// Task type this context was generated for
    pub task_type: TaskType,
}

/// Context validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the context is valid
    pub is_valid: bool,
    /// Validation score (0.0 to 1.0)
    pub score: f64,
    /// List of validation issues found
    pub issues: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
    /// Completeness score (0.0 to 1.0)
    pub completeness: f64,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,
}

/// Context learning metrics for adaptive injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLearningMetrics {
    /// Task type this context was used for
    pub task_type: TaskType,
    /// Context content hash
    pub context_hash: u64,
    /// Success score (0.0 to 1.0)
    pub success_score: f64,
    /// Response quality score (0.0 to 1.0)
    pub response_quality: f64,
    /// User satisfaction score (0.0 to 1.0)
    pub user_satisfaction: f64,
    /// Usage timestamp (timestamp in nanoseconds)
    #[serde(with = "timestamp_serde")]
    pub timestamp: Instant,
    /// Context optimization suggestions
    pub optimization_suggestions: Vec<String>,
}

/// Context optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextOptimizationConfig {
    /// Maximum token count for optimized context
    pub max_tokens: usize,
    /// Minimum relevance score to include context
    pub min_relevance_score: f64,
    /// Whether to enable semantic compression
    pub enable_semantic_compression: bool,
    /// Whether to enable structure optimization
    pub enable_structure_optimization: bool,
    /// Whether to enable relevance filtering
    pub enable_relevance_filtering: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

/// Enhanced context injector with task detection and lock file support
pub struct EnhancedContextInjector {
    scope_path: PathBuf,
    injection_rules: Vec<ContextInjectionRule>,
    lock_file_path: Option<PathBuf>,
    // New enhancement features
    context_cache: Arc<RwLock<HashMap<String, ContextCacheEntry>>>,
    learning_metrics: Arc<RwLock<Vec<ContextLearningMetrics>>>,
    optimization_config: ContextOptimizationConfig,
    cache_ttl: Duration,
}

impl EnhancedContextInjector {
    /// Create a new enhanced context injector
    pub fn new(scope_path: PathBuf) -> Self {
        let default_rules = Self::get_default_injection_rules();
        let lock_file_path = scope_path.join("rhema.lock");
        
        Self {
            scope_path,
            injection_rules: default_rules,
            lock_file_path: if lock_file_path.exists() { Some(lock_file_path) } else { None },
            // Initialize enhancement features
            context_cache: Arc::new(RwLock::new(HashMap::new())),
            learning_metrics: Arc::new(RwLock::new(Vec::new())),
            optimization_config: ContextOptimizationConfig {
                max_tokens: 4000,
                min_relevance_score: 0.7,
                enable_semantic_compression: true,
                enable_structure_optimization: true,
                enable_relevance_filtering: true,
                cache_ttl_seconds: 3600, // 1 hour
            },
            cache_ttl: Duration::from_secs(3600),
        }
    }

    /// Create a new enhanced context injector with custom optimization config
    pub fn with_config(scope_path: PathBuf, config: ContextOptimizationConfig) -> Self {
        let default_rules = Self::get_default_injection_rules();
        let lock_file_path = scope_path.join("rhema.lock");
        
        Self {
            scope_path,
            injection_rules: default_rules,
            lock_file_path: if lock_file_path.exists() { Some(lock_file_path) } else { None },
            context_cache: Arc::new(RwLock::new(HashMap::new())),
            learning_metrics: Arc::new(RwLock::new(Vec::new())),
            optimization_config: config.clone(),
            cache_ttl: Duration::from_secs(config.cache_ttl_seconds),
        }
    }

    /// Inject context into a prompt pattern based on detected task type
    pub fn inject_context(
        &self,
        pattern: &PromptPattern,
        task_type: Option<TaskType>,
    ) -> RhemaResult<String> {
        let detected_task =
            task_type.unwrap_or_else(|| self.detect_task_type().unwrap_or(TaskType::CodeReview));

        // Find the best matching injection rule
        let rule = self.find_best_rule(&detected_task)?;

        // Load context based on the rule
        let context = self.load_context_for_task(&rule)?;

        // Apply injection method
        let final_prompt = match &rule.injection_method {
            PromptInjectionMethod::Prepend => {
                format!("{}\n\n{}", context, pattern.template)
            }
            PromptInjectionMethod::Append => {
                format!("{}\n\n{}", pattern.template, context)
            }
            PromptInjectionMethod::TemplateVariable => {
                pattern.template.replace("{{CONTEXT}}", &context)
            }
        };

        Ok(final_prompt)
    }

    /// 1. Dynamic Context Injection - Runtime context injection that adapts to changing conditions
    pub async fn inject_dynamic_context(
        &self,
        pattern: &PromptPattern,
        task_type: Option<TaskType>,
    ) -> RhemaResult<String> {
        let detected_task = task_type.unwrap_or_else(|| {
            self.detect_task_type().unwrap_or(TaskType::CodeReview)
        });

        // Check cache first for performance
        let cache_key = self.generate_cache_key(&detected_task, pattern);
        if let Some(cached_context) = self.get_cached_context(&cache_key).await {
            return Ok(cached_context);
        }

        // Get dynamic context based on current state
        let dynamic_context = self.load_dynamic_context(&detected_task).await?;
        
        // Optimize the context for AI consumption
        let optimized_context = self.optimize_context(&dynamic_context).await?;
        
        // Validate the context
        let validation_result = self.validate_context(&optimized_context).await?;
        if !validation_result.is_valid {
            return Err(RhemaError::InvalidInput(format!(
                "Context validation failed: {:?}",
                validation_result.issues
            )));
        }

        // Apply injection method
        let rule = self.find_best_rule(&detected_task)?;
        let final_prompt = match &rule.injection_method {
            PromptInjectionMethod::Prepend => {
                format!("{}\n\n{}", optimized_context, pattern.template)
            }
            PromptInjectionMethod::Append => {
                format!("{}\n\n{}", pattern.template, optimized_context)
            }
            PromptInjectionMethod::TemplateVariable => {
                pattern.template.replace("{{CONTEXT}}", &optimized_context)
            }
        };

        // Cache the result
        self.cache_context(&cache_key, &final_prompt, &detected_task).await;

        Ok(final_prompt)
    }

    /// 2. Context Optimization - Optimize injected context for better AI consumption
    pub async fn optimize_context(&self, context: &str) -> RhemaResult<String> {
        let mut optimized = context.to_string();

        // Apply semantic compression if enabled
        if self.optimization_config.enable_semantic_compression {
            optimized = self.apply_semantic_compression(&optimized).await?;
        }

        // Apply structure optimization if enabled
        if self.optimization_config.enable_structure_optimization {
            optimized = self.apply_structure_optimization(&optimized).await?;
        }

        // Apply relevance filtering if enabled
        if self.optimization_config.enable_relevance_filtering {
            optimized = self.apply_relevance_filtering(&optimized).await?;
        }

        // Ensure token limit compliance
        optimized = self.ensure_token_limit(&optimized)?;

        Ok(optimized)
    }

    /// 3. Context Learning - Learn from context usage patterns to improve future injections
    pub async fn learn_from_usage(
        &self,
        context: &str,
        task_type: &TaskType,
        success_metrics: &ContextLearningMetrics,
    ) -> RhemaResult<()> {
        let mut metrics = self.learning_metrics.write().await;
        
        // Add new learning metrics
        let new_metrics = ContextLearningMetrics {
            task_type: task_type.clone(),
            context_hash: self.calculate_context_hash(context),
            success_score: success_metrics.success_score,
            response_quality: success_metrics.response_quality,
            user_satisfaction: success_metrics.user_satisfaction,
            timestamp: Instant::now(),
            optimization_suggestions: success_metrics.optimization_suggestions.clone(),
        };
        
        metrics.push(new_metrics);
        
        // Keep only recent metrics (last 1000 entries)
        if metrics.len() > 1000 {
            metrics.sort_by_key(|m| m.timestamp);
            let to_remove = metrics.len() - 1000;
            metrics.drain(0..to_remove);
        }

        Ok(())
    }

    /// 4. Context Validation - Validate injected context for accuracy and completeness
    pub async fn validate_context(&self, context: &str) -> RhemaResult<ValidationResult> {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        let mut score: f64 = 1.0;
        let mut completeness = 1.0;
        let mut relevance = 1.0;

        // Check for empty or very short context
        if context.trim().is_empty() {
            issues.push("Context is empty".to_string());
            score -= 0.5;
            completeness = 0.0;
        } else if context.len() < 50 {
            warnings.push("Context is very short, may lack detail".to_string());
            score -= 0.1;
            completeness -= 0.2;
        }

        // Check for schema validation issues
        if let Err(e) = self.validate_context_schema(context) {
            issues.push(format!("Schema validation failed: {}", e));
            score -= 0.3;
        }

        // Check for cross-reference validation
        if let Err(e) = self.validate_cross_references(context) {
            issues.push(format!("Cross-reference validation failed: {}", e));
            score -= 0.2;
        }

        // Check for completeness
        completeness = self.calculate_completeness_score(context);
        if completeness < 0.8 {
            warnings.push("Context may be incomplete".to_string());
            score -= 0.1;
        }

        // Check for relevance
        relevance = self.calculate_relevance_score(context);
        if relevance < 0.7 {
            warnings.push("Context may not be relevant".to_string());
            score -= 0.2;
        }

        // Ensure score is within bounds
        score = score.max(0.0_f64).min(1.0_f64);

        Ok(ValidationResult {
            is_valid: issues.is_empty(),
            score,
            issues,
            warnings,
            completeness,
            relevance,
        })
    }

    /// 5. Context Caching - Cache frequently used contexts for performance
    pub async fn get_cached_context(&self, key: &str) -> Option<String> {
        let cache = self.context_cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            // Check if entry is still valid
            if entry.created_at.elapsed() < self.cache_ttl {
                // Update access metrics
                let content = entry.content.clone();
                drop(cache);
                let mut cache = self.context_cache.write().await;
                if let Some(entry) = cache.get_mut(key) {
                    entry.last_accessed = Instant::now();
                    entry.access_count += 1;
                }
                return Some(content);
            }
        }
        
        None
    }

    /// Cache context for future use
    pub async fn cache_context(&self, key: &str, content: &str, task_type: &TaskType) {
        let mut cache = self.context_cache.write().await;
        
        let entry = ContextCacheEntry {
            content: content.to_string(),
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 1,
            source_hash: self.calculate_source_hash(),
            task_type: task_type.clone(),
        };
        
        cache.insert(key.to_string(), entry);
        
        // Clean up old entries if cache is too large
        if cache.len() > 1000 {
            self.cleanup_cache(&mut cache).await;
        }
    }

    /// Inject lock file context specifically for AI agents
    pub async fn inject_lock_file_context(
        &self,
        pattern: &PromptPattern,
        scope_path: &str,
        lock_context_requirement: &LockFileContextRequirement,
    ) -> RhemaResult<String> {
        let mut lock_context = String::new();
        
        // Load lock file if available
        if let Some(lock_file_path) = &self.lock_file_path {
            if let Ok(lock_file) = rhema_core::lock::LockFileOps::read_lock_file(lock_file_path) {
                lock_context.push_str("## Lock File Context\n\n");
                
                // Add dependency version information
                if lock_context_requirement.include_dependency_versions {
                    lock_context.push_str(&self.format_dependency_versions(&lock_file, scope_path)?);
                }
                
                // Add conflict prevention information
                if lock_context_requirement.include_conflict_prevention {
                    lock_context.push_str(&self.format_conflict_prevention(&lock_file)?);
                }
                
                // Add health information
                if lock_context_requirement.include_health_info {
                    lock_context.push_str(&self.format_health_info(&lock_file)?);
                }
                
                // Add recommendations
                if lock_context_requirement.include_recommendations {
                    lock_context.push_str(&self.format_recommendations(&lock_file, scope_path)?);
                }
            }
        }
        
        // Apply injection method
        let final_prompt = match pattern.injection {
            PromptInjectionMethod::Prepend => {
                format!("{}\n\n{}", lock_context, pattern.template)
            }
            PromptInjectionMethod::Append => {
                format!("{}\n\n{}", pattern.template, lock_context)
            }
            PromptInjectionMethod::TemplateVariable => {
                pattern.template.replace("{{LOCK_CONTEXT}}", &lock_context)
            }
        };

        Ok(final_prompt)
    }

    /// Format dependency version information for AI context
    fn format_dependency_versions(&self, lock_file: &rhema_core::RhemaLock, scope_path: &str) -> RhemaResult<String> {
        let mut output = String::new();
        output.push_str("### Dependency Versions\n\n");
        
        if let Some(locked_scope) = lock_file.scopes.get(scope_path) {
            output.push_str(&format!("**Scope**: {}\n", scope_path));
            output.push_str(&format!("**Version**: {}\n", locked_scope.version));
            output.push_str(&format!("**Resolved**: {}\n\n", locked_scope.resolved_at));
            
            if !locked_scope.dependencies.is_empty() {
                output.push_str("**Dependencies**:\n");
                for (dep_name, dep) in &locked_scope.dependencies {
                    output.push_str(&format!("- **{}**: {} ({:?})\n", 
                        dep_name, dep.version, dep.dependency_type));
                    
                    if let Some(constraint) = &dep.original_constraint {
                        output.push_str(&format!("  - Original constraint: {}\n", constraint));
                    }
                    
                    if dep.is_transitive {
                        output.push_str("  - Transitive dependency\n");
                    }
                }
            } else {
                output.push_str("No dependencies found.\n");
            }
        } else {
            output.push_str(&format!("No lock file information found for scope: {}\n", scope_path));
        }
        
        Ok(output)
    }

    /// Format conflict prevention information for AI context
    fn format_conflict_prevention(&self, lock_file: &rhema_core::RhemaLock) -> RhemaResult<String> {
        let mut output = String::new();
        output.push_str("### Conflict Prevention Information\n\n");
        
        output.push_str(&format!("**Total Scopes**: {}\n", lock_file.metadata.total_scopes));
        output.push_str(&format!("**Total Dependencies**: {}\n", lock_file.metadata.total_dependencies));
        output.push_str(&format!("**Circular Dependencies**: {}\n", lock_file.metadata.circular_dependencies));
        output.push_str(&format!("**Resolution Strategy**: {:?}\n", lock_file.metadata.resolution_strategy));
        output.push_str(&format!("**Conflict Resolution**: {:?}\n\n", lock_file.metadata.conflict_resolution));
        
        // Check for potential conflicts
        let mut conflicts = Vec::new();
        for (scope_path, locked_scope) in &lock_file.scopes {
            for (dep_name, dep) in &locked_scope.dependencies {
                for (other_scope_path, other_scope) in &lock_file.scopes {
                    if scope_path != other_scope_path {
                        if let Some(other_dep) = other_scope.dependencies.get(dep_name) {
                            if dep.version != other_dep.version {
                                conflicts.push((dep_name.clone(), scope_path.clone(), dep.version.clone(), 
                                              other_scope_path.clone(), other_dep.version.clone()));
                            }
                        }
                    }
                }
            }
        }
        
        if !conflicts.is_empty() {
            output.push_str("**Potential Version Conflicts**:\n");
            for (dep_name, scope1, version1, scope2, version2) in conflicts {
                output.push_str(&format!("- **{}**: {} ({}), {} ({})\n", 
                    dep_name, scope1, version1, scope2, version2));
            }
        } else {
            output.push_str("No version conflicts detected.\n");
        }
        
        Ok(output)
    }

    /// Format health information for AI context
    fn format_health_info(&self, lock_file: &rhema_core::RhemaLock) -> RhemaResult<String> {
        let mut output = String::new();
        output.push_str("### Lock File Health\n\n");
        
        let mut health_score = 100.0;
        let mut issues = Vec::new();
        
        // Check validation status
        if lock_file.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
            health_score -= 30.0;
            issues.push("Lock file validation failed".to_string());
        }
        
        // Check circular dependencies
        if lock_file.metadata.circular_dependencies > 0 {
            health_score -= 20.0 * lock_file.metadata.circular_dependencies as f64;
            issues.push(format!("{} circular dependencies detected", lock_file.metadata.circular_dependencies));
        }
        
        // Check performance metrics
        if let Some(metrics) = &lock_file.metadata.performance_metrics {
            if metrics.generation_time_ms > 5000 {
                health_score -= 10.0;
                issues.push("Lock file generation took too long".to_string());
            }
            
            // Calculate cache hit rate manually
            let total_cache_operations = metrics.cache_hits + metrics.cache_misses;
            if total_cache_operations > 0 {
                let hit_rate = metrics.cache_hits as f64 / total_cache_operations as f64;
                if hit_rate < 0.5 {
                    health_score -= 5.0;
                    issues.push("Low cache hit rate detected".to_string());
                }
            }
        }
        
        health_score = health_score.max(0.0_f64);
        
        output.push_str(&format!("**Health Score**: {:.1}/100\n", health_score));
        output.push_str(&format!("**Validation Status**: {:?}\n", lock_file.metadata.validation_status));
        
        if let Some(last_validated) = &lock_file.metadata.last_validated {
            output.push_str(&format!("**Last Validated**: {}\n", last_validated));
        }
        
        if !issues.is_empty() {
            output.push_str("\n**Issues**:\n");
            for issue in issues {
                output.push_str(&format!("- {}\n", issue));
            }
        }
        
        Ok(output)
    }

    /// Format recommendations for AI context
    fn format_recommendations(&self, lock_file: &rhema_core::RhemaLock, scope_path: &str) -> RhemaResult<String> {
        let mut output = String::new();
        output.push_str("### Dependency Recommendations\n\n");
        
        let mut recommendations = Vec::new();
        
        // Check for outdated dependencies
        if let Some(locked_scope) = lock_file.scopes.get(scope_path) {
            for (dep_name, dep) in &locked_scope.dependencies {
                if dep.version.starts_with("0.") {
                    recommendations.push(format!("Consider upgrading {} from {} to a stable version", 
                        dep_name, dep.version));
                }
            }
        }
        
        // Add recommendations based on health issues
        if lock_file.metadata.circular_dependencies > 0 {
            recommendations.push("Review and resolve circular dependencies".to_string());
        }
        
        if lock_file.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
            recommendations.push("Fix validation issues in lock file".to_string());
        }
        
        if recommendations.is_empty() {
            output.push_str("No specific recommendations at this time.\n");
        } else {
            for (i, rec) in recommendations.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, rec));
            }
        }
        
        Ok(output)
    }

    /// Detect task type based on current context
    fn detect_task_type(&self) -> RhemaResult<TaskType> {
        // Check git status for clues
        if let Ok(git_status) = self.get_git_status() {
            if git_status.contains("test") || git_status.contains("spec") {
                return Ok(TaskType::Testing);
            }
            if git_status.contains("docs") || git_status.contains("README") {
                return Ok(TaskType::Documentation);
            }
            if git_status.contains("security") || git_status.contains("auth") {
                return Ok(TaskType::SecurityReview);
            }
            if git_status.contains("perf") || git_status.contains("optimize") {
                return Ok(TaskType::PerformanceOptimization);
            }
            if git_status.contains("refactor") {
                return Ok(TaskType::Refactoring);
            }
            if git_status.contains("dep") || git_status.contains("Cargo.toml") {
                return Ok(TaskType::DependencyUpdate);
            }
            if git_status.contains("lock") || git_status.contains("rhema.lock") {
                return Ok(TaskType::LockFileManagement);
            }
        }

        // Check file types in the scope
        if let Ok(file_types) = self.get_file_types() {
            if file_types.iter().any(|ft| ft.contains("test")) {
                return Ok(TaskType::Testing);
            }
            if file_types.iter().any(|ft| ft.contains("doc")) {
                return Ok(TaskType::Documentation);
            }
            if file_types.iter().any(|ft| ft.contains("lock")) {
                return Ok(TaskType::LockFileManagement);
            }
        }

        Ok(TaskType::CodeReview)
    }

    /// Find the best matching injection rule
    fn find_best_rule(&self, task_type: &TaskType) -> RhemaResult<&ContextInjectionRule> {
        self.injection_rules
            .iter()
            .filter(|rule| rule.task_type == *task_type)
            .max_by_key(|rule| rule.priority)
            .ok_or_else(|| RhemaError::InvalidInput(format!("No injection rule found for task type: {:?}", task_type)))
    }

    /// Load context based on the rule
    fn load_context_for_task(&self, rule: &ContextInjectionRule) -> RhemaResult<String> {
        let mut context = String::new();

        // Load specified context files
        for file_name in &rule.context_files {
            let file_path = self.scope_path.join(file_name);
            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    context.push_str(&format!("## {}\n\n{}\n\n", file_name, content));
                }
            }
        }

        // Add additional context if specified
        if let Some(additional) = &rule.additional_context {
            context.push_str(&format!("## Additional Context\n\n{}\n\n", additional));
        }

        // Add lock file context if required
        if let Some(lock_requirement) = &rule.lock_file_context {
            if let Some(lock_file_path) = &self.lock_file_path {
                if let Ok(lock_file) = rhema_core::lock::LockFileOps::read_lock_file(lock_file_path) {
                    context.push_str("## Lock File Context\n\n");
                    
                    if lock_requirement.include_dependency_versions {
                        if let Ok(dep_context) = self.format_dependency_versions(&lock_file, &self.scope_path.to_string_lossy()) {
                            context.push_str(&dep_context);
                        }
                    }
                    
                    if lock_requirement.include_conflict_prevention {
                        if let Ok(conflict_context) = self.format_conflict_prevention(&lock_file) {
                            context.push_str(&conflict_context);
                        }
                    }
                    
                    if lock_requirement.include_health_info {
                        if let Ok(health_context) = self.format_health_info(&lock_file) {
                            context.push_str(&health_context);
                        }
                    }
                }
            }
        }

        Ok(context)
    }

    /// Get git status for task detection
    fn get_git_status(&self) -> RhemaResult<String> {
        let output = std::process::Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(&self.scope_path)
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get file types in the scope for task detection
    fn get_file_types(&self) -> RhemaResult<Vec<String>> {
        let mut file_types = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.scope_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(ext) = entry.path().extension() {
                        file_types.push(ext.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        Ok(file_types)
    }

    /// Generate cache key for context caching
    fn generate_cache_key(&self, task_type: &TaskType, pattern: &PromptPattern) -> String {
        let pattern_hash = self.calculate_pattern_hash(pattern);
        let source_hash = self.calculate_source_hash();
        format!("{}_{}_{}", Self::task_type_to_string(task_type), pattern_hash, source_hash)
    }

    /// Calculate hash for pattern
    fn calculate_pattern_hash(&self, pattern: &PromptPattern) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        pattern.template.hash(&mut hasher);
        // Hash the injection method as a string since it doesn't implement Hash
        match pattern.injection {
            PromptInjectionMethod::Prepend => "prepend".hash(&mut hasher),
            PromptInjectionMethod::Append => "append".hash(&mut hasher),
            PromptInjectionMethod::TemplateVariable => "template_variable".hash(&mut hasher),
        }
        hasher.finish()
    }

    /// Calculate hash for source files
    fn calculate_source_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash all context files
        for rule in &self.injection_rules {
            for file_name in &rule.context_files {
                let file_path = self.scope_path.join(file_name);
                if let Ok(metadata) = std::fs::metadata(&file_path) {
                    if let Ok(modified) = metadata.modified() {
                        modified.hash(&mut hasher);
                    }
                }
            }
        }
        
        // Hash lock file if present
        if let Some(lock_path) = &self.lock_file_path {
            if let Ok(metadata) = std::fs::metadata(lock_path) {
                if let Ok(modified) = metadata.modified() {
                    modified.hash(&mut hasher);
                }
            }
        }
        
        hasher.finish()
    }

    /// Calculate hash for context content
    fn calculate_context_hash(&self, context: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        hasher.finish()
    }

    /// Load dynamic context based on current state
    async fn load_dynamic_context(&self, task_type: &TaskType) -> RhemaResult<String> {
        let mut context = String::new();
        
        // Get current git status for dynamic context
        if let Ok(git_status) = self.get_git_status() {
            context.push_str(&format!("## Current Git Status\n{}\n\n", git_status));
        }
        
        // Get current file changes
        if let Ok(changed_files) = self.get_changed_files() {
            if !changed_files.is_empty() {
                context.push_str("## Changed Files\n");
                for file in changed_files {
                    context.push_str(&format!("- {}\n", file));
                }
                context.push_str("\n");
            }
        }
        
        // Load task-specific context
        let rule = self.find_best_rule(task_type)?;
        let task_context = self.load_context_for_task(&rule)?;
        context.push_str(&task_context);
        
        Ok(context)
    }

    /// Get list of changed files
    fn get_changed_files(&self) -> RhemaResult<Vec<String>> {
        let output = std::process::Command::new("git")
            .args(&["diff", "--name-only"])
            .current_dir(&self.scope_path)
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        Ok(files)
    }

    /// Apply semantic compression to context
    async fn apply_semantic_compression(&self, context: &str) -> RhemaResult<String> {
        // Simple semantic compression - remove redundant information
        let mut compressed = context.to_string();
        
        // Remove duplicate lines
        let lines: Vec<&str> = compressed.lines().collect();
        let mut unique_lines = Vec::new();
        for line in lines {
            if !unique_lines.contains(&line) {
                unique_lines.push(line);
            }
        }
        compressed = unique_lines.join("\n");
        
        // Remove excessive whitespace
        compressed = compressed.replace("\n\n\n", "\n\n");
        
        Ok(compressed)
    }

    /// Apply structure optimization to context
    async fn apply_structure_optimization(&self, context: &str) -> RhemaResult<String> {
        let mut optimized = context.to_string();
        
        // Ensure proper section headers
        if !optimized.contains("## ") {
            optimized = format!("## Context\n\n{}", optimized);
        }
        
        // Ensure consistent formatting
        optimized = optimized.replace("\n\n\n", "\n\n");
        
        Ok(optimized)
    }

    /// Apply relevance filtering to context
    async fn apply_relevance_filtering(&self, context: &str) -> RhemaResult<String> {
        // Simple relevance filtering - keep sections that seem relevant
        let lines: Vec<&str> = context.lines().collect();
        let mut relevant_lines = Vec::new();
        
        for line in lines {
            // Keep headers and important content
            if line.starts_with("## ") || line.starts_with("# ") || 
               line.contains("TODO") || line.contains("FIXME") ||
               line.contains("import") || line.contains("use ") ||
               line.contains("fn ") || line.contains("struct ") ||
               line.contains("enum ") || line.contains("impl ") {
                relevant_lines.push(line);
            }
        }
        
        Ok(relevant_lines.join("\n"))
    }

    /// Ensure context stays within token limit
    fn ensure_token_limit(&self, context: &str) -> RhemaResult<String> {
        // Simple token estimation (rough approximation)
        let estimated_tokens = context.split_whitespace().count() + context.lines().count();
        
        if estimated_tokens > self.optimization_config.max_tokens {
            // Truncate context while preserving structure
            let lines: Vec<&str> = context.lines().collect();
            let mut truncated = Vec::new();
            let mut token_count = 0;
            
            for line in lines {
                let line_tokens = line.split_whitespace().count() + 1;
                if token_count + line_tokens > self.optimization_config.max_tokens {
                    truncated.push("... (truncated)");
                    break;
                }
                truncated.push(line);
                token_count += line_tokens;
            }
            
            Ok(truncated.join("\n"))
        } else {
            Ok(context.to_string())
        }
    }

    /// Validate context schema
    fn validate_context_schema(&self, context: &str) -> RhemaResult<()> {
        // Basic schema validation - check for required sections
        if !context.contains("## ") && !context.contains("# ") {
            return Err(RhemaError::InvalidInput("Context lacks proper structure".to_string()));
        }
        
        Ok(())
    }

    /// Validate cross-references in context
    fn validate_cross_references(&self, _context: &str) -> RhemaResult<()> {
        // Basic cross-reference validation
        // This could be enhanced to check for broken links, missing files, etc.
        Ok(())
    }

    /// Calculate completeness score
    fn calculate_completeness_score(&self, context: &str) -> f64 {
        let mut score: f64 = 1.0;
        
        // Check for minimum content length
        if context.len() < 100 {
            score -= 0.3;
        }
        
        // Check for required sections
        if !context.contains("## ") {
            score -= 0.2;
        }
        
        // Check for code blocks or technical content
        if !context.contains("```") && !context.contains("fn ") && !context.contains("struct ") {
            score -= 0.1;
        }
        
        score.max(0.0_f64)
    }

    /// Calculate relevance score
    fn calculate_relevance_score(&self, context: &str) -> f64 {
        let mut score: f64 = 1.0;
        
        // Check for technical terms
        let technical_terms = ["fn ", "struct ", "enum ", "impl ", "use ", "import", "TODO", "FIXME"];
        let mut found_terms = 0;
        
        for term in &technical_terms {
            if context.contains(term) {
                found_terms += 1;
            }
        }
        
        if found_terms == 0 {
            score -= 0.3;
        } else if found_terms < 2 {
            score -= 0.1;
        }
        
        score.max(0.0_f64)
    }

    /// Clean up cache by removing old entries
    async fn cleanup_cache(&self, cache: &mut HashMap<String, ContextCacheEntry>) {
        let now = Instant::now();
        let mut to_remove = Vec::new();
        
        for (key, entry) in cache.iter() {
            if now.duration_since(entry.created_at) > self.cache_ttl {
                to_remove.push(key.clone());
            }
        }
        
        for key in to_remove {
            cache.remove(&key);
        }
    }

    /// Convert task type to string for cache key
    fn task_type_to_string(task_type: &TaskType) -> &'static str {
        match task_type {
            TaskType::CodeReview => "code_review",
            TaskType::BugFix => "bug_fix",
            TaskType::FeatureDevelopment => "feature_dev",
            TaskType::Testing => "testing",
            TaskType::Documentation => "documentation",
            TaskType::Refactoring => "refactoring",
            TaskType::SecurityReview => "security_review",
            TaskType::PerformanceOptimization => "perf_optimization",
            TaskType::DependencyUpdate => "dependency_update",
            TaskType::Deployment => "deployment",
            TaskType::LockFileManagement => "lock_file_mgmt",
            TaskType::DependencyResolution => "dependency_resolution",
            TaskType::ConflictResolution => "conflict_resolution",
            TaskType::Custom(_) => "custom",
        }
    }

    /// Get cache statistics for monitoring
    pub async fn get_cache_stats(&self) -> (usize, f64) {
        let cache = self.context_cache.read().await;
        let total_entries = cache.len();
        let total_accesses: u32 = cache.values().map(|entry| entry.access_count).sum();
        let avg_accesses = if total_entries > 0 {
            total_accesses as f64 / total_entries as f64
        } else {
            0.0
        };
        (total_entries, avg_accesses)
    }

    /// Get learning metrics for analysis
    pub async fn get_learning_metrics(&self) -> Vec<ContextLearningMetrics> {
        let metrics = self.learning_metrics.read().await;
        metrics.clone()
    }

    /// Clear cache for testing or maintenance
    pub async fn clear_cache(&self) {
        let mut cache = self.context_cache.write().await;
        cache.clear();
    }

    /// Get optimization configuration
    pub fn get_optimization_config(&self) -> &ContextOptimizationConfig {
        &self.optimization_config
    }

    /// Update optimization configuration
    pub fn update_optimization_config(&mut self, config: ContextOptimizationConfig) {
        self.optimization_config = config.clone();
        self.cache_ttl = Duration::from_secs(config.cache_ttl_seconds);
    }

    /// Get cache hit rate
    pub async fn get_cache_hit_rate(&self) -> f64 {
        let cache = self.context_cache.read().await;
        let total_accesses: u32 = cache.values().map(|entry| entry.access_count).sum();
        let total_entries = cache.len();
        
        if total_accesses == 0 {
            0.0
        } else {
            (total_accesses - total_entries as u32) as f64 / total_accesses as f64
        }
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> (f64, f64, f64) {
        let cache_hit_rate = self.get_cache_hit_rate().await;
        let (_cache_size, avg_accesses) = self.get_cache_stats().await;
        let metrics_count = self.learning_metrics.read().await.len();
        
        (cache_hit_rate, avg_accesses, metrics_count as f64)
    }

    /// Get default injection rules including lock file management
    fn get_default_injection_rules() -> Vec<ContextInjectionRule> {
        vec![
            ContextInjectionRule {
                task_type: TaskType::CodeReview,
                context_files: vec!["knowledge.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 1,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: false,
                    include_health_info: true,
                    include_recommendations: false,
                    target_scopes: None,
                    include_transitive_deps: false,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::BugFix,
                context_files: vec!["knowledge.yaml".to_string(), "todos.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::FeatureDevelopment,
                context_files: vec!["knowledge.yaml".to_string(), "patterns.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: false,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::Testing,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 1,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: false,
                    include_health_info: false,
                    include_recommendations: false,
                    target_scopes: None,
                    include_transitive_deps: false,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::Documentation,
                context_files: vec!["knowledge.yaml".to_string(), "conventions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 1,
                additional_context: None,
                lock_file_context: None,
            },
            ContextInjectionRule {
                task_type: TaskType::Refactoring,
                context_files: vec!["knowledge.yaml".to_string(), "patterns.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::SecurityReview,
                context_files: vec!["knowledge.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 3,
                additional_context: Some("Security review requires careful analysis of dependencies and their versions.".to_string()),
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::PerformanceOptimization,
                context_files: vec!["knowledge.yaml".to_string(), "patterns.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: false,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::DependencyUpdate,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 3,
                additional_context: Some("Dependency updates require careful consideration of version compatibility and potential breaking changes.".to_string()),
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::Deployment,
                context_files: vec!["knowledge.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: None,
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: false,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::LockFileManagement,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 4,
                additional_context: Some("Lock file management requires understanding of dependency relationships and version constraints.".to_string()),
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::DependencyResolution,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 4,
                additional_context: Some("Dependency resolution requires analysis of version conflicts and compatibility.".to_string()),
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
            ContextInjectionRule {
                task_type: TaskType::ConflictResolution,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 4,
                additional_context: Some("Conflict resolution requires understanding of dependency relationships and version constraints.".to_string()),
                lock_file_context: Some(LockFileContextRequirement {
                    include_dependency_versions: true,
                    include_conflict_prevention: true,
                    include_health_info: true,
                    include_recommendations: true,
                    target_scopes: None,
                    include_transitive_deps: true,
                }),
            },
        ]
    }

    /// Add a custom injection rule
    pub fn add_rule(&mut self, rule: ContextInjectionRule) {
        self.injection_rules.push(rule);
    }

    /// Get all injection rules
    pub fn get_rules(&self) -> &Vec<ContextInjectionRule> {
        &self.injection_rules
    }

    /// Check if lock file is available
    pub fn has_lock_file(&self) -> bool {
        self.lock_file_path.is_some()
    }

    /// Get lock file path
    pub fn get_lock_file_path(&self) -> Option<&PathBuf> {
        self.lock_file_path.as_ref()
    }
}

/// Task detector for automatic task type identification
pub struct TaskDetector;

impl TaskDetector {
    /// Detect task type from commit message
    pub fn from_commit_message(message: &str) -> TaskType {
        let message_lower = message.to_lowercase();
        
        if message_lower.contains("fix") || message_lower.contains("bug") {
            TaskType::BugFix
        } else if message_lower.contains("feat") || message_lower.contains("feature") {
            TaskType::FeatureDevelopment
        } else if message_lower.contains("test") || message_lower.contains("spec") {
            TaskType::Testing
        } else if message_lower.contains("doc") || message_lower.contains("readme") {
            TaskType::Documentation
        } else if message_lower.contains("refactor") {
            TaskType::Refactoring
        } else if message_lower.contains("security") || message_lower.contains("auth") {
            TaskType::SecurityReview
        } else if message_lower.contains("perf") || message_lower.contains("optimize") {
            TaskType::PerformanceOptimization
        } else if message_lower.contains("dep") || message_lower.contains("update") {
            TaskType::DependencyUpdate
        } else if message_lower.contains("deploy") || message_lower.contains("release") {
            TaskType::Deployment
        } else if message_lower.contains("lock") || message_lower.contains("rhema.lock") {
            TaskType::LockFileManagement
        } else {
            TaskType::CodeReview
        }
    }

    /// Detect task type from file path
    pub fn from_file_path(path: &Path) -> TaskType {
        let path_str = path.to_string_lossy().to_lowercase();
        
        if path_str.contains("test") || path_str.contains("spec") {
            TaskType::Testing
        } else if path_str.contains("doc") || path_str.contains("readme") {
            TaskType::Documentation
        } else if path_str.contains("lock") || path_str.contains("rhema.lock") {
            TaskType::LockFileManagement
        } else if path_str.contains("cargo.toml") || path_str.contains("package.json") {
            TaskType::DependencyUpdate
        } else if path_str.contains("security") || path_str.contains("auth") {
            TaskType::SecurityReview
        } else {
            TaskType::CodeReview
        }
    }
}

/// Example usage of the enhanced context injection system
#[cfg(test)]
mod tests {
    use super::*;
    use rhema_core::schema::PromptInjectionMethod;

    #[tokio::test]
    async fn test_enhanced_context_injection() {
        // Create a temporary scope path
        let temp_dir = std::env::temp_dir().join("rhema_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        // Create test context files
        let knowledge_content = r#"
# Knowledge Base
- Use async/await for I/O operations
- Follow Rust naming conventions
- Implement proper error handling
"#;
        std::fs::write(temp_dir.join("knowledge.yaml"), knowledge_content).unwrap();
        
        // Create enhanced context injector
        let mut injector = EnhancedContextInjector::new(temp_dir.clone());
        
        // Create a test prompt pattern
        let pattern = PromptPattern {
            id: "test-pattern".to_string(),
            name: "Test Pattern".to_string(),
            description: Some("Test pattern for context injection".to_string()),
            template: "Please review this code:\n{{CONTEXT}}\n\nProvide feedback.".to_string(),
            injection: PromptInjectionMethod::TemplateVariable,
            usage_analytics: rhema_core::UsageAnalytics::new(),
            version: rhema_core::PromptVersion::new("1.0.0"),
            tags: Some(vec!["test".to_string()]),
        };
        
        // Test dynamic context injection
        let result = injector.inject_dynamic_context(&pattern, Some(TaskType::CodeReview)).await;
        assert!(result.is_ok());
        
        // Test context optimization
        let test_context = "## Test Context\n\nThis is a test context with some code:\n```rust\nfn test() {\n    println!(\"Hello, world!\");\n}\n```";
        let optimized = injector.optimize_context(test_context).await;
        assert!(optimized.is_ok());
        
        // Test context validation
        let validation = injector.validate_context(test_context).await;
        assert!(validation.is_ok());
        let validation_result = validation.unwrap();
        assert!(validation_result.is_valid);
        assert!(validation_result.score > 0.5);
        
        // Test learning from usage
        let metrics = ContextLearningMetrics {
            task_type: TaskType::CodeReview,
            context_hash: 12345,
            success_score: 0.9,
            response_quality: 0.8,
            user_satisfaction: 0.85,
            timestamp: Instant::now(),
            optimization_suggestions: vec!["Add more code examples".to_string()],
        };
        
        let learn_result = injector.learn_from_usage(test_context, &TaskType::CodeReview, &metrics).await;
        assert!(learn_result.is_ok());
        
        // Test cache functionality
        let cache_key = injector.generate_cache_key(&TaskType::CodeReview, &pattern);
        injector.cache_context(&cache_key, test_context, &TaskType::CodeReview).await;
        
        let cached = injector.get_cached_context(&cache_key).await;
        assert!(cached.is_some());
        
        // Test performance metrics
        let (hit_rate, avg_accesses, metrics_count) = injector.get_performance_metrics().await;
        assert!(hit_rate >= 0.0 && hit_rate <= 1.0);
        assert!(avg_accesses >= 0.0);
        assert!(metrics_count >= 0.0);
        
        // Clean up
        std::fs::remove_dir_all(temp_dir).unwrap();
    }
}

/// Example usage of the enhanced context injection system
pub async fn example_usage() -> RhemaResult<()> {
    // Create enhanced context injector with custom configuration
    let config = ContextOptimizationConfig {
        max_tokens: 3000,
        min_relevance_score: 0.8,
        enable_semantic_compression: true,
        enable_structure_optimization: true,
        enable_relevance_filtering: true,
        cache_ttl_seconds: 1800, // 30 minutes
    };
    
    let scope_path = PathBuf::from(".");
    let injector = EnhancedContextInjector::with_config(scope_path, config);
    
    // Create a prompt pattern
    let pattern = PromptPattern {
        id: "example-pattern".to_string(),
        name: "Example Pattern".to_string(),
        description: Some("Example pattern for context injection".to_string()),
        template: "Review this code and suggest improvements:\n{{CONTEXT}}".to_string(),
        injection: PromptInjectionMethod::TemplateVariable,
        usage_analytics: rhema_core::UsageAnalytics::new(),
        version: rhema_core::PromptVersion::new("1.0.0"),
        tags: Some(vec!["example".to_string()]),
    };
    
    // Use dynamic context injection
    let result = injector.inject_dynamic_context(&pattern, Some(TaskType::CodeReview)).await?;
    println!("Generated context: {}", result);
    
    // Get performance metrics
    let (hit_rate, avg_accesses, metrics_count) = injector.get_performance_metrics().await;
    println!("Cache hit rate: {:.2}%", hit_rate * 100.0);
    println!("Average accesses: {:.2}", avg_accesses);
    println!("Learning metrics count: {:.0}", metrics_count);
    
    Ok(())
}

// Serde module for handling Instant serialization
mod timestamp_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, Instant};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.duration_since(Instant::now());
        duration.as_nanos().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u128::deserialize(deserializer)?;
        let duration = Duration::from_nanos(nanos as u64);
        Ok(Instant::now() + duration)
    }
}
