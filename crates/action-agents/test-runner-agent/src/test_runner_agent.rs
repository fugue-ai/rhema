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

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rhema_agent::agent::{
    Agent, AgentCapability, AgentConfig, AgentContext, AgentId, AgentMessage, AgentRequest,
    AgentResponse, AgentState, AgentStatus, AgentType, BaseAgent, HealthStatus, ResourceUsage,
};
use rhema_agent::error::{AgentError, AgentResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Test types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    Functional,
    Performance,
    Security,
    Regression,
    Smoke,
    Custom(String),
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Unit => write!(f, "Unit"),
            TestType::Integration => write!(f, "Integration"),
            TestType::Functional => write!(f, "Functional"),
            TestType::Performance => write!(f, "Performance"),
            TestType::Security => write!(f, "Security"),
            TestType::Regression => write!(f, "Regression"),
            TestType::Smoke => write!(f, "Smoke"),
            TestType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Test status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    Pending,
    Running,
    Passed,
    Failed,
    Skipped,
    Timeout,
    Error,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestStatus::Pending => write!(f, "Pending"),
            TestStatus::Running => write!(f, "Running"),
            TestStatus::Passed => write!(f, "Passed"),
            TestStatus::Failed => write!(f, "Failed"),
            TestStatus::Skipped => write!(f, "Skipped"),
            TestStatus::Timeout => write!(f, "Timeout"),
            TestStatus::Error => write!(f, "Error"),
        }
    }
}

/// Test result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test ID
    pub id: String,
    /// Test name
    pub name: String,
    /// Test type
    pub test_type: TestType,
    /// Test status
    pub status: TestStatus,
    /// Test duration in milliseconds
    pub duration: Option<u64>,
    /// Test output
    pub output: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Stack trace if available
    pub stack_trace: Option<String>,
    /// Test metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when test started
    pub started_at: DateTime<Utc>,
    /// Timestamp when test completed
    pub completed_at: Option<DateTime<Utc>>,
}

/// Test suite information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Suite ID
    pub id: String,
    /// Suite name
    pub name: String,
    /// Test results in this suite
    pub tests: Vec<TestResult>,
    /// Suite status
    pub status: TestStatus,
    /// Total test count
    pub total_tests: usize,
    /// Passed test count
    pub passed_tests: usize,
    /// Failed test count
    pub failed_tests: usize,
    /// Skipped test count
    pub skipped_tests: usize,
    /// Suite duration in milliseconds
    pub duration: Option<u64>,
    /// Suite timestamp
    pub timestamp: DateTime<Utc>,
}

/// Test generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationRequest {
    /// Path to the source code
    pub source_path: String,
    /// File extensions to analyze (e.g., ["rs", "py", "js"])
    pub file_extensions: Vec<String>,
    /// Test types to generate
    pub test_types: Vec<TestType>,
    /// Test framework to use
    pub test_framework: String,
    /// Output directory for generated tests
    pub output_directory: String,
    /// Test generation options
    pub options: HashMap<String, serde_json::Value>,
}

/// Test execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionRequest {
    /// Path to test files or test suite
    pub test_path: String,
    /// Test types to run
    pub test_types: Vec<TestType>,
    /// Test framework to use
    pub test_framework: String,
    /// Test filters (e.g., test names, tags)
    pub filters: Vec<String>,
    /// Execution options
    pub options: HashMap<String, serde_json::Value>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
    /// Parallel execution count
    pub parallel_count: Option<usize>,
}

/// Test execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionResponse {
    /// Execution ID
    pub execution_id: String,
    /// Test suites executed
    pub test_suites: Vec<TestSuite>,
    /// Overall execution status
    pub status: TestStatus,
    /// Total test count
    pub total_tests: usize,
    /// Passed test count
    pub passed_tests: usize,
    /// Failed test count
    pub failed_tests: usize,
    /// Skipped test count
    pub skipped_tests: usize,
    /// Total execution time in milliseconds
    pub total_duration: u64,
    /// Execution summary
    pub summary: String,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
}

/// Test Runner Agent for test generation and execution
pub struct TestRunnerAgent {
    /// Base agent functionality
    base: BaseAgent,
    /// Supported test frameworks
    supported_frameworks: HashMap<String, TestFramework>,
    /// Test generation templates
    test_templates: HashMap<String, String>,
    /// Test execution history
    execution_history: Vec<TestExecutionResponse>,
    /// Test generation history
    generation_history: Vec<String>,
    /// Test analysis engine
    test_analyzer: TestAnalyzer,
}

/// Test framework information
#[derive(Debug, Clone)]
struct TestFramework {
    /// Framework name
    name: String,
    /// Framework version
    version: String,
    /// Supported test types
    supported_types: Vec<TestType>,
    /// Command to run tests
    test_command: String,
    /// Command to generate tests
    generate_command: Option<String>,
    /// Configuration file
    config_file: Option<String>,
}

/// Test analyzer for code analysis and test generation
#[derive(Debug, Clone)]
struct TestAnalyzer {
    /// Code analysis patterns
    analysis_patterns: HashMap<String, String>,
    /// Test generation strategies
    generation_strategies: HashMap<String, TestGenerationStrategy>,
}

/// Test generation strategy
#[derive(Debug, Clone)]
struct TestGenerationStrategy {
    /// Strategy name
    name: String,
    /// Strategy description
    description: String,
    /// Template to use
    template: String,
    /// Applicable test types
    test_types: Vec<TestType>,
    /// Generation rules
    rules: Vec<String>,
}

impl TestRunnerAgent {
    /// Create a new Test Runner Agent
    pub fn new(id: AgentId) -> Self {
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
            task_timeout: 600,        // 10 minutes
            memory_limit: Some(1024), // 1 GB
            cpu_limit: Some(75.0),    // 75% CPU
            retry_attempts: 2,
            retry_delay: 5,
            parameters: HashMap::new(),
            tags: vec![
                "testing".to_string(),
                "test-generation".to_string(),
                "test-execution".to_string(),
            ],
        };

        let mut agent = Self {
            base: BaseAgent::new(id, config),
            supported_frameworks: HashMap::new(),
            test_templates: HashMap::new(),
            execution_history: Vec::new(),
            generation_history: Vec::new(),
            test_analyzer: TestAnalyzer::new(),
        };

        // Initialize supported frameworks
        agent.initialize_frameworks();
        agent.initialize_templates();

        agent
    }

    /// Initialize supported test frameworks
    fn initialize_frameworks(&mut self) {
        // Rust test framework
        self.supported_frameworks.insert(
            "rust".to_string(),
            TestFramework {
                name: "Rust Test Framework".to_string(),
                version: "1.0".to_string(),
                supported_types: vec![TestType::Unit, TestType::Integration],
                test_command: "cargo test".to_string(),
                generate_command: None, // Rust doesn't have auto-generation
                config_file: Some("Cargo.toml".to_string()),
            },
        );

        // Python pytest framework
        self.supported_frameworks.insert(
            "pytest".to_string(),
            TestFramework {
                name: "Python pytest".to_string(),
                version: "7.0".to_string(),
                supported_types: vec![TestType::Unit, TestType::Integration, TestType::Functional],
                test_command: "pytest".to_string(),
                generate_command: Some("pytest --genscript".to_string()),
                config_file: Some("pytest.ini".to_string()),
            },
        );

        // JavaScript Jest framework
        self.supported_frameworks.insert(
            "jest".to_string(),
            TestFramework {
                name: "JavaScript Jest".to_string(),
                version: "29.0".to_string(),
                supported_types: vec![TestType::Unit, TestType::Integration],
                test_command: "npm test".to_string(),
                generate_command: Some("jest --generate".to_string()),
                config_file: Some("jest.config.js".to_string()),
            },
        );
    }

    /// Initialize test templates
    fn initialize_templates(&mut self) {
        // Rust unit test template
        self.test_templates.insert(
            "rust_unit".to_string(),
            r#"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{function_name}() {{
        // Arrange
        {setup_code}
        
        // Act
        let result = {function_call};
        
        // Assert
        {assertion_code}
    }}
}}
"#
            .to_string(),
        );

        // Python unit test template
        self.test_templates.insert(
            "python_unit".to_string(),
            r#"
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

if __name__ == '__main__':
    unittest.main()
"#
            .to_string(),
        );

        // JavaScript unit test template
        self.test_templates.insert(
            "javascript_unit".to_string(),
            r#"
const { {function_name} } = require('./{module_name}');

describe('{class_name}', () => {{
    beforeEach(() => {{
        {setup_code}
    }});
    
    test('should {test_description}', () => {{
        // Arrange
        {setup_code}
        
        // Act
        const result = {function_call};
        
        // Assert
        {assertion_code}
    }});
}});
"#
            .to_string(),
        );
    }

    /// Generate tests for source code
    async fn generate_tests(
        &mut self,
        request: TestGenerationRequest,
    ) -> AgentResult<serde_json::Value> {
        info!("Starting test generation for path: {}", request.source_path);

        let files = self
            .get_source_files(&request.source_path, &request.file_extensions)
            .await?;
        let mut generated_tests = Vec::new();

        for file_path in &files {
            debug!("Analyzing file for test generation: {}", file_path);

            let content = self.read_file_content(file_path).await?;
            let testable_functions = self.analyze_code_for_testing(&content, file_path).await?;

            for function in testable_functions {
                let test_code = self
                    .generate_test_for_function(&function, &request.test_framework)
                    .await?;
                generated_tests.push(test_code);
            }
        }

        // Write generated tests to output directory
        let output_files = self
            .write_generated_tests(&generated_tests, &request.output_directory)
            .await?;

        let generation_result = serde_json::json!({
            "generation_id": Uuid::new_v4().to_string(),
            "source_files_analyzed": files,
            "testable_functions_found": generated_tests.len(),
            "generated_test_files": output_files,
            "test_framework": request.test_framework,
            "generated_at": Utc::now(),
        });

        // Store in generation history
        self.generation_history.push(
            generation_result["generation_id"]
                .as_str()
                .unwrap()
                .to_string(),
        );

        info!(
            "Test generation completed. Generated {} test files",
            output_files.len()
        );

        Ok(generation_result)
    }

    /// Execute tests
    async fn execute_tests(
        &mut self,
        request: TestExecutionRequest,
    ) -> AgentResult<serde_json::Value> {
        info!("Starting test execution for path: {}", request.test_path);

        let framework = self.get_test_framework(&request.test_framework)?;
        let test_suites = self
            .discover_test_suites(&request.test_path, &request.test_types)
            .await?;
        let mut execution_results = Vec::new();

        for suite in test_suites {
            debug!("Executing test suite: {}", suite.name);

            let suite_result = self
                .execute_test_suite(&suite, &framework, &request)
                .await?;
            execution_results.push(suite_result);
        }

        let total_tests = execution_results.iter().map(|s| s.total_tests).sum();
        let passed_tests = execution_results.iter().map(|s| s.passed_tests).sum();
        let failed_tests = execution_results.iter().map(|s| s.failed_tests).sum();
        let skipped_tests = execution_results.iter().map(|s| s.skipped_tests).sum();
        let total_duration = execution_results
            .iter()
            .map(|s| s.duration.unwrap_or(0))
            .sum();

        let overall_status = if failed_tests > 0 {
            TestStatus::Failed
        } else if passed_tests > 0 {
            TestStatus::Passed
        } else {
            TestStatus::Skipped
        };

        let response = TestExecutionResponse {
            execution_id: Uuid::new_v4().to_string(),
            test_suites: execution_results,
            status: overall_status,
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            total_duration,
            summary: self.generate_execution_summary(
                passed_tests,
                failed_tests,
                skipped_tests,
                total_duration,
            ),
            executed_at: Utc::now(),
        };

        // Store in execution history
        self.execution_history.push(response.clone());

        info!(
            "Test execution completed. {} passed, {} failed, {} skipped",
            passed_tests, failed_tests, skipped_tests
        );

        Ok(
            serde_json::to_value(response).map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?,
        )
    }

    /// Analyze code for testable functions
    async fn analyze_code_for_testing(
        &self,
        code: &str,
        file_path: &str,
    ) -> AgentResult<Vec<TestableFunction>> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        // Simple pattern matching for function detection
        // This is a basic implementation - in a real scenario, you'd use proper AST parsing
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(function) = self.extract_function_info(line, line_num, file_path) {
                functions.push(function);
            }
        }

        Ok(functions)
    }

    /// Extract function information from a line
    fn extract_function_info(
        &self,
        line: &str,
        line_num: usize,
        file_path: &str,
    ) -> Option<TestableFunction> {
        // Rust function pattern
        if let Some(caps) = regex::Regex::new(r"fn\s+(\w+)\s*\([^)]*\)\s*(?:->\s*[^{]+)?\s*\{")
            .unwrap()
            .captures(line)
        {
            return Some(TestableFunction {
                name: caps[1].to_string(),
                line_number: line_num + 1,
                file_path: file_path.to_string(),
                function_type: "function".to_string(),
                parameters: self.extract_parameters(line),
                return_type: self.extract_return_type(line),
            });
        }

        // Python function pattern
        if let Some(caps) = regex::Regex::new(r"def\s+(\w+)\s*\([^)]*\)\s*:")
            .unwrap()
            .captures(line)
        {
            return Some(TestableFunction {
                name: caps[1].to_string(),
                line_number: line_num + 1,
                file_path: file_path.to_string(),
                function_type: "function".to_string(),
                parameters: self.extract_parameters(line),
                return_type: None,
            });
        }

        // JavaScript function pattern
        if let Some(caps) = regex::Regex::new(r"(?:function\s+)?(\w+)\s*\([^)]*\)\s*\{")
            .unwrap()
            .captures(line)
        {
            return Some(TestableFunction {
                name: caps[1].to_string(),
                line_number: line_num + 1,
                file_path: file_path.to_string(),
                function_type: "function".to_string(),
                parameters: self.extract_parameters(line),
                return_type: None,
            });
        }

        None
    }

    /// Extract parameters from function signature
    fn extract_parameters(&self, line: &str) -> Vec<String> {
        if let Some(caps) = regex::Regex::new(r"\(([^)]*)\)").unwrap().captures(line) {
            caps[1]
                .split(',')
                .map(|p| p.trim().split(':').next().unwrap_or("").trim().to_string())
                .filter(|p| !p.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Extract return type from function signature
    fn extract_return_type(&self, line: &str) -> Option<String> {
        if let Some(caps) = regex::Regex::new(r"->\s*([^{]+)").unwrap().captures(line) {
            Some(caps[1].trim().to_string())
        } else {
            None
        }
    }

    /// Generate test for a specific function
    async fn generate_test_for_function(
        &self,
        function: &TestableFunction,
        framework: &str,
    ) -> AgentResult<GeneratedTest> {
        let template_key = format!("{}_unit", framework);
        let template =
            self.test_templates
                .get(&template_key)
                .ok_or_else(|| AgentError::ValidationError {
                    reason: format!("No template found for framework: {}", framework),
                })?;

        let test_code = template
            .replace("{function_name}", &function.name)
            .replace("{class_name}", &function.name.to_uppercase())
            .replace("{method_name}", &function.name)
            .replace("{module_name}", "module")
            .replace("{setup_code}", "// TODO: Add setup code")
            .replace(
                "{function_call}",
                &format!("{}(/* TODO: Add parameters */)", function.name),
            )
            .replace(
                "{method_call}",
                &format!("self.{}(/* TODO: Add parameters */)", function.name),
            )
            .replace("{assertion_code}", "// TODO: Add assertions")
            .replace("{test_description}", &format!("test {}", function.name));

        Ok(GeneratedTest {
            function_name: function.name.clone(),
            test_code,
            framework: framework.to_string(),
            test_type: TestType::Unit,
        })
    }

    /// Write generated tests to files
    async fn write_generated_tests(
        &self,
        tests: &[GeneratedTest],
        output_dir: &str,
    ) -> AgentResult<Vec<String>> {
        use std::fs;
        use std::path::Path;

        let mut output_files = Vec::new();

        // Create output directory if it doesn't exist
        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir).map_err(|e| AgentError::StorageError {
                reason: format!("Failed to create output directory: {}", e),
            })?;
        }

        for test in tests {
            let filename = format!("test_{}.rs", test.function_name);
            let filepath = Path::new(output_dir).join(&filename);

            fs::write(&filepath, &test.test_code).map_err(|e| AgentError::StorageError {
                reason: format!("Failed to write test file: {}", e),
            })?;

            output_files.push(filepath.to_string_lossy().to_string());
        }

        Ok(output_files)
    }

    /// Execute a test suite
    async fn execute_test_suite(
        &self,
        suite: &TestSuite,
        framework: &TestFramework,
        request: &TestExecutionRequest,
    ) -> AgentResult<TestSuite> {
        let start_time = std::time::Instant::now();
        let mut results = Vec::new();

        for test in &suite.tests {
            let test_result = self.execute_single_test(test, framework, request).await?;
            results.push(test_result);
        }

        let duration = start_time.elapsed().as_millis() as u64;
        let passed_tests = results
            .iter()
            .filter(|t| t.status == TestStatus::Passed)
            .count();
        let failed_tests = results
            .iter()
            .filter(|t| t.status == TestStatus::Failed)
            .count();
        let skipped_tests = results
            .iter()
            .filter(|t| t.status == TestStatus::Skipped)
            .count();

        let status = if failed_tests > 0 {
            TestStatus::Failed
        } else if passed_tests > 0 {
            TestStatus::Passed
        } else {
            TestStatus::Skipped
        };

        Ok(TestSuite {
            id: suite.id.clone(),
            name: suite.name.clone(),
            tests: results,
            status,
            total_tests: suite.total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            duration: Some(duration),
            timestamp: Utc::now(),
        })
    }

    /// Execute a single test
    async fn execute_single_test(
        &self,
        test: &TestResult,
        framework: &TestFramework,
        request: &TestExecutionRequest,
    ) -> AgentResult<TestResult> {
        let start_time = std::time::Instant::now();
        let mut result = test.clone();
        result.status = TestStatus::Running;
        result.started_at = Utc::now();

        // Simulate test execution
        // In a real implementation, you would actually run the test command
        let execution_result = self
            .run_test_command(&framework.test_command, &test.name)
            .await?;

        let duration = start_time.elapsed().as_millis() as u64;
        result.duration = Some(duration);
        result.completed_at = Some(Utc::now());

        // Parse execution result to determine status
        if execution_result.success {
            result.status = TestStatus::Passed;
            result.output = Some(execution_result.output);
        } else {
            result.status = TestStatus::Failed;
            result.error_message = Some(execution_result.error);
        }

        Ok(result)
    }

    /// Run test command
    async fn run_test_command(&self, command: &str, test_name: &str) -> AgentResult<CommandResult> {
        use tokio::process::Command;

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", command, test_name))
            .output()
            .await
            .map_err(|e| AgentError::ExecutionFailed {
                reason: format!("Failed to execute test command: {}", e),
            })?;

        Ok(CommandResult {
            success: output.status.success(),
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            error: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    /// Get source files to analyze
    async fn get_source_files(
        &self,
        source_path: &str,
        extensions: &[String],
    ) -> AgentResult<Vec<String>> {
        use std::fs;
        use std::path::Path;

        let mut files = Vec::new();

        if let Ok(entries) = fs::read_dir(source_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext_str = extension.to_string_lossy();
                            if extensions.contains(&ext_str.to_string()) {
                                if let Some(path_str) = path.to_str() {
                                    files.push(path_str.to_string());
                                }
                            }
                        }
                    } else if path.is_dir() {
                        // Recursively scan subdirectories
                        let sub_files =
                            Box::pin(self.get_source_files(path.to_str().unwrap(), extensions))
                                .await?;
                        files.extend(sub_files);
                    }
                }
            }
        }

        Ok(files)
    }

    /// Discover test suites
    async fn discover_test_suites(
        &self,
        test_path: &str,
        test_types: &[TestType],
    ) -> AgentResult<Vec<TestSuite>> {
        // This is a simplified implementation
        // In a real scenario, you would scan for test files and parse them
        let suite = TestSuite {
            id: Uuid::new_v4().to_string(),
            name: "Default Test Suite".to_string(),
            tests: vec![TestResult {
                id: Uuid::new_v4().to_string(),
                name: "sample_test".to_string(),
                test_type: TestType::Unit,
                status: TestStatus::Pending,
                duration: None,
                output: None,
                error_message: None,
                stack_trace: None,
                metadata: HashMap::new(),
                started_at: Utc::now(),
                completed_at: None,
            }],
            status: TestStatus::Pending,
            total_tests: 1,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration: None,
            timestamp: Utc::now(),
        };

        Ok(vec![suite])
    }

    /// Get test framework
    fn get_test_framework(&self, framework_name: &str) -> AgentResult<&TestFramework> {
        self.supported_frameworks
            .get(framework_name)
            .ok_or_else(|| AgentError::ValidationError {
                reason: format!("Unsupported test framework: {}", framework_name),
            })
    }

    /// Read file content
    async fn read_file_content(&self, file_path: &str) -> AgentResult<String> {
        use std::fs;
        use std::io::Read;

        let mut file = fs::File::open(file_path).map_err(|e| AgentError::StorageError {
            reason: format!("Failed to open file {}: {}", file_path, e),
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AgentError::StorageError {
                reason: format!("Failed to read file {}: {}", file_path, e),
            })?;

        Ok(content)
    }

    /// Generate execution summary
    fn generate_execution_summary(
        &self,
        passed: usize,
        failed: usize,
        skipped: usize,
        duration: u64,
    ) -> String {
        let total = passed + failed + skipped;
        let success_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        format!(
            "Test execution completed in {:.2}s. {} tests total: {} passed ({:.1}%), {} failed, {} skipped.",
            duration as f64 / 1000.0,
            total,
            passed,
            success_rate,
            failed,
            skipped
        )
    }
}

impl TestAnalyzer {
    fn new() -> Self {
        Self {
            analysis_patterns: HashMap::new(),
            generation_strategies: HashMap::new(),
        }
    }
}

/// Testable function information
#[derive(Debug, Clone)]
struct TestableFunction {
    /// Function name
    name: String,
    /// Line number where function is defined
    line_number: usize,
    /// File path containing the function
    file_path: String,
    /// Function type (function, method, etc.)
    function_type: String,
    /// Function parameters
    parameters: Vec<String>,
    /// Return type (if applicable)
    return_type: Option<String>,
}

/// Generated test information
#[derive(Debug, Clone)]
struct GeneratedTest {
    /// Function name being tested
    function_name: String,
    /// Generated test code
    test_code: String,
    /// Test framework used
    framework: String,
    /// Test type
    test_type: TestType,
}

/// Command execution result
#[derive(Debug, Clone)]
struct CommandResult {
    /// Whether the command succeeded
    success: bool,
    /// Command output
    output: String,
    /// Command error
    error: String,
}

#[async_trait]
impl Agent for TestRunnerAgent {
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
        info!("Initializing Test Runner Agent: {}", self.id());
        self.base.initialize().await?;
        info!("Test Runner Agent initialized successfully");
        Ok(())
    }

    async fn start(&mut self) -> AgentResult<()> {
        info!("Starting Test Runner Agent: {}", self.id());
        self.base.start().await?;
        info!("Test Runner Agent started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> AgentResult<()> {
        info!("Stopping Test Runner Agent: {}", self.id());
        self.base.stop().await?;
        info!("Test Runner Agent stopped successfully");
        Ok(())
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
        let start_time = std::time::Instant::now();
        self.update_state(AgentState::Busy);
        self.set_current_task(Some(request.id.clone()));

        let result = match request.request_type.as_str() {
            "generate_tests" => {
                if let Ok(generation_request) =
                    serde_json::from_value::<TestGenerationRequest>(request.payload)
                {
                    self.generate_tests(generation_request).await
                } else {
                    Err(AgentError::ValidationError {
                        reason: "Invalid test generation request format".to_string(),
                    })
                }
            }
            "execute_tests" => {
                if let Ok(execution_request) =
                    serde_json::from_value::<TestExecutionRequest>(request.payload)
                {
                    self.execute_tests(execution_request).await
                } else {
                    Err(AgentError::ValidationError {
                        reason: "Invalid test execution request format".to_string(),
                    })
                }
            }
            _ => Err(AgentError::ValidationError {
                reason: format!("Unknown request type: {}", request.request_type),
            }),
        };

        let execution_time = start_time.elapsed().as_millis() as u64;
        self.set_current_task(None);
        self.update_state(AgentState::Ready);

        match result {
            Ok(payload) => {
                self.record_task_completion(true);
                Ok(AgentResponse::success(request.id, payload).with_execution_time(execution_time))
            }
            Err(e) => {
                self.record_task_completion(false);
                Ok(AgentResponse::error(request.id, e.to_string())
                    .with_execution_time(execution_time))
            }
        }
    }

    async fn get_status(&self) -> AgentResult<AgentStatus> {
        let base_status = self.base.get_status().await?;

        Ok(AgentStatus {
            agent_id: base_status.agent_id,
            state: base_status.state,
            current_task: base_status.current_task,
            health: base_status.health,
            resources: base_status.resources,
            timestamp: Utc::now(),
        })
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
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_test_runner_agent_creation() {
        let agent = TestRunnerAgent::new("test-agent".to_string());
        assert_eq!(agent.id(), "test-agent");
        assert_eq!(agent.config().name, "Test Runner Agent");
        assert!(agent.has_capability(&AgentCapability::Testing));
    }

    #[tokio::test]
    async fn test_function_extraction() {
        let agent = TestRunnerAgent::new("test-agent".to_string());

        // Test Rust function extraction
        let rust_code = "fn calculate_sum(a: i32, b: i32) -> i32 {";
        let function = agent.extract_function_info(rust_code, 0, "test.rs");
        assert!(function.is_some());

        let function = function.unwrap();
        assert_eq!(function.name, "calculate_sum");
        assert_eq!(function.parameters, vec!["a", "b"]);
        assert_eq!(function.return_type, Some("i32".to_string()));
    }

    #[tokio::test]
    async fn test_test_generation_integration() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        // Create temporary directory with test files
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("calculator.rs");
        let output_dir = temp_dir.path().join("tests");

        fs::write(
            &source_file,
            r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            pub fn subtract(a: i32, b: i32) -> i32 {
                a - b
            }
        "#,
        )
        .unwrap();

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
        assert!(response["generated_test_files"].as_array().unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_test_execution_integration() {
        let mut agent = TestRunnerAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        let request = TestExecutionRequest {
            test_path: "tests".to_string(),
            test_types: vec![TestType::Unit],
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
    }
}
