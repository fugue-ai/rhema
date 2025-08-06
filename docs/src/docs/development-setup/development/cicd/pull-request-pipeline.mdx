# Pull Request Pipeline


This document describes the pull request pipeline for the Rhema project, which provides comprehensive testing, validation, and security checks for all pull requests.

## Overview


The pull request pipeline is defined in `.github/workflows/pull-request.yml` and includes the following components:

- **Test Suite**: Runs unit tests, integration tests, and comprehensive test suites

- **Rhema Validation**: Validates Rhema files, checks health, and validates schemas

- **Security Audit**: Runs security audits and secret scanning

- **Code Quality**: Generates code coverage reports

- **Performance Tests**: Runs performance tests and benchmarks

- **Pipeline Summary**: Provides a summary of all job results

## Automatic Triggers


The pipeline automatically runs on:

- Pull requests to `main` or `develop` branches

- Pull request events: `opened`, `synchronize`, `reopened`, `ready_for_review`

## Manual Trigger


You can manually trigger the pipeline using the GitHub Actions UI:

1. Go to the **Actions** tab in your GitHub repository

2. Select **Pull Request Pipeline** from the workflows list

3. Click **Run workflow**

4. Configure the following options:

   - **Run all tests**: Enable/disable test execution

   - **Run security checks**: Enable/disable security audits

   - **Run Rhema validation**: Enable/disable Rhema validation

## Pipeline Jobs


### 1. Test Suite


Runs on multiple Rust versions (stable, 1.75) and includes:

- Code formatting check (`cargo fmt --check`)

- Linting with clippy (`cargo clippy`)

- Unit tests (`cargo test`)

- Integration tests (`cargo test --test integration`)

- Comprehensive test suite (`cargo test --test comprehensive_test_suite`)

- Release build (`cargo build --release`)

**Artifacts**: Compiled CLI binaries for each Rust version

### 2. Rhema Validation


Validates the Rhema (Git-based AI Context Protocol) implementation:

- Builds and installs the Rhema CLI

- Validates Rhema files recursively

- Checks Rhema health status

- Lists available Rhema scopes

- Validates schemas

**Dependencies**: Requires Test Suite to complete successfully

### 3. Security Audit


Performs comprehensive security checks:

- Cargo security audit (`cargo audit`)

- Secret scanning with TruffleHog

- Additional cargo-audit with advisory database

### 4. Code Quality


Generates code coverage and quality metrics:

- Installs cargo-tarpaulin for coverage analysis

- Generates HTML and XML coverage reports

- Uploads coverage to Codecov

- Stores coverage artifacts for 30 days

### 5. Performance Tests


Runs performance-related tests:

- Performance test suite

- Benchmark compilation (without execution)

### 6. Pipeline Summary


Provides a comprehensive summary of all job results:

- Lists the status of each job

- Indicates overall pipeline success/failure

- Uses GitHub's step summary feature for better visibility

## Configuration


### Environment Variables


The pipeline uses the following environment variables:

```yaml
CARGO_TERM_COLOR: always
RUST_BACKTRACE: 1
RUST_LOG: info
RHEMA_RUN_INTEGRATION_TESTS: 1  # For integration tests
```

### Caching


The pipeline implements efficient caching for:

- Cargo registry (`~/.cargo/registry`)

- Git dependencies (`~/.cargo/git`)

- Build artifacts (`target`)

Cache keys are based on:

- Operating system

- Rust version

- Cargo.lock file hash

## Troubleshooting


### Common Issues


1. **Test Failures**: Check the test logs for specific failure reasons

2. **Validation Errors**: Ensure Rhema files are properly formatted

3. **Security Warnings**: Review cargo-audit output for dependency vulnerabilities

4. **Performance Issues**: Check benchmark results for regressions

### Manual Debugging


To debug pipeline issues:

1. Use the manual trigger with specific options disabled

2. Check the artifacts section for build outputs

3. Review the pipeline summary for overall status

4. Examine individual job logs for detailed error messages

## Integration with Other Workflows


This pipeline complements the existing release workflow (`.github/workflows/release.yml`) by providing comprehensive testing before releases. The release workflow focuses on publishing the CLI to Crates.io, while this pipeline ensures code quality and security.

## Best Practices


1. **Always run the pipeline** before merging pull requests

2. **Review security warnings** and address critical vulnerabilities

3. **Monitor performance metrics** for regressions

4. **Use manual triggers** for testing specific configurations

5. **Check coverage reports** to maintain code quality standards 