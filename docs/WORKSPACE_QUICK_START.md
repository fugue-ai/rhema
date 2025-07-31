# Workspace Migration Quick Start

This guide provides a quick overview of the workspace refactoring and how to get started.

## What Changed

Rhema has been refactored from a single monolithic crate to a multi-crate workspace:

**Before**: Single `src/` directory with all modules
**After**: Multiple focused crates in `crates/` directory

## Quick Migration Steps

### 1. Run the Migration Script
```bash
./scripts/migrate-to-workspace.sh
```

This will:
- Create a backup of your current `src/` directory
- Move files to appropriate crates
- Create initial `lib.rs` files

### 2. Build the Workspace
```bash
cargo build
```

### 3. Fix Compilation Errors
The migration will likely introduce import errors. You'll need to:

1. **Update imports** to use crate dependencies
2. **Add missing dependencies** to Cargo.toml files
3. **Resolve circular dependencies** by moving shared code to `rhema-core`

### 4. Test the Build
```bash
cargo test
cargo run -p rhema
```

## Crate Overview

| Crate | Purpose | Key Modules |
|-------|---------|-------------|
| `rhema-core` | Core data structures | `schema`, `scope`, `error` |
| `rhema-query` | Query engine | `query`, `repo_analysis` |
| `rhema-git` | Git integration | `git/`, `git_basic` |
| `rhema-ai` | AI services | `ai_service`, `context_injection` |
| `rhema-mcp` | MCP daemon | `mcp/` |
| `rhema-config` | Configuration | `config/`, `safety/` |
| `rhema-monitoring` | Performance | `performance`, `monitoring` |
| `rhema-integrations` | External services | `integrations/` |
| `rhema-cli` | Command line | `commands/`, `main.rs` |
| `rhema` | Main binary | Thin wrapper around CLI |

## Common Tasks

### Build a Specific Crate
```bash
cargo build -p rhema-core
cargo build -p rhema-cli
```

### Run Tests for a Crate
```bash
cargo test -p rhema-core
cargo test -p rhema-query
```

### Run the Binary
```bash
cargo run -p rhema
```

### Check Dependencies
```bash
cargo tree -p rhema
```

## Troubleshooting

### Import Errors
If you see errors like:
```
error[E0432]: unresolved import `crate::schema`
```

Update the import to use the crate dependency:
```rust
// Before
use crate::schema::*;

// After
use rhema_core::schema::*;
```

### Missing Dependencies
If you see errors about missing types, add the dependency to the crate's `Cargo.toml`:
```toml
[dependencies]
rhema-core = { path = "../core" }
```

### Circular Dependencies
If you get circular dependency errors, move shared code to `rhema-core`:
```rust
// Move common types to rhema-core
pub struct SharedType {
    // ...
}
```

## Rollback

If you need to rollback:
```bash
# Restore original structure
rm -rf src
cp -r src.backup.* src/
git checkout Cargo.toml
```

## Benefits You'll See

1. **Faster compilation** - Parallel builds of independent crates
2. **Better organization** - Clear separation of concerns
3. **Easier testing** - Test individual components
4. **Reusability** - Use specific crates in other projects

## Next Steps

1. Complete the migration by fixing all compilation errors
2. Update CI/CD pipelines to work with the workspace
3. Update documentation to reflect the new structure
4. Consider publishing individual crates to crates.io

For detailed information, see [REFACTORING_TO_WORKSPACE.md](./REFACTORING_TO_WORKSPACE.md). 