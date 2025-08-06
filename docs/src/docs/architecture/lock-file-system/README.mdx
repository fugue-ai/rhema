# Rhema Lock File System

The Rhema Lock File System provides deterministic dependency resolution, improved AI agent coordination, and enhanced development workflows through a comprehensive lock file mechanism.

## Overview

The lock file system ensures consistent dependency resolution across scopes, enabling reproducible builds and reliable AI agent coordination. It provides:

- **Deterministic dependency resolution** across all scopes
- **Cross-scope version consistency** and conflict detection
- **Build reproducibility** through locked dependency versions
- **Performance optimization** through intelligent caching
- **AI agent coordination improvements** with consistent context

## Architecture

### Core Components

The lock file system consists of several key components:

```rust
// Core lock file structure
pub struct RhemaLock {
    pub metadata: LockMetadata,
    pub scopes: HashMap<String, LockedScope>,
    pub dependencies: HashMap<String, LockedDependency>,
    pub checksum: String,
}
```

### Lock File Schema

The lock file uses a standardized JSON schema for consistency:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "metadata": {
      "type": "object",
      "properties": {
        "version": { "type": "string" },
        "generated_at": { "type": "string", "format": "date-time" },
        "generator_version": { "type": "string" }
      }
    },
    "scopes": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/definitions/LockedScope"
      }
    },
    "dependencies": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/definitions/LockedDependency"
      }
    },
    "checksum": { "type": "string" }
  }
}
```

## Implementation Details

### Phase 1: Core Lock File System

#### Lock File Generation Engine

The generation engine analyzes repository scopes and creates deterministic lock files:

```rust
pub struct LockGenerator {
    config: GeneratorConfig,
    resolver: Arc<DependencyResolver>,
    validator: Arc<LockValidator>,
}
```

**Features:**
- Repository scope analysis
- Semantic versioning resolution
- Version conflict detection and handling
- Checksum generation for integrity verification
- Circular dependency detection and prevention
- Configurable resolution strategies

#### Dependency Resolution Engine

Advanced dependency parsing and resolution with multiple strategies:

```rust
pub struct DependencyResolver {
    strategies: Vec<Box<dyn ResolutionStrategy>>,
    cache: Arc<ResolutionCache>,
}
```

**Resolution Strategies:**
- **Semantic**: Follows semantic versioning rules
- **Pinned**: Uses exact version pins
- **Latest**: Always uses the latest compatible version
- **Range-based**: Uses version range constraints

#### Lock File Validation

Comprehensive validation ensures lock file integrity:

```rust
pub struct LockValidator {
    schema_validator: Arc<SchemaValidator>,
    checksum_validator: Arc<ChecksumValidator>,
}
```

**Validation Features:**
- Schema compliance validation
- Checksum verification
- Circular dependency detection
- Version constraint validation
- Scope existence verification
- Lock file freshness and consistency checks

### Phase 2: Integration with Existing Systems

#### Enhanced Health Checks

Lock file consistency checks integrated into health command:

```bash
# Check lock file health
rhema health --include-lock-file

# Validate lock file consistency
rhema health --lock-file-only
```

#### Enhanced Dependency Analysis

Lock file data used for accurate dependency impact analysis:

```bash
# Analyze dependencies with lock file data
rhema deps analyze --use-lock-file

# Show dependency conflicts
rhema deps conflicts --lock-file
```

#### CI/CD Integration

Automated lock file management for continuous integration:

```bash
# Validate lock file in CI
rhema lock ci-validate

# Generate lock file in build process
rhema lock ci-generate

# Check cross-environment consistency
rhema lock ci-consistency
```

### Phase 3: Advanced Features

#### Conflict Resolution Strategies

Advanced conflict resolution with multiple strategies:

```rust
pub struct ConflictResolver {
    strategies: Vec<Box<dyn ConflictResolutionStrategy>>,
    history: Arc<ConflictHistory>,
}
```

**Resolution Strategies:**
- Latest compatible version resolution
- Pinned version enforcement
- Manual conflict resolution workflows
- Automatic conflict detection and reporting
- Conflict resolution history tracking

#### Performance Optimization

Intelligent caching and optimization:

```rust
pub struct LockCache {
    memory_cache: Arc<MemoryCache>,
    disk_cache: Arc<DiskCache>,
    ttl_manager: Arc<TTLManager>,
}
```

**Optimization Features:**
- In-memory caching of lock file data
- Disk-based persistent caching
- TTL-based cache invalidation
- Compression for large lock files
- Parallel processing for dependency resolution

## CLI Commands

### Core Lock Commands

```bash
# Generate new lock file
rhema lock generate [OPTIONS]

# Validate existing lock file
rhema lock validate [OPTIONS]

# Update lock file
rhema lock update [OPTIONS]

# Show lock file status
rhema lock status [OPTIONS]

# Show differences from current state
rhema lock diff [OPTIONS]

# Advanced conflict resolution
rhema lock resolve-conflicts [OPTIONS]
```

### CI/CD Commands

```bash
# Automated validation for pipelines
rhema lock ci-validate [OPTIONS]

# Build process integration
rhema lock ci-generate [OPTIONS]

# Cross-environment consistency checks
rhema lock ci-consistency [OPTIONS]

# Automated updates
rhema lock ci-update [OPTIONS]

# Health monitoring
rhema lock ci-health [OPTIONS]
```

### Advanced Commands

```bash
# Analyze lock file performance
rhema lock analyze [OPTIONS]

# Export lock file statistics
rhema lock stats [OPTIONS]

# Clean up lock file cache
rhema lock clean [OPTIONS]

# Repair corrupted lock file
rhema lock repair [OPTIONS]
```

## Configuration

### Lock File Configuration

```toml
[lock]
# Lock file location
file_path = ".rhema/rhema.lock"

# Generation settings
auto_generate = true
validate_on_generate = true

# Resolution settings
resolution_strategy = "semantic"
allow_prereleases = false
max_conflict_resolution_attempts = 3

# Performance settings
enable_caching = true
cache_ttl = 3600
parallel_resolution = true

# Validation settings
strict_validation = true
check_checksums = true
validate_scopes = true
```

### CI/CD Configuration

```toml
[lock.ci]
# CI/CD settings
auto_validate = true
fail_on_conflicts = true
generate_on_build = true

# Consistency checks
check_cross_environment = true
validate_checksums = true
report_conflicts = true

# Performance settings
parallel_validation = true
cache_results = true
```

## Usage Examples

### Basic Usage

```bash
# Generate lock file for current project
rhema lock generate

# Validate existing lock file
rhema lock validate

# Update lock file with latest compatible versions
rhema lock update
```

### Advanced Usage

```bash
# Generate lock file with specific strategy
rhema lock generate --strategy pinned

# Validate with detailed reporting
rhema lock validate --verbose --report

# Update with conflict resolution
rhema lock update --resolve-conflicts --strategy latest

# Show detailed differences
rhema lock diff --format json --include-metadata
```

### CI/CD Integration

```bash
# In CI pipeline
rhema lock ci-validate --fail-fast
rhema lock ci-generate --output .rhema/rhema.lock
rhema lock ci-consistency --environments staging,production

# In build script
rhema lock ci-update --auto-resolve
rhema lock ci-health --monitor
```

## Performance Considerations

### Optimization Features

- **Parallel Processing**: Dependency resolution runs in parallel
- **Intelligent Caching**: Multi-layer caching for performance
- **Incremental Updates**: Only update changed dependencies
- **Compression**: Lock files are compressed for storage efficiency

### Performance Metrics

- **Generation Time**: < 30 seconds for typical projects
- **Validation Time**: < 5 seconds for lock file validation
- **Memory Usage**: Optimized for minimal memory footprint
- **Cache Hit Rate**: > 80% for frequently accessed data

## Troubleshooting

### Common Issues

1. **Lock File Generation Fails**
   - Check for circular dependencies
   - Verify scope configurations
   - Review dependency constraints

2. **Validation Errors**
   - Check lock file integrity
   - Verify checksums
   - Review scope existence

3. **Performance Issues**
   - Enable caching
   - Use parallel processing
   - Optimize dependency resolution

### Debug Commands

```bash
# Debug lock file generation
rhema lock generate --debug --verbose

# Debug validation issues
rhema lock validate --debug --trace

# Debug performance issues
rhema lock analyze --performance --detailed
```

## Related Documentation

- **[Lock File Schema](./schema.md)** - Detailed schema documentation
- **[Dependency Resolution](./dependency-resolution.md)** - Resolution strategies and algorithms
- **[Conflict Resolution](./conflict-resolution.md)** - Conflict detection and resolution
- **[CI/CD Integration](./ci-cd.md)** - Continuous integration workflows
- **[Performance Tuning](./performance.md)** - Optimization and tuning guide 