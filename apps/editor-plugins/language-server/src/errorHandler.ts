import { type RhemaLogger, LogLevel } from './logger';

export interface ErrorInfo {
  timestamp: Date;
  message: string;
  error: Error;
  context?: any;
  handled: boolean;
}

export class RhemaErrorHandler {
  private logger: RhemaLogger;
  private errors: ErrorInfo[] = [];
  private maxErrors: number = 100;

  constructor(logger: RhemaLogger) {
    this.logger = logger;
  }

  handleError(message: string, error: Error, context?: any): void {
    const errorInfo: ErrorInfo = {
      timestamp: new Date(),
      message,
      error,
      context,
      handled: true,
    };

    this.errors.push(errorInfo);

    // Keep only the last maxErrors
    if (this.errors.length > this.maxErrors) {
      this.errors = this.errors.slice(-this.maxErrors);
    }

    // Log the error
    this.logger.error(message, context, error);

    // Additional error handling logic can be added here
    this.processError(errorInfo);
  }

  handleWarning(message: string, error?: Error, context?: any): void {
    this.logger.warn(message, context, error);
  }

  handleInfo(message: string, context?: any): void {
    this.logger.info(message, context);
  }

  getErrors(limit?: number): ErrorInfo[] {
    if (limit) {
      return this.errors.slice(-limit);
    }
    return [...this.errors];
  }

  getRecentErrors(hours: number = 1): ErrorInfo[] {
    const cutoff = new Date(Date.now() - hours * 60 * 60 * 1000);
    return this.errors.filter((error) => error.timestamp > cutoff);
  }

  getErrorStats(): {
    total: number;
    recent: number;
    byType: { [key: string]: number };
    unhandled: number;
  } {
    const byType: { [key: string]: number } = {};
    let unhandled = 0;
    const oneHourAgo = new Date(Date.now() - 60 * 60 * 1000);
    let recent = 0;

    this.errors.forEach((error) => {
      const errorType = error.error.constructor.name;
      byType[errorType] = (byType[errorType] || 0) + 1;

      if (!error.handled) {
        unhandled++;
      }

      if (error.timestamp > oneHourAgo) {
        recent++;
      }
    });

    return {
      total: this.errors.length,
      recent,
      byType,
      unhandled,
    };
  }

  clearErrors(): void {
    this.errors = [];
  }

  private processError(errorInfo: ErrorInfo): void {
    // Add specific error processing logic here
    // For example, categorize errors, send notifications, etc.

    const error = errorInfo.error;

    // Handle specific error types
    if (error.name === 'SyntaxError') {
      this.handleSyntaxError(errorInfo);
    } else if (error.name === 'ValidationError') {
      this.handleValidationError(errorInfo);
    } else if (error.name === 'NetworkError') {
      this.handleNetworkError(errorInfo);
    } else {
      this.handleGenericError(errorInfo);
    }
  }

  private handleSyntaxError(errorInfo: ErrorInfo): void {
    // Handle YAML syntax errors
    this.logger.warn('Syntax error detected, suggesting format fix', errorInfo.context);
  }

  private handleValidationError(errorInfo: ErrorInfo): void {
    // Handle validation errors
    this.logger.warn('Validation error detected', errorInfo.context);
  }

  private handleNetworkError(errorInfo: ErrorInfo): void {
    // Handle network-related errors
    this.logger.error('Network error detected', errorInfo.context, errorInfo.error);
  }

  private handleGenericError(errorInfo: ErrorInfo): void {
    // Handle generic errors
    this.logger.error('Generic error detected', errorInfo.context, errorInfo.error);
  }
}
