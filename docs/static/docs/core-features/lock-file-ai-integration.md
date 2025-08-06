# Lock File AI Integration

This document describes the integration of lock file information into MCP context for AI agents, providing comprehensive dependency awareness and decision-making capabilities.

## Overview

The lock file AI integration enables AI agents to make informed decisions about dependency management, conflict resolution, and project health by providing access to detailed lock file information through the MCP (Model Context Protocol) context system.

## Features

### 1. Lock File Context Injection for AI Operations

AI agents can now access comprehensive lock file information during their operations, including:

- **Dependency Version Information**: Current versions, constraints, and resolution details
- **Conflict Prevention Data**: Version conflicts, circular dependencies, and resolution strategies
- **Health Assessment**: Lock file validation status, performance metrics, and health scores
- **Context-Aware Recommendations**: Intelligent suggestions based on lock file analysis

### 2. Dependency Version Awareness in Agent Context

AI agents receive detailed dependency information including:

```rust
// Example: Getting dependency versions for a scope
let context = integration.get_scope_context("crates/rhema-core")?;
for dep in context.dependencies {
    println!("{}: {} ({})", dep.name, dep.version, dep.dependency_type);
}
```

### 3. Conflict Prevention Information

The system provides comprehensive conflict analysis:

- **Version Conflicts**: Detection of conflicting dependency versions across scopes
- **Circular Dependencies**: Identification of circular dependency chains
- **Resolution Strategies**: Recommendations for conflict resolution
- **Dependency Graph**: Visual representation of dependency relationships

### 4. Lock File Status and Health Information

AI agents can assess lock file health through:

- **Health Scoring**: Numerical health score (0-100) based on multiple factors
- **Validation Status**: Current validation state and any issues
- **Performance Metrics**: Generation time, cache hit rates, and efficiency data
- **Issue Tracking**: Detailed list of problems and warnings

### 5. Context-Aware Dependency Recommendations

Intelligent recommendations based on lock file analysis:

- **Security Recommendations**: Updates for vulnerable dependencies
- **Performance Optimizations**: Suggestions for faster builds and runtime
- **Maintenance Recommendations**: Updates for outdated packages
- **Best Practices**: Guidance on dependency management

## Usage Examples

### Basic Integration

```rust
use rhema_ai::agent::lock_context_integration::LockFileAIIntegration;

// Create integration instance
let mut integration = LockFileAIIntegration::new(PathBuf::from("."));
integration.initialize()?;

// Get comprehensive context
let context = integration.get_comprehensive_context()?;
println!("Health Score: {:.1}/100", context.health_assessment.overall_score);
```

### Dependency Update Prompts

```rust
// Generate AI prompt for dependency updates
let prompt = integration.generate_dependency_update_prompt(
    "crates/rhema-core",
    "Analyze dependencies and suggest updates for security and performance improvements."
)?;

// Use the prompt with an AI model
let ai_response = ai_model.generate_response(&prompt).await?;
```

### Conflict Resolution

```rust
// Get conflict analysis
let conflicts = integration.get_conflict_analysis()?;

for conflict in conflicts.version_conflicts {
    println!("Conflict: {} between {} and {}", 
        conflict.dependency_name, conflict.scope1, conflict.scope2);
}
```

### Security Review

```rust
// Generate security review prompt
let security_prompt = integration.generate_security_review_prompt(
    "crates/rhema-core",
    "Perform security analysis of dependencies and identify vulnerabilities."
)?;
```

## API Reference

### LockFileAIIntegration

Main integration class for lock file AI context.

#### Methods

- `new(project_root: PathBuf) -> Self`: Create new integration instance
- `initialize() -> RhemaResult<()>`: Load lock file data
- `get_comprehensive_context() -> RhemaResult<LockFileAIContext>`: Get full context
- `get_scope_context(scope_path: &str) -> RhemaResult<ScopeLockContext>`: Get scope-specific context
- `generate_dependency_update_prompt(scope_path: &str, template: &str) -> RhemaResult<String>`: Generate dependency update prompts
- `generate_conflict_resolution_prompt(template: &str) -> RhemaResult<String>`: Generate conflict resolution prompts
- `generate_security_review_prompt(scope_path: &str, template: &str) -> RhemaResult<String>`: Generate security review prompts
- `get_dependency_recommendations(scope_path: &str) -> RhemaResult<Vec<Recommendation>>`: Get recommendations
- `get_conflict_analysis() -> RhemaResult<ConflictAnalysis>`: Get conflict analysis
- `get_health_assessment() -> RhemaResult<HealthAssessment>`: Get health assessment

### LockFileAIContext

Comprehensive context structure containing all lock file information.

#### Fields

- `summary: LockFileSummary`: High-level lock file information
- `dependency_analysis: DependencyAnalysis`: Detailed dependency analysis
- `conflict_analysis: ConflictAnalysis`: Conflict detection and analysis
- `health_assessment: HealthAssessment`: Health metrics and status
- `recommendations: Vec<Recommendation>`: AI-generated recommendations
- `scope_details: HashMap<String, ScopeAnalysis>`: Per-scope analysis
- `last_updated: Option<DateTime<Utc>>`: Last update timestamp

### Context Injection Rules

The system supports task-specific context injection rules:

```rust
let rule = ContextInjectionRule {
    task_type: TaskType::DependencyUpdate,
    context_files: vec!["knowledge.yaml".to_string()],
    injection_method: PromptInjectionMethod::Prepend,
    priority: 3,
    additional_context: Some("Dependency updates require careful consideration...".to_string()),
    lock_file_context: Some(LockFileContextRequirement {
        include_dependency_versions: true,
        include_conflict_prevention: true,
        include_health_info: true,
        include_recommendations: true,
        target_scopes: None,
        include_transitive_deps: true,
    }),
};
```

## Task Types

The system supports various task types with appropriate context injection:

- **CodeReview**: Basic dependency information and health status
- **BugFix**: Comprehensive dependency analysis and conflict detection
- **FeatureDevelopment**: Dependency recommendations and compatibility analysis
- **Testing**: Dependency versions and test-related information
- **Documentation**: Basic dependency information
- **Refactoring**: Comprehensive analysis for safe refactoring
- **SecurityReview**: Security-focused dependency analysis
- **PerformanceOptimization**: Performance-related dependency information
- **DependencyUpdate**: Full dependency analysis and update recommendations
- **Deployment**: Deployment-related dependency information
- **LockFileManagement**: Complete lock file analysis and management
- **DependencyResolution**: Conflict resolution and dependency analysis
- **ConflictResolution**: Focused conflict detection and resolution

## Health Assessment

The system provides comprehensive health assessment with scoring:

### Health Score Calculation

- **Base Score**: 100 points
- **Validation Issues**: -30 points for failed validation
- **Circular Dependencies**: -20 points per circular dependency
- **Performance Issues**: -10 points for slow generation
- **Cache Issues**: -5 points for low cache hit rate
- **Outdated Dependencies**: -5 points per outdated dependency

### Health Status Levels

- **Good (80-100)**: Healthy lock file with minor issues
- **Fair (60-79)**: Some issues that should be addressed
- **Poor (0-59)**: Significant issues requiring immediate attention

## Recommendations System

The system generates intelligent recommendations based on analysis:

### Recommendation Categories

- **Validation**: Fix validation errors and issues
- **Dependencies**: Update, resolve conflicts, or optimize dependencies
- **Performance**: Optimize build times and efficiency
- **Security**: Address security vulnerabilities
- **Maintenance**: General maintenance tasks

### Priority Levels

- **Low**: Minor improvements and optimizations
- **Medium**: Important but not urgent issues
- **High**: Significant issues requiring attention
- **Critical**: Urgent issues requiring immediate action

## Integration with MCP Context

The lock file context is integrated into the MCP context system:

```rust
// MCP context provider with lock file support
let context_provider = ContextProvider::new(repo_root)?;
context_provider.initialize().await?;

// Get lock file information through MCP
let lock_context = context_provider.get_scope_lock_context("crates/rhema-core").await?;
let dependency_versions = context_provider.get_dependency_versions("crates/rhema-core").await?;
let conflict_info = context_provider.get_conflict_prevention_info().await?;
let health_info = context_provider.get_lock_file_health().await?;
let recommendations = context_provider.get_dependency_recommendations("crates/rhema-core").await?;
```

## Best Practices

### For AI Agents

1. **Always check lock file availability** before attempting operations
2. **Use appropriate task types** for context injection
3. **Consider health scores** when making recommendations
4. **Prioritize critical issues** in recommendations
5. **Provide actionable steps** in all recommendations

### For Developers

1. **Regular health monitoring** of lock files
2. **Timely dependency updates** based on recommendations
3. **Conflict resolution** before they become critical
4. **Security review** of new dependencies
5. **Performance optimization** of dependency trees

## Error Handling

The system provides comprehensive error handling:

```rust
match integration.get_comprehensive_context() {
    Ok(context) => {
        // Process context
        println!("Health: {:.1}/100", context.health_assessment.overall_score);
    }
    Err(e) => {
        match e {
            RhemaError::FileNotFound(_) => {
                println!("No lock file found - create one first");
            }
            RhemaError::InvalidInput(msg) => {
                println!("Invalid lock file: {}", msg);
            }
            _ => {
                println!("Unexpected error: {}", e);
            }
        }
    }
}
```

## Performance Considerations

- **Caching**: Lock file data is cached for performance
- **Lazy Loading**: Context is loaded only when needed
- **Incremental Updates**: Only changed data is updated
- **Async Operations**: Non-blocking operations for better performance

## Security Considerations

- **Input Validation**: All lock file data is validated
- **Checksum Verification**: File integrity is verified
- **Access Control**: Context access can be restricted
- **Audit Logging**: All operations are logged for security

## Future Enhancements

Planned improvements include:

- **Real-time Monitoring**: Live lock file health monitoring
- **Predictive Analysis**: AI-powered dependency trend analysis
- **Automated Resolution**: Automatic conflict resolution suggestions
- **Integration APIs**: REST APIs for external tool integration
- **Advanced Analytics**: Detailed dependency analytics and reporting

## Conclusion

The lock file AI integration provides AI agents with comprehensive dependency awareness, enabling better decision-making for dependency management, conflict resolution, and project health. By leveraging lock file information through the MCP context system, AI agents can provide more accurate and actionable recommendations for maintaining healthy, secure, and performant projects. 