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

use crate::error::{AgentError, AgentResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

/// Unique identifier for an agent
pub type AgentId = String;

/// Agent types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AgentType {
    /// Development agent for code-related tasks
    Development,
    /// Testing agent for test execution and validation
    Testing,
    /// Deployment agent for deployment operations
    Deployment,
    /// Monitoring agent for system monitoring
    Monitoring,
    /// Coordination agent for managing other agents
    Coordination,
    /// Analysis agent for data analysis
    Analysis,
    /// Security agent for security-related tasks
    Security,
    /// Documentation agent for documentation tasks
    Documentation,
    /// Custom agent type
    Custom(String),
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentType::Development => write!(f, "Development"),
            AgentType::Testing => write!(f, "Testing"),
            AgentType::Deployment => write!(f, "Deployment"),
            AgentType::Monitoring => write!(f, "Monitoring"),
            AgentType::Coordination => write!(f, "Coordination"),
            AgentType::Analysis => write!(f, "Analysis"),
            AgentType::Security => write!(f, "Security"),
            AgentType::Documentation => write!(f, "Documentation"),
            AgentType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Agent capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AgentCapability {
    /// Can execute code
    CodeExecution,
    /// Can read files
    FileRead,
    /// Can write files
    FileWrite,
    /// Can execute commands
    CommandExecution,
    /// Can communicate with other agents
    Communication,
    /// Can coordinate with other agents
    Coordination,
    /// Can monitor system resources
    Monitoring,
    /// Can analyze data
    Analysis,
    /// Can perform security checks
    Security,
    /// Can generate documentation
    Documentation,
    /// Can perform testing
    Testing,
    /// Can deploy applications
    Deployment,
    /// Custom capability
    Custom(String),
}

impl fmt::Display for AgentCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentCapability::CodeExecution => write!(f, "CodeExecution"),
            AgentCapability::FileRead => write!(f, "FileRead"),
            AgentCapability::FileWrite => write!(f, "FileWrite"),
            AgentCapability::CommandExecution => write!(f, "CommandExecution"),
            AgentCapability::Communication => write!(f, "Communication"),
            AgentCapability::Coordination => write!(f, "Coordination"),
            AgentCapability::Monitoring => write!(f, "Monitoring"),
            AgentCapability::Analysis => write!(f, "Analysis"),
            AgentCapability::Security => write!(f, "Security"),
            AgentCapability::Documentation => write!(f, "Documentation"),
            AgentCapability::Testing => write!(f, "Testing"),
            AgentCapability::Deployment => write!(f, "Deployment"),
            AgentCapability::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Agent state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AgentState {
    /// Agent is initializing
    Initializing,
    /// Agent is ready to receive tasks
    Ready,
    /// Agent is busy executing a task
    Busy,
    /// Agent is paused
    Paused,
    /// Agent is stopping
    Stopping,
    /// Agent is stopped
    Stopped,
    /// Agent is in error state
    Error,
    /// Agent is deadlocked
    Deadlocked,
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Initializing => write!(f, "Initializing"),
            AgentState::Ready => write!(f, "Ready"),
            AgentState::Busy => write!(f, "Busy"),
            AgentState::Paused => write!(f, "Paused"),
            AgentState::Stopping => write!(f, "Stopping"),
            AgentState::Stopped => write!(f, "Stopped"),
            AgentState::Error => write!(f, "Error"),
            AgentState::Deadlocked => write!(f, "Deadlocked"),
        }
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: Option<String>,
    /// Agent type
    pub agent_type: AgentType,
    /// Agent capabilities
    pub capabilities: Vec<AgentCapability>,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Task timeout in seconds
    pub task_timeout: u64,
    /// Memory limit in MB
    pub memory_limit: Option<u64>,
    /// CPU limit in percentage
    pub cpu_limit: Option<f64>,
    /// Retry attempts for failed tasks
    pub retry_attempts: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
    /// Custom configuration parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: "Unnamed Agent".to_string(),
            description: None,
            agent_type: AgentType::Custom("Unknown".to_string()),
            capabilities: vec![],
            max_concurrent_tasks: 1,
            task_timeout: 300, // 5 minutes
            memory_limit: None,
            cpu_limit: None,
            retry_attempts: 3,
            retry_delay: 5,
            parameters: HashMap::new(),
            tags: vec![],
        }
    }
}

/// Agent context for task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    /// Agent ID
    pub agent_id: AgentId,
    /// Current state
    pub state: AgentState,
    /// Current task ID if any
    pub current_task: Option<String>,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Task count
    pub task_count: u64,
    /// Success count
    pub success_count: u64,
    /// Error count
    pub error_count: u64,
    /// Performance metrics
    pub metrics: HashMap<String, f64>,
    /// Custom context data
    pub data: HashMap<String, serde_json::Value>,
}

impl AgentContext {
    pub fn new(agent_id: AgentId) -> Self {
        let now = Utc::now();
        Self {
            agent_id,
            state: AgentState::Initializing,
            current_task: None,
            start_time: now,
            last_activity: now,
            task_count: 0,
            success_count: 0,
            error_count: 0,
            metrics: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    pub fn increment_task_count(&mut self) {
        self.task_count += 1;
    }

    pub fn increment_success_count(&mut self) {
        self.success_count += 1;
    }

    pub fn increment_error_count(&mut self) {
        self.error_count += 1;
    }

    pub fn set_metric(&mut self, key: String, value: f64) {
        self.metrics.insert(key, value);
    }

    pub fn get_metric(&self, key: &str) -> Option<f64> {
        self.metrics.get(key).copied()
    }

    pub fn set_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

/// Agent message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessage {
    /// Task request
    TaskRequest(AgentRequest),
    /// Task response
    TaskResponse(AgentResponse),
    /// Status update
    StatusUpdate(AgentStatus),
    /// Heartbeat
    Heartbeat(AgentHeartbeat),
    /// Coordination message
    Coordination(CoordinationMessage),
    /// Error message
    Error(AgentErrorMessage),
    /// Custom message
    Custom(CustomMessage),
}

/// Agent request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRequest {
    /// Request ID
    pub id: String,
    /// Request type
    pub request_type: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request priority
    pub priority: u8,
    /// Request timeout
    pub timeout: Option<u64>,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

impl AgentRequest {
    pub fn new(request_type: String, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_type,
            payload,
            priority: 5, // Default priority
            timeout: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Agent response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentResponse {
    /// Response ID
    pub id: String,
    /// Request ID this response is for
    pub request_id: String,
    /// Response status
    pub status: ResponseStatus,
    /// Response payload
    pub payload: Option<serde_json::Value>,
    /// Response error if any
    pub error: Option<String>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Execution time in milliseconds
    pub execution_time: Option<u64>,
}

/// Response status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
    Timeout,
    Cancelled,
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseStatus::Success => write!(f, "Success"),
            ResponseStatus::Error => write!(f, "Error"),
            ResponseStatus::Timeout => write!(f, "Timeout"),
            ResponseStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl AgentResponse {
    pub fn success(request_id: String, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            status: ResponseStatus::Success,
            payload: Some(payload),
            error: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            execution_time: None,
        }
    }

    pub fn error(request_id: String, error: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            status: ResponseStatus::Error,
            payload: None,
            error: Some(error),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            execution_time: None,
        }
    }

    pub fn with_execution_time(mut self, execution_time: u64) -> Self {
        self.execution_time = Some(execution_time);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Agent status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentStatus {
    /// Agent ID
    pub agent_id: AgentId,
    /// Current state
    pub state: AgentState,
    /// Current task if any
    pub current_task: Option<String>,
    /// Health status
    pub health: HealthStatus,
    /// Resource usage
    pub resources: ResourceUsage,
    /// Last update time
    pub timestamp: DateTime<Utc>,
}

/// Health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Critical => write!(f, "Critical"),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Resource usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in MB
    pub memory_usage: u64,
    /// Disk usage in MB
    pub disk_usage: u64,
    /// Network usage in MB/s
    pub network_usage: f64,
}

/// Agent heartbeat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentHeartbeat {
    /// Agent ID
    pub agent_id: AgentId,
    /// Heartbeat timestamp
    pub timestamp: DateTime<Utc>,
    /// Agent status
    pub status: AgentStatus,
}

/// Coordination message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationMessage {
    /// Message ID
    pub id: String,
    /// Message type
    pub message_type: String,
    /// Sender agent ID
    pub sender: AgentId,
    /// Recipient agent IDs
    pub recipients: Vec<AgentId>,
    /// Message payload
    pub payload: serde_json::Value,
    /// Message priority
    pub priority: u8,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
}

/// Agent error message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentErrorMessage {
    /// Error ID
    pub id: String,
    /// Agent ID
    pub agent_id: AgentId,
    /// Error type
    pub error_type: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<serde_json::Value>,
    /// Error timestamp
    pub timestamp: DateTime<Utc>,
}

/// Custom message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomMessage {
    /// Message ID
    pub id: String,
    /// Message type
    pub message_type: String,
    /// Sender agent ID
    pub sender: AgentId,
    /// Recipient agent IDs
    pub recipients: Vec<AgentId>,
    /// Message payload
    pub payload: serde_json::Value,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
}

/// Core agent trait that all agents must implement
#[async_trait]
pub trait Agent: Send + Sync {
    /// Get agent ID
    fn id(&self) -> &AgentId;

    /// Get agent configuration
    fn config(&self) -> &AgentConfig;

    /// Get agent context
    fn context(&self) -> &AgentContext;

    /// Get agent context mutably
    fn context_mut(&mut self) -> &mut AgentContext;

    /// Initialize the agent
    async fn initialize(&mut self) -> AgentResult<()>;

    /// Start the agent
    async fn start(&mut self) -> AgentResult<()>;

    /// Stop the agent
    async fn stop(&mut self) -> AgentResult<()>;

    /// Handle a message
    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>>;

    /// Execute a task
    async fn execute_task(&mut self, request: AgentRequest) -> AgentResult<AgentResponse>;

    /// Get agent status
    async fn get_status(&self) -> AgentResult<AgentStatus>;

    /// Check agent health
    async fn check_health(&self) -> AgentResult<HealthStatus>;

    /// Get agent capabilities
    fn capabilities(&self) -> &[AgentCapability];

    /// Check if agent has a specific capability
    fn has_capability(&self, capability: &AgentCapability) -> bool {
        self.capabilities().contains(capability)
    }

    /// Get agent type
    fn agent_type(&self) -> &AgentType {
        &self.config().agent_type
    }

    /// Get agent name
    fn name(&self) -> &str {
        &self.config().name
    }

    /// Get agent description
    fn description(&self) -> Option<&str> {
        self.config().description.as_deref()
    }

    /// Update agent state
    fn update_state(&mut self, state: AgentState) {
        self.context_mut().state = state;
        self.context_mut().update_activity();
    }

    /// Set current task
    fn set_current_task(&mut self, task_id: Option<String>) {
        self.context_mut().current_task = task_id;
        self.context_mut().update_activity();
    }

    /// Record task completion
    fn record_task_completion(&mut self, success: bool) {
        self.context_mut().increment_task_count();
        if success {
            self.context_mut().increment_success_count();
        } else {
            self.context_mut().increment_error_count();
        }
        self.context_mut().update_activity();
    }
}

/// Base agent implementation
pub struct BaseAgent {
    /// Agent ID
    id: AgentId,
    /// Agent configuration
    config: AgentConfig,
    /// Agent context
    context: AgentContext,
}

impl BaseAgent {
    pub fn new(id: AgentId, config: AgentConfig) -> Self {
        let context = AgentContext::new(id.clone());
        Self {
            id,
            config,
            context,
        }
    }

    pub fn with_config(mut self, config: AgentConfig) -> Self {
        self.config = config;
        self
    }
}

#[async_trait]
impl Agent for BaseAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }

    fn context(&self) -> &AgentContext {
        &self.context
    }

    fn context_mut(&mut self) -> &mut AgentContext {
        &mut self.context
    }

    async fn initialize(&mut self) -> AgentResult<()> {
        self.update_state(AgentState::Initializing);
        // Default implementation does nothing
        Ok(())
    }

    async fn start(&mut self) -> AgentResult<()> {
        self.update_state(AgentState::Ready);
        Ok(())
    }

    async fn stop(&mut self) -> AgentResult<()> {
        self.update_state(AgentState::Stopping);
        self.update_state(AgentState::Stopped);
        Ok(())
    }

    async fn handle_message(&mut self, _message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        // Default implementation returns None (no response)
        Ok(None)
    }

    async fn execute_task(&mut self, request: AgentRequest) -> AgentResult<AgentResponse> {
        // Default implementation returns error
        Err(AgentError::ExecutionFailed {
            reason: "Base agent does not implement task execution".to_string(),
        })
    }

    async fn get_status(&self) -> AgentResult<AgentStatus> {
        Ok(AgentStatus {
            agent_id: self.id.clone(),
            state: self.context.state.clone(),
            current_task: self.context.current_task.clone(),
            health: HealthStatus::Unknown,
            resources: ResourceUsage::default(),
            timestamp: Utc::now(),
        })
    }

    async fn check_health(&self) -> AgentResult<HealthStatus> {
        Ok(HealthStatus::Unknown)
    }

    fn capabilities(&self) -> &[AgentCapability] {
        &self.config.capabilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.name, "Unnamed Agent");
        assert_eq!(config.max_concurrent_tasks, 1);
        assert_eq!(config.task_timeout, 300);
    }

    #[test]
    fn test_agent_context() {
        let agent_id = "test-agent".to_string();
        let mut context = AgentContext::new(agent_id.clone());
        
        assert_eq!(context.agent_id, agent_id);
        assert_eq!(context.state, AgentState::Initializing);
        assert_eq!(context.task_count, 0);
        
        context.increment_task_count();
        context.increment_success_count();
        assert_eq!(context.task_count, 1);
        assert_eq!(context.success_count, 1);
    }

    #[test]
    fn test_agent_request() {
        let payload = serde_json::json!({"test": "data"});
        let request = AgentRequest::new("test".to_string(), payload)
            .with_priority(10)
            .with_timeout(60);
        
        assert_eq!(request.request_type, "test");
        assert_eq!(request.priority, 10);
        assert_eq!(request.timeout, Some(60));
    }

    #[test]
    fn test_agent_response() {
        let request_id = "test-request".to_string();
        let payload = serde_json::json!({"result": "success"});
        let response = AgentResponse::success(request_id.clone(), payload.clone());
        
        assert_eq!(response.request_id, request_id);
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.payload, Some(payload));
    }

    #[tokio::test]
    async fn test_base_agent() {
        let config = AgentConfig::default();
        let mut agent = BaseAgent::new("test-agent".to_string(), config);
        
        assert!(agent.initialize().await.is_ok());
        assert!(agent.start().await.is_ok());
        assert_eq!(agent.context().state, AgentState::Ready);
        
        let status = agent.get_status().await.unwrap();
        assert_eq!(status.state, AgentState::Ready);
    }
} 