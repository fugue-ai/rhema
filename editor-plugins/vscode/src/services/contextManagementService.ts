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
import { ContextCacheService } from './contextCacheService';
import { AIContextService } from './aiContextService';
import { CrossScopeService } from './crossScopeService';
import { ContextPerformanceService } from './contextPerformanceService';
import {
  WorkspaceContext,
  SemanticContext,
  ContextIndex,
  ContextSuggestion,
  CompletionContext,
  FileChange,
  ScopeDependencyMap,
  UnifiedWorkspaceContext,
  ScopeRelationship,
  SemanticAnalysis,
  UserAction,
  PredictedContext,
  UserFeedback,
  ContextTask,
  ProgressInfo,
  CacheMetrics,
  TaskType,
  TaskPriority,
  ChangeType
} from '../types/context';

export class ContextManagementService {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private cacheService: ContextCacheService;
  private aiService: AIContextService;
  private crossScopeService: CrossScopeService;
  private performanceService: ContextPerformanceService;
  private disposables: vscode.Disposable[] = [];

  // Context state
  private workspaceContext: WorkspaceContext | null = null;
  private contextIndex: ContextIndex | null = null;
  private semanticContext: SemanticContext | null = null;
  private isInitialized = false;

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
    this.cacheService = new ContextCacheService();
    this.aiService = new AIContextService();
    this.crossScopeService = new CrossScopeService();
    this.performanceService = new ContextPerformanceService();
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Context Management Service...');

      // Initialize sub-services
      await this.cacheService.initialize(context);
      await this.aiService.initialize(context);
      await this.crossScopeService.initialize(context);
      await this.performanceService.initialize(context);

      // Set up workspace listeners
      await this.setupWorkspaceListeners(context);

      // Perform initial workspace analysis
      await this.performInitialAnalysis();

      this.isInitialized = true;
      this.logger.info('Context Management Service initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Context Management Service', error);
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

      // Listen for file creation/deletion
      const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.{yml,yaml}');
      const fileCreatedListener = fileWatcher.onDidCreate(this.onFileCreated.bind(this));
      const fileDeletedListener = fileWatcher.onDidDelete(this.onFileDeleted.bind(this));

      this.disposables.push(
        workspaceFolderListener,
        fileSystemListener,
        fileWatcher,
        fileCreatedListener,
        fileDeletedListener
      );

      this.logger.info('Workspace listeners setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup workspace listeners', error);
    }
  }

  private async performInitialAnalysis(): Promise<void> {
    try {
      this.logger.info('Performing initial workspace analysis...');

      // Check cache first
      const cachedContext = await this.cacheService.getCachedContext('workspace');
      if (cachedContext) {
        this.workspaceContext = cachedContext;
        this.logger.info('Loaded workspace context from cache');
        return;
      }

      // Perform fresh analysis
      await this.analyzeWorkspaceSemantics();
      await this.buildContextIndex();
      await this.analyzeScopeDependencies();

      // Cache the results
      if (this.workspaceContext) {
        await this.cacheService.cacheWorkspaceContext(this.workspaceContext);
      }

      this.logger.info('Initial workspace analysis completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to perform initial analysis', error);
    }
  }

  // Core Context Analysis Methods

  async analyzeWorkspaceSemantics(): Promise<SemanticContext> {
    try {
      this.logger.info('Analyzing workspace semantics...');

      const semanticContext: SemanticContext = {
        entities: [],
        relationships: [],
        patterns: [],
        topics: [],
        metadata: {
          analyzedAt: new Date(),
          totalFiles: 0,
          totalEntities: 0,
          confidence: 0.0
        }
      };

      // Analyze all workspace folders
      for (const folder of vscode.workspace.workspaceFolders || []) {
        const folderSemantics = await this.analyzeFolderSemantics(folder);
        semanticContext.entities.push(...folderSemantics.entities);
        semanticContext.relationships.push(...folderSemantics.relationships);
        semanticContext.patterns.push(...folderSemantics.patterns);
        semanticContext.topics.push(...folderSemantics.topics);
      }

      // Use AI service to enhance semantic understanding
      const enhancedSemantics = await this.aiService.enhanceSemanticAnalysis(semanticContext);
      this.semanticContext = enhancedSemantics;

      this.logger.info(`Semantic analysis completed: ${enhancedSemantics.entities.length} entities found`);
      return enhancedSemantics;
    } catch (error) {
      this.errorHandler.handleError('Failed to analyze workspace semantics', error);
      return {
        entities: [],
        relationships: [],
        patterns: [],
        topics: [],
        metadata: {
          analyzedAt: new Date(),
          totalFiles: 0,
          totalEntities: 0,
          confidence: 0.0
        }
      };
    }
  }

  async buildContextIndex(): Promise<ContextIndex> {
    try {
      this.logger.info('Building context index...');

      const contextIndex: ContextIndex = {
        files: new Map(),
        symbols: new Map(),
        references: new Map(),
        dependencies: new Map(),
        metadata: {
          indexedAt: new Date(),
          totalFiles: 0,
          totalSymbols: 0
        }
      };

      // Index all Rhema files
      for (const folder of vscode.workspace.workspaceFolders || []) {
        const rhemaFiles = await vscode.workspace.findFiles(
          new vscode.RelativePattern(folder, '**/*.{yml,yaml}'),
          '**/node_modules/**'
        );

        for (const file of rhemaFiles) {
          await this.indexFile(file, contextIndex);
        }
      }

      this.contextIndex = contextIndex;
      this.logger.info(`Context index built: ${contextIndex.files.size} files indexed`);
      return contextIndex;
    } catch (error) {
      this.errorHandler.handleError('Failed to build context index', error);
      return {
        files: new Map(),
        symbols: new Map(),
        references: new Map(),
        dependencies: new Map(),
        metadata: {
          indexedAt: new Date(),
          totalFiles: 0,
          totalSymbols: 0
        }
      };
    }
  }

  async getContextSuggestions(context: CompletionContext): Promise<ContextSuggestion[]> {
    try {
      this.logger.info('Getting context suggestions...');

      // Get AI-powered suggestions
      const aiSuggestions = await this.aiService.generateContextSuggestions(context.query);

      // Get cache-based suggestions
      const cacheSuggestions = await this.cacheService.getContextSuggestions(context);

      // Get cross-scope suggestions
      const crossScopeSuggestions = await this.crossScopeService.getCrossScopeSuggestions(context);

      // Combine and rank suggestions
      const allSuggestions = [
        ...aiSuggestions,
        ...cacheSuggestions,
        ...crossScopeSuggestions
      ];

      // Remove duplicates and rank by relevance
      const uniqueSuggestions = this.deduplicateSuggestions(allSuggestions);
      const rankedSuggestions = await this.rankSuggestions(uniqueSuggestions, context);

      this.logger.info(`Generated ${rankedSuggestions.length} context suggestions`);
      return rankedSuggestions;
    } catch (error) {
      this.errorHandler.handleError('Failed to get context suggestions', error);
      return [];
    }
  }

  async updateContextIncrementally(changes: FileChange[]): Promise<void> {
    try {
      this.logger.info(`Updating context incrementally for ${changes.length} changes`);

      // Process changes in background
      await this.performanceService.processInBackground({
        id: `context_update_${Date.now()}`,
        type: TaskType.ContextUpdate,
        changes,
        priority: TaskPriority.Medium,
        metadata: {
          created: new Date(),
          priority: TaskPriority.Medium,
          estimatedTime: 1000
        }
      });

      // Update cache
      await this.cacheService.invalidateCache('workspace');
      
      // Re-analyze affected files
      for (const change of changes) {
        await this.analyzeFileChange(change);
      }

      this.logger.info('Incremental context update completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to update context incrementally', error);
    }
  }

  // Cross-Scope Integration Methods

  async analyzeScopeDependencies(): Promise<ScopeDependencyMap> {
    try {
      this.logger.info('Analyzing scope dependencies...');
      return await this.crossScopeService.analyzeScopeDependencies();
    } catch (error) {
      this.errorHandler.handleError('Failed to analyze scope dependencies', error);
      return {};
    }
  }

  async getUnifiedContext(): Promise<UnifiedWorkspaceContext> {
    try {
      this.logger.info('Getting unified workspace context...');
      return await this.crossScopeService.getUnifiedContext();
    } catch (error) {
      this.errorHandler.handleError('Failed to get unified context', error);
      return {
        scopes: [],
        relationships: [],
        metadata: {
          unifiedAt: new Date(),
          totalScopes: 0
        }
      };
    }
  }

  async shareContextBetweenScopes(source: string, target: string): Promise<void> {
    try {
      this.logger.info(`Sharing context from ${source} to ${target}`);
      await this.crossScopeService.shareContextBetweenScopes(source, target);
    } catch (error) {
      this.errorHandler.handleError('Failed to share context between scopes', error);
    }
  }

  async getScopeRelationships(): Promise<ScopeRelationship[]> {
    try {
      return await this.crossScopeService.getScopeRelationships();
    } catch (error) {
      this.errorHandler.handleError('Failed to get scope relationships', error);
      return [];
    }
  }

  // Performance and Monitoring Methods

  async getCacheMetrics(): Promise<CacheMetrics> {
    try {
      return await this.cacheService.getCacheMetrics();
    } catch (error) {
      this.errorHandler.handleError('Failed to get cache metrics', error);
      return {
        hitRate: 0,
        missRate: 0,
        totalRequests: 0,
        averageResponseTime: 0,
        metadata: {
          timestamp: new Date(),
          period: '1h',
          source: 'error_fallback'
        }
      };
    }
  }

  async optimizeResourceUsage(): Promise<void> {
    try {
      this.logger.info('Optimizing resource usage...');
      await this.performanceService.optimizeResourceUsage();
    } catch (error) {
      this.errorHandler.handleError('Failed to optimize resource usage', error);
    }
  }

  // Event Handlers

  private async onWorkspaceFoldersChanged(event: vscode.WorkspaceFoldersChangeEvent): Promise<void> {
    try {
      this.logger.info('Workspace folders changed, updating context...');

      if (event.added.length > 0) {
        for (const folder of event.added) {
          await this.analyzeFolderSemantics(folder);
        }
      }

      if (event.removed.length > 0) {
        for (const folder of event.removed) {
          await this.cleanupFolderContext(folder);
        }
      }

      // Update cache
      await this.cacheService.invalidateCache('workspace');
    } catch (error) {
      this.errorHandler.handleError('Error handling workspace folder changes', error);
    }
  }

  private async onDocumentChanged(event: vscode.TextDocumentChangeEvent): Promise<void> {
    try {
      if (!this.isRhemaFile(event.document)) {
        return;
      }

      // Update context incrementally
      const changes: FileChange[] = [{
        file: event.document.uri.fsPath,
        type: ChangeType.Modified,
        timestamp: new Date()
      }];

      await this.updateContextIncrementally(changes);
    } catch (error) {
      this.errorHandler.handleError('Error handling document changes', error);
    }
  }

  private async onFileCreated(uri: vscode.Uri): Promise<void> {
    try {
      if (!this.isRhemaFile(uri)) {
        return;
      }

      const changes: FileChange[] = [{
        file: uri.fsPath,
        type: ChangeType.Created,
        timestamp: new Date()
      }];

      await this.updateContextIncrementally(changes);
    } catch (error) {
      this.errorHandler.handleError('Error handling file creation', error);
    }
  }

  private async onFileDeleted(uri: vscode.Uri): Promise<void> {
    try {
      if (!this.isRhemaFile(uri)) {
        return;
      }

      const changes: FileChange[] = [{
        file: uri.fsPath,
        type: ChangeType.Deleted,
        timestamp: new Date()
      }];

      await this.updateContextIncrementally(changes);
    } catch (error) {
      this.errorHandler.handleError('Error handling file deletion', error);
    }
  }

  // Helper Methods

  private async analyzeFolderSemantics(folder: vscode.WorkspaceFolder): Promise<SemanticContext> {
    // Implementation for folder semantic analysis
    return {
      entities: [],
      relationships: [],
      patterns: [],
      topics: [],
      metadata: {
        analyzedAt: new Date(),
        totalFiles: 0,
        totalEntities: 0,
        confidence: 0.0
      }
    };
  }

  private async indexFile(file: vscode.Uri, contextIndex: ContextIndex): Promise<void> {
    // Implementation for file indexing
  }

  private async analyzeFileChange(change: FileChange): Promise<void> {
    // Implementation for file change analysis
  }

  private async cleanupFolderContext(folder: vscode.WorkspaceFolder): Promise<void> {
    // Implementation for folder cleanup
  }

  private isRhemaFile(documentOrUri: vscode.TextDocument | vscode.Uri): boolean {
    const fileName = 'uri' in documentOrUri ? documentOrUri.uri.fsPath : documentOrUri.fsPath;
    return fileName.endsWith('.yml') || fileName.endsWith('.yaml');
  }

  private deduplicateSuggestions(suggestions: ContextSuggestion[]): ContextSuggestion[] {
    // Implementation for deduplication
    return suggestions;
  }

  private async rankSuggestions(suggestions: ContextSuggestion[], context: CompletionContext): Promise<ContextSuggestion[]> {
    // Implementation for suggestion ranking
    return suggestions;
  }

  // Public API Methods

  async getWorkspaceContext(): Promise<WorkspaceContext | null> {
    return this.workspaceContext;
  }

  async getContextIndex(): Promise<ContextIndex | null> {
    return this.contextIndex;
  }

  async getSemanticContext(): Promise<SemanticContext | null> {
    return this.semanticContext;
  }

  async dispose(): Promise<void> {
    try {
      this.logger.info('Disposing Context Management Service...');

      // Dispose sub-services
      await this.cacheService.dispose();
      await this.aiService.dispose();
      await this.crossScopeService.dispose();
      await this.performanceService.dispose();

      // Dispose listeners
      this.disposables.forEach(disposable => disposable.dispose());
      this.disposables = [];

      this.isInitialized = false;
      this.logger.info('Context Management Service disposed');
    } catch (error) {
      this.errorHandler.handleError('Failed to dispose Context Management Service', error);
    }
  }
} 