# Model Context Protocol (MCP) Implementation

This section documents Rhema's implementation of the Model Context Protocol (MCP), providing real-time context access to AI agents and other systems through a standardized protocol.

## Overview

Rhema's MCP implementation transforms the CLI tool into a comprehensive context service that provides:

- **Real-time context access**: AI agents can access context without CLI execution
- **Persistent daemon service**: Background service with health monitoring
- **Standardized protocol**: MCP provides industry-standard integration
- **Scalable architecture**: Supports multiple clients and high load
- **Caching layer**: Fast access to frequently used context

## Architecture Components

### Daemon Service

The MCP daemon runs as a persistent background process with the following components:

```rust
pub struct RhemaDaemon {
    config: DaemonConfig,
    context_provider: Arc<ContextProvider>,
    file_watcher: Arc<FileWatcher>,
    cache: Arc<ContextCache>,
    server: Arc<McpServer>,
}
```

**Key Features:**
- Persistent background process with health monitoring
- File system watcher for real-time `.rhema/` directory changes
- Context provider with in-memory and Redis caching layers
- Connection management for multiple client connections
- Authentication and security for client access

### Protocol Implementation

Rhema implements full Model Context Protocol v1.0 compliance with:

- **JSON-RPC 2.0** communication layer
- **WebSocket** support for real-time bidirectional communication
- **HTTP/HTTPS** RESTful API endpoints
- **Unix domain sockets** for local communication optimization

### Context Resources

MCP resources are defined for Rhema context data, enabling standardized access to:
- Scope information and metadata
- Dependency relationships and constraints
- Configuration and settings
- Real-time context updates

## Implementation Details

### Protocol Compliance

Rhema's MCP implementation has been migrated to use the official `rust-mcp-sdk` for full protocol compliance:

**Dependencies:**
```toml
rust-mcp-sdk = { version = "0.5.0", features = ["server", "2025_06_18", "hyper-server"] }
rust-mcp-schema = "0.7.2"
```

**Key Features:**
- Protocol version negotiation (supports MCP protocol versions 2025-06-18)
- Server capabilities (tools, resources, prompts, completions)
- Initialize handshake with capability negotiation
- Rhema-specific tools (CQL queries, pattern search, scope management)
- Proper error handling with MCP error responses and status codes

### Server Capabilities

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: Option<ToolsCapability>,
    pub resources: Option<ResourcesCapability>,
    pub prompts: Option<PromptsCapability>,
    pub completions: Option<CompletionsCapability>,
    pub experimental: Option<Value>,
    pub logging: Option<LoggingCapability>,
}
```

## Usage

### Starting the Daemon

```bash
# Start the MCP daemon
rhema daemon start

# Start with specific configuration
rhema daemon start --config /path/to/config.toml

# Start in development mode
rhema daemon start --dev
```

### Client Integration

The MCP daemon can be accessed by any MCP-compliant client:

```rust
// Example client connection
let client = McpClient::connect("ws://localhost:8080").await?;
let context = client.get_resource("rhema://context/current").await?;
```

## Configuration

### Daemon Configuration

```toml
[daemon]
host = "127.0.0.1"
port = 8080
log_level = "info"

[cache]
type = "redis"
url = "redis://localhost:6379"
ttl = 3600

[security]
auth_required = true
allowed_origins = ["http://localhost:3000"]
```

### Protocol Configuration

```toml
[mcp]
protocol_version = "2025-06-18"
enable_websocket = true
enable_http = true
enable_unix_socket = true
```

## Monitoring and Health

The MCP daemon provides comprehensive health monitoring:

```bash
# Check daemon health
rhema daemon health

# View connection status
rhema daemon status

# Monitor performance metrics
rhema daemon metrics
```

## Related Documentation

- **[MCP Protocol Specification](./protocol.md)** - Detailed protocol implementation
- **[Daemon Configuration](./configuration.md)** - Configuration options and examples
- **[Client Integration](./client-integration.md)** - How to integrate with MCP clients
- **[Troubleshooting](./troubleshooting.md)** - Common issues and solutions 