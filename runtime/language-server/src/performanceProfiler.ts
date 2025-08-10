import {
  type RhemaPerformanceOptimizer,
  PerformanceMetrics,
  type MemoryProfile,
} from './performanceOptimizer';

export interface ProfilingSession {
  id: string;
  startTime: Date;
  endTime?: Date;
  duration?: number;
  operations: ProfiledOperation[];
  memorySnapshots: MemoryProfile[];
  summary: ProfilingSummary;
}

export interface ProfiledOperation {
  name: string;
  startTime: number;
  endTime: number;
  duration: number;
  memoryBefore: number;
  memoryAfter: number;
  memoryDelta: number;
  metadata?: any;
}

export interface ProfilingSummary {
  totalOperations: number;
  totalDuration: number;
  averageOperationTime: number;
  slowestOperation: string;
  slowestOperationTime: number;
  fastestOperation: string;
  fastestOperationTime: number;
  memoryPeak: number;
  memoryAverage: number;
  operationCounts: { [key: string]: number };
  performanceScore: number;
}

export interface PerformanceRecommendation {
  type: 'optimization' | 'warning' | 'critical';
  category: 'memory' | 'cpu' | 'cache' | 'async' | 'batch';
  title: string;
  description: string;
  impact: 'low' | 'medium' | 'high';
  suggestedAction: string;
  estimatedImprovement: string;
}

export class RhemaPerformanceProfiler {
  private sessions: Map<string, ProfilingSession> = new Map();
  private currentSession: ProfilingSession | null = null;
  private operationStack: ProfiledOperation[] = [];
  private memorySnapshots: MemoryProfile[] = [];

  constructor(private optimizer: RhemaPerformanceOptimizer) {
    this.startMemoryMonitoring();
  }

  // --- Session Management ---

  startSession(sessionId?: string): string {
    const id = sessionId || `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    this.currentSession = {
      id,
      startTime: new Date(),
      operations: [],
      memorySnapshots: [],
      summary: {
        totalOperations: 0,
        totalDuration: 0,
        averageOperationTime: 0,
        slowestOperation: '',
        slowestOperationTime: 0,
        fastestOperation: '',
        fastestOperationTime: Infinity,
        memoryPeak: 0,
        memoryAverage: 0,
        operationCounts: {},
        performanceScore: 0,
      },
    };

    this.sessions.set(id, this.currentSession);
    console.log(`🚀 Started profiling session: ${id}`);

    return id;
  }

  endSession(sessionId?: string): ProfilingSession | null {
    const session = sessionId ? this.sessions.get(sessionId) : this.currentSession;

    if (!session) {
      console.warn('No active profiling session found');
      return null;
    }

    session.endTime = new Date();
    session.duration = session.endTime.getTime() - session.startTime.getTime();
    session.memorySnapshots = [...this.memorySnapshots];

    // Calculate summary
    session.summary = this.calculateSessionSummary(session);

    console.log(`🏁 Ended profiling session: ${session.id}`);
    console.log(`📊 Session summary:`, session.summary);

    if (sessionId === this.currentSession?.id) {
      this.currentSession = null;
    }

    return session;
  }

  // --- Operation Profiling ---

  profileOperation<T>(
    operationName: string,
    operation: () => Promise<T>,
    metadata?: any
  ): Promise<T> {
    return operation();
  }

  profileOperationSync<T>(operationName: string, operation: () => T, metadata?: any): Promise<T> {
    return new Promise((resolve, reject) => {
      if (!this.currentSession) {
        // If no session is active, just execute the operation
        try {
          const result = operation();
          resolve(result);
        } catch (error) {
          reject(error);
        }
        return;
      }

      const startTime = performance.now();
      const memoryBefore = process.memoryUsage().heapUsed;

      const profiledOperation: ProfiledOperation = {
        name: operationName,
        startTime,
        endTime: 0,
        duration: 0,
        memoryBefore,
        memoryAfter: 0,
        memoryDelta: 0,
        metadata,
      };

      this.operationStack.push(profiledOperation);

      try {
        const result = operation();

        const endTime = performance.now();
        const memoryAfter = process.memoryUsage().heapUsed;

        profiledOperation.endTime = endTime;
        profiledOperation.duration = endTime - startTime;
        profiledOperation.memoryAfter = memoryAfter;
        profiledOperation.memoryDelta = memoryAfter - memoryBefore;

        this.recordOperation(profiledOperation);

        resolve(result);
      } catch (error) {
        const endTime = performance.now();
        const memoryAfter = process.memoryUsage().heapUsed;

        profiledOperation.endTime = endTime;
        profiledOperation.duration = endTime - startTime;
        profiledOperation.memoryAfter = memoryAfter;
        profiledOperation.memoryDelta = memoryAfter - memoryBefore;

        this.recordOperation(profiledOperation);

        reject(error);
      } finally {
        this.operationStack.pop();
      }
    });
  }

  private recordOperation(operation: ProfiledOperation): void {
    if (!this.currentSession) return;

    this.currentSession.operations.push(operation);

    // Update summary in real-time
    this.updateSessionSummary(this.currentSession, operation);
  }

  // --- Memory Monitoring ---

  private startMemoryMonitoring(): void {
    setInterval(() => {
      this.takeMemorySnapshot();
    }, 5000); // Every 5 seconds
  }

  private takeMemorySnapshot(): void {
    const usage = process.memoryUsage();
    const snapshot: MemoryProfile = {
      heapUsed: usage.heapUsed,
      heapTotal: usage.heapTotal,
      external: usage.external,
      arrayBuffers: usage.arrayBuffers,
      timestamp: new Date(),
    };

    this.memorySnapshots.push(snapshot);

    // Keep only last 1000 snapshots
    if (this.memorySnapshots.length > 1000) {
      this.memorySnapshots = this.memorySnapshots.slice(-1000);
    }
  }

  // --- Analysis and Reporting ---

  private calculateSessionSummary(session: ProfilingSession): ProfilingSummary {
    const operations = session.operations;
    const memorySnapshots = session.memorySnapshots;

    if (operations.length === 0) {
      return {
        totalOperations: 0,
        totalDuration: 0,
        averageOperationTime: 0,
        slowestOperation: '',
        slowestOperationTime: 0,
        fastestOperation: '',
        fastestOperationTime: 0,
        memoryPeak: 0,
        memoryAverage: 0,
        operationCounts: {},
        performanceScore: 0,
      };
    }

    const totalDuration = operations.reduce((sum, op) => sum + op.duration, 0);
    const averageOperationTime = totalDuration / operations.length;

    const slowestOperation = operations.reduce((slowest, current) =>
      current.duration > slowest.duration ? current : slowest
    );

    const fastestOperation = operations.reduce((fastest, current) =>
      current.duration < fastest.duration ? current : fastest
    );

    const operationCounts: { [key: string]: number } = {};
    operations.forEach((op) => {
      operationCounts[op.name] = (operationCounts[op.name] || 0) + 1;
    });

    const memoryPeak = Math.max(...memorySnapshots.map((s) => s.heapUsed));
    const memoryAverage =
      memorySnapshots.reduce((sum, s) => sum + s.heapUsed, 0) / memorySnapshots.length;

    // Calculate performance score (0-100)
    const performanceScore = this.calculatePerformanceScore(operations, memorySnapshots);

    return {
      totalOperations: operations.length,
      totalDuration,
      averageOperationTime,
      slowestOperation: slowestOperation.name,
      slowestOperationTime: slowestOperation.duration,
      fastestOperation: fastestOperation.name,
      fastestOperationTime: fastestOperation.duration,
      memoryPeak,
      memoryAverage,
      operationCounts,
      performanceScore,
    };
  }

  private updateSessionSummary(session: ProfilingSession, operation: ProfiledOperation): void {
    const summary = session.summary;

    summary.totalOperations++;
    summary.totalDuration += operation.duration;
    summary.averageOperationTime = summary.totalDuration / summary.totalOperations;

    if (operation.duration > summary.slowestOperationTime) {
      summary.slowestOperation = operation.name;
      summary.slowestOperationTime = operation.duration;
    }

    if (operation.duration < summary.fastestOperationTime) {
      summary.fastestOperation = operation.name;
      summary.fastestOperationTime = operation.duration;
    }

    summary.operationCounts[operation.name] = (summary.operationCounts[operation.name] || 0) + 1;
  }

  private calculatePerformanceScore(
    operations: ProfiledOperation[],
    memorySnapshots: MemoryProfile[]
  ): number {
    // Base score starts at 100
    let score = 100;

    // Deduct points for slow operations
    const slowOperations = operations.filter((op) => op.duration > 100); // > 100ms
    score -= slowOperations.length * 2;

    // Deduct points for memory issues
    const memoryPeak = Math.max(...memorySnapshots.map((s) => s.heapUsed));
    if (memoryPeak > 200 * 1024 * 1024) {
      // > 200MB
      score -= 10;
    }

    // Deduct points for memory leaks
    const memoryGrowth =
      memorySnapshots[memorySnapshots.length - 1]?.heapUsed - memorySnapshots[0]?.heapUsed;
    if (memoryGrowth > 50 * 1024 * 1024) {
      // > 50MB growth
      score -= 15;
    }

    // Ensure score is between 0 and 100
    return Math.max(0, Math.min(100, score));
  }

  // --- Recommendations ---

  generateRecommendations(sessionId: string): PerformanceRecommendation[] {
    const session = this.sessions.get(sessionId);
    if (!session) {
      return [];
    }

    const recommendations: PerformanceRecommendation[] = [];
    const summary = session.summary;

    // Memory recommendations
    if (summary.memoryPeak > 200 * 1024 * 1024) {
      recommendations.push({
        type: 'warning',
        category: 'memory',
        title: 'High Memory Usage',
        description: `Peak memory usage was ${this.formatBytes(summary.memoryPeak)}`,
        impact: 'medium',
        suggestedAction: 'Consider implementing memory pooling or reducing cache size',
        estimatedImprovement: '20-30% memory reduction',
      });
    }

    // Slow operation recommendations
    if (summary.slowestOperationTime > 500) {
      recommendations.push({
        type: 'optimization',
        category: 'cpu',
        title: 'Slow Operations Detected',
        description: `${summary.slowestOperation} took ${summary.slowestOperationTime.toFixed(2)}ms`,
        impact: 'high',
        suggestedAction: 'Consider caching results or optimizing algorithm',
        estimatedImprovement: '50-70% performance improvement',
      });
    }

    // Cache recommendations
    const cacheStats = this.optimizer.getConfiguration();
    if (cacheStats.cacheSize < 100) {
      recommendations.push({
        type: 'optimization',
        category: 'cache',
        title: 'Small Cache Size',
        description: `Current cache size is ${cacheStats.cacheSize}`,
        impact: 'medium',
        suggestedAction: 'Increase cache size to improve hit rates',
        estimatedImprovement: '15-25% performance improvement',
      });
    }

    // Batch processing recommendations
    if (summary.totalOperations > 100 && !cacheStats.enableBatchProcessing) {
      recommendations.push({
        type: 'optimization',
        category: 'batch',
        title: 'Enable Batch Processing',
        description: `${summary.totalOperations} operations could benefit from batching`,
        impact: 'high',
        suggestedAction: 'Enable batch processing for similar operations',
        estimatedImprovement: '30-50% performance improvement',
      });
    }

    return recommendations;
  }

  // --- Session Management ---

  getSession(sessionId: string): ProfilingSession | null {
    return this.sessions.get(sessionId) || null;
  }

  getAllSessions(): ProfilingSession[] {
    return Array.from(this.sessions.values());
  }

  clearSessions(): void {
    this.sessions.clear();
    this.currentSession = null;
    this.memorySnapshots = [];
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
    this.clearSessions();
  }
}
