# Rhema CLI Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-cli)](https://crates.io/crates/rhema-cli)
[![Documentation](https://docs.rs/rhema-cli/badge.svg)](https://docs.rs/rhema-cli)

Command line interface and interactive mode for the Rhema Protocol, providing a comprehensive CLI for knowledge management and development workflow automation.

## Overview

The `rhema-cli` crate provides the primary user interface for Rhema, offering both command-line and interactive modes for managing project knowledge, coordinating AI agents, and automating development workflows. It serves as the main entry point for users to interact with Rhema's capabilities.

## Features

### 🖥️ Command Line Interface
- **Comprehensive Commands**: Full suite of commands for all Rhema functionality
- **Interactive Mode**: Rich interactive shell with command history and autocompletion
- **Batch Operations**: Execute multiple operations efficiently
- **Scripting Support**: Support for automation scripts and workflows

### 🧠 Knowledge Management
- **Todo Management**: Create, update, and track work items
- **Decision Tracking**: Record and manage architectural decisions
- **Pattern Documentation**: Document design patterns and best practices
- **Insight Capture**: Capture and organize development insights

### 🔍 Query and Search
- **CQL Support**: Context Query Language for advanced knowledge discovery
- **Full-Text Search**: Search across all knowledge artifacts
- **Semantic Search**: AI-powered semantic search capabilities
- **Repository Analysis**: Analyze repository structure and content

### 🤖 AI Integration
- **Agent Coordination**: Manage and coordinate AI agents
- **Context Injection**: Inject relevant context into AI conversations
- **Workflow Automation**: Automate development workflows with AI assistance
- **MCP Integration**: Model Context Protocol integration for AI tools

### 🔧 Development Tools
- **Git Integration**: Seamless Git workflow integration
- **Configuration Management**: Manage project and global configurations
- **Monitoring**: Real-time monitoring and observability
- **IDE Integration**: Support for various development environments

## Architecture

```
rhema-cli/
├── commands/        # Command implementations
│   ├── mod.rs       # Command module organization
│   ├── init.rs      # Project initialization
│   ├── query.rs     # Query and search commands
│   ├── todo.rs      # Todo management
│   ├── decision.rs  # Decision tracking
│   ├── pattern.rs   # Pattern documentation
│   ├── insight.rs   # Insight capture
│   ├── git.rs       # Git integration
│   ├── config.rs    # Configuration management
│   ├── ai.rs        # AI integration
│   ├── mcp.rs       # MCP protocol
│   ├── monitor.rs   # Monitoring and observability
│   └── batch.rs     # Batch operations
├── interactive/     # Interactive mode
├── output/          # Output formatting
└── utils/           # Utility functions
```

## Usage

### Basic Commands

```bash
# Initialize Rhema in a project
rhema init

# Start interactive mode
rhema interactive

# Query knowledge
rhema query "find all TODO comments"

# Add a todo item
rhema todo add "Implement user authentication"

# Record a decision
rhema decision add "Use JWT for authentication"
```

### Interactive Mode

```bash
# Start interactive mode
rhema interactive

# Available commands in interactive mode
> help                    # Show available commands
> query "find patterns"   # Search for patterns
> todo list              # List all todos
> decision show          # Show recent decisions
> git status             # Git integration
> ai agents              # AI agent management
```

### Advanced Usage

```bash
# Batch operations
rhema batch --file operations.yaml

# Export context for AI
rhema context export --scope user-service

# Monitor system health
rhema monitor health

# Configure global settings
rhema config set global.editor vscode
```

## Configuration

The CLI supports both global and project-specific configuration:

```yaml
# Global configuration (~/.rhema/config.yaml)
global:
  editor: vscode
  theme: dark
  ai_provider: openai
  cache_dir: ~/.rhema/cache

# Project configuration (.rhema/rhema.yaml)
rhema:
  version: "1.0.0"
  scope:
    name: "my-project"
    type: "service"
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **rhema-query**: Query engine and search
- **rhema-git**: Git integration
- **rhema-ai**: AI service integration
- **rhema-mcp**: MCP protocol support
- **rhema-config**: Configuration management
- **rhema-knowledge**: Knowledge management
- **rhema-monitoring**: Monitoring and observability
- **rhema-integrations**: External integrations

## Development Status

### ✅ Completed Features
- Basic command structure
- Interactive mode framework
- Todo management commands
- Query and search functionality
- Git integration commands

### 🔄 In Progress
- AI agent coordination
- MCP protocol integration
- Advanced batch operations
- Performance optimization

### 📋 Planned Features
- Enhanced interactive mode
- Workflow automation
- Advanced scripting support
- Plugin system

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all commands have proper help text and documentation
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 