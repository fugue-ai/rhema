# Refactoring Rhema to Multi-Crate Workspace


This document outlines the refactoring of Rhema from a monolithic single-crate structure to a multi-crate workspace architecture.

## Overview


The refactoring aims to:

- **Improve compilation times** by reducing dependencies between modules

- **Enhance maintainability** by separating concerns into focused crates

- **Increase reusability** by allowing other projects to depend on specific crates

- **Better organization** of code by logical domains

- **Easier testing** of individual components

## Architecture


### Crate Structure


```
rhema/
├── Cargo.toml (workspace root)
├── crates/
│   ├── core/           # Core data structures and fundamental operations
│   ├── query/          # Query engine and search functionality
│   ├── git/            # Advanced Git integration and workflows
│   ├── ai/             # AI service integration and context injection
│   ├── mcp/            # Model Context Protocol daemon functionality
│   ├── config/         # Configuration management, validation, and migration
│   ├── monitoring/     # Performance monitoring and analytics
│   ├── integrations/   # Third-party service integrations
│   ├── cli/            # Command line interface and interactive mode
│   └── rhema/          # Main binary (thin wrapper around cli)
└── src/                # Original monolithic source (to be migrated)
```

### Dependencies


```
rhema (binary)
└── rhema
    ├── rhema-core
    ├── rhema-query
    │   └── rhema-core
    ├── rhema-git
    │   └── rhema-core
    ├── rhema-ai
    │   ├── rhema-core
    │   └── rhema-query
    ├── rhema-mcp
    │   ├── rhema-core
    │   └── rhema-query
    ├── rhema-config
    │   └── rhema-core
    ├── rhema-monitoring
    │   └── rhema-core
    └── rhema-integrations
        └── rhema-core
```

## Crate Details


### rhema-core


**Purpose**: Core data structures, schemas, and fundamental operations

**Modules**:

- `error.rs` - Error types and result aliases

- `schema.rs` - YAML schema definitions

- `scope.rs` - Scope management

- `file_ops.rs` - File operations

- `git_basic.rs` - Basic Git operations

**Dependencies**: Minimal external dependencies (serde, git2, anyhow)

### rhema-query


**Purpose**: Query engine and search functionality

**Modules**:

- `query.rs` - CQL query execution

- `repo_analysis.rs` - Repository analysis

**Dependencies**: rhema-core, regex, walkdir, rayon

### rhema-git


**Purpose**: Advanced Git integration and workflows

**Modules**:

- `git/` - Advanced Git operations

- `git_basic.rs` - Re-exported from core

**Dependencies**: rhema-core, git2, reqwest, notify

### rhema-ai


**Purpose**: AI service integration and context injection

**Modules**:

- `ai_service.rs` - AI service integration

- `context_injection.rs` - Context injection logic

- `agent/` - Agent coordination

**Dependencies**: rhema-core, rhema-query, redis, actix-web, prometheus

### rhema-mcp


**Purpose**: Model Context Protocol daemon functionality

**Modules**:

- `mcp/` - MCP daemon implementation

**Dependencies**: rhema-core, rhema-query, axum, tower-http, async-trait

### rhema-config


**Purpose**: Configuration management, validation, and migration

**Modules**:

- `config/` - Configuration management

- `safety/` - Safety and validation

**Dependencies**: rhema-core, validator, toml

### rhema-monitoring


**Purpose**: Performance monitoring and analytics

**Modules**:

- `performance.rs` - Performance monitoring

- `monitoring.rs` - Health monitoring

**Dependencies**: rhema-core, prometheus, tracing, dashmap

### rhema-integrations


**Purpose**: Third-party service integrations

**Modules**:

- `integrations/` - External service integrations

**Dependencies**: rhema-core, reqwest

### rhema


**Purpose**: Command line interface and interactive mode

**Modules**:

- `commands/` - All CLI commands

- `main.rs` - Main CLI logic

**Dependencies**: All other crates, clap, rustyline

### rhema (binary)


**Purpose**: Main executable

**Dependencies**: rhema

## Migration Process


### Phase 1: Setup (Completed)


- [x] Create workspace Cargo.toml

- [x] Create crate directories and Cargo.toml files

- [x] Create migration script

- [x] Create documentation

### Phase 2: File Migration


- [ ] Run migration script to move files

- [ ] Create lib.rs files for each crate

- [ ] Update import statements

- [ ] Fix compilation errors

### Phase 3: Testing and Validation


- [ ] Ensure all tests pass

- [ ] Verify functionality

- [ ] Update CI/CD pipelines

- [ ] Update documentation

### Phase 4: Cleanup


- [ ] Remove original src directory

- [ ] Update README and documentation

- [ ] Update release process

## Benefits


### Compilation Performance


- **Parallel compilation**: Each crate can be compiled in parallel

- **Incremental builds**: Changes to one crate don't require recompiling others

- **Reduced dependency graph**: Smaller dependency trees per crate

### Development Experience


- **Focused development**: Work on one domain without affecting others

- **Better IDE support**: Smaller codebases are easier for IDEs to analyze

- **Clearer boundaries**: Explicit dependencies between modules

### Reusability


- **Selective dependencies**: Other projects can depend on specific crates

- **Library usage**: Individual crates can be used as libraries

- **API stability**: Each crate can have its own versioning

### Maintenance


- **Easier testing**: Test individual components in isolation

- **Clearer ownership**: Each crate has a clear responsibility

- **Reduced coupling**: Changes in one area don't affect others

## Migration Commands


### Run Migration Script


```bash
./scripts/migrate-to-workspace.sh
```

### Build Workspace


```bash
cargo build
```

### Test Workspace


```bash
cargo test
```

### Build Specific Crate


```bash
cargo build -p rhema-core
cargo build -p rhema
```

### Run Binary


```bash
cargo run -p rhema
```

## Troubleshooting


### Common Issues


1. **Import Errors**: Update import statements to use crate dependencies

2. **Circular Dependencies**: Resolve by moving shared code to core

3. **Missing Dependencies**: Add required dependencies to Cargo.toml

4. **Test Failures**: Update test imports and dependencies

### Rollback Plan


If issues arise, the original source is backed up in `src.backup.*`:
```bash
# Restore original structure


rm -rf src
cp -r src.backup.* src/
git checkout Cargo.toml
```

## Future Considerations


### Potential Further Refactoring


- **rhema-web**: Web interface and API

- **rhema-plugins**: Plugin system

- **rhema-sdk**: SDK for external integrations

### Versioning Strategy


- **Independent versioning**: Each crate can have its own version

- **Workspace versioning**: Keep all crates on same version for simplicity

- **Breaking changes**: Coordinate breaking changes across crates

### Documentation


- **Crate-level docs**: Each crate should have its own documentation

- **API docs**: Generate documentation for each crate

- **Examples**: Provide examples for each crate

## Conclusion


This refactoring will significantly improve the maintainability and performance of the Rhema project while making it more modular and reusable. The migration process is designed to be incremental and reversible, ensuring minimal disruption to development. 