# Comprehensive Configuration Validation System

## Overview

The Rhema Comprehensive Configuration Validation System provides a robust, extensible framework for validating configuration files against JSON schemas, business rules, and custom validation logic. The system is designed to be highly configurable, performant, and easy to integrate into existing workflows.

## Architecture

The validation system consists of several key components:

### 1. Schema Validator (`SchemaValidator`)

The schema validator provides JSON Schema validation using the `jsonschema` crate. It supports:

- **Built-in schemas**: Automatically loads schemas from the `schemas/` directory
- **Custom schemas**: Allows loading custom JSON schemas
- **Semantic validation**: Performs additional validation beyond JSON Schema
- **Caching**: Caches validation results for performance
- **Strict mode**: Configurable strict vs. lenient validation

### 2. Comprehensive Validator (`ComprehensiveValidator`)

The comprehensive validator combines multiple validation approaches:

- **Schema validation**: JSON Schema validation
- **Business logic validation**: Custom business rules
- **Cross-reference validation**: Validates references between configurations
- **Security validation**: Checks for security issues
- **Performance validation**: Identifies performance concerns
- **Compliance validation**: Ensures compliance with standards

### 3. Validation Rules Manager (`ValidationRulesManager`)

The rules manager provides a flexible rule-based validation system:

- **Rule definition**: Define custom validation rules in YAML
- **Condition evaluation**: Support for complex condition logic
- **Action execution**: Configurable actions for rule violations
- **Rule compilation**: Efficient rule evaluation
- **Rule sets**: Group related rules together

## Quick Start

### Basic Usage

```rust
use rhema_config::{
    ComprehensiveValidator, GlobalConfig, SchemaType, ValidationLevel
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a global configuration
    let global_config = GlobalConfig::default();

    // Create a comprehensive validator
    let validator = ComprehensiveValidator::with_settings(
        &global_config,
        300, // cache TTL
        ValidationLevel::Complete, // validation level
        true, // auto-fix
    ).await?;

    // Validate a configuration
    let config = serde_json::json!({
        "rhema": {
            "version": "1.0.0",
            "scope": {
                "type": "repository",
                "name": "test-repo"
            }
        }
    });

    let result = validator
        .validate_config_value(&config, &SchemaType::Rhema, std::path::Path::new("test.yaml"))
        .await?;

    println!("Valid: {}", result.valid);
    println!("Issues: {}", result.issues.len());

    Ok(())
}
```

### Schema Validation Only

```rust
use rhema_config::{SchemaValidator, SchemaType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = SchemaValidator::new()?;
    
    let config = serde_json::json!({
        "rhema": {
            "version": "1.0.0",
            "scope": {
                "type": "repository",
                "name": "test-repo"
            }
        }
    });

    let result = validator.validate_against_schema(&config, &SchemaType::Rhema).await?;
    
    println!("Schema valid: {}", result.valid);
    
    Ok(())
}
```

### Custom Validation Rules

```rust
use rhema_config::{
    ValidationRulesConfig, ValidationRule, RuleType, RuleCondition, 
    ConditionOperator, RuleAction, ActionType, ValidationRulesManager,
    ConfigIssueSeverity
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules_config = ValidationRulesConfig::new();

    // Create a custom rule
    let rule = ValidationRule {
        id: "required-fields".to_string(),
        name: "Required Fields Check".to_string(),
        description: "Ensure all required fields are present".to_string(),
        rule_type: RuleType::Schema,
        severity: ConfigIssueSeverity::Error,
        enabled: true,
        conditions: vec![
            RuleCondition {
                field: "rhema.version".to_string(),
                operator: ConditionOperator::Exists,
                value: serde_json::json!(true),
                case_sensitive: None,
            }
        ],
        actions: vec![
            RuleAction {
                action_type: ActionType::Error,
                parameters: HashMap::new(),
                enabled: true,
            }
        ],
        metadata: HashMap::new(),
    };

    rules_config.add_rule(rule);
    let manager = ValidationRulesManager::new(rules_config)?;

    let config = serde_json::json!({
        "rhema": {
            "version": "1.0.0"
        }
    });

    let results = manager.evaluate_rules(&config, "test").await?;
    
    for result in results {
        println!("Rule '{}': conditions met = {}", 
            result.rule.name, result.conditions_met);
    }

    Ok(())
}
```

## Validation Levels

The comprehensive validator supports four validation levels:

### 1. Basic
- Schema validation only
- Fastest validation
- Minimal resource usage

### 2. Standard
- Schema validation
- Basic business rules
- Cross-reference validation
- Good balance of speed and thoroughness

### 3. Strict
- All Standard validations
- Security validation
- Dependency validation
- Comprehensive checking

### 4. Complete
- All Strict validations
- Performance validation
- Compliance validation
- Custom rule evaluation
- Most thorough but slowest

## Validation Rules Configuration

Validation rules can be defined in YAML configuration files:

```yaml
# validation_rules.yaml
global_settings:
  strict_mode: false
  auto_fix: true
  cache_enabled: true
  cache_ttl: 300

rules:
  - id: "required-fields"
    name: "Required Fields Check"
    description: "Ensure all required fields are present"
    rule_type: "Schema"
    severity: "Error"
    enabled: true
    conditions:
      - field: "rhema.version"
        operator: "Exists"
        value: true
    actions:
      - action_type: "Error"
        parameters: {}
        enabled: true

rule_sets:
  schema_validation:
    name: "Schema Validation Rules"
    description: "Core schema validation rules"
    rules: ["required-fields"]
    enabled: true
    priority: 1
```

### Condition Operators

The validation system supports a wide range of condition operators:

#### Comparison Operators
- `Equals`: Field equals value
- `NotEquals`: Field does not equal value
- `GreaterThan`: Field is greater than value
- `LessThan`: Field is less than value
- `GreaterThanOrEqual`: Field is greater than or equal to value
- `LessThanOrEqual`: Field is less than or equal to value

#### String Operators
- `Contains`: Field contains substring
- `NotContains`: Field does not contain substring
- `StartsWith`: Field starts with prefix
- `EndsWith`: Field ends with suffix
- `Regex`: Field matches regex pattern

#### Existence Operators
- `Exists`: Field exists
- `NotExists`: Field does not exist
- `IsEmpty`: Field is empty string
- `IsNotEmpty`: Field is not empty string
- `IsNull`: Field is null
- `IsNotNull`: Field is not null

#### Array Operators
- `In`: Field value is in array
- `NotIn`: Field value is not in array

### Action Types

When validation rules are triggered, various actions can be executed:

- `Log`: Log the violation
- `Warn`: Issue a warning
- `Error`: Raise an error
- `Fix`: Attempt to auto-fix the issue
- `Skip`: Skip validation
- `Transform`: Transform the value
- `Custom`: Execute custom action

## Schema Types

The system supports validation against different schema types:

- `Rhema`: Main Rhema configuration
- `Scope`: Scope configuration
- `Knowledge`: Knowledge base configuration
- `Todos`: Todo items configuration
- `Decisions`: Architecture decision records
- `Patterns`: Design patterns configuration
- `Conventions`: Coding conventions
- `Lock`: Lock configuration
- `Action`: Action configuration
- `Custom`: Custom schema types

## Performance Features

### Caching
- Validation results are cached for improved performance
- Configurable cache TTL
- Automatic cache invalidation

### Parallel Processing
- Support for parallel validation of multiple files
- Configurable parallel processing limits
- Efficient resource utilization

### Lazy Loading
- Schemas are loaded on-demand
- Memory-efficient for large schema collections
- Background schema compilation

## Security Features

### Sensitive Information Detection
- Automatic detection of exposed secrets
- Password, token, and credential validation
- Environment variable suggestion

### Input Validation
- Comprehensive input sanitization
- Path traversal prevention
- Malicious content detection

## Error Handling

The validation system provides detailed error information:

```rust
let result = validator.validate_config_value(&config, &SchemaType::Rhema, path).await?;

if !result.valid {
    for issue in &result.issues {
        println!("[{}] {}: {}", 
            issue.severity, 
            issue.category, 
            issue.message
        );
        println!("  Path: {}", issue.path);
        println!("  Code: {}", issue.code);
        
        if issue.auto_fixable {
            println!("  Auto-fixable: Yes");
            if let Some(fix) = &issue.suggested_fix {
                println!("  Suggested fix: {:?}", fix);
            }
        }
    }
}
```

## Integration Examples

### CLI Integration

```rust
use clap::{App, Arg};
use rhema_config::{ComprehensiveValidator, GlobalConfig, ValidationLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("rhema-validate")
        .arg(Arg::with_name("config")
            .help("Configuration file to validate")
            .required(true)
            .index(1))
        .arg(Arg::with_name("level")
            .short("l")
            .long("level")
            .help("Validation level")
            .default_value("standard"))
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let level = match matches.value_of("level").unwrap() {
        "basic" => ValidationLevel::Basic,
        "standard" => ValidationLevel::Standard,
        "strict" => ValidationLevel::Strict,
        "complete" => ValidationLevel::Complete,
        _ => ValidationLevel::Standard,
    };

    let global_config = GlobalConfig::default();
    let validator = ComprehensiveValidator::with_settings(
        &global_config,
        300,
        level,
        false,
    ).await?;

    let result = validator
        .validate_config_file(config_file, &SchemaType::Rhema)
        .await?;

    if result.valid {
        println!("✅ Configuration is valid");
    } else {
        println!("❌ Configuration has {} issues", result.issues.len());
        std::process::exit(1);
    }

    Ok(())
}
```

### CI/CD Integration

```yaml
# .github/workflows/validate.yml
name: Validate Configuration

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Validate Configuration
        run: |
          cargo run --bin rhema-validate -- \
            --level complete \
            --config rhema.yaml
```

## Best Practices

### 1. Choose Appropriate Validation Level
- Use `Basic` for fast feedback during development
- Use `Standard` for most validation scenarios
- Use `Strict` for production deployments
- Use `Complete` for security-critical applications

### 2. Define Clear Validation Rules
- Use descriptive rule names and descriptions
- Group related rules into rule sets
- Set appropriate severity levels
- Include helpful error messages

### 3. Optimize Performance
- Enable caching for repeated validations
- Use parallel processing for multiple files
- Set appropriate cache TTL values
- Monitor validation performance

### 4. Handle Errors Gracefully
- Always check validation results
- Provide meaningful error messages
- Implement auto-fix where possible
- Log validation issues for debugging

### 5. Security Considerations
- Never expose sensitive information in error messages
- Validate all user inputs
- Use environment variables for secrets
- Regularly update validation rules

## Troubleshooting

### Common Issues

1. **Schema Loading Failures**
   - Ensure schema files exist in the `schemas/` directory
   - Check schema file syntax
   - Verify schema file permissions

2. **Performance Issues**
   - Reduce validation level if needed
   - Enable caching
   - Use parallel processing
   - Optimize rule conditions

3. **Memory Usage**
   - Clear validation cache periodically
   - Use lazy loading for large schemas
   - Limit parallel processing

4. **False Positives**
   - Review rule conditions
   - Adjust validation level
   - Add rule exceptions where appropriate

### Debugging

Enable debug logging to troubleshoot validation issues:

```rust
use tracing::Level;

tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();
```

## API Reference

For detailed API documentation, see the generated documentation:

```bash
cargo doc --open
```

## Contributing

When contributing to the validation system:

1. Add tests for new features
2. Update documentation
3. Follow the existing code style
4. Add validation rules for new schema types
5. Consider performance implications

## License

This validation system is part of the Rhema project and is licensed under the Apache License, Version 2.0. 