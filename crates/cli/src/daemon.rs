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

use crate::{RhemaError, RhemaResult};
use rhema_mcp::{
    AuthManager, CacheManager, ContextProvider, FileWatcher, McpConfig, McpDaemon,
    mcp::{AuthConfig, WatcherConfig, CacheConfig, LoggingConfig, RateLimitConfig, StartupConfig},
    DaemonStatistics,
};
use clap::Args;
use std::path::PathBuf;
use std::process;
use std::fs;
use std::os::unix::process::CommandExt;
use tokio::signal;
use tokio::time::{sleep, Duration};
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

        /// PID file path
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,

        /// Log file path
        #[arg(long, value_name = "LOG_FILE")]
        log_file: Option<PathBuf>,

        /// Working directory
        #[arg(long, value_name = "WORK_DIR")]
        work_dir: Option<PathBuf>,
    },

    /// Stop the MCP daemon
    Stop {
        /// Daemon PID file
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,

        /// Force stop (SIGKILL)
        #[arg(long)]
        force: bool,
    },

    /// Restart the MCP daemon
    Restart {
        /// Configuration file path
        #[arg(long, value_name = "CONFIG")]
        config: Option<PathBuf>,

        /// Daemon PID file
        #[arg(long, value_name = "PID_FILE", default_value = "/tmp/rhema-mcp.pid")]
        pid_file: PathBuf,

        /// Force restart
        #[arg(long)]
        force: bool,
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
            pid_file,
            log_file,
            work_dir,
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
                pid_file,
                log_file,
                work_dir,
            )
            .await
        }

        DaemonSubcommand::Stop { pid_file, force } => stop_daemon(pid_file, force).await,

        DaemonSubcommand::Restart { config, pid_file, force } => restart_daemon(config, pid_file, force).await,

        DaemonSubcommand::Status { pid_file } => status_daemon(pid_file).await,

        DaemonSubcommand::Health { host, port } => health_daemon(host, port).await,

        DaemonSubcommand::Stats { host, port } => stats_daemon(host, port).await,

        DaemonSubcommand::Config { output, comments } => generate_config(output, comments).await,
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
    pid_file: PathBuf,
    log_file: Option<PathBuf>,
    work_dir: Option<PathBuf>,
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
    initialize_logging(&config.logging, log_file.as_ref()).await?;

    // Get repository root
    let repo_root = work_dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    if !foreground {
        // Daemonize the process
        daemonize_process(&pid_file, &repo_root).await?;
    }

    // Create daemon
    let mut daemon = McpDaemon::new(config, repo_root).await?;

    // Set up signal handlers
    let mut daemon_clone = daemon.clone();
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            error!("Failed to listen for Ctrl+C: {}", e);
        }
        info!("Received shutdown signal");
        if let Err(e) = daemon_clone.stop().await {
            error!("Failed to stop daemon: {}", e);
        }
    });

    // Start health monitoring
    let daemon_health = daemon.clone();
    tokio::spawn(async move {
        health_monitoring_loop(daemon_health).await;
    });

    // Start the daemon
    daemon.start().await?;

    Ok(())
}

/// Daemonize the process
async fn daemonize_process(pid_file: &PathBuf, work_dir: &PathBuf) -> RhemaResult<()> {
    #[cfg(unix)]
    {
        use nix::unistd::{fork, ForkResult, setsid, chdir};
        use nix::sys::stat::umask;
        use nix::fcntl::{open, OFlag};
        use nix::sys::stat::Mode;

        // First fork
        match unsafe { fork().map_err(|e| RhemaError::SystemError(format!("Fork failed: {}", e)))? } {
            ForkResult::Parent { child } => {
                info!("Daemon started with PID: {}", child);
                process::exit(0);
            }
            ForkResult::Child => {
                // Create new session
                setsid().map_err(|e| RhemaError::SystemError(format!("setsid failed: {}", e)))?;
                
                // Second fork
                match unsafe { fork().map_err(|e| RhemaError::SystemError(format!("Second fork failed: {}", e)))? } {
                    ForkResult::Parent { child: _ } => {
                        process::exit(0);
                    }
                    ForkResult::Child => {
                        // Set file creation mask
                        umask(Mode::from_bits(0o022).unwrap());
                        
                        // Change to working directory
                        chdir(work_dir).map_err(|e| RhemaError::SystemError(format!("chdir failed: {}", e)))?;
                        
                        // Redirect standard file descriptors
                        let dev_null = open("/dev/null", OFlag::O_RDWR, Mode::empty())
                            .map_err(|e| RhemaError::SystemError(format!("open /dev/null failed: {}", e)))?;
                        nix::unistd::dup2(dev_null, 0)
                            .map_err(|e| RhemaError::SystemError(format!("dup2 stdin failed: {}", e)))?; // stdin
                        nix::unistd::dup2(dev_null, 1)
                            .map_err(|e| RhemaError::SystemError(format!("dup2 stdout failed: {}", e)))?; // stdout
                        nix::unistd::dup2(dev_null, 2)
                            .map_err(|e| RhemaError::SystemError(format!("dup2 stderr failed: {}", e)))?; // stderr
                        
                        // Write PID file
                        fs::write(pid_file, process::id().to_string())?;
                        
                        info!("Daemon process started successfully");
                    }
                }
            }
        }
    }

    #[cfg(not(unix))]
    {
        return Err(RhemaError::InvalidInput(
            "Daemonization not supported on this platform".to_string(),
        ));
    }

    Ok(())
}

/// Health monitoring loop
async fn health_monitoring_loop(mut daemon: McpDaemon) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        let health = daemon.health().await;
        if health.status != "healthy" {
            warn!("Daemon health check failed: {}", health.status);
            
            // Attempt restart if configured
            if let Err(e) = daemon.restart().await {
                error!("Failed to restart daemon: {}", e);
            }
        }
    }
}

/// Stop the MCP daemon
async fn stop_daemon(pid_file: PathBuf, force: bool) -> RhemaResult<()> {
    info!("Stopping Rhema MCP Daemon");

    if !pid_file.exists() {
        return Err(RhemaError::InvalidInput("PID file not found".to_string()));
    }

    let pid_content = fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_content
        .trim()
        .parse()
        .map_err(|_| RhemaError::InvalidInput("Invalid PID in file".to_string()))?;

    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        let signal = if force { Signal::SIGKILL } else { Signal::SIGTERM };
        
        kill(Pid::from_raw(pid as i32), signal)
            .map_err(|e| RhemaError::InvalidInput(format!("Failed to send signal: {}", e)))?;

        if !force {
            // Wait for graceful shutdown
            let mut attempts = 0;
            while attempts < 10 {
                sleep(Duration::from_secs(1)).await;
                if kill(Pid::from_raw(pid as i32), None).is_err() {
                    break; // Process has terminated
                }
                attempts += 1;
            }
            
            // Force kill if still running
            if attempts >= 10 {
                warn!("Process did not terminate gracefully, forcing kill");
                kill(Pid::from_raw(pid as i32), Signal::SIGKILL)
                    .map_err(|e| RhemaError::InvalidInput(format!("Failed to force kill: {}", e)))?;
            }
        }
    }

    #[cfg(not(unix))]
    {
        return Err(RhemaError::InvalidInput(
            "Process termination not supported on this platform".to_string(),
        ));
    }

    // Remove PID file
    if pid_file.exists() {
        fs::remove_file(&pid_file)?;
    }

    info!("Daemon stopped successfully");
    Ok(())
}

/// Restart the MCP daemon
async fn restart_daemon(config_path: Option<PathBuf>, pid_file: PathBuf, force: bool) -> RhemaResult<()> {
    info!("Restarting Rhema MCP Daemon");

    // Stop the daemon
    if let Err(e) = stop_daemon(pid_file.clone(), force).await {
        warn!("Failed to stop daemon: {}", e);
    }

    // Wait a moment
    sleep(Duration::from_secs(2)).await;

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
        pid_file,
        log_file: None,
        work_dir: None,
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
            pid_file,
            log_file,
            work_dir,
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
                pid_file,
                log_file,
                work_dir,
            )
            .await
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

    let pid_content = fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_content
        .trim()
        .parse()
        .map_err(|_| RhemaError::InvalidInput("Invalid PID in file".to_string()))?;

    #[cfg(unix)]
    {
        use nix::sys::signal::kill;
        use nix::unistd::Pid;

        match kill(Pid::from_raw(pid as i32), None) {
            Ok(_) => {
                println!("Daemon is running (PID: {})", pid);
                
                // Try to get additional status information
                if let Ok(health) = get_daemon_health("127.0.0.1", 8080).await {
                    println!("  Status: {}", health.status);
                    println!("  Uptime: {} seconds", health.uptime);
                    println!("  Connections: {}", health.connections);
                }
            }
            Err(_) => {
                println!("Daemon is not running (PID: {} not found)", pid);
                // Clean up stale PID file
                if let Err(e) = fs::remove_file(&pid_file) {
                    warn!("Failed to remove stale PID file: {}", e);
                }
            }
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
    
    match get_daemon_health(&host, port).await {
        Ok(health) => {
            println!("  Status: {}", health.status);
            println!("  Host: {}:{}", host, port);
            println!("  Uptime: {} seconds", health.uptime);
            println!("  Connections: {}", health.connections);
            println!("  Cache Hit Rate: {:.2}%", health.cache_hit_rate * 100.0);
            println!("  Memory Usage: {} MB / {} MB", 
                health.memory_usage.used / 1024 / 1024,
                health.memory_usage.total / 1024 / 1024);
            println!("  Request Count: {}", health.request_count);
            println!("  Error Count: {}", health.error_count);
            println!("  Error Rate: {:.2}%", health.error_rate * 100.0);
            println!("  Restart Count: {}", health.restart_count);
        }
        Err(e) => {
            println!("  Status: unhealthy");
            println!("  Error: {}", e);
        }
    }
    
    Ok(())
}

/// Get daemon health via HTTP
async fn get_daemon_health(host: &str, port: u16) -> RhemaResult<rhema_mcp::HealthStatus> {
    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/health", host, port);
    
    let response = client.get(&url)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;
    
    if response.status().is_success() {
        let health: rhema_mcp::HealthStatus = response.json().await?;
        Ok(health)
    } else {
        Err(RhemaError::InvalidInput(format!("Health check failed with status: {}", response.status())))
    }
}

/// Get daemon statistics
async fn stats_daemon(host: String, port: u16) -> RhemaResult<()> {
    println!("Daemon Statistics:");
    
    match get_daemon_stats(&host, port).await {
        Ok(stats) => {
            println!("  Host: {}:{}", host, port);
            println!("  Uptime: {} seconds", stats.uptime);
            println!("  Connection Count: {}", stats.connection_count);
            println!("  Request Count: {}", stats.request_count);
            println!("  Error Count: {}", stats.error_count);
            println!("  Error Rate: {:.2}%", stats.error_rate * 100.0);
            println!("  Cache Hit Rate: {:.2}%", stats.cache_hit_rate * 100.0);
            println!("  Restart Count: {}", stats.restart_count);
            println!("  Memory Usage: {} MB / {} MB", 
                stats.memory_usage.used / 1024 / 1024,
                stats.memory_usage.total / 1024 / 1024);
            println!("  Cache Size: {} MB", stats.memory_usage.cache_size / 1024 / 1024);
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
    
    Ok(())
}

/// Get daemon statistics via HTTP
async fn get_daemon_stats(host: &str, port: u16) -> RhemaResult<DaemonStatistics> {
    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/stats", host, port);
    
    let response = client.get(&url)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;
    
    if response.status().is_success() {
        let stats: DaemonStatistics = response.json().await?;
        Ok(stats)
    } else {
        Err(RhemaError::InvalidInput(format!("Stats request failed with status: {}", response.status())))
    }
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
  rate_limiting:
    http_requests_per_minute: 1000  # HTTP requests per minute
    websocket_messages_per_minute: 5000  # WebSocket messages per minute
    unix_socket_messages_per_minute: 10000  # Unix socket messages per minute
  audit_logging:
    enabled: false                  # Enable audit logging
    log_file: null                  # Audit log file path
    log_level: "info"               # Audit log level
    events: ["auth", "access", "error"]  # Events to audit
  security:
    brute_force_protection: true    # Enable brute force protection
    max_failed_attempts: 5          # Maximum failed attempts before lockout
    lockout_duration_seconds: 300   # Lockout duration in seconds
    security_monitoring: true       # Enable security monitoring
    token_encryption: true          # Enable token encryption at rest
    secure_headers: true            # Enable secure headers
    input_sanitization: true        # Enable input sanitization

# File system watching settings
watcher:
  enabled: true                     # Enable file system watching
  watch_dirs: [".rhema"]             # Directories to watch
  file_patterns: ["*.yaml", "*.yml"] # File patterns to watch
  debounce_ms: 100                  # Debounce interval in milliseconds
  recursive: true                   # Watch directories recursively
  ignore_hidden: true               # Ignore hidden files and directories

# Cache settings
cache:
  memory_enabled: true              # Enable in-memory caching
  redis_enabled: false              # Enable Redis caching
  redis_url: null                   # Redis connection URL
  ttl_seconds: 3600                 # Cache TTL in seconds
  max_size: 10000                   # Maximum cache size
  compression_enabled: true         # Enable cache compression

# Logging settings
logging:
  level: "info"                     # Log level (debug, info, warn, error)
  structured: true                  # Enable structured logging
  file: null                        # Log file path (optional)
  json: false                       # Enable JSON logging

# Startup settings
startup:
  graceful_shutdown_timeout: 30     # Graceful shutdown timeout in seconds
  health_check_interval: 30         # Health check interval in seconds
  connection_timeout: 60            # Connection timeout in seconds
  auto_restart: true                # Enable auto-restart on failure
  max_restart_attempts: 3           # Maximum restart attempts

# Use official MCP SDK
use_official_sdk: true

# Maximum concurrent connections
max_connections: null
"#
        .to_string()
    } else {
        serde_yaml::to_string(&config)?
    };

    fs::write(&output, content)?;
    println!("Configuration file generated: {:?}", output);

    Ok(())
}

/// Load configuration from file
async fn load_config(config_path: &PathBuf) -> RhemaResult<McpConfig> {
    let content = fs::read_to_string(config_path)?;
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
            rate_limiting: RateLimitConfig::default(),
            audit_logging: rhema_mcp::mcp::AuditLoggingConfig::default(),
            security: rhema_mcp::mcp::SecurityConfig::default(),
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
            json: false,
        },
        use_official_sdk: true,
        startup: StartupConfig::default(),
        max_connections: None,
    }
}

/// Initialize logging
async fn initialize_logging(config: &LoggingConfig, log_file: Option<&PathBuf>) -> RhemaResult<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.level));

    let mut builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false);

    // Configure log file if specified
    if let Some(log_file) = log_file {
        let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("rhema-mcp")
            .filename_suffix("log")
            .build(&log_file.parent().unwrap_or(&PathBuf::from(".")))
            .map_err(|e| RhemaError::SystemError(format!("Failed to create log file appender: {}", e)))?;
        
        // For now, just use console output since file logging setup is complex
        // TODO: Implement proper file logging setup
        builder = builder.with_ansi(false);
    }

    if config.structured {
        builder.json().init();
    } else {
        builder.init();
    }

    Ok(())
}
