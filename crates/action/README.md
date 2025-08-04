# Rhema Action Protocol

The Action Protocol provides a safe, validated layer for translating AI agent intent into controlled codebase modifications. This crate extends Rhema from a "map" layer to include a comprehensive "action" layer with safety controls, validation pipelines, and human oversight.

## Overview

The Action Protocol addresses the critical need for safe agent-assisted development by providing:

- **Controlled Changes**: All agent changes go through safety validation
- **Comprehensive Validation**: Pre and post-execution safety checks
- **Reliable Rollback**: Automatic rollback for failed validations
- **Human Oversight**: Required approval for high-risk operations
- **Tool Orchestration**: Support for multiple transformation tools
- **Audit Trail**: Complete history of all actions and decisions

## Core Components

### Action Protocol Schema

The protocol defines a comprehensive schema for action intents:

```yaml
rhema:
  version: "1.0.0"
  intent:
    id: "intent-001"
    type: "refactor"
    description: "Extract authentication logic into separate module"
    scope: ["src/auth/"]
    safety_level: "medium"
    
    context_refs:
      - file: "architecture.rhema.yaml"
        section: "auth_patterns"
      - file: "knowledge.rhema.yaml"
        section: "security_best_practices"
    
    transformation:
      tools: ["jscodeshift", "prettier", "eslint"]
      validation: ["typescript", "jest", "lint"]
      rollback_strategy: "git_revert"
      
    safety_checks:
      pre_execution:
        - "syntax_validation"
        - "type_checking"
        - "test_coverage"
      post_execution:
        - "build_validation"
        - "test_execution"
        - "lint_checking"
    
    approval_workflow:
      required: true
      approvers: ["senior_dev", "security_team"]
      auto_approve_for: ["low_risk", "test_only"]
```

### Safety Pipeline Architecture

The safety pipeline ensures all changes are validated and safe:

1. **Pre-execution Validation**: Syntax, types, test coverage
2. **Backup Creation**: Automatic backup before changes
3. **Transformation Execution**: Controlled tool execution
4. **Post-execution Validation**: Build, test, lint validation
5. **Commit or Rollback**: Based on validation results

### Tool Integration Framework

Support for multiple transformation and validation tools:

- **Transformation Tools**: jscodeshift, comby, ast-grep, prettier, eslint
- **Validation Tools**: TypeScript, Jest, ESLint, build systems
- **Safety Tools**: Security scanners, compliance checkers

## CLI Commands

### Action Planning and Execution

```bash
# Plan an action
rhema intent plan "Extract authentication logic into separate module"

# Preview action changes
rhema intent preview intent-001.yaml

# Execute action with approval
rhema intent execute intent-001.yaml --require-approval

# Rollback action
rhema intent rollback intent-001
```

### Action Management

```bash
# List active intents
rhema intent list --active

# Check intent status
rhema intent status intent-001

# Validate intent file
rhema intent validate intent-001.yaml

# Show recent actions
rhema intent history --days 7
```

### Safety and Validation

```bash
# Run safety checks
rhema intent safety-check intent-001.yaml

# Validate before execution
rhema intent validate --preview

# Approve pending action
rhema intent approve intent-001

# Reject with reason
rhema intent reject intent-001 --reason "Security concerns"
```

## Architecture

### Core Modules

- **`schema`**: Action protocol schema definitions
- **`pipeline`**: Safety pipeline implementation
- **`tools`**: Tool integration framework
- **`validation`**: Validation and safety checks
- **`rollback`**: Rollback mechanisms
- **`approval`**: Human approval workflows
- **`git`**: Git integration for actions
- **`cli`**: CLI command implementations

### Key Types

- `ActionIntent`: Complete action specification
- `ActionSafetyPipeline`: Main safety pipeline
- `TransformationTool`: Tool integration trait
- `ValidationTool`: Validation tool trait
- `SafetyTool`: Safety tool trait
- `ActionResult`: Action execution result
- `ActionStatus`: Current action status

## Integration

### With Existing Rhema Features

- **Schema Integration**: Extends existing YAML schema patterns
- **CLI Integration**: Follows existing CLI command patterns
- **Git Integration**: Full integration with existing Git workflow
- **MCP Integration**: Extends MCP daemon with action endpoints

### With External Tools

- **Code Transformation**: jscodeshift, comby, ast-grep
- **Code Quality**: ESLint, Prettier, TypeScript
- **Testing**: Jest, Mocha, PyTest
- **Build Systems**: npm, cargo, maven, gradle
- **Security**: OWASP ZAP, Bandit, Semgrep

## Safety Features

### Pre-execution Safety

- Syntax validation
- Type checking
- Test coverage analysis
- Security scanning
- Dependency analysis
- Impact assessment

### Post-execution Safety

- Build validation
- Test execution
- Lint checking
- Performance impact
- Security verification
- Compliance checking

### Rollback Mechanisms

- Git revert
- File restoration
- State rollback
- Dependency rollback
- Configuration rollback

## Development

### Building

```bash
cargo build --package rhema-action
```

### Testing

```bash
cargo test --package rhema-action
```

### Documentation

```bash
cargo doc --package rhema-action --open
```

## Contributing

Please read the main Rhema contributing guidelines and ensure all code follows the established patterns and safety requirements.

## License

Apache 2.0 - see the main Rhema repository for details. 