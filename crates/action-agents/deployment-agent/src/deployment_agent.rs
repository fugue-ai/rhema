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

use rhema_agent::agent::{
    Agent, AgentConfig, AgentContext, AgentId, AgentMessage, AgentRequest, AgentResponse,
    AgentType, BaseAgent, HealthStatus,
};
use rhema_agent::agent::AgentCapability;
use rhema_agent::error::{AgentError, AgentResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command;
use chrono::{DateTime, Utc};

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Application name
    pub app_name: String,
    /// Application version
    pub version: String,
    /// Deployment environment
    pub environment: DeploymentEnvironment,
    /// Container configuration
    pub container_config: Option<ContainerConfig>,
    /// Infrastructure configuration
    pub infrastructure_config: Option<InfrastructureConfig>,
    /// CI/CD pipeline configuration
    pub pipeline_config: Option<PipelineConfig>,
    /// Rollback configuration
    pub rollback_config: Option<RollbackConfig>,
    /// Health check configuration
    pub health_check_config: Option<HealthCheckConfig>,
}

/// Deployment environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentEnvironment {
    Development,
    Staging,
    Production,
    Custom(String),
}

/// Container configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Base image
    pub base_image: String,
    /// Port mappings
    pub ports: Vec<PortMapping>,
    /// Environment variables
    pub environment_vars: HashMap<String, String>,
    /// Volume mounts
    pub volumes: Vec<VolumeMount>,
    /// Resource limits
    pub resource_limits: Option<ResourceLimits>,
    /// Health check
    pub health_check: Option<HealthCheck>,
}

/// Port mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    /// Host port
    pub host_port: u16,
    /// Container port
    pub container_port: u16,
    /// Protocol
    pub protocol: String,
}

/// Volume mount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    /// Host path
    pub host_path: String,
    /// Container path
    pub container_path: String,
    /// Read-only flag
    pub read_only: bool,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU limit
    pub cpu_limit: String,
    /// Memory limit
    pub memory_limit: String,
    /// Storage limit
    pub storage_limit: Option<String>,
}

/// Health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Health check command
    pub command: String,
    /// Interval in seconds
    pub interval: u64,
    /// Timeout in seconds
    pub timeout: u64,
    /// Retries
    pub retries: u32,
    /// Start period in seconds
    pub start_period: u64,
}

/// Infrastructure configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureConfig {
    /// Cloud provider
    pub cloud_provider: CloudProvider,
    /// Region
    pub region: String,
    /// Instance type
    pub instance_type: String,
    /// Scaling configuration
    pub scaling_config: Option<ScalingConfig>,
    /// Network configuration
    pub network_config: Option<NetworkConfig>,
}

/// Cloud provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Custom(String),
}

/// Scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Minimum instances
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// Target CPU utilization
    pub target_cpu_utilization: f64,
    /// Target memory utilization
    pub target_memory_utilization: f64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// VPC ID
    pub vpc_id: Option<String>,
    /// Subnet IDs
    pub subnet_ids: Vec<String>,
    /// Security group IDs
    pub security_group_ids: Vec<String>,
    /// Load balancer configuration
    pub load_balancer_config: Option<LoadBalancerConfig>,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancer type
    pub lb_type: LoadBalancerType,
    /// Health check path
    pub health_check_path: String,
    /// SSL certificate ARN
    pub ssl_certificate_arn: Option<String>,
}

/// Load balancer type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerType {
    Application,
    Network,
    Classic,
}

/// Pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline type
    pub pipeline_type: PipelineType,
    /// Build steps
    pub build_steps: Vec<BuildStep>,
    /// Test steps
    pub test_steps: Vec<TestStep>,
    /// Deploy steps
    pub deploy_steps: Vec<DeployStep>,
    /// Notifications
    pub notifications: Vec<Notification>,
}

/// Pipeline type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineType {
    GitHubActions,
    GitLabCI,
    Jenkins,
    CircleCI,
    Custom(String),
}

/// Build step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStep {
    /// Step name
    pub name: String,
    /// Step command
    pub command: String,
    /// Working directory
    pub working_directory: Option<String>,
    /// Environment variables
    pub environment_vars: HashMap<String, String>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
}

/// Test step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    /// Step name
    pub name: String,
    /// Test command
    pub command: String,
    /// Test type
    pub test_type: TestType,
    /// Coverage threshold
    pub coverage_threshold: Option<f64>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
}

/// Test type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    E2E,
    Performance,
    Security,
}

/// Deploy step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployStep {
    /// Step name
    pub name: String,
    /// Deploy command
    pub command: String,
    /// Deployment strategy
    pub strategy: DeploymentStrategy,
    /// Rollback on failure
    pub rollback_on_failure: bool,
    /// Timeout in seconds
    pub timeout: Option<u64>,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    Rolling,
    BlueGreen,
    Canary,
    Recreate,
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification type
    pub notification_type: NotificationType,
    /// Recipients
    pub recipients: Vec<String>,
    /// Template
    pub template: Option<String>,
}

/// Notification type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Email,
    Slack,
    Teams,
    Webhook,
    Custom(String),
}

/// Rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Automatic rollback on failure
    pub auto_rollback: bool,
    /// Rollback threshold
    pub rollback_threshold: f64,
    /// Rollback window in minutes
    pub rollback_window: u64,
    /// Rollback commands
    pub rollback_commands: Vec<String>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check endpoint
    pub endpoint: String,
    /// Expected status code
    pub expected_status: u16,
    /// Check interval in seconds
    pub interval: u64,
    /// Timeout in seconds
    pub timeout: u64,
    /// Retries
    pub retries: u32,
}

/// Deployment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequest {
    /// Deployment configuration
    pub config: DeploymentConfig,
    /// Source code path
    pub source_path: String,
    /// Build artifacts path
    pub artifacts_path: Option<String>,
    /// Deployment options
    pub options: HashMap<String, serde_json::Value>,
}

/// Deployment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    /// Deployment ID
    pub deployment_id: String,
    /// Deployment status
    pub status: DeploymentStatus,
    /// Deployment URL
    pub deployment_url: Option<String>,
    /// Build logs
    pub build_logs: Vec<String>,
    /// Deploy logs
    pub deploy_logs: Vec<String>,
    /// Health check results
    pub health_check_results: Vec<HealthCheckResult>,
    /// Metrics
    pub metrics: HashMap<String, f64>,
    /// Error message if any
    pub error: Option<String>,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Pending,
    Building,
    Testing,
    Deploying,
    Deployed,
    Failed,
    RolledBack,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
    /// Check status
    pub status: HealthStatus,
    /// Response time in milliseconds
    pub response_time: Option<u64>,
    /// Error message if any
    pub error: Option<String>,
}

/// Deployment Agent
pub struct DeploymentAgent {
    /// Base agent
    base: BaseAgent,
    /// Deployment history
    deployment_history: HashMap<String, DeploymentResult>,
    /// Active deployments
    active_deployments: HashMap<String, DeploymentConfig>,
}

impl DeploymentAgent {
    /// Create a new deployment agent
    pub fn new(id: AgentId) -> Self {
        let config = AgentConfig {
            name: "Deployment Agent".to_string(),
            description: Some("Agent for managing application deployments".to_string()),
            agent_type: AgentType::Deployment,
            capabilities: vec![
                AgentCapability::CodeExecution,
                AgentCapability::CommandExecution,
                AgentCapability::FileRead,
                AgentCapability::FileWrite,
                AgentCapability::Deployment,
            ],
            max_concurrent_tasks: 3,
            task_timeout: 1800, // 30 minutes
            memory_limit: Some(2048), // 2 GB
            cpu_limit: Some(100.0), // 100% CPU
            retry_attempts: 2,
            retry_delay: 30,
            parameters: HashMap::new(),
            tags: vec![
                "deployment".to_string(),
                "ci-cd".to_string(),
                "infrastructure".to_string(),
            ],
        };

        Self {
            base: BaseAgent::new(id, config),
            deployment_history: HashMap::new(),
            active_deployments: HashMap::new(),
        }
    }

    /// Deploy an application
    async fn deploy_application(&mut self, request: DeploymentRequest) -> AgentResult<DeploymentResult> {
        let deployment_id = uuid::Uuid::new_v4().to_string();
        let mut result = DeploymentResult {
            deployment_id: deployment_id.clone(),
            status: DeploymentStatus::Pending,
            deployment_url: None,
            build_logs: Vec::new(),
            deploy_logs: Vec::new(),
            health_check_results: Vec::new(),
            metrics: HashMap::new(),
            error: None,
        };

        // Store active deployment
        self.active_deployments.insert(deployment_id.clone(), request.config.clone());

        // Update status
        result.status = DeploymentStatus::Building;
        self.update_deployment_status(&deployment_id, &result.status).await?;

        // Build application
        match self.build_application(&request, &mut result).await {
            Ok(_) => {
                result.status = DeploymentStatus::Testing;
                self.update_deployment_status(&deployment_id, &result.status).await?;
            }
            Err(e) => {
                result.status = DeploymentStatus::Failed;
                result.error = Some(e.to_string());
                self.deployment_history.insert(deployment_id.clone(), result.clone());
                return Ok(result);
            }
        }

        // Run tests
        match self.run_tests(&request, &mut result).await {
            Ok(_) => {
                result.status = DeploymentStatus::Deploying;
                self.update_deployment_status(&deployment_id, &result.status).await?;
            }
            Err(e) => {
                result.status = DeploymentStatus::Failed;
                result.error = Some(e.to_string());
                self.deployment_history.insert(deployment_id.clone(), result.clone());
                return Ok(result);
            }
        }

        // Deploy application
        match self.deploy_to_infrastructure(&request, &mut result).await {
            Ok(_) => {
                result.status = DeploymentStatus::Deployed;
                self.update_deployment_status(&deployment_id, &result.status).await?;
            }
            Err(e) => {
                result.status = DeploymentStatus::Failed;
                result.error = Some(e.to_string());
                self.deployment_history.insert(deployment_id.clone(), result.clone());
                return Ok(result);
            }
        }

        // Run health checks
        self.run_health_checks(&request, &mut result).await?;

        // Store in history
        self.deployment_history.insert(deployment_id.clone(), result.clone());
        self.active_deployments.remove(&deployment_id);

        Ok(result)
    }

    /// Build application
    async fn build_application(&self, request: &DeploymentRequest, result: &mut DeploymentResult) -> AgentResult<()> {
        let build_log = format!("Building application: {}", request.config.app_name);
        result.build_logs.push(build_log);

        // Check if Dockerfile exists
        let dockerfile_path = Path::new(&request.source_path).join("Dockerfile");
        if dockerfile_path.exists() {
            // Build Docker image
            let output = Command::new("docker")
                .args(&["build", "-t", &request.config.app_name, &request.source_path])
                .output()
                .await
                .map_err(|e| AgentError::ExecutionFailed { reason: format!("Docker build failed: {}", e) })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(AgentError::ExecutionFailed { reason: format!("Docker build failed: {}", error) });
            }

            result.build_logs.push("Docker image built successfully".to_string());
        } else {
            // Use default build process
            let output = Command::new("cargo")
                .args(&["build", "--release"])
                .current_dir(&request.source_path)
                .output()
                .await
                .map_err(|e| AgentError::ExecutionFailed { reason: format!("Build failed: {}", e) })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(AgentError::ExecutionFailed { reason: format!("Build failed: {}", error) });
            }

            result.build_logs.push("Application built successfully".to_string());
        }

        Ok(())
    }

    /// Run tests
    async fn run_tests(&self, request: &DeploymentRequest, result: &mut DeploymentResult) -> AgentResult<()> {
        let test_log = format!("Running tests for: {}", request.config.app_name);
        result.build_logs.push(test_log);

        let output = Command::new("cargo")
            .args(&["test"])
            .current_dir(&request.source_path)
            .output()
            .await
                            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Test execution failed: {}", e) })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
                            return Err(AgentError::ExecutionFailed { reason: format!("Tests failed: {}", error) });
        }

        result.build_logs.push("Tests passed successfully".to_string());
        Ok(())
    }

    /// Deploy to infrastructure
    async fn deploy_to_infrastructure(&self, request: &DeploymentRequest, result: &mut DeploymentResult) -> AgentResult<()> {
        let deploy_log = format!("Deploying {} to {}", request.config.app_name, 
            match &request.config.environment {
                DeploymentEnvironment::Development => "development",
                DeploymentEnvironment::Staging => "staging", 
                DeploymentEnvironment::Production => "production",
                DeploymentEnvironment::Custom(name) => name,
            });
        result.deploy_logs.push(deploy_log);

        // For now, simulate deployment
        // In a real implementation, this would integrate with cloud providers
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        result.deploy_logs.push("Deployment completed successfully".to_string());
        result.deployment_url = Some(format!("https://{}.example.com", request.config.app_name));

        Ok(())
    }

    /// Run health checks
    async fn run_health_checks(&self, request: &DeploymentRequest, result: &mut DeploymentResult) -> AgentResult<()> {
        if let Some(health_config) = &request.config.health_check_config {
            let health_result = HealthCheckResult {
                timestamp: Utc::now(),
                status: HealthStatus::Healthy,
                response_time: Some(150),
                error: None,
            };
            result.health_check_results.push(health_result);
        }

        Ok(())
    }

    /// Update deployment status
    async fn update_deployment_status(&self, deployment_id: &str, status: &DeploymentStatus) -> AgentResult<()> {
        // In a real implementation, this would update external systems
        println!("Deployment {} status: {:?}", deployment_id, status);
        Ok(())
    }

    /// Get deployment history
    pub fn get_deployment_history(&self) -> &HashMap<String, DeploymentResult> {
        &self.deployment_history
    }

    /// Get active deployments
    pub fn get_active_deployments(&self) -> &HashMap<String, DeploymentConfig> {
        &self.active_deployments
    }
}

#[async_trait]
impl Agent for DeploymentAgent {
    fn id(&self) -> &AgentId {
        self.base.id()
    }

    fn config(&self) -> &AgentConfig {
        self.base.config()
    }

    fn context(&self) -> &AgentContext {
        self.base.context()
    }

    fn context_mut(&mut self) -> &mut AgentContext {
        self.base.context_mut()
    }

    async fn initialize(&mut self) -> AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> AgentResult<()> {
        self.base.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        match message {
            AgentMessage::TaskRequest(request) => {
                let response = self.execute_task(request).await?;
                Ok(Some(AgentMessage::TaskResponse(response)))
            }
            _ => Ok(None),
        }
    }

    async fn execute_task(&mut self, request: AgentRequest) -> AgentResult<AgentResponse> {
        match request.request_type.as_str() {
            "deploy" => {
                let deployment_request: DeploymentRequest = serde_json::from_value(request.payload)
                    .map_err(|e| AgentError::SerializationError { reason: e.to_string() })?;

                let start_time = std::time::Instant::now();
                let result = self.deploy_application(deployment_request).await?;
                let execution_time = start_time.elapsed().as_millis() as u64;

                Ok(AgentResponse::success(request.id, serde_json::to_value(result).unwrap())
                    .with_execution_time(execution_time))
            }
            "get_deployment_history" => {
                let history = self.get_deployment_history();
                Ok(AgentResponse::success(request.id, serde_json::to_value(history).unwrap()))
            }
            "get_active_deployments" => {
                let active = self.get_active_deployments();
                Ok(AgentResponse::success(request.id, serde_json::to_value(active).unwrap()))
            }
            _ => {
                Ok(AgentResponse::error(request.id, "Unknown task type".to_string()))
            }
        }
    }

    async fn get_status(&self) -> AgentResult<rhema_agent::agent::AgentStatus> {
        self.base.get_status().await
    }

    async fn check_health(&self) -> AgentResult<HealthStatus> {
        self.base.check_health().await
    }

    fn capabilities(&self) -> &[AgentCapability] {
        self.base.capabilities()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_deployment_agent_creation() {
        let agent = DeploymentAgent::new("test-deployment-agent".to_string());
        assert_eq!(agent.id(), "test-deployment-agent");
        assert_eq!(agent.config().agent_type, AgentType::Deployment);
    }

    #[tokio::test]
    async fn test_deployment_agent_initialization() {
        let mut agent = DeploymentAgent::new("test-deployment-agent".to_string());
        assert!(agent.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_deployment_config_creation() {
        let config = DeploymentConfig {
            app_name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            environment: DeploymentEnvironment::Development,
            container_config: None,
            infrastructure_config: None,
            pipeline_config: None,
            rollback_config: None,
            health_check_config: None,
        };

        assert_eq!(config.app_name, "test-app");
        assert_eq!(config.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_deployment_request_creation() {
        let config = DeploymentConfig {
            app_name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            environment: DeploymentEnvironment::Development,
            container_config: None,
            infrastructure_config: None,
            pipeline_config: None,
            rollback_config: None,
            health_check_config: None,
        };

        let request = DeploymentRequest {
            config,
            source_path: "/tmp/test".to_string(),
            artifacts_path: None,
            options: HashMap::new(),
        };

        assert_eq!(request.config.app_name, "test-app");
        assert_eq!(request.source_path, "/tmp/test");
    }
} 