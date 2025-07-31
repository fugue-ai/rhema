# Advanced Usage Examples


Advanced Rhema patterns for complex scenarios including cross-scope coordination, batch operations, and AI integration.

## 🌐 Cross-Scope Coordination


Coordinate work across multiple scopes and teams:

```bash
# Query todos across all scopes


rhema query "*/todos WHERE status='in_progress'"

# Track dependencies across the project


rhema query "*/dependencies WHERE feature='payment-processing'"

# Monitor architectural decisions


rhema query "*/decisions WHERE impact_scope='multiple'"
```

### Use Cases


- **Multi-service projects** - Track work across microservices

- **Monorepos** - Coordinate development across packages

- **Team coordination** - Align work across different teams

- **Dependency management** - Track cross-scope dependencies

## 📦 Batch Operations


Perform operations across multiple scopes simultaneously:

```bash
# Validate all scopes in a repository


rhema batch validate

# Export context from all scopes


rhema batch export --format json

# Health check across all scopes


rhema batch health
```

### Advanced Batch Operations


```bash
# Export data from all scopes


rhema batch data export --input-path "." --output-path "./exports" --format json

# Generate comprehensive health reports


rhema batch validate health-check --output-file health-report.json --detailed

# Generate analytics reports across all scopes


rhema batch report analytics --output-file analytics-report.md --format markdown --include-details
```

### Batch Operation Benefits


- **Consistency** - Ensure all scopes follow the same standards

- **Efficiency** - Process multiple scopes in a single command

- **Reporting** - Generate comprehensive reports across the entire project

- **Validation** - Verify compliance across all scopes

## 🤖 AI Integration


Integrate Rhema with AI agents and tools:

```bash
# Start MCP daemon for AI agents


rhema daemon start

# Generate context primer for AI conversation


rhema primer generate --scope . --output primer.md
```

### MCP Daemon Features


- **Real-time context** - AI agents can query context in real-time

- **Session persistence** - Maintain context across AI interactions

- **Deterministic behavior** - Consistent AI recommendations based on explicit context

- **Context primers** - Automated context generation for AI conversations

### AI Integration Benefits


- **Session continuity** - AI agents remember past decisions and context

- **Consistent recommendations** - AI makes recommendations based on explicit knowledge

- **Team alignment** - All AI agents access the same structured context

- **Knowledge preservation** - Critical insights are preserved for AI interactions

## 🎯 Advanced Query Patterns


### Cross-Scope Epic Coordination


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

### Dependency Tracking


```bash
# Find todos that depend on specific decisions


rhema query "*/todos WHERE depends_on CONTAINS 'decision-001'"

# Track cross-service dependencies


rhema query "*/dependencies WHERE type='service' AND status='blocked'"

# Monitor architectural dependencies


rhema query "*/decisions WHERE impact CONTAINS 'multiple-services'"
```

### Performance Monitoring


```bash
# Track performance-related knowledge across scopes


rhema query "*/knowledge WHERE category='performance' ORDER BY confidence DESC"

# Find performance patterns


rhema query "*/patterns WHERE type='performance' AND usage='required'"

# Monitor performance decisions


rhema query "*/decisions WHERE category='performance' AND status='approved'"
```

## 🔧 Advanced Configuration


### Scope Dependencies


```yaml
# .rhema/rhema.yaml


rhema:
  version: "1.0.0"
  scope:
    type: "application"
    name: "main-application"
    dependencies:
      children: 

        - "../user-service"

        - "../payment-service" 

        - "../inventory-service"
      parents:

        - "../platform"
    responsibilities:

      - "Cross-service coordination"

      - "Integration testing"

      - "Deployment orchestration"
```

### Epic Management


```yaml
# .rhema/rhema.yaml


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
```

## 📊 Advanced Analytics


### Progress Tracking


```bash
# Track epic completion across scopes


rhema query "*/epics WHERE status='in_progress' ORDER BY completion DESC"

# Monitor team velocity


rhema query "*/todos WHERE status='completed' AND completed_at > '2024-01-01' GROUP BY assignee"

# Track decision impact


rhema query "*/decisions WHERE impact_scope='multiple' ORDER BY date DESC"
```

### Quality Metrics


```bash
# Assess context quality


rhema query "*/knowledge WHERE confidence='low'"

# Find incomplete decisions


rhema query "*/decisions WHERE rationale IS NULL"

# Track pattern adoption


rhema query "*/patterns WHERE usage='required' AND adoption_rate < 0.8"
```

## 🚀 Best Practices for Advanced Usage


1. **Start with Cross-Scope Queries** - Use `*/` to explore relationships between scopes

2. **Leverage Batch Operations** - Use batch commands for consistency and efficiency

3. **Integrate with AI** - Enable MCP daemon for AI agent integration

4. **Track Dependencies** - Use epic and dependency tracking for complex projects

5. **Monitor Progress** - Use analytics to track completion and quality metrics

6. **Validate Regularly** - Use batch validation to ensure compliance

## 📚 Next Steps


- **Experiment** - Try cross-scope queries in your project

- **Set up AI Integration** - Enable MCP daemon for AI agent support

- **Implement Batch Operations** - Use batch commands for efficiency

- **Track Dependencies** - Use epic management for complex projects

## 🔗 Related Examples


- [E-commerce Epic Orchestration](ecommerce-epic-orchestration.md) - Complex project coordination

- [CQL Queries](cql-queries.md) - Query language examples

- [Query Provenance](query-provenance.md) - Understanding query execution

- [Quick Start Commands](quick-start-commands.md) - Basic Rhema usage 