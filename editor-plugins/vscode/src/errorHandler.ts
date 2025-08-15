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
import type { RhemaLogger } from './logger';

export class RhemaErrorHandler {
  private logger: RhemaLogger;

  constructor(logger: RhemaLogger) {
    this.logger = logger;
  }

  handleError(message: string, error: any, showNotification: boolean = true): void {
    const errorMessage = error instanceof Error ? error.message : String(error);
    const fullMessage = `${message}: ${errorMessage}`;

    // Log the error
    this.logger.error(fullMessage, error);

    // Show notification if enabled
    if (showNotification) {
      vscode.window.showErrorMessage(fullMessage);
    }
  }

  handleWarning(message: string, error?: any, showNotification: boolean = true): void {
    const errorMessage = error instanceof Error ? error.message : String(error);
    const fullMessage = error ? `${message}: ${errorMessage}` : message;

    // Log the warning
    this.logger.warn(fullMessage, error);

    // Show notification if enabled
    if (showNotification) {
      vscode.window.showWarningMessage(fullMessage);
    }
  }

  handleInfo(message: string, showNotification: boolean = false): void {
    // Log the info
    this.logger.info(message);

    // Show notification if enabled
    if (showNotification) {
      vscode.window.showInformationMessage(message);
    }
  }

  async handleAsyncError(
    message: string,
    error: any,
    showNotification: boolean = true
  ): Promise<void> {
    this.handleError(message, error, showNotification);
  }

  async handleAsyncWarning(
    message: string,
    error?: any,
    showNotification: boolean = true
  ): Promise<void> {
    this.handleWarning(message, error, showNotification);
  }

  async handleAsyncInfo(message: string, showNotification: boolean = false): Promise<void> {
    this.handleInfo(message, showNotification);
  }

  createErrorHandler(showNotification: boolean = true) {
    return (message: string, error: any) => {
      this.handleError(message, error, showNotification);
    };
  }

  createWarningHandler(showNotification: boolean = true) {
    return (message: string, error?: any) => {
      this.handleWarning(message, error, showNotification);
    };
  }

  createInfoHandler(showNotification: boolean = false) {
    return (message: string) => {
      this.handleInfo(message, showNotification);
    };
  }
}
