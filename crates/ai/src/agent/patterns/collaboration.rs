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

use super::*;
use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use tracing::{info, error};
use uuid::Uuid;

/// Code review workflow pattern for multi-agent collaboration
pub struct CodeReviewWorkflow {
    /// Security review agent
    pub security_agent: String,
    /// Performance review agent
    pub performance_agent: String,
    /// Style review agent
    pub style_agent: String,
    /// Coordinator agent
    pub coordinator: String,
    /// Review configuration
    pub config: CodeReviewConfig,
}

/// Code review configuration
#[derive(Debug, Clone)]
pub struct CodeReviewConfig {
    /// Enable security review
    pub enable_security_review: bool,
    /// Enable performance review
    pub enable_performance_review: bool,
    /// Enable style review
    pub enable_style_review: bool,
    /// Review timeout (seconds)
    pub review_timeout_seconds: u64,
    /// Require consensus
    pub require_consensus: bool,
    /// Auto-merge on approval
    pub auto_merge_on_approval: bool,
}

impl Default for CodeReviewConfig {
    fn default() -> Self {
        Self {
            enable_security_review: true,
            enable_performance_review: true,
            enable_style_review: true,
            review_timeout_seconds: 3600, // 1 hour
            require_consensus: true,
            auto_merge_on_approval: false,
        }
    }
}

#[async_trait::async_trait]
impl CoordinationPattern for CodeReviewWorkflow {
    async fn execute(&self, context: &PatternContext) -> Result<PatternResult, PatternError> {
        info!("Starting code review workflow");
        let start_time = Utc::now();

        // Initialize review state
        let mut review_state = CodeReviewState {
            review_id: Uuid::new_v4().to_string(),
            file_paths: vec![],
            security_review: None,
            performance_review: None,
            style_review: None,
            coordinator_decision: None,
            status: ReviewStatus::InProgress,
            started_at: start_time,
            completed_at: None,
        };

        // Extract file paths from context
        if let Some(files) = context.state.data.get("file_paths") {
            if let Some(paths) = files.as_array() {
                review_state.file_paths = paths
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }

        // Start parallel reviews
        let mut review_tasks = Vec::new();

        if self.config.enable_security_review {
            let review_id = review_state.review_id.clone();
            let file_paths = review_state.file_paths.clone();
            let security_task: tokio::task::JoinHandle<Result<ReviewResult, PatternError>> = tokio::spawn(async move {
                // This is a placeholder - in a real implementation, this would call the actual method
                // For now, we'll return a mock result
                Ok(ReviewResult {
                    reviewer_id: "security-agent".to_string(),
                    review_type: ReviewType::Security,
                    status: ReviewStatus::Completed,
                    findings: vec![],
                    comments: vec!["Security review completed".to_string()],
                    review_time: 1.5,
                })
            });
            review_tasks.push(("security".to_string(), security_task));
        }

        if self.config.enable_performance_review {
            let review_id = review_state.review_id.clone();
            let file_paths = review_state.file_paths.clone();
            let performance_task: tokio::task::JoinHandle<Result<ReviewResult, PatternError>> = tokio::spawn(async move {
                // This is a placeholder - in a real implementation, this would call the actual method
                Ok(ReviewResult {
                    reviewer_id: "performance-agent".to_string(),
                    review_type: ReviewType::Performance,
                    status: ReviewStatus::Completed,
                    findings: vec![],
                    comments: vec!["Performance review completed".to_string()],
                    review_time: 2.0,
                })
            });
            review_tasks.push(("performance".to_string(), performance_task));
        }

        if self.config.enable_style_review {
            let review_id = review_state.review_id.clone();
            let file_paths = review_state.file_paths.clone();
            let style_task: tokio::task::JoinHandle<Result<ReviewResult, PatternError>> = tokio::spawn(async move {
                // This is a placeholder - in a real implementation, this would call the actual method
                Ok(ReviewResult {
                    reviewer_id: "style-agent".to_string(),
                    review_type: ReviewType::Style,
                    status: ReviewStatus::Completed,
                    findings: vec![],
                    comments: vec!["Style review completed".to_string()],
                    review_time: 1.0,
                })
            });
            review_tasks.push(("style".to_string(), style_task));
        }

        // Wait for all reviews to complete
        let review_count = review_tasks.len();
        for (review_type, task) in review_tasks {
            match task.await {
                Ok(Ok(review_result)) => {
                    match review_type.as_str() {
                        "security" => review_state.security_review = Some(review_result),
                        "performance" => review_state.performance_review = Some(review_result),
                        "style" => review_state.style_review = Some(review_result),
                        _ => {}
                    }
                }
                Ok(Err(e)) => {
                    error!("{} review failed: {}", review_type, e);
                    return Err(PatternError::ExecutionError(format!("{} review failed: {}", review_type, e)));
                }
                Err(e) => {
                    error!("{} review task failed: {}", review_type, e);
                    return Err(PatternError::ExecutionError(format!("{} review task failed: {}", review_type, e)));
                }
            }
        }

        // Coordinator makes final decision
        review_state.coordinator_decision = Some(self.make_coordinator_decision(&review_state).await?);
        review_state.status = ReviewStatus::Completed;
        review_state.completed_at = Some(Utc::now());

        // Calculate performance metrics
        let execution_time = (Utc::now() - start_time).num_seconds() as f64;
        let performance_metrics = PatternPerformanceMetrics {
            total_execution_time_seconds: execution_time,
            coordination_overhead_seconds: execution_time * 0.1, // Estimate 10% overhead
            resource_utilization: 0.8, // Estimate 80% utilization
            agent_efficiency: 0.9, // Estimate 90% efficiency
            communication_overhead: review_count * 3, // Estimate 3 messages per review
        };

        let result_data = HashMap::from([
            ("review_id".to_string(), json!(review_state.review_id)),
            ("status".to_string(), json!(review_state.status.to_string())),
            ("decision".to_string(), json!(review_state.coordinator_decision.as_ref().map(|d| d.to_string()))),
            ("file_count".to_string(), json!(review_state.file_paths.len())),
        ]);

        Ok(PatternResult {
            pattern_id: "code-review-workflow".to_string(),
            success: review_state.status == ReviewStatus::Completed,
            data: result_data,
            performance_metrics,
            error_message: None,
            completed_at: Utc::now(),
            metadata: HashMap::new(),
            execution_time_ms: 100,
        })
    }

    async fn validate(&self, context: &PatternContext) -> Result<ValidationResult, PatternError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if required agents are available
        let agent_ids: Vec<String> = context.agents.iter().map(|a| a.id.clone()).collect();
        
        if !agent_ids.contains(&self.security_agent) && self.config.enable_security_review {
            errors.push(format!("Security agent {} not found", self.security_agent));
        }
        
        if !agent_ids.contains(&self.performance_agent) && self.config.enable_performance_review {
            errors.push(format!("Performance agent {} not found", self.performance_agent));
        }
        
        if !agent_ids.contains(&self.style_agent) && self.config.enable_style_review {
            errors.push(format!("Style agent {} not found", self.style_agent));
        }
        
        if !agent_ids.contains(&self.coordinator) {
            errors.push(format!("Coordinator agent {} not found", self.coordinator));
        }

        // Check if file paths are provided
        if !context.state.data.contains_key("file_paths") {
            errors.push("No file paths provided for review".to_string());
        }

        // Check resource availability
        if context.resources.file_locks.is_empty() {
            warnings.push("No file locks available".to_string());
        }

        let is_valid = errors.is_empty();
        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
            details: HashMap::new(),
        })
    }

    async fn rollback(&self, _context: &PatternContext) -> Result<(), PatternError> {
        // Code review rollback would typically involve:
        // - Reverting any changes made during review
        // - Cancelling ongoing reviews
        // - Cleaning up review artifacts
        info!("Rolling back code review workflow");
        Ok(())
    }

    fn metadata(&self) -> PatternMetadata {
        PatternMetadata {
            id: "code-review-workflow".to_string(),
            name: "Code Review Workflow".to_string(),
            description: "Multi-agent code review with security, performance, and style analysis".to_string(),
            version: "1.0.0".to_string(),
            category: PatternCategory::Collaboration,
            author: "CodeReviewWorkflow".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: vec!["code-review".to_string(), "collaboration".to_string()],
            required_capabilities: vec![
                "code-review".to_string(),
                "security-analysis".to_string(),
                "performance-analysis".to_string(),
                "style-analysis".to_string(),
            ],
            required_resources: vec!["file-access".to_string()],
            constraints: Vec::new(),
            dependencies: Vec::new(),
            complexity: 7,
            estimated_execution_time_seconds: 3600,
        }
    }
}

impl CodeReviewWorkflow {
    pub fn new(
        security_agent: String,
        performance_agent: String,
        style_agent: String,
        coordinator: String,
        config: CodeReviewConfig,
    ) -> Self {
        Self {
            security_agent,
            performance_agent,
            style_agent,
            coordinator,
            config,
        }
    }

    async fn start_security_review(&self, review_id: &str, file_paths: &[String]) -> Result<ReviewResult, PatternError> {
        info!("Starting security review for review {}", review_id);
        
        // Simulate security review process
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        Ok(ReviewResult {
            reviewer_id: self.security_agent.clone(),
            review_type: ReviewType::Security,
            status: ReviewStatus::Approved,
            findings: vec![
                Finding {
                    severity: FindingSeverity::Low,
                    description: "Potential SQL injection vulnerability".to_string(),
                    location: "src/database.rs:45".to_string(),
                    suggestion: "Use parameterized queries".to_string(),
                }
            ],
            comments: vec!["Security review completed".to_string()],
            review_time: 10.0,
        })
    }

    async fn start_performance_review(&self, review_id: &str, file_paths: &[String]) -> Result<ReviewResult, PatternError> {
        info!("Starting performance review for review {}", review_id);
        
        // Simulate performance review process
        tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
        
        Ok(ReviewResult {
            reviewer_id: self.performance_agent.clone(),
            review_type: ReviewType::Performance,
            status: ReviewStatus::Approved,
            findings: vec![
                Finding {
                    severity: FindingSeverity::Medium,
                    description: "Inefficient algorithm in sorting function".to_string(),
                    location: "src/utils.rs:123".to_string(),
                    suggestion: "Consider using quicksort instead of bubble sort".to_string(),
                }
            ],
            comments: vec!["Performance review completed".to_string()],
            review_time: 8.0,
        })
    }

    async fn start_style_review(&self, review_id: &str, file_paths: &[String]) -> Result<ReviewResult, PatternError> {
        info!("Starting style review for review {}", review_id);
        
        // Simulate style review process
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok(ReviewResult {
            reviewer_id: self.style_agent.clone(),
            review_type: ReviewType::Style,
            status: ReviewStatus::Approved,
            findings: vec![
                Finding {
                    severity: FindingSeverity::Low,
                    description: "Inconsistent naming convention".to_string(),
                    location: "src/main.rs:67".to_string(),
                    suggestion: "Use snake_case for variable names".to_string(),
                }
            ],
            comments: vec!["Style review completed".to_string()],
            review_time: 5.0,
        })
    }

    async fn make_coordinator_decision(&self, review_state: &CodeReviewState) -> Result<CoordinatorDecision, PatternError> {
        info!("Coordinator making final decision for review {}", review_state.review_id);
        
        // Analyze all review results
        let mut total_findings = 0;
        let mut critical_findings = 0;
        
        if let Some(ref security) = review_state.security_review {
            total_findings += security.findings.len();
            critical_findings += security.findings.iter()
                .filter(|f| f.severity == FindingSeverity::Critical)
                .count();
        }
        
        if let Some(ref performance) = review_state.performance_review {
            total_findings += performance.findings.len();
            critical_findings += performance.findings.iter()
                .filter(|f| f.severity == FindingSeverity::Critical)
                .count();
        }
        
        if let Some(ref style) = review_state.style_review {
            total_findings += style.findings.len();
            critical_findings += style.findings.iter()
                .filter(|f| f.severity == FindingSeverity::Critical)
                .count();
        }
        
        // Make decision based on findings
        let decision = if critical_findings > 0 {
            CoordinatorDecision::Reject
        } else if total_findings > 5 {
            CoordinatorDecision::RequestChanges
        } else {
            CoordinatorDecision::Approve
        };
        
        Ok(decision)
    }
}

/// Test generation workflow pattern
pub struct TestGenerationWorkflow {
    /// Test strategy agent
    pub strategy_agent: String,
    /// Unit test agent
    pub unit_test_agent: String,
    /// Integration test agent
    pub integration_test_agent: String,
    /// Test runner agent
    pub test_runner_agent: String,
    /// Configuration
    pub config: TestGenerationConfig,
}

/// Test generation configuration
#[derive(Debug, Clone)]
pub struct TestGenerationConfig {
    /// Enable unit tests
    pub enable_unit_tests: bool,
    /// Enable integration tests
    pub enable_integration_tests: bool,
    /// Enable performance tests
    pub enable_performance_tests: bool,
    /// Test coverage target (0.0-1.0)
    pub coverage_target: f64,
    /// Auto-run tests
    pub auto_run_tests: bool,
    /// Test timeout (seconds)
    pub test_timeout_seconds: u64,
}

impl Default for TestGenerationConfig {
    fn default() -> Self {
        Self {
            enable_unit_tests: true,
            enable_integration_tests: true,
            enable_performance_tests: false,
            coverage_target: 0.8,
            auto_run_tests: true,
            test_timeout_seconds: 300,
        }
    }
}

#[async_trait::async_trait]
impl CoordinationPattern for TestGenerationWorkflow {
    async fn execute(&self, context: &PatternContext) -> Result<PatternResult, PatternError> {
        info!("Starting test generation workflow");
        let start_time = Utc::now();

        // Initialize test generation state
        let mut test_state = TestGenerationState {
            test_id: Uuid::new_v4().to_string(),
            target_files: vec![],
            test_strategy: None,
            unit_tests: vec![],
            integration_tests: vec![],
            performance_tests: vec![],
            test_results: vec![],
            coverage: 0.0,
            status: TestStatus::InProgress,
            started_at: start_time,
            completed_at: None,
        };

        // Extract target files from context
        if let Some(files) = context.state.data.get("target_files") {
            if let Some(paths) = files.as_array() {
                test_state.target_files = paths
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }

        // Generate test strategy
        test_state.test_strategy = Some(self.generate_test_strategy(&test_state.target_files).await?);

        // Generate tests based on strategy
        if self.config.enable_unit_tests {
            let unit_tests = self.generate_unit_tests(&test_state.test_strategy.as_ref().unwrap()).await?;
            test_state.unit_tests = unit_tests;
        }

        if self.config.enable_integration_tests {
            let integration_tests = self.generate_integration_tests(&test_state.test_strategy.as_ref().unwrap()).await?;
            test_state.integration_tests = integration_tests;
        }

        if self.config.enable_performance_tests {
            let performance_tests = self.generate_performance_tests(&test_state.test_strategy.as_ref().unwrap()).await?;
            test_state.performance_tests = performance_tests;
        }

        // Run tests if auto-run is enabled
        if self.config.auto_run_tests {
            test_state.test_results = self.run_tests(&test_state).await?;
            test_state.coverage = self.calculate_coverage(&test_state.test_results);
        }

        test_state.status = TestStatus::Completed;
        test_state.completed_at = Some(Utc::now());

        // Calculate performance metrics
        let execution_time = (Utc::now() - start_time).num_seconds() as f64;
        let performance_metrics = PatternPerformanceMetrics {
            total_execution_time_seconds: execution_time,
            coordination_overhead_seconds: execution_time * 0.15,
            resource_utilization: 0.85,
            agent_efficiency: 0.92,
            communication_overhead: 20, // Estimate 20 messages for test generation and execution
        };

        let result_data = HashMap::from([
            ("test_id".to_string(), json!(test_state.test_id)),
            ("status".to_string(), json!(test_state.status.to_string())),
            ("coverage".to_string(), json!(test_state.coverage)),
            ("test_count".to_string(), json!(test_state.unit_tests.len() + test_state.integration_tests.len() + test_state.performance_tests.len())),
        ]);

        Ok(PatternResult {
            pattern_id: "test-generation-workflow".to_string(),
            success: test_state.status == TestStatus::Completed,
            data: result_data,
            performance_metrics,
            error_message: None,
            completed_at: Utc::now(),
            metadata: HashMap::new(),
            execution_time_ms: 100,
        })
    }

    async fn validate(&self, context: &PatternContext) -> Result<ValidationResult, PatternError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if required agents are available
        let agent_ids: Vec<String> = context.agents.iter().map(|a| a.id.clone()).collect();
        
        if !agent_ids.contains(&self.strategy_agent) {
            errors.push(format!("Strategy agent {} not found", self.strategy_agent));
        }
        
        if !agent_ids.contains(&self.unit_test_agent) && self.config.enable_unit_tests {
            errors.push(format!("Unit test agent {} not found", self.unit_test_agent));
        }
        
        if !agent_ids.contains(&self.integration_test_agent) && self.config.enable_integration_tests {
            errors.push(format!("Integration test agent {} not found", self.integration_test_agent));
        }
        
        if !agent_ids.contains(&self.test_runner_agent) && self.config.auto_run_tests {
            errors.push(format!("Test runner agent {} not found", self.test_runner_agent));
        }

        // Check if target files are provided
        if !context.state.data.contains_key("target_files") {
            errors.push("No target files provided for test generation".to_string());
        }

        let is_valid = errors.is_empty();
        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
            details: HashMap::new(),
        })
    }

    async fn rollback(&self, _context: &PatternContext) -> Result<(), PatternError> {
        info!("Rolling back test generation workflow");
        Ok(())
    }

    fn metadata(&self) -> PatternMetadata {
        PatternMetadata {
            id: "test-generation-workflow".to_string(),
            name: "Test Generation Workflow".to_string(),
            description: "Multi-agent test generation with strategy, unit, integration, and performance tests".to_string(),
            version: "1.0.0".to_string(),
            category: PatternCategory::Collaboration,
            author: "TestGenerationWorkflow".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: vec!["test-generation".to_string(), "collaboration".to_string()],
            required_capabilities: vec![
                "test-generation".to_string(),
                "test-strategy".to_string(),
                "unit-testing".to_string(),
                "integration-testing".to_string(),
                "test-execution".to_string(),
            ],
            required_resources: vec!["file-access".to_string(), "test-environment".to_string()],
            constraints: Vec::new(),
            dependencies: Vec::new(),
            complexity: 8,
            estimated_execution_time_seconds: 1800,
        }
    }
}

impl TestGenerationWorkflow {
    pub fn new(
        strategy_agent: String,
        unit_test_agent: String,
        integration_test_agent: String,
        test_runner_agent: String,
        config: TestGenerationConfig,
    ) -> Self {
        Self {
            strategy_agent,
            unit_test_agent,
            integration_test_agent,
            test_runner_agent,
            config,
        }
    }

    async fn generate_test_strategy(&self, target_files: &[String]) -> Result<TestStrategy, PatternError> {
        info!("Generating test strategy for {} files", target_files.len());
        
        // Simulate test strategy generation
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok(TestStrategy {
            strategy_id: Uuid::new_v4().to_string(),
            target_files: target_files.to_vec(),
            unit_test_approach: "Comprehensive unit tests with mocking".to_string(),
            integration_test_approach: "API integration tests with database fixtures".to_string(),
            performance_test_approach: "Load testing with realistic scenarios".to_string(),
            coverage_goals: HashMap::from([
                ("unit".to_string(), 0.9),
                ("integration".to_string(), 0.7),
                ("performance".to_string(), 0.5),
            ]),
        })
    }

    async fn generate_unit_tests(&self, strategy: &TestStrategy) -> Result<Vec<UnitTest>, PatternError> {
        info!("Generating unit tests based on strategy");
        
        // Simulate unit test generation
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        
        Ok(vec![
            UnitTest {
                test_id: Uuid::new_v4().to_string(),
                file_path: "src/utils.rs".to_string(),
                function_name: "calculate_total".to_string(),
                test_code: "#[test]\nfn test_calculate_total() {\n    assert_eq!(calculate_total(&[1, 2, 3]), 6);\n}".to_string(),
                test_type: TestType::Unit,
                priority: TestPriority::High,
            }
        ])
    }

    async fn generate_integration_tests(&self, strategy: &TestStrategy) -> Result<Vec<IntegrationTest>, PatternError> {
        info!("Generating integration tests based on strategy");
        
        // Simulate integration test generation
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        Ok(vec![
            IntegrationTest {
                test_id: Uuid::new_v4().to_string(),
                test_name: "test_user_creation_flow".to_string(),
                test_code: "#[tokio::test]\nasync fn test_user_creation_flow() {\n    // Integration test code\n}".to_string(),
                dependencies: vec!["database".to_string(), "api".to_string()],
                test_type: TestType::Integration,
                priority: TestPriority::Medium,
            }
        ])
    }

    async fn generate_performance_tests(&self, strategy: &TestStrategy) -> Result<Vec<PerformanceTest>, PatternError> {
        info!("Generating performance tests based on strategy");
        
        // Simulate performance test generation
        tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
        
        Ok(vec![
            PerformanceTest {
                test_id: Uuid::new_v4().to_string(),
                test_name: "test_api_response_time".to_string(),
                test_code: "#[tokio::test]\nasync fn test_api_response_time() {\n    // Performance test code\n}".to_string(),
                performance_targets: HashMap::from([
                    ("response_time_ms".to_string(), 100),
                    ("throughput_rps".to_string(), 1000),
                ]),
                test_type: TestType::Performance,
                priority: TestPriority::Low,
            }
        ])
    }

    async fn run_tests(&self, test_state: &TestGenerationState) -> Result<Vec<TestResult>, PatternError> {
        info!("Running generated tests");
        
        // Simulate test execution
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
        
        Ok(vec![
            TestResult {
                test_id: Uuid::new_v4().to_string(),
                test_name: "test_calculate_total".to_string(),
                status: TestResultStatus::Passed,
                execution_time_ms: 15,
                coverage_percentage: 85.5,
                output: "Test passed successfully".to_string(),
            }
        ])
    }

    fn calculate_coverage(&self, test_results: &[TestResult]) -> f64 {
        if test_results.is_empty() {
            return 0.0;
        }
        
        let total_coverage: f64 = test_results.iter().map(|r| r.coverage_percentage).sum();
        total_coverage / test_results.len() as f64
    }
}

// Supporting data structures for code review workflow
#[derive(Debug, Clone)]
pub struct CodeReviewState {
    pub review_id: String,
    pub file_paths: Vec<String>,
    pub security_review: Option<ReviewResult>,
    pub performance_review: Option<ReviewResult>,
    pub style_review: Option<ReviewResult>,
    pub coordinator_decision: Option<CoordinatorDecision>,
    pub status: ReviewStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub reviewer_id: String,
    pub review_type: ReviewType,
    pub status: ReviewStatus,
    pub findings: Vec<Finding>,
    pub comments: Vec<String>,
    pub review_time: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewType {
    Security,
    Performance,
    Style,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    InProgress,
    Approved,
    Rejected,
    RequestChanges,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: FindingSeverity,
    pub description: String,
    pub location: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinatorDecision {
    Approve,
    Reject,
    RequestChanges,
}

impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewStatus::Pending => write!(f, "pending"),
            ReviewStatus::InProgress => write!(f, "in-progress"),
            ReviewStatus::Approved => write!(f, "approved"),
            ReviewStatus::Rejected => write!(f, "rejected"),
            ReviewStatus::RequestChanges => write!(f, "request-changes"),
            ReviewStatus::Completed => write!(f, "completed"),
        }
    }
}

impl std::fmt::Display for CoordinatorDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinatorDecision::Approve => write!(f, "approve"),
            CoordinatorDecision::Reject => write!(f, "reject"),
            CoordinatorDecision::RequestChanges => write!(f, "request-changes"),
        }
    }
}

// Supporting data structures for test generation workflow
#[derive(Debug, Clone)]
pub struct TestGenerationState {
    pub test_id: String,
    pub target_files: Vec<String>,
    pub test_strategy: Option<TestStrategy>,
    pub unit_tests: Vec<UnitTest>,
    pub integration_tests: Vec<IntegrationTest>,
    pub performance_tests: Vec<PerformanceTest>,
    pub test_results: Vec<TestResult>,
    pub coverage: f64,
    pub status: TestStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TestStrategy {
    pub strategy_id: String,
    pub target_files: Vec<String>,
    pub unit_test_approach: String,
    pub integration_test_approach: String,
    pub performance_test_approach: String,
    pub coverage_goals: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct UnitTest {
    pub test_id: String,
    pub file_path: String,
    pub function_name: String,
    pub test_code: String,
    pub test_type: TestType,
    pub priority: TestPriority,
}

#[derive(Debug, Clone)]
pub struct IntegrationTest {
    pub test_id: String,
    pub test_name: String,
    pub test_code: String,
    pub dependencies: Vec<String>,
    pub test_type: TestType,
    pub priority: TestPriority,
}

#[derive(Debug, Clone)]
pub struct PerformanceTest {
    pub test_id: String,
    pub test_name: String,
    pub test_code: String,
    pub performance_targets: HashMap<String, u64>,
    pub test_type: TestType,
    pub priority: TestPriority,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub test_name: String,
    pub status: TestResultStatus,
    pub execution_time_ms: u64,
    pub coverage_percentage: f64,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestType {
    Unit,
    Integration,
    Performance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestStatus::Pending => write!(f, "Pending"),
            TestStatus::InProgress => write!(f, "InProgress"),
            TestStatus::Completed => write!(f, "Completed"),
            TestStatus::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResultStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_code_review_workflow_metadata() {
        let workflow = CodeReviewWorkflow::new(
            "security-agent".to_string(),
            "performance-agent".to_string(),
            "style-agent".to_string(),
            "coordinator".to_string(),
            CodeReviewConfig::default(),
        );

        let metadata = workflow.metadata();
        assert_eq!(metadata.name, "Code Review Workflow");
        assert_eq!(metadata.category, PatternCategory::Collaboration);
        assert!(metadata.required_capabilities.contains(&"code-review".to_string()));
    }

    #[tokio::test]
    async fn test_test_generation_workflow_metadata() {
        let workflow = TestGenerationWorkflow::new(
            "strategy-agent".to_string(),
            "unit-test-agent".to_string(),
            "integration-test-agent".to_string(),
            "test-runner-agent".to_string(),
            TestGenerationConfig::default(),
        );

        let metadata = workflow.metadata();
        assert_eq!(metadata.name, "Test Generation Workflow");
        assert_eq!(metadata.category, PatternCategory::Collaboration);
        assert!(metadata.required_capabilities.contains(&"test-generation".to_string()));
    }
} 