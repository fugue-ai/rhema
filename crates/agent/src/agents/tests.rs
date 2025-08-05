/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * you may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;
use crate::agent::{AgentRequest, AgentMessage};
use tempfile::tempdir;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod code_review_agent_tests {
    use super::*;

    #[tokio::test]
    async fn test_code_review_agent_creation() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        assert_eq!(agent.id(), "test-agent");
        assert_eq!(agent.config().name, "Code Review Agent");
        assert!(agent.has_capability(&AgentCapability::Security));
        assert!(agent.has_capability(&AgentCapability::Analysis));
    }

    #[tokio::test]
    async fn test_code_review_agent_initialization() {
        let mut agent = CodeReviewAgent::new("test-agent".to_string());
        assert!(agent.initialize().await.is_ok());
        assert_eq!(agent.context().state, AgentState::Ready);
    }

    #[tokio::test]
    async fn test_security_analysis_sql_injection() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        
        let code = r#"
            let user_input = "1; DROP TABLE users;";
            let query = "SELECT * FROM users WHERE id = " + user_input;
            execute(query);
        "#;
        
        let vulnerabilities = agent.perform_security_analysis(code, "test.rs").await;
        assert!(!vulnerabilities.is_empty());
        
        let sql_injection = vulnerabilities.iter()
            .find(|v| v.title.contains("SQL Injection"))
            .expect("Should find SQL injection vulnerability");
        
        assert_eq!(sql_injection.severity, SecuritySeverity::Critical);
        assert!(sql_injection.remediation.len() > 0);
    }

    #[tokio::test]
    async fn test_security_analysis_xss() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        
        let code = r#"
            let user_input = "<script>alert('xss')</script>";
            document.getElementById("output").innerHTML = user_input;
        "#;
        
        let vulnerabilities = agent.perform_security_analysis(code, "test.js").await;
        assert!(!vulnerabilities.is_empty());
        
        let xss = vulnerabilities.iter()
            .find(|v| v.title.contains("XSS"))
            .expect("Should find XSS vulnerability");
        
        assert_eq!(xss.severity, SecuritySeverity::High);
    }

    #[tokio::test]
    async fn test_security_analysis_hardcoded_credentials() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        
        let code = r#"
            let api_key = "sk-1234567890abcdef";
            let password = "super_secret_password_123";
        "#;
        
        let vulnerabilities = agent.perform_security_analysis(code, "test.js").await;
        assert!(!vulnerabilities.is_empty());
        
        let hardcoded = vulnerabilities.iter()
            .find(|v| v.title.contains("Hardcoded Credentials"))
            .expect("Should find hardcoded credentials vulnerability");
        
        assert_eq!(hardcoded.severity, SecuritySeverity::High);
    }

    #[tokio::test]
    async fn test_quality_analysis_magic_numbers() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        
        let code = r#"
            fn calculate_tax(amount: f64) -> f64 {
                return amount * 0.15; // Magic number
            }
        "#;
        
        let findings = agent.perform_quality_analysis(code, "test.rs").await;
        assert!(!findings.is_empty());
        
        let magic_number = findings.iter()
            .find(|f| f.title.contains("Magic Numbers"))
            .expect("Should find magic number issue");
        
        assert_eq!(magic_number.severity, SecuritySeverity::Low);
    }

    #[tokio::test]
    async fn test_security_score_calculation() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        
        let vulnerabilities = vec![
            SecurityVulnerability {
                id: "test1".to_string(),
                title: "Critical Vulnerability".to_string(),
                description: "Test".to_string(),
                severity: SecuritySeverity::Critical,
                file_path: "test.rs".to_string(),
                line_number: Some(1),
                code_snippet: Some("test".to_string()),
                cve_id: None,
                remediation: vec![],
                detection_method: "test".to_string(),
                detected_at: Utc::now(),
            },
            SecurityVulnerability {
                id: "test2".to_string(),
                title: "High Vulnerability".to_string(),
                description: "Test".to_string(),
                severity: SecuritySeverity::High,
                file_path: "test.rs".to_string(),
                line_number: Some(2),
                code_snippet: Some("test".to_string()),
                cve_id: None,
                remediation: vec![],
                detection_method: "test".to_string(),
                detected_at: Utc::now(),
            }
        ];

        let score = agent.calculate_security_score(&vulnerabilities);
        assert_eq!(score, 60.0); // 100 - 25 (critical) - 15 (high)
    }

    #[tokio::test]
    async fn test_code_review_integration() {
        let mut agent = CodeReviewAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        // Create temporary directory with test files
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("vulnerable.rs");
        
        fs::write(&test_file, r#"
            fn main() {
                let password = "hardcoded_password_123";
                let query = "SELECT * FROM users WHERE id = " + user_input;
                let script = "<script>alert('xss')</script>";
            }
        "#).unwrap();

        let request = CodeReviewRequest {
            code_path: temp_dir.path().to_string_lossy().to_string(),
            file_extensions: vec!["rs".to_string()],
            security_analysis: true,
            quality_analysis: true,
            performance_analysis: true,
            custom_rules: vec![],
            ignore_patterns: vec![],
        };

        let result = agent.perform_code_review(request).await;
        assert!(result.is_ok());

        let response: CodeReviewResponse = serde_json::from_value(result.unwrap()).unwrap();
        assert!(!response.security_vulnerabilities.is_empty());
        assert!(response.security_score < 100.0);
        assert_eq!(response.files_reviewed.len(), 1);
    }

    #[tokio::test]
    async fn test_agent_message_handling() {
        let mut agent = CodeReviewAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        let review_request = CodeReviewRequest {
            code_path: "test_path".to_string(),
            file_extensions: vec!["rs".to_string()],
            security_analysis: true,
            quality_analysis: false,
            performance_analysis: false,
            custom_rules: vec![],
            ignore_patterns: vec![],
        };

        let agent_request = AgentRequest::new(
            "code_review".to_string(),
            serde_json::to_value(review_request).unwrap()
        );

        let message = AgentMessage::TaskRequest(agent_request);
        let response = agent.handle_message(message).await.unwrap();
        
        assert!(response.is_some());
        if let Some(AgentMessage::TaskResponse(response)) = response {
            assert_eq!(response.status, crate::agent::ResponseStatus::Success);
        }
    }
}

#[cfg(test)]
mod test_runner_agent_tests {
    use super::*;

    #[tokio::test]
    async fn test_test_runner_agent_creation() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        assert_eq!(agent.id(), "test-agent");
        assert_eq!(agent.config().name, "Test Runner Agent");
        assert!(agent.has_capability(&AgentCapability::Testing));
        assert!(agent.has_capability(&AgentCapability::CodeExecution));
    }

    #[tokio::test]
    async fn test_test_runner_agent_initialization() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        assert!(agent.initialize().await.is_ok());
        assert_eq!(agent.context().state, AgentState::Ready);
    }

    #[tokio::test]
    async fn test_function_extraction_rust() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        let rust_code = "fn calculate_sum(a: i32, b: i32) -> i32 {";
        let function = agent.extract_function_info(rust_code, 0, "test.rs");
        assert!(function.is_some());
        
        let function = function.unwrap();
        assert_eq!(function.name, "calculate_sum");
        assert_eq!(function.parameters, vec!["a", "b"]);
        assert_eq!(function.return_type, Some("i32".to_string()));
    }

    #[tokio::test]
    async fn test_function_extraction_python() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        let python_code = "def calculate_sum(a, b):";
        let function = agent.extract_function_info(python_code, 0, "test.py");
        assert!(function.is_some());
        
        let function = function.unwrap();
        assert_eq!(function.name, "calculate_sum");
        assert_eq!(function.parameters, vec!["a", "b"]);
        assert_eq!(function.return_type, None);
    }

    #[tokio::test]
    async fn test_function_extraction_javascript() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        let js_code = "function calculateSum(a, b) {";
        let function = agent.extract_function_info(js_code, 0, "test.js");
        assert!(function.is_some());
        
        let function = function.unwrap();
        assert_eq!(function.name, "calculateSum");
        assert_eq!(function.parameters, vec!["a", "b"]);
        assert_eq!(function.return_type, None);
    }

    #[tokio::test]
    async fn test_test_generation_integration() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        // Create temporary directory with test files
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("calculator.rs");
        let output_dir = temp_dir.path().join("tests");
        
        fs::write(&source_file, r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            pub fn subtract(a: i32, b: i32) -> i32 {
                a - b
            }
            
            pub fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
        "#).unwrap();

        let request = TestGenerationRequest {
            source_path: temp_dir.path().to_string_lossy().to_string(),
            file_extensions: vec!["rs".to_string()],
            test_types: vec![TestType::Unit],
            test_framework: "rust".to_string(),
            output_directory: output_dir.to_string_lossy().to_string(),
            options: HashMap::new(),
        };

        let result = agent.generate_tests(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        let generated_files = response["generated_test_files"].as_array().unwrap();
        assert_eq!(generated_files.len(), 3); // Should generate tests for 3 functions
    }

    #[tokio::test]
    async fn test_test_execution_integration() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        let request = TestExecutionRequest {
            test_path: "tests".to_string(),
            test_types: vec![TestType::Unit, TestType::Integration],
            test_framework: "rust".to_string(),
            filters: vec![],
            options: HashMap::new(),
            timeout: Some(30),
            parallel_count: Some(4),
        };

        let result = agent.execute_tests(request).await;
        assert!(result.is_ok());

        let response: TestExecutionResponse = serde_json::from_value(result.unwrap()).unwrap();
        assert_eq!(response.total_tests, 1); // Default test suite has 1 test
        assert!(response.total_duration > 0);
    }

    #[tokio::test]
    async fn test_supported_frameworks() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        // Test that supported frameworks are initialized
        assert!(agent.supported_frameworks.contains_key("rust"));
        assert!(agent.supported_frameworks.contains_key("pytest"));
        assert!(agent.supported_frameworks.contains_key("jest"));
        
        let rust_framework = &agent.supported_frameworks["rust"];
        assert_eq!(rust_framework.name, "Rust Test Framework");
        assert!(rust_framework.supported_types.contains(&TestType::Unit));
        assert!(rust_framework.supported_types.contains(&TestType::Integration));
        
        let pytest_framework = &agent.supported_frameworks["pytest"];
        assert_eq!(pytest_framework.name, "Python pytest");
        assert!(pytest_framework.supported_types.contains(&TestType::Unit));
        assert!(pytest_framework.supported_types.contains(&TestType::Integration));
        assert!(pytest_framework.supported_types.contains(&TestType::Functional));
    }

    #[tokio::test]
    async fn test_test_templates() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        // Test that test templates are initialized
        assert!(agent.test_templates.contains_key("rust_unit"));
        assert!(agent.test_templates.contains_key("python_unit"));
        assert!(agent.test_templates.contains_key("javascript_unit"));
        
        let rust_template = &agent.test_templates["rust_unit"];
        assert!(rust_template.contains("#[test]"));
        assert!(rust_template.contains("{function_name}"));
        
        let python_template = &agent.test_templates["python_unit"];
        assert!(python_template.contains("unittest"));
        assert!(python_template.contains("{method_name}"));
    }

    #[tokio::test]
    async fn test_agent_message_handling() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        let generation_request = TestGenerationRequest {
            source_path: "test_path".to_string(),
            file_extensions: vec!["rs".to_string()],
            test_types: vec![TestType::Unit],
            test_framework: "rust".to_string(),
            output_directory: "test_output".to_string(),
            options: HashMap::new(),
        };

        let agent_request = AgentRequest::new(
            "generate_tests".to_string(),
            serde_json::to_value(generation_request).unwrap()
        );

        let message = AgentMessage::TaskRequest(agent_request);
        let response = agent.handle_message(message).await.unwrap();
        
        assert!(response.is_some());
        if let Some(AgentMessage::TaskResponse(response)) = response {
            assert_eq!(response.status, crate::agent::ResponseStatus::Success);
        }
    }

    #[tokio::test]
    async fn test_execution_summary_generation() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        
        let summary = agent.generate_execution_summary(10, 2, 1, 5000);
        assert!(summary.contains("10 tests total"));
        assert!(summary.contains("10 passed"));
        assert!(summary.contains("2 failed"));
        assert!(summary.contains("1 skipped"));
        assert!(summary.contains("5.00s"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::RhemaAgentFramework;

    #[tokio::test]
    async fn test_agent_framework_integration() {
        let mut framework = RhemaAgentFramework::new();
        framework.initialize().await.unwrap();

        // Register CodeReviewAgent
        let code_review_agent = Box::new(CodeReviewAgent::new("code-review-1".to_string()));
        let code_review_id = framework.register_agent(code_review_agent).await.unwrap();
        framework.start_agent(&code_review_id).await.unwrap();

        // Register TestRunnerAgent
        let test_runner_agent = Box::new(TestRunnerAgent::new("test-runner-1".to_string()));
        let test_runner_id = framework.register_agent(test_runner_agent).await.unwrap();
        framework.start_agent(&test_runner_id).await.unwrap();

        // Test agent communication
        let review_request = CodeReviewRequest {
            code_path: "test_path".to_string(),
            file_extensions: vec!["rs".to_string()],
            security_analysis: true,
            quality_analysis: false,
            performance_analysis: false,
            custom_rules: vec![],
            ignore_patterns: vec![],
        };

        let message = AgentMessage::TaskRequest(AgentRequest::new(
            "code_review".to_string(),
            serde_json::to_value(review_request).unwrap()
        ));

        framework.send_message(&code_review_id, message).await.unwrap();

        // Get framework stats
        let stats = framework.get_framework_stats().await.unwrap();
        assert_eq!(stats.total_agents, 2);
        assert_eq!(stats.active_agents, 2);

        framework.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_agent_capabilities() {
        let code_review_agent = CodeReviewAgent::new("code-review-1".to_string());
        let test_runner_agent = TestRunnerAgent::new("test-runner-1".to_string());

        // Test CodeReviewAgent capabilities
        assert!(code_review_agent.has_capability(&AgentCapability::Security));
        assert!(code_review_agent.has_capability(&AgentCapability::Analysis));
        assert!(code_review_agent.has_capability(&AgentCapability::FileRead));
        assert!(!code_review_agent.has_capability(&AgentCapability::Testing));

        // Test TestRunnerAgent capabilities
        assert!(test_runner_agent.has_capability(&AgentCapability::Testing));
        assert!(test_runner_agent.has_capability(&AgentCapability::CodeExecution));
        assert!(test_runner_agent.has_capability(&AgentCapability::FileRead));
        assert!(test_runner_agent.has_capability(&AgentCapability::FileWrite));
        assert!(!test_runner_agent.has_capability(&AgentCapability::Security));
    }

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let mut code_review_agent = CodeReviewAgent::new("code-review-1".to_string());
        
        // Test initialization
        assert!(code_review_agent.initialize().await.is_ok());
        assert_eq!(code_review_agent.context().state, AgentState::Ready);
        
        // Test start
        assert!(code_review_agent.start().await.is_ok());
        assert_eq!(code_review_agent.context().state, AgentState::Ready);
        
        // Test stop
        assert!(code_review_agent.stop().await.is_ok());
        assert_eq!(code_review_agent.context().state, AgentState::Stopped);
    }
} 