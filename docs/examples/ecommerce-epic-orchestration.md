# E-commerce Platform Epic Orchestration

This example demonstrates how to use Rhema for complex project coordination across multiple services and teams in an e-commerce platform.

## 🎯 Scenario Overview

An e-commerce platform consists of multiple microservices and teams:
- **User Service** - Authentication and user management
- **Payment Service** - Payment processing and billing
- **Inventory Service** - Product catalog and inventory management
- **Frontend App** - Customer-facing web application
- **Admin Dashboard** - Internal management interface

## 🏗️ Project-Level Scope Configuration

### Main Application Scope
```yaml
# .rhema/rhema.yaml (Project-level scope)
rhema:
  version: "1.0.0"
  scope:
    type: "application"
    name: "ecommerce-platform"
    description: "Complete e-commerce platform with user auth, payments, and inventory"
    boundaries:
      includes: ["**/*"]
    dependencies:
      children: 
        - "../user-service"
        - "../payment-service" 
        - "../inventory-service"
        - "../frontend-app"
        - "../admin-dashboard"
    responsibilities:
      - "Cross-service coordination"
      - "Epic-level planning"
      - "Integration testing"
      - "Deployment orchestration"
    epics:
      user_authentication:
        description: "Complete user authentication and authorization system"
        scopes: ["user-service", "frontend-app", "admin-dashboard"]
        dependencies: ["shared-auth-library"]
        status: "in_progress"
        completion: 65
      payment_processing:
        description: "Secure payment processing with multiple providers"
        scopes: ["payment-service", "frontend-app", "user-service"]
        dependencies: ["user-authentication"]
        status: "planning"
        completion: 15
      inventory_management:
        description: "Real-time inventory tracking and management"
        scopes: ["inventory-service", "admin-dashboard", "frontend-app"]
        dependencies: ["user-authentication"]
        status: "not_started"
        completion: 0
```

## 🔄 Cross-Scope Epic Coordination

### Track Epic Progress
```bash
# Track epic progress across all affected scopes
rhema query "*/todos WHERE epic='user-authentication' GROUP BY status"

# Monitor integration points between services
rhema query "*/knowledge WHERE category='integration' AND related_epic='user-authentication'"

# Track shared patterns across epic scopes
rhema query "*/patterns WHERE epic='user-authentication' AND usage='required'"

# Monitor cross-scope decisions
rhema query "*/decisions WHERE affects_epic='user-authentication'"

# Track completion metrics
rhema query "*/todos WHERE epic='user-authentication' AND status='completed'"
```

### Expected Query Results

#### Epic Progress Tracking
```bash
$ rhema query "*/todos WHERE epic='user-authentication' GROUP BY status"

📊 Epic Progress: user-authentication
────────────────────────────────────────────────────────────────────────────────
✅ Completed: 15 todos
🔄 In Progress: 8 todos
⏳ Pending: 12 todos
❌ Blocked: 3 todos

📈 Completion Rate: 65%
```

#### Integration Knowledge
```bash
$ rhema query "*/knowledge WHERE category='integration' AND related_epic='user-authentication'"

knowledge:
  - finding: "JWT token validation requires shared secret across services"
    impact: "All services must use same JWT configuration"
    solution: "Centralized JWT configuration in shared library"
    confidence: "high"
    related_epic: "user-authentication"
    affected_services: ["user-service", "payment-service", "inventory-service"]
```

## 🎯 Service-Specific Context

### User Service Scope
```yaml
# user-service/.rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "user-service"
    description: "User authentication and management service"
    epics:
      - "user-authentication"
    dependencies:
      parents: ["../ecommerce-platform"]
      children: ["../shared-auth-library"]
```

### Payment Service Scope
```yaml
# payment-service/.rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "payment-service"
    description: "Payment processing and billing service"
    epics:
      - "payment-processing"
    dependencies:
      parents: ["../ecommerce-platform"]
      children: ["../user-service"]
```

## 📊 Epic Management Workflows

### Epic Planning
```bash
# Review all epics and their dependencies
rhema query "*/epics ORDER BY completion DESC"

# Check for blocked epics
rhema query "*/epics WHERE status='blocked'"

# Find epics ready to start
rhema query "*/epics WHERE status='planning' AND dependencies_met=true"
```

### Dependency Management
```bash
# Find todos blocked by dependencies
rhema query "*/todos WHERE status='blocked' AND depends_on IS NOT NULL"

# Track cross-service dependencies
rhema query "*/dependencies WHERE type='service' AND status='blocked'"

# Monitor integration dependencies
rhema query "*/knowledge WHERE category='integration' AND status='pending'"
```

### Progress Monitoring
```bash
# Daily progress check
rhema query "*/todos WHERE epic='user-authentication' AND updated_at > '2024-01-15'"

# Weekly completion report
rhema query "*/todos WHERE epic='user-authentication' AND status='completed' AND completed_at > '2024-01-08'"

# Epic health check
rhema query "*/epics WHERE completion < 50 AND status='in_progress'"
```

## 🔧 Integration Patterns

### Shared Authentication Library
```yaml
# shared-auth-library/.rhema/patterns.yaml
patterns:
  - name: "JWT Authentication"
    type: "security"
    usage: "required"
    epic: "user-authentication"
    description: "Standard JWT authentication pattern for all services"
    implementation:
      - service: "user-service"
        responsibility: "Token generation and validation"
      - service: "payment-service"
        responsibility: "Token validation only"
      - service: "inventory-service"
        responsibility: "Token validation only"
```

### Cross-Service Communication
```yaml
# ecommerce-platform/.rhema/patterns.yaml
patterns:
  - name: "Service-to-Service Communication"
    type: "integration"
    usage: "required"
    description: "Standard pattern for inter-service communication"
    implementation:
      protocol: "HTTP/REST"
      authentication: "JWT"
      error_handling: "Circuit breaker pattern"
      monitoring: "Distributed tracing"
```

## 📈 Benefits of Rhema Project Orchestration

### Unified Context
- **Consistent understanding** across all teams and services
- **Shared architectural decisions** and patterns
- **Centralized knowledge** about integration points
- **Coordinated development** across multiple scopes

### Dependency Tracking
- **Visualize complex dependencies** between services and epics
- **Identify blockers early** through dependency analysis
- **Manage integration points** systematically
- **Track cross-service impacts** of changes

### Progress Monitoring
- **Real-time progress tracking** across all epics and services
- **Completion metrics** for individual services and overall platform
- **Velocity tracking** across teams and epics
- **Risk identification** through progress analysis

### Knowledge Sharing
- **Architectural decisions** shared across all teams
- **Integration patterns** documented and enforced
- **Performance insights** shared across services
- **Best practices** propagated across the organization

### Risk Management
- **Early blocker identification** through dependency tracking
- **Cross-scope impact analysis** for architectural changes
- **Integration risk assessment** before implementation
- **Progress risk monitoring** for delayed epics

### AI Agent Coordination
- **Consistent AI recommendations** across all services
- **Context-aware suggestions** based on epic relationships
- **Cross-service impact analysis** by AI agents
- **Coordinated AI assistance** across multiple teams

### Scalable Coordination
- **Manage large projects** without losing context
- **Scale team coordination** as the project grows
- **Maintain consistency** across multiple services
- **Preserve knowledge** as teams change

## 🚀 Implementation Steps

1. **Set up project-level scope** - Create the main ecommerce-platform scope
2. **Configure service scopes** - Set up individual service scopes with dependencies
3. **Define epics** - Create epic definitions with scope assignments
4. **Track dependencies** - Document cross-service and epic dependencies
5. **Monitor progress** - Use queries to track epic and service progress
6. **Share knowledge** - Document integration patterns and decisions
7. **Coordinate teams** - Use Rhema for cross-team communication and alignment

## 🔗 Related Examples

- [Advanced Usage](advanced-usage.md) - Cross-scope coordination patterns
- [CQL Queries](cql-queries.md) - Query language for epic tracking
- [Implicit to Explicit Knowledge](implicit-to-explicit-knowledge.md) - Knowledge transformation
- [Quick Start Commands](quick-start-commands.md) - Basic Rhema usage 