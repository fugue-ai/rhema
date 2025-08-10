# Rhema MCP Server

A standalone Model Context Protocol (MCP) server for the Rhema Protocol.

## Overview

This crate provides a runtime implementation of the MCP server for Rhema. It wraps the core MCP functionality from the `rhema-mcp` crate and provides a standalone server binary that can be used to serve Rhema context data to MCP clients.

## Features

- **Standalone Server**: Run as a standalone MCP server
- **Configuration Support**: Load configuration from files
- **CLI Interface**: Command-line interface for server management
- **Logging**: Comprehensive logging with configurable levels
- **Performance**: Optimized for high-performance context serving

## Installation

```bash
cargo install --path runtime/rhema-mcp
```

## Usage

### Basic Usage

```bash
# Start the server on default port 3000
rhema-mcp-server

# Start on a specific port
rhema-mcp-server --port 8080

# Start on a specific host and port
rhema-mcp-server --host 0.0.0.0 --port 8080

# Enable debug logging
rhema-mcp-server --debug

# Use a configuration file
rhema-mcp-server --config config.yaml
```

### Command Line Options

- `-p, --port <PORT>`: Port to listen on (default: 3000)
- `--host <HOST>`: Host to bind to (default: 127.0.0.1)
- `-d, --debug`: Enable debug logging
- `-c, --config <FILE>`: Configuration file path
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Configuration

The server can be configured using a YAML configuration file:

```yaml
server:
  host: "127.0.0.1"
  port: 3000
  workers: 4

logging:
  level: "info"
  format: "json"

security:
  enabled: true
  jwt_secret: "your-secret-key"

cache:
  enabled: true
  ttl: 3600
  max_size: 1000
```

## Development

### Building

```bash
cargo build
cargo build --release
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run
cargo run --release
```

## Architecture

The runtime MCP server is built on top of the core `rhema-mcp` crate and provides:

1. **CLI Interface**: Command-line argument parsing and server configuration
2. **Server Wrapper**: High-level server management and lifecycle
3. **Logging Integration**: Structured logging with tracing
4. **Error Handling**: Comprehensive error handling and reporting

## Integration

This server can be integrated with:

- **MCP Clients**: Any MCP-compatible client
- **Development Tools**: IDEs and editors with MCP support
- **CI/CD Pipelines**: Automated testing and deployment
- **Monitoring**: Health checks and metrics collection

## License

Apache-2.0
