# Rhema Protocol Specification

**Version 1.0.0**  
**Copyright 2025 Cory Parent - Licensed under the Apache License, Version 2.0**

## Table of Contents

1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Schema Definitions](#schema-definitions)
4. [File Formats](#file-formats)
5. [Context Query Language (CQL)](#context-query-language-cql)
6. [Scope Resolution](#scope-resolution)
7. [AI Integration Standards](#ai-integration-standards)
8. [Git Integration](#git-integration)
9. [Validation Rules](#validation-rules)
10. [Extension Mechanisms](#extension-mechanisms)
11. [Versioning and Compatibility](#versioning-and-compatibility)
12. [Best Practices](#best-practices)
13. [Examples](#examples)
14. [Reference Implementation](#reference-implementation)

## Overview

The Rhema Protocol is an open specification for managing persistent, structured context in software projects. It addresses the fundamental problem of ephemeral knowledge in AI-assisted development by providing a standardized way to capture, organize, and query project context that survives across development sessions and AI conversations.

### Key Principles

- **Explicit over Implicit**: Transform tacit knowledge into structured, discoverable context
- **Git-Native**: Context files travel with code and are version-controlled
- **Hierarchical**: Scoped organization that mirrors project structure
- **Schema-Driven**: JSON Schema validation ensures consistency and interoperability
- **AI-Ready**: Designed for seamless integration with AI agents and tools
- **Extensible**: Support for custom fields and domain-specific extensions

### Problem Statement

Modern software development suffers from **context fragmentation**:

- **Session Amnesia**: AI agents lose context between conversations
- **Knowledge Silos**: Critical insights exist only in individual minds
- **Decision Decay**: Architectural choices lose their rationale over time
- **Onboarding Friction**: New team members struggle to understand project context
- **Inconsistent AI Behavior**: Different AI sessions produce conflicting recommendations

Rhema solves these problems by providing a **persistent context layer** that makes implicit knowledge explicit and discoverable.

## Core Concepts

### Scope

A **scope** represents a logical boundary in your codebase (service, application, library, component, etc.). Each scope has its own `.rhema/` directory containing specification-compliant context files.

**Scope Types:**
- `repository` - Entire repository context
- `service` - Microservices and API endpoints
- `application` - Full applications
- `library` - Reusable code libraries
- `component` - UI components or modules

### Context Files

Rhema defines six core context file types, each with a specific purpose and JSON Schema:

1. **`rhema.yaml`** - Scope definition, metadata, and dependencies
2. **`knowledge.yaml`** - Insights, learnings, and domain knowledge
3. **`todos.yaml`** - Work items, tasks, and completion history
4. **`decisions.yaml`** - Architecture decision records (ADRs)
5. **`patterns.yaml`** - Design patterns and architectural patterns
6. **`conventions.yaml`** - Coding conventions and team standards

### Context Query Language (CQL)

A simple, YAML path-based query syntax for cross-scope context retrieval that enables powerful context discovery and analysis.

## Schema Definitions

### Core Schema Structure

All Rhema files follow a common structure defined by the JSON Schema in `rhema.json`. The schema is versioned and extensible, supporting both required and optional fields.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://rhema.dev/schemas/v1/",
  "title": "Rhema Schema Collection",
  "description": "JSON Schema definitions for Rhema Protocol YAML files"
}
```

### Schema Components

#### 1. Rhema Scope Definition (`rhema_scope`)

Defines the boundaries and purpose of a Rhema context scope.

**Required Fields:**
- `rhema.version` - Protocol version (semantic versioning)
- `rhema.scope.type` - Scope type (repository, service, application, library, component)
- `rhema.scope.name` - Human-readable scope name
- `rhema.scope.description` - Detailed description of scope purpose

**Optional Fields:**
- `rhema.scope.boundaries` - File inclusion/exclusion patterns
- `rhema.scope.dependencies` - Parent, child, and peer scope relationships
- `rhema.scope.responsibilities` - List of scope responsibilities
- `rhema.scope.tech` - Technology stack information
- `rhema.tooling` - AI focus areas and recommended analysis

#### 2. System Knowledge (`knowledge`)

Accumulated understanding about system architecture and components.

**Components Section:**
- Component descriptions and key files
- Interface definitions and consumers
- Internal and external dependencies
- Known issues with severity levels
- Complexity assessments
- Analysis timestamps

**Architecture Section:**
- Design patterns and their effectiveness
- Data flow patterns and bottlenecks
- Architectural insights and recommendations

**Insights Section:**
- Findings with confidence levels
- Impact assessments and solutions
- Evidence and related files
- Validation status and applicability

#### 3. Work Items (`todos`)

Tracks current tasks, completed work, and lessons learned.

**Active Items:**
- Title, description, and priority
- Status tracking (todo, in_progress, blocked, review, done)
- Context information and related files
- Acceptance criteria and effort estimates
- Cross-scope dependencies

**Completed Items:**
- Completion dates and outcomes
- Impact assessments and lessons learned
- Knowledge updates and next actions
- Performance metrics and effort tracking

#### 4. Architecture Decisions (`decisions`)

Documents significant architectural choices and their context.

**Decision Structure:**
- Unique ID (ADR-XXX format)
- Title, date, and status
- Context and problem statement
- Options considered with pros/cons
- Decision rationale and implementation details
- Consequences and monitoring requirements

#### 5. Design Patterns (`patterns`)

Established patterns and best practices.

**Pattern Categories:**
- Architectural patterns with usage guidelines
- Performance patterns and optimization strategies
- Security patterns with implementation details
- Anti-patterns and warning signs
- Coding conventions and testing patterns

#### 6. Team Conventions (`conventions`)

Team-specific standards and workflows.

**Convention Areas:**
- Coding standards and style guides
- Git workflow and branching strategies
- Code review processes and guidelines
- Testing strategies and coverage requirements
- Deployment and monitoring standards

## File Formats

### YAML Format

All Rhema files use YAML format for human readability and ease of editing. The YAML must conform to the JSON Schema definitions.

**Example Structure:**
```yaml
# File: .rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "user-management-service"
    description: "A microservice responsible for user authentication and profile management"
    
    boundaries:
      includes:
        - "src/**"
        - "tests/**"
        - "migrations/**"
      excludes:
        - "docs/**"
        - "*.md"
        - "node_modules/**"
    
    dependencies:
      parent: "../shared"
      children:
        - "../user-api"
        - "../user-ui"
      peers:
        - "../auth-service"
        - "../notification-service"
    
    responsibilities:
      - "User registration and authentication"
      - "Password management and reset"
      - "User profile CRUD operations"
      - "Role-based access control"
    
    tech:
      primary_languages:
        - "Rust"
        - "TypeScript"
      frameworks:
        - "Actix Web"
        - "React"
        - "SQLx"
      databases:
        - "PostgreSQL"
        - "Redis"
  
  tooling:
    focus_areas:
      - "Security best practices"
      - "Performance optimization"
      - "Data privacy compliance"
    recommended_analysis:
      - "Dependency analysis"
      - "Security audit"
      - "Performance profiling"
```

### File Naming Conventions

- **Scope Definition**: `rhema.yaml` (required in each scope)
- **Knowledge**: `knowledge.yaml`
- **Work Items**: `todos.yaml`
- **Decisions**: `decisions.yaml`
- **Patterns**: `patterns.yaml`
- **Conventions**: `conventions.yaml`

### Directory Structure

```
project-root/
├── .rhema/
│   ├── rhema.yaml          # Scope definition
│   ├── knowledge.yaml      # System knowledge
│   ├── todos.yaml          # Work items
│   ├── decisions.yaml      # Architecture decisions
│   ├── patterns.yaml       # Design patterns
│   └── conventions.yaml    # Team conventions
├── service-a/
│   └── .rhema/
│       ├── rhema.yaml
│       ├── knowledge.yaml
│       └── ...
└── service-b/
    └── .rhema/
        ├── rhema.yaml
        ├── knowledge.yaml
        └── ...
```

## Context Query Language (CQL)

### Overview

CQL is a simple, YAML path-based query language designed for cross-scope context retrieval. It provides a unified way to query context data across multiple scopes and repositories.

### Basic Syntax

**Query Format:**
```
[scope_pattern]/[file_type].[field_path] [WHERE condition]
```

**Examples:**
```bash
# Query all todos in current scope
rhema query "todos"

# Query todos across all scopes
rhema query "*/todos"

# Query specific scope
rhema query "user-service/todos"

# Query with field path
rhema query "knowledge.components.auth_service"

# Query with conditions
rhema query "todos WHERE priority='high'"

# Complex queries
rhema query "*/todos WHERE status='in_progress' AND priority='high'"

# Cross-file queries
rhema query "todos,decisions WHERE tags CONTAINS 'security'"
```

### Query Components

#### Scope Patterns
- `*` - All scopes (recursive)
- `**` - All scopes (non-recursive)
- `scope-name` - Specific scope
- `scope-name/*` - Scope and all children
- `../scope-name` - Parent scope reference

#### File Types
- `rhema` - Scope definitions
- `knowledge` - System knowledge
- `todos` - Work items
- `decisions` - Architecture decisions
- `patterns` - Design patterns
- `conventions` - Team conventions

#### Field Paths
- Dot notation for nested fields
- Array indexing with `[n]`
- Wildcard matching with `*`

#### Conditions
- Equality: `field='value'`
- Inequality: `field!='value'`
- Contains: `field CONTAINS 'value'`
- Regex: `field MATCHES 'pattern'`
- Comparison: `field > value`, `field < value`
- Logical operators: `AND`, `OR`, `NOT`

### Advanced Features

#### Aggregation Queries
```bash
# Count todos by status
rhema query "todos GROUP BY status COUNT"

# Average completion time
rhema query "todos.completed GROUP BY priority AVG(completion_time)"
```

#### Cross-Scope Analysis
```bash
# Find dependencies across scopes
rhema query "*/rhema.scope.dependencies WHERE type='required'"

# Track decisions affecting multiple scopes
rhema query "*/decisions WHERE impact_scope='multiple'"
```

#### Temporal Queries
```bash
# Recent decisions
rhema query "decisions WHERE date > '2024-01-01'"

# Active work items
rhema query "todos WHERE created > '2024-01-01' AND status != 'done'"
```

## Scope Resolution

### Resolution Process

1. **Discovery**: Scan for `.rhema/` directories
2. **Validation**: Verify schema compliance
3. **Dependency Resolution**: Build scope dependency graph
4. **Context Merging**: Merge context from parent scopes
5. **Conflict Resolution**: Handle conflicting definitions

### Dependency Types

#### Parent-Child Relationships
- **Inheritance**: Child scopes inherit context from parents
- **Override**: Child scopes can override parent definitions
- **Extension**: Child scopes extend parent capabilities

#### Peer Relationships
- **Collaboration**: Scopes that work together
- **Dependencies**: Required relationships between scopes
- **Conflicts**: Competing or conflicting scope definitions

### Context Inheritance

Child scopes automatically inherit context from their parent scopes:

```yaml
# Parent scope (.rhema/conventions.yaml)
conventions:
  coding_standards:
    rust:
      style_guide: "rustfmt"
      line_length: 100

# Child scope inherits and can override
conventions:
  coding_standards:
    rust:
      line_length: 120  # Override parent
    typescript:
      style_guide: "prettier"  # Add new standard
```

### Conflict Resolution

When conflicts occur between scopes, the following precedence applies:

1. **Child over Parent**: Child scope definitions take precedence
2. **Explicit over Inherited**: Explicit definitions override inherited ones
3. **Recent over Old**: More recent definitions take precedence
4. **Specific over General**: Specific scope definitions override general ones

## AI Integration Standards

### MCP (Model Context Protocol) Integration

Rhema provides a standardized MCP daemon for AI agent integration:

#### Protocol Version
- **MCP Version**: 1.0.0
- **JSON-RPC**: 2.0 specification
- **Transport**: Standard input/output or WebSocket

#### Resource Types
- `rhema://scope/{scope_name}/file/{file_type}` - Context file resources
- `rhema://scope/{scope_name}/query/{query}` - Query results
- `rhema://scope/{scope_name}/export/{format}` - Exported context

#### Methods
- `rhema/listScopes` - List available scopes
- `rhema/getContext` - Retrieve context data
- `rhema/query` - Execute CQL queries
- `rhema/update` - Update context data
- `rhema/validate` - Validate context files

### Context Export Formats

#### JSON Export
```json
{
  "scope": "user-service",
  "version": "1.0.0",
  "context": {
    "rhema": { ... },
    "knowledge": { ... },
    "todos": { ... },
    "decisions": { ... }
  }
}
```

#### Markdown Export
```markdown
# User Service Context

## Scope Information
- **Type**: Service
- **Name**: user-management-service
- **Description**: User authentication and profile management

## Active Work Items
- [ ] Implement OAuth2 integration (High Priority)
- [ ] Add rate limiting (Medium Priority)

## Architecture Decisions
- **ADR-001**: Use PostgreSQL for user data storage
- **ADR-002**: Implement JWT for authentication
```

#### YAML Export
```yaml
scope: user-service
version: "1.0.0"
context:
  rhema:
    version: "1.0.0"
    scope:
      type: service
      name: user-management-service
  knowledge:
    components:
      auth_service:
        description: "Authentication service component"
  todos:
    active:
      oauth_integration:
        title: "Implement OAuth2 integration"
        priority: high
```

### AI Context Bootstrapping

#### Context Primer Generation
Automated generation of comprehensive context primers for AI conversations:

```yaml
# Generated context primer
context_primer:
  scope: user-service
  generated: "2024-01-15T10:30:00Z"
  summary:
    total_todos: 15
    active_decisions: 8
    key_insights: 12
  critical_context:
    - "PostgreSQL chosen for ACID compliance"
    - "JWT authentication implemented"
    - "Rate limiting required for production"
  active_work:
    - "OAuth2 integration in progress"
    - "Performance optimization needed"
  recent_decisions:
    - "ADR-003: Use Redis for session storage"
    - "ADR-004: Implement API versioning"
```

#### Session Persistence
Maintain context across AI interactions:

```yaml
session_context:
  session_id: "session-12345"
  scope: user-service
  start_time: "2024-01-15T10:30:00Z"
  context_snapshot:
    todos_count: 15
    decisions_count: 8
    knowledge_entries: 25
  interaction_history:
    - query: "Show me high priority todos"
      result_count: 3
      timestamp: "2024-01-15T10:31:00Z"
    - query: "What are the recent architecture decisions?"
      result_count: 5
      timestamp: "2024-01-15T10:32:00Z"
```

## Git Integration

### Version Control Strategy

#### File Tracking
- All `.rhema/` files are tracked in Git
- Context changes are committed alongside code changes
- Branch-specific context is supported

#### Merge Strategies
- **Context-Aware Merging**: Merge context files with conflict resolution
- **Branch Context**: Maintain separate context for feature branches
- **Release Context**: Prepare context for release branches

#### Git Hooks
- **Pre-commit**: Validate context file schemas
- **Post-merge**: Update context inheritance
- **Pre-push**: Check for context conflicts

### Branch Context Management

#### Feature Branch Context
```yaml
# Feature branch context
branch_context:
  branch: feature/oauth-integration
  base_branch: main
  context_changes:
    todos:
      added:
        - "Implement OAuth2 provider integration"
        - "Add OAuth2 configuration management"
      modified:
        - "Update authentication service design"
    decisions:
      added:
        - "ADR-005: Use Auth0 as OAuth2 provider"
```

#### Release Context
```yaml
# Release context preparation
release_context:
  version: "1.2.0"
  scope: user-service
  context_validation:
    todos_completed: 8
    decisions_approved: 3
    breaking_changes: 0
  release_notes:
    - "OAuth2 integration completed"
    - "Performance improvements implemented"
    - "Security enhancements added"
```

## Validation Rules

### Schema Validation

#### Required Fields
- All required fields must be present
- Field types must match schema definitions
- Enum values must be valid

#### Format Validation
- Version strings must follow semantic versioning
- Dates must be in ISO 8601 format
- URIs must be valid according to RFC 3986

#### Cross-Reference Validation
- Scope dependencies must reference valid scopes
- File references must exist
- Decision IDs must be unique

### Business Rule Validation

#### Consistency Checks
- Todo priorities must be valid enum values
- Decision statuses must follow workflow
- Component dependencies must be acyclic

#### Completeness Checks
- High-priority todos must have acceptance criteria
- Architecture decisions must have rationale
- Knowledge insights must have confidence levels

### Custom Validation

#### Extension Validation
- Custom fields must not conflict with standard fields
- Extension schemas must be valid JSON Schema
- Custom validation rules must be documented

## Extension Mechanisms

### Custom Fields

Rhema supports custom fields through the `additionalProperties` schema feature:

```yaml
# Custom fields in rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "user-service"
  # Custom fields
  custom_metadata:
    team: "backend"
    sprint: "Q1-2024"
    jira_project: "USER"
  security_requirements:
    compliance: ["GDPR", "SOC2"]
    audit_frequency: "monthly"
```

### Custom Schemas

Organizations can define custom schemas for domain-specific context:

```json
{
  "$id": "https://company.com/schemas/rhema-extensions/v1",
  "title": "Company Rhema Extensions",
  "description": "Custom extensions for company-specific context",
  "type": "object",
  "properties": {
    "security_context": {
      "type": "object",
      "properties": {
        "compliance_requirements": {
          "type": "array",
          "items": {"type": "string"}
        },
        "security_contacts": {
          "type": "array",
          "items": {"type": "string"}
        }
      }
    }
  }
}
```

### Plugin System

#### Validation Plugins
Custom validation rules for organization-specific requirements:

```rust
pub trait ValidationPlugin {
    fn validate(&self, context: &Context) -> ValidationResult;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}
```

#### Query Extensions
Custom CQL functions for domain-specific queries:

```rust
pub trait QueryExtension {
    fn execute(&self, args: &[Value]) -> QueryResult;
    fn name(&self) -> &str;
    fn signature(&self) -> &str;
}
```

## Versioning and Compatibility

### Semantic Versioning

Rhema follows semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes to schema or protocol
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Schema Evolution

#### Backward Compatibility
- New fields are optional
- Existing fields maintain their types
- Deprecated fields are marked but not removed

#### Migration Support
- Automatic migration between compatible versions
- Manual migration for breaking changes
- Migration scripts and documentation

### Protocol Versioning

#### Version Detection
```yaml
rhema:
  version: "1.0.0"  # Protocol version
  schema_version: "1.0.0"  # Schema version
```

#### Compatibility Matrix
| Protocol Version | Schema Version | Status |
|------------------|----------------|--------|
| 1.0.0 | 1.0.0 | Current |
| 0.9.0 | 0.9.0 | Deprecated |
| 0.8.0 | 0.8.0 | Unsupported |

## Best Practices

### Scope Organization

#### Scope Granularity
- **Too Fine**: Avoid scopes for individual files
- **Too Coarse**: Avoid single scope for entire monorepo
- **Just Right**: Scope by logical component or service

#### Naming Conventions
- Use kebab-case for scope names
- Include type in name: `user-service`, `auth-library`
- Be descriptive but concise

### Context Management

#### Content Guidelines
- **Be Specific**: Avoid vague descriptions
- **Include Rationale**: Explain why decisions were made
- **Link Related Items**: Use cross-references between files
- **Keep Current**: Update context as code evolves

#### Maintenance
- **Regular Reviews**: Schedule context review sessions
- **Automated Validation**: Use CI/CD for schema validation
- **Team Training**: Ensure team understands context importance

### Performance Considerations

#### Query Optimization
- Use specific scope patterns when possible
- Limit result sets with conditions
- Cache frequently accessed context

#### File Size Management
- Split large context files into logical sections
- Archive completed work items
- Use references to avoid duplication

### Security and Privacy

#### Sensitive Information
- Never store secrets in context files
- Use environment variables for sensitive data
- Implement access controls for context files

#### Audit Trail
- Track context changes in Git history
- Maintain change logs for critical decisions
- Implement approval workflows for sensitive changes

## Examples

### Complete Scope Example

```yaml
# .rhema/rhema.yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "payment-processing-service"
    description: "Handles payment processing, fraud detection, and transaction management"
    
    boundaries:
      includes:
        - "src/**"
        - "tests/**"
        - "migrations/**"
        - "config/**"
      excludes:
        - "docs/**"
        - "*.md"
        - "node_modules/**"
        - "target/**"
    
    dependencies:
      parent: "../shared"
      children:
        - "../payment-api"
        - "../payment-ui"
      peers:
        - "../user-service"
        - "../order-service"
        - "../fraud-detection-service"
    
    responsibilities:
      - "Payment processing and validation"
      - "Fraud detection and prevention"
      - "Transaction management and reconciliation"
      - "Payment method management"
      - "Compliance and audit reporting"
    
    tech:
      primary_languages:
        - "Rust"
        - "TypeScript"
      frameworks:
        - "Actix Web"
        - "React"
        - "SQLx"
      databases:
        - "PostgreSQL"
        - "Redis"
      external_services:
        - "Stripe API"
        - "Fraud detection service"
        - "Audit logging service"
  
  tooling:
    focus_areas:
      - "Security and compliance"
      - "Performance and scalability"
      - "Fraud prevention"
      - "Audit trail management"
    recommended_analysis:
      - "Security audit"
      - "Performance profiling"
      - "Compliance review"
      - "Fraud detection accuracy"
```

### Knowledge Example

```yaml
# .rhema/knowledge.yaml
components:
  payment_processor:
    description: "Core payment processing engine"
    key_files:
      - "src/payment/processor.rs"
      - "src/payment/validation.rs"
    interfaces:
      payment_api:
        endpoints:
          - path: "/api/v1/payments"
            method: "POST"
            purpose: "Process new payment"
          - path: "/api/v1/payments/{id}"
            method: "GET"
            purpose: "Get payment status"
        consumers:
          - "payment-ui"
          - "order-service"
    dependencies:
      internal:
        - "validation_service"
        - "fraud_detection"
      external:
        - "stripe_api"
        - "audit_service"
    known_issues:
      - issue: "Race condition in concurrent payment processing"
        severity: "high"
        impact: "Potential duplicate charges"
        workaround: "Use database-level locking"
        tracked_in: "JIRA-1234"
        discovered: "2024-01-10"
    complexity: "high"
    last_analyzed: "2024-01-15"

architecture:
  patterns:
    event_sourcing:
      usage: "required"
      effectiveness: "high"
      description: "Use event sourcing for payment transaction history"
      examples:
        - "PaymentCreated event"
        - "PaymentProcessed event"
        - "PaymentFailed event"
      when_to_use: "When audit trail is critical"
      benefits:
        - "Complete audit trail"
        - "Temporal queries"
        - "Event replay capability"
    saga_pattern:
      usage: "recommended"
      effectiveness: "high"
      description: "Use saga pattern for distributed payment workflows"
      examples:
        - "Payment processing saga"
        - "Refund processing saga"
      when_to_use: "For complex multi-step payment workflows"

insights:
  performance:
    - finding: "Database connection pooling improves performance by 40%"
      impact: "Reduced payment processing latency"
      solution: "Implement connection pooling with max 20 connections"
      confidence: "high"
      evidence:
        - "Load test results"
        - "Production metrics"
      related_files:
        - "src/database/pool.rs"
        - "config/database.yml"
      validated: "2024-01-12"
  security:
    - finding: "Input validation prevents 95% of injection attacks"
      impact: "Significant security improvement"
      solution: "Implement strict input validation on all payment endpoints"
      confidence: "high"
      evidence:
        - "Security audit results"
        - "Penetration testing"
      related_files:
        - "src/validation/payment.rs"
        - "tests/security/injection.rs"
```

### Todos Example

```yaml
# .rhema/todos.yaml
active:
  stripe_integration:
    title: "Integrate Stripe payment processing"
    description: "Replace legacy payment processor with Stripe API"
    priority: "high"
    status: "in_progress"
    created: "2024-01-10"
    context:
      related_files:
        - "src/payment/stripe.rs"
        - "config/stripe.yml"
        - "tests/payment/stripe_test.rs"
      related_components:
        - "payment_processor"
        - "validation_service"
      cross_scope_dependencies:
        - scope: "../user-service"
          reason: "User authentication required for payments"
          blocked_since: "2024-01-12"
    acceptance_criteria:
      - "Stripe API integration implemented"
      - "Payment processing tests passing"
      - "Error handling and retry logic implemented"
      - "Security audit completed"
    estimated_effort: "2 weeks"
    tags:
      - "payment"
      - "integration"
      - "security"

  fraud_detection:
    title: "Implement fraud detection system"
    description: "Add machine learning-based fraud detection"
    priority: "medium"
    status: "todo"
    created: "2024-01-15"
    context:
      related_files:
        - "src/fraud/detection.rs"
        - "models/fraud_detection.py"
    acceptance_criteria:
      - "ML model trained and deployed"
      - "Real-time fraud scoring implemented"
      - "False positive rate < 1%"
      - "Integration with payment processor"
    estimated_effort: "3 weeks"
    tags:
      - "fraud"
      - "ml"
      - "security"

completed:
  payment_validation:
    title: "Implement payment validation"
    description: "Add comprehensive payment validation rules"
    completed: "2024-01-08"
    outcome: "Payment validation system successfully implemented"
    impact:
      - "Reduced invalid payment attempts by 80%"
      - "Improved user experience with better error messages"
      - "Enhanced security through input validation"
    lessons_learned:
      - "Early validation prevents downstream issues"
      - "User-friendly error messages are crucial"
      - "Comprehensive test coverage catches edge cases"
    knowledge_updated:
      - "Payment validation patterns"
      - "Error handling best practices"
    effort_actual: "1 week"
    tags:
      - "validation"
      - "security"
    metrics_established:
      - endpoint: "/api/v1/payments"
        p95_response_time: "150ms"
        p99_response_time: "300ms"
        requests_per_second: "1000"
    next_actions:
      - "Monitor validation performance in production"
      - "Collect user feedback on error messages"
```

### Decisions Example

```yaml
# .rhema/decisions.yaml
decisions:
  stripe_adoption:
    id: "ADR-001"
    title: "Adopt Stripe for payment processing"
    date: "2024-01-05"
    status: "accepted"
    context: "Need to replace legacy payment processor with modern, reliable solution"
    options_considered:
      stripe:
        pros:
          - "Excellent developer experience"
          - "Comprehensive documentation"
          - "Strong security and compliance"
          - "Wide payment method support"
        cons:
          - "Higher transaction fees"
          - "Vendor lock-in"
        example: "Stripe API integration"
      paypal:
        pros:
          - "Widely recognized"
          - "Good international support"
        cons:
          - "Complex API"
          - "Limited developer tools"
        example: "PayPal REST API"
      square:
        pros:
          - "Good for in-person payments"
          - "Competitive pricing"
        cons:
          - "Limited online payment features"
          - "Smaller developer community"
        example: "Square API"
    decision: "Adopt Stripe for payment processing due to superior developer experience and comprehensive feature set"
    implementation:
      - "Integrate Stripe API in payment processor"
      - "Update payment validation rules"
      - "Implement webhook handling"
      - "Add Stripe-specific error handling"
    consequences:
      positive:
        - "Faster development velocity"
        - "Better payment method support"
        - "Improved security and compliance"
      negative:
        - "Increased transaction costs"
        - "Dependency on Stripe platform"
      mitigations:
        - "Negotiate volume discounts with Stripe"
        - "Implement abstraction layer for future flexibility"
    monitoring:
      - "Track Stripe API performance"
      - "Monitor transaction success rates"
      - "Watch for Stripe service outages"
    tags:
      - "payment"
      - "integration"
      - "vendor"

  event_sourcing:
    id: "ADR-002"
    title: "Implement event sourcing for payment transactions"
    date: "2024-01-10"
    status: "accepted"
    context: "Need complete audit trail for payment transactions for compliance and debugging"
    options_considered:
      event_sourcing:
        pros:
          - "Complete audit trail"
          - "Temporal queries"
          - "Event replay capability"
          - "Natural fit for payment workflows"
        cons:
          - "Increased complexity"
          - "Learning curve for team"
        example: "PaymentCreated, PaymentProcessed events"
      traditional_crud:
        pros:
          - "Simpler implementation"
          - "Familiar to team"
        cons:
          - "Limited audit trail"
          - "Difficult to track state changes"
        example: "Payment table with status field"
    decision: "Implement event sourcing for payment transactions to ensure complete audit trail and enable advanced querying"
    implementation:
      layers:
        events:
          purpose: "Store payment events"
          examples:
            - "PaymentCreated"
            - "PaymentProcessed"
            - "PaymentFailed"
          ttl: "7 years"
          size_limit: "1GB per event"
          eviction: "LRU"
        projections:
          purpose: "Build current state views"
          examples:
            - "PaymentStatus"
            - "UserPaymentHistory"
          ttl: "1 year"
          size_limit: "100MB per projection"
          eviction: "TTL"
      invalidation:
        strategy: "Event-driven"
        triggers:
          - "New payment event"
          - "Payment status change"
    consequences:
      positive:
        - "Complete audit trail for compliance"
        - "Ability to replay payment workflows"
        - "Temporal analysis capabilities"
      negative:
        - "Increased system complexity"
        - "Higher storage requirements"
      mitigations:
        - "Comprehensive documentation and training"
        - "Implement event archiving strategy"
    monitoring:
      - "Track event storage growth"
      - "Monitor projection build performance"
      - "Watch for event processing delays"
    tags:
      - "architecture"
      - "event-sourcing"
      - "compliance"
```

## Reference Implementation

### Rust Implementation

The reference implementation is written in Rust and provides:

- **Core CLI**: Command-line interface for context management
- **Schema Validation**: JSON Schema validation for all context files
- **Query Engine**: CQL implementation with advanced features
- **Git Integration**: Native Git workflow integration
- **MCP Daemon**: Model Context Protocol implementation
- **IDE Extensions**: VS Code, IntelliJ, and Vim integrations

### Installation

```bash
# From Cargo
cargo install rhema

# From source
git clone https://github.com/fugue-ai/rhema.git
cd rhema
cargo build --release
```

### Basic Usage

```bash
# Initialize a scope
rhema init --scope-type service --scope-name user-service

# Add context
rhema todo add "Implement authentication" --priority high
rhema decision record "Use JWT" --description "Chosen for stateless auth"

# Query context
rhema query "todos WHERE priority='high'"
rhema query "*/decisions WHERE status='accepted'"

# Export context
rhema export --format json --output context.json
rhema export --format markdown --output context.md
```

### Configuration

```yaml
# ~/.rhema/config.yaml
rhema:
  version: "1.0.0"
  default_scope_type: "service"
  schema_validation: true
  git_integration: true
  mcp_daemon:
    enabled: true
    port: 3000
  ide_integration:
    vscode: true
    intellij: true
    vim: true
```

### API Reference

#### Core Commands
- `rhema init` - Initialize a new scope
- `rhema validate` - Validate context files
- `rhema query` - Execute CQL queries
- `rhema export` - Export context in various formats
- `rhema sync` - Synchronize context across scopes

#### Context Commands
- `rhema todo` - Manage work items
- `rhema decision` - Manage architecture decisions
- `rhema insight` - Manage knowledge and insights
- `rhema pattern` - Manage design patterns
- `rhema convention` - Manage team conventions

#### Integration Commands
- `rhema mcp` - Start MCP daemon
- `rhema git` - Git integration commands
- `rhema ide` - IDE integration commands

### Contributing

The Rhema specification is open for contributions:

1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Add tests and documentation**
5. **Submit a pull request**

### License

Copyright 2025 Cory Parent - Licensed under the Apache License, Version 2.0

---

This specification is maintained by the Rhema community. For questions, issues, or contributions, please visit the [Rhema GitHub repository](https://github.com/fugue-ai/rhema). 