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

use crate::CliContext;
use clap::Subcommand;
use flate2::write::GzEncoder;
use flate2::Compression;
use rhema_api::RhemaResult;
use rhema_core::fileops::pattern_management::{
    DocFormat, ExportFormat, ImportFormat, ImportStrategy, ValidationLevel,
};
use rhema_core::{
    fileops,
    schema::knowledge::{
        CodeSnippet, ConfigFormat, ConfigurationTemplate, SnippetType, TemplateContent,
        TemplateMaturity, TemplateMetadata, TemplateVariable, TestTemplate, TestType, VariableType,
    },
    PatternMaturity, PatternUsage, ReviewStatus,
};
use std::collections::HashMap;
use std::io::Write;

use colored::*;

#[derive(Subcommand)]
pub enum PatternSubcommands {
    /// Add a new pattern
    Add {
        /// Pattern name
        #[arg(value_name = "NAME")]
        name: String,

        /// Pattern description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: String,

        /// Usage context
        #[arg(long, value_enum, default_value = "recommended")]
        usage: PatternUsage,

        /// Effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// Examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// Anti-patterns to avoid (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,

        /// Pattern category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Pattern maturity level
        #[arg(long, value_enum)]
        maturity: Option<PatternMaturity>,

        /// Implementation complexity (1-10)
        #[arg(long, value_name = "COMPLEXITY")]
        complexity: Option<u8>,

        /// Maintenance effort (1-10)
        #[arg(long, value_name = "MAINTENANCE")]
        maintenance_effort: Option<u8>,

        /// Performance impact (1-10)
        #[arg(long, value_name = "PERFORMANCE")]
        performance_impact: Option<u8>,

        /// Security implications (comma-separated)
        #[arg(long, value_name = "SECURITY")]
        security_implications: Option<String>,

        /// Testing requirements (comma-separated)
        #[arg(long, value_name = "TESTING")]
        testing_requirements: Option<String>,

        /// Documentation requirements (comma-separated)
        #[arg(long, value_name = "DOCS")]
        documentation_requirements: Option<String>,

        /// Dependencies (comma-separated)
        #[arg(long, value_name = "DEPS")]
        dependencies: Option<String>,

        /// Applicable contexts (comma-separated)
        #[arg(long, value_name = "CONTEXTS")]
        applicable_contexts: Option<String>,

        /// Pattern version
        #[arg(long, value_name = "VERSION")]
        pattern_version: Option<String>,

        /// Pattern author
        #[arg(long, value_name = "AUTHOR")]
        author: Option<String>,

        /// Review status
        #[arg(long, value_enum)]
        review_status: Option<ReviewStatus>,
    },

    /// List patterns
    List {
        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// Filter by usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// Filter by effectiveness rating (minimum)
        #[arg(long, value_name = "RATING")]
        min_effectiveness: Option<u8>,

        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by maturity level
        #[arg(long, value_enum)]
        maturity: Option<PatternMaturity>,

        /// Filter by review status
        #[arg(long, value_enum)]
        review_status: Option<ReviewStatus>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Sort by field (name, effectiveness, created_at, updated_at)
        #[arg(long, value_enum, default_value = "name")]
        sort_by: SortField,

        /// Sort order
        #[arg(long, value_enum, default_value = "asc")]
        sort_order: SortOrder,
    },

    /// Update a pattern
    Update {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// New usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// New effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// New examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// New anti-patterns (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New maturity level
        #[arg(long, value_enum)]
        maturity: Option<PatternMaturity>,

        /// New complexity rating (1-10)
        #[arg(long, value_name = "COMPLEXITY")]
        complexity: Option<u8>,

        /// New maintenance effort (1-10)
        #[arg(long, value_name = "MAINTENANCE")]
        maintenance_effort: Option<u8>,

        /// New performance impact (1-10)
        #[arg(long, value_name = "PERFORMANCE")]
        performance_impact: Option<u8>,

        /// New security implications (comma-separated)
        #[arg(long, value_name = "SECURITY")]
        security_implications: Option<String>,

        /// New testing requirements (comma-separated)
        #[arg(long, value_name = "TESTING")]
        testing_requirements: Option<String>,

        /// New documentation requirements (comma-separated)
        #[arg(long, value_name = "DOCS")]
        documentation_requirements: Option<String>,

        /// New dependencies (comma-separated)
        #[arg(long, value_name = "DEPS")]
        dependencies: Option<String>,

        /// New applicable contexts (comma-separated)
        #[arg(long, value_name = "CONTEXTS")]
        applicable_contexts: Option<String>,

        /// New version
        #[arg(long, value_name = "VERSION")]
        pattern_version: Option<String>,

        /// New author
        #[arg(long, value_name = "AUTHOR")]
        author: Option<String>,

        /// New review status
        #[arg(long, value_enum)]
        review_status: Option<ReviewStatus>,
    },

    /// Delete a pattern
    Delete {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
    },

    /// Review a pattern
    Review {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,

        /// Review status
        #[arg(long, value_enum)]
        status: ReviewStatus,

        /// Review comments
        #[arg(long, value_name = "COMMENTS")]
        comments: Option<String>,

        /// Reviewer name
        #[arg(long, value_name = "REVIEWER")]
        reviewer: Option<String>,
    },

    /// Analyze pattern usage
    Analyze {
        /// Pattern ID (optional, analyze all if not specified)
        #[arg(value_name = "ID")]
        id: Option<String>,

        /// Generate usage report
        #[arg(long)]
        report: bool,

        /// Export analysis to file
        #[arg(long, value_name = "FILE")]
        export: Option<String>,
    },

    /// Search patterns
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Search in specific fields (name, description, examples, etc.)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Case sensitive search
        #[arg(long)]
        case_sensitive: bool,

        /// Use regex pattern
        #[arg(long)]
        regex: bool,
    },

    /// Template management
    Template {
        #[command(subcommand)]
        command: TemplateSubcommands,
    },

    /// Export patterns
    Export {
        /// Export format (json, yaml, csv)
        #[arg(long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Output file path
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,

        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Include templates in export
        #[arg(long)]
        include_templates: bool,

        /// Include usage statistics
        #[arg(long)]
        include_stats: bool,

        /// Compress output
        #[arg(long)]
        compress: bool,
    },

    /// Import patterns
    Import {
        /// Input file path
        #[arg(value_name = "FILE")]
        input_file: String,

        /// Import format (auto, json, yaml, csv)
        #[arg(long, value_enum, default_value = "auto")]
        format: ImportFormat,

        /// Import strategy (overwrite, merge, skip)
        #[arg(long, value_enum, default_value = "merge")]
        strategy: ImportStrategy,

        /// Validate before import
        #[arg(long)]
        validate: bool,

        /// Backup existing patterns
        #[arg(long)]
        backup: bool,

        /// Dry run (don't actually import)
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate patterns
    Validate {
        /// Pattern ID (validate all if not specified)
        #[arg(value_name = "ID")]
        id: Option<String>,

        /// Validation level (basic, strict, comprehensive)
        #[arg(long, value_enum, default_value = "basic")]
        level: ValidationLevel,

        /// Fix issues automatically where possible
        #[arg(long)]
        auto_fix: bool,

        /// Output validation report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<String>,
    },

    /// Generate pattern documentation
    GenerateDocs {
        /// Output directory
        #[arg(long, value_name = "OUTPUT_DIR")]
        output_dir: Option<String>,

        /// Documentation format (markdown, html, pdf)
        #[arg(long, value_enum, default_value = "markdown")]
        format: DocFormat,

        /// Include examples
        #[arg(long)]
        include_examples: bool,

        /// Include templates
        #[arg(long)]
        include_templates: bool,

        /// Include usage statistics
        #[arg(long)]
        include_stats: bool,

        /// Generate index
        #[arg(long)]
        generate_index: bool,
    },
}

/// Sort fields for pattern listing
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SortField {
    Name,
    Effectiveness,
    CreatedAt,
    UpdatedAt,
    Category,
    Maturity,
    Complexity,
}

/// Sort order
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Subcommand)]
pub enum TemplateSubcommands {
    /// Create a new pattern template
    Create {
        /// Template name
        #[arg(value_name = "NAME")]
        name: String,

        /// Template description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Template category
        #[arg(long, value_name = "CATEGORY")]
        category: String,

        /// Template version
        #[arg(long, value_name = "VERSION")]
        template_version: String,

        /// Template author
        #[arg(long, value_name = "AUTHOR")]
        author: Option<String>,

        /// Template tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// Template variables (JSON format)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,

        /// Pattern definition template
        #[arg(long, value_name = "PATTERN_DEFINITION")]
        pattern_definition: String,

        /// Code snippets (JSON format)
        #[arg(long, value_name = "SNIPPETS")]
        snippets: Option<String>,

        /// Configuration templates (JSON format)
        #[arg(long, value_name = "CONFIGS")]
        configs: Option<String>,

        /// Test templates (JSON format)
        #[arg(long, value_name = "TESTS")]
        tests: Option<String>,

        /// Template maturity level
        #[arg(long, value_enum)]
        maturity: TemplateMaturity,

        /// Template complexity (1-10)
        #[arg(long, value_name = "COMPLEXITY")]
        complexity: u8,

        /// Estimated implementation time (hours)
        #[arg(long, value_name = "ESTIMATED_TIME")]
        estimated_time: Option<f32>,

        /// Prerequisites (comma-separated)
        #[arg(long, value_name = "PREREQUISITES")]
        prerequisites: Option<String>,

        /// Dependencies (comma-separated)
        #[arg(long, value_name = "DEPENDENCIES")]
        dependencies: Option<String>,

        /// Supported languages (comma-separated)
        #[arg(long, value_name = "LANGUAGES")]
        supported_languages: Option<String>,

        /// Supported frameworks (comma-separated)
        #[arg(long, value_name = "FRAMEWORKS")]
        supported_frameworks: Option<String>,

        /// License
        #[arg(long, value_name = "LICENSE")]
        license: Option<String>,

        /// Repository URL
        #[arg(long, value_name = "REPO_URL")]
        repository_url: Option<String>,

        /// Documentation URL
        #[arg(long, value_name = "DOCS_URL")]
        documentation_url: Option<String>,
    },

    /// List pattern templates
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// Minimum rating filter
        #[arg(long, value_name = "MIN_RATING")]
        min_rating: Option<f32>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Sort by field
        #[arg(long, value_enum, default_value = "usage")]
        sort_by: TemplateSortField,

        /// Sort order
        #[arg(long, value_enum, default_value = "desc")]
        sort_order: SortOrder,
    },

    /// Show template details
    Show {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,
    },

    /// Apply a pattern template
    Apply {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,

        /// Variables (key=value format, comma-separated)
        #[arg(long, value_name = "VARIABLES")]
        variables: String,

        /// Output directory
        #[arg(long, value_name = "OUTPUT_DIR")]
        output_dir: Option<String>,

        /// Preview only (don't write files)
        #[arg(long)]
        preview: bool,
    },

    /// Search pattern templates
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// Minimum rating filter
        #[arg(long, value_name = "MIN_RATING")]
        min_rating: Option<f32>,
    },

    /// Update a pattern template
    Update {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New version
        #[arg(long, value_name = "VERSION")]
        template_version: Option<String>,

        /// New author
        #[arg(long, value_name = "AUTHOR")]
        author: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// New maturity level
        #[arg(long, value_enum)]
        maturity: Option<TemplateMaturity>,

        /// New complexity (1-10)
        #[arg(long, value_name = "COMPLEXITY")]
        complexity: Option<u8>,
    },

    /// Delete a pattern template
    Delete {
        /// Template ID
        #[arg(value_name = "ID")]
        id: String,

        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },

    /// Get template statistics
    Stats {
        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,
    },
}

/// Template sort fields
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TemplateSortField {
    Name,
    Category,
    Usage,
    Rating,
    CreatedAt,
    UpdatedAt,
}

pub fn handle_pattern(
    _context: &CliContext,
    scope: &rhema_core::Scope,
    subcommand: &PatternSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        PatternSubcommands::Add {
            name,
            description,
            pattern_type,
            usage,
            effectiveness,
            examples,
            anti_patterns,
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
            pattern_version,
            author,
            review_status,
        } => {
            let id = fileops::add_enhanced_pattern_entry(
                &scope.path,
                name.clone(),
                description.clone(),
                pattern_type.clone(),
                usage.clone(),
                *effectiveness,
                examples.clone(),
                anti_patterns.clone(),
                category.clone(),
                maturity.clone(),
                *complexity,
                *maintenance_effort,
                *performance_impact,
                security_implications.clone(),
                testing_requirements.clone(),
                documentation_requirements.clone(),
                dependencies.clone(),
                applicable_contexts.clone(),
                pattern_version.clone(),
                author.clone(),
                review_status.clone(),
            )?;

            println!("✅ Pattern '{}' added with ID: {}", name, id);
            Ok(())
        }
        PatternSubcommands::List {
            pattern_type,
            usage,
            min_effectiveness,
            category,
            maturity,
            review_status,
            detailed: _,
            sort_by: _,
            sort_order: _,
        } => {
            let patterns = fileops::get_enhanced_pattern_entries(
                &scope.path,
                pattern_type.clone(),
                usage.clone(),
                *min_effectiveness,
                category.clone(),
                maturity.clone(),
                review_status.clone(),
            )?;

            if patterns.is_empty() {
                println!("📋 No patterns found matching the criteria");
            } else {
                println!("📋 Found {} pattern(s):", patterns.len());
                println!();

                for pattern in patterns {
                    println!("🆔 ID: {}", pattern.id);
                    println!("📝 Name: {}", pattern.name);
                    println!("📄 Description: {}", pattern.description);
                    println!("🏷️  Type: {}", pattern.pattern_type);

                    if let Some(category) = &pattern.category {
                        println!("📂 Category: {}", category);
                    }

                    println!("📊 Usage: {:?}", pattern.usage);

                    if let Some(maturity) = &pattern.maturity {
                        println!("🌱 Maturity: {:?}", maturity);
                    }

                    if let Some(eff) = pattern.effectiveness {
                        let effectiveness_color = if eff >= 8 {
                            "green"
                        } else if eff >= 5 {
                            "yellow"
                        } else {
                            "red"
                        };
                        println!(
                            "⭐ Effectiveness: {}",
                            format!("{}/10", eff).color(effectiveness_color)
                        );
                    }

                    if let Some(complexity) = pattern.complexity {
                        println!("🔧 Complexity: {}/10", complexity);
                    }

                    if let Some(review_status) = &pattern.review_status {
                        let status_color = match review_status {
                            ReviewStatus::Approved => "green",
                            ReviewStatus::InReview => "yellow",
                            ReviewStatus::Pending => "red",
                            ReviewStatus::Rejected => "red",
                            ReviewStatus::NeedsRevision => "yellow",
                        };
                        println!(
                            "📋 Review Status: {}",
                            format!("{:?}", review_status).color(status_color)
                        );
                    }

                    if let Some(examples) = &pattern.examples {
                        println!("💡 Examples: {}", examples.join(", "));
                    }
                    if let Some(anti_patterns) = &pattern.anti_patterns {
                        println!("⚠️  Anti-patterns: {}", anti_patterns.join(", "));
                    }

                    if let Some(author) = &pattern.author {
                        println!("👤 Author: {}", author);
                    }

                    if let Some(version) = &pattern.version {
                        println!("📦 Version: {}", version);
                    }

                    println!(
                        "📅 Created: {}",
                        pattern.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    if let Some(updated) = pattern.updated_at {
                        println!("🔄 Updated: {}", updated.format("%Y-%m-%d %H:%M:%S"));
                    }
                    println!();
                }
            }
            Ok(())
        }
        PatternSubcommands::Update {
            id,
            name,
            description,
            pattern_type,
            usage,
            effectiveness,
            examples,
            anti_patterns,
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
            pattern_version,
            author,
            review_status,
        } => {
            fileops::update_enhanced_pattern_entry(
                &scope.path,
                id,
                name.clone(),
                description.clone(),
                pattern_type.clone(),
                usage.clone(),
                *effectiveness,
                examples.clone(),
                anti_patterns.clone(),
                category.clone(),
                maturity.clone(),
                *complexity,
                *maintenance_effort,
                *performance_impact,
                security_implications.clone(),
                testing_requirements.clone(),
                documentation_requirements.clone(),
                dependencies.clone(),
                applicable_contexts.clone(),
                pattern_version.clone(),
                author.clone(),
                review_status.clone(),
            )?;

            println!("✅ Pattern '{}' updated successfully", id);
            Ok(())
        }
        PatternSubcommands::Delete { id } => {
            fileops::delete_pattern_entry(&scope.path, id)?;
            println!("✅ Pattern '{}' deleted successfully", id);
            Ok(())
        }
        PatternSubcommands::Review {
            id,
            status,
            comments,
            reviewer,
        } => {
            fileops::review_pattern(
                &scope.path,
                id,
                status.clone(),
                comments.clone(),
                reviewer.clone(),
            )?;
            println!("✅ Pattern '{}' reviewed with status: {:?}", id, status);
            Ok(())
        }
        PatternSubcommands::Analyze { id, report, export } => {
            let analysis = fileops::analyze_pattern_usage(&scope.path, id.as_deref())?;

            if *report {
                println!("📊 Pattern Analysis Report");
                println!("{}", "─".repeat(50));
                println!("Total Patterns: {}", analysis.total_patterns);
                println!(
                    "Average Effectiveness: {:.1}/10",
                    analysis.average_effectiveness
                );
                println!(
                    "Patterns Needing Review: {}",
                    analysis.patterns_needing_review
                );

                if !analysis.patterns_by_category.is_empty() {
                    println!("\n📂 By Category:");
                    for (category, count) in &analysis.patterns_by_category {
                        println!("  {}: {}", category, count);
                    }
                }

                if !analysis.top_patterns.is_empty() {
                    println!("\n⭐ Top Patterns (Effectiveness ≥ 8):");
                    for pattern in &analysis.top_patterns {
                        println!(
                            "  {} ({}): {}/10",
                            pattern.name, pattern.id, pattern.effectiveness
                        );
                    }
                }

                if !analysis.recommendations.is_empty() {
                    println!("\n💡 Recommendations:");
                    for rec in &analysis.recommendations {
                        println!("  • {}", rec);
                    }
                }
            }

            if let Some(export_path) = export {
                // Export analysis to file
                let export_content = serde_json::to_string_pretty(&analysis)?;
                std::fs::write(export_path, export_content)?;
                println!("📁 Analysis exported to: {}", export_path);
            }

            Ok(())
        }
        PatternSubcommands::Search {
            query,
            fields,
            case_sensitive,
            regex,
        } => {
            let search_fields = fields
                .as_ref()
                .map(|f| f.split(',').map(|s| s.trim().to_string()).collect());
            let patterns = fileops::search_patterns(
                &scope.path,
                query,
                search_fields,
                *case_sensitive,
                *regex,
            )?;

            if patterns.is_empty() {
                println!("🔍 No patterns found matching '{}'", query);
            } else {
                println!(
                    "🔍 Found {} pattern(s) matching '{}':",
                    patterns.len(),
                    query
                );
                println!();

                for pattern in patterns {
                    println!("🆔 ID: {}", pattern.id);
                    println!("📝 Name: {}", pattern.name);
                    println!("📄 Description: {}", pattern.description);
                    println!("🏷️  Type: {}", pattern.pattern_type);
                    if let Some(category) = &pattern.category {
                        println!("📂 Category: {}", category);
                    }
                    println!("📊 Usage: {:?}", pattern.usage);
                    if let Some(eff) = pattern.effectiveness {
                        println!("⭐ Effectiveness: {}/10", eff);
                    }
                    println!();
                }
            }
            Ok(())
        }
        PatternSubcommands::Template { command } => handle_template(&scope.path, command),
        PatternSubcommands::Export {
            format,
            output_file,
            pattern_type,
            category,
            include_templates,
            include_stats,
            compress,
        } => {
            let export_data = fileops::export_patterns(
                &scope.path,
                format.clone(),
                pattern_type.clone(),
                category.clone(),
                *include_templates,
                *include_stats,
            )?;

            let output_path = output_file.clone().unwrap_or_else(|| {
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                format!(
                    "patterns_export_{}.{}",
                    timestamp,
                    format.to_string().to_lowercase()
                )
            });

            if *compress {
                // Implement compression
                let compressed_path = format!("{}.gz", output_path);
                let file = std::fs::File::create(&compressed_path)?;
                let mut encoder = GzEncoder::new(file, Compression::default());
                encoder.write_all(export_data.as_bytes())?;
                encoder.finish()?;
                println!("✅ Patterns exported to: {} (compressed)", compressed_path);
            } else {
                std::fs::write(&output_path, export_data)?;
                println!("✅ Patterns exported to: {}", output_path);
            }
            Ok(())
        }
        PatternSubcommands::Import {
            input_file,
            format,
            strategy,
            validate,
            backup,
            dry_run,
        } => {
            if *backup {
                let backup_path = format!(
                    "{}.backup_{}",
                    input_file,
                    chrono::Utc::now().format("%Y%m%d_%H%M%S")
                );
                fileops::backup_patterns(&scope.path, &backup_path)?;
                println!("💾 Backup created: {}", backup_path);
            }

            if *validate {
                let validation_result = fileops::validate_import_file(input_file, format.clone())?;
                if !validation_result.is_valid {
                    println!("❌ Import validation failed:");
                    for error in &validation_result.errors {
                        println!("   • {}", error);
                    }
                    return Ok(());
                }
                println!("✅ Import validation passed");
            }

            if *dry_run {
                let import_summary = fileops::dry_run_import(
                    &scope.path,
                    input_file,
                    format.clone(),
                    strategy.clone(),
                )?;
                println!("🔍 Dry run import summary:");
                println!("   • Patterns to import: {}", import_summary.patterns_count);
                println!(
                    "   • Templates to import: {}",
                    import_summary.templates_count
                );
                println!(
                    "   • Conflicts detected: {}",
                    import_summary.conflicts_count
                );
                println!("   • Warnings: {}", import_summary.warnings_count);
            } else {
                let import_result = fileops::import_patterns(
                    &scope.path,
                    input_file,
                    format.clone(),
                    strategy.clone(),
                )?;
                println!("✅ Import completed successfully");
                println!(
                    "   • Patterns imported: {}",
                    import_result.patterns_imported
                );
                println!(
                    "   • Templates imported: {}",
                    import_result.templates_imported
                );
                println!("   • Skipped: {}", import_result.skipped_count);
            }
            Ok(())
        }
        PatternSubcommands::Validate {
            id,
            level,
            auto_fix,
            report_file,
        } => {
            let validation_result =
                fileops::validate_patterns(&scope.path, id.as_deref(), level.clone(), *auto_fix)?;

            println!("🔍 Pattern Validation Results");
            println!("{}", "─".repeat(50));
            println!("📊 Total Patterns: {}", validation_result.total_patterns);
            println!("✅ Valid: {}", validation_result.valid_count);
            println!("❌ Invalid: {}", validation_result.invalid_count);
            println!("⚠️  Warnings: {}", validation_result.warnings_count);
            println!("🔧 Auto-fixed: {}", validation_result.auto_fixed_count);

            if !validation_result.errors.is_empty() {
                println!("\n❌ Errors:");
                for error in &validation_result.errors {
                    println!("   • {}", error);
                }
            }

            if !validation_result.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation_result.warnings {
                    println!("   • {}", warning);
                }
            }

            if let Some(report_path) = report_file {
                let report_content = serde_json::to_string_pretty(&validation_result)?;
                std::fs::write(&report_path, report_content)?;
                println!("\n📄 Validation report saved to: {}", report_path);
            }

            Ok(())
        }
        PatternSubcommands::GenerateDocs {
            output_dir,
            format,
            include_examples,
            include_templates,
            include_stats,
            generate_index,
        } => {
            let docs_result = fileops::generate_pattern_documentation(
                &scope.path,
                output_dir.as_deref().map(|s| std::path::Path::new(s)),
                format.clone(),
                *include_examples,
                *include_templates,
                *include_stats,
                *generate_index,
            )?;

            println!("📚 Pattern Documentation Generated");
            println!("{}", "─".repeat(50));
            println!("📁 Output directory: {}", docs_result.output_dir);
            println!("📄 Files generated: {}", docs_result.files_generated);
            println!(
                "📊 Patterns documented: {}",
                docs_result.patterns_documented
            );
            if *include_templates {
                println!(
                    "📋 Templates documented: {}",
                    docs_result.templates_documented
                );
            }
            if *generate_index {
                println!("📑 Index generated: {:?}", docs_result.index_file);
            }

            Ok(())
        }
    }
}

fn handle_template(scope: &std::path::Path, subcommand: &TemplateSubcommands) -> RhemaResult<()> {
    match subcommand {
        TemplateSubcommands::Create {
            name,
            description,
            category,
            template_version,
            author,
            tags,
            variables,
            pattern_definition,
            snippets,
            configs,
            tests,
            maturity,
            complexity,
            estimated_time,
            prerequisites,
            dependencies,
            supported_languages,
            supported_frameworks,
            license,
            repository_url,
            documentation_url,
        } => {
            // Parse variables
            let template_variables = if let Some(vars_str) = variables {
                parse_template_variables(&vars_str)?
            } else {
                Vec::new()
            };

            // Parse snippets
            let code_snippets = if let Some(snippets_str) = snippets {
                parse_code_snippets(&snippets_str)?
            } else {
                None
            };

            // Parse configurations
            let configurations = if let Some(configs_str) = configs {
                parse_configurations(&configs_str)?
            } else {
                None
            };

            // Parse tests
            let test_templates = if let Some(tests_str) = tests {
                parse_test_templates(&tests_str)?
            } else {
                None
            };

            // Create template content
            let content = TemplateContent {
                pattern_definition: pattern_definition.clone(),
                examples: None,
                code_snippets,
                configurations,
                documentation: None,
                tests: test_templates,
            };

            // Create template metadata
            let metadata = TemplateMetadata {
                maturity: maturity.clone(),
                complexity: *complexity,
                estimated_time: *estimated_time,
                prerequisites: prerequisites
                    .as_ref()
                    .map(|p| p.split(',').map(|s| s.trim().to_string()).collect()),
                dependencies: dependencies
                    .as_ref()
                    .map(|d| d.split(',').map(|s| s.trim().to_string()).collect()),
                supported_languages: supported_languages
                    .as_ref()
                    .map(|l| l.split(',').map(|s| s.trim().to_string()).collect()),
                supported_frameworks: supported_frameworks
                    .as_ref()
                    .map(|f| f.split(',').map(|s| s.trim().to_string()).collect()),
                license: license.clone(),
                repository_url: repository_url.clone(),
                documentation_url: documentation_url.clone(),
            };

            let id = fileops::create_pattern_template(
                scope,
                name.clone(),
                description.clone(),
                category.clone(),
                template_variables,
                content,
                metadata,
                template_version.clone(),
                author.clone(),
                tags.as_ref()
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect()),
            )?;

            println!("✅ Template '{}' created with ID: {}", name, id);
            Ok(())
        }

        TemplateSubcommands::List {
            category,
            tags,
            min_rating,
            detailed: _,
            sort_by: _,
            sort_order: _,
        } => {
            let templates = fileops::get_pattern_templates(scope)?;

            if templates.is_empty() {
                println!("📋 No templates found");
                return Ok(());
            }

            println!("📋 Found {} template(s):", templates.len());
            println!();

            for template in templates {
                println!("🆔 ID: {}", template.id);
                println!("📝 Name: {}", template.name);
                println!("📄 Description: {}", template.description);
                println!("📂 Category: {}", template.category);
                println!("📊 Usage Count: {}", template.usage_count);

                if let Some(rating) = template.rating {
                    println!("⭐ Rating: {:.1}/5.0", rating);
                }

                println!("🌱 Maturity: {:?}", template.metadata.maturity);
                println!("🔧 Complexity: {}/10", template.metadata.complexity);

                if let Some(author) = &template.author {
                    println!("👤 Author: {}", author);
                }

                println!("📦 Version: {}", template.version);
                println!(
                    "📅 Created: {}",
                    template.created_at.format("%Y-%m-%d %H:%M:%S")
                );

                if let Some(updated) = template.updated_at {
                    println!("🔄 Updated: {}", updated.format("%Y-%m-%d %H:%M:%S"));
                }

                if let Some(tags) = &template.tags {
                    println!("🏷️  Tags: {}", tags.join(", "));
                }

                println!("📋 Variables: {}", template.variables.len());
                for var in &template.variables {
                    println!(
                        "   • {} ({:?}) - {}",
                        var.name, var.var_type, var.description
                    );
                }

                println!();
            }

            Ok(())
        }

        TemplateSubcommands::Show { id } => {
            let template = fileops::get_pattern_template(scope, id)?.ok_or_else(|| {
                rhema_api::RhemaError::ConfigError(format!("Template with ID {} not found", id))
            })?;

            println!("📋 Template Details");
            println!("{}", "─".repeat(50));
            println!("🆔 ID: {}", template.id);
            println!("📝 Name: {}", template.name);
            println!("📄 Description: {}", template.description);
            println!("📂 Category: {}", template.category);
            println!("📦 Version: {}", template.version);
            println!("🌱 Maturity: {:?}", template.metadata.maturity);
            println!("🔧 Complexity: {}/10", template.metadata.complexity);
            println!("📊 Usage Count: {}", template.usage_count);

            if let Some(rating) = template.rating {
                println!("⭐ Rating: {:.1}/5.0", rating);
            }

            if let Some(author) = &template.author {
                println!("👤 Author: {}", author);
            }

            if let Some(tags) = &template.tags {
                println!("🏷️  Tags: {}", tags.join(", "));
            }

            println!("\n📋 Variables:");
            for var in &template.variables {
                println!(
                    "   • {} ({:?}) - {}",
                    var.name, var.var_type, var.description
                );
                if var.required {
                    println!("     Required: Yes");
                }
                if let Some(default) = &var.default_value {
                    println!("     Default: {}", default);
                }
                if let Some(examples) = &var.examples {
                    println!("     Examples: {}", examples.join(", "));
                }
            }

            println!("\n📄 Pattern Definition:");
            println!("{}", template.content.pattern_definition);

            if let Some(snippets) = &template.content.code_snippets {
                println!("\n💻 Code Snippets:");
                for snippet in snippets {
                    println!(
                        "   • {} ({}) - {}",
                        snippet.name, snippet.language, snippet.description
                    );
                }
            }

            if let Some(configs) = &template.content.configurations {
                println!("\n⚙️  Configurations:");
                for config in configs {
                    println!(
                        "   • {} ({:?}) - {}",
                        config.name, config.format, config.description
                    );
                }
            }

            if let Some(tests) = &template.content.tests {
                println!("\n🧪 Tests:");
                for test in tests {
                    println!(
                        "   • {} ({}) - {}",
                        test.name, test.framework, test.description
                    );
                }
            }

            Ok(())
        }

        TemplateSubcommands::Apply {
            id,
            variables,
            output_dir,
            preview,
        } => {
            // Parse variables
            let variables_map = parse_variables_string(variables)?;

            if *preview {
                // Preview mode - don't write files
                let applied = fileops::apply_pattern_template(scope, id, variables_map, None)?;

                println!("🔍 Template Preview: {}", applied.template_name);
                println!("{}", "─".repeat(50));
                println!("📄 Applied Pattern Definition:");
                println!("{}", applied.applied_content);

                if !applied.applied_snippets.is_empty() {
                    println!("\n💻 Code Snippets:");
                    for snippet in &applied.applied_snippets {
                        println!("   • {} ({})", snippet.name, snippet.language);
                        println!("     {}", snippet.content);
                    }
                }

                if !applied.applied_configs.is_empty() {
                    println!("\n⚙️  Configurations:");
                    for config in &applied.applied_configs {
                        println!("   • {} ({:?})", config.name, config.format);
                        println!("     {}", config.content);
                    }
                }

                if !applied.applied_tests.is_empty() {
                    println!("\n🧪 Tests:");
                    for test in &applied.applied_tests {
                        println!("   • {} ({})", test.name, test.framework);
                        println!("     {}", test.content);
                    }
                }
            } else {
                // Apply mode - write files
                let output_path = output_dir.as_ref().map(|d| std::path::Path::new(d));
                let applied =
                    fileops::apply_pattern_template(scope, id, variables_map, output_path)?;

                println!(
                    "✅ Template '{}' applied successfully",
                    applied.template_name
                );
                println!("📁 Files generated:");

                if let Some(output_path) = output_path {
                    println!("   • {}", output_path.join("pattern.yaml").display());

                    if !applied.applied_snippets.is_empty() {
                        println!(
                            "   • {} ({} snippets)",
                            output_path.join("snippets").display(),
                            applied.applied_snippets.len()
                        );
                    }

                    if !applied.applied_configs.is_empty() {
                        println!(
                            "   • {} ({} configs)",
                            output_path.join("configs").display(),
                            applied.applied_configs.len()
                        );
                    }

                    if !applied.applied_tests.is_empty() {
                        println!(
                            "   • {} ({} tests)",
                            output_path.join("tests").display(),
                            applied.applied_tests.len()
                        );
                    }
                }
            }

            Ok(())
        }

        TemplateSubcommands::Search {
            query,
            category,
            tags,
            min_rating,
        } => {
            let tags_vec = tags
                .as_ref()
                .map(|t| t.split(',').map(|s| s.trim()).collect());
            let templates = fileops::search_pattern_templates(
                scope,
                query,
                category.as_deref(),
                tags_vec,
                *min_rating,
            )?;

            if templates.is_empty() {
                println!("🔍 No templates found matching '{}'", query);
            } else {
                println!(
                    "🔍 Found {} template(s) matching '{}':",
                    templates.len(),
                    query
                );
                println!();

                for template in templates {
                    println!("🆔 ID: {}", template.id);
                    println!("📝 Name: {}", template.name);
                    println!("📄 Description: {}", template.description);
                    println!("📂 Category: {}", template.category);
                    println!("📊 Usage Count: {}", template.usage_count);

                    if let Some(rating) = template.rating {
                        println!("⭐ Rating: {:.1}/5.0", rating);
                    }

                    println!("🌱 Maturity: {:?}", template.metadata.maturity);
                    println!();
                }
            }

            Ok(())
        }

        TemplateSubcommands::Update {
            id,
            name,
            description,
            category,
            template_version,
            author,
            tags,
            maturity,
            complexity,
        } => {
            fileops::update_pattern_template(
                scope,
                id,
                name.clone(),
                description.clone(),
                category.clone(),
                None, // variables
                None, // content
                None, // metadata
                template_version.clone(),
                author.clone(),
                tags.as_ref()
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect()),
            )?;

            println!("✅ Template '{}' updated successfully", id);
            Ok(())
        }

        TemplateSubcommands::Delete { id, force } => {
            if !*force {
                println!(
                    "⚠️  Are you sure you want to delete template '{}'? (y/N)",
                    id
                );
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if !input.trim().to_lowercase().starts_with('y') {
                    println!("❌ Template deletion cancelled");
                    return Ok(());
                }
            }

            fileops::delete_pattern_template(scope, id)?;
            println!("✅ Template '{}' deleted successfully", id);
            Ok(())
        }

        TemplateSubcommands::Stats { detailed: _ } => {
            let stats = fileops::get_template_statistics(scope)?;

            println!("📊 Template Statistics");
            println!("{}", "─".repeat(50));
            println!("📋 Total Templates: {}", stats.total_templates);
            println!("📊 Total Usage: {}", stats.total_usage);
            println!("⭐ Average Rating: {:.1}/5.0", stats.average_rating);

            if !stats.templates_by_category.is_empty() {
                println!("\n📂 By Category:");
                for (category, count) in &stats.templates_by_category {
                    println!("   • {}: {}", category, count);
                }
            }

            if !stats.templates_by_maturity.is_empty() {
                println!("\n🌱 By Maturity:");
                for (maturity, count) in &stats.templates_by_maturity {
                    println!("   • {}: {}", maturity, count);
                }
            }

            if !stats.top_templates.is_empty() {
                println!("\n⭐ Top Templates (by usage):");
                for (i, template) in stats.top_templates.iter().enumerate() {
                    println!(
                        "   {}. {} ({} uses)",
                        i + 1,
                        template.name,
                        template.usage_count
                    );
                    if let Some(rating) = template.rating {
                        println!("      Rating: {:.1}/5.0", rating);
                    }
                }
            }

            Ok(())
        }
    }
}

// Helper functions for parsing template data

fn parse_template_variables(variables_str: &str) -> RhemaResult<Vec<TemplateVariable>> {
    // Simple parsing for now - could be enhanced with JSON parsing
    let mut variables = Vec::new();

    for line in variables_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 {
            let name = parts[0].trim().to_string();
            let var_type = match parts[1].trim().to_lowercase().as_str() {
                "string" => VariableType::String,
                "number" => VariableType::Number,
                "boolean" => VariableType::Boolean,
                "enum" => VariableType::Enum,
                "filepath" => VariableType::FilePath,
                "url" => VariableType::Url,
                "email" => VariableType::Email,
                _ => VariableType::String,
            };
            let description = parts[2].trim().to_string();

            variables.push(TemplateVariable {
                name,
                description,
                var_type,
                default_value: None,
                required: true,
                validation: None,
                examples: None,
            });
        }
    }

    Ok(variables)
}

// Helper functions for generating content templates

fn generate_code_snippet_content(language: &str, snippet_type: &SnippetType, name: &str) -> String {
    match (language.to_lowercase().as_str(), snippet_type) {
        ("rust", SnippetType::Implementation) => format!(
            "pub fn {}() {{\n    // TODO: Implement {}\n    todo!(\"Implement {}\");\n}}",
            name.to_lowercase().replace(' ', "_"),
            name,
            name
        ),
        ("rust", SnippetType::Interface) => format!(
            "pub trait {} {{\n    // TODO: Define interface methods\n    fn {}();\n}}",
            name.replace(' ', ""),
            name.to_lowercase().replace(' ', "_")
        ),
        ("rust", SnippetType::Configuration) => format!(
            "#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]\npub struct {} {{\n    // TODO: Add configuration fields\n    pub field1: String,\n    pub field2: u32,\n}}",
            name.replace(' ', "")
        ),
        ("rust", SnippetType::Test) => format!(
            "#[cfg(test)]\nmod tests {{\n    use super::*;\n\n    #[test]\n    fn test_{}() {{\n        // TODO: Add test implementation\n        assert!(true);\n    }}\n}}",
            name.to_lowercase().replace(' ', "_")
        ),
        ("javascript", SnippetType::Implementation) => format!(
            "function {}() {{\n    // TODO: Implement {}\n    throw new Error('Not implemented');\n}}",
            name.to_lowercase().replace(' ', "_"),
            name
        ),
        ("javascript", SnippetType::Interface) => format!(
            "/**\n * @interface {}\n */\nclass {} {{\n    // TODO: Define interface methods\n    {}(param) {{\n        throw new Error('Not implemented');\n    }}\n}}",
            name,
            name.replace(' ', ""),
            name.to_lowercase().replace(' ', "_")
        ),
        ("python", SnippetType::Implementation) => format!(
            "def {}():\n    \"\"\"TODO: Implement {}\"\"\"\n    raise NotImplementedError(\"Not implemented\")\n",
            name.to_lowercase().replace(' ', "_"),
            name
        ),
        ("python", SnippetType::Test) => format!(
            "import unittest\n\nclass Test{}(unittest.TestCase):\n    def test_{}(self):\n        \"\"\"TODO: Add test implementation\"\"\"\n        self.assertTrue(True)\n",
            name.replace(' ', ""),
            name.to_lowercase().replace(' ', "_")
        ),
        _ => format!(
            "// TODO: Add {} implementation for {}\n// This is a {} snippet\n",
            language,
            name,
            format!("{:?}", snippet_type).to_lowercase()
        ),
    }
}

fn generate_configuration_content(format: &ConfigFormat, name: &str) -> String {
    match format {
        ConfigFormat::Json => format!(
            "{{\n  \"{}\": {{\n    \"enabled\": true,\n    \"version\": \"1.0.0\",\n    \"settings\": {{\n      \"setting1\": \"value1\",\n      \"setting2\": 42\n    }}\n  }}\n}}",
            name.to_lowercase().replace(' ', "_")
        ),
        ConfigFormat::Yaml => format!(
            "{}:\n  enabled: true\n  version: \"1.0.0\"\n  settings:\n    setting1: value1\n    setting2: 42",
            name.to_lowercase().replace(' ', "_")
        ),
        ConfigFormat::Toml => format!(
            "[{}]\nenabled = true\nversion = \"1.0.0\"\n\n[{}.settings]\nsetting1 = \"value1\"\nsetting2 = 42",
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_")
        ),
        ConfigFormat::Ini => format!(
            "[{}]\nenabled=true\nversion=1.0.0\n\n[{}_settings]\nsetting1=value1\nsetting2=42",
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_")
        ),
        ConfigFormat::Properties => format!(
            "# {} Configuration\n{}.enabled=true\n{}.version=1.0.0\n{}.settings.setting1=value1\n{}.settings.setting2=42",
            name,
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_")
        ),
        ConfigFormat::Custom => format!(
            "# Custom configuration for {}\n# TODO: Define custom configuration format\n",
            name
        ),
    }
}

fn generate_test_content(framework: &str, test_type: &TestType, name: &str) -> String {
    match (framework.to_lowercase().as_str(), test_type) {
        ("jest", TestType::Unit) => format!(
            "describe('{}', () => {{\n  test('should work correctly', () => {{\n    // TODO: Add test implementation\n    expect(true).toBe(true);\n  }});\n}});",
            name
        ),
        ("jest", TestType::Integration) => format!(
            "describe('{} Integration', () => {{\n  beforeAll(async () => {{\n    // TODO: Setup integration test environment\n  }});\n\n  test('should integrate correctly', async () => {{\n    // TODO: Add integration test implementation\n    expect(true).toBe(true);\n  }});\n}});",
            name
        ),
        ("pytest", TestType::Unit) => format!(
            "import pytest\n\n\ndef test_{}():\n    \"\"\"Test {}\"\"\"\n    # TODO: Add test implementation\n    assert True\n",
            name.to_lowercase().replace(' ', "_"),
            name
        ),
        ("pytest", TestType::Integration) => format!(
            "import pytest\n\n\n@pytest.fixture\ndef {}_fixture():\n    \"\"\"Setup fixture for {} integration tests\"\"\"\n    # TODO: Add fixture implementation\n    return {{}}\n\n\ndef test_{}_integration({}_fixture):\n    \"\"\"Test {} integration\"\"\"\n    # TODO: Add integration test implementation\n    assert True\n",
            name.to_lowercase().replace(' ', "_"),
            name,
            name.to_lowercase().replace(' ', "_"),
            name.to_lowercase().replace(' ', "_"),
            name
        ),
        ("cargo", TestType::Unit) => format!(
            "#[cfg(test)]\nmod tests {{\n    use super::*;\n\n    #[test]\n    fn test_{}() {{\n        // TODO: Add test implementation\n        assert!(true);\n    }}\n}}",
            name.to_lowercase().replace(' ', "_")
        ),
        ("cargo", TestType::Integration) => format!(
            "#[cfg(test)]\nmod integration_tests {{\n    use super::*;\n\n    #[test]\n    fn test_{}_integration() {{\n        // TODO: Add integration test implementation\n        assert!(true);\n    }}\n}}",
            name.to_lowercase().replace(' ', "_")
        ),
        _ => format!(
            "// TODO: Add {} test implementation for {}\n// This is a {} test\n",
            framework,
            name,
            format!("{:?}", test_type).to_lowercase()
        ),
    }
}

fn parse_code_snippets(snippets_str: &str) -> RhemaResult<Option<Vec<CodeSnippet>>> {
    // Simple parsing - could be enhanced with JSON parsing
    let mut snippets = Vec::new();

    for line in snippets_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            let name = parts[0].trim().to_string();
            let language = parts[1].trim().to_string();
            let snippet_type = match parts[2].trim().to_lowercase().as_str() {
                "implementation" => SnippetType::Implementation,
                "interface" => SnippetType::Interface,
                "configuration" => SnippetType::Configuration,
                "test" => SnippetType::Test,
                "documentation" => SnippetType::Documentation,
                _ => SnippetType::Example,
            };
            let description = parts[3].trim().to_string();

            // Generate a basic code snippet template based on language and type
            let content = generate_code_snippet_content(&language, &snippet_type, &name);

            snippets.push(CodeSnippet {
                name,
                description,
                language,
                content,
                snippet_type,
            });
        }
    }

    if snippets.is_empty() {
        Ok(None)
    } else {
        Ok(Some(snippets))
    }
}

fn parse_configurations(configs_str: &str) -> RhemaResult<Option<Vec<ConfigurationTemplate>>> {
    // Simple parsing - could be enhanced with JSON parsing
    let mut configs = Vec::new();

    for line in configs_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            let name = parts[0].trim().to_string();
            let format = match parts[1].trim().to_lowercase().as_str() {
                "json" => ConfigFormat::Json,
                "yaml" => ConfigFormat::Yaml,
                "toml" => ConfigFormat::Toml,
                "ini" => ConfigFormat::Ini,
                "properties" => ConfigFormat::Properties,
                _ => ConfigFormat::Custom,
            };
            let description = parts[2].trim().to_string();

            // Generate configuration content based on format
            let content = generate_configuration_content(&format, &name);

            configs.push(ConfigurationTemplate {
                name,
                description,
                format,
                content,
                target_path: None,
            });
        }
    }

    if configs.is_empty() {
        Ok(None)
    } else {
        Ok(Some(configs))
    }
}

fn parse_test_templates(tests_str: &str) -> RhemaResult<Option<Vec<TestTemplate>>> {
    // Simple parsing - could be enhanced with JSON parsing
    let mut tests = Vec::new();

    for line in tests_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            let name = parts[0].trim().to_string();
            let framework = parts[1].trim().to_string();
            let test_type = match parts[2].trim().to_lowercase().as_str() {
                "unit" => TestType::Unit,
                "integration" => TestType::Integration,
                "e2e" => TestType::E2e,
                "performance" => TestType::Performance,
                "security" => TestType::Security,
                _ => TestType::Custom,
            };
            let description = parts[3].trim().to_string();

            // Generate test content based on framework and type
            let content = generate_test_content(&framework, &test_type, &name);

            tests.push(TestTemplate {
                name,
                description,
                framework,
                content,
                test_type,
            });
        }
    }

    if tests.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tests))
    }
}

fn parse_variables_string(variables_str: &str) -> RhemaResult<HashMap<String, String>> {
    let mut variables = HashMap::new();

    for pair in variables_str.split(',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }

        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            variables.insert(key, value);
        } else {
            return Err(rhema_api::RhemaError::ConfigError(format!(
                "Invalid variable format: '{}'. Expected 'key=value'",
                pair
            )));
        }
    }

    Ok(variables)
}
