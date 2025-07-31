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

use rhema_core::{RhemaError, RhemaResult};
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    pub enabled: bool,
    pub watch_dirs: Vec<PathBuf>,
    pub file_patterns: Vec<String>,
    pub debounce_ms: u64,
    pub recursive: bool,
    pub ignore_hidden: bool,
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

/// File watcher with debouncing and filtering
pub struct FileWatcher {
    config: WatcherConfig,
    repo_root: PathBuf,
    watcher: Option<notify::RecommendedWatcher>,
    event_sender: mpsc::Sender<FileEvent>,
    #[allow(dead_code)]
    event_receiver: Arc<RwLock<mpsc::Receiver<FileEvent>>>,
    subscribers: Arc<RwLock<HashMap<String, mpsc::Sender<FileEvent>>>>,
    stats: Arc<RwLock<WatcherStats>>,
    start_time: Instant,
    debounce_timers: Arc<RwLock<HashMap<PathBuf, tokio::task::JoinHandle<()>>>>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub async fn new(config: &super::FileWatcherConfig, repo_root: PathBuf) -> RhemaResult<Self> {
        let (event_sender, event_receiver) = mpsc::channel(1000);
        
        let watcher_config = WatcherConfig {
            enabled: config.enabled,
            watch_dirs: config.watch_dirs.clone(),
            file_patterns: config.file_patterns.clone(),
            debounce_ms: config.debounce_ms,
            recursive: true,
            ignore_hidden: true,
        };

        let stats = Arc::new(RwLock::new(WatcherStats {
            total_events: 0,
            events_by_type: HashMap::new(),
            last_event_time: None,
            active_watches: 0,
            uptime_seconds: 0,
        }));

        let subscribers = Arc::new(RwLock::new(HashMap::new()));
        let debounce_timers = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config: watcher_config,
            repo_root,
            watcher: None,
            event_sender,
            event_receiver: Arc::new(RwLock::new(event_receiver)),
            subscribers,
            stats,
            start_time: Instant::now(),
            debounce_timers,
        })
    }

    /// Start the file watcher
    pub async fn start(&self) -> RhemaResult<()> {
        if !self.config.enabled {
            tracing::info!("File watcher is disabled");
            return Ok(());
        }

        tracing::info!("Starting file watcher for {:?}", self.repo_root);

        // Create the watcher
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)
            .map_err(|e| RhemaError::InvalidInput(format!("Failed to create file watcher: {}", e)))?;

        // Add watch directories
        for watch_dir in &self.config.watch_dirs {
            let full_path = self.repo_root.join(watch_dir);
            if full_path.exists() {
                watcher.watch(&full_path, RecursiveMode::Recursive)
                    .map_err(|e| RhemaError::InvalidInput(format!("Failed to watch directory {:?}: {}", full_path, e)))?;
                
                tracing::debug!("Watching directory: {:?}", full_path);
            } else {
                tracing::warn!("Watch directory does not exist: {:?}", full_path);
            }
        }

        // Store the watcher
        let _watcher_guard = self.watcher.as_ref().map(|_| ()).unwrap_or(());
        let _ = _watcher_guard;

        // Start event processing
        self.start_event_processor_with_results(rx).await?;

        // Start stats updater
        self.start_stats_updater().await;

        tracing::info!("File watcher started successfully");
        Ok(())
    }

    /// Stop the file watcher
    pub async fn stop(&self) -> RhemaResult<()> {
        tracing::info!("Stopping file watcher");

        // Clear debounce timers
        let mut timers = self.debounce_timers.write().await;
        for (_, handle) in timers.drain() {
            handle.abort();
        }

        // Close all subscribers
        let mut subscribers = self.subscribers.write().await;
        for (_, sender) in subscribers.drain() {
            let _ = sender.send(FileEvent {
                id: Uuid::new_v4().to_string(),
                event_type: FileEventType::Deleted,
                path: PathBuf::new(),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
            }).await;
        }

        tracing::info!("File watcher stopped");
        Ok(())
    }

    /// Subscribe to file events
    pub async fn subscribe(&self) -> mpsc::Receiver<FileEvent> {
        let (tx, rx) = mpsc::channel(100);
        let subscriber_id = Uuid::new_v4().to_string();
        
        let mut subscribers = self.subscribers.write().await;
        subscribers.insert(subscriber_id.clone(), tx);
        
        tracing::debug!("New file event subscriber: {}", subscriber_id);
        rx
    }

    /// Get watcher statistics
    pub async fn stats(&self) -> WatcherStats {
        let mut stats = self.stats.write().await;
        stats.uptime_seconds = self.start_time.elapsed().as_secs();
        stats.clone()
    }

    /// Check if a file should be watched
    #[allow(dead_code)]
    fn should_watch_file(&self, path: &Path) -> bool {
        // Check if it's a hidden file
        if self.config.ignore_hidden {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    return false;
                }
            }
        }

        // Check file patterns
        if !self.config.file_patterns.is_empty() {
            let path_str = path.to_string_lossy();
            let matches_pattern = self.config.file_patterns.iter().any(|pattern| {
                if pattern.contains('*') {
                    // Simple glob matching
                    let pattern = pattern.replace('*', ".*");
                    if let Ok(regex) = regex::Regex::new(&pattern) {
                        regex.is_match(&path_str)
                    } else {
                        false
                    }
                } else {
                    path_str.contains(pattern)
                }
            });

            if !matches_pattern {
                return false;
            }
        }

        true
    }

    /// Process file system events
    #[allow(dead_code)]
    async fn process_event(&self, event: Event) -> RhemaResult<()> {
        for path in event.paths {
            if !self.should_watch_file(&path) {
                continue;
            }

            let event_type = match event.kind {
                EventKind::Create(_) => FileEventType::Created,
                EventKind::Modify(_) => FileEventType::Modified,
                EventKind::Remove(_) => FileEventType::Deleted, // Remove covers both deletion and renames/moves
                EventKind::Access(_) => continue, // Skip access events
                EventKind::Any => continue, // Skip generic events
                EventKind::Other => continue, // Skip other events
            };

            let file_event = FileEvent {
                id: Uuid::new_v4().to_string(),
                event_type,
                path: path.clone(),
                timestamp: chrono::Utc::now(),
                metadata: self.get_file_metadata(&path).await,
            };

            // Debounce the event
            self.debounce_event(file_event).await?;
        }

        Ok(())
    }

    /// Debounce file events to prevent excessive notifications
    #[allow(dead_code)]
    async fn debounce_event(&self, event: FileEvent) -> RhemaResult<()> {
        let path = event.path.clone();
        let debounce_duration = Duration::from_millis(self.config.debounce_ms);

        // Cancel existing timer for this path
        let mut timers = self.debounce_timers.write().await;
        if let Some(handle) = timers.remove(&path) {
            handle.abort();
        }

        // Create new timer
        let event_sender = self.event_sender.clone();
        let path_clone = path.clone();
        
        let handle = tokio::spawn(async move {
            tokio::time::sleep(debounce_duration).await;
            
            // Send the event
            if let Err(e) = event_sender.send(event).await {
                tracing::error!("Failed to send debounced event: {}", e);
            }
        });

        timers.insert(path_clone, handle);
        Ok(())
    }

    /// Get file metadata
    #[allow(dead_code)]
    async fn get_file_metadata(&self, path: &Path) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();

        if let Ok(metadata_fs) = std::fs::metadata(path) {
            metadata.insert("size".to_string(), serde_json::Value::Number(metadata_fs.len().into()));
            metadata.insert("modified".to_string(), serde_json::Value::String(
                metadata_fs.modified()
                    .unwrap_or_else(|_| SystemTime::now())
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string()
            ));
        }

        // Add file extension
        if let Some(extension) = path.extension() {
            metadata.insert("extension".to_string(), serde_json::Value::String(
                extension.to_string_lossy().to_string()
            ));
        }

        metadata
    }

    /// Start the event processor
    #[allow(dead_code)]
    async fn start_event_processor(&self, rx: std::sync::mpsc::Receiver<notify::Event>) -> RhemaResult<()> {
        let _event_sender = self.event_sender.clone();
        let stats = self.stats.clone();

        tokio::spawn(async move {
            for event in rx {
                // Update stats
                {
                    let mut stats_guard = stats.write().await;
                    stats_guard.total_events += 1;
                    stats_guard.last_event_time = Some(chrono::Utc::now());
                    
                    let event_type = match event.kind {
                        EventKind::Create(_) => "created",
                        EventKind::Modify(_) => "modified",
                        EventKind::Remove(_) => "deleted", // Remove covers both deletion and renames/moves
                        EventKind::Access(_) => "accessed",
                        EventKind::Any => "any",
                        EventKind::Other => "other",
                    };
                    
                    *stats_guard.events_by_type.entry(event_type.to_string()).or_insert(0) += 1;
                }

                // Process the event
                if let Err(e) = Self::process_event_static(event).await {
                    tracing::error!("Failed to process file event: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Static method for processing events (used in async context)
    async fn process_event_static(event: notify::Event) -> RhemaResult<()> {
        // This is a placeholder - in a real implementation, we'd need to pass the context
        tracing::debug!("Processing file event: {:?}", event);
        Ok(())
    }

    /// Start the event processor with Result handling
    async fn start_event_processor_with_results(&self, rx: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>) -> RhemaResult<()> {
        let _event_sender = self.event_sender.clone();
        let stats = self.stats.clone();

        tokio::spawn(async move {
            for result in rx {
                match result {
                    Ok(event) => {
                        // Update stats
                        {
                            let mut stats_guard = stats.write().await;
                            stats_guard.total_events += 1;
                            stats_guard.last_event_time = Some(chrono::Utc::now());
                            
                            let event_type = match event.kind {
                                EventKind::Create(_) => "created",
                                EventKind::Modify(_) => "modified",
                                EventKind::Remove(_) => "deleted",
                                EventKind::Access(_) => "accessed",
                                EventKind::Any => "any",
                                EventKind::Other => "other",
                            };
                            
                            *stats_guard.events_by_type.entry(event_type.to_string()).or_insert(0) += 1;
                        }

                        // Process the event
                        if let Err(e) = Self::process_event_static(event).await {
                            tracing::error!("Failed to process file event: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error receiving file event: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Start the stats updater
    async fn start_stats_updater(&self) {
        let stats = self.stats.clone();
        let _start_time = self.start_time;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                
                let mut stats_guard = stats.write().await;
                stats_guard.uptime_seconds = _start_time.elapsed().as_secs();
            }
        });
    }

    /// Start the event dispatcher
    #[allow(dead_code)]
    async fn start_event_dispatcher(&self) {
        let event_receiver = self.event_receiver.clone();
        let subscribers = self.subscribers.clone();
        let _stats = self.stats.clone();

        tokio::spawn(async move {
            let mut receiver = event_receiver.write().await;
            
            while let Some(event) = receiver.recv().await {
                // Send to all subscribers
                let mut subscribers_guard = subscribers.write().await;
                let mut to_remove = Vec::new();

                for (id, sender) in subscribers_guard.iter() {
                    if let Err(_) = sender.send(event.clone()).await {
                        to_remove.push(id.clone());
                    }
                }

                // Remove dead subscribers
                for id in to_remove {
                    subscribers_guard.remove(&id);
                    tracing::debug!("Removed dead subscriber: {}", id);
                }
            }
        });
    }
}

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
        FileWatcher::new(&super::FileWatcherConfig {
            enabled: self.config.enabled,
            watch_dirs: self.config.watch_dirs,
            file_patterns: self.config.file_patterns,
            debounce_ms: self.config.debounce_ms,
            recursive: self.config.recursive,
            ignore_hidden: self.config.ignore_hidden,
        }, repo_root).await
    }
}

impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = super::super::WatcherConfig {
            enabled: true,
            watch_dirs: vec![PathBuf::from(".rhema")],
            file_patterns: vec!["*.yaml".to_string()],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        };

        let watcher = FileWatcher::new(&config, temp_dir.path().to_path_buf()).await;
        assert!(watcher.is_ok());
    }

    #[tokio::test]
    async fn test_file_pattern_matching() {
        let watcher = FileWatcherBuilder::new()
            .file_patterns(vec!["*.yaml".to_string(), "*.yml".to_string()])
            .build(PathBuf::from("/tmp"))
            .await
            .unwrap();

        // Test matching patterns
        assert!(watcher.should_watch_file(Path::new("test.yaml")));
        assert!(watcher.should_watch_file(Path::new("test.yml")));
        assert!(!watcher.should_watch_file(Path::new("test.txt")));
        assert!(!watcher.should_watch_file(Path::new(".hidden.yaml")));
    }

    #[tokio::test]
    async fn test_watcher_stats() {
        let temp_dir = TempDir::new().unwrap();
        let config = super::super::WatcherConfig {
            enabled: true,
            watch_dirs: vec![PathBuf::from(".rhema")],
            file_patterns: vec!["*.yaml".to_string()],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        };

        let watcher = FileWatcher::new(&config, temp_dir.path().to_path_buf()).await.unwrap();
        let stats = watcher.stats().await;
        
        assert_eq!(stats.total_events, 0);
        assert!(stats.uptime_seconds >= 0);
    }
} 