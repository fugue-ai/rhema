export enum LogLevel {
  ERROR = 0,
  WARN = 1,
  INFO = 2,
  DEBUG = 3,
  TRACE = 4,
}

export interface LogEntry {
  timestamp: Date;
  level: LogLevel;
  message: string;
  data?: any;
  error?: Error;
}

export class RhemaLogger {
  private logLevel: LogLevel = LogLevel.INFO;
  private maxEntries: number = 1000;
  private entries: LogEntry[] = [];
  private connection: any = null;

  initialize(connection: any, logLevel?: LogLevel): void {
    this.connection = connection;
    if (logLevel !== undefined) {
      this.logLevel = logLevel;
    }
  }

  setLogLevel(level: LogLevel): void {
    this.logLevel = level;
  }

  error(message: string, data?: any, error?: Error): void {
    this.log(LogLevel.ERROR, message, data, error);
  }

  warn(message: string, data?: any, error?: Error): void {
    this.log(LogLevel.WARN, message, data, error);
  }

  info(message: string, data?: any, error?: Error): void {
    this.log(LogLevel.INFO, message, data, error);
  }

  debug(message: string, data?: any, error?: Error): void {
    this.log(LogLevel.DEBUG, message, data, error);
  }

  trace(message: string, data?: any, error?: Error): void {
    this.log(LogLevel.TRACE, message, data, error);
  }

  private log(level: LogLevel, message: string, data?: any, error?: Error): void {
    if (level > this.logLevel) {
      return;
    }

    const entry: LogEntry = {
      timestamp: new Date(),
      level,
      message,
      data,
      error,
    };

    this.entries.push(entry);

    // Keep only the last maxEntries
    if (this.entries.length > this.maxEntries) {
      this.entries = this.entries.slice(-this.maxEntries);
    }

    // Send to connection if available
    if (this.connection) {
      const levelString = LogLevel[level];
      const logMessage = `[${levelString}] ${message}`;

      switch (level) {
        case LogLevel.ERROR:
          this.connection.console.error(logMessage);
          break;
        case LogLevel.WARN:
          this.connection.console.warn(logMessage);
          break;
        case LogLevel.INFO:
          this.connection.console.info(logMessage);
          break;
        case LogLevel.DEBUG:
        case LogLevel.TRACE:
          this.connection.console.log(logMessage);
          break;
      }
    }

    // Also log to console for debugging
    if (process.env.NODE_ENV === 'development') {
      const timestamp = entry.timestamp.toISOString();
      const levelString = LogLevel[level].padEnd(5);
      console.log(`${timestamp} [${levelString}] ${message}`);

      if (data) {
        console.log('Data:', data);
      }

      if (error) {
        console.error('Error:', error);
      }
    }
  }

  getEntries(level?: LogLevel, limit?: number): LogEntry[] {
    let filtered = this.entries;

    if (level !== undefined) {
      filtered = filtered.filter((entry) => entry.level <= level);
    }

    if (limit !== undefined) {
      filtered = filtered.slice(-limit);
    }

    return filtered;
  }

  clear(): void {
    this.entries = [];
  }

  getStats(): {
    total: number;
    byLevel: { [key: string]: number };
    recentErrors: number;
  } {
    const byLevel: { [key: string]: number } = {};
    let recentErrors = 0;
    const oneHourAgo = new Date(Date.now() - 60 * 60 * 1000);

    this.entries.forEach((entry) => {
      const levelName = LogLevel[entry.level];
      byLevel[levelName] = (byLevel[levelName] || 0) + 1;

      if (entry.level === LogLevel.ERROR && entry.timestamp > oneHourAgo) {
        recentErrors++;
      }
    });

    return {
      total: this.entries.length,
      byLevel,
      recentErrors,
    };
  }
}
