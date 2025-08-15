# Type Checking Tool

A comprehensive type checking safety tool for the Rhema Action Protocol that supports multiple programming languages and provides detailed type validation.

## Overview

The Type Checking Tool is a safety tool that performs static type analysis on code files before they are modified by AI agents. It helps ensure that proposed changes maintain type safety and don't introduce type-related errors.

## Features

- **Multi-language Support**: Supports TypeScript, JavaScript, Python, Rust, Go, Java, Kotlin, and Swift
- **Automatic Language Detection**: Automatically detects file types based on extensions
- **Comprehensive Error Reporting**: Provides detailed error messages and warnings
- **Tool Availability Checking**: Verifies that required type checkers are installed
- **Efficient Processing**: Groups files by language for optimized processing
- **Graceful Degradation**: Handles missing files and unsupported languages gracefully

## Supported Languages

| Language | File Extensions | Required Tool | Command |
|----------|----------------|---------------|---------|
| TypeScript | `.ts`, `.tsx` | TypeScript Compiler | `npx tsc --noEmit` |
| JavaScript | `.js`, `.jsx` | Node.js | `node --check` |
| Python | `.py` | MyPy | `mypy --ignore-missing-imports` |
| Rust | `.rs` | Rust Compiler | `rustc --emit=metadata` |
| Go | `.go` | Go Compiler | `go build` |
| Java | `.java` | Java Compiler | `javac -Xlint:all` |
| Kotlin | `.kt` | Kotlin Compiler | `kotlinc -no-stdlib` |
| Swift | `.swift` | Swift Compiler | `swiftc -typecheck` |

## Installation

The tool automatically checks for required type checkers at runtime. Make sure you have the appropriate tools installed for the languages you want to check:

### TypeScript
```bash
npm install -g typescript
# or
npm install typescript
```

### Python
```bash
pip install mypy
```

### Rust
```bash
# Rust is typically installed via rustup
rustup install stable
```

### Go
```bash
# Download from https://golang.org/dl/
```

### Java
```bash
# Install OpenJDK or Oracle JDK
```

### Kotlin
```bash
# Download from https://kotlinlang.org/docs/command-line.html
```

### Swift
```bash
# Install Xcode Command Line Tools (macOS)
xcode-select --install
```

## Usage

The Type Checking Tool is designed to be used as part of the Rhema Action Protocol safety pipeline. It automatically runs when included in the safety checks configuration.

### Example Action Intent

```yaml
rhema:
  version: "1.0.0"
  intent:
    id: "refactor-auth-001"
    action_type: "refactor"
    description: "Extract authentication logic into separate module"
    scope: 
      - "src/auth/types.ts"
      - "src/auth/utils.ts"
      - "src/auth/index.ts"
    safety_level: "medium"
    safety_checks:
      pre_execution:
        - "type_checking"
```

### Programmatic Usage

```rust
use rhema_action_type_checking::TypeCheckingTool;
use rhema_action_tool::{ActionIntent, ActionType, SafetyLevel};

#[tokio::main]
async fn main() {
    let tool = TypeCheckingTool;
    
    let intent = ActionIntent::new(
        "test-001",
        ActionType::Refactor,
        "Test refactoring",
        vec!["src/test.ts".to_string()],
        SafetyLevel::Medium,
    );
    
    let result = tool.check(&intent).await.unwrap();
    
    if result.success {
        println!("Type checking passed: {}", result.output);
    } else {
        println!("Type checking failed:");
        for error in result.errors {
            println!("  - {}", error);
        }
    }
}
```

## Configuration

The tool can be configured through the action intent's metadata or through environment variables:

### Environment Variables

- `RHEMA_TYPE_CHECK_TIMEOUT`: Maximum time (in seconds) for type checking operations (default: 30)
- `RHEMA_TYPE_CHECK_STRICT`: Enable strict mode for all type checkers (default: false)
- `RHEMA_TYPE_CHECK_IGNORE_MISSING`: Ignore missing type checker tools (default: false)

### Metadata Configuration

```yaml
metadata:
  type_checking:
    timeout: 60
    strict_mode: true
    ignore_missing_tools: false
    language_specific:
      typescript:
        strict: true
        noImplicitAny: true
      python:
        ignore_missing_imports: true
        strict_optional: true
```

## Error Handling

The tool provides comprehensive error handling:

1. **Missing Tools**: If a required type checker is not installed, the tool reports this as an error
2. **File Not Found**: Missing files are reported as warnings and skipped
3. **Type Errors**: Actual type errors are reported with file paths and line numbers
4. **Unsupported Languages**: Files with unsupported extensions are reported as warnings

### Error Types

- **ToolExecution**: When a type checker fails to run or returns an error
- **Validation**: When the input is invalid (e.g., no files specified)
- **ToolUnavailable**: When a required type checker is not installed

## Performance

The tool is optimized for performance:

- **Parallel Processing**: Files are grouped by language and processed efficiently
- **Early Termination**: Stops processing if critical errors are found
- **Caching**: Tool availability is checked once per session
- **Timeout Protection**: Prevents hanging on large files or slow type checkers

## Testing

Run the test suite:

```bash
cargo test
```

Run specific tests:

```bash
cargo test test_language_detection
cargo test test_file_grouping
```

## Integration

The Type Checking Tool integrates seamlessly with other Rhema tools:

- **Syntax Validation**: Runs before type checking to catch syntax errors first
- **Test Coverage**: Can be combined with test coverage tools for comprehensive validation
- **Security Scanning**: Works alongside security tools for complete safety validation

## Contributing

To add support for a new language:

1. Add the language detection logic in `detect_language()`
2. Implement the language-specific checking method
3. Add availability checking
4. Add tests for the new language
5. Update this documentation

## License

Apache 2.0 - see LICENSE file for details.
