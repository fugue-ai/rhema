# Support

We love your feedback! We want to make contributing to Rhema as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## We Use GitHub to Host Code, to Track Issues and Feature Requests, as Well as Accept Pull Requests

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests. Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## We Use [GitHub Issues](https://github.com/fugue-ai/rhema/issues) to Track Public Bugs

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/fugue-ai/rhema/issues/new); it's that easy!

## Write Bug Reports with Detail, Background, and Sample Code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Use a Consistent Coding Style

* Use 2 spaces for indentation rather than tabs
* You can try running `npm run lint` for style unification

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## References

This document was adapted from the open-source contribution guidelines for [Facebook's Draft](https://github.com/facebook/draft-js/blob/a9316a723f9e918afde44dea68b5f9f39b7d9b00/CONTRIBUTING.md).

## Getting Help

If you need help with Rhema, here are some resources:

### Documentation
- [Architecture Guide](ARCHITECTURE.md) - Understanding Rhema's architecture
- [API Documentation](docs/) - Comprehensive API documentation
- [Examples](examples/) - Code examples and tutorials

### Community
- [GitHub Discussions](https://github.com/fugue-ai/rhema/discussions) - Ask questions and share ideas
- [GitHub Issues](https://github.com/fugue-ai/rhema/issues) - Report bugs and request features

### Development Setup
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to Rhema
- [Development Environment](docs/DEVELOPMENT.md) - Setting up your development environment

### Troubleshooting

#### Common Issues

**Build Failures**
- Ensure you have Rust 1.70+ installed
- Run `cargo clean` and try building again
- Check that all dependencies are properly installed

**Test Failures**
- Run tests with `cargo test` to see detailed output
- Check that your environment variables are set correctly
- Ensure you have the required system dependencies

**Runtime Errors**
- Check the logs for detailed error messages
- Verify your configuration files are valid
- Ensure all required services are running

#### Still Need Help?

If you're still having trouble:

1. Search existing [issues](https://github.com/fugue-ai/rhema/issues) to see if your problem has already been reported
2. Check [GitHub Discussions](https://github.com/fugue-ai/rhema/discussions) for similar questions
3. Create a new issue with:
   - A clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment details (OS, Rust version, etc.)
   - Any relevant error messages or logs

We're here to help and want to make Rhema as accessible as possible!
