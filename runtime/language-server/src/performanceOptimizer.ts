import { type TextDocument, type Position, Range } from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';
import type { RhemaCache } from './cache';
import type { RhemaPerformanceMonitor } from './performanceMonitor';

export interface PerformanceMetrics {
  operation: string;
  duration: number;
  memoryUsage: number;
  cacheHits: number;
  cacheMisses: number;
  timestamp: Date;
}

export interface OptimizationConfig {
  enableCaching: boolean;
  enableMemoryOptimization: boolean;
  enableAsyncProcessing: boolean;
  enableBatchProcessing: boolean;
  cacheSize: number;
  memoryThreshold: number;
  batchSize: number;
  maxConcurrentOperations: number;
  gcInterval: number;
  performanceMonitoring: boolean;
}

export interface BatchOperation {
  id: string;
  type: 'parse' | 'validate' | 'complete' | 'format' | 'index';
  documents: TextDocument[];
  priority: 'high' | 'medium' | 'low';
  callback: (results: any[]) => void;
}

export interface MemoryProfile {
  heapUsed: number;
  heapTotal: number;
  external: number;
  arrayBuffers: number;
  timestamp: Date;
}

export class RhemaPerformanceOptimizer {
  private cache: RhemaCache;
  private performanceMonitor: RhemaPerformanceMonitor;
  private config: OptimizationConfig;
  private batchQueue: BatchOperation[] = [];
  private processingQueue: boolean = false;
  private memoryProfiles: MemoryProfile[] = [];
  private gcTimer: NodeJS.Timeout | null = null;
  private operationQueue: Map<string, Promise<any>> = new Map();
  private memoryThreshold: number = 100 * 1024 * 1024; // 100MB

  constructor(cache: RhemaCache, performanceMonitor: RhemaPerformanceMonitor) {
    this.cache = cache;
    this.performanceMonitor = performanceMonitor;
    this.config = this.getDefaultConfig();
    this.initializeOptimizations();
  }

  setConfiguration(config: Partial<OptimizationConfig>): void {
    this.config = { ...this.config, ...config };
    this.applyConfiguration();
  }

  getConfiguration(): OptimizationConfig {
    return this.config;
  }

  // --- Caching Strategies ---

  async getCachedResult<T>(
    key: string,
    operation: () => Promise<T>,
    ttl: number = 300000
  ): Promise<T> {
    if (!this.config.enableCaching) {
      return await operation();
    }

    const cached = this.cache.get<T>(key);
    if (cached !== null) {
      this.performanceMonitor.recordOperation('cache_hit', 0, { key });
      return cached;
    }

    this.performanceMonitor.recordOperation('cache_miss', 0, { key });
    const result = await operation();
    this.cache.set(key, result, ttl);
    return result;
  }

  generateCacheKey(operation: string, document: TextDocument, position?: Position): string {
    const contentHash = this.hashContent(document.getText());
    const version = document.version;
    const uri = document.uri;

    if (position) {
      return `${operation}:${uri}:${version}:${contentHash}:${position.line}:${position.character}`;
    }

    return `${operation}:${uri}:${version}:${contentHash}`;
  }

  private hashContent(content: string): string {
    // Simple hash function for content
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
      const char = content.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return hash.toString(36);
  }

  // --- Memory Management ---

  startMemoryMonitoring(): void {
    if (!this.config.enableMemoryOptimization) {
      return;
    }

    // Monitor memory usage every 30 seconds
    setInterval(() => {
      this.recordMemoryProfile();
      this.checkMemoryThreshold();
    }, 30000);

    // Force garbage collection periodically
    this.gcTimer = setInterval(() => {
      this.forceGarbageCollection();
    }, this.config.gcInterval);
  }

  stopMemoryMonitoring(): void {
    if (this.gcTimer) {
      clearInterval(this.gcTimer);
      this.gcTimer = null;
    }
  }

  private recordMemoryProfile(): void {
    const usage = process.memoryUsage();
    const profile: MemoryProfile = {
      heapUsed: usage.heapUsed,
      heapTotal: usage.heapTotal,
      external: usage.external,
      arrayBuffers: usage.arrayBuffers,
      timestamp: new Date(),
    };

    this.memoryProfiles.push(profile);

    // Keep only last 100 profiles
    if (this.memoryProfiles.length > 100) {
      this.memoryProfiles.shift();
    }

    this.performanceMonitor.recordOperation('memory_profile', 0, {
      heapUsed: usage.heapUsed,
      heapTotal: usage.heapTotal,
    });
  }

  private checkMemoryThreshold(): void {
    const usage = process.memoryUsage();

    if (usage.heapUsed > this.memoryThreshold) {
      console.warn(`Memory usage exceeded threshold: ${this.formatBytes(usage.heapUsed)}`);
      this.optimizeMemory();
    }
  }

  private optimizeMemory(): void {
    // Clear old cache entries
    this.cache.clear();

    // Clear old memory profiles
    if (this.memoryProfiles.length > 50) {
      this.memoryProfiles = this.memoryProfiles.slice(-50);
    }

    // Clear operation queue for low priority operations
    this.clearLowPriorityOperations();

    // Force garbage collection
    this.forceGarbageCollection();
  }

  private clearLowPriorityOperations(): void {
    for (const [key, promise] of this.operationQueue.entries()) {
      // Cancel low priority operations
      if (key.includes('low')) {
        this.operationQueue.delete(key);
      }
    }
  }

  private forceGarbageCollection(): void {
    if (global.gc) {
      global.gc();
      this.performanceMonitor.recordOperation('garbage_collection', 0);
    }
  }

  getMemoryProfile(): MemoryProfile[] {
    return [...this.memoryProfiles];
  }

  // --- Async Operation Optimization ---

  async executeWithThrottling<T>(
    operation: () => Promise<T>,
    priority: 'high' | 'medium' | 'low' = 'medium'
  ): Promise<T> {
    const startTime = Date.now();
    const operationId = this.generateOperationId(operation, priority);

    try {
      // Check if operation is already in progress
      if (this.operationQueue.has(operationId)) {
        return await this.operationQueue.get(operationId);
      }

      // Wait for available operation slot
      await this.waitForOperationSlot();

      // Execute operation with proper queuing
      const result = this.executeOperation(operation, operationId);
      this.operationQueue.set(operationId, result);

      const finalResult = await result;
      this.operationQueue.delete(operationId);

      // Record performance metrics
      const duration = Date.now() - startTime;
      this.performanceMonitor.recordOperation('throttled_operation', duration, {
        priority,
        operationId,
      });

      return finalResult;
    } catch (error) {
      this.operationQueue.delete(operationId);
      throw error;
    }
  }

  // Enhanced operation queuing with priority-based scheduling
  private async executeOperation<T>(operation: () => Promise<T>, operationId: string): Promise<T> {
    const startTime = Date.now();
    
    try {
      // Add operation to priority queue
      const queueItem = {
        id: operationId,
        operation,
        priority: this.getOperationPriority(operationId),
        timestamp: Date.now(),
        retries: 0,
      };

      // Process based on priority
      if (queueItem.priority === 'high') {
        return await this.executeHighPriorityOperation(queueItem);
      } else if (queueItem.priority === 'medium') {
        return await this.executeMediumPriorityOperation(queueItem);
      } else {
        return await this.executeLowPriorityOperation(queueItem);
      }
    } catch (error) {
      // Retry logic for failed operations
      if (this.shouldRetryOperation(operationId, error)) {
        return await this.retryOperation(operation, operationId);
      }
      throw error;
    }
  }

  private async executeHighPriorityOperation<T>(queueItem: any): Promise<T> {
    // High priority operations execute immediately
    return await queueItem.operation();
  }

  private async executeMediumPriorityOperation<T>(queueItem: any): Promise<T> {
    // Medium priority operations wait for available slots
    await this.waitForMediumPrioritySlot();
    return await queueItem.operation();
  }

  private async executeLowPriorityOperation<T>(queueItem: any): Promise<T> {
    // Low priority operations are queued and processed in background
    return new Promise((resolve, reject) => {
      this.addToBackgroundQueue({
        ...queueItem,
        resolve,
        reject,
      });
    });
  }

  private getOperationPriority(operationId: string): 'high' | 'medium' | 'low' {
    // Determine priority based on operation type
    if (operationId.includes('completion') || operationId.includes('hover')) {
      return 'high'; // User-facing operations
    } else if (operationId.includes('validation') || operationId.includes('format')) {
      return 'medium'; // Important but not critical
    } else {
      return 'low'; // Background operations
    }
  }

  private shouldRetryOperation(operationId: string, error: any): boolean {
    // Retry logic for transient errors
    const retryableErrors = ['timeout', 'network', 'temporary'];
    return retryableErrors.some(type => error.message?.includes(type));
  }

  private async retryOperation<T>(operation: () => Promise<T>, operationId: string): Promise<T> {
    const maxRetries = 3;
    let lastError: any;

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      try {
        // Exponential backoff
        if (attempt > 1) {
          await new Promise(resolve => setTimeout(resolve, Math.pow(2, attempt) * 100));
        }
        
        return await operation();
      } catch (error) {
        lastError = error;
        console.warn(`Operation ${operationId} failed on attempt ${attempt}:`, error);
      }
    }

    throw lastError;
  }

  private async waitForOperationSlot(): Promise<void> {
    const maxConcurrent = this.config.maxConcurrentOperations;
    
    while (this.operationQueue.size >= maxConcurrent) {
      await new Promise(resolve => setTimeout(resolve, 10)); // Small delay
    }
  }

  private async waitForMediumPrioritySlot(): Promise<void> {
    const maxMediumPriority = Math.floor(this.config.maxConcurrentOperations / 2);
    let mediumPriorityCount = 0;
    
    for (const [, operation] of this.operationQueue) {
      if (operation && typeof operation === 'object' && 'priority' in operation) {
        mediumPriorityCount++;
      }
    }
    
    while (mediumPriorityCount >= maxMediumPriority) {
      await new Promise(resolve => setTimeout(resolve, 20));
      mediumPriorityCount = 0;
      for (const [, operation] of this.operationQueue) {
        if (operation && typeof operation === 'object' && 'priority' in operation) {
          mediumPriorityCount++;
        }
      }
    }
  }

  // Background processing queue
  private backgroundQueue: any[] = [];
  private backgroundProcessing: boolean = false;

  private addToBackgroundQueue(item: any): void {
    this.backgroundQueue.push(item);
    
    if (!this.backgroundProcessing) {
      this.processBackgroundQueue();
    }
  }

  private async processBackgroundQueue(): Promise<void> {
    this.backgroundProcessing = true;
    
    while (this.backgroundQueue.length > 0) {
      const item = this.backgroundQueue.shift();
      
      try {
        const result = await item.operation();
        item.resolve(result);
      } catch (error) {
        item.reject(error);
      }
      
      // Small delay to prevent blocking
      await new Promise(resolve => setTimeout(resolve, 5));
    }
    
    this.backgroundProcessing = false;
  }

  private generateOperationId(operation: Function, priority: string): string {
    return `${priority}_${operation.name || 'anonymous'}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  // --- Batch Processing ---

  addToBatch(operation: BatchOperation): void {
    if (!this.config.enableBatchProcessing) {
      // Execute immediately if batch processing is disabled
      this.executeBatchOperation(operation);
      return;
    }

    this.batchQueue.push(operation);
    this.processBatchQueue();
  }

  private async processBatchQueue(): Promise<void> {
    if (this.processingQueue || this.batchQueue.length === 0) {
      return;
    }

    this.processingQueue = true;

    try {
      // Group operations by type and priority
      const groupedOperations = this.groupBatchOperations();

      for (const [type, operations] of groupedOperations) {
        await this.processBatchGroup(type, operations);
      }
    } finally {
      this.processingQueue = false;

      // Check if more operations were added while processing
      if (this.batchQueue.length > 0) {
        setImmediate(() => this.processBatchQueue());
      }
    }
  }

  private groupBatchOperations(): Map<string, BatchOperation[]> {
    const groups = new Map<string, BatchOperation[]>();

    for (const operation of this.batchQueue) {
      const key = `${operation.type}_${operation.priority}`;
      if (!groups.has(key)) {
        groups.set(key, []);
      }
      groups.get(key)!.push(operation);
    }

    return groups;
  }

  private async processBatchGroup(type: string, operations: BatchOperation[]): Promise<void> {
    const batchSize = this.config.batchSize;

    for (let i = 0; i < operations.length; i += batchSize) {
      const batch = operations.slice(i, i + batchSize);
      await this.executeBatchGroup(type, batch);
    }
  }

  private async executeBatchGroup(type: string, operations: BatchOperation[]): Promise<void> {
    const startTime = performance.now();

    try {
      const results = await Promise.all(operations.map((op) => this.executeBatchOperation(op)));

      const duration = performance.now() - startTime;

      this.performanceMonitor.recordOperation('batch_processing', duration, {
        type,
        batchSize: operations.length,
        success: true,
      });

      // Call callbacks with results
      operations.forEach((op, index) => {
        op.callback([results[index]]);
      });
    } catch (error) {
      const duration = performance.now() - startTime;

      this.performanceMonitor.recordOperation('batch_processing', duration, {
        type,
        batchSize: operations.length,
        success: false,
        error: error instanceof Error ? error.message : String(error),
      });

      throw error;
    }
  }

  private async executeBatchOperation(operation: BatchOperation): Promise<any> {
    const startTime = performance.now();

    try {
      let result: any;

      switch (operation.type) {
        case 'parse':
          result = await this.batchParse(operation.documents);
          break;
        case 'validate':
          result = await this.batchValidate(operation.documents);
          break;
        case 'complete':
          result = await this.batchComplete(operation.documents);
          break;
        case 'format':
          result = await this.batchFormat(operation.documents);
          break;
        case 'index':
          result = await this.batchIndex(operation.documents);
          break;
        default:
          throw new Error(`Unknown batch operation type: ${operation.type}`);
      }

      const duration = performance.now() - startTime;
      this.performanceMonitor.recordOperation(`batch_${operation.type}`, duration);

      return result;
    } catch (error) {
      const duration = performance.now() - startTime;
      this.performanceMonitor.recordOperation(`batch_${operation.type}`, duration, {
        error: error instanceof Error ? error.message : String(error),
      });
      throw error;
    }
  }

  // --- Batch Operation Implementations ---

  private async batchParse(documents: TextDocument[]): Promise<any[]> {
    return Promise.all(
      documents.map((doc) => ({
        uri: doc.uri,
        content: doc.getText(),
        version: doc.version,
      }))
    );
  }

  private async batchValidate(documents: TextDocument[]): Promise<any[]> {
    // Implement batch validation logic
    return documents.map((doc) => ({
      uri: doc.uri,
      valid: true,
      diagnostics: [],
    }));
  }

  private async batchComplete(documents: TextDocument[]): Promise<any[]> {
    // Implement batch completion logic
    return documents.map((doc) => ({
      uri: doc.uri,
      completions: [],
    }));
  }

  private async batchFormat(documents: TextDocument[]): Promise<any[]> {
    // Implement batch formatting logic
    return documents.map((doc) => ({
      uri: doc.uri,
      formatted: doc.getText(),
    }));
  }

  private async batchIndex(documents: TextDocument[]): Promise<any[]> {
    // Implement batch indexing logic
    return documents.map((doc) => ({
      uri: doc.uri,
      indexed: true,
    }));
  }

  // --- Performance Monitoring ---

  getPerformanceMetrics(): PerformanceMetrics[] {
    const report = this.performanceMonitor.getReport();
    return (report as any).metrics?.map((metric: any) => ({
      operation: metric.operation,
      duration: metric.duration,
      memoryUsage: process.memoryUsage().heapUsed,
      cacheHits: 0, // Would be tracked separately
      cacheMisses: 0, // Would be tracked separately
      timestamp: new Date(),
    }));
  }

  // --- Configuration ---

  private getDefaultConfig(): OptimizationConfig {
    return {
      enableCaching: true,
      enableMemoryOptimization: true,
      enableAsyncProcessing: true,
      enableBatchProcessing: true,
      cacheSize: 1000,
      memoryThreshold: 100 * 1024 * 1024, // 100MB
      batchSize: 10,
      maxConcurrentOperations: 5,
      gcInterval: 300000, // 5 minutes
      performanceMonitoring: true,
    };
  }

  private initializeOptimizations(): void {
    if (this.config.enableMemoryOptimization) {
      this.startMemoryMonitoring();
    }
  }

  private applyConfiguration(): void {
    // Apply configuration changes
    if (this.config.enableMemoryOptimization) {
      this.startMemoryMonitoring();
    } else {
      this.stopMemoryMonitoring();
    }
  }

  // --- Utility Methods ---

  private formatBytes(bytes: number): string {
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 Bytes';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round((bytes / 1024 ** i) * 100) / 100 + ' ' + sizes[i];
  }

  // --- Cleanup ---

  dispose(): void {
    this.stopMemoryMonitoring();
    this.cache.clear();
    this.batchQueue = [];
    this.operationQueue.clear();
    this.memoryProfiles = [];
  }
}

