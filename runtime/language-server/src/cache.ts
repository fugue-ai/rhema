export interface CacheEntry<T = any> {
  key: string;
  value: T;
  timestamp: Date;
  ttl: number; // Time to live in milliseconds
  accessCount: number;
  lastAccessed: Date;
  size: number; // Size in bytes
  priority: number; // Priority for eviction (lower = higher priority)
}

export interface CacheStats {
  totalEntries: number;
  totalSize: number;
  hitRate: number;
  missRate: number;
  evictions: number;
  averageAccessCount: number;
  memoryUsage: number;
  cacheEfficiency: number;
}

export class RhemaCache {
  private cache: Map<string, CacheEntry> = new Map();
  private maxSize: number = 1000;
  private maxMemory: number = 100 * 1024 * 1024; // 100MB
  private hits: number = 0;
  private misses: number = 0;
  private evictions: number = 0;
  private cleanupInterval: NodeJS.Timeout | null = null;
  private invalidationPatterns: Map<string, RegExp[]> = new Map();
  private performanceMetrics: {
    averageResponseTime: number;
    totalRequests: number;
    slowQueries: number;
  } = {
    averageResponseTime: 0,
    totalRequests: 0,
    slowQueries: 0,
  };

  initialize(): void {
    this.startCleanupInterval();
    this.initializeInvalidationPatterns();
  }

  private initializeInvalidationPatterns(): void {
    // Define patterns for intelligent cache invalidation
    this.invalidationPatterns.set('validation', [
      /validation.*/,
      /schema.*/,
      /rule.*/,
    ]);
    this.invalidationPatterns.set('completion', [
      /completion.*/,
      /suggestion.*/,
      /keyword.*/,
    ]);
    this.invalidationPatterns.set('document', [
      /document.*/,
      /parse.*/,
      /content.*/,
    ]);
  }

  set<T>(key: string, value: T, ttl: number = 300000, priority: number = 5): void {
    // Default 5 minutes, priority 5 (medium)
    const startTime = Date.now();
    
    // Remove old entry if it exists
    this.cache.delete(key);

    // Check if we need to evict entries
    if (this.cache.size >= this.maxSize || this.getCacheSize() >= this.maxMemory) {
      this.evictIntelligently();
    }

    // Calculate entry size
    const size = this.calculateSize(value);

    const entry: CacheEntry<T> = {
      key,
      value,
      timestamp: new Date(),
      ttl,
      accessCount: 0,
      lastAccessed: new Date(),
      size,
      priority,
    };

    this.cache.set(key, entry);
    
    // Update performance metrics
    const responseTime = Date.now() - startTime;
    this.updatePerformanceMetrics(responseTime);
  }

  get<T>(key: string): T | null {
    const startTime = Date.now();
    const entry = this.cache.get(key);

    if (!entry) {
      this.misses++;
      this.updatePerformanceMetrics(Date.now() - startTime);
      return null;
    }

    // Check if entry has expired
    if (this.isExpired(entry)) {
      this.cache.delete(key);
      this.misses++;
      this.updatePerformanceMetrics(Date.now() - startTime);
      return null;
    }

    // Update access statistics
    entry.accessCount++;
    entry.lastAccessed = new Date();
    this.hits++;

    // Intelligent cache warming for frequently accessed items
    if (entry.accessCount > 10 && entry.priority > 1) {
      entry.priority = Math.max(1, entry.priority - 1); // Increase priority
    }

    this.updatePerformanceMetrics(Date.now() - startTime);
    return entry.value as T;
  }

  has(key: string): boolean {
    const entry = this.cache.get(key);

    if (!entry) {
      return false;
    }

    if (this.isExpired(entry)) {
      this.cache.delete(key);
      return false;
    }

    return true;
  }

  delete(key: string): boolean {
    return this.cache.delete(key);
  }

  clear(): void {
    this.cache.clear();
    this.hits = 0;
    this.misses = 0;
    this.evictions = 0;
    this.resetPerformanceMetrics();
  }

  getStats(): CacheStats {
    const totalRequests = this.hits + this.misses;
    const hitRate = totalRequests > 0 ? this.hits / totalRequests : 0;
    const missRate = totalRequests > 0 ? this.misses / totalRequests : 0;
    const averageAccessCount = this.cache.size > 0 
      ? Array.from(this.cache.values()).reduce((sum, entry) => sum + entry.accessCount, 0) / this.cache.size 
      : 0;

    return {
      totalEntries: this.cache.size,
      totalSize: this.getCacheSize(),
      hitRate,
      missRate,
      evictions: this.evictions,
      averageAccessCount,
      memoryUsage: this.getMemoryUsage(),
      cacheEfficiency: this.calculateEfficiency(),
    };
  }

  getKeys(): string[] {
    return Array.from(this.cache.keys());
  }

  getEntries(): Array<{ key: string; entry: CacheEntry }> {
    return Array.from(this.cache.entries()).map(([key, entry]) => ({ key, entry }));
  }

  removeDocument(uri: string): void {
    // Remove all cache entries related to a specific document
    const keysToRemove: string[] = [];
    
    for (const key of this.cache.keys()) {
      if (key.includes(uri) || key.includes(encodeURIComponent(uri))) {
        keysToRemove.push(key);
      }
    }
    
    keysToRemove.forEach(key => this.cache.delete(key));
  }

  setDocument(uri: string, data: any): void {
    this.set(`document:${uri}`, data, 600000, 3); // 10 minutes, high priority
  }

  getDocument(uri: string): any {
    return this.get(`document:${uri}`);
  }

  // Intelligent cache invalidation based on patterns
  invalidateByPattern(pattern: string): void {
    const patterns = this.invalidationPatterns.get(pattern);
    if (!patterns) return;

    const keysToRemove: string[] = [];
    
    for (const key of this.cache.keys()) {
      if (patterns.some(p => p.test(key))) {
        keysToRemove.push(key);
      }
    }
    
    keysToRemove.forEach(key => this.cache.delete(key));
  }

  // Cache warming for frequently accessed patterns
  warmCache(patterns: string[]): void {
    // This would be implemented based on usage patterns
    // For now, we'll just log the warming attempt
    console.log(`Cache warming requested for patterns: ${patterns.join(', ')}`);
  }

  private isExpired(entry: CacheEntry): boolean {
    return Date.now() - entry.timestamp.getTime() > entry.ttl;
  }

  private evictIntelligently(): void {
    // Intelligent eviction based on multiple factors
    const entries = Array.from(this.cache.entries());
    
    // Sort by eviction score (lower score = higher priority to keep)
    entries.sort(([, a], [, b]) => {
      const scoreA = this.calculateEvictionScore(a);
      const scoreB = this.calculateEvictionScore(b);
      return scoreA - scoreB;
    });

    // Evict the worst entries until we're under limits
    while (this.cache.size >= this.maxSize || this.getCacheSize() >= this.maxMemory) {
      if (entries.length === 0) break;
      
      const [key] = entries.pop()!;
      this.cache.delete(key);
      this.evictions++;
    }
  }

  private calculateEvictionScore(entry: CacheEntry): number {
    const age = Date.now() - entry.timestamp.getTime();
    const timeSinceLastAccess = Date.now() - entry.lastAccessed.getTime();
    const accessFrequency = entry.accessCount / Math.max(1, age / 1000); // accesses per second
    
    // Lower score = higher priority to keep
    return (
      entry.priority * 1000 + // Priority factor
      age / 1000 + // Age factor (older = higher score)
      timeSinceLastAccess / 1000 - // Time since last access (longer = higher score)
      accessFrequency * 100 // Access frequency (higher = lower score)
    );
  }

  private getCacheSize(): number {
    return Array.from(this.cache.values()).reduce((total, entry) => total + entry.size, 0);
  }

  private calculateSize(value: any): number {
    // Simple size calculation - in production, use a more sophisticated approach
    return JSON.stringify(value).length;
  }

  private getMemoryUsage(): number {
    // Estimate memory usage (in production, use process.memoryUsage())
    return this.getCacheSize() * 1.5; // Rough estimate including overhead
  }

  private calculateEfficiency(): number {
    const totalSize = this.getCacheSize();
    const totalRequests = this.hits + this.misses;
    const hitRate = totalRequests > 0 ? this.hits / totalRequests : 0;
    
    const memoryEfficiency = 1 - (totalSize / this.maxMemory);
    const accessEfficiency = hitRate;
    const timeEfficiency = this.performanceMetrics.averageResponseTime < 50 ? 1 : 
                          this.performanceMetrics.averageResponseTime < 100 ? 0.8 : 0.6;
    
    return (memoryEfficiency + accessEfficiency + timeEfficiency) / 3;
  }

  private updatePerformanceMetrics(responseTime: number): void {
    this.performanceMetrics.totalRequests++;
    this.performanceMetrics.averageResponseTime = 
      (this.performanceMetrics.averageResponseTime * (this.performanceMetrics.totalRequests - 1) + responseTime) / 
      this.performanceMetrics.totalRequests;
    
    if (responseTime > 100) {
      this.performanceMetrics.slowQueries++;
    }
  }

  private resetPerformanceMetrics(): void {
    this.performanceMetrics = {
      averageResponseTime: 0,
      totalRequests: 0,
      slowQueries: 0,
    };
  }

  private startCleanupInterval(): void {
    this.cleanupInterval = setInterval(() => {
      this.cleanup();
    }, 60000); // Clean up every minute
  }

  private cleanup(): void {
    const now = Date.now();
    const keysToRemove: string[] = [];

    for (const [key, entry] of this.cache.entries()) {
      if (this.isExpired(entry)) {
        keysToRemove.push(key);
      }
    }

    keysToRemove.forEach(key => this.cache.delete(key));
  }

  shutdown(): void {
    if (this.cleanupInterval) {
      clearInterval(this.cleanupInterval);
      this.cleanupInterval = null;
    }
    this.cache.clear();
  }
}
