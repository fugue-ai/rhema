# Development Setup

This guide provides comprehensive information for developers contributing to Rhema. It covers environment setup, development workflows, testing, and contribution guidelines.

## 🎯 Overview

Rhema is a Rust-based project with a modular architecture. The codebase is organized into multiple crates, each handling specific functionality. This guide will help you set up a development environment and understand how to contribute effectively.

## 🏗️ Project Structure

### Core Crates
```
crates/
├── cli/                    # Command-line interface
├── core/                   # Core functionality and utilities
├── config/                 # Configuration management
├── git/                    # Git integration
├── ai/                     # AI service integration
├── agent/                  # Agent coordination
├── action/                 # Action protocol implementation
├── knowledge/              # Knowledge management
├── query/                  # Query engine
├── dependency/             # Dependency management
├── monitoring/             # Performance monitoring
├── locomo/                 # Benchmarking integration
├── mcp/                    # Model Context Protocol
├── integrations/           # Third-party integrations
└── rhema/                  # Main library crate
```

### Documentation and Tools
```
docs/                       # Documentation site
editor-plugins/             # Editor integrations
examples/                   # Usage examples
tests/                      # Test suites
schemas/                    # JSON schemas
infra/                      # Infrastructure files
```

## 🚀 Quick Start

### Prerequisites
- **Rust**: Latest stable version (1.70+)
- **Git**: For version control
- **Node.js**: For documentation development (optional)
- **Docker**: For containerized development (optional)

### Initial Setup
```bash
# Clone the repository
git clone https://github.com/fugue-ai/rhema.git
cd rhema

# Install Rust dependencies
cargo build

# Run tests to verify setup
cargo test

# Build the CLI
cargo build --bin rhema
```

## 🔧 Development Environment

### Rust Setup
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Install development tools
rustup component add rustfmt clippy rust-analyzer

# Install additional tools
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-tarpaulin  # For code coverage
```

### IDE Setup
- **VS Code**: Install Rust extension and configure settings
- **IntelliJ IDEA**: Install Rust plugin
- **Vim/Neovim**: Configure rust-analyzer LSP

### Development Tools
```bash
# Install development dependencies
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-tarpaulin

# Install documentation tools
npm install -g @sveltejs/kit
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific crate
cargo test -p rhema-cli

# Run integration tests
cargo test --test integration_tests
```

### Test Coverage
```bash
# Generate coverage report
cargo tarpaulin --out Html

# Generate coverage with specific options
cargo tarpaulin --out Html --skip-clean --ignore-tests
```

### Performance Tests
```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench performance_benchmarks
```

## 🔍 Code Quality

### Linting and Formatting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Security Audits
```bash
# Check for security vulnerabilities
cargo audit

# Update dependencies
cargo update
```

### Documentation
```bash
# Generate documentation
cargo doc

# Generate documentation with private items
cargo doc --document-private-items

# Open documentation in browser
cargo doc --open
```

## 🏗️ Building and Running

### Build Options
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific crate
cargo build -p rhema-cli

# Build with features
cargo build --features "full"
```

### Running the CLI
```bash
# Run from source
cargo run --bin rhema

# Run with specific command
cargo run --bin rhema -- init

# Run with features
cargo run --bin rhema --features "full"
```

### Development Server
```bash
# Start documentation development server
cd docs
npm run dev

# Build documentation
npm run build
```

## 📝 Development Workflow

### Feature Development
1. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Follow Rust coding conventions
   - Add tests for new functionality
   - Update documentation

3. **Test Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

### Bug Fixes
1. **Create Bug Fix Branch**
   ```bash
   git checkout -b fix/issue-description
   ```

2. **Fix the Issue**
   - Add regression tests
   - Ensure existing tests pass

3. **Test the Fix**
   ```bash
   cargo test
   cargo test --test integration_tests
   ```

### Code Review Process
1. **Self Review**
   - Run all tests
   - Check code formatting
   - Review for security issues

2. **Create Pull Request**
   - Provide clear description
   - Include test results
   - Link related issues

3. **Address Review Comments**
   - Make requested changes
   - Update tests if needed
   - Re-run tests

## 🎯 Contribution Guidelines

### Code Style
- Follow Rust coding conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions small and focused

### Testing Requirements
- Unit tests for all new functionality
- Integration tests for complex features
- Performance tests for critical paths
- Documentation tests for public APIs

### Documentation Requirements
- Update relevant documentation
- Add examples for new features
- Update API documentation
- Include usage examples

### Commit Messages
Follow conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build/tooling changes

### Pull Request Guidelines
- Provide clear description of changes
- Include test results
- Link related issues
- Request reviews from appropriate team members
- Address all review comments

## 🔧 Advanced Development

### Debugging
```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin rhema

# Run with specific log level
RUST_LOG=rhema=debug cargo run --bin rhema

# Use rust-gdb for debugging
rust-gdb target/debug/rhema
```

### Profiling
```bash
# Profile with perf
perf record --call-graph=dwarf cargo run --release
perf report

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph
```

### Benchmarking
```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench query_benchmarks

# Compare benchmarks
cargo bench --bench compare_benchmarks
```

### Cross-Platform Development
```bash
# Add target for cross-compilation
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu
```

## 🐳 Containerized Development

### Docker Setup
```bash
# Build development image
docker build -f infra/Dockerfile.dev -t rhema-dev .

# Run development container
docker run -it --rm -v $(pwd):/app rhema-dev

# Run tests in container
docker run -it --rm -v $(pwd):/app rhema-dev cargo test
```

### Docker Compose
```bash
# Start development environment
docker-compose -f infra/docker-compose.yml up -d

# Run commands in container
docker-compose -f infra/docker-compose.yml exec rhema cargo test
```

## 🔍 Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check Rust version
rustc --version
```

#### Test Failures
```bash
# Run tests with more output
cargo test -- --nocapture

# Run specific failing test
cargo test test_name -- --nocapture

# Check test environment
cargo test --test environment_check
```

#### Performance Issues
```bash
# Profile the application
cargo flamegraph

# Check memory usage
cargo install cargo-expand
cargo expand > expanded.rs
```

### Getting Help
- Check existing issues on GitHub
- Review documentation
- Ask questions in discussions
- Join community channels

## 📚 Additional Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Tools
- [rust-analyzer](https://rust-analyzer.github.io/)
- [cargo-watch](https://github.com/watchexec/cargo-watch)
- [cargo-audit](https://github.com/RustSec/cargo-audit)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

### Community
- [Rust Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [Rust Reddit](https://www.reddit.com/r/rust/)

## 🤝 Contributing to Documentation

### Documentation Structure
```
docs/src/docs/
├── getting-started/        # Getting started guides
├── user-guide/            # User documentation
├── core-features/         # Feature documentation
├── examples/              # Usage examples
├── development-setup/     # Development guides
├── architecture/          # Architecture documentation
└── reference/             # API reference
```

### Building Documentation
```bash
# Start development server
cd docs
npm run dev

# Build for production
npm run build

# Check for broken links
npm run check
```

### Documentation Guidelines
- Use clear, concise language
- Include practical examples
- Keep documentation up to date
- Follow consistent formatting
- Add diagrams when helpful

## 📋 Development Checklist

Before submitting a contribution:

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted with `cargo fmt`
- [ ] Code passes clippy checks
- [ ] Documentation is updated
- [ ] Examples are provided
- [ ] Security audit passes
- [ ] Performance impact is considered
- [ ] Cross-platform compatibility is verified
- [ ] Pull request description is clear

## 🎯 Next Steps

1. **Set up your development environment** following this guide
2. **Explore the codebase** to understand the architecture
3. **Pick an issue** from the issue tracker
4. **Start contributing** following the guidelines
5. **Join the community** and help others

For questions or help with development setup, please open an issue or join the community discussions. 