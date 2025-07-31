use rhema_core::{RhemaResult, RhemaError};
use rhema_core::schema::{PromptPattern, PromptInjectionMethod};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

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
}

/// Enhanced context injector with task detection
pub struct EnhancedContextInjector {
    scope_path: PathBuf,
    injection_rules: Vec<ContextInjectionRule>,
}

impl EnhancedContextInjector {
    /// Create a new enhanced context injector
    pub fn new(scope_path: PathBuf) -> Self {
        let default_rules = Self::get_default_injection_rules();
        Self {
            scope_path,
            injection_rules: default_rules,
        }
    }

    /// Inject context into a prompt pattern based on detected task type
    pub fn inject_context(&self, pattern: &PromptPattern, task_type: Option<TaskType>) -> RhemaResult<String> {
        let detected_task = task_type.unwrap_or_else(|| self.detect_task_type().unwrap_or(TaskType::CodeReview));
        
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
            if git_status.contains("fix") || git_status.contains("bug") {
                return Ok(TaskType::BugFix);
            }
            if git_status.contains("feat") || git_status.contains("feature") {
                return Ok(TaskType::FeatureDevelopment);
            }
        }

        // Check file types in current directory
        if let Ok(file_types) = self.get_file_types() {
            if file_types.iter().any(|ft| ft.contains("test") || ft.contains("spec")) {
                return Ok(TaskType::Testing);
            }
            if file_types.iter().any(|ft| ft.contains("md") || ft.contains("txt")) {
                return Ok(TaskType::Documentation);
            }
            if file_types.iter().any(|ft| ft.contains("lock") || ft.contains("toml")) {
                return Ok(TaskType::DependencyUpdate);
            }
        }

        // Default to code review if we can't determine
        Ok(TaskType::CodeReview)
    }

    /// Find the best matching injection rule for a task type
    fn find_best_rule(&self, task_type: &TaskType) -> RhemaResult<&ContextInjectionRule> {
        let matching_rules: Vec<&ContextInjectionRule> = self.injection_rules
            .iter()
            .filter(|rule| &rule.task_type == task_type)
            .collect();

        if matching_rules.is_empty() {
            // Fall back to default rule
            return self.injection_rules
                .iter()
                .find(|rule| rule.task_type == TaskType::CodeReview)
                .ok_or_else(|| RhemaError::ConfigError("No default injection rule found".to_string()));
        }

        // Return the rule with highest priority
        matching_rules
            .iter()
            .max_by_key(|rule| rule.priority)
            .copied()
            .ok_or_else(|| RhemaError::ConfigError("No matching injection rule found".to_string()))
    }

    /// Load context for a specific task
    fn load_context_for_task(&self, rule: &ContextInjectionRule) -> RhemaResult<String> {
        let mut context = String::new();
        
        // Load specified context files
        for file in &rule.context_files {
            let file_path = self.scope_path.join(".rhema").join(file);
            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    context.push_str(&format!("=== {} ===\n", file));
                    context.push_str(&content);
                    context.push_str("\n\n");
                }
            }
        }

        // Add task-specific context
        if let Some(additional) = &rule.additional_context {
            context.push_str(&format!("=== Task Context ===\n"));
            context.push_str(additional);
            context.push_str("\n\n");
        }

        // Add task type information
        context.push_str(&format!("=== Task Type ===\n"));
        context.push_str(&format!("Current task: {:?}\n", rule.task_type));
        context.push_str(&format!("Context files: {}\n", rule.context_files.join(", ")));
        
        if context.is_empty() {
            context = "No context files found".to_string();
        }
        
        Ok(context)
    }

    /// Get git status for task detection
    fn get_git_status(&self) -> RhemaResult<String> {
        // Simple implementation - in a real system, this would use git2 or similar
        let git_dir = self.scope_path.join(".git");
        if git_dir.exists() {
            // For now, return a placeholder - this would be enhanced with actual git status
            Ok("modified: src/main.rs".to_string())
        } else {
            Ok("no git repository".to_string())
        }
    }

    /// Get file types in current directory for task detection
    fn get_file_types(&self) -> RhemaResult<Vec<String>> {
        let mut file_types = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.scope_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        file_types.push(extension.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        Ok(file_types)
    }

    /// Get default injection rules
    fn get_default_injection_rules() -> Vec<ContextInjectionRule> {
        vec![
            ContextInjectionRule {
                task_type: TaskType::CodeReview,
                context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::TemplateVariable,
                priority: 1,
                additional_context: Some("Focus on code quality, best practices, and potential issues.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::BugFix,
                context_files: vec!["knowledge.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: Some("Consider previous bug fixes and known issues.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::Testing,
                context_files: vec!["patterns.yaml".to_string()],
                injection_method: PromptInjectionMethod::Append,
                priority: 2,
                additional_context: Some("Focus on test coverage, edge cases, and testing best practices.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::Documentation,
                context_files: vec!["knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::TemplateVariable,
                priority: 2,
                additional_context: Some("Ensure documentation is clear, accurate, and helpful.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::SecurityReview,
                context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 3,
                additional_context: Some("Focus on security vulnerabilities, authentication, and data protection.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::PerformanceOptimization,
                context_files: vec!["patterns.yaml".to_string()],
                injection_method: PromptInjectionMethod::TemplateVariable,
                priority: 2,
                additional_context: Some("Consider performance bottlenecks, optimization techniques, and benchmarks.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::Refactoring,
                context_files: vec!["patterns.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::Prepend,
                priority: 2,
                additional_context: Some("Maintain functionality while improving code structure and readability.".to_string()),
            },
            ContextInjectionRule {
                task_type: TaskType::FeatureDevelopment,
                context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string(), "decisions.yaml".to_string()],
                injection_method: PromptInjectionMethod::TemplateVariable,
                priority: 2,
                additional_context: Some("Consider existing patterns, user needs, and architectural decisions.".to_string()),
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
}

/// Task detection utilities
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
        } else if message_lower.contains("docs") || message_lower.contains("readme") {
            TaskType::Documentation
        } else if message_lower.contains("refactor") {
            TaskType::Refactoring
        } else if message_lower.contains("security") || message_lower.contains("auth") {
            TaskType::SecurityReview
        } else if message_lower.contains("perf") || message_lower.contains("optimize") {
            TaskType::PerformanceOptimization
        } else if message_lower.contains("deps") || message_lower.contains("update") {
            TaskType::DependencyUpdate
        } else {
            TaskType::CodeReview
        }
    }

    /// Detect task type from file path
    pub fn from_file_path(path: &Path) -> TaskType {
        let path_str = path.to_string_lossy().to_lowercase();
        
        if path_str.contains("test") || path_str.contains("spec") {
            TaskType::Testing
        } else if path_str.contains("docs") || path_str.contains("readme") {
            TaskType::Documentation
        } else if path_str.contains("security") || path_str.contains("auth") {
            TaskType::SecurityReview
        } else if path_str.contains("perf") || path_str.contains("benchmark") {
            TaskType::PerformanceOptimization
        } else {
            TaskType::CodeReview
        }
    }
} 