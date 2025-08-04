# Scope Management

Scope management is the foundation of Rhema's knowledge organization system. Scopes provide a logical way to organize projects, services, and components while maintaining relationships between them.

## 🎯 Overview

A scope in Rhema represents a logical unit of work - typically a service, application, library, or component. Each scope contains its own context files (todos, insights, decisions, patterns, etc.) and can reference other scopes to establish relationships.

## 🏗️ Scope Structure

### Basic Scope Configuration
```yaml
# scope.yaml
name: "user-service"
type: "service"
description: "User authentication and management service"
version: "1.0.0"

# Scope metadata
metadata:
  team: "platform"
  repository: "github.com/company/user-service"
  language: "rust"
  framework: "actix-web"

# Dependencies on other scopes
dependencies:
  - name: "auth-library"
    version: "2.1.0"
    type: "library"
  
  - name: "database-service"
    version: "1.5.0"
    type: "service"

# Scope-specific configuration
config:
  validation:
    strict: true
    recursive: true
  
  ai_integration:
    enabled: true
    context_injection: true
```

### Scope Types
Rhema supports several predefined scope types:

- **`service`**: Backend services and APIs
- **`app`**: Frontend applications and UIs
- **`library`**: Reusable code libraries
- **`component`**: Individual components or modules
- **`infrastructure`**: Infrastructure and deployment configurations
- **`documentation`**: Documentation projects
- **`tool`**: Development tools and utilities

## 🚀 Creating and Managing Scopes

### Initialize a New Scope
```bash
# Basic initialization
rhema init

# Initialize with specific type and name
rhema init --scope-type service --scope-name user-api

# Auto-configure from existing project
rhema init --auto-config
```

### List All Scopes
```bash
# List all scopes in the repository
rhema scopes

# Show detailed scope information
rhema scope

# Show specific scope
rhema scope ./services/auth
```

### View Scope Hierarchy
```bash
# Display scope tree
rhema tree
```

**Example Output:**
```
📁 rhema-project/
├── 📁 services/
│   ├── 🔧 user-service (service)
│   ├── 🔧 auth-service (service)
│   └── 🔧 payment-service (service)
├── 📁 libraries/
│   ├── 📚 auth-library (library)
│   └── 📚 common-utils (library)
├── 📁 apps/
│   ├── 🖥️ web-app (app)
│   └── 📱 mobile-app (app)
└── 📁 infrastructure/
    └── 🏗️ deployment (infrastructure)
```

## 🔗 Scope Relationships

### Dependencies
Scopes can depend on other scopes, creating a dependency graph:

```yaml
# In user-service/scope.yaml
dependencies:
  - name: "auth-library"
    version: "2.1.0"
    type: "library"
    relationship: "uses"
  
  - name: "database-service"
    version: "1.5.0"
    type: "service"
    relationship: "consumes"
```

### Analyzing Dependencies
```bash
# Show scope dependencies
rhema dependencies

# Visualize dependency graph
rhema dependencies --visualize

# Check for conflicts
rhema dependencies --conflicts

# Show impact analysis
rhema dependencies --impact
```

### Impact Analysis
```bash
# Check what would be affected by changes
rhema impact src/auth/service.rs
```

## 📁 Scope File Organization

### Standard Scope Structure
```
scope-name/
├── scope.yaml              # Scope configuration
├── todos.yaml              # Todo items
├── insights.yaml           # Knowledge insights
├── decisions.yaml          # Architectural decisions
├── patterns.yaml           # Design patterns
├── conventions.yaml        # Coding conventions
├── knowledge.yaml          # General knowledge base
└── .rhema/                 # Rhema metadata (optional)
    ├── cache/              # Local cache
    └── config/             # Scope-specific config
```

### Custom File Organization
You can customize the file organization within a scope:

```yaml
# scope.yaml
file_organization:
  todos: "tasks/work-items.yaml"
  insights: "knowledge/learnings.yaml"
  decisions: "architecture/decisions.yaml"
  patterns: "design/patterns.yaml"
```

## 🔍 Scope Discovery and Navigation

### Finding Scopes
```bash
# Search for scopes by name
rhema scopes | grep "auth"

# Find scopes by type
rhema query "find scopes where type = service"

# Find scopes by team
rhema query "find scopes where metadata.team = platform"
```

### Cross-Scope Queries
```bash
# Query across all scopes
rhema query "find all todos with priority high"

# Query specific scope types
rhema query "find insights in services containing 'performance'"

# Query with scope filtering
rhema query "find decisions where status = approved" --scope "services/*"
```

## 🎯 Scope Best Practices

### Naming Conventions
- Use descriptive, lowercase names with hyphens
- Include the scope type in the name when helpful
- Be consistent across your organization

**Good Examples:**
- `user-authentication-service`
- `payment-processing-library`
- `mobile-app-ui-components`

**Avoid:**
- Generic names like `service1`, `app2`
- Inconsistent naming patterns
- Names that don't reflect the scope's purpose

### Scope Granularity
- **Too Fine**: Each function as a separate scope
- **Too Coarse**: Entire monorepo as one scope
- **Just Right**: Logical units that can be developed independently

### Dependency Management
- Keep dependencies minimal and explicit
- Use version constraints for stability
- Document the nature of relationships
- Regularly review and update dependencies

### Configuration Management
```yaml
# scope.yaml
config:
  # Validation settings
  validation:
    strict: true
    recursive: true
    auto_fix: false
  
  # AI integration settings
  ai_integration:
    enabled: true
    context_injection: true
    prompt_optimization: true
  
  # Performance settings
  performance:
    cache_enabled: true
    cache_ttl: 3600
    query_optimization: true
```

## 🔧 Advanced Scope Features

### Scope Templates
```bash
# Generate scope template
rhema schema scope --output-file scope-template.yaml
```

### Scope Migration
```bash
# Migrate scope to new schema version
rhema migrate --recursive

# Dry run migration
rhema migrate --dry-run
```

### Scope Health Checks
```bash
# Check scope health
rhema health

# Check specific scope
rhema health --scope ./services/auth

# Detailed health report
rhema health --verbose
```

### Scope Statistics
```bash
# Show scope statistics
rhema stats

# Scope-specific stats
rhema stats --scope ./services/auth
```

## 🚨 Common Issues and Solutions

### Missing Scope Configuration
**Problem**: `Error: No scope configuration found`
**Solution**: Initialize the scope with `rhema init`

### Circular Dependencies
**Problem**: Circular dependency detected
**Solution**: Review and refactor scope relationships

### Invalid Scope Type
**Problem**: `Error: Invalid scope type 'invalid-type'`
**Solution**: Use one of the supported scope types

### Scope Not Found
**Problem**: `Error: Scope 'missing-scope' not found`
**Solution**: Check scope name and path, use `rhema scopes` to list available scopes

## 📚 Related Documentation

- **[CLI Command Reference](../user-guide/cli-command-reference.md)** - Complete command documentation
- **[Configuration Management](../user-guide/configuration-management.md)** - Managing scope configuration
- **[Dependency Management](./dependency-management.md)** - Managing scope relationships
- **[Validation System](./validation-system.md)** - Ensuring scope integrity
- **[Examples](../examples/)** - Practical scope management examples 