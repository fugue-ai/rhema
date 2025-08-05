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
    RhemaAgentFramework, 
    CodeReviewAgent, TestRunnerAgent, DeploymentAgent, DocumentationAgent, MonitoringAgent,
    CodeReviewRequest, TestGenerationRequest, TestExecutionRequest,
    DeploymentRequest, DeploymentConfig, DeploymentEnvironment,
    DocumentationRequest, DocumentationConfig, DocumentationType, OutputFormat,
    MonitoringRequest, MonitoringConfig, MetricType, Threshold, ThresholdOperator, AlertSeverity,
    AgentRequest, AgentMessage
};
use std::collections::HashMap;
use tempfile::tempdir;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Comprehensive Agent Workflow Example");
    println!("================================================");

    // Initialize the agent framework
    let mut framework = RhemaAgentFramework::new();
    framework.initialize().await?;

    // Create temporary directory for demonstration
    let temp_dir = tempdir()?;
    let source_dir = temp_dir.path().join("source");
    let test_dir = temp_dir.path().join("tests");
    let docs_dir = temp_dir.path().join("docs");
    let deploy_dir = temp_dir.path().join("deploy");
    
    fs::create_dir_all(&source_dir)?;
    fs::create_dir_all(&test_dir)?;
    fs::create_dir_all(&docs_dir)?;
    fs::create_dir_all(&deploy_dir)?;

    // Create sample Rust project structure
    create_sample_project(&source_dir)?;

    println!("\n📋 Step 1: Registering All Agents");
    println!("----------------------------------");

    // Register CodeReviewAgent
    println!("🔍 Registering CodeReviewAgent...");
    let code_review_agent = Box::new(CodeReviewAgent::new("code-review-1".to_string()));
    let code_review_id = framework.register_agent(code_review_agent).await?;
    framework.start_agent(&code_review_id).await?;
    println!("✅ CodeReviewAgent started with ID: {}", code_review_id);

    // Register TestRunnerAgent
    println!("🧪 Registering TestRunnerAgent...");
    let test_runner_agent = Box::new(TestRunnerAgent::new("test-runner-1".to_string()));
    let test_runner_id = framework.register_agent(test_runner_agent).await?;
    framework.start_agent(&test_runner_id).await?;
    println!("✅ TestRunnerAgent started with ID: {}", test_runner_id);

    // Register DeploymentAgent
    println!("🚀 Registering DeploymentAgent...");
    let deployment_agent = Box::new(DeploymentAgent::new("deployment-1".to_string()));
    let deployment_id = framework.register_agent(deployment_agent).await?;
    framework.start_agent(&deployment_id).await?;
    println!("✅ DeploymentAgent started with ID: {}", deployment_id);

    // Register DocumentationAgent
    println!("📚 Registering DocumentationAgent...");
    let documentation_agent = Box::new(DocumentationAgent::new("documentation-1".to_string()));
    let documentation_id = framework.register_agent(documentation_agent).await?;
    framework.start_agent(&documentation_id).await?;
    println!("✅ DocumentationAgent started with ID: {}", documentation_id);

    // Register MonitoringAgent
    println!("📊 Registering MonitoringAgent...");
    let monitoring_agent = Box::new(MonitoringAgent::new("monitoring-1".to_string()));
    let monitoring_id = framework.register_agent(monitoring_agent).await?;
    framework.start_agent(&monitoring_id).await?;
    println!("✅ MonitoringAgent started with ID: {}", monitoring_id);

    println!("\n🔍 Step 2: Code Review and Security Analysis");
    println!("---------------------------------------------");

    // Perform code review
    let review_request = CodeReviewRequest {
        code_path: source_dir.to_string_lossy().to_string(),
        file_extensions: vec!["rs".to_string()],
        security_analysis: true,
        quality_analysis: true,
        performance_analysis: true,
        custom_rules: vec![],
        ignore_patterns: vec![],
    };

    let review_message = AgentMessage::TaskRequest(AgentRequest::new(
        "code_review".to_string(),
        serde_json::to_value(review_request).unwrap()
    ));

    framework.send_message(&code_review_id, review_message).await?;
    println!("📤 Code review request sent");

    // Wait for code review to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n🧪 Step 3: Test Generation and Execution");
    println!("----------------------------------------");

    // Generate tests
    let generation_request = TestGenerationRequest {
        source_path: source_dir.to_string_lossy().to_string(),
        file_extensions: vec!["rs".to_string()],
        test_types: vec!["unit".to_string(), "integration".to_string()],
        test_framework: "rust".to_string(),
        output_directory: test_dir.to_string_lossy().to_string(),
        options: HashMap::new(),
    };

    let generation_message = AgentMessage::TaskRequest(AgentRequest::new(
        "generate_tests".to_string(),
        serde_json::to_value(generation_request).unwrap()
    ));

    framework.send_message(&test_runner_id, generation_message).await?;
    println!("📤 Test generation request sent");

    // Wait for test generation to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Execute tests
    let execution_request = TestExecutionRequest {
        test_path: test_dir.to_string_lossy().to_string(),
        test_types: vec!["unit".to_string(), "integration".to_string()],
        test_framework: "rust".to_string(),
        filters: vec![],
        options: HashMap::new(),
        timeout: Some(30),
        parallel_count: Some(4),
    };

    let execution_message = AgentMessage::TaskRequest(AgentRequest::new(
        "execute_tests".to_string(),
        serde_json::to_value(execution_request).unwrap()
    ));

    framework.send_message(&test_runner_id, execution_message).await?;
    println!("📤 Test execution request sent");

    // Wait for test execution to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n📚 Step 4: Documentation Generation");
    println!("-----------------------------------");

    // Generate documentation
    let doc_config = DocumentationConfig {
        project_name: "Sample Rust Project".to_string(),
        version: "1.0.0".to_string(),
        doc_type: DocumentationType::API,
        output_format: OutputFormat::HTML,
        source_path: source_dir.to_string_lossy().to_string(),
        output_directory: docs_dir.to_string_lossy().to_string(),
        template_config: None,
        api_config: None,
        code_config: None,
        options: HashMap::new(),
    };

    let doc_request = DocumentationRequest {
        config: doc_config,
        force_regenerate: false,
        include_diagrams: true,
        options: HashMap::new(),
    };

    let doc_message = AgentMessage::TaskRequest(AgentRequest::new(
        "generate_documentation".to_string(),
        serde_json::to_value(doc_request).unwrap()
    ));

    framework.send_message(&documentation_id, doc_message).await?;
    println!("📤 Documentation generation request sent");

    // Wait for documentation generation to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n🚀 Step 5: Application Deployment");
    println!("--------------------------------");

    // Deploy application
    let deploy_config = DeploymentConfig {
        app_name: "sample-rust-app".to_string(),
        version: "1.0.0".to_string(),
        environment: DeploymentEnvironment::Development,
        container_config: None,
        infrastructure_config: None,
        pipeline_config: None,
        rollback_config: None,
        health_check_config: None,
    };

    let deploy_request = DeploymentRequest {
        config: deploy_config,
        source_path: source_dir.to_string_lossy().to_string(),
        artifacts_path: None,
        options: HashMap::new(),
    };

    let deploy_message = AgentMessage::TaskRequest(AgentRequest::new(
        "deploy".to_string(),
        serde_json::to_value(deploy_request).unwrap()
    ));

    framework.send_message(&deployment_id, deploy_message).await?;
    println!("📤 Deployment request sent");

    // Wait for deployment to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("\n📊 Step 6: System Monitoring and Alerting");
    println!("----------------------------------------");

    // Set up monitoring
    let mut thresholds = HashMap::new();
    thresholds.insert("cpu_usage".to_string(), Threshold {
        value: 80.0,
        operator: ThresholdOperator::GreaterThan,
        severity: AlertSeverity::Warning,
        message: "CPU usage is high".to_string(),
    });

    thresholds.insert("memory_usage".to_string(), Threshold {
        value: 90.0,
        operator: ThresholdOperator::GreaterThan,
        severity: AlertSeverity::Critical,
        message: "Memory usage is critical".to_string(),
    });

    let monitoring_config = MonitoringConfig {
        interval: 30, // 30 seconds
        metrics: vec![
            MetricType::CPU,
            MetricType::Memory,
            MetricType::Disk,
            MetricType::Network,
            MetricType::Process,
        ],
        thresholds,
        notifications: vec![],
        retention_days: 7,
        custom_rules: vec![],
    };

    let monitoring_request = MonitoringRequest {
        config: monitoring_config,
        targets: vec![],
        options: HashMap::new(),
    };

    let monitoring_message = AgentMessage::TaskRequest(AgentRequest::new(
        "start_monitoring".to_string(),
        serde_json::to_value(monitoring_request).unwrap()
    ));

    framework.send_message(&monitoring_id, monitoring_message).await?;
    println!("📤 Monitoring request sent");

    // Let monitoring run for a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    println!("\n📈 Step 7: Collecting Results and Metrics");
    println!("----------------------------------------");

    // Get monitoring history
    let monitoring_history_message = AgentMessage::TaskRequest(AgentRequest::new(
        "get_monitoring_history".to_string(),
        serde_json::Value::Null
    ));

    framework.send_message(&monitoring_id, monitoring_history_message).await?;

    // Get alert history
    let alert_history_message = AgentMessage::TaskRequest(AgentRequest::new(
        "get_alert_history".to_string(),
        serde_json::Value::Null
    ));

    framework.send_message(&monitoring_id, alert_history_message).await?;

    // Get deployment history
    let deployment_history_message = AgentMessage::TaskRequest(AgentRequest::new(
        "get_deployment_history".to_string(),
        serde_json::Value::Null
    ));

    framework.send_message(&deployment_id, deployment_history_message).await?;

    // Get documentation history
    let doc_history_message = AgentMessage::TaskRequest(AgentRequest::new(
        "get_documentation_history".to_string(),
        serde_json::Value::Null
    ));

    framework.send_message(&documentation_id, doc_history_message).await?;

    println!("\n✅ Comprehensive Agent Workflow Completed!");
    println!("==========================================");
    println!("📁 Project created in: {}", temp_dir.path().display());
    println!("🔍 Code review completed");
    println!("🧪 Tests generated and executed");
    println!("📚 Documentation generated");
    println!("🚀 Application deployed");
    println!("📊 System monitoring active");
    println!("\n🎉 All agents worked together successfully!");

    // Keep the framework running for a bit to see monitoring in action
    println!("\n⏳ Keeping framework alive for 30 seconds to observe monitoring...");
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    println!("\n🛑 Shutting down agent framework...");
    framework.shutdown().await?;

    Ok(())
}

/// Create a sample Rust project for demonstration
fn create_sample_project(source_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create Cargo.toml
    let cargo_toml = r#"[package]
name = "sample-rust-app"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }

[dev-dependencies]
tokio-test = "0.4"
"#;

    fs::write(source_dir.join("Cargo.toml"), cargo_toml)?;

    // Create main.rs
    let main_rs = r#"use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

/// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Get all users
async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];
    Json(users)
}

/// Create a new user
async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 3, // In a real app, this would be generated
        name: payload.name,
        email: payload.email,
    };
    (StatusCode::CREATED, Json(user))
}

/// Get user by ID
async fn get_user_by_id(axum::extract::Path(id): axum::extract::Path<u32>) -> Result<Json<User>, StatusCode> {
    if id == 1 {
        Ok(Json(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#;

    fs::write(source_dir.join("src").join("main.rs"), main_rs)?;

    // Create lib.rs
    let lib_rs = r#"//! Sample Rust library for demonstration

use serde::{Deserialize, Serialize};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: u32,
    /// User name
    pub name: String,
    /// User email
    pub email: String,
}

/// User service
pub struct UserService;

impl UserService {
    /// Create a new user service
    pub fn new() -> Self {
        Self
    }

    /// Get all users
    pub fn get_users(&self) -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ]
    }

    /// Get user by ID
    pub fn get_user_by_id(&self, id: u32) -> Option<User> {
        match id {
            1 => Some(User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            }),
            2 => Some(User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            }),
            _ => None,
        }
    }

    /// Create a new user
    pub fn create_user(&self, name: String, email: String) -> User {
        User {
            id: 3, // In a real app, this would be generated
            name,
            email,
        }
    }
}

/// Utility functions
pub mod utils {
    /// Validate email format
    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    /// Generate a random ID
    pub fn generate_id() -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        hasher.finish() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_service_get_users() {
        let service = UserService::new();
        let users = service.get_users();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "Alice");
        assert_eq!(users[1].name, "Bob");
    }

    #[test]
    fn test_user_service_get_user_by_id() {
        let service = UserService::new();
        let user = service.get_user_by_id(1);
        assert!(user.is_some());
        assert_eq!(user.unwrap().name, "Alice");
    }

    #[test]
    fn test_user_service_get_user_by_id_not_found() {
        let service = UserService::new();
        let user = service.get_user_by_id(999);
        assert!(user.is_none());
    }

    #[test]
    fn test_utils_validate_email() {
        assert!(utils::validate_email("test@example.com"));
        assert!(!utils::validate_email("invalid-email"));
    }
}
"#;

    fs::write(source_dir.join("src").join("lib.rs"), lib_rs)?;

    // Create README.md
    let readme_md = r#"# Sample Rust Application

This is a sample Rust application demonstrating a REST API with user management.

## Features

- REST API endpoints for user management
- Health check endpoint
- Comprehensive test coverage
- Documentation generation

## API Endpoints

- `GET /health` - Health check
- `GET /users` - Get all users
- `POST /users` - Create a new user
- `GET /users/:id` - Get user by ID

## Development

```bash
# Run tests
cargo test

# Run the application
cargo run

# Build for production
cargo build --release
```

## Testing

The application includes comprehensive tests for all functionality.

## Documentation

Documentation is automatically generated using the DocumentationAgent.
"#;

    fs::write(source_dir.join("README.md"), readme_md)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sample_project_creation() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(source_dir.join("src")).unwrap();

        assert!(create_sample_project(&source_dir).is_ok());

        // Check that files were created
        assert!(source_dir.join("Cargo.toml").exists());
        assert!(source_dir.join("src").join("main.rs").exists());
        assert!(source_dir.join("src").join("lib.rs").exists());
        assert!(source_dir.join("README.md").exists());
    }
} 