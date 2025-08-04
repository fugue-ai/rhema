# Agent Workflows

The Agent Workflows system in Rhema provides a powerful framework for orchestrating complex multi-agent tasks and processes. It enables you to define, execute, and monitor workflows that coordinate multiple agents to achieve specific goals.

## Overview

Agent Workflows allow you to:

- **Define complex workflows** with sequential, parallel, conditional, and loop-based execution patterns
- **Coordinate multiple agents** to work together on complex tasks
- **Handle conditional logic** based on workflow variables and task results
- **Monitor workflow execution** in real-time with detailed status tracking
- **Retry failed steps** with configurable retry policies
- **Integrate with existing agents** seamlessly

## Core Concepts

### Workflow Definition

A workflow is defined by a `WorkflowDefinition` that contains:

- **Steps**: The individual tasks or operations to be executed
- **Input Parameters**: Parameters that can be passed to the workflow
- **Output Parameters**: Results that the workflow produces
- **Metadata**: Additional information about the workflow

### Workflow Steps

Each workflow step can be one of several types:

- **Task**: Execute a specific task on a designated agent
- **Parallel**: Execute multiple steps simultaneously
- **Sequential**: Execute steps one after another
- **Conditional**: Execute steps based on conditions
- **Loop**: Repeat steps based on conditions
- **Wait**: Pause execution until a condition is met
- **Message**: Send messages to agents
- **Coordinate**: Coordinate between multiple agents
- **Custom**: Execute custom step logic

### Workflow Conditions

Conditions determine when steps should be executed:

- **Always/Never**: Simple boolean conditions
- **Variable Checks**: Check workflow variables
- **Task Results**: Check if previous tasks succeeded or failed
- **Custom Conditions**: User-defined condition logic

## Usage Examples

### Basic Workflow Definition

```rust
use rhema_agent::{
    WorkflowDefinition, WorkflowStep, WorkflowStepType, WorkflowCondition,
    AgentRequest, WorkflowParameter
};
use serde_json::json;

// Create a simple workflow
let steps = vec![
    WorkflowStep::new(
        "compile".to_string(),
        "Compile Code".to_string(),
        WorkflowStepType::Task {
            agent_id: "dev-agent".to_string(),
            request: AgentRequest::new("compile_code".to_string(), json!({})),
        },
    ),
    WorkflowStep::new(
        "test".to_string(),
        "Run Tests".to_string(),
        WorkflowStepType::Task {
            agent_id: "test-agent".to_string(),
            request: AgentRequest::new("run_tests".to_string(), json!({})),
        },
    ),
];

let workflow = WorkflowDefinition::new(
    "simple-workflow".to_string(),
    "Simple Build and Test".to_string(),
    steps,
);
```

### Parallel Execution

```rust
let parallel_steps = vec![
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
];

let parallel_step = WorkflowStep::new(
    "parallel_tests".to_string(),
    "Parallel Tests".to_string(),
    WorkflowStepType::Parallel { steps: parallel_steps },
);
```

### Conditional Execution

```rust
let conditional_step = WorkflowStep::new(
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
);
```

### Loop Execution

```rust
let loop_step = WorkflowStep::new(
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
        max_iterations: Some(10),
    },
);
```

### Wait Step

```rust
let wait_step = WorkflowStep::new(
    "wait_approval".to_string(),
    "Wait for Approval".to_string(),
    WorkflowStepType::Wait {
        condition: WorkflowCondition::VariableEquals {
            variable: "manual_approval".to_string(),
            value: json!(true),
        },
        timeout: Some(3600), // 1 hour timeout
    },
);
```

## Workflow Execution

### Starting a Workflow

```rust
use rhema_agent::RhemaAgentFramework;
use std::collections::HashMap;

// Create framework and register workflow
let mut framework = RhemaAgentFramework::new();
framework.initialize().await?;
framework.register_workflow(workflow).await?;

// Start workflow execution
let mut input_params = HashMap::new();
input_params.insert("run_performance_tests".to_string(), json!(true));
input_params.insert("manual_approval".to_string(), json!(true));

let execution_id = framework.start_workflow("workflow-id", input_params).await?;
```

### Monitoring Workflow Execution

```rust
// Get workflow status
if let Some(context) = framework.get_workflow_status(&execution_id).await? {
    println!("Status: {}", context.status);
    println!("Current step: {}/{}", 
        context.current_step_index, 
        context.definition.steps.len()
    );
    
    // Show step results
    for (step_id, result) in &context.step_results {
        println!("{}: {} ({}ms)", 
            step_id, 
            result.status, 
            result.execution_time.unwrap_or(0)
        );
    }
}
```

### Canceling a Workflow

```rust
framework.cancel_workflow(&execution_id).await?;
```

## Advanced Features

### Workflow Parameters

Define input and output parameters for your workflows:

```rust
let workflow = WorkflowDefinition::new("workflow-id".to_string(), "Workflow Name".to_string(), steps)
    .with_input_parameter(WorkflowParameter {
        name: "environment".to_string(),
        description: Some("Deployment environment".to_string()),
        parameter_type: "string".to_string(),
        required: true,
        default_value: None,
    })
    .with_output_parameter(WorkflowParameter {
        name: "deployment_url".to_string(),
        description: Some("URL of deployed application".to_string()),
        parameter_type: "string".to_string(),
        required: false,
        default_value: None,
    });
```

### Step Configuration

Configure individual steps with timeouts, retries, and metadata:

```rust
let step = WorkflowStep::new("step-id".to_string(), "Step Name".to_string(), step_type)
    .with_description("Step description".to_string())
    .with_timeout(300) // 5 minutes
    .with_retry(3, 60) // 3 retries with 60 second delay
    .with_metadata("priority".to_string(), json!("high"));
```

### Workflow Variables

Use variables to pass data between steps:

```rust
// In workflow execution context
context.set_variable("build_result".to_string(), json!({
    "status": "success",
    "artifacts": ["app.jar", "config.yml"]
}));

// In conditions
WorkflowCondition::VariableEquals {
    variable: "build_result.status".to_string(),
    value: json!("success"),
}
```

### Custom Step Types

Implement custom step types for specialized functionality:

```rust
let custom_step = WorkflowStep::new(
    "custom_step".to_string(),
    "Custom Step".to_string(),
    WorkflowStepType::Custom {
        step_type: "webhook".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("url".to_string(), json!("https://api.example.com/webhook"));
            params.insert("method".to_string(), json!("POST"));
            params
        },
    },
);
```

## Best Practices

### 1. Error Handling

- Always include error handling in your workflow steps
- Use retry mechanisms for transient failures
- Implement proper cleanup in case of workflow cancellation

### 2. Resource Management

- Set appropriate timeouts for each step
- Limit the number of concurrent executions
- Monitor resource usage during workflow execution

### 3. Monitoring and Observability

- Use meaningful step names and descriptions
- Add metadata to track workflow progress
- Implement proper logging for debugging

### 4. Workflow Design

- Keep workflows focused on a single responsibility
- Use parallel execution for independent tasks
- Implement proper error recovery mechanisms

### 5. Testing

- Test workflows with various input parameters
- Verify error handling and retry mechanisms
- Test workflow cancellation and cleanup

## Integration with Existing Agents

Agent Workflows integrate seamlessly with the existing agent framework:

- **Agent Registry**: Workflows can use any registered agent
- **Agent Coordination**: Use the coordination system for complex agent interactions
- **Message Broker**: Send messages between agents during workflow execution
- **Policy Engine**: Enforce policies during workflow execution
- **Metrics Collection**: Track workflow performance and agent utilization

## Example: CI/CD Pipeline

Here's a complete example of a CI/CD pipeline workflow:

```rust
fn create_cicd_pipeline() -> WorkflowDefinition {
    let steps = vec![
        // Compile code
        WorkflowStep::new("compile".to_string(), "Compile".to_string(),
            WorkflowStepType::Task {
                agent_id: "dev-agent".to_string(),
                request: AgentRequest::new("compile".to_string(), json!({})),
            }
        ),
        
        // Run tests in parallel
        WorkflowStep::new("tests".to_string(), "Tests".to_string(),
            WorkflowStepType::Parallel {
                steps: vec![
                    WorkflowStep::new("unit".to_string(), "Unit Tests".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "test-agent".to_string(),
                            request: AgentRequest::new("unit_test".to_string(), json!({})),
                        }
                    ),
                    WorkflowStep::new("integration".to_string(), "Integration Tests".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "test-agent".to_string(),
                            request: AgentRequest::new("integration_test".to_string(), json!({})),
                        }
                    ),
                ]
            }
        ),
        
        // Deploy to staging
        WorkflowStep::new("deploy_staging".to_string(), "Deploy to Staging".to_string(),
            WorkflowStepType::Task {
                agent_id: "deploy-agent".to_string(),
                request: AgentRequest::new("deploy_staging".to_string(), json!({})),
            }
        ),
        
        // Wait for approval
        WorkflowStep::new("approval".to_string(), "Wait for Approval".to_string(),
            WorkflowStepType::Wait {
                condition: WorkflowCondition::VariableEquals {
                    variable: "approved".to_string(),
                    value: json!(true),
                },
                timeout: Some(3600),
            }
        ),
        
        // Deploy to production
        WorkflowStep::new("deploy_production".to_string(), "Deploy to Production".to_string(),
            WorkflowStepType::Task {
                agent_id: "deploy-agent".to_string(),
                request: AgentRequest::new("deploy_production".to_string(), json!({})),
            }
        ),
    ];

    WorkflowDefinition::new("ci-cd".to_string(), "CI/CD Pipeline".to_string(), steps)
        .with_description("Complete CI/CD pipeline with testing and deployment")
        .with_tag("ci-cd")
        .with_tag("deployment")
}
```

## Conclusion

Agent Workflows provide a powerful and flexible way to orchestrate complex multi-agent processes in Rhema. By combining different step types, conditions, and execution patterns, you can create sophisticated workflows that automate complex tasks and coordinate multiple agents effectively.

The workflow system is designed to be extensible, allowing you to add custom step types and conditions as needed. It integrates seamlessly with the existing agent framework, providing a comprehensive solution for agent orchestration and automation. 