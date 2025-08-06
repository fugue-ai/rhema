# Rhema MCP Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-mcp)](https://crates.io/crates/rhema-mcp)
[![Documentation](https://docs.rs/rhema-mcp/badge.svg)](https://docs.rs/rhema-mcp)

Model Context Protocol (MCP) daemon implementation, SDK integration, authentication, caching, context management, and file watching for Rhema.

## Overview

The `rhema-mcp` crate provides Model Context Protocol (MCP) integration for Rhema, enabling seamless communication between Rhema and AI tools and agents. It implements the MCP specification to provide context-aware AI interactions.

## Features

### 🤖 MCP Protocol Implementation
- **Full MCP Compliance**: Complete implementation of the Model Context Protocol
- **Daemon Implementation**: MCP daemon for serving Rhema context to AI tools
- **Protocol Handshake**: Proper MCP handshake and capability negotiation
- **Resource Management**: Efficient MCP resource management and lifecycle

### 🔐 Authentication and Security
- **Authentication Manager**: Complete authentication implementation
- **Token Validation**: Secure token validation and management
- **Session Management**: Client session management and tracking
- **Access Control**: Role-based access control for MCP resources
- **Secure Communication**: Encrypted communication with clients

### 💾 Cache Management
- **Intelligent Caching**: Cache MCP responses and context data
- **Cache Eviction**: Smart cache eviction policies
- **Cache Persistence**: Persist cache across daemon restarts
- **Cache Compression**: Compress cached data for efficiency
- **Cache Monitoring**: Monitor cache performance and usage

### 📁 Context Management
- **Context Provider**: Complete context provider implementation
- **Context Validation**: Validate context data integrity
- **Context Synchronization**: Synchronize context across clients
- **Context Versioning**: Version context data for compatibility
- **Context Compression**: Compress context data for transmission

### 👀 File Watching
- **Real-time Monitoring**: Monitor file changes in real-time
- **Event Filtering**: Filter file events based on patterns
- **Event Batching**: Batch file events for efficiency
- **Event Prioritization**: Prioritize file events based on importance
- **Recovery Mechanisms**: Recover from file watching failures

### 🔧 SDK Integration
- **Official SDK**: Integration with official MCP SDK
- **SDK Compatibility**: Ensure compatibility with MCP tools
- **SDK Features**: Implement all SDK features and capabilities
- **SDK Documentation**: Comprehensive SDK documentation and examples

## Architecture

```
rhema-mcp/
├── auth.rs           # Authentication and security
├── cache.rs          # Cache management
├── context.rs        # Context management
├── daemon.rs         # MCP daemon implementation
├── file_watcher.rs   # File watching
├── protocol.rs       # MCP protocol implementation
└── sdk.rs            # SDK integration
```

## Usage

### MCP Daemon

```rust
use rhema_mcp::daemon::MCPDaemon;

let daemon = MCPDaemon::new();

// Start the daemon
daemon.start().await?;

// Register context providers
daemon.register_context_provider("rhema", rhema_context_provider)?;

// Handle client connections
daemon.handle_connections().await?;
```

### Authentication

```rust
use rhema_mcp::auth::AuthManager;

let auth_manager = AuthManager::new();

// Validate client token
let session = auth_manager.validate_token(&token)?;

// Check access permissions
if auth_manager.has_permission(&session, "read:context")? {
    // Allow access
}
```

### Cache Management

```rust
use rhema_mcp::cache::CacheManager;

let cache_manager = CacheManager::new();

// Cache context data
cache_manager.set("user-context", &context_data)?;

// Retrieve cached data
let cached_data = cache_manager.get("user-context")?;

// Get cache statistics
let stats = cache_manager.get_statistics()?;
```

### Context Management

```rust
use rhema_mcp::context::ContextManager;

let context_manager = ContextManager::new();

// Provide context for a query
let context = context_manager.provide_context("user authentication")?;

// Validate context
context_manager.validate_context(&context)?;

// Synchronize context across clients
context_manager.sync_context(&context)?;
```

### File Watching

```rust
use rhema_mcp::file_watcher::FileWatcher;

let file_watcher = FileWatcher::new();

// Watch directory for changes
file_watcher.watch_directory(".rhema")?;

// Handle file change events
file_watcher.on_change(|event| {
    println!("File changed: {:?}", event);
});

// Filter events
file_watcher.set_filter(|path| path.ends_with(".yaml"))?;
```

## Configuration

### MCP Daemon Configuration

```yaml
# .rhema/mcp.yaml
mcp:
  daemon:
    host: "localhost"
    port: 8080
    max_connections: 100
    timeout: 30s
    
  authentication:
    enabled: true
    token_expiry: 24h
    rate_limiting:
      requests_per_minute: 100
    
  cache:
    enabled: true
    max_size: "1GB"
    ttl: 1h
    compression: true
```

### Context Configuration

```yaml
mcp:
  context:
    providers:
      - name: "rhema"
        type: "local"
        path: ".rhema"
      
      - name: "git"
        type: "git"
        repository: "."
    
    validation:
      enabled: true
      strict: false
    
    compression:
      enabled: true
      algorithm: "zstd"
```

### File Watching Configuration

```yaml
mcp:
  file_watcher:
    enabled: true
    directories:
      - ".rhema"
      - "src"
    
    filters:
      include:
        - "*.yaml"
        - "*.yml"
        - "*.json"
      exclude:
        - "*.tmp"
        - "*.log"
    
    events:
      batch_size: 10
      batch_timeout: 100ms
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **serde**: Serialization support
- **tokio**: Async runtime
- **notify**: File watching
- **tracing**: Logging and tracing
- **uuid**: Unique identifier generation
- **chrono**: Date and time handling

## Development Status

### ✅ Completed Features
- Basic MCP protocol implementation
- Authentication framework
- Cache management infrastructure
- Context provider framework

### 🔄 In Progress
- Complete MCP protocol compliance
- Advanced authentication features
- File watching implementation
- SDK integration

### 📋 Planned Features
- Advanced security features
- Performance optimization
- Monitoring and observability
- Enterprise features

#### Integration Features
- [ ] Add support for multiple LLM providers
- [ ] Implement webhook notifications
- [ ] Add support for external authentication providers
- [ ] Implement real-time collaboration features
- [ ] Add support for custom plugins/extensions

#### Monitoring & Observability
- [ ] Implement comprehensive logging
- [ ] Add metrics collection and export
- [ ] Implement health checks for all components
- [ ] Add distributed tracing
- [ ] Implement alerting and notification systems

#### Documentation & Testing
- [ ] Add comprehensive API documentation
- [ ] Implement integration tests
- [ ] Add performance benchmarks
- [ ] Create deployment guides
- [ ] Add troubleshooting documentation 

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all MCP operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 