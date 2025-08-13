use crate::{load_prompts, save_prompts};
use crate::{Rhema, RhemaError, RhemaResult};
use rhema_coordination::context_injection::{EnhancedContextInjector, TaskType};
use rhema_core::schema::{
    AdvancedVariable, CompositionBlock, CompositionBlockType, ContextCacheConfig,
    ContextLearningConfig, ContextOptimizationConfig, ContextQualityMetrics, ContextRule,
    ContextInjectionMethod, PromptPattern, TemplatePerformanceMetrics, TemplateValidationRule,
    ValidationRuleType, ValidationSeverity, VariableConstraints, VariableType, VariableValidation,
};
use std::collections::HashMap;

// PromptSubcommands will be defined in this module

#[derive(clap::Subcommand)]
pub enum PromptSubcommands {
    /// Add a new prompt pattern
    Add {
        /// Prompt name
        #[arg(value_name = "NAME")]
        name: String,

        /// Prompt description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Prompt content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// Injection method (prepend, append, template_variable)
        #[arg(long, value_name = "METHOD")]
        injection: Option<String>,

        /// Base template to extend from
        #[arg(long, value_name = "BASE_TEMPLATE")]
        extends: Option<String>,

        /// Template variables (key=value,key2=value2)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,
    },

    /// List prompt patterns
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tags
        #[arg(long, value_name = "TAGS")]
        tag: Option<String>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Show only patterns with context rules
        #[arg(long)]
        with_context_rules: bool,

        /// Show only patterns with template variables
        #[arg(long)]
        with_variables: bool,
    },

    /// Update a prompt pattern
    Update {
        /// Prompt ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,

        /// New injection method
        #[arg(long, value_name = "METHOD")]
        injection: Option<String>,

        /// New base template to extend from
        #[arg(long, value_name = "BASE_TEMPLATE")]
        extends: Option<String>,

        /// New template variables (key=value,key2=value2)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,
    },

    /// Delete a prompt pattern
    Delete {
        /// Prompt ID
        #[arg(value_name = "ID")]
        id: String,
    },

    /// Record usage and feedback for a prompt pattern
    Record {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Whether the usage was successful
        #[arg(long)]
        successful: bool,

        /// User feedback
        #[arg(long, value_name = "FEEDBACK")]
        feedback: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Show analytics for a prompt pattern
    Analytics {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,

        /// Show detailed feedback history
        #[arg(long)]
        detailed: bool,
    },

    /// Test a prompt pattern with context injection
    Test {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Task type for context injection
        #[arg(long, value_name = "TASK_TYPE")]
        task_type: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,

        /// Show context files that would be injected
        #[arg(long)]
        show_context: bool,
    },

    /// Create a new version of a prompt pattern
    Version {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// New version string
        #[arg(long, value_name = "VERSION")]
        version: String,

        /// New template content
        #[arg(long, value_name = "TEMPLATE")]
        template: Option<String>,

        /// Version description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Changes in this version
        #[arg(long, value_name = "CHANGES")]
        changes: Option<String>,

        /// Author of this version
        #[arg(long, value_name = "AUTHOR")]
        author: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Show version history for a prompt pattern
    History {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Specific version to show
        #[arg(long, value_name = "VERSION")]
        version: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Add context rules to a prompt pattern
    AddContextRule {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Condition expression
        #[arg(long, value_name = "CONDITION")]
        condition: String,

        /// Context files (comma-separated)
        #[arg(long, value_name = "FILES")]
        context_files: String,

        /// Injection method (prepend, append, template_variable)
        #[arg(long, value_name = "METHOD")]
        injection_method: String,

        /// Priority (higher number = higher priority)
        #[arg(long, value_name = "PRIORITY")]
        priority: Option<u8>,

        /// Variables for this rule (key=value,key2=value2)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// List context rules for a prompt pattern
    ListContextRules {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Add template variables to a prompt pattern
    AddVariable {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Variable name
        #[arg(long, value_name = "NAME")]
        name: String,

        /// Variable type (string, number, boolean, array, object, enum, date, email, url, filepath, json)
        #[arg(long, value_name = "TYPE")]
        var_type: String,

        /// Default value
        #[arg(long, value_name = "DEFAULT")]
        default_value: Option<String>,

        /// Variable description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Whether variable is required
        #[arg(long)]
        required: bool,

        /// Minimum length (for strings/arrays)
        #[arg(long, value_name = "MIN_LENGTH")]
        min_length: Option<usize>,

        /// Maximum length (for strings/arrays)
        #[arg(long, value_name = "MAX_LENGTH")]
        max_length: Option<usize>,

        /// Regular expression pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,

        /// Allowed values for enum types (comma-separated)
        #[arg(long, value_name = "ALLOWED_VALUES")]
        allowed_values: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// List template variables for a prompt pattern
    ListVariables {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Add composition blocks to a prompt pattern
    AddCompositionBlock {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Block ID
        #[arg(long, value_name = "ID")]
        block_id: String,

        /// Block type (conditional, loop, include, switch, fallback)
        #[arg(long, value_name = "TYPE")]
        block_type: String,

        /// Block content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Condition expression for conditional blocks
        #[arg(long, value_name = "CONDITION")]
        condition: Option<String>,

        /// Loop variable for loop blocks
        #[arg(long, value_name = "LOOP_VARIABLE")]
        loop_variable: Option<String>,

        /// Loop items for loop blocks (comma-separated)
        #[arg(long, value_name = "LOOP_ITEMS")]
        loop_items: Option<String>,

        /// Block priority for ordering
        #[arg(long, value_name = "PRIORITY")]
        priority: Option<u8>,

        /// Block variables (key=value,key2=value2)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// List composition blocks for a prompt pattern
    ListCompositionBlocks {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Filter by block type
        #[arg(long, value_name = "TYPE")]
        block_type: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Add validation rules to a prompt pattern
    AddValidationRule {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Rule ID
        #[arg(long, value_name = "ID")]
        rule_id: String,

        /// Rule type (required, format, length, range, custom, dependency, consistency)
        #[arg(long, value_name = "TYPE")]
        rule_type: String,

        /// Rule condition expression
        #[arg(long, value_name = "CONDITION")]
        condition: String,

        /// Error message for failed validation
        #[arg(long, value_name = "ERROR_MESSAGE")]
        error_message: String,

        /// Rule severity (error, warning, info)
        #[arg(long, value_name = "SEVERITY")]
        severity: String,

        /// Whether rule is enabled
        #[arg(long)]
        enabled: bool,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// List validation rules for a prompt pattern
    ListValidationRules {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Configure context caching for a prompt pattern
    ConfigureCache {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Enable caching
        #[arg(long)]
        enabled: bool,

        /// Cache TTL in seconds
        #[arg(long, value_name = "TTL_SECONDS")]
        ttl_seconds: Option<u64>,

        /// Maximum cache size in bytes
        #[arg(long, value_name = "MAX_SIZE_BYTES")]
        max_size_bytes: Option<u64>,

        /// Cache invalidation strategy (time_based, event_based, manual, adaptive)
        #[arg(long, value_name = "STRATEGY")]
        invalidation_strategy: Option<String>,

        /// Enable cache compression
        #[arg(long)]
        compression_enabled: bool,

        /// Enable cache persistence
        #[arg(long)]
        persistence_enabled: bool,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Configure context optimization for a prompt pattern
    ConfigureOptimization {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Enable optimization
        #[arg(long)]
        enabled: bool,

        /// Maximum context size in tokens
        #[arg(long, value_name = "MAX_TOKENS")]
        max_tokens: Option<usize>,

        /// Minimum relevance score
        #[arg(long, value_name = "MIN_RELEVANCE")]
        min_relevance_score: Option<f64>,

        /// Enable semantic compression
        #[arg(long)]
        semantic_compression: bool,

        /// Enable structure optimization
        #[arg(long)]
        structure_optimization: bool,

        /// Enable relevance filtering
        #[arg(long)]
        relevance_filtering: bool,

        /// Optimization algorithm (greedy, dynamic_programming, machine_learning, hybrid)
        #[arg(long, value_name = "ALGORITHM")]
        algorithm: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Configure context learning for a prompt pattern
    ConfigureLearning {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Enable learning
        #[arg(long)]
        enabled: bool,

        /// Learning rate
        #[arg(long, value_name = "RATE")]
        learning_rate: Option<f64>,

        /// Minimum sample size for learning
        #[arg(long, value_name = "MIN_SAMPLE_SIZE")]
        min_sample_size: Option<usize>,

        /// Learning window size
        #[arg(long, value_name = "WINDOW_SIZE")]
        window_size: Option<usize>,

        /// Feedback weight
        #[arg(long, value_name = "FEEDBACK_WEIGHT")]
        feedback_weight: Option<f64>,

        /// Success threshold
        #[arg(long, value_name = "SUCCESS_THRESHOLD")]
        success_threshold: Option<f64>,

        /// Learning algorithm (reinforcement, supervised, unsupervised, online)
        #[arg(long, value_name = "ALGORITHM")]
        algorithm: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Update context quality metrics for a prompt pattern
    UpdateQuality {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Relevance score (0.0 to 1.0)
        #[arg(long, value_name = "RELEVANCE")]
        relevance_score: f64,

        /// Completeness score (0.0 to 1.0)
        #[arg(long, value_name = "COMPLETENESS")]
        completeness_score: f64,

        /// Accuracy score (0.0 to 1.0)
        #[arg(long, value_name = "ACCURACY")]
        accuracy_score: f64,

        /// Timeliness score (0.0 to 1.0)
        #[arg(long, value_name = "TIMELINESS")]
        timeliness_score: f64,

        /// Improvement suggestions (comma-separated)
        #[arg(long, value_name = "SUGGESTIONS")]
        improvement_suggestions: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Show context quality metrics for a prompt pattern
    ShowQuality {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Render a prompt pattern with composition blocks
    Render {
        /// Prompt pattern ID or name
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Context content
        #[arg(long, value_name = "CONTEXT")]
        context: String,

        /// Variables (key=value,key2=value2)
        #[arg(long, value_name = "VARIABLES")]
        variables: Option<String>,

        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
}

pub fn run(rhema: &Rhema, subcommand: &PromptSubcommands) -> RhemaResult<()> {
    match subcommand {
        PromptSubcommands::Add {
            name,
            description,
            content,
            category,
            tags,
            injection,
            extends,
            variables,
        } => add_prompt(
            rhema,
            name,
            &description.as_ref().unwrap_or(&"".to_string()),
            content,
            category,
            tags,
            injection,
            extends,
            variables,
        ),
        PromptSubcommands::List {
            category,
            tag,
            detailed,
            with_context_rules,
            with_variables,
        } => list_prompts(rhema, category, tag, *detailed, *with_context_rules, *with_variables),
        PromptSubcommands::Update {
            id,
            name,
            description,
            content,
            category,
            tags,
            injection,
            extends,
            variables,
        } => update_prompt(
            rhema,
            id,
            name,
            description,
            content,
            category,
            tags,
            injection,
            extends,
            variables,
        ),
        PromptSubcommands::Delete { id } => delete_prompt(rhema, id),
        PromptSubcommands::Record {
            pattern,
            successful,
            feedback,
            scope,
        } => record_usage(rhema, pattern, *successful, feedback, scope),
        PromptSubcommands::Analytics {
            pattern,
            scope,
            detailed,
        } => show_analytics(rhema, pattern, scope, *detailed),
        PromptSubcommands::Test {
            pattern,
            task_type,
            scope,
            show_context,
        } => test_prompt(rhema, pattern, scope, task_type.as_deref(), *show_context),
        PromptSubcommands::Version {
            pattern,
            version,
            template,
            description,
            changes,
            author,
            scope,
        } => create_version(rhema, pattern, version, template, description, changes, author, scope),
        PromptSubcommands::History {
            pattern,
            version,
            scope,
        } => show_version(rhema, pattern, version.as_deref(), scope),
        PromptSubcommands::AddContextRule {
            pattern,
            condition,
            context_files,
            injection_method,
            priority,
            variables,
            scope,
        } => add_context_rule(
            rhema,
            pattern,
            condition,
            context_files,
            injection_method,
            priority,
            variables,
            scope,
        ),
        PromptSubcommands::ListContextRules { pattern, scope } => {
            list_context_rules(rhema, pattern, scope)
        }
        PromptSubcommands::AddVariable {
            pattern,
            name,
            var_type,
            default_value,
            description,
            required,
            min_length,
            max_length,
            pattern: var_pattern,
            allowed_values,
            scope,
        } => add_variable(
            rhema,
            pattern,
            name,
            var_type,
            default_value,
            description,
            *required,
            min_length,
            max_length,
            var_pattern,
            allowed_values,
            scope,
        ),
        PromptSubcommands::ListVariables { pattern, scope } => {
            list_variables(rhema, pattern, scope)
        }
        PromptSubcommands::AddCompositionBlock {
            pattern,
            block_id,
            block_type,
            content,
            condition,
            loop_variable,
            loop_items,
            priority,
            variables,
            scope,
        } => add_composition_block(
            rhema,
            pattern,
            block_id,
            block_type,
            content,
            condition,
            loop_variable,
            loop_items,
            priority,
            variables,
            scope,
        ),
        PromptSubcommands::ListCompositionBlocks {
            pattern,
            block_type,
            scope,
        } => list_composition_blocks(rhema, pattern, block_type.as_deref(), scope),
        PromptSubcommands::AddValidationRule {
            pattern,
            rule_id,
            rule_type,
            condition,
            error_message,
            severity,
            enabled,
            scope,
        } => add_validation_rule(
            rhema,
            pattern,
            rule_id,
            rule_type,
            condition,
            error_message,
            severity,
            *enabled,
            scope,
        ),
        PromptSubcommands::ListValidationRules { pattern, scope } => {
            list_validation_rules(rhema, pattern, scope)
        }
        PromptSubcommands::ConfigureCache {
            pattern,
            enabled,
            ttl_seconds,
            max_size_bytes,
            invalidation_strategy,
            compression_enabled,
            persistence_enabled,
            scope,
        } => configure_cache(
            rhema,
            pattern,
            *enabled,
            ttl_seconds,
            max_size_bytes,
            invalidation_strategy,
            *compression_enabled,
            *persistence_enabled,
            scope,
        ),
        PromptSubcommands::ConfigureOptimization {
            pattern,
            enabled,
            max_tokens,
            min_relevance_score,
            semantic_compression,
            structure_optimization,
            relevance_filtering,
            algorithm,
            scope,
        } => configure_optimization(
            rhema,
            pattern,
            *enabled,
            max_tokens,
            min_relevance_score,
            *semantic_compression,
            *structure_optimization,
            *relevance_filtering,
            algorithm,
            scope,
        ),
        PromptSubcommands::ConfigureLearning {
            pattern,
            enabled,
            learning_rate,
            min_sample_size,
            window_size,
            feedback_weight,
            success_threshold,
            algorithm,
            scope,
        } => configure_learning(
            rhema,
            pattern,
            *enabled,
            learning_rate,
            min_sample_size,
            window_size,
            feedback_weight,
            success_threshold,
            algorithm,
            scope,
        ),
        PromptSubcommands::UpdateQuality {
            pattern,
            relevance_score,
            completeness_score,
            accuracy_score,
            timeliness_score,
            improvement_suggestions,
            scope,
        } => update_quality(
            rhema,
            pattern,
            *relevance_score,
            *completeness_score,
            *accuracy_score,
            *timeliness_score,
            improvement_suggestions,
            scope,
        ),
        PromptSubcommands::ShowQuality { pattern, scope } => {
            show_quality(rhema, pattern, scope)
        }
        PromptSubcommands::Render {
            pattern,
            context,
            variables,
            scope,
        } => render_prompt(rhema, pattern, context, variables, scope),
    }
}

fn add_prompt(
    rhema: &Rhema,
    name: &str,
    description: &str,
    content: &str,
    category: &Option<String>,
    tags: &Option<String>,
    injection: &Option<String>,
    extends: &Option<String>,
    variables: &Option<String>,
) -> RhemaResult<()> {
    // TODO: Implement add prompt
    println!("✅ Added prompt: {}", name);
    println!("   Description: {}", description);
    println!("   Content: {}", content);
    if let Some(cat) = category {
        println!("   Category: {}", cat);
    }
    if let Some(tags) = tags {
        println!("   Tags: {}", tags);
    }
    if let Some(injection_method) = injection {
        println!("   Injection Method: {}", injection_method);
    }
    if let Some(extends_template) = extends {
        println!("   Extends: {}", extends_template);
    }
    if let Some(vars) = variables {
        println!("   Variables: {}", vars);
    }
    Ok(())
}

fn list_prompts(
    rhema: &Rhema,
    category: &Option<String>,
    tag: &Option<String>,
    detailed: bool,
    with_context_rules: bool,
    with_variables: bool,
) -> RhemaResult<()> {
    let scope_path = rhema.get_current_scope_path()?;

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        println!("No prompts.yaml found in {}", scope_path.display());
        return Ok(());
    }

    let prompts = load_prompts(&prompts_path)?;

    // Filter by tags if specified
    let mut filtered_prompts = if let Some(tags_str) = tag {
        let filter_tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        prompts
            .prompts
            .into_iter()
            .filter(|p| {
                if let Some(pattern_tags) = &p.tags {
                    filter_tags.iter().any(|tag| pattern_tags.contains(tag))
                } else {
                    false
                }
            })
            .collect()
    } else {
        prompts.prompts
    };

    // Additional filters
    if with_context_rules {
        filtered_prompts.retain(|p| p.context_rules.is_some() && !p.context_rules.as_ref().unwrap().is_empty());
    }

    if with_variables {
        filtered_prompts.retain(|p| p.advanced_variables.is_some() && !p.advanced_variables.as_ref().unwrap().is_empty());
    }

    if filtered_prompts.is_empty() {
        println!("No prompt patterns found");
        return Ok(());
    }

    println!("📝 Prompt Patterns in {}:", scope_path.display());
    println!("{}", "=".repeat(60));

    for pattern in filtered_prompts {
        println!("ID: {}", pattern.id);
        println!("Name: {}", pattern.name);
        if let Some(desc) = pattern.description {
            println!("Description: {}", desc);
        }
        println!(
            "Version: {} (created {})",
            pattern.version.current,
            pattern.version.created_at.format("%Y-%m-%d")
        );
        println!("Injection: {:?}", pattern.injection);
        println!(
            "Usage: {}/{} successful ({:.1}%)",
            pattern.usage_analytics.successful_uses,
            pattern.usage_analytics.total_uses,
            pattern.usage_analytics.success_rate() * 100.0
        );
        if let Some(last_used) = pattern.usage_analytics.last_used {
            println!("Last used: {}", last_used.format("%Y-%m-%d %H:%M"));
        }
        if let Some(tags) = pattern.tags {
            println!("Tags: {}", tags.join(", "));
        }
        if let Some(extends) = &pattern.extends {
            println!("Extends: {}", extends);
        }
        if let Some(variables) = &pattern.variables {
            println!("Variables: {:?}", variables);
        }
        if let Some(context_rules) = &pattern.context_rules {
            println!("Context Rules: {} rules", context_rules.len());
        }
        if let Some(advanced_variables) = &pattern.advanced_variables {
            println!("Advanced Variables: {} variables", advanced_variables.len());
        }
        if let Some(composition_blocks) = &pattern.composition_blocks {
            println!("Composition Blocks: {} blocks", composition_blocks.len());
        }
        if let Some(validation_rules) = &pattern.validation_rules {
            println!("Validation Rules: {} rules", validation_rules.len());
        }
        if detailed {
            println!("Template:");
            println!("{}", pattern.template);
        }
        println!("{}", "-".repeat(40));
    }

    Ok(())
}

fn record_usage(
    rhema: &Rhema,
    pattern: &str,
    successful: bool,
    feedback: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    // Record the usage
    prompts.prompts[pattern_index]
        .usage_analytics
        .record_usage(successful, feedback.clone());

    save_prompts(&prompts_path, &prompts)?;

    let status = if successful {
        "✅ successful"
    } else {
        "❌ unsuccessful"
    };
    println!("📊 Recorded {} usage for '{}'", status, pattern);
    println!(
        "   New success rate: {:.1}% ({}/{})",
        prompts.prompts[pattern_index]
            .usage_analytics
            .success_rate()
            * 100.0,
        prompts.prompts[pattern_index]
            .usage_analytics
            .successful_uses,
        prompts.prompts[pattern_index].usage_analytics.total_uses
    );

    Ok(())
}

fn show_analytics(rhema: &Rhema, pattern: &str, scope: &Option<String>, detailed: bool) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📊 Analytics for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));
    println!("Total uses: {}", pattern_entry.usage_analytics.total_uses);
    println!(
        "Successful uses: {}",
        pattern_entry.usage_analytics.successful_uses
    );
    println!(
        "Success rate: {:.1}%",
        pattern_entry.usage_analytics.success_rate() * 100.0
    );

    if let Some(last_used) = pattern_entry.usage_analytics.last_used {
        println!("Last used: {}", last_used.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("Last used: Never");
    }

    if !pattern_entry.usage_analytics.feedback_history.is_empty() {
        println!("\n📝 Recent Feedback:");
        println!("{}", "-".repeat(40));

        // Show last 5 feedback entries
        let recent_feedback: Vec<_> = pattern_entry
            .usage_analytics
            .feedback_history
            .iter()
            .rev()
            .take(5)
            .collect();

        for feedback in recent_feedback {
            let status = if feedback.successful { "✅" } else { "❌" };
            println!(
                "{} {} - {}",
                status,
                feedback.timestamp.format("%Y-%m-%d %H:%M"),
                feedback.feedback
            );
        }
    }

    Ok(())
}

fn test_prompt(
    rhema: &Rhema,
    pattern: &str,
    scope: &Option<String>,
    task_type: Option<&str>,
    show_context: bool,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    // Parse task type if provided
    let parsed_task_type = if let Some(task_str) = task_type {
        Some(parse_task_type(task_str)?)
    } else {
        None
    };

    // Create context injector
    let injector = EnhancedContextInjector::new(scope_path.clone());

    // Inject context into the prompt
    let final_prompt = injector.inject_context(pattern_entry, parsed_task_type.clone())?;

    println!("🎯 Testing Prompt Pattern: {}", pattern_entry.name);
    println!("{}", "=".repeat(60));

    if let Some(task) = parsed_task_type {
        println!("Task Type: {:?}", task);
    } else {
        println!("Task Type: Auto-detected");
    }

    println!("Template:");
    println!("{}", pattern_entry.template);
    println!("{}", "=".repeat(60));
    println!("Final Prompt with Context:");
    println!("{}", final_prompt);

    Ok(())
}

fn test_prompt_with_task(
    rhema: &Rhema,
    pattern: &str,
    task_type: &str,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let _parsed_task = parse_task_type(task_type)?;
    test_prompt(rhema, pattern, scope, Some(task_type), false)
}

fn parse_task_type(task_str: &str) -> RhemaResult<TaskType> {
    match task_str.to_lowercase().as_str() {
        "code_review" | "review" => Ok(TaskType::CodeReview),
        "bug_fix" | "fix" | "bug" => Ok(TaskType::BugFix),
        "feature" | "feature_development" | "feat" => Ok(TaskType::FeatureDevelopment),
        "testing" | "test" => Ok(TaskType::Testing),
        "documentation" | "docs" => Ok(TaskType::Documentation),
        "refactoring" | "refactor" => Ok(TaskType::Refactoring),
        "security" | "security_review" => Ok(TaskType::SecurityReview),
        "performance" | "perf" | "optimization" => Ok(TaskType::PerformanceOptimization),
        "dependency" | "deps" | "update" => Ok(TaskType::DependencyUpdate),
        "deployment" | "deploy" => Ok(TaskType::Deployment),
        _ => Ok(TaskType::Custom(task_str.to_string())),
    }
}

fn create_version(
    rhema: &Rhema,
    pattern: &str,
    new_version: &str,
    template: &Option<String>,
    description: &Option<String>,
    changes: &Option<String>,
    author: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Get the new template (use current if not provided)
    let new_template = template.as_deref().unwrap_or(&pattern_entry.template);

    // Get description (use default if not provided)
    let version_description = description.as_deref().unwrap_or("Version update");

    // Parse changes
    let changes_list = if let Some(changes_str) = changes {
        changes_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        vec!["General improvements".to_string()]
    };

    // Create new version
    pattern_entry.version.create_version(
        new_version,
        new_template,
        version_description,
        changes_list.clone(),
        author.as_deref().map(|s| s.to_string()),
    );

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!(
        "✅ Created version '{}' for pattern '{}'",
        new_version, pattern
    );
    println!("   Description: {}", version_description);
    println!("   Changes: {}", changes_list.join(", "));
    if let Some(author_name) = author {
        println!("   Author: {}", author_name);
    }

    Ok(())
}

fn show_version(
    rhema: &Rhema,
    pattern: &str,
    version: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📋 Version History for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));
    println!("Current version: {}", pattern_entry.version.current);
    println!(
        "Created: {}",
        pattern_entry.version.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "Last updated: {}",
        pattern_entry.version.updated_at.format("%Y-%m-%d %H:%M")
    );
    println!();

    if let Some(specific_version) = version {
        // Show specific version
        if let Some(version_entry) = pattern_entry.version.get_version(specific_version) {
            println!("📝 Version {}:", version_entry.version);
            println!("{}", "-".repeat(40));
            println!("Description: {}", version_entry.description);
            println!(
                "Created: {}",
                version_entry.timestamp.format("%Y-%m-%d %H:%M")
            );
            if let Some(author) = &version_entry.author {
                println!("Author: {}", author);
            }
            println!("Changes:");
            for change in &version_entry.changes {
                println!("  • {}", change);
            }
            println!("\nTemplate:");
            println!("{}", version_entry.template);
        } else {
            println!("❌ Version '{}' not found", specific_version);
            println!("Available versions:");
            for entry in &pattern_entry.version.history {
                println!("  • {}", entry.version);
            }
        }
    } else {
        // Show all versions
        println!("📝 Version History:");
        println!("{}", "-".repeat(40));

        for (i, entry) in pattern_entry.version.history.iter().enumerate() {
            let is_current = entry.version == pattern_entry.version.current;
            let marker = if is_current { "🟢" } else { "⚪" };

            println!(
                "{} Version {} ({})",
                marker,
                entry.version,
                entry.timestamp.format("%Y-%m-%d %H:%M")
            );
            println!("   Description: {}", entry.description);
            if let Some(author) = &entry.author {
                println!("   Author: {}", author);
            }
            println!("   Changes: {}", entry.changes.join(", "));

            if i < pattern_entry.version.history.len() - 1 {
                println!();
            }
        }

        println!("\n🟢 = Current version");
    }

    Ok(())
}

fn update_prompt(
    rhema: &Rhema,
    id: &str,
    name: &Option<String>,
    description: &Option<String>,
    content: &Option<String>,
    category: &Option<String>,
    tags: &Option<String>,
    injection: &Option<String>,
    extends: &Option<String>,
    variables: &Option<String>,
) -> RhemaResult<()> {
    // TODO: Implement update prompt
    println!("Updating prompt: {}", id);
    Ok(())
}

fn delete_prompt(rhema: &Rhema, id: &str) -> RhemaResult<()> {
    // TODO: Implement delete prompt
    println!("Deleting prompt: {}", id);
    Ok(())
}

fn add_context_rule(
    rhema: &Rhema,
    pattern: &str,
    condition: &str,
    context_files: &str,
    injection_method: &str,
    priority: Option<u8>,
    variables: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse context files
    let context_files: Vec<String> = context_files
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Parse variables
    let variables: HashMap<String, String> = if let Some(vars_str) = variables {
        vars_str
            .split(',')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    (parts[0].trim().to_string(), "".to_string())
                }
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Add context rule
    let context_rule = ContextRule {
        condition: condition.to_string(),
        context_files,
        injection_method: parse_injection_method(injection_method)?,
        priority,
        variables: Some(variables),
    };
    
    if let Some(ref mut rules) = pattern_entry.context_rules {
        rules.push(context_rule);
    } else {
        pattern_entry.context_rules = Some(vec![context_rule]);
    }

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Added context rule to '{}'", pattern_entry.name);
    println!("   Condition: {}", condition);
    println!("   Context Files: {}", context_files.join(", "));
    println!("   Injection Method: {:?}", pattern_entry.injection);
    println!("   Priority: {}", priority.unwrap_or(0));
    println!("   Variables: {:?}", variables);

    Ok(())
}

fn list_context_rules(rhema: &Rhema, pattern: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📋 Context Rules for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));

    if let Some(rules) = &pattern_entry.context_rules {
        if rules.is_empty() {
            println!("No context rules found for this pattern.");
        } else {
            for rule in rules {
                println!("Condition: {}", rule.condition);
                println!("Context Files: {}", rule.context_files.join(", "));
                println!("Injection Method: {:?}", rule.injection_method);
                println!("Priority: {:?}", rule.priority);
                println!("Variables: {:?}", rule.variables);
                println!("{}", "-".repeat(40));
            }
        }
    } else {
        println!("No context rules found for this pattern.");
    }

    Ok(())
}

fn add_variable(
    rhema: &Rhema,
    pattern: &str,
    name: &str,
    var_type: &str,
    default_value: Option<&str>,
    description: Option<&str>,
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<&str>,
    allowed_values: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse allowed values for enum types
    let allowed_values: Vec<String> = if let Some(values_str) = allowed_values {
        values_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![]
    };

    // Create validation rules
    let validation = VariableValidation {
        min_length,
        max_length,
        min_value: None,
        max_value: None,
        pattern: pattern.map(|s| s.to_string()),
        custom_validator: None,
    };

    // Create constraints
    let constraints = VariableConstraints {
        allowed_values: if allowed_values.is_empty() { None } else { Some(allowed_values) },
        forbidden_values: None,
        depends_on: None,
        visible_when: None,
    };

    // Add variable
    let advanced_variable = AdvancedVariable {
        name: name.to_string(),
        var_type: parse_variable_type(var_type)?,
        default_value: default_value.map(|s| s.to_string()),
        validation: Some(validation),
        description: description.map(|s| s.to_string()),
        required,
        constraints: Some(constraints),
    };
    
    if let Some(ref mut vars) = pattern_entry.advanced_variables {
        vars.push(advanced_variable);
    } else {
        pattern_entry.advanced_variables = Some(vec![advanced_variable]);
    }

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Added variable '{}' to '{}'", name, pattern_entry.name);
    let var = pattern_entry.advanced_variables.as_ref().unwrap().last().unwrap();
    println!("   Type: {:?}", var.var_type);
    println!("   Required: {}", var.required);
    println!("   Default Value: {:?}", var.default_value);
    println!("   Description: {:?}", var.description);
    if let Some(ref validation) = var.validation {
        println!("   Min Length: {:?}", validation.min_length);
        println!("   Max Length: {:?}", validation.max_length);
        println!("   Pattern: {:?}", validation.pattern);
    }
    if let Some(ref constraints) = var.constraints {
        println!("   Allowed Values: {:?}", constraints.allowed_values);
    }

    Ok(())
}

fn list_variables(rhema: &Rhema, pattern: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📋 Variables for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));

    if let Some(variables) = &pattern_entry.advanced_variables {
        if variables.is_empty() {
            println!("No variables found for this pattern.");
        } else {
            for var in variables {
                println!("Name: {}", var.name);
                println!("Type: {:?}", var.var_type);
                println!("Default Value: {:?}", var.default_value);
                println!("Description: {:?}", var.description);
                println!("Required: {}", var.required);
                if let Some(ref validation) = var.validation {
                    println!("Min Length: {:?}", validation.min_length);
                    println!("Max Length: {:?}", validation.max_length);
                    println!("Pattern: {:?}", validation.pattern);
                }
                if let Some(ref constraints) = var.constraints {
                    println!("Allowed Values: {:?}", constraints.allowed_values);
                }
                println!("{}", "-".repeat(40));
            }
        }
    } else {
        println!("No variables found for this pattern.");
    }

    Ok(())
}

fn add_composition_block(
    rhema: &Rhema,
    pattern: &str,
    block_id: &str,
    block_type: &str,
    content: &str,
    condition: Option<&str>,
    loop_variable: Option<&str>,
    loop_items: Option<&str>,
    priority: Option<u8>,
    variables: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse variables
    let variables: HashMap<String, String> = if let Some(vars_str) = variables {
        vars_str
            .split(',')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    (parts[0].trim().to_string(), "".to_string())
                }
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Parse loop items
    let loop_items_vec: Option<Vec<String>> = if let Some(items_str) = loop_items {
        Some(items_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    } else {
        None
    };

    // Add composition block
    let composition_block = CompositionBlock {
        id: block_id.to_string(),
        block_type: parse_composition_block_type(block_type)?,
        condition: condition.map(|s| s.to_string()),
        loop_variable: loop_variable.map(|s| s.to_string()),
        loop_items: loop_items_vec,
        content: content.to_string(),
        priority,
        variables: Some(variables),
    };
    
    if let Some(ref mut blocks) = pattern_entry.composition_blocks {
        blocks.push(composition_block);
    } else {
        pattern_entry.composition_blocks = Some(vec![composition_block]);
    }

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Added composition block '{}' to '{}'", block_id, pattern_entry.name);
    let block = pattern_entry.composition_blocks.as_ref().unwrap().last().unwrap();
    println!("   Type: {:?}", block.block_type);
    println!("   Content: {}", block.content);
    println!("   Condition: {:?}", block.condition);
    println!("   Loop Variable: {:?}", block.loop_variable);
    println!("   Loop Items: {:?}", block.loop_items);
    println!("   Priority: {:?}", block.priority);
    println!("   Variables: {:?}", block.variables);

    Ok(())
}

fn list_composition_blocks(rhema: &Rhema, pattern: &str, block_type: Option<&str>, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📋 Composition Blocks for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));

    if let Some(blocks) = &pattern_entry.composition_blocks {
        let filtered_blocks: Vec<&CompositionBlock> = if let Some(filter_type) = block_type {
            let parsed_type = parse_composition_block_type(filter_type)?;
            blocks.iter().filter(|b| b.block_type == parsed_type).collect()
        } else {
            blocks.iter().collect()
        };

        if filtered_blocks.is_empty() {
            println!("No composition blocks found for this pattern.");
        } else {
            for block in filtered_blocks {
                println!("ID: {}", block.id);
                println!("Type: {:?}", block.block_type);
                println!("Content: {}", block.content);
                println!("Condition: {:?}", block.condition);
                println!("Loop Variable: {:?}", block.loop_variable);
                println!("Loop Items: {:?}", block.loop_items);
                println!("Priority: {:?}", block.priority);
                println!("Variables: {:?}", block.variables);
                println!("{}", "-".repeat(40));
            }
        }
    } else {
        println!("No composition blocks found for this pattern.");
    }

    Ok(())
}

fn add_validation_rule(
    rhema: &Rhema,
    pattern: &str,
    rule_id: &str,
    rule_type: &str,
    condition: &str,
    error_message: &str,
    severity: &str,
    enabled: bool,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Add validation rule
    let validation_rule = TemplateValidationRule {
        id: rule_id.to_string(),
        rule_type: parse_validation_rule_type(rule_type)?,
        condition: condition.to_string(),
        error_message: error_message.to_string(),
        severity: parse_validation_severity(severity)?,
        enabled,
    };
    
    if let Some(ref mut rules) = pattern_entry.validation_rules {
        rules.push(validation_rule);
    } else {
        pattern_entry.validation_rules = Some(vec![validation_rule]);
    }

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Added validation rule '{}' to '{}'", rule_id, pattern_entry.name);
    println!("   Type: {:?}", pattern_entry.validation_rules.last().unwrap().rule_type);
    println!("   Condition: {}", pattern_entry.validation_rules.last().unwrap().condition);
    println!("   Error Message: {}", pattern_entry.validation_rules.last().unwrap().error_message);
    println!("   Severity: {:?}", pattern_entry.validation_rules.last().unwrap().severity);
    println!("   Enabled: {}", pattern_entry.validation_rules.last().unwrap().enabled);

    Ok(())
}

fn list_validation_rules(rhema: &Rhema, pattern: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📋 Validation Rules for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));

    if let Some(rules) = &pattern_entry.validation_rules {
        if rules.is_empty() {
            println!("No validation rules found for this pattern.");
        } else {
            for rule in rules {
                println!("ID: {}", rule.id);
                println!("Type: {:?}", rule.rule_type);
                println!("Condition: {}", rule.condition);
                println!("Error Message: {}", rule.error_message);
                println!("Severity: {:?}", rule.severity);
                println!("Enabled: {}", rule.enabled);
                println!("{}", "-".repeat(40));
            }
        }
    } else {
        println!("No validation rules found for this pattern.");
    }

    Ok(())
}

fn configure_cache(
    rhema: &Rhema,
    pattern: &str,
    enabled: bool,
    ttl_seconds: Option<u64>,
    max_size_bytes: Option<u64>,
    invalidation_strategy: Option<&str>,
    compression_enabled: bool,
    persistence_enabled: bool,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse invalidation strategy
    let invalidation_strategy_enum = match invalidation_strategy.unwrap_or("time_based").to_lowercase().as_str() {
        "time_based" => rhema_core::schema::CacheInvalidationStrategy::TimeBased,
        "event_based" => rhema_core::schema::CacheInvalidationStrategy::EventBased,
        "manual" => rhema_core::schema::CacheInvalidationStrategy::Manual,
        "adaptive" => rhema_core::schema::CacheInvalidationStrategy::Adaptive,
        _ => rhema_core::schema::CacheInvalidationStrategy::TimeBased,
    };

    // Update cache config
    pattern_entry.context_cache = Some(ContextCacheConfig {
        enabled,
        ttl_seconds: ttl_seconds.unwrap_or(3600),
        max_size_bytes: max_size_bytes.unwrap_or(1024 * 1024), // 1MB default
        invalidation_strategy: invalidation_strategy_enum,
        compression_enabled,
        persistence_enabled,
    });

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Configured cache for '{}'", pattern_entry.name);
    println!("   Enabled: {}", enabled);
    println!("   TTL Seconds: {:?}", ttl_seconds);
    println!("   Max Size Bytes: {:?}", max_size_bytes);
    println!("   Invalidation Strategy: {:?}", pattern_entry.context_cache.as_ref().unwrap().invalidation_strategy);
    println!("   Compression Enabled: {}", compression_enabled);
    println!("   Persistence Enabled: {}", persistence_enabled);

    Ok(())
}

fn configure_optimization(
    rhema: &Rhema,
    pattern: &str,
    enabled: bool,
    max_tokens: Option<usize>,
    min_relevance_score: Option<f64>,
    semantic_compression: bool,
    structure_optimization: bool,
    relevance_filtering: bool,
    algorithm: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse optimization algorithm
    let algorithm_enum = match algorithm.unwrap_or("greedy").to_lowercase().as_str() {
        "greedy" => rhema_core::schema::OptimizationAlgorithm::Greedy,
        "dynamic_programming" => rhema_core::schema::OptimizationAlgorithm::DynamicProgramming,
        "machine_learning" => rhema_core::schema::OptimizationAlgorithm::MachineLearning,
        "hybrid" => rhema_core::schema::OptimizationAlgorithm::Hybrid,
        _ => rhema_core::schema::OptimizationAlgorithm::Greedy,
    };

    // Update optimization config
    pattern_entry.context_optimization = Some(ContextOptimizationConfig {
        enabled,
        max_tokens: max_tokens.unwrap_or(4096),
        min_relevance_score: min_relevance_score.unwrap_or(0.5),
        semantic_compression,
        structure_optimization,
        relevance_filtering,
        algorithm: algorithm_enum,
    });

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Configured optimization for '{}'", pattern_entry.name);
    println!("   Enabled: {}", enabled);
    println!("   Max Tokens: {:?}", max_tokens);
    println!("   Min Relevance Score: {:?}", min_relevance_score);
    println!("   Semantic Compression: {}", semantic_compression);
    println!("   Structure Optimization: {}", structure_optimization);
    println!("   Relevance Filtering: {}", relevance_filtering);
    println!("   Algorithm: {:?}", pattern_entry.context_optimization.as_ref().unwrap().algorithm);

    Ok(())
}

fn configure_learning(
    rhema: &Rhema,
    pattern: &str,
    enabled: bool,
    learning_rate: Option<f64>,
    min_sample_size: Option<usize>,
    window_size: Option<usize>,
    feedback_weight: Option<f64>,
    success_threshold: Option<f64>,
    algorithm: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse learning algorithm
    let algorithm_enum = match algorithm.unwrap_or("reinforcement").to_lowercase().as_str() {
        "reinforcement" => rhema_core::schema::LearningAlgorithm::Reinforcement,
        "supervised" => rhema_core::schema::LearningAlgorithm::Supervised,
        "unsupervised" => rhema_core::schema::LearningAlgorithm::Unsupervised,
        "online" => rhema_core::schema::LearningAlgorithm::Online,
        _ => rhema_core::schema::LearningAlgorithm::Reinforcement,
    };

    // Update learning config
    pattern_entry.context_learning = Some(ContextLearningConfig {
        enabled,
        learning_rate: learning_rate.unwrap_or(0.1),
        min_sample_size: min_sample_size.unwrap_or(10),
        window_size: window_size.unwrap_or(100),
        feedback_weight: feedback_weight.unwrap_or(0.5),
        success_threshold: success_threshold.unwrap_or(0.8),
        algorithm: algorithm_enum,
    });

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Configured learning for '{}'", pattern_entry.name);
    println!("   Enabled: {}", enabled);
    println!("   Learning Rate: {:?}", learning_rate);
    println!("   Min Sample Size: {:?}", min_sample_size);
    println!("   Window Size: {:?}", window_size);
    println!("   Feedback Weight: {:?}", feedback_weight);
    println!("   Success Threshold: {:?}", success_threshold);
    println!("   Algorithm: {:?}", pattern_entry.context_learning.as_ref().unwrap().algorithm);

    Ok(())
}

fn update_quality(
    rhema: &Rhema,
    pattern: &str,
    relevance_score: f64,
    completeness_score: f64,
    accuracy_score: f64,
    timeliness_score: f64,
    improvement_suggestions: Option<&str>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let mut prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_index = prompts
        .prompts
        .iter()
        .position(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    let pattern_entry = &mut prompts.prompts[pattern_index];

    // Parse improvement suggestions
    let improvement_suggestions_vec: Vec<String> = if let Some(suggestions_str) = improvement_suggestions {
        suggestions_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![]
    };

    // Calculate overall score
    let overall_score = (relevance_score + completeness_score + accuracy_score + timeliness_score) / 4.0;

    // Update quality metrics
    pattern_entry.context_quality = Some(ContextQualityMetrics {
        relevance_score,
        completeness_score,
        accuracy_score,
        timeliness_score,
        overall_score,
        assessed_at: chrono::Utc::now(),
        improvement_suggestions: improvement_suggestions_vec,
    });

    // Save updated prompts
    save_prompts(&prompts_path, &prompts)?;

    println!("✅ Updated quality metrics for '{}'", pattern_entry.name);
    println!("   Relevance Score: {}", relevance_score);
    println!("   Completeness Score: {}", completeness_score);
    println!("   Accuracy Score: {}", accuracy_score);
    println!("   Timeliness Score: {}", timeliness_score);
    println!("   Improvement Suggestions: {:?}", pattern_entry.context_quality.as_ref().unwrap().improvement_suggestions);

    Ok(())
}

fn show_quality(rhema: &Rhema, pattern: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    println!("📊 Quality Metrics for '{}':", pattern_entry.name);
    println!("{}", "=".repeat(60));
    if let Some(quality) = &pattern_entry.context_quality {
        println!("Relevance Score: {}", quality.relevance_score);
        println!("Completeness Score: {}", quality.completeness_score);
        println!("Accuracy Score: {}", quality.accuracy_score);
        println!("Timeliness Score: {}", quality.timeliness_score);
        println!("Overall Score: {}", quality.overall_score);
        println!("Assessed At: {}", quality.assessed_at.format("%Y-%m-%d %H:%M:%S"));
        println!("Improvement Suggestions: {:?}", quality.improvement_suggestions);
    } else {
        println!("No quality metrics found for this pattern.");
    }

    Ok(())
}

fn render_prompt(rhema: &Rhema, pattern: &str, context: &str, variables: Option<&str>, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Find pattern by ID or name
    let pattern_entry = prompts
        .prompts
        .iter()
        .find(|p| p.id == pattern || p.name == pattern)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Pattern '{}' not found", pattern)))?;

    // Parse variables
    let variables: HashMap<String, String> = if let Some(vars_str) = variables {
        vars_str
            .split(',')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    (parts[0].trim().to_string(), "".to_string())
                }
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Parse variables
    let variables: HashMap<String, String> = if let Some(vars_str) = variables {
        vars_str
            .split(',')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    (parts[0].trim().to_string(), "".to_string())
                }
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Render the prompt with composition blocks
    let final_prompt = pattern_entry.render_with_composition(context, &variables);

    println!("🎯 Rendering Prompt Pattern: {}", pattern_entry.name);
    println!("{}", "=".repeat(60));

    println!("Template:");
    println!("{}", pattern_entry.template);
    println!("{}", "=".repeat(60));

    println!("Final Prompt with Context:");
    println!("{}", final_prompt);

    Ok(())
}

fn parse_injection_method(method: &str) -> RhemaResult<ContextInjectionMethod> {
    match method.to_lowercase().as_str() {
        "prepend" => Ok(ContextInjectionMethod::Prepend),
        "append" => Ok(ContextInjectionMethod::Append),
        "template_variable" => Ok(ContextInjectionMethod::TemplateVariable),
        _ => Err(RhemaError::InvalidCommand(format!(
            "Invalid injection method: {}. Must be one of: prepend, append, template_variable",
            method
        ))),
    }
}

fn parse_variable_type(var_type: &str) -> RhemaResult<VariableType> {
    match var_type.to_lowercase().as_str() {
        "string" => Ok(VariableType::String),
        "number" => Ok(VariableType::Number),
        "boolean" => Ok(VariableType::Boolean),
        "array" => Ok(VariableType::Array),
        "object" => Ok(VariableType::Object),
        "date" => Ok(VariableType::Date),
        "email" => Ok(VariableType::Email),
        "url" => Ok(VariableType::Url),
        "filepath" => Ok(VariableType::FilePath),
        "json" => Ok(VariableType::Json),
        _ => {
            if var_type.starts_with("enum(") && var_type.ends_with(")") {
                let enum_values = var_type[5..var_type.len() - 1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                Ok(VariableType::Enum(enum_values))
            } else {
                Err(RhemaError::InvalidCommand(format!(
                    "Invalid variable type: {}. Must be one of: string, number, boolean, array, object, enum(values), date, email, url, filepath, json",
                    var_type
                )))
            }
        }
    }
}

fn parse_composition_block_type(block_type: &str) -> RhemaResult<CompositionBlockType> {
    match block_type.to_lowercase().as_str() {
        "conditional" => Ok(CompositionBlockType::Conditional),
        "loop" => Ok(CompositionBlockType::Loop),
        "include" => Ok(CompositionBlockType::Include),
        "switch" => Ok(CompositionBlockType::Switch),
        "fallback" => Ok(CompositionBlockType::Fallback),
        _ => Err(RhemaError::InvalidCommand(format!(
            "Invalid composition block type: {}. Must be one of: conditional, loop, include, switch, fallback",
            block_type
        ))),
    }
}

fn parse_validation_rule_type(rule_type: &str) -> RhemaResult<ValidationRuleType> {
    match rule_type.to_lowercase().as_str() {
        "required" => Ok(ValidationRuleType::Required),
        "format" => Ok(ValidationRuleType::Format),
        "length" => Ok(ValidationRuleType::Length),
        "range" => Ok(ValidationRuleType::Range),
        "custom" => Ok(ValidationRuleType::Custom),
        "dependency" => Ok(ValidationRuleType::Dependency),
        "consistency" => Ok(ValidationRuleType::Consistency),
        _ => Err(RhemaError::InvalidCommand(format!(
            "Invalid validation rule type: {}. Must be one of: required, format, length, range, custom, dependency, consistency",
            rule_type
        ))),
    }
}

fn parse_validation_severity(severity: &str) -> RhemaResult<ValidationSeverity> {
    match severity.to_lowercase().as_str() {
        "error" => Ok(ValidationSeverity::Error),
        "warning" => Ok(ValidationSeverity::Warning),
        "info" => Ok(ValidationSeverity::Info),
        _ => Err(RhemaError::InvalidCommand(format!(
            "Invalid validation severity: {}. Must be one of: error, warning, info",
            severity
        ))),
    }
}
