# CLI Command Reference

This comprehensive reference documents all Rhema CLI commands with examples, options, and usage patterns.

## 🚀 Initialization and Discovery

### Initialize New Scope
```bash
rhema init [--scope-type TYPE] [--scope-name NAME] [--auto-config]
```
Initialize a new Rhema scope in the current directory.

**Options:**
- `--scope-type TYPE`: Specify scope type (service, app, library, etc.)
- `--scope-name NAME`: Set custom scope name
- `--auto-config`: Auto-detect configuration from repository structure

**Examples:**
```bash
# Basic initialization
rhema init

# Initialize with specific type
rhema init --scope-type service --scope-name user-api

# Auto-configure from existing project
rhema init --auto-config
```

### List Scopes
```bash
rhema scopes
```
List all scopes in the repository with their types and status.

### Show Scope Details
```bash
rhema scope [PATH]
```
Display detailed information about a specific scope.

**Examples:**
```bash
# Show current scope
rhema scope

# Show specific scope
rhema scope ./services/auth
```

### Show Scope Hierarchy
```bash
rhema tree
```
Display the scope hierarchy as a tree structure.

## 📄 Content Management

### Display YAML File Content
```bash
rhema show FILE [--scope SCOPE]
```
Display the content of a YAML file with syntax highlighting.

**Examples:**
```bash
# Show todos file
rhema show todos

# Show scope-specific file
rhema show decisions --scope ./services/auth
```

### Execute Context Query
```bash
rhema query "CQL_QUERY" [--stats] [--format FORMAT] [--provenance] [--field-provenance]
```
Execute a Context Query Language (CQL) query across all scopes.

**Options:**
- `--stats`: Include query statistics
- `--format FORMAT`: Output format (yaml, json, table, count)
- `--provenance`: Include provenance tracking
- `--field-provenance`: Include field-level provenance

**Examples:**
```bash
# Basic query
rhema query "find all todos with priority high"

# Query with statistics
rhema query "find decisions where status = approved" --stats

# Query with custom format
rhema query "count todos by status" --format table
```

### Search Across Context Files
```bash
rhema search "TERM" [--in FILE] [--regex]
```
Search for text across all context files.

**Options:**
- `--in FILE`: Search in specific file type
- `--regex`: Use regex pattern instead of simple text search

**Examples:**
```bash
# Simple search
rhema search "authentication"

# Search in specific file type
rhema search "bug" --in todos

# Regex search
rhema search "TODO.*urgent" --regex
```

## ✅ Validation and Health

### Validate YAML Files
```bash
rhema validate [--recursive] [--json-schema] [--migrate] [--lock-file] [--lock-only] [--strict]
```
Validate YAML files against their schemas.

**Options:**
- `--recursive`: Validate recursively in subdirectories
- `--json-schema`: Show JSON schemas
- `--migrate`: Migrate schemas to latest version
- `--lock-file`: Validate against lock file
- `--lock-only`: Validate lock file only
- `--strict`: Treat warnings as errors

**Examples:**
```bash
# Basic validation
rhema validate

# Recursive validation
rhema validate --recursive

# Validate with lock file
rhema validate --lock-file --strict
```

### Check Scope Health
```bash
rhema health [--scope SCOPE]
```
Check the health and completeness of scopes.

**Examples:**
```bash
# Check all scopes
rhema health

# Check specific scope
rhema health --scope ./services/auth
```

### Show Context Statistics
```bash
rhema stats
```
Display comprehensive statistics about the context repository.

## 📋 Work Item Management

### Todo Management
```bash
rhema todo <subcommand>
```

**Subcommands:**
- `add "TITLE" [--priority LEVEL] [--description DESC] [--tags TAGS]`
- `list [--status STATUS] [--priority PRIORITY] [--scope SCOPE]`
- `show ID`: Show todo details
- `update ID [--title TITLE] [--status STATUS] [--priority PRIORITY]`
- `complete ID [--outcome "DESCRIPTION"]`
- `delete ID`

**Examples:**
```bash
# Add todo
rhema todo add "Fix authentication bug" --priority high --tags "bug,security"

# List todos
rhema todo list --status todo --priority high

# Complete todo
rhema todo complete 123 --outcome "Fixed by updating JWT validation"
```

### Insight Management
```bash
rhema insight <subcommand>
```

**Subcommands:**
- `record "INSIGHT" [--confidence LEVEL] [--tags TAGS]`
- `list [--confidence LEVEL] [--scope SCOPE]`
- `show ID`: Show insight details
- `update ID [--insight INSIGHT] [--confidence LEVEL]`
- `delete ID`

**Examples:**
```bash
# Record insight
rhema insight record "Using Redis for session storage improves performance by 40%" --confidence high

# List insights
rhema insight list --confidence high
```

### Pattern Management
```bash
rhema pattern <subcommand>
```

**Subcommands:**
- `add "NAME" [--effectiveness LEVEL] [--description DESC] [--tags TAGS]`
- `list [--effectiveness LEVEL] [--scope SCOPE]`
- `show ID`: Show pattern details
- `update ID [--name NAME] [--effectiveness LEVEL]`
- `delete ID`

**Examples:**
```bash
# Add pattern
rhema pattern add "Circuit Breaker" --effectiveness high --description "Prevents cascade failures"

# List patterns
rhema pattern list --effectiveness high
```

### Decision Management
```bash
rhema decision <subcommand>
```

**Subcommands:**
- `record "TITLE" [--status STATUS] [--description DESC] [--tags TAGS]`
- `list [--status STATUS] [--scope SCOPE]`
- `show ID`: Show decision details
- `update ID [--title TITLE] [--status STATUS]`
- `delete ID`

**Examples:**
```bash
# Record decision
rhema decision record "Use GraphQL for API" --status approved --description "Better for mobile clients"

# List decisions
rhema decision list --status approved
```

## 🔗 Cross-Scope Operations

### Show Dependencies
```bash
rhema dependencies [--lock-file] [--compare] [--visualize] [--conflicts] [--impact] [--format FORMAT]
```
Show scope dependencies and relationships.

**Options:**
- `--lock-file`: Analyze from lock file instead of current state
- `--compare`: Compare lock file with current state
- `--visualize`: Show dependency chain visualization
- `--conflicts`: Detect version conflicts
- `--impact`: Show detailed impact analysis
- `--format FORMAT`: Output format (text, json, yaml)

**Examples:**
```bash
# Show dependencies
rhema dependencies

# Show with visualization
rhema dependencies --visualize

# Check for conflicts
rhema dependencies --conflicts --impact
```

### Show Impact of Changes
```bash
rhema impact FILE
```
Show which scopes would be affected by changes to a file.

**Examples:**
```bash
# Show impact of changes
rhema impact src/auth/service.rs
```

### Sync Knowledge
```bash
rhema sync-knowledge
```
Sync knowledge across scopes, updating cross-references.

## 🔧 Advanced Operations

### Export Context Data
```bash
rhema export-context [--format FORMAT] [--output-file FILE] [--scope-filter SCOPE] [--include-protocol] [--include-knowledge] [--include-todos] [--include-decisions] [--include-patterns] [--include-conventions] [--summarize] [--ai-agent-format]
```
Export context data in various formats.

**Options:**
- `--format FORMAT`: Output format (json, yaml, markdown, text)
- `--output-file FILE`: Output file path
- `--scope-filter SCOPE`: Filter by scope
- `--include-*`: Include specific data types
- `--summarize`: Summarize data
- `--ai-agent-format`: Format for AI agent consumption

**Examples:**
```bash
# Export all data
rhema export-context --format json --output-file context.json

# Export for AI agent
rhema export-context --ai-agent-format --include-knowledge --include-todos
```

### Generate Context Primer
```bash
rhema primer [--scope-name SCOPE] [--output-dir DIR] [--template-type TEMPLATE] [--include-examples] [--validate]
```
Generate context primer files for onboarding.

**Examples:**
```bash
# Generate primer
rhema primer --scope-name auth-service --include-examples

# Generate with validation
rhema primer --validate --output-dir ./docs
```

### Generate README with Context
```bash
rhema generate-readme [--scope-name SCOPE] [--output-file FILE] [--template TEMPLATE] [--include-context] [--seo-optimized] [--custom-sections SECTIONS]
```
Generate README files with embedded context.

**Examples:**
```bash
# Generate README
rhema generate-readme --include-context --seo-optimized

# Generate with custom sections
rhema generate-readme --custom-sections "Installation,API,Examples"
```

### Bootstrap Context for AI Agents
```bash
rhema bootstrap-context [--use-case USE_CASE] [--output-format FORMAT] [--output-dir DIR] [--scope-filter SCOPE] [--include-all] [--optimize-for-ai] [--create-primer] [--create-readme]
```
Bootstrap context data for AI agent consumption.

**Use Cases:**
- `code_review`: Optimized for code review tasks
- `feature_development`: Optimized for feature development
- `debugging`: Optimized for debugging tasks
- `documentation`: Optimized for documentation tasks
- `onboarding`: Optimized for team onboarding

**Examples:**
```bash
# Bootstrap for code review
rhema bootstrap-context --use-case code_review --optimize-for-ai

# Bootstrap for onboarding
rhema bootstrap-context --use-case onboarding --create-primer --create-readme
```

## 🔄 Migration and Schema Management

### Migrate Schema Files
```bash
rhema migrate [--recursive] [--dry-run]
```
Migrate schema files to the latest version.

**Examples:**
```bash
# Dry run migration
rhema migrate --dry-run

# Migrate recursively
rhema migrate --recursive
```

### Generate Schema Templates
```bash
rhema schema TEMPLATE_TYPE [--output-file FILE]
```
Generate schema templates for various document types.

**Template Types:**
- `scope`: Scope configuration template
- `knowledge`: Knowledge base template
- `todos`: Todo items template
- `decisions`: Decisions template
- `patterns`: Patterns template
- `conventions`: Conventions template
- `all`: All templates

**Examples:**
```bash
# Generate scope template
rhema schema scope --output-file scope.yaml

# Generate all templates
rhema schema all
```

## 🎯 Interactive Mode

### Start Interactive Mode
```bash
rhema interactive [--config CONFIG] [--no-auto-complete] [--no-syntax-highlighting] [--no-context-aware]
```
Start Rhema's interactive interface for dynamic workflows.

**Examples:**
```bash
# Start interactive mode
rhema interactive

# Start with custom config
rhema interactive --config ./rhema-interactive.yaml
```

### Start Enhanced Interactive Mode
```bash
rhema enhanced [--config CONFIG] [--no-auto-complete] [--no-syntax-highlighting] [--no-context-aware]
```
Start enhanced interactive mode with advanced features.

## 🚀 Performance Monitoring

### Performance Commands
```bash
rhema performance <subcommand>
```

**Subcommands:**
- `start`: Start performance monitoring
- `stop`: Stop performance monitoring
- `status`: Show system performance status
- `report [--hours HOURS]`: Generate performance report
- `config`: Show monitoring configuration

**Examples:**
```bash
# Start monitoring
rhema performance start

# Generate 24-hour report
rhema performance report --hours 24

# Check status
rhema performance status
```

## ⚙️ Configuration Management

### Configuration Commands
```bash
rhema config <subcommand>
```

**Subcommands:**
- `show`: Show current configuration
- `set KEY VALUE`: Set configuration value
- `get KEY`: Get configuration value
- `reset`: Reset to default configuration
- `validate`: Validate configuration

**Examples:**
```bash
# Show configuration
rhema config show

# Set configuration
rhema config set default_scope_type service

# Validate configuration
rhema config validate
```

## 🤖 AI and Coordination

### Prompt Management
```bash
rhema prompt <subcommand>
```

**Subcommands:**
- `add "NAME" [--content CONTENT] [--tags TAGS]`
- `list [--tags TAGS]`
- `show ID`: Show prompt details
- `update ID [--name NAME] [--content CONTENT]`
- `delete ID`
- `test ID`: Test prompt effectiveness

### Context Rules Management
```bash
rhema context-rules <subcommand>
```

**Subcommands:**
- `add "RULE" [--priority PRIORITY] [--tags TAGS]`
- `list [--priority PRIORITY]`
- `show ID`: Show rule details
- `update ID [--rule RULE] [--priority PRIORITY]`
- `delete ID`

### Workflow Management
```bash
rhema workflow <subcommand>
```

**Subcommands:**
- `add "NAME" [--description DESC] [--steps STEPS]`
- `list [--tags TAGS]`
- `show ID`: Show workflow details
- `run ID`: Execute workflow
- `update ID [--name NAME] [--steps STEPS]`
- `delete ID`

### Template Management
```bash
rhema template <subcommand>
```

**Subcommands:**
- `add "NAME" [--content CONTENT] [--tags TAGS]`
- `list [--tags TAGS]`
- `show ID`: Show template details
- `update ID [--name NAME] [--content CONTENT]`
- `delete ID`
- `share ID`: Share template

### Coordination Management
```bash
rhema coordination <subcommand>
```

**Subcommands:**
- `start`: Start coordination service
- `stop`: Stop coordination service
- `status`: Show coordination status
- `agents`: List registered agents
- `register AGENT_ID`: Register agent
- `unregister AGENT_ID`: Unregister agent

### Intent Management (Action Protocol)
```bash
rhema intent <subcommand>
```

**Subcommands:**
- `propose "DESCRIPTION" [--scope SCOPE]`: Propose action
- `list [--status STATUS]`: List intents
- `show ID`: Show intent details
- `approve ID`: Approve intent
- `reject ID [--reason REASON]`: Reject intent
- `execute ID`: Execute approved intent

## 🔧 Git Integration

### Git Commands
```bash
rhema git <subcommand>
```

**Subcommands:**
- `commit [--message MESSAGE]`: Create context-aware commit
- `branch NAME [--from BRANCH]`: Create context-aware branch
- `merge BRANCH [--strategy STRATEGY]`: Merge with context awareness
- `rebase BRANCH`: Rebase with context preservation
- `hooks install`: Install Git hooks
- `hooks uninstall`: Uninstall Git hooks

## 🔒 Lock File Management

### Lock Commands
```bash
rhema lock <subcommand>
```

**Subcommands:**
- `create`: Create lock file
- `update`: Update lock file
- `validate`: Validate lock file
- `diff`: Show differences
- `resolve`: Resolve conflicts
- `clean`: Clean outdated entries

## 🎛️ MCP Daemon Management

### Daemon Commands
```bash
rhema daemon <subcommand>
```

**Subcommands:**
- `start [--config CONFIG]`: Start MCP daemon
- `stop`: Stop MCP daemon
- `status`: Show daemon status
- `restart`: Restart daemon
- `logs`: Show daemon logs

## 📊 Global Options

All commands support these global options:

- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress output
- `--help, -h`: Show help information
- `--version`: Show version information

## 🎯 Command Examples by Use Case

### Getting Started
```bash
# Initialize project
rhema init --auto-config

# Check health
rhema health

# View structure
rhema tree
```

### Daily Workflow
```bash
# Add todo
rhema todo add "Implement user authentication" --priority high

# Record insight
rhema insight record "JWT tokens work better than sessions for mobile apps"

# Check dependencies
rhema dependencies --visualize
```

### Code Review
```bash
# Bootstrap context for review
rhema bootstrap-context --use-case code_review --optimize-for-ai

# Check impact of changes
rhema impact src/auth/service.rs

# Validate all files
rhema validate --recursive
```

### Team Onboarding
```bash
# Generate primer
rhema primer --include-examples --validate

# Generate README
rhema generate-readme --include-context --seo-optimized

# Export context
rhema export-context --ai-agent-format --include-knowledge
```

### Performance Monitoring
```bash
# Start monitoring
rhema performance start

# Check status
rhema performance status

# Generate report
rhema performance report --hours 24
``` 