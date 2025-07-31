# GACP MCP Daemon

The GACP MCP (Model Context Protocol) Daemon is a comprehensive daemon service that provides real-time context access to AI agents and other systems, eliminating the need for CLI-based context injection.

## Overview

The MCP daemon implements the Model Context Protocol v1.0 specification, providing:

- **Persistent background daemon process** with health monitoring
- **File system watcher** for real-time `.gacp/` directory changes
- **Context provider** with in-memory and Redis caching layers
- **Connection management** for multiple client connections
- **Authentication and security** for client access
- **JSON-RPC 2.0** communication layer
- **WebSocket support** for real-time bidirectional communication
- **HTTP/HTTPS RESTful API** endpoints
- **Unix domain sockets** for local communication optimization

## Architecture

### Core Components

1. **Daemon Service** (`src/mcp/daemon.rs`)
   - Main orchestrator for all daemon components
   - Manages HTTP, WebSocket, and Unix socket servers
   - Handles graceful shutdown and signal handling

2. **Protocol Handler** (`src/mcp/protocol.rs`)
   - Implements JSON-RPC 2.0 specification
   - Handles MCP resource management
   - Provides query execution capabilities
   - System health and information endpoints

3. **Context Provider** (`src/mcp/context.rs`)
   - Manages GACP context data (scopes, knowledge, todos, decisions, patterns)
   - Provides real-time access to context information
   - Handles CQL query execution
   - Tracks context changes and statistics

4. **Cache Manager** (`src/mcp/cache.rs`)
   - Multi-layer caching (in-memory + Redis)
   - LRU eviction policies
   - Cache statistics and monitoring
   - TTL-based expiration

5. **File Watcher** (`src/mcp/watcher.rs`)
   - Real-time file system monitoring
   - Debounced event handling
   - Pattern-based filtering
   - Change notification broadcasting

6. **Authentication Manager** (`src/mcp/auth.rs`)
   - API key authentication
   - JWT token support
   - Permission-based access control
   - CORS validation

7. **Client Library** (`src/mcp/client.rs`)
   - Rust client for daemon communication
   - Builder pattern for easy configuration
   - Support for HTTP, WebSocket, and Unix socket connections

## Installation

### Prerequisites

- Rust 1.70+ with Cargo
- Redis (optional, for distributed caching)
- Unix-like system (for Unix socket support)

### Building

```bash
# Clone the repository
git clone https://github.com/fugue-ai/gacp.git
cd gacp

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

## Usage

### Command Line Interface

The MCP daemon is integrated into the GACP CLI with the `daemon` command:

```bash
# Start the daemon
gacp daemon start

# Start with custom configuration
gacp daemon start --host 0.0.0.0 --port 8080 --auth --api-key my-secret-key

# Start with configuration file
gacp daemon start --config gacp-mcp.yaml

# Check daemon status
gacp daemon status

# Get daemon health
gacp daemon health

# Get daemon statistics
gacp daemon stats

# Stop the daemon
gacp daemon stop

# Restart the daemon
gacp daemon restart

# Generate configuration file
gacp daemon config --output gacp-mcp.yaml --comments
```

### Configuration

The daemon can be configured via YAML file or command-line arguments:

```yaml
# gacp-mcp.yaml
host: "127.0.0.1"
port: 8080

# Unix socket for local communication (optional)
unix_socket: "/tmp/gacp-mcp.sock"

# Redis configuration for distributed caching (optional)
redis_url: "redis://localhost:6379"

# Authentication settings
auth:
  enabled: false
  api_key: null
  jwt_secret: null
  allowed_origins: ["*"]

# File system watching settings
watcher:
  enabled: true
  watch_dirs: [".gacp"]
  file_patterns: ["*.yaml", "*.yml"]
  debounce_ms: 100

# Cache settings
cache:
  memory_enabled: true
  redis_enabled: false
  ttl_seconds: 3600
  max_size: 10000

# Logging settings
logging:
  level: "info"
  structured: true
  file: null
```

## API Reference

### HTTP Endpoints

#### Health Check
```http
GET /health
```

Response:
```json
{
  "status": "healthy",
  "uptime": 3600,
  "connections": 5,
  "cache_hit_rate": 0.85,
  "memory_usage": {
    "used": 1048576,
    "total": 8589934592,
    "cache_size": 524288
  }
}
```

#### System Information
```http
GET /info
```

Response:
```json
{
  "name": "GACP MCP Daemon",
  "version": "1.0.0",
  "description": "Model Context Protocol daemon for GACP",
  "capabilities": {
    "resources": true,
    "queries": true,
    "subscriptions": true
  }
}
```

#### JSON-RPC Endpoint
```http
POST /rpc
Content-Type: application/json

{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/list",
  "params": {
    "uri": "gacp://scopes"
  }
}
```

#### Resource Endpoints
```http
GET /resources
GET /resources/{uri}
GET /scopes
GET /scopes/{scope_id}
GET /scopes/{scope_id}/knowledge
GET /scopes/{scope_id}/todos
GET /scopes/{scope_id}/decisions
GET /scopes/{scope_id}/patterns
```

#### Query Endpoint
```http
POST /query
Content-Type: application/json

{
  "query": "SELECT * FROM knowledge WHERE category = 'architecture'",
  "parameters": {
    "limit": 10
  },
  "timeout_ms": 5000
}
```

#### Statistics
```http
GET /stats
```

### WebSocket API

The daemon supports WebSocket connections for real-time communication:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  // Subscribe to resource changes
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "resources/subscribe",
    params: {
      uri: "gacp://scopes"
    }
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

### Client Library

#### Rust Client

```rust
use gacp::mcp::{McpClientBuilder, ConnectionType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = McpClientBuilder::new()
        .host("127.0.0.1".to_string())
        .port(8080)
        .timeout_seconds(30)
        .api_key("my-secret-key".to_string())
        .http()
        .build();

    // Connect to daemon
    client.connect().await?;

    // Get scopes
    let scopes = client.get_scopes().await?;
    println!("Found {} scopes", scopes.len());

    // Execute query
    let result = client.execute_query(
        "SELECT * FROM knowledge WHERE confidence > 8",
        None
    ).await?;
    println!("Query result: {:?}", result);

    // Get health status
    let health = client.health().await?;
    println!("Daemon health: {}", health.status);

    // Disconnect
    client.disconnect().await?;

    Ok(())
}
```

#### Python Client (Example)

```python
import requests
import json

class GacpMcpClient:
    def __init__(self, host="127.0.0.1", port=8080, api_key=None):
        self.base_url = f"http://{host}:{port}"
        self.headers = {"Content-Type": "application/json"}
        if api_key:
            self.headers["Authorization"] = f"ApiKey {api_key}"

    def health(self):
        response = requests.get(f"{self.base_url}/health", headers=self.headers)
        return response.json()

    def execute_query(self, query, parameters=None):
        data = {"query": query}
        if parameters:
            data["parameters"] = parameters
        
        response = requests.post(
            f"{self.base_url}/query",
            headers=self.headers,
            json=data
        )
        return response.json()

    def get_scopes(self):
        response = requests.get(f"{self.base_url}/scopes", headers=self.headers)
        return response.json()

# Usage
client = GacpMcpClient(api_key="my-secret-key")
health = client.health()
print(f"Daemon status: {health['status']}")

scopes = client.get_scopes()
print(f"Found {len(scopes)} scopes")

result = client.execute_query("SELECT * FROM knowledge LIMIT 5")
print(f"Query returned {len(result['results'])} results")
```

## Resource URIs

The daemon provides access to GACP resources through standardized URIs:

### Core Resources

- `gacp://scopes` - List all scopes
- `gacp://scopes/{id}` - Get specific scope
- `gacp://scopes/{id}/knowledge` - Get scope knowledge
- `gacp://scopes/{id}/todos` - Get scope todos
- `gacp://scopes/{id}/decisions` - Get scope decisions
- `gacp://scopes/{id}/patterns` - Get scope patterns
- `gacp://query` - Execute CQL queries

### Query Language (CQL)

The daemon supports GACP Query Language (CQL) for complex queries:

```sql
-- Get all knowledge entries with high confidence
SELECT * FROM knowledge WHERE confidence > 8

-- Get todos assigned to specific user
SELECT * FROM todos WHERE assigned_to = 'john.doe'

-- Get decisions made in the last month
SELECT * FROM decisions WHERE decided_at > '2024-01-01'

-- Get patterns by type
SELECT * FROM patterns WHERE pattern_type = 'architecture'

-- Complex queries with joins
SELECT k.title, k.content, s.name as scope_name 
FROM knowledge k 
JOIN scopes s ON k.scope_id = s.id 
WHERE k.category = 'design'
```

## Performance

### Benchmarks

- **Response Time**: < 100ms for cached context access
- **Throughput**: 1000+ concurrent client connections
- **Cache Hit Rate**: > 80% for frequently accessed context
- **System Reliability**: 99.9% uptime target

### Optimization Features

- **Multi-layer Caching**: In-memory + Redis for optimal performance
- **Connection Pooling**: Efficient resource management
- **Debounced File Watching**: Minimizes unnecessary updates
- **Compression**: HTTP response compression
- **Connection Limits**: Prevents resource exhaustion

## Security

### Authentication

The daemon supports multiple authentication methods:

1. **API Key Authentication**
   ```bash
   gacp daemon start --auth --api-key "your-secret-key"
   ```

2. **JWT Token Authentication**
   ```bash
   gacp daemon start --auth --jwt-secret "your-jwt-secret"
   ```

3. **CORS Configuration**
   ```yaml
   auth:
     allowed_origins: ["https://yourdomain.com", "http://localhost:3000"]
   ```

### Security Best Practices

- Use HTTPS in production
- Implement proper API key rotation
- Configure CORS appropriately
- Use Unix sockets for local communication
- Monitor authentication logs
- Regular security updates

## Monitoring

### Health Checks

```bash
# Check daemon health
gacp daemon health

# Get detailed statistics
gacp daemon stats
```

### Metrics

The daemon exposes various metrics:

- Request count and response times
- Cache hit/miss rates
- Memory usage
- Active connections
- File system events
- Authentication success/failure rates

### Logging

Configure logging levels and output:

```yaml
logging:
  level: "info"  # debug, info, warn, error
  structured: true
  file: "/var/log/gacp-mcp.log"
```

## Deployment

### Systemd Service

Create `/etc/systemd/system/gacp-mcp.service`:

```ini
[Unit]
Description=GACP MCP Daemon
After=network.target

[Service]
Type=simple
User=gacp
WorkingDirectory=/opt/gacp
ExecStart=/usr/local/bin/gacp daemon start --config /etc/gacp/mcp.yaml
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl enable gacp-mcp
sudo systemctl start gacp-mcp
sudo systemctl status gacp-mcp
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/gacp /usr/local/bin/
EXPOSE 8080
CMD ["gacp", "daemon", "start", "--host", "0.0.0.0"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: gacp-mcp
spec:
  replicas: 1
  selector:
    matchLabels:
      app: gacp-mcp
  template:
    metadata:
      labels:
        app: gacp-mcp
    spec:
      containers:
      - name: gacp-mcp
        image: gacp/mcp:latest
        ports:
        - containerPort: 8080
        env:
        - name: GACP_CONFIG
          value: "/etc/gacp/mcp.yaml"
        volumeMounts:
        - name: config
          mountPath: /etc/gacp
      volumes:
      - name: config
        configMap:
          name: gacp-mcp-config
---
apiVersion: v1
kind: Service
metadata:
  name: gacp-mcp
spec:
  selector:
    app: gacp-mcp
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

## Troubleshooting

### Common Issues

1. **Daemon won't start**
   - Check port availability
   - Verify configuration file syntax
   - Check file permissions

2. **High memory usage**
   - Adjust cache size limits
   - Enable Redis for distributed caching
   - Monitor for memory leaks

3. **Slow response times**
   - Check cache hit rates
   - Optimize file watching patterns
   - Consider Redis for distributed caching

4. **Authentication failures**
   - Verify API keys
   - Check CORS configuration
   - Review authentication logs

### Debug Mode

Enable debug logging:

```bash
gacp daemon start --log-level debug
```

### Log Analysis

```bash
# Monitor daemon logs
tail -f /var/log/gacp-mcp.log

# Search for errors
grep ERROR /var/log/gacp-mcp.log

# Monitor performance
grep "response_time" /var/log/gacp-mcp.log
```

## Contributing

### Development Setup

```bash
# Clone repository
git clone https://github.com/fugue-ai/gacp.git
cd gacp

# Install dependencies
cargo build

# Run tests
cargo test

# Run daemon in development mode
cargo run -- daemon start --foreground --log-level debug
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test mcp_tests

# Run integration tests
cargo test --test integration_tests

# Run performance benchmarks
cargo bench
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](../LICENSE) file for details.

## Support

- **Documentation**: [https://docs.rs/gacp](https://docs.rs/gacp)
- **Issues**: [https://github.com/fugue-ai/gacp/issues](https://github.com/fugue-ai/gacp/issues)
- **Discussions**: [https://github.com/fugue-ai/gacp/discussions](https://github.com/fugue-ai/gacp/discussions) 