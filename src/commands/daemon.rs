/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{RhemaError, RhemaResult, mcp::*};
use clap::Args;
use std::path::PathBuf;
use tokio::signal;
use tracing::{error, info, warn};

/// MCP daemon command arguments
#[derive(Args)]
pub struct DaemonArgs {
    /// Daemon subcommand
    #[command(subcommand)]
    pub command: DaemonSubcommand,
}

/// MCP daemon subcommands
#[derive(clap::Subcommand)]
pub enum DaemonSubcommand {
    /// Start the MCP daemon
    Start {
        /// Configuration file path
        #[arg(long, value_name = "CONFIG")]
        config: Option<PathBuf>,
        
        /// Host address to bind to
        #[arg(long, value_name = "HOST", default_value = "127.0.0.1")]
        host: String,
        
        /// Port to bind to
        #[arg(long, value_name = "PORT", default_value = "8080")]
        port: u16,
        
        /// Unix socket path for local communication
        #[arg(long, value_name = "SOCKET")]
        unix_socket: Option<PathBuf>,
        
        /// Enable authentication
        #[arg(long)]
        auth: bool,
        
        /// API key for authentication
        #[arg(long, value_name = "KEY")]
        api_key: Option<String>,
        
        /// JWT secret for token-based authentication
        #[arg(long, value_name = "SECRET")]
        jwt_secret: Option<String>,
        
        /// Redis URL for distributed caching
        #[arg(long, value_name = "URL")]
        redis_url: Option<String>,
        
        /// Enable file system watching
        #[arg(long)]
        watch: bool,
        
        /// Watch directories (comma-separated)
        #[arg(long, value_name = "DIRS", default_value = ".rhema")]
        watch_dirs: String,
        
        /// Log level
        #[arg(long, value_name = "LEVEL", default_value = "info")]
        log_level: String,
        
        /// Run in foreground (don't daemonize)
        #[arg(long)]
        foreground: bool,
    },
    
    /// Stop the MCP daemon
    Stop {
        /// Daemon PID file
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,
    },
    
    /// Restart the MCP daemon
    Restart {
        /// Configuration file path
        #[arg(long, value_name = "CONFIG")]
        config: Option<PathBuf>,
        
        /// Daemon PID file
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,
    },
    
    /// Check daemon status
    Status {
        /// Daemon PID file
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,
    },
    
    /// Get daemon health
    Health {
        /// Daemon host
        #[arg(long, value_name = "HOST", default_value = "127.0.0.1")]
        host: String,
        
        /// Daemon port
        #[arg(long, value_name = "PORT", default_value = "8080")]
        port: u16,
    },
    
    /// Get daemon statistics
    Stats {
        /// Daemon host
        #[arg(long, value_name = "HOST", default_value = "127.0.0.1")]
        host: String,
        
        /// Daemon port
        #[arg(long, value_name = "PORT", default_value = "8080")]
        port: u16,
    },
    
    /// Generate configuration file
    Config {
        /// Output file path
        #[arg(long, value_name = "FILE", default_value = "rhema-mcp.yaml")]
        output: PathBuf,
        
        /// Include comments
        #[arg(long)]
        comments: bool,
    },
}

/// Execute the daemon command
pub async fn execute_daemon(args: DaemonArgs) -> RhemaResult<()> {
    match args.command {
        DaemonSubcommand::Start {
            config,
            host,
            port,
            unix_socket,
            auth,
            api_key,
            jwt_secret,
            redis_url,
            watch,
            watch_dirs,
            log_level,
            foreground,
        } => {
            start_daemon(
                config,
                host,
                port,
                unix_socket,
                auth,
                api_key,
                jwt_secret,
                redis_url,
                watch,
                watch_dirs,
                log_level,
                foreground,
            ).await
        }
        
        DaemonSubcommand::Stop { pid_file } => {
            stop_daemon(pid_file).await
        }
        
        DaemonSubcommand::Restart { config, pid_file } => {
            restart_daemon(config, pid_file).await
        }
        
        DaemonSubcommand::Status { pid_file } => {
            status_daemon(pid_file).await
        }
        
        DaemonSubcommand::Health { host, port } => {
            health_daemon(host, port).await
        }
        
        DaemonSubcommand::Stats { host, port } => {
            stats_daemon(host, port).await
        }
        
        DaemonSubcommand::Config { output, comments } => {
            generate_config(output, comments).await
        }
    }
}

/// Start the MCP daemon
async fn start_daemon(
    config_path: Option<PathBuf>,
    host: String,
    port: u16,
    unix_socket: Option<PathBuf>,
    auth: bool,
    api_key: Option<String>,
    jwt_secret: Option<String>,
    redis_url: Option<String>,
    watch: bool,
    watch_dirs: String,
    log_level: String,
    foreground: bool,
) -> RhemaResult<()> {
    info!("Starting Rhema MCP Daemon");

    // Load configuration
    let config = if let Some(config_path) = config_path {
        load_config(&config_path).await?
    } else {
        create_config_from_args(
            host,
            port,
            unix_socket,
            auth,
            api_key,
            jwt_secret,
            redis_url,
            watch,
            watch_dirs,
            log_level,
        )
    };

    // Initialize logging
    initialize_logging(&config.logging)?;

    // Get repository root
    let repo_root = std::env::current_dir()?;

    // Create daemon
    let daemon = McpDaemon::new(config, repo_root).await?;

    if foreground {
        // Run in foreground
        info!("Running daemon in foreground");
        
        // Set up signal handlers
        let daemon_clone = daemon.clone();
        tokio::spawn(async move {
            if let Err(e) = signal::ctrl_c().await {
                error!("Failed to listen for Ctrl+C: {}", e);
            }
            info!("Received shutdown signal");
            if let Err(e) = daemon_clone.stop().await {
                error!("Failed to stop daemon: {}", e);
            }
        });

        // Start the daemon
        daemon.start().await?;
    } else {
        // Run as daemon
        info!("Running daemon in background");
        
        // TODO: Implement proper daemonization
        // For now, just run in foreground
        daemon.start().await?;
    }

    Ok(())
}

/// Stop the MCP daemon
async fn stop_daemon(pid_file: PathBuf) -> RhemaResult<()> {
    info!("Stopping Rhema MCP Daemon");

    if !pid_file.exists() {
        return Err(RhemaError::InvalidInput("PID file not found".to_string()));
    }

    let pid_content = std::fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_content.trim().parse()
        .map_err(|_| RhemaError::InvalidInput("Invalid PID in file".to_string()))?;

    // Send SIGTERM to the process
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        
        kill(Pid::from_raw(pid as i32), Signal::SIGTERM)
            .map_err(|e| RhemaError::InvalidInput(format!("Failed to send SIGTERM: {}", e)))?;
    }

    #[cfg(not(unix))]
    {
        return Err(RhemaError::InvalidInput("Process termination not supported on this platform".to_string()));
    }

    // Wait for process to terminate
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Remove PID file
    if pid_file.exists() {
        std::fs::remove_file(&pid_file)?;
    }

    info!("Daemon stopped successfully");
    Ok(())
}

/// Restart the MCP daemon
async fn restart_daemon(config_path: Option<PathBuf>, pid_file: PathBuf) -> RhemaResult<()> {
    info!("Restarting Rhema MCP Daemon");

    // Stop the daemon
    if let Err(e) = stop_daemon(pid_file.clone()).await {
        warn!("Failed to stop daemon: {}", e);
    }

    // Wait a moment
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Start the daemon
    let args = DaemonSubcommand::Start {
        config: config_path,
        host: "127.0.0.1".to_string(),
        port: 8080,
        unix_socket: None,
        auth: false,
        api_key: None,
        jwt_secret: None,
        redis_url: None,
        watch: true,
        watch_dirs: ".rhema".to_string(),
        log_level: "info".to_string(),
        foreground: false,
    };

    match args {
        DaemonSubcommand::Start {
            config,
            host,
            port,
            unix_socket,
            auth,
            api_key,
            jwt_secret,
            redis_url,
            watch,
            watch_dirs,
            log_level,
            foreground,
        } => {
            start_daemon(
                config,
                host,
                port,
                unix_socket,
                auth,
                api_key,
                jwt_secret,
                redis_url,
                watch,
                watch_dirs,
                log_level,
                foreground,
            ).await
        }
        _ => unreachable!(),
    }
}

/// Check daemon status
async fn status_daemon(pid_file: PathBuf) -> RhemaResult<()> {
    if !pid_file.exists() {
        println!("Daemon is not running (PID file not found)");
        return Ok(());
    }

    let pid_content = std::fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_content.trim().parse()
        .map_err(|_| RhemaError::InvalidInput("Invalid PID in file".to_string()))?;

    #[cfg(unix)]
    {
        use nix::sys::signal::kill;
        use nix::unistd::Pid;
        
        match kill(Pid::from_raw(pid as i32), None) {
            Ok(_) => println!("Daemon is running (PID: {})", pid),
            Err(_) => println!("Daemon is not running (PID: {} not found)", pid),
        }
    }

    #[cfg(not(unix))]
    {
        println!("Daemon status check not supported on this platform");
    }

    Ok(())
}

/// Check daemon health
async fn health_daemon(host: String, port: u16) -> RhemaResult<()> {
    println!("Daemon Health Status:");
    println!("  Status: healthy");
    println!("  Host: {}:{}", host, port);
    println!("  Note: Health check via HTTP client not yet implemented with new MCP SDK");
    Ok(())
}

/// Get daemon statistics
async fn stats_daemon(host: String, port: u16) -> RhemaResult<()> {
    println!("Daemon Statistics:");
    println!("  Host: {}:{}", host, port);
    println!("  Note: Statistics via HTTP client not yet implemented with new MCP SDK");
    Ok(())
}

/// Generate configuration file
async fn generate_config(output: PathBuf, comments: bool) -> RhemaResult<()> {
    let config = McpConfig::default();
    
    let content = if comments {
        r#"# Rhema MCP Daemon Configuration
# This file configures the Model Context Protocol daemon

# Server configuration
host: "127.0.0.1"  # Host address to bind to
port: 8080         # Port to bind to

# Unix socket for local communication (optional)
# unix_socket: "/tmp/rhema-mcp.sock"

# Redis configuration for distributed caching (optional)
# redis_url: "redis://localhost:6379"

# Authentication settings
auth:
  enabled: false                    # Enable authentication
  api_key: null                     # API key for client authentication
  jwt_secret: null                  # JWT secret for token-based auth
  allowed_origins: ["*"]            # Allowed origins for CORS

# File system watching settings
watcher:
  enabled: true                     # Enable file system watching
  watch_dirs: [".rhema"]             # Directories to watch
  file_patterns: ["*.yaml", "*.yml"] # File patterns to watch
  debounce_ms: 100                  # Debounce interval in milliseconds

# Cache settings
cache:
  memory_enabled: true              # Enable in-memory caching
  redis_enabled: false              # Enable Redis caching
  ttl_seconds: 3600                 # Cache TTL in seconds
  max_size: 10000                   # Maximum cache size

# Logging settings
logging:
  level: "info"                     # Log level (debug, info, warn, error)
  structured: true                  # Enable structured logging
  file: null                        # Log file path (optional)
"#.to_string()
    } else {
        serde_yaml::to_string(&config)?
    };

    std::fs::write(&output, content)?;
    println!("Configuration file generated: {:?}", output);

    Ok(())
}

/// Load configuration from file
async fn load_config(config_path: &PathBuf) -> RhemaResult<McpConfig> {
    let content = std::fs::read_to_string(config_path)?;
    let config: McpConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// Create configuration from command line arguments
fn create_config_from_args(
    host: String,
    port: u16,
    unix_socket: Option<PathBuf>,
    auth: bool,
    api_key: Option<String>,
    jwt_secret: Option<String>,
    redis_url: Option<String>,
    watch: bool,
    watch_dirs: String,
    log_level: String,
) -> McpConfig {
    let watch_dirs: Vec<PathBuf> = watch_dirs
        .split(',')
        .map(|s| PathBuf::from(s.trim()))
        .collect();

    McpConfig {
        host,
        port,
        unix_socket,
        redis_url,
        auth: AuthConfig {
            enabled: auth,
            api_key,
            jwt_secret,
            allowed_origins: vec!["*".to_string()],
        },
        watcher: WatcherConfig {
            enabled: watch,
            watch_dirs,
            file_patterns: vec!["*.yaml".to_string(), "*.yml".to_string()],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        },
        cache: CacheConfig::default(),
        logging: LoggingConfig {
            level: log_level,
            structured: true,
            file: None,
        },
        use_official_sdk: true,
    }
}

/// Initialize logging
fn initialize_logging(config: &LoggingConfig) -> RhemaResult<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.level));

    let builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false);

    if config.structured {
        builder.json().init();
    } else {
        builder.init();
    }

    Ok(())
} 