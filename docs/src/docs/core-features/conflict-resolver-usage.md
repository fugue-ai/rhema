# Advanced Conflict Resolution in Rhema

This document describes the advanced conflict resolution strategies implemented in Rhema's lock file system.

## Overview

The conflict resolver provides comprehensive strategies for detecting and resolving dependency conflicts in Rhema projects. It supports multiple resolution approaches and provides clear guidance for users when conflicts occur.

## Features

### 1. Latest Compatible Version Resolution
Automatically selects the latest version that satisfies all conflicting requirements.

```rust
use rhema::lock::{ConflictResolver, ConflictResolutionConfig, ConflictResolutionStrategy};

let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::LatestCompatible,
    ..Default::default()
};

let mut resolver = ConflictResolver::with_config(config);
let result = resolver.resolve_conflicts(&dependencies, &repo_path)?;
```

### 2. Pinned Version Enforcement
Enforces pinned versions when available, ensuring deterministic builds.

```rust
let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::PinnedVersion,
    strict_pinning: true,
    ..Default::default()
};
```

### 3. Manual Conflict Resolution Workflows
Provides interactive workflows for manual conflict resolution.

```rust
let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::ManualResolution,
    allow_user_prompts: true,
    ..Default::default()
};
```

### 4. Automatic Conflict Detection and Reporting
Detects conflicts and generates detailed reports without automatic resolution.

```rust
let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::AutomaticDetection,
    enable_auto_detection: true,
    ..Default::default()
};
```

### 5. Conflict Resolution History Tracking
Tracks resolution history to inform future conflict resolution decisions.

```rust
let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::HistoryTracking,
    track_history: true,
    ..Default::default()
};
```

## Resolution Strategies

### Latest Compatible
- **Purpose**: Automatically select the latest version that satisfies all requirements
- **Use Case**: When you want to stay up-to-date with the latest compatible versions
- **Behavior**: Finds the highest version that satisfies all conflicting constraints

### Pinned Version
- **Purpose**: Enforce specific pinned versions when available
- **Use Case**: When you need deterministic builds or have specific version requirements
- **Behavior**: Uses pinned versions and fails if no pinned version is available

### Manual Resolution
- **Purpose**: Require user intervention for conflict resolution
- **Use Case**: When you want full control over version selection
- **Behavior**: Detects conflicts and prompts user for resolution decisions

### Smart Selection
- **Purpose**: Use compatibility scores to select the best version
- **Use Case**: When you want intelligent version selection based on compatibility
- **Behavior**: Calculates compatibility scores and selects the version with the highest score

### Conservative
- **Purpose**: Prefer stable, well-tested versions
- **Use Case**: When stability is more important than latest features
- **Behavior**: Prefers stable versions over pre-release or development versions

### Aggressive
- **Purpose**: Prefer latest versions with newest features
- **Use Case**: When you want to stay on the cutting edge
- **Behavior**: Selects the latest available version regardless of stability

### Hybrid
- **Purpose**: Try multiple strategies in sequence
- **Use Case**: When you want a fallback approach
- **Behavior**: Attempts multiple strategies until one succeeds

## Configuration Options

```rust
pub struct ConflictResolutionConfig {
    /// Primary resolution strategy
    pub primary_strategy: ConflictResolutionStrategy,
    
    /// Fallback strategies in order of preference
    pub fallback_strategies: Vec<ConflictResolutionStrategy>,
    
    /// Whether to enable automatic conflict detection
    pub enable_auto_detection: bool,
    
    /// Whether to track resolution history
    pub track_history: bool,
    
    /// Maximum resolution attempts
    pub max_attempts: usize,
    
    /// Whether to allow user prompts for manual resolution
    pub allow_user_prompts: bool,
    
    /// Whether to prefer stable versions
    pub prefer_stable: bool,
    
    /// Whether to enforce pinned versions strictly
    pub strict_pinning: bool,
    
    /// Compatibility threshold for smart selection (0.0-1.0)
    pub compatibility_threshold: f64,
    
    /// Whether to enable parallel resolution
    pub parallel_resolution: bool,
    
    /// Maximum parallel resolution threads
    pub max_parallel_threads: usize,
    
    /// Timeout for resolution operations (in seconds)
    pub timeout_seconds: u64,
}
```

## Usage Examples

### Basic Usage
```rust
use rhema::lock::{ConflictResolver, LockSystem};

// Create dependencies
let dependencies = vec![/* your dependencies */];
let repo_path = PathBuf::from("/path/to/repo");

// Resolve conflicts with default configuration
let result = LockSystem::resolve_conflicts(&dependencies, &repo_path, None)?;

if result.successful {
    println!("All conflicts resolved successfully!");
    println!("Resolved {} dependencies", result.resolved_dependencies.len());
} else {
    println!("Some conflicts require manual resolution");
    for warning in &result.warnings {
        println!("Warning: {}", warning);
    }
}
```

### Custom Configuration
```rust
use rhema::lock::{ConflictResolver, ConflictResolutionConfig, ConflictResolutionStrategy};

let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::SmartSelection,
    fallback_strategies: vec![
        ConflictResolutionStrategy::Conservative,
        ConflictResolutionStrategy::ManualResolution,
    ],
    compatibility_threshold: 0.8,
    prefer_stable: true,
    track_history: true,
    ..Default::default()
};

let mut resolver = ConflictResolver::with_config(config);
let result = resolver.resolve_conflicts(&dependencies, &repo_path)?;
```

### Getting Conflict Guidance
```rust
use rhema::lock::LockSystem;

let guidance = LockSystem::get_conflict_guidance(&result.detected_conflicts);
for line in guidance {
    println!("{}", line);
}
```

## Conflict Types

The resolver can detect and handle various types of conflicts:

- **Version Incompatibility**: Conflicting version requirements
- **Circular Dependencies**: Dependency cycles
- **Missing Dependencies**: Required dependencies not found
- **Ambiguous Resolution**: Multiple valid resolution options
- **Security Vulnerabilities**: Known security issues
- **License Incompatibilities**: License conflicts
- **Architecture Incompatibilities**: Platform-specific issues

## Conflict Severity Levels

- **Low**: Can be auto-resolved safely
- **Medium**: Requires user attention
- **High**: May break functionality
- **Critical**: Must be resolved manually

## Performance Considerations

- **Caching**: The resolver maintains caches for compatibility scores and version information
- **Parallel Resolution**: Can be enabled for faster resolution of multiple conflicts
- **Timeout**: Configurable timeout prevents infinite resolution attempts
- **Memory Usage**: History tracking can be disabled to reduce memory usage

## Best Practices

1. **Start with Conservative Strategy**: Use conservative resolution for production environments
2. **Enable History Tracking**: Track resolution history to improve future decisions
3. **Set Appropriate Timeouts**: Configure timeouts based on your dependency complexity
4. **Review Manual Resolutions**: Always review conflicts that require manual resolution
5. **Use Fallback Strategies**: Configure fallback strategies for robust resolution

## Integration with Lock System

The conflict resolver integrates seamlessly with Rhema's lock system:

```rust
use rhema::lock::{LockSystem, ConflictResolutionConfig, ConflictResolutionStrategy};

// Generate lock file with conflict resolution
let config = ConflictResolutionConfig {
    primary_strategy: ConflictResolutionStrategy::LatestCompatible,
    ..Default::default()
};

let lock_file = LockSystem::generate_lock_file(&repo_path)?;
```

## Error Handling

The resolver provides comprehensive error handling and reporting:

- **Detailed Error Messages**: Clear descriptions of what went wrong
- **Resolution Recommendations**: Suggestions for fixing conflicts
- **Performance Metrics**: Timing and resource usage information
- **Conflict Reports**: Detailed reports for each detected conflict

## Future Enhancements

Planned enhancements include:

- **Machine Learning Integration**: Use ML to improve resolution decisions
- **Dependency Graph Visualization**: Visual representation of conflicts
- **Integration with Package Managers**: Direct integration with npm, cargo, etc.
- **Real-time Conflict Monitoring**: Continuous monitoring for new conflicts
- **Team Collaboration Features**: Shared resolution history and decisions 