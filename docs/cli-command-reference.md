# CLI Command Reference

## Initialization and Discovery

```bash
rhema init [--scope-type TYPE] [--scope-name NAME]  # Initialize new scope
rhema scopes                                         # List all scopes
rhema scope [PATH]                                   # Show scope details
rhema tree                                           # Show scope hierarchy
```

## Content Management

```bash
rhema show FILE [--scope SCOPE]                      # Display YAML file content
rhema query "CQL_QUERY"                              # Execute context query
rhema search "TERM" [--in FILE]                      # Search across context files
```

## Validation and Health

```bash
rhema validate [--recursive]                         # Validate YAML files
rhema health [--scope SCOPE]                         # Check scope completeness
rhema stats                                          # Show context metrics
```

## Work Item Management

```bash
rhema todo add "TITLE" [--priority LEVEL]            # Add todo
rhema todo list [--status STATUS]                    # List todos
rhema todo complete ID [--outcome "DESCRIPTION"]     # Complete todo
```

## Knowledge Management

```bash
rhema insight record "INSIGHT" [--confidence LEVEL]  # Record insight
rhema pattern add "NAME" [--effectiveness LEVEL]     # Add pattern
rhema decision record "TITLE" [--status STATUS]      # Record decision
```

## Cross-Scope Operations

```bash
rhema dependencies                                   # Show scope relationships
rhema impact FILE                                    # Show affected scopes
rhema sync-knowledge                                 # Update cross-scope references
```

## Performance Monitoring and Analytics

```bash
rhema performance start                              # Start performance monitoring
rhema performance stop                               # Stop performance monitoring
rhema performance status                             # Show system performance status
rhema performance report [--hours HOURS]             # Generate performance report
rhema performance config                             # Show monitoring configuration
```

### Performance Monitoring Features

- **System Performance Monitoring**: CPU, memory, disk I/O, network latency, file system operations
- **User Experience Monitoring**: Command execution time, success rates, response times, error tracking
- **Usage Analytics**: Command usage patterns, feature adoption, workflow completion rates
- **Performance Reporting**: Automated reports with trends, recommendations, and impact assessment
- **Threshold Alerts**: Configurable alerts for performance bottlenecks and degradations
- **Real-time Dashboards**: Web-based dashboards for live performance monitoring

### Example Performance Report

```bash
# Generate a 24-hour performance report
rhema performance report --hours 24
```

**Output:**
```
📈 Performance Report
═══════════════════════════════════════════════════════════════════════════════
Report ID: 550e8400-e29b-41d4-a716-446655440000
Generated: 2024-12-19 15:30:00 UTC
Period: 2024-12-18 15:30:00 UTC to 2024-12-19 15:30:00 UTC

💻 System Performance Summary
──────────────────────────────────────────────────────────────────────────────
CPU Usage: 25.0% avg, 75.0% peak
Memory Usage: 50.0% avg, 80.0% peak
Network Latency: 10.0 ms avg
Total Disk I/O: 100.0 MB
Total Network I/O: 50.0 MB

👤 User Experience Summary
──────────────────────────────────────────────────────────────────────────────
Command Execution Time: 150.0 ms avg
Command Success Rate: 95.0%
Response Time: 50.0 ms avg
User Satisfaction: 8.5/10 avg
Error Rate: 5.0%

📊 Usage Analytics Summary
──────────────────────────────────────────────────────────────────────────────
Total Commands: 1000
Feature Adoption Rate: 75.0%
Session Duration: 300.0 seconds avg
Workflow Completion Rate: 85.0%

📈 Performance Trends
──────────────────────────────────────────────────────────────────────────────
📈 Command execution time: -15.0% change (confidence: 95.0%)
   Command execution time has improved by 15% over the reporting period
➡️ Memory usage: 2.0% change (confidence: 90.0%)
   Memory usage has remained stable with only 2% increase

🔧 Optimization Recommendations
──────────────────────────────────────────────────────────────────────────────
🔴 Optimize query execution (Priority: High)
   Implement query caching to reduce execution time
   Expected Impact: Reduce query execution time by 30%
   Implementation Effort: Medium

🟡 Improve memory management (Priority: Medium)
   Implement memory pooling for large operations
   Expected Impact: Reduce memory usage by 20%
   Implementation Effort: High
``` 