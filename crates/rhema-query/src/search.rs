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

use chrono::{DateTime, Utc};
use glob::Pattern;
use rayon::prelude::*;
use regex::Regex;
use rhema_core::{scope::Scope, RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

/// Search engine for advanced search capabilities
#[derive(Debug, Clone)]
pub struct SearchEngine {
    /// Search configuration
    config: SearchConfig,
    /// Search index for full-text search
    search_index: Option<SearchIndex>,
    /// Semantic search model
    semantic_model: Option<SemanticModel>,
    /// Performance metrics
    performance_metrics: SearchPerformanceMetrics,
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Enable full-text search
    pub full_text_enabled: bool,
    /// Enable semantic search
    pub semantic_enabled: bool,
    /// Enable hybrid search
    pub hybrid_enabled: bool,
    /// Enable regex search
    pub regex_enabled: bool,
    /// Default search limit
    pub default_limit: usize,
    /// Search timeout in seconds
    pub timeout_seconds: u64,
    /// Minimum similarity threshold for semantic search
    pub min_similarity_threshold: f64,
    /// Enable parallel processing
    pub parallel_processing: bool,
    /// Maximum file size to index (in bytes)
    pub max_file_size: usize,
    /// File types to include in search
    pub included_file_types: Vec<String>,
    /// File types to exclude from search
    pub excluded_file_types: Vec<String>,
    /// Enable search result caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

/// Search index for full-text search
#[derive(Debug, Clone)]
pub struct SearchIndex {
    /// Indexed documents
    documents: HashMap<String, IndexedDocument>,
    /// Inverted index for fast keyword lookup
    inverted_index: HashMap<String, Vec<String>>,
    /// Document frequency for TF-IDF scoring
    document_frequency: HashMap<String, usize>,
    /// Total number of documents
    total_documents: usize,
    /// Index metadata
    metadata: HashMap<String, Value>,
}

/// Indexed document
#[derive(Debug, Clone)]
pub struct IndexedDocument {
    /// Document ID
    pub id: String,
    /// Document content
    pub content: String,
    /// Document metadata
    pub metadata: HashMap<String, Value>,
    /// Document path
    pub path: String,
    /// Document size in bytes
    pub size_bytes: usize,
    /// Last indexed timestamp
    pub indexed_at: DateTime<Utc>,
    /// Document type
    pub doc_type: DocumentType,
    /// Document language (if detected)
    pub language: Option<String>,
}

/// Document types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    YAML,
    JSON,
    Markdown,
    Text,
    Code,
    Configuration,
    Documentation,
    Other,
}

/// Semantic search model
#[derive(Debug, Clone)]
pub struct SemanticModel {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Document embeddings
    pub embeddings: HashMap<String, Vec<f32>>,
    /// Model configuration
    pub config: HashMap<String, Value>,
}

/// Search options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Search type
    pub search_type: SearchType,
    /// Search limit
    pub limit: Option<usize>,
    /// Search filters
    pub filters: Vec<SearchFilter>,
    /// Semantic weight for hybrid search
    pub semantic_weight: Option<f64>,
    /// Keyword weight for hybrid search
    pub keyword_weight: Option<f64>,
    /// Minimum similarity threshold
    pub min_similarity: Option<f64>,
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Enable fuzzy matching
    pub fuzzy_matching: bool,
    /// Fuzzy distance threshold
    pub fuzzy_distance: Option<u32>,
    /// Search in specific fields
    pub search_fields: Vec<String>,
    /// Boost certain fields
    pub field_boosts: HashMap<String, f64>,
}

/// Search types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    FullText,
    Semantic,
    Hybrid,
    Regex,
    Fuzzy,
    Exact,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchFilter {
    FileType(String),
    Path(String),
    DateRange(DateTime<Utc>, DateTime<Utc>),
    Scope(String),
    SizeRange(usize, usize),
    Language(String),
    Custom(String, Value),
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Result ID
    pub id: String,
    /// Result content
    pub content: String,
    /// Result path
    pub path: String,
    /// Result score
    pub score: f64,
    /// Result metadata
    pub metadata: HashMap<String, Value>,
    /// Search type used
    pub search_type: SearchType,
    /// Highlighted matches
    pub highlights: Vec<String>,
    /// Match positions
    pub match_positions: Vec<MatchPosition>,
    /// Relevance explanation
    pub relevance_explanation: Option<String>,
    /// Document type
    pub doc_type: DocumentType,
    /// File size in bytes
    pub file_size: usize,
    /// Last modified timestamp
    pub last_modified: Option<DateTime<Utc>>,
}

/// Match position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPosition {
    /// Start position
    pub start: usize,
    /// End position
    pub end: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Match text
    pub text: String,
}

/// Search suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    /// Suggestion text
    pub text: String,
    /// Suggestion score
    pub score: f64,
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Suggestion metadata
    pub metadata: HashMap<String, Value>,
}

/// Suggestion types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    QueryCompletion,
    PopularQuery,
    RelatedQuery,
    AutoCorrect,
    Synonym,
    TypoCorrection,
}

/// Search performance metrics
#[derive(Debug, Clone)]
pub struct SearchPerformanceMetrics {
    /// Total searches performed
    pub total_searches: u64,
    /// Average search time in milliseconds
    pub avg_search_time_ms: f64,
    /// Total search time in milliseconds
    pub total_search_time_ms: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Index size in bytes
    pub index_size_bytes: usize,
    /// Last performance update
    pub last_updated: DateTime<Utc>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            full_text_enabled: true,
            semantic_enabled: true,
            hybrid_enabled: true,
            regex_enabled: true,
            default_limit: 20,
            timeout_seconds: 30,
            min_similarity_threshold: 0.7,
            parallel_processing: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            included_file_types: vec![
                "*.yaml".to_string(),
                "*.yml".to_string(),
                "*.json".to_string(),
                "*.md".to_string(),
                "*.txt".to_string(),
                "*.rs".to_string(),
                "*.toml".to_string(),
            ],
            excluded_file_types: vec![
                "*.git".to_string(),
                "*.target".to_string(),
                "*.node_modules".to_string(),
            ],
            enable_caching: true,
            cache_ttl_seconds: 300, // 5 minutes
        }
    }
}

impl SearchEngine {
    /// Create a new search engine
    pub fn new() -> Self {
        Self {
            config: SearchConfig::default(),
            search_index: None,
            semantic_model: None,
            performance_metrics: SearchPerformanceMetrics {
                total_searches: 0,
                avg_search_time_ms: 0.0,
                total_search_time_ms: 0,
                cache_hit_rate: 0.0,
                cache_hits: 0,
                cache_misses: 0,
                index_size_bytes: 0,
                last_updated: Utc::now(),
            },
        }
    }

    /// Create a search engine with custom configuration
    pub fn with_config(config: SearchConfig) -> Self {
        Self {
            config,
            search_index: None,
            semantic_model: None,
            performance_metrics: SearchPerformanceMetrics {
                total_searches: 0,
                avg_search_time_ms: 0.0,
                total_search_time_ms: 0,
                cache_hit_rate: 0.0,
                cache_hits: 0,
                cache_misses: 0,
                index_size_bytes: 0,
                last_updated: Utc::now(),
            },
        }
    }

    /// Build search index from repository
    pub async fn build_index(&mut self, repo_path: &Path, scopes: &[Scope]) -> RhemaResult<()> {
        let start_time = Instant::now();
        let mut index = SearchIndex {
            documents: HashMap::new(),
            inverted_index: HashMap::new(),
            document_frequency: HashMap::new(),
            total_documents: 0,
            metadata: HashMap::new(),
        };

        // Index all scope files
        for scope in scopes {
            for (file_name, file_path) in &scope.files {
                let full_path = repo_path.join(file_path);
                if full_path.exists() {
                    // Check file size
                    if let Ok(metadata) = std::fs::metadata(&full_path) {
                        if metadata.len() > self.config.max_file_size as u64 {
                            continue; // Skip large files
                        }
                    }

                    // Check file type filters
                    if !self.should_index_file(file_name) {
                        continue;
                    }

                    if let Ok(content) = std::fs::read_to_string(&full_path) {
                        let doc_id = format!("{}:{}", scope.definition.name, file_name);
                        let doc_type = self.detect_document_type(file_name, &content);

                        let document = IndexedDocument {
                            id: doc_id.clone(),
                            content: content.clone(),
                            metadata: HashMap::new(), // TODO: Add metadata support
                            path: file_name.clone(),
                            size_bytes: content.len(),
                            indexed_at: Utc::now(),
                            doc_type,
                            language: self.detect_language(&content),
                        };

                        index.documents.insert(doc_id.clone(), document);
                        self.index_document(&mut index, &doc_id, &content);
                    }
                }
            }
        }

        index.total_documents = index.documents.len();
        let total_docs = index.total_documents;
        self.search_index = Some(index);

        // Update performance metrics
        let build_time = start_time.elapsed();
        self.performance_metrics.index_size_bytes = self.calculate_index_size();
        self.performance_metrics.last_updated = Utc::now();

        tracing::info!(
            "Search index built in {:?} with {} documents",
            build_time,
            total_docs
        );
        Ok(())
    }

    /// Check if a file should be indexed based on configuration
    fn should_index_file(&self, file_name: &str) -> bool {
        // Check excluded patterns first
        for pattern in &self.config.excluded_file_types {
            if let Ok(glob_pattern) = Pattern::new(pattern) {
                if glob_pattern.matches(file_name) {
                    return false;
                }
            }
        }

        // Check included patterns
        if self.config.included_file_types.is_empty() {
            return true; // Include all if no specific patterns
        }

        for pattern in &self.config.included_file_types {
            if let Ok(glob_pattern) = Pattern::new(pattern) {
                if glob_pattern.matches(file_name) {
                    return true;
                }
            }
        }

        false
    }

    /// Detect document type based on filename and content
    fn detect_document_type(&self, file_name: &str, content: &str) -> DocumentType {
        let extension = file_name.split('.').last().unwrap_or("").to_lowercase();

        match extension.as_str() {
            "yaml" | "yml" => DocumentType::YAML,
            "json" => DocumentType::JSON,
            "md" | "markdown" => DocumentType::Markdown,
            "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "go" => DocumentType::Code,
            "toml" | "ini" | "cfg" | "conf" => DocumentType::Configuration,
            "txt" => DocumentType::Text,
            _ => {
                // Try to detect from content
                if content.trim().starts_with("---") || content.contains(": ") {
                    DocumentType::YAML
                } else if content.trim().starts_with('{') || content.trim().starts_with('[') {
                    DocumentType::JSON
                } else if content.contains("# ") || content.contains("## ") {
                    DocumentType::Markdown
                } else {
                    DocumentType::Other
                }
            }
        }
    }

    /// Detect language from content (basic implementation)
    fn detect_language(&self, content: &str) -> Option<String> {
        // Basic language detection based on file patterns
        if content.contains("fn ") && content.contains("->") {
            Some("rust".to_string())
        } else if content.contains("def ") && content.contains("import ") {
            Some("python".to_string())
        } else if content.contains("function ") && content.contains("const ") {
            Some("javascript".to_string())
        } else {
            None
        }
    }

    /// Index a document for full-text search
    fn index_document(&self, index: &mut SearchIndex, doc_id: &str, content: &str) {
        // Tokenize content
        let tokens = self.tokenize(content);

        // Update document frequency
        for token in &tokens {
            *index.document_frequency.entry(token.clone()).or_insert(0) += 1;
        }

        // Build inverted index
        for token in tokens {
            index
                .inverted_index
                .entry(token.to_lowercase())
                .or_insert_with(Vec::new)
                .push(doc_id.to_string());
        }
    }

    /// Tokenize content for indexing
    fn tokenize(&self, content: &str) -> Vec<String> {
        content
            .split_whitespace()
            .filter_map(|word| {
                let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
                if !cleaned.is_empty() && cleaned.len() > 1 {
                    Some(cleaned.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Perform full-text search with advanced ranking
    pub async fn full_text_search(
        &mut self,
        query: &str,
        options: Option<SearchOptions>,
    ) -> RhemaResult<Vec<SearchResult>> {
        let start_time = Instant::now();
        let options = options.unwrap_or_else(|| SearchOptions {
            search_type: SearchType::FullText,
            limit: Some(self.config.default_limit),
            filters: Vec::new(),
            semantic_weight: None,
            keyword_weight: None,
            min_similarity: None,
            case_sensitive: false,
            fuzzy_matching: false,
            fuzzy_distance: None,
            search_fields: Vec::new(),
            field_boosts: HashMap::new(),
        });

        let index = self
            .search_index
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Search index not built".to_string()))?;

        let query_tokens = self.tokenize(query);
        let mut doc_scores: HashMap<String, f64> = HashMap::new();

        // Calculate TF-IDF scores
        for token in query_tokens {
            let token_lower = token.to_lowercase();
            if let Some(doc_ids) = index.inverted_index.get(&token_lower) {
                let df = index.document_frequency.get(&token).unwrap_or(&1);
                let idf = (index.total_documents as f64 / *df as f64).ln();

                for doc_id in doc_ids {
                    let tf = doc_ids.iter().filter(|&&ref id| id == doc_id).count() as f64;
                    let tf_idf = tf * idf;
                    *doc_scores.entry(doc_id.clone()).or_insert(0.0) += tf_idf;
                }
            }
        }

        // Convert to search results with enhanced metadata
        let mut results: Vec<SearchResult> = doc_scores
            .into_iter()
            .filter_map(|(doc_id, score)| {
                if let Some(doc) = index.documents.get(&doc_id) {
                    let highlights = self.highlight_matches(&doc.content, query);
                    let match_positions = self.find_match_positions(&doc.content, query);

                    Some(SearchResult {
                        id: doc_id,
                        content: doc.content.clone(),
                        path: doc.path.clone(),
                        score,
                        metadata: doc.metadata.clone(),
                        search_type: SearchType::FullText,
                        highlights,
                        match_positions,
                        relevance_explanation: Some(format!("TF-IDF score: {:.3}", score)),
                        doc_type: doc.doc_type.clone(),
                        file_size: doc.size_bytes,
                        last_modified: None, // TODO: Add file modification time
                    })
                } else {
                    None
                }
            })
            .collect();

        // Apply filters
        results = self.apply_filters(results, &options.filters);

        // Sort by score and apply limit
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(limit) = options.limit {
            results.truncate(limit);
        }

        // Update performance metrics
        let search_time = start_time.elapsed();
        self.update_performance_metrics(search_time);

        Ok(results)
    }

    /// Perform enhanced regex search
    pub async fn regex_search(
        &mut self,
        pattern: &str,
        options: Option<SearchOptions>,
    ) -> RhemaResult<Vec<SearchResult>> {
        let start_time = Instant::now();
        let options = options.unwrap_or_else(|| SearchOptions {
            search_type: SearchType::Regex,
            limit: Some(self.config.default_limit),
            filters: Vec::new(),
            semantic_weight: None,
            keyword_weight: None,
            min_similarity: None,
            case_sensitive: false,
            fuzzy_matching: false,
            fuzzy_distance: None,
            search_fields: Vec::new(),
            field_boosts: HashMap::new(),
        });

        let regex = Regex::new(pattern)
            .map_err(|e| RhemaError::ConfigError(format!("Invalid regex pattern: {}", e)))?;

        let index = self
            .search_index
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Search index not built".to_string()))?;

        let mut results: Vec<SearchResult> = Vec::new();

        // Use parallel processing if enabled
        let documents: Vec<_> = if self.config.parallel_processing {
            index.documents.par_iter().collect()
        } else {
            index.documents.iter().collect()
        };

        for (doc_id, doc) in documents {
            if let Some(matches) = regex.find(&doc.content) {
                let score = self.calculate_regex_score(&doc.content, &regex, matches.len());
                let highlights = self.extract_regex_highlights(&doc.content, &regex);
                let match_positions = self.extract_regex_positions(&doc.content, &regex);

                results.push(SearchResult {
                    id: doc_id.clone(),
                    content: doc.content.clone(),
                    path: doc.path.clone(),
                    score,
                    metadata: doc.metadata.clone(),
                    search_type: SearchType::Regex,
                    highlights,
                    match_positions,
                    relevance_explanation: Some(format!("Regex matches: {}", matches.len())),
                    doc_type: doc.doc_type.clone(),
                    file_size: doc.size_bytes,
                    last_modified: None,
                });
            }
        }

        // Apply filters
        results = self.apply_filters(results, &options.filters);

        // Sort by score and apply limit
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(limit) = options.limit {
            results.truncate(limit);
        }

        // Update performance metrics
        let search_time = start_time.elapsed();
        self.update_performance_metrics(search_time);

        Ok(results)
    }

    /// Calculate regex search score
    fn calculate_regex_score(&self, content: &str, _regex: &Regex, match_count: usize) -> f64 {
        let content_length = content.len() as f64;
        let match_density = match_count as f64 / content_length;
        let base_score = match_count as f64;

        // Boost score for shorter content (more relevant matches)
        let length_boost = 1.0 / (1.0 + content_length / 1000.0);

        base_score * (1.0 + match_density) * length_boost
    }

    /// Extract regex highlights
    fn extract_regex_highlights(&self, content: &str, regex: &Regex) -> Vec<String> {
        let mut highlights = Vec::new();

        for cap in regex.find_iter(content) {
            let start = cap.start().saturating_sub(50);
            let end = (cap.end() + 50).min(content.len());
            let snippet = &content[start..end];
            highlights.push(snippet.to_string());
        }

        highlights
    }

    /// Extract regex match positions
    fn extract_regex_positions(&self, content: &str, regex: &Regex) -> Vec<MatchPosition> {
        let mut positions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for cap in regex.find_iter(content) {
            let start = cap.start();
            let end = cap.end();

            // Calculate line and column
            let mut _line_num = 1;
            let mut char_count = 0;

            for (line_idx, line) in lines.iter().enumerate() {
                if char_count + line.len() >= start {
                    let column = start - char_count;
                    positions.push(MatchPosition {
                        start,
                        end,
                        line: line_idx + 1,
                        column,
                        text: cap.as_str().to_string(),
                    });
                    break;
                }
                char_count += line.len() + 1; // +1 for newline
                _line_num += 1;
            }
        }

        positions
    }

    /// Find match positions in content
    fn find_match_positions(&self, content: &str, query: &str) -> Vec<MatchPosition> {
        let mut positions = Vec::new();
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();
        let lines: Vec<&str> = content.lines().collect();

        let mut offset = 0;
        while let Some(pos) = content_lower[offset..].find(&query_lower) {
            let start = offset + pos;
            let end = start + query.len();

            // Calculate line and column
            let mut char_count = 0;
            for (line_idx, line) in lines.iter().enumerate() {
                if char_count + line.len() >= start {
                    let column = start - char_count;
                    positions.push(MatchPosition {
                        start,
                        end,
                        line: line_idx + 1,
                        column,
                        text: query.to_string(),
                    });
                    break;
                }
                char_count += line.len() + 1; // +1 for newline
            }

            offset = end;
        }

        positions
    }

    /// Apply search filters
    fn apply_filters(
        &self,
        mut results: Vec<SearchResult>,
        filters: &[SearchFilter],
    ) -> Vec<SearchResult> {
        for filter in filters {
            results.retain(|result| self.matches_filter(result, filter));
        }
        results
    }

    /// Check if a result matches a filter
    fn matches_filter(&self, result: &SearchResult, filter: &SearchFilter) -> bool {
        match filter {
            SearchFilter::FileType(file_type) => {
                result.path.ends_with(file_type) || result.path.contains(file_type)
            }
            SearchFilter::Path(path_pattern) => {
                if let Ok(pattern) = Pattern::new(path_pattern) {
                    pattern.matches(&result.path)
                } else {
                    result.path.contains(path_pattern)
                }
            }
            SearchFilter::DateRange(start, end) => {
                if let Some(last_modified) = result.last_modified {
                    last_modified >= *start && last_modified <= *end
                } else {
                    true // Include if no date available
                }
            }
            SearchFilter::Scope(scope_name) => result.id.starts_with(&format!("{}:", scope_name)),
            SearchFilter::SizeRange(min_size, max_size) => {
                result.file_size >= *min_size && result.file_size <= *max_size
            }
            SearchFilter::Language(lang) => {
                if let Some(doc_lang) = &result.metadata.get("language").and_then(|v| v.as_str()) {
                    doc_lang == lang
                } else {
                    false
                }
            }
            SearchFilter::Custom(key, value) => result.metadata.get(key) == Some(value),
        }
    }

    /// Highlight matches in content
    fn highlight_matches(&self, content: &str, query: &str) -> Vec<String> {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();
        let mut highlights = Vec::new();

        if let Some(pos) = content_lower.find(&query_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            let snippet = &content[start..end];
            highlights.push(snippet.to_string());
        }

        highlights
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self, search_time: std::time::Duration) {
        self.performance_metrics.total_searches += 1;
        self.performance_metrics.total_search_time_ms += search_time.as_millis() as u64;
        self.performance_metrics.avg_search_time_ms = self.performance_metrics.total_search_time_ms
            as f64
            / self.performance_metrics.total_searches as f64;
        self.performance_metrics.last_updated = Utc::now();
    }

    /// Calculate index size in bytes
    fn calculate_index_size(&self) -> usize {
        if let Some(index) = &self.search_index {
            let mut size = 0;

            // Calculate size of documents
            for doc in index.documents.values() {
                size += doc.content.len();
                size += doc.path.len();
                size += std::mem::size_of_val(&doc.metadata);
            }

            // Calculate size of inverted index
            for (term, doc_ids) in &index.inverted_index {
                size += term.len();
                size += doc_ids.len() * std::mem::size_of::<String>();
            }

            size
        } else {
            0
        }
    }

    /// Get search statistics
    pub fn get_stats(&self) -> HashMap<String, Value> {
        let mut stats = HashMap::new();

        if let Some(index) = &self.search_index {
            stats.insert(
                "total_documents".to_string(),
                Value::Number(index.total_documents.into()),
            );
            stats.insert(
                "total_terms".to_string(),
                Value::Number(index.inverted_index.len().into()),
            );
            stats.insert(
                "index_size_bytes".to_string(),
                Value::Number(self.performance_metrics.index_size_bytes.into()),
            );
        }

        stats.insert(
            "full_text_enabled".to_string(),
            Value::Bool(self.config.full_text_enabled),
        );
        stats.insert(
            "semantic_enabled".to_string(),
            Value::Bool(self.config.semantic_enabled),
        );
        stats.insert(
            "hybrid_enabled".to_string(),
            Value::Bool(self.config.hybrid_enabled),
        );
        stats.insert(
            "regex_enabled".to_string(),
            Value::Bool(self.config.regex_enabled),
        );
        stats.insert(
            "parallel_processing".to_string(),
            Value::Bool(self.config.parallel_processing),
        );

        // Performance metrics
        stats.insert(
            "total_searches".to_string(),
            Value::Number(self.performance_metrics.total_searches.into()),
        );
        stats.insert(
            "avg_search_time_ms".to_string(),
            Value::Number(serde_yaml::Number::from(
                self.performance_metrics.avg_search_time_ms,
            )),
        );
        stats.insert(
            "cache_hit_rate".to_string(),
            Value::Number(serde_yaml::Number::from(
                self.performance_metrics.cache_hit_rate,
            )),
        );

        stats
    }

    /// Get search suggestions
    pub async fn get_suggestions(&self, query: &str) -> RhemaResult<Vec<SearchSuggestion>> {
        let index = self
            .search_index
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Search index not built".to_string()))?;

        let mut suggestions: Vec<SearchSuggestion> = Vec::new();
        let query_lower = query.to_lowercase();

        // Find matching terms from index
        for term in index.inverted_index.keys() {
            if term.starts_with(&query_lower) && term != &query_lower {
                suggestions.push(SearchSuggestion {
                    text: term.clone(),
                    score: 1.0 / (term.len() as f64), // Shorter terms get higher scores
                    suggestion_type: SuggestionType::QueryCompletion,
                    metadata: HashMap::new(),
                });
            }
        }

        // Sort by score and limit results
        suggestions.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(10);

        Ok(suggestions)
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}
