import {
  type WorkspaceFolder,
  type FileEvent,
  FileChangeType,
  TextDocument,
} from 'vscode-languageserver/node';
import type { RhemaDocument } from './parser';
import * as path from 'path';
import * as fs from 'fs-extra';
import { glob } from 'glob';

export interface WorkspaceFile {
  uri: string;
  path: string;
  name: string;
  type: 'scope' | 'knowledge' | 'todos' | 'decisions' | 'patterns' | 'conventions' | 'unknown';
  content?: RhemaDocument;
  lastModified: Date;
  size: number;
}

export interface WorkspaceIndex {
  files: Map<string, WorkspaceFile>;
  documents: Map<string, RhemaDocument>;
  symbols: Map<string, SymbolInfo[]>;
  references: Map<string, ReferenceInfo[]>;
  dependencies: Map<string, string[]>;
  lastIndexed: Date;
}

export interface SymbolInfo {
  name: string;
  type: string;
  uri: string;
  range: any;
  container?: string;
}

export interface ReferenceInfo {
  symbol: string;
  uri: string;
  range: any;
  context: string;
}

export interface WorkspaceConfiguration {
  enabled: boolean;
  autoIndex: boolean;
  crossDocumentValidation: boolean;
  symbolSearch: boolean;
  dependencyTracking: boolean;
  filePatterns: string[];
  excludePatterns: string[];
  maxFileSize: number;
  indexInterval: number;
}

export interface WorkspaceStats {
  totalFiles: number;
  totalDocuments: number;
  totalSymbols: number;
  totalReferences: number;
  averageFileSize: number;
  lastIndexed: Date;
  indexingDuration: number;
}

export class RhemaWorkspaceManager {
  private workspaceFolders: WorkspaceFolder[] = [];
  private workspaceIndex: WorkspaceIndex;
  private configuration: WorkspaceConfiguration;
  private indexingInProgress: boolean = false;
  private fileWatchers: Map<string, any> = new Map();

  constructor() {
    this.workspaceIndex = {
      files: new Map(),
      documents: new Map(),
      symbols: new Map(),
      references: new Map(),
      dependencies: new Map(),
      lastIndexed: new Date(),
    };

    this.configuration = this.getDefaultConfiguration();
  }

  initialize(workspaceFolders: WorkspaceFolder[]): void {
    this.workspaceFolders = workspaceFolders;
    this.startFileWatching();
    this.indexWorkspace();
  }

  setConfiguration(config: Partial<WorkspaceConfiguration>): void {
    this.configuration = { ...this.configuration, ...config };
  }

  getConfiguration(): WorkspaceConfiguration {
    return this.configuration;
  }

  async indexWorkspace(): Promise<void> {
    if (this.indexingInProgress) {
      return;
    }

    this.indexingInProgress = true;
    const startTime = Date.now();

    try {
      console.log('Starting workspace indexing...');

      // Clear existing index
      this.workspaceIndex.files.clear();
      this.workspaceIndex.documents.clear();
      this.workspaceIndex.symbols.clear();
      this.workspaceIndex.references.clear();
      this.workspaceIndex.dependencies.clear();

      // Index each workspace folder
      for (const folder of this.workspaceFolders) {
        await this.indexWorkspaceFolder(folder);
      }

      // Build cross-document references
      if (this.configuration.dependencyTracking) {
        this.buildCrossDocumentReferences();
      }

      this.workspaceIndex.lastIndexed = new Date();
      const duration = Date.now() - startTime;

      console.log(`Workspace indexing completed in ${duration}ms`);
      console.log(
        `Indexed ${this.workspaceIndex.files.size} files, ${this.workspaceIndex.documents.size} documents`
      );
    } catch (error) {
      console.error('Error indexing workspace:', error);
    } finally {
      this.indexingInProgress = false;
    }
  }

  private async indexWorkspaceFolder(folder: WorkspaceFolder): Promise<void> {
    const folderPath = folder.uri.replace('file://', '');

    // Find all Rhema files in the workspace
    const patterns = this.configuration.filePatterns.map((pattern) =>
      path.join(folderPath, '**', pattern)
    );

    for (const pattern of patterns) {
      try {
        const files = await this.findFiles(pattern);

        for (const filePath of files) {
          await this.indexFile(filePath, folder);
        }
      } catch (error) {
        console.error(`Error indexing pattern ${pattern}:`, error);
      }
    }
  }

  private findFiles(pattern: string): Promise<string[]> {
    return glob(pattern, { ignore: this.configuration.excludePatterns });
  }

  private async indexFile(filePath: string, folder: WorkspaceFolder): Promise<void> {
    try {
      const stats = await fs.stat(filePath);

      // Check file size limit
      if (stats.size > this.configuration.maxFileSize) {
        console.warn(`File ${filePath} exceeds size limit, skipping`);
        return;
      }

      const uri = `file://${filePath}`;
      const fileName = path.basename(filePath);
      const fileType = this.determineFileType(fileName);

      const workspaceFile: WorkspaceFile = {
        uri,
        path: filePath,
        name: fileName,
        type: fileType,
        lastModified: stats.mtime,
        size: stats.size,
      };

      // Parse and index the document
      if (fileType !== 'unknown') {
        const content = await fs.readFile(filePath, 'utf-8');
        const document = this.parseDocument(content, fileType);

        if (document) {
          workspaceFile.content = document;
          this.workspaceIndex.documents.set(uri, document);
          this.indexDocumentSymbols(uri, document);
        }
      }

      this.workspaceIndex.files.set(uri, workspaceFile);
    } catch (error) {
      console.error(`Error indexing file ${filePath}:`, error);
    }
  }

  private determineFileType(fileName: string): WorkspaceFile['type'] {
    const lowerFileName = fileName.toLowerCase();

    if (lowerFileName.includes('scope') || lowerFileName.endsWith('.rhema.yml')) {
      return 'scope';
    } else if (lowerFileName.includes('knowledge')) {
      return 'knowledge';
    } else if (lowerFileName.includes('todos')) {
      return 'todos';
    } else if (lowerFileName.includes('decisions')) {
      return 'decisions';
    } else if (lowerFileName.includes('patterns')) {
      return 'patterns';
    } else if (lowerFileName.includes('conventions')) {
      return 'conventions';
    }

    return 'unknown';
  }

  private parseDocument(content: string, type: WorkspaceFile['type']): RhemaDocument | null {
    try {
      // This would use the RhemaParser to parse the document
      // For now, we'll create a simple document structure
      const document: RhemaDocument = {
        type: type as RhemaDocument['type'],
        content: this.parseYamlContent(content),
        metadata: {
          version: '1.0.0',
          created: new Date().toISOString(),
          modified: new Date().toISOString(),
        },
      };

      return document;
    } catch (error) {
      console.error('Error parsing document:', error);
      return null;
    }
  }

  private parseYamlContent(content: string): any {
    // Simple YAML parsing - in a real implementation, you'd use a proper YAML parser
    try {
      // This is a simplified parser for demonstration
      const lines = content.split('\n');
      const result: any = {};
      let currentKey = '';
      let currentValue = '';

      for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith('#')) continue;

        const colonIndex = trimmed.indexOf(':');
        if (colonIndex > 0) {
          if (currentKey) {
            result[currentKey] = currentValue.trim();
          }
          currentKey = trimmed.substring(0, colonIndex);
          currentValue = trimmed.substring(colonIndex + 1);
        } else if (currentKey) {
          currentValue += ' ' + trimmed;
        }
      }

      if (currentKey) {
        result[currentKey] = currentValue.trim();
      }

      return result;
    } catch (error) {
      return {};
    }
  }

  private indexDocumentSymbols(uri: string, document: RhemaDocument): void {
    const symbols: SymbolInfo[] = [];

    if (document.content) {
      // Index top-level keys as symbols
      Object.keys(document.content).forEach((key) => {
        symbols.push({
          name: key,
          type: 'field',
          uri,
          range: { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } },
        });
      });
    }

    this.workspaceIndex.symbols.set(uri, symbols);
  }

  private buildCrossDocumentReferences(): void {
    // Build dependency graph and cross-document references
    this.workspaceIndex.documents.forEach((document, uri) => {
      if (document.content.dependencies) {
        const deps: string[] = [];
        document.content.dependencies.forEach((dep: any) => {
          if (dep.name) {
            deps.push(dep.name);
          }
        });
        this.workspaceIndex.dependencies.set(uri, deps);
      }
    });
  }

  // --- Workspace Query Methods ---

  getWorkspaceFiles(): WorkspaceFile[] {
    return Array.from(this.workspaceIndex.files.values());
  }

  getWorkspaceDocuments(): RhemaDocument[] {
    return Array.from(this.workspaceIndex.documents.values());
  }

  findSymbols(query: string): SymbolInfo[] {
    const results: SymbolInfo[] = [];

    this.workspaceIndex.symbols.forEach((symbols, uri) => {
      symbols.forEach((symbol) => {
        if (symbol.name.toLowerCase().includes(query.toLowerCase())) {
          results.push(symbol);
        }
      });
    });

    return results;
  }

  findReferences(symbol: string): ReferenceInfo[] {
    const results: ReferenceInfo[] = [];

    // Search for references to the symbol across all documents
    this.workspaceIndex.documents.forEach((document, uri) => {
      if (document.content) {
        const contentStr = JSON.stringify(document.content);
        if (contentStr.includes(symbol)) {
          results.push({
            symbol,
            uri,
            range: { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } },
            context: 'document',
          });
        }
      }
    });

    return results;
  }

  getDependencies(uri: string): string[] {
    return this.workspaceIndex.dependencies.get(uri) || [];
  }

  getDependents(uri: string): string[] {
    const dependents: string[] = [];

    this.workspaceIndex.dependencies.forEach((deps, docUri) => {
      if (deps.includes(uri)) {
        dependents.push(docUri);
      }
    });

    return dependents;
  }

  // --- File Watching ---

  private startFileWatching(): void {
    // In a real implementation, you would set up file watchers
    // for each workspace folder to monitor file changes
    console.log('File watching started');
  }

  onFileChanged(event: FileEvent): void {
    const { uri, type } = event;

    switch (type) {
      case FileChangeType.Created:
        this.handleFileCreated(uri);
        break;
      case FileChangeType.Changed:
        this.handleFileChanged(uri);
        break;
      case FileChangeType.Deleted:
        this.handleFileDeleted(uri);
        break;
    }
  }

  private async handleFileCreated(uri: string): Promise<void> {
    console.log(`File created: ${uri}`);
    // Re-index the file
    const folder = this.workspaceFolders.find((f) => uri.startsWith(f.uri));
    if (folder) {
      const filePath = uri.replace('file://', '');
      await this.indexFile(filePath, folder);
    }
  }

  private async handleFileChanged(uri: string): Promise<void> {
    console.log(`File changed: ${uri}`);
    // Re-index the file
    const folder = this.workspaceFolders.find((f) => uri.startsWith(f.uri));
    if (folder) {
      const filePath = uri.replace('file://', '');
      await this.indexFile(filePath, folder);
    }
  }

  private handleFileDeleted(uri: string): void {
    console.log(`File deleted: ${uri}`);
    // Remove from index
    this.workspaceIndex.files.delete(uri);
    this.workspaceIndex.documents.delete(uri);
    this.workspaceIndex.symbols.delete(uri);
    this.workspaceIndex.dependencies.delete(uri);
  }

  // --- Statistics and Monitoring ---

  getWorkspaceStats(): WorkspaceStats {
    const totalFiles = this.workspaceIndex.files.size;
    const totalDocuments = this.workspaceIndex.documents.size;
    const totalSymbols = Array.from(this.workspaceIndex.symbols.values()).reduce(
      (sum, symbols) => sum + symbols.length,
      0
    );
    const totalReferences = Array.from(this.workspaceIndex.references.values()).reduce(
      (sum, refs) => sum + refs.length,
      0
    );

    const totalSize = Array.from(this.workspaceIndex.files.values()).reduce(
      (sum, file) => sum + file.size,
      0
    );
    const averageFileSize = totalFiles > 0 ? totalSize / totalFiles : 0;

    return {
      totalFiles,
      totalDocuments,
      totalSymbols,
      totalReferences,
      averageFileSize,
      lastIndexed: this.workspaceIndex.lastIndexed,
      indexingDuration: 0, // Would be calculated during indexing
    };
  }

  isIndexing(): boolean {
    return this.indexingInProgress;
  }

  // --- Configuration ---

  private getDefaultConfiguration(): WorkspaceConfiguration {
    return {
      enabled: true,
      autoIndex: true,
      crossDocumentValidation: true,
      symbolSearch: true,
      dependencyTracking: true,
      filePatterns: [
        '*.rhema.yml',
        'scope.yml',
        'knowledge.yml',
        'todos.yml',
        'decisions.yml',
        'patterns.yml',
        'conventions.yml',
      ],
      excludePatterns: ['**/node_modules/**', '**/.git/**', '**/target/**', '**/dist/**'],
      maxFileSize: 1024 * 1024, // 1MB
      indexInterval: 300000, // 5 minutes
    };
  }
}
