# Test Directory Organization

This directory contains all tests for the Rhema project, organized by category for better maintainability and discovery.

## Directory Structure

### `unit/`
Unit tests for individual components and functions.
- `basic.rs` - Basic functionality tests
- `unit_tests.rs` - Core unit tests
- `send_sync_test.rs` - Thread safety tests
- `interactive_parser_tests.rs` - Parser unit tests
- `resolver_tests.rs` - Dependency resolver tests
- `validation_command_tests.rs` - Command validation tests
- `health_command_tests.rs` - Health check tests
- `enhanced_dependencies_tests.rs` - Dependency management tests
- `interactive_tests.rs` - Interactive mode tests
- `cql_improvements.rs` - Query language improvements

### `integration/`
Integration tests that test multiple components working together.
- `coordination_tests.rs` - Coordination system tests
- `command_tests.rs` - Command integration tests
- `search_integration_tests.rs` - Search functionality tests
- `coordination_integration_test.rs` - Coordination integration
- `pattern_*_tests.rs` - Pattern matching and execution tests
- `rag_cache_integration_test.rs` - RAG cache integration
- `cache_integration_test.rs` - Cache system integration
- `lock_file_ai_integration_test.rs` - AI lock file integration
- `context_bootstrapping_tests.rs` - Context initialization tests
- `batch_operations_tests.rs` - Batch operation tests
- `integration_tests.rs` - General integration tests

### `automation/`
Tests for automation features and workflows.
- `automation_integration_tests.rs` - Automation integration
- `feature_automation_tests.rs` - Feature branch automation
- `hotfix_automation_tests.rs` - Hotfix automation
- `release_automation_tests.rs` - Release automation
- `performance_monitoring_tests.rs` - Performance monitoring

### `git/`
Git-related functionality tests.
- `git_workflow_integration_test.rs` - Git workflow integration
- `git_workflow_test.rs` - Git workflow tests
- `git_integration_tests.rs` - Git integration tests
- `git_integration_demo.rs` - Git integration demos

### `mcp/`
Model Context Protocol tests.
- `mcp_protocol_compliance_test.rs` - MCP compliance tests
- `mcp_migration_tests.rs` - MCP migration tests

### `config/`
Configuration management tests.
- `config_management_tests.rs` - Configuration management
- `lock_config_tests.rs` - Lock configuration tests
- `test_config.rs` - Configuration test utilities
- `test_auto_config.rs` - Auto-configuration tests
- `scope_file_locations.rs` - Scope file location tests

### `validation/`
Validation and conflict resolution tests.
- `comprehensive_validation_tests.rs` - Comprehensive validation
- `conflict_resolution_cli_test.rs` - Conflict resolution CLI

### `security/`
Security-related tests.
- `security_tests.rs` - General security tests
- `coordination_security_tests.rs` - Coordination security

### `performance/`
Performance and benchmark tests.
- `benchmark_tests.rs` - General benchmarks
- `coordination_benchmarks.rs` - Coordination benchmarks
- `enhanced_benchmarks.rs` - Enhanced benchmarks
- `http_server_performance_tests.rs` - HTTP server performance
- `search_benchmarks.rs` - Search benchmarks

### `runners/`
Test runner and test suite files.
- `test_runner.rs` - Main test runner
- `coordination_test_runner.rs` - Coordination test runner
- `comprehensive_test_suite.rs` - Comprehensive test suite

### `daemon/`
Daemon-related tests.
- `test_daemon.rs` - Daemon tests
- `test_daemon_simple.rs` - Simple daemon tests

### `common/`
Common test utilities, fixtures, and helpers.
- `mod.rs` - Common module organization
- `assertions.rs` - Test assertions
- `coordination_fixtures.rs` - Coordination test fixtures
- `enhanced_fixtures.rs` - Enhanced test fixtures
- `enhanced_mocks.rs` - Enhanced mocks
- `fixtures.rs` - Basic test fixtures
- `generators.rs` - Test data generators
- `helpers.rs` - Test helper functions
- `mocks.rs` - Mock objects

### `shell/`
Shell script tests.
- `run-tests.sh` - Test execution scripts
- `test_config_management.sh` - Config management tests

### `tla/`
TLA+ specification and verification files.
- Various TLA+ specification files for formal verification

## Running Tests

To run all tests:
```bash
cargo test
```

To run tests in a specific category:
```bash
cargo test --test unit
cargo test --test integration
cargo test --test automation
```

To run a specific test file:
```bash
cargo test --test path/to/test_file
```

## Adding New Tests

When adding new tests, please place them in the appropriate category directory and update the corresponding `mod.rs` file to include the new module. 