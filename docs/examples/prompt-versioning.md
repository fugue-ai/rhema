# Prompt Versioning System

The prompt versioning system tracks the evolution of prompt patterns over time, allowing you to maintain a complete history of changes and improvements.

## Overview

The versioning system provides:
- **Semantic versioning** - Track major, minor, and patch changes
- **Version history** - Complete history of all prompt versions
- **Change tracking** - Document what changed in each version
- **Author attribution** - Track who made changes
- **Template preservation** - Keep templates from all versions
- **Timestamps** - Track when versions were created

## Version Structure

### PromptVersion
```yaml
version:
  current: "1.2.0"                    # Current version
  created_at: "2025-01-01T00:00:00Z"  # When first created
  updated_at: "2025-01-15T10:30:00Z"  # Last updated
  history:                            # Version history
    - version: "1.0.0"
      template: "Original template..."
      description: "Initial version"
      timestamp: "2025-01-01T00:00:00Z"
      author: null
      changes: ["Initial creation"]
```

### VersionEntry
```yaml
- version: "1.1.0"                    # Version string
  template: "Updated template..."      # Template at this version
  description: "Added improvements"    # Version description
  timestamp: "2025-01-10T14:30:00Z"   # When created
  author: "alice@example.com"         # Who created it
  changes:                            # List of changes
    - "Added specific instructions"
    - "Improved clarity"
```

## Usage Examples

### Creating New Versions

```bash
# Create a new version with updated template
rhema prompt version "Code Review" "1.1.0" \
  --template "Please review this code: {{CONTEXT}}" \
  --description "Added more specific instructions" \
  --changes "Added specific instructions,Improved clarity" \
  --author "alice@example.com"

# Create a version with just metadata (no template change)
rhema prompt version "Code Review" "1.1.1" \
  --description "Minor documentation update" \
  --changes "Updated documentation" \
  --author "bob@example.com"

# Create a major version update
rhema prompt version "Code Review" "2.0.0" \
  --template "SECURITY REVIEW: {{CONTEXT}}" \
  --description "Major security-focused redesign" \
  --changes "Redesigned for security focus,Added security context" \
  --author "security-team@example.com"
```

### Viewing Version History

```bash
# Show all versions
rhema prompt show-version "Code Review"

# Show specific version
rhema prompt show-version "Code Review" --version "1.1.0"

# Show current version details
rhema prompt show-version "Code Review" --version "1.2.0"
```

### Example Output

```bash
$ rhema prompt show-version "Code Review"

📋 Version History for 'Code Review':
============================================================
Current version: 1.2.0
Created: 2025-01-01 00:00
Last updated: 2025-01-15 10:30

📝 Version History:
----------------------------------------
⚪ Version 1.0.0 (2025-01-01 00:00)
   Description: Initial version
   Changes: Initial creation

🟢 Version 1.1.0 (2025-01-10 14:30)
   Author: alice@example.com
   Changes: Added specific instructions, Improved clarity

🟢 Version 1.2.0 (2025-01-15 10:30)
   Author: bob@example.com
   Changes: Added improvements section, Enhanced feedback request

🟢 = Current version
```

## Semantic Versioning

Follow semantic versioning principles:

- **Major version (X.0.0)** - Breaking changes, major redesigns
- **Minor version (X.Y.0)** - New features, improvements
- **Patch version (X.Y.Z)** - Bug fixes, minor updates

### Examples

```bash
# Major version - Complete redesign
rhema prompt version "Code Review" "2.0.0" \
  --description "Complete security-focused redesign"

# Minor version - New features
rhema prompt version "Code Review" "1.1.0" \
  --description "Added security context injection"

# Patch version - Bug fixes
rhema prompt version "Code Review" "1.0.1" \
  --description "Fixed template variable syntax"
```

## Best Practices

### Version Naming

1. **Use semantic versioning** - Follow MAJOR.MINOR.PATCH format
2. **Be descriptive** - Use clear version descriptions
3. **Document changes** - List specific changes in each version
4. **Attribute authors** - Track who made changes

### Change Documentation

```bash
# Good - Specific changes
--changes "Added security context,Improved error handling,Updated documentation"

# Bad - Vague changes
--changes "General improvements"
```

### Version Frequency

1. **Patch versions** - For minor fixes and updates
2. **Minor versions** - For new features and improvements
3. **Major versions** - For breaking changes and redesigns

## Integration with Other Features

### Versioning + Analytics

Track how effectiveness changes across versions:

```bash
# Test different versions
rhema prompt test "Code Review" --task-type security

# Record usage for current version
rhema prompt record-usage "Code Review" true --feedback "Version 1.2.0 works great"

# Create new version based on feedback
rhema prompt version "Code Review" "1.3.0" \
  --description "Improved based on user feedback" \
  --changes "Enhanced security focus,Better error messages"
```

### Versioning + Context Injection

Different versions can work with different context injection methods:

```bash
# Version 1.0.0 - Basic template variable
rhema prompt show-version "Code Review" --version "1.0.0"

# Version 2.0.0 - Enhanced with prepend context
rhema prompt version "Code Review" "2.0.0" \
  --template "Security Context: {{CONTEXT}}\n\nPlease review this code:" \
  --description "Added security context prepend"
```

## Advanced Features

### Version Comparison

Compare templates across versions:

```bash
# View specific version template
rhema prompt show-version "Code Review" --version "1.0.0"

# View current version template
rhema prompt show-version "Code Review" --version "1.2.0"
```

### Version Rollback

While not directly supported, you can recreate previous versions:

```bash
# View old version template
rhema prompt show-version "Code Review" --version "1.0.0"

# Create new version with old template
rhema prompt version "Code Review" "1.3.0" \
  --template "Please review this code: {{CONTEXT}}" \
  --description "Rollback to simpler template" \
  --changes "Rollback to version 1.0.0 template"
```

### Team Collaboration

Track contributions from different team members:

```bash
# Security team contribution
rhema prompt version "Code Review" "2.0.0" \
  --author "security-team@example.com" \
  --description "Security-focused redesign"

# Documentation team contribution
rhema prompt version "Code Review" "2.0.1" \
  --author "docs-team@example.com" \
  --description "Improved documentation"
```

## Troubleshooting

### Common Issues

1. **Version not found** - Check available versions with `show-version`
2. **Invalid version format** - Use semantic versioning (X.Y.Z)
3. **Template not updated** - Ensure `--template` flag is provided

### Debugging

```bash
# Check current version
rhema prompt show-version "Pattern Name"

# List all versions
rhema prompt show-version "Pattern Name"

# Verify version creation
rhema prompt version "Pattern Name" "1.1.0" --description "Test"
```

## Future Enhancements

Planned improvements include:

- **Version branching** - Support for feature branches
- **Version merging** - Merge changes from different branches
- **Version tags** - Tag important versions (stable, beta, etc.)
- **Version comparison** - Diff between versions
- **Version rollback** - Direct rollback to previous versions
- **Version search** - Search through version history
- **Version analytics** - Track effectiveness by version

## Migration from Non-Versioned Prompts

If you have existing prompts without versioning:

1. **Backup your data** - Save existing `prompts.yaml` files
2. **Add versioning** - The system will automatically add version "1.0.0"
3. **Update templates** - Create new versions as needed
4. **Document changes** - Add proper version descriptions and changes

The versioning system provides a complete audit trail of prompt evolution, making it easier to track improvements and maintain prompt quality over time. 