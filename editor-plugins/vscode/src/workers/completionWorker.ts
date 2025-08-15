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

import { parentPort, workerData } from 'worker_threads';
import * as vscode from 'vscode';

interface CompletionRequest {
  id: string;
  document: {
    fileName: string;
    languageId: string;
    content: string;
  };
  position: {
    line: number;
    character: number;
  };
  line: string;
  word: string;
  context: any;
  timestamp: number;
}

interface CompletionResponse {
  id: string;
  completions: vscode.CompletionItem[];
  error?: string;
  timestamp: number;
}

interface CompletionWorkerMessage {
  type: 'request' | 'response' | 'error' | 'ping' | 'pong';
  data: CompletionRequest | CompletionResponse | string;
}

class CompletionWorker {
  private completionItems: Map<string, vscode.CompletionItem[]> = new Map();
  private hoverItems: Map<string, vscode.MarkdownString> = new Map();
  private workspaceContext: any = {};
  private aiCompletionCache: Map<string, vscode.CompletionItem[]> = new Map();
  private contextCache: Map<string, any> = new Map();

  constructor() {
    this.initializeCompletionItems();
    this.initializeHoverItems();
    this.setupMessageHandling();
  }

  private setupMessageHandling(): void {
    if (!parentPort) {
      throw new Error('Parent port not available');
    }

    parentPort.on('message', async (message: CompletionWorkerMessage) => {
      try {
        switch (message.type) {
          case 'request':
            await this.handleCompletionRequest(message.data as CompletionRequest);
            break;
          case 'ping':
            this.sendMessage({ type: 'pong', data: 'pong' });
            break;
          default:
            console.warn(`Unknown message type: ${message.type}`);
        }
      } catch (error) {
        console.error('Error handling message:', error);
        this.sendMessage({ type: 'error', data: error instanceof Error ? error.message : 'Unknown error' });
      }
    });
  }

  private async handleCompletionRequest(request: CompletionRequest): Promise<void> {
    try {
      const completions = await this.generateCompletions(request);
      
      const response: CompletionResponse = {
        id: request.id,
        completions,
        timestamp: Date.now(),
      };

      this.sendMessage({ type: 'response', data: response });
    } catch (error) {
      const response: CompletionResponse = {
        id: request.id,
        completions: [],
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: Date.now(),
      };

      this.sendMessage({ type: 'response', data: response });
    }
  }

  private async generateCompletions(request: CompletionRequest): Promise<vscode.CompletionItem[]> {
    const completions: vscode.CompletionItem[] = [];

    // Get basic completions based on current position
    const basicCompletions = this.getBasicCompletions(request.line);
    completions.push(...basicCompletions);

    // Get context-aware completions
    const contextCompletions = await this.getContextAwareCompletions(request);
    completions.push(...contextCompletions);

    // Get AI-powered completions if enabled
    const aiCompletions = await this.getAICompletions(request);
    completions.push(...aiCompletions);

    // Remove duplicates and sort by relevance
    const uniqueCompletions = this.removeDuplicateCompletions(completions);
    const sortedCompletions = this.sortCompletionsByRelevance(uniqueCompletions, request.word);

    return sortedCompletions;
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

  private async getContextAwareCompletions(request: CompletionRequest): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get workspace context
      const workspaceContext = await this.getWorkspaceContext();

      // Get document context
      const documentContext = await this.getDocumentContext(request.document);

      // Context-aware completions based on workspace state
      if (request.line.includes('files:') || request.line.includes('Files:')) {
        const fileCompletions = this.getFileCompletions(workspaceContext, documentContext);
        completions.push(...fileCompletions);
      }

      // Context-aware completions based on existing scopes
      if (request.line.includes('dependencies:') || request.line.includes('Dependencies:')) {
        const scopeCompletions = this.getScopeCompletions(workspaceContext);
        completions.push(...scopeCompletions);
      }

      // Context-aware completions based on existing patterns
      if (request.line.includes('patterns:') || request.line.includes('Patterns:')) {
        const patternCompletions = this.getPatternCompletions(workspaceContext);
        completions.push(...patternCompletions);
      }

      // Context-aware completions based on existing decisions
      if (request.line.includes('decisions:') || request.line.includes('Decisions:')) {
        const decisionCompletions = this.getDecisionCompletions(workspaceContext);
        completions.push(...decisionCompletions);
      }

      return completions;
    } catch (error) {
      console.error('Error getting context-aware completions:', error);
      return [];
    }
  }

  private async getAICompletions(request: CompletionRequest): Promise<vscode.CompletionItem[]> {
    try {
      // Check cache first
      const cacheKey = `${request.document.fileName}:${request.position.line}:${request.position.character}`;
      if (this.aiCompletionCache.has(cacheKey)) {
        return this.aiCompletionCache.get(cacheKey) || [];
      }

      // Generate AI-powered completions
      const aiCompletions = await this.generateAICompletions(request);

      // Cache the results
      this.aiCompletionCache.set(cacheKey, aiCompletions);

      return aiCompletions;
    } catch (error) {
      console.error('Error getting AI completions:', error);
      return [];
    }
  }

  private async generateAICompletions(request: CompletionRequest): Promise<vscode.CompletionItem[]> {
    try {
      const completions: vscode.CompletionItem[] = [];

      // Get context for AI analysis
      const context = await this.getAICompletionContext(request);

      // Analyze the current line and suggest intelligent completions
      const suggestions = await this.analyzeLineForSuggestions(request.line, context);

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
      console.error('Error generating AI completions:', error);
      return [];
    }
  }

  private async getAICompletionContext(request: CompletionRequest): Promise<any> {
    try {
      // Get document content around the current position
      const lines = request.document.content.split('\n');
      const startLine = Math.max(0, request.position.line - 10);
      const endLine = Math.min(lines.length - 1, request.position.line + 10);

      const contextLines: string[] = [];
      for (let i = startLine; i <= endLine; i++) {
        contextLines.push(lines[i] || '');
      }

      // Get workspace context
      const workspaceContext = await this.getWorkspaceContext();

      return {
        documentContent: contextLines.join('\n'),
        currentLine: lines[request.position.line] || '',
        position: request.position,
        workspaceContext: workspaceContext,
        documentPath: request.document.fileName,
      };
    } catch (error) {
      console.error('Error getting AI completion context:', error);
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
      console.error('Error analyzing line for suggestions:', error);
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
      console.error('Error getting workspace context:', error);
      return {};
    }
  }

  private async getDocumentContext(document: any): Promise<any> {
    try {
      // Get document-specific context
      const text = document.content;
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
      console.error('Error getting document context:', error);
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
    // Initialize hover items (not used in worker but kept for consistency)
    this.hoverItems.set(
      'scope',
      new vscode.MarkdownString('# Rhema Scope\n\nA scope defines a bounded context for AI agents.')
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
      case 'file':
        return vscode.CompletionItemKind.File;
      case 'snippet':
        return vscode.CompletionItemKind.Snippet;
      default:
        return vscode.CompletionItemKind.Text;
    }
  }

  private sendMessage(message: CompletionWorkerMessage): void {
    if (parentPort) {
      parentPort.postMessage(message);
    }
  }
}

// Initialize the worker
new CompletionWorker();
