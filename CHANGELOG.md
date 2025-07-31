# Changelog

All notable changes to Rhema CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release setup with GitHub Actions CI/CD
- Comprehensive test suite with unit, integration, and performance tests
- Security audit integration
- Multi-platform binary builds (Linux, macOS, Windows)

### Changed
- Updated Cargo.toml metadata for better crates.io presentation
- Enhanced project documentation and release process

## [0.1.0] - 2025-07-27

### Added
- Initial implementation of Rhema CLI
- Core protocol support for Git-based agent context management
- YAML-based configuration and context files
- Command-line interface with comprehensive subcommands
- Schema validation for Rhema protocol compliance
- Context Query Language (CQL) for cross-scope queries
- Scope management and discovery
- Todo, insight, pattern, and decision tracking
- Full-text and regex search capabilities
- Health checks and validation tools
- Performance analytics and impact analysis

### Features
- `rhema init` - Initialize new Rhema scopes
- `rhema scopes` - Discover and list scopes
- `rhema query` - Execute CQL queries
- `rhema todo` - Manage todo items
- `rhema insight` - Manage insights and knowledge
- `rhema pattern` - Manage design patterns
- `rhema decision` - Manage architecture decisions
- `rhema search` - Search across context files
- `rhema validate` - Validate protocol compliance
- `rhema health` - Check scope health
- `rhema stats` - View analytics and metrics
- `rhema sync` - Synchronize scopes
- `rhema migrate` - Migrate between protocol versions

### Technical
- Built with Rust for performance and reliability
- Comprehensive error handling with custom error types
- Extensive test coverage with unit, integration, and property tests
- Performance benchmarking and optimization
- Security-focused development practices
- Cross-platform compatibility (Linux, macOS, Windows)

---

## Release Types

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** in case of vulnerabilities 