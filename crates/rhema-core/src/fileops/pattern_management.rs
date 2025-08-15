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
    fileops::{get_or_create_patterns_file, read_yaml_file, write_yaml_file},
    schema::knowledge::{
        PatternEntry, PatternMaturity, PatternUsage, PatternUsageStats, Patterns, ReviewStatus,
    },
    RhemaError, RhemaResult,
};
use chrono::Utc;
use serde_json;
use serde_yaml;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

/// Add a new enhanced pattern entry with all advanced features
pub fn add_enhanced_pattern_entry(
    scope_path: &Path,
    name: String,
    description: String,
    pattern_type: String,
    usage: PatternUsage,
    effectiveness: Option<u8>,
    examples: Option<String>,
    anti_patterns: Option<String>,
    category: Option<String>,
    maturity: Option<PatternMaturity>,
    complexity: Option<u8>,
    maintenance_effort: Option<u8>,
    performance_impact: Option<u8>,
    security_implications: Option<String>,
    testing_requirements: Option<String>,
    documentation_requirements: Option<String>,
    dependencies: Option<String>,
    applicable_contexts: Option<String>,
    version: Option<String>,
    author: Option<String>,
    review_status: Option<ReviewStatus>,
) -> RhemaResult<String> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let pattern_entry = PatternEntry {
        id: id.clone(),
        name,
        description,
        pattern_type,
        usage,
        effectiveness,
        examples: examples.map(|e| e.split(',').map(|s| s.trim().to_string()).collect()),
        anti_patterns: anti_patterns.map(|a| a.split(',').map(|s| s.trim().to_string()).collect()),
        related_patterns: None,
        category,
        maturity,
        complexity,
        maintenance_effort,
        performance_impact,
        security_implications: security_implications
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        testing_requirements: testing_requirements
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        documentation_requirements: documentation_requirements
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        dependencies: dependencies.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        applicable_contexts: applicable_contexts
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        version,
        author,
        review_status,
        last_reviewed: None,
        reviewers: None,
        usage_stats: Some(PatternUsageStats {
            usage_count: 0,
            success_count: 0,
            success_rate: 0.0,
            avg_implementation_time: None,
            last_used: None,
            common_use_cases: None,
            reported_issues: None,
            satisfaction_rating: None,
        }),
        created_at: now,
        updated_at: None,
        custom: HashMap::new(),
    };

    patterns.patterns.push(pattern_entry);
    write_yaml_file(&patterns_file, &patterns)?;

    Ok(id)
}

/// Get enhanced pattern entries with advanced filtering
pub fn get_enhanced_pattern_entries(
    scope_path: &Path,
    pattern_type: Option<String>,
    usage: Option<PatternUsage>,
    min_effectiveness: Option<u8>,
    category: Option<String>,
    maturity: Option<PatternMaturity>,
    review_status: Option<ReviewStatus>,
) -> RhemaResult<Vec<PatternEntry>> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let patterns: Patterns = read_yaml_file(&patterns_file)?;

    let mut filtered_patterns = patterns.patterns;

    if let Some(pattern_type) = pattern_type {
        filtered_patterns.retain(|pattern| pattern.pattern_type == pattern_type);
    }

    if let Some(usage) = usage {
        filtered_patterns.retain(|pattern| pattern.usage == usage);
    }

    if let Some(min_effectiveness) = min_effectiveness {
        filtered_patterns.retain(|pattern| {
            pattern
                .effectiveness
                .map_or(false, |eff| eff >= min_effectiveness)
        });
    }

    if let Some(category) = category {
        filtered_patterns
            .retain(|pattern| pattern.category.as_ref().map_or(false, |c| c == &category));
    }

    if let Some(maturity) = maturity {
        filtered_patterns
            .retain(|pattern| pattern.maturity.as_ref().map_or(false, |m| m == &maturity));
    }

    if let Some(review_status) = review_status {
        filtered_patterns.retain(|pattern| {
            pattern
                .review_status
                .as_ref()
                .map_or(false, |r| r == &review_status)
        });
    }

    Ok(filtered_patterns)
}

/// Update an enhanced pattern entry
pub fn update_enhanced_pattern_entry(
    scope_path: &Path,
    id: &str,
    name: Option<String>,
    description: Option<String>,
    pattern_type: Option<String>,
    usage: Option<PatternUsage>,
    effectiveness: Option<u8>,
    examples: Option<String>,
    anti_patterns: Option<String>,
    category: Option<String>,
    maturity: Option<PatternMaturity>,
    complexity: Option<u8>,
    maintenance_effort: Option<u8>,
    performance_impact: Option<u8>,
    security_implications: Option<String>,
    testing_requirements: Option<String>,
    documentation_requirements: Option<String>,
    dependencies: Option<String>,
    applicable_contexts: Option<String>,
    version: Option<String>,
    author: Option<String>,
    review_status: Option<ReviewStatus>,
) -> RhemaResult<()> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;

    let pattern = patterns
        .patterns
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Pattern with ID {} not found", id)))?;

    if let Some(name) = name {
        pattern.name = name;
    }
    if let Some(description) = description {
        pattern.description = description;
    }
    if let Some(pattern_type) = pattern_type {
        pattern.pattern_type = pattern_type;
    }
    if let Some(usage) = usage {
        pattern.usage = usage;
    }
    if let Some(effectiveness) = effectiveness {
        pattern.effectiveness = Some(effectiveness);
    }
    if let Some(examples) = examples {
        pattern.examples = Some(examples.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Some(anti_patterns) = anti_patterns {
        pattern.anti_patterns = Some(
            anti_patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(category) = category {
        pattern.category = Some(category);
    }
    if let Some(maturity) = maturity {
        pattern.maturity = Some(maturity);
    }
    if let Some(complexity) = complexity {
        pattern.complexity = Some(complexity);
    }
    if let Some(maintenance_effort) = maintenance_effort {
        pattern.maintenance_effort = Some(maintenance_effort);
    }
    if let Some(performance_impact) = performance_impact {
        pattern.performance_impact = Some(performance_impact);
    }
    if let Some(security_implications) = security_implications {
        pattern.security_implications = Some(
            security_implications
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(testing_requirements) = testing_requirements {
        pattern.testing_requirements = Some(
            testing_requirements
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(documentation_requirements) = documentation_requirements {
        pattern.documentation_requirements = Some(
            documentation_requirements
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(dependencies) = dependencies {
        pattern.dependencies = Some(
            dependencies
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(applicable_contexts) = applicable_contexts {
        pattern.applicable_contexts = Some(
            applicable_contexts
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(version) = version {
        pattern.version = Some(version);
    }
    if let Some(author) = author {
        pattern.author = Some(author);
    }
    if let Some(review_status) = review_status {
        pattern.review_status = Some(review_status);
        pattern.last_reviewed = Some(Utc::now());
    }

    pattern.updated_at = Some(Utc::now());
    write_yaml_file(&patterns_file, &patterns)?;

    Ok(())
}

/// Review a pattern
pub fn review_pattern(
    scope_path: &Path,
    id: &str,
    status: ReviewStatus,
    comments: Option<String>,
    reviewer: Option<String>,
) -> RhemaResult<()> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;

    let pattern = patterns
        .patterns
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Pattern with ID {} not found", id)))?;

    pattern.review_status = Some(status);
    pattern.last_reviewed = Some(Utc::now());

    if let Some(reviewer) = reviewer {
        let mut reviewers = pattern.reviewers.clone().unwrap_or_default();
        reviewers.push(reviewer);
        pattern.reviewers = Some(reviewers);
    }

    if let Some(comments) = comments {
        pattern.custom.insert(
            "review_comments".to_string(),
            serde_yaml::Value::String(comments),
        );
    }

    pattern.updated_at = Some(Utc::now());
    write_yaml_file(&patterns_file, &patterns)?;

    Ok(())
}

/// Search patterns with advanced query capabilities
pub fn search_patterns(
    scope_path: &Path,
    query: &str,
    fields: Option<Vec<String>>,
    case_sensitive: bool,
    use_regex: bool,
) -> RhemaResult<Vec<PatternEntry>> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let patterns: Patterns = read_yaml_file(&patterns_file)?;

    let search_fields = fields.unwrap_or_else(|| {
        vec![
            "name".to_string(),
            "description".to_string(),
            "pattern_type".to_string(),
            "category".to_string(),
        ]
    });

    let query_lower = if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    };

    let filtered_patterns: Vec<PatternEntry> = patterns
        .patterns
        .into_iter()
        .filter(|pattern| {
            search_fields.iter().any(|field| {
                let examples_str = pattern.examples.as_ref().map(|e| e.join(", "));
                let anti_patterns_str = pattern.anti_patterns.as_ref().map(|a| a.join(", "));

                let field_value = match field.as_str() {
                    "name" => Some(&pattern.name),
                    "description" => Some(&pattern.description),
                    "pattern_type" => Some(&pattern.pattern_type),
                    "category" => pattern.category.as_ref(),
                    "examples" => examples_str.as_ref(),
                    "anti_patterns" => anti_patterns_str.as_ref(),
                    _ => None,
                };

                if let Some(value) = field_value {
                    if use_regex {
                        // Simple regex matching - in production, use a proper regex library
                        value.contains(&query)
                    } else {
                        let value_lower = if case_sensitive {
                            value.to_string()
                        } else {
                            value.to_lowercase()
                        };
                        value_lower.contains(&query_lower)
                    }
                } else {
                    false
                }
            })
        })
        .collect();

    Ok(filtered_patterns)
}

/// Analyze pattern usage and generate statistics
pub fn analyze_pattern_usage(
    scope_path: &Path,
    pattern_id: Option<&str>,
) -> RhemaResult<PatternAnalysisReport> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let patterns: Patterns = read_yaml_file(&patterns_file)?;

    let mut report = PatternAnalysisReport {
        total_patterns: patterns.patterns.len(),
        patterns_by_category: HashMap::new(),
        patterns_by_maturity: HashMap::new(),
        patterns_by_usage: HashMap::new(),
        average_effectiveness: 0.0,
        patterns_needing_review: 0,
        top_patterns: Vec::new(),
        recommendations: Vec::new(),
    };

    let mut total_effectiveness = 0.0;
    let mut effectiveness_count = 0;

    for pattern in &patterns.patterns {
        // Filter by pattern ID if specified
        if let Some(id) = pattern_id {
            if pattern.id != id {
                continue;
            }
        }

        // Category statistics
        if let Some(category) = &pattern.category {
            *report
                .patterns_by_category
                .entry(category.clone())
                .or_insert(0) += 1;
        }

        // Maturity statistics
        if let Some(maturity) = &pattern.maturity {
            *report
                .patterns_by_maturity
                .entry(format!("{:?}", maturity))
                .or_insert(0) += 1;
        }

        // Usage statistics
        let usage_str = format!("{:?}", pattern.usage);
        *report.patterns_by_usage.entry(usage_str).or_insert(0) += 1;

        // Effectiveness statistics
        if let Some(effectiveness) = pattern.effectiveness {
            total_effectiveness += effectiveness as f32;
            effectiveness_count += 1;
        }

        // Review status
        if pattern.review_status.is_none()
            || pattern.review_status.as_ref() == Some(&ReviewStatus::Pending)
        {
            report.patterns_needing_review += 1;
        }

        // Top patterns by effectiveness
        if let Some(effectiveness) = pattern.effectiveness {
            if effectiveness >= 8 {
                report.top_patterns.push(PatternSummary {
                    id: pattern.id.clone(),
                    name: pattern.name.clone(),
                    effectiveness,
                    category: pattern.category.clone(),
                    maturity: pattern.maturity.clone(),
                });
            }
        }
    }

    if effectiveness_count > 0 {
        report.average_effectiveness = total_effectiveness / effectiveness_count as f32;
    }

    // Generate recommendations
    if report.patterns_needing_review > 0 {
        report.recommendations.push(format!(
            "{} patterns need review. Consider reviewing patterns with pending or missing review status.",
            report.patterns_needing_review
        ));
    }

    if report.average_effectiveness < 6.0 {
        report.recommendations.push(
            "Average pattern effectiveness is low. Consider improving pattern documentation and examples."
                .to_string(),
        );
    }

    Ok(report)
}

/// Pattern analysis report
#[derive(Debug, Clone, serde::Serialize)]
pub struct PatternAnalysisReport {
    pub total_patterns: usize,
    pub patterns_by_category: HashMap<String, usize>,
    pub patterns_by_maturity: HashMap<String, usize>,
    pub patterns_by_usage: HashMap<String, usize>,
    pub average_effectiveness: f32,
    pub patterns_needing_review: usize,
    pub top_patterns: Vec<PatternSummary>,
    pub recommendations: Vec<String>,
}

/// Pattern summary for analysis
#[derive(Debug, Clone, serde::Serialize)]
pub struct PatternSummary {
    pub id: String,
    pub name: String,
    pub effectiveness: u8,
    pub category: Option<String>,
    pub maturity: Option<PatternMaturity>,
}

// Export and Import Functions

/// Export patterns to various formats
pub fn export_patterns(
    scope_path: &Path,
    format: ExportFormat,
    pattern_type: Option<String>,
    category: Option<String>,
    include_templates: bool,
    include_stats: bool,
) -> RhemaResult<String> {
    let patterns = get_enhanced_pattern_entries(
        scope_path,
        pattern_type,
        None, // usage
        None, // min_effectiveness
        category,
        None, // maturity
        None, // review_status
    )?;

    let mut export_data = serde_json::Map::new();
    export_data.insert(
        "version".to_string(),
        serde_json::Value::String("1.0.0".to_string()),
    );
    export_data.insert(
        "exported_at".to_string(),
        serde_json::Value::String(Utc::now().to_rfc3339()),
    );
    export_data.insert("patterns".to_string(), serde_json::to_value(&patterns)?);

    if include_templates {
        let templates = crate::fileops::template_management::get_pattern_templates(scope_path)?;
        export_data.insert("templates".to_string(), serde_json::to_value(templates)?);
    }

    if include_stats {
        let analysis = analyze_pattern_usage(scope_path, None)?;
        export_data.insert("statistics".to_string(), serde_json::to_value(analysis)?);
    }

    let json_data = serde_json::to_string_pretty(&export_data)?;

    match format {
        ExportFormat::Json => Ok(json_data),
        ExportFormat::Yaml => {
            let yaml_data = serde_yaml::to_string(&export_data)?;
            Ok(yaml_data)
        }
        ExportFormat::Csv => {
            // Simple CSV export of pattern names and basic info
            let mut csv_data =
                String::from("id,name,description,pattern_type,category,effectiveness\n");
            for pattern in patterns {
                csv_data.push_str(&format!(
                    "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                    pattern.id,
                    pattern.name.replace("\"", "\"\""),
                    pattern.description.replace("\"", "\"\""),
                    pattern.pattern_type,
                    pattern.category.unwrap_or_default(),
                    pattern.effectiveness.unwrap_or(0)
                ));
            }
            Ok(csv_data)
        }
    }
}

/// Import patterns from various formats
pub fn import_patterns(
    scope_path: &Path,
    input_file: &str,
    format: ImportFormat,
    strategy: ImportStrategy,
) -> RhemaResult<ImportResult> {
    let file_content = std::fs::read_to_string(input_file)?;
    let import_data: serde_json::Value = match format {
        ImportFormat::Auto => {
            if input_file.ends_with(".json") {
                serde_json::from_str(&file_content)?
            } else if input_file.ends_with(".yaml") || input_file.ends_with(".yml") {
                serde_yaml::from_str(&file_content)?
            } else {
                return Err(RhemaError::ConfigError(
                    "Unsupported file format".to_string(),
                ));
            }
        }
        ImportFormat::Json => serde_json::from_str(&file_content)?,
        ImportFormat::Yaml => serde_yaml::from_str(&file_content)?,
        ImportFormat::Csv => {
            // Parse CSV data and convert to JSON format
            let csv_data = parse_csv_to_patterns(&file_content)?;
            serde_json::to_value(csv_data)?
        }
    };

    let mut result = ImportResult {
        patterns_imported: 0,
        templates_imported: 0,
        skipped_count: 0,
        errors: Vec::new(),
    };

    if let Some(patterns_value) = import_data.get("patterns") {
        if let Ok(patterns) = serde_json::from_value::<Vec<PatternEntry>>(patterns_value.clone()) {
            for pattern in patterns {
                match strategy {
                    ImportStrategy::Overwrite => {
                        // Always import, overwriting existing
                        let pattern_name = pattern.name.clone();
                        if let Err(e) = update_enhanced_pattern_entry(
                            scope_path,
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
                            pattern.applicable_contexts.as_ref().map(|a| a.join(", ")),
                            pattern.version.clone(),
                            pattern.author.clone(),
                            pattern.review_status.clone(),
                        ) {
                            result
                                .errors
                                .push(format!("Failed to import pattern {}: {}", pattern_name, e));
                        } else {
                            result.patterns_imported += 1;
                        }
                    }
                    ImportStrategy::Merge => {
                        // Import only if doesn't exist
                        let existing_patterns = get_enhanced_pattern_entries(
                            scope_path, None, None, None, None, None, None,
                        )?;
                        if !existing_patterns.iter().any(|p| p.id == pattern.id) {
                            let pattern_name = pattern.name.clone();
                            if let Err(e) = add_enhanced_pattern_entry(
                                scope_path,
                                pattern.name,
                                pattern.description,
                                pattern.pattern_type,
                                pattern.usage,
                                pattern.effectiveness,
                                pattern.examples.as_ref().map(|e| e.join(", ")),
                                pattern.anti_patterns.as_ref().map(|a| a.join(", ")),
                                pattern.category,
                                pattern.maturity,
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
                                pattern.applicable_contexts.as_ref().map(|a| a.join(", ")),
                                pattern.version,
                                pattern.author,
                                pattern.review_status,
                            ) {
                                result.errors.push(format!(
                                    "Failed to import pattern {}: {}",
                                    pattern_name, e
                                ));
                            } else {
                                result.patterns_imported += 1;
                            }
                        } else {
                            result.skipped_count += 1;
                        }
                    }
                    ImportStrategy::Skip => {
                        // Skip if exists
                        let existing_patterns = get_enhanced_pattern_entries(
                            scope_path, None, None, None, None, None, None,
                        )?;
                        if !existing_patterns.iter().any(|p| p.id == pattern.id) {
                            let pattern_name = pattern.name.clone();
                            if let Err(e) = add_enhanced_pattern_entry(
                                scope_path,
                                pattern.name,
                                pattern.description,
                                pattern.pattern_type,
                                pattern.usage,
                                pattern.effectiveness,
                                pattern.examples.as_ref().map(|e| e.join(", ")),
                                pattern.anti_patterns.as_ref().map(|a| a.join(", ")),
                                pattern.category,
                                pattern.maturity,
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
                                pattern.applicable_contexts.as_ref().map(|a| a.join(", ")),
                                pattern.version,
                                pattern.author,
                                pattern.review_status,
                            ) {
                                result.errors.push(format!(
                                    "Failed to import pattern {}: {}",
                                    pattern_name, e
                                ));
                            } else {
                                result.patterns_imported += 1;
                            }
                        } else {
                            result.skipped_count += 1;
                        }
                    }
                }
            }
        }
    }

    // Import templates if available
    if let Some(templates_value) = import_data.get("templates") {
        result
            .errors
            .push("Template import is not fully implemented yet".to_string());
    }

    Ok(result)
}

/// Validate import file before importing
pub fn validate_import_file(
    input_file: &str,
    format: ImportFormat,
) -> RhemaResult<ValidationResult> {
    let file_content = std::fs::read_to_string(input_file)?;
    let mut result = ValidationResult {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        details: HashMap::new(),
    };

    // Basic file format validation
    match format {
        ImportFormat::Auto => {
            if !input_file.ends_with(".json")
                && !input_file.ends_with(".yaml")
                && !input_file.ends_with(".yml")
            {
                result.is_valid = false;
                result.errors.push("Unsupported file format".to_string());
            }
        }
        ImportFormat::Json => {
            if let Err(e) = serde_json::from_str::<serde_json::Value>(&file_content) {
                result.is_valid = false;
                result.errors.push(format!("Invalid JSON: {}", e));
            }
        }
        ImportFormat::Yaml => {
            if let Err(e) = serde_yaml::from_str::<serde_yaml::Value>(&file_content) {
                result.is_valid = false;
                result.errors.push(format!("Invalid YAML: {}", e));
            }
        }
        ImportFormat::Csv => {
            // Validate CSV format and content
            validate_csv_format(&file_content, &mut result)?;
        }
    }

    Ok(result)
}

/// Dry run import to see what would be imported
pub fn dry_run_import(
    scope_path: &Path,
    input_file: &str,
    format: ImportFormat,
    strategy: ImportStrategy,
) -> RhemaResult<ImportSummary> {
    let file_content = std::fs::read_to_string(input_file)?;
    let import_data: serde_json::Value = match format {
        ImportFormat::Auto => {
            if input_file.ends_with(".json") {
                serde_json::from_str(&file_content)?
            } else if input_file.ends_with(".yaml") || input_file.ends_with(".yml") {
                serde_yaml::from_str(&file_content)?
            } else {
                return Err(RhemaError::ConfigError(
                    "Unsupported file format".to_string(),
                ));
            }
        }
        ImportFormat::Json => serde_json::from_str(&file_content)?,
        ImportFormat::Yaml => serde_yaml::from_str(&file_content)?,
        ImportFormat::Csv => {
            return Err(RhemaError::ConfigError(
                "CSV import not yet implemented".to_string(),
            ));
        }
    };

    let mut summary = ImportSummary {
        patterns_count: 0,
        templates_count: 0,
        conflicts_count: 0,
        warnings_count: 0,
    };

    if let Some(patterns_value) = import_data.get("patterns") {
        if let Ok(patterns) = serde_json::from_value::<Vec<PatternEntry>>(patterns_value.clone()) {
            summary.patterns_count = patterns.len();

            let existing_patterns =
                get_enhanced_pattern_entries(scope_path, None, None, None, None, None, None)?;

            for pattern in patterns {
                if existing_patterns.iter().any(|p| p.id == pattern.id) {
                    summary.conflicts_count += 1;
                }
            }
        }
    }

    if import_data.get("templates").is_some() {
        summary.warnings_count += 1; // Template import not implemented
    }

    Ok(summary)
}

/// Backup existing patterns
pub fn backup_patterns(scope_path: &Path, backup_path: &str) -> RhemaResult<()> {
    let patterns = get_enhanced_pattern_entries(scope_path, None, None, None, None, None, None)?;

    let backup_data = serde_json::to_string_pretty(&patterns)?;
    std::fs::write(backup_path, backup_data)?;

    Ok(())
}

/// Validate patterns
pub fn validate_patterns(
    scope_path: &Path,
    pattern_id: Option<&str>,
    level: ValidationLevel,
    auto_fix: bool,
) -> RhemaResult<PatternValidationResult> {
    let patterns = if let Some(id) = pattern_id {
        let all_patterns =
            get_enhanced_pattern_entries(scope_path, None, None, None, None, None, None)?;
        all_patterns.into_iter().filter(|p| p.id == id).collect()
    } else {
        get_enhanced_pattern_entries(scope_path, None, None, None, None, None, None)?
    };

    let mut result = PatternValidationResult {
        total_patterns: patterns.len(),
        valid_count: 0,
        invalid_count: 0,
        warnings_count: 0,
        auto_fixed_count: 0,
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    for pattern in patterns {
        let mut pattern_valid = true;
        let mut pattern_warnings = Vec::new();

        // Basic validation
        if pattern.name.is_empty() {
            result
                .errors
                .push(format!("Pattern {}: Name is empty", pattern.id));
            pattern_valid = false;
        }

        if pattern.description.is_empty() {
            result
                .warnings
                .push(format!("Pattern {}: Description is empty", pattern.id));
            pattern_warnings.push("Empty description".to_string());
        }

        if pattern.pattern_type.is_empty() {
            result
                .errors
                .push(format!("Pattern {}: Pattern type is empty", pattern.id));
            pattern_valid = false;
        }

        // Effectiveness validation
        if let Some(effectiveness) = pattern.effectiveness {
            if effectiveness > 10 {
                result.errors.push(format!(
                    "Pattern {}: Effectiveness rating exceeds 10",
                    pattern.id
                ));
                pattern_valid = false;
            }
        }

        // Complexity validation
        if let Some(complexity) = pattern.complexity {
            if complexity > 10 {
                result.errors.push(format!(
                    "Pattern {}: Complexity rating exceeds 10",
                    pattern.id
                ));
                pattern_valid = false;
            }
        }

        // Advanced validation for strict and comprehensive levels
        if matches!(
            level,
            ValidationLevel::Strict | ValidationLevel::Comprehensive
        ) {
            if pattern.examples.is_none() || pattern.examples.as_ref().unwrap().is_empty() {
                result
                    .warnings
                    .push(format!("Pattern {}: No examples provided", pattern.id));
                pattern_warnings.push("No examples".to_string());
            }

            if pattern.category.is_none() {
                result
                    .warnings
                    .push(format!("Pattern {}: No category specified", pattern.id));
                pattern_warnings.push("No category".to_string());
            }

            if pattern.maturity.is_none() {
                result.warnings.push(format!(
                    "Pattern {}: No maturity level specified",
                    pattern.id
                ));
                pattern_warnings.push("No maturity level".to_string());
            }
        }

        // Comprehensive validation
        if matches!(level, ValidationLevel::Comprehensive) {
            if pattern.author.is_none() {
                result
                    .warnings
                    .push(format!("Pattern {}: No author specified", pattern.id));
                pattern_warnings.push("No author".to_string());
            }

            if pattern.version.is_none() {
                result
                    .warnings
                    .push(format!("Pattern {}: No version specified", pattern.id));
                pattern_warnings.push("No version".to_string());
            }
        }

        if pattern_valid {
            result.valid_count += 1;
        } else {
            result.invalid_count += 1;
        }

        result.warnings_count += pattern_warnings.len();
    }

    Ok(result)
}

/// Generate pattern documentation
pub fn generate_pattern_documentation(
    scope_path: &Path,
    output_dir: Option<&Path>,
    format: DocFormat,
    include_examples: bool,
    include_templates: bool,
    include_stats: bool,
    generate_index: bool,
) -> RhemaResult<DocumentationResult> {
    let patterns = get_enhanced_pattern_entries(scope_path, None, None, None, None, None, None)?;

    let output_path = output_dir.unwrap_or_else(|| Path::new("pattern_docs"));
    std::fs::create_dir_all(output_path)?;

    let mut result = DocumentationResult {
        output_dir: output_path.to_string_lossy().to_string(),
        files_generated: 0,
        patterns_documented: patterns.len(),
        templates_documented: 0,
        index_file: None,
        errors: Vec::new(),
    };

    match format {
        DocFormat::Markdown => {
            // Generate individual pattern files
            for pattern in &patterns {
                let filename = format!("{}.md", pattern.id);
                let filepath = output_path.join(&filename);

                let mut content = String::new();
                content.push_str(&format!("# {}\n\n", pattern.name));
                content.push_str(&format!("**ID**: {}\n\n", pattern.id));
                content.push_str(&format!("**Description**: {}\n\n", pattern.description));
                content.push_str(&format!("**Type**: {}\n\n", pattern.pattern_type));
                content.push_str(&format!("**Usage**: {:?}\n\n", pattern.usage));

                if let Some(category) = &pattern.category {
                    content.push_str(&format!("**Category**: {}\n\n", category));
                }

                if let Some(effectiveness) = pattern.effectiveness {
                    content.push_str(&format!("**Effectiveness**: {}/10\n\n", effectiveness));
                }

                if let Some(maturity) = &pattern.maturity {
                    content.push_str(&format!("**Maturity**: {:?}\n\n", maturity));
                }

                if include_examples {
                    if let Some(examples) = &pattern.examples {
                        content.push_str("## Examples\n\n");
                        for example in examples {
                            content.push_str(&format!("- {}\n", example));
                        }
                        content.push_str("\n");
                    }
                }

                if let Some(author) = &pattern.author {
                    content.push_str(&format!("**Author**: {}\n\n", author));
                }

                if let Some(version) = &pattern.version {
                    content.push_str(&format!("**Version**: {}\n\n", version));
                }

                content.push_str(&format!(
                    "**Created**: {}\n",
                    pattern.created_at.format("%Y-%m-%d %H:%M:%S")
                ));

                std::fs::write(filepath, content)?;
                result.files_generated += 1;
            }

            // Generate index if requested
            if generate_index {
                let index_content = generate_markdown_index(&patterns);
                let index_path = output_path.join("README.md");
                std::fs::write(&index_path, index_content)?;
                result.index_file = Some("README.md".to_string());
                result.files_generated += 1;
            }
        }
        DocFormat::Html => {
            // Generate HTML documentation
            let html_content =
                generate_html_documentation(&patterns, include_examples, include_stats)?;
            let html_path = output_path.join("patterns.html");
            std::fs::write(&html_path, html_content)?;
            result.files_generated += 1;

            // Generate individual pattern HTML files
            for pattern in &patterns {
                let filename = format!("{}.html", pattern.id);
                let filepath = output_path.join(&filename);

                let pattern_html = generate_individual_pattern_html(pattern, include_examples)?;
                std::fs::write(filepath, pattern_html)?;
                result.files_generated += 1;
            }

            // Generate index if requested
            if generate_index {
                let index_html = generate_html_index(&patterns);
                let index_path = output_path.join("index.html");
                std::fs::write(&index_path, index_html)?;
                result.index_file = Some("index.html".to_string());
                result.files_generated += 1;
            }
        }
        DocFormat::Pdf => {
            // Generate PDF-compatible text format
            let pdf_content =
                generate_pdf_documentation(&patterns, include_examples, include_stats)?;
            let pdf_path = output_path.join("patterns.txt");
            std::fs::write(&pdf_path, pdf_content)?;
            result.files_generated += 1;

            // Note: This generates a text file that can be converted to PDF using external tools
            // like pandoc: pandoc patterns.txt -o patterns.pdf
            result.errors.push("PDF generation creates a text file. Use external tools like pandoc to convert to PDF.".to_string());
        }
    }

    Ok(result)
}

// Helper function to generate markdown index
fn generate_markdown_index(patterns: &[PatternEntry]) -> String {
    let mut content = String::new();
    content.push_str("# Pattern Documentation\n\n");
    content.push_str(&format!("Total Patterns: {}\n\n", patterns.len()));

    // Group by category
    let mut categories: HashMap<String, Vec<&PatternEntry>> = HashMap::new();
    for pattern in patterns {
        let category = pattern.category.as_deref().unwrap_or("Uncategorized");
        categories
            .entry(category.to_string())
            .or_default()
            .push(pattern);
    }

    for (category, category_patterns) in categories {
        content.push_str(&format!("## {}\n\n", category));
        for pattern in category_patterns {
            content.push_str(&format!(
                "- [{}]({}.md) - {}\n",
                pattern.name, pattern.id, pattern.description
            ));
        }
        content.push_str("\n");
    }

    content
}

/// Generate HTML documentation for all patterns
fn generate_html_documentation(
    patterns: &[PatternEntry],
    include_examples: bool,
    include_stats: bool,
) -> RhemaResult<String> {
    let mut html = String::new();

    // HTML header
    html.push_str(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pattern Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        .pattern { border: 1px solid #ddd; margin: 20px 0; padding: 20px; border-radius: 5px; }
        .pattern h2 { color: #333; margin-top: 0; }
        .pattern-meta { background: #f5f5f5; padding: 10px; border-radius: 3px; margin: 10px 0; }
        .pattern-meta span { margin-right: 20px; }
        .examples { background: #e8f4f8; padding: 15px; border-radius: 3px; margin: 10px 0; }
        .examples h3 { margin-top: 0; }
        .stats { background: #fff3cd; padding: 15px; border-radius: 3px; margin: 10px 0; }
        .category { color: #666; font-style: italic; }
        .effectiveness { font-weight: bold; }
        .high-effectiveness { color: #28a745; }
        .medium-effectiveness { color: #ffc107; }
        .low-effectiveness { color: #dc3545; }
    </style>
</head>
<body>
    <h1>Pattern Documentation</h1>
    <p>Total Patterns: "#,
    );
    html.push_str(&patterns.len().to_string());
    html.push_str("</p>\n");

    // Generate statistics if requested
    if include_stats {
        html.push_str(&generate_html_stats(patterns));
    }

    // Generate pattern entries
    for pattern in patterns {
        html.push_str(&format!(
            r#"
    <div class="pattern">
        <h2>{}</h2>
        <div class="pattern-meta">
            <span><strong>ID:</strong> {}</span>
            <span><strong>Type:</strong> {}</span>
            <span><strong>Usage:</strong> {:?}</span>"#,
            pattern.name, pattern.id, pattern.pattern_type, pattern.usage
        ));

        if let Some(category) = &pattern.category {
            html.push_str(&format!(
                r#"<span class="category"><strong>Category:</strong> {}</span>"#,
                category
            ));
        }

        if let Some(effectiveness) = pattern.effectiveness {
            let effectiveness_class = if effectiveness >= 8 {
                "high-effectiveness"
            } else if effectiveness >= 5 {
                "medium-effectiveness"
            } else {
                "low-effectiveness"
            };
            html.push_str(&format!(
                r#"<span class="effectiveness {}"><strong>Effectiveness:</strong> {}/10</span>"#,
                effectiveness_class, effectiveness
            ));
        }

        if let Some(maturity) = &pattern.maturity {
            html.push_str(&format!(
                r#"<span><strong>Maturity:</strong> {:?}</span>"#,
                maturity
            ));
        }

        html.push_str("</div>\n");
        html.push_str(&format!(
            "<p><strong>Description:</strong> {}</p>\n",
            pattern.description
        ));

        if include_examples {
            if let Some(examples) = &pattern.examples {
                html.push_str(
                    r#"<div class="examples">
            <h3>Examples</h3>
            <ul>"#,
                );
                for example in examples {
                    html.push_str(&format!("<li>{}</li>\n", example));
                }
                html.push_str("</ul></div>\n");
            }
        }

        if let Some(author) = &pattern.author {
            html.push_str(&format!("<p><strong>Author:</strong> {}</p>\n", author));
        }

        if let Some(version) = &pattern.version {
            html.push_str(&format!("<p><strong>Version:</strong> {}</p>\n", version));
        }

        html.push_str(&format!(
            "<p><strong>Created:</strong> {}</p>\n",
            pattern.created_at.format("%Y-%m-%d %H:%M:%S")
        ));

        html.push_str("</div>\n");
    }

    html.push_str(
        r#"
</body>
</html>"#,
    );

    Ok(html)
}

/// Generate HTML for individual pattern
fn generate_individual_pattern_html(
    pattern: &PatternEntry,
    include_examples: bool,
) -> RhemaResult<String> {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }}
        .pattern {{ border: 1px solid #ddd; margin: 20px 0; padding: 20px; border-radius: 5px; }}
        .pattern h1 {{ color: #333; margin-top: 0; }}
        .pattern-meta {{ background: #f5f5f5; padding: 10px; border-radius: 3px; margin: 10px 0; }}
        .pattern-meta span {{ margin-right: 20px; }}
        .examples {{ background: #e8f4f8; padding: 15px; border-radius: 3px; margin: 10px 0; }}
        .examples h3 {{ margin-top: 0; }}
        .back-link {{ margin-bottom: 20px; }}
        .back-link a {{ color: #007bff; text-decoration: none; }}
        .back-link a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="back-link">
        <a href="index.html">← Back to Pattern Index</a>
    </div>
    <div class="pattern">
        <h1>{}</h1>
        <div class="pattern-meta">
            <span><strong>ID:</strong> {}</span>
            <span><strong>Type:</strong> {}</span>
            <span><strong>Usage:</strong> {:?}</span>"#,
        pattern.name, pattern.name, pattern.id, pattern.pattern_type, pattern.usage
    ));

    if let Some(category) = &pattern.category {
        html.push_str(&format!(
            r#"<span><strong>Category:</strong> {}</span>"#,
            category
        ));
    }

    if let Some(effectiveness) = pattern.effectiveness {
        html.push_str(&format!(
            r#"<span><strong>Effectiveness:</strong> {}/10</span>"#,
            effectiveness
        ));
    }

    if let Some(maturity) = &pattern.maturity {
        html.push_str(&format!(
            r#"<span><strong>Maturity:</strong> {:?}</span>"#,
            maturity
        ));
    }

    html.push_str("</div>\n");
    html.push_str(&format!(
        "<p><strong>Description:</strong> {}</p>\n",
        pattern.description
    ));

    if include_examples {
        if let Some(examples) = &pattern.examples {
            html.push_str(
                r#"<div class="examples">
        <h3>Examples</h3>
        <ul>"#,
            );
            for example in examples {
                html.push_str(&format!("<li>{}</li>\n", example));
            }
            html.push_str("</ul></div>\n");
        }
    }

    if let Some(author) = &pattern.author {
        html.push_str(&format!("<p><strong>Author:</strong> {}</p>\n", author));
    }

    if let Some(version) = &pattern.version {
        html.push_str(&format!("<p><strong>Version:</strong> {}</p>\n", version));
    }

    html.push_str(&format!(
        "<p><strong>Created:</strong> {}</p>\n",
        pattern.created_at.format("%Y-%m-%d %H:%M:%S")
    ));

    html.push_str("</div>\n</body>\n</html>");

    Ok(html)
}

/// Generate HTML index page
fn generate_html_index(patterns: &[PatternEntry]) -> String {
    let mut html = String::new();

    html.push_str(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pattern Index</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        .category { margin: 30px 0; }
        .category h2 { color: #333; border-bottom: 2px solid #007bff; padding-bottom: 5px; }
        .pattern-link { margin: 10px 0; }
        .pattern-link a { color: #007bff; text-decoration: none; }
        .pattern-link a:hover { text-decoration: underline; }
        .pattern-description { color: #666; margin-left: 20px; }
    </style>
</head>
<body>
    <h1>Pattern Index</h1>
    <p>Total Patterns: "#,
    );
    html.push_str(&patterns.len().to_string());
    html.push_str("</p>\n");

    // Group by category
    let mut categories: HashMap<String, Vec<&PatternEntry>> = HashMap::new();
    for pattern in patterns {
        let category = pattern.category.as_deref().unwrap_or("Uncategorized");
        categories
            .entry(category.to_string())
            .or_default()
            .push(pattern);
    }

    for (category, category_patterns) in categories {
        html.push_str(&format!(
            r#"
    <div class="category">
        <h2>{}</h2>"#,
            category
        ));

        for pattern in category_patterns {
            html.push_str(&format!(
                r#"
        <div class="pattern-link">
            <a href="{}.html">{}</a>
            <div class="pattern-description">{}</div>
        </div>"#,
                pattern.id, pattern.name, pattern.description
            ));
        }

        html.push_str("\n    </div>\n");
    }

    html.push_str(
        r#"
</body>
</html>"#,
    );

    html
}

/// Generate HTML statistics section
fn generate_html_stats(patterns: &[PatternEntry]) -> String {
    let mut html = String::new();

    // Calculate statistics
    let mut categories = HashMap::new();
    let mut maturities = HashMap::new();
    let mut total_effectiveness = 0.0;
    let mut effectiveness_count = 0;

    for pattern in patterns {
        if let Some(category) = &pattern.category {
            *categories.entry(category.clone()).or_insert(0) += 1;
        }

        if let Some(maturity) = &pattern.maturity {
            *maturities.entry(format!("{:?}", maturity)).or_insert(0) += 1;
        }

        if let Some(effectiveness) = pattern.effectiveness {
            total_effectiveness += effectiveness as f32;
            effectiveness_count += 1;
        }
    }

    let avg_effectiveness = if effectiveness_count > 0 {
        total_effectiveness / effectiveness_count as f32
    } else {
        0.0
    };

    html.push_str(&format!(
        r#"
    <div class="stats">
        <h2>Statistics</h2>
        <p><strong>Average Effectiveness:</strong> {:.1}/10</p>
        <p><strong>Patterns by Category:</strong></p>
        <ul>"#,
        avg_effectiveness
    ));

    for (category, count) in categories {
        html.push_str(&format!("<li>{}: {}</li>\n", category, count));
    }

    html.push_str("</ul>\n");

    if !maturities.is_empty() {
        html.push_str("<p><strong>Patterns by Maturity:</strong></p>\n<ul>");
        for (maturity, count) in maturities {
            html.push_str(&format!("<li>{}: {}</li>\n", maturity, count));
        }
        html.push_str("</ul>\n");
    }

    html.push_str("</div>\n");

    html
}

/// Generate PDF-compatible documentation
fn generate_pdf_documentation(
    patterns: &[PatternEntry],
    include_examples: bool,
    include_stats: bool,
) -> RhemaResult<String> {
    let mut content = String::new();

    // Title page
    content.push_str("PATTERN DOCUMENTATION\n");
    content.push_str("====================\n\n");
    content.push_str(&format!("Total Patterns: {}\n", patterns.len()));
    content.push_str(&format!(
        "Generated: {}\n\n",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    ));

    // Table of contents
    content.push_str("TABLE OF CONTENTS\n");
    content.push_str("=================\n");
    for (i, pattern) in patterns.iter().enumerate() {
        content.push_str(&format!("{}. {}\n", i + 1, pattern.name));
    }
    content.push_str("\n\n");

    // Generate statistics if requested
    if include_stats {
        content.push_str("STATISTICS\n");
        content.push_str("==========\n");

        let mut categories = HashMap::new();
        let mut maturities = HashMap::new();
        let mut total_effectiveness = 0.0;
        let mut effectiveness_count = 0;

        for pattern in patterns {
            if let Some(category) = &pattern.category {
                *categories.entry(category.clone()).or_insert(0) += 1;
            }

            if let Some(maturity) = &pattern.maturity {
                *maturities.entry(format!("{:?}", maturity)).or_insert(0) += 1;
            }

            if let Some(effectiveness) = pattern.effectiveness {
                total_effectiveness += effectiveness as f32;
                effectiveness_count += 1;
            }
        }

        let avg_effectiveness = if effectiveness_count > 0 {
            total_effectiveness / effectiveness_count as f32
        } else {
            0.0
        };

        content.push_str(&format!(
            "Average Effectiveness: {:.1}/10\n\n",
            avg_effectiveness
        ));

        content.push_str("Patterns by Category:\n");
        for (category, count) in categories {
            content.push_str(&format!("  {}: {}\n", category, count));
        }
        content.push_str("\n");

        if !maturities.is_empty() {
            content.push_str("Patterns by Maturity:\n");
            for (maturity, count) in maturities {
                content.push_str(&format!("  {}: {}\n", maturity, count));
            }
            content.push_str("\n");
        }
    }

    // Generate pattern entries
    for (i, pattern) in patterns.iter().enumerate() {
        content.push_str(&format!("{}. {}\n", i + 1, pattern.name));
        content.push_str(&format!("{}\n", "=".repeat(pattern.name.len() + 3)));
        content.push_str(&format!("ID: {}\n", pattern.id));
        content.push_str(&format!("Type: {}\n", pattern.pattern_type));
        content.push_str(&format!("Usage: {:?}\n", pattern.usage));

        if let Some(category) = &pattern.category {
            content.push_str(&format!("Category: {}\n", category));
        }

        if let Some(effectiveness) = pattern.effectiveness {
            content.push_str(&format!("Effectiveness: {}/10\n", effectiveness));
        }

        if let Some(maturity) = &pattern.maturity {
            content.push_str(&format!("Maturity: {:?}\n", maturity));
        }

        content.push_str(&format!("Description: {}\n", pattern.description));

        if include_examples {
            if let Some(examples) = &pattern.examples {
                content.push_str("Examples:\n");
                for example in examples {
                    content.push_str(&format!("  - {}\n", example));
                }
            }
        }

        if let Some(author) = &pattern.author {
            content.push_str(&format!("Author: {}\n", author));
        }

        if let Some(version) = &pattern.version {
            content.push_str(&format!("Version: {}\n", version));
        }

        content.push_str(&format!(
            "Created: {}\n",
            pattern.created_at.format("%Y-%m-%d %H:%M:%S")
        ));

        content.push_str("\n\n");
    }

    // Footer
    content.push_str("END OF DOCUMENT\n");
    content.push_str("===============\n");

    Ok(content)
}

/// Parse CSV content and convert to patterns
fn parse_csv_to_patterns(csv_content: &str) -> RhemaResult<Vec<PatternEntry>> {
    let mut patterns = Vec::new();
    let lines: Vec<&str> = csv_content.lines().collect();

    if lines.is_empty() {
        return Err(RhemaError::ConfigError("CSV file is empty".to_string()));
    }

    // Parse header to understand column mapping
    let header_line = lines[0];
    let headers: Vec<&str> = header_line
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .collect();

    // Create column index mapping
    let mut column_map = HashMap::new();
    for (index, header) in headers.iter().enumerate() {
        column_map.insert(header.to_lowercase(), index);
    }

    // Process data rows
    for (line_num, line) in lines.iter().skip(1).enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let fields = parse_csv_line(line);
        if fields.is_empty() {
            continue;
        }

        let pattern = create_pattern_from_csv_fields(&fields, &column_map, line_num + 2)?;
        patterns.push(pattern);
    }

    Ok(patterns)
}

/// Parse a single CSV line, handling quoted fields
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes {
                    // Check for escaped quote
                    if chars.peek() == Some(&'"') {
                        chars.next(); // consume the second quote
                        current_field.push('"');
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            ',' => {
                if in_quotes {
                    current_field.push(ch);
                } else {
                    fields.push(current_field.trim().to_string());
                    current_field.clear();
                }
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    // Add the last field
    fields.push(current_field.trim().to_string());

    fields
}

/// Create a PatternEntry from CSV fields
fn create_pattern_from_csv_fields(
    fields: &[String],
    column_map: &HashMap<String, usize>,
    line_num: usize,
) -> RhemaResult<PatternEntry> {
    let get_field = |column_name: &str| -> Option<String> {
        column_map
            .get(column_name)
            .and_then(|&index| fields.get(index).cloned())
    };

    let id = get_field("id").unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = get_field("name").ok_or_else(|| {
        RhemaError::ConfigError(format!("Missing 'name' field on line {}", line_num))
    })?;
    let description = get_field("description").ok_or_else(|| {
        RhemaError::ConfigError(format!("Missing 'description' field on line {}", line_num))
    })?;
    let pattern_type = get_field("pattern_type").ok_or_else(|| {
        RhemaError::ConfigError(format!("Missing 'pattern_type' field on line {}", line_num))
    })?;

    // Parse usage enum
    let usage_str = get_field("usage").unwrap_or_else(|| "recommended".to_string());
    let usage = match usage_str.to_lowercase().as_str() {
        "required" => PatternUsage::Required,
        "recommended" => PatternUsage::Recommended,
        "optional" => PatternUsage::Optional,
        "deprecated" => PatternUsage::Deprecated,
        _ => PatternUsage::Recommended,
    };

    // Parse effectiveness
    let effectiveness = get_field("effectiveness")
        .and_then(|s| s.parse::<u8>().ok())
        .filter(|&v| v <= 10);

    // Parse examples (comma-separated)
    let examples =
        get_field("examples").map(|s| s.split(',').map(|e| e.trim().to_string()).collect());

    // Parse anti-patterns (comma-separated)
    let anti_patterns =
        get_field("anti_patterns").map(|s| s.split(',').map(|a| a.trim().to_string()).collect());

    // Parse category
    let category = get_field("category");

    // Parse maturity
    let maturity = get_field("maturity").and_then(|s| match s.to_lowercase().as_str() {
        "experimental" => Some(PatternMaturity::Experimental),
        "emerging" => Some(PatternMaturity::Emerging),
        "established" => Some(PatternMaturity::Established),
        "mature" => Some(PatternMaturity::Mature),
        "legacy" => Some(PatternMaturity::Legacy),
        _ => None,
    });

    // Parse numeric fields
    let complexity = get_field("complexity").and_then(|s| s.parse::<u8>().ok());
    let maintenance_effort = get_field("maintenance_effort").and_then(|s| s.parse::<u8>().ok());
    let performance_impact = get_field("performance_impact").and_then(|s| s.parse::<u8>().ok());

    // Parse list fields
    let security_implications = get_field("security_implications")
        .map(|s| s.split(',').map(|i| i.trim().to_string()).collect());
    let testing_requirements = get_field("testing_requirements")
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect());
    let documentation_requirements = get_field("documentation_requirements")
        .map(|s| s.split(',').map(|d| d.trim().to_string()).collect());
    let dependencies =
        get_field("dependencies").map(|s| s.split(',').map(|d| d.trim().to_string()).collect());
    let applicable_contexts = get_field("applicable_contexts")
        .map(|s| s.split(',').map(|c| c.trim().to_string()).collect());

    // Parse other fields
    let version = get_field("version");
    let author = get_field("author");

    // Parse review status
    let review_status = get_field("review_status").and_then(|s| match s.to_lowercase().as_str() {
        "approved" => Some(ReviewStatus::Approved),
        "in_review" => Some(ReviewStatus::InReview),
        "pending" => Some(ReviewStatus::Pending),
        "rejected" => Some(ReviewStatus::Rejected),
        "needs_revision" => Some(ReviewStatus::NeedsRevision),
        _ => None,
    });

    Ok(PatternEntry {
        id,
        name,
        description,
        pattern_type,
        usage,
        effectiveness,
        examples,
        anti_patterns,
        related_patterns: None,
        category,
        maturity,
        complexity,
        maintenance_effort,
        performance_impact,
        security_implications,
        testing_requirements,
        documentation_requirements,
        dependencies,
        applicable_contexts,
        version,
        author,
        review_status,
        last_reviewed: None,
        reviewers: None,
        usage_stats: Some(PatternUsageStats {
            usage_count: 0,
            success_count: 0,
            success_rate: 0.0,
            avg_implementation_time: None,
            last_used: None,
            common_use_cases: None,
            reported_issues: None,
            satisfaction_rating: None,
        }),
        created_at: Utc::now(),
        updated_at: None,
        custom: HashMap::new(),
    })
}

/// Validate CSV format and content
fn validate_csv_format(csv_content: &str, result: &mut ValidationResult) -> RhemaResult<()> {
    let lines: Vec<&str> = csv_content.lines().collect();
    let header_line = lines
        .get(0)
        .ok_or_else(|| RhemaError::ConfigError("CSV file is empty".to_string()))?;
    let headers: Vec<&str> = header_line
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .collect();

    let mut required_headers = vec![
        "id",
        "name",
        "description",
        "pattern_type",
        "usage",
        "effectiveness",
    ];
    let mut optional_headers = vec![
        "examples",
        "anti_patterns",
        "category",
        "maturity",
        "complexity",
        "maintenance_effort",
        "performance_impact",
        "security_implications",
        "testing_requirements",
        "documentation_requirements",
        "dependencies",
        "applicable_contexts",
        "version",
        "author",
        "review_status",
    ];

    for header in &required_headers {
        if !headers.contains(&header) {
            result.is_valid = false;
            result
                .errors
                .push(format!("Missing required header: {}", header));
        }
    }

    for header in &optional_headers {
        if headers.contains(&header) {
            // Check if it's a list field (comma-separated)
            if header.ends_with("_implications")
                || header.ends_with("requirements")
                || header.ends_with("contexts")
            {
                if let Some(header_index) = headers.iter().position(|&h| h == *header) {
                    for (line_num, line) in lines.iter().skip(1).enumerate() {
                        let fields: Vec<&str> = line
                            .split(',')
                            .map(|s| s.trim().trim_matches('"'))
                            .collect();
                        if let Some(value) = fields.get(header_index) {
                            if value.contains(',') {
                                result.is_valid = false;
                                result.errors.push(format!(
                                    "List field '{}' contains comma on line {}: {}",
                                    header,
                                    line_num + 2,
                                    value
                                ));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // Check for duplicate IDs
    let mut seen_ids = HashSet::new();
    for line in lines.iter().skip(1) {
        let fields: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        let id = fields
            .get(headers.iter().position(|&h| h == "id").unwrap_or(0))
            .cloned()
            .unwrap_or_default();
        if !seen_ids.insert(id) {
            result.is_valid = false;
            result.errors.push(format!("Duplicate ID found: {}", id));
        }
    }

    // Check for empty fields
    for (i, header) in headers.iter().enumerate() {
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if let Some(value) = fields.get(i) {
                if value.is_empty() {
                    result.is_valid = false;
                    result.errors.push(format!(
                        "Empty field at line {} column '{}'",
                        line_num + 2,
                        header
                    ));
                }
            }
        }
    }

    // Check for invalid enum values (e.g., usage, maturity)
    if let Some(usage_index) = headers.iter().position(|&h| h == "usage") {
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if let Some(usage_value) = fields.get(usage_index) {
                let usage_str = usage_value.to_lowercase();
                if !matches!(
                    usage_str.as_str(),
                    "required" | "recommended" | "optional" | "deprecated"
                ) {
                    result.is_valid = false;
                    result.errors.push(format!(
                        "Invalid usage value on line {}: {}",
                        line_num + 2,
                        usage_value
                    ));
                }
            }
        }
    }

    if let Some(maturity_index) = headers.iter().position(|&h| h == "maturity") {
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if let Some(maturity_value) = fields.get(maturity_index) {
                let maturity_str = maturity_value.to_lowercase();
                if !matches!(
                    maturity_str.as_str(),
                    "experimental" | "emerging" | "established" | "mature" | "legacy"
                ) {
                    result.is_valid = false;
                    result.errors.push(format!(
                        "Invalid maturity value on line {}: {}",
                        line_num + 2,
                        maturity_value
                    ));
                }
            }
        }
    }

    // Check for invalid numeric values (e.g., effectiveness, complexity)
    if let Some(effectiveness_index) = headers.iter().position(|&h| h == "effectiveness") {
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if let Some(effectiveness_value) = fields.get(effectiveness_index) {
                if let Ok(num) = effectiveness_value.parse::<u8>() {
                    if num > 10 {
                        result.is_valid = false;
                        result.errors.push(format!(
                            "Effectiveness rating exceeds 10 on line {}: {}",
                            line_num + 2,
                            effectiveness_value
                        ));
                    }
                }
            }
        }
    }

    if let Some(complexity_index) = headers.iter().position(|&h| h == "complexity") {
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if let Some(complexity_value) = fields.get(complexity_index) {
                if let Ok(num) = complexity_value.parse::<u8>() {
                    if num > 10 {
                        result.is_valid = false;
                        result.errors.push(format!(
                            "Complexity rating exceeds 10 on line {}: {}",
                            line_num + 2,
                            complexity_value
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

// Additional types for new functionality

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ExportFormat {
    Json,
    Yaml,
    Csv,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Json => write!(f, "json"),
            ExportFormat::Yaml => write!(f, "yaml"),
            ExportFormat::Csv => write!(f, "csv"),
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ImportFormat {
    Auto,
    Json,
    Yaml,
    Csv,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ImportStrategy {
    Overwrite,
    Merge,
    Skip,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ValidationLevel {
    Basic,
    Strict,
    Comprehensive,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DocFormat {
    Markdown,
    Html,
    Pdf,
}

#[derive(Debug, Clone)]
pub struct ImportResult {
    pub patterns_imported: usize,
    pub templates_imported: usize,
    pub skipped_count: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportSummary {
    pub patterns_count: usize,
    pub templates_count: usize,
    pub conflicts_count: usize,
    pub warnings_count: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PatternValidationResult {
    pub total_patterns: usize,
    pub valid_count: usize,
    pub invalid_count: usize,
    pub warnings_count: usize,
    pub auto_fixed_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DocumentationResult {
    pub output_dir: String,
    pub files_generated: usize,
    pub patterns_documented: usize,
    pub templates_documented: usize,
    pub index_file: Option<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub details: HashMap<String, serde_json::Value>,
}
