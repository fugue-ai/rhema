# AI Service Lock File Enhancement

This document describes the enhancement of the AI service to be lock file aware, providing AI agents with comprehensive tools to understand and work with lock file constraints.

## Overview

The enhanced AI service now includes lock file awareness capabilities that enable AI agents to make informed decisions about dependency management, conflict resolution, and project health by providing access to detailed lock file information and validation.

## Key Features

### 1. Lock File Context in AI Operations

AI agents can now access comprehensive lock file information during their operations:

- **Dependency Version Information**: Current versions, constraints, and resolution details
- **Conflict Prevention Data**: Version conflicts, circular dependencies, and resolution strategies
- **Health Assessment**: Lock file validation status, performance metrics, and health scores
- **Context-Aware Recommendations**: Intelligent suggestions based on lock file analysis

### 2. Consistent Dependency Versions Across Agents

The system ensures consistent dependency versions across multiple AI agents:

- **Version Consistency Checks**: Automatic detection of version conflicts across scopes
- **Multi-Agent Coordination**: Shared dependency version cache and validation
- **Conflict Prevention**: Proactive detection and resolution of version conflicts
- **Cross-Scope Validation**: Validation of dependencies across all project scopes

### 3. Conflict Prevention in AI Workflows

Comprehensive conflict prevention mechanisms:

- **Pre-Execution Validation**: Lock file validation before AI operations
- **Conflict Detection**: Automatic detection of version conflicts and circular dependencies
- **Resolution Strategies**: Multiple conflict resolution modes (automatic, manual, prompt, skip, fail)
- **Rollback Capabilities**: Automatic rollback on conflict resolution failures

### 4. Lock File Validation in AI Operations

Built-in validation for all AI operations:

- **Health Scoring**: Numerical health score (0-100) based on multiple factors
- **Validation Status**: Current validation state and any issues
- **Performance Metrics**: Generation time, cache hit rates, and efficiency data
- **Issue Tracking**: Detailed list of problems and warnings

### 5. AI-Assisted Conflict Resolution

Intelligent conflict resolution capabilities:

- **Automated Resolution**: AI-powered conflict resolution suggestions
- **Manual Override**: Support for manual conflict resolution when needed
- **Rollback Support**: Automatic rollback capabilities for failed resolutions
- **Impact Analysis**: Detailed analysis of resolution impact and risks

## Architecture

### Enhanced AI Service Structure

```rust
pub struct AIService {
    config: AIServiceConfig,
    _cache: Arc<TimedCache<String, AIResponse>>,
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    metrics: Arc<RwLock<AIServiceMetrics>>,
    client: reqwest::Client,
    // Lock file awareness components
    lock_file_integration: Option<Arc<LockFileAIIntegration>>,
    context_injector: Option<Arc<EnhancedContextInjector>>,
    lock_file_cache: Arc<RwLock<Option<LockFileAIContext>>>,
    dependency_version_cache: Arc<RwLock<HashMap<String, String>>>,
}
```

### Configuration Options

```rust
pub struct AIServiceConfig {
    // ... existing fields ...
    
    // Lock file awareness configuration
    pub enable_lock_file_awareness: bool,
    pub lock_file_path: Option<PathBuf>,
    pub auto_validate_lock_file: bool,
    pub conflict_prevention_enabled: bool,
    pub dependency_version_consistency: bool,
}
```

### Enhanced Request Structure

```rust
pub struct AIRequest {
    // ... existing fields ...
    
    // Lock file context
    pub lock_file_context: Option<LockFileRequestContext>,
    pub task_type: Option<TaskType>,
    pub scope_path: Option<String>,
}

pub struct LockFileRequestContext {
    pub include_dependency_versions: bool,
    pub include_conflict_prevention: bool,
    pub include_health_info: bool,
    pub include_recommendations: bool,
    pub target_scopes: Option<Vec<String>>,
    pub include_transitive_deps: bool,
    pub validate_before_processing: bool,
    pub conflict_resolution_mode: ConflictResolutionMode,
}
```

### Enhanced Response Structure

```rust
pub struct AIResponse {
    // ... existing fields ...
    
    // Lock file awareness
    pub lock_file_validation: Option<LockFileValidationResult>,
    pub dependency_consistency_check: Option<DependencyConsistencyResult>,
    pub conflict_analysis: Option<ConflictAnalysisResult>,
    pub recommendations: Option<Vec<AIRecommendation>>,
}
```

## Usage Examples

### Basic AI Request with Lock File Context

```rust
use rhema_ai::ai_service::{AIService, AIServiceConfig, AIRequest, LockFileRequestContext, ConflictResolutionMode};
use rhema_ai::context_injection::TaskType;

// Configure AI service with lock file awareness
let config = AIServiceConfig {
    // ... basic configuration ...
    enable_lock_file_awareness: true,
    lock_file_path: Some(PathBuf::from(".")),
    auto_validate_lock_file: true,
    conflict_prevention_enabled: true,
    dependency_version_consistency: true,
};

let ai_service = Arc::new(AIService::new(config).await?);

// Create request with lock file context
let lock_context = LockFileRequestContext {
    include_dependency_versions: true,
    include_conflict_prevention: true,
    include_health_info: true,
    include_recommendations: true,
    target_scopes: Some(vec!["crates/rhema-core".to_string()]),
    include_transitive_deps: true,
    validate_before_processing: true,
    conflict_resolution_mode: ConflictResolutionMode::Automatic,
};

let request = AIRequest {
    id: uuid::Uuid::new_v4().to_string(),
    prompt: "Analyze the current dependency structure and provide recommendations.".to_string(),
    model: "gpt-4".to_string(),
    temperature: 0.3,
    max_tokens: 1000,
    user_id: Some("example-user".to_string()),
    session_id: Some("example-session".to_string()),
    created_at: chrono::Utc::now(),
    lock_file_context: Some(lock_context),
    task_type: Some(TaskType::DependencyUpdate),
    scope_path: Some("crates/rhema-core".to_string()),
};

let response = ai_service.process_request(request).await?;

// Access lock file awareness results
if let Some(validation) = &response.lock_file_validation {
    println!("Lock file validation: {} (score: {:.1}/100)", 
        if validation.is_valid { "PASSED" } else { "FAILED" }, 
        validation.validation_score);
}

if let Some(consistency) = &response.dependency_consistency_check {
    println!("Dependency consistency: {}", 
        if consistency.is_consistent { "CONSISTENT" } else { "INCONSISTENT" });
}

if let Some(recommendations) = &response.recommendations {
    for rec in recommendations {
        println!("Recommendation: {} ({:?} priority)", rec.title, rec.priority);
    }
}
```

### Consistent Dependency Versions Across Agents

```rust
// Simulate multiple AI agents working on the same project
let agents = vec!["agent-1", "agent-2", "agent-3"];

for agent_id in agents {
    let lock_context = LockFileRequestContext {
        include_dependency_versions: true,
        include_conflict_prevention: true,
        include_health_info: false,
        include_recommendations: false,
        target_scopes: None,
        include_transitive_deps: true,
        validate_before_processing: true,
        conflict_resolution_mode: ConflictResolutionMode::Automatic,
    };

    let request = AIRequest {
        // ... request configuration ...
        lock_file_context: Some(lock_context),
        task_type: Some(TaskType::DependencyUpdate),
        scope_path: None,
    };

    let response = ai_service.process_request(request).await?;

    if let Some(consistency) = &response.dependency_consistency_check {
        if consistency.is_consistent {
            println!("Agent {}: Dependencies are consistent", agent_id);
        } else {
            println!("Agent {}: Found {} version conflicts", 
                agent_id, consistency.version_conflicts.len());
        }
    }
}
```

### Conflict Prevention in AI Workflows

```rust
use rhema_ai::agent::workflow_manager::{AIWorkflowManager, WorkflowConfig};

let workflow_manager = Arc::new(AIWorkflowManager::new(ai_service.clone(), PathBuf::from(".")).await?);

let workflow_config = WorkflowConfig {
    auto_validate: true,
    auto_resolve_conflicts: true,
    require_confirmation: false,
    rollback_on_failure: true,
    max_retry_attempts: 3,
    timeout_seconds: 300,
    include_security_checks: true,
    include_performance_checks: true,
};

// Start a workflow for dependency updates
let workflow_id = workflow_manager.start_workflow(
    "conflict-prevention-agent",
    TaskType::DependencyUpdate,
    Some("crates/rhema-core".to_string()),
    workflow_config.clone(),
).await?;

// Execute the workflow with conflict prevention
let prompt = "Update dependencies while preventing conflicts. Check for version compatibility and resolve any issues automatically.";

let result = workflow_manager.execute_workflow(&workflow_id, prompt, workflow_config).await?;

println!("Workflow completed: {}", if result.success { "SUCCESS" } else { "FAILED" });
println!("Actions taken: {}", result.actions_taken.len());
println!("Recommendations: {}", result.recommendations.len());
```

### Lock File Validation in AI Operations

```rust
// Validate lock file before processing
let validation_result = ai_service.validate_lock_file_consistency().await?;

println!("Lock file validation result:");
println!("  Consistent: {}", validation_result.is_consistent);
println!("  Version conflicts: {}", validation_result.version_conflicts.len());
println!("  Circular dependencies: {}", validation_result.circular_dependencies.len());
println!("  Outdated dependencies: {}", validation_result.outdated_dependencies.len());
println!("  Security concerns: {}", validation_result.security_concerns.len());

if !validation_result.is_consistent {
    println!("⚠️  Lock file has issues that need attention");
    
    // Use AI to generate recommendations for fixing issues
    let lock_context = LockFileRequestContext {
        include_dependency_versions: true,
        include_conflict_prevention: true,
        include_health_info: true,
        include_recommendations: true,
        target_scopes: None,
        include_transitive_deps: true,
        validate_before_processing: false,
        conflict_resolution_mode: ConflictResolutionMode::Manual,
    };

    let request = AIRequest {
        prompt: "The lock file has validation issues. Provide specific steps to fix these problems.".to_string(),
        lock_file_context: Some(lock_context),
        task_type: Some(TaskType::LockFileManagement),
        // ... other fields ...
    };

    let response = ai_service.process_request(request).await?;
    println!("AI recommendations for fixing validation issues:");
    println!("{}", response.content);
}
```

### AI-Assisted Conflict Resolution

```rust
use rhema_ai::agent::workflow_manager::AIAgentTools;

let agent_tools = AIAgentTools::new(workflow_manager.clone()).await;

// Use AI agent tools for conflict resolution
let conflict_result = agent_tools.resolve_dependency_conflicts(
    "conflict-resolution-agent",
    Some("crates/rhema-core".to_string())
).await?;

println!("Conflict resolution workflow: {}", conflict_result.workflow_id);
println!("Conflicts resolved: {}", conflict_result.conflicts_resolved);
println!("Actions taken: {}", conflict_result.actions_taken.len());
println!("Recommendations: {}", conflict_result.recommendations.len());

// Show detailed conflict resolution steps
for action in &conflict_result.actions_taken {
    println!("Action: {} ({})", 
        action.description,
        if action.success { "SUCCESS" } else { "FAILED" });
    
    if let Some(details) = &action.details {
        println!("  Details: {}", details);
    }
}
```

## Workflow Management

### Workflow States

The system tracks workflow states for AI agent operations:

- **Initialized**: Workflow has been created
- **Validating**: Lock file validation in progress
- **Analyzing**: Dependency analysis in progress
- **Resolving**: Conflict resolution in progress
- **Completed**: Workflow completed successfully
- **Failed**: Workflow failed
- **RolledBack**: Workflow was rolled back

### Workflow Actions

The system tracks various types of workflow actions:

- **LockFileValidation**: Lock file validation operations
- **DependencyAnalysis**: Dependency analysis operations
- **ConflictDetection**: Conflict detection operations
- **ConflictResolution**: Conflict resolution operations
- **DependencyUpdate**: Dependency update operations
- **SecurityReview**: Security review operations
- **PerformanceOptimization**: Performance optimization operations
- **Rollback**: Rollback operations

### Agent Sessions

The system manages AI agent sessions with capabilities:

- **DependencyAnalysis**: Ability to analyze dependencies
- **ConflictResolution**: Ability to resolve conflicts
- **SecurityReview**: Ability to perform security reviews
- **PerformanceOptimization**: Ability to optimize performance
- **Validation**: Ability to validate lock files
- **Rollback**: Ability to rollback changes
- **AutomatedFixes**: Ability to apply automated fixes

## Metrics and Monitoring

### Enhanced Metrics

The AI service now includes lock file awareness metrics:

```rust
pub struct AIServiceMetrics {
    // ... existing metrics ...
    
    // Lock file awareness metrics
    pub lock_file_validation_requests: u64,
    pub conflict_resolution_requests: u64,
    pub dependency_consistency_checks: u64,
    pub ai_assisted_resolutions: u64,
    pub validation_failures: u64,
    pub conflict_detections: u64,
}
```

### Workflow Statistics

The workflow manager provides comprehensive statistics:

```rust
pub struct WorkflowStatistics {
    pub total_workflows: usize,
    pub active_workflows: usize,
    pub completed_workflows: usize,
    pub failed_workflows: usize,
    pub active_agents: usize,
    pub total_recommendations: usize,
    pub total_actions: usize,
    pub average_workflow_duration_ms: u64,
}
```

## Conflict Resolution Modes

The system supports multiple conflict resolution modes:

- **Automatic**: Automatically resolve conflicts without user intervention
- **Manual**: Require manual intervention for conflict resolution
- **Prompt**: Prompt user for conflict resolution decisions
- **Skip**: Skip conflict resolution and continue
- **Fail**: Fail the operation if conflicts are detected

## Health Assessment

### Health Score Calculation

The system calculates health scores based on multiple factors:

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

## Best Practices

### For AI Agents

1. **Always enable lock file awareness** for dependency-related operations
2. **Use appropriate conflict resolution modes** based on the operation type
3. **Validate lock files before processing** critical operations
4. **Monitor health scores** and address issues promptly
5. **Use workflow management** for complex operations

### For Developers

1. **Configure lock file awareness** in AI service configuration
2. **Set appropriate validation levels** for different operation types
3. **Monitor metrics** to track lock file health and AI performance
4. **Use workflow management** for complex multi-step operations
5. **Implement proper error handling** for lock file operations

## Error Handling

The system provides comprehensive error handling for lock file operations:

```rust
match ai_service.process_request(request).await {
    Ok(response) => {
        // Process successful response
        if let Some(validation) = &response.lock_file_validation {
            if !validation.is_valid {
                println!("Warning: Lock file validation failed");
            }
        }
    }
    Err(e) => {
        match e {
            RhemaError::ConfigError(msg) => {
                println!("Configuration error: {}", msg);
            }
            RhemaError::ValidationError(msg) => {
                println!("Validation error: {}", msg);
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
- **Batch Processing**: Support for batch operations on multiple scopes

## Security Considerations

- **Input Validation**: All lock file data is validated
- **Checksum Verification**: File integrity is verified
- **Access Control**: Context access can be restricted
- **Audit Logging**: All operations are logged for security
- **Rollback Capabilities**: Automatic rollback on security issues

## Future Enhancements

Planned improvements include:

- **Real-time Monitoring**: Live lock file health monitoring
- **Predictive Analysis**: AI-powered dependency trend analysis
- **Automated Resolution**: Automatic conflict resolution suggestions
- **Integration APIs**: REST APIs for external tool integration
- **Advanced Analytics**: Detailed dependency analytics and reporting
- **Multi-Project Support**: Support for multiple projects and workspaces
- **Collaborative Resolution**: Multi-agent collaborative conflict resolution

## Conclusion

The enhanced AI service with lock file awareness provides AI agents with comprehensive tools to understand and work with lock file constraints. By integrating lock file information into AI operations, the system enables better decision-making for dependency management, conflict resolution, and project health maintenance.

The system provides:

- **Comprehensive lock file context** for AI operations
- **Consistent dependency versions** across multiple agents
- **Proactive conflict prevention** in AI workflows
- **Built-in lock file validation** for all operations
- **AI-assisted conflict resolution** with multiple resolution modes
- **Workflow management** for complex operations
- **Comprehensive metrics and monitoring** for system health

This enhancement ensures that AI agents can work effectively with lock file constraints while maintaining project stability and consistency. 