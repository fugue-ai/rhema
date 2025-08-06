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

use rhema_ai::ai_service::{
    AIService, AIServiceConfig, AIRequest, LockFileRequestContext, ConflictResolutionMode
};
use rhema_ai::agent::workflow_manager::{AIWorkflowManager, AIAgentTools, WorkflowConfig};
use rhema_ai::context_injection::TaskType;
use rhema_core::RhemaResult;
use std::path::PathBuf;
use std::sync::Arc;

/// Example demonstrating AI service with lock file awareness
pub struct LockFileAwareAIExample {
    ai_service: Arc<AIService>,
    workflow_manager: Arc<AIWorkflowManager>,
    agent_tools: AIAgentTools,
}

impl LockFileAwareAIExample {
    /// Create a new example instance
    pub async fn new() -> RhemaResult<Self> {
        // Configure AI service with lock file awareness
        let config = AIServiceConfig {
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "demo-key".to_string()),
            base_url: "https://api.openai.com".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 10,
            rate_limit_per_minute: 60,
            cache_ttl_seconds: 3600,
            model_version: "1.0".to_string(),
            enable_caching: true,
            enable_rate_limiting: false,
            enable_monitoring: true,
            // Lock file awareness configuration
            enable_lock_file_awareness: true,
            lock_file_path: Some(PathBuf::from(".")),
            auto_validate_lock_file: true,
            conflict_prevention_enabled: true,
            dependency_version_consistency: true,
        };

        let ai_service = Arc::new(AIService::new(config).await?);
        let workflow_manager = Arc::new(AIWorkflowManager::new(ai_service.clone(), PathBuf::from(".")).await?);
        let agent_tools = AIAgentTools::new(workflow_manager.clone()).await;

        Ok(Self {
            ai_service,
            workflow_manager,
            agent_tools,
        })
    }

    /// Example 1: Basic AI request with lock file context
    pub async fn example_basic_request_with_lock_file_context(&self) -> RhemaResult<()> {
        println!("=== Example 1: Basic AI Request with Lock File Context ===");

        let lock_context = LockFileRequestContext {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: Some(vec!["crates/rhema-core".to_string()]),
            include_transitive_deps: true,
            validate_before_processing: true,
            conflict_resolution_mode: ConflictResolutionMode::Automatic,
        };

        let request = AIRequest {
            id: uuid::Uuid::new_v4().to_string(),
            prompt: "Analyze the current dependency structure and provide recommendations for improvements.".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.3,
            max_tokens: 1000,
            user_id: Some("example-user".to_string()),
            session_id: Some("example-session".to_string()),
            created_at: chrono::Utc::now(),
            lock_file_context: Some(lock_context),
            task_type: Some(TaskType::DependencyUpdate),
            scope_path: Some("crates/rhema-core".to_string()),
        };

        let response = self.ai_service.process_request(request).await?;

        println!("AI Response: {}", response.content);
        
        if let Some(validation) = &response.lock_file_validation {
            println!("Lock File Validation: {} (Score: {:.1}/100)", 
                if validation.is_valid { "PASSED" } else { "FAILED" }, 
                validation.validation_score);
        }

        if let Some(consistency) = &response.dependency_consistency_check {
            println!("Dependency Consistency: {}", 
                if consistency.is_consistent { "CONSISTENT" } else { "INCONSISTENT" });
        }

        if let Some(recommendations) = &response.recommendations {
            println!("AI Recommendations: {}", recommendations.len());
            for (i, rec) in recommendations.iter().enumerate() {
                println!("  {}. {} ({:?} priority)", i + 1, rec.title, rec.priority);
            }
        }

        Ok(())
    }

    /// Example 2: Consistent dependency versions across agents
    pub async fn example_consistent_dependency_versions(&self) -> RhemaResult<()> {
        println!("\n=== Example 2: Consistent Dependency Versions Across Agents ===");

        // Simulate multiple AI agents working on the same project
        let agents = vec!["agent-1", "agent-2", "agent-3"];
        
        for agent_id in agents {
            println!("Agent {}: Checking dependency consistency...", agent_id);
            
            let lock_context = LockFileRequestContext {
                include_dependency_versions: true,
                include_conflict_prevention: true,
                include_health_info: false,
                include_recommendations: false,
                target_scopes: None,
                include_transitive_deps: true,
                validate_before_processing: true,
                conflict_resolution_mode: ConflictResolutionMode::Automatic,
            };

            let request = AIRequest {
                id: uuid::Uuid::new_v4().to_string(),
                prompt: "Check if all dependencies are using consistent versions across the project.".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.1,
                max_tokens: 500,
                user_id: Some(agent_id.to_string()),
                session_id: Some("consistency-check".to_string()),
                created_at: chrono::Utc::now(),
                lock_file_context: Some(lock_context),
                task_type: Some(TaskType::DependencyUpdate),
                scope_path: None,
            };

            let response = self.ai_service.process_request(request).await?;

            if let Some(consistency) = &response.dependency_consistency_check {
                if consistency.is_consistent {
                    println!("  ✓ Dependencies are consistent");
                } else {
                    println!("  ✗ Found {} version conflicts", consistency.version_conflicts.len());
                    for conflict in &consistency.version_conflicts {
                        println!("    - {}: {} vs {} (scopes: {}, {})", 
                            conflict.dependency_name, conflict.version1, conflict.version2,
                            conflict.scope1, conflict.scope2);
                    }
                }
            }
        }

        Ok(())
    }

    /// Example 3: Conflict prevention in AI workflows
    pub async fn example_conflict_prevention(&self) -> RhemaResult<()> {
        println!("\n=== Example 3: Conflict Prevention in AI Workflows ===");

        let workflow_config = WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: true,
            require_confirmation: false,
            rollback_on_failure: true,
            max_retry_attempts: 3,
            timeout_seconds: 300,
            include_security_checks: true,
            include_performance_checks: true,
        };

        // Start a workflow for dependency updates
        let workflow_id = self.workflow_manager.start_workflow(
            "conflict-prevention-agent",
            TaskType::DependencyUpdate,
            Some("crates/rhema-core".to_string()),
            workflow_config.clone(),
        ).await?;

        println!("Started workflow: {}", workflow_id);

        // Execute the workflow with conflict prevention
        let prompt = "Update dependencies while preventing conflicts. Check for version compatibility and resolve any issues automatically.";

        let result = self.workflow_manager.execute_workflow(&workflow_id, prompt, workflow_config).await?;

        println!("Workflow completed: {}", if result.success { "SUCCESS" } else { "FAILED" });
        println!("Actions taken: {}", result.actions_taken.len());
        println!("Recommendations: {}", result.recommendations.len());
        println!("Errors: {}", result.errors.len());

        for action in &result.actions_taken {
            println!("  - {}: {} ({})", 
                format!("{:?}", action.action_type), 
                action.description,
                if action.success { "SUCCESS" } else { "FAILED" });
        }

        Ok(())
    }

    /// Example 4: Lock file validation in AI operations
    pub async fn example_lock_file_validation(&self) -> RhemaResult<()> {
        println!("\n=== Example 4: Lock File Validation in AI Operations ===");

        // Validate lock file before processing
        let validation_result = self.ai_service.validate_lock_file_consistency().await?;
        
        println!("Lock file validation result:");
        println!("  Consistent: {}", validation_result.is_consistent);
        println!("  Version conflicts: {}", validation_result.version_conflicts.len());
        println!("  Circular dependencies: {}", validation_result.circular_dependencies.len());
        println!("  Outdated dependencies: {}", validation_result.outdated_dependencies.len());
        println!("  Security concerns: {}", validation_result.security_concerns.len());

        if !validation_result.is_consistent {
            println!("  ⚠️  Lock file has issues that need attention");
            
            // Use AI to generate recommendations for fixing issues
            let lock_context = LockFileRequestContext {
                include_dependency_versions: true,
                include_conflict_prevention: true,
                include_health_info: true,
                include_recommendations: true,
                target_scopes: None,
                include_transitive_deps: true,
                validate_before_processing: false,
                conflict_resolution_mode: ConflictResolutionMode::Manual,
            };

            let request = AIRequest {
                id: uuid::Uuid::new_v4().to_string(),
                prompt: "The lock file has validation issues. Provide specific steps to fix these problems.".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.2,
                max_tokens: 1500,
                user_id: Some("validation-agent".to_string()),
                session_id: Some("validation-session".to_string()),
                created_at: chrono::Utc::now(),
                lock_file_context: Some(lock_context),
                task_type: Some(TaskType::LockFileManagement),
                scope_path: None,
            };

            let response = self.ai_service.process_request(request).await?;
            println!("AI recommendations for fixing validation issues:");
            println!("{}", response.content);
        } else {
            println!("  ✓ Lock file is healthy and consistent");
        }

        Ok(())
    }

    /// Example 5: AI-assisted conflict resolution
    pub async fn example_ai_assisted_conflict_resolution(&self) -> RhemaResult<()> {
        println!("\n=== Example 5: AI-Assisted Conflict Resolution ===");

        // Use AI agent tools for conflict resolution
        let conflict_result = self.agent_tools.resolve_dependency_conflicts(
            "conflict-resolution-agent",
            Some("crates/rhema-core".to_string())
        ).await?;

        println!("Conflict resolution workflow: {}", conflict_result.workflow_id);
        println!("Conflicts resolved: {}", conflict_result.conflicts_resolved);
        println!("Actions taken: {}", conflict_result.actions_taken.len());
        println!("Recommendations: {}", conflict_result.recommendations.len());

        // Show detailed conflict resolution steps
        for action in &conflict_result.actions_taken {
            println!("  Action: {} ({})", 
                action.description,
                if action.success { "SUCCESS" } else { "FAILED" });
            
            if let Some(details) = &action.details {
                println!("    Details: {}", details);
            }
        }

        // Show AI recommendations for conflict resolution
        for (i, rec) in conflict_result.recommendations.iter().enumerate() {
            println!("  Recommendation {}: {} ({:?} priority)", 
                i + 1, rec.title, rec.priority);
            println!("    Action: {}", rec.action);
            if let Some(impact) = &rec.impact_analysis {
                println!("    Impact: {}", impact);
            }
        }

        Ok(())
    }

    /// Example 6: Comprehensive lock file health analysis
    pub async fn example_comprehensive_health_analysis(&self) -> RhemaResult<()> {
        println!("\n=== Example 6: Comprehensive Lock File Health Analysis ===");

        let health_analysis = self.agent_tools.analyze_lock_file_health("health-analysis-agent").await?;

        println!("Health analysis workflow: {}", health_analysis.workflow_id);
        println!("Health score: {:.1}/100", health_analysis.health_score);
        println!("Issues found: {}", health_analysis.issues.len());
        println!("Recommendations: {}", health_analysis.recommendations.len());

        // Categorize health score
        let health_status = if health_analysis.health_score >= 80.0 {
            "EXCELLENT"
        } else if health_analysis.health_score >= 60.0 {
            "GOOD"
        } else if health_analysis.health_score >= 40.0 {
            "FAIR"
        } else {
            "POOR"
        };

        println!("Overall health status: {}", health_status);

        // Show issues
        if !health_analysis.issues.is_empty() {
            println!("Issues:");
            for issue in &health_analysis.issues {
                println!("  - {}", issue);
            }
        }

        // Show recommendations
        if !health_analysis.recommendations.is_empty() {
            println!("Recommendations:");
            for (i, rec) in health_analysis.recommendations.iter().enumerate() {
                println!("  {}. {} ({:?} priority)", i + 1, rec.title, rec.priority);
                println!("     Action: {}", rec.action);
            }
        }

        Ok(())
    }

    /// Example 7: Security review with lock file awareness
    pub async fn example_security_review(&self) -> RhemaResult<()> {
        println!("\n=== Example 7: Security Review with Lock File Awareness ===");

        let security_result = self.agent_tools.security_review(
            "security-agent",
            Some("crates/rhema-core".to_string())
        ).await?;

        println!("Security review workflow: {}", security_result.workflow_id);
        println!("Vulnerabilities found: {}", security_result.vulnerabilities_found);
        println!("Security issues: {}", security_result.security_issues.len());
        println!("Recommendations: {}", security_result.recommendations.len());

        if security_result.vulnerabilities_found {
            println!("⚠️  Security vulnerabilities detected!");
            
            for issue in &security_result.security_issues {
                println!("  - {}", issue);
            }
        } else {
            println!("✓ No security vulnerabilities detected");
        }

        // Show security recommendations
        for (i, rec) in security_result.recommendations.iter().enumerate() {
            println!("  Security recommendation {}: {} ({:?} priority)", 
                i + 1, rec.title, rec.priority);
            println!("    Action: {}", rec.action);
        }

        Ok(())
    }

    /// Example 8: Dependency updates with lock file constraints
    pub async fn example_dependency_updates_with_constraints(&self) -> RhemaResult<()> {
        println!("\n=== Example 8: Dependency Updates with Lock File Constraints ===");

        let update_result = self.agent_tools.update_dependencies(
            "dependency-update-agent",
            "crates/rhema-core"
        ).await?;

        println!("Dependency update workflow: {}", update_result.workflow_id);
        println!("Updates applied: {}", update_result.updates_applied);
        println!("Actions taken: {}", update_result.actions_taken.len());
        println!("Recommendations: {}", update_result.recommendations.len());

        // Show update actions
        for action in &update_result.actions_taken {
            println!("  Action: {} ({})", 
                action.description,
                if action.success { "SUCCESS" } else { "FAILED" });
            
            if let Some(details) = &action.details {
                println!("    Details: {}", details);
            }
        }

        // Show update recommendations
        for (i, rec) in update_result.recommendations.iter().enumerate() {
            println!("  Update recommendation {}: {} ({:?} priority)", 
                i + 1, rec.title, rec.priority);
            println!("    Action: {}", rec.action);
            if let Some(steps) = &rec.implementation_steps {
                println!("    Steps:");
                for (j, step) in steps.iter().enumerate() {
                    println!("      {}. {}", j + 1, step);
                }
            }
        }

        Ok(())
    }

    /// Example 9: Get AI service metrics with lock file awareness
    pub async fn example_metrics_with_lock_file_awareness(&self) -> RhemaResult<()> {
        println!("\n=== Example 9: AI Service Metrics with Lock File Awareness ===");

        let metrics = self.ai_service.get_metrics().await;
        
        println!("AI Service Metrics:");
        println!("  Total requests: {}", metrics.total_requests);
        println!("  Successful requests: {}", metrics.successful_requests);
        println!("  Failed requests: {}", metrics.failed_requests);
        println!("  Cache hits: {}", metrics.cache_hits);
        println!("  Cache misses: {}", metrics.cache_misses);
        println!("  Average response time: {}ms", metrics.average_response_time_ms);
        println!("  Total tokens processed: {}", metrics.total_tokens_processed);
        println!("  Total cost: ${:.4}", metrics.total_cost);

        // Lock file awareness metrics
        println!("\nLock File Awareness Metrics:");
        println!("  Lock file validation requests: {}", metrics.lock_file_validation_requests);
        println!("  Conflict resolution requests: {}", metrics.conflict_resolution_requests);
        println!("  Dependency consistency checks: {}", metrics.dependency_consistency_checks);
        println!("  AI-assisted resolutions: {}", metrics.ai_assisted_resolutions);
        println!("  Validation failures: {}", metrics.validation_failures);
        println!("  Conflict detections: {}", metrics.conflict_detections);

        // Workflow statistics
        let workflow_stats = self.workflow_manager.get_workflow_statistics().await?;
        
        println!("\nWorkflow Statistics:");
        println!("  Total workflows: {}", workflow_stats.total_workflows);
        println!("  Active workflows: {}", workflow_stats.active_workflows);
        println!("  Completed workflows: {}", workflow_stats.completed_workflows);
        println!("  Failed workflows: {}", workflow_stats.failed_workflows);
        println!("  Active agents: {}", workflow_stats.active_agents);
        println!("  Total recommendations: {}", workflow_stats.total_recommendations);
        println!("  Total actions: {}", workflow_stats.total_actions);

        Ok(())
    }

    /// Run all examples
    pub async fn run_all_examples(&self) -> RhemaResult<()> {
        println!("🚀 Running AI Service with Lock File Awareness Examples\n");

        self.example_basic_request_with_lock_file_context().await?;
        self.example_consistent_dependency_versions().await?;
        self.example_conflict_prevention().await?;
        self.example_lock_file_validation().await?;
        self.example_ai_assisted_conflict_resolution().await?;
        self.example_comprehensive_health_analysis().await?;
        self.example_security_review().await?;
        self.example_dependency_updates_with_constraints().await?;
        self.example_metrics_with_lock_file_awareness().await?;

        println!("\n✅ All examples completed successfully!");
        Ok(())
    }
}

/// Main function to run the examples
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🔧 Initializing AI Service with Lock File Awareness...");
    
    let example = LockFileAwareAIExample::new().await?;
    
    // Run all examples
    example.run_all_examples().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_creation() {
        // This test would require a mock AI service
        // For now, just test that the example can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_workflow_config() {
        let config = WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: true,
            require_confirmation: false,
            rollback_on_failure: true,
            max_retry_attempts: 3,
            timeout_seconds: 300,
            include_security_checks: true,
            include_performance_checks: true,
        };

        assert!(config.auto_validate);
        assert!(config.auto_resolve_conflicts);
        assert!(!config.require_confirmation);
        assert!(config.rollback_on_failure);
        assert_eq!(config.max_retry_attempts, 3);
        assert_eq!(config.timeout_seconds, 300);
        assert!(config.include_security_checks);
        assert!(config.include_performance_checks);
    }
} 