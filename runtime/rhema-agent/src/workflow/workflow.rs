use crate::{load_prompts, load_workflows, save_workflows};
use crate::{Rhema, RhemaError, RhemaResult};
use chrono::Utc;
use clap::Subcommand;
use colored::Colorize;
use rhema_core::schema::{ChainMetadata, ChainStep, ChainUsageStats, PromptChain, Workflows, PromptPattern, PromptInjectionMethod, PromptVersion, UsageAnalytics};
use rhema_coordination::context_injection::{EnhancedContextInjector, TaskType};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum WorkflowSubcommands {
    Add {
        name: String,
        description: Option<String>,
        tags: Option<String>,
        scope: Option<String>,
    },
    List {
        scope: Option<String>,
        tags: Option<String>,
    },
    AddStep {
        workflow: String,
        name: String,
        prompt_pattern: String,
        task_type: Option<String>,
        description: Option<String>,
        required: bool,
        dependencies: Option<String>,
        variables: Option<String>,
        scope: Option<String>,
    },
    Execute {
        workflow: String,
        scope: Option<String>,
        dry_run: bool,
    },
    Show {
        workflow: String,
        scope: Option<String>,
    },
    RecordExecution {
        workflow: String,
        successful: bool,
        execution_time: Option<f64>,
        scope: Option<String>,
    },
}

pub fn run(rhema: &Rhema, subcommand: &WorkflowSubcommands) -> RhemaResult<()> {
    match subcommand {
        WorkflowSubcommands::Add {
            name,
            description,
            tags,
            scope,
        } => add_workflow(rhema, name, description, tags, scope),
        WorkflowSubcommands::List { scope, tags } => list_workflows(rhema, scope, tags),
        WorkflowSubcommands::AddStep {
            workflow,
            name,
            prompt_pattern,
            task_type,
            description,
            required,
            dependencies,
            variables,
            scope,
        } => add_step(
            rhema,
            workflow,
            name,
            prompt_pattern,
            task_type,
            description,
            *required,
            dependencies,
            variables,
            scope,
        ),
        WorkflowSubcommands::Execute {
            workflow,
            scope,
            dry_run,
        } => execute_workflow(rhema, workflow, scope, *dry_run),
        WorkflowSubcommands::Show { workflow, scope } => show_workflow(rhema, workflow, scope),
        WorkflowSubcommands::RecordExecution {
            workflow,
            successful,
            execution_time,
            scope,
        } => record_execution(rhema, workflow, *successful, *execution_time, scope),
    }
}

fn add_workflow(
    rhema: &Rhema,
    name: &str,
    description: &Option<String>,
    tags: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    // Parse tags
    let tags_vec = if let Some(tags_str) = tags {
        Some(tags_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Create new workflow
    let new_workflow = PromptChain {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: description.clone(),
        steps: Vec::new(),
        metadata: ChainMetadata {
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            author: None,
            usage_stats: ChainUsageStats::new(),
            success_criteria: None,
        },
        tags: tags_vec,
    };

    // Load existing workflows or create new
    let mut workflows = if workflows_path.exists() {
        load_workflows(&workflows_path)?
    } else {
        Workflows {
            workflows: Vec::new(),
        }
    };

    // Add new workflow
    workflows.workflows.push(new_workflow);

    // Save back to file
    save_workflows(&workflows_path, &workflows)?;

    println!(
        "✅ Added workflow '{}' to {}",
        name,
        workflows_path.display()
    );
    println!("   Use 'rhema workflow add-step' to add steps to this workflow");

    Ok(())
}

fn list_workflows(rhema: &Rhema, scope: &Option<String>, tags: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    if !workflows_path.exists() {
        println!("No workflows.yaml found in {}", scope_path.display());
        return Ok(());
    }

    let workflows = load_workflows(&workflows_path)?;

    // Filter by tags if specified
    let filtered_workflows = if let Some(tags_str) = tags {
        let filter_tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        workflows
            .workflows
            .into_iter()
            .filter(|w| {
                if let Some(workflow_tags) = &w.tags {
                    filter_tags.iter().any(|tag| workflow_tags.contains(tag))
                } else {
                    false
                }
            })
            .collect()
    } else {
        workflows.workflows
    };

    if filtered_workflows.is_empty() {
        println!("No workflows found");
        return Ok(());
    }

    println!("🔄 Workflows in {}:", scope_path.display());
    println!("{}", "=".repeat(60));

    for workflow in filtered_workflows {
        println!("ID: {}", workflow.id);
        println!("Name: {}", workflow.name);
        if let Some(desc) = workflow.description {
            println!("Description: {}", desc);
        }
        println!("Version: {}", workflow.metadata.version);
        println!("Steps: {}", workflow.steps.len());
        println!(
            "Usage: {}/{} successful ({:.1}%)",
            workflow.metadata.usage_stats.successful_executions,
            workflow.metadata.usage_stats.total_executions,
            workflow.metadata.usage_stats.success_rate() * 100.0
        );
        if let Some(last_executed) = workflow.metadata.usage_stats.last_executed {
            println!("Last executed: {}", last_executed.format("%Y-%m-%d %H:%M"));
        }
        if let Some(tags) = workflow.tags {
            println!("Tags: {}", tags.join(", "));
        }
        println!("{}", "-".repeat(40));
    }

    Ok(())
}

fn add_step(
    rhema: &Rhema,
    workflow: &str,
    name: &str,
    prompt_pattern: &str,
    task_type: &Option<String>,
    description: &Option<String>,
    required: bool,
    dependencies: &Option<String>,
    variables: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    if !workflows_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No workflows.yaml found".to_string(),
        ));
    }

    let mut workflows = load_workflows(&workflows_path)?;

    // Find workflow by ID or name
    let workflow_index = workflows
        .workflows
        .iter()
        .position(|w| w.id == workflow || w.name == workflow)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Workflow '{}' not found", workflow)))?;

    // Verify prompt pattern exists
    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");
    if prompts_path.exists() {
        let prompts = load_prompts(&prompts_path)?;
        let pattern_exists = prompts
            .prompts
            .iter()
            .any(|p| p.id == prompt_pattern || p.name == prompt_pattern);

        if !pattern_exists {
            return Err(RhemaError::InvalidCommand(format!(
                "Prompt pattern '{}' not found",
                prompt_pattern
            )));
        }
    }

    // Parse dependencies
    let dependencies_vec = if let Some(deps_str) = dependencies {
        Some(deps_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Parse variables
    let variables_map = if let Some(vars_str) = variables {
        let mut vars = HashMap::new();
        for pair in vars_str.split(',') {
            if let Some((key, value)) = pair.split_once('=') {
                vars.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Some(vars)
    } else {
        None
    };

    // Create new step
    let new_step = ChainStep {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: description.clone(),
        prompt_pattern: prompt_pattern.to_string(),
        task_type: task_type.clone(),
        order: workflows.workflows[workflow_index].steps.len() as u32 + 1,
        required,
        dependencies: dependencies_vec,
        variables: variables_map,
        conditions: None,
    };

    // Add step to workflow
    workflows.workflows[workflow_index].steps.push(new_step);
    workflows.workflows[workflow_index].metadata.updated_at = Utc::now();

    save_workflows(&workflows_path, &workflows)?;

    println!("✅ Added step '{}' to workflow '{}'", name, workflow);
    println!("   Prompt pattern: {}", prompt_pattern);
    if let Some(task) = task_type {
        println!("   Task type: {}", task);
    }
    println!(
        "   Order: {}",
        workflows.workflows[workflow_index].steps.len()
    );

    Ok(())
}

fn execute_workflow(
    rhema: &Rhema,
    workflow: &str,
    scope: &Option<String>,
    dry_run: bool,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    if !workflows_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No workflows.yaml found".to_string(),
        ));
    }

    let workflows = load_workflows(&workflows_path)?;

    // Find workflow by ID or name
    let workflow_entry = workflows
        .workflows
        .iter()
        .find(|w| w.id == workflow || w.name == workflow)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Workflow '{}' not found", workflow)))?;

    if workflow_entry.steps.is_empty() {
        return Err(RhemaError::InvalidCommand(
            "Workflow has no steps to execute".to_string(),
        ));
    }

    println!("🔄 Executing workflow '{}':", workflow_entry.name);
    if dry_run {
        println!("{}", "DRY RUN MODE - No actual execution".yellow());
    }
    println!("{}", "=".repeat(60));

    let start_time = Instant::now();
    let mut executed_steps = Vec::new();
    let mut step_results = HashMap::new();

    // Sort steps by order
    let mut sorted_steps: Vec<_> = workflow_entry.steps.iter().collect();
    sorted_steps.sort_by_key(|s| s.order);

    for step in sorted_steps {
        // Check dependencies
        if let Some(deps) = &step.dependencies {
            for dep in deps {
                if !executed_steps.contains(dep) {
                    return Err(RhemaError::InvalidCommand(format!(
                        "Step '{}' depends on '{}' which hasn't been executed",
                        step.name, dep
                    )));
                }
            }
        }

        println!("📋 Step {}: {}", step.order, step.name);
        if let Some(desc) = &step.description {
            println!("   Description: {}", desc);
        }
        println!("   Prompt pattern: {}", step.prompt_pattern);
        if let Some(task) = &step.task_type {
            println!("   Task type: {}", task);
        }
        println!("   Required: {}", step.required);

        if !dry_run {
            // Execute the step with context injection
            println!("   Executing...");
            
            // Parse task type if specified
            let task_type = if let Some(task_str) = &step.task_type {
                parse_task_type(task_str)?
            } else {
                TaskType::Custom("workflow_step".to_string())
            };
            
            // Create prompt pattern from step with variable substitution
            let mut template = step.prompt_pattern.clone();
            
            // Substitute variables if any are defined
            if let Some(variables) = &step.variables {
                for (key, value) in variables {
                    let placeholder = format!("{{{{{}}}}}", key);
                    template = template.replace(&placeholder, value);
                }
            }
            
            let prompt_pattern = PromptPattern {
                id: step.id.clone(),
                name: step.name.clone(),
                description: step.description.clone(),
                template,
                injection: PromptInjectionMethod::TemplateVariable,
                usage_analytics: UsageAnalytics::new(),
                version: PromptVersion::new("1.0.0"),
                tags: None,
            };
            
            // Create context injector
            let injector = EnhancedContextInjector::new(scope_path.clone());
            
            // Inject context into the prompt
            let final_prompt = injector.inject_context(&prompt_pattern, Some(task_type))?;
            
            // Execute the prompt (for now, just display it)
            println!("   📝 Executed prompt with context injection:");
            println!("   Task Type: {:?}", task_type);
            if let Some(variables) = &step.variables {
                println!("   Variables: {:?}", variables);
            }
            println!("   {}", "-".repeat(50));
            println!("   {}", final_prompt);
            println!("   {}", "-".repeat(50));
            
            // TODO: In a real implementation, this would send the prompt to an AI service
            // and return the response. For now, we'll simulate success.
            step_results.insert(step.id.clone(), "Success (Context Injected)".to_string());
        } else {
            println!(
                "   [DRY RUN] Would execute prompt pattern: {}",
                step.prompt_pattern
            );
        }

        executed_steps.push(step.id.clone());
        println!();
    }

    let execution_time = start_time.elapsed().as_secs_f64();

    println!("✅ Workflow execution completed");
    println!("   Total steps: {}", workflow_entry.steps.len());
    println!("   Executed steps: {}", executed_steps.len());
    println!("   Execution time: {:.2}s", execution_time);

    if !dry_run {
        println!("\n📊 Step Results:");
        for step in &workflow_entry.steps {
            if let Some(result) = step_results.get(&step.id) {
                println!("   {}: {}", step.name, result);
            }
        }
    }

    Ok(())
}

fn show_workflow(rhema: &Rhema, workflow: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    if !workflows_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No workflows.yaml found".to_string(),
        ));
    }

    let workflows = load_workflows(&workflows_path)?;

    // Find workflow by ID or name
    let workflow_entry = workflows
        .workflows
        .iter()
        .find(|w| w.id == workflow || w.name == workflow)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Workflow '{}' not found", workflow)))?;

    println!("🔄 Workflow: {}", workflow_entry.name);
    println!("{}", "=".repeat(60));
    println!("ID: {}", workflow_entry.id);
    if let Some(desc) = &workflow_entry.description {
        println!("Description: {}", desc);
    }
    println!("Version: {}", workflow_entry.metadata.version);
    println!(
        "Created: {}",
        workflow_entry.metadata.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "Updated: {}",
        workflow_entry.metadata.updated_at.format("%Y-%m-%d %H:%M")
    );
    if let Some(author) = &workflow_entry.metadata.author {
        println!("Author: {}", author);
    }
    println!();

    println!("📊 Usage Statistics:");
    println!(
        "   Total executions: {}",
        workflow_entry.metadata.usage_stats.total_executions
    );
    println!(
        "   Successful executions: {}",
        workflow_entry.metadata.usage_stats.successful_executions
    );
    println!(
        "   Success rate: {:.1}%",
        workflow_entry.metadata.usage_stats.success_rate() * 100.0
    );
    if let Some(avg_time) = workflow_entry.metadata.usage_stats.average_execution_time {
        println!("   Average execution time: {:.2}s", avg_time);
    }
    if let Some(last_executed) = workflow_entry.metadata.usage_stats.last_executed {
        println!(
            "   Last executed: {}",
            last_executed.format("%Y-%m-%d %H:%M")
        );
    }
    println!();

    if let Some(success_criteria) = &workflow_entry.metadata.success_criteria {
        println!("✅ Success Criteria:");
        for criterion in success_criteria {
            println!("   • {}", criterion);
        }
        println!();
    }

    if let Some(tags) = &workflow_entry.tags {
        println!("🏷️  Tags: {}", tags.join(", "));
        println!();
    }

    println!("📋 Steps ({}):", workflow_entry.steps.len());
    println!("{}", "-".repeat(40));

    // Sort steps by order
    let mut sorted_steps: Vec<_> = workflow_entry.steps.iter().collect();
    sorted_steps.sort_by_key(|s| s.order);

    for step in sorted_steps {
        println!("{}. {}", step.order, step.name);
        if let Some(desc) = &step.description {
            println!("   Description: {}", desc);
        }
        println!("   Prompt pattern: {}", step.prompt_pattern);
        if let Some(task) = &step.task_type {
            println!("   Task type: {}", task);
        }
        println!("   Required: {}", step.required);
        if let Some(deps) = &step.dependencies {
            println!("   Dependencies: {}", deps.join(", "));
        }
        if let Some(vars) = &step.variables {
            println!(
                "   Variables: {}",
                vars.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        println!();
    }

    Ok(())
}

fn record_execution(
    rhema: &Rhema,
    workflow: &str,
    successful: bool,
    execution_time: Option<f64>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let workflows_path = scope_path.join(".rhema").join("workflows.yaml");

    if !workflows_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No workflows.yaml found".to_string(),
        ));
    }

    let mut workflows = load_workflows(&workflows_path)?;

    // Find workflow by ID or name
    let workflow_index = workflows
        .workflows
        .iter()
        .position(|w| w.id == workflow || w.name == workflow)
        .ok_or_else(|| RhemaError::InvalidCommand(format!("Workflow '{}' not found", workflow)))?;

    // Record the execution
    workflows.workflows[workflow_index]
        .metadata
        .usage_stats
        .record_execution(successful, execution_time);

    save_workflows(&workflows_path, &workflows)?;

    let status = if successful {
        "✅ successful"
    } else {
        "❌ unsuccessful"
    };
    println!(
        "📊 Recorded {} execution for workflow '{}'",
        status, workflow
    );
    println!(
        "   New success rate: {:.1}% ({}/{})",
        workflows.workflows[workflow_index]
            .metadata
            .usage_stats
            .success_rate()
            * 100.0,
        workflows.workflows[workflow_index]
            .metadata
            .usage_stats
            .successful_executions,
        workflows.workflows[workflow_index]
            .metadata
            .usage_stats
            .total_executions
    );

    Ok(())
}

/// Parse task type string into TaskType enum
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
