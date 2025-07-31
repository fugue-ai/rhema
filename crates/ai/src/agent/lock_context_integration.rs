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

use crate::agent::lock_context::*;
use crate::context_injection::{EnhancedContextInjector, LockFileContextRequirement, TaskType};
use rhema_core::{RhemaResult, PromptPattern, PromptInjectionMethod};
use std::collections::HashMap;
use std::path::PathBuf;

/// Integration example for using lock file context with AI agents
pub struct LockFileAIIntegration {
    context_provider: LockFileContextProvider,
    context_injector: EnhancedContextInjector,
}

impl LockFileAIIntegration {
    /// Create a new lock file AI integration
    pub fn new(project_root: PathBuf) -> Self {
        let lock_file_path = project_root.join("rhema.lock");
        let context_provider = LockFileContextProvider::new(lock_file_path);
        let context_injector = EnhancedContextInjector::new(project_root);
        
        Self {
            context_provider,
            context_injector,
        }
    }

    /// Initialize the integration by loading lock file data
    pub fn initialize(&mut self) -> RhemaResult<()> {
        self.context_provider.load_lock_file()?;
        Ok(())
    }

    /// Get comprehensive AI context with lock file information
    pub fn get_comprehensive_context(&self) -> RhemaResult<LockFileAIContext> {
        self.context_provider.get_ai_context()
    }

    /// Get scope-specific context for AI agents
    pub fn get_scope_context(&self, scope_path: &str) -> RhemaResult<ScopeLockContext> {
        self.context_provider.get_scope_context(scope_path)
    }

    /// Generate AI prompt with lock file context for dependency updates
    pub fn generate_dependency_update_prompt(&self, scope_path: &str, prompt_template: &str) -> RhemaResult<String> {
        let pattern = PromptPattern {
            id: "dependency_update".to_string(),
            name: "Dependency Update".to_string(),
            description: Some("AI prompt for dependency updates".to_string()),
            template: prompt_template.to_string(),
            injection: PromptInjectionMethod::Prepend,
            usage_analytics: rhema_core::schema::UsageAnalytics::new(),
            version: rhema_core::schema::PromptVersion::new("1.0.0"),
            tags: None,
        };

        let lock_requirement = LockFileContextRequirement {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: Some(vec![scope_path.to_string()]),
            include_transitive_deps: true,
        };

        // Use async runtime for the async method
        tokio::runtime::Runtime::new()?.block_on(
            self.context_injector.inject_lock_file_context(&pattern, scope_path, &lock_requirement)
        )
    }

    /// Generate AI prompt with lock file context for conflict resolution
    pub fn generate_conflict_resolution_prompt(&self, prompt_template: &str) -> RhemaResult<String> {
        let pattern = PromptPattern {
            id: "conflict_resolution".to_string(),
            name: "Conflict Resolution".to_string(),
            description: Some("AI prompt for conflict resolution".to_string()),
            template: prompt_template.to_string(),
            injection: PromptInjectionMethod::Prepend,
            usage_analytics: rhema_core::schema::UsageAnalytics::new(),
            version: rhema_core::schema::PromptVersion::new("1.0.0"),
            tags: None,
        };

        let lock_requirement = LockFileContextRequirement {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: None,
            include_transitive_deps: true,
        };

        // Get the project root scope
        let project_root = self.context_injector.get_lock_file_path()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("No lock file found".to_string()))?
            .parent()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Invalid lock file path".to_string()))?
            .to_string_lossy()
            .to_string();

        tokio::runtime::Runtime::new()?.block_on(
            self.context_injector.inject_lock_file_context(&pattern, &project_root, &lock_requirement)
        )
    }

    /// Generate AI prompt with lock file context for health assessment
    pub fn generate_health_assessment_prompt(&self, prompt_template: &str) -> RhemaResult<String> {
        let pattern = PromptPattern {
            id: "health_assessment".to_string(),
            name: "Health Assessment".to_string(),
            description: Some("AI prompt for health assessment".to_string()),
            template: prompt_template.to_string(),
            injection: PromptInjectionMethod::Prepend,
            usage_analytics: rhema_core::schema::UsageAnalytics::new(),
            version: rhema_core::schema::PromptVersion::new("1.0.0"),
            tags: None,
        };

        let lock_requirement = LockFileContextRequirement {
            include_dependency_versions: false,
            include_conflict_prevention: false,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: None,
            include_transitive_deps: false,
        };

        let project_root = self.context_injector.get_lock_file_path()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("No lock file found".to_string()))?
            .parent()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Invalid lock file path".to_string()))?
            .to_string_lossy()
            .to_string();

        tokio::runtime::Runtime::new()?.block_on(
            self.context_injector.inject_lock_file_context(&pattern, &project_root, &lock_requirement)
        )
    }

    /// Generate AI prompt with lock file context for security review
    pub fn generate_security_review_prompt(&self, scope_path: &str, prompt_template: &str) -> RhemaResult<String> {
        let pattern = PromptPattern {
            id: "security_review".to_string(),
            name: "Security Review".to_string(),
            description: Some("AI prompt for security review".to_string()),
            template: prompt_template.to_string(),
            injection: PromptInjectionMethod::Prepend,
            usage_analytics: rhema_core::schema::UsageAnalytics::new(),
            version: rhema_core::schema::PromptVersion::new("1.0.0"),
            tags: None,
        };

        let lock_requirement = LockFileContextRequirement {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: Some(vec![scope_path.to_string()]),
            include_transitive_deps: true,
        };

        tokio::runtime::Runtime::new()?.block_on(
            self.context_injector.inject_lock_file_context(&pattern, scope_path, &lock_requirement)
        )
    }

    /// Generate AI prompt with lock file context for performance optimization
    pub fn generate_performance_prompt(&self, scope_path: &str, prompt_template: &str) -> RhemaResult<String> {
        let pattern = PromptPattern {
            id: "performance_optimization".to_string(),
            name: "Performance Optimization".to_string(),
            description: Some("AI prompt for performance optimization".to_string()),
            template: prompt_template.to_string(),
            injection: PromptInjectionMethod::Prepend,
            usage_analytics: rhema_core::schema::UsageAnalytics::new(),
            version: rhema_core::schema::PromptVersion::new("1.0.0"),
            tags: None,
        };

        let lock_requirement = LockFileContextRequirement {
            include_dependency_versions: true,
            include_conflict_prevention: false,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: Some(vec![scope_path.to_string()]),
            include_transitive_deps: true,
        };

        tokio::runtime::Runtime::new()?.block_on(
            self.context_injector.inject_lock_file_context(&pattern, scope_path, &lock_requirement)
        )
    }

    /// Get dependency recommendations for AI agents
    pub fn get_dependency_recommendations(&self, scope_path: &str) -> RhemaResult<Vec<Recommendation>> {
        let context = self.context_provider.get_scope_context(scope_path)?;
        Ok(context.recommendations)
    }

    /// Get conflict analysis for AI agents
    pub fn get_conflict_analysis(&self) -> RhemaResult<ConflictAnalysis> {
        let context = self.context_provider.get_ai_context()?;
        Ok(context.conflict_analysis)
    }

    /// Get health assessment for AI agents
    pub fn get_health_assessment(&self) -> RhemaResult<HealthAssessment> {
        let context = self.context_provider.get_ai_context()?;
        Ok(context.health_assessment)
    }

    /// Check if lock file is available
    pub fn has_lock_file(&self) -> bool {
        self.context_provider.has_lock_file()
    }

    /// Get lock file statistics for AI agents
    pub fn get_lock_file_stats(&self) -> RhemaResult<LockFileSummary> {
        let context = self.context_provider.get_ai_context()?;
        Ok(context.summary)
    }
}

/// Example usage functions for AI agents
pub mod examples {
    use super::*;

    /// Example: Generate a dependency update prompt for AI agents
    pub fn example_dependency_update_prompt() -> RhemaResult<String> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;

        let prompt_template = r#"
You are an AI agent tasked with updating dependencies for the scope: {{SCOPE_PATH}}.

Please analyze the current dependency versions and provide recommendations for updates.
Consider:
1. Security vulnerabilities
2. Performance improvements
3. Breaking changes
4. Compatibility issues

Provide specific version recommendations and migration steps.
"#;

        integration.generate_dependency_update_prompt("crates/core", prompt_template)
    }

    /// Example: Generate a conflict resolution prompt for AI agents
    pub fn example_conflict_resolution_prompt() -> RhemaResult<String> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;

        let prompt_template = r#"
You are an AI agent tasked with resolving dependency conflicts in the project.

Please analyze the lock file for version conflicts and circular dependencies.
Provide specific recommendations for:
1. Resolving version conflicts
2. Breaking circular dependencies
3. Optimizing dependency resolution
4. Improving lock file health

Provide actionable steps for each identified issue.
"#;

        integration.generate_conflict_resolution_prompt(prompt_template)
    }

    /// Example: Generate a security review prompt for AI agents
    pub fn example_security_review_prompt() -> RhemaResult<String> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;

        let prompt_template = r#"
You are an AI agent tasked with performing a security review of dependencies for scope: {{SCOPE_PATH}}.

Please analyze the dependencies for:
1. Known security vulnerabilities
2. Outdated packages with security patches
3. Suspicious or malicious packages
4. Overly permissive version constraints

Provide a security assessment and recommendations for addressing any issues found.
"#;

        integration.generate_security_review_prompt("crates/core", prompt_template)
    }

    /// Example: Generate a performance optimization prompt for AI agents
    pub fn example_performance_prompt() -> RhemaResult<String> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;

        let prompt_template = r#"
You are an AI agent tasked with optimizing performance for scope: {{SCOPE_PATH}}.

Please analyze the dependencies for:
1. Performance bottlenecks
2. Heavy dependencies that could be replaced
3. Duplicate functionality
4. Unused dependencies

Provide recommendations for optimizing the dependency tree and improving build/run performance.
"#;

        integration.generate_performance_prompt("crates/core", prompt_template)
    }

    /// Example: Get comprehensive lock file analysis for AI agents
    pub fn example_comprehensive_analysis() -> RhemaResult<LockFileAIContext> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;
        integration.get_comprehensive_context()
    }

    /// Example: Get scope-specific recommendations for AI agents
    pub fn example_scope_recommendations() -> RhemaResult<Vec<Recommendation>> {
        let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
        integration.initialize()?;
        integration.get_dependency_recommendations("crates/core")
    }
}

/// Utility functions for AI agent integration
pub mod utils {
    use super::*;
    use std::collections::HashMap;

    /// Convert lock file context to a format suitable for AI model input
    pub fn format_context_for_ai(context: &LockFileAIContext) -> String {
        let mut output = String::new();
        
        // Summary
        output.push_str(&format!("## Lock File Summary\n"));
        output.push_str(&format!("- Total Scopes: {}\n", context.summary.total_scopes));
        output.push_str(&format!("- Total Dependencies: {}\n", context.summary.total_dependencies));
        output.push_str(&format!("- Circular Dependencies: {}\n", context.summary.circular_dependencies));
        output.push_str(&format!("- Validation Status: {}\n", context.summary.validation_status));
        output.push_str(&format!("- Resolution Strategy: {}\n", context.summary.resolution_strategy));
        output.push_str(&format!("- Conflict Resolution: {}\n\n", context.summary.conflict_resolution));

        // Health Assessment
        output.push_str(&format!("## Health Assessment\n"));
        output.push_str(&format!("- Overall Score: {:.1}/100\n", context.health_assessment.overall_score));
        output.push_str(&format!("- Status: {:?}\n", context.health_assessment.status));
        
        if !context.health_assessment.issues.is_empty() {
            output.push_str("- Issues:\n");
            for issue in &context.health_assessment.issues {
                output.push_str(&format!("  - {}\n", issue));
            }
        }
        
        if !context.health_assessment.warnings.is_empty() {
            output.push_str("- Warnings:\n");
            for warning in &context.health_assessment.warnings {
                output.push_str(&format!("  - {}\n", warning));
            }
        }
        output.push_str("\n");

        // Recommendations
        if !context.recommendations.is_empty() {
            output.push_str("## Recommendations\n");
            for (i, rec) in context.recommendations.iter().enumerate() {
                output.push_str(&format!("{}. **{}** ({:?} priority)\n", i + 1, rec.title, rec.priority));
                output.push_str(&format!("   - {}\n", rec.description));
                output.push_str(&format!("   - Action: {}\n", rec.action));
            }
            output.push_str("\n");
        }

        // Dependency Analysis
        output.push_str(&format!("## Dependency Analysis\n"));
        output.push_str(&format!("- Direct Dependencies: {}\n", context.dependency_analysis.direct_dependencies));
        output.push_str(&format!("- Transitive Dependencies: {}\n", context.dependency_analysis.transitive_dependencies));
        
        if !context.dependency_analysis.outdated_dependencies.is_empty() {
            output.push_str("- Outdated Dependencies:\n");
            for dep in &context.dependency_analysis.outdated_dependencies {
                output.push_str(&format!("  - {} ({}): {}\n", dep.name, dep.current_version, dep.reason));
            }
        }
        output.push_str("\n");

        // Conflict Analysis
        if !context.conflict_analysis.version_conflicts.is_empty() {
            output.push_str("## Version Conflicts\n");
            for conflict in &context.conflict_analysis.version_conflicts {
                output.push_str(&format!("- **{}**: {} ({}) vs {} ({})\n", 
                    conflict.dependency_name, conflict.scope1, conflict.version1, 
                    conflict.scope2, conflict.version2));
            }
            output.push_str("\n");
        }

        output
    }

    /// Create a context-aware prompt template for AI agents
    pub fn create_context_aware_prompt(task: &str, scope_path: Option<&str>) -> String {
        let mut prompt = format!("You are an AI agent tasked with: {}\n\n", task);
        
        if let Some(scope) = scope_path {
            prompt.push_str(&format!("Target scope: {}\n\n", scope));
        }
        
        prompt.push_str("Please analyze the provided lock file context and provide recommendations.\n");
        prompt.push_str("Consider:\n");
        prompt.push_str("1. Current dependency versions and constraints\n");
        prompt.push_str("2. Potential conflicts and circular dependencies\n");
        prompt.push_str("3. Security and performance implications\n");
        prompt.push_str("4. Best practices for dependency management\n\n");
        prompt.push_str("Provide specific, actionable recommendations with clear reasoning.\n");
        
        prompt
    }

    /// Extract key metrics from lock file context for AI decision making
    pub fn extract_key_metrics(context: &LockFileAIContext) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        
        metrics.insert("health_score".to_string(), context.health_assessment.overall_score);
        metrics.insert("total_scopes".to_string(), context.summary.total_scopes as f64);
        metrics.insert("total_dependencies".to_string(), context.summary.total_dependencies as f64);
        metrics.insert("circular_dependencies".to_string(), context.summary.circular_dependencies as f64);
        metrics.insert("direct_dependencies".to_string(), context.dependency_analysis.direct_dependencies as f64);
        metrics.insert("transitive_dependencies".to_string(), context.dependency_analysis.transitive_dependencies as f64);
        metrics.insert("outdated_dependencies".to_string(), context.dependency_analysis.outdated_dependencies.len() as f64);
        metrics.insert("security_concerns".to_string(), context.dependency_analysis.security_concerns.len() as f64);
        metrics.insert("version_conflicts".to_string(), context.conflict_analysis.version_conflicts.len() as f64);
        metrics.insert("recommendations_count".to_string(), context.recommendations.len() as f64);
        
        metrics
    }
} 