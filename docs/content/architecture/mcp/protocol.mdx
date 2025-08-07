# MCP Protocol Implementation

This document provides detailed information about Rhema's Model Context Protocol (MCP) implementation, including protocol compliance, message formats, and integration details.

## Protocol Compliance

Rhema's MCP implementation is fully compliant with the official Model Context Protocol specification and uses the official `rust-mcp-sdk` for protocol handling.

### Supported Protocol Versions

- **MCP Protocol 2025-06-18**: Full support for the latest protocol version
- **Backward Compatibility**: Support for previous protocol versions where possible

### Dependencies

```toml
[dependencies]
rust-mcp-sdk = { version = "0.5.0", features = ["server", "2025_06_18", "hyper-server"] }
rust-mcp-schema = "0.7.2"
```

## Server Capabilities

Rhema's MCP server implements the following capabilities:

### Tools Capability

Provides access to Rhema-specific tools for context management:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    pub listChanged: Option<bool>,
}
```

**Available Tools:**
- `rhema.query` - Execute CQL queries against Rhema context
- `rhema.scope.list` - List available scopes
- `rhema.scope.get` - Get specific scope information
- `rhema.knowledge.search` - Search knowledge entries
- `rhema.todo.list` - List todos and tasks
- `rhema.decision.list` - List decisions and architectural choices

### Resources Capability

Provides access to Rhema context resources:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesCapability {
    pub listChanged: Option<bool>,
    pub resourceChanged: Option<bool>,
}
```

**Available Resources:**
- `rhema://scopes` - List of all scopes
- `rhema://scopes/{id}` - Specific scope information
- `rhema://scopes/{id}/knowledge` - Scope knowledge entries
- `rhema://scopes/{id}/todos` - Scope todos and tasks
- `rhema://scopes/{id}/decisions` - Scope decisions
- `rhema://scopes/{id}/patterns` - Scope patterns

### Prompts Capability

Provides access to Rhema prompt patterns and templates:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsCapability {
    pub listChanged: Option<bool>,
}
```

**Available Prompts:**
- `rhema://prompts/context` - Context injection prompts
- `rhema://prompts/analysis` - Analysis and review prompts
- `rhema://prompts/development` - Development workflow prompts

## Message Formats

### Initialize Request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-06-18",
    "capabilities": {
      "tools": {},
      "resources": {},
      "prompts": {}
    },
    "clientInfo": {
      "name": "rhema-client",
      "version": "1.0.0"
    }
  }
}
```

### Initialize Response

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2025-06-18",
    "capabilities": {
      "tools": {
        "listChanged": true
      },
      "resources": {
        "listChanged": true,
        "resourceChanged": true
      },
      "prompts": {
        "listChanged": true
      }
    },
    "serverInfo": {
      "name": "rhema-mcp",
      "version": "1.0.0"
    }
  }
}
```

### Tool Call Request

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "rhema.query",
    "arguments": {
      "query": "SELECT * FROM knowledge WHERE category = 'architecture'",
      "limit": 10
    }
  }
}
```

### Tool Call Response

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Found 5 architecture knowledge entries"
      }
    ]
  }
}
```

### Resource List Request

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "resources/list",
  "params": {
    "uri": "rhema://scopes"
  }
}
```

### Resource List Response

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "resources": [
      {
        "uri": "rhema://scopes/core",
        "name": "Core Scope",
        "description": "Core Rhema functionality",
        "mimeType": "application/json"
      },
      {
        "uri": "rhema://scopes/agent",
        "name": "Agent Scope",
        "description": "AI agent capabilities",
        "mimeType": "application/json"
      }
    ]
  }
}
```

## Error Handling

Rhema's MCP implementation provides comprehensive error handling with proper MCP error codes:

### Error Response Format

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "details": "Query parameter is required"
    }
  }
}
```

### Common Error Codes

- **-32600**: Invalid Request
- **-32601**: Method not found
- **-32602**: Invalid params
- **-32603**: Internal error
- **-32700**: Parse error

### Rhema-Specific Error Codes

- **-32001**: Scope not found
- **-32002**: Query execution failed
- **-32003**: Resource access denied
- **-32004**: Invalid CQL syntax

## Integration Examples

### Rust Client Example

```rust
use rust_mcp_sdk::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::default()
        .with_server_url("ws://localhost:8080");
    
    let client = Client::new(config).await?;
    
    // Initialize connection
    client.initialize().await?;
    
    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);
    
    // Execute a query
    let result = client.call_tool("rhema.query", serde_json::json!({
        "query": "SELECT * FROM knowledge LIMIT 5"
    })).await?;
    
    println!("Query result: {:?}", result);
    
    Ok(())
}
```

### Python Client Example

```python
import asyncio
from mcp import ClientSession, StdioServerParameters

async def main():
    async with ClientSession(StdioServerParameters(
        command="rhema",
        args=["daemon", "start"]
    )) as session:
        # Initialize
        await session.initialize()
        
        # List tools
        tools = await session.list_tools()
        print(f"Available tools: {tools}")
        
        # Call a tool
        result = await session.call_tool("rhema.query", {
            "query": "SELECT * FROM knowledge LIMIT 5"
        })
        print(f"Query result: {result}")

asyncio.run(main())
```

### JavaScript Client Example

```javascript
import { Client } from '@modelcontextprotocol/sdk/client/index.js';

async function main() {
    const client = new Client({
        server: {
            command: 'rhema',
            args: ['daemon', 'start']
        }
    });
    
    await client.initialize();
    
    // List tools
    const tools = await client.listTools();
    console.log('Available tools:', tools);
    
    // Call a tool
    const result = await client.callTool('rhema.query', {
        query: 'SELECT * FROM knowledge LIMIT 5'
    });
    console.log('Query result:', result);
}

main().catch(console.error);
```

## Performance Considerations

### Optimization Features

- **Connection Pooling**: Efficient management of client connections
- **Request Batching**: Support for batched requests to reduce overhead
- **Caching**: Built-in caching for frequently accessed resources
- **Compression**: Automatic compression of large responses

### Performance Metrics

- **Latency**: < 50ms for typical operations
- **Throughput**: 1000+ concurrent connections
- **Memory Usage**: Optimized for minimal memory footprint
- **CPU Usage**: Efficient async/await implementation

## Security

### Authentication

Rhema's MCP implementation supports multiple authentication methods:

- **API Key Authentication**: Simple key-based authentication
- **JWT Token Authentication**: Token-based authentication with expiration
- **Unix Socket Authentication**: File system-based authentication for local connections

### Authorization

- **Resource-Level Access Control**: Fine-grained permissions for different resources
- **Tool-Level Permissions**: Control over which tools can be executed
- **Scope-Based Access**: Access control based on scope membership

### Transport Security

- **TLS/SSL**: Encrypted communication over HTTPS/WSS
- **Unix Sockets**: Secure local communication
- **CORS Configuration**: Configurable cross-origin resource sharing

## Troubleshooting

### Common Issues

1. **Connection Failures**
   - Check if daemon is running
   - Verify port availability
   - Check firewall settings

2. **Authentication Errors**
   - Verify API keys or tokens
   - Check authentication configuration
   - Review access permissions

3. **Protocol Errors**
   - Ensure client supports correct protocol version
   - Check message format compliance
   - Verify capability negotiation

### Debug Mode

Enable debug logging for detailed protocol information:

```bash
rhema daemon start --log-level debug --protocol-debug
```

### Log Analysis

Monitor protocol communication:

```bash
# Monitor protocol messages
tail -f /var/log/rhema-mcp.log | grep "protocol"

# Check for errors
grep "ERROR" /var/log/rhema-mcp.log

# Monitor performance
grep "latency" /var/log/rhema-mcp.log
``` 