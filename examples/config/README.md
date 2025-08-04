# Configuration Examples

This directory contains examples demonstrating Rhema configuration patterns, validation rules, and setup strategies.

## Examples

### YAML Configuration Files
- **`action_intent_example.yaml`**: Complete action intent configuration with safety checks and approval workflows
- **`validation_rules_example.yaml`**: Comprehensive validation rule definitions for configuration validation

### Configuration Validation
- **`comprehensive_validation_example.rs`**: Advanced validation scenarios and custom validation rules

## Key Features Demonstrated

### Action Intent Configuration
- **Safety Levels**: Risk assessment and safety checks
- **Approval Workflows**: Multi-level approval processes
- **Transformation Rules**: Code transformation and validation
- **Metadata Management**: Priority, effort estimation, and impact analysis
- **Rollback Strategies**: Automated rollback mechanisms

### Validation Rules
- **Schema Validation**: Ensuring configuration structure compliance
- **Business Rules**: Domain-specific validation logic
- **Security Checks**: Sensitive information detection
- **Performance Validation**: Resource usage and performance constraints
- **Compliance Checking**: Regulatory and policy compliance

## Configuration Patterns

### Basic Structure
```yaml
rhema:
  version: "1.0.0"
  intent:
    id: "unique-id"
    action_type: "operation-type"
    description: "Human-readable description"
    scope: ["path/to/files"]
    safety_level: "low|medium|high"
```

### Safety Configuration
```yaml
safety_checks:
  pre_execution:
    - "syntax_validation"
    - "type_checking"
  post_execution:
    - "build_validation"
    - "test_execution"
```

### Approval Workflow
```yaml
approval_workflow:
  required: true
  approvers: ["role1", "role2"]
  auto_approve_for: ["low_risk"]
  timeout: 3600
```

## Usage

These examples are essential for:
- **Setting up Rhema projects**: Understanding configuration structure
- **Implementing validation**: Creating custom validation rules
- **Security compliance**: Ensuring secure configuration practices
- **Production deployment**: Configuring for production environments 