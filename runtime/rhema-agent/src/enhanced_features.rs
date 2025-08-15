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

use rhema_api::{Rhema, RhemaResult};
use rhema_core::scope::Scope;
use rhema_query::QueryResult;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

/// Enhanced search functionality with advanced filtering and ranking
pub struct EnhancedSearch<'a> {
    rhema: &'a Rhema,
}

impl<'a> EnhancedSearch<'a> {
    pub fn new(rhema: &'a Rhema) -> Self {
        Self { rhema }
    }

    /// Perform advanced search with multiple criteria
    pub async fn advanced_search(
        &self,
        query: &str,
        filters: &SearchFilters,
    ) -> RhemaResult<Vec<EnhancedSearchResult>> {
        let start_time = Instant::now();

        // Perform basic search
        let basic_results = self.rhema.search_regex(query, None)?;

        // Apply filters
        let mut filtered_results = Vec::new();
        for result in basic_results {
            if self.matches_filters(&result, filters) {
                // Convert QueryResult to our SearchResult format
                let content = match &result.data {
                    serde_yaml::Value::String(s) => s.clone(),
                    _ => format!("{:?}", result.data),
                };

                filtered_results.push(EnhancedSearchResult {
                    content,
                    file_path: result.path.clone(),
                    line_number: 0, // QueryResult doesn't have line numbers
                    relevance_score: self.calculate_relevance_score(&result, query),
                    metadata: result.metadata.clone(),
                });
            }
        }

        // Sort by relevance score
        filtered_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        // Apply limit
        if let Some(limit) = filters.limit {
            filtered_results.truncate(limit);
        }

        let duration = start_time.elapsed();
        tracing::info!("Advanced search completed in {:?}", duration);

        Ok(filtered_results)
    }

    /// Search with semantic understanding
    pub async fn semantic_search(
        &self,
        query: &str,
        scope: Option<&str>,
    ) -> RhemaResult<Vec<EnhancedSearchResult>> {
        let start_time = Instant::now();

        // This would integrate with the knowledge system for semantic search
        // For now, we'll use basic search with enhanced ranking
        let filters = SearchFilters {
            scope: scope.map(|s| s.to_string()),
            content_types: Some(vec!["code".to_string(), "documentation".to_string()]),
            limit: Some(20),
            ..Default::default()
        };

        let results = self.advanced_search(query, &filters).await?;

        let duration = start_time.elapsed();
        tracing::info!("Semantic search completed in {:?}", duration);

        Ok(results)
    }

    fn matches_filters(&self, result: &QueryResult, filters: &SearchFilters) -> bool {
        // Check scope filter
        if let Some(ref scope) = filters.scope {
            if !result.path.contains(scope) {
                return false;
            }
        }

        // Check content type filter
        if let Some(ref content_types) = filters.content_types {
            let file_extension = result.path.split('.').last().unwrap_or("");
            let content_type = self.get_content_type(file_extension);
            if !content_types.contains(&content_type) {
                return false;
            }
        }

        // Check file size filter
        if let Some(max_size) = filters.max_file_size {
            if let Ok(metadata) = std::fs::metadata(&result.path) {
                if metadata.len() > max_size {
                    return false;
                }
            }
        }

        true
    }

    fn calculate_relevance_score(&self, result: &QueryResult, query: &str) -> f64 {
        let mut score = 0.0;

        // Get content from result data
        let content = match &result.data {
            serde_yaml::Value::String(s) => s,
            _ => return score,
        };

        // Exact match bonus
        if content.to_lowercase().contains(&query.to_lowercase()) {
            score += 10.0;
        }

        // File type bonus
        let file_extension = result.path.split('.').last().unwrap_or("");
        match file_extension {
            "rs" | "ts" | "js" | "py" => score += 5.0, // Code files
            "md" | "txt" | "rst" => score += 3.0,      // Documentation
            "yml" | "yaml" | "toml" | "json" => score += 2.0, // Config files
            _ => score += 1.0,
        }

        // Scope bonus (prefer matches in current scope)
        if result.scope == "current" {
            score += 3.0;
        }

        score
    }

    fn get_content_type(&self, extension: &str) -> String {
        match extension {
            "rs" | "ts" | "js" | "py" | "java" | "cpp" | "c" => "code".to_string(),
            "md" | "txt" | "rst" | "adoc" => "documentation".to_string(),
            "yml" | "yaml" | "toml" | "json" | "xml" => "configuration".to_string(),
            "sql" => "database".to_string(),
            "sh" | "bash" | "zsh" => "script".to_string(),
            _ => "other".to_string(),
        }
    }
}

/// Search filters for advanced search
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SearchFilters {
    pub scope: Option<String>,
    pub content_types: Option<Vec<String>>,
    pub max_file_size: Option<u64>,
    pub limit: Option<usize>,
    pub include_hidden: bool,
}

/// Enhanced search result with relevance scoring
#[derive(Debug, Clone)]
pub struct EnhancedSearchResult {
    pub content: String,
    pub file_path: String,
    pub line_number: usize,
    pub relevance_score: f64,
    pub metadata: HashMap<String, serde_yaml::Value>,
}

/// Batch command processor for handling multiple operations
pub struct BatchProcessor<'a> {
    rhema: &'a Rhema,
}

impl<'a> BatchProcessor<'a> {
    pub fn new(rhema: &'a Rhema) -> Self {
        Self { rhema }
    }

    /// Process multiple commands in batch
    pub async fn process_batch(
        &self,
        commands: Vec<BatchCommand>,
    ) -> RhemaResult<Vec<BatchResult>> {
        let start_time = Instant::now();
        let mut results = Vec::new();

        for (index, command) in commands.into_iter().enumerate() {
            let command_start = Instant::now();

            let result = match command {
                BatchCommand::Search { query, filters } => {
                    let enhanced_search = EnhancedSearch::new(self.rhema);
                    let search_results = enhanced_search.advanced_search(&query, &filters).await?;

                    BatchResult::Search {
                        query,
                        results: search_results,
                        duration: command_start.elapsed(),
                    }
                }
                BatchCommand::Validate { scope } => {
                    let scopes = if let Some(scope_name) = scope {
                        vec![self.rhema.get_scope(&scope_name)?]
                    } else {
                        self.rhema.discover_scopes()?
                    };

                    let mut validation_results = Vec::new();
                    for scope in scopes {
                        let scope_result = self.validate_scope(&scope).await?;
                        validation_results.push(scope_result);
                    }

                    BatchResult::Validate {
                        scopes: validation_results,
                        duration: command_start.elapsed(),
                    }
                }
                BatchCommand::Query { query, format } => {
                    let result = self.rhema.query(&query)?;
                    let formatted_result = match format.as_str() {
                        "json" => serde_json::to_string_pretty(&result)?,
                        "yaml" => serde_yaml::to_string(&result)?,
                        _ => format!("{:?}", result),
                    };

                    BatchResult::Query {
                        query,
                        result: formatted_result,
                        duration: command_start.elapsed(),
                    }
                }
            };

            results.push(result);
            tracing::info!(
                "Batch command {} completed in {:?}",
                index + 1,
                command_start.elapsed()
            );
        }

        let total_duration = start_time.elapsed();
        tracing::info!("Batch processing completed in {:?}", total_duration);

        Ok(results)
    }

    async fn validate_scope(&self, scope: &Scope) -> RhemaResult<ScopeValidationResult> {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check if scope path exists
        if !scope.path.exists() {
            issues.push("Scope path does not exist".to_string());
        }

        // Check for required files
        let required_files = ["knowledge.yaml", "todos.yaml", "decisions.yaml"];
        for file in &required_files {
            let file_path = scope.path.join(file);
            if !file_path.exists() {
                warnings.push(format!("Missing recommended file: {}", file));
            }
        }

        // Check for configuration files
        let config_files = ["Cargo.toml", "package.json", "pyproject.toml"];
        let mut has_config = false;
        for file in &config_files {
            if scope.path.join(file).exists() {
                has_config = true;
                break;
            }
        }

        if !has_config {
            warnings.push("No configuration file found".to_string());
        }

        Ok(ScopeValidationResult {
            scope_name: scope.definition.name.clone(),
            path: scope.path.clone(),
            is_valid: issues.is_empty(),
            issues,
            warnings,
        })
    }
}

/// Batch command types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BatchCommand {
    Search {
        query: String,
        filters: SearchFilters,
    },
    Validate {
        scope: Option<String>,
    },
    Query {
        query: String,
        format: String,
    },
}

/// Batch command results
#[derive(Debug, Clone)]
pub enum BatchResult {
    Search {
        query: String,
        results: Vec<EnhancedSearchResult>,
        duration: std::time::Duration,
    },
    Validate {
        scopes: Vec<ScopeValidationResult>,
        duration: std::time::Duration,
    },
    Query {
        query: String,
        result: String,
        duration: std::time::Duration,
    },
}

/// Scope validation result
#[derive(Debug, Clone)]
pub struct ScopeValidationResult {
    pub scope_name: String,
    pub path: PathBuf,
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

/// Performance monitoring for CLI operations
pub struct PerformanceMonitor {
    operations: HashMap<String, Vec<OperationMetrics>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            operations: HashMap::new(),
        }
    }

    /// Record operation metrics
    pub fn record_operation(&mut self, operation: &str, duration: std::time::Duration) {
        let metrics = OperationMetrics {
            timestamp: chrono::Utc::now(),
            duration,
        };

        self.operations
            .entry(operation.to_string())
            .or_insert_with(Vec::new)
            .push(metrics);
    }

    /// Get performance statistics
    pub fn get_statistics(&self) -> PerformanceStatistics {
        let mut stats = PerformanceStatistics {
            operations: HashMap::new(),
        };

        for (operation, metrics) in &self.operations {
            if !metrics.is_empty() {
                let total_duration: std::time::Duration = metrics.iter().map(|m| m.duration).sum();
                let avg_duration = total_duration / metrics.len() as u32;
                let min_duration = metrics.iter().map(|m| m.duration).min().unwrap();
                let max_duration = metrics.iter().map(|m| m.duration).max().unwrap();

                stats.operations.insert(
                    operation.clone(),
                    OperationStatistics {
                        count: metrics.len(),
                        total_duration,
                        avg_duration,
                        min_duration,
                        max_duration,
                    },
                );
            }
        }

        stats
    }

    /// Generate performance report
    pub fn generate_report(&self) -> String {
        let stats = self.get_statistics();
        let mut report = String::new();

        report.push_str("Performance Report\n");
        report.push_str("==================\n\n");

        for (operation, stat) in stats.operations {
            report.push_str(&format!("Operation: {}\n", operation));
            report.push_str(&format!("  Count: {}\n", stat.count));
            report.push_str(&format!("  Total Duration: {:?}\n", stat.total_duration));
            report.push_str(&format!("  Average Duration: {:?}\n", stat.avg_duration));
            report.push_str(&format!("  Min Duration: {:?}\n", stat.min_duration));
            report.push_str(&format!("  Max Duration: {:?}\n", stat.max_duration));
            report.push_str("\n");
        }

        report
    }
}

/// Operation metrics
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub duration: std::time::Duration,
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStatistics {
    pub operations: HashMap<String, OperationStatistics>,
}

/// Operation statistics
#[derive(Debug, Clone)]
pub struct OperationStatistics {
    pub count: usize,
    pub total_duration: std::time::Duration,
    pub avg_duration: std::time::Duration,
    pub min_duration: std::time::Duration,
    pub max_duration: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filters_struct() {
        let filters = SearchFilters {
            scope: Some("test-scope".to_string()),
            content_types: Some(vec!["code".to_string(), "documentation".to_string()]),
            max_file_size: Some(1024 * 1024),
            limit: Some(10),
            include_hidden: false,
        };

        assert_eq!(filters.scope, Some("test-scope".to_string()));
        assert_eq!(
            filters.content_types,
            Some(vec!["code".to_string(), "documentation".to_string()])
        );
        assert_eq!(filters.max_file_size, Some(1024 * 1024));
        assert_eq!(filters.limit, Some(10));
        assert_eq!(filters.include_hidden, false);
    }

    #[test]
    fn test_enhanced_search_result() {
        let result = EnhancedSearchResult {
            content: "fn test_function() {}".to_string(),
            file_path: "src/test.rs".to_string(),
            line_number: 42,
            relevance_score: 8.5,
            metadata: HashMap::new(),
        };

        assert_eq!(result.content, "fn test_function() {}");
        assert_eq!(result.file_path, "src/test.rs");
        assert_eq!(result.line_number, 42);
        assert_eq!(result.relevance_score, 8.5);
    }

    #[test]
    fn test_batch_command_serialization() {
        let search_command = BatchCommand::Search {
            query: "test query".to_string(),
            filters: SearchFilters {
                limit: Some(5),
                ..Default::default()
            },
        };

        // Test that it can be serialized and deserialized
        let serialized = serde_json::to_string(&search_command).unwrap();
        let deserialized: BatchCommand = serde_json::from_str(&serialized).unwrap();

        if let BatchCommand::Search { query, filters } = deserialized {
            assert_eq!(query, "test query");
            assert_eq!(filters.limit, Some(5));
        } else {
            panic!("Expected Search command");
        }
    }

    #[test]
    fn test_batch_result_creation() {
        let search_results = vec![EnhancedSearchResult {
            content: "fn test_function() {}".to_string(),
            file_path: "src/test.rs".to_string(),
            line_number: 1,
            relevance_score: 9.0,
            metadata: HashMap::new(),
        }];

        let batch_result = BatchResult::Search {
            query: "test".to_string(),
            results: search_results,
            duration: std::time::Duration::from_millis(100),
        };

        if let BatchResult::Search {
            query,
            results,
            duration,
        } = batch_result
        {
            assert_eq!(query, "test");
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].content, "fn test_function() {}");
            assert_eq!(duration.as_millis(), 100);
        } else {
            panic!("Expected Search result");
        }
    }

    #[test]
    fn test_relevance_score_ordering() {
        let mut results = vec![
            EnhancedSearchResult {
                content: "fn low_score() {}".to_string(),
                file_path: "src/low.rs".to_string(),
                line_number: 1,
                relevance_score: 3.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn high_score() {}".to_string(),
                file_path: "src/high.rs".to_string(),
                line_number: 1,
                relevance_score: 9.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn medium_score() {}".to_string(),
                file_path: "src/medium.rs".to_string(),
                line_number: 1,
                relevance_score: 6.0,
                metadata: HashMap::new(),
            },
        ];

        // Sort by relevance score (descending)
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        assert_eq!(results[0].relevance_score, 9.0);
        assert_eq!(results[1].relevance_score, 6.0);
        assert_eq!(results[2].relevance_score, 3.0);
    }

    #[test]
    fn test_search_limit_application() {
        let mut results = vec![
            EnhancedSearchResult {
                content: "fn test1() {}".to_string(),
                file_path: "src/test1.rs".to_string(),
                line_number: 1,
                relevance_score: 9.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn test2() {}".to_string(),
                file_path: "src/test2.rs".to_string(),
                line_number: 1,
                relevance_score: 8.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn test3() {}".to_string(),
                file_path: "src/test3.rs".to_string(),
                line_number: 1,
                relevance_score: 7.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn test4() {}".to_string(),
                file_path: "src/test4.rs".to_string(),
                line_number: 1,
                relevance_score: 6.0,
                metadata: HashMap::new(),
            },
            EnhancedSearchResult {
                content: "fn test5() {}".to_string(),
                file_path: "src/test5.rs".to_string(),
                line_number: 1,
                relevance_score: 5.0,
                metadata: HashMap::new(),
            },
        ];

        // Apply limit of 3
        results.truncate(3);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].relevance_score, 9.0);
        assert_eq!(results[1].relevance_score, 8.0);
        assert_eq!(results[2].relevance_score, 7.0);
    }
}
