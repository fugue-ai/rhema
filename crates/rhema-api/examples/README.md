# Rhema API Examples

This directory contains comprehensive examples and test suites for the Rhema API crate.

## Examples

### `simple_coordination_example.rs`
Demonstrates basic coordination system usage with Rhema API.

### `unit_tests.rs`
Comprehensive unit tests for the main Rhema API functionality including:
- Rhema initialization
- API input validation
- Query execution with error recovery
- Scope discovery and management
- Performance monitoring
- Security features
- Error handling
- Concurrent operations

### `init_unit_tests.rs` ⭐ **NEW**
Comprehensive unit test suite for the `init.rs` module functionality.

## Running the Tests

### Run All Examples
```bash
cargo run --example unit_tests
cargo run --example init_unit_tests
cargo run --example simple_coordination_example
```

### Run Specific Tests
```bash
# Run only init unit tests
cargo run --example init_unit_tests

# Run with verbose output
RUST_LOG=debug cargo run --example init_unit_tests
```

## Init Unit Tests Overview

The `init_unit_tests.rs` file provides comprehensive testing for the Rhema initialization module, covering:

### Test Categories

1. **Basic Initialization Tests**
   - `test_basic_init_at_repo_root()` - Tests initialization at repository root
   - `test_basic_init_in_subdirectory()` - Tests initialization in subdirectories

2. **Auto-Configuration Tests**
   - `test_auto_configuration()` - Tests repository analysis and auto-detection
   - Repository structure analysis
   - Project type detection
   - Framework and language detection

3. **Error Handling Tests**
   - `test_error_handling_existing_files()` - Tests proper error handling when files already exist
   - `test_edge_cases_and_errors()` - Tests edge cases and error conditions

4. **Custom Configuration Tests**
   - `test_custom_scope_type_and_name()` - Tests custom scope type and name specification
   - `test_init_with_different_scope_types()` - Tests various scope types

5. **Template Creation Tests**
   - `test_template_file_creation()` - Tests creation of all template files
   - `test_protocol_info_generation()` - Tests protocol information generation

6. **Integration Tests**
   - `test_rhema_integration()` - Tests integration with Rhema instance
   - `test_performance_and_concurrent_init()` - Tests performance and concurrent operations

7. **Special Character Tests**
   - `test_init_with_special_characters()` - Tests special characters in names
   - `test_init_with_unicode_characters()` - Tests unicode character support

### Test Fixture

The `InitTestFixture` struct provides:
- Temporary repository creation
- Git repository initialization
- Repository structure setup for testing
- File verification utilities
- Template content validation

### Key Features Tested

- ✅ Scope initialization at repository root and subdirectories
- ✅ Auto-configuration with repository analysis
- ✅ Custom scope type and name specification
- ✅ Template file creation (knowledge.yaml, todos.yaml, etc.)
- ✅ Protocol information generation
- ✅ Error handling for existing files
- ✅ Edge cases and error conditions
- ✅ Integration with Rhema instance
- ✅ Performance and concurrent initialization
- ✅ Special character and unicode support
- ✅ Different scope types (service, library, application, etc.)

### Test Output

The test suite provides detailed output showing:
- Test progress with emoji indicators
- Performance timing information
- Detailed error messages for failures
- Success confirmation for each test category

### Running Individual Tests

You can also run individual test functions by modifying the main function:

```rust
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Run only specific tests
    test_basic_init_at_repo_root().await?;
    test_auto_configuration().await?;
    Ok(())
}
```

### Debugging Tests

For debugging, you can enable verbose logging:

```bash
RUST_LOG=debug cargo run --example init_unit_tests
```

This will show detailed information about file operations, repository analysis, and error conditions.

## Contributing

When adding new functionality to the `init.rs` module, please add corresponding tests to `init_unit_tests.rs` to ensure comprehensive coverage. 