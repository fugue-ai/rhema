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
    fileops::{read_yaml_file, write_yaml_file},
    schema::knowledge::{
        CodeSnippet, ConfigFormat, ConfigurationTemplate, PatternTemplate, PatternTemplates,
        SnippetType, TemplateContent, TemplateMaturity, TemplateMetadata, TemplateVariable,
        TestTemplate, TestType, VariableType,
    },
    RhemaError, RhemaResult,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// Get or create templates file path
fn get_or_create_templates_file(scope_path: &Path) -> RhemaResult<std::path::PathBuf> {
    let templates_file = scope_path.join("templates.yaml");

    // Create the file if it doesn't exist
    if !templates_file.exists() {
        let default_templates = PatternTemplates {
            templates: Vec::new(),
            categories: None,
            tags: None,
        };
        write_yaml_file(&templates_file, &default_templates)?;
    }

    Ok(templates_file)
}

/// Create a new pattern template
pub fn create_pattern_template(
    scope_path: &Path,
    name: String,
    description: String,
    category: String,
    variables: Vec<TemplateVariable>,
    content: TemplateContent,
    metadata: TemplateMetadata,
    version: String,
    author: Option<String>,
    tags: Option<Vec<String>>,
) -> RhemaResult<String> {
    let templates_file = get_or_create_templates_file(scope_path)?;
    let mut templates: PatternTemplates =
        read_yaml_file(&templates_file).unwrap_or_else(|_| PatternTemplates {
            templates: Vec::new(),
            categories: None,
            tags: None,
        });

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let template = PatternTemplate {
        id: id.clone(),
        name,
        description,
        category,
        variables,
        content,
        metadata,
        version,
        author,
        tags,
        usage_count: 0,
        rating: None,
        created_at: now,
        updated_at: None,
    };

    templates.templates.push(template);
    write_yaml_file(&templates_file, &templates)?;

    Ok(id)
}

/// Get all pattern templates
pub fn get_pattern_templates(scope_path: &Path) -> RhemaResult<Vec<PatternTemplate>> {
    let templates_file = get_or_create_templates_file(scope_path)?;
    let templates: PatternTemplates =
        read_yaml_file(&templates_file).unwrap_or_else(|_| PatternTemplates {
            templates: Vec::new(),
            categories: None,
            tags: None,
        });

    Ok(templates.templates)
}

/// Get pattern template by ID
pub fn get_pattern_template(
    scope_path: &Path,
    template_id: &str,
) -> RhemaResult<Option<PatternTemplate>> {
    let templates = get_pattern_templates(scope_path)?;
    Ok(templates.into_iter().find(|t| t.id == template_id))
}

/// Update pattern template
pub fn update_pattern_template(
    scope_path: &Path,
    template_id: &str,
    name: Option<String>,
    description: Option<String>,
    category: Option<String>,
    variables: Option<Vec<TemplateVariable>>,
    content: Option<TemplateContent>,
    metadata: Option<TemplateMetadata>,
    version: Option<String>,
    author: Option<String>,
    tags: Option<Vec<String>>,
) -> RhemaResult<()> {
    let templates_file = get_or_create_templates_file(scope_path)?;
    let mut templates: PatternTemplates = read_yaml_file(&templates_file)?;

    let template = templates
        .templates
        .iter_mut()
        .find(|t| t.id == template_id)
        .ok_or_else(|| {
            RhemaError::ConfigError(format!("Template with ID {} not found", template_id))
        })?;

    if let Some(name) = name {
        template.name = name;
    }
    if let Some(description) = description {
        template.description = description;
    }
    if let Some(category) = category {
        template.category = category;
    }
    if let Some(variables) = variables {
        template.variables = variables;
    }
    if let Some(content) = content {
        template.content = content;
    }
    if let Some(metadata) = metadata {
        template.metadata = metadata;
    }
    if let Some(version) = version {
        template.version = version;
    }
    if let Some(author) = author {
        template.author = Some(author);
    }
    if let Some(tags) = tags {
        template.tags = Some(tags);
    }

    template.updated_at = Some(Utc::now());
    write_yaml_file(&templates_file, &templates)?;

    Ok(())
}

/// Delete pattern template
pub fn delete_pattern_template(scope_path: &Path, template_id: &str) -> RhemaResult<()> {
    let templates_file = get_or_create_templates_file(scope_path)?;
    let mut templates: PatternTemplates = read_yaml_file(&templates_file)?;

    templates.templates.retain(|t| t.id != template_id);
    write_yaml_file(&templates_file, &templates)?;

    Ok(())
}

/// Apply pattern template with variable substitution
pub fn apply_pattern_template(
    scope_path: &Path,
    template_id: &str,
    variables: HashMap<String, String>,
    output_path: Option<&Path>,
) -> RhemaResult<AppliedTemplate> {
    let template = get_pattern_template(scope_path, template_id)?.ok_or_else(|| {
        RhemaError::ConfigError(format!("Template with ID {} not found", template_id))
    })?;

    // Validate required variables
    for var in &template.variables {
        if var.required && !variables.contains_key(&var.name) {
            return Err(RhemaError::ConfigError(format!(
                "Required variable '{}' not provided",
                var.name
            )));
        }
    }

    // Apply variable substitution
    let mut applied_content = template.content.pattern_definition.clone();
    for (var_name, var_value) in &variables {
        let placeholder = format!("{{{{{}}}}}", var_name);
        applied_content = applied_content.replace(&placeholder, var_value);
    }

    // Process code snippets
    let mut applied_snippets = Vec::new();
    if let Some(snippets) = &template.content.code_snippets {
        for snippet in snippets {
            let mut applied_snippet = snippet.clone();
            for (var_name, var_value) in &variables {
                let placeholder = format!("{{{{{}}}}}", var_name);
                applied_snippet.content = applied_snippet.content.replace(&placeholder, var_value);
            }
            applied_snippets.push(applied_snippet);
        }
    }

    // Process configurations
    let mut applied_configs = Vec::new();
    if let Some(configs) = &template.content.configurations {
        for config in configs {
            let mut applied_config = config.clone();
            for (var_name, var_value) in &variables {
                let placeholder = format!("{{{{{}}}}}", var_name);
                applied_config.content = applied_config.content.replace(&placeholder, var_value);
            }
            applied_configs.push(applied_config);
        }
    }

    // Process tests
    let mut applied_tests = Vec::new();
    if let Some(tests) = &template.content.tests {
        for test in tests {
            let mut applied_test = test.clone();
            for (var_name, var_value) in &variables {
                let placeholder = format!("{{{{{}}}}}", var_name);
                applied_test.content = applied_test.content.replace(&placeholder, var_value);
            }
            applied_tests.push(applied_test);
        }
    }

    // Update template usage count
    let templates_file = get_or_create_templates_file(scope_path)?;
    let mut templates: PatternTemplates = read_yaml_file(&templates_file)?;
    if let Some(template) = templates.templates.iter_mut().find(|t| t.id == template_id) {
        template.usage_count += 1;
        template.updated_at = Some(Utc::now());
    }
    write_yaml_file(&templates_file, &templates)?;

    let result = AppliedTemplate {
        template_id: template_id.to_string(),
        template_name: template.name.clone(),
        applied_content,
        applied_snippets,
        applied_configs,
        applied_tests,
        variables_used: variables,
        applied_at: Utc::now(),
    };

    // Write output files if specified
    if let Some(output_path) = output_path {
        write_template_output(output_path, &result)?;
    }

    Ok(result)
}

/// Write template output to files
fn write_template_output(
    output_path: &Path,
    applied_template: &AppliedTemplate,
) -> RhemaResult<()> {
    use std::fs;
    use std::io::Write;

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write main pattern definition
    let pattern_file = output_path.join("pattern.yaml");
    let mut file = fs::File::create(pattern_file)?;
    writeln!(
        file,
        "# Generated from template: {}",
        applied_template.template_name
    )?;
    writeln!(
        file,
        "# Applied at: {}",
        applied_template.applied_at.format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(file, "# Variables used:")?;
    for (var_name, var_value) in &applied_template.variables_used {
        writeln!(file, "#   {}: {}", var_name, var_value)?;
    }
    writeln!(file)?;
    writeln!(file, "{}", applied_template.applied_content)?;

    // Write code snippets
    if !applied_template.applied_snippets.is_empty() {
        let snippets_dir = output_path.join("snippets");
        fs::create_dir_all(&snippets_dir)?;

        for snippet in &applied_template.applied_snippets {
            let file_name = format!("{}.{}", snippet.name, get_file_extension(&snippet.language));
            let snippet_file = snippets_dir.join(file_name);
            let mut file = fs::File::create(snippet_file)?;
            writeln!(
                file,
                "// Generated from template: {}",
                applied_template.template_name
            )?;
            writeln!(file, "// Snippet: {}", snippet.name)?;
            writeln!(file, "// Language: {}", snippet.language)?;
            writeln!(file)?;
            writeln!(file, "{}", snippet.content)?;
        }
    }

    // Write configurations
    if !applied_template.applied_configs.is_empty() {
        let configs_dir = output_path.join("configs");
        fs::create_dir_all(&configs_dir)?;

        for config in &applied_template.applied_configs {
            let file_name = format!("{}.{}", config.name, get_config_extension(&config.format));
            let config_file = configs_dir.join(file_name);
            let mut file = fs::File::create(config_file)?;
            writeln!(
                file,
                "# Generated from template: {}",
                applied_template.template_name
            )?;
            writeln!(file, "# Configuration: {}", config.name)?;
            writeln!(file)?;
            writeln!(file, "{}", config.content)?;
        }
    }

    // Write tests
    if !applied_template.applied_tests.is_empty() {
        let tests_dir = output_path.join("tests");
        fs::create_dir_all(&tests_dir)?;

        for test in &applied_template.applied_tests {
            let file_name = format!("{}.{}", test.name, get_file_extension(&test.framework));
            let test_file = tests_dir.join(file_name);
            let mut file = fs::File::create(test_file)?;
            writeln!(
                file,
                "// Generated from template: {}",
                applied_template.template_name
            )?;
            writeln!(file, "// Test: {}", test.name)?;
            writeln!(file, "// Framework: {}", test.framework)?;
            writeln!(file)?;
            writeln!(file, "{}", test.content)?;
        }
    }

    Ok(())
}

/// Get file extension for programming language
fn get_file_extension(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "rust" => "rs",
        "python" => "py",
        "javascript" => "js",
        "typescript" => "ts",
        "java" => "java",
        "csharp" => "cs",
        "go" => "go",
        "php" => "php",
        "ruby" => "rb",
        "swift" => "swift",
        "kotlin" => "kt",
        "scala" => "scala",
        "cpp" => "cpp",
        "c" => "c",
        "html" => "html",
        "css" => "css",
        "sql" => "sql",
        "yaml" => "yml",
        "json" => "json",
        "toml" => "toml",
        _ => "txt",
    }
}

/// Get file extension for configuration format
fn get_config_extension(format: &ConfigFormat) -> &str {
    match format {
        ConfigFormat::Json => "json",
        ConfigFormat::Yaml => "yml",
        ConfigFormat::Toml => "toml",
        ConfigFormat::Ini => "ini",
        ConfigFormat::Properties => "properties",
        ConfigFormat::Custom => "conf",
    }
}

/// Search pattern templates
pub fn search_pattern_templates(
    scope_path: &Path,
    query: &str,
    category: Option<&str>,
    tags: Option<Vec<&str>>,
    min_rating: Option<f32>,
) -> RhemaResult<Vec<PatternTemplate>> {
    let templates = get_pattern_templates(scope_path)?;
    let query_lower = query.to_lowercase();

    let mut filtered_templates = templates
        .into_iter()
        .filter(|template| {
            // Text search
            let matches_query = template.name.to_lowercase().contains(&query_lower)
                || template.description.to_lowercase().contains(&query_lower)
                || template.category.to_lowercase().contains(&query_lower);

            // Category filter
            let matches_category = category
                .map(|cat| template.category.to_lowercase() == cat.to_lowercase())
                .unwrap_or(true);

            // Tags filter
            let matches_tags = tags
                .as_ref()
                .map(|tag_list| {
                    template
                        .tags
                        .as_ref()
                        .map(|template_tags| {
                            tag_list.iter().any(|tag| {
                                template_tags
                                    .iter()
                                    .any(|t| t.to_lowercase() == tag.to_lowercase())
                            })
                        })
                        .unwrap_or(false)
                })
                .unwrap_or(true);

            // Rating filter
            let matches_rating = min_rating
                .map(|min| template.rating.map_or(false, |rating| rating >= min))
                .unwrap_or(true);

            matches_query && matches_category && matches_tags && matches_rating
        })
        .collect::<Vec<_>>();

    // Sort by usage count and rating
    filtered_templates.sort_by(|a, b| {
        b.usage_count.cmp(&a.usage_count).then(
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    });

    Ok(filtered_templates)
}

/// Get template statistics
pub fn get_template_statistics(scope_path: &Path) -> RhemaResult<TemplateStatistics> {
    let templates = get_pattern_templates(scope_path)?;

    let mut stats = TemplateStatistics {
        total_templates: templates.len(),
        templates_by_category: HashMap::new(),
        templates_by_maturity: HashMap::new(),
        average_rating: 0.0,
        total_usage: 0,
        top_templates: Vec::new(),
    };

    let mut total_rating = 0.0;
    let mut rated_templates = 0;

    for template in &templates {
        // Category statistics
        *stats
            .templates_by_category
            .entry(template.category.clone())
            .or_insert(0) += 1;

        // Maturity statistics
        let maturity_str = format!("{:?}", template.metadata.maturity);
        *stats.templates_by_maturity.entry(maturity_str).or_insert(0) += 1;

        // Rating statistics
        if let Some(rating) = template.rating {
            total_rating += rating;
            rated_templates += 1;
        }

        // Usage statistics
        stats.total_usage += template.usage_count;

        // Top templates
        if template.usage_count > 0 {
            stats.top_templates.push(TemplateSummary {
                id: template.id.clone(),
                name: template.name.clone(),
                category: template.category.clone(),
                usage_count: template.usage_count,
                rating: template.rating,
            });
        }
    }

    if rated_templates > 0 {
        stats.average_rating = total_rating / rated_templates as f32;
    }

    // Sort top templates by usage
    stats
        .top_templates
        .sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
    stats.top_templates.truncate(10);

    Ok(stats)
}

/// Applied template result
#[derive(Debug, Clone)]
pub struct AppliedTemplate {
    pub template_id: String,
    pub template_name: String,
    pub applied_content: String,
    pub applied_snippets: Vec<CodeSnippet>,
    pub applied_configs: Vec<ConfigurationTemplate>,
    pub applied_tests: Vec<TestTemplate>,
    pub variables_used: HashMap<String, String>,
    pub applied_at: chrono::DateTime<Utc>,
}

/// Template statistics
#[derive(Debug, Clone)]
pub struct TemplateStatistics {
    pub total_templates: usize,
    pub templates_by_category: HashMap<String, usize>,
    pub templates_by_maturity: HashMap<String, usize>,
    pub average_rating: f32,
    pub total_usage: u64,
    pub top_templates: Vec<TemplateSummary>,
}

/// Template summary
#[derive(Debug, Clone)]
pub struct TemplateSummary {
    pub id: String,
    pub name: String,
    pub category: String,
    pub usage_count: u64,
    pub rating: Option<f32>,
}
