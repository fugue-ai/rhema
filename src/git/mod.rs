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

pub mod hooks;
pub mod branch;
pub mod workflow;
pub mod history;
pub mod automation;
pub mod security;
pub mod monitoring;

use crate::{RhemaError, RhemaResult};
use git2::Repository;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::Utc;

// Re-export main types for convenience
pub use hooks::{HookManager, HookConfig, HookType, HookResult, default_hook_config};
pub use branch::{BranchContextManager, BranchContext, ValidationStatus, MergeStrategy};
pub use workflow::{WorkflowManager, WorkflowConfig, WorkflowType, FlowBranchType, default_git_flow_config};
pub use history::{ContextHistoryManager, ContextEvolution, ContextBlame, ContextVersion};
pub use automation::{GitAutomationManager, AutomationConfig, AutomationTask, default_automation_config};
pub use security::{SecurityManager, SecurityConfig, default_security_config};
pub use monitoring::{GitMonitoringManager, MonitoringConfig, default_monitoring_config};

/// Advanced Git integration manager for Rhema
pub struct AdvancedGitIntegration {
    repo_path: PathBuf,
    config: GitIntegrationConfig,
}

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegrationConfig {
    /// Hook configuration
    pub hooks: HookConfig,
    
    /// Workflow configuration
    pub workflow: WorkflowConfig,
    
    /// Automation configuration
    pub automation: AutomationConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    
    /// Integration settings
    pub settings: IntegrationSettings,
}

/// Integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSettings {
    /// Enable advanced Git integration
    pub enabled: bool,
    
    /// Auto-initialize on repository setup
    pub auto_initialize: bool,
    
    /// Enable context-aware operations
    pub context_aware: bool,
    
    /// Enable security features
    pub security_enabled: bool,
    
    /// Logging level
    pub logging_level: String,
    
    /// Performance monitoring
    pub performance_monitoring: bool,
}

impl AdvancedGitIntegration {
    /// Create a new advanced Git integration manager
    pub fn new(repo_path: &Path, config: GitIntegrationConfig) -> RhemaResult<Self> {
        // Verify repository exists
        if !repo_path.join(".git").exists() {
            return Err(RhemaError::GitRepoNotFound(
                format!("No Git repository found at {}", repo_path.display())
            ));
        }
        
        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            config,
        })
    }
    
    /// Initialize advanced Git integration
    pub fn initialize(&mut self) -> RhemaResult<()> {
        if !self.config.settings.enabled {
            return Ok(());
        }
        
        // Create .rhema directory
        let rhema_dir = self.repo_path.join(".rhema");
        std::fs::create_dir_all(&rhema_dir)?;
        
        // Save configuration
        let config_file = rhema_dir.join("git-integration.yaml");
        let config_content = serde_yaml::to_string(&self.config)?;
        std::fs::write(config_file, config_content)?;
        
        println!("Advanced Git integration initialized at {}", self.repo_path.display());
        
        Ok(())
    }
    
    /// Get repository path
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
    
    /// Get branch context manager
    pub fn branches(&self) -> branch::BranchContextManager {
        let repo = Repository::open(&self.repo_path).expect("Failed to open repository");
        branch::BranchContextManager::new(repo)
    }
    
    /// Execute a Git hook
    pub fn execute_hook(&self, hook_type: HookType) -> RhemaResult<HookResult> {
        let repo = Repository::open(&self.repo_path)?;
        let hook_manager = HookManager::new(repo, self.config.hooks.clone());
        hook_manager.execute_hook(hook_type)
    }
    
    /// Create a feature branch with context isolation
    pub fn create_feature_branch(&mut self, feature_name: &str, _base_branch: &str) -> RhemaResult<workflow::FeatureBranch> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.start_feature(feature_name)
    }
    
    /// Finish a feature branch
    pub fn finish_feature_branch(&mut self, feature_name: &str) -> RhemaResult<workflow::FeatureResult> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.finish_feature(feature_name)
    }
    
    /// Start a release branch
    pub fn start_release_branch(&mut self, version: &str) -> RhemaResult<workflow::ReleaseBranch> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.start_release(version)
    }
    
    /// Finish a release branch
    pub fn finish_release_branch(&mut self, version: &str) -> RhemaResult<workflow::ReleaseResult> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.finish_release(version)
    }
    
    /// Start a hotfix branch
    pub fn start_hotfix_branch(&mut self, version: &str) -> RhemaResult<workflow::HotfixBranch> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.start_hotfix(version)
    }
    
    /// Finish a hotfix branch
    pub fn finish_hotfix_branch(&mut self, version: &str) -> RhemaResult<workflow::HotfixResult> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.finish_hotfix(version)
    }
    
    /// Analyze pull request
    pub fn analyze_pull_request(&self, pr_number: u64) -> RhemaResult<workflow::PullRequestAnalysis> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.analyze_pull_request(pr_number)
    }
    
    /// Track context evolution
    pub fn track_context_evolution(&mut self, scope_path: &str, limit: Option<usize>) -> RhemaResult<Vec<ContextEvolution>> {
        let repo = Repository::open(&self.repo_path)?;
        let mut history_manager = ContextHistoryManager::new(repo);
        history_manager.track_context_evolution(scope_path, limit)
    }
    
    /// Get context blame
    pub fn get_context_blame(&mut self, file_path: &Path) -> RhemaResult<Vec<ContextBlame>> {
        let repo = Repository::open(&self.repo_path)?;
        let mut history_manager = ContextHistoryManager::new(repo);
        history_manager.get_context_blame(file_path)
    }
    
    /// Create context version
    pub fn create_context_version(&mut self, version: &str, version_type: history::VersionType, description: &str) -> RhemaResult<ContextVersion> {
        let repo = Repository::open(&self.repo_path)?;
        let mut history_manager = ContextHistoryManager::new(repo);
        history_manager.create_context_version(version, version_type, description)
    }
    
    /// Rollback to context version
    pub fn rollback_to_version(&self, version: &str) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let history_manager = ContextHistoryManager::new(repo);
        history_manager.rollback_to_version(version)
    }
    
    /// Get automation status
    pub fn get_automation_status(&self) -> RhemaResult<automation::AutomationStatus> {
        let repo = Repository::open(&self.repo_path)?;
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.get_status()
    }
    
    /// Get security manager
    pub fn security(&self) -> security::SecurityManager {
        let repo = Repository::open(&self.repo_path).expect("Failed to open repository");
        security::SecurityManager::new(repo, self.config.security.clone()).expect("Failed to create security manager")
    }
    
    /// Validate access for a user
    pub fn validate_access(&self, user: &str, operation: &security::Operation, resource: &str) -> RhemaResult<bool> {
        let security_manager = self.security();
        security_manager.validate_access(user, operation, resource)
    }
    
    /// Run security scan
    pub fn run_security_scan(&self, path: &Path) -> RhemaResult<security::SecurityScanResult> {
        let security_manager = self.security();
        security_manager.run_security_scan(path)
    }
    
    /// Validate commit security
    pub fn validate_commit_security(&self, commit_hash: &str) -> RhemaResult<security::SecurityValidationResult> {
        let repo = Repository::open(&self.repo_path)?;
        let commit = repo.find_commit(git2::Oid::from_str(commit_hash)?)?;
        let security_manager = self.security();
        security_manager.validate_commit_security(&commit)
    }
    
    /// Get monitoring manager
    pub fn monitoring(&self) -> monitoring::GitMonitoringManager {
        let repo = Repository::open(&self.repo_path).expect("Failed to open repository");
        monitoring::GitMonitoringManager::new(repo, self.config.monitoring.clone()).expect("Failed to create monitoring manager")
    }
    
    /// Start monitoring
    pub fn start_monitoring(&self) -> RhemaResult<()> {
        let monitoring_manager = self.monitoring();
        monitoring_manager.start_monitoring()
    }
    
    /// Stop monitoring
    pub fn stop_monitoring(&self) -> RhemaResult<()> {
        let monitoring_manager = self.monitoring();
        monitoring_manager.stop_monitoring()
    }
    
    /// Get monitoring status
    pub fn get_monitoring_status(&self) -> RhemaResult<monitoring::MonitoringStatus> {
        let monitoring_manager = self.monitoring();
        monitoring_manager.get_status()
    }
    
    /// Record Git operation for monitoring
    pub fn record_git_operation(&self, operation: &str, duration: chrono::Duration) -> RhemaResult<()> {
        let monitoring_manager = self.monitoring();
        monitoring_manager.record_git_operation(operation, duration)
    }
    
    /// Record context operation for monitoring
    pub fn record_context_operation(&self, operation: &str, duration: chrono::Duration) -> RhemaResult<()> {
        let monitoring_manager = self.monitoring();
        monitoring_manager.record_context_operation(operation, duration)
    }
    
    /// Get task history
    pub fn get_task_history(&self, limit: Option<usize>) -> Vec<AutomationTask> {
        let repo = match Repository::open(&self.repo_path) {
            Ok(repo) => repo,
            Err(_) => return Vec::new(),
        };
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.get_task_history(limit)
    }
    
    /// Validate branch context
    pub fn validate_branch_context(&mut self) -> RhemaResult<ValidationStatus> {
        let repo = Repository::open(&self.repo_path)?;
        let mut branch_manager = BranchContextManager::new(repo);
        branch_manager.validate_branch_context()
    }
    
    /// Merge branch context
    pub fn merge_branch_context(&mut self, source_branch: &str, target_branch: &str) -> RhemaResult<branch::MergeResult> {
        let repo = Repository::open(&self.repo_path)?;
        let mut branch_manager = BranchContextManager::new(repo);
        branch_manager.merge_branch_context(source_branch, target_branch)
    }
    
    /// Check context conflicts
    pub fn check_context_conflicts(&self, source_branch: &str, target_branch: &str) -> RhemaResult<Vec<branch::ContextConflict>> {
        let repo = Repository::open(&self.repo_path)?;
        let branch_manager = BranchContextManager::new(repo);
        branch_manager.check_context_conflicts(source_branch, target_branch)
    }
    
    /// Resolve context conflicts
    pub fn resolve_context_conflicts(&mut self, conflicts: Vec<branch::ContextConflict>) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let mut branch_manager = BranchContextManager::new(repo);
        branch_manager.resolve_context_conflicts(conflicts)
    }
    
    /// Backup branch context
    pub fn backup_branch_context(&self, branch_name: &str) -> RhemaResult<PathBuf> {
        let repo = Repository::open(&self.repo_path)?;
        let branch_manager = BranchContextManager::new(repo);
        branch_manager.backup_branch_context(branch_name)
    }
    
    /// Restore branch context
    pub fn restore_branch_context(&mut self, backup_file: &Path) -> RhemaResult<BranchContext> {
        let repo = Repository::open(&self.repo_path)?;
        let mut branch_manager = BranchContextManager::new(repo);
        branch_manager.restore_branch_context(backup_file)
    }
    
    /// Get workflow status
    pub fn get_workflow_status(&self) -> RhemaResult<workflow::WorkflowStatus> {
        let repo = Repository::open(&self.repo_path)?;
        let workflow_manager = WorkflowManager::new(repo, self.config.workflow.clone());
        workflow_manager.get_workflow_status()
    }
    
    /// Generate evolution report
    pub fn generate_evolution_report(&self, scope_path: &str, since: Option<chrono::DateTime<Utc>>) -> RhemaResult<history::EvolutionReport> {
        let repo = Repository::open(&self.repo_path)?;
        let mut history_manager = ContextHistoryManager::new(repo);
        history_manager.generate_evolution_report(scope_path, since)
    }
    
    /// Get change history
    pub fn get_change_history(&self, file_path: &Path, limit: Option<usize>) -> RhemaResult<Vec<ContextEvolution>> {
        let repo = Repository::open(&self.repo_path)?;
        let history_manager = ContextHistoryManager::new(repo);
        history_manager.get_change_history(file_path, limit)
    }
    
    /// Start automation
    pub fn start_automation(&self) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.start_automation()
    }
    
    /// Stop automation
    pub fn stop_automation(&self) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.stop_automation()
    }
    
    /// Cancel automation task
    pub fn cancel_task(&self, task_id: &str) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.cancel_task(task_id)
    }
    
    /// Clear task history
    pub fn clear_task_history(&self) -> RhemaResult<()> {
        let repo = Repository::open(&self.repo_path)?;
        let automation_manager = GitAutomationManager::new(repo, self.config.automation.clone());
        automation_manager.clear_task_history()
    }
    
    /// Get integration status
    pub fn get_integration_status(&self) -> RhemaResult<IntegrationStatus> {
        let repo = Repository::open(&self.repo_path)?;
        let hook_manager = HookManager::new(repo, self.config.hooks.clone());
        let workflow_manager = WorkflowManager::new(Repository::open(&self.repo_path)?, self.config.workflow.clone());
        let automation_manager = GitAutomationManager::new(Repository::open(&self.repo_path)?, self.config.automation.clone());
        
        let hook_status = hook_manager.get_hook_status()?;
        let workflow_status = workflow_manager.get_workflow_status()?;
        let automation_status = automation_manager.get_status()?;
        
        Ok(IntegrationStatus {
            hooks_installed: hook_manager.hooks_installed()?,
            hook_status,
            workflow_status,
            automation_status,
            enabled: self.config.settings.enabled,
        })
    }
    
    /// Shutdown integration
    pub fn shutdown(&mut self) -> RhemaResult<()> {
        // Save any pending state
        self.save_state()?;
        
        println!("Advanced Git integration shut down successfully!");
        
        Ok(())
    }
    
    /// Save integration state
    fn save_state(&self) -> RhemaResult<()> {
        let state_file = self.repo_path
            .join(".rhema")
            .join("integration-state.yaml");
        
        let state = IntegrationState {
            timestamp: Utc::now(),
            config: self.config.clone(),
        };
        
        let content = serde_yaml::to_string(&state)?;
        std::fs::write(state_file, content)?;
        
        Ok(())
    }
}

/// Integration status
#[derive(Debug, Clone)]
pub struct IntegrationStatus {
    pub hooks_installed: bool,
    pub hook_status: HashMap<HookType, bool>,
    pub workflow_status: workflow::WorkflowStatus,
    pub automation_status: automation::AutomationStatus,
    pub enabled: bool,
}

/// Integration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationState {
    pub timestamp: chrono::DateTime<Utc>,
    pub config: GitIntegrationConfig,
}

/// Default Git integration configuration
pub fn default_git_integration_config() -> GitIntegrationConfig {
    GitIntegrationConfig {
        hooks: default_hook_config(),
        workflow: default_git_flow_config(),
        automation: default_automation_config(),
        security: default_security_config(),
        monitoring: default_monitoring_config(),
        settings: IntegrationSettings {
            enabled: true,
            auto_initialize: true,
            context_aware: true,
            security_enabled: true,
            logging_level: "info".to_string(),
            performance_monitoring: true,
        },
    }
}

/// Create advanced Git integration for a repository
pub fn create_advanced_git_integration(repo_path: &Path) -> RhemaResult<AdvancedGitIntegration> {
    let config = default_git_integration_config();
    AdvancedGitIntegration::new(repo_path, config)
}

/// Create advanced Git integration with custom configuration
pub fn create_advanced_git_integration_with_config(
    repo_path: &Path,
    config: GitIntegrationConfig,
) -> RhemaResult<AdvancedGitIntegration> {
    AdvancedGitIntegration::new(repo_path, config)
} 
