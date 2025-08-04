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

import * as vscode from 'vscode';
import { RhemaLogger } from '../logger';
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';
import {
  ContextTask,
  ProgressInfo,
  TaskType,
  TaskPriority,
  TaskStatus
} from '../types/context';

interface TaskQueueItem {
  task: ContextTask;
  priority: number;
  timestamp: number;
}

interface PerformanceMetrics {
  totalTasks: number;
  completedTasks: number;
  failedTasks: number;
  averageTaskTime: number;
  memoryUsage: number;
  cpuUsage: number;
  lastUpdated: Date;
}

interface ResourceUsage {
  memory: {
    used: number;
    available: number;
    percentage: number;
  };
  cpu: {
    usage: number;
    cores: number;
  };
  disk: {
    used: number;
    available: number;
    percentage: number;
  };
}

export class ContextPerformanceService {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private disposables: vscode.Disposable[] = [];

  // Task management
  private taskQueue: TaskQueueItem[] = [];
  private activeTasks: Map<string, { task: ContextTask; startTime: number }> = new Map();
  private completedTasks: Map<string, { task: ContextTask; duration: number; success: boolean }> = new Map();
  private maxConcurrentTasks: number = 3;
  private maxQueueSize: number = 100;

  // Performance tracking
  private performanceMetrics: PerformanceMetrics = {
    totalTasks: 0,
    completedTasks: 0,
    failedTasks: 0,
    averageTaskTime: 0,
    memoryUsage: 0,
    cpuUsage: 0,
    lastUpdated: new Date()
  };

  // Resource monitoring
  private resourceUsage: ResourceUsage = {
    memory: { used: 0, available: 0, percentage: 0 },
    cpu: { usage: 0, cores: 1 },
    disk: { used: 0, available: 0, percentage: 0 }
  };

  // Configuration
  private monitoringEnabled: boolean = true;
  private optimizationEnabled: boolean = true;
  private monitoringInterval: number = 30000; // 30 seconds
  private optimizationThreshold: number = 0.8; // 80%

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Context Performance Service...');

      // Load configuration
      await this.loadConfiguration();

      // Set up task processing
      await this.setupTaskProcessing();

      // Set up performance monitoring
      await this.setupPerformanceMonitoring();

      // Set up resource monitoring
      await this.setupResourceMonitoring();

      this.logger.info('Context Performance Service initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Context Performance Service', error);
    }
  }

  private async loadConfiguration(): Promise<void> {
    try {
      this.maxConcurrentTasks = this.settings.getConfiguration('rhema.maxConcurrentTasks', 3);
      this.maxQueueSize = this.settings.getConfiguration('rhema.maxQueueSize', 100);
      this.monitoringEnabled = this.settings.getConfiguration('rhema.performanceMonitoring', true);
      this.optimizationEnabled = this.settings.getConfiguration('rhema.performanceOptimization', true);
      this.monitoringInterval = this.settings.getConfiguration('rhema.monitoringInterval', 30000);
      this.optimizationThreshold = this.settings.getConfiguration('rhema.optimizationThreshold', 0.8);

      this.logger.info(`Performance Service configured: maxTasks=${this.maxConcurrentTasks}, monitoring=${this.monitoringEnabled}`);
    } catch (error) {
      this.errorHandler.handleError('Failed to load configuration', error);
    }
  }

  private async setupTaskProcessing(): Promise<void> {
    try {
      // Start task processing loop
      const taskProcessor = setInterval(async () => {
        try {
          await this.processTaskQueue();
        } catch (error) {
          this.errorHandler.handleError('Failed to process task queue', error);
        }
      }, 1000); // Process every second

      this.disposables.push({ dispose: () => clearInterval(taskProcessor) });
      this.logger.info('Task processing setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup task processing', error);
    }
  }

  private async setupPerformanceMonitoring(): Promise<void> {
    try {
      if (!this.monitoringEnabled) {
        this.logger.info('Performance monitoring disabled');
        return;
      }

      const performanceMonitor = setInterval(async () => {
        try {
          await this.updatePerformanceMetrics();
        } catch (error) {
          this.errorHandler.handleError('Failed to update performance metrics', error);
        }
      }, this.monitoringInterval);

      this.disposables.push({ dispose: () => clearInterval(performanceMonitor) });
      this.logger.info('Performance monitoring setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup performance monitoring', error);
    }
  }

  private async setupResourceMonitoring(): Promise<void> {
    try {
      const resourceMonitor = setInterval(async () => {
        try {
          await this.updateResourceUsage();
        } catch (error) {
          this.errorHandler.handleError('Failed to update resource usage', error);
        }
      }, 60000); // Every minute

      this.disposables.push({ dispose: () => clearInterval(resourceMonitor) });
      this.logger.info('Resource monitoring setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup resource monitoring', error);
    }
  }

  // Core Performance Methods

  async processInBackground(task: ContextTask): Promise<void> {
    try {
      this.logger.info(`Queuing background task: ${task.type}`);

      // Check queue size limit
      if (this.taskQueue.length >= this.maxQueueSize) {
        this.logger.warn('Task queue full, removing lowest priority task');
        this.removeLowestPriorityTask();
      }

      // Calculate priority score
      const priority = this.calculateTaskPriority(task);

      // Add to queue
      const queueItem: TaskQueueItem = {
        task,
        priority,
        timestamp: Date.now()
      };

      this.taskQueue.push(queueItem);
      this.taskQueue.sort((a, b) => b.priority - a.priority); // Sort by priority

      this.performanceMetrics.totalTasks++;
      this.logger.info(`Task queued with priority ${priority}: ${task.type}`);
    } catch (error) {
      this.errorHandler.handleError('Failed to queue background task', error);
    }
  }

  async optimizeResourceUsage(): Promise<void> {
    try {
      if (!this.optimizationEnabled) {
        this.logger.debug('Resource optimization disabled');
        return;
      }

      this.logger.info('Optimizing resource usage...');

      // Check if optimization is needed
      if (this.resourceUsage.memory.percentage > this.optimizationThreshold ||
          this.resourceUsage.cpu.usage > this.optimizationThreshold) {
        
        // Perform optimizations
        await this.performMemoryOptimization();
        await this.performTaskOptimization();
        await this.performCacheOptimization();

        this.logger.info('Resource optimization completed');
      } else {
        this.logger.debug('Resource usage within acceptable limits');
      }
    } catch (error) {
      this.errorHandler.handleError('Failed to optimize resource usage', error);
    }
  }

  async reportProgress(progress: ProgressInfo): Promise<void> {
    try {
      this.logger.debug(`Task progress: ${progress.taskId} - ${progress.progress}% (${progress.status})`);

      // Update task status
      const activeTask = this.activeTasks.get(progress.taskId);
      if (activeTask) {
        // Update progress in the task
        activeTask.task.metadata = {
          ...activeTask.task.metadata,
          estimatedTime: progress.progress
        };
      }

      // Notify VS Code of progress (if needed)
      if (progress.status === TaskStatus.Completed || progress.status === TaskStatus.Failed) {
        this.completeTask(progress.taskId, progress.status === TaskStatus.Completed);
      }
    } catch (error) {
      this.errorHandler.handleError('Failed to report progress', error);
    }
  }

  // Task Processing Methods

  private async processTaskQueue(): Promise<void> {
    try {
      // Check if we can process more tasks
      if (this.activeTasks.size >= this.maxConcurrentTasks) {
        return;
      }

      // Get next task from queue
      const nextTask = this.taskQueue.shift();
      if (!nextTask) {
        return;
      }

      // Start processing the task
      await this.startTask(nextTask.task);
    } catch (error) {
      this.errorHandler.handleError('Failed to process task queue', error);
    }
  }

  private async startTask(task: ContextTask): Promise<void> {
    try {
      this.logger.info(`Starting task: ${task.type} (${task.id})`);

      // Add to active tasks
      this.activeTasks.set(task.id, {
        task,
        startTime: Date.now()
      });

      // Execute task based on type
      switch (task.type) {
        case TaskType.ContextUpdate:
          await this.executeContextUpdateTask(task);
          break;
        case TaskType.Indexing:
          await this.executeIndexingTask(task);
          break;
        case TaskType.Analysis:
          await this.executeAnalysisTask(task);
          break;
        case TaskType.Optimization:
          await this.executeOptimizationTask(task);
          break;
        default:
          this.logger.warn(`Unknown task type: ${task.type}`);
          this.completeTask(task.id, false);
      }
    } catch (error) {
      this.errorHandler.handleError(`Failed to start task: ${task.id}`, error);
      this.completeTask(task.id, false);
    }
  }

  private async executeContextUpdateTask(task: ContextTask): Promise<void> {
    try {
      this.logger.debug(`Executing context update task: ${task.id}`);

      // Simulate context update processing
      const changes = task.changes || [];
      for (let i = 0; i < changes.length; i++) {
        const progress = Math.round((i / changes.length) * 100);
        await this.reportProgress({
          taskId: task.id,
          progress,
          status: TaskStatus.Running,
          message: `Processing change ${i + 1} of ${changes.length}`,
          metadata: {
            timestamp: new Date(),
            taskId: task.id,
            status: TaskStatus.Running
          }
        });

        // Simulate processing time
        await this.delay(100);
      }

      this.completeTask(task.id, true);
    } catch (error) {
      this.errorHandler.handleError(`Failed to execute context update task: ${task.id}`, error);
      this.completeTask(task.id, false);
    }
  }

  private async executeIndexingTask(task: ContextTask): Promise<void> {
    try {
      this.logger.debug(`Executing indexing task: ${task.id}`);

      // Simulate indexing processing
      for (let i = 0; i < 10; i++) {
        const progress = i * 10;
        await this.reportProgress({
          taskId: task.id,
          progress,
          status: TaskStatus.Running,
          message: `Indexing step ${i + 1} of 10`,
          metadata: {
            timestamp: new Date(),
            taskId: task.id,
            status: TaskStatus.Running
          }
        });

        // Simulate processing time
        await this.delay(200);
      }

      this.completeTask(task.id, true);
    } catch (error) {
      this.errorHandler.handleError(`Failed to execute indexing task: ${task.id}`, error);
      this.completeTask(task.id, false);
    }
  }

  private async executeAnalysisTask(task: ContextTask): Promise<void> {
    try {
      this.logger.debug(`Executing analysis task: ${task.id}`);

      // Simulate analysis processing
      for (let i = 0; i < 5; i++) {
        const progress = i * 20;
        await this.reportProgress({
          taskId: task.id,
          progress,
          status: TaskStatus.Running,
          message: `Analysis phase ${i + 1} of 5`,
          metadata: {
            timestamp: new Date(),
            taskId: task.id,
            status: TaskStatus.Running
          }
        });

        // Simulate processing time
        await this.delay(300);
      }

      this.completeTask(task.id, true);
    } catch (error) {
      this.errorHandler.handleError(`Failed to execute analysis task: ${task.id}`, error);
      this.completeTask(task.id, false);
    }
  }

  private async executeOptimizationTask(task: ContextTask): Promise<void> {
    try {
      this.logger.debug(`Executing optimization task: ${task.id}`);

      // Perform actual optimization
      await this.optimizeResourceUsage();

      this.completeTask(task.id, true);
    } catch (error) {
      this.errorHandler.handleError(`Failed to execute optimization task: ${task.id}`, error);
      this.completeTask(task.id, false);
    }
  }

  private completeTask(taskId: string, success: boolean): void {
    try {
      const activeTask = this.activeTasks.get(taskId);
      if (!activeTask) {
        this.logger.warn(`Task not found in active tasks: ${taskId}`);
        return;
      }

      const duration = Date.now() - activeTask.startTime;
      
      // Remove from active tasks
      this.activeTasks.delete(taskId);

      // Add to completed tasks
      this.completedTasks.set(taskId, {
        task: activeTask.task,
        duration,
        success
      });

      // Update metrics
      if (success) {
        this.performanceMetrics.completedTasks++;
      } else {
        this.performanceMetrics.failedTasks++;
      }

      this.logger.info(`Task completed: ${taskId} (${success ? 'success' : 'failed'}) in ${duration}ms`);
    } catch (error) {
      this.errorHandler.handleError(`Failed to complete task: ${taskId}`, error);
    }
  }

  // Optimization Methods

  private async performMemoryOptimization(): Promise<void> {
    try {
      this.logger.info('Performing memory optimization...');

      // Clear completed tasks history if too large
      if (this.completedTasks.size > 1000) {
        const entries = Array.from(this.completedTasks.entries());
        entries.sort((a, b) => a[1].task.metadata.created.getTime() - b[1].task.metadata.created.getTime());
        
        // Keep only the most recent 500 entries
        const toRemove = entries.slice(0, entries.length - 500);
        for (const [taskId] of toRemove) {
          this.completedTasks.delete(taskId);
        }

        this.logger.info(`Cleared ${toRemove.length} old task entries`);
      }

      // Suggest garbage collection
      if (global.gc) {
        global.gc();
        this.logger.debug('Garbage collection triggered');
      }
    } catch (error) {
      this.errorHandler.handleError('Failed to perform memory optimization', error);
    }
  }

  private async performTaskOptimization(): Promise<void> {
    try {
      this.logger.info('Performing task optimization...');

      // Reduce concurrent tasks if resource usage is high
      if (this.resourceUsage.cpu.usage > 0.9) {
        this.maxConcurrentTasks = Math.max(1, this.maxConcurrentTasks - 1);
        this.logger.info(`Reduced max concurrent tasks to ${this.maxConcurrentTasks}`);
      }

      // Increase concurrent tasks if resource usage is low
      if (this.resourceUsage.cpu.usage < 0.3 && this.maxConcurrentTasks < 5) {
        this.maxConcurrentTasks++;
        this.logger.info(`Increased max concurrent tasks to ${this.maxConcurrentTasks}`);
      }
    } catch (error) {
      this.errorHandler.handleError('Failed to perform task optimization', error);
    }
  }

  private async performCacheOptimization(): Promise<void> {
    try {
      this.logger.info('Performing cache optimization...');

      // This would integrate with the cache service to optimize cache usage
      // For now, just log the intention
      this.logger.debug('Cache optimization would be performed here');
    } catch (error) {
      this.errorHandler.handleError('Failed to perform cache optimization', error);
    }
  }

  // Monitoring Methods

  private async updatePerformanceMetrics(): Promise<void> {
    try {
      // Calculate average task time
      const completedTaskDurations = Array.from(this.completedTasks.values())
        .map(ct => ct.duration);
      
      if (completedTaskDurations.length > 0) {
        const totalTime = completedTaskDurations.reduce((sum, time) => sum + time, 0);
        this.performanceMetrics.averageTaskTime = totalTime / completedTaskDurations.length;
      }

      // Update memory usage
      this.performanceMetrics.memoryUsage = this.resourceUsage.memory.percentage;
      this.performanceMetrics.cpuUsage = this.resourceUsage.cpu.usage;
      this.performanceMetrics.lastUpdated = new Date();

      this.logger.debug(`Performance metrics updated: avgTaskTime=${this.performanceMetrics.averageTaskTime}ms, memory=${this.performanceMetrics.memoryUsage}%, cpu=${this.performanceMetrics.cpuUsage}%`);
    } catch (error) {
      this.errorHandler.handleError('Failed to update performance metrics', error);
    }
  }

  private async updateResourceUsage(): Promise<void> {
    try {
      // Get memory usage
      const memUsage = process.memoryUsage();
      this.resourceUsage.memory.used = memUsage.heapUsed;
      this.resourceUsage.memory.available = memUsage.heapTotal;
      this.resourceUsage.memory.percentage = (memUsage.heapUsed / memUsage.heapTotal) * 100;

      // Get CPU usage (simplified)
      this.resourceUsage.cpu.usage = Math.random() * 0.5 + 0.2; // Mock CPU usage
      this.resourceUsage.cpu.cores = require('os').cpus().length;

      // Get disk usage (simplified)
      this.resourceUsage.disk.used = 0; // Would be calculated from actual disk usage
      this.resourceUsage.disk.available = 0;
      this.resourceUsage.disk.percentage = 0;

      this.logger.debug(`Resource usage: memory=${this.resourceUsage.memory.percentage.toFixed(1)}%, cpu=${(this.resourceUsage.cpu.usage * 100).toFixed(1)}%`);
    } catch (error) {
      this.errorHandler.handleError('Failed to update resource usage', error);
    }
  }

  // Helper Methods

  private calculateTaskPriority(task: ContextTask): number {
    let priority = 0;

    // Base priority by task type
    switch (task.type) {
      case TaskType.ContextUpdate:
        priority = 80;
        break;
      case TaskType.Indexing:
        priority = 60;
        break;
      case TaskType.Analysis:
        priority = 70;
        break;
      case TaskType.Optimization:
        priority = 40;
        break;
      default:
        priority = 50;
    }

    // Adjust by task priority
    switch (task.priority) {
      case TaskPriority.Critical:
        priority += 50;
        break;
      case TaskPriority.High:
        priority += 30;
        break;
      case TaskPriority.Medium:
        priority += 10;
        break;
      case TaskPriority.Low:
        priority -= 10;
        break;
    }

    return Math.max(0, Math.min(100, priority));
  }

  private removeLowestPriorityTask(): void {
    if (this.taskQueue.length === 0) {
      return;
    }

    // Find task with lowest priority
    let lowestIndex = 0;
    let lowestPriority = this.taskQueue[0].priority;

    for (let i = 1; i < this.taskQueue.length; i++) {
      if (this.taskQueue[i].priority < lowestPriority) {
        lowestPriority = this.taskQueue[i].priority;
        lowestIndex = i;
      }
    }

    // Remove the lowest priority task
    const removedTask = this.taskQueue.splice(lowestIndex, 1)[0];
    this.logger.warn(`Removed low priority task: ${removedTask.task.type} (priority: ${removedTask.priority})`);
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  // Public API Methods

  getPerformanceMetrics(): PerformanceMetrics {
    return { ...this.performanceMetrics };
  }

  getResourceUsage(): ResourceUsage {
    return { ...this.resourceUsage };
  }

  getTaskQueueStatus(): { queued: number; active: number; completed: number } {
    return {
      queued: this.taskQueue.length,
      active: this.activeTasks.size,
      completed: this.completedTasks.size
    };
  }

  async dispose(): Promise<void> {
    try {
      this.logger.info('Disposing Context Performance Service...');

      // Dispose listeners
      this.disposables.forEach(disposable => disposable.dispose());
      this.disposables = [];

      // Clear data
      this.taskQueue = [];
      this.activeTasks.clear();
      this.completedTasks.clear();

      this.logger.info('Context Performance Service disposed');
    } catch (error) {
      this.errorHandler.handleError('Failed to dispose Context Performance Service', error);
    }
  }
} 