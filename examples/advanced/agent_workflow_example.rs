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

use rhema_agent::{
    RhemaAgentFramework, Agent, AgentId, AgentConfig, AgentType, AgentCapability,
    AgentRequest, AgentResponse, AgentMessage, AgentState, AgentContext,
    WorkflowDefinition, WorkflowStep, WorkflowStepType, WorkflowCondition,
    WorkflowParameter, BaseAgent,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Example development agent
struct DevelopmentAgent {
    base: BaseAgent,
}

impl DevelopmentAgent {
    fn new(id: AgentId, config: AgentConfig) -> Self {
        Self {
            base: BaseAgent::new(id, config),
        }
    }
}

#[async_trait::async_trait]
impl Agent for DevelopmentAgent {
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

    async fn initialize(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> rhema_agent::AgentResult<Option<AgentMessage>> {
        self.base.handle_message(message).await
    }

    async fn execute_task(&mut self, request: AgentRequest) -> rhema_agent::AgentResult<AgentResponse> {
        match request.request_type.as_str() {
            "compile_code" => {
                // Simulate code compilation
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "compiled",
                        "warnings": 2,
                        "errors": 0
                    })
                ))
            }
            "run_tests" => {
                // Simulate test execution
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "passed",
                        "tests_run": 15,
                        "tests_passed": 15,
                        "tests_failed": 0
                    })
                ))
            }
            "lint_code" => {
                // Simulate code linting
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "clean",
                        "issues_found": 0,
                        "style_violations": 0
                    })
                ))
            }
            _ => {
                Ok(AgentResponse::error(
                    request.id,
                    format!("Unknown task type: {}", request.request_type)
                ))
            }
        }
    }

    async fn get_status(&self) -> rhema_agent::AgentResult<rhema_agent::AgentStatus> {
        self.base.get_status().await
    }

    async fn check_health(&self) -> rhema_agent::AgentResult<rhema_agent::HealthStatus> {
        self.base.check_health().await
    }

    fn capabilities(&self) -> &[AgentCapability] {
        self.base.capabilities()
    }
}

/// Example testing agent
struct TestingAgent {
    base: BaseAgent,
}

impl TestingAgent {
    fn new(id: AgentId, config: AgentConfig) -> Self {
        Self {
            base: BaseAgent::new(id, config),
        }
    }
}

#[async_trait::async_trait]
impl Agent for TestingAgent {
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

    async fn initialize(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> rhema_agent::AgentResult<Option<AgentMessage>> {
        self.base.handle_message(message).await
    }

    async fn execute_task(&mut self, request: AgentRequest) -> rhema_agent::AgentResult<AgentResponse> {
        match request.request_type.as_str() {
            "unit_test" => {
                // Simulate unit testing
                tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "passed",
                        "tests_run": 25,
                        "coverage": 85.5
                    })
                ))
            }
            "integration_test" => {
                // Simulate integration testing
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "passed",
                        "scenarios_run": 8,
                        "endpoints_tested": 12
                    })
                ))
            }
            "performance_test" => {
                // Simulate performance testing
                tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "passed",
                        "avg_response_time": 120,
                        "throughput": 1000
                    })
                ))
            }
            _ => {
                Ok(AgentResponse::error(
                    request.id,
                    format!("Unknown task type: {}", request.request_type)
                ))
            }
        }
    }

    async fn get_status(&self) -> rhema_agent::AgentResult<rhema_agent::AgentStatus> {
        self.base.get_status().await
    }

    async fn check_health(&self) -> rhema_agent::AgentResult<rhema_agent::HealthStatus> {
        self.base.check_health().await
    }

    fn capabilities(&self) -> &[AgentCapability] {
        self.base.capabilities()
    }
}

/// Example deployment agent
struct DeploymentAgent {
    base: BaseAgent,
}

impl DeploymentAgent {
    fn new(id: AgentId, config: AgentConfig) -> Self {
        Self {
            base: BaseAgent::new(id, config),
        }
    }
}

#[async_trait::async_trait]
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

    async fn initialize(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> rhema_agent::AgentResult<()> {
        self.base.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> rhema_agent::AgentResult<Option<AgentMessage>> {
        self.base.handle_message(message).await
    }

    async fn execute_task(&mut self, request: AgentRequest) -> rhema_agent::AgentResult<AgentResponse> {
        match request.request_type.as_str() {
            "build_image" => {
                // Simulate Docker image building
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "built",
                        "image_id": "sha256:abc123",
                        "size": "245MB"
                    })
                ))
            }
            "deploy_staging" => {
                // Simulate staging deployment
                tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "deployed",
                        "environment": "staging",
                        "url": "https://staging.example.com"
                    })
                ))
            }
            "deploy_production" => {
                // Simulate production deployment
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "deployed",
                        "environment": "production",
                        "url": "https://example.com"
                    })
                ))
            }
            _ => {
                Ok(AgentResponse::error(
                    request.id,
                    format!("Unknown task type: {}", request.request_type)
                ))
            }
        }
    }

    async fn get_status(&self) -> rhema_agent::AgentResult<rhema_agent::AgentStatus> {
        self.base.get_status().await
    }

    async fn check_health(&self) -> rhema_agent::AgentResult<rhema_agent::HealthStatus> {
        self.base.check_health().await
    }

    fn capabilities(&self) -> &[AgentCapability] {
        self.base.capabilities()
    }
}

/// Create a CI/CD workflow
fn create_cicd_workflow() -> WorkflowDefinition {
    let steps = vec![
        // Step 1: Code compilation
        WorkflowStep::new(
            "compile".to_string(),
            "Compile Code".to_string(),
            WorkflowStepType::Task {
                agent_id: "dev-agent".to_string(),
                request: AgentRequest::new("compile_code".to_string(), json!({})),
            },
        ).with_description("Compile the source code".to_string()),

        // Step 2: Code linting
        WorkflowStep::new(
            "lint".to_string(),
            "Lint Code".to_string(),
            WorkflowStepType::Task {
                agent_id: "dev-agent".to_string(),
                request: AgentRequest::new("lint_code".to_string(), json!({})),
            },
        ).with_description("Run code linting".to_string()),

        // Step 3: Parallel testing
        WorkflowStep::new(
            "parallel_tests".to_string(),
            "Parallel Tests".to_string(),
            WorkflowStepType::Parallel {
                steps: vec![
                    WorkflowStep::new(
                        "unit_tests".to_string(),
                        "Unit Tests".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "test-agent".to_string(),
                            request: AgentRequest::new("unit_test".to_string(), json!({})),
                        },
                    ),
                    WorkflowStep::new(
                        "integration_tests".to_string(),
                        "Integration Tests".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "test-agent".to_string(),
                            request: AgentRequest::new("integration_test".to_string(), json!({})),
                        },
                    ),
                ],
            },
        ).with_description("Run tests in parallel".to_string()),

        // Step 4: Conditional performance testing
        WorkflowStep::new(
            "performance_test".to_string(),
            "Performance Test".to_string(),
            WorkflowStepType::Conditional {
                condition: WorkflowCondition::VariableEquals {
                    variable: "run_performance_tests".to_string(),
                    value: json!(true),
                },
                if_true: vec![
                    WorkflowStep::new(
                        "perf_test".to_string(),
                        "Performance Test".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "test-agent".to_string(),
                            request: AgentRequest::new("performance_test".to_string(), json!({})),
                        },
                    ),
                ],
                if_false: None,
            },
        ).with_description("Run performance tests if enabled".to_string()),

        // Step 5: Build and deploy to staging
        WorkflowStep::new(
            "staging_deploy".to_string(),
            "Staging Deployment".to_string(),
            WorkflowStepType::Sequential {
                steps: vec![
                    WorkflowStep::new(
                        "build_image".to_string(),
                        "Build Image".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "deploy-agent".to_string(),
                            request: AgentRequest::new("build_image".to_string(), json!({})),
                        },
                    ),
                    WorkflowStep::new(
                        "deploy_staging".to_string(),
                        "Deploy to Staging".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "deploy-agent".to_string(),
                            request: AgentRequest::new("deploy_staging".to_string(), json!({})),
                        },
                    ),
                ],
            },
        ).with_description("Build and deploy to staging".to_string()),

        // Step 6: Wait for manual approval (simulated)
        WorkflowStep::new(
            "wait_approval".to_string(),
            "Wait for Approval".to_string(),
            WorkflowStepType::Wait {
                condition: WorkflowCondition::VariableEquals {
                    variable: "manual_approval".to_string(),
                    value: json!(true),
                },
                timeout: Some(3600), // 1 hour timeout
            },
        ).with_description("Wait for manual approval".to_string()),

        // Step 7: Deploy to production
        WorkflowStep::new(
            "production_deploy".to_string(),
            "Production Deployment".to_string(),
            WorkflowStepType::Task {
                agent_id: "deploy-agent".to_string(),
                request: AgentRequest::new("deploy_production".to_string(), json!({})),
            },
        ).with_description("Deploy to production".to_string()),
    ];

    WorkflowDefinition::new(
        "ci-cd-workflow".to_string(),
        "CI/CD Pipeline".to_string(),
        steps,
    )
    .with_description("Complete CI/CD pipeline with testing and deployment".to_string())
    .with_input_parameter(WorkflowParameter {
        name: "run_performance_tests".to_string(),
        description: Some("Whether to run performance tests".to_string()),
        parameter_type: "boolean".to_string(),
        required: false,
        default_value: Some(json!(false)),
    })
    .with_input_parameter(WorkflowParameter {
        name: "manual_approval".to_string(),
        description: Some("Manual approval for production deployment".to_string()),
        parameter_type: "boolean".to_string(),
        required: true,
        default_value: None,
    })
    .with_tag("ci-cd".to_string())
    .with_tag("deployment".to_string())
}

/// Create a monitoring workflow
fn create_monitoring_workflow() -> WorkflowDefinition {
    let steps = vec![
        // Step 1: Check system health
        WorkflowStep::new(
            "health_check".to_string(),
            "Health Check".to_string(),
            WorkflowStepType::Task {
                agent_id: "monitor-agent".to_string(),
                request: AgentRequest::new("health_check".to_string(), json!({})),
            },
        ),

        // Step 2: Conditional alerting
        WorkflowStep::new(
            "alert_if_needed".to_string(),
            "Alert if Needed".to_string(),
            WorkflowStepType::Conditional {
                condition: WorkflowCondition::TaskFailed {
                    task_id: "health_check".to_string(),
                },
                if_true: vec![
                    WorkflowStep::new(
                        "send_alert".to_string(),
                        "Send Alert".to_string(),
                        WorkflowStepType::Message {
                            agent_ids: vec!["alert-agent".to_string()],
                            message_type: "system_alert".to_string(),
                            payload: json!({
                                "severity": "high",
                                "message": "System health check failed"
                            }),
                        },
                    ),
                ],
                if_false: None,
            },
        ),

        // Step 3: Loop for continuous monitoring
        WorkflowStep::new(
            "monitoring_loop".to_string(),
            "Monitoring Loop".to_string(),
            WorkflowStepType::Loop {
                condition: WorkflowCondition::Always,
                steps: vec![
                    WorkflowStep::new(
                        "collect_metrics".to_string(),
                        "Collect Metrics".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "monitor-agent".to_string(),
                            request: AgentRequest::new("collect_metrics".to_string(), json!({})),
                        },
                    ),
                ],
                max_iterations: Some(10), // Limit to 10 iterations for demo
            },
        ),
    ];

    WorkflowDefinition::new(
        "monitoring-workflow".to_string(),
        "System Monitoring".to_string(),
        steps,
    )
    .with_description("Continuous system monitoring with alerting".to_string())
    .with_tag("monitoring".to_string())
    .with_tag("alerting".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Rhema Agent Workflows Example");
    println!("==========================================");

    // Create the agent framework
    let mut framework = RhemaAgentFramework::new();
    framework.initialize().await?;

    // Create and register agents
    println!("\n📋 Creating and registering agents...");

    // Development agent
    let dev_config = AgentConfig {
        name: "Development Agent".to_string(),
        description: Some("Handles code compilation and linting".to_string()),
        agent_type: AgentType::Development,
        capabilities: vec![
            AgentCapability::CodeExecution,
            AgentCapability::FileRead,
            AgentCapability::FileWrite,
        ],
        max_concurrent_tasks: 3,
        task_timeout: 300,
        retry_attempts: 2,
        retry_delay: 5,
        ..Default::default()
    };
    let dev_agent = DevelopmentAgent::new("dev-agent".to_string(), dev_config);
    framework.register_agent(Box::new(dev_agent)).await?;

    // Testing agent
    let test_config = AgentConfig {
        name: "Testing Agent".to_string(),
        description: Some("Handles test execution".to_string()),
        agent_type: AgentType::Testing,
        capabilities: vec![
            AgentCapability::CodeExecution,
            AgentCapability::Testing,
        ],
        max_concurrent_tasks: 2,
        task_timeout: 600,
        retry_attempts: 1,
        retry_delay: 10,
        ..Default::default()
    };
    let test_agent = TestingAgent::new("test-agent".to_string(), test_config);
    framework.register_agent(Box::new(test_agent)).await?;

    // Deployment agent
    let deploy_config = AgentConfig {
        name: "Deployment Agent".to_string(),
        description: Some("Handles deployment operations".to_string()),
        agent_type: AgentType::Deployment,
        capabilities: vec![
            AgentCapability::CommandExecution,
            AgentCapability::Deployment,
        ],
        max_concurrent_tasks: 1,
        task_timeout: 1800,
        retry_attempts: 3,
        retry_delay: 30,
        ..Default::default()
    };
    let deploy_agent = DeploymentAgent::new("deploy-agent".to_string(), deploy_config);
    framework.register_agent(Box::new(deploy_agent)).await?;

    // Start all agents
    println!("🚀 Starting agents...");
    framework.start_agent(&"dev-agent".to_string()).await?;
    framework.start_agent(&"test-agent".to_string()).await?;
    framework.start_agent(&"deploy-agent".to_string()).await?;

    // Register workflows
    println!("\n📋 Registering workflows...");

    let cicd_workflow = create_cicd_workflow();
    framework.register_workflow(cicd_workflow).await?;
    println!("✅ Registered CI/CD workflow");

    let monitoring_workflow = create_monitoring_workflow();
    framework.register_workflow(monitoring_workflow).await?;
    println!("✅ Registered monitoring workflow");

    // List registered workflows
    println!("\n📋 Registered workflows:");
    let workflows = framework.workflow_engine.list_workflows().await;
    for workflow in workflows {
        println!("  - {}: {}", workflow.id, workflow.name);
        if let Some(desc) = workflow.description {
            println!("    Description: {}", desc);
        }
        println!("    Steps: {}", workflow.steps.len());
        println!("    Tags: {:?}", workflow.tags);
    }

    // Start CI/CD workflow
    println!("\n🚀 Starting CI/CD workflow...");
    let mut input_params = HashMap::new();
    input_params.insert("run_performance_tests".to_string(), json!(true));
    input_params.insert("manual_approval".to_string(), json!(true));

    let execution_id = framework.start_workflow("ci-cd-workflow", input_params).await?;
    println!("✅ Started workflow execution: {}", execution_id);

    // Monitor workflow execution
    println!("\n📊 Monitoring workflow execution...");
    let mut completed = false;
    let mut check_count = 0;
    const MAX_CHECKS: usize = 30; // Maximum 30 checks

    while !completed && check_count < MAX_CHECKS {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        if let Some(context) = framework.get_workflow_status(&execution_id).await? {
            println!("  Status: {} | Step: {}/{}", 
                context.status, 
                context.current_step_index, 
                context.definition.steps.len()
            );

            // Show step results
            for (step_id, result) in &context.step_results {
                println!("    {}: {} ({}ms)", 
                    step_id, 
                    result.status, 
                    result.execution_time.unwrap_or(0)
                );
            }

            completed = context.is_completed();
        } else {
            println!("  Workflow execution not found");
            break;
        }

        check_count += 1;
    }

    if completed {
        println!("\n✅ Workflow execution completed!");
    } else {
        println!("\n⏰ Workflow execution timeout or still running");
    }

    // Get framework statistics
    println!("\n📊 Framework Statistics:");
    let stats = framework.get_framework_stats().await?;
    println!("{}", stats);

    // Get workflow statistics
    println!("\n📊 Workflow Statistics:");
    let workflow_stats = framework.workflow_engine.get_workflow_stats().await;
    println!("{}", workflow_stats);

    // Get active workflows
    println!("\n📋 Active workflows:");
    let active_workflows = framework.get_active_workflows().await;
    for workflow in active_workflows {
        println!("  - {}: {} (Step {}/{})", 
            workflow.execution_id, 
            workflow.definition.name,
            workflow.current_step_index,
            workflow.definition.steps.len()
        );
    }

    // Shutdown framework
    println!("\n🛑 Shutting down framework...");
    framework.shutdown().await?;
    println!("✅ Framework shutdown complete");

    println!("\n🎉 Agent Workflows Example completed successfully!");
    Ok(())
} 