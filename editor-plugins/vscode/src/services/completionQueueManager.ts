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
import { RhemaErrorHandler } from '../errorHandler';
import { CompletionWorkerService, CompletionRequest } from './completionWorkerService';

export interface QueuedCompletionRequest {
  id: string;
  document: vscode.TextDocument;
  position: vscode.Position;
  line: string;
  word: string;
  context: any;
  priority: number;
  timestamp: number;
  resolve: (completions: vscode.CompletionItem[]) => void;
  reject: (error: Error) => void;
}

export interface CompletionQueueStats {
  pendingRequests: number;
  completedRequests: number;
  failedRequests: number;
  averageResponseTime: number;
  workerReady: boolean;
}

export class CompletionQueueManager {
  private queue: QueuedCompletionRequest[] = [];
  private workerService: CompletionWorkerService;
  private logger: RhemaLogger;
  private errorHandler: RhemaErrorHandler;
  private isProcessing = false;
  private maxConcurrentRequests = 3;
  private completionTimeout = 10000;
  private activeRequests = 0;
  private stats = {
    pendingRequests: 0,
    completedRequests: 0,
    failedRequests: 0,
    totalResponseTime: 0,
    responseCount: 0,
  };
  private processingInterval: NodeJS.Timeout | null = null;

  constructor(workerService: CompletionWorkerService) {
    this.workerService = workerService;
    this.logger = new RhemaLogger();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  setMaxConcurrentRequests(max: number): void {
    this.maxConcurrentRequests = Math.max(1, Math.min(10, max)); // Limit between 1 and 10
    this.logger.info(`Max concurrent requests set to ${this.maxConcurrentRequests}`);
  }

  setCompletionTimeout(timeout: number): void {
    this.completionTimeout = Math.max(1000, Math.min(30000, timeout)); // Limit between 1s and 30s
    this.logger.info(`Completion timeout set to ${this.completionTimeout}ms`);
  }

  async initialize(): Promise<void> {
    try {
      this.logger.info('Initializing CompletionQueueManager...');
      
      // Start the processing loop
      this.startProcessingLoop();
      
      this.logger.info('CompletionQueueManager initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize CompletionQueueManager', error);
      throw error;
    }
  }

  async enqueueCompletion(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string,
    context: any,
    priority: number = 0
  ): Promise<vscode.CompletionItem[]> {
    return new Promise((resolve, reject) => {
      const request: QueuedCompletionRequest = {
        id: this.generateRequestId(),
        document,
        position,
        line,
        word,
        context,
        priority,
        timestamp: Date.now(),
        resolve,
        reject,
      };

      // Add to queue with priority sorting
      this.addToQueue(request);
      this.stats.pendingRequests++;
    });
  }

  private addToQueue(request: QueuedCompletionRequest): void {
    // Insert based on priority (higher priority first)
    let insertIndex = 0;
    for (let i = 0; i < this.queue.length; i++) {
      if (this.queue[i].priority < request.priority) {
        insertIndex = i;
        break;
      }
      insertIndex = i + 1;
    }

    this.queue.splice(insertIndex, 0, request);
    this.logger.debug(`Added completion request to queue: ${request.id} (priority: ${request.priority})`);
  }

  private startProcessingLoop(): void {
    this.processingInterval = setInterval(() => {
      this.processQueue();
    }, 50); // Process every 50ms
  }

  private async processQueue(): Promise<void> {
    if (this.isProcessing || this.queue.length === 0 || this.activeRequests >= this.maxConcurrentRequests) {
      return;
    }

    if (!this.workerService.isReady()) {
      this.logger.debug('Worker service not ready, skipping queue processing');
      return;
    }

    this.isProcessing = true;

    try {
      // Process up to maxConcurrentRequests items
      const requestsToProcess = Math.min(
        this.maxConcurrentRequests - this.activeRequests,
        this.queue.length
      );

      for (let i = 0; i < requestsToProcess; i++) {
        const request = this.queue.shift();
        if (request) {
          this.processRequest(request);
        }
      }
    } catch (error) {
      this.errorHandler.handleError('Error processing completion queue', error);
    } finally {
      this.isProcessing = false;
    }
  }

  private async processRequest(request: QueuedCompletionRequest): Promise<void> {
    this.activeRequests++;
    this.stats.pendingRequests--;

    try {
      const startTime = Date.now();

      // Convert to worker request format
      const workerRequest: Omit<CompletionRequest, 'id' | 'timestamp'> = {
        document: {
          fileName: request.document.fileName,
          languageId: request.document.languageId,
          content: request.document.getText(),
        },
        position: {
          line: request.position.line,
          character: request.position.character,
        },
        line: request.line,
        word: request.word,
        context: request.context,
      };

      // Send to worker
      const completions = await this.workerService.getCompletions(workerRequest);

      // Calculate response time
      const responseTime = Date.now() - startTime;
      this.stats.totalResponseTime += responseTime;
      this.stats.responseCount++;
      this.stats.completedRequests++;

      this.logger.debug(`Completion request ${request.id} completed in ${responseTime}ms`);

      // Resolve the promise
      request.resolve(completions);
    } catch (error) {
      this.stats.failedRequests++;
      this.errorHandler.handleError(`Error processing completion request ${request.id}`, error);
      request.reject(error as Error);
    } finally {
      this.activeRequests--;
    }
  }

  private generateRequestId(): string {
    return `queue_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  getStats(): CompletionQueueStats {
    const averageResponseTime = this.stats.responseCount > 0
      ? this.stats.totalResponseTime / this.stats.responseCount
      : 0;

    return {
      pendingRequests: this.queue.length,
      completedRequests: this.stats.completedRequests,
      failedRequests: this.stats.failedRequests,
      averageResponseTime,
      workerReady: this.workerService.isReady(),
    };
  }

  clearQueue(): void {
    // Reject all pending requests
    for (const request of this.queue) {
      request.reject(new Error('Queue cleared'));
    }
    this.queue = [];
    this.stats.pendingRequests = 0;
    this.logger.info('Completion queue cleared');
  }

  async dispose(): Promise<void> {
    try {
      // Stop the processing loop
      if (this.processingInterval) {
        clearInterval(this.processingInterval);
        this.processingInterval = null;
      }

      // Clear the queue
      this.clearQueue();

      this.logger.info('CompletionQueueManager disposed');
    } catch (error) {
      this.errorHandler.handleError('Error disposing CompletionQueueManager', error);
    }
  }
}
