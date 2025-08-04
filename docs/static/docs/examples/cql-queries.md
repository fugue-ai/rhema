# CQL (Context Query Language) Examples


CQL is a simple YAML path-based query syntax for cross-scope context retrieval. This guide shows practical examples of how to query your Rhema context effectively.

## 🔍 Basic Queries


Start with simple queries to explore your context:

```bash
# Query all todos in the current scope


rhema query "todos"

# Query knowledge entries


rhema query "knowledge.entries"

# Query decisions


rhema query "decisions"
```

## 🎯 Filtering with WHERE Clauses


Use WHERE conditions to filter results:

```bash
# Find pending todos


rhema query "todos WHERE status='pending'"

# Find high-confidence knowledge


rhema query "knowledge WHERE confidence>7"

# Find approved decisions


rhema query "decisions WHERE status='approved'"
```

## 🌐 Cross-Scope Queries


Query across multiple scopes and repositories:

```bash
# Query todos in a specific scope


rhema query "../backend/.rhema/todos"

# Query patterns across all scopes


rhema query "*/patterns WHERE usage='required'"

# Query todos across all scopes


rhema query "*/todos WHERE status='in_progress'"
```

## 🔗 Complex Queries


Combine multiple conditions for precise filtering:

```bash
# Find high-priority todos assigned to a specific person


rhema query "todos WHERE priority='high' AND assignee='alice'"

# Find security or performance knowledge


rhema query "knowledge WHERE category='security' OR category='performance'"

# Find decisions affecting multiple services


rhema query "decisions WHERE impact_scope='multiple'"
```

## 📊 Provenance Tracking


Track query execution and data lineage:

```bash
# Basic provenance tracking


rhema query "todos WHERE status='pending'" --provenance

# Field-level provenance for detailed lineage


rhema query "knowledge WHERE confidence>7" --field-provenance
```

## 🎯 Common Query Patterns


### Finding Work Items


```bash
# High-priority work


rhema query "todos WHERE priority='high'"

# Work in progress


rhema query "todos WHERE status='in_progress'"

# Overdue work


rhema query "todos WHERE due_date < '2024-01-01'"
```

### Discovering Knowledge


```bash
# Performance insights


rhema query "knowledge WHERE category='performance'"

# High-confidence findings


rhema query "knowledge WHERE confidence='high'"

# Recent discoveries


rhema query "knowledge WHERE date > '2024-01-01'"
```

### Tracking Decisions


```bash
# Recent decisions


rhema query "decisions WHERE date > '2024-01-01'"

# Decisions affecting specific services


rhema query "decisions WHERE impact CONTAINS 'user-service'"

# Pending decisions


rhema query "decisions WHERE status='pending'"
```

### Finding Patterns


```bash
# Required patterns


rhema query "patterns WHERE usage='required'"

# Design patterns


rhema query "patterns WHERE type='design'"

# Patterns with examples


rhema query "patterns WHERE examples IS NOT NULL"
```

## 🔍 Advanced Query Techniques


### Text Search


```bash
# Search for todos containing "authentication"


rhema query "todos WHERE title CONTAINS 'authentication'"

# Search knowledge for "performance"


rhema query "knowledge WHERE finding CONTAINS 'performance'"
```

### Date Ranges


```bash
# Todos created in the last week


rhema query "todos WHERE created_at > '2024-01-08'"

# Decisions made this month


rhema query "decisions WHERE date >= '2024-01-01' AND date <= '2024-01-31'"
```

### Nested Queries


```bash
# Knowledge with specific evidence


rhema query "knowledge WHERE evidence CONTAINS 'logs'"

# Decisions with multiple alternatives


rhema query "decisions WHERE alternatives_considered COUNT > 2"
```

## 📈 Query Best Practices


1. **Start Simple** - Begin with basic queries and add complexity

2. **Use Specific Filters** - Narrow down results with precise WHERE conditions

3. **Leverage Cross-Scope** - Use `*/` to search across all scopes

4. **Track Provenance** - Use `--provenance` for audit trails

5. **Combine Conditions** - Use AND/OR for complex filtering

6. **Query Regularly** - Make querying part of your daily workflow

## 🚀 Next Steps


- **Practice** - Try these queries in your own project

- **Explore** - Discover what context you have available

- **Build** - Create queries specific to your team's needs

- **Share** - Document useful queries for your team

## 🔗 Related Examples


- [Query Provenance](query-provenance.md) - Understanding query execution details

- [Advanced Usage](advanced-usage.md) - Cross-scope coordination examples

- [Quick Start Commands](quick-start-commands.md) - Basic Rhema usage 