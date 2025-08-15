# Rhema Core API Documentation

This document provides comprehensive API documentation for the Rhema Core crate, including all modules, functions, and usage examples.

## Table of Contents

1. [Overview](#overview)
2. [Validation Module](#validation-module)
3. [Cache Module](#cache-module)
4. [Audit Module](#audit-module)
5. [Async File Operations](#async-file-operations)
6. [File Operations](#file-operations)
7. [Error Handling](#error-handling)
8. [Best Practices](#best-practices)
9. [Performance Considerations](#performance-considerations)
10. [Security Guidelines](#security-guidelines)

## Overview

The Rhema Core crate provides the foundational building blocks for the Rhema Protocol, including data structures, file operations, validation, caching, and audit logging. It serves as the base upon which all other Rhema crates are built.

### Key Features

- **Data Validation**: Comprehensive input validation for security and data integrity
- **Caching**: High-performance caching with TTL and eviction policies
- **Audit Logging**: Complete audit trail for security and compliance
- **Async Operations**: Non-blocking file operations for better performance
- **Security**: Path validation, input sanitization, and security monitoring

## Validation Module

The validation module provides comprehensive input validation for all Rhema data structures and operations.

### ValidationRules

```rust
use rhema_core::validation::ValidationRules;

// Validate file paths for security
ValidationRules::validate_file_path(path)?;

// Validate user input
ValidationRules::validate_title("My Todo")?;
ValidationRules::validate_id("todo-123")?;
ValidationRules::validate_priority(3)?;
ValidationRules::validate_email("user@example.com")?;
ValidationRules::validate_url("https://example.com")?;
```

### Available Validation Functions

| Function | Purpose | Example |
|----------|---------|---------|
| `validate_file_path` | Security validation for file paths | `validate_file_path(Path::new("data/todos.yaml"))?` |
| `validate_title` | Validate todo/decision titles | `validate_title("Implement authentication")?` |
| `validate_id` | Validate ID format | `validate_id("todo-123")?` |
| `validate_priority` | Validate priority values (1-5) | `validate_priority(3)?` |
| `validate_date_string` | Validate ISO 8601 dates | `validate_date_string("2023-12-01T10:00:00Z")?` |
| `validate_url` | Validate URL format | `validate_url("https://example.com")?` |
| `validate_email` | Validate email format | `validate_email("user@example.com")?` |
| `validate_scope_path` | Validate scope directory paths | `validate_scope_path(Path::new("project/"))?` |

### Validation Rules

- **Titles**: 1-200 characters, no dangerous characters (`<`, `>`, `&`, etc.)
- **IDs**: 1-100 characters, alphanumeric, hyphens, underscores only
- **Priorities**: Integer values 1-5
- **File Paths**: No path traversal (`..`), no absolute paths, no double slashes
- **URLs**: Must start with `http://` or `https://`
- **Emails**: Standard email format validation

## Cache Module

The cache module provides high-performance caching with automatic expiration and eviction policies.

### Synchronous Cache

```rust
use rhema_core::cache::Cache;
use std::time::Duration;

// Create a cache with 1000 entries and 5-minute TTL
let cache = Cache::new(1000, Some(Duration::from_secs(300)));

// Store and retrieve values
cache.set("key".to_string(), "value".to_string())?;
let value = cache.get(&"key".to_string());

// Set with custom TTL
cache.set_with_ttl("temp_key".to_string(), "temp_value".to_string(), Duration::from_secs(60))?;

// Cache statistics
println!("Cache size: {}", cache.len());
println!("Cleaned entries: {}", cache.cleanup());
```

### Asynchronous Cache

```rust
use rhema_core::cache::AsyncCache;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let cache = AsyncCache::new(1000, Some(Duration::from_secs(300)));
    
    // Async operations
    cache.set("key".to_string(), "value".to_string()).await?;
    let value = cache.get(&"key".to_string()).await;
    
    Ok(())
}
```

### Global Caches

```rust
use rhema_core::cache::GlobalCaches;

let caches = GlobalCaches::new();

// File content cache (5-minute TTL)
caches.file_content.set("file.txt".to_string(), content)?;

// YAML data cache (10-minute TTL)
caches.yaml_data.set("data.yaml".to_string(), yaml_value)?;

// Scope data cache (30-minute TTL)
caches.scope_data.set("scope".to_string(), scope)?;
```

### Cache Features

- **Automatic Expiration**: Configurable TTL for all entries
- **LRU Eviction**: Least Recently Used eviction when cache is full
- **Thread Safety**: Safe for concurrent access
- **Memory Management**: Automatic cleanup of expired entries
- **Statistics**: Cache size and cleanup metrics

## Audit Module

The audit module provides comprehensive logging for security, compliance, and debugging.

### Basic Audit Logging

```rust
use rhema_core::audit::{AuditEvent, AuditEventType, AuditSeverity};

// Create an audit event
let event = AuditEvent::new(
    AuditEventType::FileRead,
    AuditSeverity::Info,
    "read_todos".to_string(),
    true,
)
.with_user("user123".to_string())
.with_resource("/todos.yaml".to_string())
.with_detail("size".to_string(), "1024".to_string());

// Log the event
rhema_core::audit::log_audit_event(event)?;
```

### File Operation Logging

```rust
use rhema_core::audit::log_file_operation;

// Log file operations automatically
log_file_operation("read", path, true, None)?;
log_file_operation("write", path, false, Some("Permission denied".to_string()))?;
```

### Security Violation Logging

```rust
use rhema_core::audit::log_security_violation;
use std::collections::HashMap;

let mut details = HashMap::new();
details.insert("attempted_path".to_string(), "/etc/passwd".to_string());
details.insert("user_ip".to_string(), "192.168.1.1".to_string());

log_security_violation(
    "path_traversal_attempt",
    details,
    Some("Path traversal detected".to_string())
)?;
```

### Audit Logger Configuration

```rust
use rhema_core::audit::{AuditLogger, AuditLoggerConfig};

let config = AuditLoggerConfig {
    enabled: true,
    log_to_file: true,
    log_to_stdout: false,
    log_file_path: Some("/var/log/rhema_audit.log".to_string()),
    max_file_size: 10 * 1024 * 1024, // 10MB
    retention_days: 90,
    include_sensitive_data: false,
};

let logger = AuditLogger::new(config)?;
```

### Audit Event Types

| Event Type | Description | Severity |
|------------|-------------|----------|
| `FileRead` | File read operations | Info |
| `FileWrite` | File write operations | Info |
| `FileDelete` | File deletion operations | Warning |
| `FileCreate` | File creation operations | Info |
| `ScopeAccess` | Scope access operations | Info |
| `ScopeModify` | Scope modification operations | Warning |
| `GitOperation` | Git operations | Info |
| `ValidationError` | Input validation failures | Warning |
| `SecurityViolation` | Security violations | Security |
| `ConfigurationChange` | Configuration changes | Warning |
| `UserAction` | User-initiated actions | Info |
| `SystemEvent` | System events | Info |

## Async File Operations

The async file operations module provides high-performance, non-blocking file operations with caching and validation.

### Basic Async Operations

```rust
use rhema_core::async_file_ops::{AsyncFileOps, read_file_async, write_file_async};

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    
    // Read file with caching
    let content = file_ops.read_file(Path::new("data.txt")).await?;
    
    // Write file
    file_ops.write_file(Path::new("output.txt"), b"Hello, World!").await?;
    
    // YAML operations
    let todos: Todos = file_ops.read_yaml_file(Path::new("todos.yaml")).await?;
    file_ops.write_yaml_file(Path::new("todos.yaml"), &todos).await?;
    
    Ok(())
}
```

### Global Async Functions

```rust
use rhema_core::async_file_ops::*;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Global convenience functions
    let content = read_file_async(Path::new("data.txt")).await?;
    write_file_async(Path::new("output.txt"), b"content").await?;
    
    let todos: Todos = read_yaml_file_async(Path::new("todos.yaml")).await?;
    write_yaml_file_async(Path::new("todos.yaml"), &todos).await?;
    
    Ok(())
}
```

### Advanced Operations

```rust
// File operations with metadata
let metadata = file_ops.get_file_metadata(path).await?;
let exists = file_ops.file_exists(path).await?;

// Copy and move operations
file_ops.copy_file(source, destination).await?;
file_ops.move_file(source, destination).await?;

// Directory operations
file_ops.create_dir(path).await?;
let entries = file_ops.read_dir(path).await?;
file_ops.remove_dir(path).await?;

// Cache management
let (file_cache_size, yaml_cache_size) = file_ops.get_cache_stats().await;
file_ops.clear_caches().await;
```

### Performance Features

- **Automatic Caching**: File content and YAML data are cached automatically
- **Async I/O**: Non-blocking file operations using tokio
- **Batch Operations**: Efficient handling of multiple files
- **Memory Optimization**: Configurable cache sizes and TTL
- **Concurrent Access**: Thread-safe operations

## File Operations

The traditional file operations module provides synchronous file operations with validation and audit logging.

### Basic Operations

```rust
use rhema_core::file_ops::{read_yaml_file, write_yaml_file};

// Read YAML file
let todos: Todos = read_yaml_file(Path::new("todos.yaml"))?;

// Write YAML file
write_yaml_file(Path::new("todos.yaml"), &todos)?;
```

### Todo Operations

```rust
use rhema_core::file_ops::{add_todo, list_todos, update_todo, delete_todo};

// Add a new todo
let todo_id = add_todo(
    scope_path,
    "Implement authentication".to_string(),
    Some("Add JWT-based authentication".to_string()),
    Priority::High,
    Some("developer".to_string()),
    Some("2023-12-31T23:59:59Z".to_string()),
)?;

// List todos with filtering
let todos = list_todos(scope_path, Some(TodoStatus::Pending), Some(Priority::High))?;

// Update todo
update_todo(scope_path, &todo_id, "title".to_string(), "Updated title".to_string())?;

// Delete todo
delete_todo(scope_path, &todo_id)?;
```

### Decision Operations

```rust
use rhema_core::file_ops::{add_decision, list_decisions, update_decision, delete_decision};

// Add a decision
let decision_id = add_decision(
    scope_path,
    "Use JWT for authentication".to_string(),
    "Implement JWT-based authentication".to_string(),
    "Better scalability and performance".to_string(),
    vec!["Session-based".to_string()],
    DecisionStatus::Accepted,
)?;

// List decisions
let decisions = list_decisions(scope_path, Some(DecisionStatus::Accepted))?;
```

## Error Handling

The Rhema Core crate provides comprehensive error handling with detailed error types and context.

### Error Types

```rust
use rhema_core::RhemaError;

match result {
    Ok(data) => {
        // Process data
    },
    Err(RhemaError::FileNotFound(path)) => {
        println!("File not found: {}", path);
    },
    Err(RhemaError::ValidationError(message)) => {
        println!("Validation error: {}", message);
    },
    Err(RhemaError::SecurityError(message)) => {
        println!("Security violation: {}", message);
    },
    Err(RhemaError::InvalidYaml { file, message }) => {
        println!("Invalid YAML in {}: {}", file, message);
    },
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

### Common Error Types

| Error Type | Description | Common Causes |
|------------|-------------|---------------|
| `FileNotFound` | File or directory not found | Incorrect path, missing files |
| `ValidationError` | Input validation failed | Invalid data format, security violations |
| `SecurityError` | Security violation detected | Path traversal, unauthorized access |
| `InvalidYaml` | YAML parsing failed | Malformed YAML, encoding issues |
| `IoError` | System I/O error | Permission denied, disk full |
| `ConfigError` | Configuration error | Invalid settings, missing config |

### Error Recovery

```rust
use rhema_core::RhemaResult;

fn robust_file_operation(path: &Path) -> RhemaResult<()> {
    match read_yaml_file::<Todos>(path) {
        Ok(todos) => {
            // Process todos
            Ok(())
        },
        Err(RhemaError::FileNotFound(_)) => {
            // Create empty todos file
            let empty_todos = Todos { todos: vec![], custom: HashMap::new() };
            write_yaml_file(path, &empty_todos)
        },
        Err(RhemaError::InvalidYaml { .. }) => {
            // Backup corrupted file and create new one
            backup_corrupted_file(path)?;
            let empty_todos = Todos { todos: vec![], custom: HashMap::new() };
            write_yaml_file(path, &empty_todos)
        },
        Err(e) => Err(e)
    }
}
```

## Best Practices

### Security

1. **Always validate input**: Use validation functions for all user input
2. **Validate file paths**: Prevent path traversal attacks
3. **Log security events**: Monitor for suspicious activity
4. **Use secure defaults**: Configure secure settings by default

```rust
// Good: Validate all input
let title = ValidationRules::validate_title(user_input)?;
let path = ValidationRules::validate_file_path(user_path)?;

// Bad: No validation
let title = user_input; // Could contain malicious content
```

### Performance

1. **Use async operations**: For I/O-intensive operations
2. **Leverage caching**: Cache frequently accessed data
3. **Batch operations**: Group related operations
4. **Monitor cache performance**: Track cache hit rates

```rust
// Good: Use async operations with caching
let todos: Todos = read_yaml_file_async(path).await?;

// Good: Batch operations
let mut todos = Vec::new();
for path in todo_paths {
    let todo: Todos = read_yaml_file_async(path).await?;
    todos.push(todo);
}
```

### Error Handling

1. **Use specific error types**: Handle different error cases appropriately
2. **Provide context**: Include relevant information in error messages
3. **Log errors**: Use audit logging for error tracking
4. **Graceful degradation**: Provide fallback behavior when possible

```rust
// Good: Specific error handling
match operation() {
    Ok(result) => Ok(result),
    Err(RhemaError::FileNotFound(path)) => {
        // Create missing file
        create_default_file(path)
    },
    Err(RhemaError::ValidationError(msg)) => {
        // Log validation error and return user-friendly message
        log_validation_error("field", "value", &msg)?;
        Err(RhemaError::ValidationError("Invalid input provided".to_string()))
    },
    Err(e) => Err(e)
}
```

### Audit Logging

1. **Log all operations**: Track all file and data operations
2. **Include context**: Add relevant details to audit events
3. **Monitor security events**: Pay attention to security violations
4. **Retain logs**: Keep audit logs for compliance

```rust
// Good: Comprehensive audit logging
let event = AuditEvent::new(
    AuditEventType::FileWrite,
    AuditSeverity::Info,
    "update_todo".to_string(),
    true,
)
.with_user(user_id)
.with_resource(file_path.to_string_lossy().to_string())
.with_detail("todo_id", todo_id)
.with_detail("changes", format!("{:?}", changes));

log_audit_event(event)?;
```

## Performance Considerations

### Caching Strategy

- **File Content Cache**: 5-minute TTL for frequently accessed files
- **YAML Data Cache**: 10-minute TTL for parsed YAML data
- **Scope Data Cache**: 30-minute TTL for scope information
- **Cache Size Limits**: Prevent memory exhaustion

### Async Operations

- **Use async for I/O**: File operations, network requests
- **Batch operations**: Group related async operations
- **Concurrent access**: Leverage async for parallel processing
- **Resource limits**: Control concurrent operation limits

### Memory Management

- **Cache eviction**: Automatic cleanup of expired entries
- **Memory monitoring**: Track cache memory usage
- **Configurable limits**: Set appropriate cache sizes
- **Garbage collection**: Regular cleanup of unused resources

## Security Guidelines

### Input Validation

1. **Validate all inputs**: Never trust user input
2. **Use whitelist approach**: Allow only known good values
3. **Sanitize data**: Remove or escape dangerous characters
4. **Validate file paths**: Prevent path traversal attacks

### Access Control

1. **File permissions**: Set appropriate file permissions
2. **Scope isolation**: Isolate different scopes
3. **User context**: Track user actions
4. **Audit trails**: Log all access attempts

### Security Monitoring

1. **Monitor audit logs**: Review security events regularly
2. **Alert on violations**: Set up alerts for security violations
3. **Track patterns**: Identify suspicious activity patterns
4. **Incident response**: Have procedures for security incidents

### Secure Configuration

1. **Secure defaults**: Configure secure settings by default
2. **Environment variables**: Use environment variables for sensitive config
3. **Configuration validation**: Validate all configuration
4. **Documentation**: Document security requirements

## Troubleshooting

### Common Issues

1. **File not found errors**: Check file paths and permissions
2. **Validation errors**: Review input data format
3. **Cache performance**: Monitor cache hit rates and sizes
4. **Audit log size**: Configure log rotation and retention

### Debugging

1. **Enable debug logging**: Set appropriate log levels
2. **Check audit logs**: Review recent audit events
3. **Validate configuration**: Ensure all settings are correct
4. **Test with sample data**: Use known good data for testing

### Performance Tuning

1. **Cache tuning**: Adjust cache sizes and TTL values
2. **Async operations**: Use async for I/O-intensive operations
3. **Batch processing**: Group related operations
4. **Resource limits**: Set appropriate concurrency limits

## Examples

### Complete Todo Management

```rust
use rhema_core::*;
use std::path::Path;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let scope_path = Path::new("project");
    
    // Validate scope path
    ValidationRules::validate_scope_path(scope_path)?;
    
    // Add todo with validation
    let title = "Implement user authentication";
    ValidationRules::validate_title(title)?;
    
    let todo_id = file_ops::add_todo(
        scope_path,
        title.to_string(),
        Some("Add JWT-based authentication".to_string()),
        Priority::High,
        Some("developer".to_string()),
        Some("2023-12-31T23:59:59Z".to_string()),
    )?;
    
    // Read todos with caching
    let todos: Todos = read_yaml_file_async(&scope_path.join("todos.yaml")).await?;
    
    // Update todo
    file_ops::update_todo(scope_path, &todo_id, "status".to_string(), "in_progress".to_string())?;
    
    // Log completion
    let event = AuditEvent::new(
        AuditEventType::UserAction,
        AuditSeverity::Info,
        "todo_updated".to_string(),
        true,
    )
    .with_detail("todo_id", todo_id)
    .with_detail("new_status", "in_progress".to_string());
    
    log_audit_event(event)?;
    
    Ok(())
}
```

### Secure File Processing

```rust
use rhema_core::*;
use std::path::Path;

async fn process_files_safely(file_paths: Vec<&Path>) -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    
    for path in file_paths {
        // Validate path for security
        ValidationRules::validate_file_path(path)?;
        
        // Check if file exists
        if !file_ops.file_exists(path).await? {
            continue;
        }
        
        // Read file with caching and audit logging
        match file_ops.read_yaml_file::<Todos>(path).await {
            Ok(todos) => {
                // Process todos
                process_todos(todos).await?;
            },
            Err(RhemaError::InvalidYaml { file, message }) => {
                // Log validation error
                log_validation_error("yaml_file", &file, &message)?;
                
                // Log security event
                let mut details = HashMap::new();
                details.insert("file_path".to_string(), file);
                details.insert("error".to_string(), message);
                
                log_security_violation("invalid_yaml_file", details, None)?;
            },
            Err(e) => return Err(e)
        }
    }
    
    Ok(())
}
```

This documentation provides a comprehensive guide to using the Rhema Core crate effectively and securely. For more specific examples and advanced usage patterns, refer to the test files and examples in the source code.
