export interface PerformanceMetric {
  operation: string;
  duration: number;
  timestamp: Date;
  metadata?: any;
}

export interface PerformanceReport {
  totalOperations: number;
  averageDuration: number;
  slowestOperations: PerformanceMetric[];
  fastestOperations: PerformanceMetric[];
  operationsByType: { [key: string]: number };
  memoryUsage: number;
  uptime: number;
}

export class RhemaPerformanceMonitor {
  private metrics: PerformanceMetric[] = [];
  private maxMetrics: number = 1000;
  private startTime: Date = new Date();
  private isEnabled: boolean = true;

  initialize(): void {
    this.startTime = new Date();
    this.isEnabled = true;
  }

  recordOperation(operation: string, duration: number, metadata?: any): void {
    if (!this.isEnabled) return;

    const metric: PerformanceMetric = {
      operation,
      duration,
      timestamp: new Date(),
      metadata,
    };

    this.metrics.push(metric);

    // Keep only the last maxMetrics
    if (this.metrics.length > this.maxMetrics) {
      this.metrics = this.metrics.slice(-this.maxMetrics);
    }
  }

  getReport(): PerformanceReport {
    if (this.metrics.length === 0) {
      return {
        totalOperations: 0,
        averageDuration: 0,
        slowestOperations: [],
        fastestOperations: [],
        operationsByType: {},
        memoryUsage: this.getMemoryUsage(),
        uptime: this.getUptime(),
      };
    }

    // Calculate statistics
    const totalDuration = this.metrics.reduce((sum, metric) => sum + metric.duration, 0);
    const averageDuration = totalDuration / this.metrics.length;

    // Group by operation type
    const operationsByType: { [key: string]: number } = {};
    this.metrics.forEach((metric) => {
      operationsByType[metric.operation] = (operationsByType[metric.operation] || 0) + 1;
    });

    // Get slowest and fastest operations
    const sortedByDuration = [...this.metrics].sort((a, b) => b.duration - a.duration);
    const slowestOperations = sortedByDuration.slice(0, 5);
    const fastestOperations = sortedByDuration.slice(-5).reverse();

    return {
      totalOperations: this.metrics.length,
      averageDuration,
      slowestOperations,
      fastestOperations,
      operationsByType,
      memoryUsage: this.getMemoryUsage(),
      uptime: this.getUptime(),
    };
  }

  getOperationStats(operation: string): {
    count: number;
    averageDuration: number;
    minDuration: number;
    maxDuration: number;
    totalDuration: number;
  } {
    const operationMetrics = this.metrics.filter((m) => m.operation === operation);

    if (operationMetrics.length === 0) {
      return {
        count: 0,
        averageDuration: 0,
        minDuration: 0,
        maxDuration: 0,
        totalDuration: 0,
      };
    }

    const durations = operationMetrics.map((m) => m.duration);
    const totalDuration = durations.reduce((sum, duration) => sum + duration, 0);
    const averageDuration = totalDuration / operationMetrics.length;
    const minDuration = Math.min(...durations);
    const maxDuration = Math.max(...durations);

    return {
      count: operationMetrics.length,
      averageDuration,
      minDuration,
      maxDuration,
      totalDuration,
    };
  }

  getRecentMetrics(seconds: number = 60): PerformanceMetric[] {
    const cutoff = new Date(Date.now() - seconds * 1000);
    return this.metrics.filter((metric) => metric.timestamp > cutoff);
  }

  clearMetrics(): void {
    this.metrics = [];
  }

  enable(): void {
    this.isEnabled = true;
  }

  disable(): void {
    this.isEnabled = false;
  }

  isMonitoringEnabled(): boolean {
    return this.isEnabled;
  }

  private getMemoryUsage(): number {
    if (typeof process !== 'undefined' && process.memoryUsage) {
      return process.memoryUsage().heapUsed / 1024 / 1024; // MB
    }
    return 0;
  }

  private getUptime(): number {
    return Date.now() - this.startTime.getTime();
  }

  shutdown(): void {
    this.isEnabled = false;
    // Could add cleanup logic here
  }
}
