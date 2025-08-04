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

/// Custom Security Analysis Agent
/// 
/// This agent specializes in security analysis tasks including:
/// - Code security scanning
/// - Dependency vulnerability analysis
/// - Security policy enforcement
/// - Compliance checking
struct SecurityAnalysisAgent {
    base: BaseAgent,
    security_scanner: Arc<SecurityScanner>,
    vulnerability_db: Arc<VulnerabilityDatabase>,
}

impl SecurityAnalysisAgent {
    fn new(id: AgentId, config: AgentConfig) -> Self {
        Self {
            base: BaseAgent::new(id, config),
            security_scanner: Arc::new(SecurityScanner::new()),
            vulnerability_db: Arc::new(VulnerabilityDatabase::new()),
        }
    }

    async fn scan_code_security(&self, code_path: &str) -> SecurityScanResult {
        // Simulate security scanning
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        
        SecurityScanResult {
            vulnerabilities_found: 2,
            critical_issues: 0,
            high_issues: 1,
            medium_issues: 1,
            low_issues: 0,
            scan_duration_ms: 300,
            files_scanned: 45,
        }
    }

    async fn analyze_dependencies(&self, dependencies: &[String]) -> DependencyAnalysisResult {
        // Simulate dependency analysis
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        
        DependencyAnalysisResult {
            total_dependencies: dependencies.len(),
            vulnerable_dependencies: 1,
            outdated_dependencies: 3,
            license_issues: 0,
            analysis_duration_ms: 200,
        }
    }

    async fn check_compliance(&self, compliance_standard: &str) -> ComplianceResult {
        // Simulate compliance checking
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        ComplianceResult {
            standard: compliance_standard.to_string(),
            compliant: true,
            violations: 0,
            warnings: 2,
            check_duration_ms: 150,
        }
    }
}

#[async_trait::async_trait]
impl Agent for SecurityAnalysisAgent {
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
        // Initialize security scanner and vulnerability database
        self.security_scanner.initialize().await;
        self.vulnerability_db.load_database().await;
        
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
            "security_scan" => {
                let code_path = request.payload["code_path"].as_str().unwrap_or("./src");
                let scan_result = self.scan_code_security(code_path).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "vulnerabilities_found": scan_result.vulnerabilities_found,
                        "critical_issues": scan_result.critical_issues,
                        "high_issues": scan_result.high_issues,
                        "medium_issues": scan_result.medium_issues,
                        "low_issues": scan_result.low_issues,
                        "files_scanned": scan_result.files_scanned,
                        "scan_duration_ms": scan_result.scan_duration_ms
                    })
                ))
            }
            "dependency_analysis" => {
                let dependencies = request.payload["dependencies"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                
                let analysis_result = self.analyze_dependencies(&dependencies).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "total_dependencies": analysis_result.total_dependencies,
                        "vulnerable_dependencies": analysis_result.vulnerable_dependencies,
                        "outdated_dependencies": analysis_result.outdated_dependencies,
                        "license_issues": analysis_result.license_issues,
                        "analysis_duration_ms": analysis_result.analysis_duration_ms
                    })
                ))
            }
            "compliance_check" => {
                let standard = request.payload["standard"].as_str().unwrap_or("OWASP");
                let compliance_result = self.check_compliance(standard).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "standard": compliance_result.standard,
                        "compliant": compliance_result.compliant,
                        "violations": compliance_result.violations,
                        "warnings": compliance_result.warnings,
                        "check_duration_ms": compliance_result.check_duration_ms
                    })
                ))
            }
            "security_audit" => {
                // Comprehensive security audit combining all checks
                let code_path = request.payload["code_path"].as_str().unwrap_or("./src");
                let dependencies = request.payload["dependencies"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                let standard = request.payload["standard"].as_str().unwrap_or("OWASP");
                
                // Run all security checks in parallel
                let (scan_result, analysis_result, compliance_result) = tokio::join!(
                    self.scan_code_security(code_path),
                    self.analyze_dependencies(&dependencies),
                    self.check_compliance(standard)
                );
                
                let total_issues = scan_result.vulnerabilities_found + 
                                 analysis_result.vulnerable_dependencies + 
                                 compliance_result.violations;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "audit_summary": {
                            "total_issues": total_issues,
                            "code_scan": {
                                "vulnerabilities_found": scan_result.vulnerabilities_found,
                                "files_scanned": scan_result.files_scanned
                            },
                            "dependency_analysis": {
                                "vulnerable_dependencies": analysis_result.vulnerable_dependencies,
                                "outdated_dependencies": analysis_result.outdated_dependencies
                            },
                            "compliance": {
                                "compliant": compliance_result.compliant,
                                "violations": compliance_result.violations,
                                "warnings": compliance_result.warnings
                            }
                        },
                        "recommendations": [
                            "Update vulnerable dependencies",
                            "Fix medium priority security issues",
                            "Review compliance warnings"
                        ]
                    })
                ))
            }
            _ => {
                Ok(AgentResponse::error(
                    request.id,
                    format!("Unknown security task type: {}", request.request_type)
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

/// Security Scanner - Simulates a code security scanning engine
struct SecurityScanner {
    initialized: bool,
}

impl SecurityScanner {
    fn new() -> Self {
        Self { initialized: false }
    }

    async fn initialize(&mut self) {
        // Simulate initialization
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        self.initialized = true;
    }
}

/// Vulnerability Database - Simulates a vulnerability database
struct VulnerabilityDatabase {
    loaded: bool,
}

impl VulnerabilityDatabase {
    fn new() -> Self {
        Self { loaded: false }
    }

    async fn load_database(&mut self) {
        // Simulate database loading
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        self.loaded = true;
    }
}

/// Result types for security analysis
#[derive(Debug)]
struct SecurityScanResult {
    vulnerabilities_found: u32,
    critical_issues: u32,
    high_issues: u32,
    medium_issues: u32,
    low_issues: u32,
    scan_duration_ms: u64,
    files_scanned: u32,
}

#[derive(Debug)]
struct DependencyAnalysisResult {
    total_dependencies: usize,
    vulnerable_dependencies: u32,
    outdated_dependencies: u32,
    license_issues: u32,
    analysis_duration_ms: u64,
}

#[derive(Debug)]
struct ComplianceResult {
    standard: String,
    compliant: bool,
    violations: u32,
    warnings: u32,
    check_duration_ms: u64,
}

/// Create a comprehensive security workflow
fn create_security_workflow() -> WorkflowDefinition {
    let steps = vec![
        // Step 1: Initial security scan
        WorkflowStep::new(
            "initial_scan".to_string(),
            "Initial Security Scan".to_string(),
            WorkflowStepType::Task {
                agent_id: "security-agent".to_string(),
                request: AgentRequest::new("security_scan".to_string(), json!({
                    "code_path": "./src"
                })),
            },
        ).with_description("Perform initial security scan of codebase".to_string()),

        // Step 2: Dependency analysis
        WorkflowStep::new(
            "dependency_analysis".to_string(),
            "Dependency Analysis".to_string(),
            WorkflowStepType::Task {
                agent_id: "security-agent".to_string(),
                request: AgentRequest::new("dependency_analysis".to_string(), json!({
                    "dependencies": ["cargo", "npm", "pip"]
                })),
            },
        ).with_description("Analyze dependencies for vulnerabilities".to_string()),

        // Step 3: Compliance check
        WorkflowStep::new(
            "compliance_check".to_string(),
            "Compliance Check".to_string(),
            WorkflowStepType::Task {
                agent_id: "security-agent".to_string(),
                request: AgentRequest::new("compliance_check".to_string(), json!({
                    "standard": "OWASP"
                })),
            },
        ).with_description("Check compliance with security standards".to_string()),

        // Step 4: Conditional deep audit
        WorkflowStep::new(
            "deep_audit".to_string(),
            "Deep Security Audit".to_string(),
            WorkflowStepType::Conditional {
                condition: WorkflowCondition::VariableEquals {
                    variable: "issues_found".to_string(),
                    value: json!(true),
                },
                if_true: vec![
                    WorkflowStep::new(
                        "comprehensive_audit".to_string(),
                        "Comprehensive Security Audit".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "security-agent".to_string(),
                            request: AgentRequest::new("security_audit".to_string(), json!({
                                "code_path": "./src",
                                "dependencies": ["cargo", "npm", "pip"],
                                "standard": "OWASP"
                            })),
                        },
                    ),
                ],
                if_false: None,
            },
        ).with_description("Perform deep audit if issues found".to_string()),

        // Step 5: Generate security report
        WorkflowStep::new(
            "generate_report".to_string(),
            "Generate Security Report".to_string(),
            WorkflowStepType::Task {
                agent_id: "security-agent".to_string(),
                request: AgentRequest::new("generate_report".to_string(), json!({
                    "format": "json",
                    "include_recommendations": true
                })),
            },
        ).with_description("Generate comprehensive security report".to_string()),
    ];

    WorkflowDefinition::new(
        "security-workflow".to_string(),
        "Security Analysis Pipeline".to_string(),
        steps,
    )
    .with_description("Comprehensive security analysis and compliance checking".to_string())
    .with_input_parameter(WorkflowParameter {
        name: "code_path".to_string(),
        description: Some("Path to code for security scanning".to_string()),
        parameter_type: "string".to_string(),
        required: false,
        default_value: Some(json!("./src")),
    })
    .with_input_parameter(WorkflowParameter {
        name: "compliance_standard".to_string(),
        description: Some("Security compliance standard to check".to_string()),
        parameter_type: "string".to_string(),
        required: false,
        default_value: Some(json!("OWASP")),
    })
    .with_tag("security".to_string())
    .with_tag("compliance".to_string())
    .with_tag("audit".to_string())
}

/// Custom Documentation Agent
/// 
/// This agent specializes in documentation tasks including:
/// - API documentation generation
/// - Code documentation analysis
/// - Documentation quality checks
/// - Documentation deployment
struct DocumentationAgent {
    base: BaseAgent,
    doc_generator: Arc<DocumentationGenerator>,
    quality_checker: Arc<DocumentationQualityChecker>,
}

impl DocumentationAgent {
    fn new(id: AgentId, config: AgentConfig) -> Self {
        Self {
            base: BaseAgent::new(id, config),
            doc_generator: Arc::new(DocumentationGenerator::new()),
            quality_checker: Arc::new(DocumentationQualityChecker::new()),
        }
    }

    async fn generate_api_docs(&self, api_spec_path: &str) -> DocumentationResult {
        // Simulate API documentation generation
        tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
        
        DocumentationResult {
            pages_generated: 25,
            endpoints_documented: 45,
            examples_created: 12,
            generation_duration_ms: 400,
        }
    }

    async fn analyze_code_documentation(&self, code_path: &str) -> CodeDocAnalysisResult {
        // Simulate code documentation analysis
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        
        CodeDocAnalysisResult {
            files_analyzed: 120,
            documented_functions: 85,
            undocumented_functions: 15,
            documentation_coverage: 85.0,
            analysis_duration_ms: 250,
        }
    }

    async fn check_documentation_quality(&self, doc_path: &str) -> QualityCheckResult {
        // Simulate documentation quality checking
        tokio::time::sleep(tokio::time::Duration::from_millis(180)).await;
        
        QualityCheckResult {
            pages_checked: 30,
            quality_score: 92.5,
            issues_found: 3,
            suggestions: 8,
            check_duration_ms: 180,
        }
    }
}

#[async_trait::async_trait]
impl Agent for DocumentationAgent {
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
        // Initialize documentation tools
        self.doc_generator.initialize().await;
        self.quality_checker.initialize().await;
        
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
            "generate_api_docs" => {
                let api_spec_path = request.payload["api_spec_path"].as_str().unwrap_or("./api/openapi.yaml");
                let doc_result = self.generate_api_docs(api_spec_path).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "pages_generated": doc_result.pages_generated,
                        "endpoints_documented": doc_result.endpoints_documented,
                        "examples_created": doc_result.examples_created,
                        "generation_duration_ms": doc_result.generation_duration_ms
                    })
                ))
            }
            "analyze_code_docs" => {
                let code_path = request.payload["code_path"].as_str().unwrap_or("./src");
                let analysis_result = self.analyze_code_documentation(code_path).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "files_analyzed": analysis_result.files_analyzed,
                        "documented_functions": analysis_result.documented_functions,
                        "undocumented_functions": analysis_result.undocumented_functions,
                        "documentation_coverage": analysis_result.documentation_coverage,
                        "analysis_duration_ms": analysis_result.analysis_duration_ms
                    })
                ))
            }
            "check_doc_quality" => {
                let doc_path = request.payload["doc_path"].as_str().unwrap_or("./docs");
                let quality_result = self.check_documentation_quality(doc_path).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "completed",
                        "pages_checked": quality_result.pages_checked,
                        "quality_score": quality_result.quality_score,
                        "issues_found": quality_result.issues_found,
                        "suggestions": quality_result.suggestions,
                        "check_duration_ms": quality_result.check_duration_ms
                    })
                ))
            }
            "deploy_documentation" => {
                let doc_path = request.payload["doc_path"].as_str().unwrap_or("./docs");
                let target_url = request.payload["target_url"].as_str().unwrap_or("https://docs.example.com");
                
                // Simulate documentation deployment
                tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
                
                Ok(AgentResponse::success(
                    request.id,
                    json!({
                        "status": "deployed",
                        "doc_path": doc_path,
                        "target_url": target_url,
                        "deployment_duration_ms": 600,
                        "files_deployed": 45
                    })
                ))
            }
            _ => {
                Ok(AgentResponse::error(
                    request.id,
                    format!("Unknown documentation task type: {}", request.request_type)
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

/// Documentation Generator - Simulates a documentation generation engine
struct DocumentationGenerator {
    initialized: bool,
}

impl DocumentationGenerator {
    fn new() -> Self {
        Self { initialized: false }
    }

    async fn initialize(&mut self) {
        // Simulate initialization
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        self.initialized = true;
    }
}

/// Documentation Quality Checker - Simulates a documentation quality checking engine
struct DocumentationQualityChecker {
    initialized: bool,
}

impl DocumentationQualityChecker {
    fn new() -> Self {
        Self { initialized: false }
    }

    async fn initialize(&mut self) {
        // Simulate initialization
        tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
        self.initialized = true;
    }
}

/// Result types for documentation tasks
#[derive(Debug)]
struct DocumentationResult {
    pages_generated: u32,
    endpoints_documented: u32,
    examples_created: u32,
    generation_duration_ms: u64,
}

#[derive(Debug)]
struct CodeDocAnalysisResult {
    files_analyzed: u32,
    documented_functions: u32,
    undocumented_functions: u32,
    documentation_coverage: f64,
    analysis_duration_ms: u64,
}

#[derive(Debug)]
struct QualityCheckResult {
    pages_checked: u32,
    quality_score: f64,
    issues_found: u32,
    suggestions: u32,
    check_duration_ms: u64,
}

/// Create a documentation workflow
fn create_documentation_workflow() -> WorkflowDefinition {
    let steps = vec![
        // Step 1: Analyze existing documentation
        WorkflowStep::new(
            "analyze_docs".to_string(),
            "Analyze Documentation".to_string(),
            WorkflowStepType::Task {
                agent_id: "doc-agent".to_string(),
                request: AgentRequest::new("analyze_code_docs".to_string(), json!({
                    "code_path": "./src"
                })),
            },
        ).with_description("Analyze existing code documentation".to_string()),

        // Step 2: Generate API documentation
        WorkflowStep::new(
            "generate_api_docs".to_string(),
            "Generate API Documentation".to_string(),
            WorkflowStepType::Task {
                agent_id: "doc-agent".to_string(),
                request: AgentRequest::new("generate_api_docs".to_string(), json!({
                    "api_spec_path": "./api/openapi.yaml"
                })),
            },
        ).with_description("Generate API documentation from OpenAPI spec".to_string()),

        // Step 3: Check documentation quality
        WorkflowStep::new(
            "check_quality".to_string(),
            "Check Documentation Quality".to_string(),
            WorkflowStepType::Task {
                agent_id: "doc-agent".to_string(),
                request: AgentRequest::new("check_doc_quality".to_string(), json!({
                    "doc_path": "./docs"
                })),
            },
        ).with_description("Check documentation quality and completeness".to_string()),

        // Step 4: Conditional documentation improvement
        WorkflowStep::new(
            "improve_docs".to_string(),
            "Improve Documentation".to_string(),
            WorkflowStepType::Conditional {
                condition: WorkflowCondition::VariableEquals {
                    variable: "quality_issues".to_string(),
                    value: json!(true),
                },
                if_true: vec![
                    WorkflowStep::new(
                        "fix_issues".to_string(),
                        "Fix Documentation Issues".to_string(),
                        WorkflowStepType::Task {
                            agent_id: "doc-agent".to_string(),
                            request: AgentRequest::new("fix_doc_issues".to_string(), json!({})),
                        },
                    ),
                ],
                if_false: None,
            },
        ).with_description("Fix documentation issues if found".to_string()),

        // Step 5: Deploy documentation
        WorkflowStep::new(
            "deploy_docs".to_string(),
            "Deploy Documentation".to_string(),
            WorkflowStepType::Task {
                agent_id: "doc-agent".to_string(),
                request: AgentRequest::new("deploy_documentation".to_string(), json!({
                    "doc_path": "./docs",
                    "target_url": "https://docs.example.com"
                })),
            },
        ).with_description("Deploy documentation to hosting platform".to_string()),
    ];

    WorkflowDefinition::new(
        "documentation-workflow".to_string(),
        "Documentation Pipeline".to_string(),
        steps,
    )
    .with_description("Complete documentation generation and deployment pipeline".to_string())
    .with_input_parameter(WorkflowParameter {
        name: "api_spec_path".to_string(),
        description: Some("Path to OpenAPI specification file".to_string()),
        parameter_type: "string".to_string(),
        required: false,
        default_value: Some(json!("./api/openapi.yaml")),
    })
    .with_input_parameter(WorkflowParameter {
        name: "target_url".to_string(),
        description: Some("Target URL for documentation deployment".to_string()),
        parameter_type: "string".to_string(),
        required: false,
        default_value: Some(json!("https://docs.example.com")),
    })
    .with_tag("documentation".to_string())
    .with_tag("api".to_string())
    .with_tag("deployment".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Custom Agent Implementation Example");
    println!("===============================================");

    // Create the agent framework
    let mut framework = RhemaAgentFramework::new();
    framework.initialize().await?;

    // Create and register custom agents
    println!("\n📋 Creating and registering custom agents...");

    // Security Analysis Agent
    let security_config = AgentConfig {
        name: "Security Analysis Agent".to_string(),
        description: Some("Handles security scanning, dependency analysis, and compliance checking".to_string()),
        agent_type: AgentType::Security,
        capabilities: vec![
            AgentCapability::CodeExecution,
            AgentCapability::FileRead,
            AgentCapability::Analysis,
            AgentCapability::Security,
        ],
        max_concurrent_tasks: 2,
        task_timeout: 600,
        retry_attempts: 2,
        retry_delay: 10,
        memory_limit: Some(1024), // 1GB
        cpu_limit: Some(75.0),    // 75% CPU
        parameters: HashMap::new(),
        tags: vec!["security".to_string(), "analysis".to_string(), "compliance".to_string()],
    };
    
    let security_agent = SecurityAnalysisAgent::new("security-agent".to_string(), security_config);
    framework.register_agent(Box::new(security_agent)).await?;

    // Documentation Agent
    let doc_config = AgentConfig {
        name: "Documentation Agent".to_string(),
        description: Some("Handles documentation generation, analysis, and deployment".to_string()),
        agent_type: AgentType::Documentation,
        capabilities: vec![
            AgentCapability::FileRead,
            AgentCapability::FileWrite,
            AgentCapability::Documentation,
        ],
        max_concurrent_tasks: 3,
        task_timeout: 900,
        retry_attempts: 1,
        retry_delay: 5,
        memory_limit: Some(512), // 512MB
        cpu_limit: Some(50.0),   // 50% CPU
        parameters: HashMap::new(),
        tags: vec!["documentation".to_string(), "generation".to_string(), "deployment".to_string()],
    };
    
    let doc_agent = DocumentationAgent::new("doc-agent".to_string(), doc_config);
    framework.register_agent(Box::new(doc_agent)).await?;

    // Start all agents
    println!("🚀 Starting agents...");
    framework.start_agent(&"security-agent".to_string()).await?;
    framework.start_agent(&"doc-agent".to_string()).await?;

    // Register workflows
    println!("\n📋 Registering workflows...");

    let security_workflow = create_security_workflow();
    framework.register_workflow(security_workflow).await?;
    println!("✅ Registered security workflow");

    let documentation_workflow = create_documentation_workflow();
    framework.register_workflow(documentation_workflow).await?;
    println!("✅ Registered documentation workflow");

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

    // Test individual agent tasks
    println!("\n🧪 Testing individual agent tasks...");

    // Test security agent
    let security_request = AgentRequest::new("security_scan".to_string(), json!({
        "code_path": "./src"
    }));
    let security_response = framework.execute_task(&"security-agent".to_string(), security_request).await?;
    println!("Security scan result: {:?}", security_response.payload);

    // Test documentation agent
    let doc_request = AgentRequest::new("analyze_code_docs".to_string(), json!({
        "code_path": "./src"
    }));
    let doc_response = framework.execute_task(&"doc-agent".to_string(), doc_request).await?;
    println!("Documentation analysis result: {:?}", doc_response.payload);

    // Start security workflow
    println!("\n🚀 Starting security workflow...");
    let mut security_params = HashMap::new();
    security_params.insert("code_path".to_string(), json!("./src"));
    security_params.insert("compliance_standard".to_string(), json!("OWASP"));

    let security_execution_id = framework.start_workflow("security-workflow", security_params).await?;
    println!("✅ Started security workflow execution: {}", security_execution_id);

    // Start documentation workflow
    println!("\n🚀 Starting documentation workflow...");
    let mut doc_params = HashMap::new();
    doc_params.insert("api_spec_path".to_string(), json!("./api/openapi.yaml"));
    doc_params.insert("target_url".to_string(), json!("https://docs.example.com"));

    let doc_execution_id = framework.start_workflow("documentation-workflow", doc_params).await?;
    println!("✅ Started documentation workflow execution: {}", doc_execution_id);

    // Monitor workflow executions
    println!("\n📊 Monitoring workflow executions...");
    let mut security_completed = false;
    let mut doc_completed = false;
    let mut check_count = 0;
    const MAX_CHECKS: usize = 20;

    while (!security_completed || !doc_completed) && check_count < MAX_CHECKS {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // Check security workflow
        if !security_completed {
            if let Some(context) = framework.get_workflow_status(&security_execution_id).await? {
                println!("Security workflow: {} | Step: {}/{}", 
                    context.status, 
                    context.current_step_index, 
                    context.definition.steps.len()
                );
                security_completed = context.is_completed();
            }
        }

        // Check documentation workflow
        if !doc_completed {
            if let Some(context) = framework.get_workflow_status(&doc_execution_id).await? {
                println!("Documentation workflow: {} | Step: {}/{}", 
                    context.status, 
                    context.current_step_index, 
                    context.definition.steps.len()
                );
                doc_completed = context.is_completed();
            }
        }

        check_count += 1;
    }

    if security_completed && doc_completed {
        println!("\n✅ Both workflows completed successfully!");
    } else {
        println!("\n⏰ Some workflows may still be running");
    }

    // Get framework statistics
    println!("\n📊 Framework Statistics:");
    let stats = framework.get_framework_stats().await?;
    println!("{}", stats);

    // Get agent metrics
    println!("\n📊 Agent Metrics:");
    let security_metrics = framework.get_agent_metrics(&"security-agent".to_string()).await?;
    println!("Security Agent: {} tasks completed, {} errors", 
        security_metrics.task_count, security_metrics.error_count);

    let doc_metrics = framework.get_agent_metrics(&"doc-agent".to_string()).await?;
    println!("Documentation Agent: {} tasks completed, {} errors", 
        doc_metrics.task_count, doc_metrics.error_count);

    // Shutdown framework
    println!("\n🛑 Shutting down framework...");
    framework.shutdown().await?;
    println!("✅ Framework shutdown complete");

    println!("\n🎉 Custom Agent Implementation Example completed successfully!");
    Ok(())
} 