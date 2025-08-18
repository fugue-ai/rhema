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

use rhema_mcp::runtime::{choose_runtime, create_runtime, create_runtime_with_daemon, RuntimeType};
use rhema_mcp::{
    AuthConfig, CacheConfig, FileWatcherConfig, LoggingConfig, McpConfig, StartupConfig,
    WatcherConfig,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    println!("=== Rhema MCP Runtime Selection Example ===\n");

    // Example 1: HTTP Server Runtime
    println!("1. HTTP Server Runtime Example:");
    let http_config = McpConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        unix_socket: None,
        redis_url: None,
        auth: AuthConfig::default(),
        watcher: WatcherConfig::default(),
        cache: CacheConfig::default(),
        logging: LoggingConfig::default(),
        use_official_sdk: false,
        startup: StartupConfig::default(),
        max_connections: Some(1000),
    };

    let http_selection = choose_runtime(&http_config);
    println!("   Selected runtime: {:?}", http_selection.runtime_type);
    println!("   Enable HTTP: {}", http_selection.enable_http);
    println!("   Max connections: {:?}", http_selection.max_connections);
    println!();

    // Example 2: Official SDK Runtime
    println!("2. Official SDK Runtime Example:");
    let official_config = McpConfig {
        host: "127.0.0.1".to_string(),
        port: 0, // No HTTP port
        unix_socket: None,
        redis_url: Some("redis://localhost:6379".to_string()),
        auth: AuthConfig::default(),
        watcher: WatcherConfig::default(),
        cache: CacheConfig::default(),
        logging: LoggingConfig::default(),
        use_official_sdk: true,
        startup: StartupConfig::default(),
        max_connections: None,
    };

    let official_selection = choose_runtime(&official_config);
    println!("   Selected runtime: {:?}", official_selection.runtime_type);
    println!(
        "   Use official protocol: {}",
        official_selection.use_official_protocol
    );
    println!("   Enable caching: {}", official_selection.enable_caching);
    println!();

    // Example 3: Custom SDK Runtime
    println!("3. Custom SDK Runtime Example:");
    let custom_config = McpConfig {
        host: "127.0.0.1".to_string(),
        port: 0, // No HTTP port
        unix_socket: None,
        redis_url: None,
        auth: AuthConfig::default(),
        watcher: WatcherConfig::default(),
        cache: CacheConfig::default(),
        logging: LoggingConfig::default(),
        use_official_sdk: false,
        startup: StartupConfig::default(),
        max_connections: None,
    };

    let custom_selection = choose_runtime(&custom_config);
    println!("   Selected runtime: {:?}", custom_selection.runtime_type);
    println!(
        "   Use official protocol: {}",
        custom_selection.use_official_protocol
    );
    println!(
        "   Enable monitoring: {}",
        custom_selection.enable_monitoring
    );
    println!();

    // Example 4: Creating runtime instances (without actual dependencies)
    println!("4. Runtime Creation Examples:");

    // Note: In a real application, you would have actual instances of these
    // For demonstration purposes, we'll show the structure

    println!("   To create an Official SDK runtime:");
    println!("   ```rust");
    println!("   let selection = choose_runtime(&config);");
    println!("   let runtime = create_runtime(");
    println!("       &selection,");
    println!("       context_provider,");
    println!("       cache_manager,");
    println!("       file_watcher,");
    println!("       auth_manager,");
    println!("       &config");
    println!("   ).await?;");
    println!("   ```");
    println!();

    println!("   To create an HTTP Server runtime:");
    println!("   ```rust");
    println!("   let selection = choose_runtime(&config);");
    println!("   let daemon = Arc::new(McpDaemon::new(config.clone(), repo_root).await?);");
    println!("   let runtime = create_runtime_with_daemon(");
    println!("       &selection,");
    println!("       context_provider,");
    println!("       cache_manager,");
    println!("       file_watcher,");
    println!("       auth_manager,");
    println!("       &config,");
    println!("       Some(daemon)");
    println!("   ).await?;");
    println!("   ```");
    println!();

    println!("=== Runtime Selection Complete ===");
    Ok(())
}
