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

export class RhemaIntelliSense implements vscode.CompletionItemProvider, vscode.HoverProvider, vscode.SignatureHelpProvider {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;
    private completionItems: Map<string, vscode.CompletionItem[]> = new Map();
    private hoverItems: Map<string, vscode.MarkdownString> = new Map();

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
        this.initializeCompletionItems();
        this.initializeHoverItems();
    }

    async initialize(context: vscode.ExtensionContext): Promise<void> {
        try {
            this.logger.info('Initializing Rhema IntelliSense...');

            // Note: Provider registration is now handled in the main extension
            // to avoid duplicate registrations and ensure proper coordination

            this.logger.info('Rhema IntelliSense initialized successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to initialize Rhema IntelliSense', error);
        }
    }

    private initializeCompletionItems(): void {
        // Scope-related completions
        this.completionItems.set('scope', [
            this.createCompletionItem('name', 'string', 'The name of the scope'),
            this.createCompletionItem('description', 'string', 'Description of the scope'),
            this.createCompletionItem('version', 'string', 'Version of the scope'),
            this.createCompletionItem('author', 'string', 'Author of the scope'),
            this.createCompletionItem('created', 'string', 'Creation date'),
            this.createCompletionItem('updated', 'string', 'Last update date'),
            this.createCompletionItem('tags', 'array', 'Tags for categorization'),
            this.createCompletionItem('dependencies', 'array', 'Scope dependencies'),
            this.createCompletionItem('context', 'object', 'Context configuration'),
            this.createCompletionItem('settings', 'object', 'Scope settings')
        ]);

        // Context-related completions
        this.completionItems.set('context', [
            this.createCompletionItem('files', 'array', 'Context files'),
            this.createCompletionItem('patterns', 'array', 'Context patterns'),
            this.createCompletionItem('exclusions', 'array', 'Excluded files/patterns'),
            this.createCompletionItem('maxTokens', 'number', 'Maximum tokens for context'),
            this.createCompletionItem('includeHidden', 'boolean', 'Include hidden files'),
            this.createCompletionItem('recursive', 'boolean', 'Recursive file scanning')
        ]);

        // Todo-related completions
        this.completionItems.set('todo', [
            this.createCompletionItem('id', 'string', 'Unique todo identifier'),
            this.createCompletionItem('title', 'string', 'Todo title'),
            this.createCompletionItem('description', 'string', 'Todo description'),
            this.createCompletionItem('priority', 'string', 'Priority level'),
            this.createCompletionItem('status', 'string', 'Todo status'),
            this.createCompletionItem('assignee', 'string', 'Assigned person'),
            this.createCompletionItem('dueDate', 'string', 'Due date'),
            this.createCompletionItem('tags', 'array', 'Todo tags'),
            this.createCompletionItem('related', 'array', 'Related items')
        ]);

        // Insight-related completions
        this.completionItems.set('insight', [
            this.createCompletionItem('id', 'string', 'Unique insight identifier'),
            this.createCompletionItem('title', 'string', 'Insight title'),
            this.createCompletionItem('description', 'string', 'Insight description'),
            this.createCompletionItem('type', 'string', 'Insight type'),
            this.createCompletionItem('confidence', 'number', 'Confidence level'),
            this.createCompletionItem('source', 'string', 'Insight source'),
            this.createCompletionItem('tags', 'array', 'Insight tags'),
            this.createCompletionItem('related', 'array', 'Related items')
        ]);

        // Pattern-related completions
        this.completionItems.set('pattern', [
            this.createCompletionItem('id', 'string', 'Unique pattern identifier'),
            this.createCompletionItem('name', 'string', 'Pattern name'),
            this.createCompletionItem('description', 'string', 'Pattern description'),
            this.createCompletionItem('type', 'string', 'Pattern type'),
            this.createCompletionItem('regex', 'string', 'Regular expression'),
            this.createCompletionItem('examples', 'array', 'Pattern examples'),
            this.createCompletionItem('tags', 'array', 'Pattern tags')
        ]);

        // Decision-related completions
        this.completionItems.set('decision', [
            this.createCompletionItem('id', 'string', 'Unique decision identifier'),
            this.createCompletionItem('title', 'string', 'Decision title'),
            this.createCompletionItem('description', 'string', 'Decision description'),
            this.createCompletionItem('status', 'string', 'Decision status'),
            this.createCompletionItem('rationale', 'string', 'Decision rationale'),
            this.createCompletionItem('alternatives', 'array', 'Considered alternatives'),
            this.createCompletionItem('impact', 'string', 'Expected impact'),
            this.createCompletionItem('date', 'string', 'Decision date'),
            this.createCompletionItem('reviewDate', 'string', 'Review date')
        ]);
    }

    private initializeHoverItems(): void {
        // Scope hover information
        this.hoverItems.set('scope', new vscode.MarkdownString(`
# Rhema Scope

A scope defines a bounded context for AI agents to work within. It contains all the necessary information for understanding and working with a specific domain or project.

## Properties:
- **name**: The unique identifier for the scope
- **description**: Human-readable description of what the scope contains
- **version**: Semantic version of the scope
- **author**: Who created or maintains this scope
- **context**: Configuration for how context is gathered and processed
- **settings**: Scope-specific configuration options
        `));

        // Context hover information
        this.hoverItems.set('context', new vscode.MarkdownString(`
# Rhema Context

Context defines how files and information are gathered and processed for AI agents.

## Properties:
- **files**: Array of file patterns to include
- **patterns**: Array of content patterns to match
- **exclusions**: Array of patterns to exclude
- **maxTokens**: Maximum number of tokens to include
- **includeHidden**: Whether to include hidden files
- **recursive**: Whether to scan subdirectories recursively
        `));

        // Todo hover information
        this.hoverItems.set('todo', new vscode.MarkdownString(`
# Rhema Todo

A todo item represents a task or action item within the scope.

## Properties:
- **id**: Unique identifier for the todo
- **title**: Short, descriptive title
- **description**: Detailed description of the task
- **priority**: Priority level (high, medium, low)
- **status**: Current status (pending, in-progress, completed, cancelled)
- **assignee**: Person responsible for the todo
- **dueDate**: When the todo should be completed
        `));
    }

    private createCompletionItem(label: string, kind: string, detail: string): vscode.CompletionItem {
        const item = new vscode.CompletionItem(label, this.getCompletionItemKind(kind));
        item.detail = detail;
        item.documentation = new vscode.MarkdownString(detail);
        return item;
    }

    private getCompletionItemKind(kind: string): vscode.CompletionItemKind {
        switch (kind) {
            case 'string': return vscode.CompletionItemKind.Text;
            case 'number': return vscode.CompletionItemKind.Value;
            case 'boolean': return vscode.CompletionItemKind.Value;
            case 'array': return vscode.CompletionItemKind.Array;
            case 'object': return vscode.CompletionItemKind.Class;
            default: return vscode.CompletionItemKind.Text;
        }
    }

    // CompletionItemProvider implementation
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList<vscode.CompletionItem>> {
        try {
            const line = document.lineAt(position.line).text;
            const wordRange = document.getWordRangeAtPosition(position);
            const word = wordRange ? document.getText(wordRange) : '';

            // Context-aware completions based on current position
            const completions: vscode.CompletionItem[] = [];

            // Add scope-related completions
            if (line.includes('scope:') || line.includes('Scope:')) {
                completions.push(...this.completionItems.get('scope') || []);
            }

            // Add context-related completions
            if (line.includes('context:') || line.includes('Context:')) {
                completions.push(...this.completionItems.get('context') || []);
            }

            // Add todo-related completions
            if (line.includes('todo:') || line.includes('Todo:')) {
                completions.push(...this.completionItems.get('todo') || []);
            }

            // Add insight-related completions
            if (line.includes('insight:') || line.includes('Insight:')) {
                completions.push(...this.completionItems.get('insight') || []);
            }

            // Add pattern-related completions
            if (line.includes('pattern:') || line.includes('Pattern:')) {
                completions.push(...this.completionItems.get('pattern') || []);
            }

            // Add decision-related completions
            if (line.includes('decision:') || line.includes('Decision:')) {
                completions.push(...this.completionItems.get('decision') || []);
            }

            return completions;
        } catch (error) {
            this.errorHandler.handleError('Error providing completion items', error);
            return [];
        }
    }

    // HoverProvider implementation
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        try {
            const wordRange = document.getWordRangeAtPosition(position);
            if (!wordRange) return null;

            const word = document.getText(wordRange).toLowerCase();
            const hoverItem = this.hoverItems.get(word);

            if (hoverItem) {
                return new vscode.Hover(hoverItem);
            }

            return null;
        } catch (error) {
            this.errorHandler.handleError('Error providing hover information', error);
            return null;
        }
    }

    // SignatureHelpProvider implementation
    provideSignatureHelp(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.SignatureHelpContext
    ): vscode.ProviderResult<vscode.SignatureHelp> {
        // Basic signature help for Rhema functions
        const signatureHelp = new vscode.SignatureHelp();
        signatureHelp.activeSignature = 0;
        signatureHelp.activeParameter = 0;

        const signature = new vscode.SignatureInformation(
            'rhema_function(param1: string, param2?: number)',
            new vscode.MarkdownString('Rhema function with optional parameters')
        );

        signature.parameters = [
            new vscode.ParameterInformation('param1', 'Required string parameter'),
            new vscode.ParameterInformation('param2', 'Optional number parameter')
        ];

        signatureHelp.signatures = [signature];
        return signatureHelp;
    }

    async dispose(): Promise<void> {
        this.completionItems.clear();
        this.hoverItems.clear();
    }
} 