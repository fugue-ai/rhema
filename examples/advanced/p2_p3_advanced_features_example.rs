use rhema_core::schema::{
    AdvancedVariable, CacheInvalidationStrategy, CompositionBlock, CompositionBlockType,
    ContextCacheConfig, ContextInjectionMethod, ContextLearningConfig, ContextOptimizationConfig,
    ContextQualityMetrics, ContextRule, LearningAlgorithm, OptimizationAlgorithm,
    PromptInjectionMethod, PromptPattern, Prompts, TemplateValidationRule, ValidationRuleType,
    ValidationSeverity, VariableType, VariableValidation,
};
use std::collections::HashMap;

/// Example demonstrating P2 and P3 advanced features
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 P2 & P3 Advanced Features Example");
    println!("=====================================\n");

    // P2: Advanced Template Features

    println!("📋 P2: Advanced Template Features");
    println!("================================\n");

    // Example 1: Template Composition Blocks
    println!("1️⃣ Creating a pattern with composition blocks:");
    let mut advanced_pattern = PromptPattern::new(
        "advanced-template",
        "Advanced Template Pattern",
        "Review this {{LANGUAGE}} code: {{CONTEXT}}\n\n{{COMPOSITION_BLOCKS}}",
        PromptInjectionMethod::TemplateVariable,
    );

    // Add conditional composition block
    let security_block = CompositionBlock {
        id: "security-check".to_string(),
        block_type: CompositionBlockType::Conditional,
        condition: Some("task_type == 'security_review'".to_string()),
        loop_variable: None,
        loop_items: None,
        content: "\n🔒 Security Analysis:\n- Check for {{SECURITY_CHECKS}}\n- Verify {{VERIFICATION_POINTS}}\n".to_string(),
        priority: Some(1),
        variables: None,
    };

    // Add loop composition block
    let checklist_block = CompositionBlock {
        id: "checklist".to_string(),
        block_type: CompositionBlockType::Loop,
        condition: None,
        loop_variable: Some("check_item".to_string()),
        loop_items: Some(vec![
            "code quality".to_string(),
            "performance".to_string(),
            "documentation".to_string(),
        ]),
        content: "- Review {{check_item}}\n".to_string(),
        priority: Some(2),
        variables: None,
    };

    // Add include composition block
    let include_block = CompositionBlock {
        id: "standard-header".to_string(),
        block_type: CompositionBlockType::Include,
        condition: None,
        loop_variable: None,
        loop_items: None,
        content: "📝 Code Review Report\n==================\n".to_string(),
        priority: Some(0),
        variables: None,
    };

    advanced_pattern.add_composition_block(security_block);
    advanced_pattern.add_composition_block(checklist_block);
    advanced_pattern.add_composition_block(include_block);

    println!("✅ Added 3 composition blocks (conditional, loop, include)\n");

    // Example 2: Advanced Variables with Validation
    println!("2️⃣ Adding advanced variables with validation:");

    let language_var = AdvancedVariable {
        name: "LANGUAGE".to_string(),
        var_type: VariableType::Enum(vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
        ]),
        default_value: Some("Rust".to_string()),
        validation: Some(VariableValidation {
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            pattern: None,
            custom_validator: None,
        }),
        description: Some("Programming language for the code review".to_string()),
        required: true,
        constraints: None,
    };

    let severity_var = AdvancedVariable {
        name: "SEVERITY".to_string(),
        var_type: VariableType::Enum(vec![
            "low".to_string(),
            "medium".to_string(),
            "high".to_string(),
            "critical".to_string(),
        ]),
        default_value: Some("medium".to_string()),
        validation: Some(VariableValidation {
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            pattern: None,
            custom_validator: None,
        }),
        description: Some("Severity level of the review".to_string()),
        required: true,
        constraints: None,
    };

    let max_lines_var = AdvancedVariable {
        name: "MAX_LINES".to_string(),
        var_type: VariableType::Number,
        default_value: Some("1000".to_string()),
        validation: Some(VariableValidation {
            min_length: None,
            max_length: None,
            min_value: Some(1.0),
            max_value: Some(10000.0),
            pattern: None,
            custom_validator: None,
        }),
        description: Some("Maximum number of lines to review".to_string()),
        required: false,
        constraints: None,
    };

    advanced_pattern.add_advanced_variable(language_var);
    advanced_pattern.add_advanced_variable(severity_var);
    advanced_pattern.add_advanced_variable(max_lines_var);

    println!("✅ Added 3 advanced variables with validation rules\n");

    // Example 3: Template Validation Rules
    println!("3️⃣ Adding template validation rules:");

    let required_rule = TemplateValidationRule {
        id: "required-language".to_string(),
        rule_type: ValidationRuleType::Required,
        condition: "LANGUAGE == ''".to_string(),
        error_message: "Language must be specified".to_string(),
        severity: ValidationSeverity::Error,
        enabled: true,
    };

    let format_rule = TemplateValidationRule {
        id: "valid-severity".to_string(),
        rule_type: ValidationRuleType::Format,
        condition: "SEVERITY not in ['low', 'medium', 'high', 'critical']".to_string(),
        error_message: "Severity must be one of: low, medium, high, critical".to_string(),
        severity: ValidationSeverity::Warning,
        enabled: true,
    };

    let consistency_rule = TemplateValidationRule {
        id: "consistency-check".to_string(),
        rule_type: ValidationRuleType::Consistency,
        condition: "LANGUAGE == 'Rust' && not contains('unsafe')".to_string(),
        error_message: "Rust code should mention unsafe blocks if present".to_string(),
        severity: ValidationSeverity::Info,
        enabled: true,
    };

    advanced_pattern.add_validation_rule(required_rule);
    advanced_pattern.add_validation_rule(format_rule);
    advanced_pattern.add_validation_rule(consistency_rule);

    println!("✅ Added 3 validation rules (required, format, consistency)\n");

    // Example 4: Testing Advanced Variable Validation
    println!("4️⃣ Testing advanced variable validation:");

    let mut test_variables = HashMap::new();
    test_variables.insert("LANGUAGE".to_string(), "Rust".to_string());
    test_variables.insert("SEVERITY".to_string(), "high".to_string());
    test_variables.insert("MAX_LINES".to_string(), "500".to_string());

    let validation_errors = advanced_pattern.validate_advanced_variables(&test_variables);
    if validation_errors.is_empty() {
        println!("✅ All variables passed validation");
    } else {
        println!("❌ Validation errors:");
        for error in &validation_errors {
            println!("   - {}", error);
        }
    }

    // Test with invalid values
    let mut invalid_variables = HashMap::new();
    invalid_variables.insert("LANGUAGE".to_string(), "InvalidLanguage".to_string());
    invalid_variables.insert("MAX_LINES".to_string(), "15000".to_string()); // Exceeds max

    let invalid_errors = advanced_pattern.validate_advanced_variables(&invalid_variables);
    if !invalid_errors.is_empty() {
        println!("✅ Caught validation errors:");
        for error in &invalid_errors {
            println!("   - {}", error);
        }
    }

    println!();

    // Example 5: Testing Template Composition
    println!("5️⃣ Testing template composition:");

    let context = "fn main() { println!(\"Hello, world!\"); }";
    let mut composition_vars = HashMap::new();
    composition_vars.insert("LANGUAGE".to_string(), "Rust".to_string());
    composition_vars.insert(
        "SECURITY_CHECKS".to_string(),
        "buffer overflows, memory leaks".to_string(),
    );
    composition_vars.insert(
        "VERIFICATION_POINTS".to_string(),
        "input validation, output encoding".to_string(),
    );

    let composed_result = advanced_pattern.render_with_composition(context, &composition_vars);
    println!("Composed template result:");
    println!("{}", composed_result);
    println!();

    // P3: Enhanced Context Management

    println!("📊 P3: Enhanced Context Management");
    println!("=================================\n");

    // Example 6: Context Caching Configuration
    println!("6️⃣ Configuring context caching:");

    let cache_config = ContextCacheConfig {
        enabled: true,
        ttl_seconds: 3600,                 // 1 hour
        max_size_bytes: 100 * 1024 * 1024, // 100MB
        invalidation_strategy: CacheInvalidationStrategy::TimeBased,
        compression_enabled: true,
        persistence_enabled: true,
    };

    advanced_pattern.configure_context_cache(cache_config);
    println!("✅ Configured context caching (TTL: 1h, Max: 100MB, Compression: enabled)");

    // Example 7: Context Optimization Configuration
    println!("7️⃣ Configuring context optimization:");

    let optimization_config = ContextOptimizationConfig {
        enabled: true,
        max_tokens: 4000,
        min_relevance_score: 0.8,
        semantic_compression: true,
        structure_optimization: true,
        relevance_filtering: true,
        algorithm: OptimizationAlgorithm::Hybrid,
    };

    advanced_pattern.configure_context_optimization(optimization_config);
    println!("✅ Configured context optimization (Max: 4000 tokens, Min relevance: 0.8, Algorithm: Hybrid)");

    // Example 8: Context Learning Configuration
    println!("8️⃣ Configuring context learning:");

    let learning_config = ContextLearningConfig {
        enabled: true,
        learning_rate: 0.1,
        min_sample_size: 10,
        window_size: 100,
        feedback_weight: 0.7,
        success_threshold: 0.8,
        algorithm: LearningAlgorithm::Reinforcement,
    };

    advanced_pattern.configure_context_learning(learning_config);
    println!(
        "✅ Configured context learning (Rate: 0.1, Min samples: 10, Algorithm: Reinforcement)"
    );

    // Example 9: Performance Metrics Tracking
    println!("9️⃣ Tracking performance metrics:");

    // Simulate multiple renders with different performance
    advanced_pattern.update_performance_metrics(150.0, true); // Cache hit
    advanced_pattern.update_performance_metrics(300.0, false); // Cache miss
    advanced_pattern.update_performance_metrics(120.0, true); // Cache hit
    advanced_pattern.update_performance_metrics(250.0, false); // Cache miss
    advanced_pattern.update_performance_metrics(180.0, true); // Cache hit

    if let Some(metrics) = &advanced_pattern.performance_metrics {
        println!("✅ Performance metrics:");
        println!(
            "   - Average rendering time: {:.1}ms",
            metrics.avg_rendering_time
        );
        println!(
            "   - Cache hit rate: {:.1}%",
            metrics.cache_hit_rate * 100.0
        );
        println!("   - Total renders: {}", metrics.total_renders);
        println!(
            "   - Min/Max render time: {:.1}ms / {:.1}ms",
            metrics.min_rendering_time, metrics.max_rendering_time
        );
    }

    // Example 10: Context Quality Metrics
    println!("🔟 Setting context quality metrics:");

    let quality_metrics = ContextQualityMetrics {
        relevance_score: 0.92,
        completeness_score: 0.88,
        accuracy_score: 0.95,
        timeliness_score: 0.90,
        overall_score: 0.91,
        assessed_at: chrono::Utc::now(),
        improvement_suggestions: vec![
            "Add more context about dependencies".to_string(),
            "Include recent commit history".to_string(),
            "Add performance benchmarks".to_string(),
        ],
    };

    advanced_pattern.update_context_quality(quality_metrics);
    println!("✅ Set context quality metrics (Overall score: 0.91)");

    // Example 11: Testing All Features Together
    println!("1️⃣1️⃣ Testing all features together:");

    // Test template validation
    let template_errors = advanced_pattern.validate_template(context, &composition_vars);
    if template_errors.is_empty() {
        println!("✅ Template validation passed");
    } else {
        println!("❌ Template validation errors:");
        for error in &template_errors {
            println!("   - {}", error);
        }
    }

    // Test feature flags
    println!("✅ Feature status:");
    println!(
        "   - Context caching: {}",
        advanced_pattern.is_context_caching_enabled()
    );
    println!(
        "   - Context optimization: {}",
        advanced_pattern.is_context_optimization_enabled()
    );
    println!(
        "   - Context learning: {}",
        advanced_pattern.is_context_learning_enabled()
    );
    println!(
        "   - Multi-file context: {}",
        advanced_pattern.supports_multi_file_context()
    );

    // Test composition blocks
    let conditional_blocks =
        advanced_pattern.get_composition_blocks(Some(CompositionBlockType::Conditional));
    let loop_blocks = advanced_pattern.get_composition_blocks(Some(CompositionBlockType::Loop));
    let include_blocks =
        advanced_pattern.get_composition_blocks(Some(CompositionBlockType::Include));

    println!("✅ Composition blocks:");
    println!("   - Conditional blocks: {}", conditional_blocks.len());
    println!("   - Loop blocks: {}", loop_blocks.len());
    println!("   - Include blocks: {}", include_blocks.len());

    // Test advanced variables
    if let Some(lang_var) = advanced_pattern.get_advanced_variable("LANGUAGE") {
        println!("✅ Advanced variable 'LANGUAGE':");
        println!("   - Type: {:?}", lang_var.var_type);
        println!("   - Required: {}", lang_var.required);
        println!(
            "   - Description: {}",
            lang_var
                .description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        );
    }

    // Example 12: Creating a Complete Prompts Structure
    println!("1️⃣2️⃣ Creating a complete prompts.yaml structure:");

    let mut prompts = Prompts {
        prompts: vec![advanced_pattern],
    };

    // Add a simple pattern for comparison
    let mut simple_pattern = PromptPattern::new(
        "simple-template",
        "Simple Template",
        "Simple review: {{CONTEXT}}",
        PromptInjectionMethod::TemplateVariable,
    );

    // Add basic context caching
    let simple_cache_config = ContextCacheConfig {
        enabled: true,
        ttl_seconds: 1800,                // 30 minutes
        max_size_bytes: 50 * 1024 * 1024, // 50MB
        invalidation_strategy: CacheInvalidationStrategy::TimeBased,
        compression_enabled: false,
        persistence_enabled: false,
    };
    simple_pattern.configure_context_cache(simple_cache_config);

    prompts.prompts.push(simple_pattern);

    println!(
        "✅ Created prompts.yaml with {} patterns:",
        prompts.prompts.len()
    );
    for pattern in &prompts.prompts {
        println!("   - {} (ID: {})", pattern.name, pattern.id);
        println!("     Caching: {}", pattern.is_context_caching_enabled());
        println!(
            "     Optimization: {}",
            pattern.is_context_optimization_enabled()
        );
        println!("     Learning: {}", pattern.is_context_learning_enabled());

        if let Some(blocks) = &pattern.composition_blocks {
            println!("     Composition blocks: {}", blocks.len());
        }
        if let Some(vars) = &pattern.advanced_variables {
            println!("     Advanced variables: {}", vars.len());
        }
        if let Some(rules) = &pattern.validation_rules {
            println!("     Validation rules: {}", rules.len());
        }
    }

    // Example 13: Serialization Test
    println!("1️⃣3️⃣ Testing serialization:");

    let yaml = serde_yaml::to_string(&prompts)?;
    println!("✅ Generated YAML (first 800 chars):");
    println!("{}", &yaml[..yaml.len().min(800)]);

    println!("\n🎉 P2 & P3 Advanced Features Example Completed!");
    println!("===============================================");
    println!("✅ Template composition blocks (conditional, loop, include)");
    println!("✅ Advanced variables with validation");
    println!("✅ Template validation rules");
    println!("✅ Context caching configuration");
    println!("✅ Context optimization settings");
    println!("✅ Context learning configuration");
    println!("✅ Performance metrics tracking");
    println!("✅ Context quality assessment");
    println!("✅ Complete prompts.yaml structure");
    println!("✅ Full serialization support");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_template_features() {
        let mut pattern = PromptPattern::new(
            "test-advanced",
            "Test Advanced",
            "Test: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Test composition blocks
        let block = CompositionBlock {
            id: "test-block".to_string(),
            block_type: CompositionBlockType::Conditional,
            condition: Some("true".to_string()),
            loop_variable: None,
            loop_items: None,
            content: "Test content".to_string(),
            priority: Some(1),
            variables: None,
        };
        pattern.add_composition_block(block);

        let blocks = pattern.get_composition_blocks(None);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].id, "test-block");

        // Test advanced variables
        let var = AdvancedVariable {
            name: "TEST_VAR".to_string(),
            var_type: VariableType::String,
            default_value: Some("default".to_string()),
            validation: None,
            description: None,
            required: true,
            constraints: None,
        };
        pattern.add_advanced_variable(var);

        let found_var = pattern.get_advanced_variable("TEST_VAR");
        assert!(found_var.is_some());
        assert_eq!(found_var.unwrap().name, "TEST_VAR");

        // Test validation
        let mut test_vars = HashMap::new();
        test_vars.insert("TEST_VAR".to_string(), "value".to_string());
        let errors = pattern.validate_advanced_variables(&test_vars);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_enhanced_context_management() {
        let mut pattern = PromptPattern::new(
            "test-context",
            "Test Context",
            "Test: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Test context caching
        let cache_config = ContextCacheConfig {
            enabled: true,
            ttl_seconds: 3600,
            max_size_bytes: 1000000,
            invalidation_strategy: CacheInvalidationStrategy::TimeBased,
            compression_enabled: true,
            persistence_enabled: false,
        };
        pattern.configure_context_cache(cache_config);
        assert!(pattern.is_context_caching_enabled());

        // Test context optimization
        let opt_config = ContextOptimizationConfig {
            enabled: true,
            max_tokens: 4000,
            min_relevance_score: 0.8,
            semantic_compression: true,
            structure_optimization: true,
            relevance_filtering: true,
            algorithm: OptimizationAlgorithm::Hybrid,
        };
        pattern.configure_context_optimization(opt_config);
        assert!(pattern.is_context_optimization_enabled());

        // Test context learning
        let learn_config = ContextLearningConfig {
            enabled: true,
            learning_rate: 0.1,
            min_sample_size: 10,
            window_size: 100,
            feedback_weight: 0.7,
            success_threshold: 0.8,
            algorithm: LearningAlgorithm::Reinforcement,
        };
        pattern.configure_context_learning(learn_config);
        assert!(pattern.is_context_learning_enabled());

        // Test performance metrics
        pattern.update_performance_metrics(150.0, true);
        pattern.update_performance_metrics(200.0, false);

        if let Some(metrics) = &pattern.performance_metrics {
            assert_eq!(metrics.total_renders, 2);
            assert!(metrics.cache_hit_rate > 0.0);
        }

        // Test context quality
        let quality = ContextQualityMetrics {
            relevance_score: 0.9,
            completeness_score: 0.8,
            accuracy_score: 0.95,
            timeliness_score: 0.85,
            overall_score: 0.875,
            assessed_at: chrono::Utc::now(),
            improvement_suggestions: vec!["Test suggestion".to_string()],
        };
        pattern.update_context_quality(quality);

        let score = pattern.get_context_quality_score();
        assert!(score.is_some());
        assert_eq!(score.unwrap(), 0.875);
    }

    #[test]
    fn test_template_composition() {
        let mut pattern = PromptPattern::new(
            "test-composition",
            "Test Composition",
            "Base: {{CONTEXT}}",
            PromptInjectionMethod::TemplateVariable,
        );

        // Add conditional block
        let conditional_block = CompositionBlock {
            id: "conditional".to_string(),
            block_type: CompositionBlockType::Conditional,
            condition: Some("true".to_string()),
            loop_variable: None,
            loop_items: None,
            content: "\nConditional content".to_string(),
            priority: Some(1),
            variables: None,
        };
        pattern.add_composition_block(conditional_block);

        // Add loop block
        let loop_block = CompositionBlock {
            id: "loop".to_string(),
            block_type: CompositionBlockType::Loop,
            condition: None,
            loop_variable: Some("item".to_string()),
            loop_items: Some(vec!["a".to_string(), "b".to_string()]),
            content: "- {{item}}\n".to_string(),
            priority: Some(2),
            variables: None,
        };
        pattern.add_composition_block(loop_block);

        let mut variables = HashMap::new();
        variables.insert("LANGUAGE".to_string(), "Rust".to_string());

        let result = pattern.render_with_composition("test context", &variables);
        assert!(result.contains("Conditional content"));
        assert!(result.contains("- a"));
        assert!(result.contains("- b"));
    }
}
