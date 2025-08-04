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
import * as fs from 'fs';
import * as path from 'path';
import * as crypto from 'crypto';
import { RhemaLogger } from '../logger';
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';
import {
  WorkspaceContext,
  ContextSuggestion,
  CompletionContext,
  CacheMetrics,
  MetricsMetadata
} from '../types/context';

interface CacheEntry<T> {
  key: string;
  value: T;
  timestamp: number;
  accessCount: number;
  lastAccessed: number;
  size: number;
  ttl: number;
}

interface CacheStats {
  hits: number;
  misses: number;
  totalRequests: number;
  averageResponseTime: number;
  memoryUsage: number;
  diskUsage: number;
}

export class ContextCacheService {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private disposables: vscode.Disposable[] = [];

  // Memory cache (L1)
  private memoryCache: Map<string, CacheEntry<any>> = new Map();
  private memoryStats: CacheStats = {
    hits: 0,
    misses: 0,
    totalRequests: 0,
    averageResponseTime: 0,
    memoryUsage: 0,
    diskUsage: 0
  };

  // Disk cache (L2)
  private diskCachePath: string = '';
  private diskStats: CacheStats = {
    hits: 0,
    misses: 0,
    totalRequests: 0,
    averageResponseTime: 0,
    memoryUsage: 0,
    diskUsage: 0
  };

  // Configuration
  private maxMemorySize: number = 100 * 1024 * 1024; // 100MB
  private maxDiskSize: number = 1024 * 1024 * 1024; // 1GB
  private defaultTTL: number = 3600 * 1000; // 1 hour
  private cleanupInterval: number = 300 * 1000; // 5 minutes

  // Performance tracking
  private responseTimes: number[] = [];
  private maxResponseTimeHistory: number = 100;

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Context Cache Service...');

      // Set up disk cache path
      this.diskCachePath = path.join(context.globalStorageUri.fsPath, 'context-cache');
      await this.ensureDiskCacheDirectory();

      // Load existing cache from disk
      await this.loadDiskCache();

      // Set up periodic cleanup
      await this.setupPeriodicCleanup();

      // Set up cache monitoring
      await this.setupCacheMonitoring();

      this.logger.info('Context Cache Service initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Context Cache Service', error);
    }
  }

  private async ensureDiskCacheDirectory(): Promise<void> {
    try {
      if (!fs.existsSync(this.diskCachePath)) {
        await fs.promises.mkdir(this.diskCachePath, { recursive: true });
        this.logger.info(`Created disk cache directory: ${this.diskCachePath}`);
      }
    } catch (error) {
      this.errorHandler.handleError('Failed to create disk cache directory', error);
    }
  }

  private async loadDiskCache(): Promise<void> {
    try {
      const files = await fs.promises.readdir(this.diskCachePath);
      let loadedCount = 0;

      for (const file of files) {
        if (file.endsWith('.cache')) {
          try {
            const filePath = path.join(this.diskCachePath, file);
            const data = await fs.promises.readFile(filePath, 'utf8');
            const entry: CacheEntry<any> = JSON.parse(data);

            // Check if entry is still valid
            if (this.isEntryValid(entry)) {
              const key = path.basename(file, '.cache');
              this.memoryCache.set(key, entry);
              loadedCount++;
            } else {
              // Remove expired entry
              await fs.promises.unlink(filePath);
            }
          } catch {
            this.logger.warn(`Failed to load cache entry: ${file}`);
          }
        }
      }

      this.logger.info(`Loaded ${loadedCount} cache entries from disk`);
    } catch (error) {
      this.errorHandler.handleError('Failed to load disk cache', error);
    }
  }

  private async setupPeriodicCleanup(): Promise<void> {
    const cleanupTimer = setInterval(async () => {
      try {
        await this.performCleanup();
      } catch (error) {
        this.errorHandler.handleError('Failed to perform periodic cleanup', error);
      }
    }, this.cleanupInterval);

    this.disposables.push({ dispose: () => clearInterval(cleanupTimer) });
  }

  private async setupCacheMonitoring(): Promise<void> {
    const monitoringTimer = setInterval(async () => {
      try {
        await this.updateCacheStats();
      } catch (error) {
        this.errorHandler.handleError('Failed to update cache stats', error);
      }
    }, 60000); // Every minute

    this.disposables.push({ dispose: () => clearInterval(monitoringTimer) });
  }

  // Core Cache Operations

  async cacheWorkspaceContext(context: WorkspaceContext): Promise<void> {
    try {
      const key = 'workspace';
      const entry = this.createCacheEntry(key, context, this.defaultTTL);
      
      // Store in memory cache
      await this.storeInMemoryCache(key, entry);
      
      // Store in disk cache
      await this.storeInDiskCache(key, entry);

      this.logger.info('Workspace context cached successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to cache workspace context', error);
    }
  }

  async getCachedContext(key: string): Promise<WorkspaceContext | null> {
    const startTime = Date.now();
    
    try {
      // Try memory cache first
      let entry = this.memoryCache.get(key);
      
      if (entry && this.isEntryValid(entry)) {
        // Update access statistics
        entry.accessCount++;
        entry.lastAccessed = Date.now();
        this.memoryCache.set(key, entry);
        
        this.updateResponseTime(Date.now() - startTime);
        this.memoryStats.hits++;
        this.memoryStats.totalRequests++;
        
        this.logger.debug(`Cache hit (memory): ${key}`);
        return entry.value;
      }

      // Try disk cache
      entry = await this.getFromDiskCache(key);
      if (entry && this.isEntryValid(entry)) {
        // Move to memory cache
        await this.storeInMemoryCache(key, entry);
        
        this.updateResponseTime(Date.now() - startTime);
        this.diskStats.hits++;
        this.diskStats.totalRequests++;
        
        this.logger.debug(`Cache hit (disk): ${key}`);
        return entry.value;
      }

      // Cache miss
      this.updateResponseTime(Date.now() - startTime);
      this.memoryStats.misses++;
      this.memoryStats.totalRequests++;
      
      this.logger.debug(`Cache miss: ${key}`);
      return null;
    } catch (error) {
      this.errorHandler.handleError(`Failed to get cached context: ${key}`, error);
      return null;
    }
  }

  async getContextSuggestions(context: CompletionContext): Promise<ContextSuggestion[]> {
    try {
      const key = this.generateSuggestionKey(context);
      const entry = await this.getCachedContext(key);
      
      if (entry && 'suggestions' in entry) {
        return (entry as any).suggestions;
      }
      
      return [];
    } catch (error) {
      this.errorHandler.handleError('Failed to get context suggestions from cache', error);
      return [];
    }
  }

  async cacheContextSuggestions(context: CompletionContext, suggestions: ContextSuggestion[]): Promise<void> {
    try {
      const key = this.generateSuggestionKey(context);
      const entry = this.createCacheEntry(key, { suggestions }, this.defaultTTL);
      
      await this.storeInMemoryCache(key, entry);
      await this.storeInDiskCache(key, entry);
    } catch (error) {
      this.errorHandler.handleError('Failed to cache context suggestions', error);
    }
  }

  async invalidateCache(pattern: string): Promise<void> {
    try {
      this.logger.info(`Invalidating cache with pattern: ${pattern}`);

      // Invalidate memory cache
      const memoryKeys = Array.from(this.memoryCache.keys());
      for (const key of memoryKeys) {
        if (this.matchesPattern(key, pattern)) {
          this.memoryCache.delete(key);
        }
      }

      // Invalidate disk cache
      const files = await fs.promises.readdir(this.diskCachePath);
      for (const file of files) {
        if (file.endsWith('.cache')) {
          const key = path.basename(file, '.cache');
          if (this.matchesPattern(key, pattern)) {
            const filePath = path.join(this.diskCachePath, file);
            await fs.promises.unlink(filePath);
          }
        }
      }

      this.logger.info(`Cache invalidation completed for pattern: ${pattern}`);
    } catch (error) {
      this.errorHandler.handleError('Failed to invalidate cache', error);
    }
  }

  async getCacheMetrics(): Promise<CacheMetrics> {
    try {
      await this.updateCacheStats();
      
      const totalRequests = this.memoryStats.totalRequests + this.diskStats.totalRequests;
      const totalHits = this.memoryStats.hits + this.diskStats.hits;
      const hitRate = totalRequests > 0 ? totalHits / totalRequests : 0;
      const missRate = 1 - hitRate;

      const averageResponseTime = this.calculateAverageResponseTime();

      return {
        hitRate,
        missRate,
        totalRequests,
        averageResponseTime,
        metadata: {
          timestamp: new Date(),
          period: 'current',
          source: 'context_cache'
        }
      };
    } catch (error) {
      this.errorHandler.handleError('Failed to get cache metrics', error);
      return {
        hitRate: 0,
        missRate: 0,
        totalRequests: 0,
        averageResponseTime: 0,
        metadata: {
          timestamp: new Date(),
          period: 'current',
          source: 'context_cache'
        }
      };
    }
  }

  // Private Helper Methods

  private createCacheEntry<T>(key: string, value: T, ttl: number): CacheEntry<T> {
    const size = this.estimateSize(value);
    return {
      key,
      value,
      timestamp: Date.now(),
      accessCount: 0,
      lastAccessed: Date.now(),
      size,
      ttl
    };
  }

  private isEntryValid(entry: CacheEntry<any>): boolean {
    const now = Date.now();
    return (now - entry.timestamp) < entry.ttl;
  }

  private async storeInMemoryCache(key: string, entry: CacheEntry<any>): Promise<void> {
    try {
      // Check if we need to evict entries
      if (this.memoryStats.memoryUsage + entry.size > this.maxMemorySize) {
        await this.evictMemoryCacheEntries(entry.size);
      }

      this.memoryCache.set(key, entry);
      this.memoryStats.memoryUsage += entry.size;
    } catch (error) {
      this.errorHandler.handleError('Failed to store in memory cache', error);
    }
  }

  private async storeInDiskCache(key: string, entry: CacheEntry<any>): Promise<void> {
    try {
      const filePath = path.join(this.diskCachePath, `${key}.cache`);
      const data = JSON.stringify(entry);
      
      // Check disk space
      if (this.diskStats.diskUsage + entry.size > this.maxDiskSize) {
        await this.evictDiskCacheEntries(entry.size);
      }

      await fs.promises.writeFile(filePath, data, 'utf8');
      this.diskStats.diskUsage += entry.size;
    } catch (error) {
      this.errorHandler.handleError('Failed to store in disk cache', error);
    }
  }

  private async getFromDiskCache(key: string): Promise<CacheEntry<any> | null> {
    try {
      const filePath = path.join(this.diskCachePath, `${key}.cache`);
      
      if (!fs.existsSync(filePath)) {
        return null;
      }

      const data = await fs.promises.readFile(filePath, 'utf8');
      return JSON.parse(data);
    } catch {
      this.logger.warn(`Failed to read from disk cache: ${key}`);
      return null;
    }
  }

  private async evictMemoryCacheEntries(requiredSpace: number): Promise<void> {
    try {
      const entries = Array.from(this.memoryCache.entries());
      
      // Sort by access count and last accessed time (LRU-like)
      entries.sort((a, b) => {
        const scoreA = a[1].accessCount * 0.7 + (Date.now() - a[1].lastAccessed) * 0.3;
        const scoreB = b[1].accessCount * 0.7 + (Date.now() - b[1].lastAccessed) * 0.3;
        return scoreA - scoreB;
      });

      let freedSpace = 0;
      for (const [key, entry] of entries) {
        if (freedSpace >= requiredSpace) {
          break;
        }
        
        this.memoryCache.delete(key);
        freedSpace += entry.size;
        this.memoryStats.memoryUsage -= entry.size;
      }

      this.logger.debug(`Evicted ${freedSpace} bytes from memory cache`);
    } catch (error) {
      this.errorHandler.handleError('Failed to evict memory cache entries', error);
    }
  }

  private async evictDiskCacheEntries(requiredSpace: number): Promise<void> {
    try {
      const files = await fs.promises.readdir(this.diskCachePath);
      const fileEntries: Array<{ file: string; entry: CacheEntry<any> }> = [];

      for (const file of files) {
        if (file.endsWith('.cache')) {
          try {
            const filePath = path.join(this.diskCachePath, file);
            const data = await fs.promises.readFile(filePath, 'utf8');
            const entry: CacheEntry<any> = JSON.parse(data);
            fileEntries.push({ file, entry });
                     } catch {
             // Skip corrupted files
             continue;
           }
        }
      }

      // Sort by access count and last accessed time
      fileEntries.sort((a, b) => {
        const scoreA = a.entry.accessCount * 0.7 + (Date.now() - a.entry.lastAccessed) * 0.3;
        const scoreB = b.entry.accessCount * 0.7 + (Date.now() - b.entry.lastAccessed) * 0.3;
        return scoreA - scoreB;
      });

      let freedSpace = 0;
      for (const { file, entry } of fileEntries) {
        if (freedSpace >= requiredSpace) {
          break;
        }

        const filePath = path.join(this.diskCachePath, file);
        await fs.promises.unlink(filePath);
        freedSpace += entry.size;
        this.diskStats.diskUsage -= entry.size;
      }

      this.logger.debug(`Evicted ${freedSpace} bytes from disk cache`);
    } catch (error) {
      this.errorHandler.handleError('Failed to evict disk cache entries', error);
    }
  }

  private async performCleanup(): Promise<void> {
    try {
      // Clean up expired entries from memory cache
      const memoryKeys = Array.from(this.memoryCache.keys());
      for (const key of memoryKeys) {
        const entry = this.memoryCache.get(key);
        if (entry && !this.isEntryValid(entry)) {
          this.memoryCache.delete(key);
          this.memoryStats.memoryUsage -= entry.size;
        }
      }

      // Clean up expired entries from disk cache
      const files = await fs.promises.readdir(this.diskCachePath);
      for (const file of files) {
        if (file.endsWith('.cache')) {
          try {
            const filePath = path.join(this.diskCachePath, file);
            const data = await fs.promises.readFile(filePath, 'utf8');
            const entry: CacheEntry<any> = JSON.parse(data);
            
            if (!this.isEntryValid(entry)) {
              await fs.promises.unlink(filePath);
              this.diskStats.diskUsage -= entry.size;
            }
                     } catch {
             // Remove corrupted files
             const filePath = path.join(this.diskCachePath, file);
             await fs.promises.unlink(filePath);
           }
        }
      }

      this.logger.debug('Cache cleanup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to perform cache cleanup', error);
    }
  }

  private async updateCacheStats(): Promise<void> {
    try {
      // Update memory usage
      this.memoryStats.memoryUsage = Array.from(this.memoryCache.values())
        .reduce((total, entry) => total + entry.size, 0);

      // Update disk usage
      const files = await fs.promises.readdir(this.diskCachePath);
      let diskUsage = 0;
      
      for (const file of files) {
        if (file.endsWith('.cache')) {
          try {
            const filePath = path.join(this.diskCachePath, file);
            const stats = await fs.promises.stat(filePath);
            diskUsage += stats.size;
                     } catch {
             // Skip files that can't be stat'd
           }
        }
      }
      
      this.diskStats.diskUsage = diskUsage;
    } catch (error) {
      this.errorHandler.handleError('Failed to update cache stats', error);
    }
  }

  private updateResponseTime(responseTime: number): void {
    this.responseTimes.push(responseTime);
    
    if (this.responseTimes.length > this.maxResponseTimeHistory) {
      this.responseTimes.shift();
    }
  }

  private calculateAverageResponseTime(): number {
    if (this.responseTimes.length === 0) {
      return 0;
    }
    
    const sum = this.responseTimes.reduce((total, time) => total + time, 0);
    return sum / this.responseTimes.length;
  }

  private estimateSize(value: any): number {
    try {
      return JSON.stringify(value).length;
    } catch {
      return 1024; // Default size estimate
    }
  }

  private generateSuggestionKey(context: CompletionContext): string {
    const hash = crypto.createHash('md5');
    hash.update(JSON.stringify(context));
    return `suggestion_${hash.digest('hex')}`;
  }

  private matchesPattern(key: string, pattern: string): boolean {
    if (pattern === '*') {
      return true;
    }
    
    if (pattern.includes('*')) {
      const regex = new RegExp(pattern.replace(/\*/g, '.*'));
      return regex.test(key);
    }
    
    return key === pattern;
  }

  async dispose(): Promise<void> {
    try {
      this.logger.info('Disposing Context Cache Service...');

      // Dispose listeners
      this.disposables.forEach(disposable => disposable.dispose());
      this.disposables = [];

      // Clear memory cache
      this.memoryCache.clear();

      this.logger.info('Context Cache Service disposed');
    } catch (error) {
      this.errorHandler.handleError('Failed to dispose Context Cache Service', error);
    }
  }
} 