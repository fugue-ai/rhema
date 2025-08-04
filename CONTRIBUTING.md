# Contributing

We welcome contributions to both the Rhema specification and the CLI implementation!

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