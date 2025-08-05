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

pub mod agent;
pub mod ai_service;
pub mod context_injection;
pub mod grpc;
pub mod coordination_integration;
pub mod persistence;
pub mod distributed;
pub mod advanced_features;
pub mod production_config;
pub mod production_integration;

// Re-export main components for easy access
pub use agent::coordination::SyncCoordinator;
pub use agent::lock_context::LockFileContextProvider;
pub use agent::constraint_system::{
    ConstraintSystem, Constraint, ConstraintType, ConstraintSeverity, 
    EnforcementMode, ConstraintContext, EnforcementResult
};
pub use agent::task_scoring::{
    TaskScoringSystem, Task, TaskPriority, TaskStatus, TaskType,
    TaskScoringFactors, TaskScore, TaskPrioritization, PrioritizationStrategy
};
pub use agent::real_time_coordination::{
    RealTimeCoordinationSystem, AgentInfo, AgentStatus, AgentMessage,
    MessageType, MessagePriority, CoordinationSession, SessionStatus
};
pub use grpc::{GrpcCoordinationClient, GrpcCoordinationServer, GrpcClientConfig, GrpcServerConfig};
pub use agent::conflict_prevention::{
    ConflictPreventionSystem, Conflict, ConflictType, ConflictSeverity,
    ConflictStatus, ResolutionStrategy, ConflictResolution
};
pub use coordination_integration::{
    CoordinationIntegration, CoordinationConfig, IntegrationStats
};
pub use persistence::{PersistenceManager, PersistenceConfig, StorageStats};
pub use distributed::{DistributedManager, DistributedConfig, NodeInfo, ServiceInfo};
pub use advanced_features::{AdvancedFeaturesManager, AdvancedFeaturesConfig, PerformanceMetric, PerformanceAlert};
pub use production_config::{ProductionAIService, ProductionConfig, ServiceHealth, ServiceStats};
pub use production_integration::{ProductionIntegration, ProductionConfig as IntegrationProductionConfig};

// Error type conversions
impl From<agent::coordination::SyncError> for rhema_core::RhemaError {
    fn from(err: agent::coordination::SyncError) -> Self {
        rhema_core::RhemaError::CoordinationError(err.to_string())
    }
}

impl From<agent::constraint_system::ConstraintError> for rhema_core::RhemaError {
    fn from(err: agent::constraint_system::ConstraintError) -> Self {
        rhema_core::RhemaError::ConstraintError(err.to_string())
    }
}

impl From<agent::task_scoring::TaskScoringError> for rhema_core::RhemaError {
    fn from(err: agent::task_scoring::TaskScoringError) -> Self {
        rhema_core::RhemaError::TaskScoringError(err.to_string())
    }
}

impl From<agent::real_time_coordination::CoordinationError> for rhema_core::RhemaError {
    fn from(err: agent::real_time_coordination::CoordinationError) -> Self {
        rhema_core::RhemaError::CoordinationError(err.to_string())
    }
}

impl From<agent::conflict_prevention::ConflictPreventionError> for rhema_core::RhemaError {
    fn from(err: agent::conflict_prevention::ConflictPreventionError) -> Self {
        rhema_core::RhemaError::ConflictPreventionError(err.to_string())
    }
}

impl From<agent::patterns::PatternError> for rhema_core::RhemaError {
    fn from(err: agent::patterns::PatternError) -> Self {
        match err {
            agent::patterns::PatternError::ValidationError(msg) => rhema_core::RhemaError::ValidationError(msg),
            agent::patterns::PatternError::ExecutionError(msg) => rhema_core::RhemaError::SystemError(msg),
            agent::patterns::PatternError::ResourceNotAvailable(msg) => rhema_core::RhemaError::SystemError(msg),
            agent::patterns::PatternError::AgentNotAvailable(msg) => rhema_core::RhemaError::AgentError(msg),
            agent::patterns::PatternError::PatternTimeout(msg) => rhema_core::RhemaError::PerformanceError(msg),
            agent::patterns::PatternError::RollbackError(msg) => rhema_core::RhemaError::SystemError(msg),
            agent::patterns::PatternError::InvalidState(msg) => rhema_core::RhemaError::ValidationError(msg),
            agent::patterns::PatternError::ConfigurationError(msg) => rhema_core::RhemaError::ConfigError(msg),
            agent::patterns::PatternError::CommunicationError(msg) => rhema_core::RhemaError::NetworkError(msg),
            agent::patterns::PatternError::ConstraintViolation(msg) => rhema_core::RhemaError::ConstraintError(msg),
            agent::patterns::PatternError::TemplateNotFound(msg) => rhema_core::RhemaError::NotFound(msg),
            agent::patterns::PatternError::PatternNotFound(msg) => rhema_core::RhemaError::NotFound(msg),
        }
    }
}

// Main AI service for agentic development
pub struct AgenticDevelopmentService {
    /// Constraint system for enforcing development constraints
    pub constraint_system: ConstraintSystem,
    /// Task scoring system for prioritizing work
    pub task_scoring_system: TaskScoringSystem,
    /// Real-time coordination system for agent communication
    pub coordination_system: RealTimeCoordinationSystem,
    /// Conflict prevention system for detecting and resolving conflicts
    pub conflict_prevention_system: ConflictPreventionSystem,
    /// Lock file context provider for dependency management
    pub lock_context_provider: LockFileContextProvider,
    /// Sync coordinator for cross-scope synchronization
    pub sync_coordinator: SyncCoordinator,
    /// Syneidesis integration for enhanced coordination
    pub coordination_integration: Option<CoordinationIntegration>,
}

impl AgenticDevelopmentService {
    /// Create a new agentic development service
    pub fn new(lock_file_path: std::path::PathBuf) -> Self {
        Self {
            constraint_system: ConstraintSystem::new(),
            task_scoring_system: TaskScoringSystem::new(),
            coordination_system: RealTimeCoordinationSystem::new(),
            conflict_prevention_system: ConflictPreventionSystem::new(),
            lock_context_provider: LockFileContextProvider::new(lock_file_path),
            sync_coordinator: SyncCoordinator::new(),
            coordination_integration: None,
        }
    }

    /// Create a new agentic development service with Syneidesis integration
    pub async fn new_with_syneidesis(
        lock_file_path: std::path::PathBuf,
        syneidesis_config: Option<CoordinationConfig>,
    ) -> rhema_core::RhemaResult<Self> {
        let mut service = Self::new(lock_file_path);
        
        // Initialize Syneidesis integration
        let coordination_integration = CoordinationIntegration::new(
            service.coordination_system.clone(),
            syneidesis_config,
        ).await?;
        
        service.coordination_integration = Some(coordination_integration);
        Ok(service)
    }

    /// Initialize the service
    pub async fn initialize(&mut self) -> rhema_core::RhemaResult<()> {
        // Load lock file context
        self.lock_context_provider.load_lock_file()?;
        
        // Start coordination system heartbeat monitoring
        let _ = self.coordination_system.start_heartbeat_monitoring();
        
        Ok(())
    }

    /// Register an agent with all systems
    pub async fn register_agent(&self, agent_info: agent::real_time_coordination::AgentInfo) -> rhema_core::RhemaResult<()> {
        // Register with coordination system
        self.coordination_system.register_agent(agent_info).await?;
        
        Ok(())
    }

    /// Add a task to the scoring system
    pub fn add_task(&mut self, task: agent::task_scoring::Task) -> rhema_core::RhemaResult<()> {
        self.task_scoring_system.add_task(task)
    }

    /// Prioritize tasks for a scope
    pub fn prioritize_tasks(
        &mut self,
        scope: &str,
        strategy: agent::task_scoring::PrioritizationStrategy,
    ) -> rhema_core::RhemaResult<agent::task_scoring::TaskPrioritization> {
        self.task_scoring_system.prioritize_tasks(scope, strategy)
    }

    /// Enforce constraints for a context
    pub async fn enforce_constraints(
        &mut self,
        scope: &str,
        context: &agent::constraint_system::ConstraintContext,
    ) -> rhema_core::RhemaResult<agent::constraint_system::EnforcementResult> {
        self.constraint_system.enforce_constraints(scope, context).await
    }

    /// Detect conflicts
    pub async fn detect_conflicts(&mut self) -> rhema_core::RhemaResult<Vec<agent::conflict_prevention::Conflict>> {
        self.conflict_prevention_system.detect_conflicts().await
    }

    /// Resolve a conflict
    pub async fn resolve_conflict(
        &mut self,
        conflict_id: &str,
        strategy: agent::conflict_prevention::ResolutionStrategy,
    ) -> rhema_core::RhemaResult<agent::conflict_prevention::ConflictResolution> {
        self.conflict_prevention_system.resolve_conflict(conflict_id, strategy).await
    }

    /// Send a message to agents
    pub async fn send_message(&self, message: agent::real_time_coordination::AgentMessage) -> rhema_core::RhemaResult<()> {
        self.coordination_system.send_message(message).await
    }

    /// Create a coordination session
    pub async fn create_session(
        &self,
        topic: String,
        participants: Vec<String>,
    ) -> rhema_core::RhemaResult<String> {
        self.coordination_system.create_session(topic, participants).await
    }

    /// Get lock file context for AI agents
    pub fn get_lock_context(&self) -> rhema_core::RhemaResult<agent::lock_context::LockFileAIContext> {
        self.lock_context_provider.get_ai_context()
    }

    /// Get sync statistics
    pub fn get_sync_statistics(&self) -> agent::coordination::SyncStatistics {
        self.sync_coordinator.get_sync_statistics()
    }

    /// Get system statistics
    pub fn get_system_statistics(&self) -> SystemStatistics {
        SystemStatistics {
            coordination_stats: self.coordination_system.get_stats(),
            sync_stats: self.sync_coordinator.get_sync_statistics(),
            constraint_stats: self.constraint_system.get_stats().clone(),
            task_stats: self.task_scoring_system.get_prioritization_history().len(),
            conflict_stats: self.conflict_prevention_system.get_active_conflicts().len(),
        }
    }

    /// Register agent with Syneidesis (if integration is enabled)
    pub async fn register_agent_with_syneidesis(&self, agent_info: agent::real_time_coordination::AgentInfo) -> rhema_core::RhemaResult<()> {
        // First register with Rhema coordination system
        self.register_agent(agent_info.clone()).await?;
        
        // Then register with Syneidesis if integration is enabled
        if let Some(ref integration) = self.coordination_integration {
            integration.register_rhema_agent(&agent_info).await?;
        }
        
        Ok(())
    }

    /// Send message with Syneidesis bridging
    pub async fn send_message_with_syneidesis(&self, message: agent::real_time_coordination::AgentMessage) -> rhema_core::RhemaResult<()> {
        // Send through Rhema coordination system
        self.send_message(message.clone()).await?;
        
        // Bridge to Syneidesis if integration is enabled
        if let Some(ref integration) = self.coordination_integration {
            integration.bridge_rhema_message(&message).await?;
        }
        
        Ok(())
    }

    /// Get Syneidesis integration statistics
    pub async fn get_syneidesis_stats(&self) -> Option<IntegrationStats> {
        self.coordination_integration.as_ref().map(|integration| {
            // This would need to be async, but we're returning Option<IntegrationStats>
            // For now, return None if integration is not available
            None
        }).flatten()
    }

    /// Start Syneidesis health monitoring
    pub async fn start_syneidesis_health_monitoring(&self) -> rhema_core::RhemaResult<()> {
        if let Some(ref integration) = self.coordination_integration {
            integration.start_health_monitoring().await?;
        }
        Ok(())
    }

    /// Shutdown Syneidesis integration
    pub async fn shutdown_syneidesis(&self) -> rhema_core::RhemaResult<()> {
        if let Some(ref integration) = self.coordination_integration {
            integration.shutdown().await?;
        }
        Ok(())
    }

    /// Check if Syneidesis integration is enabled
    pub fn has_syneidesis_integration(&self) -> bool {
        self.coordination_integration.is_some()
    }
}

/// System statistics
#[derive(Debug, Clone)]
pub struct SystemStatistics {
    pub coordination_stats: agent::real_time_coordination::CoordinationStats,
    pub sync_stats: agent::coordination::SyncStatistics,
    pub constraint_stats: agent::constraint_system::EnforcementStats,
    pub task_stats: usize,
    pub conflict_stats: usize,
}

impl std::fmt::Display for SystemStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Coordination: {} agents, {} sessions | Sync: {} | Constraints: {} | Tasks: {} | Conflicts: {}",
            self.coordination_stats.active_agents,
            self.coordination_stats.active_sessions,
            self.sync_stats,
            format!("{} satisfied, {} violated", 
                self.constraint_stats.satisfied_constraints, 
                self.constraint_stats.violated_constraints),
            self.task_stats,
            self.conflict_stats
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_agentic_development_service_creation() {
        let lock_file_path = PathBuf::from("test.lock");
        let service = AgenticDevelopmentService::new(lock_file_path);
        
        // Verify all systems are initialized
        assert_eq!(service.coordination_system.get_stats().active_agents, 0);
        assert_eq!(service.sync_coordinator.get_sync_statistics().total_scopes, 0);
    }

    #[tokio::test]
    async fn test_service_initialization() {
        let lock_file_path = PathBuf::from("test.lock");
        let mut service = AgenticDevelopmentService::new(lock_file_path);
        
        // Should not fail even if lock file doesn't exist
        assert!(service.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_task_management() {
        let lock_file_path = PathBuf::from("test.lock");
        let mut service = AgenticDevelopmentService::new(lock_file_path);
        
        let task = agent::task_scoring::Task {
            id: "test-task".to_string(),
            title: "Test Task".to_string(),
            description: "A test task".to_string(),
            task_type: agent::task_scoring::TaskType::Feature,
            priority: agent::task_scoring::TaskPriority::High,
            status: agent::task_scoring::TaskStatus::Pending,
            complexity: agent::task_scoring::TaskComplexity::Moderate,
            scoring_factors: agent::task_scoring::TaskScoringFactors {
                business_value: 0.8,
                technical_debt_impact: 0.3,
                user_impact: 0.7,
                dependencies_count: 2,
                estimated_effort_hours: 8.0,
                risk_level: 0.2,
                urgency: 0.6,
                team_capacity_impact: 0.4,
                learning_value: 0.5,
                strategic_alignment: 0.6,
            },
            scope: "test-scope".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: std::collections::HashMap::new(),
        };

        assert!(service.add_task(task).is_ok());
        
        let prioritization = service.prioritize_tasks(
            "test-scope",
            agent::task_scoring::PrioritizationStrategy::WeightedScoring,
        ).unwrap();
        
        assert_eq!(prioritization.prioritized_tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_constraint_enforcement() {
        let lock_file_path = PathBuf::from("test.lock");
        let mut service = AgenticDevelopmentService::new(lock_file_path);
        
        let context = agent::constraint_system::ConstraintContext::new();
        
        let result = service.enforce_constraints("test-scope", &context).await.unwrap();
        assert!(result.satisfied);
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let lock_file_path = PathBuf::from("test.lock");
        let mut service = AgenticDevelopmentService::new(lock_file_path);
        
        let conflicts = service.detect_conflicts().await.unwrap();
        assert_eq!(conflicts.len(), 0); // No conflicts initially
    }
}
