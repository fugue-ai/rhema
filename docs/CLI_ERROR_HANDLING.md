# CLI Error Handling Standards

This document outlines the standardized error handling approach for the Rhema CLI to ensure consistent, user-friendly error messages across all commands.

## Overview

The Rhema CLI uses a centralized error handling system that provides:
- Consistent error formatting and styling
- Appropriate error severity classification
- Helpful context and suggestions for resolution
- Support for verbose and quiet modes
- Color-coded output when appropriate

## Error Severity Levels

### Fatal Errors (Exit Code: 1)
Errors that prevent the CLI from functioning and require immediate attention:
- `GitRepoNotFound` - No Git repository found
- `ConfigError` - Configuration issues
- `AuthenticationError` - Authentication failures
- `AuthorizationError` - Permission issues
- `SafetyViolation` - Security violations

### Errors (Exit Code: 1)
Operation failures that may be recoverable:
- `FileNotFound` - Missing files
- `ScopeNotFound` - Invalid scope references
- `Validation` - Data validation failures
- `InvalidQuery` - Query syntax errors
- `NetworkError` - Connection issues

### Warnings (Exit Code: 0)
Issues that don't prevent success but should be noted:
- `CircularDependency` - Dependency cycles
- `ValidationError` - Non-critical validation issues
- `PerformanceError` - Performance concerns

### Info (Exit Code: 0)
Informational messages:
- General status updates
- Non-critical notifications

## Error Message Format

### Standard Format
```
[ICON] [SEVERITY]: [DESCRIPTION]
```

### Examples
```
❌ Error: File not found: config.yaml
💥 Fatal Error: Git repository not found: /path/to/repo
⚠️  Warning: Circular dependency detected in scope 'api'
ℹ️  Info: Configuration loaded successfully
```

### Verbose Mode
When `--verbose` is enabled, additional context is provided:
```
❌ Error: File not found: config.yaml
📁 File path: /path/to/config.yaml
💡 Verify the file exists and you have read permissions
```

## Implementation Guidelines

### 1. Use the Centralized Error Handler

```rust
use crate::error_handler::{ErrorHandler, display_error_and_exit};

// In your command function
pub fn my_command(args: &MyArgs, verbose: bool, quiet: bool) -> RhemaResult<()> {
    let handler = ErrorHandler::new(verbose, quiet);
    
    match some_operation() {
        Ok(result) => {
            handler.display_info("Operation completed successfully")?;
            Ok(result)
        }
        Err(e) => {
            handler.display_error(&e)?;
            Err(e)
        }
    }
}
```

### 2. Handle Errors at the Top Level

```rust
#[tokio::main]
async fn main() -> RhemaResult<()> {
    let cli = Cli::parse();
    
    let rhema = match Rhema::new() {
        Ok(rhema) => rhema,
        Err(e) => display_error_and_exit(&e, cli.verbose, cli.quiet),
    };
    
    // ... rest of main function
}
```

### 3. Provide Contextual Error Messages

When creating custom errors, include helpful context:

```rust
// Good
RhemaError::ConfigError("Missing required field 'api_key' in config.yaml".to_string())

// Better
RhemaError::ConfigError(
    "Missing required field 'api_key' in config.yaml. Please add your API key to continue.".to_string()
)
```

### 4. Use Appropriate Error Types

Choose the most specific error type for your situation:

```rust
// For file operations
RhemaError::FileNotFound(path.to_string_lossy().to_string())

// For validation
RhemaError::Validation("Invalid email format".to_string())

// For configuration
RhemaError::ConfigError("Invalid configuration format".to_string())
```

## Error Message Best Practices

### 1. Be Specific
- Include the specific resource that failed
- Mention the operation that was attempted
- Provide the exact error condition

### 2. Be Actionable
- Suggest specific steps to resolve the issue
- Reference relevant commands or documentation
- Explain what the user should check

### 3. Be Consistent
- Use consistent terminology across all commands
- Follow the same formatting patterns
- Maintain consistent severity classifications

### 4. Be User-Friendly
- Avoid technical jargon when possible
- Use clear, concise language
- Provide context that helps users understand the issue

## Examples

### Good Error Messages

```
❌ Error: Scope 'frontend' not found in repository
💡 Run 'rhema scopes' to see available scopes

❌ Error: Invalid query syntax: SELECT * FROM todos WHERE status = 'invalid_status'
💡 Review the query syntax and ensure it's valid

💥 Fatal Error: Git repository not found: /path/to/project
💡 Try running this command from a Git repository directory
```

### Poor Error Messages

```
Error: NotFound
Error: Invalid input
Error: Something went wrong
```

## Testing Error Handling

### 1. Test Error Scenarios
- Test with missing files
- Test with invalid configurations
- Test with network failures
- Test with permission issues

### 2. Test Output Formats
- Test with `--verbose` flag
- Test with `--quiet` flag
- Test with color disabled
- Test with different terminal types

### 3. Test Exit Codes
- Verify correct exit codes for different error types
- Ensure warnings don't cause non-zero exit codes
- Test that fatal errors exit immediately

## Integration with Other Tools

### 1. Logging
- Error messages should be logged for debugging
- Include error context in logs
- Maintain correlation between user messages and logs

### 2. CI/CD
- Ensure error messages are parseable by CI systems
- Provide machine-readable output when needed
- Support structured logging formats

### 3. Documentation
- Keep error message documentation up to date
- Include common error scenarios in user guides
- Provide troubleshooting sections

## Future Improvements

### 1. Internationalization
- Support for multiple languages
- Locale-specific error messages
- Cultural considerations in message formatting

### 2. Accessibility
- Support for screen readers
- High contrast mode support
- Alternative text for icons

### 3. Advanced Features
- Error reporting to central systems
- Automatic error recovery suggestions
- Integration with help systems
