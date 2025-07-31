use rhema_core::schema::{PromptInjectionMethod, PromptPattern};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

/// Enhanced context injector with task detection and lock file support
pub struct EnhancedContextInjector {
    scope_path: PathBuf,
    injection_rules: Vec<ContextInjectionRule>,
    lock_file_path: Option<PathBuf>,
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
        
        health_score = health_score.max(0.0);
        
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
