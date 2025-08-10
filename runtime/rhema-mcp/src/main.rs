use clap::Parser;
use rhema_mcp_server::RhemaMcpServer;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "rhema-mcp-server")]
#[command(about = "Model Context Protocol server for the Rhema Protocol")]
#[command(version)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    info!("Starting Rhema MCP Server...");
    info!("Listening on {}:{}", cli.host, cli.port);

    // Create and start the MCP server
    let mut server = RhemaMcpServer::new(cli.config)?;
    
    match server.start(&cli.host, cli.port).await {
        Ok(_) => {
            info!("MCP Server stopped gracefully");
            Ok(())
        }
        Err(e) => {
            error!("MCP Server failed: {}", e);
            Err(e)
        }
    }
}
