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

use std::time::Instant;
use rhema_core::{RhemaResult, scope::Scope};
use rhema_query::search::{SearchEngine, SearchOptions, SearchType, SearchFilter};
use std::path::Path;
use std::collections::HashMap;

/// Search performance benchmark suite
pub struct SearchBenchmark {
    search_engine: SearchEngine,
    test_data: Vec<TestDocument>,
}

/// Test document for benchmarking
pub struct TestDocument {
    pub id: String,
    pub content: String,
    pub path: String,
    pub doc_type: String,
}

impl SearchBenchmark {
    /// Create a new search benchmark
    pub fn new() -> Self {
        Self {
            search_engine: SearchEngine::new(),
            test_data: Vec::new(),
        }
    }

    /// Generate test data for benchmarking
    pub fn generate_test_data(&mut self, num_documents: usize) {
        self.test_data.clear();
        
        for i in 0..num_documents {
            let doc_type = match i % 4 {
                0 => "yaml",
                1 => "json", 
                2 => "markdown",
                _ => "code",
            };
            
            let content = self.generate_document_content(i, doc_type);
            
            self.test_data.push(TestDocument {
                id: format!("doc-{}", i),
                content,
                path: format!("test/{}.{}", i, doc_type),
                doc_type: doc_type.to_string(),
            });
        }
    }

    /// Generate document content for testing
    fn generate_document_content(&self, index: usize, doc_type: &str) -> String {
        match doc_type {
            "yaml" => format!(
                "name: test-document-{}\nversion: 1.0.0\ndescription: Test document for search benchmarking\nkeywords:\n  - test\n  - benchmark\n  - search\n  - performance\n",
                index
            ),
            "json" => format!(
                r#"{{"name": "test-document-{}", "version": "1.0.0", "description": "Test document for search benchmarking", "keywords": ["test", "benchmark", "search", "performance"]}}"#,
                index
            ),
            "markdown" => format!(
                "# Test Document {}\n\nThis is a test document for search benchmarking.\n\n## Features\n\n- Test feature 1\n- Test feature 2\n- Search performance\n- Benchmark results\n\n## Keywords\n\n- test\n- benchmark\n- search\n- performance\n",
                index
            ),
            "code" => format!(
                "// Test document {}\nfn test_function() {{\n    let test_var = \"test value\";\n    println!(\"{{}}\", test_var);\n}}\n\n// Benchmark function\nfn benchmark_search() {{\n    // Search implementation\n    let search_term = \"test\";\n    // ...\n}}",
                index
            ),
            _ => format!("Test document {} content", index),
        }
    }

    /// Benchmark regex search performance
    pub async fn benchmark_regex_search(&mut self, pattern: &str, num_iterations: usize) -> RhemaResult<SearchBenchmarkResult> {
        let mut total_time = 0u64;
        let mut results_count = 0usize;
        
        for _ in 0..num_iterations {
            let start = Instant::now();
            
            let options = SearchOptions {
                search_type: SearchType::Regex,
                limit: Some(100),
                filters: Vec::new(),
                semantic_weight: None,
                keyword_weight: None,
                min_similarity: None,
                case_sensitive: false,
                fuzzy_matching: false,
                fuzzy_distance: None,
                search_fields: Vec::new(),
                field_boosts: HashMap::new(),
            };
            
            let results = self.search_engine.regex_search(pattern, Some(options)).await?;
            let duration = start.elapsed();
            
            total_time += duration.as_millis() as u64;
            results_count += results.len();
        }
        
        Ok(SearchBenchmarkResult {
            search_type: "regex".to_string(),
            pattern: pattern.to_string(),
            iterations: num_iterations,
            total_time_ms: total_time,
            avg_time_ms: total_time as f64 / num_iterations as f64,
            total_results: results_count,
            avg_results_per_search: results_count as f64 / num_iterations as f64,
        })
    }

    /// Benchmark full-text search performance
    pub async fn benchmark_fulltext_search(&mut self, query: &str, num_iterations: usize) -> RhemaResult<SearchBenchmarkResult> {
        let mut total_time = 0u64;
        let mut results_count = 0usize;
        
        for _ in 0..num_iterations {
            let start = Instant::now();
            
            let options = SearchOptions {
                search_type: SearchType::FullText,
                limit: Some(100),
                filters: Vec::new(),
                semantic_weight: None,
                keyword_weight: None,
                min_similarity: None,
                case_sensitive: false,
                fuzzy_matching: false,
                fuzzy_distance: None,
                search_fields: Vec::new(),
                field_boosts: HashMap::new(),
            };
            
            let results = self.search_engine.full_text_search(query, Some(options)).await?;
            let duration = start.elapsed();
            
            total_time += duration.as_millis() as u64;
            results_count += results.len();
        }
        
        Ok(SearchBenchmarkResult {
            search_type: "fulltext".to_string(),
            pattern: query.to_string(),
            iterations: num_iterations,
            total_time_ms: total_time,
            avg_time_ms: total_time as f64 / num_iterations as f64,
            total_results: results_count,
            avg_results_per_search: results_count as f64 / num_iterations as f64,
        })
    }

    /// Benchmark index building performance
    pub async fn benchmark_index_building(&mut self, num_documents: usize) -> RhemaResult<IndexBenchmarkResult> {
        let start = Instant::now();
        
        // Create mock scopes for testing
        let mut scopes = Vec::new();
        for i in 0..(num_documents / 10).max(1) {
            let mut scope = Scope {
                definition: rhema_core::schema::ScopeDefinition {
                    name: format!("test-scope-{}", i),
                    version: "1.0.0".to_string(),
                    description: Some("Test scope for benchmarking".to_string()),
                    keywords: vec!["test".to_string(), "benchmark".to_string()],
                    maintainers: Vec::new(),
                    repository: None,
                    license: None,
                },
                path: format!("test-scope-{}", i),
                files: HashMap::new(),
            };
            
            // Add files to scope
            for j in 0..10 {
                let doc_index = i * 10 + j;
                if doc_index < self.test_data.len() {
                    let doc = &self.test_data[doc_index];
                    scope.files.insert(doc.path.clone(), doc.path.clone());
                }
            }
            
            scopes.push(scope);
        }
        
        // Build index
        self.search_engine.build_index(Path::new("."), &scopes).await?;
        
        let duration = start.elapsed();
        
        Ok(IndexBenchmarkResult {
            documents: num_documents,
            build_time_ms: duration.as_millis() as u64,
            documents_per_second: num_documents as f64 / (duration.as_secs_f64()),
        })
    }

    /// Run comprehensive search benchmarks
    pub async fn run_comprehensive_benchmarks(&mut self) -> RhemaResult<ComprehensiveBenchmarkResult> {
        let mut results = ComprehensiveBenchmarkResult {
            index_building: Vec::new(),
            regex_search: Vec::new(),
            fulltext_search: Vec::new(),
        };

        // Test different document sizes
        for doc_count in [100, 500, 1000, 5000] {
            self.generate_test_data(doc_count);
            
            // Benchmark index building
            let index_result = self.benchmark_index_building(doc_count).await?;
            results.index_building.push(index_result);
            
            // Benchmark regex search
            let regex_result = self.benchmark_regex_search("test", 10).await?;
            results.regex_search.push(regex_result);
            
            // Benchmark full-text search
            let fulltext_result = self.benchmark_fulltext_search("benchmark", 10).await?;
            results.fulltext_search.push(fulltext_result);
        }

        Ok(results)
    }
}

/// Search benchmark result
#[derive(Debug, Clone)]
pub struct SearchBenchmarkResult {
    pub search_type: String,
    pub pattern: String,
    pub iterations: usize,
    pub total_time_ms: u64,
    pub avg_time_ms: f64,
    pub total_results: usize,
    pub avg_results_per_search: f64,
}

/// Index building benchmark result
#[derive(Debug, Clone)]
pub struct IndexBenchmarkResult {
    pub documents: usize,
    pub build_time_ms: u64,
    pub documents_per_second: f64,
}

/// Comprehensive benchmark results
#[derive(Debug, Clone)]
pub struct ComprehensiveBenchmarkResult {
    pub index_building: Vec<IndexBenchmarkResult>,
    pub regex_search: Vec<SearchBenchmarkResult>,
    pub fulltext_search: Vec<SearchBenchmarkResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_benchmark_basic() -> RhemaResult<()> {
        let mut benchmark = SearchBenchmark::new();
        benchmark.generate_test_data(100);
        
        let result = benchmark.benchmark_regex_search("test", 5).await?;
        assert!(result.avg_time_ms > 0.0);
        assert!(result.iterations == 5);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_index_building_benchmark() -> RhemaResult<()> {
        let mut benchmark = SearchBenchmark::new();
        benchmark.generate_test_data(500);
        
        let result = benchmark.benchmark_index_building(500).await?;
        assert!(result.build_time_ms > 0);
        assert!(result.documents_per_second > 0.0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_comprehensive_benchmarks() -> RhemaResult<()> {
        let mut benchmark = SearchBenchmark::new();
        
        let results = benchmark.run_comprehensive_benchmarks().await?;
        
        // Verify we have results for different document sizes
        assert!(!results.index_building.is_empty());
        assert!(!results.regex_search.is_empty());
        assert!(!results.fulltext_search.is_empty());
        
        // Verify performance characteristics
        for index_result in &results.index_building {
            assert!(index_result.build_time_ms > 0);
            assert!(index_result.documents_per_second > 0.0);
        }
        
        for search_result in &results.regex_search {
            assert!(search_result.avg_time_ms > 0.0);
        }
        
        for search_result in &results.fulltext_search {
            assert!(search_result.avg_time_ms > 0.0);
        }
        
        Ok(())
    }
} 