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
import * as path from 'path';
import { LanguageClient } from 'vscode-languageclient/node';
import { RhemaLogger } from './logger';
import { RhemaSettings } from './settings';
import { RhemaErrorHandler } from './errorHandler';

export class RhemaLanguageServer {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private client: LanguageClient | null = null;
  private disposables: vscode.Disposable[] = [];

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Rhema language server...');

      // Get the language server path
      const serverPath = this.getLanguageServerPath(context);

      if (!serverPath) {
        this.logger.warn('Language server not found, skipping initialization');
        return;
      }

      // Create the language client
      this.client = new LanguageClient(
        'rhema-language-server',
        'Rhema Language Server',
        {
          run: {
            command: serverPath,
            args: ['--log-level=info'],
          },
          debug: {
            command: serverPath,
            args: ['--log-level=debug'],
          },
        },
        {
          documentSelector: [
            { language: 'yaml', scheme: 'file' },
            { language: 'rhema-yaml', scheme: 'file' },
          ],
          synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.rhema.{yaml,yml}'),
          },
        }
      );

      // Start the client
      await this.client.start();

      this.logger.info('Rhema language server initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema language server', error);
    }
  }

  private getLanguageServerPath(context: vscode.ExtensionContext): string | null {
    // Try to find the language server in the extension directory
    const extensionPath = context.extensionPath;
    const possiblePaths = [
      path.join(extensionPath, '..', 'language-server', 'dist', 'server.js'),
      path.join(extensionPath, '..', 'language-server', 'dist', 'cli.js'),
      path.join(extensionPath, 'node_modules', 'rhema-language-server', 'dist', 'server.js'),
      path.join(extensionPath, 'node_modules', 'rhema-language-server', 'dist', 'cli.js'),
    ];

    for (const serverPath of possiblePaths) {
      if (require('fs').existsSync(serverPath)) {
        return serverPath;
      }
    }

    // Try to find it in PATH
    try {
      const { execSync } = require('child_process');
      execSync('rhema-language-server --version', { stdio: 'ignore' });
      return 'rhema-language-server';
    } catch (error) {
      // Language server not found in PATH
    }

    return null;
  }

  async sendRequest<T>(method: string, params: any): Promise<T> {
    if (!this.client) {
      throw new Error('Language server client not initialized');
    }

    try {
      return await this.client.sendRequest(method, params);
    } catch (error) {
      this.errorHandler.handleError(`Failed to send request to language server: ${method}`, error);
      throw error;
    }
  }

  async sendNotification(method: string, params: any): Promise<void> {
    if (!this.client) {
      throw new Error('Language server client not initialized');
    }

    try {
      await this.client.sendNotification(method, params);
    } catch (error) {
      this.errorHandler.handleError(
        `Failed to send notification to language server: ${method}`,
        error
      );
      throw error;
    }
  }

  async deactivate(): Promise<void> {
    await this.dispose();
  }

  async dispose(): Promise<void> {
    if (this.client) {
      await this.client.stop();
      this.client.dispose();
      this.client = null;
    }

    this.disposables.forEach((disposable) => disposable.dispose());
    this.disposables = [];
  }

  isInitialized(): boolean {
    return this.client !== null;
  }

  getClient(): LanguageClient | null {
    return this.client;
  }
}
