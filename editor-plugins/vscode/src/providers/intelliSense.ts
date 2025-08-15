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
import { CompletionWorkerService } from '../services/completionWorkerService';
import { CompletionQueueManager } from '../services/completionQueueManager';

export class RhemaIntelliSense
  implements vscode.CompletionItemProvider, vscode.HoverProvider, vscode.SignatureHelpProvider
{
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private completionItems: Map<string, vscode.CompletionItem[]> = new Map();
  private hoverItems: Map<string, vscode.MarkdownString> = new Map();
  private workspaceContext: any = {};
  private aiCompletionCache: Map<string, vscode.CompletionItem[]> = new Map();
  private contextCache: Map<string, any> = new Map();
  private completionWorkerService: CompletionWorkerService | null = null;
  private completionQueueManager: CompletionQueueManager | null = null;
  private asyncCompletionsEnabled = true;

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

      // Initialize async completion system
      await this.initializeAsyncCompletionSystem(context);

      // Note: Provider registration is now handled in the main extension
      // to avoid duplicate registrations and ensure proper coordination

      this.logger.info('Rhema IntelliSense initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema IntelliSense', error);
    }
  }

  private async initializeAsyncCompletionSystem(context: vscode.ExtensionContext): Promise<void> {
    try {
      // Check if async completions are enabled
      this.asyncCompletionsEnabled = this.settings.isEnabled() && this.settings.isAsyncCompletionsEnabled();

      if (!this.asyncCompletionsEnabled) {
        this.logger.info('Async completions disabled, using synchronous mode');
        return;
      }

      this.logger.info('Initializing async completion system...');

      // Initialize completion worker service
      this.completionWorkerService = new CompletionWorkerService();
      await this.completionWorkerService.initialize(context);

      // Initialize completion queue manager
      this.completionQueueManager = new CompletionQueueManager(this.completionWorkerService);
      await this.completionQueueManager.initialize();

      // Configure settings
      this.completionQueueManager.setMaxConcurrentRequests(this.settings.getMaxConcurrentCompletions());
      this.completionWorkerService.setCompletionTimeout(this.settings.getCompletionTimeout());

      this.logger.info('Async completion system initialized successfully');
    } catch (error) {
      this.logger.warn('Failed to initialize async completion system, falling back to synchronous mode');
      this.asyncCompletionsEnabled = false;
      this.errorHandler.handleError('Failed to initialize async completion system', error);
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
      this.createCompletionItem('settings', 'object', 'Scope settings'),
    ]);

    // Context-related completions
    this.completionItems.set('context', [
      this.createCompletionItem('files', 'array', 'Context files'),
      this.createCompletionItem('patterns', 'array', 'Context patterns'),
      this.createCompletionItem('exclusions', 'array', 'Excluded files/patterns'),
      this.createCompletionItem('maxTokens', 'number', 'Maximum tokens for context'),
      this.createCompletionItem('includeHidden', 'boolean', 'Include hidden files'),
      this.createCompletionItem('recursive', 'boolean', 'Recursive file scanning'),
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
      this.createCompletionItem('related', 'array', 'Related items'),
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
      this.createCompletionItem('related', 'array', 'Related items'),
    ]);

    // Pattern-related completions
    this.completionItems.set('pattern', [
      this.createCompletionItem('id', 'string', 'Unique pattern identifier'),
      this.createCompletionItem('name', 'string', 'Pattern name'),
      this.createCompletionItem('description', 'string', 'Pattern description'),
      this.createCompletionItem('type', 'string', 'Pattern type'),
      this.createCompletionItem('regex', 'string', 'Regular expression'),
      this.createCompletionItem('examples', 'array', 'Pattern examples'),
      this.createCompletionItem('tags', 'array', 'Pattern tags'),
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
      this.createCompletionItem('reviewDate', 'string', 'Review date'),
    ]);
  }

  private initializeHoverItems(): void {
    // Scope hover information
    this.hoverItems.set(
      'scope',
      new vscode.MarkdownString(`
# Rhema Scope

A scope defines a bounded context for AI agents to work within. It contains all the necessary information for understanding and working with a specific domain or project.

## Properties:
- **name**: The unique identifier for the scope
- **description**: Human-readable description of what the scope contains
- **version**: Semantic version of the scope
- **author**: Who created or maintains this scope
- **context**: Configuration for how context is gathered and processed
- **settings**: Scope-specific configuration options
        `)
    );

    // Context hover information
    this.hoverItems.set(
      'context',
      new vscode.MarkdownString(`
# Rhema Context

Context defines how files and information are gathered and processed for AI agents.

## Properties:
- **files**: Array of file patterns to include
- **patterns**: Array of content patterns to match
- **exclusions**: Array of patterns to exclude
- **maxTokens**: Maximum number of tokens to include
- **includeHidden**: Whether to include hidden files
- **recursive**: Whether to scan subdirectories recursively
        `)
    );

    // Todo hover information
    this.hoverItems.set(
      'todo',
      new vscode.MarkdownString(`
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
        `)
    );
  }

  private createCompletionItem(label: string, kind: string, detail: string): vscode.CompletionItem {
    const item = new vscode.CompletionItem(label, this.getCompletionItemKind(kind));
    item.detail = detail;
    item.documentation = new vscode.MarkdownString(detail);
    return item;
  }

  private getCompletionItemKind(kind: string): vscode.CompletionItemKind {
    switch (kind) {
      case 'string':
        return vscode.CompletionItemKind.Text;
      case 'number':
        return vscode.CompletionItemKind.Value;
      case 'boolean':
        return vscode.CompletionItemKind.Value;
      case 'array':
        return vscode.CompletionItemKind.Field;
      case 'object':
        return vscode.CompletionItemKind.Class;
      default:
        return vscode.CompletionItemKind.Text;
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

      // Get basic completions synchronously for immediate response
      const basicCompletions = this.getBasicCompletions(line);

      // If async completions are enabled and we have the queue manager, use async loading
      if (this.asyncCompletionsEnabled && this.completionQueueManager && !token.isCancellationRequested) {
        return this.provideAsyncCompletions(document, position, line, word, basicCompletions, token);
      }

      // Fallback to synchronous completions
      return this.provideSyncCompletions(line, word, basicCompletions);
    } catch (error) {
      this.errorHandler.handleError('Error providing completion items', error);
      return [];
    }
  }

  private async provideAsyncCompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string,
    basicCompletions: vscode.CompletionItem[],
    token: vscode.CancellationToken
  ): Promise<vscode.CompletionItem[]> {
    try {
      // Return basic completions immediately
      const allCompletions = [...basicCompletions];

      // Queue async completion request
      if (this.completionQueueManager) {
        const asyncCompletions = await this.completionQueueManager.enqueueCompletion(
          document,
          position,
          line,
          word,
          { basicCompletions },
          1 // Priority for async completions
        );

        // Add async completions to the list
        allCompletions.push(...asyncCompletions);
      }

      // Remove duplicates and sort by relevance
      const uniqueCompletions = this.removeDuplicateCompletions(allCompletions);
      const sortedCompletions = this.sortCompletionsByRelevance(uniqueCompletions, word);

      return sortedCompletions;
    } catch (error) {
      this.errorHandler.handleError('Error providing async completions', error);
      // Fallback to basic completions
      return this.provideSyncCompletions(line, word, basicCompletions);
    }
  }

  private provideSyncCompletions(
    line: string,
    word: string,
    basicCompletions: vscode.CompletionItem[]
  ): vscode.CompletionItem[] {
    try {
      const allCompletions = [...basicCompletions];

      // Get context-aware completions synchronously
      const contextCompletions = this.getContextAwareCompletionsSync(line);
      allCompletions.push(...contextCompletions);

      // Remove duplicates and sort by relevance
      const uniqueCompletions = this.removeDuplicateCompletions(allCompletions);
      const sortedCompletions = this.sortCompletionsByRelevance(uniqueCompletions, word);

      return sortedCompletions;
    } catch (error) {
      this.errorHandler.handleError('Error providing sync completions', error);
      return basicCompletions;
    }
  }

  private getContextAwareCompletionsSync(line: string): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Add context-aware completions based on line content
    if (line.includes('files:') || line.includes('Files:')) {
      // Add file-related completions
      completions.push(
        this.createCompletionItem('*.rs', 'file', 'Rust source files'),
        this.createCompletionItem('*.ts', 'file', 'TypeScript files'),
        this.createCompletionItem('*.js', 'file', 'JavaScript files'),
        this.createCompletionItem('*.json', 'file', 'JSON files'),
        this.createCompletionItem('*.yaml', 'file', 'YAML files'),
        this.createCompletionItem('*.md', 'file', 'Markdown files')
      );
    }

    if (line.includes('dependencies:') || line.includes('Dependencies:')) {
      // Add dependency-related completions
      completions.push(
        this.createCompletionItem('rhema-core', 'class', 'Core Rhema functionality'),
        this.createCompletionItem('rhema-config', 'class', 'Configuration management'),
        this.createCompletionItem('rhema-git', 'class', 'Git integration'),
        this.createCompletionItem('rhema-knowledge', 'class', 'Knowledge management')
      );
    }

    return completions;
  }

  private getBasicCompletions(line: string): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Add scope-related completions
    if (line.includes('scope:') || line.includes('Scope:')) {
      completions.push(...(this.completionItems.get('scope') || []));
    }

    // Add context-related completions
    if (line.includes('context:') || line.includes('Context:')) {
      completions.push(...(this.completionItems.get('context') || []));
    }

    // Add todo-related completions
    if (line.includes('todo:') || line.includes('Todo:')) {
      completions.push(...(this.completionItems.get('todo') || []));
    }

    // Add insight-related completions
    if (line.includes('insight:') || line.includes('Insight:')) {
      completions.push(...(this.completionItems.get('insight') || []));
    }

    // Add pattern-related completions
    if (line.includes('pattern:') || line.includes('Pattern:')) {
      completions.push(...(this.completionItems.get('pattern') || []));
    }

    // Add decision-related completions
    if (line.includes('decision:') || line.includes('Decision:')) {
      completions.push(...(this.completionItems.get('decision') || []));
    }

    return completions;
  }

  private async getContextAwareCompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string
  ): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get workspace context
      const workspaceContext = await this.getWorkspaceContext();

      // Get document context
      const documentContext = await this.getDocumentContext(document);

      // Context-aware completions based on workspace state
      if (line.includes('files:') || line.includes('Files:')) {
        const fileCompletions = this.getFileCompletions(workspaceContext, documentContext);
        completions.push(...fileCompletions);
      }

      // Context-aware completions based on existing scopes
      if (line.includes('dependencies:') || line.includes('Dependencies:')) {
        const scopeCompletions = this.getScopeCompletions(workspaceContext);
        completions.push(...scopeCompletions);
      }

      // Context-aware completions based on existing patterns
      if (line.includes('patterns:') || line.includes('Patterns:')) {
        const patternCompletions = this.getPatternCompletions(workspaceContext);
        completions.push(...patternCompletions);
      }

      // Context-aware completions based on existing decisions
      if (line.includes('decisions:') || line.includes('Decisions:')) {
        const decisionCompletions = this.getDecisionCompletions(workspaceContext);
        completions.push(...decisionCompletions);
      }

      return completions;
    } catch (error) {
      this.errorHandler.handleError('Error getting context-aware completions', error);
      return [];
    }
  }

  private async getAICompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string,
    token: vscode.CancellationToken
  ): Promise<vscode.CompletionItem[]> {
    try {
      // Check if AI completions are enabled
      // Note: isAICompletionsEnabled method doesn't exist, so we'll assume it's enabled
      if (!this.settings.isEnabled()) {
        return [];
      }

      // Check cache first
      const cacheKey = `${document.fileName}:${position.line}:${position.character}`;
      if (this.aiCompletionCache.has(cacheKey)) {
        return this.aiCompletionCache.get(cacheKey) || [];
      }

      // Generate AI-powered completions
      const aiCompletions = await this.generateAICompletions(document, position, line, word, token);

      // Cache the results
      this.aiCompletionCache.set(cacheKey, aiCompletions);

      return aiCompletions;
    } catch (error) {
      this.errorHandler.handleError('Error getting AI completions', error);
      return [];
    }
  }

  private async generateAICompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string,
    token: vscode.CancellationToken
  ): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get context for AI analysis
      const context = await this.getAICompletionContext(document, position);

      // Analyze the current line and suggest intelligent completions
      const suggestions = await this.analyzeLineForSuggestions(line, context);

      // Convert suggestions to completion items
      for (const suggestion of suggestions) {
        const completion = this.createCompletionItem(
          suggestion.label,
          suggestion.kind || 'text',
          suggestion.detail || ''
        );
        completion.insertText = suggestion.insertText || suggestion.label;
        completion.sortText = suggestion.sortText || suggestion.label;
        completion.filterText = suggestion.filterText || suggestion.label;
        completion.documentation = suggestion.documentation
          ? new vscode.MarkdownString(suggestion.documentation)
          : undefined;

        completions.push(completion);
      }

      return completions;
    } catch (error) {
      this.errorHandler.handleError('Error generating AI completions', error);
      return [];
    }
  }

  private async getAICompletionContext(
    document: vscode.TextDocument,
    position: vscode.Position
  ): Promise<any> {
    try {
      // Get document content around the current position
      const startLine = Math.max(0, position.line - 10);
      const endLine = Math.min(document.lineCount - 1, position.line + 10);

      const contextLines: string[] = [];
      for (let i = startLine; i <= endLine; i++) {
        contextLines.push(document.lineAt(i).text);
      }

      // Get workspace context
      const workspaceContext = await this.getWorkspaceContext();

      return {
        documentContent: contextLines.join('\n'),
        currentLine: document.lineAt(position.line).text,
        position: position,
        workspaceContext: workspaceContext,
        documentPath: document.fileName,
      };
    } catch (error) {
      this.errorHandler.handleError('Error getting AI completion context', error);
      return {};
    }
  }

  private async analyzeLineForSuggestions(line: string, context: any): Promise<any[]> {
    try {
      const suggestions: any[] = [];

      // Analyze line patterns and suggest completions
      if (line.trim().endsWith(':')) {
        // Suggest section headers
        suggestions.push(
          { label: 'scope', kind: 'class', detail: 'Scope configuration section' },
          { label: 'context', kind: 'class', detail: 'Context configuration section' },
          { label: 'todos', kind: 'class', detail: 'Todo items section' },
          { label: 'insights', kind: 'class', detail: 'Insights section' },
          { label: 'patterns', kind: 'class', detail: 'Patterns section' },
          { label: 'decisions', kind: 'class', detail: 'Decisions section' }
        );
      }

      // Suggest based on existing patterns in workspace
      if (context.workspaceContext && context.workspaceContext.patterns) {
        for (const pattern of context.workspaceContext.patterns) {
          suggestions.push({
            label: pattern.name || pattern.id,
            kind: 'snippet',
            detail: `Pattern: ${pattern.description || ''}`,
            documentation: pattern.documentation || '',
          });
        }
      }

      // Suggest based on existing decisions in workspace
      if (context.workspaceContext && context.workspaceContext.decisions) {
        for (const decision of context.workspaceContext.decisions) {
          suggestions.push({
            label: decision.name || decision.id,
            kind: 'snippet',
            detail: `Decision: ${decision.description || ''}`,
            documentation: decision.documentation || '',
          });
        }
      }

      return suggestions;
    } catch (error) {
      this.errorHandler.handleError('Error analyzing line for suggestions', error);
      return [];
    }
  }

  private getFileCompletions(workspaceContext: any, documentContext: any): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Suggest files from the workspace
    if (workspaceContext.contexts) {
      for (const context of workspaceContext.contexts) {
        if (context.files) {
          for (const file of context.files) {
            const completion = this.createCompletionItem(file, 'file', `File: ${file}`);
            completion.insertText = `"${file}"`;
            completions.push(completion);
          }
        }
      }
    }

    return completions;
  }

  private getScopeCompletions(workspaceContext: any): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Suggest existing scopes
    if (workspaceContext.scopes) {
      for (const scope of workspaceContext.scopes) {
        const completion = this.createCompletionItem(
          scope.name || scope.id,
          'class',
          `Scope: ${scope.description || ''}`
        );
        completion.insertText = `"${scope.name || scope.id}"`;
        completions.push(completion);
      }
    }

    return completions;
  }

  private getPatternCompletions(workspaceContext: any): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Suggest existing patterns
    if (workspaceContext.patterns) {
      for (const pattern of workspaceContext.patterns) {
        const completion = this.createCompletionItem(
          pattern.name || pattern.id,
          'snippet',
          `Pattern: ${pattern.description || ''}`
        );
        completion.insertText = `"${pattern.name || pattern.id}"`;
        completions.push(completion);
      }
    }

    return completions;
  }

  private getDecisionCompletions(workspaceContext: any): vscode.CompletionItem[] {
    const completions: vscode.CompletionItem[] = [];

    // Suggest existing decisions
    if (workspaceContext.decisions) {
      for (const decision of workspaceContext.decisions) {
        const completion = this.createCompletionItem(
          decision.name || decision.id,
          'snippet',
          `Decision: ${decision.description || ''}`
        );
        completion.insertText = `"${decision.name || decision.id}"`;
        completions.push(completion);
      }
    }

    return completions;
  }

  private async getWorkspaceContext(): Promise<any> {
    try {
      // This would typically get context from the RhemaProvider
      // For now, return a basic structure
      return {
        scopes: [],
        contexts: [],
        todos: [],
        insights: [],
        patterns: [],
        decisions: [],
      };
    } catch (error) {
      this.errorHandler.handleError('Error getting workspace context', error);
      return {};
    }
  }

  private async getDocumentContext(document: vscode.TextDocument): Promise<any> {
    try {
      // Get document-specific context
      const text = document.getText();
      const context: any = {};

      // Extract basic information from the document
      const scopeMatch = text.match(/scope:\s*\n\s*name:\s*(.+)/);
      if (scopeMatch) {
        context.scopeName = scopeMatch[1].trim();
      }

      const contextMatch = text.match(/context:\s*\n\s*files:\s*\[([^\]]+)\]/);
      if (contextMatch) {
        context.contextFiles = contextMatch[1].split(',').map((f: string) => f.trim());
      }

      return context;
    } catch (error) {
      this.errorHandler.handleError('Error getting document context', error);
      return {};
    }
  }

  private removeDuplicateCompletions(
    completions: vscode.CompletionItem[]
  ): vscode.CompletionItem[] {
    const seen = new Set<string>();
    return completions.filter((completion) => {
      const key = completion.label.toString();
      if (seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    });
  }

  private sortCompletionsByRelevance(
    completions: vscode.CompletionItem[],
    word: string
  ): vscode.CompletionItem[] {
    return completions.sort((a, b) => {
      const aLabel = a.label.toString().toLowerCase();
      const bLabel = b.label.toString().toLowerCase();
      const wordLower = word.toLowerCase();

      // Prioritize exact matches
      if (aLabel.startsWith(wordLower) && !bLabel.startsWith(wordLower)) {
        return -1;
      }
      if (!aLabel.startsWith(wordLower) && bLabel.startsWith(wordLower)) {
        return 1;
      }

      // Then prioritize contains matches
      if (aLabel.includes(wordLower) && !bLabel.includes(wordLower)) {
        return -1;
      }
      if (!aLabel.includes(wordLower) && bLabel.includes(wordLower)) {
        return 1;
      }

      // Finally, sort alphabetically
      return aLabel.localeCompare(bLabel);
    });
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
      new vscode.ParameterInformation('param2', 'Optional number parameter'),
    ];

    signatureHelp.signatures = [signature];
    return signatureHelp;
  }

  private async getAsyncCompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string
  ): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get context-aware completions asynchronously
      const contextCompletions = await this.getContextAwareCompletions(
        document,
        position,
        line,
        word
      );
      completions.push(...contextCompletions);

      // Get AI-powered completions if enabled
      if (this.settings.isAICompletionsEnabled()) {
        const aiCompletions = await this.getAiCompletions(document, position, line, word);
        completions.push(...aiCompletions);
      }

      // Get workspace-based completions
      const workspaceCompletions = await this.getWorkspaceCompletions(document, position, line, word);
      completions.push(...workspaceCompletions);

      return completions;
    } catch (error) {
      this.errorHandler.handleError('Error getting async completions', error);
      return [];
    }
  }

  private async getAiCompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string
  ): Promise<vscode.CompletionItem[]> {
    try {
      // Check cache first
      const cacheKey = `${document.uri.toString()}:${position.line}:${position.character}`;
      const cached = this.aiCompletionCache.get(cacheKey);
      if (cached) {
        return cached;
      }

      // Generate AI-powered completions based on context
      const completions: vscode.CompletionItem[] = [];

      // Analyze the current context and generate intelligent suggestions
      const context = await this.getDocumentContext(document);
      const workspaceContext = await this.getWorkspaceContext();

      // Generate completions based on patterns in the workspace
      if (workspaceContext.patterns && Array.isArray(workspaceContext.patterns)) {
        for (const pattern of workspaceContext.patterns) {
          if (pattern.name && pattern.name.toLowerCase().includes(word.toLowerCase())) {
            const completion = this.createCompletionItem(
              pattern.name,
              'pattern',
              `Pattern: ${pattern.description || 'No description'}`
            );
            completions.push(completion);
          }
        }
      }

      // Generate completions based on existing todos
      if (workspaceContext.todos && Array.isArray(workspaceContext.todos)) {
        for (const todo of workspaceContext.todos) {
          if (todo.title && todo.title.toLowerCase().includes(word.toLowerCase())) {
            const completion = this.createCompletionItem(
              todo.title,
              'todo',
              `Todo: ${todo.description || 'No description'}`
            );
            completions.push(completion);
          }
        }
      }

      // Cache the results
      this.aiCompletionCache.set(cacheKey, completions);

      return completions;
    } catch (error) {
      this.errorHandler.handleError('Error getting AI completions', error);
      return [];
    }
  }

  private async getWorkspaceCompletions(
    document: vscode.TextDocument,
    position: vscode.Position,
    line: string,
    word: string
  ): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get file-based completions
      if (line.includes('files:') || line.includes('path:')) {
        const files = await vscode.workspace.findFiles('**/*.{rs,ts,js,py,java,cpp,c,h,go}', '**/node_modules/**');
        for (const file of files.slice(0, 10)) { // Limit to first 10 files
          const fileName = file.path.split('/').pop() || '';
          if (fileName.toLowerCase().includes(word.toLowerCase())) {
            const completion = this.createCompletionItem(
              fileName,
              'file',
              `File: ${file.path}`
            );
            completions.push(completion);
          }
        }
      }

      // Get folder-based completions
      if (line.includes('folders:') || line.includes('directory:')) {
        const folders = await vscode.workspace.findFiles('**/', '**/node_modules/**');
        for (const folder of folders.slice(0, 10)) { // Limit to first 10 folders
          const folderName = folder.path.split('/').pop() || '';
          if (folderName.toLowerCase().includes(word.toLowerCase())) {
            const completion = this.createCompletionItem(
              folderName,
              'folder',
              `Folder: ${folder.path}`
            );
            completions.push(completion);
          }
        }
      }

      return completions;
    } catch (error) {
      this.errorHandler.handleError('Error getting workspace completions', error);
      return [];
    }
  }

  async dispose(): Promise<void> {
    try {
      // Dispose async completion system
      if (this.completionQueueManager) {
        await this.completionQueueManager.dispose();
        this.completionQueueManager = null;
      }

      if (this.completionWorkerService) {
        await this.completionWorkerService.dispose();
        this.completionWorkerService = null;
      }

      // Clear caches
      this.completionItems.clear();
      this.hoverItems.clear();
      this.aiCompletionCache.clear();
      this.contextCache.clear();

      this.logger.info('Rhema IntelliSense disposed');
    } catch (error) {
      this.errorHandler.handleError('Error disposing Rhema IntelliSense', error);
    }
  }

  getAsyncCompletionStatus(): {
    enabled: boolean;
    workerReady: boolean;
    queueStats: any;
    pendingRequests: number;
  } {
    return {
      enabled: this.asyncCompletionsEnabled,
      workerReady: this.completionWorkerService?.isReady() ?? false,
      queueStats: this.completionQueueManager?.getStats() ?? null,
      pendingRequests: this.completionWorkerService?.getPendingRequestCount() ?? 0,
    };
  }
}
