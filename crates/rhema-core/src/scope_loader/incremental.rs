use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

use super::types::*;
use crate::scope::Scope;

/// Configuration for incremental updates
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Whether to enable incremental updates
    pub enabled: bool,
    /// File change detection interval
    pub detection_interval: Duration,
    /// Maximum number of files to track
    pub max_tracked_files: usize,
    /// Whether to use file hashing for change detection
    pub use_file_hashing: bool,
    /// Cache duration for file hashes
    pub hash_cache_duration: Duration,
    /// Whether to enable background processing
    pub background_processing: bool,
    /// Batch size for processing changes
    pub batch_size: usize,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_interval: Duration::from_secs(5),
            max_tracked_files: 10000,
            use_file_hashing: true,
            hash_cache_duration: Duration::from_secs(3600),
            background_processing: true,
            batch_size: 100,
        }
    }
}

/// File change event
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub change_type: FileChangeType,
    pub timestamp: Instant,
    pub file_size: Option<u64>,
    pub file_hash: Option<String>,
}

/// Type of file change
#[derive(Debug, Clone)]
pub enum FileChangeType {
    Created,
    Modified,
    Deleted,
    Renamed { old_path: PathBuf },
}

/// File hash cache entry
#[derive(Debug, Clone)]
pub struct FileHashEntry {
    pub hash: String,
    pub timestamp: Instant,
    pub file_size: u64,
    pub last_modified: std::time::SystemTime,
}

/// Incremental update manager
pub struct IncrementalUpdateManager {
    config: IncrementalConfig,
    file_hashes: Arc<RwLock<HashMap<PathBuf, FileHashEntry>>>,
    tracked_files: Arc<RwLock<HashSet<PathBuf>>>,
    change_queue: Arc<RwLock<Vec<FileChangeEvent>>>,
    scope_cache: Arc<RwLock<HashMap<PathBuf, Scope>>>,
    processing_stats: Arc<RwLock<IncrementalStats>>,
}

/// Statistics for incremental updates
#[derive(Debug, Clone)]
pub struct IncrementalStats {
    pub total_files_tracked: usize,
    pub total_changes_detected: usize,
    pub total_updates_processed: usize,
    pub average_update_time: Duration,
    pub cache_hit_rate: f64,
    pub last_update: Option<Instant>,
}

impl Default for IncrementalStats {
    fn default() -> Self {
        Self {
            total_files_tracked: 0,
            total_changes_detected: 0,
            total_updates_processed: 0,
            average_update_time: Duration::ZERO,
            cache_hit_rate: 0.0,
            last_update: None,
        }
    }
}

impl IncrementalUpdateManager {
    /// Create a new incremental update manager
    pub fn new(config: IncrementalConfig) -> Self {
        Self {
            config,
            file_hashes: Arc::new(RwLock::new(HashMap::new())),
            tracked_files: Arc::new(RwLock::new(HashSet::new())),
            change_queue: Arc::new(RwLock::new(Vec::new())),
            scope_cache: Arc::new(RwLock::new(HashMap::new())),
            processing_stats: Arc::new(RwLock::new(IncrementalStats::default())),
        }
    }

    /// Start tracking a file for changes
    pub async fn track_file(&self, path: &Path) -> Result<(), ScopeLoaderError> {
        let mut tracked_files = self.tracked_files.write().await;

        if tracked_files.len() >= self.config.max_tracked_files {
            return Err(ScopeLoaderError::ConfigurationError(
                "Maximum number of tracked files reached".to_string(),
            ));
        }

        tracked_files.insert(path.to_path_buf());

        // Initialize file hash if enabled
        if self.config.use_file_hashing {
            if let Ok(hash) = self.calculate_file_hash(path).await {
                let mut file_hashes = self.file_hashes.write().await;
                file_hashes.insert(
                    path.to_path_buf(),
                    FileHashEntry {
                        hash,
                        timestamp: Instant::now(),
                        file_size: self.get_file_size(path).unwrap_or(0),
                        last_modified: self
                            .get_file_modified_time(path)
                            .unwrap_or_else(|_| std::time::SystemTime::now()),
                    },
                );
            }
        }

        // Update statistics
        let mut stats = self.processing_stats.write().await;
        stats.total_files_tracked = tracked_files.len();

        Ok(())
    }

    /// Stop tracking a file
    pub async fn untrack_file(&self, path: &Path) {
        let mut tracked_files = self.tracked_files.write().await;
        tracked_files.remove(path);

        let mut file_hashes = self.file_hashes.write().await;
        file_hashes.remove(path);

        // Update statistics
        let mut stats = self.processing_stats.write().await;
        stats.total_files_tracked = tracked_files.len();
    }

    /// Detect changes in tracked files
    pub async fn detect_changes(&self) -> Result<Vec<FileChangeEvent>, ScopeLoaderError> {
        let mut changes = Vec::new();
        let tracked_files = self.tracked_files.read().await;

        for path in tracked_files.iter() {
            if let Some(change) = self.detect_file_change(path).await? {
                changes.push(change);
            }
        }

        // Update statistics
        let mut stats = self.processing_stats.write().await;
        stats.total_changes_detected += changes.len();

        Ok(changes)
    }

    /// Detect changes in a specific file
    async fn detect_file_change(
        &self,
        path: &Path,
    ) -> Result<Option<FileChangeEvent>, ScopeLoaderError> {
        if !path.exists() {
            // File was deleted
            return Ok(Some(FileChangeEvent {
                path: path.to_path_buf(),
                change_type: FileChangeType::Deleted,
                timestamp: Instant::now(),
                file_size: None,
                file_hash: None,
            }));
        }

        let current_size = self.get_file_size(path)?;
        let current_modified = self.get_file_modified_time(path)?;

        if self.config.use_file_hashing {
            // Check file hash
            let file_hashes = self.file_hashes.read().await;
            if let Some(entry) = file_hashes.get(path) {
                let current_hash = self.calculate_file_hash(path).await?;

                if current_hash != entry.hash {
                    // File was modified
                    return Ok(Some(FileChangeEvent {
                        path: path.to_path_buf(),
                        change_type: FileChangeType::Modified,
                        timestamp: Instant::now(),
                        file_size: Some(current_size),
                        file_hash: Some(current_hash),
                    }));
                }
            } else {
                // New file
                let current_hash = self.calculate_file_hash(path).await?;
                return Ok(Some(FileChangeEvent {
                    path: path.to_path_buf(),
                    change_type: FileChangeType::Created,
                    timestamp: Instant::now(),
                    file_size: Some(current_size),
                    file_hash: Some(current_hash),
                }));
            }
        } else {
            // Check file size and modification time
            let file_hashes = self.file_hashes.read().await;
            if let Some(entry) = file_hashes.get(path) {
                if current_size != entry.file_size || current_modified != entry.last_modified {
                    // File was modified
                    return Ok(Some(FileChangeEvent {
                        path: path.to_path_buf(),
                        change_type: FileChangeType::Modified,
                        timestamp: Instant::now(),
                        file_size: Some(current_size),
                        file_hash: None,
                    }));
                }
            } else {
                // New file
                return Ok(Some(FileChangeEvent {
                    path: path.to_path_buf(),
                    change_type: FileChangeType::Created,
                    timestamp: Instant::now(),
                    file_size: Some(current_size),
                    file_hash: None,
                }));
            }
        }

        Ok(None)
    }

    /// Process incremental updates for changed files
    pub async fn process_incremental_updates(
        &self,
        changes: Vec<FileChangeEvent>,
    ) -> Result<Vec<ScopeUpdate>, ScopeLoaderError> {
        let start_time = Instant::now();
        let mut updates = Vec::new();

        // Update file hashes for modified files first
        self.update_file_hashes(&changes).await;

        // Group changes by directory to batch process
        let mut changes_by_dir: HashMap<PathBuf, Vec<FileChangeEvent>> = HashMap::new();
        for change in changes {
            let dir = change.path.parent().unwrap_or_else(|| Path::new("."));
            changes_by_dir
                .entry(dir.to_path_buf())
                .or_default()
                .push(change);
        }

        // Process each directory's changes
        for (dir, dir_changes) in changes_by_dir {
            let dir_updates = self.process_directory_changes(&dir, dir_changes).await?;
            updates.extend(dir_updates);
        }

        // Update statistics
        let processing_time = start_time.elapsed();
        let mut stats = self.processing_stats.write().await;
        stats.total_updates_processed += updates.len();
        stats.last_update = Some(Instant::now());

        // Update average processing time
        if stats.total_updates_processed > 0 {
            let total_time = stats.average_update_time * (stats.total_updates_processed - 1) as u32
                + processing_time;
            stats.average_update_time = total_time / stats.total_updates_processed as u32;
        }

        Ok(updates)
    }

    /// Process changes for a specific directory
    async fn process_directory_changes(
        &self,
        dir: &Path,
        changes: Vec<FileChangeEvent>,
    ) -> Result<Vec<ScopeUpdate>, ScopeLoaderError> {
        let mut updates = Vec::new();

        // Check if this directory has an existing scope
        let scope_cache = self.scope_cache.read().await;
        let existing_scope = scope_cache.get(dir);

        // Analyze the impact of changes
        let impact = self.analyze_change_impact(&changes).await?;

        match impact {
            ChangeImpact::NoImpact => {
                // No changes needed
            }
            ChangeImpact::MinorChanges => {
                // Update existing scope
                if let Some(scope) = existing_scope {
                    updates.push(ScopeUpdate {
                        scope_path: dir.to_path_buf(),
                        update_type: ScopeUpdateType::Modified,
                        changes: changes.clone(),
                        impact: impact.clone(),
                    });
                }
            }
            ChangeImpact::MajorChanges => {
                // Recreate scope
                updates.push(ScopeUpdate {
                    scope_path: dir.to_path_buf(),
                    update_type: ScopeUpdateType::Recreated,
                    changes: changes.clone(),
                    impact: impact.clone(),
                });
            }
            ChangeImpact::NewScope => {
                // Create new scope
                updates.push(ScopeUpdate {
                    scope_path: dir.to_path_buf(),
                    update_type: ScopeUpdateType::Created,
                    changes: changes.clone(),
                    impact: impact.clone(),
                });
            }
        }

        Ok(updates)
    }

    /// Analyze the impact of file changes
    async fn analyze_change_impact(
        &self,
        changes: &[FileChangeEvent],
    ) -> Result<ChangeImpact, ScopeLoaderError> {
        let mut impact_score = 0.0;
        let mut critical_files_changed = false;

        for change in changes {
            let file_impact = self
                .calculate_file_impact(&change.path, &change.change_type)
                .await?;
            impact_score += file_impact;

            // Check if critical files were changed
            if self.is_critical_file(&change.path) {
                critical_files_changed = true;
            }
        }

        // Determine overall impact
        if critical_files_changed || impact_score > 10.0 {
            Ok(ChangeImpact::MajorChanges)
        } else if impact_score > 5.0 {
            Ok(ChangeImpact::MinorChanges)
        } else if impact_score > 0.0 {
            Ok(ChangeImpact::MinorChanges)
        } else {
            Ok(ChangeImpact::NoImpact)
        }
    }

    /// Calculate the impact of a specific file change
    async fn calculate_file_impact(
        &self,
        path: &Path,
        change_type: &FileChangeType,
    ) -> Result<f64, ScopeLoaderError> {
        let mut impact = 0.0;

        // Base impact based on change type
        match change_type {
            FileChangeType::Created => impact += 1.0,
            FileChangeType::Modified => impact += 0.5,
            FileChangeType::Deleted => impact += 2.0,
            FileChangeType::Renamed { .. } => impact += 1.5,
        }

        // Additional impact based on file type
        if let Some(extension) = path.extension() {
            match extension.to_str().unwrap_or("") {
                "toml" | "yaml" | "yml" | "json" => impact += 3.0, // Configuration files
                "rs" | "js" | "ts" | "py" | "go" => impact += 2.0, // Source code
                "md" | "txt" => impact += 0.1,                     // Documentation
                _ => impact += 0.5,                                // Other files
            }
        }

        // Additional impact based on file name
        if let Some(file_name) = path.file_name() {
            let name = file_name.to_str().unwrap_or("");
            if name.contains("Cargo.toml")
                || name.contains("package.json")
                || name.contains("pyproject.toml")
            {
                impact += 5.0; // Package manager files
            } else if name.contains("README") || name.contains("LICENSE") {
                impact += 0.1; // Documentation files
            }
        }

        Ok(impact)
    }

    /// Check if a file is critical for scope detection
    fn is_critical_file(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name() {
            let name = file_name.to_str().unwrap_or("");
            matches!(
                name,
                "Cargo.toml"
                    | "package.json"
                    | "pyproject.toml"
                    | "go.mod"
                    | "pom.xml"
                    | "build.gradle"
            )
        } else {
            false
        }
    }

    /// Update file hashes for changed files
    async fn update_file_hashes(&self, changes: &[FileChangeEvent]) {
        let mut file_hashes = self.file_hashes.write().await;

        for change in changes {
            match &change.change_type {
                FileChangeType::Created | FileChangeType::Modified => {
                    if let (Some(hash), Some(size)) = (&change.file_hash, change.file_size) {
                        file_hashes.insert(
                            change.path.clone(),
                            FileHashEntry {
                                hash: hash.clone(),
                                timestamp: Instant::now(),
                                file_size: size,
                                last_modified: self
                                    .get_file_modified_time(&change.path)
                                    .unwrap_or_else(|_| std::time::SystemTime::now()),
                            },
                        );
                    }
                }
                FileChangeType::Deleted => {
                    file_hashes.remove(&change.path);
                }
                FileChangeType::Renamed { old_path } => {
                    if let Some(entry) = file_hashes.remove(old_path) {
                        file_hashes.insert(change.path.clone(), entry);
                    }
                }
            }
        }
    }

    /// Calculate file hash
    async fn calculate_file_hash(&self, path: &Path) -> Result<String, ScopeLoaderError> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path).map_err(|e| {
            ScopeLoaderError::ConfigurationError(format!("Failed to open file for hashing: {}", e))
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| {
            ScopeLoaderError::ConfigurationError(format!("Failed to read file for hashing: {}", e))
        })?;

        // Use a simple hash for now - in production, use a proper hashing library
        let hash = format!("{:x}", md5::compute(&buffer));
        Ok(hash)
    }

    /// Get file size
    fn get_file_size(&self, path: &Path) -> Result<u64, ScopeLoaderError> {
        std::fs::metadata(path)
            .map(|metadata| metadata.len())
            .map_err(|e| {
                ScopeLoaderError::ConfigurationError(format!("Failed to get file size: {}", e))
            })
    }

    /// Get file modification time
    fn get_file_modified_time(
        &self,
        path: &Path,
    ) -> Result<std::time::SystemTime, ScopeLoaderError> {
        std::fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .map_err(|e| {
                ScopeLoaderError::ConfigurationError(format!(
                    "Failed to get file modification time: {}",
                    e
                ))
            })
    }

    /// Get processing statistics
    pub async fn get_stats(&self) -> IncrementalStats {
        self.processing_stats.read().await.clone()
    }

    /// Clear the file hash cache
    pub async fn clear_hash_cache(&self) {
        self.file_hashes.write().await.clear();
    }

    /// Get tracked files
    pub async fn get_tracked_files(&self) -> Vec<PathBuf> {
        self.tracked_files.read().await.iter().cloned().collect()
    }
}

/// Scope update result
#[derive(Debug, Clone)]
pub struct ScopeUpdate {
    pub scope_path: PathBuf,
    pub update_type: ScopeUpdateType,
    pub changes: Vec<FileChangeEvent>,
    pub impact: ChangeImpact,
}

/// Type of scope update
#[derive(Debug, Clone)]
pub enum ScopeUpdateType {
    Created,
    Modified,
    Recreated,
    Deleted,
}

/// Impact of changes on scope
#[derive(Debug, Clone)]
pub enum ChangeImpact {
    NoImpact,
    MinorChanges,
    MajorChanges,
    NewScope,
}
