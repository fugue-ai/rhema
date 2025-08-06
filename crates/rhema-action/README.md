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

## Future Enhancements

### External Tool Integration 🔴 HIGH PRIORITY
- [ ] **Implement actual jscodeshift integration** - Replace placeholder with real implementation
- [ ] **Implement actual comby integration** - Replace placeholder with real implementation
- [ ] **Implement actual ast-grep integration** - Replace placeholder with real implementation
- [ ] **Implement actual prettier integration** - Replace placeholder with real implementation
- [ ] **Implement actual ESLint integration** - Replace placeholder with real implementation
- [ ] **Implement actual TypeScript validation** - Replace placeholder with real implementation
- [ ] **Implement actual Jest test execution** - Replace placeholder with real implementation
- [ ] **Implement actual Mocha test execution** - Replace placeholder with real implementation
- [ ] **Implement actual PyTest execution** - Replace placeholder with real implementation
- [ ] **Implement actual Cargo check** - Replace placeholder with real implementation

**Status**: Core functionality required for action protocol
**Estimated Effort**: 2-3 weeks
**Dependencies**: ✅ **RESOLVED** - Knowledge crate integration completed, CLI daemon implementation completed

### Safety and Validation Tools
- [ ] **Implement actual syntax validation** - Replace placeholder with real implementation
- [ ] **Implement actual type checking** - Replace placeholder with real implementation
- [ ] **Implement actual test coverage analysis** - Replace placeholder with real implementation
- [ ] **Implement actual security scanning** - Replace placeholder with real implementation
- [ ] **Implement actual performance checking** - Replace placeholder with real implementation
- [ ] **Implement actual dependency analysis** - Replace placeholder with real implementation

### Tool Registry Enhancements
- [ ] **Add tool availability detection** - Detect if tools are installed and available
- [ ] **Add tool version checking** - Check tool versions for compatibility
- [ ] **Add tool configuration management** - Manage tool-specific configurations
- [ ] **Add tool performance monitoring** - Monitor tool execution performance
- [ ] **Add tool error handling and recovery** - Handle tool failures gracefully

### Human Approval Workflows
- [ ] **Implement interactive approval UI** - User interface for approval workflows
- [ ] **Add email notification system** - Email notifications for approval requests
- [ ] **Add Slack/Teams integration** - Slack and Teams integration for notifications
- [ ] **Add approval request management** - Manage approval requests and responses
- [ ] **Add approval history tracking** - Track approval history and decisions
- [ ] **Add approval delegation** - Delegate approvals to other users

### Security and Compliance
- [ ] **Implement security scanning integration** - Integrate with security scanning tools
- [ ] **Add compliance checking** - Check compliance with organizational policies
- [ ] **Add vulnerability detection** - Detect vulnerabilities in code changes
- [ ] **Add license compliance checking** - Check license compliance for dependencies
- [ ] **Add code quality metrics** - Track and enforce code quality metrics
- [ ] **Add dependency vulnerability scanning** - Scan dependencies for vulnerabilities

### Advanced Rollback
- [ ] **Implement intelligent rollback strategies** - Smart rollback based on change analysis
- [ ] **Add rollback verification** - Verify rollback success and system health
- [ ] **Add rollback history tracking** - Track rollback history and reasons
- [ ] **Add rollback impact analysis** - Analyze impact of rollbacks
- [ ] **Add rollback notification system** - Notify stakeholders of rollbacks

### Machine Learning Integration
- [ ] **Add ML-powered safety analysis** - Use ML to analyze safety of changes
- [ ] **Implement predictive validation** - Predict potential issues before they occur
- [ ] **Add intelligent tool selection** - Automatically select appropriate tools
- [ ] **Add risk assessment ML models** - ML models for risk assessment
- [ ] **Add performance prediction** - Predict performance impact of changes

### Advanced Monitoring
- [ ] **Add comprehensive audit trails** - Complete audit trails for all actions
- [ ] **Add performance monitoring** - Monitor action execution performance
- [ ] **Add resource usage tracking** - Track resource usage during actions
- [ ] **Add execution analytics** - Analytics on action execution patterns
- [ ] **Add success rate tracking** - Track success rates of different actions

### Plugin System
- [ ] **Design plugin architecture** - Design extensible plugin architecture
- [ ] **Add custom tool plugin support** - Support for custom tool plugins
- [ ] **Add custom validation plugin support** - Support for custom validation plugins
- [ ] **Add custom safety check plugin support** - Support for custom safety plugins
- [ ] **Add plugin marketplace infrastructure** - Infrastructure for plugin distribution

### Advanced Features
- [ ] **Add AI-powered action suggestions** - AI suggestions for action improvements
- [ ] **Add automated action generation** - Automatically generate actions from requirements
- [ ] **Add action templates and patterns** - Templates and patterns for common actions
- [ ] **Add action composition** - Compose complex actions from simpler ones
- [ ] **Add action orchestration** - Orchestrate complex action workflows

### Ecosystem
- [ ] **Add plugin marketplace** - Marketplace for action plugins
- [ ] **Add community contributions** - Support for community contributions
- [ ] **Add third-party integrations** - Third-party tool integrations
- [ ] **Add API ecosystem** - API ecosystem for action system
- [ ] **Add developer tools** - Developer tools for action development

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