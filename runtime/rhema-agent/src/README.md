# Rhema App Source Organization

This directory contains the organized source code for the Rhema application. The files have been grouped into logical categories for better maintainability and navigation.

## Directory Structure

### `core/` - Core Functionality
Core application functionality and basic operations:
- `commands.rs` - Command definitions and implementations
- `config.rs` - Configuration management
- `init.rs` - Initialization logic
- `scopes.rs` - Scope management utilities

### `interactive/` - Interactive Features
All interactive and user interface related modules:
- `interactive.rs` - Basic interactive functionality
- `interactive_advanced.rs` - Advanced interactive features
- `interactive_enhanced.rs` - Enhanced interactive capabilities
- `interactive_parser.rs` - Interactive command parsing
- `interactive_builder.rs` - Interactive UI building utilities

### `data/` - Data Management
Schema, validation, migration, and template management:
- `schema.rs` - Data schema definitions
- `validate.rs` - Data validation logic
- `migrate.rs` - Data migration utilities
- `template.rs` - Template management

### `integration/` - Integration Features
External system integrations and synchronization:
- `git.rs` - Git integration
- `dependencies.rs` - Dependency management
- `integrations.rs` - General integration utilities
- `sync.rs` - Synchronization logic

### `analysis/` - Analysis and Reporting
Query, search, statistics, and health monitoring:
- `query.rs` - Query processing
- `search.rs` - Search functionality
- `stats.rs` - Statistics and metrics
- `health.rs` - Health monitoring
- `show.rs` - Data display utilities

### `workflow/` - Workflow and Automation
Workflow management, batch processing, and performance monitoring:
- `workflow.rs` - Workflow definitions and execution
- `batch.rs` - Batch processing operations
- `performance.rs` - Performance monitoring
- `locomo.rs` - LOCOMO integration for analytics
- `coordination.rs` - Coordination between components
- `daemon.rs` - Daemon process management
- `lock.rs` - Lock management

### `context/` - Context and Knowledge
Context management, knowledge base, and insights:
- `context_rules.rs` - Context rule definitions
- `prompt.rs` - Prompt management
- `knowledge.rs` - Knowledge base operations
- `todo.rs` - Todo item management
- `insight.rs` - Insight generation
- `pattern.rs` - Pattern recognition and management
- `decision.rs` - Decision tracking
- `impact.rs` - Impact analysis
- `export_context.rs` - Context export utilities
- `primer.rs` - Primer generation
- `generate_readme.rs` - README generation
- `bootstrap_context.rs` - Context bootstrapping

## Root Files

- `lib.rs` - Main library entry point and re-exports
- `main.rs` - Application entry point
- `mod.rs` - Module declarations and common argument structures

## Benefits of This Organization

1. **Logical Grouping**: Related functionality is grouped together
2. **Easier Navigation**: Developers can quickly find relevant code
3. **Better Maintainability**: Changes to related features are co-located
4. **Clearer Dependencies**: Module relationships are more obvious
5. **Scalability**: New features can be added to appropriate directories

## Module Access

All modules are re-exported through the main `mod.rs` file, so existing code that imports from the rhema crate will continue to work without changes. The organization is purely internal and doesn't affect the public API. 