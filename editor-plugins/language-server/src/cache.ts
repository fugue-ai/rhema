export interface CacheEntry<T = any> {
  key: string;
  value: T;
  timestamp: Date;
  ttl: number; // Time to live in milliseconds
  accessCount: number;
  lastAccessed: Date;
}

export interface CacheStats {
  totalEntries: number;
  totalSize: number;
  hitRate: number;
  missRate: number;
  evictions: number;
  averageAccessCount: number;
}

export class RhemaCache {
  private cache: Map<string, CacheEntry> = new Map();
  private maxSize: number = 1000;
  private maxMemory: number = 100 * 1024 * 1024; // 100MB
  private hits: number = 0;
  private misses: number = 0;
  private evictions: number = 0;
  private cleanupInterval: NodeJS.Timeout | null = null;

  initialize(): void {
    this.startCleanupInterval();
  }

  set<T>(key: string, value: T, ttl: number = 300000): void {
    // Default 5 minutes
    // Remove old entry if it exists
    this.cache.delete(key);

    // Check if we need to evict entries
    if (this.cache.size >= this.maxSize) {
      this.evictOldest();
    }

    const entry: CacheEntry<T> = {
      key,
      value,
      timestamp: new Date(),
      ttl,
      accessCount: 0,
      lastAccessed: new Date(),
    };

    this.cache.set(key, entry);
  }

  get<T>(key: string): T | null {
    const entry = this.cache.get(key);

    if (!entry) {
      this.misses++;
      return null;
    }

    // Check if entry has expired
    if (this.isExpired(entry)) {
      this.cache.delete(key);
      this.misses++;
      return null;
    }

    // Update access statistics
    entry.accessCount++;
    entry.lastAccessed = new Date();
    this.hits++;

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
  }

  getStats(): CacheStats {
    const totalEntries = this.cache.size;
    const totalRequests = this.hits + this.misses;
    const hitRate = totalRequests > 0 ? this.hits / totalRequests : 0;
    const missRate = totalRequests > 0 ? this.misses / totalRequests : 0;

    let totalAccessCount = 0;
    this.cache.forEach((entry) => {
      totalAccessCount += entry.accessCount;
    });

    const averageAccessCount = totalEntries > 0 ? totalAccessCount / totalEntries : 0;

    return {
      totalEntries,
      totalSize: this.getCacheSize(),
      hitRate,
      missRate,
      evictions: this.evictions,
      averageAccessCount,
    };
  }

  getKeys(): string[] {
    return Array.from(this.cache.keys());
  }

  getEntries(): Array<{ key: string; entry: CacheEntry }> {
    return Array.from(this.cache.entries()).map(([key, entry]) => ({ key, entry }));
  }

  removeDocument(uri: string): void {
    this.delete(`document:${uri}`);
    this.delete(`parsed:${uri}`);
    this.delete(`validated:${uri}`);
  }

  setDocument(uri: string, data: any): void {
    this.set(`document:${uri}`, data);
  }

  getDocument(uri: string): any {
    return this.get(`document:${uri}`);
  }

  private isExpired(entry: CacheEntry): boolean {
    const now = new Date();
    const expirationTime = new Date(entry.timestamp.getTime() + entry.ttl);
    return now > expirationTime;
  }

  private evictOldest(): void {
    let oldestEntry: CacheEntry | null = null;
    let oldestKey: string | null = null;

    this.cache.forEach((entry, key) => {
      if (!oldestEntry || entry.lastAccessed < oldestEntry.lastAccessed) {
        oldestEntry = entry;
        oldestKey = key;
      }
    });

    if (oldestKey) {
      this.cache.delete(oldestKey);
      this.evictions++;
    }
  }

  private getCacheSize(): number {
    let size = 0;
    this.cache.forEach((entry) => {
      size += JSON.stringify(entry.value).length;
    });
    return size;
  }

  private startCleanupInterval(): void {
    // Clean up expired entries every minute
    this.cleanupInterval = setInterval(() => {
      this.cleanup();
    }, 60000);
  }

  private cleanup(): void {
    const keysToRemove: string[] = [];

    this.cache.forEach((entry, key) => {
      if (this.isExpired(entry)) {
        keysToRemove.push(key);
      }
    });

    keysToRemove.forEach((key) => {
      this.cache.delete(key);
    });
  }

  shutdown(): void {
    if (this.cleanupInterval) {
      clearInterval(this.cleanupInterval);
      this.cleanupInterval = null;
    }
    this.clear();
  }
}
