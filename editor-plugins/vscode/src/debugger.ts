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

export class RhemaDebugger {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;
    private debugOutputChannel: vscode.OutputChannel;
    private disposables: vscode.Disposable[] = [];

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
        this.debugOutputChannel = vscode.window.createOutputChannel('Rhema Debug');
    }

    async initialize(context: vscode.ExtensionContext): Promise<void> {
        try {
            this.logger.info('Initializing Rhema debugger...');

            // Register debug configuration provider
            const debugProvider = vscode.debug.registerDebugConfigurationProvider('rhema', {
                provideDebugConfigurations: this.provideDebugConfigurations.bind(this),
                resolveDebugConfiguration: this.resolveDebugConfiguration.bind(this)
            });

            // Register debug adapter descriptor factory
            const debugAdapterFactory = vscode.debug.registerDebugAdapterDescriptorFactory('rhema', {
                createDebugAdapterDescriptor: this.createDebugAdapterDescriptor.bind(this)
            });

            // Add disposables
            this.disposables.push(
                debugProvider,
                debugAdapterFactory,
                this.debugOutputChannel
            );

            this.logger.info('Rhema debugger initialized successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to initialize Rhema debugger', error);
        }
    }

    private provideDebugConfigurations(): vscode.DebugConfiguration[] {
        return [
            {
                name: 'Debug Rhema Context',
                type: 'rhema',
                request: 'launch',
                program: '${workspaceFolder}/.rhema/context.yaml',
                args: [],
                env: {},
                console: 'integratedTerminal'
            },
            {
                name: 'Debug Rhema Query',
                type: 'rhema',
                request: 'launch',
                program: '${workspaceFolder}/.rhema/query.yaml',
                args: [],
                env: {},
                console: 'integratedTerminal'
            }
        ];
    }

    private resolveDebugConfiguration(
        folder: vscode.WorkspaceFolder | undefined,
        config: vscode.DebugConfiguration
    ): vscode.DebugConfiguration | undefined {
        if (!config.type && !config.request && !config.name) {
            const activeEditor = vscode.window.activeTextEditor;
            if (activeEditor && this.isRhemaFile(activeEditor.document)) {
                config.type = 'rhema';
                config.name = 'Debug Rhema File';
                config.request = 'launch';
                config.program = activeEditor.document.uri.fsPath;
            }
        }

        return config;
    }

    private createDebugAdapterDescriptor(
        session: vscode.DebugSession,
        executable: vscode.DebugAdapterExecutable | undefined
    ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
        // Create a debug adapter executable for Rhema
        const executablePath = this.settings.getExecutablePath();
        const args = ['debug', '--debug-mode'];
        
        return new vscode.DebugAdapterExecutable(executablePath, args);
    }

    private isRhemaFile(document: vscode.TextDocument): boolean {
        return document.languageId === 'yaml' || 
               document.languageId === 'rhema-yaml' ||
               document.fileName.endsWith('.rhema.yaml') ||
               document.fileName.endsWith('.rhema.yml');
    }

    async debugContext(contextFile: string): Promise<void> {
        try {
            this.debugOutputChannel.appendLine(`Starting debug session for context: ${contextFile}`);
            
            // Start debug session
            const debugConfig: vscode.DebugConfiguration = {
                name: 'Debug Rhema Context',
                type: 'rhema',
                request: 'launch',
                program: contextFile,
                args: ['--debug-mode'],
                env: {},
                console: 'integratedTerminal'
            };

            await vscode.debug.startDebugging(undefined, debugConfig);
            
            this.debugOutputChannel.appendLine('Debug session started successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to start debug session', error);
        }
    }

    async debugQuery(queryFile: string): Promise<void> {
        try {
            this.debugOutputChannel.appendLine(`Starting debug session for query: ${queryFile}`);
            
            // Start debug session
            const debugConfig: vscode.DebugConfiguration = {
                name: 'Debug Rhema Query',
                type: 'rhema',
                request: 'launch',
                program: queryFile,
                args: ['--debug-mode'],
                env: {},
                console: 'integratedTerminal'
            };

            await vscode.debug.startDebugging(undefined, debugConfig);
            
            this.debugOutputChannel.appendLine('Debug session started successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to start debug session', error);
        }
    }

    showDebugOutput(): void {
        this.debugOutputChannel.show();
    }

    logDebug(message: string, ...args: any[]): void {
        this.debugOutputChannel.appendLine(`[DEBUG] ${message}`);
        if (args.length > 0) {
            args.forEach(arg => {
                if (typeof arg === 'object') {
                    this.debugOutputChannel.appendLine(JSON.stringify(arg, null, 2));
                } else {
                    this.debugOutputChannel.appendLine(String(arg));
                }
            });
        }
    }

    async dispose(): Promise<void> {
        this.disposables.forEach(disposable => disposable.dispose());
        this.disposables = [];
    }

    async deactivate(): Promise<void> {
        this.dispose();
    }
} 