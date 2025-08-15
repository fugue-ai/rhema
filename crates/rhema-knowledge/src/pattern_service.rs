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

use crate::{
    engine::UnifiedKnowledgeEngine,
    proactive::{UsageAnalyzer, UsagePattern},
    temporal::TemporalContextManager,
    types::WorkflowType,
};
use rhema_core::{
    schema::knowledge::{PatternEntry, PatternMaturity, ReviewStatus},
    RhemaResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Pattern service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternServiceConfig {
    /// Enable pattern discovery from code analysis
    pub enable_pattern_discovery: bool,

    /// Enable pattern recommendation engine
    pub enable_recommendations: bool,

    /// Enable pattern usage tracking
    pub enable_usage_tracking: bool,

    /// Enable pattern quality scoring
    pub enable_quality_scoring: bool,

    /// Pattern discovery confidence threshold
    pub discovery_confidence_threshold: f32,

    /// Maximum patterns to recommend
    pub max_recommendations: usize,

    /// Pattern analysis window in hours
    pub analysis_window_hours: u64,
}

impl Default for PatternServiceConfig {
    fn default() -> Self {
        Self {
            enable_pattern_discovery: true,
            enable_recommendations: true,
            enable_usage_tracking: true,
            enable_quality_scoring: true,
            discovery_confidence_threshold: 0.7,
            max_recommendations: 10,
            analysis_window_hours: 24,
        }
    }
}

/// Pattern recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecommendation {
    /// Pattern ID
    pub pattern_id: String,

    /// Pattern name
    pub pattern_name: String,

    /// Recommendation score (0.0 - 1.0)
    pub score: f32,

    /// Recommendation reason
    pub reason: String,

    /// Applicable context
    pub context: String,

    /// Expected benefits
    pub benefits: Vec<String>,

    /// Implementation effort
    pub effort: ImplementationEffort,
}

/// Implementation effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Pattern quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternQualityMetrics {
    /// Overall quality score (0.0 - 1.0)
    pub overall_score: f32,

    /// Documentation quality (0.0 - 1.0)
    pub documentation_score: f32,

    /// Example quality (0.0 - 1.0)
    pub example_score: f32,

    /// Usage tracking quality (0.0 - 1.0)
    pub usage_score: f32,

    /// Review quality (0.0 - 1.0)
    pub review_score: f32,

    /// Improvement suggestions
    pub suggestions: Vec<String>,
}

/// Pattern discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDiscoveryResult {
    /// Discovered pattern name
    pub name: String,

    /// Pattern description
    pub description: String,

    /// Pattern type
    pub pattern_type: String,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,

    /// Source files where pattern was found
    pub source_files: Vec<String>,

    /// Pattern examples
    pub examples: Vec<String>,

    /// Suggested category
    pub suggested_category: Option<String>,

    /// Suggested maturity level
    pub suggested_maturity: Option<PatternMaturity>,
}

/// Pattern service for advanced pattern management
pub struct PatternService {
    config: PatternServiceConfig,
    knowledge_engine: Arc<UnifiedKnowledgeEngine>,
    temporal_manager: Arc<TemporalContextManager>,
    usage_analyzer: Arc<UsageAnalyzer>,
    pattern_cache: Arc<RwLock<HashMap<String, PatternEntry>>>,
    usage_patterns: Arc<RwLock<HashMap<String, UsagePattern>>>,
}

impl PatternService {
    /// Create a new pattern service
    pub async fn new(
        config: PatternServiceConfig,
        knowledge_engine: Arc<UnifiedKnowledgeEngine>,
        temporal_manager: Arc<TemporalContextManager>,
        usage_analyzer: Arc<UsageAnalyzer>,
    ) -> RhemaResult<Self> {
        let service = Self {
            config,
            knowledge_engine,
            temporal_manager,
            usage_analyzer,
            pattern_cache: Arc::new(RwLock::new(HashMap::new())),
            usage_patterns: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize pattern cache
        service.initialize_pattern_cache().await?;

        Ok(service)
    }

    /// Initialize pattern cache from knowledge engine
    async fn initialize_pattern_cache(&self) -> RhemaResult<()> {
        // Load patterns from knowledge engine storage
        let mut patterns = self.pattern_cache.write().await;

        // Search for pattern-related content in the knowledge engine
        let pattern_results = self
            .knowledge_engine
            .search_semantic("pattern", 100)
            .await?;

        for result in pattern_results {
            // Try to parse as PatternEntry from content
            if let Ok(pattern_entry) = serde_json::from_str::<PatternEntry>(&result.content) {
                patterns.insert(pattern_entry.id.clone(), pattern_entry);
            }
        }

        // Also load from file-based pattern storage if available
        // Note: We'll use a default scope path for now since get_scope_path doesn't exist
        let scope_path = std::path::PathBuf::from(".");
        if let Ok(file_patterns) =
            rhema_core::fileops::get_pattern_entries(&scope_path, None, None, None)
        {
            for pattern in file_patterns {
                patterns.insert(pattern.id.clone(), pattern);
            }
        }

        info!("Initialized pattern cache with {} patterns", patterns.len());
        Ok(())
    }

    /// Discover patterns from code analysis
    pub async fn discover_patterns(
        &self,
        codebase_path: &Path,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        if !self.config.enable_pattern_discovery {
            return Ok(Vec::new());
        }

        let mut discoveries = Vec::new();

        // Analyze code structure for common patterns
        let structural_patterns = self.analyze_structural_patterns(codebase_path).await?;
        discoveries.extend(structural_patterns);

        // Analyze naming conventions
        let naming_patterns = self.analyze_naming_patterns(codebase_path).await?;
        discoveries.extend(naming_patterns);

        // Analyze architectural patterns
        let architectural_patterns = self.analyze_architectural_patterns(codebase_path).await?;
        discoveries.extend(architectural_patterns);

        // Filter by confidence threshold
        discoveries.retain(|d| d.confidence >= self.config.discovery_confidence_threshold);

        Ok(discoveries)
    }

    /// Analyze structural patterns in code
    async fn analyze_structural_patterns(
        &self,
        codebase_path: &Path,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();

        // Walk through the codebase to find structural patterns
        if let Ok(entries) = std::fs::read_dir(codebase_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        match extension.to_str() {
                            Some("rs") | Some("py") | Some("js") | Some("ts") | Some("java") => {
                                // Analyze file structure
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let file_discoveries =
                                        self.analyze_file_structure(&path, &content).await?;
                                    discoveries.extend(file_discoveries);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(discoveries)
    }

    /// Analyze individual file structure for patterns
    async fn analyze_file_structure(
        &self,
        file_path: &Path,
        content: &str,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Detect function organization patterns
        if let Some(function_pattern) = self.detect_function_organization_pattern(&lines) {
            discoveries.push(function_pattern);
        }

        // Detect class structure patterns
        if let Some(class_pattern) = self.detect_class_structure_pattern(&lines) {
            discoveries.push(class_pattern);
        }

        // Detect module organization patterns
        if let Some(module_pattern) = self.detect_module_organization_pattern(&lines) {
            discoveries.push(module_pattern);
        }

        // Add file path to discoveries
        for discovery in &mut discoveries {
            discovery.source_files.push(file_path.display().to_string());
        }

        Ok(discoveries)
    }

    /// Detect function organization patterns
    fn detect_function_organization_pattern(
        &self,
        lines: &[&str],
    ) -> Option<PatternDiscoveryResult> {
        let mut public_functions = 0;
        let mut private_functions = 0;
        let mut constructors = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub async fn ") {
                public_functions += 1;
            } else if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
                private_functions += 1;
            } else if trimmed.contains("impl") && trimmed.contains("new") {
                constructors += 1;
            }
        }

        if public_functions > 0 || private_functions > 0 {
            let pattern_type = if public_functions > private_functions {
                "Public Interface Pattern"
            } else {
                "Encapsulation Pattern"
            };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Function organization with {} public and {} private functions",
                    public_functions, private_functions
                ),
                pattern_type: "structural".to_string(),
                confidence: 0.8,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Public functions: {}, Private functions: {}",
                    public_functions, private_functions
                )],
                suggested_category: Some("code_organization".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect class structure patterns
    fn detect_class_structure_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut structs = 0;
        let mut enums = 0;
        let mut traits = 0;
        let mut impls = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("struct ") {
                structs += 1;
            } else if trimmed.starts_with("enum ") {
                enums += 1;
            } else if trimmed.starts_with("trait ") {
                traits += 1;
            } else if trimmed.starts_with("impl ") {
                impls += 1;
            }
        }

        if structs > 0 || enums > 0 || traits > 0 {
            let pattern_type = if traits > 0 && impls > 0 {
                "Trait-Based Design Pattern"
            } else if structs > 0 && impls > 0 {
                "Struct-Based Design Pattern"
            } else if enums > 0 {
                "Enum-Based Design Pattern"
            } else {
                "Data Structure Pattern"
            };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Class structure with {} structs, {} enums, {} traits, {} impls",
                    structs, enums, traits, impls
                ),
                pattern_type: "structural".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Structs: {}, Enums: {}, Traits: {}, Impls: {}",
                    structs, enums, traits, impls
                )],
                suggested_category: Some("object_oriented".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect module organization patterns
    fn detect_module_organization_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut modules = 0;
        let mut imports = 0;
        let mut exports = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("mod ") || trimmed.starts_with("pub mod ") {
                modules += 1;
            } else if trimmed.starts_with("use ") || trimmed.starts_with("import ") {
                imports += 1;
            } else if trimmed.starts_with("pub use ") || trimmed.starts_with("export ") {
                exports += 1;
            }
        }

        if modules > 0 || imports > 0 {
            let pattern_type = if modules > 0 {
                "Module Organization Pattern"
            } else if imports > exports {
                "Consumer Module Pattern"
            } else if exports > imports {
                "Provider Module Pattern"
            } else {
                "Balanced Module Pattern"
            };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Module organization with {} modules, {} imports, {} exports",
                    modules, imports, exports
                ),
                pattern_type: "structural".to_string(),
                confidence: 0.6,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Modules: {}, Imports: {}, Exports: {}",
                    modules, imports, exports
                )],
                suggested_category: Some("module_organization".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Analyze naming convention patterns
    async fn analyze_naming_patterns(
        &self,
        codebase_path: &Path,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();

        // Walk through the codebase to find naming patterns
        if let Ok(entries) = std::fs::read_dir(codebase_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        match extension.to_str() {
                            Some("rs") | Some("py") | Some("js") | Some("ts") | Some("java") => {
                                // Analyze file naming patterns
                                if let Some(naming_pattern) =
                                    self.analyze_file_naming_pattern(&path)
                                {
                                    discoveries.push(naming_pattern);
                                }

                                // Analyze content naming patterns
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let content_patterns = self
                                        .analyze_content_naming_patterns(&path, &content)
                                        .await?;
                                    discoveries.extend(content_patterns);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(discoveries)
    }

    /// Analyze file naming patterns
    fn analyze_file_naming_pattern(&self, file_path: &Path) -> Option<PatternDiscoveryResult> {
        if let Some(file_name) = file_path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                let pattern_type = if name_str.contains('_') {
                    "Snake Case File Naming"
                } else if name_str.chars().next().map_or(false, |c| c.is_uppercase()) {
                    "Pascal Case File Naming"
                } else if name_str.contains('-') {
                    "Kebab Case File Naming"
                } else {
                    "Camel Case File Naming"
                };

                return Some(PatternDiscoveryResult {
                    name: pattern_type.to_string(),
                    description: format!("File naming convention: {}", name_str),
                    pattern_type: "naming".to_string(),
                    confidence: 0.9,
                    source_files: vec![file_path.display().to_string()],
                    examples: vec![name_str.to_string()],
                    suggested_category: Some("naming_conventions".to_string()),
                    suggested_maturity: Some(PatternMaturity::Established),
                });
            }
        }
        None
    }

    /// Analyze content naming patterns
    async fn analyze_content_naming_patterns(
        &self,
        file_path: &Path,
        content: &str,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Analyze variable naming patterns
        if let Some(variable_pattern) = self.detect_variable_naming_pattern(&lines) {
            discoveries.push(variable_pattern);
        }

        // Analyze function naming patterns
        if let Some(function_pattern) = self.detect_function_naming_pattern(&lines) {
            discoveries.push(function_pattern);
        }

        // Analyze constant naming patterns
        if let Some(constant_pattern) = self.detect_constant_naming_pattern(&lines) {
            discoveries.push(constant_pattern);
        }

        // Add file path to discoveries
        for discovery in &mut discoveries {
            discovery.source_files.push(file_path.display().to_string());
        }

        Ok(discoveries)
    }

    /// Detect variable naming patterns
    fn detect_variable_naming_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut snake_case_vars = 0;
        let mut camel_case_vars = 0;
        let mut pascal_case_vars = 0;

        for line in lines {
            let trimmed = line.trim();
            // Look for variable declarations
            if trimmed.contains("let ") || trimmed.contains("mut ") || trimmed.contains("const ") {
                // Extract variable names using simple regex-like patterns
                let words: Vec<&str> = trimmed.split_whitespace().collect();
                for word in words {
                    if word.contains('_')
                        && word
                            .chars()
                            .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
                    {
                        snake_case_vars += 1;
                    } else if word.chars().next().map_or(false, |c| c.is_lowercase())
                        && !word.contains('_')
                    {
                        camel_case_vars += 1;
                    } else if word.chars().next().map_or(false, |c| c.is_uppercase())
                        && !word.contains('_')
                    {
                        pascal_case_vars += 1;
                    }
                }
            }
        }

        if snake_case_vars > 0 || camel_case_vars > 0 || pascal_case_vars > 0 {
            let pattern_type =
                if snake_case_vars > camel_case_vars && snake_case_vars > pascal_case_vars {
                    "Snake Case Variable Naming"
                } else if camel_case_vars > snake_case_vars && camel_case_vars > pascal_case_vars {
                    "Camel Case Variable Naming"
                } else if pascal_case_vars > snake_case_vars && pascal_case_vars > camel_case_vars {
                    "Pascal Case Variable Naming"
                } else {
                    "Mixed Variable Naming"
                };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Variable naming with {} snake_case, {} camelCase, {} PascalCase",
                    snake_case_vars, camel_case_vars, pascal_case_vars
                ),
                pattern_type: "naming".to_string(),
                confidence: 0.8,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Snake: {}, Camel: {}, Pascal: {}",
                    snake_case_vars, camel_case_vars, pascal_case_vars
                )],
                suggested_category: Some("naming_conventions".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect function naming patterns
    fn detect_function_naming_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut snake_case_funcs = 0;
        let mut camel_case_funcs = 0;
        let mut pascal_case_funcs = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("fn ")
                || trimmed.starts_with("pub fn ")
                || trimmed.starts_with("async fn ")
            {
                // Extract function names
                if let Some(func_name) = trimmed.split_whitespace().nth(1) {
                    if func_name.contains('_')
                        && func_name
                            .chars()
                            .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
                    {
                        snake_case_funcs += 1;
                    } else if func_name.chars().next().map_or(false, |c| c.is_lowercase())
                        && !func_name.contains('_')
                    {
                        camel_case_funcs += 1;
                    } else if func_name.chars().next().map_or(false, |c| c.is_uppercase())
                        && !func_name.contains('_')
                    {
                        pascal_case_funcs += 1;
                    }
                }
            }
        }

        if snake_case_funcs > 0 || camel_case_funcs > 0 || pascal_case_funcs > 0 {
            let pattern_type = if snake_case_funcs > camel_case_funcs
                && snake_case_funcs > pascal_case_funcs
            {
                "Snake Case Function Naming"
            } else if camel_case_funcs > snake_case_funcs && camel_case_funcs > pascal_case_funcs {
                "Camel Case Function Naming"
            } else if pascal_case_funcs > snake_case_funcs && pascal_case_funcs > camel_case_funcs {
                "Pascal Case Function Naming"
            } else {
                "Mixed Function Naming"
            };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Function naming with {} snake_case, {} camelCase, {} PascalCase",
                    snake_case_funcs, camel_case_funcs, pascal_case_funcs
                ),
                pattern_type: "naming".to_string(),
                confidence: 0.8,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Snake: {}, Camel: {}, Pascal: {}",
                    snake_case_funcs, camel_case_funcs, pascal_case_funcs
                )],
                suggested_category: Some("naming_conventions".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect constant naming patterns
    fn detect_constant_naming_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut upper_snake_case = 0;
        let mut upper_camel_case = 0;
        let mut other_constants = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("const ") || trimmed.starts_with("static ") {
                // Extract constant names
                if let Some(const_name) = trimmed.split_whitespace().nth(1) {
                    if const_name
                        .chars()
                        .all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
                    {
                        upper_snake_case += 1;
                    } else if const_name
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_uppercase())
                        && !const_name.contains('_')
                    {
                        upper_camel_case += 1;
                    } else {
                        other_constants += 1;
                    }
                }
            }
        }

        if upper_snake_case > 0 || upper_camel_case > 0 || other_constants > 0 {
            let pattern_type = if upper_snake_case > upper_camel_case
                && upper_snake_case > other_constants
            {
                "Upper Snake Case Constant Naming"
            } else if upper_camel_case > upper_snake_case && upper_camel_case > other_constants {
                "Upper Camel Case Constant Naming"
            } else {
                "Mixed Constant Naming"
            };

            Some(PatternDiscoveryResult {
                name: pattern_type.to_string(),
                description: format!(
                    "Constant naming with {} UPPER_SNAKE_CASE, {} UpperCamelCase, {} others",
                    upper_snake_case, upper_camel_case, other_constants
                ),
                pattern_type: "naming".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec![format!(
                    "Upper Snake: {}, Upper Camel: {}, Others: {}",
                    upper_snake_case, upper_camel_case, other_constants
                )],
                suggested_category: Some("naming_conventions".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Analyze architectural patterns
    async fn analyze_architectural_patterns(
        &self,
        codebase_path: &Path,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();

        // Walk through the codebase to find architectural patterns
        if let Ok(entries) = std::fs::read_dir(codebase_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        match extension.to_str() {
                            Some("rs") | Some("py") | Some("js") | Some("ts") | Some("java") => {
                                // Analyze file for architectural patterns
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let arch_patterns = self
                                        .analyze_file_architectural_patterns(&path, &content)
                                        .await?;
                                    discoveries.extend(arch_patterns);
                                }
                            }
                            Some("yaml") | Some("yml") | Some("json") | Some("toml") => {
                                // Analyze configuration files for architectural patterns
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let config_patterns = self
                                        .analyze_config_architectural_patterns(&path, &content)
                                        .await?;
                                    discoveries.extend(config_patterns);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Analyze directory structure for architectural patterns
        let dir_patterns = self
            .analyze_directory_structure_patterns(codebase_path)
            .await?;
        discoveries.extend(dir_patterns);

        Ok(discoveries)
    }

    /// Analyze file for architectural patterns
    async fn analyze_file_architectural_patterns(
        &self,
        file_path: &Path,
        content: &str,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Detect design patterns
        if let Some(singleton_pattern) = self.detect_singleton_pattern(&lines) {
            discoveries.push(singleton_pattern);
        }

        if let Some(factory_pattern) = self.detect_factory_pattern(&lines) {
            discoveries.push(factory_pattern);
        }

        if let Some(observer_pattern) = self.detect_observer_pattern(&lines) {
            discoveries.push(observer_pattern);
        }

        if let Some(strategy_pattern) = self.detect_strategy_pattern(&lines) {
            discoveries.push(strategy_pattern);
        }

        if let Some(repository_pattern) = self.detect_repository_pattern(&lines) {
            discoveries.push(repository_pattern);
        }

        // Detect architectural styles
        if let Some(mvc_pattern) = self.detect_mvc_pattern(&lines) {
            discoveries.push(mvc_pattern);
        }

        if let Some(microservice_pattern) = self.detect_microservice_pattern(&lines) {
            discoveries.push(microservice_pattern);
        }

        if let Some(layered_pattern) = self.detect_layered_pattern(&lines) {
            discoveries.push(layered_pattern);
        }

        // Add file path to discoveries
        for discovery in &mut discoveries {
            discovery.source_files.push(file_path.display().to_string());
        }

        Ok(discoveries)
    }

    /// Analyze configuration files for architectural patterns
    async fn analyze_config_architectural_patterns(
        &self,
        file_path: &Path,
        content: &str,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();

        // Detect Docker/Kubernetes patterns
        if file_path.file_name().map_or(false, |name| {
            name.to_str().unwrap_or("").contains("Dockerfile")
        }) {
            discoveries.push(PatternDiscoveryResult {
                name: "Containerization Pattern".to_string(),
                description: "Docker containerization for application deployment".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.9,
                source_files: vec![file_path.display().to_string()],
                examples: vec!["Dockerfile".to_string()],
                suggested_category: Some("deployment".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        // Detect Kubernetes patterns
        if file_path.file_name().map_or(false, |name| {
            name.to_str().unwrap_or("").contains("k8s")
                || name.to_str().unwrap_or("").contains("kubernetes")
        }) {
            discoveries.push(PatternDiscoveryResult {
                name: "Kubernetes Orchestration Pattern".to_string(),
                description: "Kubernetes-based container orchestration".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.9,
                source_files: vec![file_path.display().to_string()],
                examples: vec!["kubernetes deployment".to_string()],
                suggested_category: Some("orchestration".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        // Detect database configuration patterns
        if content.contains("database") || content.contains("db_") || content.contains("connection")
        {
            discoveries.push(PatternDiscoveryResult {
                name: "Database Configuration Pattern".to_string(),
                description: "Database connection and configuration management".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.7,
                source_files: vec![file_path.display().to_string()],
                examples: vec!["Database configuration".to_string()],
                suggested_category: Some("data_persistence".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        Ok(discoveries)
    }

    /// Analyze directory structure for architectural patterns
    async fn analyze_directory_structure_patterns(
        &self,
        codebase_path: &Path,
    ) -> RhemaResult<Vec<PatternDiscoveryResult>> {
        let mut discoveries = Vec::new();

        // Check for common architectural directory patterns
        let mut has_src = false;
        let mut has_tests = false;
        let mut has_docs = false;
        let mut has_config = false;
        let mut has_migrations = false;
        let mut has_controllers = false;
        let mut has_models = false;
        let mut has_services = false;

        if let Ok(entries) = std::fs::read_dir(codebase_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name() {
                        let name = dir_name.to_str().unwrap_or("");
                        match name {
                            "src" => has_src = true,
                            "tests" | "test" => has_tests = true,
                            "docs" | "documentation" => has_docs = true,
                            "config" | "configuration" => has_config = true,
                            "migrations" => has_migrations = true,
                            "controllers" => has_controllers = true,
                            "models" => has_models = true,
                            "services" => has_services = true,
                            _ => {}
                        }
                    }
                }
            }
        }

        // Detect MVC pattern
        if has_controllers && has_models {
            discoveries.push(PatternDiscoveryResult {
                name: "MVC Architecture Pattern".to_string(),
                description: "Model-View-Controller architectural pattern".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.8,
                source_files: vec![codebase_path.display().to_string()],
                examples: vec!["controllers/ and models/ directories".to_string()],
                suggested_category: Some("web_architecture".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        // Detect layered architecture
        if has_services && has_models {
            discoveries.push(PatternDiscoveryResult {
                name: "Layered Architecture Pattern".to_string(),
                description: "Service layer with data models".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.7,
                source_files: vec![codebase_path.display().to_string()],
                examples: vec!["services/ and models/ directories".to_string()],
                suggested_category: Some("service_architecture".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        // Detect standard project structure
        if has_src && has_tests {
            discoveries.push(PatternDiscoveryResult {
                name: "Standard Project Structure".to_string(),
                description: "Standard source and test directory organization".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.9,
                source_files: vec![codebase_path.display().to_string()],
                examples: vec!["src/ and tests/ directories".to_string()],
                suggested_category: Some("project_organization".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            });
        }

        Ok(discoveries)
    }

    /// Detect singleton pattern
    fn detect_singleton_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_static_instance = false;
        let mut has_private_constructor = false;
        let mut has_get_instance = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("static") && trimmed.contains("instance") {
                has_static_instance = true;
            }
            if trimmed.contains("new") && trimmed.contains("private") {
                has_private_constructor = true;
            }
            if trimmed.contains("get_instance") || trimmed.contains("getInstance") {
                has_get_instance = true;
            }
        }

        if has_static_instance && has_get_instance {
            Some(PatternDiscoveryResult {
                name: "Singleton Pattern".to_string(),
                description: "Singleton design pattern with static instance".to_string(),
                pattern_type: "design".to_string(),
                confidence: 0.8,
                source_files: Vec::new(),
                examples: vec!["Static instance with get_instance method".to_string()],
                suggested_category: Some("creational_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect factory pattern
    fn detect_factory_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_factory_method = false;
        let mut has_create_method = false;
        let mut has_factory_class = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("factory") && trimmed.contains("create") {
                has_factory_method = true;
            }
            if trimmed.contains("create_") || trimmed.contains("create") {
                has_create_method = true;
            }
            if trimmed.contains("Factory") {
                has_factory_class = true;
            }
        }

        if (has_factory_method || has_factory_class) && has_create_method {
            Some(PatternDiscoveryResult {
                name: "Factory Pattern".to_string(),
                description: "Factory design pattern for object creation".to_string(),
                pattern_type: "design".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec!["Factory class with create methods".to_string()],
                suggested_category: Some("creational_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect observer pattern
    fn detect_observer_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_subscribe = false;
        let mut has_notify = false;
        let mut has_observer = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("subscribe") || trimmed.contains("add_listener") {
                has_subscribe = true;
            }
            if trimmed.contains("notify") || trimmed.contains("emit") {
                has_notify = true;
            }
            if trimmed.contains("Observer") || trimmed.contains("observer") {
                has_observer = true;
            }
        }

        if has_subscribe && has_notify {
            Some(PatternDiscoveryResult {
                name: "Observer Pattern".to_string(),
                description: "Observer design pattern for event handling".to_string(),
                pattern_type: "design".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec!["Subscribe and notify methods".to_string()],
                suggested_category: Some("behavioral_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect strategy pattern
    fn detect_strategy_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_strategy = false;
        let mut has_execute = false;
        let mut has_algorithm = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("Strategy") || trimmed.contains("strategy") {
                has_strategy = true;
            }
            if trimmed.contains("execute") || trimmed.contains("run") {
                has_execute = true;
            }
            if trimmed.contains("algorithm") || trimmed.contains("Algorithm") {
                has_algorithm = true;
            }
        }

        if has_strategy && has_execute {
            Some(PatternDiscoveryResult {
                name: "Strategy Pattern".to_string(),
                description: "Strategy design pattern for algorithm selection".to_string(),
                pattern_type: "design".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec!["Strategy interface with execute method".to_string()],
                suggested_category: Some("behavioral_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect repository pattern
    fn detect_repository_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_repository = false;
        let mut has_find = false;
        let mut has_save = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("Repository") || trimmed.contains("repository") {
                has_repository = true;
            }
            if trimmed.contains("find_") || trimmed.contains("find") {
                has_find = true;
            }
            if trimmed.contains("save") || trimmed.contains("insert") || trimmed.contains("update")
            {
                has_save = true;
            }
        }

        if has_repository && (has_find || has_save) {
            Some(PatternDiscoveryResult {
                name: "Repository Pattern".to_string(),
                description: "Repository pattern for data access abstraction".to_string(),
                pattern_type: "design".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec!["Repository class with data access methods".to_string()],
                suggested_category: Some("data_access_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect MVC pattern
    fn detect_mvc_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_controller = false;
        let mut has_model = false;
        let mut has_view = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("Controller") || trimmed.contains("controller") {
                has_controller = true;
            }
            if trimmed.contains("Model") || trimmed.contains("model") {
                has_model = true;
            }
            if trimmed.contains("View") || trimmed.contains("view") {
                has_view = true;
            }
        }

        if has_controller && has_model {
            Some(PatternDiscoveryResult {
                name: "MVC Pattern".to_string(),
                description: "Model-View-Controller architectural pattern".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.8,
                source_files: Vec::new(),
                examples: vec!["Controller and Model classes".to_string()],
                suggested_category: Some("web_architecture".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect microservice pattern
    fn detect_microservice_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_service = false;
        let mut has_api = false;
        let mut has_endpoint = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("Service") || trimmed.contains("service") {
                has_service = true;
            }
            if trimmed.contains("api") || trimmed.contains("API") {
                has_api = true;
            }
            if trimmed.contains("endpoint") || trimmed.contains("route") {
                has_endpoint = true;
            }
        }

        if has_service && (has_api || has_endpoint) {
            Some(PatternDiscoveryResult {
                name: "Microservice Pattern".to_string(),
                description: "Microservice architectural pattern".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.6,
                source_files: Vec::new(),
                examples: vec!["Service with API endpoints".to_string()],
                suggested_category: Some("service_architecture".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Detect layered pattern
    fn detect_layered_pattern(&self, lines: &[&str]) -> Option<PatternDiscoveryResult> {
        let mut has_layer = false;
        let mut has_tier = false;
        let mut has_presentation = false;
        let mut has_business = false;
        let mut has_data = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains("layer") || trimmed.contains("Layer") {
                has_layer = true;
            }
            if trimmed.contains("tier") || trimmed.contains("Tier") {
                has_tier = true;
            }
            if trimmed.contains("presentation") || trimmed.contains("Presentation") {
                has_presentation = true;
            }
            if trimmed.contains("business") || trimmed.contains("Business") {
                has_business = true;
            }
            if trimmed.contains("data") || trimmed.contains("Data") {
                has_data = true;
            }
        }

        if has_layer || has_tier || (has_presentation && has_business) {
            Some(PatternDiscoveryResult {
                name: "Layered Architecture Pattern".to_string(),
                description: "Layered architectural pattern".to_string(),
                pattern_type: "architectural".to_string(),
                confidence: 0.7,
                source_files: Vec::new(),
                examples: vec!["Layered architecture with separation of concerns".to_string()],
                suggested_category: Some("architecture_patterns".to_string()),
                suggested_maturity: Some(PatternMaturity::Established),
            })
        } else {
            None
        }
    }

    /// Get pattern recommendations for a context
    pub async fn get_recommendations(
        &self,
        context: &str,
        current_patterns: &[String],
        max_recommendations: Option<usize>,
    ) -> RhemaResult<Vec<PatternRecommendation>> {
        if !self.config.enable_recommendations {
            return Ok(Vec::new());
        }

        let max_recs = max_recommendations.unwrap_or(self.config.max_recommendations);
        let mut recommendations = Vec::new();

        // Get all available patterns
        let patterns = self.get_all_patterns().await?;

        for pattern in patterns {
            // Skip if already using this pattern
            if current_patterns.contains(&pattern.id) {
                continue;
            }

            // Calculate recommendation score
            let score = self
                .calculate_recommendation_score(&pattern, context)
                .await?;

            if score > 0.5 {
                let recommendation = PatternRecommendation {
                    pattern_id: pattern.id.clone(),
                    pattern_name: pattern.name.clone(),
                    score,
                    reason: self
                        .generate_recommendation_reason(&pattern, context)
                        .await?,
                    context: context.to_string(),
                    benefits: self.extract_benefits(&pattern).await?,
                    effort: self.estimate_implementation_effort(&pattern).await?,
                };

                recommendations.push(recommendation);
            }
        }

        // Sort by score and limit results
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        recommendations.truncate(max_recs);

        Ok(recommendations)
    }

    /// Calculate recommendation score for a pattern in a context
    async fn calculate_recommendation_score(
        &self,
        pattern: &PatternEntry,
        context: &str,
    ) -> RhemaResult<f32> {
        let mut score = 0.0;

        // Base score from effectiveness
        if let Some(effectiveness) = pattern.effectiveness {
            score += (effectiveness as f32 / 10.0) * 0.3;
        }

        // Context relevance
        if let Some(contexts) = &pattern.applicable_contexts {
            if contexts
                .iter()
                .any(|c| context.to_lowercase().contains(&c.to_lowercase()))
            {
                score += 0.3;
            }
        }

        // Maturity bonus
        if let Some(maturity) = &pattern.maturity {
            score += match maturity {
                PatternMaturity::Mature => 0.2,
                PatternMaturity::Established => 0.15,
                PatternMaturity::Emerging => 0.1,
                PatternMaturity::Experimental => 0.05,
                PatternMaturity::Legacy => 0.0,
            };
        }

        // Usage statistics bonus
        if let Some(stats) = &pattern.usage_stats {
            score += stats.success_rate * 0.2;
        }

        Ok(score.min(1.0))
    }

    /// Generate recommendation reason
    async fn generate_recommendation_reason(
        &self,
        pattern: &PatternEntry,
        context: &str,
    ) -> RhemaResult<String> {
        let mut reasons = Vec::new();

        if let Some(effectiveness) = pattern.effectiveness {
            if effectiveness >= 8 {
                reasons.push("High effectiveness rating".to_string());
            }
        }

        if let Some(maturity) = &pattern.maturity {
            if matches!(
                maturity,
                PatternMaturity::Mature | PatternMaturity::Established
            ) {
                reasons.push("Proven and established pattern".to_string());
            }
        }

        if let Some(stats) = &pattern.usage_stats {
            if stats.success_rate > 0.8 {
                reasons.push("High success rate in similar contexts".to_string());
            }
        }

        if reasons.is_empty() {
            reasons.push("Good fit for current context".to_string());
        }

        Ok(reasons.join(", "))
    }

    /// Extract benefits from pattern
    async fn extract_benefits(&self, pattern: &PatternEntry) -> RhemaResult<Vec<String>> {
        let mut benefits = Vec::new();

        // Add benefits based on pattern type
        match pattern.pattern_type.to_lowercase().as_str() {
            "performance" => benefits.push("Improved performance".to_string()),
            "security" => benefits.push("Enhanced security".to_string()),
            "maintainability" => benefits.push("Better maintainability".to_string()),
            "scalability" => benefits.push("Improved scalability".to_string()),
            "testability" => benefits.push("Better testability".to_string()),
            _ => benefits.push("Improved code quality".to_string()),
        }

        // Add benefits from effectiveness
        if let Some(effectiveness) = pattern.effectiveness {
            if effectiveness >= 8 {
                benefits.push("Proven effectiveness".to_string());
            }
        }

        Ok(benefits)
    }

    /// Estimate implementation effort
    async fn estimate_implementation_effort(
        &self,
        pattern: &PatternEntry,
    ) -> RhemaResult<ImplementationEffort> {
        let mut effort_score = 0;

        // Complexity factor
        if let Some(complexity) = pattern.complexity {
            effort_score += complexity as i32;
        }

        // Maintenance effort factor
        if let Some(maintenance) = pattern.maintenance_effort {
            effort_score += maintenance as i32;
        }

        // Dependencies factor
        if let Some(dependencies) = &pattern.dependencies {
            effort_score += dependencies.len() as i32 * 2;
        }

        Ok(match effort_score {
            0..=10 => ImplementationEffort::Low,
            11..=20 => ImplementationEffort::Medium,
            21..=30 => ImplementationEffort::High,
            _ => ImplementationEffort::VeryHigh,
        })
    }

    /// Track pattern usage
    pub async fn track_pattern_usage(
        &self,
        pattern_id: &str,
        context: &str,
        success: bool,
        implementation_time: Option<f32>,
    ) -> RhemaResult<()> {
        if !self.config.enable_usage_tracking {
            return Ok(());
        }

        // Update usage statistics
        let mut patterns = self.pattern_cache.write().await;
        if let Some(pattern) = patterns.get_mut(pattern_id) {
            if let Some(stats) = &mut pattern.usage_stats {
                stats.usage_count += 1;
                if success {
                    stats.success_count += 1;
                }
                stats.success_rate = stats.success_count as f32 / stats.usage_count as f32;
                stats.last_used = Some(chrono::Utc::now());

                if let Some(time) = implementation_time {
                    if let Some(avg_time) = stats.avg_implementation_time {
                        stats.avg_implementation_time = Some((avg_time + time) / 2.0);
                    } else {
                        stats.avg_implementation_time = Some(time);
                    }
                }
            }
        }

        // Store usage pattern
        let usage_pattern = UsagePattern {
            pattern_id: pattern_id.to_string(),
            agent_id: "pattern_service".to_string(),
            workflow_type: WorkflowType::CodeReview,
            context_keys: vec![context.to_string()],
            frequency: 1,
            last_used: chrono::Utc::now(),
            confidence: 1.0,
            success_rate: if success { 1.0 } else { 0.0 },
        };

        let mut usage_patterns = self.usage_patterns.write().await;
        usage_patterns.insert(format!("{}_{}", pattern_id, context), usage_pattern);

        Ok(())
    }

    /// Calculate pattern quality metrics
    pub async fn calculate_quality_metrics(
        &self,
        pattern: &PatternEntry,
    ) -> RhemaResult<PatternQualityMetrics> {
        if !self.config.enable_quality_scoring {
            return Ok(PatternQualityMetrics {
                overall_score: 0.5,
                documentation_score: 0.5,
                example_score: 0.5,
                usage_score: 0.5,
                review_score: 0.5,
                suggestions: Vec::new(),
            });
        }

        let mut suggestions = Vec::new();
        let mut scores = Vec::new();

        // Documentation quality
        let doc_score = self.calculate_documentation_score(pattern, &mut suggestions);
        scores.push(doc_score);

        // Example quality
        let example_score = self.calculate_example_score(pattern, &mut suggestions);
        scores.push(example_score);

        // Usage tracking quality
        let usage_score = self.calculate_usage_score(pattern, &mut suggestions);
        scores.push(usage_score);

        // Review quality
        let review_score = self.calculate_review_score(pattern, &mut suggestions);
        scores.push(review_score);

        let overall_score = scores.iter().sum::<f32>() / scores.len() as f32;

        Ok(PatternQualityMetrics {
            overall_score,
            documentation_score: doc_score,
            example_score,
            usage_score,
            review_score,
            suggestions,
        })
    }

    /// Calculate documentation quality score
    fn calculate_documentation_score(
        &self,
        pattern: &PatternEntry,
        suggestions: &mut Vec<String>,
    ) -> f32 {
        let mut score: f32 = 0.0;

        // Description quality
        if pattern.description.len() > 50 {
            score += 0.3;
        } else {
            suggestions.push("Add more detailed description".to_string());
        }

        // When to use/avoid
        if pattern.description.contains("when") || pattern.description.contains("use") {
            score += 0.2;
        } else {
            suggestions.push("Add guidance on when to use this pattern".to_string());
        }

        // Benefits listed
        if pattern.description.contains("benefit") || pattern.description.contains("improve") {
            score += 0.2;
        }

        // Implementation guidance
        if pattern.description.contains("implement") || pattern.description.contains("how") {
            score += 0.3;
        } else {
            suggestions.push("Add implementation guidance".to_string());
        }

        score
    }

    /// Calculate example quality score
    fn calculate_example_score(
        &self,
        pattern: &PatternEntry,
        suggestions: &mut Vec<String>,
    ) -> f32 {
        let mut score: f32 = 0.0;

        if let Some(examples) = &pattern.examples {
            if !examples.is_empty() {
                score += 0.5;

                // Check example quality
                for example in examples {
                    if example.len() > 20 {
                        score += 0.1;
                    }
                }
            } else {
                suggestions.push("Add implementation examples".to_string());
            }
        } else {
            suggestions.push("Add implementation examples".to_string());
        }

        // Anti-patterns
        if let Some(anti_patterns) = &pattern.anti_patterns {
            if !anti_patterns.is_empty() {
                score += 0.3;
            }
        } else {
            suggestions.push("Add anti-patterns to avoid".to_string());
        }

        score.min(1.0)
    }

    /// Calculate usage tracking quality score
    fn calculate_usage_score(&self, pattern: &PatternEntry, suggestions: &mut Vec<String>) -> f32 {
        let mut score: f32 = 0.0;

        if let Some(stats) = &pattern.usage_stats {
            if stats.usage_count > 0 {
                score += 0.4;

                if stats.success_rate > 0.8 {
                    score += 0.3;
                } else if stats.success_rate < 0.5 {
                    suggestions.push("Low success rate - consider improving pattern".to_string());
                }
            } else {
                suggestions.push("Track pattern usage to improve quality".to_string());
            }
        } else {
            suggestions.push("Add usage statistics tracking".to_string());
        }

        score
    }

    /// Calculate review quality score
    fn calculate_review_score(&self, pattern: &PatternEntry, suggestions: &mut Vec<String>) -> f32 {
        let mut score: f32 = 0.0;

        if let Some(review_status) = &pattern.review_status {
            match review_status {
                ReviewStatus::Approved => score += 1.0,
                ReviewStatus::InReview => score += 0.5,
                ReviewStatus::Pending => {
                    score += 0.2;
                    suggestions.push("Complete pattern review".to_string());
                }
                ReviewStatus::Rejected => {
                    suggestions.push("Address review feedback".to_string());
                }
                ReviewStatus::NeedsRevision => {
                    score += 0.3;
                    suggestions.push("Address review feedback".to_string());
                }
            }
        } else {
            suggestions.push("Submit pattern for review".to_string());
        }

        if let Some(reviewers) = &pattern.reviewers {
            if !reviewers.is_empty() {
                score += 0.2;
            }
        }

        score.min(1.0)
    }

    /// Get all patterns
    async fn get_all_patterns(&self) -> RhemaResult<Vec<PatternEntry>> {
        let mut all_patterns = Vec::new();

        // Get patterns from cache
        let cache_patterns = self.pattern_cache.read().await;
        all_patterns.extend(cache_patterns.values().cloned());

        // Get patterns from file-based storage if available
        // Note: We'll use a default scope path for now since get_scope_path doesn't exist
        let scope_path = std::path::PathBuf::from(".");
        if let Ok(file_patterns) =
            rhema_core::fileops::get_pattern_entries(&scope_path, None, None, None)
        {
            // Add file patterns that aren't already in cache
            let cache_ids: std::collections::HashSet<String> =
                cache_patterns.keys().cloned().collect();
            for pattern in file_patterns {
                if !cache_ids.contains(&pattern.id) {
                    all_patterns.push(pattern);
                }
            }
        }

        // Get patterns from knowledge engine storage
        let pattern_results = self
            .knowledge_engine
            .search_semantic("pattern", 100)
            .await?;
        for result in pattern_results {
            if let Ok(pattern_entry) = serde_json::from_str::<PatternEntry>(&result.content) {
                // Check if pattern is already included
                if !all_patterns.iter().any(|p| p.id == pattern_entry.id) {
                    all_patterns.push(pattern_entry);
                }
            }
        }

        Ok(all_patterns)
    }

    /// Get scope path from knowledge engine
    async fn get_scope_path(&self) -> RhemaResult<Option<std::path::PathBuf>> {
        // Try to get scope path from knowledge engine configuration
        // This is a simplified implementation - in practice, you'd get this from the engine config
        Ok(None)
    }

    /// Get pattern by ID
    pub async fn get_pattern(&self, pattern_id: &str) -> RhemaResult<Option<PatternEntry>> {
        let patterns = self.pattern_cache.read().await;
        Ok(patterns.get(pattern_id).cloned())
    }

    /// Add pattern to service
    pub async fn add_pattern(&self, pattern: PatternEntry) -> RhemaResult<()> {
        // Add to cache
        let mut patterns = self.pattern_cache.write().await;
        patterns.insert(pattern.id.clone(), pattern.clone());

        // Store in knowledge engine for persistence
        let pattern_data = serde_json::to_vec(&pattern)?;
        self.knowledge_engine
            .set_with_semantic_indexing(&format!("pattern:{}", pattern.id), &pattern_data, &None)
            .await?;

        // Store in file-based storage if scope path is available
        if let Some(scope_path) = self.get_scope_path().await? {
            let _ = rhema_core::fileops::add_enhanced_pattern_entry(
                &scope_path,
                pattern.name.clone(),
                pattern.description.clone(),
                pattern.pattern_type.clone(),
                pattern.usage.clone(),
                pattern.effectiveness,
                pattern.examples.as_ref().map(|e| e.join(", ")),
                pattern.anti_patterns.as_ref().map(|a| a.join(", ")),
                pattern.category.clone(),
                pattern.maturity.clone(),
                pattern.complexity,
                pattern.maintenance_effort,
                pattern.performance_impact,
                pattern.security_implications.as_ref().map(|s| s.join(", ")),
                pattern.testing_requirements.as_ref().map(|t| t.join(", ")),
                pattern
                    .documentation_requirements
                    .as_ref()
                    .map(|d| d.join(", ")),
                pattern.dependencies.as_ref().map(|d| d.join(", ")),
                pattern.applicable_contexts.as_ref().map(|c| c.join(", ")),
                pattern.version.clone(),
                pattern.author.clone(),
                pattern.review_status.clone(),
            );
        }

        Ok(())
    }

    /// Update pattern in service
    pub async fn update_pattern(&self, pattern: PatternEntry) -> RhemaResult<()> {
        // Update cache
        let mut patterns = self.pattern_cache.write().await;
        patterns.insert(pattern.id.clone(), pattern.clone());

        // Update in knowledge engine
        let pattern_data = serde_json::to_vec(&pattern)?;
        self.knowledge_engine
            .set_with_semantic_indexing(&format!("pattern:{}", pattern.id), &pattern_data, &None)
            .await?;

        // Update in file-based storage if scope path is available
        if let Some(scope_path) = self.get_scope_path().await? {
            let _ = rhema_core::fileops::update_enhanced_pattern_entry(
                &scope_path,
                &pattern.id,
                Some(pattern.name.clone()),
                Some(pattern.description.clone()),
                Some(pattern.pattern_type.clone()),
                Some(pattern.usage.clone()),
                pattern.effectiveness,
                pattern.examples.as_ref().map(|e| e.join(", ")),
                pattern.anti_patterns.as_ref().map(|a| a.join(", ")),
                pattern.category.clone(),
                pattern.maturity.clone(),
                pattern.complexity,
                pattern.maintenance_effort,
                pattern.performance_impact,
                pattern.security_implications.as_ref().map(|s| s.join(", ")),
                pattern.testing_requirements.as_ref().map(|t| t.join(", ")),
                pattern
                    .documentation_requirements
                    .as_ref()
                    .map(|d| d.join(", ")),
                pattern.dependencies.as_ref().map(|d| d.join(", ")),
                pattern.applicable_contexts.as_ref().map(|c| c.join(", ")),
                pattern.version.clone(),
                pattern.author.clone(),
                pattern.review_status.clone(),
            );
        }

        Ok(())
    }

    /// Remove pattern from service
    pub async fn remove_pattern(&self, pattern_id: &str) -> RhemaResult<()> {
        // Remove from cache
        let mut patterns = self.pattern_cache.write().await;
        patterns.remove(pattern_id);

        // Remove from knowledge engine
        // Note: Knowledge engine doesn't have a direct delete method, so we'll just update the cache

        // Remove from file-based storage if scope path is available
        if let Some(scope_path) = self.get_scope_path().await? {
            let _ = rhema_core::fileops::delete_pattern_entry(&scope_path, pattern_id);
        }

        Ok(())
    }
}
