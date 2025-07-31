# Rhema

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://github.com/fugue-ai/rhema/workflows/Rust/badge.svg)](https://github.com/fugue-ai/rhema/actions)
[![Crates.io](https://img.shields.io/crates/v/rhema)](https://crates.io/crates/rhema)

**Transform implicit knowledge into explicit, persistent context that survives across AI conversations and development sessions.**

Rhema (/ˈreɪmə/ "RAY-muh") is a Git-native toolkit that captures, organizes, and shares project knowledge through structured YAML files. It solves the fundamental problem of ephemeral context in AI-assisted development by making implicit knowledge explicit and persistent.



The name Rhema comes from the Greek word ῥῆμα, meaning "utterance" or "that which is spoken." Just as rhema represents the ephemeral nature of spoken knowledge, Rhema captures the ephemeral nature of development knowledge—those crucial insights, decisions, and context that exist in conversations, code reviews, and AI interactions but are often lost when the moment passes. By transforming these transient "utterances" of development wisdom into persistent, structured records, Rhema ensures that valuable knowledge endures beyond the immediate conversation or development session.

## Core Values

Rhema is built on fundamental principles that guide every aspect of its design and functionality. These values shape how we approach knowledge management, team collaboration, and AI integration.

### 🤝 Collaborative Knowledge

**Knowledge is a team asset, not an individual possession.**

Rhema breaks down knowledge silos by making context discoverable and shareable across your entire organization. Every team member can contribute to and benefit from the collective understanding of your codebase.

### 🎯 AI-First, Universal Design

**Context should be optimized for AI consumption, human reading, and traditional machines.**

Rhema structures knowledge in ways that AI agents can effectively query, understand, and act upon. This enables consistent, context-aware AI behavior across all your development tools and conversations. In a tradeoff between human and traditional machine interactions, YAML is used as alternative to the traditional use of Markdown.

### 🎯 Explicit Over Implicit

**Rhema's fundamental value proposition is making implicit knowledge explicit and persistent.**

**Before Rhema:** Knowledge exists in individual minds, temporary chats, and scattered docs, leading to knowledge silos and inconsistent AI behavior.

**After Rhema:** Knowledge is structured, persistent, and discoverable across your entire team and AI interactions.

### 📈 Continuous Learning

**Knowledge should accumulate and improve over time, not degrade.**

Rhema preserves the full history of decisions, learnings, and patterns, allowing teams to build upon past insights rather than repeating the same discoveries. Context becomes a living, growing asset.

### 🔍 Discoverable Truth

**The right information should find you, not require you to find it.**

Rhema's powerful querying capabilities ensure that relevant context surfaces when and where it's needed. Whether through structured queries, full-text search, or AI-driven recommendations, knowledge flows to where it creates the most value.

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