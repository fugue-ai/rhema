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

use rhema_core::RhemaResult;
use std::path::PathBuf;

use super::file_watcher::FileWatcher;
use super::types::WatcherConfig;

/// File watcher builder for easy configuration
pub struct FileWatcherBuilder {
    config: WatcherConfig,
}

impl FileWatcherBuilder {
    /// Create a new file watcher builder
    pub fn new() -> Self {
        Self {
            config: WatcherConfig {
                enabled: true,
                watch_dirs: vec![PathBuf::from(".rhema")],
                file_patterns: vec!["*.yaml".to_string(), "*.yml".to_string()],
                debounce_ms: 100,
                recursive: true,
                ignore_hidden: true,
            },
        }
    }

    /// Set watch directories
    pub fn watch_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.config.watch_dirs = dirs;
        self
    }

    /// Set file patterns
    pub fn file_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.file_patterns = patterns;
        self
    }

    /// Set debounce interval
    pub fn debounce_ms(mut self, ms: u64) -> Self {
        self.config.debounce_ms = ms;
        self
    }

    /// Set recursive watching
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }

    /// Set ignore hidden files
    pub fn ignore_hidden(mut self, ignore: bool) -> Self {
        self.config.ignore_hidden = ignore;
        self
    }

    /// Build the file watcher
    pub async fn build(self, repo_root: PathBuf) -> RhemaResult<FileWatcher> {
        FileWatcher::new(
            &crate::mcp::WatcherConfig {
                enabled: self.config.enabled,
                watch_dirs: self.config.watch_dirs,
                file_patterns: self.config.file_patterns,
                debounce_ms: self.config.debounce_ms,
                recursive: self.config.recursive,
                ignore_hidden: self.config.ignore_hidden,
            },
            repo_root,
        )
        .await
    }
}

impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}
