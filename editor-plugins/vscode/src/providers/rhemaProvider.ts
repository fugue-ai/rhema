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
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';
import { RhemaProvider as GacpProvider } from './gacpProvider';

export class RhemaProvider {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private gacpProvider: GacpProvider;
  private disposables: vscode.Disposable[] = [];

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
    this.gacpProvider = new GacpProvider();
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Rhema provider...');

      // Initialize the GACP provider
      await this.gacpProvider.initialize(context);

      // Set up workspace change listeners for context-aware features
      await this.setupWorkspaceListeners(context);

      this.logger.info('Rhema provider initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema provider', error);
    }
  }

  private async setupWorkspaceListeners(context: vscode.ExtensionContext): Promise<void> {
    try {
      // Listen for workspace folder changes
      const workspaceFolderListener = vscode.workspace.onDidChangeWorkspaceFolders(
        this.onWorkspaceFoldersChanged.bind(this)
      );

      // Listen for file system changes
      const fileSystemListener = vscode.workspace.onDidChangeTextDocument(
        this.onDocumentChanged.bind(this)
      );

      // Listen for configuration changes
      const configListener = vscode.workspace.onDidChangeConfiguration(
        this.onConfigurationChanged.bind(this)
      );

      this.disposables.push(workspaceFolderListener, fileSystemListener, configListener);

      this.logger.info('Workspace listeners setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup workspace listeners', error);
    }
  }

  private async onWorkspaceFoldersChanged(
    event: vscode.WorkspaceFoldersChangeEvent
  ): Promise<void> {
    try {
      this.logger.info('Workspace folders changed, updating context...');

      // Update context when workspace folders change
      if (event.added.length > 0) {
        for (const folder of event.added) {
          await this.analyzeWorkspaceFolder(folder);
        }
      }

      if (event.removed.length > 0) {
        for (const folder of event.removed) {
          await this.cleanupWorkspaceFolder(folder);
        }
      }
    } catch (error) {
      this.errorHandler.handleError('Error handling workspace folder changes', error);
    }
  }

  private async onDocumentChanged(event: vscode.TextDocumentChangeEvent): Promise<void> {
    try {
      // Only process Rhema files
      if (!this.isRhemaFile(event.document)) {
        return;
      }

      // Update context when Rhema files change
      await this.updateContextFromDocument(event.document);
    } catch (error) {
      this.errorHandler.handleError('Error handling document changes', error);
    }
  }

  private async onConfigurationChanged(event: vscode.ConfigurationChangeEvent): Promise<void> {
    try {
      if (event.affectsConfiguration('rhema')) {
        this.logger.info('Rhema configuration changed, updating settings...');
        // Note: settings.refresh() method doesn't exist, so we'll just log the change
        this.logger.info('Configuration change detected for Rhema settings');
      }
    } catch (error) {
      this.errorHandler.handleError('Error handling configuration changes', error);
    }
  }

  private async analyzeWorkspaceFolder(folder: vscode.WorkspaceFolder): Promise<void> {
    try {
      this.logger.info(`Analyzing workspace folder: ${folder.name}`);

      // Find all Rhema files in the workspace
      const rhemaFiles = await vscode.workspace.findFiles(
        new vscode.RelativePattern(folder, '**/*.{yml,yaml}'),
        '**/node_modules/**'
      );

      // Analyze each Rhema file
      for (const file of rhemaFiles) {
        try {
          const document = await vscode.workspace.openTextDocument(file);
          if (this.isRhemaFile(document)) {
            await this.analyzeRhemaFile(document);
          }
        } catch (error) {
          this.logger.warn(`Failed to analyze file: ${file.fsPath}`, error);
        }
      }

      this.logger.info(`Workspace folder analysis completed: ${folder.name}`);
    } catch (error) {
      this.errorHandler.handleError(`Error analyzing workspace folder: ${folder.name}`, error);
    }
  }

  private async cleanupWorkspaceFolder(folder: vscode.WorkspaceFolder): Promise<void> {
    try {
      this.logger.info(`Cleaning up workspace folder: ${folder.name}`);
      // Clean up any cached data for the removed folder
      // This would typically involve clearing caches, removing listeners, etc.
    } catch (error) {
      this.errorHandler.handleError(`Error cleaning up workspace folder: ${folder.name}`, error);
    }
  }

  private async analyzeRhemaFile(document: vscode.TextDocument): Promise<void> {
    try {
      // Analyze the Rhema file for context, symbols, and relationships
      // This information will be used for context-aware completions and validation
      const text = document.getText();

      // Extract scope information
      const scopeMatch = text.match(/scope:\s*\n\s*name:\s*(.+)/);
      if (scopeMatch) {
        const scopeName = scopeMatch[1].trim();
        this.logger.info(`Found Rhema scope: ${scopeName} in ${document.fileName}`);
      }

      // Extract context information
      const contextMatch = text.match(/context:\s*\n\s*files:\s*\[([^\]]+)\]/);
      if (contextMatch) {
        const contextFiles = contextMatch[1].split(',').map((f: string) => f.trim());
        this.logger.info(`Found context files: ${contextFiles.join(', ')} in ${document.fileName}`);
      }
    } catch (error) {
      this.errorHandler.handleError(`Error analyzing Rhema file: ${document.fileName}`, error);
    }
  }

  private async updateContextFromDocument(document: vscode.TextDocument): Promise<void> {
    try {
      // Update context when a Rhema file is modified
      // This ensures that context-aware features stay up to date
      await this.analyzeRhemaFile(document);
    } catch (error) {
      this.errorHandler.handleError(
        `Error updating context from document: ${document.fileName}`,
        error
      );
    }
  }

  private isRhemaFile(document: vscode.TextDocument): boolean {
    // Check if the document is a Rhema file
    // This could be based on file extension, content, or location
    const fileName = document.fileName.toLowerCase();
    const text = document.getText();

    // Check for Rhema-specific content patterns
    const hasRhemaContent =
      text.includes('scope:') ||
      text.includes('context:') ||
      text.includes('todos:') ||
      text.includes('insights:') ||
      text.includes('patterns:') ||
      text.includes('decisions:');

    // Check for Rhema file naming patterns
    const hasRhemaName =
      fileName.includes('rhema') || fileName.includes('scope') || fileName.includes('context');

    return hasRhemaContent || hasRhemaName;
  }

  // Public methods for other components to use
  async getWorkspaceContext(): Promise<any> {
    try {
      // Return the current workspace context
      // This would include information about all Rhema files, their relationships, etc.
      const context: { [key: string]: any[] } = {
        scopes: [],
        contexts: [],
        todos: [],
        insights: [],
        patterns: [],
        decisions: [],
      };

      // Collect context from all workspace folders
      for (const folder of vscode.workspace.workspaceFolders || []) {
        const folderContext = await this.getFolderContext(folder);
        // Merge folder context into workspace context
        Object.keys(folderContext).forEach((key) => {
          if (context[key] && Array.isArray(folderContext[key])) {
            context[key].push(...folderContext[key]);
          }
        });
      }

      return context;
    } catch (error) {
      this.errorHandler.handleError('Error getting workspace context', error);
      return {};
    }
  }

  private async getFolderContext(folder: vscode.WorkspaceFolder): Promise<any> {
    try {
      // Get context for a specific folder
      const context: { [key: string]: any[] } = {
        scopes: [],
        contexts: [],
        todos: [],
        insights: [],
        patterns: [],
        decisions: [],
      };

      // Find and analyze Rhema files in the folder
      const rhemaFiles = await vscode.workspace.findFiles(
        new vscode.RelativePattern(folder, '**/*.{yml,yaml}'),
        '**/node_modules/**'
      );

      for (const file of rhemaFiles) {
        try {
          const document = await vscode.workspace.openTextDocument(file);
          if (this.isRhemaFile(document)) {
            const fileContext = await this.extractFileContext(document);
            // Merge file context into folder context
            Object.keys(fileContext).forEach((key) => {
              if (fileContext[key] && context[key]) {
                context[key].push(fileContext[key]);
              }
            });
          }
        } catch (error) {
          this.logger.warn(`Failed to get context from file: ${file.fsPath}`, error);
        }
      }

      return context;
    } catch (error) {
      this.errorHandler.handleError(`Error getting folder context: ${folder.name}`, error);
      return {};
    }
  }

  private async extractFileContext(document: vscode.TextDocument): Promise<any> {
    try {
      const text = document.getText();
      const context: any = {};

      // Extract scope information
      const scopeMatch = text.match(/scope:\s*\n\s*name:\s*(.+)/);
      if (scopeMatch) {
        context.scope = {
          name: scopeMatch[1].trim(),
          file: document.fileName,
        };
      }

      // Extract context information
      const contextMatch = text.match(/context:\s*\n\s*files:\s*\[([^\]]+)\]/);
      if (contextMatch) {
        context.context = {
          files: contextMatch[1].split(',').map((f) => f.trim()),
          file: document.fileName,
        };
      }

      // Extract other sections as needed
      // This is a simplified extraction - in practice, you'd want more robust parsing

      return context;
    } catch (error) {
      this.errorHandler.handleError(`Error extracting file context: ${document.fileName}`, error);
      return {};
    }
  }

  // Delegate to GACP provider for language service features
  getGacpProvider(): GacpProvider {
    return this.gacpProvider;
  }

  // Provider methods required by VS Code extension
  async provideDefinition(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken
  ): Promise<vscode.Definition | undefined> {
    try {
      const result = await this.gacpProvider.provideDefinition(document, position, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing definition', error);
      return undefined;
    }
  }

  async provideReferences(
    document: vscode.TextDocument,
    position: vscode.Position,
    context: vscode.ReferenceContext,
    token: vscode.CancellationToken
  ): Promise<vscode.Location[] | undefined> {
    try {
      return await this.gacpProvider.provideReferences(document, position, context, token);
    } catch (error) {
      this.errorHandler.handleError('Error providing references', error);
      return undefined;
    }
  }

  async provideDocumentSymbols(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): Promise<vscode.SymbolInformation[] | undefined> {
    try {
      const result = await this.gacpProvider.provideDocumentSymbols(document, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing document symbols', error);
      return undefined;
    }
  }

  async provideWorkspaceSymbols(
    query: string,
    token: vscode.CancellationToken
  ): Promise<vscode.SymbolInformation[] | undefined> {
    try {
      const result = await this.gacpProvider.provideWorkspaceSymbols(query, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing workspace symbols', error);
      return undefined;
    }
  }

  async provideCodeActions(
    document: vscode.TextDocument,
    range: vscode.Range | vscode.Selection,
    context: vscode.CodeActionContext,
    token: vscode.CancellationToken
  ): Promise<vscode.CodeAction[] | undefined> {
    try {
      const result = await this.gacpProvider.provideCodeActions(document, range, context, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing code actions', error);
      return undefined;
    }
  }

  async provideFoldingRanges(
    document: vscode.TextDocument,
    context: vscode.FoldingContext,
    token: vscode.CancellationToken
  ): Promise<vscode.FoldingRange[] | undefined> {
    try {
      const result = await this.gacpProvider.provideFoldingRanges(document, context, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing folding ranges', error);
      return undefined;
    }
  }

  async provideSelectionRanges(
    document: vscode.TextDocument,
    positions: readonly vscode.Position[],
    token: vscode.CancellationToken
  ): Promise<vscode.SelectionRange[] | undefined> {
    try {
      const result = await this.gacpProvider.provideSelectionRanges(document, positions, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing selection ranges', error);
      return undefined;
    }
  }

  async provideDocumentHighlights(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken
  ): Promise<vscode.DocumentHighlight[] | undefined> {
    try {
      const result = await this.gacpProvider.provideDocumentHighlights(document, position, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing document highlights', error);
      return undefined;
    }
  }

  async provideDocumentLinks(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): Promise<vscode.DocumentLink[] | undefined> {
    try {
      const result = await this.gacpProvider.provideDocumentLinks(document, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing document links', error);
      return undefined;
    }
  }

  async provideRenameEdits(
    document: vscode.TextDocument,
    position: vscode.Position,
    newName: string,
    token: vscode.CancellationToken
  ): Promise<vscode.WorkspaceEdit | undefined> {
    try {
      const result = await this.gacpProvider.provideRenameEdits(document, position, newName, token);
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing rename edits', error);
      return undefined;
    }
  }

  async provideOnTypeFormattingEdits(
    document: vscode.TextDocument,
    position: vscode.Position,
    ch: string,
    options: vscode.FormattingOptions,
    token: vscode.CancellationToken
  ): Promise<vscode.TextEdit[] | undefined> {
    try {
      const result = await this.gacpProvider.provideOnTypeFormattingEdits(
        document,
        position,
        ch,
        options,
        token
      );
      return result || undefined;
    } catch (error) {
      this.errorHandler.handleError('Error providing on-type formatting edits', error);
      return undefined;
    }
  }

  async dispose(): Promise<void> {
    try {
      this.logger.info('Disposing Rhema provider...');

      // Dispose of the GACP provider
      await this.gacpProvider.dispose();

      // Dispose of all disposables
      for (const disposable of this.disposables) {
        disposable.dispose();
      }

      this.logger.info('Rhema provider disposed successfully');
    } catch (error) {
      this.errorHandler.handleError('Error disposing Rhema provider', error);
    }
  }
}
