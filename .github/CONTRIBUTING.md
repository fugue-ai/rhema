# Contributing

We welcome contributions to both the Rhema specification and the CLI implementation!

## Prerequisites

- Rust 1.70 or later
- Node.js 18.12+ (for documentation and tooling)
- Git
- pnpm 10.14.0+ (for package management)

## Development Tools

This project uses a modern development stack:

- **pnpm 10** - Fast, disk space efficient package manager
- **Nx** - Monorepo build system with intelligent caching
- **Biome** - Fast linter and formatter for TypeScript/JavaScript
- **Vitest** - Fast unit testing framework
- **SvelteKit** - Full-stack web framework for documentation

## Building from Source

```bash
# Clone the repository
git clone https://github.com/fugue-ai/rhema.git
cd rhema

# Build the project
cargo build --release

# Run tests
cargo test

# Run integration tests
RHEMA_RUN_INTEGRATION_TESTS=1 cargo test --test integration
```

## Development Setup

See the [Development Setup Guide](docs/src/docs/development-setup/development.md) for detailed instructions on setting up your development environment.

## Development Workflow

```bash
# Install dependencies
pnpm install

# Start documentation development server
pnpm docs:dev

# Run linting and formatting
pnpm format          # Format all code
pnpm lint:biome      # Lint all code
pnpm check           # Run all checks

# Run tests
pnpm test            # Run all tests
pnpm test:ui         # Run tests with UI
pnpm test:coverage   # Run tests with coverage

# Build projects
pnpm build           # Build all projects
pnpm docs:build      # Build documentation only

# Nx monorepo commands
npx nx graph         # View project dependencies
npx nx affected --target=test  # Test affected projects
```

## Documentation Commands

```bash
./docs.sh dev      # Start development server
./docs.sh build    # Build for production
./docs.sh preview  # Preview production build
./docs.sh install  # Install dependencies
./docs.sh migrate  # Run migration script
./docs.sh clean    # Clean build artifacts
./docs.sh help     # Show all commands
```

## Nx Commands (Monorepo Management)

```bash
npx nx graph              # Visualize project dependencies
npx nx show projects      # List all projects
npx nx run docs:build     # Build specific project
npx nx run-many --target=build  # Build all projects
npx nx affected --target=test   # Test only affected projects
```

## Docker Deployment

For containerized deployment and development, see the [Docker Directory](docker/README.md) which contains:

- **Dockerfile** - Main Rhema application container
- **Dockerfile.mcp** - MCP daemon container  
- **docker-compose.yml** - Complete development/production environment

Quick start with Docker:
```bash
cd docker
docker-compose up -d
```

## Contributing to the Specification

1. Review the current specification schemas in `schemas/`
2. Propose changes through GitHub issues
3. Update JSON schemas and documentation
4. Ensure backward compatibility or provide migration paths

## Contributing to the CLI

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Submit a pull request

## Contributing to Documentation

1. Install dependencies: `pnpm install`
2. Start development server: `pnpm docs:dev`
3. Edit markdown files in `docs/src/docs/`
4. View changes at `http://localhost:5173`
5. Run linting: `pnpm lint:biome`
6. Run formatting: `pnpm format`
7. Submit a pull request

## Development Guidelines

### Rust Development
- Follow Rust best practices and style guidelines
- Add comprehensive tests for new features
- Ensure schema validation works correctly
- Consider performance implications for large repositories

### TypeScript/JavaScript Development
- Use Biome for linting and formatting
- Write tests with Vitest
- Follow SvelteKit best practices for documentation
- Use Nx for monorepo management

### General Guidelines
- Update documentation for specification changes
- Ensure all tests pass before submitting PRs
- Use conventional commit messages
- Consider the impact on the monorepo structure 