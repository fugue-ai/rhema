use crate::{Rhema, RhemaError, RhemaResult};
use super::commands::WatchSubcommands;
use colored::*;
use std::path::PathBuf;
use tracing::info;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedFile {
    path: PathBuf,
    watched_since: DateTime<Utc>,
    last_modified: DateTime<Utc>,
    event_count: u64,
    semantic_indexed: bool,
    file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedDirectory {
    path: PathBuf,
    watched_since: DateTime<Utc>,
    patterns: Vec<String>,
    ignore_patterns: Vec<String>,
    total_files: u64,
    matching_files: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    path: PathBuf,
    event_type: FileEventType,
    timestamp: DateTime<Utc>,
    file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchStats {
    total_events: u64,
    files_modified: u64,
    files_created: u64,
    files_deleted: u64,
    files_renamed: u64,
    average_response_time_ms: u64,
    last_event_time: DateTime<Utc>,
    total_watched_files: u64,
    total_watched_directories: u64,
}

struct FileWatcher {
    watched_files: Arc<RwLock<HashMap<PathBuf, WatchedFile>>>,
    watched_directories: Arc<RwLock<HashMap<PathBuf, WatchedDirectory>>>,
    file_events: Arc<RwLock<Vec<FileEvent>>>,
    stats: Arc<RwLock<WatchStats>>,
    pattern_cache: Arc<RwLock<HashMap<String, Regex>>>,
}

impl FileWatcher {
    fn new() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            watched_directories: Arc::new(RwLock::new(HashMap::new())),
            file_events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(WatchStats {
                total_events: 0,
                files_modified: 0,
                files_created: 0,
                files_deleted: 0,
                files_renamed: 0,
                average_response_time_ms: 0,
                last_event_time: Utc::now(),
                total_watched_files: 0,
                total_watched_directories: 0,
            })),
            pattern_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn watch_file(&self, file_path: PathBuf, index: bool) -> RhemaResult<()> {
        let mut files = self.watched_files.write()
            .map_err(|e| RhemaError::InternalError(format!("File watcher lock error: {}", e)))?;
        
        let metadata = std::fs::metadata(&file_path)
            .map_err(|e| RhemaError::FileSystemError(format!("Failed to get file metadata: {}", e)))?;
        
        let watched_file = WatchedFile {
            path: file_path.clone(),
            watched_since: Utc::now(),
            last_modified: DateTime::from_timestamp(
                metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs() as i64, 0
            ).unwrap_or_else(|| Utc::now()),
            event_count: 0,
            semantic_indexed: index,
            file_size: metadata.len(),
        };
        
        files.insert(file_path, watched_file);
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_watched_files = files.len() as u64;
        
        Ok(())
    }

    fn watch_directory(
        &self,
        dir_path: PathBuf,
        patterns: Vec<String>,
        ignore_patterns: Vec<String>,
    ) -> RhemaResult<()> {
        let mut directories = self.watched_directories.write()
            .map_err(|e| RhemaError::InternalError(format!("Directory watcher lock error: {}", e)))?;
        
        // Count matching files
        let matching_files = self.count_matching_files(&dir_path, &patterns, &ignore_patterns)?;
        
        let watched_directory = WatchedDirectory {
            path: dir_path.clone(),
            watched_since: Utc::now(),
            patterns,
            ignore_patterns,
            total_files: self.count_total_files(&dir_path)?,
            matching_files,
        };
        
        directories.insert(dir_path, watched_directory);
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_watched_directories = directories.len() as u64;
        
        Ok(())
    }

    fn list_watched_files(&self, detailed: bool, pattern: Option<&str>) -> RhemaResult<Vec<WatchedFile>> {
        let files = self.watched_files.read()
            .map_err(|e| RhemaError::InternalError(format!("File watcher lock error: {}", e)))?;
        
        let mut result: Vec<WatchedFile> = if let Some(pattern) = pattern {
            let regex = self.get_or_create_regex(pattern)?;
            files.values()
                .filter(|file| {
                    regex.is_match(file.path.to_string_lossy().as_ref()) ||
                    regex.is_match(file.path.file_name().unwrap_or_default().to_string_lossy().as_ref())
                })
                .cloned()
                .collect()
        } else {
            files.values().cloned().collect()
        };
        
        // Sort by last modified time (newest first)
        result.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        
        Ok(result)
    }

    fn list_watched_directories(&self) -> RhemaResult<Vec<WatchedDirectory>> {
        let directories = self.watched_directories.read()
            .map_err(|e| RhemaError::InternalError(format!("Directory watcher lock error: {}", e)))?;
        
        Ok(directories.values().cloned().collect())
    }

    fn get_stats(&self) -> RhemaResult<WatchStats> {
        let stats = self.stats.read()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        Ok(stats.clone())
    }

    fn get_recent_events(&self, limit: usize) -> RhemaResult<Vec<FileEvent>> {
        let events = self.file_events.read()
            .map_err(|e| RhemaError::InternalError(format!("Events lock error: {}", e)))?;
        
        let mut result = events.clone();
        result.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        result.truncate(limit);
        
        Ok(result)
    }

    fn unwatch_file(&self, file_path: &PathBuf) -> RhemaResult<bool> {
        let mut files = self.watched_files.write()
            .map_err(|e| RhemaError::InternalError(format!("File watcher lock error: {}", e)))?;
        
        let was_watched = files.remove(file_path).is_some();
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_watched_files = files.len() as u64;
        
        Ok(was_watched)
    }

    fn unwatch_directory(&self, dir_path: &PathBuf) -> RhemaResult<bool> {
        let mut directories = self.watched_directories.write()
            .map_err(|e| RhemaError::InternalError(format!("Directory watcher lock error: {}", e)))?;
        
        let was_watched = directories.remove(dir_path).is_some();
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_watched_directories = directories.len() as u64;
        
        Ok(was_watched)
    }

    fn record_event(&self, path: PathBuf, event_type: FileEventType) -> RhemaResult<()> {
        let start_time = Instant::now();
        
        let file_size = if path.exists() {
            std::fs::metadata(&path).ok().map(|m| m.len())
        } else {
            None
        };
        
        let event = FileEvent {
            path,
            event_type: event_type.clone(),
            timestamp: Utc::now(),
            file_size,
        };
        
        // Add to events list
        let mut events = self.file_events.write()
            .map_err(|e| RhemaError::InternalError(format!("Events lock error: {}", e)))?;
        events.push(event);
        
        // Keep only last 1000 events
        if events.len() > 1000 {
            events.drain(0..events.len() - 1000);
        }
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        
        stats.total_events += 1;
        stats.last_event_time = Utc::now();
        
        match event_type {
            FileEventType::Created => stats.files_created += 1,
            FileEventType::Modified => stats.files_modified += 1,
            FileEventType::Deleted => stats.files_deleted += 1,
            FileEventType::Renamed => stats.files_renamed += 1,
        }
        
        // Update average response time
        let response_time = start_time.elapsed().as_millis() as u64;
        if stats.total_events > 1 {
            stats.average_response_time_ms = 
                (stats.average_response_time_ms * (stats.total_events - 1) + response_time) / stats.total_events;
        } else {
            stats.average_response_time_ms = response_time;
        }
        
        Ok(())
    }

    // Helper methods
    
    fn count_total_files(&self, dir_path: &PathBuf) -> RhemaResult<u64> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(0);
        }
        
        let mut count = 0;
        for entry in walkdir::WalkDir::new(dir_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                count += 1;
            }
        }
        
        Ok(count)
    }

    fn count_matching_files(&self, dir_path: &PathBuf, patterns: &[String], ignore_patterns: &[String]) -> RhemaResult<u64> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(0);
        }
        
        let mut count = 0;
        for entry in walkdir::WalkDir::new(dir_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path_str = entry.path().to_string_lossy();
                
                // Check ignore patterns first
                let should_ignore = ignore_patterns.iter().any(|pattern| {
                    if let Ok(regex) = Regex::new(pattern) {
                        regex.is_match(&path_str)
                    } else {
                        false
                    }
                });
                
                if should_ignore {
                    continue;
                }
                
                // Check include patterns
                let should_include = patterns.is_empty() || patterns.iter().any(|pattern| {
                    if let Ok(regex) = Regex::new(pattern) {
                        regex.is_match(&path_str)
                    } else {
                        false
                    }
                });
                
                if should_include {
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }

    fn get_or_create_regex(&self, pattern: &str) -> RhemaResult<Regex> {
        let mut cache = self.pattern_cache.write()
            .map_err(|e| RhemaError::InternalError(format!("Pattern cache lock error: {}", e)))?;
        
        if let Some(regex) = cache.get(pattern) {
            Ok(regex.clone())
        } else {
            let regex = Regex::new(pattern)
                .map_err(|e| RhemaError::InvalidInput(format!("Invalid regex pattern '{}': {}", pattern, e)))?;
            cache.insert(pattern.to_string(), regex.clone());
            Ok(regex)
        }
    }
}

// Global file watcher instance
lazy_static! {
    static ref FILE_WATCHER: Arc<FileWatcher> = {
        Arc::new(FileWatcher::new())
    };
}

pub struct WatchOperations;

impl WatchOperations {
    pub async fn handle_watch_commands(rhema: &Rhema, subcommand: WatchSubcommands) -> RhemaResult<()> {
        match subcommand {
            WatchSubcommands::File { file_path, index } => {
                Self::watch_file(rhema, file_path, index).await
            }
            WatchSubcommands::Directory { dir_path, patterns, ignore_patterns } => {
                Self::watch_directory(rhema, dir_path, patterns, ignore_patterns).await
            }
            WatchSubcommands::List { detailed, pattern, format } => {
                Self::list_watched_files(rhema, detailed, pattern, format).await
            }
            WatchSubcommands::Stats { format } => {
                Self::show_watch_stats(rhema, format).await
            }
            WatchSubcommands::Changed { format } => {
                Self::show_changed_files(rhema, format).await
            }
            WatchSubcommands::Unwatch { file_path } => {
                Self::unwatch_file(rhema, file_path).await
            }
            WatchSubcommands::UnwatchDir { dir_path } => {
                Self::unwatch_directory(rhema, dir_path).await
            }
        }
    }

    async fn watch_file(_rhema: &Rhema, file_path: PathBuf, index: bool) -> RhemaResult<()> {
        info!("Watching file: {}", file_path.display());
        
        // Validate file exists
        if !file_path.exists() {
            return Err(RhemaError::FileSystemError(format!("File does not exist: {}", file_path.display())));
        }
        
        if !file_path.is_file() {
            return Err(RhemaError::FileSystemError(format!("Path is not a file: {}", file_path.display())));
        }
        
        FILE_WATCHER.watch_file(file_path.clone(), index)?;
        
        println!("👁️  Watching file: {}", file_path.display());
        println!("  - Semantic indexing: {}", index);
        
        // Record initial event
        FILE_WATCHER.record_event(file_path, FileEventType::Created)?;
        
        Ok(())
    }

    async fn watch_directory(
        _rhema: &Rhema,
        dir_path: PathBuf,
        patterns: Vec<String>,
        ignore_patterns: Vec<String>,
    ) -> RhemaResult<()> {
        info!("Watching directory: {}", dir_path.display());
        
        // Validate directory exists
        if !dir_path.exists() {
            return Err(RhemaError::FileSystemError(format!("Directory does not exist: {}", dir_path.display())));
        }
        
        if !dir_path.is_dir() {
            return Err(RhemaError::FileSystemError(format!("Path is not a directory: {}", dir_path.display())));
        }
        
        FILE_WATCHER.watch_directory(dir_path.clone(), patterns.clone(), ignore_patterns.clone())?;
        
        println!("👁️  Watching directory: {}", dir_path.display());
        println!("  - Patterns: {:?}", patterns);
        println!("  - Ignore patterns: {:?}", ignore_patterns);
        
        // Get matching file count
        let matching_files = FILE_WATCHER.count_matching_files(&dir_path, &patterns, &ignore_patterns)?;
        println!("  - Matching files: {}", matching_files);
        
        Ok(())
    }

    async fn list_watched_files(
        _rhema: &Rhema,
        detailed: bool,
        pattern: Option<String>,
        _format: String,
    ) -> RhemaResult<()> {
        info!("Listing watched files");
        
        let files = FILE_WATCHER.list_watched_files(detailed, pattern.as_deref())?;
        let directories = FILE_WATCHER.list_watched_directories()?;
        
        println!("📋 Watched Files ({}):", files.len());
        for (i, file) in files.iter().enumerate() {
            println!("  {}. {}", i + 1, file.path.display());
            if detailed {
                println!("     Watched since: {}", file.watched_since.format("%Y-%m-%d %H:%M:%S"));
                println!("     Last modified: {}", file.last_modified.format("%Y-%m-%d %H:%M:%S"));
                println!("     Event count: {}", file.event_count);
                println!("     Semantic indexed: {}", file.semantic_indexed);
                println!("     File size: {} bytes", file.file_size);
            }
        }
        
        println!("\n📁 Watched Directories ({}):", directories.len());
        for (i, dir) in directories.iter().enumerate() {
            println!("  {}. {}", i + 1, dir.path.display());
            if detailed {
                println!("     Watched since: {}", dir.watched_since.format("%Y-%m-%d %H:%M:%S"));
                println!("     Patterns: {:?}", dir.patterns);
                println!("     Ignore patterns: {:?}", dir.ignore_patterns);
                println!("     Total files: {}", dir.total_files);
                println!("     Matching files: {}", dir.matching_files);
            }
        }
        
        if detailed {
            let stats = FILE_WATCHER.get_stats()?;
            println!("\n📊 Summary:");
            println!("  - Total watched files: {}", stats.total_watched_files);
            println!("  - Total watched directories: {}", stats.total_watched_directories);
            println!("  - Total events: {}", stats.total_events);
        }
        
        Ok(())
    }

    async fn show_watch_stats(_rhema: &Rhema, _format: String) -> RhemaResult<()> {
        info!("Showing watch statistics");
        
        let stats = FILE_WATCHER.get_stats()?;
        
        println!("📊 Watch Statistics:");
        println!("  - Total events: {}", stats.total_events);
        println!("  - Files modified: {}", stats.files_modified);
        println!("  - Files created: {}", stats.files_created);
        println!("  - Files deleted: {}", stats.files_deleted);
        println!("  - Files renamed: {}", stats.files_renamed);
        println!("  - Average response time: {}ms", stats.average_response_time_ms);
        println!("  - Last event: {}", stats.last_event_time.format("%Y-%m-%d %H:%M:%S"));
        println!("  - Total watched files: {}", stats.total_watched_files);
        println!("  - Total watched directories: {}", stats.total_watched_directories);
        
        // Calculate event rates
        if stats.total_events > 0 {
            let total_time = Utc::now().signed_duration_since(stats.last_event_time).num_seconds() as f64;
            if total_time > 0.0 {
                let events_per_second = stats.total_events as f64 / total_time;
                println!("  - Event rate: {:.2} events/second", events_per_second);
            }
        }
        
        Ok(())
    }

    async fn show_changed_files(_rhema: &Rhema, _format: String) -> RhemaResult<()> {
        info!("Showing changed files");
        
        let events = FILE_WATCHER.get_recent_events(20)?;
        
        println!("📝 Recent File Changes:");
        for (i, event) in events.iter().enumerate() {
            let event_icon = match event.event_type {
                FileEventType::Created => "🆕",
                FileEventType::Modified => "✏️",
                FileEventType::Deleted => "🗑️",
                FileEventType::Renamed => "🔄",
            };
            
            let event_name = match event.event_type {
                FileEventType::Created => "created",
                FileEventType::Modified => "modified",
                FileEventType::Deleted => "deleted",
                FileEventType::Renamed => "renamed",
            };
            
            println!("  {}. {} {} ({})", 
                i + 1, 
                event_icon, 
                event.path.display(), 
                event_name
            );
            println!("     Time: {}", event.timestamp.format("%Y-%m-%d %H:%M:%S"));
            if let Some(size) = event.file_size {
                println!("     Size: {} bytes", size);
            }
        }
        
        if events.is_empty() {
            println!("  No recent file changes detected.");
        }
        
        Ok(())
    }

    async fn unwatch_file(_rhema: &Rhema, file_path: PathBuf) -> RhemaResult<()> {
        info!("Unwatching file: {}", file_path.display());
        
        let was_watched = FILE_WATCHER.unwatch_file(&file_path)?;
        
        if was_watched {
            println!("👁️  Stopped watching file: {}", file_path.display());
        } else {
            println!("❌ File was not being watched: {}", file_path.display());
        }
        
        Ok(())
    }

    async fn unwatch_directory(_rhema: &Rhema, dir_path: PathBuf) -> RhemaResult<()> {
        info!("Unwatching directory: {}", dir_path.display());
        
        let was_watched = FILE_WATCHER.unwatch_directory(&dir_path)?;
        
        if was_watched {
            println!("👁️  Stopped watching directory: {}", dir_path.display());
        } else {
            println!("❌ Directory was not being watched: {}", dir_path.display());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_file_watcher_basic_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("test.txt");
        
        // Create a test file
        fs::write(&test_file, "test content").expect("Failed to write test file");
        
        let rhema = Rhema::new().expect("Failed to create Rhema instance");
        
        // Test watching a file
        let watch_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(WatchOperations::watch_file(&rhema, test_file.clone(), true));
        assert!(watch_result.is_ok());
        
        // Test listing watched files
        let list_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(WatchOperations::list_watched_files(&rhema, false, None, "table".to_string()));
        assert!(list_result.is_ok());
        
        // Test unwatching a file
        let unwatch_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(WatchOperations::unwatch_file(&rhema, test_file));
        assert!(unwatch_result.is_ok());
    }
}
