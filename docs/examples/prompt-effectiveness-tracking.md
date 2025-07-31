# Prompt Effectiveness Tracking

The prompt effectiveness tracking system provides meaningful analytics for prompt patterns based on actual usage data.

## Overview

The system tracks:
- **Total uses** - How many times a prompt was used
- **Successful uses** - How many times it was marked as successful
- **Success rate** - Calculated as `successful_uses / total_uses`
- **Last used** - Timestamp of the most recent usage
- **Feedback history** - Detailed feedback from users

## Usage Examples

### Recording Usage

```bash
# Record a successful usage
rhema prompt record-usage "Code Review" true --feedback "Great for security reviews"

# Record an unsuccessful usage
rhema prompt record-usage "Bug Report" false --feedback "Template needs more specific fields"

# Record usage without feedback
rhema prompt record-usage "Code Review" true
```

### Viewing Analytics

```bash
# Show detailed analytics for a prompt pattern
rhema prompt show-analytics "Code Review"

# List all patterns with usage statistics
rhema prompt list
```

### Example Output

```bash
$ rhema prompt show-analytics "Code Review"

📊 Analytics for 'Code Review':
============================================================
Total uses: 15
Successful uses: 13
Success rate: 86.7%
Last used: 2025-01-15 10:30:00

📝 Recent Feedback:
----------------------------------------
✅ 2025-01-15 10:30 - Great for security-focused reviews
✅ 2025-01-14 14:20 - Very helpful for catching bugs
❌ 2025-01-13 09:15 - Could be more specific about performance concerns
✅ 2025-01-12 16:45 - Perfect for our code review workflow
✅ 2025-01-11 11:30 - Excellent template structure
```

## Data Structure

### UsageAnalytics

```yaml
usage_analytics:
  total_uses: 15          # Total number of times used
  successful_uses: 13     # Number of successful uses
  last_used: "2025-01-15T10:30:00Z"  # Last usage timestamp
  feedback_history:       # Array of feedback entries
    - timestamp: "2025-01-15T10:30:00Z"
      successful: true
      feedback: "Great for security-focused reviews"
```

### FeedbackEntry

```yaml
- timestamp: "2025-01-15T10:30:00Z"  # When feedback was recorded
  successful: true                    # Whether the usage was successful
  feedback: "User feedback text"      # Optional feedback comment
```

## Success Rate Calculation

The success rate is automatically calculated as:
```
success_rate = successful_uses / total_uses
```

- **0 uses**: 0.0 (0%)
- **5 successful out of 10 total**: 0.5 (50%)
- **8 successful out of 10 total**: 0.8 (80%)

## Best Practices

### Recording Usage

1. **Record every usage** - Even unsuccessful ones provide valuable data
2. **Provide feedback** - Detailed feedback helps improve prompts
3. **Be consistent** - Use the same criteria for success/failure
4. **Record promptly** - Record usage soon after using the prompt

### Interpreting Analytics

1. **Sample size matters** - Success rates are more reliable with more uses
2. **Look at trends** - Recent feedback may be more relevant
3. **Consider context** - Different task types may have different success rates
4. **Use feedback** - Read feedback to understand why prompts succeed or fail

### Improving Prompts

1. **Identify patterns** - Look for common feedback themes
2. **Test variations** - Create multiple versions of prompts
3. **Track improvements** - Monitor success rates after changes
4. **Iterate** - Use feedback to continuously improve prompts

## Integration with Context Injection

The effectiveness tracking works seamlessly with the context injection system:

```bash
# Test a prompt with context injection
rhema prompt test "Code Review" --task-type security

# Record the result
rhema prompt record-usage "Code Review" true --feedback "Security context made it much more effective"

# View updated analytics
rhema prompt show-analytics "Code Review"
```

## Advanced Features

### Feedback Analysis

The system stores detailed feedback that can be analyzed for patterns:

```bash
# View recent feedback for insights
rhema prompt show-analytics "Code Review"
```

Common feedback patterns to look for:
- **Positive patterns**: "Great for X", "Very helpful", "Perfect for Y"
- **Negative patterns**: "Could be more specific", "Needs improvement", "Too generic"
- **Context-specific feedback**: "Security context helped", "Better for large codebases"

### Success Rate Trends

Monitor how success rates change over time:

```bash
# Check if recent usage is improving
rhema prompt show-analytics "Code Review"
```

Look for:
- **Improving trends**: Higher success rates in recent feedback
- **Declining trends**: Lower success rates may indicate prompt drift
- **Consistent performance**: Stable success rates suggest good prompts

## Troubleshooting

### Common Issues

1. **No usage data** - Prompt patterns start with 0 uses
2. **Low success rates** - May indicate prompt needs improvement
3. **Inconsistent feedback** - May indicate unclear success criteria

### Debugging

```bash
# Check if prompt exists
rhema prompt list

# Verify usage was recorded
rhema prompt show-analytics "Pattern Name"

# Check for errors
rhema prompt record-usage "Pattern Name" true --feedback "Test"
```

## Future Enhancements

Planned improvements include:

- **A/B Testing** - Compare multiple prompt variations
- **Trend Analysis** - Track success rates over time
- **Automated Insights** - AI-powered analysis of feedback patterns
- **Export Analytics** - Export data for external analysis
- **Team Analytics** - Aggregate usage across team members
- **Performance Metrics** - Track response time and other metrics

## Migration from Old System

If you have existing prompt patterns with the old `success_rate` field:

1. **Backup your data** - Save existing `prompts.yaml` files
2. **Update format** - Convert to new `usage_analytics` format
3. **Start tracking** - Begin recording actual usage data
4. **Validate** - Ensure new analytics are working correctly

The new system provides much more meaningful insights than the previous simple success rate field. 