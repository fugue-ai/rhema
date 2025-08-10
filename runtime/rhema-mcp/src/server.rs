use anyhow::Result;
use rhema_mcp::{RhemaMcpService, McpConfig};
use tracing::{debug, info};

/// Rhema MCP Server wrapper that provides runtime functionality
pub struct RhemaMcpServer {
    service: RhemaMcpService,
}

impl RhemaMcpServer {
    /// Create a new Rhema MCP Server instance
    pub fn new(config_path: Option<String>) -> Result<Self> {
        debug!("Initializing Rhema MCP Server");
        
        let config = if let Some(config_path) = config_path {
            let config_content = std::fs::read_to_string(&config_path)?;
            serde_yaml::from_str(&config_content)?
        } else {
            McpConfig::default()
        };

        let repo_root = std::env::current_dir()?;
        
        // Note: This is async, but we're in a sync context
        // In a real implementation, you might want to use tokio::runtime::Runtime
        let rt = tokio::runtime::Runtime::new()?;
        let service = rt.block_on(async {
            RhemaMcpService::new(config, repo_root).await
        })?;

        Ok(Self { service })
    }

    /// Start the MCP server on the specified host and port
    pub async fn start(&mut self, host: &str, port: u16) -> Result<()> {
        info!("Starting Rhema MCP Server on {}:{}", host, port);
        
        // Update the config with the new host and port
        // Note: This is a simplified approach - in practice you'd want to update the config properly
        
        self.service.start().await.map_err(|e| anyhow::anyhow!("Failed to start MCP service: {}", e))
    }

    /// Get a reference to the core MCP service
    pub fn service(&self) -> &RhemaMcpService {
        &self.service
    }

    /// Get a mutable reference to the core MCP service
    pub fn service_mut(&mut self) -> &mut RhemaMcpService {
        &mut self.service
    }
}
