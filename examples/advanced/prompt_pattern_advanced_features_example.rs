use rhema_core::schema::{
    PromptPattern, PromptInjectionMethod, ContextRule, ContextInjectionMethod,
    Prompts, UsageAnalytics
};
use std::collections::HashMap;

/// Example demonstrating the advanced prompt pattern features
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced Prompt Pattern Features Example");
    println!("============================================\n");

    // Example 1: Basic prompt pattern with advanced features
    println!("1️⃣ Creating a basic prompt pattern with advanced features:");
    let mut code_review_pattern = PromptPattern::new(
        "code-review-advanced",
        "Advanced Code Review",
        "Review this {{LANGUAGE}} code for {{REVIEW_FOCUS}}: {{CONTEXT}}",
        PromptInjectionMethod::TemplateVariable,
    );

    // Set template variables
    let mut variables = HashMap::new();
    variables.insert("LANGUAGE".to_string(), "Rust".to_string());
    variables.insert("REVIEW_FOCUS".to_string(), "security and performance".to_string());
    code_review_pattern.set_variables(variables);

    // Add conditional context rules
    let security_rule = ContextRule::new(
        "task_type == 'code_review' && severity == 'high'",
        vec!["security-patterns.yaml".to_string(), "vulnerability-db.yaml".to_string()],
        ContextInjectionMethod::Prepend,
    ).with_priority(2);

    let general_rule = ContextRule::new(
        "task_type == 'code_review'",
        vec!["code-review-patterns.yaml".to_string()],
        ContextInjectionMethod::Prepend,
    ).with_priority(1);

    code_review_pattern.add_context_rule(security_rule);
    code_review_pattern.add_context_rule(general_rule);

    // Enable multi-file context support
    code_review_pattern.enable_multi_file_context();

    println!("✅ Created advanced code review pattern");
    println!("   - Template variables: LANGUAGE, REVIEW_FOCUS");
    println!("   - Context rules: 2 conditional rules");
    println!("   - Multi-file context: enabled\n");

    // Example 2: Template inheritance
    println!("2️⃣ Creating a specialized pattern that extends the base pattern:");
    let mut security_review_pattern = PromptPattern::new(
        "security-review",
        "Security-Focused Code Review",
        "{{BASE_TEMPLATE}}\n\nAdditional security checks:\n- Check for {{SECURITY_CHECKS}}\n- Verify {{VERIFICATION_POINTS}}",
        PromptInjectionMethod::TemplateVariable,
    );

    security_review_pattern.set_extends("code-review-advanced");
    
    let mut security_vars = HashMap::new();
    security_vars.insert("SECURITY_CHECKS".to_string(), "SQL injection, XSS, buffer overflows".to_string());
    security_vars.insert("VERIFICATION_POINTS".to_string(), "input validation, output encoding".to_string());
    security_review_pattern.set_variables(security_vars);

    println!("✅ Created security review pattern");
    println!("   - Extends: code-review-advanced");
    println!("   - Additional security-specific variables\n");

    // Example 3: Testing context rule matching
    println!("3️⃣ Testing context rule matching:");
    
    // Test high-severity code review
    let high_severity_rules = code_review_pattern.get_matching_rules(
        Some("code_review"), 
        None, 
        Some("high")
    );
    println!("   High severity code review matches {} rules:", high_severity_rules.len());
    for (i, rule) in high_severity_rules.iter().enumerate() {
        println!("     {}. {} (priority: {})", 
                 i + 1, 
                 rule.condition, 
                 rule.priority.unwrap_or(0));
    }

    // Test regular code review
    let regular_rules = code_review_pattern.get_matching_rules(
        Some("code_review"), 
        None, 
        None
    );
    println!("   Regular code review matches {} rules:", regular_rules.len());
    for (i, rule) in regular_rules.iter().enumerate() {
        println!("     {}. {} (priority: {})", 
                 i + 1, 
                 rule.condition, 
                 rule.priority.unwrap_or(0));
    }

    // Example 4: Testing template variable substitution
    println!("\n4️⃣ Testing template variable substitution:");
    let substituted = code_review_pattern.substitute_variables("fn main() { println!(\"Hello, world!\"); }");
    println!("   Original template: {}", code_review_pattern.template);
    println!("   Substituted result: {}", substituted);

    // Example 5: Testing context file collection
    println!("\n5️⃣ Testing context file collection:");
    let context_files = code_review_pattern.get_context_files(
        Some("code_review"), 
        None, 
        Some("high")
    );
    println!("   Context files for high-severity code review:");
    for file in &context_files {
        println!("     - {}", file);
    }

    // Example 6: Testing usage tracking
    println!("\n6️⃣ Testing usage tracking:");
    code_review_pattern.record_usage(true, Some("Excellent for security reviews".to_string()));
    code_review_pattern.record_usage(true, Some("Very helpful for catching bugs".to_string()));
    code_review_pattern.record_usage(false, Some("Could be more specific about performance".to_string()));
    code_review_pattern.record_usage(true, Some("Perfect for our workflow".to_string()));

    println!("   Usage statistics:");
    println!("     - Total uses: {}", code_review_pattern.total_uses());
    println!("     - Successful uses: {}", code_review_pattern.successful_uses());
    println!("     - Success rate: {:.1}%", code_review_pattern.success_rate() * 100.0);

    // Example 7: Creating a complete prompts.yaml structure
    println!("\n7️⃣ Creating a complete prompts.yaml structure:");
    let mut prompts = Prompts {
        prompts: vec![code_review_pattern.clone(), security_review_pattern],
    };

    // Add a bug fix pattern
    let mut bug_fix_pattern = PromptPattern::new(
        "bug-fix",
        "Bug Fix Pattern",
        "Fix this {{LANGUAGE}} bug: {{CONTEXT}}\n\nFocus on: {{FOCUS_AREAS}}",
        PromptInjectionMethod::TemplateVariable,
    );

    let mut bug_vars = HashMap::new();
    bug_vars.insert("LANGUAGE".to_string(), "Rust".to_string());
    bug_vars.insert("FOCUS_AREAS".to_string(), "error handling, edge cases".to_string());
    bug_fix_pattern.set_variables(bug_vars);

    // Add bug fix specific context rules
    let bug_rule = ContextRule::new(
        "task_type == 'bug_fix'",
        vec!["bug-patterns.yaml".to_string(), "error-handling.yaml".to_string()],
        ContextInjectionMethod::Prepend,
    );
    bug_fix_pattern.add_context_rule(bug_rule);

    prompts.prompts.push(bug_fix_pattern);

    println!("   Created prompts.yaml with {} patterns:", prompts.prompts.len());
    for pattern in &prompts.prompts {
        println!("     - {} (ID: {})", pattern.name, pattern.id);
        if let Some(rules) = &pattern.context_rules {
            println!("       Context rules: {}", rules.len());
        }
        if let Some(vars) = &pattern.variables {
            println!("       Variables: {}", vars.len());
        }
        if pattern.has_extends() {
            println!("       Extends: {}", pattern.extends.as_ref().unwrap());
        }
    }

    // Example 8: Demonstrating variable access
    println!("\n8️⃣ Demonstrating variable access:");
    if let Some(language) = code_review_pattern.get_variable("LANGUAGE") {
        println!("   LANGUAGE variable: {}", language);
    }
    if let Some(focus) = code_review_pattern.get_variable("REVIEW_FOCUS") {
        println!("   REVIEW_FOCUS variable: {}", focus);
    }

    // Example 9: Testing multi-file context support
    println!("\n9️⃣ Testing multi-file context support:");
    println!("   Code review pattern supports multi-file context: {}", 
             code_review_pattern.supports_multi_file_context());

    // Example 10: Serialization example
    println!("\n🔟 Serialization example:");
    let yaml = serde_yaml::to_string(&prompts)?;
    println!("   Generated YAML (first 500 chars):");
    println!("   {}", &yaml[..yaml.len().min(500)]);

    println!("\n🎉 Advanced Prompt Pattern Features Example Completed!");
    println!("=====================================================");
    println!("✅ Conditional context injection");
    println!("✅ Template variables beyond {{CONTEXT}}");
    println!("✅ Template inheritance");
    println!("✅ Multi-file context support");
    println!("✅ Context rule priority system");
    println!("✅ Usage analytics and feedback tracking");
    println!("✅ Variable substitution and access");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_prompt_pattern_creation() {
        let mut pattern = PromptPattern::new(
            "test-advanced",
            "Test Advanced Pattern",
            "Test {{VARIABLE}}: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Test variables
        let mut variables = HashMap::new();
        variables.insert("VARIABLE".to_string(), "value".to_string());
        pattern.set_variables(variables);

        // Test context rules
        let rule = ContextRule::new(
            "task_type == 'test'",
            vec!["test.yaml".to_string()],
            ContextInjectionMethod::Prepend,
        );
        pattern.add_context_rule(rule);

        // Test multi-file context
        pattern.enable_multi_file_context();

        // Verify
        assert_eq!(pattern.get_variable("VARIABLE"), Some(&"value".to_string()));
        assert_eq!(pattern.get_matching_rules(Some("test"), None, None).len(), 1);
        assert!(pattern.supports_multi_file_context());
    }

    #[test]
    fn test_template_inheritance() {
        let mut base_pattern = PromptPattern::new(
            "base",
            "Base Pattern",
            "Base: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        let mut extended_pattern = PromptPattern::new(
            "extended",
            "Extended Pattern",
            "Extended: {{BASE_TEMPLATE}}",
            PromptInjectionMethod::TemplateVariable,
        );

        extended_pattern.set_extends("base");

        assert!(extended_pattern.has_extends());
        assert_eq!(extended_pattern.extends, Some("base".to_string()));
    }

    #[test]
    fn test_usage_tracking() {
        let mut pattern = PromptPattern::new(
            "usage-test",
            "Usage Test",
            "Test: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        pattern.record_usage(true, Some("Great!".to_string()));
        pattern.record_usage(false, Some("Could be better".to_string()));

        assert_eq!(pattern.total_uses(), 2);
        assert_eq!(pattern.successful_uses(), 1);
        assert_eq!(pattern.success_rate(), 0.5);
    }
}
