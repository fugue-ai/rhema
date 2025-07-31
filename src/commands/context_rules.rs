use crate::{Rhema, RhemaResult};
use crate::context_injection::{ContextInjectionRule, TaskType, EnhancedContextInjector};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ContextRulesSubcommands {
    /// List all context injection rules
    List {
        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
    
    /// Add a new context injection rule
    Add {
        /// Task type
        #[arg(value_name = "TASK_TYPE")]
        task_type: String,
        
        /// Context files (comma-separated)
        #[arg(long, value_name = "FILES")]
        context_files: String,
        
        /// Injection method (prepend, append, template_variable)
        #[arg(long, value_name = "METHOD", default_value = "template_variable")]
        injection_method: String,
        
        /// Priority (1-10, higher = more specific)
        #[arg(long, value_name = "PRIORITY", default_value = "1")]
        priority: u8,
        
        /// Additional context
        #[arg(long, value_name = "CONTEXT")]
        additional_context: Option<String>,
        
        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
    
    /// Test context injection for a task type
    Test {
        /// Task type to test
        #[arg(value_name = "TASK_TYPE")]
        task_type: String,
        
        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
}

pub fn run(rhema: &Rhema, subcommand: &ContextRulesSubcommands) -> RhemaResult<()> {
    match subcommand {
        ContextRulesSubcommands::List { scope } => {
            list_context_rules(rhema, scope)
        }
        ContextRulesSubcommands::Add { task_type, context_files, injection_method, priority, additional_context, scope } => {
            add_context_rule(rhema, task_type, context_files, injection_method, *priority, additional_context, scope)
        }
        ContextRulesSubcommands::Test { task_type, scope } => {
            test_context_injection(rhema, task_type, scope)
        }
    }
}

fn list_context_rules(
    rhema: &Rhema,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

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
    task_type: &str,
    context_files: &str,
    injection_method: &str,
    priority: u8,
    additional_context: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let _scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    // Parse task type
    let parsed_task_type = parse_task_type(task_type)?;
    
    // Parse context files
    let files: Vec<String> = context_files.split(',').map(|s| s.trim().to_string()).collect();
    
    // Parse injection method
    let injection = match injection_method.to_lowercase().as_str() {
        "prepend" => crate::schema::PromptInjectionMethod::Prepend,
        "append" => crate::schema::PromptInjectionMethod::Append,
        "template_variable" => crate::schema::PromptInjectionMethod::TemplateVariable,
        _ => return Err(crate::error::RhemaError::InvalidCommand(
            "Invalid injection method. Use: prepend, append, or template_variable".to_string()
        )),
    };
    
    // Create new rule
    let new_rule = ContextInjectionRule {
        task_type: parsed_task_type,
        context_files: files,
        injection_method: injection,
        priority,
        additional_context: additional_context.clone(),
    };
    
    // TODO: Save rule to configuration file
    // For now, just print the rule
    println!("✅ Added context injection rule:");
    println!("   Task Type: {:?}", new_rule.task_type);
    println!("   Context Files: {}", new_rule.context_files.join(", "));
    println!("   Injection Method: {:?}", new_rule.injection_method);
    println!("   Priority: {}", new_rule.priority);
    if let Some(context) = &new_rule.additional_context {
        println!("   Additional Context: {}", context);
    }

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
    let test_pattern = crate::schema::PromptPattern {
        id: "test-pattern".to_string(),
        name: "Test Pattern".to_string(),
        description: Some("Test pattern for context injection".to_string()),
        template: "This is a test prompt with {{CONTEXT}}".to_string(),
        injection: crate::schema::PromptInjectionMethod::TemplateVariable,
        usage_analytics: crate::schema::UsageAnalytics::new(),
        version: crate::schema::PromptVersion::new("1.0.0"),
        tags: None,
    };

    let final_prompt = injector.inject_context(&test_pattern, Some(parsed_task_type.clone()))?;

    println!("🧪 Testing context injection for task type: {:?}", parsed_task_type);
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