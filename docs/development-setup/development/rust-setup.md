# Rust Development Setup for GACP

This guide will help you set up a complete Rust development environment for contributing to the GACP CLI. The GACP CLI is written in Rust, so this guide covers everything you need to build, test, and contribute to the project.

## Prerequisites

- [Rust](https://rustup.rs/) toolchain installed
- [Git](https://git-scm.com/) for version control
- A code editor (see [Editor Setup Guides](../editor-setup/))
- Basic familiarity with Rust (see [Rust Book](https://doc.rust-lang.org/book/))

## Installation

### 1. Install Rust Toolchain

Install Rust using rustup:

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows
# Download and run rustup-init.exe from https://rustup.rs/

# Reload your shell
source ~/.bashrc  # or ~/.zshrc, ~/.profile, etc.

# Verify installation
rustc --version
cargo --version
```

### 2. Install Development Tools

Install essential development tools:

```bash
# Install additional components
rustup component add rustfmt
rustup component add clippy
rustup component add rust-analyzer

# Install useful cargo tools
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-tarpaulin  # for code coverage
cargo install cargo-expand     # for macro debugging
cargo install cargo-tree       # for dependency visualization
```

### 3. Clone and Build GACP

```bash
# Clone the repository
git clone https://github.com/fugue-ai/gacp.git
cd gacp

# Build the project
cargo build

# Run tests
cargo test

# Install the CLI locally
cargo install --path .
```

## Development Environment

### 1. IDE/Editor Setup

#### VS Code
- Install [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Install [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) for dependency management
- Install [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) for Cargo.toml support

#### IntelliJ IDEA
- Install [Rust Plugin](https://plugins.jetbrains.com/plugin/8182-rust)
- Configure Rust toolchain in Settings → Languages & Frameworks → Rust

#### Vim/Neovim
- Install [rust-analyzer](https://github.com/rust-lang/rust-analyzer) via COC.nvim or ALE
- Install [vim-rust](https://github.com/rust-lang/rust.vim) for syntax highlighting

### 2. Project Structure

The GACP CLI follows a typical Rust project structure:

```
gacp/
├── Cargo.toml              # Project configuration and dependencies
├── src/
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports and CLI definitions
│   ├── commands/          # CLI command implementations
│   ├── schema/            # Protocol schema definitions
│   ├── query/             # Context query engine (CQL)
│   ├── git/               # Git integration utilities
│   ├── scope/             # Scope discovery and management
│   └── error.rs           # Error types and handling
├── schemas/               # Protocol JSON Schema definitions
├── tests/                 # Test suite
└── docs/                  # Documentation
```

### 3. Cargo Configuration

Create a `.cargo/config.toml` file for project-specific settings:

```toml
[build]
# Enable incremental compilation for faster builds
incremental = true

[profile.dev]
# Optimize for faster compilation
opt-level = 0
debug = true

[profile.release]
# Optimize for performance
opt-level = 3
lto = true
codegen-units = 1

[profile.test]
# Optimize tests for speed
opt-level = 1
debug = true

[profile.bench]
# Optimize benchmarks for accuracy
opt-level = 3
lto = true
codegen-units = 1

[unstable]
# Enable nightly features if needed
build-std = false

[target.'cfg(all())']
# Common target settings
rustflags = [
    "-C", "target-cpu=native",
    "-C", "link-arg=-Wl,-rpath,$ORIGIN"
]
```

## Development Workflow

### 1. Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Build with specific features
cargo build --features "full"

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu
```

### 2. Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests

# Run tests with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### 3. Code Quality

```bash
# Format code
cargo fmt

# Check code style
cargo clippy

# Check for security vulnerabilities
cargo audit

# Check for unused dependencies
cargo udeps

# Run all checks
cargo check && cargo clippy && cargo test
```

### 4. Debugging

#### Using GDB/LLDB

```bash
# Build with debug symbols
cargo build

# Debug with GDB
gdb target/debug/gacp

# Debug with LLDB (macOS)
lldb target/debug/gacp
```

#### Using VS Code

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug GACP CLI",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/gacp",
      "args": ["--help"],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_BACKTRACE": "1"
      }
    },
    {
      "name": "Debug Tests",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/deps/gacp-*",
      "args": [],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_BACKTRACE": "1"
      }
    }
  ]
}
```

#### Using IntelliJ IDEA

1. Go to **Run** → **Edit Configurations**
2. Add **Rust** configuration
3. Set **Executable** to `target/debug/gacp`
4. Set **Program arguments** as needed

## Testing Strategy

### 1. Unit Tests

Unit tests are located alongside the code they test:

```rust
// src/commands/mod.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        // Test implementation
    }
}
```

### 2. Integration Tests

Integration tests are in the `tests/` directory:

```rust
// tests/integration_tests.rs
use gacp::commands::Command;

#[test]
fn test_end_to_end_workflow() {
    // Test complete workflows
}
```

### 3. Property-Based Testing

Use [proptest](https://github.com/AltSysrq/proptest) for property-based testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_query_parsing(input in "[a-zA-Z0-9_]+") {
        // Test that any valid input can be parsed
    }
}
```

### 4. Performance Testing

Use [criterion](https://github.com/bheisler/criterion.rs) for benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_query_parsing(c: &mut Criterion) {
    c.bench_function("parse_query", |b| {
        b.iter(|| parse_query(black_box("todos WHERE status='active'")))
    });
}

criterion_group!(benches, benchmark_query_parsing);
criterion_main!(benches);
```

## Code Style and Standards

### 1. Rustfmt Configuration

Create `rustfmt.toml`:

```toml
# Rustfmt configuration
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### 2. Clippy Configuration

Add to `Cargo.toml`:

```toml
[package.metadata.clippy]
all-targets = true
```

Or create `.clippy.toml`:

```toml
# Clippy configuration
disallowed-methods = ["std::env::set_var"]
```

### 3. Pre-commit Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
# Pre-commit hook for Rust development

echo "Running Rust checks..."

# Format check
if ! cargo fmt -- --check; then
    echo "Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Clippy check
if ! cargo clippy -- -D warnings; then
    echo "Clippy check failed. Fix warnings before committing."
    exit 1
fi

# Test check
if ! cargo test; then
    echo "Tests failed. Fix tests before committing."
    exit 1
fi

echo "All checks passed!"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Performance Optimization

### 1. Profiling

#### Using perf (Linux)

```bash
# Install perf
sudo apt-get install linux-tools-common

# Profile the application
perf record -g target/release/gacp --some-command
perf report
```

#### Using Instruments (macOS)

```bash
# Profile with Instruments
xcrun xctrace record --template "Time Profiler" --launch target/release/gacp --some-command
```

#### Using flamegraph

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph -- some-command
```

### 2. Memory Profiling

```bash
# Install memory profiler
cargo install memory-profiler

# Profile memory usage
cargo memory-profiler -- some-command
```

## Documentation

### 1. Code Documentation

```rust
/// Parse a GACP query string into a structured query object.
///
/// # Arguments
///
/// * `query` - The query string to parse
///
/// # Returns
///
/// A `Result` containing the parsed query or an error.
///
/// # Examples
///
/// ```
/// use gacp::query::parse_query;
///
/// let query = parse_query("todos WHERE status='active'")?;
/// assert_eq!(query.table, "todos");
/// ```
pub fn parse_query(query: &str) -> Result<Query, QueryError> {
    // Implementation
}
```

### 2. Generate Documentation

```bash
# Generate documentation
cargo doc

# Generate and open documentation
cargo doc --open

# Generate documentation for all dependencies
cargo doc --document-private-items
```

### 3. Documentation Tests

```rust
/// Add two numbers together.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Continuous Integration

### 1. GitHub Actions

Create `.github/workflows/rust.yml`:

```yaml
name: Rust CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
      
    - name: Run tests
      run: cargo test --verbose
      
    - name: Run integration tests
      run: cargo test --test '*'
      
    - name: Check for security vulnerabilities
      run: cargo audit
      
    - name: Generate coverage report
      run: cargo tarpaulin --out Html
      
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./target/tarpaulin/tarpaulin-report.html
```

## Troubleshooting

### Common Issues

1. **Build errors**: Check Rust version with `rustc --version`
2. **Test failures**: Run `cargo test --verbose` for detailed output
3. **Performance issues**: Use profiling tools to identify bottlenecks
4. **Memory leaks**: Use memory profilers to track allocations
5. **Dependency conflicts**: Run `cargo tree` to visualize dependencies

### Getting Help

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

## Next Steps

1. **Set up your editor**: See [Editor Setup Guides](../editor-setup/)
2. **Read the codebase**: Start with `src/main.rs` and `src/lib.rs`
3. **Run the tests**: `cargo test` to understand the test suite
4. **Make your first contribution**: See [Contributing Guide](contributing.md)
5. **Join the community**: Participate in discussions and code reviews

For more information, see the [GACP CLI Reference](../README.md#cli-command-reference) and [Protocol Documentation](../schemas/). 