use crate::{Rhema, RhemaResult};
use rhema_coordination::context_injection::{EnhancedContextInjector, TaskType, ContextInjectionRule, PromptInjectionMethod};
use rhema_core::Priority;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

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

/// Context rule storage for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextRuleStorage {
    rules: HashMap<String, ContextRuleData>,
}

/// Context rule data for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextRuleData {
    id: String,
    name: String,
    description: String,
    pattern: String,
    context: String,
    priority: Priority,
    task_type: TaskType,
    context_files: Vec<String>,
    injection_method: PromptInjectionMethod,
    additional_context: Option<String>,
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
    let scope_path = rhema.get_current_scope_path()?;
    let rules_file = scope_path.join(".rhema").join("context_rules.json");
    
    // Create .rhema directory if it doesn't exist
    if !rules_file.parent().unwrap().exists() {
        fs::create_dir_all(rules_file.parent().unwrap())?;
    }
    
    // Load existing rules or create new storage
    let mut storage = if rules_file.exists() {
        let content = fs::read_to_string(&rules_file)?;
        serde_json::from_str::<ContextRuleStorage>(&content)
            .unwrap_or_else(|_| ContextRuleStorage { rules: HashMap::new() })
    } else {
        ContextRuleStorage { rules: HashMap::new() }
    };
    
    // Generate unique ID
    let id = format!("rule_{}", uuid::Uuid::new_v4().simple());
    
    // Parse task type from pattern (simple heuristic)
    let task_type = parse_task_type_from_pattern(pattern);
    
    // Create rule data
    let rule_data = ContextRuleData {
        id: id.clone(),
        name: name.to_string(),
        description: description.to_string(),
        pattern: pattern.to_string(),
        context: context.to_string(),
        priority: priority.clone(),
        task_type: task_type.clone(),
        context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
        injection_method: PromptInjectionMethod::TemplateVariable,
        additional_context: Some(context.to_string()),
    };
    
    // Add to storage
    storage.rules.insert(id.clone(), rule_data);
    
    // Save to file
    let content = serde_json::to_string_pretty(&storage)?;
    fs::write(&rules_file, content)?;
    
    // Also add to the injector for immediate use
    let mut injector = EnhancedContextInjector::new(scope_path);
    let injection_rule = ContextInjectionRule {
        task_type,
        context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
        injection_method: PromptInjectionMethod::TemplateVariable,
        priority: priority_to_u8(priority),
        additional_context: Some(context.to_string()),
        lock_file_context: None,
    };
    injector.add_rule(injection_rule);
    
    println!("✅ Added context injection rule:");
    println!("   ID: {}", id);
    println!("   Name: {}", name);
    println!("   Description: {}", description);
    println!("   Pattern: {}", pattern);
    println!("   Context: {}", context);
    println!("   Priority: {:?}", priority);
    println!("   Saved to: {}", rules_file.display());

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

fn parse_task_type_from_pattern(pattern: &str) -> TaskType {
    let pattern_lower = pattern.to_lowercase();
    
    if pattern_lower.contains("test") || pattern_lower.contains("spec") {
        TaskType::Testing
    } else if pattern_lower.contains("doc") || pattern_lower.contains("readme") {
        TaskType::Documentation
    } else if pattern_lower.contains("security") || pattern_lower.contains("auth") {
        TaskType::SecurityReview
    } else if pattern_lower.contains("perf") || pattern_lower.contains("optimize") {
        TaskType::PerformanceOptimization
    } else if pattern_lower.contains("dep") || pattern_lower.contains("update") {
        TaskType::DependencyUpdate
    } else if pattern_lower.contains("deploy") || pattern_lower.contains("release") {
        TaskType::Deployment
    } else if pattern_lower.contains("refactor") {
        TaskType::Refactoring
    } else if pattern_lower.contains("fix") || pattern_lower.contains("bug") {
        TaskType::BugFix
    } else if pattern_lower.contains("feat") || pattern_lower.contains("feature") {
        TaskType::FeatureDevelopment
    } else {
        TaskType::CodeReview
    }
}

fn priority_to_u8(priority: &Priority) -> u8 {
    match priority {
        Priority::Low => 1,
        Priority::Medium => 5,
        Priority::High => 10,
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
    let scope_path = rhema.get_current_scope_path()?;
    let rules_file = scope_path.join(".rhema").join("context_rules.json");
    
    if !rules_file.exists() {
        return Err(rhema_core::RhemaError::NotFound(format!(
            "No context rules file found at {}",
            rules_file.display()
        )));
    }
    
    // Load existing rules
    let content = fs::read_to_string(&rules_file)?;
    let mut storage: ContextRuleStorage = serde_json::from_str(&content)?;
    
    // Find the rule to update
    let rule_data = storage.rules.get_mut(id).ok_or_else(|| {
        rhema_core::RhemaError::NotFound(format!("Context rule with ID '{}' not found", id))
    })?;
    
    // Update fields if provided
    if let Some(new_name) = name {
        rule_data.name = new_name.clone();
    }
    if let Some(new_description) = description {
        rule_data.description = new_description.clone();
    }
    if let Some(new_pattern) = pattern {
        rule_data.pattern = new_pattern.clone();
        // Update task type based on new pattern
        rule_data.task_type = parse_task_type_from_pattern(new_pattern);
    }
    if let Some(new_context) = context {
        rule_data.context = new_context.clone();
        rule_data.additional_context = Some(new_context.clone());
    }
    if let Some(new_priority) = priority {
        rule_data.priority = new_priority.clone();
    }
    
    // Save updated rules
    let content = serde_json::to_string_pretty(&storage)?;
    fs::write(&rules_file, content)?;
    
    println!("✅ Updated context rule: {}", id);
    println!("   Name: {}", rule_data.name);
    println!("   Description: {}", rule_data.description);
    println!("   Pattern: {}", rule_data.pattern);
    println!("   Context: {}", rule_data.context);
    println!("   Priority: {:?}", rule_data.priority);
    println!("   Task Type: {:?}", rule_data.task_type);
    
    Ok(())
}

fn delete_context_rule(rhema: &Rhema, id: &str) -> RhemaResult<()> {
    let scope_path = rhema.get_current_scope_path()?;
    let rules_file = scope_path.join(".rhema").join("context_rules.json");
    
    if !rules_file.exists() {
        return Err(rhema_core::RhemaError::NotFound(format!(
            "No context rules file found at {}",
            rules_file.display()
        )));
    }
    
    // Load existing rules
    let content = fs::read_to_string(&rules_file)?;
    let mut storage: ContextRuleStorage = serde_json::from_str(&content)?;
    
    // Check if rule exists
    if !storage.rules.contains_key(id) {
        return Err(rhema_core::RhemaError::NotFound(format!(
            "Context rule with ID '{}' not found",
            id
        )));
    }
    
    // Remove the rule
    let removed_rule = storage.rules.remove(id).unwrap();
    
    // Save updated rules
    let content = serde_json::to_string_pretty(&storage)?;
    fs::write(&rules_file, content)?;
    
    println!("✅ Deleted context rule: {}", id);
    println!("   Name: {}", removed_rule.name);
    println!("   Description: {}", removed_rule.description);
    println!("   Pattern: {}", removed_rule.pattern);
    println!("   Context: {}", removed_rule.context);
    println!("   Priority: {:?}", removed_rule.priority);
    println!("   Task Type: {:?}", removed_rule.task_type);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    fn create_test_rhema(temp_dir: &TempDir) -> Rhema {
        Rhema::new(temp_dir.path().to_path_buf()).unwrap()
    }

    #[test]
    fn test_parse_task_type_from_pattern() {
        assert!(matches!(parse_task_type_from_pattern("test"), TaskType::Testing));
        assert!(matches!(parse_task_type_from_pattern("docs"), TaskType::Documentation));
        assert!(matches!(parse_task_type_from_pattern("security"), TaskType::SecurityReview));
        assert!(matches!(parse_task_type_from_pattern("perf"), TaskType::PerformanceOptimization));
        assert!(matches!(parse_task_type_from_pattern("dep"), TaskType::DependencyUpdate));
        assert!(matches!(parse_task_type_from_pattern("deploy"), TaskType::Deployment));
        assert!(matches!(parse_task_type_from_pattern("refactor"), TaskType::Refactoring));
        assert!(matches!(parse_task_type_from_pattern("fix"), TaskType::BugFix));
        assert!(matches!(parse_task_type_from_pattern("feat"), TaskType::FeatureDevelopment));
        assert!(matches!(parse_task_type_from_pattern("review"), TaskType::CodeReview));
    }

    #[test]
    fn test_priority_to_u8() {
        assert_eq!(priority_to_u8(&Priority::Low), 1);
        assert_eq!(priority_to_u8(&Priority::Medium), 5);
        assert_eq!(priority_to_u8(&Priority::High), 10);
    }

    #[test]
    fn test_add_context_rule() {
        let temp_dir = TempDir::new().unwrap();
        let rhema = create_test_rhema(&temp_dir);
        
        let result = add_context_rule(
            &rhema,
            "Test Rule",
            "Test Description",
            "test pattern",
            "test context",
            &Priority::Medium,
        );
        
        assert!(result.is_ok());
        
        // Verify the file was created
        let rules_file = temp_dir.path().join(".rhema").join("context_rules.json");
        assert!(rules_file.exists());
        
        // Verify the content
        let content = fs::read_to_string(&rules_file).unwrap();
        let storage: ContextRuleStorage = serde_json::from_str(&content).unwrap();
        assert_eq!(storage.rules.len(), 1);
        
        let rule = storage.rules.values().next().unwrap();
        assert_eq!(rule.name, "Test Rule");
        assert_eq!(rule.description, "Test Description");
        assert_eq!(rule.pattern, "test pattern");
        assert_eq!(rule.context, "test context");
        assert!(matches!(rule.priority, Priority::Medium));
        assert!(matches!(rule.task_type, TaskType::Testing));
    }
}
