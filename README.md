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

## 📖 Documentation

- [CLI Command Reference](docs/cli-command-reference.md) - Complete CLI command documentation
- [Specification Schema Examples](docs/specification-schema-examples.md) - Example YAML files and schemas
- [Development Guide](docs/development.md) - Building, testing, and contributing to Rhema
- [Specification Documentation](schemas/README.md) - Detailed specification schema documentation
- [API Reference](docs/api.md) - CLI API documentation
- [Examples](docs/examples/README.md) - Usage examples and patterns
- [Migration Guide](docs/migration.md) - Specification version migration


## 📋 Table of Contents

- [The Problem: Lost Context](#-the-problem-lost-context)
- [The Rhema Solution](#-the-rhema-solution)
- [Key Capabilities](#️-key-capabilities)
  - [Git-Native Context Management](#git-native-context-management)
  - [Powerful Context Querying](#powerful-context-querying)
  - [AI Agent Integration](#ai-agent-integration)
  - [Team Collaboration](#team-collaboration)
- [Installation](#-installation)
  - [From Cargo (Recommended)](#from-cargo-recommended)
  - [From Source](#from-source)
  - [From Binary Releases](#from-binary-releases)
- [Quick Start](#️-quick-start)
- [Example: From Implicit to Explicit Knowledge](#️-example-from-implicit-to-explicit-knowledge)
- [Advanced Usage](#️-advanced-usage)
- [Core Specification Concepts](#-core-specification-concepts)
  - [Scopes](#scopes)
  - [Specification File Types](#specification-file-types)
  - [CQL (Context Query Language)](#cql-context-query-language)
  - [Query Provenance Tracking](#query-provenance-tracking)
- [Specification vs Implementation](#specification-vs-implementation)
  - [🏗️ Rhema Protocol Specification](#️-rhema-protocol-specification)
  - [⚙️ Rhema CLI (Implementation)](#️-rhema-cli-implementation)
  - [🔄 Relationship](#️-relationship)
- [Contributing](#️-contributing)
- [Roadmap](#️-roadmap)
- [Support](#️-support)
- [License](#-license)
- [Acknowledgments](#️-acknowledgments)

## 🎯 The Problem: Lost Context

In modern development, critical knowledge exists in:
- **Individual minds** - Developer memories and experiences
- **Temporary chats** - AI conversations that disappear
- **Scattered docs** - Unstructured, stale documentation
- **Forgotten decisions** - Architectural choices with lost rationale

This creates **knowledge silos**, **session amnesia**, and **inconsistent AI behavior** across your team.

## ✨ The Rhema Solution

Rhema transforms ephemeral knowledge into **persistent, structured context** that:

- **🔄 Survives sessions** - Context persists across AI conversations and development sessions
- **👥 Scales with teams** - Knowledge is shared and discoverable across your organization  
- **📈 Evolves with code** - Context changes are tracked alongside code in Git
- **🎯 Enables consistency** - AI agents access the same structured context
- **⚡ Accelerates onboarding** - New team members quickly understand project context

## 🚀 Key Capabilities

### Git-Native Context Management
- **Distributed YAML files** - Context travels with your repository
- **Schema validation** - JSON Schema ensures specification compliance
- **Version control** - Context evolves with your codebase
- **Cross-scope relationships** - Explicit dependencies between components

### Powerful Context Querying
- **CQL (Context Query Language)** - Query across multiple scopes and repositories
- **Full-text search** - Find knowledge across your entire codebase
- **Provenance tracking** - Complete audit trail of query execution
- **Real-time updates** - Context stays current with your development

### AI Agent Integration
- **MCP Daemon** - Real-time context service for AI agents
- **Context primers** - Automated context generation for AI conversations
- **Session persistence** - Maintain context across AI interactions

## 🏗️ Rhema Ecosystem

### Core Platform: Rhema
- **Context Management**: Git-native context capture and organization
- **Knowledge Persistence**: Structured YAML-based knowledge storage
- **AI Integration**: MCP daemon for AI agent context services
- **Team Collaboration**: Shared knowledge across development teams



### Team Collaboration
- **Distributed ownership** - Teams manage their scope's context independently
- **Project orchestration** - Coordinate work across multiple scopes and teams
- **Knowledge discovery** - Find and leverage existing context across the codebase
- **Decision preservation** - Architecture decisions and rationale are permanently recorded

## 📦 Installation

### From Cargo (Recommended)
```bash
cargo install rhema
```

### From Source
```bash
git clone https://github.com/fugue-ai/rhema.git
cd rhema
cargo build --release
```

### From Binary Releases
Download the latest release binary for your platform from the [releases page](https://github.com/fugue-ai/rhema/releases).

## 🏃‍♂️ Quick Start

For a comprehensive getting started guide, see [docs/quick-start.md](docs/quick-start.md).

The guide covers:
- **Basic setup** - Installation and initialization
- **Project scenarios** - Solo projects, microservices, monorepos, open source
- **Context management** - Adding todos, decisions, knowledge, and patterns
- **Querying** - Finding and filtering context across scopes
- **Team collaboration** - Multi-team coordination and onboarding
- **AI integration** - MCP daemon setup and context primers
- **Advanced patterns** - Context-driven development and analytics
- **Troubleshooting** - Common issues and solutions

**Quick commands to get started:**
```bash
# Install and initialize
cargo install rhema
rhema init

# Add some context
rhema todo add "Implement user authentication" --priority high
rhema decision record "Use PostgreSQL" --status approved

# Query your context
rhema query "todos WHERE priority='high'"
rhema query "decisions WHERE status='approved'"
```

For detailed examples, see [docs/examples/](docs/examples/README.md).

## 💡 Example: From Implicit to Explicit Knowledge

See [docs/examples/implicit-to-explicit-knowledge.md](docs/examples/implicit-to-explicit-knowledge.md) for a detailed example of how Rhema transforms scattered knowledge into structured, persistent context.

## 🚀 Advanced Usage

See [docs/examples/advanced-usage.md](docs/examples/advanced-usage.md) for advanced patterns including cross-scope coordination, batch operations, and AI integration.

For complex project coordination examples, see [docs/examples/ecommerce-epic-orchestration.md](docs/examples/ecommerce-epic-orchestration.md).

## 📚 Core Specification Concepts

### Scopes

A scope represents a logical boundary in your codebase (service, app, library, etc.). Each scope has its own `.rhema/` directory containing specification-compliant context files.

**Scope Types:**
- `service` - Microservices and API endpoints
- `library` - Reusable code libraries
- `application` - Full applications
- `component` - UI components or modules
- `infrastructure` - Infrastructure and deployment configs

### Specification File Types

The Rhema specification defines six core file types, each with a specific JSON Schema:

- **`rhema.yaml`** - Scope definition, metadata, and dependencies (also supported as `scope.yaml`)
- **`knowledge.yaml`** - Insights, learnings, and domain knowledge
- **`todos.yaml`** - Work items, tasks, and completion history
- **`decisions.yaml`** - Architecture decision records (ADRs)
- **`patterns.yaml`** - Design patterns and architectural patterns
- **`conventions.yaml`** - Coding conventions and team standards

### CQL (Context Query Language)

A simple YAML path-based query syntax for cross-scope context retrieval. See [docs/examples/cql-queries.md](docs/examples/cql-queries.md) for comprehensive examples.

### Query Provenance Tracking

Rhema provides comprehensive provenance tracking for all queries, enabling full audit trails and data lineage. See [docs/examples/query-provenance.md](docs/examples/query-provenance.md) for detailed examples and use cases.

## Specification vs Implementation

Rhema consists of two distinct but complementary components:

### 🏗️ Rhema Protocol Specification
The **Rhema Protocol Specification** is an open specification that defines:
- **📋 JSON Schemas** - Formal definitions for all context file types (scopes, knowledge, decisions, etc.)
- **🔗 Resolution Process** - How context is discovered and merged across scopes and repositories
- **📝 Conventions** - Naming, organization, and best practices for context files
- **🔍 Query Language** - CQL (Context Query Language) specification for cross-scope queries
- **🤝 Integration Standards** - Specifications for AI agent integration (MCP, etc.)

**Key Benefits:**
- **Vendor Neutral** - Any tool can implement the specification
- **Future Proof** - Specification evolves independently of implementations
- **Interoperable** - Different tools can work with the same context files
- **Extensible** - Custom fields and extensions are supported

### ⚙️ Rhema CLI (Implementation)
The **Rhema CLI** is a production-ready Rust implementation that provides:
- **🛠️ Core Tools** - Initialize, validate, query, and manage context files
- **🔍 Advanced Search** - Full-text search across multiple scopes and repositories
- **🤖 AI Integration** - MCP daemon for real-time context service to AI agents
- **📊 Analytics** - Usage patterns, quality metrics, and context evolution tracking
- **🔧 Developer Experience** - Interactive mode, batch operations, and IDE integration

**Key Benefits:**
- **High Performance** - Rust implementation for speed and reliability
- **Production Ready** - Comprehensive error handling, logging, and monitoring
- **Extensible** - Plugin architecture for custom integrations
- **Cross-Platform** - Works on Windows, macOS, and Linux

### 🔄 Relationship
- **Specification First** - The specification drives all implementations
- **Reference Implementation** - The CLI serves as the reference implementation
- **Ecosystem Growth** - Other tools can implement the specification independently
- **Continuous Evolution** - Both specification and implementation evolve together based on community feedback

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed information on how to contribute to Rhema.

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for current development priorities and future plans.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/your-org/rhema/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/rhema/discussions)
- **Documentation**: [Project Wiki](https://github.com/your-org/rhema/wiki)

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by Architecture Decision Records (ADRs)
- Built on the principles of Git-native documentation
- Designed for AI agent collaboration and human-AI interaction
- Embraces the philosophy of explicit knowledge over implicit assumptions
- Addresses the challenge of ephemeral context in AI-assisted development 