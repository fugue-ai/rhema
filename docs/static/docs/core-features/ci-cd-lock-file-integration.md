# CI/CD Lock File Integration Guide

## Overview

This guide provides comprehensive instructions for integrating Rhema lock file validation, generation, and management into your CI/CD pipelines. The lock file system ensures deterministic dependency resolution and reproducible builds across different environments.

## Features

### 1. Automated Lock File Validation
- **Strict validation**: Ensures lock file integrity and consistency
- **Configurable thresholds**: Set maximum circular dependencies, age limits, and warning policies
- **Multiple output formats**: JSON, YAML, Text, and JUnit XML reports
- **Exit code control**: Configurable exit codes for pipeline integration

### 2. Lock File Generation
- **Build process integration**: Generate lock files as part of the build process
- **Resolution strategies**: Latest, earliest, pinned, range, and compatible versions
- **Timeout controls**: Prevent infinite resolution loops
- **Circular dependency detection**: Fail builds on circular dependencies

### 3. Lock File Consistency Checks
- **Cross-environment validation**: Ensure consistency across development, staging, and production
- **Git branch comparison**: Compare against reference branches
- **Semantic versioning support**: Allow controlled version drift
- **Version drift limits**: Configurable maximum allowed version differences

### 4. Automated Lock File Updates
- **Scheduled updates**: Automatically update dependencies on schedule
- **Security-focused updates**: Prioritize security-related dependency updates
- **Update strategies**: Auto, manual, and security-only modes
- **Backup creation**: Automatic backup before updates

### 5. Health Monitoring
- **Integrity checks**: Verify lock file checksums and structure
- **Freshness validation**: Ensure lock files are not stale
- **Availability checks**: Verify dependency availability
- **Performance metrics**: Monitor resolution and validation performance

## CLI Commands

### CI/CD Validation Command

```bash
rhema lock ci-validate [OPTIONS]
```

**Options:**
- `--file <FILE>`: Lock file path (default: rhema.lock)
- `--exit-code <EXIT_CODE>`: Exit code for validation failures (default: 1)
- `--max-circular-deps <MAX_CIRCULAR_DEPS>`: Maximum allowed circular dependencies (default: 0)
- `--max-age <HOURS>`: Maximum allowed lock file age in hours
- `--fail-on-warnings`: Fail on warnings
- `--report-file <REPORT_FILE>`: Output validation report to file
- `--format <FORMAT>`: Output format (text, json, yaml, junit)

**Example:**
```bash
rhema lock ci-validate \
  --file rhema.lock \
  --exit-code 1 \
  --max-circular-deps 0 \
  --max-age 24 \
  --fail-on-warnings \
  --report-file validation-report.json \
  --format json
```

### CI/CD Generation Command

```bash
rhema lock ci-generate [OPTIONS]
```

**Options:**
- `--output <FILE>`: Output file path (default: rhema.lock)
- `--strategy <STRATEGY>`: Resolution strategy (latest, earliest, pinned, range, compatible)
- `--fail-on-circular`: Fail if circular dependencies detected
- `--timeout <SECONDS>`: Maximum resolution time in seconds (default: 300)
- `--cache-dir <CACHE_DIR>`: Cache directory for resolution
- `--report-file <REPORT_FILE>`: Output generation report to file
- `--format <FORMAT>`: Output format (text, json, yaml)
- `--exit-code <EXIT_CODE>`: Exit code for generation failures (default: 1)

**Example:**
```bash
rhema lock ci-generate \
  --output rhema.lock \
  --strategy latest \
  --fail-on-circular \
  --timeout 300 \
  --report-file generation-report.json \
  --format json \
  --exit-code 1
```

### CI/CD Consistency Command

```bash
rhema lock ci-consistency [OPTIONS]
```

**Options:**
- `--file <FILE>`: Lock file path (default: rhema.lock)
- `--reference-file <REFERENCE_FILE>`: Reference lock file for comparison
- `--git-branch <BRANCH>`: Git branch to compare against
- `--allow-semver-diffs`: Allow version differences within semantic versioning rules
- `--max-version-drift <DRIFT>`: Maximum allowed version drift (major.minor.patch)
- `--report-file <REPORT_FILE>`: Output consistency report to file
- `--format <FORMAT>`: Output format (text, json, yaml)
- `--exit-code <EXIT_CODE>`: Exit code for consistency failures (default: 1)

**Example:**
```bash
rhema lock ci-consistency \
  --file rhema.lock \
  --git-branch main \
  --allow-semver-diffs \
  --max-version-drift "0.1.0" \
  --report-file consistency-report.json \
  --format json \
  --exit-code 1
```

### CI/CD Update Command

```bash
rhema lock ci-update [OPTIONS]
```

**Options:**
- `--file <FILE>`: Lock file path (default: rhema.lock)
- `--update-strategy <STRATEGY>`: Update strategy (auto, manual, security-only)
- `--strategy <STRATEGY>`: Resolution strategy for updates
- `--security-only`: Update only security-related dependencies
- `--max-updates <MAX_UPDATES>`: Maximum number of dependencies to update
- `--backup`: Create backup before updating
- `--report-file <REPORT_FILE>`: Output update report to file
- `--format <FORMAT>`: Output format (text, json, yaml)
- `--exit-code <EXIT_CODE>`: Exit code for update failures (default: 1)

**Example:**
```bash
rhema lock ci-update \
  --file rhema.lock \
  --update-strategy auto \
  --strategy latest \
  --security-only \
  --max-updates 10 \
  --backup \
  --report-file update-report.json \
  --format json \
  --exit-code 1
```

### CI/CD Health Command

```bash
rhema lock ci-health [OPTIONS]
```

**Options:**
- `--file <FILE>`: Lock file path (default: rhema.lock)
- `--integrity`: Check lock file integrity
- `--freshness`: Check lock file freshness
- `--availability`: Check dependency availability
- `--performance`: Check performance metrics
- `--report-file <REPORT_FILE>`: Output health report to file
- `--format <FORMAT>`: Output format (text, json, yaml)
- `--exit-code <EXIT_CODE>`: Exit code for health failures (default: 1)

**Example:**
```bash
rhema lock ci-health \
  --file rhema.lock \
  --integrity \
  --freshness \
  --availability \
  --performance \
  --report-file health-report.json \
  --format json \
  --exit-code 1
```

## GitHub Actions Integration

### Workflow Configuration

Create a new workflow file `.github/workflows/lock-file-ci.yml`:

```yaml
name: Lock File CI/CD Integration

on:
  push:
    branches: [ main, develop, feature/* ]
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'rhema.lock'
      - '.github/workflows/lock-file-ci.yml'
  pull_request:
    branches: [ main, develop ]
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'rhema.lock'
  workflow_dispatch:
    inputs:
      lock_operation:
        description: 'Lock file operation to perform'
        required: true
        default: 'validate'
        type: choice
        options:
          - validate
          - generate
          - consistency
          - update
          - health

env:
  LOCK_FILE_PATH: rhema.lock
  LOCK_REPORTS_DIR: lock-reports

jobs:
  lock-file-validation:
    name: Lock File Validation
    runs-on: ubuntu-latest
    if: ${{ github.event_name == 'pull_request' || github.event.inputs.lock_operation == 'validate' }}
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust stable
      uses: dtolnay/rust-toolchain@stable
      with:
        toolchain: stable
        
    - name: Build Rhema CLI
      run: cargo build --release
      
    - name: Install Rhema CLI
      run: cargo install --path .
      
    - name: Create reports directory
      run: mkdir -p ${{ env.LOCK_REPORTS_DIR }}
      
    - name: Validate lock file
      run: |
        rhema lock ci-validate \
          --file ${{ env.LOCK_FILE_PATH }} \
          --exit-code 1 \
          --max-circular-deps 0 \
          --max-age 24 \
          --fail-on-warnings \
          --report-file ${{ env.LOCK_REPORTS_DIR }}/validation-report.json \
          --format json
      
    - name: Upload validation report
      uses: actions/upload-artifact@v4
      if: always()
      with:
        name: lock-validation-report
        path: ${{ env.LOCK_REPORTS_DIR }}/validation-report.json
        retention-days: 30
```

### Manual Trigger

You can manually trigger lock file operations using the GitHub Actions UI:

1. Go to your repository's Actions tab
2. Select "Lock File CI/CD Integration"
3. Click "Run workflow"
4. Choose the operation and environment
5. Click "Run workflow"

## GitLab CI Integration

### Pipeline Configuration

Include the lock file configuration in your `.gitlab-ci.yml`:

```yaml
include: '.gitlab-ci-lock-file.yml'

variables:
  LOCK_FILE_PATH: "rhema.lock"
  LOCK_REPORTS_DIR: "lock-reports"

stages:
  - test
  - build
  - deploy

# Your existing jobs...
```

### Manual Pipeline Trigger

Trigger lock file operations manually:

```bash
gitlab-ci-trigger --variables LOCK_OPERATION=validate
```

## Jenkins Integration

### Pipeline Configuration

Use the provided `Jenkinsfile.lock-file` or include it in your existing pipeline:

```groovy
// Include lock file stages in your existing pipeline
stage('Lock File Validation') {
    when {
        anyOf {
            expression { params.LOCK_OPERATION == 'validate' }
            expression { env.BRANCH_NAME == 'main' || env.BRANCH_NAME == 'develop' }
        }
    }
    steps {
        script {
            def validationCommand = """
                rhema lock ci-validate \\
                    --file ${LOCK_FILE_PATH} \\
                    --exit-code 1 \\
                    --max-circular-deps ${params.MAX_CIRCULAR_DEPS} \\
                    --max-age 24 \\
                    --fail-on-warnings \\
                    --report-file ${LOCK_REPORTS_DIR}/validation-report.json \\
                    --format json
            """
            
            sh validationCommand
        }
    }
}
```

### Parameterized Build

Configure Jenkins with parameters:

- `LOCK_OPERATION`: Choice parameter (validate, generate, consistency, update, health)
- `ENVIRONMENT`: Choice parameter (development, staging, production)
- `FAIL_ON_WARNINGS`: Boolean parameter
- `MAX_CIRCULAR_DEPS`: String parameter

## Configuration Best Practices

### 1. Environment-Specific Settings

**Development:**
```bash
rhema lock ci-validate \
  --max-circular-deps 1 \
  --max-age 48 \
  --fail-on-warnings false
```

**Staging:**
```bash
rhema lock ci-validate \
  --max-circular-deps 0 \
  --max-age 24 \
  --fail-on-warnings true
```

**Production:**
```bash
rhema lock ci-validate \
  --max-circular-deps 0 \
  --max-age 12 \
  --fail-on-warnings true
```

### 2. Scheduled Updates

Configure scheduled updates for security patches:

```yaml
# GitHub Actions
on:
  schedule:
    - cron: '0 2 * * 1'  # Every Monday at 2 AM

# GitLab CI
lock-file-update:
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"

# Jenkins
stage('Lock File Update') {
    when {
        expression { env.BUILD_CAUSE == 'TIMERTRIGGER' }
    }
}
```

### 3. Report Integration

Integrate reports with your monitoring systems:

```bash
# Upload to monitoring system
curl -X POST \
  -H "Content-Type: application/json" \
  -d @validation-report.json \
  https://your-monitoring-system.com/api/reports

# Send notifications
if [ $? -ne 0 ]; then
    curl -X POST \
      -H "Content-Type: application/json" \
      -d '{"text":"Lock file validation failed"}' \
      https://hooks.slack.com/services/YOUR_WEBHOOK
fi
```

## Troubleshooting

### Common Issues

1. **Lock file not found**
   ```bash
   # Ensure lock file exists
   rhema lock generate --output rhema.lock
   ```

2. **Circular dependencies detected**
   ```bash
   # Increase threshold for development
   rhema lock ci-validate --max-circular-deps 1
   ```

3. **Lock file too old**
   ```bash
   # Regenerate lock file
   rhema lock ci-generate --output rhema.lock
   ```

4. **Validation timeout**
   ```bash
   # Increase timeout
   rhema lock ci-generate --timeout 600
   ```

### Debug Mode

Enable debug output for troubleshooting:

```bash
export RUST_LOG=debug
rhema lock ci-validate --verbose
```

## Security Considerations

### 1. Access Control

- Restrict lock file operations to authorized users
- Use environment-specific credentials
- Implement audit logging for all operations

### 2. Dependency Scanning

- Integrate with security scanning tools
- Automatically update vulnerable dependencies
- Block deployments with known vulnerabilities

### 3. Checksum Verification

- Always verify lock file checksums
- Use secure hash algorithms (SHA-256)
- Implement integrity checks in all environments

## Performance Optimization

### 1. Caching

- Cache lock file resolution results
- Use shared cache directories
- Implement incremental updates

### 2. Parallel Processing

- Run validation and generation in parallel
- Use multiple workers for large projects
- Implement concurrent dependency resolution

### 3. Resource Limits

- Set appropriate timeouts
- Limit memory usage
- Monitor resource consumption

## Monitoring and Alerting

### 1. Metrics Collection

- Track validation success rates
- Monitor resolution times
- Collect performance metrics

### 2. Alerting Rules

- Alert on validation failures
- Notify on security updates
- Warn about stale lock files

### 3. Dashboard Integration

- Create lock file health dashboards
- Display dependency graphs
- Show update history

## Conclusion

The CI/CD lock file integration provides a robust foundation for managing dependencies across your development pipeline. By following these guidelines and best practices, you can ensure consistent, secure, and reliable dependency management in your projects.

For additional support and advanced configurations, refer to the main Rhema documentation and community resources. 