# Rhema Agent Implementation

This directory contains the implementation of specialized agents for the Rhema framework.

## Overview

The agents module provides five main agent types for comprehensive development workflows:

1. **CodeReviewAgent** - Performs security analysis and code review
2. **TestRunnerAgent** - Generates and executes tests
3. **DeploymentAgent** - Manages application deployment and CI/CD
4. **DocumentationAgent** - Generates and maintains documentation
5. **MonitoringAgent** - Monitors system performance and alerts

## CodeReviewAgent

The CodeReviewAgent is designed to perform comprehensive code analysis with a focus on security vulnerabilities, code quality issues, and performance problems.

### Features

- **Security Analysis**: Detects common security vulnerabilities
  - SQL Injection
  - Cross-Site Scripting (XSS)
  - Hardcoded credentials
  - Command injection
- **Code Quality Analysis**: Identifies code quality issues
  - Magic numbers
  - Long functions
  - Code style violations
- **Performance Analysis**: Detects performance issues
  - N+1 query problems
  - Inefficient algorithms
- **Scoring System**: Provides security and quality scores (0-100)

### Usage

```rust
use rhema_agent::{CodeReviewAgent, CodeReviewRequest};

// Create agent
let mut agent = CodeReviewAgent::new("code-review-1".to_string());
agent.initialize().await?;

// Perform code review
let request = CodeReviewRequest {
    code_path: "./src".to_string(),
    file_extensions: vec!["rs".to_string(), "py".to_string()],
    security_analysis: true,
    quality_analysis: true,
    performance_analysis: true,
    custom_rules: vec![],
    ignore_patterns: vec![],
};

let result = agent.perform_code_review(request).await?;
println!("Security Score: {}", result.security_score);
println!("Quality Score: {}", result.quality_score);
```

### Security Rules

The agent includes built-in security rules for common vulnerabilities:

| Vulnerability | Severity | CWE | Detection Method |
|---------------|----------|-----|------------------|
| SQL Injection | Critical | CWE-89 | Pattern matching |
| XSS | High | CWE-79 | Pattern matching |
| Hardcoded Credentials | High | CWE-259 | Pattern matching |
| Command Injection | Critical | CWE-78 | Pattern matching |

## TestRunnerAgent

The TestRunnerAgent is designed to automatically generate and execute tests for codebases.

### Features

- **Test Generation**: Automatically generates tests for functions
- **Multi-Framework Support**: Supports multiple testing frameworks
  - Rust (built-in test framework)
  - Python (pytest)
  - JavaScript (Jest)
- **Test Execution**: Runs tests and provides detailed results
- **Test Analysis**: Analyzes code to identify testable functions

### Supported Test Types

- Unit tests
- Integration tests
- Functional tests
- Performance tests
- Security tests
- Regression tests
- Smoke tests

### Usage

```rust
use rhema_agent::{TestRunnerAgent, TestGenerationRequest, TestExecutionRequest};

// Create agent
let mut agent = TestRunnerAgent::new("test-runner-1".to_string());
agent.initialize().await?;

// Generate tests
let generation_request = TestGenerationRequest {
    source_path: "./src".to_string(),
    file_extensions: vec!["rs".to_string()],
    test_types: vec![TestType::Unit],
    test_framework: "rust".to_string(),
    output_directory: "./tests".to_string(),
    options: HashMap::new(),
};

let generation_result = agent.generate_tests(generation_request).await?;
println!("Generated {} test files", generation_result.generated_test_files.len());

// Execute tests
let execution_request = TestExecutionRequest {
    test_path: "./tests".to_string(),
    test_types: vec![TestType::Unit],
    test_framework: "rust".to_string(),
    filters: vec![],
    options: HashMap::new(),
    timeout: Some(30),
    parallel_count: Some(4),
};

let execution_result = agent.execute_tests(execution_request).await?;
println!("Tests executed: {} passed, {} failed", 
         execution_result.passed_tests, execution_result.failed_tests);
```

### Test Templates

The agent includes templates for different programming languages:

#### Rust Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{function_name}() {
        // Arrange
        {setup_code}
        
        // Act
        let result = {function_call};
        
        // Assert
        {assertion_code}
    }
}
```

#### Python Template
```python
import unittest
from {module_name} import {class_name}

class Test{class_name}(unittest.TestCase):
    def setUp(self):
        {setup_code}
    
    def test_{method_name}(self):
        # Arrange
        {setup_code}
        
        # Act
        result = {method_call}
        
        # Assert
        {assertion_code}
```

#### JavaScript Template
```javascript
const { {function_name} } = require('./{module_name}');

describe('{class_name}', () => {
    beforeEach(() => {
        {setup_code}
    });
    
    test('should {test_description}', () => {
        // Arrange
        {setup_code}
        
        // Act
        const result = {function_call};
        
        // Assert
        {assertion_code}
    });
});
```

## DeploymentAgent

The DeploymentAgent is designed to manage application deployment, CI/CD pipelines, and infrastructure provisioning.

### Features

- **Application Deployment**: Deploys applications to various environments
  - Development, staging, and production environments
  - Container-based deployments (Docker)
  - Cloud platform integration (AWS, GCP, Azure)
- **CI/CD Pipeline Management**: Manages continuous integration and deployment
  - GitHub Actions, GitLab CI, Jenkins support
  - Automated build, test, and deploy workflows
  - Rollback capabilities
- **Infrastructure Provisioning**: Manages infrastructure resources
  - Cloud resource provisioning
  - Load balancer configuration
  - Auto-scaling setup
- **Health Monitoring**: Monitors deployed applications
  - Health check endpoints
  - Performance metrics
  - Automatic rollback on failures

### Usage

```rust
use rhema_agent::{DeploymentAgent, DeploymentRequest, DeploymentConfig, DeploymentEnvironment};

// Create agent
let mut agent = DeploymentAgent::new("deployment-1".to_string());
agent.initialize().await?;

// Deploy application
let config = DeploymentConfig {
    app_name: "my-app".to_string(),
    version: "1.0.0".to_string(),
    environment: DeploymentEnvironment::Production,
    container_config: None,
    infrastructure_config: None,
    pipeline_config: None,
    rollback_config: None,
    health_check_config: None,
};

let request = DeploymentRequest {
    config,
    source_path: "./src".to_string(),
    artifacts_path: None,
    options: HashMap::new(),
};

let result = agent.deploy_application(request).await?;
println!("Deployment Status: {:?}", result.status);
```

## DocumentationAgent

The DocumentationAgent is designed to automatically generate and maintain documentation for codebases.

### Features

- **API Documentation**: Generates comprehensive API documentation
  - OpenAPI/Swagger specifications
  - Interactive API documentation
  - Code examples and usage guides
- **Code Documentation**: Generates code documentation
  - Function and class documentation
  - Parameter and return type documentation
  - Code examples and usage patterns
- **Project Documentation**: Creates project documentation
  - README files
  - Architecture documentation
  - Deployment guides
- **Multiple Formats**: Supports various output formats
  - HTML documentation
  - Markdown files
  - PDF reports
  - JSON schemas

### Usage

```rust
use rhema_agent::{DocumentationAgent, DocumentationRequest, DocumentationConfig, DocumentationType, OutputFormat};

// Create agent
let mut agent = DocumentationAgent::new("documentation-1".to_string());
agent.initialize().await?;

// Generate documentation
let config = DocumentationConfig {
    project_name: "My Project".to_string(),
    version: "1.0.0".to_string(),
    doc_type: DocumentationType::API,
    output_format: OutputFormat::HTML,
    source_path: "./src".to_string(),
    output_directory: "./docs".to_string(),
    template_config: None,
    api_config: None,
    code_config: None,
    options: HashMap::new(),
};

let request = DocumentationRequest {
    config,
    force_regenerate: false,
    include_diagrams: true,
    options: HashMap::new(),
};

let result = agent.generate_documentation(request).await?;
println!("Generated {} files", result.generated_files.len());
```

## MonitoringAgent

The MonitoringAgent is designed to monitor system performance, collect metrics, and provide alerting capabilities.

### Features

- **System Monitoring**: Monitors system resources
  - CPU, memory, disk, and network usage
  - Process monitoring and analysis
  - Performance metrics collection
- **Alert Management**: Provides intelligent alerting
  - Configurable thresholds
  - Multiple severity levels
  - Custom alert rules
- **Notification Channels**: Supports various notification methods
  - Email notifications
  - Slack integration
  - Webhook callbacks
  - PagerDuty integration
- **Data Retention**: Manages monitoring data
  - Configurable retention periods
  - Historical data analysis
  - Performance trending

### Usage

```rust
use rhema_agent::{MonitoringAgent, MonitoringRequest, MonitoringConfig, MetricType, Threshold, ThresholdOperator, AlertSeverity};

// Create agent
let mut agent = MonitoringAgent::new("monitoring-1".to_string());
agent.initialize().await?;

// Set up monitoring
let mut thresholds = HashMap::new();
thresholds.insert("cpu_usage".to_string(), Threshold {
    value: 80.0,
    operator: ThresholdOperator::GreaterThan,
    severity: AlertSeverity::Warning,
    message: "CPU usage is high".to_string(),
});

let config = MonitoringConfig {
    interval: 60, // 1 minute
    metrics: vec![MetricType::CPU, MetricType::Memory, MetricType::Disk],
    thresholds,
    notifications: vec![],
    retention_days: 30,
    custom_rules: vec![],
};

let request = MonitoringRequest {
    config,
    targets: vec![],
    options: HashMap::new(),
};

let result = agent.start_monitoring(request).await?;
println!("Monitoring started with ID: {}", result.monitoring_id);
```

## Integration with Rhema Framework

All agents integrate seamlessly with the Rhema Agent Framework:

```rust
use rhema_agent::RhemaAgentFramework;

// Initialize framework
let mut framework = RhemaAgentFramework::new();
framework.initialize().await?;

// Register agents
    let code_review_agent = Box::new(CodeReviewAgent::new("code-review-1".to_string()));
    let code_review_id = framework.register_agent(code_review_agent).await?;

    let test_runner_agent = Box::new(TestRunnerAgent::new("test-runner-1".to_string()));
    let test_runner_id = framework.register_agent(test_runner_agent).await?;

    let deployment_agent = Box::new(DeploymentAgent::new("deployment-1".to_string()));
    let deployment_id = framework.register_agent(deployment_agent).await?;

    let documentation_agent = Box::new(DocumentationAgent::new("documentation-1".to_string()));
    let documentation_id = framework.register_agent(documentation_agent).await?;

    let monitoring_agent = Box::new(MonitoringAgent::new("monitoring-1".to_string()));
    let monitoring_id = framework.register_agent(monitoring_agent).await?;

    // Start agents
    framework.start_agent(&code_review_id).await?;
    framework.start_agent(&test_runner_id).await?;
    framework.start_agent(&deployment_id).await?;
    framework.start_agent(&documentation_id).await?;
    framework.start_agent(&monitoring_id).await?;

    // Send tasks to different agents
    let review_request = CodeReviewRequest { /* ... */ };
    let review_message = AgentMessage::TaskRequest(AgentRequest::new(
        "code_review".to_string(),
        serde_json::to_value(review_request).unwrap()
    ));
    framework.send_message(&code_review_id, review_message).await?;

    let deploy_request = DeploymentRequest { /* ... */ };
    let deploy_message = AgentMessage::TaskRequest(AgentRequest::new(
        "deploy".to_string(),
        serde_json::to_value(deploy_request).unwrap()
    ));
    framework.send_message(&deployment_id, deploy_message).await?;
```

## Testing

Comprehensive tests are included for all agents:

```bash
# Run all agent tests
cargo test --package rhema-agent

# Run specific agent tests
cargo test code_review_agent_tests
cargo test test_runner_agent_tests
cargo test deployment_agent_tests
cargo test documentation_agent_tests
cargo test monitoring_agent_tests
cargo test integration_tests
```

## Configuration

### CodeReviewAgent Configuration

```rust
let config = AgentConfig {
    name: "Code Review Agent".to_string(),
    description: Some("Agent for code review and security analysis".to_string()),
    agent_type: AgentType::Security,
    capabilities: vec![
        AgentCapability::FileRead,
        AgentCapability::Analysis,
        AgentCapability::Security,
    ],
    max_concurrent_tasks: 5,
    task_timeout: 300, // 5 minutes
    memory_limit: Some(512), // 512 MB
    cpu_limit: Some(50.0), // 50% CPU
    retry_attempts: 3,
    retry_delay: 10,
    parameters: HashMap::new(),
    tags: vec!["security".to_string(), "code-review".to_string(), "analysis".to_string()],
};
```

### TestRunnerAgent Configuration

```rust
let config = AgentConfig {
    name: "Test Runner Agent".to_string(),
    description: Some("Agent for test generation and execution".to_string()),
    agent_type: AgentType::Testing,
    capabilities: vec![
        AgentCapability::CodeExecution,
        AgentCapability::FileRead,
        AgentCapability::FileWrite,
        AgentCapability::Testing,
        AgentCapability::Analysis,
    ],
    max_concurrent_tasks: 10,
    task_timeout: 600, // 10 minutes
    memory_limit: Some(1024), // 1 GB
    cpu_limit: Some(75.0), // 75% CPU
    retry_attempts: 2,
    retry_delay: 5,
    parameters: HashMap::new(),
    tags: vec!["testing".to_string(), "test-generation".to_string(), "test-execution".to_string()],
};
```

### DeploymentAgent Configuration

```rust
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
    tags: vec!["deployment".to_string(), "ci-cd".to_string(), "infrastructure".to_string()],
};
```

### DocumentationAgent Configuration

```rust
let config = AgentConfig {
    name: "Documentation Agent".to_string(),
    description: Some("Agent for generating and maintaining documentation".to_string()),
    agent_type: AgentType::Documentation,
    capabilities: vec![
        AgentCapability::FileRead,
        AgentCapability::FileWrite,
        AgentCapability::Analysis,
        AgentCapability::Documentation,
    ],
    max_concurrent_tasks: 5,
    task_timeout: 600, // 10 minutes
    memory_limit: Some(1024), // 1 GB
    cpu_limit: Some(50.0), // 50% CPU
    retry_attempts: 2,
    retry_delay: 10,
    parameters: HashMap::new(),
    tags: vec!["documentation".to_string(), "api-docs".to_string(), "code-docs".to_string()],
};
```

### MonitoringAgent Configuration

```rust
let config = AgentConfig {
    name: "Monitoring Agent".to_string(),
    description: Some("Agent for system monitoring and alerting".to_string()),
    agent_type: AgentType::Monitoring,
    capabilities: vec![
        AgentCapability::Monitoring,
        AgentCapability::Analysis,
        AgentCapability::Communication,
    ],
    max_concurrent_tasks: 10,
    task_timeout: 300, // 5 minutes
    memory_limit: Some(512), // 512 MB
    cpu_limit: Some(25.0), // 25% CPU
    retry_attempts: 3,
    retry_delay: 5,
    parameters: HashMap::new(),
    tags: vec!["monitoring".to_string(), "metrics".to_string(), "alerting".to_string()],
};
```

## Dependencies

The agents module requires the following dependencies:

- `regex = "1.10"` - For pattern matching in security analysis
- `tempfile = "3.8"` - For temporary file operations in tests
- `tokio-test = "0.4"` - For async testing

## Future Enhancements

Planned improvements for the agents:

1. **Enhanced Security Analysis**
   - AST-based vulnerability detection
   - Dependency vulnerability scanning
   - SAST/DAST integration

2. **Advanced Test Generation**
   - Property-based testing
   - Mutation testing
   - Coverage-guided test generation

3. **Deployment Enhancements**
   - Multi-cloud deployment support
   - Blue-green deployment strategies
   - Infrastructure as Code integration

4. **Documentation Improvements**
   - AI-powered documentation generation
   - Interactive documentation
   - Multi-language support

5. **Monitoring Enhancements**
   - Machine learning-based anomaly detection
   - Predictive analytics
   - Advanced alert correlation

6. **Performance Optimization**
   - Parallel processing
   - Caching mechanisms
   - Incremental analysis

7. **Framework Extensions**
   - Plugin system for custom rules
   - Configuration management
   - Metrics and monitoring
   - Agent orchestration and coordination

## Contributing

When contributing to the agents module:

1. Follow the existing code style and patterns
2. Add comprehensive tests for new features
3. Update documentation for any API changes
4. Ensure all tests pass before submitting

## License

This module is licensed under the Apache License, Version 2.0. 