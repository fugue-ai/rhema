# Nx Rust Integration

This document explains how to use Nx with the Rust crates in the Rhema project.

## Overview

All Rust crates in the `crates/` directory now have Nx project configurations that allow you to use Nx for building, testing, and managing Rust projects alongside TypeScript/Node.js projects.

## Available Projects

Nx can now manage the following projects:

### Rust Crates (Applications)
- `rhema` - Main CLI application
- `rhema-cli` - CLI utilities
- `rhema-action` - Action handling

### Rust Crates (Libraries)
- `rhema-core` - Core functionality
- `rhema-query` - Query processing
- `rhema-git` - Git integration
- `rhema-ai` - AI services
- `rhema-mcp` - MCP protocol
- `rhema-config` - Configuration management
- `rhema-monitoring` - Monitoring utilities
- `rhema-integrations` - External integrations
- `rhema-dependency` - Dependency management
- `rhema-locomo` - Performance monitoring
- `rhema-knowledge` - Knowledge management
- `rhema-agent` - Agent functionality

### TypeScript/Node.js Projects
- `docs` - Documentation site
- `language-server` - Language server
- `vscode-extension` - VS Code extension

## Available Targets

Each Rust crate has the following Nx targets:

### Build Targets
- `build` - Build in debug mode
- `build:release` - Build in release mode

### Development Targets
- `check` - Run `cargo check`
- `clippy` - Run `cargo clippy`
- `fmt` - Run `cargo fmt`
- `fmt:check` - Check formatting with `cargo fmt --check`

### Testing Targets
- `test` - Run tests

### Documentation Targets
- `doc` - Generate documentation
- `doc:open` - Generate and open documentation (applications only)

### Execution Targets (Applications only)
- `run` - Run in debug mode
- `run:release` - Run in release mode

### Utility Targets
- `clean` - Clean build artifacts

## Usage Examples

### Individual Crate Operations

```bash
# Build a specific crate
npx nx build rhema
npx nx build rhema-core

# Test a specific crate
npx nx test rhema-cli

# Check formatting
npx nx fmt:check rhema-ai

# Run clippy
npx nx clippy rhema-config

# Run an application
npx nx run rhema
npx nx run rhema-cli
```

### Workspace-wide Operations

Use the npm scripts for common operations across all Rust crates:

```bash
# Build all Rust crates
pnpm run rust:build

# Build all Rust crates in release mode
pnpm run rust:build:release

# Test all Rust crates
pnpm run rust:test

# Check all Rust crates
pnpm run rust:check

# Run clippy on all Rust crates
pnpm run rust:clippy

# Format all Rust crates
pnpm run rust:fmt

# Check formatting on all Rust crates
pnpm run rust:fmt:check

# Clean all Rust crates
pnpm run rust:clean

# Generate documentation for all Rust crates
pnpm run rust:doc
```

### Affected Projects

Nx can automatically determine which projects are affected by changes:

```bash
# Build only affected projects
pnpm run rust:affected:build

# Test only affected projects
pnpm run rust:affected:test

# Check only affected projects
pnpm run rust:affected:check
```

### Parallel Execution

Nx can run operations in parallel for better performance:

```bash
# Build multiple crates in parallel
npx nx run-many --target=build --projects=rhema-core,rhema-git,rhema-ai

# Test multiple crates in parallel
npx nx run-many --target=test --projects=rhema-*,docs
```

## Project Dependencies

Nx automatically handles dependencies between projects. For example:
- Building `rhema` will first build all its dependencies (`rhema-core`, `rhema-git`, etc.)
- Testing `rhema-cli` will ensure all dependencies are built first

## Caching

Nx provides intelligent caching:
- Build results are cached and reused when possible
- Only affected projects are rebuilt when changes are made
- Cache can be cleared with `npx nx reset`

## Project Graph

View the dependency graph between projects:

```bash
# Open the project graph in your browser
npx nx graph
```

## Integration with Existing Workflow

The Nx setup integrates seamlessly with existing workflows:

- `cargo build` still works as before
- `cargo test` still works as before
- Nx provides additional benefits like parallel execution and caching
- You can use either `cargo` commands directly or Nx commands

## Troubleshooting

### Project Not Found
If a project is not found, ensure the `project.json` file exists in the crate directory.

### Build Failures
Check that all dependencies are properly configured in the crate's `Cargo.toml`.

### Cache Issues
Clear the Nx cache with `npx nx reset` if you encounter unexpected behavior.

## Adding New Crates

To add a new Rust crate to Nx:

1. Create the crate in the `crates/` directory
2. Add it to the workspace members in the root `Cargo.toml`
3. Run the generation script: `node scripts/generate-rust-projects.js`
4. The new crate will automatically be available in Nx

## Scripts

The `scripts/generate-rust-projects.js` script automatically generates `project.json` files for all Rust crates. Run it after adding new crates or modifying the workspace structure. 