# Validation System

Rhema's validation system ensures data integrity, consistency, and compliance across all context files. It provides comprehensive validation for YAML files, schemas, dependencies, and cross-references.

## 🎯 Overview

The validation system helps you:
- Ensure data integrity and consistency
- Catch errors early in the development process
- Maintain compliance with schemas and standards
- Validate dependencies and relationships
- Prevent data corruption and inconsistencies

## 🔍 Types of Validation

### Schema Validation
Validates YAML files against their JSON schemas to ensure proper structure and data types.

### Cross-Reference Validation
Checks that references between files and scopes are valid and consistent.

### Dependency Validation
Validates scope dependencies and ensures they're properly configured.

### Lock File Validation
Validates lock files against current state to ensure consistency.

### Custom Validation Rules
Allows you to define custom validation rules for your specific needs.

## 🚀 Basic Validation

### Validate All Files
```bash
# Validate all YAML files in the current scope
rhema validate

# Validate recursively in subdirectories
rhema validate --recursive

# Validate with verbose output
rhema validate --verbose
```

### Validate Specific Files
```bash
# Validate specific file types
rhema validate --file todos.yaml
rhema validate --file insights.yaml
rhema validate --file decisions.yaml

# Validate multiple files
rhema validate --file todos.yaml,insights.yaml,decisions.yaml
```

### Validation Output
```bash
# Default output (errors only)
rhema validate

# Include warnings
rhema validate --include-warnings

# Show JSON schemas
rhema validate --json-schema

# Output in different formats
rhema validate --format json
rhema validate --format table
```

## 🔧 Advanced Validation Options

### Strict Validation
```bash
# Treat warnings as errors
rhema validate --strict

# Strict validation with recursive checking
rhema validate --strict --recursive
```

### Schema Migration
```bash
# Migrate schemas to latest version
rhema validate --migrate

# Dry run migration (don't modify files)
rhema validate --migrate --dry-run

# Migrate recursively
rhema validate --migrate --recursive
```

### Lock File Validation
```bash
# Validate against lock file
rhema validate --lock-file

# Validate lock file only
rhema validate --lock-only

# Strict lock file validation
rhema validate --lock-file --strict
```

## 📋 Validation Rules

### Todo Validation
```yaml
# todos.yaml
tasks:
  - id: "TASK-001"
    title: "Implement user authentication"
    description: "Add JWT-based authentication system"
    priority: "high"
    status: "todo"
    tags:
      - "feature"
      - "security"
    created_date: "2024-01-15T10:00:00Z"
    due_date: "2024-01-30T17:00:00Z"
    assignee: "john.doe@company.com"
    context:
      related_files:
        - "src/auth/service.rs"
        - "src/auth/middleware.rs"
      dependencies:
        - "auth-library"
    acceptance_criteria:
      - "JWT tokens are properly generated"
      - "Authentication middleware is implemented"
      - "Tests are written and passing"
```

**Validation Rules:**
- `id` must be unique within the scope
- `title` is required and non-empty
- `priority` must be one of: low, medium, high, critical
- `status` must be one of: todo, in_progress, review, done, cancelled
- `created_date` must be a valid ISO 8601 date
- `due_date` must be after `created_date`
- `tags` must be an array of strings
- `context.related_files` must contain valid file paths
- `acceptance_criteria` must be a non-empty array

### Insight Validation
```yaml
# insights.yaml
insights:
  - id: "INSIGHT-001"
    insight: "Using Redis for session storage improves performance by 40%"
    confidence: "high"
    category: "performance"
    tags:
      - "performance"
      - "caching"
      - "redis"
    created_date: "2024-01-15T14:30:00Z"
    context:
      related_files:
        - "src/session/service.rs"
      evidence:
        - "Benchmark results show 40% improvement"
        - "Memory usage reduced by 60%"
      impact:
        scope: "user-service"
        severity: "high"
    references:
      - type: "benchmark"
        url: "https://example.com/benchmark-results"
      - type: "documentation"
        url: "https://redis.io/docs/manual/patterns/distributed-locks/"
```

**Validation Rules:**
- `insight` is required and non-empty
- `confidence` must be one of: low, medium, high
- `category` must be one of: performance, security, architecture, process, other
- `created_date` must be a valid ISO 8601 date
- `context.evidence` must be a non-empty array
- `context.impact.severity` must be one of: low, medium, high, critical
- `references` must contain valid URLs

### Decision Validation
```yaml
# decisions.yaml
decisions:
  - id: "DECISION-001"
    title: "Use GraphQL for API"
    description: "GraphQL provides better flexibility for mobile clients"
    status: "approved"
    decision_type: "architecture"
    tags:
      - "api"
      - "graphql"
      - "mobile"
    created_date: "2024-01-15T09:00:00Z"
    decided_date: "2024-01-20T16:00:00Z"
    context:
      problem: "REST API is too rigid for mobile client needs"
      alternatives:
        - name: "REST API"
          description: "Traditional REST endpoints"
          pros: ["Simple", "Well understood"]
          cons: ["Rigid", "Over-fetching"]
        - name: "GraphQL"
          description: "Query language for APIs"
          pros: ["Flexible", "Efficient"]
          cons: ["Complex", "Learning curve"]
      criteria:
        - "Mobile client flexibility"
        - "Performance"
        - "Development speed"
      outcome: "GraphQL provides the best balance of flexibility and performance"
    stakeholders:
      - name: "John Doe"
        role: "Tech Lead"
        email: "john.doe@company.com"
    implementation:
      timeline: "2 weeks"
      resources:
        - "Backend team"
        - "Mobile team"
      risks:
        - "Learning curve for team"
        - "Additional complexity"
```

**Validation Rules:**
- `title` is required and non-empty
- `status` must be one of: proposed, approved, rejected, implemented, deprecated
- `decision_type` must be one of: architecture, technology, process, business, other
- `created_date` must be a valid ISO 8601 date
- `decided_date` must be after `created_date`
- `context.alternatives` must contain at least 2 alternatives
- `stakeholders` must be a non-empty array
- `implementation.timeline` must be a valid duration

## 🔗 Cross-Reference Validation

### File References
```yaml
# Validates that referenced files exist
context:
  related_files:
    - "src/auth/service.rs"  # Must exist
    - "src/auth/middleware.rs"  # Must exist
```

### Scope References
```yaml
# Validates that referenced scopes exist
dependencies:
  - name: "auth-library"  # Must exist
    version: "2.1.0"
  - name: "database-service"  # Must exist
    version: "1.5.0"
```

### ID References
```yaml
# Validates that referenced IDs exist
related_items:
  - type: "todo"
    id: "TASK-001"  # Must exist in todos.yaml
  - type: "insight"
    id: "INSIGHT-001"  # Must exist in insights.yaml
```

## 🔒 Lock File Validation

### Lock File Structure
```yaml
# rhema.lock
version: "1.0.0"
generated: "2024-01-15T10:00:00Z"
scopes:
  user-service:
    version: "1.0.0"
    dependencies:
      auth-library:
        version: "2.1.0"
        resolved: "2.1.0"
        integrity: "sha256:abc123..."
      database-service:
        version: "1.5.0"
        resolved: "1.5.0"
        integrity: "sha256:def456..."
  auth-library:
    version: "2.1.0"
    dependencies: {}
```

### Lock File Validation
```bash
# Validate lock file against current state
rhema validate --lock-file

# Validate lock file only
rhema validate --lock-only

# Strict lock file validation
rhema validate --lock-file --strict

# Compare lock file with current state
rhema dependencies --compare
```

## 🎯 Custom Validation Rules

### Custom Schema Validation
```yaml
# custom-validation.yaml
rules:
  - name: "todo_priority_consistency"
    description: "High priority todos must have due dates"
    condition: "priority = high"
    validation: "due_date != null"
    severity: "error"
  
  - name: "insight_evidence_required"
    description: "High confidence insights must have evidence"
    condition: "confidence = high"
    validation: "context.evidence.length > 0"
    severity: "warning"
  
  - name: "decision_stakeholder_required"
    description: "Approved decisions must have stakeholders"
    condition: "status = approved"
    validation: "stakeholders.length > 0"
    severity: "error"
```

### Apply Custom Rules
```bash
# Apply custom validation rules
rhema validate --rules custom-validation.yaml

# Apply rules with specific severity
rhema validate --rules custom-validation.yaml --severity error
```

## 📊 Validation Reports

### Generate Validation Report
```bash
# Generate detailed validation report
rhema validate --report

# Save report to file
rhema validate --report --output-file validation-report.json

# Include statistics in report
rhema validate --report --stats
```

### Report Format
```json
{
  "validation_report": {
    "generated": "2024-01-15T10:00:00Z",
    "scope": "user-service",
    "summary": {
      "total_files": 5,
      "valid_files": 4,
      "invalid_files": 1,
      "warnings": 3,
      "errors": 1
    },
    "files": [
      {
        "file": "todos.yaml",
        "status": "valid",
        "warnings": 1,
        "errors": 0,
        "issues": [
          {
            "type": "warning",
            "message": "Todo TASK-001 has no due date",
            "line": 5,
            "column": 10
          }
        ]
      }
    ]
  }
}
```

## 🔧 Validation Configuration

### Global Validation Settings
```yaml
# .rhema/config.yaml
validation:
  strict: false
  recursive: true
  auto_fix: false
  include_warnings: true
  custom_rules: "custom-validation.yaml"
  
  schemas:
    todos: "schemas/todos.json"
    insights: "schemas/insights.json"
    decisions: "schemas/decisions.json"
  
  lock_file:
    enabled: true
    strict: false
    auto_update: false
```

### Scope-Specific Validation
```yaml
# scope.yaml
config:
  validation:
    strict: true
    recursive: true
    custom_rules: "scope-validation.yaml"
    
    rules:
      - name: "scope_specific_rule"
        description: "Scope-specific validation rule"
        condition: "priority = critical"
        validation: "assignee != null"
        severity: "error"
```

## 🚨 Common Validation Issues

### Schema Violations
**Problem**: `Error: Invalid field 'invalid_field' in todos.yaml`
**Solution**: Remove invalid fields or update schema

### Missing Required Fields
**Problem**: `Error: Required field 'title' missing in todo`
**Solution**: Add missing required fields

### Invalid Data Types
**Problem**: `Error: Field 'priority' must be string, got number`
**Solution**: Convert data to correct type

### Cross-Reference Errors
**Problem**: `Error: Referenced file 'missing.rs' not found`
**Solution**: Fix file paths or remove invalid references

### Lock File Conflicts
**Problem**: `Error: Lock file version mismatch`
**Solution**: Update lock file with `rhema lock update`

## 🔄 Validation in CI/CD

### GitHub Actions Example
```yaml
# .github/workflows/validate.yml
name: Validate Rhema Files

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install Rhema
        run: cargo install --path .
      
      - name: Validate Files
        run: rhema validate --recursive --strict
      
      - name: Validate Lock File
        run: rhema validate --lock-file --strict
      
      - name: Generate Validation Report
        run: rhema validate --report --output-file validation-report.json
      
      - name: Upload Validation Report
        uses: actions/upload-artifact@v3
        with:
          name: validation-report
          path: validation-report.json
```

### Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Validating Rhema files..."

# Validate all files
if ! rhema validate --recursive; then
  echo "Validation failed. Please fix the issues before committing."
  exit 1
fi

# Validate lock file
if ! rhema validate --lock-file; then
  echo "Lock file validation failed. Please update the lock file."
  exit 1
fi

echo "Validation passed!"
```

## 📚 Related Documentation

- **[CLI Command Reference](../user-guide/cli-command-reference.md)** - Complete command documentation
- **[Configuration Management](../user-guide/configuration-management.md)** - Managing validation configuration
- **[Lock File System](./lock-file-system.md)** - Understanding lock file validation
- **[Examples](../examples/)** - Validation examples and patterns 