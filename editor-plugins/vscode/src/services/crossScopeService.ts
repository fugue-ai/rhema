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
import * as path from 'path';
import * as fs from 'fs';
import { RhemaLogger } from '../logger';
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';
import {
  ScopeDependencyMap,
  UnifiedWorkspaceContext,
  ScopeRelationship,
  ContextSuggestion,
  CompletionContext,
  DependencyType,
  RelationshipType
} from '../types/context';

interface ScopeInfo {
  id: string;
  name: string;
  path: string;
  dependencies: string[];
  dependents: string[];
  metadata: {
    created: Date;
    modified: Date;
    size: number;
    complexity: number;
  };
}

interface ScopeDependency {
  source: string;
  target: string;
  type: DependencyType;
  strength: number;
  metadata: {
    created: Date;
    modified: Date;
    bidirectional: boolean;
    strength: number;
  };
}

export class CrossScopeService {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private disposables: vscode.Disposable[] = [];

  // Scope tracking
  private scopes: Map<string, ScopeInfo> = new Map();
  private dependencies: Map<string, ScopeDependency[]> = new Map();
  private relationships: Map<string, ScopeRelationship[]> = new Map();

  // Configuration
  private maxScopeDepth: number = 10;
  private dependencyStrengthThreshold: number = 0.5;
  private autoAnalysisEnabled: boolean = true;

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Cross-Scope Service...');

      // Load configuration
      await this.loadConfiguration();

      // Discover scopes in workspace
      await this.discoverScopes();

      // Analyze scope dependencies
      await this.analyzeScopeDependencies();

      // Set up scope monitoring
      await this.setupScopeMonitoring();

      this.logger.info('Cross-Scope Service initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Cross-Scope Service', error);
    }
  }

  private async loadConfiguration(): Promise<void> {
    try {
      this.maxScopeDepth = this.settings.getConfiguration('rhema.maxScopeDepth', 10);
      this.dependencyStrengthThreshold = this.settings.getConfiguration('rhema.dependencyStrengthThreshold', 0.5);
      this.autoAnalysisEnabled = this.settings.getConfiguration('rhema.autoAnalysisEnabled', true);

      this.logger.info(`Cross-Scope Service configured: maxDepth=${this.maxScopeDepth}, threshold=${this.dependencyStrengthThreshold}`);
    } catch (error) {
      this.errorHandler.handleError('Failed to load configuration', error);
    }
  }

  private async discoverScopes(): Promise<void> {
    try {
      this.logger.info('Discovering scopes in workspace...');

      const workspaceFolders = vscode.workspace.workspaceFolders;
      if (!workspaceFolders) {
        this.logger.warn('No workspace folders found');
        return;
      }

      for (const folder of workspaceFolders) {
        await this.discoverScopesInFolder(folder);
      }

      this.logger.info(`Discovered ${this.scopes.size} scopes in workspace`);
    } catch (error) {
      this.errorHandler.handleError('Failed to discover scopes', error);
    }
  }

  private async discoverScopesInFolder(folder: vscode.WorkspaceFolder): Promise<void> {
    try {
      const rhemaFiles = await vscode.workspace.findFiles(
        new vscode.RelativePattern(folder, '**/*.{yml,yaml}'),
        '**/node_modules/**'
      );

      for (const file of rhemaFiles) {
        await this.analyzeScopeFile(file);
      }
    } catch (error) {
      this.errorHandler.handleError(`Failed to discover scopes in folder: ${folder.name}`, error);
    }
  }

  private async analyzeScopeFile(file: vscode.Uri): Promise<void> {
    try {
      const content = await fs.promises.readFile(file.fsPath, 'utf8');
      const scopeInfo = this.extractScopeInfo(file.fsPath, content);
      
      if (scopeInfo) {
        this.scopes.set(scopeInfo.id, scopeInfo);
        this.logger.debug(`Analyzed scope: ${scopeInfo.name} at ${file.fsPath}`);
      }
         } catch {
       this.logger.warn(`Failed to analyze scope file: ${file.fsPath}`);
     }
  }

  private extractScopeInfo(filePath: string, content: string): ScopeInfo | null {
    try {
      // Extract scope information from YAML content
      const scopeMatch = content.match(/scope:\s*\n\s*name:\s*(.+)/);
      if (!scopeMatch) {
        return null;
      }

      const scopeName = scopeMatch[1].trim();
      const scopeId = this.generateScopeId(filePath, scopeName);
      
      // Extract dependencies from content
      const dependencies = this.extractDependencies(content);
      
      // Get file metadata
      const stats = fs.statSync(filePath);
      
      return {
        id: scopeId,
        name: scopeName,
        path: filePath,
        dependencies,
        dependents: [],
        metadata: {
          created: stats.birthtime,
          modified: stats.mtime,
          size: stats.size,
          complexity: this.calculateComplexity(content)
        }
      };
         } catch {
       this.logger.warn(`Failed to extract scope info from: ${filePath}`);
       return null;
     }
  }

  private extractDependencies(content: string): string[] {
    const dependencies: string[] = [];
    
    // Look for dependency patterns in the content
    const dependencyPatterns = [
      /dependencies:\s*\[([^\]]+)\]/g,
      /imports:\s*\[([^\]]+)\]/g,
      /requires:\s*\[([^\]]+)\]/g
    ];

    for (const pattern of dependencyPatterns) {
      const matches = content.matchAll(pattern);
      for (const match of matches) {
        const deps = match[1].split(',').map(d => d.trim());
        dependencies.push(...deps);
      }
    }

    return [...new Set(dependencies)]; // Remove duplicates
  }

  private calculateComplexity(content: string): number {
    try {
      // Simple complexity calculation based on content size and structure
      const lines = content.split('\n').length;
      const sections = (content.match(/^[a-zA-Z]+:/gm) || []).length;
      const nestedLevels = Math.max(...content.split('\n').map(line => 
        (line.match(/^\s*/)?.[0].length || 0) / 2
      ));
      
      return Math.min((lines * 0.1 + sections * 0.5 + nestedLevels * 0.3), 10);
         } catch {
       return 1.0; // Default complexity
     }
  }

  private generateScopeId(filePath: string, scopeName: string): string {
    const normalizedPath = path.normalize(filePath);
    const hash = require('crypto').createHash('md5');
    hash.update(normalizedPath + scopeName);
    return hash.digest('hex').substring(0, 8);
  }

  // Core Cross-Scope Methods

  async analyzeScopeDependencies(): Promise<ScopeDependencyMap> {
    try {
      this.logger.info('Analyzing scope dependencies...');

      const dependencyMap: ScopeDependencyMap = {};

      for (const [scopeId, scopeInfo] of this.scopes) {
        const scopeDependencies: ScopeDependency[] = [];
        
        for (const depName of scopeInfo.dependencies) {
          const targetScope = this.findScopeByName(depName);
          if (targetScope) {
            const strength = this.calculateDependencyStrength(scopeInfo, targetScope);
            const dependency: ScopeDependency = {
              source: scopeId,
              target: targetScope.id,
              type: this.determineDependencyType(scopeInfo, targetScope),
              strength: strength,
              metadata: {
                created: new Date(),
                modified: new Date(),
                bidirectional: false,
                strength: strength
              }
            };

            if (dependency.strength >= this.dependencyStrengthThreshold) {
              scopeDependencies.push(dependency);
              
              // Update dependents
              targetScope.dependents.push(scopeId);
            }
          }
        }

        if (scopeDependencies.length > 0) {
          dependencyMap[scopeId] = scopeDependencies;
          this.dependencies.set(scopeId, scopeDependencies);
        }
      }

      // Analyze bidirectional dependencies
      this.analyzeBidirectionalDependencies(dependencyMap);

      this.logger.info(`Analyzed dependencies for ${Object.keys(dependencyMap).length} scopes`);
      return dependencyMap;
    } catch (error) {
      this.errorHandler.handleError('Failed to analyze scope dependencies', error);
      return {};
    }
  }

  async getUnifiedContext(): Promise<UnifiedWorkspaceContext> {
    try {
      this.logger.info('Getting unified workspace context...');

      const unifiedScopes = [];
      const relationships: ScopeRelationship[] = [];

      // Convert scopes to unified format
      for (const [scopeId, scopeInfo] of this.scopes) {
        const scopeDependencies = this.dependencies.get(scopeId) || [];
        
        unifiedScopes.push({
          id: scopeId,
          name: scopeInfo.name,
          contexts: [], // Would be populated from actual context data
          dependencies: scopeDependencies.map(d => d.target),
          metadata: {
            created: scopeInfo.metadata.created,
            modified: scopeInfo.metadata.modified,
            contextCount: 0, // Would be calculated from actual data
            dependencyCount: scopeDependencies.length
          }
        });

        // Convert dependencies to relationships
        for (const dep of scopeDependencies) {
          relationships.push({
            id: `rel_${scopeId}_${dep.target}`,
            source: scopeId,
            target: dep.target,
            type: this.mapDependencyTypeToRelationshipType(dep.type),
            strength: dep.strength,
            metadata: {
              created: dep.metadata.created,
              modified: dep.metadata.modified,
              confidence: dep.strength,
              bidirectional: dep.metadata.bidirectional
            }
          });
        }
      }

      return {
        scopes: unifiedScopes,
        relationships,
        metadata: {
          unifiedAt: new Date(),
          totalScopes: unifiedScopes.length
        }
      };
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

      const sourceScope = this.scopes.get(source);
      const targetScope = this.scopes.get(target);

      if (!sourceScope || !targetScope) {
        throw new Error(`Source or target scope not found: ${source} -> ${target}`);
      }

      // Create or update dependency relationship
      const dependency: ScopeDependency = {
        source: target, // Target depends on source
        target: source,
        type: DependencyType.Reference,
        strength: 0.8, // High strength for explicit sharing
        metadata: {
          created: new Date(),
          modified: new Date(),
          bidirectional: false,
          strength: 0.8
        }
      };

      // Update dependencies
      const targetDeps = this.dependencies.get(target) || [];
      targetDeps.push(dependency);
      this.dependencies.set(target, targetDeps);

      // Update scope info
      targetScope.dependencies.push(sourceScope.name);
      sourceScope.dependents.push(target);

      this.logger.info(`Context sharing established: ${source} -> ${target}`);
    } catch (error) {
      this.errorHandler.handleError('Failed to share context between scopes', error);
    }
  }

  async getScopeRelationships(): Promise<ScopeRelationship[]> {
    try {
      const relationships: ScopeRelationship[] = [];

      for (const [, deps] of this.dependencies) {
        for (const dep of deps) {
          relationships.push({
            id: `rel_${dep.source}_${dep.target}`,
            source: dep.source,
            target: dep.target,
            type: this.mapDependencyTypeToRelationshipType(dep.type),
            strength: dep.strength,
            metadata: {
              created: dep.metadata.created,
              modified: dep.metadata.modified,
              confidence: dep.strength,
              bidirectional: dep.metadata.bidirectional
            }
          });
        }
      }

      return relationships;
    } catch (error) {
      this.errorHandler.handleError('Failed to get scope relationships', error);
      return [];
    }
  }

  async getCrossScopeSuggestions(context: CompletionContext): Promise<ContextSuggestion[]> {
    try {
      const suggestions: ContextSuggestion[] = [];
      const currentScope = await this.getCurrentScope();

      if (!currentScope) {
        return suggestions;
      }

      // Get related scopes
      const relatedScopes = this.getRelatedScopes(currentScope);
      
      for (const relatedScope of relatedScopes) {
        suggestions.push({
          id: `cross_scope_${relatedScope.id}`,
          type: require('../types/context').SuggestionType.Completion,
          title: `Use context from ${relatedScope.name}`,
          description: `Access context and components from the ${relatedScope.name} scope`,
          relevance: relatedScope.strength,
          confidence: relatedScope.strength,
          source: require('../types/context').SuggestionSource.CrossScope,
          action: {
            type: require('../types/context').ActionType.Completion,
            title: `Import from ${relatedScope.name}`,
            command: 'rhema.scope.import',
            arguments: [relatedScope.id]
          },
          metadata: {
            created: new Date(),
            modified: new Date(),
            source: 'cross_scope_service',
            tags: ['cross-scope', 'import', relatedScope.name]
          }
        });
      }

      return suggestions;
    } catch (error) {
      this.errorHandler.handleError('Failed to get cross-scope suggestions', error);
      return [];
    }
  }

  // Helper Methods

  private findScopeByName(name: string): ScopeInfo | null {
    for (const scope of this.scopes.values()) {
      if (scope.name === name) {
        return scope;
      }
    }
    return null;
  }

  private determineDependencyType(source: ScopeInfo, target: ScopeInfo): DependencyType {
    // Simple heuristic to determine dependency type
    if (source.path.includes(target.name) || target.path.includes(source.name)) {
      return DependencyType.Composition;
    }
    
    if (source.dependencies.includes(target.name)) {
      return DependencyType.Import;
    }
    
    return DependencyType.Reference;
  }

  private calculateDependencyStrength(source: ScopeInfo, target: ScopeInfo): number {
    // Calculate dependency strength based on various factors
    let strength = 0.5; // Base strength

    // Factor 1: Path proximity
    const sourceDir = path.dirname(source.path);
    const targetDir = path.dirname(target.path);
    const pathDistance = this.calculatePathDistance(sourceDir, targetDir);
    strength += (1 - pathDistance / 10) * 0.2;

    // Factor 2: Complexity correlation
    const complexityDiff = Math.abs(source.metadata.complexity - target.metadata.complexity);
    strength += (1 - complexityDiff / 10) * 0.2;

    // Factor 3: Explicit dependency
    if (source.dependencies.includes(target.name)) {
      strength += 0.3;
    }

    return Math.min(Math.max(strength, 0), 1);
  }

  private calculatePathDistance(path1: string, path2: string): number {
    const parts1 = path1.split(path.sep);
    const parts2 = path2.split(path.sep);
    
    let commonPrefix = 0;
    for (let i = 0; i < Math.min(parts1.length, parts2.length); i++) {
      if (parts1[i] === parts2[i]) {
        commonPrefix++;
      } else {
        break;
      }
    }
    
    return parts1.length + parts2.length - 2 * commonPrefix;
  }

  private analyzeBidirectionalDependencies(dependencyMap: ScopeDependencyMap): void {
    for (const [scopeId, deps] of Object.entries(dependencyMap)) {
      for (const dep of deps) {
        const reverseDeps = dependencyMap[dep.target] || [];
        const reverseDep = reverseDeps.find(d => d.target === scopeId);
        
        if (reverseDep) {
          dep.metadata.bidirectional = true;
          reverseDep.metadata.bidirectional = true;
          
          // Adjust strengths for bidirectional dependencies
          const avgStrength = (dep.strength + reverseDep.strength) / 2;
          dep.strength = avgStrength;
          reverseDep.strength = avgStrength;
        }
      }
    }
  }

  private mapDependencyTypeToRelationshipType(depType: DependencyType): RelationshipType {
    switch (depType) {
      case DependencyType.Composition:
        return RelationshipType.Composition;
      case DependencyType.Aggregation:
        return RelationshipType.Aggregation;
      case DependencyType.Inheritance:
        return RelationshipType.Inheritance;
      case DependencyType.Import:
        return RelationshipType.Import;
      case DependencyType.Reference:
      default:
        return RelationshipType.Reference;
    }
  }

  private async getCurrentScope(): Promise<string | null> {
    try {
      const activeDocument = vscode.window.activeTextEditor?.document;
      if (!activeDocument) {
        return null;
      }

      const filePath = activeDocument.fileName;
      for (const [scopeId, scopeInfo] of this.scopes) {
        if (filePath.includes(scopeInfo.name) || filePath.includes(path.dirname(scopeInfo.path))) {
          return scopeId;
        }
      }

      return null;
    } catch (error) {
      this.errorHandler.handleError('Failed to get current scope', error);
      return null;
    }
  }

  private getRelatedScopes(currentScopeId: string): Array<{ id: string; name: string; strength: number }> {
    const related: Array<{ id: string; name: string; strength: number }> = [];
    
    // Get direct dependencies
    const deps = this.dependencies.get(currentScopeId) || [];
    for (const dep of deps) {
      const targetScope = this.scopes.get(dep.target);
      if (targetScope) {
        related.push({
          id: dep.target,
          name: targetScope.name,
          strength: dep.strength
        });
      }
    }

    // Get dependents
    const currentScope = this.scopes.get(currentScopeId);
    if (currentScope) {
      for (const dependentId of currentScope.dependents) {
        const dependentScope = this.scopes.get(dependentId);
        if (dependentScope) {
          related.push({
            id: dependentId,
            name: dependentScope.name,
            strength: 0.6 // Default strength for dependents
          });
        }
      }
    }

    // Sort by strength and remove duplicates
    const uniqueRelated = related.filter((item, index, self) => 
      index === self.findIndex(t => t.id === item.id)
    );
    
    return uniqueRelated.sort((a, b) => b.strength - a.strength);
  }

  private async setupScopeMonitoring(): Promise<void> {
    try {
      // Monitor for scope file changes
      const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.{yml,yaml}');
      
      const fileCreatedListener = fileWatcher.onDidCreate(this.onScopeFileCreated.bind(this));
      const fileChangedListener = fileWatcher.onDidChange(this.onScopeFileChanged.bind(this));
      const fileDeletedListener = fileWatcher.onDidDelete(this.onScopeFileDeleted.bind(this));

      this.disposables.push(
        fileWatcher,
        fileCreatedListener,
        fileChangedListener,
        fileDeletedListener
      );

      this.logger.info('Scope monitoring setup completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to setup scope monitoring', error);
    }
  }

  private async onScopeFileCreated(uri: vscode.Uri): Promise<void> {
    try {
      await this.analyzeScopeFile(uri);
      await this.analyzeScopeDependencies();
    } catch (error) {
      this.errorHandler.handleError('Failed to handle scope file creation', error);
    }
  }

  private async onScopeFileChanged(uri: vscode.Uri): Promise<void> {
    try {
      await this.analyzeScopeFile(uri);
      await this.analyzeScopeDependencies();
    } catch (error) {
      this.errorHandler.handleError('Failed to handle scope file change', error);
    }
  }

  private async onScopeFileDeleted(uri: vscode.Uri): Promise<void> {
    try {
      // Remove scope from tracking
      for (const [scopeId, scopeInfo] of this.scopes) {
        if (scopeInfo.path === uri.fsPath) {
          this.scopes.delete(scopeId);
          this.dependencies.delete(scopeId);
          break;
        }
      }
      
      await this.analyzeScopeDependencies();
    } catch (error) {
      this.errorHandler.handleError('Failed to handle scope file deletion', error);
    }
  }

  async dispose(): Promise<void> {
    try {
      this.logger.info('Disposing Cross-Scope Service...');

      // Dispose listeners
      this.disposables.forEach(disposable => disposable.dispose());
      this.disposables = [];

      // Clear data
      this.scopes.clear();
      this.dependencies.clear();
      this.relationships.clear();

      this.logger.info('Cross-Scope Service disposed');
    } catch (error) {
      this.errorHandler.handleError('Failed to dispose Cross-Scope Service', error);
    }
  }
} 