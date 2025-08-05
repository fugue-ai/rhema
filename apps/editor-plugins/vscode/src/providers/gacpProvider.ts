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
import * as yaml from 'yaml';
import * as path from 'path';
import * as fs from 'fs';
import { RhemaLogger } from '../logger';
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';

export class RhemaProvider
  implements
    vscode.DefinitionProvider,
    vscode.ReferenceProvider,
    vscode.DocumentSymbolProvider,
    vscode.WorkspaceSymbolProvider,
    vscode.CodeActionProvider,
    vscode.FoldingRangeProvider,
    vscode.SelectionRangeProvider,
    vscode.DocumentHighlightProvider,
    vscode.DocumentLinkProvider,
    vscode.RenameProvider,
    vscode.OnTypeFormattingEditProvider
{
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private statusBarItem: vscode.StatusBarItem;
  private outputChannel: vscode.OutputChannel;
  private symbolCache: Map<string, any> = new Map();

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
    this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.outputChannel = vscode.window.createOutputChannel('RHEMA');
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Rhema provider...');

      // Initialize status bar
      this.statusBarItem.text = '$(git-branch) RHEMA';
      this.statusBarItem.tooltip = 'Rhema - Git-Based Agent Context Protocol';
      this.statusBarItem.command = 'rhema.showContext';
      this.statusBarItem.show();

      // Register disposables
      context.subscriptions.push(this.statusBarItem, this.outputChannel);

      // Check Rhema installation
      await this.checkRhemaInstallation();

      this.logger.info('Rhema provider initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema provider', error);
    }
  }

  private async checkRhemaInstallation(): Promise<void> {
    try {
      const executablePath = this.settings.getExecutablePath();
      const { exec } = require('child_process');
      const { promisify } = require('util');
      const execAsync = promisify(exec);

      await execAsync(`${executablePath} --version`);
      this.statusBarItem.text = '$(git-branch) RHEMA';
      this.outputChannel.appendLine('Rhema CLI found and working');
    } catch (error) {
      this.statusBarItem.text = '$(error) RHEMA';
      this.statusBarItem.tooltip = 'Rhema CLI not found. Please install Rhema CLI.';
      this.outputChannel.appendLine('Rhema CLI not found. Please install Rhema CLI.');
      vscode.window.showWarningMessage(
        'Rhema CLI not found. Please install Rhema CLI to use this extension.'
      );
    }
  }

  async executeRhemaCommand(command: string, args: string[] = []): Promise<string> {
    try {
      const executablePath = this.settings.getExecutablePath();
      const { exec } = require('child_process');
      const { promisify } = require('util');
      const execAsync = promisify(exec);

      const fullCommand = `${executablePath} ${command} ${args.join(' ')}`;
      this.outputChannel.appendLine(`Executing: ${fullCommand}`);

      const { stdout, stderr } = await execAsync(fullCommand, {
        cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath,
      });

      if (stderr) {
        this.outputChannel.appendLine(`Stderr: ${stderr}`);
      }

      this.outputChannel.appendLine(`Output: ${stdout}`);
      return stdout;
    } catch (error) {
      this.errorHandler.handleError(`Failed to execute Rhema command: ${command}`, error);
      throw error;
    }
  }

  getOutputChannel(): vscode.OutputChannel {
    return this.outputChannel;
  }

  updateStatusBar(text: string, tooltip?: string): void {
    this.statusBarItem.text = text;
    if (tooltip) {
      this.statusBarItem.tooltip = tooltip;
    }
  }

  showOutput(): void {
    this.outputChannel.show();
  }

  // Helper methods for parsing and analyzing Rhema files
  private isRhemaFile(document: vscode.TextDocument): boolean {
    const fileName = document.fileName.toLowerCase();
    return (
      fileName.includes('.rhema.') ||
      fileName.includes('scope.yaml') ||
      fileName.includes('knowledge.yaml') ||
      fileName.includes('todos.yaml') ||
      fileName.includes('decisions.yaml') ||
      fileName.includes('patterns.yaml') ||
      fileName.includes('conventions.yaml')
    );
  }

  private async parseRhemaDocument(document: vscode.TextDocument): Promise<any> {
    try {
      const text = document.getText();
      const parsed = yaml.parse(text);
      return parsed;
    } catch (error) {
      this.logger.warn(`Failed to parse Rhema document: ${error}`);
      return null;
    }
  }

  private getWordAtPosition(document: vscode.TextDocument, position: vscode.Position): string {
    const range = document.getWordRangeAtPosition(position);
    return range ? document.getText(range) : '';
  }

  private findSymbolInDocument(
    document: vscode.TextDocument,
    symbolName: string
  ): vscode.Location[] {
    const locations: vscode.Location[] = [];
    const text = document.getText();
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const index = line.indexOf(symbolName);
      if (index !== -1) {
        const start = new vscode.Position(i, index);
        const end = new vscode.Position(i, index + symbolName.length);
        locations.push(new vscode.Location(document.uri, new vscode.Range(start, end)));
      }
    }

    return locations;
  }

  // DefinitionProvider implementation
  provideDefinition(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.Definition | vscode.DefinitionLink[]> {
    if (!this.isRhemaFile(document)) {
      return undefined;
    }

    try {
      const word = this.getWordAtPosition(document, position);
      if (!word) return undefined;

      // Look for definitions in the current document
      const locations = this.findSymbolInDocument(document, word);
      if (locations.length > 0) {
        return locations[0];
      }

      // Look for definitions in workspace
      return this.findDefinitionInWorkspace(word);
    } catch (error) {
      this.errorHandler.handleError('Error in provideDefinition', error);
      return undefined;
    }
  }

  private async findDefinitionInWorkspace(
    symbolName: string
  ): Promise<vscode.Location | undefined> {
    try {
      const files = await vscode.workspace.findFiles('**/*.{yaml,yml}');

      for (const file of files) {
        const document = await vscode.workspace.openTextDocument(file);
        if (this.isRhemaFile(document)) {
          const locations = this.findSymbolInDocument(document, symbolName);
          if (locations.length > 0) {
            return locations[0];
          }
        }
      }
    } catch (error) {
      this.errorHandler.handleError('Error finding definition in workspace', error);
    }

    return undefined;
  }

  // ReferenceProvider implementation
  async provideReferences(
    document: vscode.TextDocument,
    position: vscode.Position,
    context: vscode.ReferenceContext,
    token: vscode.CancellationToken
  ): Promise<vscode.Location[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const word = this.getWordAtPosition(document, position);
      if (!word) return [];

      const locations: vscode.Location[] = [];

      // Find references in current document
      locations.push(...this.findSymbolInDocument(document, word));

      // Find references in workspace
      if (context.includeDeclaration) {
        const workspaceRefs = await this.findReferencesInWorkspace(word);
        locations.push(...workspaceRefs);
      }

      return locations;
    } catch (error) {
      this.errorHandler.handleError('Error in provideReferences', error);
      return [];
    }
  }

  private async findReferencesInWorkspace(symbolName: string): Promise<vscode.Location[]> {
    const locations: vscode.Location[] = [];

    try {
      const files = await vscode.workspace.findFiles('**/*.{yaml,yml}');

      for (const file of files) {
        const document = await vscode.workspace.openTextDocument(file);
        if (this.isRhemaFile(document)) {
          const refs = this.findSymbolInDocument(document, symbolName);
          locations.push(...refs);
        }
      }
    } catch (error) {
      this.errorHandler.handleError('Error finding references in workspace', error);
    }

    return locations;
  }

  // DocumentSymbolProvider implementation
  provideDocumentSymbols(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.SymbolInformation[] | vscode.DocumentSymbol[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const parsed = yaml.parse(document.getText());
      const symbols: vscode.DocumentSymbol[] = [];

      if (parsed.scope) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Scope',
            'Rhema Scope Definition',
            vscode.SymbolKind.Namespace,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      if (parsed.context) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Context',
            'Rhema Context Configuration',
            vscode.SymbolKind.Object,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      if (parsed.todos && Array.isArray(parsed.todos)) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Todos',
            'Rhema Todo Items',
            vscode.SymbolKind.Array,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      if (parsed.insights && Array.isArray(parsed.insights)) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Insights',
            'Rhema Insights',
            vscode.SymbolKind.Array,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      if (parsed.patterns && Array.isArray(parsed.patterns)) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Patterns',
            'Rhema Patterns',
            vscode.SymbolKind.Array,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      if (parsed.decisions && Array.isArray(parsed.decisions)) {
        symbols.push(
          new vscode.DocumentSymbol(
            'Decisions',
            'Rhema Decisions',
            vscode.SymbolKind.Array,
            new vscode.Range(0, 0, document.lineCount - 1, 0),
            new vscode.Range(0, 0, 0, 0)
          )
        );
      }

      return symbols;
    } catch (error) {
      this.errorHandler.handleError('Error in provideDocumentSymbols', error);
      return [];
    }
  }

  // WorkspaceSymbolProvider implementation
  provideWorkspaceSymbols(
    query: string,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.SymbolInformation[]> {
    try {
      const symbols: vscode.SymbolInformation[] = [];

      // This would typically search through all Rhema files in the workspace
      // For now, return a basic implementation
      if (query.toLowerCase().includes('scope')) {
        symbols.push(
          new vscode.SymbolInformation(
            'Scope',
            vscode.SymbolKind.Namespace,
            'Rhema Scope',
            new vscode.Location(vscode.Uri.file('scope.yaml'), new vscode.Position(0, 0))
          )
        );
      }

      if (query.toLowerCase().includes('todo')) {
        symbols.push(
          new vscode.SymbolInformation(
            'Todos',
            vscode.SymbolKind.Array,
            'Rhema Todos',
            new vscode.Location(vscode.Uri.file('todos.yaml'), new vscode.Position(0, 0))
          )
        );
      }

      return symbols;
    } catch (error) {
      this.errorHandler.handleError('Error in provideWorkspaceSymbols', error);
      return [];
    }
  }

  // CodeActionProvider implementation
  provideCodeActions(
    document: vscode.TextDocument,
    range: vscode.Range | vscode.Selection,
    context: vscode.CodeActionContext,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<(vscode.CodeAction | vscode.Command)[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    const actions: vscode.CodeAction[] = [];

    try {
      // Add quick fixes for common Rhema issues
      if (context.diagnostics.length > 0) {
        const fixAction = new vscode.CodeAction(
          'Fix Rhema Validation Issues',
          vscode.CodeActionKind.QuickFix
        );
        fixAction.command = {
          command: 'rhema.validateFiles',
          title: 'Fix Rhema Validation Issues',
        };
        actions.push(fixAction);
      }

      // Add refactoring actions
      const refactorAction = new vscode.CodeAction(
        'Refactor Rhema Context',
        vscode.CodeActionKind.Refactor
      );
      refactorAction.command = {
        command: 'rhema.refactorContext',
        title: 'Refactor Rhema Context',
      };
      actions.push(refactorAction);

      // Add source actions
      const sourceAction = new vscode.CodeAction(
        'Generate Rhema Documentation',
        vscode.CodeActionKind.Source
      );
      sourceAction.command = {
        command: 'rhema.generateDocumentation',
        title: 'Generate Rhema Documentation',
      };
      actions.push(sourceAction);

      return actions;
    } catch (error) {
      this.errorHandler.handleError('Error in provideCodeActions', error);
      return [];
    }
  }

  // FoldingRangeProvider implementation
  provideFoldingRanges(
    document: vscode.TextDocument,
    context: vscode.FoldingContext,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.FoldingRange[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const ranges: vscode.FoldingRange[] = [];
      const lines = document.getText().split('\n');
      let startLine = -1;

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i].trim();

        // Start folding for major sections
        if (
          line === 'scope:' ||
          line === 'context:' ||
          line === 'todos:' ||
          line === 'insights:' ||
          line === 'patterns:' ||
          line === 'decisions:'
        ) {
          if (startLine !== -1) {
            ranges.push(new vscode.FoldingRange(startLine, i - 1));
          }
          startLine = i;
        }

        // End folding for empty lines or new sections
        if (line === '' && startLine !== -1) {
          ranges.push(new vscode.FoldingRange(startLine, i - 1));
          startLine = -1;
        }
      }

      // Handle the last section
      if (startLine !== -1) {
        ranges.push(new vscode.FoldingRange(startLine, lines.length - 1));
      }

      return ranges;
    } catch (error) {
      this.errorHandler.handleError('Error in provideFoldingRanges', error);
      return [];
    }
  }

  // SelectionRangeProvider implementation
  provideSelectionRanges(
    document: vscode.TextDocument,
    positions: vscode.Position[],
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.SelectionRange[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      return positions.map((position) => {
        const wordRange = document.getWordRangeAtPosition(position);
        if (wordRange) {
          const lineRange = new vscode.Range(
            new vscode.Position(position.line, 0),
            new vscode.Position(position.line, document.lineAt(position.line).text.length)
          );

          return new vscode.SelectionRange(wordRange, new vscode.SelectionRange(lineRange));
        }
        return new vscode.SelectionRange(new vscode.Range(position, position));
      });
    } catch (error) {
      this.errorHandler.handleError('Error in provideSelectionRanges', error);
      return [];
    }
  }

  // DocumentHighlightProvider implementation
  provideDocumentHighlights(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.DocumentHighlight[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const word = this.getWordAtPosition(document, position);
      if (!word) return [];

      const locations = this.findSymbolInDocument(document, word);
      return locations.map((location) => new vscode.DocumentHighlight(location.range));
    } catch (error) {
      this.errorHandler.handleError('Error in provideDocumentHighlights', error);
      return [];
    }
  }

  // DocumentLinkProvider implementation
  provideDocumentLinks(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.DocumentLink[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const links: vscode.DocumentLink[] = [];
      const text = document.getText();
      const lines = text.split('\n');

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Find file references
        const fileMatch = line.match(/(file|path):\s*(.+\.(yaml|yml|md|txt))/);
        if (fileMatch) {
          const start = line.indexOf(fileMatch[2]);
          const end = start + fileMatch[2].length;
          const range = new vscode.Range(i, start, i, end);

          const link = new vscode.DocumentLink(range);
          link.target = vscode.Uri.file(
            path.resolve(path.dirname(document.fileName), fileMatch[2])
          );
          links.push(link);
        }

        // Find URL references
        const urlMatch = line.match(/(https?:\/\/[^\s]+)/);
        if (urlMatch) {
          const start = line.indexOf(urlMatch[1]);
          const end = start + urlMatch[1].length;
          const range = new vscode.Range(i, start, i, end);

          const link = new vscode.DocumentLink(range);
          link.target = vscode.Uri.parse(urlMatch[1]);
          links.push(link);
        }
      }

      return links;
    } catch (error) {
      this.errorHandler.handleError('Error in provideDocumentLinks', error);
      return [];
    }
  }

  // RenameProvider implementation
  provideRenameEdits(
    document: vscode.TextDocument,
    position: vscode.Position,
    newName: string,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.WorkspaceEdit> {
    if (!this.isRhemaFile(document)) {
      return undefined;
    }

    try {
      const word = this.getWordAtPosition(document, position);
      if (!word) return undefined;

      const edit = new vscode.WorkspaceEdit();
      const locations = this.findSymbolInDocument(document, word);

      locations.forEach((location) => {
        edit.replace(location.uri, location.range, newName);
      });

      return edit;
    } catch (error) {
      this.errorHandler.handleError('Error in provideRenameEdits', error);
      return undefined;
    }
  }

  // OnTypeFormattingEditProvider implementation
  provideOnTypeFormattingEdits(
    document: vscode.TextDocument,
    position: vscode.Position,
    ch: string,
    options: vscode.FormattingOptions,
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.TextEdit[]> {
    if (!this.isRhemaFile(document)) {
      return [];
    }

    try {
      const edits: vscode.TextEdit[] = [];
      const line = document.lineAt(position.line);
      const lineText = line.text;

      // Auto-indent for YAML structures
      if (ch === '\n') {
        const prevLine = position.line > 0 ? document.lineAt(position.line - 1) : null;
        if (prevLine) {
          const prevText = prevLine.text;
          const match = prevText.match(/^(\s*)([a-zA-Z_][a-zA-Z0-9_]*):\s*$/);
          if (match) {
            const indent = match[1] + '  '; // Add 2 spaces for nested items
            const newLine = indent + '- '; // Start with a list item
            edits.push(vscode.TextEdit.insert(position, newLine));
          }
        }
      }

      // Auto-complete colons for keys
      if (ch === ':') {
        const wordRange = document.getWordRangeAtPosition(
          new vscode.Position(position.line, position.character - 1)
        );
        if (wordRange) {
          const word = document.getText(wordRange);
          if (word && !lineText.includes(':')) {
            edits.push(vscode.TextEdit.insert(position, ' '));
          }
        }
      }

      return edits;
    } catch (error) {
      this.errorHandler.handleError('Error in provideOnTypeFormattingEdits', error);
      return [];
    }
  }

  async dispose(): Promise<void> {
    this.statusBarItem.dispose();
    this.outputChannel.dispose();
    this.symbolCache.clear();
  }
}
