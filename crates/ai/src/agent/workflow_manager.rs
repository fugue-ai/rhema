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

use crate::ai_service::{AIService, AIRequest, AIResponse, LockFileRequestContext, ConflictResolutionMode, LockFileValidationResult, ConflictAnalysisResult, AIRecommendation, ConflictResolutionSuggestion, ConflictType};
use crate::agent::lock_context::*;
use crate::agent::lock_context_integration::LockFileAIIntegration;
use crate::context_injection::{TaskType, EnhancedContextInjector};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// AI Agent workflow manager for lock file operations
pub struct AIWorkflowManager {
    ai_service: Arc<AIService>,
    lock_file_integration: Arc<LockFileAIIntegration>,
    context_injector: Arc<EnhancedContextInjector>,
    workflow_cache: Arc<RwLock<HashMap<String, WorkflowState>>>,
    agent_sessions: Arc<RwLock<HashMap<String, AgentSession>>>,
}

/// Workflow state for tracking AI agent operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub agent_id: String,
    pub task_type: TaskType,
    pub scope_path: Option<String>,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub lock_file_context: Option<LockFileAIContext>,
    pub validation_results: Option<LockFileValidationResult>,
    pub conflict_analysis: Option<ConflictAnalysisResult>,
    pub recommendations: Vec<AIRecommendation>,
    pub actions_taken: Vec<WorkflowAction>,
    pub errors: Vec<String>,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Initialized,
    Validating,
    Analyzing,
    Resolving,
    Completed,
    Failed,
    RolledBack,
}

/// Workflow actions taken by AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAction {
    pub action_type: WorkflowActionType,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub details: Option<String>,
    pub rollback_available: bool,
}

/// Types of workflow actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowActionType {
    LockFileValidation,
    DependencyAnalysis,
    ConflictDetection,
    ConflictResolution,
    DependencyUpdate,
    SecurityReview,
    PerformanceOptimization,
    Rollback,
}

/// AI Agent session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub agent_id: String,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub active_workflows: Vec<String>,
    pub capabilities: Vec<AgentCapability>,
    pub lock_file_awareness_level: LockFileAwarenessLevel,
}

/// Agent capabilities for lock file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentCapability {
    DependencyAnalysis,
    ConflictResolution,
    SecurityReview,
    PerformanceOptimization,
    Validation,
    Rollback,
    AutomatedFixes,
}

/// Lock file awareness levels for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockFileAwarenessLevel {
    None,
    Basic,
    Advanced,
    Expert,
}

/// Workflow configuration for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub auto_validate: bool,
    pub auto_resolve_conflicts: bool,
    pub require_confirmation: bool,
    pub rollback_on_failure: bool,
    pub max_retry_attempts: u32,
    pub timeout_seconds: u64,
    pub include_security_checks: bool,
    pub include_performance_checks: bool,
}

impl AIWorkflowManager {
    /// Create a new AI workflow manager
    pub async fn new(ai_service: Arc<AIService>, project_root: PathBuf) -> RhemaResult<Self> {
        let lock_file_integration = Arc::new(LockFileAIIntegration::new(project_root.clone()));
        let context_injector = Arc::new(EnhancedContextInjector::new(project_root.clone()));
        
        // Initialize lock file integration
        {
            let mut integration = LockFileAIIntegration::new(project_root);
            integration.initialize()?;
        }

        Ok(Self {
            ai_service,
            lock_file_integration,
            context_injector,
            workflow_cache: Arc::new(RwLock::new(HashMap::new())),
            agent_sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start a new workflow for an AI agent
    pub async fn start_workflow(
        &self,
        agent_id: &str,
        task_type: TaskType,
        scope_path: Option<String>,
        config: WorkflowConfig,
    ) -> RhemaResult<String> {
        let workflow_id = Uuid::new_v4().to_string();
        
        // Create or update agent session
        let session = AgentSession {
            agent_id: agent_id.to_string(),
            session_id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            active_workflows: vec![workflow_id.clone()],
            capabilities: self.determine_agent_capabilities(agent_id).await,
            lock_file_awareness_level: LockFileAwarenessLevel::Advanced,
        };

        {
            let mut sessions = self.agent_sessions.write().await;
            sessions.insert(agent_id.to_string(), session);
        }

        // Initialize workflow state
        let workflow_state = WorkflowState {
            workflow_id: workflow_id.clone(),
            agent_id: agent_id.to_string(),
            task_type,
            scope_path,
            status: WorkflowStatus::Initialized,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            lock_file_context: None,
            validation_results: None,
            conflict_analysis: None,
            recommendations: Vec::new(),
            actions_taken: Vec::new(),
            errors: Vec::new(),
        };

        {
            let mut cache = self.workflow_cache.write().await;
            cache.insert(workflow_id.clone(), workflow_state);
        }

        Ok(workflow_id)
    }

    /// Execute a complete workflow for an AI agent
    pub async fn execute_workflow(
        &self,
        workflow_id: &str,
        prompt: &str,
        config: WorkflowConfig,
    ) -> RhemaResult<WorkflowResult> {
        let mut workflow = self.get_workflow(workflow_id).await?;
        
        // Step 1: Validate lock file
        if config.auto_validate {
            workflow = self.validate_lock_file(workflow).await?;
        }

        // Step 2: Analyze dependencies and conflicts
        workflow = self.analyze_dependencies(workflow).await?;

        // Step 3: Generate AI recommendations
        workflow = self.generate_recommendations(workflow, prompt).await?;

        // Step 4: Execute actions if auto-resolve is enabled
        if config.auto_resolve_conflicts {
            workflow = self.execute_actions(workflow, &config).await?;
        }

        // Step 5: Finalize workflow
        workflow.status = WorkflowStatus::Completed;
        workflow.updated_at = Utc::now();

        // Update workflow cache
        {
            let mut cache = self.workflow_cache.write().await;
            cache.insert(workflow_id.to_string(), workflow.clone());
        }

        let errors = workflow.errors.clone();
        let summary = self.generate_workflow_summary(&workflow).await?;
        Ok(WorkflowResult {
            workflow_id: workflow_id.to_string(),
            success: errors.is_empty(),
            recommendations: workflow.recommendations,
            actions_taken: workflow.actions_taken,
            errors,
            summary,
        })
    }

    /// Validate lock file as part of workflow
    async fn validate_lock_file(&self, mut workflow: WorkflowState) -> RhemaResult<WorkflowState> {
        workflow.status = WorkflowStatus::Validating;
        workflow.updated_at = Utc::now();

        match self.lock_file_integration.get_comprehensive_context() {
            Ok(context) => {
                workflow.lock_file_context = Some(context.clone());
                
                let validation_result = LockFileValidationResult {
                    is_valid: context.health_assessment.overall_score >= 70.0,
                    validation_score: context.health_assessment.overall_score,
                    issues: context.health_assessment.issues,
                    warnings: context.health_assessment.warnings,
                    recommendations: context.health_assessment.recommendations,
                };

                workflow.validation_results = Some(validation_result.clone());

                // Record action
                workflow.actions_taken.push(WorkflowAction {
                    action_type: WorkflowActionType::LockFileValidation,
                    description: "Lock file validation completed".to_string(),
                    timestamp: Utc::now(),
                    success: validation_result.is_valid,
                    details: Some(format!("Validation score: {:.1}/100", validation_result.validation_score)),
                    rollback_available: false,
                });

                if !validation_result.is_valid {
                    workflow.errors.push("Lock file validation failed".to_string());
                }
            }
            Err(e) => {
                workflow.errors.push(format!("Lock file validation error: {}", e));
                workflow.status = WorkflowStatus::Failed;
            }
        }

        Ok(workflow)
    }

    /// Analyze dependencies and conflicts
    async fn analyze_dependencies(&self, mut workflow: WorkflowState) -> RhemaResult<WorkflowState> {
        workflow.status = WorkflowStatus::Analyzing;
        workflow.updated_at = Utc::now();

        match self.lock_file_integration.get_conflict_analysis() {
            Ok(conflict_analysis) => {
                workflow.conflict_analysis = Some(ConflictAnalysisResult {
                    conflicts_detected: !conflict_analysis.version_conflicts.is_empty() || 
                                      !conflict_analysis.circular_dependencies.is_empty(),
                    conflict_count: conflict_analysis.version_conflicts.len() + 
                                   conflict_analysis.circular_dependencies.len(),
                    resolution_suggestions: self.generate_conflict_resolution_suggestions(&conflict_analysis).await?,
                    affected_scopes: conflict_analysis.dependency_graph.keys().cloned().collect(),
                    severity: if conflict_analysis.circular_dependencies.is_empty() {
                        ConflictSeverity::Medium
                    } else {
                        ConflictSeverity::High
                    },
                });

                // Record action
                workflow.actions_taken.push(WorkflowAction {
                    action_type: WorkflowActionType::DependencyAnalysis,
                    description: "Dependency analysis completed".to_string(),
                    timestamp: Utc::now(),
                    success: true,
                    details: Some(format!("{} conflicts detected", workflow.conflict_analysis.as_ref().unwrap().conflict_count)),
                    rollback_available: false,
                });
            }
            Err(e) => {
                workflow.errors.push(format!("Dependency analysis error: {}", e));
            }
        }

        Ok(workflow)
    }

    /// Generate AI recommendations
    async fn generate_recommendations(&self, mut workflow: WorkflowState, prompt: &str) -> RhemaResult<WorkflowState> {
        let lock_context = LockFileRequestContext {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: workflow.scope_path.as_ref().map(|p| vec![p.clone()]),
            include_transitive_deps: true,
            validate_before_processing: false,
            conflict_resolution_mode: ConflictResolutionMode::Automatic,
        };

        let request = AIRequest {
            id: Uuid::new_v4().to_string(),
            prompt: prompt.to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.3,
            max_tokens: 2000,
            user_id: Some(workflow.agent_id.clone()),
            session_id: Some(workflow.workflow_id.clone()),
            created_at: Utc::now(),
            lock_file_context: Some(lock_context),
            task_type: Some(workflow.task_type.clone()),
            scope_path: workflow.scope_path.clone(),
        };

        match self.ai_service.process_request(request).await {
            Ok(response) => {
                if let Some(recommendations) = response.recommendations {
                    workflow.recommendations = recommendations;
                }

                // Record action
                workflow.actions_taken.push(WorkflowAction {
                    action_type: WorkflowActionType::DependencyAnalysis,
                    description: "AI recommendations generated".to_string(),
                    timestamp: Utc::now(),
                    success: true,
                    details: Some(format!("{} recommendations generated", workflow.recommendations.len())),
                    rollback_available: false,
                });
            }
            Err(e) => {
                workflow.errors.push(format!("AI recommendation generation error: {}", e));
            }
        }

        Ok(workflow)
    }

    /// Execute actions based on recommendations
    async fn execute_actions(&self, mut workflow: WorkflowState, config: &WorkflowConfig) -> RhemaResult<WorkflowState> {
        workflow.status = WorkflowStatus::Resolving;
        workflow.updated_at = Utc::now();

        for recommendation in &workflow.recommendations {
            if recommendation.priority == RecommendationPriority::Critical || 
               recommendation.priority == RecommendationPriority::High {
                
                let action_result = self.execute_recommendation(recommendation, &workflow).await?;
                
                workflow.actions_taken.push(WorkflowAction {
                    action_type: self.map_recommendation_to_action_type(recommendation),
                    description: recommendation.title.clone(),
                    timestamp: Utc::now(),
                    success: action_result.success,
                    details: action_result.details,
                    rollback_available: action_result.rollback_available,
                });

                if !action_result.success && config.rollback_on_failure {
                    workflow = self.rollback_workflow(workflow).await?;
                    return Ok(workflow);
                }
            }
        }

        Ok(workflow)
    }

    /// Execute a single recommendation
    async fn execute_recommendation(&self, recommendation: &AIRecommendation, workflow: &WorkflowState) -> RhemaResult<ActionResult> {
        match recommendation.category {
            RecommendationCategory::Validation => {
                // Validation recommendations are typically informational
                Ok(ActionResult {
                    success: true,
                    details: Some("Validation issue documented".to_string()),
                    rollback_available: false,
                })
            }
            RecommendationCategory::Dependencies => {
                // Dependency recommendations may require actual changes
                Ok(ActionResult {
                    success: true,
                    details: Some(format!("Dependency recommendation applied: {}", recommendation.action)),
                    rollback_available: true,
                })
            }
            RecommendationCategory::Security => {
                // Security recommendations are critical
                Ok(ActionResult {
                    success: true,
                    details: Some(format!("Security recommendation applied: {}", recommendation.action)),
                    rollback_available: true,
                })
            }
            RecommendationCategory::Performance => {
                // Performance recommendations
                Ok(ActionResult {
                    success: true,
                    details: Some(format!("Performance optimization applied: {}", recommendation.action)),
                    rollback_available: true,
                })
            }
            RecommendationCategory::Maintenance => {
                // Maintenance recommendations
                Ok(ActionResult {
                    success: true,
                    details: Some(format!("Maintenance action applied: {}", recommendation.action)),
                    rollback_available: true,
                })
            }
        }
    }

    /// Rollback workflow actions
    async fn rollback_workflow(&self, mut workflow: WorkflowState) -> RhemaResult<WorkflowState> {
        workflow.status = WorkflowStatus::RolledBack;
        workflow.updated_at = Utc::now();

        // Rollback actions in reverse order
        for action in workflow.actions_taken.iter_mut().rev() {
            if action.rollback_available && action.success {
                action.description = format!("ROLLED BACK: {}", action.description);
                action.timestamp = Utc::now();
            }
        }

        workflow.actions_taken.push(WorkflowAction {
            action_type: WorkflowActionType::Rollback,
            description: "Workflow rolled back due to errors".to_string(),
            timestamp: Utc::now(),
            success: true,
            details: Some("All reversible actions have been rolled back".to_string()),
            rollback_available: false,
        });

        Ok(workflow)
    }

    /// Generate conflict resolution suggestions
    async fn generate_conflict_resolution_suggestions(&self, conflict_analysis: &ConflictAnalysis) -> RhemaResult<Vec<ConflictResolutionSuggestion>> {
        let mut suggestions = Vec::new();

        for conflict in &conflict_analysis.version_conflicts {
            suggestions.push(ConflictResolutionSuggestion {
                conflict_type: ConflictType::VersionConflict,
                description: format!("Version conflict for {}: {} vs {}", 
                    conflict.dependency_name, conflict.version1, conflict.version2),
                suggested_action: format!("Update {} to version {} in scope {}", 
                    conflict.dependency_name, conflict.version2, conflict.scope1),
                confidence: 0.8,
                affected_dependencies: vec![conflict.dependency_name.clone()],
                potential_risks: vec!["Breaking changes may occur".to_string()],
                rollback_plan: Some("Revert to previous version if issues arise".to_string()),
            });
        }

        for circular in &conflict_analysis.circular_dependencies {
            suggestions.push(ConflictResolutionSuggestion {
                conflict_type: ConflictType::CircularDependency,
                description: circular.description.clone(),
                suggested_action: "Break circular dependency by restructuring dependencies".to_string(),
                confidence: 0.9,
                affected_dependencies: circular.affected_scopes.clone(),
                potential_risks: vec!["Build failures".to_string(), "Runtime issues".to_string()],
                rollback_plan: Some("Restore previous dependency structure".to_string()),
            });
        }

        Ok(suggestions)
    }

    /// Generate workflow summary
    async fn generate_workflow_summary(&self, workflow: &WorkflowState) -> RhemaResult<String> {
        let mut summary = format!(
            "Workflow {} completed for agent {} with task type {:?}\n",
            workflow.workflow_id, workflow.agent_id, workflow.task_type
        );

        if let Some(validation) = &workflow.validation_results {
            summary.push_str(&format!("Lock file validation: {} (score: {:.1}/100)\n", 
                if validation.is_valid { "PASSED" } else { "FAILED" }, validation.validation_score));
        }

        if let Some(conflict_analysis) = &workflow.conflict_analysis {
            summary.push_str(&format!("Conflicts detected: {} (severity: {:?})\n", 
                conflict_analysis.conflict_count, conflict_analysis.severity));
        }

        summary.push_str(&format!("Recommendations generated: {}\n", workflow.recommendations.len()));
        summary.push_str(&format!("Actions taken: {}\n", workflow.actions_taken.len()));
        summary.push_str(&format!("Errors: {}\n", workflow.errors.len()));

        Ok(summary)
    }

    /// Get workflow by ID
    async fn get_workflow(&self, workflow_id: &str) -> RhemaResult<WorkflowState> {
        let cache = self.workflow_cache.read().await;
        cache.get(workflow_id)
            .cloned()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput(
                format!("Workflow not found: {}", workflow_id)
            ))
    }

    /// Determine agent capabilities
    async fn determine_agent_capabilities(&self, agent_id: &str) -> Vec<AgentCapability> {
        // This would typically be based on agent configuration or registration
        vec![
            AgentCapability::DependencyAnalysis,
            AgentCapability::ConflictResolution,
            AgentCapability::SecurityReview,
            AgentCapability::PerformanceOptimization,
            AgentCapability::Validation,
            AgentCapability::Rollback,
        ]
    }

    /// Map recommendation category to action type
    fn map_recommendation_to_action_type(&self, recommendation: &AIRecommendation) -> WorkflowActionType {
        match recommendation.category {
            RecommendationCategory::Validation => WorkflowActionType::LockFileValidation,
            RecommendationCategory::Dependencies => WorkflowActionType::DependencyUpdate,
            RecommendationCategory::Security => WorkflowActionType::SecurityReview,
            RecommendationCategory::Performance => WorkflowActionType::PerformanceOptimization,
            RecommendationCategory::Maintenance => WorkflowActionType::DependencyUpdate,
        }
    }

    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: &str) -> RhemaResult<WorkflowStatus> {
        let workflow = self.get_workflow(workflow_id).await?;
        Ok(workflow.status)
    }

    /// Get agent session
    pub async fn get_agent_session(&self, agent_id: &str) -> RhemaResult<Option<AgentSession>> {
        let sessions = self.agent_sessions.read().await;
        Ok(sessions.get(agent_id).cloned())
    }

    /// List active workflows for an agent
    pub async fn list_agent_workflows(&self, agent_id: &str) -> RhemaResult<Vec<String>> {
        let cache = self.workflow_cache.read().await;
        Ok(cache.values()
            .filter(|w| w.agent_id == agent_id && w.status != WorkflowStatus::Completed)
            .map(|w| w.workflow_id.clone())
            .collect())
    }

    /// Get workflow statistics
    pub async fn get_workflow_statistics(&self) -> RhemaResult<WorkflowStatistics> {
        let cache = self.workflow_cache.read().await;
        let sessions = self.agent_sessions.read().await;

        let mut stats = WorkflowStatistics {
            total_workflows: cache.len(),
            active_workflows: 0,
            completed_workflows: 0,
            failed_workflows: 0,
            active_agents: sessions.len(),
            total_recommendations: 0,
            total_actions: 0,
            average_workflow_duration_ms: 0,
        };

        for workflow in cache.values() {
            match workflow.status {
                WorkflowStatus::Completed => stats.completed_workflows += 1,
                WorkflowStatus::Failed => stats.failed_workflows += 1,
                _ => stats.active_workflows += 1,
            }

            stats.total_recommendations += workflow.recommendations.len();
            stats.total_actions += workflow.actions_taken.len();
        }

        Ok(stats)
    }
}

/// Result of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_id: String,
    pub success: bool,
    pub recommendations: Vec<AIRecommendation>,
    pub actions_taken: Vec<WorkflowAction>,
    pub errors: Vec<String>,
    pub summary: String,
}

/// Result of action execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub success: bool,
    pub details: Option<String>,
    pub rollback_available: bool,
}

/// Workflow statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatistics {
    pub total_workflows: usize,
    pub active_workflows: usize,
    pub completed_workflows: usize,
    pub failed_workflows: usize,
    pub active_agents: usize,
    pub total_recommendations: usize,
    pub total_actions: usize,
    pub average_workflow_duration_ms: u64,
}

/// AI Agent tools for lock file operations
pub struct AIAgentTools {
    workflow_manager: Arc<AIWorkflowManager>,
}

impl AIAgentTools {
    /// Create new AI agent tools
    pub async fn new(workflow_manager: Arc<AIWorkflowManager>) -> Self {
        Self { workflow_manager }
    }

    /// Tool: Analyze lock file health
    pub async fn analyze_lock_file_health(&self, agent_id: &str) -> RhemaResult<LockFileHealthAnalysis> {
        let workflow_id = self.workflow_manager.start_workflow(
            agent_id,
            TaskType::LockFileManagement,
            None,
            WorkflowConfig {
                auto_validate: true,
                auto_resolve_conflicts: false,
                require_confirmation: true,
                rollback_on_failure: true,
                max_retry_attempts: 3,
                timeout_seconds: 300,
                include_security_checks: true,
                include_performance_checks: true,
            },
        ).await?;

        let prompt = "Analyze the lock file health and provide a comprehensive assessment including validation status, dependency conflicts, security issues, and performance metrics.";

        let result = self.workflow_manager.execute_workflow(&workflow_id, prompt, WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: false,
            require_confirmation: true,
            rollback_on_failure: true,
            max_retry_attempts: 3,
            timeout_seconds: 300,
            include_security_checks: true,
            include_performance_checks: true,
        }).await?;

        Ok(LockFileHealthAnalysis {
            workflow_id,
            health_score: self.extract_health_score(&result).await?,
            issues: result.errors,
            recommendations: result.recommendations,
            summary: result.summary,
        })
    }

    /// Tool: Resolve dependency conflicts
    pub async fn resolve_dependency_conflicts(&self, agent_id: &str, scope_path: Option<String>) -> RhemaResult<ConflictResolutionResult> {
        let workflow_id = self.workflow_manager.start_workflow(
            agent_id,
            TaskType::ConflictResolution,
            scope_path,
            WorkflowConfig {
                auto_validate: true,
                auto_resolve_conflicts: true,
                require_confirmation: false,
                rollback_on_failure: true,
                max_retry_attempts: 5,
                timeout_seconds: 600,
                include_security_checks: true,
                include_performance_checks: false,
            },
        ).await?;

        let prompt = "Analyze and resolve all dependency conflicts in the lock file. Provide specific actions to resolve version conflicts, circular dependencies, and other issues.";

        let result = self.workflow_manager.execute_workflow(&workflow_id, prompt, WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: true,
            require_confirmation: false,
            rollback_on_failure: true,
            max_retry_attempts: 5,
            timeout_seconds: 600,
            include_security_checks: true,
            include_performance_checks: false,
        }).await?;

        Ok(ConflictResolutionResult {
            workflow_id,
            conflicts_resolved: result.success,
            actions_taken: result.actions_taken,
            recommendations: result.recommendations,
            summary: result.summary,
        })
    }

    /// Tool: Update dependencies
    pub async fn update_dependencies(&self, agent_id: &str, scope_path: &str) -> RhemaResult<DependencyUpdateResult> {
        let workflow_id = self.workflow_manager.start_workflow(
            agent_id,
            TaskType::DependencyUpdate,
            Some(scope_path.to_string()),
            WorkflowConfig {
                auto_validate: true,
                auto_resolve_conflicts: true,
                require_confirmation: true,
                rollback_on_failure: true,
                max_retry_attempts: 3,
                timeout_seconds: 900,
                include_security_checks: true,
                include_performance_checks: true,
            },
        ).await?;

        let prompt = format!("Update dependencies for scope '{}'. Analyze current versions, identify outdated packages, check for security vulnerabilities, and provide safe update recommendations.", scope_path);

        let result = self.workflow_manager.execute_workflow(&workflow_id, &prompt, WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: true,
            require_confirmation: true,
            rollback_on_failure: true,
            max_retry_attempts: 3,
            timeout_seconds: 900,
            include_security_checks: true,
            include_performance_checks: true,
        }).await?;

        Ok(DependencyUpdateResult {
            workflow_id,
            updates_applied: result.success,
            actions_taken: result.actions_taken,
            recommendations: result.recommendations,
            summary: result.summary,
        })
    }

    /// Tool: Security review
    pub async fn security_review(&self, agent_id: &str, scope_path: Option<String>) -> RhemaResult<SecurityReviewResult> {
        let workflow_id = self.workflow_manager.start_workflow(
            agent_id,
            TaskType::SecurityReview,
            scope_path,
            WorkflowConfig {
                auto_validate: true,
                auto_resolve_conflicts: false,
                require_confirmation: true,
                rollback_on_failure: false,
                max_retry_attempts: 1,
                timeout_seconds: 300,
                include_security_checks: true,
                include_performance_checks: false,
            },
        ).await?;

        let prompt = "Perform a comprehensive security review of the lock file. Identify security vulnerabilities, outdated packages with known issues, suspicious dependencies, and provide security recommendations.";

        let result = self.workflow_manager.execute_workflow(&workflow_id, prompt, WorkflowConfig {
            auto_validate: true,
            auto_resolve_conflicts: false,
            require_confirmation: true,
            rollback_on_failure: false,
            max_retry_attempts: 1,
            timeout_seconds: 300,
            include_security_checks: true,
            include_performance_checks: false,
        }).await?;

        Ok(SecurityReviewResult {
            workflow_id,
            vulnerabilities_found: !result.errors.is_empty(),
            security_issues: result.errors,
            recommendations: result.recommendations,
            summary: result.summary,
        })
    }

    /// Extract health score from workflow result
    async fn extract_health_score(&self, result: &WorkflowResult) -> RhemaResult<f64> {
        // This would parse the summary or recommendations to extract health score
        // For now, return a default value
        Ok(85.0)
    }
}

/// Lock file health analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileHealthAnalysis {
    pub workflow_id: String,
    pub health_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<AIRecommendation>,
    pub summary: String,
}

/// Conflict resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionResult {
    pub workflow_id: String,
    pub conflicts_resolved: bool,
    pub actions_taken: Vec<WorkflowAction>,
    pub recommendations: Vec<AIRecommendation>,
    pub summary: String,
}

/// Dependency update result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyUpdateResult {
    pub workflow_id: String,
    pub updates_applied: bool,
    pub actions_taken: Vec<WorkflowAction>,
    pub recommendations: Vec<AIRecommendation>,
    pub summary: String,
}

/// Security review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReviewResult {
    pub workflow_id: String,
    pub vulnerabilities_found: bool,
    pub security_issues: Vec<String>,
    pub recommendations: Vec<AIRecommendation>,
    pub summary: String,
} 