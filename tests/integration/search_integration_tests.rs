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

use rhema_core::{RhemaResult, scope::Scope};
use rhema_query::search::{SearchEngine, SearchOptions, SearchType, SearchFilter};
use std::path::Path;
use std::collections::HashMap;
use tempfile::TempDir;
use std::fs;
use rhema_core::schema::ConceptDefinition;
use std::path::PathBuf;
use rhema_core::schema::RhemaScope;

/// Integration test environment for search functionality
pub struct SearchIntegrationTest {
    temp_dir: TempDir,
    search_engine: SearchEngine,
    test_scopes: Vec<Scope>,
}

impl SearchIntegrationTest {
    /// Create a new search integration test environment
    pub fn new() -> RhemaResult<Self> {
        let temp_dir = tempfile::tempdir()?;
        let search_engine = SearchEngine::new();
        
        Ok(Self {
            temp_dir,
            search_engine,
            test_scopes: Vec::new(),
        })
    }

    /// Set up test data with various file types
    pub fn setup_test_data(&mut self) -> RhemaResult<()> {
        // Create test scopes
        let scope1 = self.create_test_scope("service-a", &[
            ("knowledge.yaml", "name: Service A\nversion: 1.0.0\ndescription: Test service for search\nkeywords:\n  - service\n  - api\n  - test"),
            ("todos.yaml", "todos:\n  - id: todo-001\n    title: Implement search\n    status: pending\n    priority: high"),
            ("decisions.yaml", "decisions:\n  - id: decision-001\n    title: Use Rust for backend\n    status: approved\n    impact: high"),
            ("README.md", "# Service A\n\nThis is a test service for search functionality.\n\n## Features\n\n- Search integration\n- Performance testing\n- Benchmark results"),
        ])?;

        let scope2 = self.create_test_scope("service-b", &[
            ("knowledge.yaml", "name: Service B\nversion: 2.0.0\ndescription: Another test service\nkeywords:\n  - service\n  - database\n  - cache"),
            ("todos.yaml", "todos:\n  - id: todo-002\n    title: Optimize database queries\n    status: in-progress\n    priority: medium"),
            ("patterns.yaml", "patterns:\n  - id: pattern-001\n    name: Repository Pattern\n    description: Data access pattern\n    usage: recommended"),
        ])?;

        self.test_scopes = vec![scope1, scope2];
        Ok(())
    }

    /// Create a test scope with files
    fn create_test_scope(&self, name: &str, files: &[(&str, &str)]) -> RhemaResult<Scope> {
        let scope_path = self.temp_dir.path().join(name);
        fs::create_dir_all(&scope_path)?;

        let mut scope_files = HashMap::new();
        
        for (filename, content) in files {
            let file_path = scope_path.join(filename);
            fs::write(&file_path, content)?;
            scope_files.insert(filename.to_string(), file_path);
        }

        let scope = Scope {
            path: PathBuf::from(name),
            definition: RhemaScope {
                name: name.to_string(),
                scope_type: "test".to_string(),
                description: Some(format!("Test scope {}", name)),
                version: "1.0.0".to_string(),
                schema_version: Some("1.0.0".to_string()),
                dependencies: None,
                protocol_info: None,
                custom: HashMap::new(),
            },
            files: scope_files,
        };

        Ok(scope)
    }

    /// Test regex search functionality
    pub async fn test_regex_search(&mut self) -> RhemaResult<()> {
        // Build search index
        self.search_engine.build_index(self.temp_dir.path(), &self.test_scopes).await?;

        // Test basic regex search
        let options = SearchOptions {
            search_type: SearchType::Regex,
            limit: Some(10),
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

        let results = self.search_engine.regex_search("service", Some(options)).await?;
        assert!(!results.is_empty(), "Regex search should find results for 'service'");

        // Test regex search with file filter
        let mut filters = Vec::new();
        filters.push(SearchFilter::FileType("yaml".to_string()));
        
        let options = SearchOptions {
            search_type: SearchType::Regex,
            limit: Some(10),
            filters,
            semantic_weight: None,
            keyword_weight: None,
            min_similarity: None,
            case_sensitive: false,
            fuzzy_matching: false,
            fuzzy_distance: None,
            search_fields: Vec::new(),
            field_boosts: HashMap::new(),
        };

        let results = self.search_engine.regex_search("version", Some(options)).await?;
        assert!(!results.is_empty(), "Regex search with file filter should find results");

        Ok(())
    }

    /// Test full-text search functionality
    pub async fn test_fulltext_search(&mut self) -> RhemaResult<()> {
        // Build search index
        self.search_engine.build_index(self.temp_dir.path(), &self.test_scopes).await?;

        // Test basic full-text search
        let options = SearchOptions {
            search_type: SearchType::FullText,
            limit: Some(10),
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

        let results = self.search_engine.full_text_search("search", Some(options)).await?;
        assert!(!results.is_empty(), "Full-text search should find results for 'search'");

        // Test search with scoring
        for result in &results {
            assert!(result.score > 0.0, "Search results should have positive scores");
            assert!(!result.highlights.is_empty(), "Search results should have highlights");
        }

        Ok(())
    }

    /// Test search filtering functionality
    pub async fn test_search_filtering(&mut self) -> RhemaResult<()> {
        // Build search index
        self.search_engine.build_index(self.temp_dir.path(), &self.test_scopes).await?;

        // Test file type filtering
        let mut filters = Vec::new();
        filters.push(SearchFilter::FileType("md".to_string()));
        
        let options = SearchOptions {
            search_type: SearchType::FullText,
            limit: Some(10),
            filters,
            semantic_weight: None,
            keyword_weight: None,
            min_similarity: None,
            case_sensitive: false,
            fuzzy_matching: false,
            fuzzy_distance: None,
            search_fields: Vec::new(),
            field_boosts: HashMap::new(),
        };

        let results = self.search_engine.full_text_search("test", Some(options)).await?;
        
        // All results should be markdown files
        for result in &results {
            assert!(result.path.ends_with(".md"), "All results should be markdown files");
        }

        // Test scope filtering
        let mut filters = Vec::new();
        filters.push(SearchFilter::Scope("service-a".to_string()));
        
        let options = SearchOptions {
            search_type: SearchType::FullText,
            limit: Some(10),
            filters,
            semantic_weight: None,
            keyword_weight: None,
            min_similarity: None,
            case_sensitive: false,
            fuzzy_matching: false,
            fuzzy_distance: None,
            search_fields: Vec::new(),
            field_boosts: HashMap::new(),
        };

        let results = self.search_engine.full_text_search("service", Some(options)).await?;
        
        // All results should be from service-a scope
        for result in &results {
            assert!(result.id.starts_with("service-a:"), "All results should be from service-a scope");
        }

        Ok(())
    }

    /// Test search performance
    pub async fn test_search_performance(&mut self) -> RhemaResult<()> {
        // Build search index
        self.search_engine.build_index(self.temp_dir.path(), &self.test_scopes).await?;

        // Test search performance
        let start = std::time::Instant::now();
        
        let options = SearchOptions {
            search_type: SearchType::FullText,
            limit: Some(10),
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

        let _results = self.search_engine.full_text_search("test", Some(options)).await?;
        
        let duration = start.elapsed();
        
        // Search should complete within reasonable time (100ms for small dataset)
        assert!(duration.as_millis() < 100, "Search should complete quickly");

        Ok(())
    }

    /// Test search suggestions
    pub async fn test_search_suggestions(&mut self) -> RhemaResult<()> {
        // Build search index
        self.search_engine.build_index(self.temp_dir.path(), &self.test_scopes).await?;

        // Test search suggestions
        let suggestions = self.search_engine.get_suggestions("ser").await?;
        
        // Should find suggestions for "service"
        let has_service_suggestion = suggestions.iter().any(|s| s.text.contains("service"));
        assert!(has_service_suggestion, "Should suggest 'service' for 'ser'");

        Ok(())
    }

    /// Test search statistics
    pub fn test_search_stats(&self) -> RhemaResult<()> {
        let stats = self.search_engine.get_stats();
        
        // Verify stats contain expected fields
        assert!(stats.contains_key("total_documents"));
        assert!(stats.contains_key("total_terms"));
        assert!(stats.contains_key("full_text_enabled"));
        assert!(stats.contains_key("regex_enabled"));

        Ok(())
    }

    /// Run all integration tests
    pub async fn run_all_tests(&mut self) -> RhemaResult<()> {
        println!("Setting up test data...");
        self.setup_test_data()?;

        println!("Testing regex search...");
        self.test_regex_search().await?;

        println!("Testing full-text search...");
        self.test_fulltext_search().await?;

        println!("Testing search filtering...");
        self.test_search_filtering().await?;

        println!("Testing search performance...");
        self.test_search_performance().await?;

        println!("Testing search suggestions...");
        self.test_search_suggestions().await?;

        println!("Testing search statistics...");
        self.test_search_stats()?;

        println!("All search integration tests passed!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_integration() -> RhemaResult<()> {
        let mut test_env = SearchIntegrationTest::new()?;
        test_env.run_all_tests().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_regex_search_integration() -> RhemaResult<()> {
        let mut test_env = SearchIntegrationTest::new()?;
        test_env.setup_test_data()?;
        test_env.test_regex_search().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_fulltext_search_integration() -> RhemaResult<()> {
        let mut test_env = SearchIntegrationTest::new()?;
        test_env.setup_test_data()?;
        test_env.test_fulltext_search().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_search_filtering_integration() -> RhemaResult<()> {
        let mut test_env = SearchIntegrationTest::new()?;
        test_env.setup_test_data()?;
        test_env.test_search_filtering().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_search_performance_integration() -> RhemaResult<()> {
        let mut test_env = SearchIntegrationTest::new()?;
        test_env.setup_test_data()?;
        test_env.test_search_performance().await?;
        Ok(())
    }
} 