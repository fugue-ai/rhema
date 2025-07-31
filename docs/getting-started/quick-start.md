# Quick Start Guide

Welcome to Rhema! This guide will walk you through getting started with Rhema, from basic setup to advanced usage patterns. Whether you're a solo developer or part of a team, this guide will help you transform implicit knowledge into explicit, persistent context.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Basic Quick Start](#basic-quick-start)
- [Project Setup Scenarios](#project-setup-scenarios)
- [Working with Context](#working-with-context)
- [Querying and Discovery](#querying-and-discovery)
- [Team Collaboration](#team-collaboration)
- [AI Integration](#ai-integration)
- [Advanced Patterns](#advanced-patterns)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before you begin, ensure you have:

- **Git**: Rhema is Git-native, so you'll need Git installed and configured
- **Rust**: For building from source (optional, but recommended for development)
- **Basic familiarity with YAML**: Rhema uses YAML for all context files
- **A project to work with**: Any codebase can benefit from Rhema

## Installation

### Option 1: From Cargo (Recommended)
```bash
cargo install rhema
```

### Option 2: From Source
```bash
git clone https://github.com/fugue-ai/rhema.git
cd rhema
cargo build --release
# The binary will be in target/release/rhema
```

### Option 3: From Binary Releases
Download the latest release for your platform from the [releases page](https://github.com/fugue-ai/rhema/releases).

### Verify Installation
```bash
rhema --version
rhema --help
```

## Basic Quick Start

This section covers the essential steps to get started with Rhema in any project.

### 1. Initialize a Scope

A scope represents a logical boundary in your codebase. Think of it as a container for related context.

```bash
# Initialize in current directory (interactive)
rhema init

# Initialize with specific parameters
rhema init --scope-type service --scope-name user-service --description "User authentication and management service"
```

**What happens during initialization:**
- Creates a `.rhema/` directory in your project
- Generates initial YAML files based on the Rhema specification
- Sets up basic scope configuration
- Creates a `.gitignore` entry for temporary files

**Scope Types:**
- `service` - Microservices, APIs, backend services
- `library` - Reusable code libraries, packages
- `application` - Full applications, monoliths
- `component` - UI components, modules, widgets
- `infrastructure` - Infrastructure, deployment, configuration

### 2. Add Context

Context is the heart of Rhema. Let's add different types of context to your scope.

#### Add Todos (Work Items)
```bash
# Basic todo
rhema todo add "Implement user authentication" --priority high

# Todo with more details
rhema todo add "Add password reset functionality" \
  --priority medium \
  --assignee "alice@example.com" \
  --description "Users need to be able to reset their passwords via email" \
  --tags "security,user-experience"

# Todo with dependencies
rhema todo add "Set up email service" \
  --priority high \
  --dependencies "user-auth,email-provider" \
  --estimated-hours 8
```

#### Record Decisions (Architecture Decision Records)
```bash
# Basic decision
rhema decision record "Use PostgreSQL" \
  --description "Chosen for ACID compliance" \
  --status approved

# Detailed decision with alternatives
rhema decision record "Authentication Strategy" \
  --description "Implement JWT-based authentication with refresh tokens" \
  --status approved \
  --rationale "Provides stateless authentication with good security properties" \
  --alternatives "Session-based auth, OAuth2 only" \
  --impact "Affects all API endpoints and frontend authentication flow" \
  --date "2024-01-15"
```

#### Capture Knowledge (Insights and Learnings)
```bash
# Performance insight
rhema insight record "Database connection pooling improves performance by 40%" \
  --confidence 8 \
  --category performance \
  --evidence "Load test results, monitoring data"

# Security finding
rhema insight record "JWT tokens should be stored in httpOnly cookies" \
  --confidence 9 \
  --category security \
  --impact "Prevents XSS attacks on token theft" \
  --related-files "src/auth/jwt.rs,src/middleware/auth.rs"
```

#### Define Patterns (Design Patterns)
```bash
# Code pattern
rhema pattern record "Error Handling Pattern" \
  --description "Use Result<T, AppError> for all fallible operations" \
  --usage required \
  --examples "src/error.rs,src/handlers/user.rs" \
  --rationale "Provides consistent error handling across the application"

# Architecture pattern
rhema pattern record "Repository Pattern" \
  --description "Abstract data access behind repository interfaces" \
  --usage recommended \
  --examples "src/repositories/user.rs,src/repositories/order.rs"
```

### 3. Query Your Context

Now that you have context, let's explore how to find and use it.

#### Basic Queries
```bash
# List all todos
rhema query "todos"

# Find high-priority todos
rhema query "todos WHERE priority='high'"

# Search for security-related knowledge
rhema query "knowledge WHERE category='security'"

# Find approved decisions
rhema query "decisions WHERE status='approved'"
```

#### Advanced Queries
```bash
# Complex filtering
rhema query "todos WHERE priority='high' AND status='pending'"

# Search across multiple fields
rhema query "knowledge WHERE category='performance' OR category='security'"

# Query with ordering
rhema query "todos ORDER BY priority DESC, created_at ASC"

# Limit results
rhema query "todos WHERE status='pending' LIMIT 5"
```

#### Cross-Scope Queries
```bash
# Query across all scopes in repository
rhema query "*/todos WHERE priority='high'"

# Query specific scope
rhema query "../auth-service/todos WHERE status='in_progress'"

# Find patterns across all scopes
rhema query "*/patterns WHERE usage='required'"
```

### 4. Discover and Navigate

Rhema helps you understand the structure of your project and its context.

```bash
# List all scopes in the repository
rhema scopes

# Show scope hierarchy
rhema tree

# Show detailed scope information
rhema show --scope .

# Validate scope configuration
rhema validate
```

## Project Setup Scenarios

Different projects have different needs. Here are common scenarios and how to set up Rhema for each.

### Scenario 1: Solo Developer Project

**Use Case**: You're building a personal project and want to maintain context for future you.

```bash
# Initialize with minimal configuration
rhema init --scope-type application --scope-name my-project

# Add context as you work
rhema todo add "Fix the login bug" --priority high
rhema insight record "Using React Query improved data fetching performance" --confidence 7
rhema decision record "Use SQLite for development" --status approved

# Query when you need context
rhema query "todos WHERE status='pending'"
rhema query "knowledge WHERE category='performance'"
```

### Scenario 2: Microservices Architecture

**Use Case**: You have multiple services that need coordinated context management.

```bash
# Initialize each service as a scope
cd user-service && rhema init --scope-type service --scope-name user-service
cd ../auth-service && rhema init --scope-type service --scope-name auth-service
cd ../payment-service && rhema init --scope-type service --scope-name payment-service

# Add cross-service dependencies
cd user-service
rhema scope add-dependency ../auth-service --type required
rhema scope add-dependency ../payment-service --type optional

# Track cross-service todos
rhema todo add "Integrate with auth-service" --dependencies "auth-service-api" --scope auth-service
rhema todo add "Add payment processing" --dependencies "payment-service-integration" --scope payment-service

# Query across services
rhema query "*/todos WHERE status='blocked'"
rhema query "*/decisions WHERE impact_scope='multiple'"
```

### Scenario 3: Monorepo with Multiple Teams

**Use Case**: Multiple teams work in the same repository on different components.

```bash
# Initialize project-level scope
rhema init --scope-type application --scope-name ecommerce-platform

# Initialize team scopes
cd frontend && rhema init --scope-type component --scope-name frontend-app
cd ../backend && rhema init --scope-type service --scope-name backend-api
cd ../mobile && rhema init --scope-type application --scope-name mobile-app

# Set up team boundaries
cd frontend
rhema scope set-boundaries --includes "src/**/*" --excludes "node_modules/**/*"

cd ../backend
rhema scope set-boundaries --includes "src/**/*" --excludes "target/**/*"

# Track team responsibilities
rhema scope set-responsibilities "User interface development" "State management" "API integration"
```

### Scenario 4: Open Source Library

**Use Case**: You're maintaining an open source library and want to track decisions and patterns.

```bash
# Initialize as library scope
rhema init --scope-type library --scope-name my-library

# Document API decisions
rhema decision record "Use builder pattern for configuration" \
  --status approved \
  --rationale "Provides fluent API and prevents invalid configurations" \
  --alternatives "Constructor parameters, configuration struct"

# Document usage patterns
rhema pattern record "Error Handling" \
  --description "Return Result<T, LibraryError> for all public functions" \
  --usage required \
  --examples "examples/basic_usage.rs"

# Track breaking changes
rhema todo add "Deprecate old API in v2.0" \
  --priority high \
  --tags "breaking-change" \
  --description "Remove deprecated functions and update documentation"
```

## Working with Context

Context in Rhema is structured and persistent. Here's how to work with it effectively.

### Context File Structure

After initialization, your `.rhema/` directory contains:

```
.rhema/
├── rhema.yaml          # Scope definition and metadata
├── todos.yaml          # Work items and tasks
├── decisions.yaml      # Architecture decision records
├── knowledge.yaml      # Insights and learnings
├── patterns.yaml       # Design patterns and conventions
└── conventions.yaml    # Coding standards and team practices
```

### Adding Context Interactively

Rhema provides interactive modes for adding context:

```bash
# Interactive todo creation
rhema todo add --interactive

# Interactive decision recording
rhema decision record --interactive

# Interactive knowledge capture
rhema insight record --interactive
```

### Batch Operations

For large projects, you can perform batch operations:

```bash
# Validate all scopes
rhema batch validate

# Export context from all scopes
rhema batch export --format json --output ./context-export

# Health check across all scopes
rhema batch health --detailed
```

### Context Validation

Rhema validates all context against JSON schemas:

```bash
# Validate current scope
rhema validate

# Validate with detailed output
rhema validate --detailed

# Validate specific file
rhema validate --file todos.yaml
```

## Querying and Discovery

Rhema's query language (CQL) is powerful and flexible. Here are common query patterns.

### Basic Query Patterns

```bash
# List all items of a type
rhema query "todos"
rhema query "decisions"
rhema query "knowledge"

# Filter by status
rhema query "todos WHERE status='pending'"
rhema query "decisions WHERE status='approved'"

# Filter by priority
rhema query "todos WHERE priority='high'"
rhema query "todos WHERE priority IN ('high', 'critical')"

# Filter by date
rhema query "todos WHERE created_at > '2024-01-01'"
rhema query "decisions WHERE date >= '2024-01-01'"
```

### Advanced Query Patterns

```bash
# Complex conditions
rhema query "todos WHERE priority='high' AND (status='pending' OR status='in_progress')"

# Search in descriptions
rhema query "todos WHERE description CONTAINS 'authentication'"

# Filter by tags
rhema query "todos WHERE tags CONTAINS 'security'"

# Query with ordering
rhema query "todos ORDER BY priority DESC, created_at ASC"

# Limit and offset
rhema query "todos WHERE status='pending' LIMIT 10 OFFSET 20"
```

### Cross-Scope Queries

```bash
# Query all scopes
rhema query "*/todos WHERE priority='high'"

# Query specific scope
rhema query "../auth-service/todos"

# Query multiple specific scopes
rhema query "{frontend,backend}/todos WHERE status='pending'"

# Find dependencies across scopes
rhema query "*/dependencies WHERE type='required'"
```

### Search and Discovery

```bash
# Full-text search
rhema search "authentication"

# Search with filters
rhema search "performance" --category knowledge

# Search across multiple scopes
rhema search "security" --scope "*/"

# Show search results with context
rhema search "database" --show-context
```

## Team Collaboration

Rhema shines in team environments. Here's how to use it effectively with your team.

### Setting Up Team Context

```bash
# Initialize team scope
rhema init --scope-type service --scope-name team-service

# Set team boundaries
rhema scope set-boundaries --includes "src/**/*" --excludes "tests/**/*"

# Define team responsibilities
rhema scope set-responsibilities \
  "User management" \
  "Authentication" \
  "API development" \
  "Database operations"
```

### Collaborative Context Management

```bash
# Add team conventions
rhema convention record "Code Style" \
  --description "Use rustfmt for all Rust code" \
  --usage required \
  --enforcement "pre-commit hook"

# Document team patterns
rhema pattern record "Error Handling" \
  --description "Use thiserror for custom error types" \
  --usage required \
  --examples "src/error.rs"

# Track team decisions
rhema decision record "Use async/await" \
  --status approved \
  --rationale "Better performance and resource utilization" \
  --impact "All new code must be async"
```

### Cross-Team Coordination

```bash
# Track cross-team dependencies
rhema scope add-dependency ../other-team-service --type required

# Document integration points
rhema insight record "API versioning strategy" \
  --description "Use semantic versioning for API changes" \
  --confidence 8 \
  --category "integration"

# Monitor cross-team blockers
rhema query "*/todos WHERE status='blocked' AND assignee='other-team'"
```

### Team Onboarding

```bash
# Generate onboarding context
rhema primer generate --scope . --output onboarding.md

# Export team context
rhema export --format markdown --output team-context.md

# Show scope overview
rhema show --scope . --detailed
```

## AI Integration

Rhema is designed to work seamlessly with AI agents. Here's how to integrate it.

### MCP Daemon Setup

The MCP (Model Context Protocol) daemon provides real-time context to AI agents.

```bash
# Start the daemon
rhema daemon start

# Start with specific configuration
rhema daemon start --port 3000 --host localhost

# Start in background
rhema daemon start --background

# Check daemon status
rhema daemon status

# Stop the daemon
rhema daemon stop
```

### Context Primers

Generate context primers for AI conversations:

```bash
# Generate basic primer
rhema primer generate --scope . --output primer.md

# Generate detailed primer
rhema primer generate --scope . --detailed --output detailed-primer.md

# Generate primer for specific context
rhema primer generate --scope . --context "authentication" --output auth-primer.md
```

### AI Agent Configuration

Configure your AI agent to use Rhema context:

```yaml
# Example AI agent configuration
mcp_servers:
  rhema:
    command: rhema
    args: ["daemon", "start"]
    env:
      RHEMA_SCOPE_PATH: "."
```

### Context-Aware AI Interactions

With the daemon running, your AI agent can:

- Query current project context
- Access architectural decisions
- Understand current todos and priorities
- Get insights about performance and security
- Learn about established patterns and conventions

## Advanced Patterns

Once you're comfortable with the basics, here are some advanced patterns.

### Context-Driven Development

```bash
# Start development session with context
rhema primer generate --scope . --output session-context.md

# Track development progress
rhema todo add "Implement feature X" --status in_progress
rhema insight record "Feature X requires careful error handling" --confidence 7

# Update context as you work
rhema todo update "todo-123" --status completed --notes "Implemented with tests"
```

### Context Migration

When refactoring or restructuring:

```bash
# Export current context
rhema export --format json --output context-backup.json

# Initialize new scope structure
rhema init --scope-type service --scope-name new-service

# Import relevant context
rhema import --file context-backup.json --filter "category='architecture'"
```

### Context Analytics

```bash
# Generate context analytics
rhema stats --scope . --detailed

# Track context evolution
rhema stats --scope . --evolution --output evolution-report.md

# Analyze context quality
rhema validate --quality --output quality-report.json
```

### Continuous Integration

Integrate Rhema into your CI/CD pipeline:

```yaml
# Example GitHub Actions workflow
- name: Validate Rhema Context
  run: rhema batch validate

- name: Generate Context Report
  run: rhema batch report --output context-report.md

- name: Check Context Quality
  run: rhema batch validate --quality --fail-on-errors
```

## Troubleshooting

Common issues and their solutions.

### Installation Issues

**Problem**: `rhema: command not found`
```bash
# Ensure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reinstall Rhema
cargo install rhema --force
```

**Problem**: Permission denied
```bash
# Check file permissions
ls -la ~/.cargo/bin/rhema

# Fix permissions if needed
chmod +x ~/.cargo/bin/rhema
```

### Initialization Issues

**Problem**: `rhema init` fails
```bash
# Check if .rhema directory already exists
ls -la .rhema

# Remove existing directory if corrupted
rm -rf .rhema

# Reinitialize
rhema init
```

**Problem**: Invalid scope type
```bash
# List valid scope types
rhema init --help

# Use valid scope type
rhema init --scope-type service
```

### Query Issues

**Problem**: Query returns no results
```bash
# Check if context exists
rhema query "todos"

# Validate context files
rhema validate

# Check file permissions
ls -la .rhema/
```

**Problem**: Cross-scope query fails
```bash
# Check scope structure
rhema tree

# Verify scope paths
rhema scopes

# Use correct path syntax
rhema query "../other-scope/todos"
```

### Daemon Issues

**Problem**: Daemon won't start
```bash
# Check if port is in use
lsof -i :3000

# Use different port
rhema daemon start --port 3001

# Check daemon logs
rhema daemon status --verbose
```

**Problem**: AI agent can't connect
```bash
# Verify daemon is running
rhema daemon status

# Check network configuration
rhema daemon start --host 0.0.0.0 --port 3000

# Test connection
curl http://localhost:3000/health
```

### Performance Issues

**Problem**: Queries are slow
```bash
# Check scope size
rhema stats --scope . --size

# Optimize large files
rhema validate --optimize

# Use more specific queries
rhema query "todos WHERE status='pending'" --limit 10
```

**Problem**: High memory usage
```bash
# Check memory usage
rhema stats --scope . --memory

# Optimize context files
rhema validate --optimize --aggressive

# Consider splitting large scopes
rhema scope split --max-size 1000
```

## Next Steps

Now that you've completed the quick start guide, here are some next steps:

1. **Explore the Documentation**: Check out the [CLI Command Reference](cli-command-reference.md) for detailed command documentation
2. **Review Examples**: See [Specification Schema Examples](specification-schema-examples.md) for YAML file examples
3. **Learn Advanced Features**: Dive into [Advanced Usage](advanced-usage.md) for complex scenarios
4. **Join the Community**: Contribute to Rhema and share your experiences
5. **Integrate with Your Workflow**: Set up Rhema in your daily development process

Remember, Rhema is designed to grow with your project. Start simple and add more context as your project evolves. The key is making implicit knowledge explicit and persistent! 