# Template Management System

The template management system allows teams to create, share, import, export, and manage prompt templates across different Rhema scopes and organizations.

## Overview

Template management enables:
- **Template libraries** - Organize templates into shareable collections
- **Cross-team sharing** - Share templates between different teams and scopes
- **Import/export** - Move templates between different Rhema installations
- **Usage tracking** - Monitor template downloads, usage, and ratings
- **Access control** - Control who can access and modify templates
- **Metadata management** - Rich metadata for template categorization and discovery

## Template Library Structure

### TemplateLibrary
```yaml
name: "Security Templates Library"
description: "Collection of security-focused prompt templates"
owner: "security-team@example.com"
version: "1.0.0"
created_at: "2025-01-01T00:00:00Z"
updated_at: "2025-01-15T10:30:00Z"
tags: ["security", "code-review", "vulnerability-assessment"]
access_control:
  public: true
  allowed_teams: null
  allowed_users: null
  read_only: false
templates: [...]  # Array of SharedTemplate
```

### SharedTemplate
```yaml
- id: "security-code-review-1"
  name: "Security Code Review"
  description: "Comprehensive security-focused code review template"
  template: "SECURITY CODE REVIEW\n\nPlease review: {{CONTEXT}}"
  metadata:
    author: "security-team@example.com"
    version: "1.0.0"
    category: "security"
    complexity: "Advanced"
    language: "general"
    examples: ["Use for authentication code", "Use for API endpoints"]
  tags: ["security", "code-review"]
  usage_stats:
    total_downloads: 45
    total_uses: 32
    average_rating: 4.8
    rating_count: 12
```

## Usage Examples

### Creating Template Libraries

```bash
# Create a public template library
rhema template create-library "Security Templates" \
  --description "Security-focused prompt templates" \
  --owner "security-team@example.com" \
  --tags "security,code-review,vulnerability-assessment" \
  --public

# Create a private template library
rhema template create-library "Internal Templates" \
  --description "Internal team templates" \
  --owner "team@example.com" \
  --tags "internal,team-specific" \
  --public false
```

### Adding Templates to Libraries

```bash
# Add a security template
rhema template add-template "Security Templates" \
  "Security Code Review" \
  "SECURITY CODE REVIEW\n\nPlease review: {{CONTEXT}}" \
  --description "Comprehensive security-focused code review template" \
  --category "security" \
  --complexity "Advanced" \
  --language "general" \
  --examples "Use for authentication code|Use for API endpoints" \
  --tags "security,code-review,vulnerability-assessment"

# Add a template with dependencies
rhema template add-template "Security Templates" \
  "Vulnerability Assessment" \
  "VULNERABILITY ASSESSMENT\n\nAssess: {{CONTEXT}}" \
  --description "Template for assessing potential vulnerabilities" \
  --category "security" \
  --complexity "Expert" \
  --dependencies "Security Code Review" \
  --examples "Use for new feature assessment|Use for third-party evaluation"
```

### Managing Template Libraries

```bash
# List all template libraries
rhema template list-libraries

# List libraries by tags
rhema template list-libraries --tags "security,code-review"

# Show detailed library information
rhema template show-library "Security Templates"

# Show specific template details
rhema template show-template "Security Templates" "Security Code Review"
```

### Importing and Exporting Templates

```bash
# Export templates from prompts.yaml
rhema template export-templates "Code Review Request,Bug Report Template" \
  "security-templates-export.yaml" \
  --description "Security-focused templates export" \
  --tags "security,export"

# Import templates to a library
rhema template import-templates "security-templates-export.yaml" \
  --library "Security Templates"

# Import templates to prompts.yaml
rhema template import-templates "security-templates-export.yaml"
```

### Sharing Templates Across Teams

```bash
# Share a library with another scope
rhema template share-library "Security Templates" "frontend-team"

# Download a template from a library
rhema template download-template "Security Templates" "Security Code Review"

# Rate a template
rhema template rate-template "Security Templates" "Security Code Review" 4.5
```

## Template Features

### Template Categories

Organize templates by category:

```bash
rhema template add-template "Library" "Template" "Content" \
  --category "security"      # Security-related templates
  --category "performance"   # Performance optimization templates
  --category "documentation" # Documentation templates
  --category "testing"       # Testing templates
```

### Template Complexity Levels

Specify complexity for different skill levels:

```bash
rhema template add-template "Library" "Template" "Content" \
  --complexity "Beginner"     # Basic templates for newcomers
  --complexity "Intermediate" # Moderate complexity templates
  --complexity "Advanced"     # Complex templates for experts
  --complexity "Expert"       # Expert-level templates
```

### Template Dependencies

Define dependencies between templates:

```bash
rhema template add-template "Library" "Advanced Review" "Content" \
  --dependencies "Basic Review,Intermediate Review"
```

### Template Examples

Provide usage examples:

```bash
rhema template add-template "Library" "Template" "Content" \
  --examples "Use for authentication code|Use for API endpoints|Use for database queries"
```

## Access Control

### Public vs Private Libraries

```bash
# Public library (accessible to all)
rhema template create-library "Public Templates" --public

# Private library (restricted access)
rhema template create-library "Private Templates" --public false
```

### Team-Specific Access

```yaml
access_control:
  public: false
  allowed_teams: ["security-team", "devops-team"]
  allowed_users: ["alice@example.com", "bob@example.com"]
  read_only: false
```

## Usage Statistics

### Tracking Template Usage

The system automatically tracks:
- **Downloads** - How many times a template was downloaded
- **Usage** - How many times a template was actually used
- **Ratings** - User ratings (1.0-5.0 scale)
- **Timestamps** - Last downloaded and used times

### Viewing Statistics

```bash
# View template statistics
rhema template show-template "Security Templates" "Security Code Review"
```

Output includes:
- Total downloads and uses
- Average rating and rating count
- Last downloaded and used timestamps

## Template Rating System

### Rating Templates

```bash
# Rate a template (1.0-5.0 scale)
rhema template rate-template "Security Templates" "Security Code Review" 4.5
rhema template rate-template "Security Templates" "Vulnerability Assessment" 5.0
```

### Rating Guidelines

- **5.0** - Excellent, highly effective template
- **4.0** - Very good, effective with minor improvements
- **3.0** - Good, functional but could be better
- **2.0** - Fair, needs significant improvements
- **1.0** - Poor, not effective or useful

## Cross-Team Collaboration

### Sharing Best Practices

```bash
# Security team creates and shares templates
rhema template create-library "Security Best Practices" \
  --owner "security-team@example.com" \
  --tags "security,best-practices" \
  --public

# Frontend team shares templates
rhema template create-library "Frontend Patterns" \
  --owner "frontend-team@example.com" \
  --tags "frontend,ui,ux" \
  --public

# DevOps team shares templates
rhema template create-library "DevOps Workflows" \
  --owner "devops-team@example.com" \
  --tags "devops,deployment,monitoring" \
  --public
```

### Template Discovery

```bash
# Discover security templates
rhema template list-libraries --tags "security"

# Discover frontend templates
rhema template list-libraries --tags "frontend"

# Discover all public templates
rhema template list-libraries
```

## Integration with Other Features

### Templates + Prompt Patterns

Templates can be imported as prompt patterns:

```bash
# Import template to prompts.yaml
rhema template import-templates "security-templates.yaml"

# Use imported template in workflows
rhema workflow add-step "Security Workflow" \
  "Security Review" "Security Code Review" \
  --task-type "security_review"
```

### Templates + Workflows

Templates can be used in workflow steps:

```bash
# Create workflow using shared templates
rhema workflow add "Security Review Workflow" \
  --description "Multi-step security review using shared templates"

rhema workflow add-step "Security Review Workflow" \
  "Initial Review" "Security Code Review" \
  --task-type "security_review"

rhema workflow add-step "Security Review Workflow" \
  "Vulnerability Assessment" "Vulnerability Assessment" \
  --task-type "vulnerability_assessment"
```

### Templates + Versioning

Templates support versioning:

```yaml
metadata:
  version: "1.0.0"
  created_at: "2025-01-01T00:00:00Z"
  updated_at: "2025-01-15T10:30:00Z"
```

## Best Practices

### Template Organization

1. **Use descriptive names** - Clear, specific template names
2. **Categorize templates** - Use categories for organization
3. **Set appropriate complexity** - Match complexity to target users
4. **Provide examples** - Include usage examples for clarity
5. **Use tags effectively** - Tag templates for discovery

### Template Content

1. **Keep templates focused** - Single purpose, clear intent
2. **Use consistent formatting** - Standardize template structure
3. **Include context variables** - Use {{CONTEXT}} for flexibility
4. **Document requirements** - Specify when and how to use
5. **Provide clear instructions** - Detailed guidance for users

### Library Management

1. **Organize by domain** - Group related templates together
2. **Set appropriate access** - Public for sharing, private for internal
3. **Maintain quality** - Regular review and updates
4. **Track usage** - Monitor effectiveness and popularity
5. **Gather feedback** - Encourage ratings and comments

### Team Collaboration

1. **Share best practices** - Create public libraries for common patterns
2. **Document standards** - Establish team template standards
3. **Review and approve** - Peer review for template quality
4. **Version control** - Track template evolution
5. **Cross-team sharing** - Share templates between teams

## Troubleshooting

### Common Issues

1. **Template not found** - Check template name and library
2. **Access denied** - Verify library access permissions
3. **Import failed** - Check file format and structure
4. **Rating invalid** - Ensure rating is between 1.0-5.0

### Debugging

```bash
# Check library structure
rhema template show-library "Library Name"

# Check template details
rhema template show-template "Library Name" "Template Name"

# Verify file exists
ls -la .rhema/template-libraries/

# Check import file format
cat import-file.yaml
```

## Future Enhancements

Planned improvements include:

- **Template search** - Full-text search across templates
- **Template recommendations** - AI-powered template suggestions
- **Template analytics** - Advanced usage analytics and insights
- **Template marketplace** - Centralized template repository
- **Template validation** - Automated template quality checks
- **Template collaboration** - Real-time collaborative editing
- **Template branching** - Version control for templates
- **Template automation** - Automated template generation

The template management system provides a powerful foundation for sharing and collaborating on prompt templates across teams and organizations. 