# Complete Getting Started Guide

Welcome to Rhema! This comprehensive guide will walk you through everything you need to know to get started with Rhema, from installation to your first successful project setup.

## 🚀 Quick Installation

### Prerequisites
- **Rust**: Version 1.70 or higher
- **Git**: For version control integration
- **Node.js**: Version 18+ (for editor plugins)

### Installation Methods

#### Method 1: From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/fugue-ai/rhema.git
cd rhema

# Build and install
cargo build --release
cargo install --path .
```

#### Method 2: Using Cargo
```bash
# Install directly from crates.io (when available)
cargo install rhema
```

#### Method 3: Using Package Managers
```bash
# macOS (using Homebrew)
brew install rhema

# Linux (using package manager)
# Check your distribution's package repository
```

### Verify Installation
```bash
# Check Rhema version
rhema --version

# Check available commands
rhema --help
```

## 📁 Your First Rhema Project

### Step 1: Initialize a New Project
```bash
# Create a new directory for your project
mkdir my-rhema-project
cd my-rhema-project

# Initialize Rhema in the project
rhema init
```

This creates the following structure:
```
my-rhema-project/
├── .rhema/
│   ├── config.yaml
│   ├── scope.yaml
│   └── lock.yaml
├── src/
├── tests/
└── README.md
```

### Step 2: Configure Your Project
Edit the generated `scope.yaml` file:

```yaml
name: my-rhema-project
description: My first Rhema project
version: "1.0.0"
author: "Your Name <your.email@example.com>"

# Define your project scope
scope:
  type: application
  language: rust
  framework: none
  
# Set up context patterns
context:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
    - "Cargo.toml"
  patterns:
    - "*.rs"
    - "*.toml"
  exclusions:
    - "target/**"
    - ".git/**"
  max_tokens: 1000
  include_hidden: false
  recursive: true
```

### Step 3: Add Your First Todo
```bash
# Add a todo item
rhema todo add --title "Set up basic project structure" \
               --description "Create initial source files and tests" \
               --priority high \
               --assignee "developer"
```

### Step 4: Record Your First Insight
```bash
# Record an insight about your project
rhema insight add --title "Project structure considerations" \
                  --content "Consider using a modular architecture for better maintainability" \
                  --confidence 8 \
                  --category "architecture"
```

## 🔧 Basic Workflow

### Managing Todos
```bash
# List all todos
rhema todo list

# Filter todos by status
rhema todo list --status pending

# Filter todos by priority
rhema todo list --priority high

# Update a todo
rhema todo update --id "TODO-001" --status "in-progress"

# Complete a todo
rhema todo complete --id "TODO-001" --outcome "Successfully implemented feature"
```

### Recording Insights
```bash
# Add insights as you work
rhema insight add --title "Performance optimization opportunity" \
                  --content "Database queries can be optimized by adding indexes" \
                  --confidence 9 \
                  --category "performance" \
                  --tags "database,optimization"

# Search insights
rhema insight search --query "performance"

# List insights by category
rhema insight list --category "performance"
```

### Managing Patterns
```bash
# Record a pattern you've identified
rhema pattern add --name "Error handling pattern" \
                  --description "Consistent error handling across modules" \
                  --type "code" \
                  --examples "src/error.rs,src/validation.rs"

# Search for patterns
rhema pattern search --query "error handling"
```

### Tracking Decisions
```bash
# Record an architectural decision
rhema decision add --title "Use async/await for I/O operations" \
                   --description "Decision to use async/await for better performance" \
                   --status "approved" \
                   --rationale "Improves performance and resource utilization" \
                   --alternatives "Synchronous I/O,Thread pools" \
                   --impact "Better scalability and user experience"
```

## 🔍 Advanced Features

### Using Interactive Mode
```bash
# Start interactive mode
rhema interactive

# In interactive mode, you can:
# - Type commands without 'rhema' prefix
# - Use tab completion
# - See command history
# - Get contextual help
```

### Batch Operations
```bash
# Update multiple todos at once
rhema todo batch --file todos-update.yaml

# Add multiple insights from a file
rhema insight batch --file insights.yaml
```

### Context Query Language (CQL)
```bash
# Query across all context data
rhema query "todos where priority = 'high' and status = 'pending'"

# Complex queries
rhema query "insights where confidence > 8 and category = 'performance'"

# Cross-scope queries
rhema query "todos,insights where assignee = 'developer'"
```

### Validation and Health Checks
```bash
# Validate your project configuration
rhema validate

# Check project health
rhema health

# Validate lock file
rhema lock validate
```

## 🛠️ Integration Setup

### Git Integration
```bash
# Initialize Git repository
git init

# Add Rhema files to Git
git add .rhema/
git commit -m "Initial Rhema configuration"

# Set up Git hooks (optional)
rhema git setup-hooks
```

### VS Code Extension
1. Install the Rhema VS Code extension
2. Open your Rhema project
3. Use the command palette (`Ctrl+Shift+P`) to access Rhema commands
4. Use the Rhema sidebar for quick access to todos, insights, and more

### CI/CD Integration
Create a `.github/workflows/rhema.yml` file:

```yaml
name: Rhema Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install rhema
      - run: rhema validate
      - run: rhema health
```

## 🔧 Configuration Management

### Global Configuration
```bash
# Set global configuration
rhema config set --global editor "code"
rhema config set --global default-priority "medium"
rhema config set --global auto-save true

# View global configuration
rhema config get --global
```

### Project-Specific Configuration
Edit `.rhema/config.yaml`:

```yaml
# Project-specific settings
project:
  name: "my-rhema-project"
  auto_backup: true
  backup_interval: "1h"
  
# Editor integration
editor:
  default: "code"
  auto_open: true
  
# Validation settings
validation:
  strict: true
  auto_fix: false
  
# Performance settings
performance:
  cache_enabled: true
  cache_size: "100MB"
  parallel_processing: true
```

## 🚨 Troubleshooting

### Common Issues

#### Installation Problems
```bash
# If cargo install fails
rustup update
cargo clean
cargo install --force rhema
```

#### Configuration Issues
```bash
# Reset configuration
rhema config reset

# Validate configuration
rhema config validate
```

#### Permission Issues
```bash
# Fix file permissions
chmod +x $(which rhema)
chmod -R 755 .rhema/
```

### Getting Help
```bash
# Get help for any command
rhema --help
rhema <command> --help

# Check Rhema status
rhema status

# View logs
rhema logs
```

## 📚 Next Steps

### Recommended Learning Path
1. **Master the Basics**: Complete this guide and practice with a small project
2. **Explore Advanced Features**: Learn about agent coordination, knowledge management, and performance monitoring
3. **Integrate with Your Workflow**: Set up Rhema in your existing projects
4. **Contribute**: Join the community and contribute to Rhema development

### Additional Resources
- [CLI Command Reference](../cli-command-reference.md)
- [Configuration Management](../configuration-management.md)
- [Advanced Features](../core-features/README.md)
- [Troubleshooting Guide](../../troubleshooting/common-issues.md)

### Community Support
- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share experiences
- **Documentation**: Contribute to improving documentation
- **Code**: Submit pull requests and contribute code

## 🎉 Congratulations!

You've successfully set up Rhema and completed your first project! You now have:
- ✅ A working Rhema installation
- ✅ A configured project with proper structure
- ✅ Experience with basic Rhema commands
- ✅ Understanding of core workflows
- ✅ Integration with your development environment

Continue exploring Rhema's advanced features and integrate it into your daily development workflow. Happy coding with Rhema! 🚀 