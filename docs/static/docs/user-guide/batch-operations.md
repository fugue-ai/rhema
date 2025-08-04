# Rhema CLI Batch Operations


## Overview


The Rhema CLI provides comprehensive batch operations for bulk processing across multiple scopes. This feature enables efficient management of large repositories with multiple Rhema scopes through automated, parallel processing capabilities.

## Features


### 1. Bulk Context File Operations


- **Validation**: Bulk validate YAML files across multiple scopes

- **Migration**: Batch migrate schema files to latest versions

- **Export**: Mass export context data from multiple scopes

- **Import**: Bulk import context data into multiple scopes

- **Health Checking**: Comprehensive health checks across all scopes

### 2. Batch Command Execution


- **Parallel Processing**: Execute commands across multiple scopes in parallel

- **Command Scripting**: Define command sequences in YAML files

- **Error Handling**: Robust error handling and reporting

- **Progress Tracking**: Real-time progress indicators

### 3. Mass Data Import/Export


- **Multiple Formats**: Support for JSON, YAML, CSV formats

- **Selective Export**: Export specific data types or all data

- **Bulk Import**: Import data into multiple scopes simultaneously

- **Data Validation**: Validate data before import operations

### 4. Bulk Validation and Health Checking


- **Schema Validation**: Validate all schema files across scopes

- **Dependency Checking**: Check cross-scope dependencies

- **Health Assessment**: Comprehensive health status reporting

- **Detailed Reports**: Generate detailed validation reports

### 5. Batch Reporting and Analytics


- **Summary Reports**: Generate summary reports across all scopes

- **Analytics**: Cross-scope analytics and metrics

- **Custom Reports**: Generate custom reports based on specific criteria

- **Multiple Formats**: Export reports in various formats

## Command Structure


```bash
rhema batch <subcommand> [options]
```

### Subcommands


#### Context Operations


```bash
rhema batch context <operation> --input-file <file> [options]
```

**Operations:**

- `validate` - Bulk validate context files

- `migrate` - Batch migrate schema files

- `export` - Mass export context data

- `import` - Bulk import context data

- `health-check` - Comprehensive health checking

**Options:**

- `--input-file <file>` - Input file with operation parameters

- `--scope-filter <filter>` - Filter scopes by pattern

- `--dry-run` - Preview changes without modifying files

#### Command Execution


```bash
rhema batch commands --command-file <file> [options]
```

**Options:**

- `--command-file <file>` - YAML file with batch commands

- `--scope-filter <filter>` - Filter scopes by pattern

- `--parallel` - Execute commands in parallel

- `--max-workers <count>` - Maximum parallel workers (default: 4)

#### Data Operations


```bash
rhema batch data <operation> --input-path <path> --output-path <path> [options]
```

**Operations:**

- `export` - Export data from multiple scopes

- `import` - Import data into multiple scopes

**Options:**

- `--input-path <path>` - Input path for import or base path for export

- `--output-path <path>` - Output path for export or target path for import

- `--format <format>` - Data format (json, yaml, csv)

- `--scope-filter <filter>` - Filter scopes by pattern

#### Validation Operations


```bash
rhema batch validate <type> [options]
```

**Types:**

- `validate` - Comprehensive validation

- `health-check` - Health status checking

- `schema-check` - Schema compliance checking

- `dependency-check` - Dependency validation

**Options:**

- `--scope-filter <filter>` - Filter scopes by pattern

- `--output-file <file>` - Output file for detailed report

- `--detailed` - Include detailed information

#### Reporting Operations


```bash
rhema batch report <type> --output-file <file> [options]
```

**Types:**

- `summary` - Summary reports

- `analytics` - Analytics reports

- `health` - Health reports

- `dependencies` - Dependency reports

- `todos` - Todo reports

- `knowledge` - Knowledge reports

**Options:**

- `--output-file <file>` - Output file for the report

- `--format <format>` - Report format (json, yaml, markdown, html, csv)

- `--scope-filter <filter>` - Filter scopes by pattern

- `--include-details` - Include detailed information

## Configuration Files


### Batch Commands File


Define a sequence of commands to execute across multiple scopes:

```yaml
# batch-commands.yaml


- command: "validate"
  description: "Validate all YAML files in each scope"
  args:
    recursive: true
    json_schema: false
    migrate: false

- command: "health"
  description: "Check health status of each scope"
  args:
    scope: null  # Will be replaced with actual scope path

- command: "query"
  description: "Query todos across all scopes"
  args:
    query: "todos WHERE status='in_progress'"
    format: "json"
```

### Batch Input File


Define parameters for batch operations:

```yaml
# batch-input.yaml


validation:
  recursive: true
  json_schema: false
  migrate: false
  strict_mode: true
  ignore_warnings: false

migration:
  target_version: "1.0.0"
  backup_files: true
  backup_directory: "./backups"
  preserve_comments: true
  update_timestamps: true

export:
  format: "json"
  include_protocol: true
  include_knowledge: true
  include_todos: true
  include_decisions: true
  include_patterns: true
  include_conventions: true
  summarize: true
  ai_agent_format: false
  compress_output: false
  output_directory: "./exports"

scope_filters:
  include_patterns:

    - "**/services/**"

    - "**/apps/**"
  exclude_patterns:

    - "**/tests/**"

    - "**/docs/**"
  scope_types:

    - "service"

    - "application"
  min_scope_version: "0.1.0"

processing:
  parallel_execution: true
  max_workers: 4
  timeout_seconds: 300
  retry_failed: true
  max_retries: 3
  continue_on_error: false
```

## Usage Examples


### 1. Bulk Validation


Validate all scopes in a repository:

```bash
# Validate all scopes


rhema batch context validate --input-file batch-input.yaml

# Validate specific scopes


rhema batch context validate --input-file batch-input.yaml --scope-filter "services/*"

# Dry run validation


rhema batch context validate --input-file batch-input.yaml --dry-run
```

### 2. Batch Command Execution


Execute a sequence of commands across multiple scopes:

```bash
# Execute commands sequentially


rhema batch commands --command-file batch-commands.yaml

# Execute commands in parallel


rhema batch commands --command-file batch-commands.yaml --parallel --max-workers 8

# Execute on specific scopes


rhema batch commands --command-file batch-commands.yaml --scope-filter "apps/*"
```

### 3. Mass Data Export


Export data from all scopes:

```bash
# Export all data


rhema batch data export --input-path "." --output-path "./exports" --format json

# Export specific data types


rhema batch data export --input-path "." --output-path "./exports" --format yaml --scope-filter "services/*"

# Export with custom format


rhema batch data export --input-path "." --output-path "./exports" --format csv
```

### 4. Bulk Health Checking


Perform comprehensive health checks:

```bash
# Health check all scopes


rhema batch validate health-check --detailed

# Generate detailed report


rhema batch validate health-check --output-file health-report.json --detailed

# Check specific validation type


rhema batch validate schema-check --scope-filter "services/*"
```

### 5. Batch Reporting


Generate comprehensive reports:

```bash
# Generate summary report


rhema batch report summary --output-file summary-report.json --format json

# Generate analytics report


rhema batch report analytics --output-file analytics-report.md --format markdown --include-details

# Generate health report


rhema batch report health --output-file health-report.html --format html
```

## Error Handling


### Error Types


1. **Validation Errors**: Schema validation failures

2. **File System Errors**: File not found, permission denied

3. **Processing Errors**: Command execution failures

4. **Data Errors**: Data format or integrity issues

### Error Reporting


Batch operations provide detailed error reporting:

- **Individual Item Results**: Each processed item includes success/failure status

- **Error Messages**: Detailed error messages for failed operations

- **Warning Messages**: Non-critical issues that don't stop processing

- **Statistics**: Summary of successful vs failed operations

### Error Recovery


- **Continue on Error**: Option to continue processing even if some items fail

- **Retry Logic**: Automatic retry of failed operations

- **Partial Results**: Return partial results even with failures

- **Dry Run Mode**: Preview operations without making changes

## Performance Considerations


### Parallel Processing


- **Worker Pool**: Configurable number of parallel workers

- **Resource Management**: Automatic resource allocation and cleanup

- **Progress Tracking**: Real-time progress indicators

- **Memory Management**: Efficient memory usage for large operations

### Optimization Strategies


1. **Scope Filtering**: Use scope filters to limit processing scope

2. **Parallel Execution**: Enable parallel processing for I/O bound operations

3. **Batch Size**: Configure appropriate batch sizes for memory-intensive operations

4. **Caching**: Leverage caching for repeated operations

### Monitoring


- **Progress Bars**: Visual progress indicators

- **Timing Information**: Processing time statistics

- **Resource Usage**: Memory and CPU usage monitoring

- **Performance Metrics**: Throughput and efficiency metrics

## Best Practices


### 1. Planning Batch Operations


- **Start Small**: Test with a subset of scopes first

- **Use Dry Run**: Preview operations before execution

- **Backup Data**: Always backup before destructive operations

- **Document Commands**: Keep command files in version control

### 2. Error Handling


- **Monitor Progress**: Watch for errors during execution

- **Review Reports**: Analyze detailed reports for issues

- **Retry Strategy**: Configure appropriate retry policies

- **Fallback Plans**: Have fallback strategies for critical operations

### 3. Performance Optimization


- **Parallel Processing**: Use parallel execution for I/O bound operations

- **Scope Filtering**: Limit scope to necessary directories

- **Resource Limits**: Set appropriate resource limits

- **Monitoring**: Monitor system resources during execution

### 4. Data Management


- **Validation**: Always validate data before import

- **Backup**: Backup existing data before import operations

- **Incremental Operations**: Use incremental operations for large datasets

- **Data Integrity**: Verify data integrity after operations

## Integration with CI/CD


### GitHub Actions Example


```yaml
name: Rhema Batch Operations

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  batch-validation:
    runs-on: ubuntu-latest
    steps:

      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install Rhema
        run: cargo install rhema
        
      - name: Run Batch Validation
        run: |
          rhema batch context validate \
            --input-file .github/batch-input.yaml \
            --scope-filter "services/*"
            
      - name: Generate Health Report
        run: |
          rhema batch report health \
            --output-file health-report.json \
            --format json
            
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: health-report
          path: health-report.json
```

### GitLab CI Example


```yaml
stages:

  - validate

  - report

batch-validation:
  stage: validate
  script:

    - cargo install rhema

    - rhema batch context validate --input-file .gitlab/batch-input.yaml

    - rhema batch validate health-check --output-file health-report.json
  artifacts:
    reports:
      junit: health-report.json
```

## Troubleshooting


### Common Issues


1. **Permission Errors**

   - Ensure proper file permissions

   - Check directory access rights

   - Verify user permissions

2. **Memory Issues**

   - Reduce parallel workers

   - Use scope filtering

   - Process in smaller batches

3. **Timeout Issues**

   - Increase timeout values

   - Optimize scope filtering

   - Use parallel processing

4. **Data Corruption**

   - Validate data before operations

   - Use backup and restore procedures

   - Check data integrity

### Debugging


- **Verbose Output**: Use verbose mode for detailed logging

- **Dry Run**: Test operations with dry run mode

- **Scope Isolation**: Test with single scope first

- **Log Analysis**: Review detailed logs for issues

## Future Enhancements


### Planned Features


1. **Advanced Filtering**: More sophisticated scope filtering options

2. **Custom Commands**: Support for custom command definitions

3. **Web Interface**: Web-based batch operation management

4. **Scheduling**: Scheduled batch operations

5. **Notifications**: Email/Slack notifications for batch operations

6. **Advanced Analytics**: More sophisticated analytics and reporting

7. **Integration APIs**: REST APIs for batch operations

8. **Plugin System**: Extensible plugin system for custom operations

### Performance Improvements


1. **Distributed Processing**: Support for distributed processing across multiple machines

2. **Streaming Operations**: Streaming support for large datasets

3. **Caching Layer**: Advanced caching for repeated operations

4. **Compression**: Built-in compression for large exports

5. **Incremental Processing**: Support for incremental operations

## Conclusion


The Rhema CLI batch operations provide a powerful and flexible way to manage large repositories with multiple scopes. By following best practices and using the appropriate configuration, you can efficiently perform bulk operations while maintaining data integrity and system performance.

For more information, see the main Rhema documentation and examples in the `examples/` directory. 