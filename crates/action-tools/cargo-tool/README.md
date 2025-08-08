# Cargo Tool

A comprehensive Cargo validation and transformation tool for the Rhema Action Protocol. This tool provides extensive Rust project management capabilities including compilation checking, testing, linting, formatting, and dependency analysis.

## Features

### Validation Commands
- **`check`**: Compile without producing executables (fastest)
- **`build`**: Full compilation with executable generation
- **`test`**: Run all tests in the project
- **`clippy`**: Run Clippy linter for additional checks
- **`audit`**: Security vulnerability scanning
- **`outdated`**: Check for outdated dependencies

### Transformation Commands
- **`fmt`**: Code formatting using rustfmt
- **`clippy --fix`**: Auto-fix Clippy suggestions

### Advanced Features
- **JSON Output Parsing**: Structured error and warning reporting
- **Parallel Execution**: Run commands in parallel for better performance
- **Configurable Output**: Toggle between JSON and human-readable output
- **Verbose Logging**: Detailed operation logging
- **Error Categorization**: Separate error and warning handling
- **File Location Tracking**: Precise error location reporting
- **Workspace Support**: Multi-crate workspace handling with flexible execution modes

## Usage

### Basic Validation

```rust
use rhema_action_tool::{ActionIntent, ActionType, SafetyLevel};
use rhema_action_cargo::CargoTool;

let intent = ActionIntent {
    id: "basic-check".to_string(),
    action_type: ActionType::Validation,
    scope: vec!["Cargo.toml".to_string()],
    metadata: None,
    safety_level: SafetyLevel::Low,
};

let cargo_tool = CargoTool;
let result = cargo_tool.validate(&intent).await?;
```

### Comprehensive Validation

```rust
use serde_json::json;

let intent = ActionIntent {
    id: "comprehensive".to_string(),
    action_type: ActionType::Validation,
    scope: vec!["Cargo.toml".to_string()],
    metadata: Some(json!({
        "commands": ["check", "clippy", "test", "audit"],
        "json_output": true,
        "verbose": true
    })),
    safety_level: SafetyLevel::Medium,
};
```

### Code Transformation

```rust
let intent = ActionIntent {
    id: "format-code".to_string(),
    action_type: ActionType::Transformation,
    scope: vec!["Cargo.toml".to_string()],
    metadata: Some(json!({
        "commands": ["fmt", "clippy"],
        "json_output": true
    })),
    safety_level: SafetyLevel::Medium,
};

let result = cargo_tool.execute(&intent).await?;
```

### Workspace Support

```rust
// Execute on all workspace members
let mut workspace_intent = ActionIntent::new(
    "workspace-validation",
    ActionType::Test,
    "Validate workspace",
    vec!["Cargo.toml".to_string()],
    SafetyLevel::Medium,
);
workspace_intent.metadata = json!({
    "commands": ["check", "clippy", "test"],
    "workspace_mode": "all_members",
    "json_output": true,
    "verbose": false
});

let result = cargo_tool.validate(&workspace_intent).await?;
```

## Configuration

The tool accepts configuration through the `metadata` field of `ActionIntent`:

### Available Options

- **`commands`**: Array of command strings to execute
  - `"check"` - Cargo check
  - `"build"` - Cargo build
  - `"test"` - Cargo test
  - `"clippy"` - Cargo clippy
  - `"fmt"` - Cargo fmt
  - `"audit"` - Cargo audit
  - `"outdated"` - Cargo outdated

- **`parallel`**: Boolean (default: `true`)
  - Enable parallel execution of commands

- **`json_output`**: Boolean (default: `true`)
  - Use JSON output format for better parsing

- **`verbose`**: Boolean (default: `false`)
  - Enable verbose logging

- **`workspace_mode`**: String (default: `"root_and_members"`)
  - `"root_only"` - Execute on workspace root only
  - `"all_members"` - Execute on all workspace members
  - `"root_and_members"` - Execute on workspace root and all members
  - `"selected_members"` - Execute only on specified members

- **`member_filter`**: Array of strings (optional)
  - List of member names to include when using `selected_members` mode

- **`exclude_members`**: Array of strings (optional)
  - List of member names to exclude from execution

### Default Configuration

```json
{
  "commands": ["check"],
  "parallel": true,
  "json_output": true,
  "verbose": false,
  "workspace_mode": "root_and_members"
}
```

## Output Format

### ToolResult Structure

```rust
pub struct ToolResult {
    pub success: bool,           // Overall success status
    pub changes: Vec<String>,    // Applied changes
    pub output: String,          // General output message
    pub errors: Vec<String>,     // Error messages with locations
    pub warnings: Vec<String>,   // Warning messages
    pub duration: Duration,      // Execution time
}
```

### Error Format

Errors include file locations and line numbers when available:
```
src/main.rs:15: expected `;`, found `}`
```

## Examples

See `examples/enhanced_cargo_example.rs` for comprehensive usage examples.

See `examples/workspace_example.rs` for workspace-specific examples.

## Dependencies

- `async-trait`: Async trait support
- `tokio`: Async runtime
- `tracing`: Logging framework
- `serde_json`: JSON parsing
- `rhema-action-tool`: Core action tool traits

## Safety Levels

- **Low**: Basic validation commands (check, outdated)
- **Medium**: Transformation commands (fmt, clippy)
- **High**: Build and test commands (build, test)

## Error Handling

The tool provides comprehensive error handling:

1. **Tool Execution Errors**: Cargo command failures
2. **Validation Errors**: Invalid configurations or file paths
3. **Parsing Errors**: JSON output parsing failures
4. **Availability Errors**: Cargo not installed or accessible

## Performance Considerations

- **Parallel Execution**: Commands run in parallel by default
- **JSON Output**: Faster parsing than text output
- **Incremental Compilation**: Leverages Cargo's built-in caching
- **Selective Commands**: Only run necessary commands

## Integration

The cargo tool integrates seamlessly with the Rhema Action Protocol:

- **Validation Tool**: For checking code quality and correctness
- **Transformation Tool**: For automatic code improvements
- **Safety Tool**: For security and dependency analysis

## Future Enhancements

Planned improvements include:

- **Cross-compilation**: Target-specific builds
- **Feature Flags**: Conditional compilation support
- **Profile Support**: Debug/release profile handling
- **Metrics Collection**: Compilation time and size tracking
- **Dependency Graph**: Visual dependency analysis
- **Advanced Workspace Features**: Workspace dependency resolution, member ordering 