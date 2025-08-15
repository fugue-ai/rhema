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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// Architectural patterns that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchitecturalPattern {
    /// Monorepo structure
    Monorepo,
    /// Microservices architecture
    Microservices,
    /// Layered architecture
    Layered,
    /// Event-driven architecture
    EventDriven,
    /// Domain-driven design
    DomainDriven,
    /// Clean architecture
    CleanArchitecture,
    /// Hexagonal architecture
    Hexagonal,
    /// CQRS (Command Query Responsibility Segregation)
    CQRS,
    /// Event sourcing
    EventSourcing,
    /// API-first design
    ApiFirst,
    /// Serverless architecture
    Serverless,
    /// Container-based architecture
    Containerized,
    /// Plugin-based architecture
    PluginBased,
    /// Modular monolith
    ModularMonolith,
    /// Custom pattern
    Custom(String),
}

/// Project structure patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectStructurePattern {
    /// Standard source layout (src/, lib/, bin/)
    StandardSourceLayout,
    /// Library layout
    LibraryLayout,
    /// Binary layout
    BinaryLayout,
    /// Test layout
    TestLayout,
    /// Documentation layout
    DocumentationLayout,
    /// Example layout
    ExampleLayout,
    /// Benchmark layout
    BenchmarkLayout,
    /// Configuration layout
    ConfigurationLayout,
    /// Scripts layout
    ScriptsLayout,
    /// Assets layout
    AssetsLayout,
    /// Custom layout
    Custom(String),
}

/// Technology stack patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TechnologyStackPattern {
    /// Rust ecosystem
    RustEcosystem,
    /// Node.js ecosystem
    NodeJSEcosystem,
    /// Python ecosystem
    PythonEcosystem,
    /// Java ecosystem
    JavaEcosystem,
    /// Go ecosystem
    GoEcosystem,
    /// .NET ecosystem
    DotNetEcosystem,
    /// PHP ecosystem
    PhpEcosystem,
    /// Ruby ecosystem
    RubyEcosystem,
    /// Elixir ecosystem
    ElixirEcosystem,
    /// Clojure ecosystem
    ClojureEcosystem,
    /// Haskell ecosystem
    HaskellEcosystem,
    /// Scala ecosystem
    ScalaEcosystem,
    /// Custom stack
    Custom(String),
}

/// Build system patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildSystemPattern {
    /// Cargo (Rust)
    Cargo,
    /// npm/yarn (Node.js)
    NpmYarn,
    /// pip/poetry (Python)
    PipPoetry,
    /// Maven/Gradle (Java)
    MavenGradle,
    /// go modules
    GoModules,
    /// dotnet (C#)
    DotNet,
    /// Composer (PHP)
    Composer,
    /// Bundler (Ruby)
    Bundler,
    /// Mix (Elixir)
    Mix,
    /// Leiningen (Clojure)
    Leiningen,
    /// Stack (Haskell)
    Stack,
    /// SBT (Scala)
    Sbt,
    /// Make
    Make,
    /// CMake
    CMake,
    /// Custom build system
    Custom(String),
}

/// Detected pattern with confidence score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub pattern_type: PatternType,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PatternType {
    Architectural(ArchitecturalPattern),
    ProjectStructure(ProjectStructurePattern),
    TechnologyStack(TechnologyStackPattern),
    BuildSystem(BuildSystemPattern),
}

/// Pattern recognition engine
pub struct PatternRecognitionEngine {
    /// Pattern detection rules
    detection_rules: HashMap<PatternType, Vec<DetectionRule>>,
    /// Pattern weights for confidence calculation
    pattern_weights: HashMap<PatternType, f64>,
    /// Historical pattern data
    historical_patterns: Vec<HistoricalPattern>,
}

/// Detection rule for a pattern
#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub name: String,
    pub file_patterns: Vec<String>,
    pub directory_patterns: Vec<String>,
    pub content_patterns: Vec<String>,
    pub weight: f64,
    pub description: String,
}

/// Historical pattern data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalPattern {
    pub pattern_type: PatternType,
    pub confidence: f64,
    pub success_rate: f64,
    pub usage_count: usize,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

impl PatternRecognitionEngine {
    /// Create a new pattern recognition engine
    pub fn new() -> Self {
        let mut engine = Self {
            detection_rules: HashMap::new(),
            pattern_weights: HashMap::new(),
            historical_patterns: Vec::new(),
        };

        // Initialize detection rules
        engine.initialize_detection_rules();
        engine.initialize_pattern_weights();

        engine
    }

    /// Initialize detection rules for all pattern types
    fn initialize_detection_rules(&mut self) {
        // Architectural patterns
        self.add_architectural_patterns();

        // Project structure patterns
        self.add_project_structure_patterns();

        // Technology stack patterns
        self.add_technology_stack_patterns();

        // Build system patterns
        self.add_build_system_patterns();
    }

    /// Add architectural pattern detection rules
    fn add_architectural_patterns(&mut self) {
        // Monorepo pattern
        let monorepo_rules = vec![DetectionRule {
            name: "workspace_config".to_string(),
            file_patterns: vec![
                "Cargo.toml".to_string(),
                "package.json".to_string(),
                "lerna.json".to_string(),
                "nx.json".to_string(),
            ],
            directory_patterns: vec![
                "packages/".to_string(),
                "crates/".to_string(),
                "apps/".to_string(),
                "libs/".to_string(),
            ],
            content_patterns: vec![
                r#"\[workspace\]"#.to_string(),
                r#""workspaces""#.to_string(),
                r#""projects""#.to_string(),
            ],
            weight: 0.8,
            description: "Detected workspace configuration".to_string(),
        }];

        self.detection_rules.insert(
            PatternType::Architectural(ArchitecturalPattern::Monorepo),
            monorepo_rules,
        );

        // Microservices pattern
        let microservices_rules = vec![DetectionRule {
            name: "service_structure".to_string(),
            file_patterns: vec![
                "docker-compose.yml".to_string(),
                "docker-compose.yaml".to_string(),
                "kubernetes/".to_string(),
                "helm/".to_string(),
            ],
            directory_patterns: vec![
                "services/".to_string(),
                "microservices/".to_string(),
                "api/".to_string(),
            ],
            content_patterns: vec![
                r#"service:"#.to_string(),
                r#"api:"#.to_string(),
                r#"microservice"#.to_string(),
            ],
            weight: 0.7,
            description: "Detected microservices structure".to_string(),
        }];

        self.detection_rules.insert(
            PatternType::Architectural(ArchitecturalPattern::Microservices),
            microservices_rules,
        );

        // Clean Architecture pattern
        let clean_arch_rules = vec![DetectionRule {
            name: "clean_architecture_layers".to_string(),
            file_patterns: vec![],
            directory_patterns: vec![
                "domain/".to_string(),
                "application/".to_string(),
                "infrastructure/".to_string(),
                "interfaces/".to_string(),
            ],
            content_patterns: vec![
                r#"domain"#.to_string(),
                r#"application"#.to_string(),
                r#"infrastructure"#.to_string(),
            ],
            weight: 0.9,
            description: "Detected clean architecture layers".to_string(),
        }];

        self.detection_rules.insert(
            PatternType::Architectural(ArchitecturalPattern::CleanArchitecture),
            clean_arch_rules,
        );
    }

    /// Add project structure pattern detection rules
    fn add_project_structure_patterns(&mut self) {
        // Standard source layout
        let standard_layout_rules = vec![
            DetectionRule {
                name: "src_directory".to_string(),
                file_patterns: vec![],
                directory_patterns: vec!["src/".to_string()],
                content_patterns: vec![],
                weight: 0.6,
                description: "Detected src/ directory".to_string(),
            },
            DetectionRule {
                name: "lib_directory".to_string(),
                file_patterns: vec![],
                directory_patterns: vec!["lib/".to_string()],
                content_patterns: vec![],
                weight: 0.6,
                description: "Detected lib/ directory".to_string(),
            },
        ];

        self.detection_rules.insert(
            PatternType::ProjectStructure(ProjectStructurePattern::StandardSourceLayout),
            standard_layout_rules,
        );

        // Test layout
        let test_layout_rules = vec![
            DetectionRule {
                name: "tests_directory".to_string(),
                file_patterns: vec![],
                directory_patterns: vec!["tests/".to_string()],
                content_patterns: vec![],
                weight: 0.7,
                description: "Detected tests/ directory".to_string(),
            },
            DetectionRule {
                name: "test_files".to_string(),
                file_patterns: vec![
                    "*_test.rs".to_string(),
                    "*.test.js".to_string(),
                    "*.test.ts".to_string(),
                    "test_*.py".to_string(),
                ],
                directory_patterns: vec![],
                content_patterns: vec![
                    r#"#\[test\]"#.to_string(),
                    r#"describe\("#.to_string(),
                    r#"def test_"#.to_string(),
                ],
                weight: 0.8,
                description: "Detected test files".to_string(),
            },
        ];

        self.detection_rules.insert(
            PatternType::ProjectStructure(ProjectStructurePattern::TestLayout),
            test_layout_rules,
        );
    }

    /// Add technology stack pattern detection rules
    fn add_technology_stack_patterns(&mut self) {
        // Rust ecosystem
        let rust_rules = vec![
            DetectionRule {
                name: "cargo_toml".to_string(),
                file_patterns: vec!["Cargo.toml".to_string()],
                directory_patterns: vec![],
                content_patterns: vec![r#"\[package\]"#.to_string()],
                weight: 0.9,
                description: "Detected Cargo.toml".to_string(),
            },
            DetectionRule {
                name: "rust_files".to_string(),
                file_patterns: vec!["*.rs".to_string()],
                directory_patterns: vec![],
                content_patterns: vec![r#"fn "#.to_string(), r#"use "#.to_string()],
                weight: 0.8,
                description: "Detected Rust source files".to_string(),
            },
        ];

        self.detection_rules.insert(
            PatternType::TechnologyStack(TechnologyStackPattern::RustEcosystem),
            rust_rules,
        );

        // Node.js ecosystem
        let nodejs_rules = vec![
            DetectionRule {
                name: "package_json".to_string(),
                file_patterns: vec!["package.json".to_string()],
                directory_patterns: vec![],
                content_patterns: vec![r#""name""#.to_string(), r#""dependencies""#.to_string()],
                weight: 0.9,
                description: "Detected package.json".to_string(),
            },
            DetectionRule {
                name: "node_modules".to_string(),
                file_patterns: vec![],
                directory_patterns: vec!["node_modules/".to_string()],
                content_patterns: vec![],
                weight: 0.7,
                description: "Detected node_modules/ directory".to_string(),
            },
        ];

        self.detection_rules.insert(
            PatternType::TechnologyStack(TechnologyStackPattern::NodeJSEcosystem),
            nodejs_rules,
        );
    }

    /// Add build system pattern detection rules
    fn add_build_system_patterns(&mut self) {
        // Cargo build system
        let cargo_rules = vec![DetectionRule {
            name: "cargo_build".to_string(),
            file_patterns: vec!["Cargo.toml".to_string(), "Cargo.lock".to_string()],
            directory_patterns: vec![],
            content_patterns: vec![r#"\[dependencies\]"#.to_string()],
            weight: 0.9,
            description: "Detected Cargo build system".to_string(),
        }];

        self.detection_rules.insert(
            PatternType::BuildSystem(BuildSystemPattern::Cargo),
            cargo_rules,
        );

        // npm/yarn build system
        let npm_rules = vec![DetectionRule {
            name: "npm_build".to_string(),
            file_patterns: vec![
                "package.json".to_string(),
                "package-lock.json".to_string(),
                "yarn.lock".to_string(),
            ],
            directory_patterns: vec![],
            content_patterns: vec![r#""scripts""#.to_string()],
            weight: 0.9,
            description: "Detected npm/yarn build system".to_string(),
        }];

        self.detection_rules.insert(
            PatternType::BuildSystem(BuildSystemPattern::NpmYarn),
            npm_rules,
        );
    }

    /// Initialize pattern weights
    fn initialize_pattern_weights(&mut self) {
        // Architectural patterns have high weight
        self.pattern_weights.insert(
            PatternType::Architectural(ArchitecturalPattern::Monorepo),
            0.9,
        );
        self.pattern_weights.insert(
            PatternType::Architectural(ArchitecturalPattern::Microservices),
            0.8,
        );
        self.pattern_weights.insert(
            PatternType::Architectural(ArchitecturalPattern::CleanArchitecture),
            0.85,
        );

        // Technology stack patterns have medium-high weight
        self.pattern_weights.insert(
            PatternType::TechnologyStack(TechnologyStackPattern::RustEcosystem),
            0.8,
        );
        self.pattern_weights.insert(
            PatternType::TechnologyStack(TechnologyStackPattern::NodeJSEcosystem),
            0.8,
        );

        // Build system patterns have medium weight
        self.pattern_weights
            .insert(PatternType::BuildSystem(BuildSystemPattern::Cargo), 0.7);
        self.pattern_weights
            .insert(PatternType::BuildSystem(BuildSystemPattern::NpmYarn), 0.7);

        // Project structure patterns have medium weight
        self.pattern_weights.insert(
            PatternType::ProjectStructure(ProjectStructurePattern::StandardSourceLayout),
            0.6,
        );
        self.pattern_weights.insert(
            PatternType::ProjectStructure(ProjectStructurePattern::TestLayout),
            0.6,
        );
    }

    /// Detect patterns in a project directory
    pub fn detect_patterns(
        &self,
        path: &Path,
    ) -> Result<Vec<DetectedPattern>, Box<dyn std::error::Error>> {
        let mut detected_patterns = Vec::new();

        for (pattern_type, rules) in &self.detection_rules {
            let pattern_confidence = self.calculate_pattern_confidence(path, rules)?;

            if pattern_confidence > 0.3 {
                let evidence = self.collect_evidence(path, rules)?;
                let metadata = self.extract_metadata(path, pattern_type)?;

                detected_patterns.push(DetectedPattern {
                    pattern_type: pattern_type.clone(),
                    confidence: pattern_confidence,
                    evidence,
                    metadata,
                });
            }
        }

        // Sort by confidence (highest first)
        detected_patterns.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(detected_patterns)
    }

    /// Calculate confidence for a specific pattern
    fn calculate_pattern_confidence(
        &self,
        path: &Path,
        rules: &[DetectionRule],
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut total_confidence = 0.0;
        let mut total_weight = 0.0;

        for rule in rules {
            let rule_confidence = self.evaluate_rule(path, rule)?;
            total_confidence += rule_confidence * rule.weight;
            total_weight += rule.weight;
        }

        if total_weight > 0.0 {
            Ok(total_confidence / total_weight)
        } else {
            Ok(0.0)
        }
    }

    /// Evaluate a single detection rule
    fn evaluate_rule(
        &self,
        path: &Path,
        rule: &DetectionRule,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut confidence: f64 = 0.0;
        let mut evidence_count = 0;

        // Check file patterns
        for file_pattern in &rule.file_patterns {
            if self.matches_file_pattern(path, file_pattern)? {
                confidence += 0.3;
                evidence_count += 1;
            }
        }

        // Check directory patterns
        for dir_pattern in &rule.directory_patterns {
            if self.matches_directory_pattern(path, dir_pattern)? {
                confidence += 0.4;
                evidence_count += 1;
            }
        }

        // Check content patterns
        for content_pattern in &rule.content_patterns {
            if self.matches_content_pattern(path, content_pattern)? {
                confidence += 0.3;
                evidence_count += 1;
            }
        }

        // Normalize confidence based on evidence count
        if evidence_count > 0 {
            confidence = confidence.min(1.0);
        }

        Ok(confidence)
    }

    /// Check if a file pattern matches
    fn matches_file_pattern(
        &self,
        path: &Path,
        pattern: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for entry in WalkDir::new(path)
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(file_name) = entry_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();

                    // Simple glob-like pattern matching
                    if self.matches_glob_pattern(&file_name_str, pattern) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Check if a directory pattern matches
    fn matches_directory_pattern(
        &self,
        path: &Path,
        pattern: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for entry in WalkDir::new(path)
            .max_depth(2)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if let Some(dir_name) = entry_path.file_name() {
                    let dir_name_str = dir_name.to_string_lossy();

                    // Remove trailing slash from pattern
                    let clean_pattern = pattern.trim_end_matches('/');

                    if dir_name_str == clean_pattern {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Check if a content pattern matches
    fn matches_content_pattern(
        &self,
        path: &Path,
        pattern: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let regex = regex::Regex::new(pattern)?;

        for entry in WalkDir::new(path)
            .max_depth(2)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Ok(content) = std::fs::read_to_string(entry_path) {
                    if regex.is_match(&content) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Simple glob pattern matching
    fn matches_glob_pattern(&self, text: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('*') && pattern.ends_with('*') {
            let inner = &pattern[1..pattern.len() - 1];
            return text.contains(inner);
        }

        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            return text.ends_with(suffix);
        }

        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            return text.starts_with(prefix);
        }

        text == pattern
    }

    /// Collect evidence for a pattern
    fn collect_evidence(
        &self,
        path: &Path,
        rules: &[DetectionRule],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut evidence = Vec::new();

        for rule in rules {
            if self.evaluate_rule(path, rule)? > 0.0 {
                evidence.push(rule.description.clone());
            }
        }

        Ok(evidence)
    }

    /// Extract metadata for a pattern
    fn extract_metadata(
        &self,
        path: &Path,
        pattern_type: &PatternType,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let mut metadata = HashMap::new();

        match pattern_type {
            PatternType::Architectural(ArchitecturalPattern::Monorepo) => {
                // Extract workspace information
                if let Ok(workspace_info) = self.extract_workspace_info(path) {
                    metadata.insert(
                        "workspace_info".to_string(),
                        serde_json::to_value(workspace_info)?,
                    );
                }
            }
            PatternType::TechnologyStack(TechnologyStackPattern::RustEcosystem) => {
                // Extract Rust-specific information
                if let Ok(rust_info) = self.extract_rust_info(path) {
                    metadata.insert("rust_info".to_string(), serde_json::to_value(rust_info)?);
                }
            }
            PatternType::TechnologyStack(TechnologyStackPattern::NodeJSEcosystem) => {
                // Extract Node.js-specific information
                if let Ok(nodejs_info) = self.extract_nodejs_info(path) {
                    metadata.insert(
                        "nodejs_info".to_string(),
                        serde_json::to_value(nodejs_info)?,
                    );
                }
            }
            _ => {}
        }

        Ok(metadata)
    }

    /// Extract workspace information
    fn extract_workspace_info(
        &self,
        path: &Path,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let mut info = HashMap::new();

        // Count packages/crates
        let mut package_count = 0;
        for entry in WalkDir::new(path)
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(file_name) = entry_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str == "Cargo.toml" || file_name_str == "package.json" {
                        package_count += 1;
                    }
                }
            }
        }

        info.insert(
            "package_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(package_count)),
        );

        Ok(info)
    }

    /// Extract Rust-specific information
    fn extract_rust_info(
        &self,
        path: &Path,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let mut info = HashMap::new();

        // Check for Cargo.toml
        let cargo_toml = path.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                // Extract basic package info
                if let Some(name_match) =
                    regex::Regex::new(r#"name\s*=\s*"([^"]+)"#)?.captures(&content)
                {
                    if let Some(name) = name_match.get(1) {
                        info.insert(
                            "package_name".to_string(),
                            serde_json::Value::String(name.as_str().to_string()),
                        );
                    }
                }

                if let Some(version_match) =
                    regex::Regex::new(r#"version\s*=\s*"([^"]+)"#)?.captures(&content)
                {
                    if let Some(version) = version_match.get(1) {
                        info.insert(
                            "version".to_string(),
                            serde_json::Value::String(version.as_str().to_string()),
                        );
                    }
                }

                // Check if it's a workspace
                if content.contains("[workspace]") {
                    info.insert("is_workspace".to_string(), serde_json::Value::Bool(true));
                }
            }
        }

        Ok(info)
    }

    /// Extract Node.js-specific information
    fn extract_nodejs_info(
        &self,
        path: &Path,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let mut info = HashMap::new();

        // Check for package.json
        let package_json = path.join("package.json");
        if package_json.exists() {
            if let Ok(content) = std::fs::read_to_string(&package_json) {
                if let Ok(package_data) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(name) = package_data.get("name") {
                        info.insert("package_name".to_string(), name.clone());
                    }
                    if let Some(version) = package_data.get("version") {
                        info.insert("version".to_string(), version.clone());
                    }
                    if let Some(workspaces) = package_data.get("workspaces") {
                        info.insert("workspaces".to_string(), workspaces.clone());
                    }
                }
            }
        }

        Ok(info)
    }

    /// Get pattern statistics
    pub fn get_pattern_stats(&self) -> PatternStats {
        let mut stats = PatternStats {
            total_patterns: 0,
            pattern_distribution: HashMap::new(),
            avg_confidence: 0.0,
        };

        for pattern in &self.historical_patterns {
            stats.total_patterns += 1;

            let pattern_key = match &pattern.pattern_type {
                PatternType::Architectural(arch) => format!("architectural_{:?}", arch),
                PatternType::ProjectStructure(proj) => format!("project_{:?}", proj),
                PatternType::TechnologyStack(tech) => format!("technology_{:?}", tech),
                PatternType::BuildSystem(build) => format!("build_{:?}", build),
            };

            *stats.pattern_distribution.entry(pattern_key).or_insert(0) += 1;
        }

        if stats.total_patterns > 0 {
            let total_confidence: f64 = self.historical_patterns.iter().map(|p| p.confidence).sum();
            stats.avg_confidence = total_confidence / stats.total_patterns as f64;
        }

        stats
    }

    /// Record a pattern detection for learning
    pub fn record_pattern_detection(
        &mut self,
        pattern_type: PatternType,
        confidence: f64,
        success: bool,
    ) {
        let record = HistoricalPattern {
            pattern_type,
            confidence,
            success_rate: if success { 1.0 } else { 0.0 },
            usage_count: 1,
            last_seen: chrono::Utc::now(),
        };

        self.historical_patterns.push(record);

        // Keep only recent patterns (last 1000)
        if self.historical_patterns.len() > 1000 {
            self.historical_patterns.remove(0);
        }
    }
}

/// Pattern statistics
#[derive(Debug, Clone)]
pub struct PatternStats {
    pub total_patterns: usize,
    pub pattern_distribution: HashMap<String, usize>,
    pub avg_confidence: f64,
}

impl Default for PatternRecognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}
