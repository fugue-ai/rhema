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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// File system event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
    Renamed,
}

/// File system event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    pub id: String,
    pub event_type: FileEventType,
    pub path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// File watcher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    /// Enable file watching
    pub enabled: bool,

    /// Directories to watch
    pub watch_dirs: Vec<PathBuf>,

    /// File patterns to watch
    pub file_patterns: Vec<String>,

    /// Debounce interval in milliseconds
    pub debounce_ms: u64,

    /// Watch recursively
    pub recursive: bool,

    /// Ignore hidden files
    pub ignore_hidden: bool,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_dirs: vec![PathBuf::from(".")],
            file_patterns: vec![
                "*.yaml".to_string(),
                "*.yml".to_string(),
                "*.json".to_string(),
            ],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        }
    }
}

/// File watcher statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherStats {
    pub total_events: u64,
    pub events_by_type: HashMap<String, u64>,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
    pub active_watches: usize,
    pub uptime_seconds: u64,
}
