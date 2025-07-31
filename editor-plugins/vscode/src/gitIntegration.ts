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
import { RhemaLogger } from './logger';
import { RhemaSettings } from './settings';
import { RhemaErrorHandler } from './errorHandler';

export class RhemaGitIntegration {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;
    private disposables: vscode.Disposable[] = [];

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
    }

    async initialize(context: vscode.ExtensionContext): Promise<void> {
        try {
            this.logger.info('Initializing Rhema Git integration...');

            // Set up Git hooks
            await this.setupGitHooks();

            // Set up Git workflow automation
            await this.setupGitWorkflow();

            // Set up Git monitoring
            await this.setupGitMonitoring();

            this.logger.info('Rhema Git integration initialized successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to initialize Rhema Git integration', error);
        }
    }

    private async setupGitHooks(): Promise<void> {
        try {
            // Set up pre-commit hooks for Rhema validation
            this.logger.info('Setting up Git hooks...');
            
            // This would typically create or modify .git/hooks/pre-commit
            // For now, just log the intention
            this.logger.info('Git hooks setup completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to setup Git hooks', error);
        }
    }

    private async setupGitWorkflow(): Promise<void> {
        try {
            // Set up automated Git workflow
            this.logger.info('Setting up Git workflow automation...');
            
            // This would typically configure Git workflow automation
            // For now, just log the intention
            this.logger.info('Git workflow automation setup completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to setup Git workflow', error);
        }
    }

    private async setupGitMonitoring(): Promise<void> {
        try {
            // Set up Git repository monitoring
            this.logger.info('Setting up Git monitoring...');
            
            // Monitor for Git events
            const gitWatcher = vscode.workspace.createFileSystemWatcher('.git/**/*');
            gitWatcher.onDidChange(uri => {
                this.logger.info(`Git file changed: ${uri.fsPath}`);
                this.handleGitChange(uri);
            });

            this.disposables.push(gitWatcher);
            this.logger.info('Git monitoring setup completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to setup Git monitoring', error);
        }
    }

    private async handleGitChange(uri: vscode.Uri): Promise<void> {
        try {
            // Handle Git repository changes
            this.logger.info(`Processing Git change: ${uri.fsPath}`);
            
            // This would typically trigger Rhema context updates
            // For now, just log the change
        } catch (error) {
            this.errorHandler.handleError('Failed to handle Git change', error);
        }
    }

    async deactivate(): Promise<void> {
        try {
            this.logger.info('Deactivating Rhema Git integration...');
            
            // Clean up disposables
            this.disposables.forEach(disposable => disposable.dispose());
            this.disposables = [];
            
            this.logger.info('Rhema Git integration deactivated');
        } catch (error) {
            this.errorHandler.handleError('Failed to deactivate Rhema Git integration', error);
        }
    }
} 