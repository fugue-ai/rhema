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

/// Security vulnerability severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for SecuritySeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecuritySeverity::Low => write!(f, "Low"),
            SecuritySeverity::Medium => write!(f, "Medium"),
            SecuritySeverity::High => write!(f, "High"),
            SecuritySeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Security vulnerability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityVulnerability {
    /// Vulnerability ID
    pub id: String,
    /// Vulnerability title
    pub title: String,
    /// Vulnerability description
    pub description: String,
    /// Severity level
    pub severity: SecuritySeverity,
    /// File path where vulnerability was found
    pub file_path: String,
    /// Line number where vulnerability was found
    pub line_number: Option<u32>,
    /// Code snippet containing the vulnerability
    pub code_snippet: Option<String>,
    /// CVE ID if applicable
    pub cve_id: Option<String>,
    /// Remediation suggestions
    pub remediation: Vec<String>,
    /// Detection method used
    pub detection_method: String,
    /// Timestamp when vulnerability was detected
    pub detected_at: DateTime<Utc>,
}

/// Code review findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewFinding {
    /// Finding ID
    pub id: String,
    /// Finding type
    pub finding_type: FindingType,
    /// Finding title
    pub title: String,
    /// Finding description
    pub description: String,
    /// Severity level
    pub severity: SecuritySeverity,
    /// File path where finding was found
    pub file_path: String,
    /// Line number where finding was found
    pub line_number: Option<u32>,
    /// Code snippet containing the finding
    pub code_snippet: Option<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
    /// Timestamp when finding was detected
    pub detected_at: DateTime<Utc>,
}

/// Types of code review findings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingType {
    SecurityVulnerability,
    CodeQuality,
    PerformanceIssue,
    MaintainabilityIssue,
    DocumentationIssue,
    BestPracticeViolation,
}

impl std::fmt::Display for FindingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindingType::SecurityVulnerability => write!(f, "Security Vulnerability"),
            FindingType::CodeQuality => write!(f, "Code Quality"),
            FindingType::PerformanceIssue => write!(f, "Performance Issue"),
            FindingType::MaintainabilityIssue => write!(f, "Maintainability Issue"),
            FindingType::DocumentationIssue => write!(f, "Documentation Issue"),
            FindingType::BestPracticeViolation => write!(f, "Best Practice Violation"),
        }
    }
}

/// Code review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewRequest {
    /// Path to the code to review
    pub code_path: String,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    pub file_extensions: Vec<String>,
    /// Security analysis enabled
    pub security_analysis: bool,
    /// Code quality analysis enabled
    pub quality_analysis: bool,
    /// Performance analysis enabled
    pub performance_analysis: bool,
    /// Custom rules to apply
    pub custom_rules: Vec<String>,
    /// Ignore patterns
    pub ignore_patterns: Vec<String>,
}

/// Code review response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewResponse {
    /// Review ID
    pub review_id: String,
    /// Files reviewed
    pub files_reviewed: Vec<String>,
    /// Security vulnerabilities found
    pub security_vulnerabilities: Vec<SecurityVulnerability>,
    /// Code quality findings
    pub quality_findings: Vec<CodeReviewFinding>,
    /// Performance findings
    pub performance_findings: Vec<CodeReviewFinding>,
    /// Overall security score (0-100)
    pub security_score: f64,
    /// Overall quality score (0-100)
    pub quality_score: f64,
    /// Review summary
    pub summary: String,
    /// Review timestamp
    pub reviewed_at: DateTime<Utc>,
}

/// Code Review Agent for security analysis and code review
pub struct CodeReviewAgent {
    /// Base agent functionality
    base: BaseAgent,
    /// Security rules and patterns
    security_rules: HashMap<String, SecurityRule>,
    /// Quality rules and patterns
    quality_rules: HashMap<String, QualityRule>,
    /// Performance rules and patterns
    performance_rules: HashMap<String, PerformanceRule>,
    /// Review history
    review_history: Vec<CodeReviewResponse>,
    /// Security knowledge base
    security_knowledge_base: SecurityKnowledgeBase,
}

/// Security rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityRule {
    /// Rule ID
    id: String,
    /// Rule name
    name: String,
    /// Rule description
    description: String,
    /// Pattern to match
    pattern: String,
    /// Severity level
    severity: SecuritySeverity,
    /// CVE ID if applicable
    cve_id: Option<String>,
    /// Remediation suggestions
    remediation: Vec<String>,
    /// Detection method
    detection_method: String,
}

/// Quality rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QualityRule {
    /// Rule ID
    id: String,
    /// Rule name
    name: String,
    /// Rule description
    description: String,
    /// Pattern to match
    pattern: String,
    /// Severity level
    severity: SecuritySeverity,
    /// Suggestions for improvement
    suggestions: Vec<String>,
}

/// Performance rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceRule {
    /// Rule ID
    id: String,
    /// Rule name
    name: String,
    /// Rule description
    description: String,
    /// Pattern to match
    pattern: String,
    /// Severity level
    severity: SecuritySeverity,
    /// Performance impact description
    impact_description: String,
    /// Optimization suggestions
    suggestions: Vec<String>,
}

/// Security knowledge base
#[derive(Debug, Clone)]
struct SecurityKnowledgeBase {
    /// Known vulnerabilities
    known_vulnerabilities: HashMap<String, SecurityVulnerability>,
    /// Security best practices
    best_practices: Vec<String>,
    /// Common attack patterns
    attack_patterns: Vec<String>,
}

impl CodeReviewAgent {
    /// Create a new Code Review Agent
    pub fn new(id: AgentId) -> Self {
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
            task_timeout: 300,       // 5 minutes
            memory_limit: Some(512), // 512 MB
            cpu_limit: Some(50.0),   // 50% CPU
            retry_attempts: 3,
            retry_delay: 10,
            parameters: HashMap::new(),
            tags: vec![
                "security".to_string(),
                "code-review".to_string(),
                "analysis".to_string(),
            ],
        };

        let mut agent = Self {
            base: BaseAgent::new(id, config),
            security_rules: HashMap::new(),
            quality_rules: HashMap::new(),
            performance_rules: HashMap::new(),
            review_history: Vec::new(),
            security_knowledge_base: SecurityKnowledgeBase::new(),
        };

        // Initialize security rules
        agent.initialize_security_rules();
        agent.initialize_quality_rules();
        agent.initialize_performance_rules();

        agent
    }

    /// Initialize security rules
    fn initialize_security_rules(&mut self) {
        // SQL Injection patterns
        self.security_rules.insert(
            "sql_injection".to_string(),
            SecurityRule {
                id: "sql_injection".to_string(),
                name: "SQL Injection Vulnerability".to_string(),
                description: "Detects potential SQL injection vulnerabilities".to_string(),
                pattern: r"(?i)(execute|query|prepare).*\+.*\$|.*\$\{.*\}|.*\$\w+".to_string(),
                severity: SecuritySeverity::Critical,
                cve_id: Some("CWE-89".to_string()),
                remediation: vec![
                    "Use parameterized queries or prepared statements".to_string(),
                    "Validate and sanitize all user inputs".to_string(),
                    "Use an ORM framework".to_string(),
                ],
                detection_method: "Pattern matching".to_string(),
            },
        );

        // XSS patterns
        self.security_rules.insert(
            "xss".to_string(),
            SecurityRule {
                id: "xss".to_string(),
                name: "Cross-Site Scripting (XSS)".to_string(),
                description: "Detects potential XSS vulnerabilities".to_string(),
                pattern: r"(?i)<script|javascript:|on\w+\s*=|eval\s*\(".to_string(),
                severity: SecuritySeverity::High,
                cve_id: Some("CWE-79".to_string()),
                remediation: vec![
                    "Sanitize user inputs".to_string(),
                    "Use Content Security Policy (CSP)".to_string(),
                    "Escape output properly".to_string(),
                ],
                detection_method: "Pattern matching".to_string(),
            },
        );

        // Hardcoded credentials
        self.security_rules.insert(
            "hardcoded_credentials".to_string(),
            SecurityRule {
                id: "hardcoded_credentials".to_string(),
                name: "Hardcoded Credentials".to_string(),
                description: "Detects hardcoded passwords, API keys, or tokens".to_string(),
                pattern: r#"(?i)(password|secret|key|token)\s*=\s*["'][^"']{8,}["']"#.to_string(),
                severity: SecuritySeverity::High,
                cve_id: Some("CWE-259".to_string()),
                remediation: vec![
                    "Use environment variables".to_string(),
                    "Use secure configuration management".to_string(),
                    "Use secrets management services".to_string(),
                ],
                detection_method: "Pattern matching".to_string(),
            },
        );

        // Command injection
        self.security_rules.insert(
            "command_injection".to_string(),
            SecurityRule {
                id: "command_injection".to_string(),
                name: "Command Injection".to_string(),
                description: "Detects potential command injection vulnerabilities".to_string(),
                pattern: r#"(?i)(system|exec|shell_exec|passthru|eval)\s*\(.*\$"#.to_string(),
                severity: SecuritySeverity::Critical,
                cve_id: Some("CWE-78".to_string()),
                remediation: vec![
                    "Avoid using shell commands with user input".to_string(),
                    "Use built-in language functions instead".to_string(),
                    "Validate and sanitize all inputs".to_string(),
                ],
                detection_method: "Pattern matching".to_string(),
            },
        );
    }

    /// Initialize quality rules
    fn initialize_quality_rules(&mut self) {
        // Magic numbers
        self.quality_rules.insert(
            "magic_numbers".to_string(),
            QualityRule {
                id: "magic_numbers".to_string(),
                name: "Magic Numbers".to_string(),
                description: "Detects magic numbers that should be constants".to_string(),
                pattern: r#"\b\d{2,}\b"#.to_string(),
                severity: SecuritySeverity::Low,
                suggestions: vec![
                    "Define constants for magic numbers".to_string(),
                    "Use named constants instead of literal values".to_string(),
                ],
            },
        );

        // Long functions
        self.quality_rules.insert(
            "long_functions".to_string(),
            QualityRule {
                id: "long_functions".to_string(),
                name: "Long Functions".to_string(),
                description: "Detects functions that are too long".to_string(),
                pattern: r#"fn\s+\w+\s*\([^)]*\)\s*\{[^}]{500,}"#.to_string(),
                severity: SecuritySeverity::Medium,
                suggestions: vec![
                    "Break long functions into smaller functions".to_string(),
                    "Extract helper methods".to_string(),
                    "Follow single responsibility principle".to_string(),
                ],
            },
        );
    }

    /// Initialize performance rules
    fn initialize_performance_rules(&mut self) {
        // N+1 query problem
        self.performance_rules.insert(
            "n_plus_one_query".to_string(),
            PerformanceRule {
                id: "n_plus_one_query".to_string(),
                name: "N+1 Query Problem".to_string(),
                description: "Detects potential N+1 query problems".to_string(),
                pattern: r#"(?i)(select|query).*in.*loop"#.to_string(),
                severity: SecuritySeverity::Medium,
                impact_description:
                    "Can cause significant performance degradation with large datasets".to_string(),
                suggestions: vec![
                    "Use eager loading".to_string(),
                    "Batch queries".to_string(),
                    "Use JOIN statements".to_string(),
                ],
            },
        );
    }

    /// Perform security analysis on code
    async fn perform_security_analysis(
        &self,
        code: &str,
        file_path: &str,
    ) -> Vec<SecurityVulnerability> {
        let mut vulnerabilities = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            for (rule_id, rule) in &self.security_rules {
                if let Ok(regex) = regex::Regex::new(&rule.pattern) {
                    if regex.is_match(line) {
                        let vulnerability = SecurityVulnerability {
                            id: format!("{}_{}_{}", rule_id, file_path, line_num + 1),
                            title: rule.name.clone(),
                            description: rule.description.clone(),
                            severity: rule.severity.clone(),
                            file_path: file_path.to_string(),
                            line_number: Some((line_num + 1) as u32),
                            code_snippet: Some(line.to_string()),
                            cve_id: rule.cve_id.clone(),
                            remediation: rule.remediation.clone(),
                            detection_method: rule.detection_method.clone(),
                            detected_at: Utc::now(),
                        };
                        vulnerabilities.push(vulnerability);
                    }
                }
            }
        }

        vulnerabilities
    }

    /// Perform quality analysis on code
    async fn perform_quality_analysis(
        &self,
        code: &str,
        file_path: &str,
    ) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            for (rule_id, rule) in &self.quality_rules {
                if let Ok(regex) = regex::Regex::new(&rule.pattern) {
                    if regex.is_match(line) {
                        let finding = CodeReviewFinding {
                            id: format!("{}_{}_{}", rule_id, file_path, line_num + 1),
                            finding_type: FindingType::CodeQuality,
                            title: rule.name.clone(),
                            description: rule.description.clone(),
                            severity: rule.severity.clone(),
                            file_path: file_path.to_string(),
                            line_number: Some((line_num + 1) as u32),
                            code_snippet: Some(line.to_string()),
                            suggestions: rule.suggestions.clone(),
                            detected_at: Utc::now(),
                        };
                        findings.push(finding);
                    }
                }
            }
        }

        findings
    }

    /// Perform performance analysis on code
    async fn perform_performance_analysis(
        &self,
        code: &str,
        file_path: &str,
    ) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            for (rule_id, rule) in &self.performance_rules {
                if let Ok(regex) = regex::Regex::new(&rule.pattern) {
                    if regex.is_match(line) {
                        let finding = CodeReviewFinding {
                            id: format!("{}_{}_{}", rule_id, file_path, line_num + 1),
                            finding_type: FindingType::PerformanceIssue,
                            title: rule.name.clone(),
                            description: rule.description.clone(),
                            severity: rule.severity.clone(),
                            file_path: file_path.to_string(),
                            line_number: Some((line_num + 1) as u32),
                            code_snippet: Some(line.to_string()),
                            suggestions: rule.suggestions.clone(),
                            detected_at: Utc::now(),
                        };
                        findings.push(finding);
                    }
                }
            }
        }

        findings
    }

    /// Calculate security score based on vulnerabilities
    fn calculate_security_score(&self, vulnerabilities: &[SecurityVulnerability]) -> f64 {
        let mut score: f64 = 100.0;

        for vulnerability in vulnerabilities {
            let deduction = match vulnerability.severity {
                SecuritySeverity::Critical => 25.0,
                SecuritySeverity::High => 15.0,
                SecuritySeverity::Medium => 8.0,
                SecuritySeverity::Low => 3.0,
            };
            score -= deduction;
        }

        score.max(0.0)
    }

    /// Calculate quality score based on findings
    fn calculate_quality_score(&self, findings: &[CodeReviewFinding]) -> f64 {
        let mut score: f64 = 100.0;

        for finding in findings {
            let deduction = match finding.severity {
                SecuritySeverity::Critical => 20.0,
                SecuritySeverity::High => 12.0,
                SecuritySeverity::Medium => 6.0,
                SecuritySeverity::Low => 2.0,
            };
            score -= deduction;
        }

        score.max(0.0)
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

    /// Get files to review
    async fn get_files_to_review(
        &self,
        code_path: &str,
        extensions: &[String],
    ) -> AgentResult<Vec<String>> {
        use std::fs;
        use std::path::Path;

        let mut files = Vec::new();

        if let Ok(entries) = fs::read_dir(code_path) {
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
                            Box::pin(self.get_files_to_review(path.to_str().unwrap(), extensions))
                                .await?;
                        files.extend(sub_files);
                    }
                }
            }
        }

        Ok(files)
    }
}

impl SecurityKnowledgeBase {
    fn new() -> Self {
        Self {
            known_vulnerabilities: HashMap::new(),
            best_practices: vec![
                "Always validate and sanitize user inputs".to_string(),
                "Use parameterized queries to prevent SQL injection".to_string(),
                "Implement proper authentication and authorization".to_string(),
                "Use HTTPS for all communications".to_string(),
                "Keep dependencies updated".to_string(),
                "Implement proper error handling".to_string(),
                "Use secure coding practices".to_string(),
                "Regular security audits".to_string(),
            ],
            attack_patterns: vec![
                "SQL Injection".to_string(),
                "Cross-Site Scripting (XSS)".to_string(),
                "Cross-Site Request Forgery (CSRF)".to_string(),
                "Command Injection".to_string(),
                "Path Traversal".to_string(),
                "Insecure Deserialization".to_string(),
                "Broken Authentication".to_string(),
                "Sensitive Data Exposure".to_string(),
            ],
        }
    }
}

#[async_trait]
impl Agent for CodeReviewAgent {
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
        info!("Initializing Code Review Agent: {}", self.id());
        self.base.initialize().await?;
        info!("Code Review Agent initialized successfully");
        Ok(())
    }

    async fn start(&mut self) -> AgentResult<()> {
        info!("Starting Code Review Agent: {}", self.id());
        self.base.start().await?;
        info!("Code Review Agent started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> AgentResult<()> {
        info!("Stopping Code Review Agent: {}", self.id());
        self.base.stop().await?;
        info!("Code Review Agent stopped successfully");
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
            "code_review" => {
                if let Ok(review_request) =
                    serde_json::from_value::<CodeReviewRequest>(request.payload)
                {
                    self.perform_code_review(review_request).await
                } else {
                    Err(AgentError::ValidationError {
                        reason: "Invalid code review request format".to_string(),
                    })
                }
            }
            "security_scan" => {
                if let Ok(file_path) = serde_json::from_value::<String>(request.payload) {
                    self.perform_security_scan(&file_path).await
                } else {
                    Err(AgentError::ValidationError {
                        reason: "Invalid security scan request format".to_string(),
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

impl CodeReviewAgent {
    /// Perform comprehensive code review
    async fn perform_code_review(
        &mut self,
        request: CodeReviewRequest,
    ) -> AgentResult<serde_json::Value> {
        info!("Starting code review for path: {}", request.code_path);

        let files = self
            .get_files_to_review(&request.code_path, &request.file_extensions)
            .await?;
        let mut all_vulnerabilities = Vec::new();
        let mut all_quality_findings = Vec::new();
        let mut all_performance_findings = Vec::new();

        for file_path in &files {
            debug!("Reviewing file: {}", file_path);

            let content = self.read_file_content(file_path).await?;

            if request.security_analysis {
                let vulnerabilities = self.perform_security_analysis(&content, file_path).await;
                all_vulnerabilities.extend(vulnerabilities);
            }

            if request.quality_analysis {
                let quality_findings = self.perform_quality_analysis(&content, file_path).await;
                all_quality_findings.extend(quality_findings);
            }

            if request.performance_analysis {
                let performance_findings =
                    self.perform_performance_analysis(&content, file_path).await;
                all_performance_findings.extend(performance_findings);
            }
        }

        let security_score = self.calculate_security_score(&all_vulnerabilities);
        let quality_score = self.calculate_quality_score(&all_quality_findings);

        let response = CodeReviewResponse {
            review_id: Uuid::new_v4().to_string(),
            files_reviewed: files,
            security_vulnerabilities: all_vulnerabilities.clone(),
            quality_findings: all_quality_findings.clone(),
            performance_findings: all_performance_findings.clone(),
            security_score,
            quality_score,
            summary: self.generate_review_summary(
                &all_vulnerabilities,
                &all_quality_findings,
                &all_performance_findings,
            ),
            reviewed_at: Utc::now(),
        };

        // Store in review history
        self.review_history.push(response.clone());

        info!("Code review completed. Found {} vulnerabilities, {} quality issues, {} performance issues",
              all_vulnerabilities.len(), all_quality_findings.len(), all_performance_findings.len());

        Ok(
            serde_json::to_value(response).map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?,
        )
    }

    /// Perform security scan on a specific file
    async fn perform_security_scan(&self, file_path: &str) -> AgentResult<serde_json::Value> {
        info!("Performing security scan on file: {}", file_path);

        let content = self.read_file_content(file_path).await?;
        let vulnerabilities = self.perform_security_analysis(&content, file_path).await;
        let security_score = self.calculate_security_score(&vulnerabilities);

        let scan_result = serde_json::json!({
            "file_path": file_path,
            "vulnerabilities": vulnerabilities,
            "security_score": security_score,
            "scanned_at": Utc::now(),
        });

        info!(
            "Security scan completed. Found {} vulnerabilities",
            vulnerabilities.len()
        );

        Ok(scan_result)
    }

    /// Generate review summary
    fn generate_review_summary(
        &self,
        vulnerabilities: &[SecurityVulnerability],
        quality_findings: &[CodeReviewFinding],
        performance_findings: &[CodeReviewFinding],
    ) -> String {
        let critical_vulns = vulnerabilities
            .iter()
            .filter(|v| v.severity == SecuritySeverity::Critical)
            .count();
        let high_vulns = vulnerabilities
            .iter()
            .filter(|v| v.severity == SecuritySeverity::High)
            .count();
        let total_issues =
            vulnerabilities.len() + quality_findings.len() + performance_findings.len();

        format!(
            "Code review completed with {} total issues found. \
             Security: {} critical, {} high vulnerabilities. \
             Quality: {} issues. Performance: {} issues. \
             Overall security score: {:.1}/100, Quality score: {:.1}/100.",
            total_issues,
            critical_vulns,
            high_vulns,
            quality_findings.len(),
            performance_findings.len(),
            self.calculate_security_score(vulnerabilities),
            self.calculate_quality_score(quality_findings)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_code_review_agent_creation() {
        let agent = CodeReviewAgent::new("test-agent".to_string());
        assert_eq!(agent.id(), "test-agent");
        assert_eq!(agent.config().name, "Code Review Agent");
        assert!(agent.has_capability(&AgentCapability::Security));
    }

    #[tokio::test]
    async fn test_security_analysis() {
        let mut agent = CodeReviewAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        // Test SQL injection detection
        let code = r#"
            let query = "SELECT * FROM users WHERE id = " + $user_input;
            execute(query);
        "#;

        let vulnerabilities = agent.perform_security_analysis(code, "test.rs").await;
        assert!(!vulnerabilities.is_empty());

        let sql_injection = vulnerabilities
            .iter()
            .find(|v| v.title.contains("SQL Injection"))
            .expect("Should find SQL injection vulnerability");

        assert_eq!(sql_injection.severity, SecuritySeverity::Critical);
    }

    #[tokio::test]
    async fn test_code_review_integration() {
        let mut agent = CodeReviewAgent::new("test-agent".to_string());
        agent.initialize().await.unwrap();

        // Create temporary directory with test files
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.rs");

        fs::write(
            &test_file,
            r#"
            fn main() {
                let password = "hardcoded_password_123";
                let query = "SELECT * FROM users WHERE id = " + user_input;
            }
        "#,
        )
        .unwrap();

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
    }

    #[tokio::test]
    async fn test_security_score_calculation() {
        let agent = CodeReviewAgent::new("test-agent".to_string());

        let vulnerabilities = vec![SecurityVulnerability {
            id: "test1".to_string(),
            title: "Test Critical".to_string(),
            description: "Test".to_string(),
            severity: SecuritySeverity::Critical,
            file_path: "test.rs".to_string(),
            line_number: Some(1),
            code_snippet: Some("test".to_string()),
            cve_id: None,
            remediation: vec![],
            detection_method: "test".to_string(),
            detected_at: Utc::now(),
        }];

        let score = agent.calculate_security_score(&vulnerabilities);
        assert_eq!(score, 75.0); // 100 - 25 for critical
    }
}
