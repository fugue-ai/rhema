use crate::{load_prompts, save_prompts};
use crate::{Rhema, RhemaError, RhemaResult};
use rhema_coordination::context_injection::{EnhancedContextInjector, TaskType};

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
    },

    /// List prompt patterns
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tags
        #[arg(long, value_name = "TAGS")]
        tag: Option<String>,
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
    },

    /// Delete a prompt pattern
    Delete {
        /// Prompt ID
        #[arg(value_name = "ID")]
        id: String,
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
        } => add_prompt(
            rhema,
            name,
            &description.as_ref().unwrap_or(&"".to_string()),
            content,
            category,
            tags,
        ),
        PromptSubcommands::List { category, tag } => list_prompts(rhema, category, tag),
        PromptSubcommands::Update {
            id,
            name,
            description,
            content,
            category,
            tags,
        } => update_prompt(rhema, id, name, description, content, category, tags),
        PromptSubcommands::Delete { id } => delete_prompt(rhema, id),
    }
}

fn add_prompt(
    rhema: &Rhema,
    name: &str,
    description: &str,
    content: &str,
    category: &Option<String>,
    tags: &Option<String>,
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
    Ok(())
}

fn list_prompts(rhema: &Rhema, category: &Option<String>, tag: &Option<String>) -> RhemaResult<()> {
    let scope_path = rhema.get_current_scope_path()?;

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        println!("No prompts.yaml found in {}", scope_path.display());
        return Ok(());
    }

    let prompts = load_prompts(&prompts_path)?;

    // Filter by tags if specified
    let filtered_prompts = if let Some(tags_str) = tag {
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
        println!("Template:");
        println!("{}", pattern.template);
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

fn show_analytics(rhema: &Rhema, pattern: &str, scope: &Option<String>) -> RhemaResult<()> {
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
    test_prompt(rhema, pattern, scope, Some(task_type))
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
