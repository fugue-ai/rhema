# Development


## Building


```bash
cargo build
cargo test
cargo clippy
```

## Project Structure


```
rhema/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports and CLI definitions
│   ├── commands/            # CLI command implementations
│   ├── schema/              # Specification schema definitions
│   ├── query/               # Context query engine (CQL)
│   ├── git/                 # Git integration utilities
│   ├── scope/               # Scope discovery and management
│   └── error.rs             # Error types and handling
├── schemas/
│   ├── rhema.json           # Specification JSON Schema definitions
│   └── README.md           # Schema documentation
└── tests/                  # Comprehensive test suite
```

## Testing


```bash
# Run all tests


cargo test

# Run integration tests


cargo test --test integration_tests

# Run with coverage


cargo tarpaulin

# Run performance benchmarks


cargo bench
```

## Releasing


The project uses GitHub Actions for automated releases to [crates.io](https://crates.io/crates/rhema).

**To create a new release:**

1. Update the version in `Cargo.toml`

2. Create and push a new tag:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```

3. The GitHub Action will automatically:

   - Run tests on multiple Rust versions

   - Publish the crate to crates.io

   - Create a GitHub release

**Required Secrets:**

- `CARGO_REGISTRY_TOKEN`: Your crates.io API token for publishing

The release workflow is simplified and only handles crate publishing - no deployment or complex CI/CD processes.

## Local Development with Pipeline


You can run the pull request pipeline locally using [nektos/act](https://github.com/nektos/act):

```bash
# Install act (macOS)


brew install act

# Run the complete pipeline locally


./scripts/run-pipeline-local.sh full

# Run specific jobs


./scripts/run-pipeline-local.sh test
./scripts/run-pipeline-local.sh validation

# See all available commands


./scripts/run-pipeline-local.sh help
```

For detailed instructions, see [Local Pipeline Execution](development/cicd/local-pipeline-execution.md).

## Shell-Based End-to-End Tests


Run shell-based end-to-end tests to verify CLI functionality:

```bash
# Run all shell tests


cd tests/shell
./run-tests.sh

# Run specific test


./run-tests.sh test_config_management

# List available tests


./run-tests.sh --list
```

For detailed information, see [Shell Tests Documentation](tests/shell/README.md). 