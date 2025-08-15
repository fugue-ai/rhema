use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::interval;

use super::types::*;
use crate::scope::Scope;

/// Configuration for background processing
#[derive(Debug, Clone)]
pub struct BackgroundProcessingConfig {
    /// Whether to enable background processing
    pub enabled: bool,
    /// Number of background worker threads
    pub worker_threads: usize,
    /// Maximum queue size for background tasks
    pub max_queue_size: usize,
    /// Task timeout duration
    pub task_timeout: Duration,
    /// Retry attempts for failed tasks
    pub max_retries: u32,
    /// Retry delay between attempts
    pub retry_delay: Duration,
    /// Whether to enable task prioritization
    pub enable_prioritization: bool,
    /// Background processing interval
    pub processing_interval: Duration,
}

impl Default for BackgroundProcessingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            worker_threads: 4,
            max_queue_size: 1000,
            task_timeout: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            retry_delay: Duration::from_secs(30),
            enable_prioritization: true,
            processing_interval: Duration::from_secs(10), // 10 seconds
        }
    }
}

/// Background task
#[derive(Debug, Clone)]
pub struct BackgroundTask {
    pub task_id: String,
    pub task_type: BackgroundTaskType,
    pub path: PathBuf,
    pub priority: TaskPriority,
    pub created_at: Instant,
    pub retry_count: u32,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of background task
#[derive(Debug, Clone)]
pub enum BackgroundTaskType {
    DiscoverScopes,
    UpdateScopes,
    AnalyzePatterns,
    ValidateScopes,
    GenerateSuggestions,
    CleanupCache,
    SyncWithRemote,
}

/// Task priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Background task result
#[derive(Debug, Clone)]
pub struct BackgroundTaskResult {
    pub task_id: String,
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub processing_time: Duration,
    pub completed_at: Instant,
}

/// Background task status
#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(BackgroundTaskResult),
    Failed(BackgroundTaskResult),
    Cancelled,
}

/// Background processing manager
pub struct BackgroundProcessingManager {
    config: BackgroundProcessingConfig,
    task_queue: Arc<Mutex<Vec<BackgroundTask>>>,
    running_tasks: Arc<RwLock<HashMap<String, BackgroundTask>>>,
    completed_tasks: Arc<RwLock<HashMap<String, BackgroundTaskResult>>>,
    task_status: Arc<RwLock<HashMap<String, TaskStatus>>>,
    workers: Vec<JoinHandle<()>>,
    result_sender: mpsc::UnboundedSender<BackgroundTaskResult>,
    result_receiver: Option<mpsc::UnboundedReceiver<BackgroundTaskResult>>,
    is_running: Arc<RwLock<bool>>,
    stats: Arc<RwLock<BackgroundProcessingStats>>,
}

/// Background processing statistics
#[derive(Debug, Clone)]
pub struct BackgroundProcessingStats {
    pub total_tasks_submitted: usize,
    pub total_tasks_completed: usize,
    pub total_tasks_failed: usize,
    pub average_processing_time: Duration,
    pub queue_size: usize,
    pub active_workers: usize,
    pub last_activity: Option<Instant>,
}

impl Default for BackgroundProcessingStats {
    fn default() -> Self {
        Self {
            total_tasks_submitted: 0,
            total_tasks_completed: 0,
            total_tasks_failed: 0,
            average_processing_time: Duration::ZERO,
            queue_size: 0,
            active_workers: 0,
            last_activity: None,
        }
    }
}

impl BackgroundProcessingManager {
    /// Create a new background processing manager
    pub fn new(config: BackgroundProcessingConfig) -> Self {
        let (result_sender, result_receiver) = mpsc::unbounded_channel();

        Self {
            config,
            task_queue: Arc::new(Mutex::new(Vec::new())),
            running_tasks: Arc::new(RwLock::new(HashMap::new())),
            completed_tasks: Arc::new(RwLock::new(HashMap::new())),
            task_status: Arc::new(RwLock::new(HashMap::new())),
            workers: Vec::new(),
            result_sender,
            result_receiver: Some(result_receiver),
            is_running: Arc::new(RwLock::new(false)),
            stats: Arc::new(RwLock::new(BackgroundProcessingStats::default())),
        }
    }

    /// Start background processing
    pub async fn start(&mut self) -> Result<(), ScopeLoaderError> {
        if *self.is_running.read().await {
            return Err(ScopeLoaderError::ConfigurationError(
                "Background processing is already running".to_string(),
            ));
        }

        // Start worker threads
        for worker_id in 0..self.config.worker_threads {
            let task_queue = self.task_queue.clone();
            let running_tasks = self.running_tasks.clone();
            let task_status = self.task_status.clone();
            let result_sender = self.result_sender.clone();
            let config = self.config.clone();
            let stats = self.stats.clone();

            let worker = tokio::spawn(async move {
                Self::worker_loop(
                    worker_id,
                    task_queue,
                    running_tasks,
                    task_status,
                    result_sender,
                    config,
                    stats,
                )
                .await;
            });
            self.workers.push(worker);
        }

        // Start result processing
        let result_receiver = self.result_receiver.take().ok_or_else(|| {
            ScopeLoaderError::ConfigurationError("Result receiver already taken".to_string())
        })?;
        let completed_tasks = self.completed_tasks.clone();
        let task_status = self.task_status.clone();
        let stats = self.stats.clone();

        let result_processor = tokio::spawn(async move {
            Self::process_results(result_receiver, completed_tasks, task_status, stats).await;
        });

        self.workers.push(result_processor);

        // Mark as running
        *self.is_running.write().await = true;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_workers = self.config.worker_threads;
        stats.last_activity = Some(Instant::now());

        Ok(())
    }

    /// Stop background processing
    pub async fn stop(&mut self) {
        // Mark as not running
        *self.is_running.write().await = false;

        // Wait for all workers to finish
        for worker in self.workers.drain(..) {
            let _ = worker.await;
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_workers = 0;
    }

    /// Submit a background task
    pub async fn submit_task(&self, task: BackgroundTask) -> Result<(), ScopeLoaderError> {
        if !*self.is_running.read().await {
            return Err(ScopeLoaderError::ConfigurationError(
                "Background processing is not running".to_string(),
            ));
        }

        let mut queue = self.task_queue.lock().await;

        if queue.len() >= self.config.max_queue_size {
            return Err(ScopeLoaderError::ConfigurationError(
                "Task queue is full".to_string(),
            ));
        }

        // Add task to queue
        queue.push(task.clone());

        // Sort by priority if enabled
        if self.config.enable_prioritization {
            queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        // Update task status
        let mut task_status = self.task_status.write().await;
        task_status.insert(task.task_id.clone(), TaskStatus::Pending);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_tasks_submitted += 1;
        stats.queue_size = queue.len();
        stats.last_activity = Some(Instant::now());

        Ok(())
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        self.task_status.read().await.get(task_id).cloned()
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> Result<bool, ScopeLoaderError> {
        let mut task_status = self.task_status.write().await;

        if let Some(status) = task_status.get_mut(task_id) {
            match status {
                TaskStatus::Pending => {
                    // Remove from queue
                    let mut queue = self.task_queue.lock().await;
                    queue.retain(|task| task.task_id != task_id);

                    *status = TaskStatus::Cancelled;

                    // Update stats
                    let mut stats = self.stats.write().await;
                    stats.queue_size = queue.len();

                    Ok(true)
                }
                TaskStatus::Running => {
                    // Mark as cancelled (will be handled when task completes)
                    *status = TaskStatus::Cancelled;
                    Ok(true)
                }
                _ => Ok(false), // Already completed or cancelled
            }
        } else {
            Ok(false) // Task not found
        }
    }

    /// Get processing statistics
    pub async fn get_stats(&self) -> BackgroundProcessingStats {
        let mut stats = self.stats.read().await.clone();
        stats.queue_size = self.task_queue.lock().await.len();
        stats
    }

    /// Get completed tasks
    pub async fn get_completed_tasks(&self) -> Vec<BackgroundTaskResult> {
        self.completed_tasks
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    /// Get running tasks
    pub async fn get_running_tasks(&self) -> Vec<BackgroundTask> {
        self.running_tasks.read().await.values().cloned().collect()
    }

    /// Worker loop for processing tasks
    async fn worker_loop(
        worker_id: usize,
        task_queue: Arc<Mutex<Vec<BackgroundTask>>>,
        running_tasks: Arc<RwLock<HashMap<String, BackgroundTask>>>,
        task_status: Arc<RwLock<HashMap<String, TaskStatus>>>,
        result_sender: mpsc::UnboundedSender<BackgroundTaskResult>,
        config: BackgroundProcessingConfig,
        stats: Arc<RwLock<BackgroundProcessingStats>>,
    ) {
        let mut interval = interval(config.processing_interval);

        loop {
            interval.tick().await;

            // Get next task from queue
            let task = {
                let mut queue = task_queue.lock().await;
                if queue.is_empty() {
                    continue;
                }
                queue.remove(0)
            };

            // Check if task was cancelled
            {
                let task_status = task_status.read().await;
                if let Some(TaskStatus::Cancelled) = task_status.get(&task.task_id) {
                    continue;
                }
            }

            // Mark task as running
            {
                let mut running = running_tasks.write().await;
                running.insert(task.task_id.clone(), task.clone());

                let mut status = task_status.write().await;
                status.insert(task.task_id.clone(), TaskStatus::Running);
            }

            // Process the task
            let start_time = Instant::now();
            let result = Self::process_task(task.clone(), &config).await;
            let processing_time = start_time.elapsed();

            // Create result
            let (success, result_value, error) = match &result {
                Ok(value) => (true, Some(value.clone()), None),
                Err(e) => (false, None, Some(e.to_string())),
            };

            let task_result = BackgroundTaskResult {
                task_id: task.task_id.clone(),
                success,
                result: result_value,
                error,
                processing_time,
                completed_at: Instant::now(),
            };

            // Send result
            if let Err(e) = result_sender.send(task_result) {
                eprintln!("Worker {} failed to send result: {}", worker_id, e);
            }

            // Remove from running tasks
            running_tasks.write().await.remove(&task.task_id);
        }
    }

    /// Process a single background task
    async fn process_task(
        task: BackgroundTask,
        config: &BackgroundProcessingConfig,
    ) -> Result<serde_json::Value, ScopeLoaderError> {
        // Check timeout
        if task.created_at.elapsed() > config.task_timeout {
            return Err(ScopeLoaderError::ConfigurationError(
                "Task timeout exceeded".to_string(),
            ));
        }

        // Process based on task type
        match task.task_type {
            BackgroundTaskType::DiscoverScopes => {
                // This would integrate with the existing scope discovery
                Ok(serde_json::json!({
                    "scopes_discovered": 0,
                    "path": task.path.to_string_lossy()
                }))
            }
            BackgroundTaskType::UpdateScopes => Ok(serde_json::json!({
                "scopes_updated": 0,
                "path": task.path.to_string_lossy()
            })),
            BackgroundTaskType::AnalyzePatterns => Ok(serde_json::json!({
                "patterns_found": 0,
                "path": task.path.to_string_lossy()
            })),
            BackgroundTaskType::ValidateScopes => Ok(serde_json::json!({
                "validation_passed": true,
                "path": task.path.to_string_lossy()
            })),
            BackgroundTaskType::GenerateSuggestions => Ok(serde_json::json!({
                "suggestions_generated": 0,
                "path": task.path.to_string_lossy()
            })),
            BackgroundTaskType::CleanupCache => Ok(serde_json::json!({
                "cache_cleaned": true,
                "entries_removed": 0
            })),
            BackgroundTaskType::SyncWithRemote => Ok(serde_json::json!({
                "sync_completed": true,
                "changes_synced": 0
            })),
        }
    }

    /// Process results from workers
    async fn process_results(
        mut result_receiver: mpsc::UnboundedReceiver<BackgroundTaskResult>,
        completed_tasks: Arc<RwLock<HashMap<String, BackgroundTaskResult>>>,
        task_status: Arc<RwLock<HashMap<String, TaskStatus>>>,
        stats: Arc<RwLock<BackgroundProcessingStats>>,
    ) {
        while let Some(result) = result_receiver.recv().await {
            // Store completed task
            completed_tasks
                .write()
                .await
                .insert(result.task_id.clone(), result.clone());

            // Update task status
            let status = if result.success {
                TaskStatus::Completed(result.clone())
            } else {
                TaskStatus::Failed(result.clone())
            };
            task_status.write().await.insert(result.task_id, status);

            // Update statistics
            let mut stats = stats.write().await;
            stats.total_tasks_completed += 1;
            if !result.success {
                stats.total_tasks_failed += 1;
            }

            // Update average processing time
            if stats.total_tasks_completed > 0 {
                let total_time = stats.average_processing_time
                    * (stats.total_tasks_completed - 1) as u32
                    + result.processing_time;
                stats.average_processing_time = total_time / stats.total_tasks_completed as u32;
            }

            stats.last_activity = Some(Instant::now());
        }
    }

    /// Create a background task
    pub fn create_task(
        task_type: BackgroundTaskType,
        path: PathBuf,
        priority: TaskPriority,
    ) -> BackgroundTask {
        BackgroundTask {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type,
            path,
            priority,
            created_at: Instant::now(),
            retry_count: 0,
            metadata: HashMap::new(),
        }
    }

    /// Check if background processing is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get queue size
    pub async fn queue_size(&self) -> usize {
        self.task_queue.lock().await.len()
    }

    /// Clear completed tasks
    pub async fn clear_completed_tasks(&self) {
        self.completed_tasks.write().await.clear();
    }

    /// Get tasks by status
    pub async fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<String> {
        let task_status = self.task_status.read().await;
        task_status
            .iter()
            .filter(|(_, s)| match (s, &status) {
                (TaskStatus::Pending, TaskStatus::Pending) => true,
                (TaskStatus::Running, TaskStatus::Running) => true,
                (TaskStatus::Completed(_), TaskStatus::Completed(_)) => true,
                (TaskStatus::Failed(_), TaskStatus::Failed(_)) => true,
                (TaskStatus::Cancelled, TaskStatus::Cancelled) => true,
                _ => false,
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
}

/// Background task builder
pub struct BackgroundTaskBuilder {
    task_type: Option<BackgroundTaskType>,
    path: Option<PathBuf>,
    priority: TaskPriority,
    metadata: HashMap<String, serde_json::Value>,
}

impl BackgroundTaskBuilder {
    /// Create a new task builder
    pub fn new() -> Self {
        Self {
            task_type: None,
            path: None,
            priority: TaskPriority::Normal,
            metadata: HashMap::new(),
        }
    }

    /// Set task type
    pub fn task_type(mut self, task_type: BackgroundTaskType) -> Self {
        self.task_type = Some(task_type);
        self
    }

    /// Set path
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the task
    pub fn build(self) -> Result<BackgroundTask, ScopeLoaderError> {
        let task_type = self.task_type.ok_or_else(|| {
            ScopeLoaderError::ConfigurationError("Task type is required".to_string())
        })?;

        let path = self
            .path
            .ok_or_else(|| ScopeLoaderError::ConfigurationError("Path is required".to_string()))?;

        Ok(BackgroundTask {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type,
            path,
            priority: self.priority,
            created_at: Instant::now(),
            retry_count: 0,
            metadata: self.metadata,
        })
    }
}

impl Default for BackgroundTaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
