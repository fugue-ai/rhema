# Rhema Language Server API Documentation

## Overview

The Rhema Language Server provides comprehensive language support for Rhema YAML files, including IntelliSense, validation, formatting, and advanced features. This document provides detailed API documentation for all components.

## Table of Contents

1. [Core Components](#core-components)
2. [Language Features](#language-features)
3. [Configuration](#configuration)
4. [Performance & Caching](#performance--caching)
5. [Integration Guide](#integration-guide)
6. [Examples](#examples)

## Core Components

### RhemaLanguageServer

The main language server class that orchestrates all language features.

```typescript
class RhemaLanguageServer {
  constructor(
    completer: RhemaCompleter,
    validator: RhemaValidator,
    hoverProvider: RhemaHoverProvider,
    formatter: RhemaFormatter,
    codeActionProvider: RhemaCodeActionProvider,
    definitionProvider: RhemaDefinitionProvider,
    referenceProvider: RhemaReferenceProvider,
    semanticTokensProvider: RhemaSemanticTokensProvider,
    workspaceManager: RhemaWorkspaceManager,
    cache: RhemaCache,
    configManager: RhemaConfigurationManager
  );

  // Initialize the language server
  initialize(params: InitializeParams): InitializeResult;

  // Handle document changes
  handleDocumentChange(change: DidChangeTextDocumentParams): void;

  // Handle document saves
  handleDocumentSave(save: DidSaveTextDocumentParams): void;

  // Shutdown the language server
  shutdown(): void;
}
```

### RhemaCompleter

Provides intelligent code completion for Rhema YAML files.

```typescript
class RhemaCompleter {
  // Initialize with capabilities
  initialize(capabilities: any): void;

  // Provide completions at a specific position
  provideCompletion(
    document: TextDocument,
    position: Position,
    cachedDocument?: any
  ): CompletionItem[];

  // Resolve additional information for a completion item
  resolveCompletion(item: CompletionItem): CompletionItem;
}
```

**Features:**
- Context-aware completions based on YAML path
- Snippet completions for common patterns
- Enum value completions
- Fuzzy matching with semantic keyword matching
- Intelligent ranking and filtering

### RhemaValidator

Validates Rhema YAML files against schemas and custom rules.

```typescript
class RhemaValidator {
  // Initialize with capabilities
  initialize(capabilities: any, hasDiagnosticRelatedInformation: boolean): void;

  // Validate a document
  validate(
    document: RhemaDocument,
    uri: string,
    workspaceDocuments?: RhemaDocument[]
  ): ValidationResult;

  // Add custom validation rules
  addCustomRule(rule: ValidationRule): void;

  // Remove custom validation rules
  removeCustomRule(ruleId: string): void;

  // Clear validation cache
  clearValidationCache(): void;
}
```

**Validation Types:**
- Schema validation against JSON schemas
- Custom rule validation
- Cross-document validation
- Performance validation
- Style and best practice validation

### RhemaHoverProvider

Provides hover information for Rhema YAML elements.

```typescript
class RhemaHoverProvider {
  // Initialize with capabilities
  initialize(capabilities: any): void;

  // Provide hover information at a position
  provideHover(document: TextDocument, position: Position): Hover | null;
}
```

### RhemaFormatter

Formats Rhema YAML files according to style guidelines.

```typescript
class RhemaFormatter {
  // Initialize with capabilities
  initialize(capabilities: any): void;

  // Format entire document
  formatDocument(
    document: TextDocument,
    options: FormattingOptions
  ): TextEdit[];

  // Format document range
  formatDocumentRange(
    document: TextDocument,
    range: Range,
    options: FormattingOptions
  ): TextEdit[];

  // Provide folding ranges
  provideFoldingRanges(document: TextDocument): FoldingRange[];
}
```

### RhemaCodeActionProvider

Provides code actions for quick fixes and refactoring.

```typescript
class RhemaCodeActionProvider {
  // Initialize with capabilities
  initialize(capabilities: any, hasCodeActionLiteralSupport: boolean): void;

  // Provide code actions
  provideCodeActions(
    document: TextDocument,
    range: Range,
    context: CodeActionContext
  ): CodeAction[];

  // Execute a command
  executeCommand(command: Command): Promise<any>;
}
```

## Language Features

### Document Types

The language server supports the following Rhema document types:

1. **Scope Documents** (`.rhema.yml`)
   - Project scope definitions
   - Technology stack specifications
   - Context and patterns

2. **Knowledge Documents** (`knowledge.yml`)
   - Context information
   - File patterns and exclusions
   - Knowledge base entries

3. **Todos Documents** (`todos.yml`)
   - Task management
   - Status tracking
   - Priority and assignment

4. **Decisions Documents** (`decisions.yml`)
   - Decision records
   - Rationale and alternatives
   - Impact analysis

5. **Patterns Documents** (`patterns.yml`)
   - Code patterns
   - Conventions
   - Best practices

### YAML Path Detection

The language server provides intelligent YAML path detection:

```typescript
// Example YAML path detection
const path = completer.getYamlPath(document, position);
// Returns: ['scope', 'tech', 'primary_languages']
```

**Features:**
- Handles nested structures
- Supports array items
- Recognizes quoted keys
- Context-aware path resolution

### Completion Context

Completion context includes:

```typescript
interface CompletionContext {
  document: RhemaDocument;
  position: Position;
  triggerCharacter?: string;
  yamlPath: string[];
  documentType?: string;
  currentLine: string;
  beforeCursor: string;
  afterCursor: string;
}
```

## Configuration

### Configuration Options

```typescript
interface RhemaConfiguration {
  // Validation settings
  validation: {
    enabled: boolean;
    strict: boolean;
    schemaValidation: boolean;
    customRules: boolean;
    crossDocumentValidation: boolean;
    performanceOptimization: boolean;
  };

  // Completion settings
  completion: {
    enabled: boolean;
    fuzzyMatching: boolean;
    semanticMatching: boolean;
    snippetCompletions: boolean;
    aiCompletions: boolean;
  };

  // Formatting settings
  formatting: {
    enabled: boolean;
    tabSize: number;
    insertSpaces: boolean;
    trimTrailingWhitespace: boolean;
    insertFinalNewline: boolean;
  };

  // Performance settings
  performance: {
    enableCaching: boolean;
    enableMemoryOptimization: boolean;
    enableAsyncProcessing: boolean;
    enableBatchProcessing: boolean;
    cacheSize: number;
    memoryThreshold: number;
    batchSize: number;
    maxConcurrentOperations: number;
  };
}
```

### Configuration Management

```typescript
class RhemaConfigurationManager {
  // Get current configuration
  getConfiguration(): RhemaConfiguration;

  // Update configuration
  updateConfiguration(config: Partial<RhemaConfiguration>): void;

  // Get document-specific configuration
  getDocumentConfiguration(uri: string): RhemaConfiguration;

  // Get default configuration
  getDefaultConfiguration(): RhemaConfiguration;
}
```

## Performance & Caching

### Cache Management

```typescript
class RhemaCache {
  // Set cache entry with TTL and priority
  set<T>(key: string, value: T, ttl?: number, priority?: number): void;

  // Get cache entry
  get<T>(key: string): T | null;

  // Check if key exists
  has(key: string): boolean;

  // Delete cache entry
  delete(key: string): boolean;

  // Clear all cache
  clear(): void;

  // Get cache statistics
  getStats(): CacheStats;

  // Invalidate by pattern
  invalidateByPattern(pattern: string): void;

  // Warm cache for patterns
  warmCache(patterns: string[]): void;
}
```

### Performance Optimization

```typescript
class RhemaPerformanceOptimizer {
  // Execute operation with throttling
  executeWithThrottling<T>(
    operation: () => Promise<T>,
    priority?: 'high' | 'medium' | 'low'
  ): Promise<T>;

  // Get cached result or execute operation
  getCachedResult<T>(
    key: string,
    operation: () => Promise<T>,
    ttl?: number
  ): Promise<T>;

  // Add operation to batch queue
  addToBatch(operation: BatchOperation): void;

  // Get performance metrics
  getPerformanceMetrics(): PerformanceMetrics[];
}
```

## Integration Guide

### VS Code Extension Integration

```typescript
// Example VS Code extension integration
import * as vscode from 'vscode';
import { RhemaLanguageServer } from './language-server';

export function activate(context: vscode.ExtensionContext) {
  // Create language server
  const server = new RhemaLanguageServer(/* components */);

  // Register language support
  const provider = vscode.languages.registerCompletionItemProvider(
    { language: 'yaml', pattern: '**/*.rhema.yml' },
    {
      provideCompletionItems(document, position, token, context) {
        return server.completer.provideCompletion(document, position);
      }
    }
  );

  context.subscriptions.push(provider);
}
```

### LSP Client Integration

```typescript
// Example LSP client integration
import { createConnection, TextDocuments } from 'vscode-languageserver/node';
import { RhemaLanguageServer } from './language-server';

const connection = createConnection();
const documents = new TextDocuments();

const server = new RhemaLanguageServer(/* components */);

connection.onInitialize((params) => {
  return server.initialize(params);
});

connection.onDidChangeTextDocument((params) => {
  server.handleDocumentChange(params);
});

connection.listen();
```

## Examples

### Custom Validation Rule

```typescript
const customRule: ValidationRule = {
  id: 'custom-naming-convention',
  name: 'Custom Naming Convention',
  description: 'Enforce custom naming conventions',
  category: 'style',
  severity: DiagnosticSeverity.Warning,
  enabled: true,
  validate: (document, context) => {
    const errors: ValidationError[] = [];
    
    // Check naming conventions
    if (document.content.name && !/^[A-Z][a-zA-Z0-9]*$/.test(document.content.name)) {
      errors.push({
        range: { start: { line: 0, character: 0 }, end: { line: 0, character: 10 } },
        message: 'Name should follow PascalCase convention',
        severity: DiagnosticSeverity.Warning,
      });
    }
    
    return errors;
  }
};

validator.addCustomRule(customRule);
```

### Custom Completion Provider

```typescript
// Add custom completions
const customCompletions: RhemaCompletionItem[] = [
  {
    label: 'custom-field',
    kind: CompletionItemKind.Field,
    insertText: 'custom_field: "$1"',
    insertTextFormat: InsertTextFormat.Snippet,
    detail: 'Custom field',
    documentation: 'A custom field for your Rhema document',
    category: 'field',
    priority: 1,
  }
];

completer.addCustomCompletions(customCompletions);
```

### Performance Monitoring

```typescript
// Monitor performance
const stats = cache.getStats();
console.log(`Cache hit rate: ${(stats.hitRate * 100).toFixed(2)}%`);
console.log(`Cache efficiency: ${(stats.cacheEfficiency * 100).toFixed(2)}%`);

const metrics = optimizer.getPerformanceMetrics();
metrics.forEach(metric => {
  console.log(`${metric.operation}: ${metric.duration}ms`);
});
```

## Error Handling

### Common Error Types

1. **Validation Errors**
   - Schema violations
   - Custom rule violations
   - Cross-document reference errors

2. **Performance Errors**
   - Memory threshold exceeded
   - Cache eviction failures
   - Operation timeout

3. **Integration Errors**
   - LSP protocol errors
   - Document parsing errors
   - Configuration errors

### Error Recovery

```typescript
// Example error recovery
try {
  const result = await server.validate(document, uri);
  return result;
} catch (error) {
  if (error.code === 'VALIDATION_ERROR') {
    // Handle validation errors
    return { valid: false, diagnostics: [] };
  } else if (error.code === 'PERFORMANCE_ERROR') {
    // Handle performance errors
    optimizer.optimizeMemory();
    return await server.validate(document, uri);
  } else {
    // Handle other errors
    throw error;
  }
}
```

## Best Practices

### Performance Optimization

1. **Use Caching Effectively**
   - Set appropriate TTL values
   - Use priority-based caching
   - Monitor cache hit rates

2. **Optimize Async Operations**
   - Use appropriate priority levels
   - Implement retry logic
   - Monitor operation queues

3. **Memory Management**
   - Set memory thresholds
   - Implement garbage collection
   - Monitor memory usage

### Configuration Management

1. **Document-Specific Settings**
   - Use URI-based configuration
   - Override defaults appropriately
   - Validate configuration values

2. **Workspace Settings**
   - Share configuration across documents
   - Use workspace-specific defaults
   - Implement configuration inheritance

### Error Handling

1. **Graceful Degradation**
   - Handle missing components
   - Provide fallback behavior
   - Log errors appropriately

2. **User Feedback**
   - Provide meaningful error messages
   - Suggest solutions
   - Maintain user experience

## Troubleshooting

### Common Issues

1. **High Memory Usage**
   - Check cache size settings
   - Monitor memory profiles
   - Adjust memory thresholds

2. **Slow Performance**
   - Check cache hit rates
   - Monitor operation queues
   - Optimize async operations

3. **Validation Errors**
   - Check schema definitions
   - Verify custom rules
   - Review document structure

### Debugging

```typescript
// Enable debug logging
const config = {
  logging: {
    level: 'debug',
    enablePerformanceLogging: true,
    enableCacheLogging: true,
  }
};

configManager.updateConfiguration(config);
```

## API Reference

For complete API reference, see the TypeScript definitions in the source code:

- `src/server.ts` - Main language server
- `src/completer.ts` - Completion provider
- `src/validator.ts` - Validation provider
- `src/cache.ts` - Cache management
- `src/performanceOptimizer.ts` - Performance optimization
- `src/configuration.ts` - Configuration management

## Contributing

When contributing to the language server:

1. Follow the existing code style
2. Add comprehensive tests
3. Update documentation
4. Consider performance implications
5. Handle errors gracefully

## License

This project is licensed under the Apache License 2.0. See the LICENSE file for details. 