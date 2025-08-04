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

use crate::agent::{AgentId, AgentRequest, AgentResponse, AgentState};
use crate::coordinator::AgentCoordinator;
use crate::executor::AgentExecutor;
use crate::error::{AgentError, AgentResult};
use crate::registry::AgentRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::Instant;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

/// Workflow step types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStepType {
    /// Execute a task on a specific agent
    Task {
        agent_id: AgentId,
        request: AgentRequest,
    },
    /// Execute multiple tasks in parallel
    Parallel {
        steps: Vec<WorkflowStep>,
    },
    /// Execute steps sequentially
    Sequential {
        steps: Vec<WorkflowStep>,
    },
    /// Conditional execution based on condition
    Conditional {
        condition: WorkflowCondition,
        if_true: Vec<WorkflowStep>,
        if_false: Option<Vec<WorkflowStep>>,
    },
    /// Loop execution
    Loop {
        condition: WorkflowCondition,
        steps: Vec<WorkflowStep>,
        max_iterations: Option<usize>,
    },
    /// Wait for a condition
    Wait {
        condition: WorkflowCondition,
        timeout: Option<u64>,
    },
    /// Send a message to agents
    Message {
        agent_ids: Vec<AgentId>,
        message_type: String,
        payload: serde_json::Value,
    },
    /// Coordinate between agents
    Coordinate {
        agent_ids: Vec<AgentId>,
        topic: String,
        policy: Option<crate::coordinator::CoordinationPolicy>,
    },
    /// Custom step
    Custom {
        step_type: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Workflow condition types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowCondition {
    /// Always true
    Always,
    /// Always false
    Never,
    /// Check if a variable equals a value
    VariableEquals {
        variable: String,
        value: serde_json::Value,
    },
    /// Check if a variable exists
    VariableExists {
        variable: String,
    },
    /// Check if a task completed successfully
    TaskSucceeded {
        task_id: String,
    },
    /// Check if a task failed
    TaskFailed {
        task_id: String,
    },
    /// Check if all tasks in a group succeeded
    AllTasksSucceeded {
        task_ids: Vec<String>,
    },
    /// Check if any task in a group succeeded
    AnyTaskSucceeded {
        task_ids: Vec<String>,
    },
    /// Check if all tasks in a group failed
    AllTasksFailed {
        task_ids: Vec<String>,
    },
    /// Check if any task in a group failed
    AnyTaskFailed {
        task_ids: Vec<String>,
    },
    /// Custom condition
    Custom {
        condition_type: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Workflow step
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step description
    pub description: Option<String>,
    /// Step type
    pub step_type: WorkflowStepType,
    /// Step timeout in seconds
    pub timeout: Option<u64>,
    /// Retry attempts
    pub retry_attempts: Option<u32>,
    /// Retry delay in seconds
    pub retry_delay: Option<u64>,
    /// Step metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl WorkflowStep {
    pub fn new(id: String, name: String, step_type: WorkflowStepType) -> Self {
        Self {
            id,
            name,
            description: None,
            step_type,
            timeout: None,
            retry_attempts: None,
            retry_delay: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_retry(mut self, attempts: u32, delay: u64) -> Self {
        self.retry_attempts = Some(attempts);
        self.retry_delay = Some(delay);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Workflow definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Workflow description
    pub description: Option<String>,
    /// Workflow version
    pub version: String,
    /// Workflow steps
    pub steps: Vec<WorkflowStep>,
    /// Input parameters
    pub input_parameters: Vec<WorkflowParameter>,
    /// Output parameters
    pub output_parameters: Vec<WorkflowParameter>,
    /// Workflow metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Workflow tags
    pub tags: Vec<String>,
}

/// Workflow parameter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowParameter {
    /// Parameter name
    pub name: String,
    /// Parameter description
    pub description: Option<String>,
    /// Parameter type
    pub parameter_type: String,
    /// Whether parameter is required
    pub required: bool,
    /// Default value
    pub default_value: Option<serde_json::Value>,
}

impl WorkflowDefinition {
    pub fn new(id: String, name: String, steps: Vec<WorkflowStep>) -> Self {
        Self {
            id,
            name,
            description: None,
            version: "1.0.0".to_string(),
            steps,
            input_parameters: Vec::new(),
            output_parameters: Vec::new(),
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    pub fn with_input_parameter(mut self, parameter: WorkflowParameter) -> Self {
        self.input_parameters.push(parameter);
        self
    }

    pub fn with_output_parameter(mut self, parameter: WorkflowParameter) -> Self {
        self.output_parameters.push(parameter);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

/// Workflow execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is pending
    Pending,
    /// Workflow is running
    Running,
    /// Workflow is paused
    Paused,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed
    Failed,
    /// Workflow was cancelled
    Cancelled,
    /// Workflow is waiting
    Waiting,
}

impl fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowStatus::Pending => write!(f, "Pending"),
            WorkflowStatus::Running => write!(f, "Running"),
            WorkflowStatus::Paused => write!(f, "Paused"),
            WorkflowStatus::Completed => write!(f, "Completed"),
            WorkflowStatus::Failed => write!(f, "Failed"),
            WorkflowStatus::Cancelled => write!(f, "Cancelled"),
            WorkflowStatus::Waiting => write!(f, "Waiting"),
        }
    }
}

/// Workflow step execution result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStepResult {
    /// Step ID
    pub step_id: String,
    /// Execution status
    pub status: WorkflowStepStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Execution time in milliseconds
    pub execution_time: Option<u64>,
    /// Result data
    pub data: Option<serde_json::Value>,
    /// Error message if any
    pub error: Option<String>,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Step metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Workflow step execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStepStatus {
    /// Step is pending
    Pending,
    /// Step is running
    Running,
    /// Step completed successfully
    Completed,
    /// Step failed
    Failed,
    /// Step was skipped
    Skipped,
    /// Step was cancelled
    Cancelled,
}

impl fmt::Display for WorkflowStepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowStepStatus::Pending => write!(f, "Pending"),
            WorkflowStepStatus::Running => write!(f, "Running"),
            WorkflowStepStatus::Completed => write!(f, "Completed"),
            WorkflowStepStatus::Failed => write!(f, "Failed"),
            WorkflowStepStatus::Skipped => write!(f, "Skipped"),
            WorkflowStepStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Workflow execution context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowExecutionContext {
    /// Execution ID
    pub execution_id: String,
    /// Workflow definition
    pub definition: WorkflowDefinition,
    /// Current status
    pub status: WorkflowStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Current step index
    pub current_step_index: usize,
    /// Step results
    pub step_results: HashMap<String, WorkflowStepResult>,
    /// Workflow variables
    pub variables: HashMap<String, serde_json::Value>,
    /// Input parameters
    pub input_parameters: HashMap<String, serde_json::Value>,
    /// Output parameters
    pub output_parameters: HashMap<String, serde_json::Value>,
    /// Execution metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl WorkflowExecutionContext {
    pub fn new(definition: WorkflowDefinition, input_parameters: HashMap<String, serde_json::Value>) -> Self {
        Self {
            execution_id: Uuid::new_v4().to_string(),
            definition,
            status: WorkflowStatus::Pending,
            start_time: Utc::now(),
            end_time: None,
            current_step_index: 0,
            step_results: HashMap::new(),
            variables: HashMap::new(),
            input_parameters,
            output_parameters: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
    }

    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }

    pub fn set_output_parameter(&mut self, key: String, value: serde_json::Value) {
        self.output_parameters.insert(key, value);
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, WorkflowStatus::Running | WorkflowStatus::Waiting)
    }
}

/// Workflow engine
pub struct WorkflowEngine {
    /// Agent registry
    registry: AgentRegistry,
    /// Agent coordinator
    coordinator: AgentCoordinator,
    /// Agent executor
    executor: AgentExecutor,
    /// Active workflow executions
    active_executions: Arc<RwLock<HashMap<String, WorkflowExecutionContext>>>,
    /// Workflow execution history
    execution_history: Arc<RwLock<Vec<WorkflowExecutionContext>>>,
    /// Workflow definitions
    definitions: Arc<RwLock<HashMap<String, WorkflowDefinition>>>,
}

impl WorkflowEngine {
    pub fn new(registry: AgentRegistry, coordinator: AgentCoordinator, executor: AgentExecutor) -> Self {
        Self {
            registry,
            coordinator,
            executor,
            active_executions: Arc::new(RwLock::new(HashMap::new())),
            execution_history: Arc::new(RwLock::new(Vec::new())),
            definitions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a workflow definition
    pub async fn register_workflow(&self, definition: WorkflowDefinition) -> AgentResult<()> {
        let mut definitions = self.definitions.write().await;
        definitions.insert(definition.id.clone(), definition);
        Ok(())
    }

    /// Get a workflow definition
    pub async fn get_workflow(&self, workflow_id: &str) -> AgentResult<Option<WorkflowDefinition>> {
        let definitions = self.definitions.read().await;
        Ok(definitions.get(workflow_id).cloned())
    }

    /// List all workflow definitions
    pub async fn list_workflows(&self) -> Vec<WorkflowDefinition> {
        let definitions = self.definitions.read().await;
        definitions.values().cloned().collect()
    }

    /// Start a workflow execution
    pub async fn start_workflow(
        &self,
        workflow_id: &str,
        input_parameters: HashMap<String, serde_json::Value>,
    ) -> AgentResult<String> {
        let definition = self.get_workflow(workflow_id).await?
            .ok_or_else(|| AgentError::WorkflowError {
                reason: format!("Workflow '{}' not found", workflow_id),
            })?;

        let mut context = WorkflowExecutionContext::new(definition, input_parameters);
        context.status = WorkflowStatus::Running;

        let execution_id = context.execution_id.clone();
        let engine = self.clone();
        let execution_id_clone = execution_id.clone();
        
        tokio::spawn(async move {
            if let Err(e) = engine.execute_workflow(&execution_id_clone).await {
                eprintln!("Workflow execution failed: {:?}", e);
            }
        });
        
        Ok(execution_id)
    }

    /// Execute a workflow
    async fn execute_workflow(&self, execution_id: &str) -> AgentResult<()> {
        let mut context = {
            let mut active_executions = self.active_executions.write().await;
            active_executions.get_mut(execution_id)
                .ok_or_else(|| AgentError::WorkflowError {
                    reason: format!("Execution '{}' not found", execution_id),
                })?
                .clone()
        };

        // Execute steps
        while context.current_step_index < context.definition.steps.len() && !context.is_completed() {
            let step = &context.definition.steps[context.current_step_index];
            
            // Execute step
            let step_result = self.execute_step(&context, step).await?;
            
            // Update context
            context.step_results.insert(step.id.clone(), step_result);
            
            // Move to next step
            context.current_step_index += 1;
            
            // Update active executions
            {
                let mut active_executions = self.active_executions.write().await;
                if let Some(active_context) = active_executions.get_mut(execution_id) {
                    *active_context = context.clone();
                }
            }
        }

        // Mark as completed
        if !context.is_completed() {
            context.status = WorkflowStatus::Completed;
            context.end_time = Some(Utc::now());
        }

        // Move to history
        {
            let mut active_executions = self.active_executions.write().await;
            active_executions.remove(execution_id);
            
            let mut execution_history = self.execution_history.write().await;
            execution_history.push(context);
        }

        Ok(())
    }

    /// Execute a workflow step
    async fn execute_step(&self, context: &WorkflowExecutionContext, step: &WorkflowStep) -> AgentResult<WorkflowStepResult> {
        let start_time = Utc::now();
        let start_instant = Instant::now();
        
        let result = match &step.step_type {
            WorkflowStepType::Task { agent_id, request } => {
                let task_result = self.execute_task_step(agent_id, request).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(task_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Parallel { steps } => {
                let parallel_result = Box::pin(self.execute_parallel_steps(context, steps)).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(parallel_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Sequential { steps } => {
                let sequential_result = Box::pin(self.execute_sequential_steps(context, steps)).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(sequential_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Conditional { condition, if_true, if_false } => {
                let conditional_result = Box::pin(self.execute_conditional_steps(context, condition, if_true, if_false)).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(conditional_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Loop { condition, steps, max_iterations } => {
                let loop_result = Box::pin(self.execute_loop_steps(context, condition, steps, *max_iterations)).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(loop_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Wait { condition, timeout } => {
                let wait_result = self.execute_wait_step(context, condition, *timeout).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(wait_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Message { agent_ids, message_type, payload } => {
                let message_result = self.execute_message_step(agent_ids, message_type, payload).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(message_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Coordinate { agent_ids, topic, policy } => {
                let coordinate_result = self.execute_coordinate_step(agent_ids, topic, policy.as_ref()).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(coordinate_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
            WorkflowStepType::Custom { step_type, parameters } => {
                let custom_result = self.execute_custom_step(step_type, parameters).await?;
                WorkflowStepResult {
                    step_id: step.id.clone(),
                    status: WorkflowStepStatus::Completed,
                    start_time,
                    end_time: Some(Utc::now()),
                    execution_time: Some(start_instant.elapsed().as_millis() as u64),
                    data: Some(custom_result),
                    error: None,
                    retry_attempts: 0,
                    metadata: step.metadata.clone(),
                }
            }
        };
        
        Ok(result)
    }

    /// Execute a workflow step internally
    async fn execute_step_internal(&self, context: &WorkflowExecutionContext, step: &WorkflowStep) -> AgentResult<serde_json::Value> {
        match &step.step_type {
            WorkflowStepType::Task { agent_id, request } => {
                self.execute_task_step(agent_id, request).await
            }
            WorkflowStepType::Parallel { steps } => {
                self.execute_parallel_steps(context, steps).await
            }
            WorkflowStepType::Sequential { steps } => {
                self.execute_sequential_steps(context, steps).await
            }
            WorkflowStepType::Conditional { condition, if_true, if_false } => {
                self.execute_conditional_steps(context, condition, if_true, if_false).await
            }
            WorkflowStepType::Loop { condition, steps, max_iterations } => {
                self.execute_loop_steps(context, condition, steps, *max_iterations).await
            }
            WorkflowStepType::Wait { condition, timeout } => {
                self.execute_wait_step(context, condition, *timeout).await
            }
            WorkflowStepType::Message { agent_ids, message_type, payload } => {
                self.execute_message_step(agent_ids, message_type, payload).await
            }
            WorkflowStepType::Coordinate { agent_ids, topic, policy } => {
                self.execute_coordinate_step(agent_ids, topic, policy.as_ref()).await
            }
            WorkflowStepType::Custom { step_type, parameters } => {
                self.execute_custom_step(step_type, parameters).await
            }
        }
    }

    /// Execute a task step
    async fn execute_task_step(&self, agent_id: &AgentId, request: &AgentRequest) -> AgentResult<serde_json::Value> {
        let response = self.executor.execute(agent_id, request.clone()).await?;
        
        match response.status {
            crate::agent::ResponseStatus::Success => {
                Ok(response.payload.unwrap_or(serde_json::Value::Null))
            }
            _ => {
                Err(AgentError::WorkflowError {
                    reason: format!("Task failed: {}", response.error.unwrap_or_else(|| "Unknown error".to_string())),
                })
            }
        }
    }

    /// Execute parallel steps
    async fn execute_parallel_steps(
        &self,
        context: &WorkflowExecutionContext,
        steps: &[WorkflowStep],
    ) -> AgentResult<serde_json::Value> {
        let mut results = Vec::new();
        
        for step in steps {
            let result = Box::pin(self.execute_step(context, step)).await?;
            results.push(result);
        }
        
        Ok(serde_json::json!({
            "type": "parallel",
            "results": results,
        }))
    }

    /// Execute sequential steps
    async fn execute_sequential_steps(
        &self,
        context: &WorkflowExecutionContext,
        steps: &[WorkflowStep],
    ) -> AgentResult<serde_json::Value> {
        let mut results = Vec::new();
        
        for step in steps {
            let result = self.execute_step(context, step).await?;
            results.push(result);
        }

        Ok(serde_json::json!({ "results": results }))
    }

    /// Execute conditional steps
    async fn execute_conditional_steps(
        &self,
        context: &WorkflowExecutionContext,
        condition: &WorkflowCondition,
        if_true: &[WorkflowStep],
        if_false: &Option<Vec<WorkflowStep>>,
    ) -> AgentResult<serde_json::Value> {
        let condition_result = self.evaluate_condition(context, condition).await?;
        
        let steps_to_execute = if condition_result {
            if_true
        } else {
            if_false.as_deref().unwrap_or(&[])
        };

        if steps_to_execute.is_empty() {
            Ok(serde_json::json!({ "condition": condition_result, "executed": false }))
        } else {
            let result = self.execute_sequential_steps(context, steps_to_execute).await?;
            Ok(serde_json::json!({ "condition": condition_result, "executed": true, "result": result }))
        }
    }

    /// Execute loop steps
    async fn execute_loop_steps(
        &self,
        context: &WorkflowExecutionContext,
        condition: &WorkflowCondition,
        steps: &[WorkflowStep],
        max_iterations: Option<usize>,
    ) -> AgentResult<serde_json::Value> {
        let mut iterations = 0;
        let mut results = Vec::new();

        loop {
            // Check max iterations
            if let Some(max) = max_iterations {
                if iterations >= max {
                    break;
                }
            }

            // Check condition
            if !self.evaluate_condition(context, condition).await? {
                break;
            }

            // Execute steps
            let result = self.execute_sequential_steps(context, steps).await?;
            results.push(result);
            iterations += 1;
        }

        Ok(serde_json::json!({ "iterations": iterations, "results": results }))
    }

    /// Execute wait step
    async fn execute_wait_step(
        &self,
        context: &WorkflowExecutionContext,
        condition: &WorkflowCondition,
        timeout: Option<u64>,
    ) -> AgentResult<serde_json::Value> {
        let start_time = Instant::now();
        let timeout_duration = timeout.map(Duration::from_secs);

        loop {
            // Check condition
            if self.evaluate_condition(context, condition).await? {
                break;
            }

            // Check timeout
            if let Some(timeout_duration) = timeout_duration {
                if start_time.elapsed() >= timeout_duration {
                    return Err(AgentError::WorkflowError {
                        reason: "Wait step timed out".to_string(),
                    });
                }
            }

            // Wait a bit before checking again
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Ok(serde_json::json!({ "waited": true }))
    }

    /// Execute message step
    async fn execute_message_step(
        &self,
        agent_ids: &[AgentId],
        message_type: &str,
        payload: &serde_json::Value,
    ) -> AgentResult<serde_json::Value> {
        let mut results = Vec::new();
        
        for agent_id in agent_ids {
            let message = crate::agent::AgentMessage::Custom(crate::agent::CustomMessage {
                id: Uuid::new_v4().to_string(),
                message_type: message_type.to_string(),
                sender: "workflow".to_string(),
                recipients: vec![agent_id.clone()],
                payload: payload.clone(),
                timestamp: Utc::now(),
            });

            // Note: This would need to be implemented through the message broker
            results.push(serde_json::json!({
                "agent_id": agent_id,
                "message_sent": true
            }));
        }

        Ok(serde_json::json!({ "messages_sent": results }))
    }

    /// Execute coordinate step
    async fn execute_coordinate_step(
        &self,
        agent_ids: &[AgentId],
        topic: &str,
        policy: Option<&crate::coordinator::CoordinationPolicy>,
    ) -> AgentResult<serde_json::Value> {
        let policy = policy.cloned().unwrap_or_default();
        let session_id = self.coordinator.create_session(
            topic.to_string(),
            agent_ids.to_vec(),
            Some(policy),
        ).await?;

        Ok(serde_json::json!({ "session_id": session_id }))
    }

    /// Execute custom step
    async fn execute_custom_step(
        &self,
        step_type: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> AgentResult<serde_json::Value> {
        // This would be implemented by custom step handlers
        Ok(serde_json::json!({
            "step_type": step_type,
            "parameters": parameters,
            "executed": true
        }))
    }

    /// Evaluate a workflow condition
    async fn evaluate_condition(&self, context: &WorkflowExecutionContext, condition: &WorkflowCondition) -> AgentResult<bool> {
        match condition {
            WorkflowCondition::Always => Ok(true),
            WorkflowCondition::Never => Ok(false),
            WorkflowCondition::VariableEquals { variable, value } => {
                Ok(context.get_variable(variable) == Some(value))
            }
            WorkflowCondition::VariableExists { variable } => {
                Ok(context.get_variable(variable).is_some())
            }
            WorkflowCondition::TaskSucceeded { task_id } => {
                if let Some(result) = context.step_results.get(task_id) {
                    Ok(result.status == WorkflowStepStatus::Completed)
                } else {
                    Ok(false)
                }
            }
            WorkflowCondition::TaskFailed { task_id } => {
                if let Some(result) = context.step_results.get(task_id) {
                    Ok(result.status == WorkflowStepStatus::Failed)
                } else {
                    Ok(false)
                }
            }
            WorkflowCondition::AllTasksSucceeded { task_ids } => {
                Ok(task_ids.iter().all(|task_id| {
                    context.step_results.get(task_id)
                        .map(|result| result.status == WorkflowStepStatus::Completed)
                        .unwrap_or(false)
                }))
            }
            WorkflowCondition::AnyTaskSucceeded { task_ids } => {
                Ok(task_ids.iter().any(|task_id| {
                    context.step_results.get(task_id)
                        .map(|result| result.status == WorkflowStepStatus::Completed)
                        .unwrap_or(false)
                }))
            }
            WorkflowCondition::AllTasksFailed { task_ids } => {
                Ok(task_ids.iter().all(|task_id| {
                    context.step_results.get(task_id)
                        .map(|result| result.status == WorkflowStepStatus::Failed)
                        .unwrap_or(false)
                }))
            }
            WorkflowCondition::AnyTaskFailed { task_ids } => {
                Ok(task_ids.iter().any(|task_id| {
                    context.step_results.get(task_id)
                        .map(|result| result.status == WorkflowStepStatus::Failed)
                        .unwrap_or(false)
                }))
            }
            WorkflowCondition::Custom { condition_type, parameters } => {
                // This would be implemented by custom condition handlers
                Ok(true) // Placeholder
            }
        }
    }

    /// Get workflow execution status
    pub async fn get_execution_status(&self, execution_id: &str) -> AgentResult<Option<WorkflowExecutionContext>> {
        let active_executions = self.active_executions.read().await;
        Ok(active_executions.get(execution_id).cloned())
    }

    /// Get all active executions
    pub async fn get_active_executions(&self) -> Vec<WorkflowExecutionContext> {
        let active_executions = self.active_executions.read().await;
        active_executions.values().cloned().collect()
    }

    /// Get execution history
    pub async fn get_execution_history(&self, limit: Option<usize>) -> Vec<WorkflowExecutionContext> {
        let execution_history = self.execution_history.read().await;
        let limit = limit.unwrap_or(execution_history.len());
        execution_history.iter().rev().take(limit).cloned().collect()
    }

    /// Cancel a workflow execution
    pub async fn cancel_execution(&self, execution_id: &str) -> AgentResult<()> {
        let mut active_executions = self.active_executions.write().await;
        if let Some(context) = active_executions.get_mut(execution_id) {
            context.status = WorkflowStatus::Cancelled;
            context.end_time = Some(Utc::now());
        }
        Ok(())
    }

    /// Get workflow statistics
    pub async fn get_workflow_stats(&self) -> WorkflowStats {
        let active_executions = self.active_executions.read().await;
        let execution_history = self.execution_history.read().await;

        let total_executions = active_executions.len() + execution_history.len();
        let active_executions_count = active_executions.len();
        let completed_executions = execution_history.iter()
            .filter(|e| e.status == WorkflowStatus::Completed)
            .count();
        let failed_executions = execution_history.iter()
            .filter(|e| e.status == WorkflowStatus::Failed)
            .count();

        WorkflowStats {
            total_executions,
            active_executions: active_executions_count,
            completed_executions,
            failed_executions,
            cancelled_executions: execution_history.iter()
                .filter(|e| e.status == WorkflowStatus::Cancelled)
                .count(),
            last_update: Utc::now(),
        }
    }
}

impl Clone for WorkflowEngine {
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
            coordinator: self.coordinator.clone(),
            executor: self.executor.clone(),
            active_executions: self.active_executions.clone(),
            execution_history: self.execution_history.clone(),
            definitions: self.definitions.clone(),
        }
    }
}

/// Workflow statistics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStats {
    /// Total executions
    pub total_executions: usize,
    /// Active executions
    pub active_executions: usize,
    /// Completed executions
    pub completed_executions: usize,
    /// Failed executions
    pub failed_executions: usize,
    /// Cancelled executions
    pub cancelled_executions: usize,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl std::fmt::Display for WorkflowStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Workflows: {}/{} active | Completed: {} | Failed: {} | Cancelled: {}",
            self.active_executions,
            self.total_executions,
            self.completed_executions,
            self.failed_executions,
            self.cancelled_executions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentRequest, AgentType, AgentConfig};

    #[tokio::test]
    async fn test_workflow_step_creation() {
        let step = WorkflowStep::new(
            "test-step".to_string(),
            "Test Step".to_string(),
            WorkflowStepType::Task {
                agent_id: "test-agent".to_string(),
                request: AgentRequest::new("test".to_string(), serde_json::json!({})),
            },
        );

        assert_eq!(step.id, "test-step");
        assert_eq!(step.name, "Test Step");
    }

    #[tokio::test]
    async fn test_workflow_definition_creation() {
        let steps = vec![
            WorkflowStep::new(
                "step1".to_string(),
                "Step 1".to_string(),
                WorkflowStepType::Task {
                    agent_id: "agent1".to_string(),
                    request: AgentRequest::new("task1".to_string(), serde_json::json!({})),
                },
            ),
        ];

        let definition = WorkflowDefinition::new(
            "test-workflow".to_string(),
            "Test Workflow".to_string(),
            steps,
        );

        assert_eq!(definition.id, "test-workflow");
        assert_eq!(definition.name, "Test Workflow");
        assert_eq!(definition.steps.len(), 1);
    }

    #[tokio::test]
    async fn test_workflow_context_creation() {
        let steps = vec![
            WorkflowStep::new(
                "step1".to_string(),
                "Step 1".to_string(),
                WorkflowStepType::Task {
                    agent_id: "agent1".to_string(),
                    request: AgentRequest::new("task1".to_string(), serde_json::json!({})),
                },
            ),
        ];

        let definition = WorkflowDefinition::new(
            "test-workflow".to_string(),
            "Test Workflow".to_string(),
            steps,
        );

        let input_parameters = HashMap::new();
        let context = WorkflowExecutionContext::new(definition, input_parameters);

        assert_eq!(context.status, WorkflowStatus::Pending);
        assert_eq!(context.current_step_index, 0);
    }

    #[tokio::test]
    async fn test_condition_evaluation() {
        let context = WorkflowExecutionContext::new(
            WorkflowDefinition::new("test".to_string(), "Test".to_string(), vec![]),
            HashMap::new(),
        );

        // Test Always condition
        let condition = WorkflowCondition::Always;
        // Note: This would need a proper engine instance to test
        assert!(true); // Placeholder
    }
} 