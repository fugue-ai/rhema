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
    RhemaAgentFramework, CodeReviewAgent, TestRunnerAgent,
    CodeReviewRequest, TestGenerationRequest, TestExecutionRequest,
    AgentRequest, AgentMessage
};
use std::collections::HashMap;
use tempfile::tempdir;
use std::fs;

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
    let code_review_agent = Box::new(CodeReviewAgent::new("code-review-1".to_string()));
    let code_review_id = framework.register_agent(code_review_agent).await?;
    framework.start_agent(&code_review_id).await?;
    println!("✅ CodeReviewAgent started with ID: {}", code_review_id);

    // Register and start TestRunnerAgent
    println!("🧪 Registering TestRunnerAgent...");
    let test_runner_agent = Box::new(TestRunnerAgent::new("test-runner-1".to_string()));
    let test_runner_id = framework.register_agent(test_runner_agent).await?;
    framework.start_agent(&test_runner_id).await?;
    println!("✅ TestRunnerAgent started with ID: {}", test_runner_id);

    // Perform code review
    println!("\n🔍 Performing code review...");
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

    // Wait a moment for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Generate tests
    println!("\n🧪 Generating tests...");
    let generation_request = TestGenerationRequest {
        source_path: source_dir.to_string_lossy().to_string(),
        file_extensions: vec!["rs".to_string()],
        test_types: vec![rhema_agent::TestType::Unit],
        test_framework: "rust".to_string(),
        output_directory: test_dir.to_string_lossy().to_string(),
        options: HashMap::new(),
    };

    let generation_message = AgentMessage::TaskRequest(AgentRequest::new(
        "generate_tests".to_string(),
        serde_json::to_value(generation_request).unwrap()
    ));

    framework.send_message(&test_runner_id, generation_message).await?;

    // Wait a moment for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Execute tests
    println!("\n🏃 Executing tests...");
    let execution_request = TestExecutionRequest {
        test_path: test_dir.to_string_lossy().to_string(),
        test_types: vec![rhema_agent::TestType::Unit],
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
    println!("  Tasks Completed: {}", code_review_metrics.task_count);
    println!("  Success Rate: {:.1}%", 
        if code_review_metrics.task_count > 0 {
            (code_review_metrics.success_count as f64 / code_review_metrics.task_count as f64) * 100.0
        } else {
            0.0
        }
    );

    println!("TestRunnerAgent Metrics:");
    println!("  Tasks Completed: {}", test_runner_metrics.task_count);
    println!("  Success Rate: {:.1}%", 
        if test_runner_metrics.task_count > 0 {
            (test_runner_metrics.success_count as f64 / test_runner_metrics.task_count as f64) * 100.0
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

        let code_review_agent = Box::new(CodeReviewAgent::new("test-code-review".to_string()));
        let code_review_id = framework.register_agent(code_review_agent).await.unwrap();
        
        let test_runner_agent = Box::new(TestRunnerAgent::new("test-test-runner".to_string()));
        let test_runner_id = framework.register_agent(test_runner_agent).await.unwrap();

        let stats = framework.get_framework_stats().await.unwrap();
        assert_eq!(stats.total_agents, 2);

        framework.shutdown().await.unwrap();
    }
} 