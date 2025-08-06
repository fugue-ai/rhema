# Enhanced Validation Command

The Rhema validation command has been enhanced to include comprehensive lock file validation capabilities. This allows you to validate your project's dependency state against the lock file and ensure consistency across your entire project.

## Overview

The enhanced validation command provides multiple validation modes and comprehensive checks for both schema files and lock file consistency. It helps identify issues that could lead to dependency conflicts, build failures, or inconsistent behavior.

## Command Options

### Basic Validation Options

- `--recursive`: Validate all scopes in the repository recursively
- `--json-schema`: Show JSON schemas for all context file types
- `--migrate`: Migrate schemas to the latest version during validation

### Lock File Validation Options

- `--lock-file`: Include lock file validation in addition to schema validation
- `--lock-only`: Validate only the lock file (skip other validations)
- `--strict`: Treat lock file warnings as errors

## Usage Examples

### Basic Schema Validation

```bash
# Validate current scope only
rhema validate

# Validate all scopes recursively
rhema validate --recursive

# Validate and migrate schemas
rhema validate --migrate
```

### Lock File Validation

```bash
# Validate schemas and lock file
rhema validate --lock-file

# Validate only the lock file
rhema validate --lock-only

# Strict lock file validation (warnings become errors)
rhema validate --lock-only --strict

# Full validation with lock file checks
rhema validate --recursive --lock-file --strict
```

## Lock File Validation Checks

When using lock file validation, the following comprehensive checks are performed:

### 1. Lock File Existence and Structure

**Check**: Verifies that the lock file exists and can be parsed correctly.

**Issues Detected**:
- Missing lock file (`rhema.lock`)
- Invalid YAML syntax
- Corrupted file structure

**Example Output**:
```
🔒 Validating lock file...
  🔍 Checking lock file structure...
  ✅ Lock file structure is valid
```

### 2. Scope Existence Validation

**Check**: Ensures all scopes referenced in the lock file actually exist in the filesystem.

**Issues Detected**:
- Scopes in lock file that don't exist in filesystem
- Missing scope definition files

**Example Output**:
```
  🔍 Checking scope existence...
  ❌ Scope 'missing-scope' in lock file does not exist in filesystem
```

### 3. Dependency Validation

**Check**: Validates all dependencies in the lock file are valid and accessible.

**Issues Detected**:
- Dependencies that don't exist
- Missing dependency scope files
- Dependency type mismatches

**Example Output**:
```
    🔍 Validating dependencies for scope 'my-service'...
  ❌ Dependency 'invalid-dep' in scope 'my-service' does not exist
```

### 4. Checksum Validation

**Check**: Validates that scope and dependency checksums match current file states.

**Issues Detected**:
- Checksum mismatches indicating file changes
- Corrupted or modified files

**Example Output**:
```
  ❌ Scope 'my-service' checksum mismatch: expected abc123, got def456
```

### 5. Circular Dependency Detection

**Check**: Identifies circular dependencies that could cause build or runtime issues.

**Issues Detected**:
- Circular dependency chains
- Self-referential dependencies

**Example Output**:
```
  🔍 Checking for circular dependencies...
  ❌ Circular dependency detected: scope1 -> scope2 -> scope1
```

### 6. Version Constraint Validation

**Check**: Ensures version constraints are satisfied by current dependency versions.

**Issues Detected**:
- Version constraint violations
- Incompatible dependency versions

**Example Output**:
```
  🔍 Validating version constraints...
  ❌ Version constraint '>=2.0.0' not satisfied for dependency 'lib-dep' in scope 'my-service'
```

### 7. Lock File Age Check

**Check**: Warns about stale lock files that may be out of date.

**Issues Detected**:
- Lock files older than 30 days (configurable)

**Example Output**:
```
  🔍 Checking lock file age...
  ⚠️  Lock file is 45 days old (last modified: 2024-01-15 10:30:00)
```

## Validation Output

### Success Case

```
🔍 Validating Rhema context files...
────────────────────────────────────────────────────────────────────────────────
📁 Validating scope: my-service
  ✅ rhema.yaml
  ✅ knowledge.yaml
  ✅ todos.yaml

🔒 Validating lock file...
  🔍 Checking scope existence...
  🔍 Validating dependencies for scope 'my-service'...
  🔍 Checking for circular dependencies...
  🔍 Validating version constraints...
  🔍 Checking lock file age...

────────────────────────────────────────────────────────────────────────────────
📊 Validation Summary:
  📄 Total files: 3
  ✅ Valid files: 3
  ❌ Errors: 0
  🔒 Lock file errors: 0

🎉 All files are valid!
🔒 Lock file validation passed!
```

### Error Case

```
🔍 Validating Rhema context files...
────────────────────────────────────────────────────────────────────────────────
📁 Validating scope: my-service
  ✅ rhema.yaml
  ❌ knowledge.yaml: Invalid YAML syntax at line 5

🔒 Validating lock file...
  🔍 Checking scope existence...
  ❌ Scope 'missing-scope' in lock file does not exist in filesystem
  🔍 Validating dependencies for scope 'my-service'...
  ❌ Dependency 'invalid-dep' in scope 'my-service' does not exist

────────────────────────────────────────────────────────────────────────────────
📊 Validation Summary:
  📄 Total files: 2
  ✅ Valid files: 1
  ❌ Errors: 1
  🔒 Lock file errors: 2

❌ Validation Errors:
  1. knowledge.yaml: Invalid YAML syntax at line 5
  2. Scope 'missing-scope' in lock file does not exist in filesystem
  3. Dependency 'invalid-dep' in scope 'my-service' does not exist
```

## Integration with CI/CD

The enhanced validation command is perfect for integration into CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Validate Rhema Project
  run: |
    rhema validate --recursive --lock-file --strict
```

```bash
# Git hooks example
#!/bin/sh
# pre-commit hook
rhema validate --lock-file --strict || exit 1
```

## Best Practices

1. **Regular Validation**: Run validation regularly during development to catch issues early
2. **CI/CD Integration**: Include validation in your CI/CD pipeline with `--strict` flag
3. **Pre-commit Hooks**: Use validation in git hooks to prevent invalid commits
4. **Lock File Maintenance**: Keep your lock file up to date by running validation after dependency changes

## Troubleshooting

### Common Issues

1. **Missing Lock File**: Run `rhema lock generate` to create a lock file
2. **Checksum Mismatches**: Update the lock file with `rhema lock update`
3. **Circular Dependencies**: Review and refactor your dependency structure
4. **Version Conflicts**: Update dependencies to compatible versions

### Debug Mode

For detailed debugging information, use verbose output:

```bash
rhema --verbose validate --lock-file
```

This will provide additional details about each validation step and help identify the root cause of issues. 