# Action Tools

This directory contains modular tool implementations for the Rhema Action Protocol. Each tool is implemented as a separate crate to promote modularity, maintainability, and extensibility.

## Structure

```
action-tools/
├── traits/                    # Shared traits and interfaces
├── jscodeshift-tool/         # Jscodeshift transformation tool
├── comby-tool/              # Comby transformation tool
├── ast-grep-tool/           # Ast-grep transformation tool
├── prettier-tool/           # Prettier formatting tool
├── eslint-tool/             # ESLint linting tool
├── typescript-tool/         # TypeScript validation tool
├── jest-tool/              # Jest testing tool
├── mocha-tool/             # Mocha testing tool
├── pytest-tool/            # PyTest testing tool
├── cargo-tool/             # Cargo validation tool
├── syntax-validation-tool/ # Syntax validation safety tool
├── type-checking-tool/     # Type checking safety tool
├── test-coverage-tool/     # Test coverage safety tool
└── security-scanning-tool/ # Security scanning safety tool
```

## Tool Categories

### Transformation Tools
Tools that modify code files:
- **jscodeshift-tool**: JavaScript/TypeScript code transformations
- **comby-tool**: Multi-language code transformations
- **ast-grep-tool**: AST-based code analysis and transformation
- **prettier-tool**: Code formatting
- **eslint-tool**: Code linting and auto-fixing

### Validation Tools
Tools that validate code without modifying it:
- **typescript-tool**: TypeScript type checking
- **jest-tool**: JavaScript/TypeScript testing
- **mocha-tool**: JavaScript/TypeScript testing (alternative)
- **pytest-tool**: Python testing
- **cargo-tool**: Rust compilation checking

### Safety Tools
Tools that perform safety checks:
- **syntax-validation-tool**: Syntax validation for multiple languages
- **type-checking-tool**: Type checking (placeholder implementation)
- **test-coverage-tool**: Test coverage analysis (placeholder implementation)
- **security-scanning-tool**: Security vulnerability scanning (placeholder implementation)

## Adding New Tools

To add a new tool:

1. Create a new directory in `action-tools/` with the tool name
2. Create `Cargo.toml` with appropriate dependencies
3. Implement the tool by implementing the appropriate trait from `traits/`
4. Add the tool to the workspace members in the root `Cargo.toml`
5. Register the tool in `crates/rhema-action/src/tools.rs`

## Dependencies

Each tool crate depends on:
- `rhema-action-traits`: For shared interfaces
- `rhema-action`: For core action types and error handling
- `tokio`: For async operations
- `tracing`: For logging

## Usage

Tools are automatically registered and available through the `ToolRegistry` in the main `rhema-action` crate. The registry provides methods to:

- Execute transformation tools
- Run validation tools
- Perform safety checks
- List available tools
- Check tool availability

## Benefits of Modular Design

1. **Separation of Concerns**: Each tool is isolated and can be developed independently
2. **Maintainability**: Easier to maintain and update individual tools
3. **Extensibility**: New tools can be added without modifying existing code
4. **Testing**: Each tool can be tested in isolation
5. **Dependencies**: Tools can have their own specific dependencies
6. **Compilation**: Faster incremental compilation when only one tool changes 