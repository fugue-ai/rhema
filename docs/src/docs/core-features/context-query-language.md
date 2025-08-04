# Context Query Language (CQL)

The Context Query Language (CQL) is Rhema's powerful querying system that allows you to search, filter, and analyze context data across all scopes. CQL provides a natural, SQL-like syntax for querying todos, insights, decisions, patterns, and other context data.

## 🎯 Overview

CQL enables you to:
- Search across all context data with a single query
- Filter by any field or combination of fields
- Perform complex aggregations and analysis
- Get provenance information about query results
- Export results in various formats

## 📝 Basic Syntax

### Simple Queries
```bash
# Find all todos
rhema query "find all todos"

# Find all insights
rhema query "find all insights"

# Find all decisions
rhema query "find all decisions"
```

### Field Filtering
```bash
# Find todos with high priority
rhema query "find todos where priority = high"

# Find insights with high confidence
rhema query "find insights where confidence = high"

# Find approved decisions
rhema query "find decisions where status = approved"
```

### Text Search
```bash
# Find todos containing "authentication"
rhema query "find todos containing 'authentication'"

# Find insights about performance
rhema query "find insights containing 'performance'"

# Find decisions about API design
rhema query "find decisions containing 'API'"
```

## 🔍 Query Operators

### Comparison Operators
```bash
# Equality
rhema query "find todos where priority = high"
rhema query "find insights where confidence = medium"

# Inequality
rhema query "find todos where priority != low"
rhema query "find decisions where status != rejected"

# Greater/Less than
rhema query "find todos where created_date > 2024-01-01"
rhema query "find insights where confidence > medium"
```

### Logical Operators
```bash
# AND operator
rhema query "find todos where priority = high AND status = todo"

# OR operator
rhema query "find insights where confidence = high OR tags contains 'critical'"

# NOT operator
rhema query "find decisions where status != approved AND status != pending"
```

### String Operators
```bash
# Contains
rhema query "find todos containing 'bug'"
rhema query "find insights containing 'performance'"

# Starts with
rhema query "find todos where title starts with 'Fix'"

# Ends with
rhema query "find decisions where title ends with 'API'"

# Regex matching
rhema query "find todos where title matches 'TODO.*urgent'"
```

### Array Operators
```bash
# Contains
rhema query "find todos where tags contains 'security'"
rhema query "find insights where tags contains 'performance'"

# Not contains
rhema query "find decisions where tags not contains 'deprecated'"

# Any/All
rhema query "find todos where any tags in ['bug', 'critical']"
rhema query "find insights where all tags in ['performance', 'optimization']"
```

## 📊 Aggregation Queries

### Counting
```bash
# Count all todos
rhema query "count todos"

# Count by status
rhema query "count todos by status"

# Count by priority
rhema query "count todos by priority"

# Count with filtering
rhema query "count todos where priority = high"
```

### Grouping
```bash
# Group todos by status
rhema query "group todos by status"

# Group insights by confidence
rhema query "group insights by confidence"

# Group decisions by status
rhema query "group decisions by status"
```

### Complex Aggregations
```bash
# Count todos by status and priority
rhema query "count todos by status, priority"

# Group insights by confidence and tags
rhema query "group insights by confidence, tags"

# Count decisions by status and team
rhema query "count decisions by status, metadata.team"
```

## 🔗 Cross-Scope Queries

### Scope Filtering
```bash
# Query specific scope
rhema query "find todos in 'user-service'"

# Query multiple scopes
rhema query "find todos in ['user-service', 'auth-service']"

# Query by scope type
rhema query "find insights in services"

# Query by scope pattern
rhema query "find decisions in 'services/*'"
```

### Cross-Scope Analysis
```bash
# Find todos across all services
rhema query "find todos in services where priority = high"

# Find insights across all libraries
rhema query "find insights in libraries containing 'performance'"

# Compare decisions across teams
rhema query "find decisions where metadata.team = platform"
```

## 📈 Advanced Queries

### Nested Field Access
```bash
# Access nested metadata
rhema query "find todos where metadata.team = platform"

# Access nested configuration
rhema query "find scopes where config.validation.strict = true"

# Access array elements
rhema query "find todos where tags[0] = 'urgent'"
```

### Date and Time Queries
```bash
# Find recent todos
rhema query "find todos where created_date > 2024-01-01"

# Find todos from last week
rhema query "find todos where created_date > now() - 7 days"

# Find insights from this month
rhema query "find insights where created_date > now() - 30 days"
```

### Complex Conditions
```bash
# Multiple conditions
rhema query "find todos where priority = high AND (status = todo OR status = in_progress)"

# Nested conditions
rhema query "find insights where confidence = high AND (tags contains 'performance' OR tags contains 'optimization')"

# Complex filtering
rhema query "find decisions where status = approved AND metadata.team = platform AND created_date > 2024-01-01"
```

## 🎯 Query Output Formats

### Default Format (YAML)
```bash
rhema query "find todos where priority = high"
```

### JSON Format
```bash
rhema query "find todos where priority = high" --format json
```

### Table Format
```bash
rhema query "count todos by status" --format table
```

### Count Format
```bash
rhema query "count todos where priority = high" --format count
```

## 📊 Query Statistics

### Enable Statistics
```bash
# Include query statistics
rhema query "find todos where priority = high" --stats

# Statistics with custom format
rhema query "count todos by status" --stats --format table
```

### Statistics Information
Statistics include:
- **Query execution time**
- **Number of scopes searched**
- **Number of files processed**
- **Cache hit/miss rates**
- **Memory usage**

## 🔍 Provenance Tracking

### Enable Provenance
```bash
# Include provenance information
rhema query "find todos where priority = high" --provenance

# Include field-level provenance
rhema query "find insights where confidence = high" --field-provenance
```

### Provenance Information
Provenance tracking provides:
- **Source scope and file**
- **Last modification date**
- **Modification history**
- **Data lineage**
- **Confidence scores**

## 🚀 Practical Examples

### Daily Workflow Queries
```bash
# Find my high-priority todos
rhema query "find todos where priority = high AND status = todo"

# Find recent insights
rhema query "find insights where created_date > now() - 7 days"

# Find pending decisions
rhema query "find decisions where status = pending"
```

### Project Analysis Queries
```bash
# Find all bugs across the project
rhema query "find todos where tags contains 'bug'"

# Find performance-related insights
rhema query "find insights containing 'performance'"

# Find architectural decisions
rhema query "find decisions where tags contains 'architecture'"
```

### Team Coordination Queries
```bash
# Find todos for my team
rhema query "find todos where metadata.team = platform"

# Find insights from other teams
rhema query "find insights where metadata.team != platform"

# Find cross-team decisions
rhema query "find decisions where metadata.team in ['platform', 'mobile']"
```

### Quality Assurance Queries
```bash
# Find todos without descriptions
rhema query "find todos where description = null"

# Find insights with low confidence
rhema query "find insights where confidence = low"

# Find decisions without rationale
rhema query "find decisions where description = null"
```

## 🔧 Query Optimization

### Performance Tips
```bash
# Use specific scope filtering
rhema query "find todos in 'user-service' where priority = high"

# Use indexed fields
rhema query "find todos where status = todo"  # status is indexed

# Limit result size
rhema query "find todos where priority = high" | head -10
```

### Caching
```bash
# Queries are automatically cached
rhema query "find todos where priority = high"

# Clear cache if needed
rhema config set cache.enabled false
```

## 🚨 Common Query Patterns

### Finding Related Items
```bash
# Find todos related to authentication
rhema query "find todos containing 'auth' OR tags contains 'authentication'"

# Find insights about a specific topic
rhema query "find insights containing 'database' OR tags contains 'database'"

# Find decisions affecting a component
rhema query "find decisions containing 'user-service' OR metadata.scope = 'user-service'"
```

### Trend Analysis
```bash
# Find todos created this week
rhema query "find todos where created_date > now() - 7 days"

# Find insights by month
rhema query "find insights where created_date > 2024-01-01 AND created_date < 2024-02-01"

# Find decision patterns
rhema query "count decisions by status, created_date"
```

## 📚 Related Documentation

- **[CLI Command Reference](../user-guide/cli-command-reference.md)** - Complete command documentation
- **[Examples](../examples/cql-queries.md)** - More CQL examples
- **[Query Provenance](../examples/query-provenance.md)** - Understanding query provenance
- **[Performance Monitoring](../user-guide/performance-monitoring.md)** - Optimizing query performance 