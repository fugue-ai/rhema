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

use crate::agent::{Agent, AgentId, AgentRequest, AgentResponse, AgentState};
use crate::error::{AgentError, AgentResult};
use crate::registry::AgentRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Execution context for task execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Execution ID
    pub execution_id: String,
    /// Agent ID
    pub agent_id: AgentId,
    /// Request being executed
    pub request: AgentRequest,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Expected end time
    pub expected_end_time: DateTime<Utc>,
    /// Execution policy
    pub policy: ExecutionPolicy,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
}

impl ExecutionContext {
    pub fn new(agent_id: AgentId, request: AgentRequest, policy: ExecutionPolicy) -> Self {
        let start_time = Utc::now();
        let expected_end_time = start_time + chrono::Duration::seconds(
            request.timeout.unwrap_or(policy.default_timeout) as i64
        );
        
        Self {
            execution_id: Uuid::new_v4().to_string(),
            agent_id,
            request,
            start_time,
            expected_end_time,
            policy,
            metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn is_timed_out(&self) -> bool {
        Utc::now() > self.expected_end_time
    }

    pub fn remaining_time(&self) -> Duration {
        let remaining = self.expected_end_time - Utc::now();
        if remaining.num_seconds() > 0 {
            Duration::from_secs(remaining.num_seconds() as u64)
        } else {
            Duration::from_secs(0)
        }
    }
}

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Execution ID
    pub execution_id: String,
    /// Agent response
    pub response: AgentResponse,
    /// Execution time in milliseconds
    pub execution_time: u64,
    /// Success flag
    pub success: bool,
    /// Error message if any
    pub error: Option<String>,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
}

impl ExecutionResult {
    pub fn success(execution_id: String, response: AgentResponse, execution_time: u64) -> Self {
        Self {
            execution_id,
            response,
            execution_time,
            success: true,
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn failure(execution_id: String, error: String, execution_time: u64) -> Self {
        Self {
            execution_id,
            response: AgentResponse::error("".to_string(), error.clone()),
            execution_time,
            success: false,
            error: Some(error),
            metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Execution policy
#[derive(Debug, Clone)]
pub struct ExecutionPolicy {
    /// Default timeout in seconds
    pub default_timeout: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
    /// Whether to allow concurrent execution
    pub allow_concurrent: bool,
    /// Maximum concurrent executions per agent
    pub max_concurrent_per_agent: usize,
    /// Whether to enable timeout
    pub enable_timeout: bool,
    /// Whether to enable retries
    pub enable_retries: bool,
    /// Whether to enable logging
    pub enable_logging: bool,
    /// Custom policy parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            default_timeout: 300, // 5 minutes
            max_retries: 3,
            retry_delay: 5,
            allow_concurrent: false,
            max_concurrent_per_agent: 1,
            enable_timeout: true,
            enable_retries: true,
            enable_logging: true,
            parameters: HashMap::new(),
        }
    }
}

impl ExecutionPolicy {
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.default_timeout = timeout;
        self
    }

    pub fn with_retries(mut self, max_retries: u32, retry_delay: u64) -> Self {
        self.max_retries = max_retries;
        self.retry_delay = retry_delay;
        self
    }

    pub fn with_concurrency(mut self, allow_concurrent: bool, max_concurrent: usize) -> Self {
        self.allow_concurrent = allow_concurrent;
        self.max_concurrent_per_agent = max_concurrent;
        self
    }

    pub fn with_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

/// Agent executor for task execution and management
#[derive(Clone)]
pub struct AgentExecutor {
    /// Agent registry
    registry: AgentRegistry,
    /// Active executions
    active_executions: Arc<RwLock<HashMap<String, ExecutionContext>>>,
    /// Execution results
    execution_results: Arc<RwLock<Vec<ExecutionResult>>>,
    /// Execution statistics
    stats: Arc<RwLock<ExecutionStats>>,
    /// Execution policy
    policy: ExecutionPolicy,
}

impl AgentExecutor {
    pub fn new(registry: AgentRegistry) -> Self {
        Self {
            registry,
            active_executions: Arc::new(RwLock::new(HashMap::new())),
            execution_results: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(ExecutionStats {
                total_executions: 0,
                successful_executions: 0,
                failed_executions: 0,
                active_executions: 0,
                avg_execution_time: 0,
                last_update: Utc::now(),
            })),
            policy: ExecutionPolicy::default(),
        }
    }

    /// Execute a task with an agent
    pub async fn execute(&self, agent_id: &AgentId, request: AgentRequest) -> AgentResult<AgentResponse> {
        let start_time = Instant::now();
        
        // Get the agent
        let agent = self.registry.get_agent(agent_id).await?;
        
        // Check if agent is ready
        let agent_guard = agent.read().await;
        if agent_guard.context().state != AgentState::Ready {
            return Err(AgentError::AgentNotActive {
                agent_id: agent_id.clone(),
            });
        }
        
        // Create execution context
        let policy = self.get_execution_policy(&request).await;
        let mut context = ExecutionContext::new(agent_id.clone(), request.clone(), policy.clone());
        
        // Check concurrency limits
        if !policy.allow_concurrent {
            let active_count = self.get_active_executions_count(agent_id).await;
            if active_count >= policy.max_concurrent_per_agent {
                return Err(AgentError::ExecutionFailed {
                    reason: "Agent is busy with another task".to_string(),
                });
            }
        }
        
        // Add to active executions
        {
            let mut active = self.active_executions.write().await;
            active.insert(context.execution_id.clone(), context.clone());
        }
        
        // Execute with retries
        let mut last_error = None;
        let mut attempt = 0;
        
        while attempt <= policy.max_retries {
            attempt += 1;
            
            // Update context for retry
            if attempt > 1 {
                context.add_metadata("retry_attempt".to_string(), attempt.to_string());
            }
            
            // Execute the task
            let result = self.execute_single_attempt(&agent, &context).await;
            
            match result {
                Ok(response) => {
                    // Success - record result and return
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    let execution_result = ExecutionResult::success(
                        context.execution_id.clone(),
                        response.clone(),
                        execution_time,
                    );
                    
                    // Remove from active executions
                    {
                        let mut active = self.active_executions.write().await;
                        active.remove(&context.execution_id);
                    }
                    
                    // Add to history
                    {
                        let mut history = self.execution_results.write().await;
                        history.push(execution_result);
                    }
                    
                    return Ok(response);
                }
                Err(error) => {
                    last_error = Some(error);
                    
                    // Check if we should retry
                    if attempt <= policy.max_retries && policy.enable_retries {
                        // Wait before retry
                        tokio::time::sleep(Duration::from_secs(policy.retry_delay)).await;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
        
        // All attempts failed
        let execution_time = start_time.elapsed().as_millis() as u64;
        let error_msg = last_error
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Unknown execution error".to_string());
        
        let execution_result = ExecutionResult::failure(
            context.execution_id.clone(),
            error_msg.clone(),
            execution_time,
        );
        
        // Remove from active executions
        {
            let mut active = self.active_executions.write().await;
            active.remove(&context.execution_id);
        }
        
        // Add to history
        {
            let mut history = self.execution_results.write().await;
            history.push(execution_result);
        }
        
        Err(AgentError::ExecutionFailed { reason: error_msg })
    }

    /// Execute a single attempt
    async fn execute_single_attempt(
        &self,
        agent: &Arc<RwLock<Box<dyn Agent>>>,
        context: &ExecutionContext,
    ) -> AgentResult<AgentResponse> {
        // Check timeout
        if context.is_timed_out() {
            return Err(AgentError::AgentTimeout {
                agent_id: context.agent_id.clone(),
            });
        }
        
        // Execute with timeout
        let timeout_duration = context.remaining_time();
        
        let execution_result = if timeout_duration.as_secs() > 0 {
            tokio::time::timeout(timeout_duration, async {
                let mut agent_guard = agent.write().await;
                agent_guard.execute_task(context.request.clone()).await
            })
            .await
        } else {
            let mut agent_guard = agent.write().await;
            Ok(agent_guard.execute_task(context.request.clone()).await)
        };
        
        match execution_result {
            Ok(result) => result,
            Err(_) => Err(AgentError::AgentTimeout {
                agent_id: context.agent_id.clone(),
            }),
        }
    }

    /// Get execution policy for a request
    async fn get_execution_policy(&self, request: &AgentRequest) -> ExecutionPolicy {
        // For now, return default policy
        // In the future, this could be customized based on request metadata
        self.policy.clone()
    }

    /// Get active executions count for an agent
    async fn get_active_executions_count(&self, agent_id: &AgentId) -> usize {
        let active = self.active_executions.read().await;
        active.values()
            .filter(|context| &context.agent_id == agent_id)
            .count()
    }

    /// Get active executions for an agent
    pub async fn get_active_executions(&self, agent_id: &AgentId) -> Vec<ExecutionContext> {
        let active = self.active_executions.read().await;
        active.values()
            .filter(|context| &context.agent_id == agent_id)
            .cloned()
            .collect()
    }

    /// Get all active executions
    pub async fn get_all_active_executions(&self) -> Vec<ExecutionContext> {
        let active = self.active_executions.read().await;
        active.values().cloned().collect()
    }

    /// Get execution history
    pub async fn get_execution_history(&self, limit: Option<usize>) -> Vec<ExecutionResult> {
        let history = self.execution_results.read().await;
        if let Some(limit) = limit {
            history.iter().rev().take(limit).cloned().collect()
        } else {
            history.iter().rev().cloned().collect()
        }
    }

    /// Get execution history for an agent
    pub async fn get_agent_execution_history(&self, agent_id: &AgentId, limit: Option<usize>) -> Vec<ExecutionResult> {
        let history = self.execution_results.read().await;
        let filtered: Vec<ExecutionResult> = history
            .iter()
            .filter(|result| result.response.request_id == *agent_id)
            .cloned()
            .collect();
        
        if let Some(limit) = limit {
            filtered.into_iter().rev().take(limit).collect()
        } else {
            filtered.into_iter().rev().collect()
        }
    }

    /// Cancel an execution
    pub async fn cancel_execution(&self, execution_id: &str) -> AgentResult<()> {
        let mut active = self.active_executions.write().await;
        
        if let Some(context) = active.remove(execution_id) {
            // Add cancellation to history
            let execution_result = ExecutionResult::failure(
                execution_id.to_string(),
                "Execution cancelled".to_string(),
                0,
            );
            
            let mut history = self.execution_results.write().await;
            history.push(execution_result);
            
            Ok(())
        } else {
            Err(AgentError::ExecutionFailed {
                reason: "Execution not found".to_string(),
            })
        }
    }

    /// Set default execution policy
    pub fn set_default_policy(&mut self, policy: ExecutionPolicy) {
        self.policy = policy;
    }

    /// Get execution statistics
    pub async fn get_execution_stats(&self) -> ExecutionStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Clear execution history
    pub async fn clear_history(&self) {
        let mut history = self.execution_results.write().await;
        history.clear();
    }
}

/// Execution statistics
#[derive(Debug, Clone)]
pub struct ExecutionStats {
    /// Total executions
    pub total_executions: usize,
    /// Successful executions
    pub successful_executions: usize,
    /// Failed executions
    pub failed_executions: usize,
    /// Active executions
    pub active_executions: usize,
    /// Average execution time in milliseconds
    pub avg_execution_time: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl std::fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Executions: {}/{} successful, {} active | Avg time: {}ms",
            self.successful_executions,
            self.total_executions,
            self.active_executions,
            self.avg_execution_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{BaseAgent, AgentConfig, AgentType, AgentCapability};

    #[tokio::test]
    async fn test_execution_policy_default() {
        let policy = ExecutionPolicy::default();
        assert_eq!(policy.default_timeout, 300);
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.retry_delay, 5);
    }

    #[tokio::test]
    async fn test_execution_context() {
        let agent_id = "test-agent".to_string();
        let request = AgentRequest::new("test".to_string(), serde_json::json!({}));
        let policy = ExecutionPolicy::default();
        
        let context = ExecutionContext::new(agent_id.clone(), request.clone(), policy);
        
        assert_eq!(context.agent_id, agent_id);
        assert_eq!(context.request.id, request.id);
        assert!(!context.is_timed_out());
    }

    #[tokio::test]
    async fn test_execution_result() {
        let execution_id = "test-execution".to_string();
        let response = AgentResponse::success("test-request".to_string(), serde_json::json!({}));
        
        let result = ExecutionResult::success(execution_id.clone(), response.clone(), 100);
        
        assert_eq!(result.execution_id, execution_id);
        assert!(result.success);
        assert_eq!(result.execution_time, 100);
    }

    #[tokio::test]
    async fn test_agent_executor_creation() {
        let registry = AgentRegistry::new();
        let executor = AgentExecutor::new(registry);
        
        let stats = executor.get_execution_stats().await;
        assert_eq!(stats.total_executions, 0);
        assert_eq!(stats.active_executions, 0);
    }
} 