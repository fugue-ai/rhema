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
pub mod capabilities;
pub mod communication;
pub mod coordinator;
pub mod error;
pub mod executor;
pub mod lifecycle;
pub mod metrics;
pub mod policies;
pub mod registry;
pub mod workflow;
// Re-export main components for easy access
pub use agent::{
    Agent, AgentCapability, AgentConfig, AgentContext, AgentId, AgentMessage, AgentRequest,
    AgentResponse, AgentState, AgentType, BaseAgent,
};
pub use capabilities::{CapabilityManager, CapabilityRequest, CapabilityResponse};
pub use communication::{MessageBroker, MessageHandler, MessagePriority, MessageType};
pub use coordinator::{AgentCoordinator, CoordinationPolicy, CoordinationResult};
pub use error::{AgentError, AgentResult};
pub use executor::{AgentExecutor, ExecutionContext, ExecutionPolicy, ExecutionResult};
pub use lifecycle::{AgentLifecycle, LifecycleEvent, LifecycleState};
pub use metrics::{AgentMetrics, MetricsCollector, PerformanceMetrics};
pub use policies::{Policy, PolicyEnforcement, PolicyEngine, PolicyViolation};
pub use registry::{AgentRegistry, RegistryEntry, RegistryQuery};
pub use workflow::{
    WorkflowCondition, WorkflowDefinition, WorkflowEngine, WorkflowExecutionContext, WorkflowStats,
    WorkflowStatus, WorkflowStep, WorkflowStepType,
};

/// Main agent framework for Rhema
pub struct RhemaAgentFramework {
    /// Agent registry for managing all agents
    pub registry: AgentRegistry,
    /// Agent coordinator for managing agent interactions
    pub coordinator: AgentCoordinator,
    /// Message broker for agent communication
    pub message_broker: MessageBroker,
    /// Capability manager for agent capabilities
    pub capability_manager: CapabilityManager,
    /// Policy engine for enforcing agent policies
    pub policy_engine: PolicyEngine,
    /// Metrics collector for agent performance
    pub metrics_collector: MetricsCollector,
    /// Workflow engine for managing agent workflows
    pub workflow_engine: WorkflowEngine,
}

impl RhemaAgentFramework {
    /// Create a new Rhema agent framework
    pub fn new() -> Self {
        let registry = AgentRegistry::new();
        let coordinator = AgentCoordinator::new();
        let executor = AgentExecutor::new(registry.clone());

        Self {
            registry: registry.clone(),
            coordinator: coordinator.clone(),
            message_broker: MessageBroker::new(registry.clone()),
            capability_manager: CapabilityManager::new(),
            policy_engine: PolicyEngine::new(),
            metrics_collector: MetricsCollector::new(),
            workflow_engine: WorkflowEngine::new(registry, coordinator, executor),
        }
    }

    /// Initialize the framework
    pub async fn initialize(&mut self) -> AgentResult<()> {
        // Initialize all components
        self.registry.initialize().await?;
        self.coordinator.initialize().await?;
        self.message_broker.initialize().await?;
        self.capability_manager.initialize().await?;
        self.policy_engine.initialize().await?;
        self.metrics_collector.initialize().await?;

        Ok(())
    }

    /// Register an agent with the framework
    pub async fn register_agent(&mut self, agent: Box<dyn Agent>) -> AgentResult<AgentId> {
        let agent_id = agent.id().clone();

        // Register with registry
        self.registry.register(agent).await?;

        // Register with coordinator
        self.coordinator.register_agent(&agent_id).await?;

        // Register with message broker
        self.message_broker.register_agent(&agent_id).await?;

        Ok(agent_id)
    }

    /// Start an agent
    pub async fn start_agent(&mut self, agent_id: &AgentId) -> AgentResult<()> {
        // Start the agent lifecycle
        self.registry.start_agent(agent_id).await?;

        // Notify coordinator
        self.coordinator.agent_started(agent_id).await?;

        Ok(())
    }

    /// Stop an agent
    pub async fn stop_agent(&mut self, agent_id: &AgentId) -> AgentResult<()> {
        // Stop the agent lifecycle
        self.registry.stop_agent(agent_id).await?;

        // Notify coordinator
        self.coordinator.agent_stopped(agent_id).await?;

        Ok(())
    }

    /// Send a message to an agent
    pub async fn send_message(&self, agent_id: &AgentId, message: AgentMessage) -> AgentResult<()> {
        self.message_broker.send_message(agent_id, message).await
    }

    /// Execute a task with an agent
    pub async fn execute_task(
        &self,
        agent_id: &AgentId,
        task: AgentRequest,
    ) -> AgentResult<AgentResponse> {
        let executor = AgentExecutor::new(self.registry.clone());
        executor.execute(agent_id, task).await
    }

    /// Get agent metrics
    pub async fn get_agent_metrics(&self, agent_id: &AgentId) -> AgentResult<AgentMetrics> {
        self.metrics_collector.get_agent_metrics(agent_id).await
    }

    /// Get framework statistics
    pub async fn get_framework_stats(&self) -> AgentResult<FrameworkStats> {
        let workflow_stats = self.workflow_engine.get_workflow_stats().await;

        Ok(FrameworkStats {
            total_agents: self.registry.count_agents().await,
            active_agents: self.registry.count_active_agents().await,
            total_messages: self.message_broker.get_message_count().await,
            coordination_sessions: self.coordinator.get_session_count().await,
            policy_violations: self.policy_engine.get_violation_count().await,
            workflow_stats,
        })
    }

    /// Register a workflow definition
    pub async fn register_workflow(&self, definition: WorkflowDefinition) -> AgentResult<()> {
        self.workflow_engine.register_workflow(definition).await
    }

    /// Start a workflow execution
    pub async fn start_workflow(
        &self,
        workflow_id: &str,
        input_parameters: std::collections::HashMap<String, serde_json::Value>,
    ) -> AgentResult<String> {
        self.workflow_engine
            .start_workflow(workflow_id, input_parameters)
            .await
    }

    /// Get workflow execution status
    pub async fn get_workflow_status(
        &self,
        execution_id: &str,
    ) -> AgentResult<Option<WorkflowExecutionContext>> {
        self.workflow_engine
            .get_execution_status(execution_id)
            .await
    }

    /// Get all active workflow executions
    pub async fn get_active_workflows(&self) -> Vec<WorkflowExecutionContext> {
        self.workflow_engine.get_active_executions().await
    }

    /// Cancel a workflow execution
    pub async fn cancel_workflow(&self, execution_id: &str) -> AgentResult<()> {
        self.workflow_engine.cancel_execution(execution_id).await
    }

    /// Shutdown the framework
    pub async fn shutdown(&mut self) -> AgentResult<()> {
        // Stop all agents
        let agent_ids = self.registry.get_all_agent_ids().await?;
        for agent_id in agent_ids {
            let _ = self.stop_agent(&agent_id).await;
        }

        // Cancel all active workflows
        let active_workflows = self.workflow_engine.get_active_executions().await;
        for workflow in active_workflows {
            let _ = self
                .workflow_engine
                .cancel_execution(&workflow.execution_id)
                .await;
        }

        // Shutdown components
        self.registry.shutdown().await?;
        self.coordinator.shutdown().await?;
        self.message_broker.shutdown().await?;
        self.capability_manager.shutdown().await?;
        self.policy_engine.shutdown().await?;
        self.metrics_collector.shutdown().await?;

        Ok(())
    }
}

/// Framework statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameworkStats {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_messages: usize,
    pub coordination_sessions: usize,
    pub policy_violations: usize,
    pub workflow_stats: WorkflowStats,
}

impl std::fmt::Display for FrameworkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Agents: {}/{} active | Messages: {} | Sessions: {} | Violations: {} | {}",
            self.active_agents,
            self.total_agents,
            self.total_messages,
            self.coordination_sessions,
            self.policy_violations,
            self.workflow_stats
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentConfig, AgentType};

    #[tokio::test]
    async fn test_framework_creation() {
        let framework = RhemaAgentFramework::new();
        assert_eq!(framework.registry.count_agents().await, 0);
    }

    #[tokio::test]
    async fn test_framework_initialization() {
        let mut framework = RhemaAgentFramework::new();
        assert!(framework.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_framework_shutdown() {
        let mut framework = RhemaAgentFramework::new();
        framework.initialize().await.unwrap();
        assert!(framework.shutdown().await.is_ok());
    }
}
