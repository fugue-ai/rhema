# Shell-Based End-to-End Tests

This directory contains shell-based end-to-end tests for the Rhema CLI. These tests verify the complete functionality of the CLI from a user's perspective, including configuration management, CLI commands, and integration scenarios.

## Overview

Shell tests are designed to:
- **Test complete workflows** - End-to-end scenarios that users would perform
- **Verify CLI functionality** - Test actual command execution and output
- **Check configuration management** - Validate configuration files and settings
- **Test integration scenarios** - Verify interactions between different components

## Available Tests

### `test_config_management.sh`
Tests the configuration management system including:
- Configuration module compilation
- CLI command integration
- Configuration file validation
- Feature availability checks

## Running Tests

### Run All Tests
```bash
cd tests/shell
./run-tests.sh
```

### Run Specific Test
```bash
cd tests/shell
./run-tests.sh test_config_management
```

### List Available Tests
```bash
cd tests/shell
./run-tests.sh --list
```

### Get Help
```bash
cd tests/shell
./run-tests.sh --help
```

## Test Structure

Each test script should follow this structure:

```bash
#!/bin/bash

# Test description and purpose
echo "🧪 Testing [Feature Name]"
echo "========================"

# Prerequisites check
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ cargo is required but not installed"
    exit 1
fi

# Test steps
echo "📦 Testing [specific functionality]..."
# ... test logic ...

# Results
if [ $? -eq 0 ]; then
    echo "✅ Test passed"
else
    echo "❌ Test failed"
    exit 1
fi
```

## Writing New Tests

### Guidelines

1. **Descriptive Names** - Use clear, descriptive names (e.g., `test_config_management.sh`)
2. **Self-Contained** - Tests should be independent and not rely on other tests
3. **Clear Output** - Use emojis and clear status messages
4. **Proper Exit Codes** - Return 0 for success, non-zero for failure
5. **Prerequisites Check** - Verify required tools are available
6. **Cleanup** - Clean up any temporary files or state changes

### Template

```bash
#!/bin/bash

# Test: [Test Name]
# Purpose: [Brief description of what this test verifies]

set -e

echo "🧪 Testing [Test Name]"
echo "====================="

# Check prerequisites
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ cargo is required but not installed"
    exit 1
fi

# Test setup
echo "📦 Setting up test environment..."
# ... setup code ...

# Run test
echo "🔍 Running test..."
# ... test logic ...

# Verify results
if [ $? -eq 0 ]; then
    echo "✅ Test passed: [specific success message]"
else
    echo "❌ Test failed: [specific failure message]"
    exit 1
fi

echo "🎯 Test completed successfully!"
```

## Integration with CI/CD

These shell tests can be integrated into the CI/CD pipeline by:

1. **Adding to GitHub Actions** - Include shell test execution in workflows
2. **Local Development** - Run tests before committing changes
3. **Pre-release Checks** - Verify functionality before releases

### Example CI Integration

```yaml
# In .github/workflows/pull-request.yml
- name: Run Shell Tests
  run: |
    cd tests/shell
    ./run-tests.sh
```

## Troubleshooting

### Common Issues

1. **Permission Denied** - Make sure test scripts are executable:
   ```bash
   chmod +x tests/shell/*.sh
   ```

2. **Missing Dependencies** - Ensure required tools are installed:
   ```bash
   # Check cargo
   cargo --version
   
   # Check git
   git --version
   ```

3. **Test Failures** - Check the test output for specific error messages and verify the expected functionality

### Debugging

- Use `set -x` at the beginning of a test script for verbose output
- Add `echo "Debug: [message]"` statements for debugging
- Check exit codes and error messages carefully

## Best Practices

1. **Keep Tests Focused** - Each test should verify one specific feature or workflow
2. **Use Descriptive Names** - Test names should clearly indicate what they test
3. **Handle Errors Gracefully** - Provide clear error messages and cleanup
4. **Test Real Scenarios** - Focus on actual user workflows rather than edge cases
5. **Maintain Independence** - Tests should not depend on each other or external state

## Contributing

When adding new shell tests:

1. Follow the naming convention: `test_[feature_name].sh`
2. Include a clear description in the script header
3. Add the test to this README
4. Ensure the test passes locally before submitting
5. Consider adding the test to CI/CD workflows if appropriate 