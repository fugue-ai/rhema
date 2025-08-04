# Quick Start Guide

Welcome to Rhema! This guide will help you get started quickly and begin transforming your implicit knowledge into explicit, persistent context.

## 🎯 What is Rhema?

Rhema (/ˈreɪmə/ "RAY-muh") is a Git-native toolkit that captures, organizes, and shares project knowledge through structured YAML files. It solves the fundamental problem of ephemeral context in AI-assisted development by making implicit knowledge explicit and persistent.

**Key Benefits:**
- **Persistent Context**: Never lose important insights or decisions
- **AI-Optimized**: Structured for AI consumption and understanding
- **Team Collaboration**: Share knowledge across your entire organization
- **Git-Native**: Works seamlessly with your existing Git workflow

## 🚀 Installation

### Prerequisites
- **Git**: For version control integration
- **Rust**: For building and running Rhema (1.70+)

### Install Rhema

#### Option 1: From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/fugue-ai/rhema.git
cd rhema

# Build and install
cargo install --path .
```

#### Option 2: Using Cargo (when available on crates.io)
```bash
cargo install rhema
```

#### Option 3: Using Package Manager
```bash
# macOS (when available)
brew install rhema

# Linux (when available)
# Download from releases page
```

### Verify Installation
```bash
# Check if Rhema is installed
rhema --version

# Check available commands
rhema --help
```

## 🏗️ Your First Project

### 1. Initialize a New Scope
```bash
# Navigate to your project directory
cd your-project

# Initialize Rhema
rhema init

# Or initialize with specific type
rhema init --scope-type service --scope-name my-service
```

This creates the basic Rhema structure:
```
your-project/
├── scope.yaml              # Scope configuration
├── todos.yaml              # Todo items
├── insights.yaml           # Knowledge insights
├── decisions.yaml          # Architectural decisions
├── patterns.yaml           # Design patterns
├── conventions.yaml        # Coding conventions
└── knowledge.yaml          # General knowledge base
```

### 2. Check Your Setup
```bash
# Verify the scope was created correctly
rhema scope

# Check the health of your setup
rhema health

# View the scope hierarchy
rhema tree
```

### 3. Add Your First Knowledge
```bash
# Add a todo item
rhema todo add "Set up authentication system" --priority high

# Record an insight
rhema insight record "JWT tokens work better than sessions for mobile apps" --confidence high

# Document a decision
rhema decision record "Use GraphQL for API" --status approved --description "Better for mobile clients"
```

### 4. Query Your Knowledge
```bash
# Find all high-priority todos
rhema query "find todos where priority = high"

# Search for authentication-related insights
rhema query "find insights containing 'authentication'"

# Get decision history
rhema query "find decisions where status = approved"
```

## 📝 Understanding Rhema Files

### Scope Configuration (`scope.yaml`)
```yaml
name: "my-service"
type: "service"
description: "My awesome service"
version: "1.0.0"

metadata:
  team: "platform"
  repository: "github.com/company/my-service"
  language: "rust"

dependencies:
  - name: "auth-library"
    version: "2.1.0"
    type: "library"
```

### Todo Items (`todos.yaml`)
```yaml
tasks:
  - id: "TASK-001"
    title: "Implement user authentication"
    description: "Add JWT-based authentication system"
    priority: "high"
    status: "todo"
    tags:
      - "feature"
      - "security"
    created_date: "2024-01-15T10:00:00Z"
    due_date: "2024-01-30T17:00:00Z"
    assignee: "john.doe@company.com"
    context:
      related_files:
        - "src/auth/service.rs"
      dependencies:
        - "auth-library"
    acceptance_criteria:
      - "JWT tokens are properly generated"
      - "Authentication middleware is implemented"
      - "Tests are written and passing"
```

### Insights (`insights.yaml`)
```yaml
insights:
  - id: "INSIGHT-001"
    insight: "Using Redis for session storage improves performance by 40%"
    confidence: "high"
    category: "performance"
    tags:
      - "performance"
      - "caching"
      - "redis"
    created_date: "2024-01-15T14:30:00Z"
    context:
      related_files:
        - "src/session/service.rs"
      evidence:
        - "Benchmark results show 40% improvement"
        - "Memory usage reduced by 60%"
```

### Decisions (`decisions.yaml`)
```yaml
decisions:
  - id: "DECISION-001"
    title: "Use GraphQL for API"
    description: "GraphQL provides better flexibility for mobile clients"
    status: "approved"
    decision_type: "architecture"
    tags:
      - "api"
      - "graphql"
      - "mobile"
    created_date: "2024-01-15T09:00:00Z"
    context:
      problem: "REST API is too rigid for mobile client needs"
      alternatives:
        - name: "REST API"
          description: "Traditional REST endpoints"
          pros: ["Simple", "Well understood"]
          cons: ["Rigid", "Over-fetching"]
        - name: "GraphQL"
          description: "Query language for APIs"
          pros: ["Flexible", "Efficient"]
          cons: ["Complex", "Learning curve"]
      outcome: "GraphQL provides the best balance of flexibility and performance"
```

## 🔍 Querying Your Knowledge

### Basic Queries
```bash
# Find all todos
rhema query "find all todos"

# Find high-priority todos
rhema query "find todos where priority = high"

# Find insights about performance
rhema query "find insights containing 'performance'"

# Find approved decisions
rhema query "find decisions where status = approved"
```

### Advanced Queries
```bash
# Find todos with specific tags
rhema query "find todos where tags contains 'security'"

# Find insights with high confidence
rhema query "find insights where confidence = high"

# Find decisions by type
rhema query "find decisions where decision_type = architecture"

# Complex queries
rhema query "find todos where priority = high AND status = todo"
rhema query "find insights where confidence = high OR tags contains 'critical'"
```

### Aggregation Queries
```bash
# Count todos by status
rhema query "count todos by status"

# Count insights by confidence
rhema query "count insights by confidence"

# Group decisions by type
rhema query "group decisions by decision_type"
```

## 🔧 Essential Commands

### Knowledge Management
```bash
# Todo management
rhema todo add "Task title" --priority high --tags "feature,security"
rhema todo list --status todo --priority high
rhema todo complete TASK-001 --outcome "Successfully implemented"

# Insight recording
rhema insight record "Your insight here" --confidence high --tags "performance"
rhema insight list --confidence high

# Decision tracking
rhema decision record "Decision title" --status approved --description "Rationale"
rhema decision list --status approved
```

### Validation and Health
```bash
# Validate your files
rhema validate

# Check scope health
rhema health

# Show statistics
rhema stats
```

### Export and Sharing
```bash
# Export context for AI agents
rhema export-context --ai-agent-format --include-knowledge --include-todos

# Generate context primer
rhema primer --include-examples --validate

# Generate README with context
rhema generate-readme --include-context --seo-optimized
```

## 🎯 Common Workflows

### Daily Development Workflow
```bash
# 1. Check your todos
rhema todo list --status todo

# 2. Record insights as you work
rhema insight record "This approach works better than the previous one" --confidence medium

# 3. Document decisions
rhema decision record "Use this library" --status approved --description "Better performance"

# 4. Update todo status
rhema todo update TASK-001 --status in_progress
```

### Code Review Workflow
```bash
# 1. Bootstrap context for review
rhema bootstrap-context --use-case code_review --optimize-for-ai

# 2. Check for related insights
rhema query "find insights containing 'authentication'"

# 3. Review related decisions
rhema query "find decisions containing 'API design'"

# 4. Record review insights
rhema insight record "This pattern is consistent with our architecture decisions" --confidence high
```

### Team Onboarding Workflow
```bash
# 1. Generate onboarding materials
rhema primer --include-examples --validate

# 2. Export context for new team member
rhema export-context --ai-agent-format --include-all

# 3. Generate project README
rhema generate-readme --include-context --seo-optimized
```

## 🔗 Integration with AI Tools

### Context Injection
Rhema automatically provides context to AI tools through the Model Context Protocol (MCP):

```bash
# Start MCP daemon
rhema daemon start

# Bootstrap context for AI consumption
rhema bootstrap-context --use-case code_review --optimize-for-ai
```

### AI-Optimized Export
```bash
# Export context optimized for AI
rhema export-context --ai-agent-format --include-knowledge --include-todos --include-decisions
```

## 🚨 Troubleshooting

### Common Issues

#### "No scope configuration found"
```bash
# Initialize the scope
rhema init
```

#### "Validation failed"
```bash
# Check what's wrong
rhema validate --verbose

# Fix issues and revalidate
rhema validate
```

#### "Query returned no results"
```bash
# Check if you have data
rhema stats

# Try a broader query
rhema query "find all todos"
```

### Getting Help
```bash
# Get help for any command
rhema --help
rhema todo --help
rhema query --help

# Check Rhema version
rhema --version

# Validate your setup
rhema health
```

## 📚 Next Steps

### 1. Explore Advanced Features
- **[Interactive Mode](../user-guide/interactive-mode.md)** - Use Rhema's interactive interface
- **[Batch Operations](../user-guide/batch-operations.md)** - Perform operations on multiple items
- **[Performance Monitoring](../user-guide/performance-monitoring.md)** - Monitor and optimize performance

### 2. Learn About Core Features
- **[Scope Management](../core-features/scope-management.md)** - Organize projects into scopes
- **[Context Query Language](../core-features/context-query-language.md)** - Master CQL for powerful queries
- **[Validation System](../core-features/validation-system.md)** - Ensure data integrity

### 3. Check Out Examples
- **[Quick Start Commands](../examples/quick-start-commands.md)** - Essential commands for beginners
- **[Advanced Usage](../examples/advanced-usage.md)** - Complex usage scenarios
- **[CQL Queries](../examples/cql-queries.md)** - Query language examples

### 4. Join the Community
- **GitHub Discussions**: Ask questions and share experiences
- **Issues**: Report bugs or request features
- **Contributing**: Help improve Rhema

## 🎉 Congratulations!

You've successfully set up Rhema and started capturing your project knowledge! As you continue using Rhema, you'll discover how it transforms your development workflow by making implicit knowledge explicit and persistent.

**Remember:**
- Start small and build up your knowledge base gradually
- Record insights as you discover them
- Document decisions as you make them
- Use queries to find relevant information when you need it
- Share knowledge with your team

Happy knowledge management! 🚀 