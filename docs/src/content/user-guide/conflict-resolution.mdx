# Conflict Resolution Guide

## Overview

Rhema's advanced conflict resolution system provides comprehensive dependency conflict detection and resolution capabilities. This guide covers how to use the conflict resolution features through the CLI and programmatically.

## CLI Commands

### Basic Conflict Resolution

```bash
# Resolve conflicts in the current lock file
rhema lock resolve-conflicts

# Resolve conflicts in a specific lock file
rhema lock resolve-conflicts --file path/to/rhema.lock

# Show detailed information about conflicts
rhema lock resolve-conflicts --verbose

# Show only conflicts that need manual resolution
rhema lock resolve-conflicts --manual-only
```

### Advanced Configuration

```bash
# Use a specific resolution strategy
rhema lock resolve-conflicts --strategy smart_selection

# Configure fallback strategies
rhema lock resolve-conflicts --fallback-strategies "conservative,manual_resolution"

# Set compatibility threshold for smart selection
rhema lock resolve-conflicts --compatibility-threshold 0.9

# Enable parallel resolution for better performance
rhema lock resolve-conflicts --parallel --max-threads 8

# Prefer stable versions
rhema lock resolve-conflicts --prefer-stable

# Enforce pinned versions strictly
rhema lock resolve-conflicts --strict-pinning
```

### Output and Reporting

```bash
# Generate a detailed report
rhema lock resolve-conflicts --report-file conflict-report.json --format json

# Show performance metrics
rhema lock resolve-conflicts --show-metrics

# Apply resolved changes to lock file
rhema lock resolve-conflicts --apply

# Dry run (don't apply changes)
rhema lock resolve-conflicts --apply=false
```

## Resolution Strategies

### 1. Latest Compatible (Default)
Automatically selects the latest version that satisfies all requirements.

```bash
rhema lock resolve-conflicts --strategy latest_compatible
```

**Best for**: Projects that want the newest features while maintaining compatibility.

### 2. Pinned Version
Enforces pinned versions when available, ensuring deterministic builds.

```bash
rhema lock resolve-conflicts --strategy pinned_version
```

**Best for**: Production environments requiring exact version reproducibility.

### 3. Smart Selection
Uses compatibility scores for intelligent version selection.

```bash
rhema lock resolve-conflicts --strategy smart_selection --compatibility-threshold 0.8
```

**Best for**: Complex dependency trees where compatibility is critical.

### 4. Conservative
Prefers stable, well-tested versions over the latest features.

```bash
rhema lock resolve-conflicts --strategy conservative
```

**Best for**: Production systems where stability is more important than new features.

### 5. Aggressive
Prefers latest versions with newest features.

```bash
rhema lock resolve-conflicts --strategy aggressive
```

**Best for**: Development environments where you want the latest features.

### 6. Manual Resolution
Detects conflicts and provides guidance for manual resolution.

```bash
rhema lock resolve-conflicts --strategy manual_resolution --allow-prompts
```

**Best for**: Complex conflicts requiring human judgment.

### 7. Automatic Detection
Comprehensive conflict detection without auto-resolution.

```bash
rhema lock resolve-conflicts --strategy automatic_detection
```

**Best for**: Understanding dependency conflicts before deciding how to resolve them.

### 8. History Tracking
Uses historical resolution data to inform decisions.

```bash
rhema lock resolve-conflicts --strategy history_tracking
```

**Best for**: Teams with established resolution patterns.

### 9. Hybrid
Combines multiple strategies in sequence.

```bash
rhema lock resolve-conflicts --strategy hybrid
```

**Best for**: Complex scenarios requiring multiple resolution approaches.

## Configuration Options

### Primary Strategy
The main resolution strategy to use:
- `latest_compatible` (default)
- `pinned_version`
- `manual_resolution`
- `automatic_detection`
- `history_tracking`
- `smart_selection`
- `conservative`
- `aggressive`
- `hybrid`

### Fallback Strategies
Comma-separated list of fallback strategies to try if the primary strategy fails.

### Compatibility Threshold
For smart selection strategy, the minimum compatibility score (0.0-1.0) required for a version to be considered.

### Performance Options
- `--parallel`: Enable parallel resolution
- `--max-threads`: Maximum number of parallel threads (default: 4)
- `--timeout`: Timeout for resolution operations in seconds (default: 300)

### Behavior Control
- `--prefer-stable`: Prefer stable versions over latest
- `--strict-pinning`: Enforce pinned versions strictly
- `--allow-prompts`: Allow user prompts for manual resolution
- `--auto-detection`: Enable automatic conflict detection
- `--track-history`: Track resolution history

## Understanding Conflict Types

### Version Incompatibility
Different scopes require incompatible versions of the same dependency.

**Example**: Scope A requires `lib@^1.0.0` while Scope B requires `lib@^2.0.0`

### Circular Dependencies
Dependencies form a cycle, making resolution impossible.

**Example**: A → B → C → A

### Missing Dependencies
A required dependency is not available or cannot be found.

### Ambiguous Resolution
Multiple versions satisfy the constraints, making the choice unclear.

### Security Vulnerabilities
A dependency version has known security issues.

### License Incompatibility
Dependencies have incompatible licenses.

## Conflict Severity Levels

### Low
Can be auto-resolved without significant risk.

### Medium
Requires user attention but can often be resolved automatically.

### High
May break functionality and requires careful consideration.

### Critical
Must be resolved manually and may require architectural changes.

## Best Practices

### 1. Start with Detection
Before resolving conflicts, understand what conflicts exist:

```bash
rhema lock resolve-conflicts --strategy automatic_detection --verbose
```

### 2. Use Appropriate Strategies
- **Development**: Use `aggressive` or `latest_compatible`
- **Staging**: Use `conservative` or `smart_selection`
- **Production**: Use `pinned_version` or `conservative`

### 3. Review Before Applying
Always review the resolution plan before applying changes:

```bash
rhema lock resolve-conflicts --apply=false --verbose
```

### 4. Use Fallback Strategies
Configure multiple fallback strategies for robust resolution:

```bash
rhema lock resolve-conflicts \
  --strategy smart_selection \
  --fallback-strategies "conservative,manual_resolution"
```

### 5. Monitor Performance
Use performance metrics to optimize resolution:

```bash
rhema lock resolve-conflicts --show-metrics
```

### 6. Track Resolution History
Enable history tracking to learn from past resolutions:

```bash
rhema lock resolve-conflicts --track-history
```

## Integration with CI/CD

### Automated Conflict Resolution
```bash
# In CI pipeline
rhema lock resolve-conflicts \
  --strategy conservative \
  --apply \
  --report-file conflict-report.json \
  --format json
```

### Conflict Detection in PRs
```bash
# Check for conflicts without resolving
rhema lock resolve-conflicts \
  --strategy automatic_detection \
  --report-file conflicts.json \
  --format json
```

### Security-Focused Resolution
```bash
# Focus on security vulnerabilities
rhema lock resolve-conflicts \
  --strategy conservative \
  --prefer-stable \
  --strict-pinning
```

## Troubleshooting

### Common Issues

#### Resolution Timeout
If resolution takes too long, try:
```bash
rhema lock resolve-conflicts --timeout 600 --parallel --max-threads 8
```

#### Too Many Conflicts
If there are too many conflicts to resolve automatically:
```bash
rhema lock resolve-conflicts --strategy manual_resolution --manual-only
```

#### Incompatible Versions
For version incompatibilities:
```bash
rhema lock resolve-conflicts --strategy conservative --prefer-stable
```

#### Circular Dependencies
For circular dependencies:
```bash
rhema lock resolve-conflicts --strategy manual_resolution --verbose
```

### Debugging

Enable verbose output for detailed information:
```bash
rhema lock resolve-conflicts --verbose --show-metrics
```

Generate detailed reports for analysis:
```bash
rhema lock resolve-conflicts \
  --report-file debug-report.json \
  --format json \
  --verbose \
  --show-metrics
```

## API Usage

### Programmatic Conflict Resolution

```rust
use rhema::lock::{
    ConflictResolver, ConflictResolutionConfig, ConflictResolutionStrategy,
    DependencySpec, LockSystem
};

// Create configuration
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

// Resolve conflicts
let dependencies = vec![/* your dependencies */];
let repo_path = PathBuf::from("/path/to/repo");

let result = LockSystem::resolve_conflicts(&dependencies, &repo_path, Some(config))?;

if result.successful {
    println!("Resolved {} conflicts", result.stats.auto_resolved);
} else {
    for warning in &result.warnings {
        println!("Warning: {}", warning);
    }
}
```

### Custom Conflict Resolver

```rust
use rhema::lock::ConflictResolver;

let mut resolver = ConflictResolver::with_config(config);
let result = resolver.resolve_conflicts(&dependencies, &repo_path)?;

// Access detailed information
println!("Total conflicts: {}", result.stats.total_conflicts);
println!("Auto-resolved: {}", result.stats.auto_resolved);
println!("Manual resolution required: {}", result.stats.manual_resolution_required);

// Get performance metrics
let metrics = &result.performance_metrics;
println!("Total time: {}ms", metrics.total_time_ms);
println!("Memory usage: {} bytes", metrics.memory_usage_bytes);
```

## Examples

### Example 1: Basic Conflict Resolution
```bash
# Simple conflict resolution
rhema lock resolve-conflicts --apply
```

### Example 2: Conservative Resolution for Production
```bash
# Production-ready conflict resolution
rhema lock resolve-conflicts \
  --strategy conservative \
  --prefer-stable \
  --strict-pinning \
  --apply \
  --report-file production-resolution.json
```

### Example 3: Development with Latest Features
```bash
# Development environment with latest features
rhema lock resolve-conflicts \
  --strategy aggressive \
  --parallel \
  --max-threads 8 \
  --apply
```

### Example 4: Complex Conflict Analysis
```bash
# Detailed conflict analysis
rhema lock resolve-conflicts \
  --strategy automatic_detection \
  --verbose \
  --manual-only \
  --report-file conflict-analysis.json \
  --format json \
  --show-metrics
```

### Example 5: CI/CD Integration
```bash
# Automated resolution in CI
rhema lock resolve-conflicts \
  --strategy smart_selection \
  --compatibility-threshold 0.9 \
  --apply \
  --report-file ci-resolution.json \
  --format json
```

## Conclusion

Rhema's conflict resolution system provides powerful tools for managing dependency conflicts. By understanding the available strategies and best practices, you can effectively resolve conflicts in your projects while maintaining stability and compatibility.

For more advanced usage and customization, refer to the API documentation and examples in the codebase. 