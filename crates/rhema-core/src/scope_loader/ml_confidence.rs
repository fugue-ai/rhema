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

use crate::scope_loader::pattern_recognition::*;
use crate::scope_loader::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Features used for ML confidence scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceFeatures {
    /// File-based features
    pub file_features: FileFeatures,
    /// Content-based features
    pub content_features: ContentFeatures,
    /// Structure-based features
    pub structure_features: StructureFeatures,
    /// Historical features
    pub historical_features: HistoricalFeatures,
    /// Context features
    pub context_features: ContextFeatures,
}

/// File-based confidence features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFeatures {
    /// Number of source files
    pub source_file_count: usize,
    /// Number of configuration files
    pub config_file_count: usize,
    /// Number of documentation files
    pub doc_file_count: usize,
    /// Total file size in bytes
    pub total_size_bytes: u64,
    /// File extension diversity
    pub extension_diversity: f64,
    /// Presence of key files (package.json, Cargo.toml, etc.)
    pub key_files_present: Vec<String>,
}

/// Content-based confidence features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFeatures {
    /// Code complexity (lines of code, cyclomatic complexity)
    pub complexity_score: f64,
    /// Documentation coverage
    pub documentation_coverage: f64,
    /// Test coverage
    pub test_coverage: f64,
    /// Code quality indicators
    pub quality_indicators: HashMap<String, f64>,
    /// Language-specific metrics
    pub language_metrics: HashMap<String, f64>,
}

/// Structure-based confidence features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureFeatures {
    /// Directory depth
    pub directory_depth: usize,
    /// Module organization score
    pub module_organization: f64,
    /// Dependency complexity
    pub dependency_complexity: f64,
    /// Architecture patterns detected
    pub architecture_patterns: Vec<String>,
    /// Build system sophistication
    pub build_system_score: f64,
}

/// Historical confidence features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalFeatures {
    /// Git commit frequency
    pub commit_frequency: f64,
    /// Recent activity score
    pub recent_activity: f64,
    /// Contributor count
    pub contributor_count: usize,
    /// Issue/PR activity
    pub issue_activity: f64,
    /// Release frequency
    pub release_frequency: f64,
}

/// Context-based confidence features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFeatures {
    /// Similarity to known patterns
    pub pattern_similarity: f64,
    /// Industry standard compliance
    pub standard_compliance: f64,
    /// Team size indicators
    pub team_size_indicators: f64,
    /// Project maturity indicators
    pub maturity_indicators: f64,
    /// Integration complexity
    pub integration_complexity: f64,
}

/// ML confidence scoring engine
pub struct MLConfidenceEngine {
    /// Feature weights for different scope types
    feature_weights: HashMap<ScopeType, FeatureWeights>,
    /// Historical confidence data for learning
    historical_data: Vec<HistoricalConfidenceRecord>,
    /// Pattern recognition models
    pattern_models: HashMap<String, PatternModel>,
    /// Pattern recognition engine
    pattern_engine: PatternRecognitionEngine,
}

/// Feature weights for different scope types
#[derive(Debug, Clone)]
pub struct FeatureWeights {
    pub file_weight: f64,
    pub content_weight: f64,
    pub structure_weight: f64,
    pub historical_weight: f64,
    pub context_weight: f64,
}

/// Historical confidence record for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalConfidenceRecord {
    pub features: ConfidenceFeatures,
    pub predicted_confidence: f64,
    pub actual_confidence: f64,
    pub scope_type: ScopeType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

/// Pattern recognition model
#[derive(Debug, Clone)]
pub struct PatternModel {
    pub pattern_name: String,
    pub confidence_threshold: f64,
    pub feature_importance: HashMap<String, f64>,
    pub success_rate: f64,
}

impl MLConfidenceEngine {
    /// Create a new ML confidence engine
    pub fn new() -> Self {
        let mut feature_weights = HashMap::new();

        // Default weights for different scope types
        feature_weights.insert(
            ScopeType::Library,
            FeatureWeights {
                file_weight: 0.25,
                content_weight: 0.30,
                structure_weight: 0.20,
                historical_weight: 0.15,
                context_weight: 0.10,
            },
        );

        feature_weights.insert(
            ScopeType::Service,
            FeatureWeights {
                file_weight: 0.20,
                content_weight: 0.25,
                structure_weight: 0.25,
                historical_weight: 0.20,
                context_weight: 0.10,
            },
        );

        feature_weights.insert(
            ScopeType::Application,
            FeatureWeights {
                file_weight: 0.30,
                content_weight: 0.25,
                structure_weight: 0.20,
                historical_weight: 0.15,
                context_weight: 0.10,
            },
        );

        feature_weights.insert(
            ScopeType::Workspace,
            FeatureWeights {
                file_weight: 0.35,
                content_weight: 0.15,
                structure_weight: 0.30,
                historical_weight: 0.10,
                context_weight: 0.10,
            },
        );

        Self {
            feature_weights,
            historical_data: Vec::new(),
            pattern_models: HashMap::new(),
            pattern_engine: PatternRecognitionEngine::new(),
        }
    }

    /// Calculate confidence score for a scope suggestion
    pub fn calculate_confidence(
        &self,
        suggestion: &ScopeSuggestion,
        path: &Path,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        // Extract features
        let features = self.extract_features(suggestion, path)?;

        // Calculate base confidence
        let base_confidence = self.calculate_base_confidence(&features, &suggestion.scope_type)?;

        // Apply pattern recognition
        let pattern_confidence =
            self.apply_pattern_recognition(&features, &suggestion.scope_type)?;

        // Apply historical learning
        let historical_confidence =
            self.apply_historical_learning(&features, &suggestion.scope_type)?;

        // Combine confidences with weights
        let final_confidence =
            (base_confidence * 0.6) + (pattern_confidence * 0.25) + (historical_confidence * 0.15);

        // Clamp to valid range
        Ok(final_confidence.max(0.0).min(1.0))
    }

    /// Extract features from a scope suggestion
    fn extract_features(
        &self,
        suggestion: &ScopeSuggestion,
        path: &Path,
    ) -> Result<ConfidenceFeatures, Box<dyn std::error::Error>> {
        let file_features = self.extract_file_features(path)?;
        let content_features = self.extract_content_features(path)?;
        let structure_features = self.extract_structure_features(path)?;
        let historical_features = self.extract_historical_features(path)?;
        let context_features = self.extract_context_features(suggestion, path)?;

        Ok(ConfidenceFeatures {
            file_features,
            content_features,
            structure_features,
            historical_features,
            context_features,
        })
    }

    /// Extract file-based features
    fn extract_file_features(
        &self,
        path: &Path,
    ) -> Result<FileFeatures, Box<dyn std::error::Error>> {
        let mut source_files = 0;
        let mut config_files = 0;
        let mut doc_files = 0;
        let mut total_size = 0u64;
        let mut extensions = std::collections::HashSet::new();
        let mut key_files = Vec::new();

        // Common key files for different project types
        let key_file_patterns = [
            "package.json",
            "Cargo.toml",
            "pom.xml",
            "build.gradle",
            "requirements.txt",
            "setup.py",
            "go.mod",
            "composer.json",
            "Gemfile",
            "mix.exs",
            "project.clj",
            "build.sbt",
        ];

        for entry in walkdir::WalkDir::new(path)
            .max_depth(5)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                // Count file size
                if let Ok(metadata) = entry_path.metadata() {
                    total_size += metadata.len();
                }

                // Categorize files
                if let Some(extension) = entry_path.extension() {
                    extensions.insert(extension.to_string_lossy().to_string());

                    match extension.to_string_lossy().to_lowercase().as_str() {
                        "rs" | "js" | "ts" | "py" | "java" | "go" | "rb" | "php" | "cs"
                        | "swift" => {
                            source_files += 1;
                        }
                        "json" | "toml" | "yaml" | "yml" | "xml" | "gradle" | "properties" => {
                            config_files += 1;
                        }
                        "md" | "txt" | "rst" | "adoc" => {
                            doc_files += 1;
                        }
                        _ => {}
                    }
                }

                // Check for key files
                if let Some(file_name) = entry_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if key_file_patterns.contains(&file_name_str.as_ref()) {
                        key_files.push(file_name_str.to_string());
                    }
                }
            }
        }

        // Calculate extension diversity (normalized by total files)
        let total_files = source_files + config_files + doc_files;
        let extension_diversity = if total_files > 0 {
            extensions.len() as f64 / total_files as f64
        } else {
            0.0
        };

        Ok(FileFeatures {
            source_file_count: source_files,
            config_file_count: config_files,
            doc_file_count: doc_files,
            total_size_bytes: total_size,
            extension_diversity,
            key_files_present: key_files,
        })
    }

    /// Extract content-based features
    fn extract_content_features(
        &self,
        path: &Path,
    ) -> Result<ContentFeatures, Box<dyn std::error::Error>> {
        // For now, use simplified metrics
        // In a full implementation, this would analyze code complexity, documentation, etc.

        let mut complexity_score: f64 = 0.0;
        let mut documentation_coverage: f64 = 0.0;
        let mut test_coverage: f64 = 0.0;
        let mut quality_indicators = HashMap::new();
        let language_metrics = HashMap::new();

        // Analyze source files for complexity
        for entry in walkdir::WalkDir::new(path)
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(extension) = entry_path.extension() {
                    match extension.to_string_lossy().to_lowercase().as_str() {
                        "rs" => {
                            // Rust-specific metrics
                            if let Ok(content) = std::fs::read_to_string(entry_path) {
                                let lines = content.lines().count();
                                let functions = content.matches("fn ").count();
                                complexity_score += (functions as f64 / lines.max(1) as f64) * 0.1;

                                // Check for documentation comments
                                let doc_comments =
                                    content.matches("///").count() + content.matches("//!").count();
                                documentation_coverage +=
                                    (doc_comments as f64 / lines.max(1) as f64) * 0.3;

                                // Check for tests
                                if content.contains("#[cfg(test)]") || content.contains("#[test]") {
                                    test_coverage += 0.2;
                                }
                            }
                        }
                        "js" | "ts" => {
                            // JavaScript/TypeScript metrics
                            if let Ok(content) = std::fs::read_to_string(entry_path) {
                                let lines = content.lines().count();
                                let functions = content.matches("function ").count()
                                    + content.matches("=>").count();
                                complexity_score += (functions as f64 / lines.max(1) as f64) * 0.1;

                                // Check for JSDoc comments
                                let doc_comments = content.matches("/**").count();
                                documentation_coverage +=
                                    (doc_comments as f64 / lines.max(1) as f64) * 0.3;

                                // Check for tests
                                if content.contains("describe(")
                                    || content.contains("test(")
                                    || content.contains("it(")
                                {
                                    test_coverage += 0.2;
                                }
                            }
                        }
                        "py" => {
                            // Python metrics
                            if let Ok(content) = std::fs::read_to_string(entry_path) {
                                let lines = content.lines().count();
                                let functions = content.matches("def ").count();
                                complexity_score += (functions as f64 / lines.max(1) as f64) * 0.1;

                                // Check for docstrings
                                let doc_comments = content.matches("\"\"\"").count()
                                    + content.matches("'''").count();
                                documentation_coverage +=
                                    (doc_comments as f64 / lines.max(1) as f64) * 0.3;

                                // Check for tests
                                if content.contains("def test_") || content.contains("unittest") {
                                    test_coverage += 0.2;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Normalize scores
        complexity_score = complexity_score.min(1.0_f64);
        documentation_coverage = documentation_coverage.min(1.0_f64);
        test_coverage = test_coverage.min(1.0_f64);

        // Add quality indicators
        quality_indicators.insert("code_quality".to_string(), complexity_score);
        quality_indicators.insert("documentation_quality".to_string(), documentation_coverage);
        quality_indicators.insert("test_quality".to_string(), test_coverage);

        Ok(ContentFeatures {
            complexity_score,
            documentation_coverage,
            test_coverage,
            quality_indicators,
            language_metrics,
        })
    }

    /// Extract structure-based features
    fn extract_structure_features(
        &self,
        path: &Path,
    ) -> Result<StructureFeatures, Box<dyn std::error::Error>> {
        let mut directory_depth = 0;
        let mut module_organization: f64 = 0.0;
        let dependency_complexity: f64 = 0.0;
        let mut architecture_patterns = Vec::new();
        let mut build_system_score: f64 = 0.0;

        // Calculate directory depth
        for entry in walkdir::WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_dir() {
                let depth = entry.depth();
                directory_depth = directory_depth.max(depth);
            }
        }

        // Check for common architecture patterns
        let patterns = [
            ("src/", "standard_source_layout"),
            ("lib/", "library_layout"),
            ("bin/", "binary_layout"),
            ("tests/", "test_layout"),
            ("docs/", "documentation_layout"),
            ("examples/", "example_layout"),
        ];

        for (pattern, name) in &patterns {
            if path.join(pattern).exists() {
                architecture_patterns.push(name.to_string());
                module_organization += 0.2;
            }
        }

        // Check build system sophistication
        let build_files = [
            "Cargo.toml",
            "package.json",
            "pom.xml",
            "build.gradle",
            "Makefile",
            "CMakeLists.txt",
            "Dockerfile",
            "docker-compose.yml",
        ];

        for build_file in &build_files {
            if path.join(build_file).exists() {
                build_system_score += 0.15;
            }
        }

        // Normalize scores
        module_organization = module_organization.min(1.0_f64);
        build_system_score = build_system_score.min(1.0_f64);

        Ok(StructureFeatures {
            directory_depth,
            module_organization,
            dependency_complexity,
            architecture_patterns,
            build_system_score,
        })
    }

    /// Extract historical features
    fn extract_historical_features(
        &self,
        _path: &Path,
    ) -> Result<HistoricalFeatures, Box<dyn std::error::Error>> {
        // For now, use simplified metrics
        // In a full implementation, this would analyze git history

        Ok(HistoricalFeatures {
            commit_frequency: 0.5,  // Placeholder
            recent_activity: 0.5,   // Placeholder
            contributor_count: 1,   // Placeholder
            issue_activity: 0.5,    // Placeholder
            release_frequency: 0.5, // Placeholder
        })
    }

    /// Extract context features
    fn extract_context_features(
        &self,
        suggestion: &ScopeSuggestion,
        path: &Path,
    ) -> Result<ContextFeatures, Box<dyn std::error::Error>> {
        let pattern_similarity: f64 = 0.0;
        let mut standard_compliance: f64 = 0.0;
        let team_size_indicators: f64 = 0.0;
        let mut maturity_indicators: f64 = 0.0;
        let integration_complexity: f64 = 0.0;

        // Check for standard compliance
        let standards = [
            ("README.md", 0.2),
            ("LICENSE", 0.1),
            ("CHANGELOG.md", 0.1),
            (".gitignore", 0.1),
            ("CONTRIBUTING.md", 0.1),
        ];

        for (file, score) in &standards {
            if path.join(file).exists() {
                standard_compliance += score;
            }
        }

        // Check for maturity indicators
        let maturity_files = [
            "docs/",
            "tests/",
            "examples/",
            "benchmarks/",
            "ci/",
            ".github/",
        ];

        for maturity_file in &maturity_files {
            if path.join(maturity_file).exists() {
                maturity_indicators += 0.2;
            }
        }

        // Normalize scores
        standard_compliance = standard_compliance.min(1.0_f64);
        maturity_indicators = maturity_indicators.min(1.0_f64);

        Ok(ContextFeatures {
            pattern_similarity,
            standard_compliance,
            team_size_indicators,
            maturity_indicators,
            integration_complexity,
        })
    }

    /// Calculate base confidence using weighted features
    fn calculate_base_confidence(
        &self,
        features: &ConfidenceFeatures,
        scope_type: &ScopeType,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let weights = self
            .feature_weights
            .get(scope_type)
            .ok_or("Unknown scope type")?;

        // Calculate weighted scores for each feature category
        let file_score = self.calculate_file_score(&features.file_features) * weights.file_weight;
        let content_score =
            self.calculate_content_score(&features.content_features) * weights.content_weight;
        let structure_score =
            self.calculate_structure_score(&features.structure_features) * weights.structure_weight;
        let historical_score = self.calculate_historical_score(&features.historical_features)
            * weights.historical_weight;
        let context_score =
            self.calculate_context_score(&features.context_features) * weights.context_weight;

        Ok(file_score + content_score + structure_score + historical_score + context_score)
    }

    /// Calculate file-based score
    fn calculate_file_score(&self, features: &FileFeatures) -> f64 {
        let mut score = 0.0;

        // Source file presence
        if features.source_file_count > 0 {
            score += 0.3;
        }

        // Configuration file presence
        if features.config_file_count > 0 {
            score += 0.2;
        }

        // Documentation presence
        if features.doc_file_count > 0 {
            score += 0.1;
        }

        // Key files presence
        score += (features.key_files_present.len() as f64 * 0.1).min(0.3);

        // Extension diversity (indicates well-structured project)
        score += features.extension_diversity * 0.1;

        score.min(1.0)
    }

    /// Calculate content-based score
    fn calculate_content_score(&self, features: &ContentFeatures) -> f64 {
        let mut score = 0.0;

        // Code quality
        score += features.complexity_score * 0.3;

        // Documentation coverage
        score += features.documentation_coverage * 0.3;

        // Test coverage
        score += features.test_coverage * 0.4;

        score.min(1.0)
    }

    /// Calculate structure-based score
    fn calculate_structure_score(&self, features: &StructureFeatures) -> f64 {
        let mut score = 0.0;

        // Module organization
        score += features.module_organization * 0.4;

        // Build system sophistication
        score += features.build_system_score * 0.4;

        // Architecture patterns
        score += (features.architecture_patterns.len() as f64 * 0.1).min(0.2);

        score.min(1.0)
    }

    /// Calculate historical-based score
    fn calculate_historical_score(&self, features: &HistoricalFeatures) -> f64 {
        let mut score = 0.0;

        // Recent activity
        score += features.recent_activity * 0.4;

        // Commit frequency
        score += features.commit_frequency * 0.3;

        // Contributor count (normalized)
        score += (features.contributor_count as f64 / 10.0).min(1.0) * 0.2;

        // Issue activity
        score += features.issue_activity * 0.1;

        score.min(1.0)
    }

    /// Calculate context-based score
    fn calculate_context_score(&self, features: &ContextFeatures) -> f64 {
        let mut score = 0.0;

        // Standard compliance
        score += features.standard_compliance * 0.4;

        // Maturity indicators
        score += features.maturity_indicators * 0.4;

        // Pattern similarity
        score += features.pattern_similarity * 0.2;

        score.min(1.0)
    }

    /// Apply pattern recognition
    fn apply_pattern_recognition(
        &self,
        features: &ConfidenceFeatures,
        scope_type: &ScopeType,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        // Use the pattern recognition engine to detect patterns
        // For now, use current directory - in a full implementation, this would use the actual project path
        let detected_patterns = self.pattern_engine.detect_patterns(&Path::new("."))?;

        if detected_patterns.is_empty() {
            return Ok(0.5); // Base confidence if no patterns detected
        }

        // Calculate weighted pattern confidence
        let mut total_confidence = 0.0;
        let mut total_weight = 0.0;

        for pattern in &detected_patterns {
            let weight = self.get_pattern_weight(&pattern.pattern_type);
            total_confidence += pattern.confidence * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            Ok(total_confidence / total_weight)
        } else {
            Ok(0.5)
        }
    }

    /// Get weight for a pattern type
    fn get_pattern_weight(&self, pattern_type: &PatternType) -> f64 {
        match pattern_type {
            PatternType::Architectural(_) => 0.4,
            PatternType::TechnologyStack(_) => 0.3,
            PatternType::BuildSystem(_) => 0.2,
            PatternType::ProjectStructure(_) => 0.1,
        }
    }

    /// Apply historical learning
    fn apply_historical_learning(
        &self,
        _features: &ConfidenceFeatures,
        _scope_type: &ScopeType,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        // For now, return a base historical confidence
        // In a full implementation, this would use historical data
        Ok(0.7)
    }

    /// Record confidence prediction for learning
    pub fn record_prediction(
        &mut self,
        features: ConfidenceFeatures,
        predicted_confidence: f64,
        actual_confidence: f64,
        scope_type: ScopeType,
        success: bool,
    ) {
        let record = HistoricalConfidenceRecord {
            features,
            predicted_confidence,
            actual_confidence,
            scope_type,
            timestamp: chrono::Utc::now(),
            success,
        };

        self.historical_data.push(record);

        // Keep only recent data (last 1000 records)
        if self.historical_data.len() > 1000 {
            self.historical_data.remove(0);
        }
    }

    /// Get confidence statistics
    pub fn get_confidence_stats(&self) -> ConfidenceStats {
        let total_predictions = self.historical_data.len();
        let successful_predictions = self.historical_data.iter().filter(|r| r.success).count();
        let avg_accuracy = if total_predictions > 0 {
            successful_predictions as f64 / total_predictions as f64
        } else {
            0.0
        };

        let avg_predicted_confidence = if total_predictions > 0 {
            self.historical_data
                .iter()
                .map(|r| r.predicted_confidence)
                .sum::<f64>()
                / total_predictions as f64
        } else {
            0.0
        };

        let avg_actual_confidence = if total_predictions > 0 {
            self.historical_data
                .iter()
                .map(|r| r.actual_confidence)
                .sum::<f64>()
                / total_predictions as f64
        } else {
            0.0
        };

        ConfidenceStats {
            total_predictions,
            successful_predictions,
            accuracy: avg_accuracy,
            avg_predicted_confidence,
            avg_actual_confidence,
        }
    }
}

/// Confidence statistics
#[derive(Debug, Clone)]
pub struct ConfidenceStats {
    pub total_predictions: usize,
    pub successful_predictions: usize,
    pub accuracy: f64,
    pub avg_predicted_confidence: f64,
    pub avg_actual_confidence: f64,
}

impl Default for MLConfidenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
