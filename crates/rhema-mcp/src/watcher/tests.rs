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

use super::*;
use crate::watcher::types::WatcherConfig;
use rhema_core::RhemaResult;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_file_watcher_creation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = WatcherConfig::default();
    let mcp_config = crate::mcp::WatcherConfig {
        enabled: config.enabled,
        watch_dirs: config.watch_dirs,
        file_patterns: config.file_patterns,
        debounce_ms: config.debounce_ms,
        recursive: config.recursive,
        ignore_hidden: config.ignore_hidden,
    };
    let file_watcher = FileWatcher::new(&mcp_config, temp_dir.path().to_path_buf()).await?;
    // Test that it was created successfully
    assert!(file_watcher.config().enabled == config.enabled);
    Ok(())
}

#[tokio::test]
async fn test_file_watcher_config() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let mut config = WatcherConfig::default();
    config.enabled = true;
    config.file_patterns = vec!["*.txt".to_string()];

    let mcp_config = crate::mcp::WatcherConfig {
        enabled: config.enabled,
        watch_dirs: config.watch_dirs,
        file_patterns: config.file_patterns,
        debounce_ms: config.debounce_ms,
        recursive: config.recursive,
        ignore_hidden: config.ignore_hidden,
    };

    let file_watcher = FileWatcher::new(&mcp_config, temp_dir.path().to_path_buf()).await?;
    assert!(file_watcher.config().enabled);
    assert_eq!(
        file_watcher.config().file_patterns,
        vec!["*.txt".to_string()]
    );
    Ok(())
}

#[tokio::test]
async fn test_file_watcher_stats() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = WatcherConfig::default();
    let mcp_config = crate::mcp::WatcherConfig {
        enabled: config.enabled,
        watch_dirs: config.watch_dirs,
        file_patterns: config.file_patterns,
        debounce_ms: config.debounce_ms,
        recursive: config.recursive,
        ignore_hidden: config.ignore_hidden,
    };
    let file_watcher = FileWatcher::new(&mcp_config, temp_dir.path().to_path_buf()).await?;

    let stats = file_watcher.stats().await;
    assert_eq!(stats.total_events, 0);
    assert!(stats.uptime_seconds >= 0);
    Ok(())
}
