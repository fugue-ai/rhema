# Query Provenance Tracking

Rhema provides comprehensive provenance tracking for all queries, enabling full audit trails and data lineage. This example shows how to use provenance tracking to understand query execution and data origins.

## 🎯 What is Provenance Tracking?

Provenance tracking provides detailed information about:
- **Query execution** - How queries are processed
- **Data lineage** - Where each piece of data comes from
- **Performance metrics** - Execution time and resource usage
- **Applied transformations** - Filters, sorting, and other operations

## 📊 Query-Level Provenance

Track execution metadata and performance:

```bash
# Basic provenance tracking
rhema query "todos WHERE status='pending'" --provenance
```

### Example Output
```bash
$ rhema query "todos WHERE status='pending'" --provenance

📊 Query Provenance:
────────────────────────────────────────────────────────────────────────────────
🔍 Original Query: todos WHERE status='pending'
⏰ Executed At: 2024-01-15T10:30:00Z
⏱️  Execution Time: 45ms
📁 Scopes Searched: test-scope
📄 Files Accessed: todos.yaml

📈 Performance Metrics:
  Total Time: 45ms
  Files Read: 1
  YAML Documents Processed: 1
  Phase Times:
    parsing: 2ms
    scope_discovery: 5ms
    execution: 38ms

🔧 Execution Steps:
  • Query Parsing (2ms)
  • Scope Discovery (5ms)
  • File Access (10ms)
  • Condition Filtering (15ms)
  • Result Assembly (13ms)

🔍 Applied Filters:
  • WhereCondition: Applied 1 WHERE conditions (2 → 1 items)
  • Limit: Applied LIMIT=None OFFSET=None (1 → 1 items)

📋 Query Result:
────────────────────────────────────────────────────────────────────────────────
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
```

## 🔍 Field-Level Provenance

Track the origin of each field in query results:

```bash
# Field-level provenance for detailed lineage
rhema query "knowledge WHERE confidence>7" --field-provenance
```

### What Field-Level Provenance Shows

- **Data lineage** - Track the origin of each field in query results
- **Transformation history** - Record all transformations applied to fields
- **Source tracking** - Identify which scope, file, and YAML path each field came from
- **Confidence scoring** - Assign confidence levels to field values based on data quality

## 📈 Provenance Components

### Execution Metadata
- **Timestamp** - When the query was executed
- **Duration** - Total execution time
- **Scopes searched** - Which scopes were examined
- **Files accessed** - Which files were read

### Performance Metrics
- **Phase-by-phase timing** - Detailed breakdown of execution stages
- **Memory usage** - Resource consumption during execution
- **Cache statistics** - Cache hit/miss rates
- **File operations** - Number of files read and processed

### Execution Steps
- **Query parsing** - How the query was interpreted
- **Scope discovery** - How scopes were identified
- **File access** - How files were located and read
- **Condition filtering** - How WHERE conditions were applied
- **Result assembly** - How final results were constructed

### Applied Filters
- **WHERE conditions** - Complete record of filtering operations
- **YAML paths** - Which paths were traversed
- **Ordering** - Sort operations applied
- **Limits** - Pagination and result limiting

## 🎯 Use Cases for Provenance Tracking

### Debugging Queries
```bash
# Understand why a query returns unexpected results
rhema query "todos WHERE priority='high'" --provenance
```

### Performance Optimization
```bash
# Identify slow query phases
rhema query "*/knowledge WHERE confidence='high'" --provenance
```

### Audit Trails
```bash
# Track data lineage for compliance
rhema query "decisions WHERE status='approved'" --field-provenance
```

### Data Quality Assessment
```bash
# Assess confidence in query results
rhema query "knowledge WHERE category='performance'" --field-provenance
```

## 🔧 Provenance Best Practices

1. **Use for Debugging** - Enable provenance when queries don't work as expected
2. **Monitor Performance** - Track execution times to identify bottlenecks
3. **Audit Compliance** - Use field-level provenance for regulatory requirements
4. **Data Quality** - Assess confidence levels in query results
5. **Team Collaboration** - Share provenance information to help team members understand queries

## 🚀 Advanced Provenance Features

### Custom Provenance Output
```bash
# Export provenance to JSON for analysis
rhema query "todos WHERE status='pending'" --provenance --format json

# Save provenance to file
rhema query "knowledge WHERE confidence>7" --provenance --output provenance.json
```

### Provenance Comparison
```bash
# Compare query performance over time
rhema query "todos WHERE status='pending'" --provenance --compare-with previous-run.json
```

## 📚 Next Steps

- **Experiment** - Try provenance tracking with your own queries
- **Monitor** - Use provenance to optimize query performance
- **Document** - Share provenance insights with your team
- **Automate** - Integrate provenance tracking into your workflows

## 🔗 Related Examples

- [CQL Queries](cql-queries.md) - Learn the query language
- [Advanced Usage](advanced-usage.md) - Explore advanced features
- [Quick Start Commands](quick-start-commands.md) - Basic Rhema usage 