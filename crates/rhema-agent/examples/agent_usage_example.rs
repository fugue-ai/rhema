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
    Agent, AgentCapability, AgentConfig, AgentId, AgentMessage, AgentRequest, AgentType, BaseAgent,
    RhemaAgentFramework,
};
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the agent framework
    let mut framework = RhemaAgentFramework::new();
    framework.initialize().await?;

    println!("🚀 Rhema Agent Framework initialized successfully!");

    // Create temporary directory for demonstration
    let temp_dir = tempdir()?;
    let source_dir = temp_dir.path().join("source");
    let test_dir = temp_dir.path().join("tests");
    fs::create_dir_all(&source_dir)?;
    fs::create_dir_all(&test_dir)?;

    // Create sample source code with vulnerabilities
    let vulnerable_code = r#"
        // Sample vulnerable code for demonstration
        pub fn unsafe_query(user_input: &str) -> String {
            let query = format!("SELECT * FROM users WHERE id = {}", user_input);
            execute_query(&query)
        }

        pub fn unsafe_render(user_input: &str) -> String {
            format!("<div>{}</div>", user_input)
        }

        pub fn hardcoded_auth() -> bool {
            let api_key = "sk-1234567890abcdef";
            let password = "super_secret_password_123";
            authenticate(api_key, password)
        }

        pub fn calculate_tax(amount: f64) -> f64 {
            amount * 0.15 // Magic number
        }
    "#;

    fs::write(source_dir.join("vulnerable.rs"), vulnerable_code)?;

    // Register and start CodeReviewAgent
    println!("🔍 Registering CodeReviewAgent...");
    let code_review_config = AgentConfig {
        name: "code-review-1".to_string(),
        description: Some("Code review agent".to_string()),
        agent_type: AgentType::Security,
        capabilities: vec![AgentCapability::Analysis, AgentCapability::Security],
        max_concurrent_tasks: 1,
        task_timeout: 300,
        memory_limit: Some(512),
        cpu_limit: Some(50.0),
        retry_attempts: 3,
        retry_delay: 5,
        parameters: HashMap::new(),
        tags: vec!["code-review".to_string(), "security".to_string()],
    };
    let code_review_agent = Box::new(BaseAgent::new(
        "code-review-1".to_string(),
        code_review_config,
    ));
    let code_review_id = framework.register_agent(code_review_agent).await?;
    framework.start_agent(&code_review_id).await?;
    println!("✅ CodeReviewAgent started with ID: {}", code_review_id);

    // Register and start TestRunnerAgent
    println!("🧪 Registering TestRunnerAgent...");
    let test_runner_config = AgentConfig {
        name: "test-runner-1".to_string(),
        description: Some("Test runner agent".to_string()),
        agent_type: AgentType::Testing,
        capabilities: vec![AgentCapability::Testing, AgentCapability::CodeExecution],
        max_concurrent_tasks: 2,
        task_timeout: 600,
        memory_limit: Some(1024),
        cpu_limit: Some(75.0),
        retry_attempts: 2,
        retry_delay: 10,
        parameters: HashMap::new(),
        tags: vec!["testing".to_string(), "automation".to_string()],
    };
    let test_runner_agent = Box::new(BaseAgent::new(
        "test-runner-1".to_string(),
        test_runner_config,
    ));
    let test_runner_id = framework.register_agent(test_runner_agent).await?;
    framework.start_agent(&test_runner_id).await?;
    println!("✅ TestRunnerAgent started with ID: {}", test_runner_id);

    // Perform code review
    println!("\n🔍 Performing code review...");
    let review_request = serde_json::json!({
        "code_path": source_dir.to_string_lossy().to_string(),
        "file_extensions": vec!["rs".to_string()],
        "security_analysis": true,
        "quality_analysis": true,
        "performance_analysis": true,
        "custom_rules": Vec::<String>::new(),
        "ignore_patterns": Vec::<String>::new(),
    });

    let review_message =
        AgentMessage::TaskRequest(AgentRequest::new("code_review".to_string(), review_request));

    framework
        .send_message(&code_review_id, review_message)
        .await?;

    // Wait a moment for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Generate tests
    println!("\n🧪 Generating tests...");
    let generation_request = serde_json::json!({
        "source_path": source_dir.to_string_lossy().to_string(),
        "file_extensions": Vec::<String>::new(),
        "test_types": Vec::<String>::new(),
        "test_framework": "rust",
        "output_directory": test_dir.to_string_lossy().to_string(),
        "options": HashMap::<String, String>::new(),
    });

    let generation_message = AgentMessage::TaskRequest(AgentRequest::new(
        "generate_tests".to_string(),
        generation_request,
    ));

    framework
        .send_message(&test_runner_id, generation_message)
        .await?;

    // Wait a moment for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Execute tests
    println!("\n🏃 Executing tests...");
    let execution_request = serde_json::json!({
        "test_path": test_dir.to_string_lossy().to_string(),
        "test_types": Vec::<String>::new(),
        "test_framework": "rust",
        "filters": Vec::<String>::new(),
        "options": HashMap::<String, String>::new(),
        "timeout": 30,
        "parallel_count": 4,
    });

    let execution_message = AgentMessage::TaskRequest(AgentRequest::new(
        "execute_tests".to_string(),
        execution_request,
    ));

    framework
        .send_message(&test_runner_id, execution_message)
        .await?;

    // Wait a moment for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Get framework statistics
    println!("\n📊 Getting framework statistics...");
    let stats = framework.get_framework_stats().await?;
    println!("Framework Statistics:");
    println!("  Total Agents: {}", stats.total_agents);
    println!("  Active Agents: {}", stats.active_agents);
    println!("  Total Messages: {}", stats.total_messages);
    println!("  Coordination Sessions: {}", stats.coordination_sessions);
    println!("  Policy Violations: {}", stats.policy_violations);

    // Get agent metrics
    println!("\n📈 Getting agent metrics...");
    let code_review_metrics = framework.get_agent_metrics(&code_review_id).await?;
    let test_runner_metrics = framework.get_agent_metrics(&test_runner_id).await?;

    println!("CodeReviewAgent Metrics:");
    println!(
        "  Tasks Completed: {}",
        code_review_metrics.tasks.total_tasks
    );
    println!(
        "  Success Rate: {:.1}%",
        if code_review_metrics.tasks.total_tasks > 0 {
            (code_review_metrics.tasks.successful_tasks as f64
                / code_review_metrics.tasks.total_tasks as f64)
                * 100.0
        } else {
            0.0
        }
    );

    println!("TestRunnerAgent Metrics:");
    println!(
        "  Tasks Completed: {}",
        test_runner_metrics.tasks.total_tasks
    );
    println!(
        "  Success Rate: {:.1}%",
        if test_runner_metrics.tasks.total_tasks > 0 {
            (test_runner_metrics.tasks.successful_tasks as f64
                / test_runner_metrics.tasks.total_tasks as f64)
                * 100.0
        } else {
            0.0
        }
    );

    // Shutdown the framework
    println!("\n🛑 Shutting down framework...");
    framework.shutdown().await?;
    println!("✅ Framework shutdown complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_framework_initialization() {
        let mut framework = RhemaAgentFramework::new();
        assert!(framework.initialize().await.is_ok());

        let stats = framework.get_framework_stats().await.unwrap();
        assert_eq!(stats.total_agents, 0);
        assert_eq!(stats.active_agents, 0);

        framework.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let mut framework = RhemaAgentFramework::new();
        framework.initialize().await.unwrap();

        let code_review_config = AgentConfig {
            name: "test-code-review".to_string(),
            description: Some("Test code review agent".to_string()),
            agent_type: AgentType::Security,
            capabilities: vec![AgentCapability::Analysis, AgentCapability::Security],
            max_concurrent_tasks: 1,
            task_timeout: 300,
            memory_limit: Some(512),
            cpu_limit: Some(50.0),
            retry_attempts: 3,
            retry_delay: 5,
            parameters: HashMap::new(),
            tags: vec!["test".to_string()],
        };
        let code_review_agent = Box::new(BaseAgent::new(
            "test-code-review".to_string(),
            code_review_config,
        ));
        let code_review_id = framework.register_agent(code_review_agent).await.unwrap();

        let test_runner_config = AgentConfig {
            name: "test-test-runner".to_string(),
            description: Some("Test test runner agent".to_string()),
            agent_type: AgentType::Testing,
            capabilities: vec![AgentCapability::Testing, AgentCapability::CodeExecution],
            max_concurrent_tasks: 2,
            task_timeout: 600,
            memory_limit: Some(1024),
            cpu_limit: Some(75.0),
            retry_attempts: 2,
            retry_delay: 10,
            parameters: HashMap::new(),
            tags: vec!["test".to_string()],
        };
        let test_runner_agent = Box::new(BaseAgent::new(
            "test-test-runner".to_string(),
            test_runner_config,
        ));
        let test_runner_id = framework.register_agent(test_runner_agent).await.unwrap();

        let stats = framework.get_framework_stats().await.unwrap();
        assert_eq!(stats.total_agents, 2);

        framework.shutdown().await.unwrap();
    }
}
