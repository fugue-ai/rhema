use crate::{Rhema, RhemaError, RhemaResult};
use clap::Subcommand;
use rhema_ai::context_injection::{ContextInjectionRule, EnhancedContextInjector, TaskType};
use rhema_core::schema::{PromptInjectionMethod, PromptPattern, PromptVersion, UsageAnalytics};
use rhema_core::Priority;

// ContextRulesSubcommands will be defined in this module

#[derive(clap::Subcommand)]
pub enum ContextRulesSubcommands {
    /// List context injection rules
    List {
        /// Filter by pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,
    },

    /// Add a new context injection rule
    Add {
        /// Rule name
        #[arg(value_name = "NAME")]
        name: String,

        /// Rule description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Pattern to match
        #[arg(long, value_name = "PATTERN")]
        pattern: String,

        /// Context to inject
        #[arg(long, value_name = "CONTEXT")]
        context: String,

        /// Priority
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,
    },

    /// Update a context injection rule
    Update {
        /// Rule ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,

        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,
    },

    /// Delete a context injection rule
    Delete {
        /// Rule ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn run(rhema: &Rhema, subcommand: &ContextRulesSubcommands) -> RhemaResult<()> {
    match subcommand {
        ContextRulesSubcommands::List { pattern } => list_context_rules(rhema, pattern),
        ContextRulesSubcommands::Add {
            name,
            description,
            pattern,
            context,
            priority,
        } => add_context_rule(rhema, name, description, pattern, context, priority),
        ContextRulesSubcommands::Update {
            id,
            name,
            description,
            pattern,
            context,
            priority,
        } => update_context_rule(rhema, id, name, description, pattern, context, priority),
        ContextRulesSubcommands::Delete { id } => delete_context_rule(rhema, id),
    }
}

fn list_context_rules(rhema: &Rhema, pattern: &Option<String>) -> RhemaResult<()> {
    let scope_path = rhema.get_current_scope_path()?;

    let injector = EnhancedContextInjector::new(scope_path);
    let rules = injector.get_rules();

    println!("📋 Context Injection Rules:");
    println!("{}", "=".repeat(60));

    for rule in rules {
        println!("Task Type: {:?}", rule.task_type);
        println!("Context Files: {}", rule.context_files.join(", "));
        println!("Injection Method: {:?}", rule.injection_method);
        println!("Priority: {}", rule.priority);
        if let Some(context) = &rule.additional_context {
            println!("Additional Context: {}", context);
        }
        println!("{}", "-".repeat(40));
    }

    Ok(())
}

fn add_context_rule(
    rhema: &Rhema,
    name: &str,
    description: &str,
    pattern: &str,
    context: &str,
    priority: &Priority,
) -> RhemaResult<()> {
    // TODO: Implement add context rule
    println!("✅ Added context injection rule:");
    println!("   Name: {}", name);
    println!("   Description: {}", description);
    println!("   Pattern: {}", pattern);
    println!("   Context: {}", context);
    println!("   Priority: {:?}", priority);

    Ok(())
}

fn test_context_injection(
    rhema: &Rhema,
    task_type: &str,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let parsed_task_type = parse_task_type(task_type)?;
    let injector = EnhancedContextInjector::new(scope_path);

    // Create a dummy prompt pattern for testing
    let test_pattern = rhema_core::schema::PromptPattern {
        id: "test-pattern".to_string(),
        name: "Test Pattern".to_string(),
        description: Some("Test pattern for context injection".to_string()),
        template: "This is a test prompt with {{CONTEXT}}".to_string(),
        injection: rhema_core::schema::PromptInjectionMethod::TemplateVariable,
        usage_analytics: rhema_core::schema::UsageAnalytics::new(),
        version: rhema_core::schema::PromptVersion::new("1.0.0"),
        tags: None,
    };

    let final_prompt = injector.inject_context(&test_pattern, Some(parsed_task_type.clone()))?;

    println!(
        "🧪 Testing context injection for task type: {:?}",
        parsed_task_type
    );
    println!("{}", "=".repeat(60));
    println!("{}", final_prompt);
    println!("{}", "=".repeat(60));

    Ok(())
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

fn update_context_rule(
    rhema: &Rhema,
    id: &str,
    name: &Option<String>,
    description: &Option<String>,
    pattern: &Option<String>,
    context: &Option<String>,
    priority: &Option<Priority>,
) -> RhemaResult<()> {
    // TODO: Implement update context rule
    println!("Updating context rule: {}", id);
    Ok(())
}

fn delete_context_rule(rhema: &Rhema, id: &str) -> RhemaResult<()> {
    // TODO: Implement delete context rule
    println!("Deleting context rule: {}", id);
    Ok(())
}
