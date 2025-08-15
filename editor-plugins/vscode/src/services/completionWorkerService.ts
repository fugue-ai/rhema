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
import { Worker } from 'worker_threads';
import * as path from 'path';
import { RhemaLogger } from '../logger';
import { RhemaErrorHandler } from '../errorHandler';

export interface CompletionRequest {
  id: string;
  document: {
    fileName: string;
    languageId: string;
    content: string;
  };
  position: {
    line: number;
    character: number;
  };
  line: string;
  word: string;
  context: any;
  timestamp: number;
}

export interface CompletionResponse {
  id: string;
  completions: vscode.CompletionItem[];
  error?: string;
  timestamp: number;
}

export interface CompletionWorkerMessage {
  type: 'request' | 'response' | 'error' | 'ping' | 'pong';
  data: CompletionRequest | CompletionResponse | string;
}

export class CompletionWorkerService {
  private worker: Worker | null = null;
  private logger: RhemaLogger;
  private errorHandler: RhemaErrorHandler;
  private pendingRequests: Map<string, {
    resolve: (completions: vscode.CompletionItem[]) => void;
    reject: (error: Error) => void;
    timeout: NodeJS.Timeout;
  }> = new Map();
  private isInitialized = false;
  private workerPath: string;
  private completionTimeout = 10000;

  constructor() {
    this.logger = new RhemaLogger();
    this.errorHandler = new RhemaErrorHandler(this.logger);
    this.workerPath = path.join(__dirname, '..', 'workers', 'completionWorker.js');
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing CompletionWorkerService...');
      
      // Create the worker thread
      await this.createWorker();
      
      // Set up message handling
      this.setupMessageHandling();
      
      // Test the worker connection
      await this.testWorkerConnection();
      
      this.isInitialized = true;
      this.logger.info('CompletionWorkerService initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize CompletionWorkerService', error);
      throw error;
    }
  }

  private async createWorker(): Promise<void> {
    try {
      this.worker = new Worker(this.workerPath, {
        workerData: {
          extensionPath: path.dirname(this.workerPath),
        },
      });

      this.worker.on('error', (error) => {
        this.errorHandler.handleError('Completion worker error', error);
        this.handleWorkerError();
      });

      this.worker.on('exit', (code) => {
        if (code !== 0) {
          this.logger.warn(`Completion worker exited with code ${code}`);
          this.handleWorkerError();
        }
      });

      this.logger.info('Completion worker created successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to create completion worker', error);
      throw error;
    }
  }

  private setupMessageHandling(): void {
    if (!this.worker) {
      throw new Error('Worker not initialized');
    }

    this.worker.on('message', (message: CompletionWorkerMessage) => {
      try {
        switch (message.type) {
          case 'response':
            this.handleCompletionResponse(message.data as CompletionResponse);
            break;
          case 'error':
            this.handleWorkerError();
            break;
          case 'pong':
            this.logger.debug('Worker ping-pong successful');
            break;
          default:
            this.logger.warn(`Unknown message type: ${message.type}`);
        }
      } catch (error) {
        this.errorHandler.handleError('Error handling worker message', error);
      }
    });
  }

  private handleCompletionResponse(response: CompletionResponse): void {
    const request = this.pendingRequests.get(response.id);
    if (!request) {
      this.logger.warn(`Received response for unknown request: ${response.id}`);
      return;
    }

    // Clear the timeout
    clearTimeout(request.timeout);

    // Remove from pending requests
    this.pendingRequests.delete(response.id);

    if (response.error) {
      request.reject(new Error(response.error));
    } else {
      request.resolve(response.completions);
    }
  }

  private handleWorkerError(): void {
    // Reject all pending requests
    for (const [id, request] of this.pendingRequests) {
      clearTimeout(request.timeout);
      request.reject(new Error('Worker error occurred'));
    }
    this.pendingRequests.clear();

    // Attempt to restart the worker
    this.restartWorker();
  }

  private async restartWorker(): Promise<void> {
    try {
      this.logger.info('Restarting completion worker...');
      
      if (this.worker) {
        this.worker.terminate();
        this.worker = null;
      }

      await this.createWorker();
      this.setupMessageHandling();
      
      this.logger.info('Completion worker restarted successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to restart completion worker', error);
    }
  }

  private async testWorkerConnection(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error('Worker not initialized'));
        return;
      }

      const timeout = setTimeout(() => {
        reject(new Error('Worker connection test timeout'));
      }, 5000);

      const messageHandler = (message: CompletionWorkerMessage) => {
        if (message.type === 'pong') {
          clearTimeout(timeout);
          this.worker!.off('message', messageHandler);
          resolve();
        }
      };

      this.worker.on('message', messageHandler);
      this.worker.postMessage({ type: 'ping', data: 'test' });
    });
  }

  async getCompletions(request: Omit<CompletionRequest, 'id' | 'timestamp'>): Promise<vscode.CompletionItem[]> {
    if (!this.isInitialized || !this.worker) {
      throw new Error('CompletionWorkerService not initialized');
    }

    const id = this.generateRequestId();
    const fullRequest: CompletionRequest = {
      ...request,
      id,
      timestamp: Date.now(),
    };

    return new Promise((resolve, reject) => {
      // Set up timeout
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(id);
        reject(new Error('Completion request timeout'));
      }, this.completionTimeout);

      // Store the request
      this.pendingRequests.set(id, { resolve, reject, timeout });

      // Send the request to the worker
      this.worker!.postMessage({
        type: 'request',
        data: fullRequest,
      });
    });
  }

  private generateRequestId(): string {
    return `completion_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  async dispose(): Promise<void> {
    try {
      // Reject all pending requests
      for (const [id, request] of this.pendingRequests) {
        clearTimeout(request.timeout);
        request.reject(new Error('Service disposed'));
      }
      this.pendingRequests.clear();

      // Terminate the worker
      if (this.worker) {
        this.worker.terminate();
        this.worker = null;
      }

      this.isInitialized = false;
      this.logger.info('CompletionWorkerService disposed');
    } catch (error) {
      this.errorHandler.handleError('Error disposing CompletionWorkerService', error);
    }
  }

  isReady(): boolean {
    return this.isInitialized && this.worker !== null;
  }

  getPendingRequestCount(): number {
    return this.pendingRequests.size;
  }

  setCompletionTimeout(timeout: number): void {
    this.completionTimeout = Math.max(1000, Math.min(30000, timeout)); // Limit between 1s and 30s
    this.logger.info(`Completion timeout set to ${this.completionTimeout}ms`);
  }
}
