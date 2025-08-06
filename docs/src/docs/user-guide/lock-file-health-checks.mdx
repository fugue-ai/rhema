# Lock File Health Checks

The Rhema health command now includes comprehensive lock file consistency checks to ensure your project's dependency state is valid and up-to-date.

## Overview

Lock file health checks verify the integrity and consistency of your `rhema.lock` file against the current state of your project. These checks help identify issues that could lead to dependency conflicts, build failures, or inconsistent behavior.

## Health Checks Performed

### 1. Lock File Existence and Validity

**Check**: Verifies that the lock file exists and can be parsed correctly.

**Issues Detected**:
- Missing lock file (`rhema.lock`)
- Invalid YAML syntax
- Corrupted file structure

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 1 lock file issue(s):
    • Lock file does not exist (rhema.lock)
```

### 2. Lock File Checksum Validation

**Check**: Validates the SHA-256 checksum of the lock file to ensure it hasn't been corrupted.

**Issues Detected**:
- Checksum mismatch indicating file corruption
- Invalid checksum format

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 1 lock file issue(s):
    • Lock file checksum is invalid - file may be corrupted
```

### 3. Scope Consistency

**Check**: Ensures that all scopes in the lock file correspond to actual scopes in your project.

**Issues Detected**:
- Locked scopes that no longer exist
- Current scopes that aren't locked
- Missing scope definitions

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 2 lock file issue(s):
    • Locked scope no longer exists: old-service
    • Scope not locked: new-service
```

### 4. Dependency Version Mismatches

**Check**: Compares locked dependency versions with current scope definitions.

**Issues Detected**:
- Scope version mismatches
- Dependency type inconsistencies
- Missing or extra dependencies
- Dependencies that no longer exist

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 2 lock file issue(s):
    • Scope version mismatch for user-service: locked=1.0.0, current=2.0.0
    • Dependency not locked: ../shared-lib in user-service
```

### 5. Lock File Staleness

**Check**: Determines if the lock file is outdated and needs regeneration.

**Issues Detected**:
- Lock file older than 30 days
- Source files modified after lock file generation
- Outdated dependency information

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 1 lock file issue(s):
    • Lock file is stale (45 days old) - consider regenerating
```

### 6. Checksum Validation

**Check**: Validates individual scope and dependency checksums to ensure file integrity.

**Issues Detected**:
- Scope source checksum mismatches
- Dependency checksum mismatches
- File content changes not reflected in lock file

**Example Output**:
```
🔒 Checking lock file health...
  ⚠️  Found 1 lock file issue(s):
    • Scope checksum mismatch for user-service: expected=abc123, current=def456
```

## Running Health Checks

### Check All Scopes and Lock Files

```bash
rhema health
```

### Check Specific Scope

```bash
rhema health --scope crates/user-service
```

## Interpreting Results

### Healthy State

When all checks pass, you'll see:

```
🔒 Checking lock file health...
  ✅ Lock file is healthy

📊 Health Summary:
  📁 Total scopes: 5
  ✅ Healthy scopes: 5
  ⚠️  Total issues: 0

🎉 All scopes and lock files are healthy!
```

### Issues Found

When issues are detected:

```
🔒 Checking lock file health...
  ⚠️  Found 3 lock file issue(s):
    • Lock file is stale (45 days old) - consider regenerating
    • Scope version mismatch for user-service: locked=1.0.0, current=2.0.0
    • Dependency not locked: ../shared-lib in user-service

📊 Health Summary:
  📁 Total scopes: 5
  ✅ Healthy scopes: 3
  ⚠️  Total issues: 5

🔧 Consider running 'rhema validate' for detailed validation
```

## Resolving Issues

### Missing Lock File

If the lock file doesn't exist, generate it:

```bash
rhema lock generate
```

### Stale Lock File

If the lock file is outdated, regenerate it:

```bash
rhema lock regenerate
```

### Version Mismatches

If scope versions don't match:

1. Update the scope version in `rhema.yaml`
2. Regenerate the lock file
3. Or update the lock file to match current versions

### Missing Dependencies

If dependencies are missing from the lock file:

1. Add the dependency to the scope's `rhema.yaml`
2. Regenerate the lock file

### Checksum Mismatches

If checksums don't match:

1. Check if files have been modified
2. Regenerate the lock file to update checksums
3. Verify file integrity

## Best Practices

1. **Regular Health Checks**: Run `rhema health` regularly to catch issues early
2. **CI/CD Integration**: Include health checks in your CI/CD pipeline
3. **Lock File Management**: Keep lock files up-to-date with project changes
4. **Version Control**: Commit lock files to version control for reproducible builds
5. **Documentation**: Document any manual lock file modifications

## Configuration

Health check behavior can be configured through environment variables:

- `RHEMA_LOCK_MAX_AGE_DAYS`: Maximum age for lock files (default: 30)
- `RHEMA_HEALTH_STRICT_MODE`: Enable strict validation (default: false)

## Troubleshooting

### Common Issues

1. **False Positives**: Some checks may report issues that are intentional
2. **Performance**: Large projects may take longer to validate
3. **File Permissions**: Ensure read access to all scope directories

### Debug Mode

For detailed debugging information:

```bash
RHEMA_LOG=debug rhema health
```

This will show additional information about each check performed.

## Integration with Other Commands

The health command integrates with other Rhema commands:

- **Validate**: `rhema validate` provides detailed validation
- **Lock**: `rhema lock` commands for lock file management
- **Sync**: `rhema sync` to synchronize dependencies

## Future Enhancements

Planned improvements to lock file health checks:

- Performance optimizations for large projects
- Custom validation rules
- Integration with external dependency scanners
- Automated issue resolution suggestions
- Historical health trend analysis 