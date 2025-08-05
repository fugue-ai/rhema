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

use crate::agent::{
    Agent, AgentConfig, AgentContext, AgentId, AgentMessage, AgentRequest, AgentResponse,
    AgentType, BaseAgent, HealthStatus,
};
use crate::agent::AgentCapability;
use crate::error::{AgentError, AgentResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tokio::process::Command;
use chrono::{DateTime, Utc};
use regex::Regex;

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Project name
    pub project_name: String,
    /// Project version
    pub version: String,
    /// Documentation type
    pub doc_type: DocumentationType,
    /// Output format
    pub output_format: OutputFormat,
    /// Source code path
    pub source_path: String,
    /// Output directory
    pub output_directory: String,
    /// Template configuration
    pub template_config: Option<TemplateConfig>,
    /// API documentation configuration
    pub api_config: Option<ApiDocConfig>,
    /// Code documentation configuration
    pub code_config: Option<CodeDocConfig>,
    /// Custom options
    pub options: HashMap<String, serde_json::Value>,
}

/// Documentation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentationType {
    API,
    Code,
    UserGuide,
    DeveloperGuide,
    Architecture,
    Deployment,
    Custom(String),
}

/// Output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    HTML,
    Markdown,
    PDF,
    JSON,
    OpenAPI,
    Custom(String),
}

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Template path
    pub template_path: String,
    /// Template variables
    pub variables: HashMap<String, String>,
    /// Custom CSS
    pub custom_css: Option<String>,
    /// Custom JavaScript
    pub custom_js: Option<String>,
}

/// API documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDocConfig {
    /// API base URL
    pub base_url: String,
    /// API version
    pub api_version: String,
    /// Authentication methods
    pub auth_methods: Vec<AuthMethod>,
    /// Include examples
    pub include_examples: bool,
    /// Include schemas
    pub include_schemas: bool,
    /// Include responses
    pub include_responses: bool,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Bearer,
    APIKey,
    OAuth2,
    Basic,
    Custom(String),
}

/// Code documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDocConfig {
    /// Include private items
    pub include_private: bool,
    /// Include tests
    pub include_tests: bool,
    /// Include examples
    pub include_examples: bool,
    /// Documentation style
    pub style: DocStyle,
    /// Language-specific options
    pub language_options: HashMap<String, serde_json::Value>,
}

/// Documentation style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocStyle {
    RustDoc,
    JSDoc,
    PythonDoc,
    JavaDoc,
    Custom(String),
}

/// Documentation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationRequest {
    /// Documentation configuration
    pub config: DocumentationConfig,
    /// Force regeneration
    pub force_regenerate: bool,
    /// Include diagrams
    pub include_diagrams: bool,
    /// Custom options
    pub options: HashMap<String, serde_json::Value>,
}

/// Documentation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationResult {
    /// Documentation ID
    pub doc_id: String,
    /// Generation status
    pub status: DocumentationStatus,
    /// Generated files
    pub generated_files: Vec<GeneratedFile>,
    /// Documentation metrics
    pub metrics: DocumentationMetrics,
    /// Generation logs
    pub logs: Vec<String>,
    /// Error message if any
    pub error: Option<String>,
}

/// Documentation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentationStatus {
    Pending,
    Analyzing,
    Generating,
    Completed,
    Failed,
}

/// Generated file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// File path
    pub path: String,
    /// File size in bytes
    pub size: u64,
    /// File type
    pub file_type: String,
    /// Generation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Documentation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationMetrics {
    /// Total files processed
    pub files_processed: u32,
    /// Total functions documented
    pub functions_documented: u32,
    /// Total classes documented
    pub classes_documented: u32,
    /// Total APIs documented
    pub apis_documented: u32,
    /// Documentation coverage percentage
    pub coverage_percentage: f64,
    /// Generation time in seconds
    pub generation_time: f64,
}

/// Code analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysisResult {
    /// Functions found
    pub functions: Vec<FunctionInfo>,
    /// Classes found
    pub classes: Vec<ClassInfo>,
    /// Modules found
    pub modules: Vec<ModuleInfo>,
    /// Dependencies found
    pub dependencies: Vec<DependencyInfo>,
}

/// Function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    /// Function signature
    pub signature: String,
    /// Function documentation
    pub documentation: Option<String>,
    /// Function location
    pub location: String,
    /// Function parameters
    pub parameters: Vec<ParameterInfo>,
    /// Return type
    pub return_type: Option<String>,
    /// Function visibility
    pub visibility: Visibility,
}

/// Class information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    /// Class name
    pub name: String,
    /// Class documentation
    pub documentation: Option<String>,
    /// Class location
    pub location: String,
    /// Class methods
    pub methods: Vec<FunctionInfo>,
    /// Class properties
    pub properties: Vec<PropertyInfo>,
    /// Class inheritance
    pub inheritance: Vec<String>,
}

/// Module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Module path
    pub path: String,
    /// Module documentation
    pub documentation: Option<String>,
    /// Module exports
    pub exports: Vec<String>,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Dependency type
    pub dep_type: DependencyType,
    /// Dependency description
    pub description: Option<String>,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter documentation
    pub documentation: Option<String>,
    /// Parameter default value
    pub default_value: Option<String>,
}

/// Property information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: String,
    /// Property documentation
    pub documentation: Option<String>,
    /// Property visibility
    pub visibility: Visibility,
}

/// Visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Dependency type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Runtime,
    Development,
    Test,
    Build,
}

/// Documentation Agent
pub struct DocumentationAgent {
    /// Base agent
    base: BaseAgent,
    /// Documentation history
    doc_history: HashMap<String, DocumentationResult>,
    /// Active generations
    active_generations: HashMap<String, DocumentationConfig>,
    /// Code analysis cache
    analysis_cache: HashMap<String, CodeAnalysisResult>,
}

impl DocumentationAgent {
    /// Create a new documentation agent
    pub fn new(id: AgentId) -> Self {
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
            tags: vec![
                "documentation".to_string(),
                "api-docs".to_string(),
                "code-docs".to_string(),
            ],
        };

        Self {
            base: BaseAgent::new(id, config),
            doc_history: HashMap::new(),
            active_generations: HashMap::new(),
            analysis_cache: HashMap::new(),
        }
    }

    /// Generate documentation
    async fn generate_documentation(&mut self, request: DocumentationRequest) -> AgentResult<DocumentationResult> {
        let doc_id = uuid::Uuid::new_v4().to_string();
        let mut result = DocumentationResult {
            doc_id: doc_id.clone(),
            status: DocumentationStatus::Pending,
            generated_files: Vec::new(),
            metrics: DocumentationMetrics {
                files_processed: 0,
                functions_documented: 0,
                classes_documented: 0,
                apis_documented: 0,
                coverage_percentage: 0.0,
                generation_time: 0.0,
            },
            logs: Vec::new(),
            error: None,
        };

        // Store active generation
        self.active_generations.insert(doc_id.clone(), request.config.clone());

        let start_time = std::time::Instant::now();

        // Update status
        result.status = DocumentationStatus::Analyzing;
        result.logs.push("Starting code analysis...".to_string());

        // Analyze code
        match self.analyze_code(&request.config, &mut result).await {
            Ok(analysis) => {
                result.status = DocumentationStatus::Generating;
                result.logs.push("Code analysis completed, generating documentation...".to_string());
                self.analysis_cache.insert(request.config.source_path.clone(), analysis);
            }
            Err(e) => {
                result.status = DocumentationStatus::Failed;
                result.error = Some(e.to_string());
                result.logs.push(format!("Code analysis failed: {}", e));
                self.doc_history.insert(doc_id.clone(), result.clone());
                return Ok(result);
            }
        }

        // Generate documentation based on type
        match request.config.doc_type {
            DocumentationType::API => {
                self.generate_api_documentation(&request.config, &mut result).await?;
            }
            DocumentationType::Code => {
                self.generate_code_documentation(&request.config, &mut result).await?;
            }
            DocumentationType::UserGuide => {
                self.generate_user_guide(&request.config, &mut result).await?;
            }
            DocumentationType::DeveloperGuide => {
                self.generate_developer_guide(&request.config, &mut result).await?;
            }
            DocumentationType::Architecture => {
                self.generate_architecture_docs(&request.config, &mut result).await?;
            }
            DocumentationType::Deployment => {
                self.generate_deployment_docs(&request.config, &mut result).await?;
            }
            DocumentationType::Custom(_) => {
                self.generate_custom_documentation(&request.config, &mut result).await?;
            }
        }

        // Calculate metrics
        let generation_time = start_time.elapsed().as_secs_f64();
        result.metrics.generation_time = generation_time;
        result.status = DocumentationStatus::Completed;

        result.logs.push("Documentation generation completed successfully".to_string());

        // Store in history
        self.doc_history.insert(doc_id.clone(), result.clone());
        self.active_generations.remove(&doc_id);

        Ok(result)
    }

    /// Analyze code structure
    async fn analyze_code(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<CodeAnalysisResult> {
        let source_path = Path::new(&config.source_path);
        if !source_path.exists() {
            return Err(AgentError::ExecutionFailed { reason: "Source path does not exist".to_string() });
        }

        let mut analysis = CodeAnalysisResult {
            functions: Vec::new(),
            classes: Vec::new(),
            modules: Vec::new(),
            dependencies: Vec::new(),
        };

        // Analyze Rust code
        if let Some(cargo_toml) = self.find_cargo_toml(source_path).await {
            analysis.dependencies = self.parse_cargo_dependencies(&cargo_toml).await?;
        }

        // Parse source files
        self.parse_source_files(source_path, &mut analysis).await?;

        // Update metrics
        result.metrics.functions_documented = analysis.functions.len() as u32;
        result.metrics.classes_documented = analysis.classes.len() as u32;
        result.metrics.files_processed = self.count_source_files(source_path).await?;

        Ok(analysis)
    }

    /// Find Cargo.toml file
    async fn find_cargo_toml(&self, source_path: &Path) -> Option<PathBuf> {
        let cargo_toml = source_path.join("Cargo.toml");
        if cargo_toml.exists() {
            Some(cargo_toml)
        } else {
            None
        }
    }

    /// Parse Cargo dependencies
    async fn parse_cargo_dependencies(&self, cargo_toml_path: &Path) -> AgentResult<Vec<DependencyInfo>> {
        let content = fs::read_to_string(cargo_toml_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read Cargo.toml: {}", e) })?;

        let mut dependencies = Vec::new();

        // Simple regex-based parsing (in production, use proper TOML parser)
        let dep_regex = Regex::new(r#"(\w+)\s*=\s*"([^"]+)"#).unwrap();
        for cap in dep_regex.captures_iter(&content) {
            dependencies.push(DependencyInfo {
                name: cap[1].to_string(),
                version: cap[2].to_string(),
                dep_type: DependencyType::Runtime,
                description: None,
            });
        }

        Ok(dependencies)
    }

    /// Parse source files
    async fn parse_source_files(&self, source_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        if source_path.is_file() {
            self.parse_single_file(source_path, analysis).await?;
        } else if source_path.is_dir() {
            self.parse_directory(source_path, analysis).await?;
        }

        Ok(())
    }

    /// Parse single file
    async fn parse_single_file(&self, file_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        if let Some(extension) = file_path.extension() {
            match extension.to_str().unwrap() {
                "rs" => self.parse_rust_file(file_path, analysis).await?,
                "py" => self.parse_python_file(file_path, analysis).await?,
                "js" | "ts" => self.parse_javascript_file(file_path, analysis).await?,
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse directory recursively
    async fn parse_directory(&self, dir_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read directory: {}", e) })?;

        for entry in entries {
            let entry = entry.map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read entry: {}", e) })?;
            let path = entry.path();

            if path.is_file() {
                self.parse_single_file(&path, analysis).await?;
            } else if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                Box::pin(self.parse_directory(&path, analysis)).await?;
            }
        }

        Ok(())
    }

    /// Parse Rust file
    async fn parse_rust_file(&self, file_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read file: {}", e) })?;

        // Parse functions
        let fn_regex = Regex::new(r#"pub\s+fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^{]+))?\s*\{?"#).unwrap();
        for cap in fn_regex.captures_iter(&content) {
            let function = FunctionInfo {
                name: cap[1].to_string(),
                signature: cap[0].to_string(),
                documentation: self.extract_rust_doc(&content, &cap[1]).await,
                location: file_path.to_string_lossy().to_string(),
                parameters: self.parse_rust_parameters(&cap[2]).await,
                return_type: cap.get(3).map(|m| m.as_str().trim().to_string()),
                visibility: Visibility::Public,
            };
            analysis.functions.push(function);
        }

        // Parse modules
        let mod_regex = Regex::new(r#"pub\s+mod\s+(\w+)"#).unwrap();
        for cap in mod_regex.captures_iter(&content) {
            let module = ModuleInfo {
                name: cap[1].to_string(),
                path: file_path.to_string_lossy().to_string(),
                documentation: None,
                exports: Vec::new(),
            };
            analysis.modules.push(module);
        }

        Ok(())
    }

    /// Parse Python file
    async fn parse_python_file(&self, file_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read file: {}", e) })?;

        // Parse functions
        let fn_regex = Regex::new(r#"def\s+(\w+)\s*\(([^)]*)\):"#).unwrap();
        for cap in fn_regex.captures_iter(&content) {
            let function = FunctionInfo {
                name: cap[1].to_string(),
                signature: cap[0].to_string(),
                documentation: self.extract_python_doc(&content, &cap[1]).await,
                location: file_path.to_string_lossy().to_string(),
                parameters: self.parse_python_parameters(&cap[2]).await,
                return_type: None,
                visibility: Visibility::Public,
            };
            analysis.functions.push(function);
        }

        // Parse classes
        let class_regex = Regex::new(r#"class\s+(\w+)"#).unwrap();
        for cap in class_regex.captures_iter(&content) {
            let class = ClassInfo {
                name: cap[1].to_string(),
                documentation: None,
                location: file_path.to_string_lossy().to_string(),
                methods: Vec::new(),
                properties: Vec::new(),
                inheritance: Vec::new(),
            };
            analysis.classes.push(class);
        }

        Ok(())
    }

    /// Parse JavaScript/TypeScript file
    async fn parse_javascript_file(&self, file_path: &Path, analysis: &mut CodeAnalysisResult) -> AgentResult<()> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read file: {}", e) })?;

        // Parse functions
        let fn_regex = Regex::new(r#"(?:export\s+)?(?:function\s+)?(\w+)\s*\(([^)]*)\)"#).unwrap();
        for cap in fn_regex.captures_iter(&content) {
            let function = FunctionInfo {
                name: cap[1].to_string(),
                signature: cap[0].to_string(),
                documentation: self.extract_js_doc(&content, &cap[1]).await,
                location: file_path.to_string_lossy().to_string(),
                parameters: self.parse_js_parameters(&cap[2]).await,
                return_type: None,
                visibility: Visibility::Public,
            };
            analysis.functions.push(function);
        }

        Ok(())
    }

    /// Extract Rust documentation
    async fn extract_rust_doc(&self, content: &str, function_name: &str) -> Option<String> {
        let doc_regex = Regex::new(&format!(r#"(?s)/\*\*?\s*(.*?)\s*\*/\s*(?:pub\s+)?fn\s+{}"#, function_name)).unwrap();
        doc_regex.captures(content).map(|cap| cap[1].trim().to_string())
    }

    /// Extract Python documentation
    async fn extract_python_doc(&self, content: &str, function_name: &str) -> Option<String> {
        let doc_regex = Regex::new(&format!(r#""""(.*?)"""\s*def\s+{}"#, function_name)).unwrap();
        doc_regex.captures(content).map(|cap| cap[1].trim().to_string())
    }

    /// Extract JavaScript documentation
    async fn extract_js_doc(&self, content: &str, function_name: &str) -> Option<String> {
        let doc_regex = Regex::new(&format!(r#"/\*\*\s*(.*?)\s*\*/\s*(?:export\s+)?(?:function\s+)?{}"#, function_name)).unwrap();
        doc_regex.captures(content).map(|cap| cap[1].trim().to_string())
    }

    /// Parse Rust parameters
    async fn parse_rust_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let parts: Vec<&str> = param.trim().split(':').collect();
                if parts.len() >= 2 {
                    Some(ParameterInfo {
                        name: parts[0].trim().to_string(),
                        param_type: parts[1].trim().to_string(),
                        documentation: None,
                        default_value: None,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Parse Python parameters
    async fn parse_python_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if !param.is_empty() {
                    Some(ParameterInfo {
                        name: param.to_string(),
                        param_type: "Any".to_string(),
                        documentation: None,
                        default_value: None,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Parse JavaScript parameters
    async fn parse_js_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if !param.is_empty() {
                    Some(ParameterInfo {
                        name: param.to_string(),
                        param_type: "any".to_string(),
                        documentation: None,
                        default_value: None,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Count source files
    async fn count_source_files(&self, source_path: &Path) -> AgentResult<u32> {
        let mut count = 0;
        if source_path.is_file() {
            count = 1;
        } else if source_path.is_dir() {
                    let entries = fs::read_dir(source_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read directory: {}", e) })?;

        for entry in entries {
            let entry = entry.map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to read entry: {}", e) })?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        match extension.to_str().unwrap() {
                            "rs" | "py" | "js" | "ts" => count += 1,
                            _ => {}
                        }
                    }
                } else if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                    count += Box::pin(self.count_source_files(&path)).await?;
                }
            }
        }

        Ok(count)
    }

    /// Generate API documentation
    async fn generate_api_documentation(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating API documentation...".to_string());

        let output_path = Path::new(&config.output_directory);
        fs::create_dir_all(output_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to create output directory: {}", e) })?;

        // Generate OpenAPI specification
        if let Some(api_config) = &config.api_config {
            let openapi_spec = self.generate_openapi_spec(api_config, result).await?;
            let openapi_path = output_path.join("openapi.json");
            fs::write(&openapi_path, serde_json::to_string_pretty(&openapi_spec).unwrap())
                .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to write OpenAPI spec: {}", e) })?;

            result.generated_files.push(GeneratedFile {
                path: openapi_path.to_string_lossy().to_string(),
                size: fs::metadata(&openapi_path).unwrap().len(),
                file_type: "application/json".to_string(),
                timestamp: Utc::now(),
            });
        }

        // Generate HTML documentation
        if config.output_format == OutputFormat::HTML {
            let html_content = self.generate_html_docs(config, result).await?;
            let html_path = output_path.join("index.html");
            fs::write(&html_path, html_content)
                .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to write HTML docs: {}", e) })?;

            result.generated_files.push(GeneratedFile {
                path: html_path.to_string_lossy().to_string(),
                size: fs::metadata(&html_path).unwrap().len(),
                file_type: "text/html".to_string(),
                timestamp: Utc::now(),
            });
        }

        Ok(())
    }

    /// Generate code documentation
    async fn generate_code_documentation(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating code documentation...".to_string());

        let output_path = Path::new(&config.output_directory);
        fs::create_dir_all(output_path)
            .map_err(|e| AgentError::ExecutionFailed { reason: format!("Failed to create output directory: {}", e) })?;

        // Use rustdoc for Rust projects
        if let Some(cargo_toml) = self.find_cargo_toml(Path::new(&config.source_path)).await {
            let output = Command::new("cargo")
                .args(&["doc", "--no-deps", "--output-dir", &config.output_directory])
                .current_dir(&config.source_path)
                .output()
                .await
                .map_err(|e| AgentError::ExecutionFailed { reason: format!("rustdoc failed: {}", e) })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(AgentError::ExecutionFailed { reason: format!("rustdoc failed: {}", error) });
            }

            result.logs.push("rustdoc documentation generated successfully".to_string());
        }

        Ok(())
    }

    /// Generate user guide
    async fn generate_user_guide(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating user guide...".to_string());
        // Implementation for user guide generation
        Ok(())
    }

    /// Generate developer guide
    async fn generate_developer_guide(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating developer guide...".to_string());
        // Implementation for developer guide generation
        Ok(())
    }

    /// Generate architecture documentation
    async fn generate_architecture_docs(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating architecture documentation...".to_string());
        // Implementation for architecture documentation generation
        Ok(())
    }

    /// Generate deployment documentation
    async fn generate_deployment_docs(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating deployment documentation...".to_string());
        // Implementation for deployment documentation generation
        Ok(())
    }

    /// Generate custom documentation
    async fn generate_custom_documentation(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<()> {
        result.logs.push("Generating custom documentation...".to_string());
        // Implementation for custom documentation generation
        Ok(())
    }

    /// Generate OpenAPI specification
    async fn generate_openapi_spec(&self, api_config: &ApiDocConfig, result: &mut DocumentationResult) -> AgentResult<serde_json::Value> {
        let mut spec = serde_json::json!({
            "openapi": "3.0.0",
            "info": {
                "title": "API Documentation",
                "version": api_config.api_version,
                "description": "Auto-generated API documentation"
            },
            "servers": [
                {
                    "url": api_config.base_url,
                    "description": "API Server"
                }
            ],
            "paths": {},
            "components": {
                "securitySchemes": {}
            }
        });

        // Add security schemes
        for auth_method in &api_config.auth_methods {
            match auth_method {
                AuthMethod::Bearer => {
                    spec["components"]["securitySchemes"]["bearerAuth"] = serde_json::json!({
                        "type": "http",
                        "scheme": "bearer",
                        "bearerFormat": "JWT"
                    });
                }
                AuthMethod::APIKey => {
                    spec["components"]["securitySchemes"]["apiKeyAuth"] = serde_json::json!({
                        "type": "apiKey",
                        "in": "header",
                        "name": "X-API-Key"
                    });
                }
                _ => {}
            }
        }

        Ok(spec)
    }

    /// Generate HTML documentation
    async fn generate_html_docs(&self, config: &DocumentationConfig, result: &mut DocumentationResult) -> AgentResult<String> {
        let html_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{project_name}} Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; }
        .header { background: #f5f5f5; padding: 20px; border-radius: 5px; }
        .content { margin-top: 20px; }
        .section { margin-bottom: 30px; }
        .function { background: #f9f9f9; padding: 15px; margin: 10px 0; border-radius: 3px; }
        .function-name { font-weight: bold; color: #333; }
        .function-signature { font-family: monospace; background: #eee; padding: 5px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>{{project_name}} Documentation</h1>
        <p>Version: {{version}}</p>
        <p>Generated on: {{timestamp}}</p>
    </div>
    <div class="content">
        <div class="section">
            <h2>Functions</h2>
            {{#functions}}
            <div class="function">
                <div class="function-name">{{name}}</div>
                <div class="function-signature">{{signature}}</div>
                {{#documentation}}
                <p>{{documentation}}</p>
                {{/documentation}}
            </div>
            {{/functions}}
        </div>
    </div>
</body>
</html>
        "#;

        // Simple template replacement (in production, use a proper template engine)
        let html = html_template
            .replace("{{project_name}}", &config.project_name)
            .replace("{{version}}", &config.version)
            .replace("{{timestamp}}", &Utc::now().to_rfc3339());

        Ok(html)
    }

    /// Get documentation history
    pub fn get_documentation_history(&self) -> &HashMap<String, DocumentationResult> {
        &self.doc_history
    }

    /// Get active generations
    pub fn get_active_generations(&self) -> &HashMap<String, DocumentationConfig> {
        &self.active_generations
    }
}

#[async_trait]
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

    async fn initialize(&mut self) -> AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> AgentResult<()> {
        self.base.stop().await
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
        match request.request_type.as_str() {
            "generate_documentation" => {
                let doc_request: DocumentationRequest = serde_json::from_value(request.payload)
                    .map_err(|e| AgentError::SerializationError { reason: e.to_string() })?;

                let start_time = std::time::Instant::now();
                let result = self.generate_documentation(doc_request).await?;
                let execution_time = start_time.elapsed().as_millis() as u64;

                Ok(AgentResponse::success(request.id, serde_json::to_value(result).unwrap())
                    .with_execution_time(execution_time))
            }
            "get_documentation_history" => {
                let history = self.get_documentation_history();
                Ok(AgentResponse::success(request.id, serde_json::to_value(history).unwrap()))
            }
            "get_active_generations" => {
                let active = self.get_active_generations();
                Ok(AgentResponse::success(request.id, serde_json::to_value(active).unwrap()))
            }
            _ => {
                Ok(AgentResponse::error(request.id, "Unknown task type".to_string()))
            }
        }
    }

    async fn get_status(&self) -> AgentResult<crate::agent::AgentStatus> {
        self.base.get_status().await
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
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_documentation_agent_creation() {
        let agent = DocumentationAgent::new("test-doc-agent".to_string());
        assert_eq!(agent.id(), "test-doc-agent");
        assert_eq!(agent.config().agent_type, AgentType::Documentation);
    }

    #[tokio::test]
    async fn test_documentation_agent_initialization() {
        let mut agent = DocumentationAgent::new("test-doc-agent".to_string());
        assert!(agent.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_documentation_config_creation() {
        let config = DocumentationConfig {
            project_name: "test-project".to_string(),
            version: "1.0.0".to_string(),
            doc_type: DocumentationType::API,
            output_format: OutputFormat::HTML,
            source_path: "/tmp/test".to_string(),
            output_directory: "/tmp/output".to_string(),
            template_config: None,
            api_config: None,
            code_config: None,
            options: HashMap::new(),
        };

        assert_eq!(config.project_name, "test-project");
        assert_eq!(config.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_documentation_request_creation() {
        let config = DocumentationConfig {
            project_name: "test-project".to_string(),
            version: "1.0.0".to_string(),
            doc_type: DocumentationType::API,
            output_format: OutputFormat::HTML,
            source_path: "/tmp/test".to_string(),
            output_directory: "/tmp/output".to_string(),
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

        assert_eq!(request.config.project_name, "test-project");
        assert_eq!(request.include_diagrams, true);
    }
} 