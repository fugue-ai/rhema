# Rhema Project Justfile
# A comprehensive task runner for the Rhema Git-Based Agent Context Protocol

# Default recipe - show available tasks
default:
    @echo "=== Rhema Project Tasks ==="
    @echo ""
    @echo "🔧 BUILD TASKS:"
    @echo "  build          - Build Rust workspace"
    @echo "  build-ts       - Build TypeScript components"
    @echo "  build-all      - Build everything (Rust + TypeScript)"
    @echo ""
    @echo "🧪 TEST TASKS:"
    @echo "  test           - Run all tests (Rust + TypeScript)"
    @echo "  test-rust      - Run Rust tests only"
    @echo "  test-ts        - Run TypeScript tests only"
    @echo ""
    @echo "📋 OTHER TASKS:"
    @echo "  help           - Show all available tasks"
    @echo "  status         - Show project status"
    @echo ""

# =============================================================================
# RUST TASKS
# =============================================================================

# Build the entire workspace
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Check if code compiles without producing artifacts
check:
    cargo check

# Run clippy linter
clippy:
    cargo clippy

# Run clippy with all warnings as errors
clippy-strict:
    cargo clippy -- -D warnings

# Format Rust code
fmt:
    cargo fmt

# Check formatting without making changes
fmt-check:
    cargo fmt -- --check

# Run Rust tests only
test-rust:
    cargo test

# Run Rust tests with output
test-rust-verbose:
    cargo test -- --nocapture

# Run Rust tests in release mode
test-rust-release:
    cargo test --release

# Run specific Rust test file
test-rust-file file:
    cargo test --test {{file}}

# Run Rust integration tests
test-rust-integration:
    cargo test --test integration

# Run Rust unit tests only
test-rust-unit:
    cargo test --lib

# Run Rust tests with coverage
test-rust-coverage:
    cargo test --coverage

# Generate documentation
doc:
    cargo doc --no-deps --open

# Generate documentation for all workspace members
doc-workspace:
    cargo doc --workspace --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Audit dependencies for security issues
audit:
    cargo audit

# =============================================================================
# TYPESCRIPT/NODE.JS TASKS
# =============================================================================

# Install all dependencies
install:
    pnpm install

# Build TypeScript components
build-ts:
    pnpm run build:typescript

# Build VSCode extension
build-vscode:
    pnpm run build:vscode

# Build language server
build-language-server:
    pnpm run build:language-server

# Build documentation
build-docs:
    pnpm run build:docs

# Clean TypeScript build artifacts
clean-ts:
    pnpm run clean:typescript

# Test TypeScript components only
test-ts:
    pnpm run test:typescript

# Test VSCode extension
test-vscode:
    pnpm run test:vscode

# Test language server
test-language-server:
    pnpm run test:language-server

# Start language server in development mode
start-language-server:
    pnpm run start:language-server

# =============================================================================
# COMBINED TASKS
# =============================================================================

# Build everything (Rust + TypeScript)
build-all:
    pnpm run build:all

# Clean everything
clean-all:
    pnpm run clean:all

# Test everything (Rust + TypeScript) - DEFAULT TEST TARGET
test:
    @echo "🧪 Running all tests..."
    @echo "📦 Testing TypeScript components..."
    pnpm run test:typescript
    @echo "🦀 Testing Rust components..."
    cargo test
    @echo "✅ All tests completed!"

# Format all code (Rust + TypeScript)
fmt-all:
    cargo fmt
    pnpm run docs:format

# Check formatting for all code
fmt-check-all:
    cargo fmt -- --check
    pnpm run docs:format

# Lint all code
lint-all:
    cargo clippy
    pnpm run docs:lint

# =============================================================================
# DEVELOPMENT TASKS
# =============================================================================

# Run the main binary
run:
    cargo run --bin rhema

# Run with specific arguments
run-with args:
    cargo run --bin rhema -- {{args}}

# Run in release mode
run-release:
    cargo run --release --bin rhema

# Run tests in watch mode (requires cargo-watch)
watch-test:
    cargo watch -x test

# Run clippy in watch mode
watch-clippy:
    cargo watch -x clippy

# Run the daemon
run-daemon:
    cargo run --bin test_daemon

# Run the simple daemon
run-daemon-simple:
    cargo run --bin test_daemon_simple

# =============================================================================
# DOCUMENTATION TASKS
# =============================================================================

# Start documentation development server
docs-dev:
    pnpm run docs:dev

# Build documentation
docs-build:
    pnpm run docs:build

# Preview built documentation
docs-preview:
    pnpm run docs:preview

# Test documentation
docs-test:
    pnpm run docs:test

# Check documentation
docs-check:
    pnpm run docs:check

# Install documentation dependencies
docs-install:
    pnpm run docs:install

# Clean documentation build
docs-clean:
    pnpm run docs:clean

# =============================================================================
# PIPELINE TASKS
# =============================================================================

# Run all pipeline checks
pipeline-all:
    pnpm run pipeline:all

# Run affected pipeline checks
pipeline-affected-all:
    pnpm run pipeline:affected:all

# Run pipeline tests
pipeline-test:
    pnpm run pipeline:test

# Run pipeline build
pipeline-build:
    pnpm run pipeline:build

# Run pipeline checks
pipeline-check:
    pnpm run pipeline:check

# Run pipeline clippy
pipeline-clippy:
    pnpm run pipeline:clippy

# Run pipeline format check
pipeline-fmt-check:
    pnpm run pipeline:fmt:check

# =============================================================================
# DOCKER TASKS
# =============================================================================

# Build Docker image
docker-build:
    docker build -t rhema .

# Build MCP Docker image
docker-build-mcp:
    docker build -f Dockerfile.mcp -t rhema-mcp .

# Run Docker container
docker-run:
    docker run -it rhema

# Run Docker Compose
docker-compose-up:
    docker-compose up -d

# Stop Docker Compose
docker-compose-down:
    docker-compose down

# =============================================================================
# UTILITY TASKS
# =============================================================================

# Show project status
status:
    @echo "=== Rust Status ==="
    cargo check --quiet || echo "❌ Rust check failed"
    @echo "=== TypeScript Status ==="
    pnpm run status:typescript

# Show dependency tree
deps:
    cargo tree

# Show workspace members
workspace:
    cargo metadata --format-version 1 | jq '.workspace_members'

# Generate lock file
lock:
    cargo generate-lockfile

# Check for outdated dependencies
outdated:
    cargo outdated

# =============================================================================
# BENCHMARK TASKS
# =============================================================================

# Run benchmarks
bench:
    cargo bench

# Run specific benchmark
bench-specific name:
    cargo bench {{name}}

# =============================================================================
# RELEASE TASKS
# =============================================================================

# Prepare for release
release-prep:
    cargo update
    cargo audit
    cargo clippy -- -D warnings
    cargo test --release
    pnpm run test:typescript

# Create a new release
release:
    just release-prep
    @echo "Ready for release!"

# =============================================================================
# HELP TASKS
# =============================================================================

# Show available tasks
help:
    @just --list

# Show task descriptions
help-detailed:
    @just --list --unsorted 