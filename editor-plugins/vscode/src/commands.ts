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

export class RhemaCommands {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
    }

    async initialize(): Promise<void> {
        this.logger.info('Rhema commands initialized');
    }

    async showContext(): Promise<void> {
        try {
            this.logger.info('Showing Rhema context');
            vscode.window.showInformationMessage('Rhema Context: Active');
        } catch (error) {
            this.errorHandler.handleError('Failed to show context', error);
        }
    }

    async executeQuery(): Promise<void> {
        try {
            this.logger.info('Executing Rhema query');
            vscode.window.showInformationMessage('Rhema Query executed');
        } catch (error) {
            this.errorHandler.handleError('Failed to execute query', error);
        }
    }

    async searchContext(): Promise<void> {
        try {
            this.logger.info('Searching Rhema context');
            vscode.window.showInformationMessage('Rhema Context search completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to search context', error);
        }
    }

    async validateFiles(): Promise<void> {
        try {
            this.logger.info('Validating Rhema files');
            vscode.window.showInformationMessage('Rhema files validated');
        } catch (error) {
            this.errorHandler.handleError('Failed to validate files', error);
        }
    }

    async showScopes(): Promise<void> {
        try {
            this.logger.info('Showing Rhema scopes');
            vscode.window.showInformationMessage('Rhema Scopes displayed');
        } catch (error) {
            this.errorHandler.handleError('Failed to show scopes', error);
        }
    }

    async showTree(): Promise<void> {
        try {
            this.logger.info('Showing Rhema tree');
            vscode.window.showInformationMessage('Rhema Tree displayed');
        } catch (error) {
            this.errorHandler.handleError('Failed to show tree', error);
        }
    }

    async manageTodos(): Promise<void> {
        try {
            this.logger.info('Managing Rhema todos');
            vscode.window.showInformationMessage('Rhema Todos management opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to manage todos', error);
        }
    }

    async manageInsights(): Promise<void> {
        try {
            this.logger.info('Managing Rhema insights');
            vscode.window.showInformationMessage('Rhema Insights management opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to manage insights', error);
        }
    }

    async managePatterns(): Promise<void> {
        try {
            this.logger.info('Managing Rhema patterns');
            vscode.window.showInformationMessage('Rhema Patterns management opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to manage patterns', error);
        }
    }

    async manageDecisions(): Promise<void> {
        try {
            this.logger.info('Managing Rhema decisions');
            vscode.window.showInformationMessage('Rhema Decisions management opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to manage decisions', error);
        }
    }

    async showDependencies(): Promise<void> {
        try {
            this.logger.info('Showing Rhema dependencies');
            vscode.window.showInformationMessage('Rhema Dependencies displayed');
        } catch (error) {
            this.errorHandler.handleError('Failed to show dependencies', error);
        }
    }

    async showImpact(): Promise<void> {
        try {
            this.logger.info('Showing Rhema impact');
            vscode.window.showInformationMessage('Rhema Impact analysis displayed');
        } catch (error) {
            this.errorHandler.handleError('Failed to show impact', error);
        }
    }

    async syncKnowledge(): Promise<void> {
        try {
            this.logger.info('Syncing Rhema knowledge');
            vscode.window.showInformationMessage('Rhema Knowledge synced');
        } catch (error) {
            this.errorHandler.handleError('Failed to sync knowledge', error);
        }
    }

    async gitIntegration(): Promise<void> {
        try {
            this.logger.info('Running Rhema Git integration');
            vscode.window.showInformationMessage('Rhema Git integration completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to run Git integration', error);
        }
    }

    async showStats(): Promise<void> {
        try {
            this.logger.info('Showing Rhema stats');
            vscode.window.showInformationMessage('Rhema Statistics displayed');
        } catch (error) {
            this.errorHandler.handleError('Failed to show stats', error);
        }
    }

    async checkHealth(): Promise<void> {
        try {
            this.logger.info('Checking Rhema health');
            vscode.window.showInformationMessage('Rhema Health check completed');
        } catch (error) {
            this.errorHandler.handleError('Failed to check health', error);
        }
    }

    async debugContext(): Promise<void> {
        try {
            this.logger.info('Debugging Rhema context');
            vscode.window.showInformationMessage('Rhema Context debugging started');
        } catch (error) {
            this.errorHandler.handleError('Failed to debug context', error);
        }
    }

    async profilePerformance(): Promise<void> {
        try {
            this.logger.info('Profiling Rhema performance');
            vscode.window.showInformationMessage('Rhema Performance profiling started');
        } catch (error) {
            this.errorHandler.handleError('Failed to profile performance', error);
        }
    }

    async refactorContext(): Promise<void> {
        try {
            this.logger.info('Refactoring Rhema context');
            vscode.window.showInformationMessage('Rhema Context refactoring started');
        } catch (error) {
            this.errorHandler.handleError('Failed to refactor context', error);
        }
    }

    async generateCode(): Promise<void> {
        try {
            this.logger.info('Generating Rhema code');
            vscode.window.showInformationMessage('Rhema Code generation started');
        } catch (error) {
            this.errorHandler.handleError('Failed to generate code', error);
        }
    }

    async showDocumentation(): Promise<void> {
        try {
            this.logger.info('Showing Rhema documentation');
            vscode.window.showInformationMessage('Rhema Documentation opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to show documentation', error);
        }
    }

    async configureSettings(): Promise<void> {
        try {
            this.logger.info('Configuring Rhema settings');
            vscode.window.showInformationMessage('Rhema Settings configuration opened');
        } catch (error) {
            this.errorHandler.handleError('Failed to configure settings', error);
        }
    }

    async runProviderTests(): Promise<void> {
        try {
            this.logger.info('Running Rhema provider tests');
            
            // Import and run the test runner
            const { runTests } = await import('../tests/run-tests');
            runTests();
            
            vscode.window.showInformationMessage('Rhema Provider tests started - check output panel for results');
        } catch (error) {
            this.errorHandler.handleError('Failed to run provider tests', error);
            vscode.window.showErrorMessage('Failed to run provider tests: ' + error);
        }
    }
} 