# Rhema

Rhema is an AI-driven development workflow tool that helps developers manage context, automate tasks, and enhance productivity through intelligent orchestration of development processes.

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/rhema-ai/rhema.git
cd rhema

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install Rhema
cargo install --path .
```

### Basic Usage

```bash
# Initialize Rhema in your project
rhema init

# Start interactive mode
rhema interactive

# Run a specific command
rhema query "find all TODO comments"
```

## 📚 Documentation

The documentation has been converted to use **MkDocs** with a modern, searchable interface.

### Viewing Documentation

- **Online**: Visit the [Rhema Documentation](https://rhema-ai.github.io/rhema/)
- **Local Development**: Run `./scripts/docs.sh serve` to start a local development server

### Documentation Structure

- **Getting Started** - Essential guides for new users
- **User Guide** - Comprehensive feature documentation
- **Reference** - Technical reference materials
- **Development Setup** - Guides for contributors
- **Architecture** - Design decisions and proposals
- **Examples** - Practical use cases and examples

### Contributing to Documentation

1. Install dependencies: `./scripts/docs.sh install`
2. Start development server: `./scripts/docs.sh serve`
3. Make your changes in the `docs/` directory
4. Test locally and commit your changes

For more details, see the [MkDocs Migration Guide](MKDOCS_MIGRATION.md).

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
- Git
- Python 3.8+ (for documentation)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/rhema-ai/rhema.git
cd rhema

# Build the project
cargo build --release

# Run tests
cargo test

# Run integration tests
RHEMA_RUN_INTEGRATION_TESTS=1 cargo test --test integration
```

### Development Setup

See the [Development Setup Guide](docs/development-setup/development.md) for detailed instructions on setting up your development environment.

## 📖 Features

- **Context Management** - Intelligent context injection and management
- **Interactive Mode** - Command-line interface with AI assistance
- **Batch Operations** - Process multiple files and repositories
- **Git Integration** - Seamless Git workflow integration
- **Configuration Management** - Flexible configuration system
- **Performance Monitoring** - Built-in performance tracking and optimization

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

### Code Style

- Follow Rust conventions
- Use meaningful commit messages
- Add documentation for new features
- Include tests for new functionality

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [Rhema Documentation](https://rhema-ai.github.io/rhema/)
- **Issues**: [GitHub Issues](https://github.com/rhema-ai/rhema/issues)
- **Discussions**: [GitHub Discussions](https://github.com/rhema-ai/rhema/discussions)

## 🗺️ Roadmap

See our [Roadmap](ROADMAP.md) for upcoming features and improvements.

## 📊 Status

[![CI](https://github.com/rhema-ai/rhema/workflows/CI/badge.svg)](https://github.com/rhema-ai/rhema/actions)
[![Crates.io](https://img.shields.io/crates/v/rhema)](https://crates.io/crates/rhema)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 