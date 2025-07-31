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
import { RhemaProvider } from './providers/rhemaProvider';
import { RhemaIntelliSense } from './providers/intelliSense';
import { RhemaValidation } from './providers/validation';
import { RhemaCommands } from './commands';
import { RhemaViews } from './views';
import { RhemaLanguageServer } from './languageServer';
import { RhemaDebugger } from './debugger';
import { RhemaProfiler } from './profiler';
import { RhemaRefactoring } from './refactoring';
import { RhemaCodeGeneration } from './codeGeneration';
import { RhemaDocumentation } from './documentation';
import { RhemaSettings } from './settings';
import { RhemaGitIntegration } from './gitIntegration';
import { RhemaContextExplorer } from './contextExplorer';
import { RhemaPerformanceMonitor } from './performanceMonitor';
import { RhemaErrorHandler } from './errorHandler';
import { RhemaLogger } from './logger';

export class RhemaExtension {
    private context: vscode.ExtensionContext;
    private provider: RhemaProvider;
    private intelliSense: RhemaIntelliSense;
    private validation: RhemaValidation;
    private commands: RhemaCommands;
    private views: RhemaViews;
    private languageServer: RhemaLanguageServer;
    private debugger: RhemaDebugger;
    private profiler: RhemaProfiler;
    private refactoring: RhemaRefactoring;
    private codeGeneration: RhemaCodeGeneration;
    private documentation: RhemaDocumentation;
    private settings: RhemaSettings;
    private gitIntegration: RhemaGitIntegration;
    private contextExplorer: RhemaContextExplorer;
    private performanceMonitor: RhemaPerformanceMonitor;
    private errorHandler: RhemaErrorHandler;
    private logger: RhemaLogger;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.logger = new RhemaLogger();
        this.errorHandler = new RhemaErrorHandler(this.logger);
        this.settings = new RhemaSettings();
        this.provider = new RhemaProvider();
        this.intelliSense = new RhemaIntelliSense();
        this.validation = new RhemaValidation();
        this.commands = new RhemaCommands();
        this.views = new RhemaViews();
        this.languageServer = new RhemaLanguageServer();
        this.debugger = new RhemaDebugger();
        this.profiler = new RhemaProfiler();
        this.refactoring = new RhemaRefactoring();
        this.codeGeneration = new RhemaCodeGeneration();
        this.documentation = new RhemaDocumentation();
        this.gitIntegration = new RhemaGitIntegration();
        this.contextExplorer = new RhemaContextExplorer();
        this.performanceMonitor = new RhemaPerformanceMonitor();
    }

    async activate(): Promise<void> {
        try {
            this.logger.info('Activating Rhema extension...');

            // Initialize settings
            await this.settings.initialize(this.context);

            // Check if Rhema is enabled
            if (!this.settings.isEnabled()) {
                this.logger.info('Rhema extension is disabled');
                return;
            }

            // Initialize core components
            await this.initializeCoreComponents();

            // Register commands
            await this.registerCommands();

            // Register views
            await this.registerViews();

            // Register providers
            await this.registerProviders();

            // Initialize language server
            await this.languageServer.initialize(this.context);

            // Initialize debugger
            await this.debugger.initialize(this.context);

            // Initialize profiler
            await this.profiler.initialize(this.context);

            // Initialize refactoring
            await this.refactoring.initialize(this.context);

            // Initialize code generation
            await this.codeGeneration.initialize(this.context);

            // Initialize documentation
            await this.documentation.initialize(this.context);

            // Initialize Git integration
            await this.gitIntegration.initialize(this.context);

            // Initialize context explorer
            await this.contextExplorer.initialize(this.context);

            // Initialize performance monitor
            await this.performanceMonitor.initialize(this.context);

            // Set up event listeners
            this.setupEventListeners();

            // Start performance monitoring if enabled
            if (this.settings.isPerformanceProfilingEnabled()) {
                await this.performanceMonitor.start();
            }

            this.logger.info('Rhema extension activated successfully');
            this.showWelcomeMessage();

        } catch (error) {
            this.errorHandler.handleError('Failed to activate Rhema extension', error);
        }
    }

    private async initializeCoreComponents(): Promise<void> {
        // Initialize provider
        await this.provider.initialize(this.context);

        // Initialize IntelliSense
        await this.intelliSense.initialize(this.context);

        // Initialize validation
        await this.validation.initialize(this.context);
    }

    private async registerCommands(): Promise<void> {
        // Register all Rhema commands
        const commandRegistrations = [
            vscode.commands.registerCommand('rhema.initialize', () => this.commands.initialize()),
            vscode.commands.registerCommand('rhema.showContext', () => this.commands.showContext()),
            vscode.commands.registerCommand('rhema.executeQuery', () => this.commands.executeQuery()),
            vscode.commands.registerCommand('rhema.searchContext', () => this.commands.searchContext()),
            vscode.commands.registerCommand('rhema.validateFiles', () => this.commands.validateFiles()),
            vscode.commands.registerCommand('rhema.showScopes', () => this.commands.showScopes()),
            vscode.commands.registerCommand('rhema.showTree', () => this.commands.showTree()),
            vscode.commands.registerCommand('rhema.manageTodos', () => this.commands.manageTodos()),
            vscode.commands.registerCommand('rhema.manageInsights', () => this.commands.manageInsights()),
            vscode.commands.registerCommand('rhema.managePatterns', () => this.commands.managePatterns()),
            vscode.commands.registerCommand('rhema.manageDecisions', () => this.commands.manageDecisions()),
            vscode.commands.registerCommand('rhema.showDependencies', () => this.commands.showDependencies()),
            vscode.commands.registerCommand('rhema.showImpact', () => this.commands.showImpact()),
            vscode.commands.registerCommand('rhema.syncKnowledge', () => this.commands.syncKnowledge()),
            vscode.commands.registerCommand('rhema.gitIntegration', () => this.commands.gitIntegration()),
            vscode.commands.registerCommand('rhema.showStats', () => this.commands.showStats()),
            vscode.commands.registerCommand('rhema.checkHealth', () => this.commands.checkHealth()),
            vscode.commands.registerCommand('rhema.debugContext', () => this.commands.debugContext()),
            vscode.commands.registerCommand('rhema.profilePerformance', () => this.commands.profilePerformance()),
            vscode.commands.registerCommand('rhema.refactorContext', () => this.commands.refactorContext()),
            vscode.commands.registerCommand('rhema.generateCode', () => this.commands.generateCode()),
            vscode.commands.registerCommand('rhema.showDocumentation', () => this.commands.showDocumentation()),
            vscode.commands.registerCommand('rhema.configureSettings', () => this.commands.configureSettings()),
            vscode.commands.registerCommand('rhema.runProviderTests', () => this.commands.runProviderTests()),
        ];

        // Store command registrations for disposal
        commandRegistrations.forEach(registration => {
            this.context.subscriptions.push(registration);
        });

        this.logger.info('Rhema commands registered');
    }

    private async registerViews(): Promise<void> {
        // Register Rhema views
        await this.views.initialize(this.context);
        this.logger.info('Rhema views registered');
    }

    private async registerProviders(): Promise<void> {
        // Register IntelliSense providers (completion, hover, signature help)
        const intelliSenseRegistration = vscode.languages.registerCompletionItemProvider(
            { language: 'yaml', scheme: 'file' },
            this.intelliSense,
            '.', ':', '-', ' '
        );
        this.context.subscriptions.push(intelliSenseRegistration);

        // Register hover provider from IntelliSense
        const hoverRegistration = vscode.languages.registerHoverProvider(
            { language: 'yaml', scheme: 'file' },
            this.intelliSense
        );
        this.context.subscriptions.push(hoverRegistration);

        // Register signature help provider from IntelliSense
        const signatureHelpRegistration = vscode.languages.registerSignatureHelpProvider(
            { language: 'yaml', scheme: 'file' },
            this.intelliSense,
            '(', ','
        );
        this.context.subscriptions.push(signatureHelpRegistration);

        // Register validation provider
        const validationRegistration = vscode.languages.registerDiagnosticsCollection('rhema');
        this.context.subscriptions.push(validationRegistration);

        // Register definition provider
        const definitionRegistration = vscode.languages.registerDefinitionProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(definitionRegistration);

        // Register reference provider
        const referenceRegistration = vscode.languages.registerReferenceProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(referenceRegistration);

        // Register symbol provider
        const symbolRegistration = vscode.languages.registerDocumentSymbolProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(symbolRegistration);

        // Register workspace symbol provider
        const workspaceSymbolRegistration = vscode.languages.registerWorkspaceSymbolProvider(
            this.provider
        );
        this.context.subscriptions.push(workspaceSymbolRegistration);

        // Register code actions provider
        const codeActionsRegistration = vscode.languages.registerCodeActionsProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider,
            {
                providedCodeActionKinds: [
                    vscode.CodeActionKind.QuickFix,
                    vscode.CodeActionKind.Refactor,
                    vscode.CodeActionKind.Source
                ]
            }
        );
        this.context.subscriptions.push(codeActionsRegistration);

        // Register folding range provider
        const foldingRangeRegistration = vscode.languages.registerFoldingRangeProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(foldingRangeRegistration);

        // Register selection range provider
        const selectionRangeRegistration = vscode.languages.registerSelectionRangeProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(selectionRangeRegistration);

        // Register document highlight provider
        const documentHighlightRegistration = vscode.languages.registerDocumentHighlightProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(documentHighlightRegistration);

        // Register document link provider
        const documentLinkRegistration = vscode.languages.registerDocumentLinkProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(documentLinkRegistration);

        // Register rename provider
        const renameRegistration = vscode.languages.registerRenameProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider
        );
        this.context.subscriptions.push(renameRegistration);

        // Register format on type provider
        const formatOnTypeRegistration = vscode.languages.registerOnTypeFormattingEditProvider(
            { language: 'yaml', scheme: 'file' },
            this.provider,
            { firstTriggerCharacter: '\n', moreTriggerCharacter: [':', '-'] }
        );
        this.context.subscriptions.push(formatOnTypeRegistration);

        this.logger.info('Rhema providers registered');
    }

    private setupEventListeners(): void {
        // Listen for configuration changes
        vscode.workspace.onDidChangeConfiguration((event: vscode.ConfigurationChangeEvent) => {
            if (event.affectsConfiguration('rhema')) {
                this.settings.reload();
                this.logger.info('Rhema settings reloaded');
            }
        });

        // Listen for document changes
        vscode.workspace.onDidChangeTextDocument((event: vscode.TextDocumentChangeEvent) => {
            if (this.settings.isAutoValidateEnabled() && this.isRhemaFile(event.document)) {
                this.validation.validateDocumentPublic(event.document);
            }
        });

        // Listen for document saves
        vscode.workspace.onDidSaveTextDocument((document: vscode.TextDocument) => {
            if (this.settings.isAutoValidateEnabled() && this.isRhemaFile(document)) {
                this.validation.validateDocumentPublic(document);
            }
        });

        // Listen for workspace folder changes
        vscode.workspace.onDidChangeWorkspaceFolders((event: vscode.WorkspaceFoldersChangeEvent) => {
            this.views.refreshViews();
            this.logger.info('Workspace folders changed, views refreshed');
        });

        // Listen for file system changes
        const fileSystemWatcher = vscode.workspace.createFileSystemWatcher('**/*.{yaml,yml}');
        fileSystemWatcher.onDidChange((uri: vscode.Uri) => {
            if (this.isRhemaFile(uri)) {
                this.views.refreshViews();
                this.logger.info(`Rhema file changed: ${uri.fsPath}`);
            }
        });
        fileSystemWatcher.onDidCreate((uri: vscode.Uri) => {
            if (this.isRhemaFile(uri)) {
                this.views.refreshViews();
                this.logger.info(`Rhema file created: ${uri.fsPath}`);
            }
        });
        fileSystemWatcher.onDidDelete((uri: vscode.Uri) => {
            if (this.isRhemaFile(uri)) {
                this.views.refreshViews();
                this.logger.info(`Rhema file deleted: ${uri.fsPath}`);
            }
        });

        this.context.subscriptions.push(fileSystemWatcher);
        this.logger.info('Rhema event listeners set up');
    }

    private isRhemaFile(document: vscode.TextDocument | vscode.Uri): boolean {
        const uri = document instanceof vscode.Uri ? document : document.uri;
        const fileName = uri.fsPath.toLowerCase();
        return fileName.includes('.rhema.') || 
               fileName.includes('scope.yaml') || 
               fileName.includes('knowledge.yaml') ||
               fileName.includes('todos.yaml') ||
               fileName.includes('decisions.yaml') ||
               fileName.includes('patterns.yaml') ||
               fileName.includes('conventions.yaml');
    }

    private showWelcomeMessage(): void {
        // For now, always show the welcome message
        vscode.window.showInformationMessage(
            'Rhema extension activated! Use Ctrl+Shift+P and search for "RHEMA" to get started.',
            'Show Documentation',
            'Configure Settings'
        ).then((selection: string | undefined) => {
            if (selection === 'Show Documentation') {
                this.commands.showDocumentation();
            } else if (selection === 'Configure Settings') {
                this.commands.configureSettings();
            }
        });
    }

    async deactivate(): Promise<void> {
        try {
            this.logger.info('Deactivating Rhema extension...');

            // Stop performance monitoring
            await this.performanceMonitor.stop();

            // Deactivate language server
            await this.languageServer.deactivate();

            // Deactivate debugger
            await this.debugger.deactivate();

            // Deactivate profiler
            await this.profiler.deactivate();

            // Deactivate refactoring
            await this.refactoring.deactivate();

            // Deactivate code generation
            await this.codeGeneration.deactivate();

            // Deactivate documentation
            await this.documentation.deactivate();

            // Deactivate Git integration
            await this.gitIntegration.deactivate();

            // Deactivate context explorer
            await this.contextExplorer.deactivate();

            this.logger.info('Rhema extension deactivated');
        } catch (error) {
            this.errorHandler.handleError('Failed to deactivate Rhema extension', error);
        }
    }
}

// Extension activation function
export function activate(context: vscode.ExtensionContext): Promise<void> {
    const extension = new RhemaExtension(context);
    return extension.activate();
}

// Extension deactivation function
export function deactivate(): Promise<void> {
    // The extension instance will be garbage collected
    return Promise.resolve();
} 