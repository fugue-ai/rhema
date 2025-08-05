import { jest, describe, it, beforeEach, afterEach } from '@jest/globals';
import assert from 'assert';
import { RhemaCompleter } from '../completer';
import { RhemaValidator } from '../validator';
import { RhemaHoverProvider } from '../hover';
import { RhemaFormatter } from '../formatter';
import { RhemaCodeActionProvider } from '../codeAction';
import { RhemaDefinitionProvider } from '../definition';
import { RhemaReferenceProvider } from '../reference';
import { RhemaSemanticTokensProvider } from '../semanticTokens';
import { RhemaWorkspaceManager } from '../workspaceManager';
import { RhemaCache } from '../cache';
import { RhemaConfigurationManager } from '../configuration';
import { RhemaParser } from '../parser';
import type { TextDocument, Position, Range, CompletionContext, ReferenceContext, Diagnostic } from 'vscode-languageserver/node';
import { DiagnosticSeverity } from 'vscode-languageserver/node';
import type { RhemaDocument } from '../parser';

describe('Language Server Integration Tests', () => {
  let completer: RhemaCompleter;
  let validator: RhemaValidator;
  let hoverProvider: RhemaHoverProvider;
  let formatter: RhemaFormatter;
  let codeActionProvider: RhemaCodeActionProvider;
  let definitionProvider: RhemaDefinitionProvider;
  let referenceProvider: RhemaReferenceProvider;
  let semanticTokensProvider: RhemaSemanticTokensProvider;
  let workspaceManager: RhemaWorkspaceManager;
  let cache: RhemaCache;
  let configManager: RhemaConfigurationManager;
  let parser: RhemaParser;

  const testDocument: TextDocument = {
    uri: 'file:///test/test.rhema.yml',
    languageId: 'yaml',
    version: 1,
    lineCount: 30,
    getText(): string {
      return `scope:
  name: "Test Scope"
  description: "A test scope for integration testing"
  version: "1.0.0"
  author: "Test Author"

context:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  patterns:
    - "*.rs"
  exclusions:
    - "target/**"
  maxTokens: 1000
  includeHidden: false
  recursive: true

todos:
  - id: "TODO-001"
    title: "Implement integration tests"
    description: "Add comprehensive integration tests"
    priority: "high"
    status: "pending"
    assignee: "developer"
    dueDate: "2024-12-31"
    tags: ["testing", "integration"]

insights:
  - id: "INSIGHT-001"
    title: "Integration Test Analysis"
    description: "Analysis of integration test coverage"
    type: "analysis"
    confidence: 0.85
    source: "static-analysis"
    tags: ["quality", "analysis"]`;
    },
    positionAt(offset: number) {
      return { line: 0, character: offset };
    },
    offsetAt(position: Position) {
      return position.line * 100 + position.character;
    },
  };

  beforeEach(async () => {
    // Initialize all components
    cache = new RhemaCache();
    configManager = new RhemaConfigurationManager();
    parser = new RhemaParser();
    workspaceManager = new RhemaWorkspaceManager();
    completer = new RhemaCompleter();
    validator = new RhemaValidator();
    hoverProvider = new RhemaHoverProvider();
    formatter = new RhemaFormatter();
    codeActionProvider = new RhemaCodeActionProvider();
    definitionProvider = new RhemaDefinitionProvider();
    referenceProvider = new RhemaReferenceProvider();
    semanticTokensProvider = new RhemaSemanticTokensProvider();

    // Initialize components
    completer.initialize({});
    validator.initialize({}, false);
    hoverProvider.initialize({});
    formatter.initialize({});
    codeActionProvider.initialize({}, false);
    definitionProvider.initialize({});
    referenceProvider.initialize({});
    semanticTokensProvider.initialize({}, false);

    // Initialize workspace
    await workspaceManager.initialize([{ uri: 'file:///test', name: 'test' }]);
  });

  afterEach(async () => {
    // Cleanup
    cache.clear();
    
    // Wait for any pending async operations to complete
    await new Promise(resolve => setTimeout(resolve, 100));
  });

  describe('VS Code Integration Testing', () => {
    it('should provide completions in VS Code context', async () => {
      const position: Position = { line: 1, character: 6 }; // After "name:"
      
      const completions = await completer.provideCompletion(testDocument, position);
      
      assert(completions && completions.length > 0, 'Should provide completions');
      assert(completions.some(c => c.label === 'description'), 'Should include common fields');
    });

    it('should provide hover information in VS Code', async () => {
      const position: Position = { line: 1, character: 6 }; // On "name" field
      
      const hover = await hoverProvider.provideHover(testDocument, position);
      
      assert(hover, 'Should provide hover information');
      if (hover && Array.isArray(hover.contents)) {
        assert(hover.contents.length > 0, 'Hover should have content');
      }
    });

    it('should validate documents in real-time', async () => {
      const invalidDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `scope:
  name: "Test"
  invalid_field: "This should cause validation error"`;
        },
      };

      const parseResult = parser.parse(invalidDocument.getText(), invalidDocument.uri);
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, invalidDocument.uri);
        
        assert(diagnostics.diagnostics.length > 0, 'Should detect validation errors');
        assert(diagnostics.diagnostics.some(d => d.message.includes('invalid_field')), 'Should flag invalid fields');
      }
    });

    it('should format documents on save', async () => {
      const unformattedDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `scope:
name:"Test"
description:"A test"`;
        },
      };

      const edits = await formatter.formatDocument(unformattedDocument, { tabSize: 2, insertSpaces: true });
      
      assert(edits.length > 0, 'Should provide formatting edits');
    });

    it('should provide code actions for quick fixes', async () => {
      const range: Range = { start: { line: 1, character: 0 }, end: { line: 1, character: 10 } };
      const context = {
        diagnostics: [
          {
            range,
            message: 'Missing required field',
            severity: DiagnosticSeverity.Error,
          } as Diagnostic,
        ],
        only: ['quickfix'],
      };

      const actions = await codeActionProvider.provideCodeActions(testDocument, range, context);
      
      assert(actions && actions.length > 0, 'Should provide code actions');
    });

    it('should provide go-to-definition functionality', async () => {
      const position: Position = { line: 1, character: 6 }; // On "name" field
      
      const definition = await definitionProvider.provideDefinition(testDocument, position);
      
      // For YAML fields, definition might be the same position or null
      assert(definition !== undefined, 'Should provide definition or null');
    });

    it('should find all references', async () => {
      const position: Position = { line: 1, character: 6 }; // On "name" field
      const context: ReferenceContext = { includeDeclaration: true };
      
      const references = await referenceProvider.provideReferences(testDocument, position, context);
      
      assert(Array.isArray(references), 'Should return array of references');
    });

    it('should provide semantic tokens for syntax highlighting', async () => {
      const range: Range = { start: { line: 0, character: 0 }, end: { line: 10, character: 0 } };
      
      const tokens = await semanticTokensProvider.provideSemanticTokens(testDocument, range);
      
      assert(tokens, 'Should provide semantic tokens');
      assert(Array.isArray(tokens.data), 'Tokens should be an array');
    });
  });

  describe('IntelliSense and Completion Features', () => {
    it('should provide context-aware completions', async () => {
      // Test completion at different positions
      const positions: Position[] = [
        { line: 0, character: 7 }, // After "scope:"
        { line: 1, character: 8 }, // After "name:"
        { line: 2, character: 15 }, // After "description:"
      ];

      for (const position of positions) {
        const completions = await completer.provideCompletion(testDocument, position);
        assert(completions && completions.length > 0, `Should provide completions at position ${position.line}:${position.character}`);
      }
    });

    it('should provide snippet completions', async () => {
      const position: Position = { line: 0, character: 0 };
      
      const completions = await completer.provideCompletion(testDocument, position);
      
      const snippetCompletions = completions?.filter(c => c.insertTextFormat === 2); // Snippet format
      assert(snippetCompletions && snippetCompletions.length > 0, 'Should provide snippet completions');
    });

    it('should provide enum completions for status fields', async () => {
      const statusDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `todos:
  - id: "TODO-001"
    status: ""`;
        },
      };
      
      const position: Position = { line: 2, character: 10 }; // After "status:"
      const completions = await completer.provideCompletion(statusDocument, position);
      
      const statusValues = completions?.filter(c => ['pending', 'in-progress', 'completed'].includes(c.label));
      assert(statusValues && statusValues.length > 0, 'Should provide status enum values');
    });
  });

  describe('Validation and Error Reporting', () => {
    it('should validate YAML structure', async () => {
      const malformedDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `scope:
  name: "Test"
  description: "Missing closing quote
  version: "1.0.0"`;
        },
      };

      const parseResult = parser.parse(malformedDocument.getText(), malformedDocument.uri);
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, malformedDocument.uri);
        
        assert(diagnostics.diagnostics.length > 0, 'Should detect YAML syntax errors');
        assert(diagnostics.diagnostics.some(d => d.message.includes('YAML')), 'Should include YAML parsing errors');
      }
    });

    it('should validate schema compliance', async () => {
      const invalidSchemaDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `scope:
  invalid_field: "This field doesn't exist in schema"
  name: 123 # Should be string`;
        },
      };

      const parseResult = parser.parse(invalidSchemaDocument.getText(), invalidSchemaDocument.uri);
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, invalidSchemaDocument.uri);
        
        assert(diagnostics.diagnostics.length > 0, 'Should detect schema violations');
        assert(diagnostics.diagnostics.some(d => d.message.includes('invalid_field')), 'Should flag invalid fields');
        assert(diagnostics.diagnostics.some(d => d.message.includes('string')), 'Should flag type errors');
      }
    });

    it('should provide quick fixes for common errors', async () => {
      const errorDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `scope:
  name: "Test"
  verson: "1.0.0" # Typo in version`;
        },
      };

      const parseResult = parser.parse(errorDocument.getText(), errorDocument.uri);
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, errorDocument.uri);
        const range: Range = { start: { line: 2, character: 0 }, end: { line: 2, character: 20 } };
        const context = { diagnostics: diagnostics.diagnostics, only: ['quickfix'] };

        const actions = await codeActionProvider.provideCodeActions(errorDocument, range, context);
        
        assert(actions && actions.length > 0, 'Should provide quick fix actions');
        const fixAction = actions.find(a => a.title.includes('version'));
        assert(fixAction, 'Should provide fix for typo');
      }
    });
  });

  describe('Performance and Caching', () => {
    it('should cache validation results', async () => {
      const parseResult = parser.parse(testDocument.getText(), testDocument.uri);
      if (parseResult.success && parseResult.data) {
        const startTime = Date.now();
        
        // First validation
        await validator.validate(parseResult.data, testDocument.uri);
        const firstValidationTime = Date.now() - startTime;
        
        // Second validation (should use cache)
        const cacheStartTime = Date.now();
        await validator.validate(parseResult.data, testDocument.uri);
        const cachedValidationTime = Date.now() - cacheStartTime;
        
        assert(cachedValidationTime < firstValidationTime, 'Cached validation should be faster');
      }
    });

    it('should provide completions quickly', async () => {
      const position: Position = { line: 1, character: 6 };
      const startTime = Date.now();
      
      await completer.provideCompletion(testDocument, position);
      const completionTime = Date.now() - startTime;
      
      assert(completionTime < 50, 'Completion should be provided in under 50ms');
    });

    it('should handle large documents efficiently', async () => {
      const largeDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          let content = 'scope:\n  name: "Large Test"\n';
          // Add many todos to create a large document
          for (let i = 0; i < 100; i++) {
            content += `todos:\n  - id: "TODO-${i}"\n    title: "Todo ${i}"\n    status: "pending"\n`;
          }
          return content;
        },
      };

      const parseResult = parser.parse(largeDocument.getText(), largeDocument.uri);
      if (parseResult.success && parseResult.data) {
        const startTime = Date.now();
        await validator.validate(parseResult.data, largeDocument.uri);
        const validationTime = Date.now() - startTime;
        
        assert(validationTime < 1000, 'Large document validation should complete in under 1 second');
      }
    });
  });

  describe('Workspace Integration', () => {
    it('should index workspace files', async () => {
      const stats = workspaceManager.getWorkspaceStats();
      
      assert(stats, 'Should provide workspace statistics');
      assert(typeof stats.totalFiles === 'number', 'Should track total files');
    });

    it('should provide cross-file references', async () => {
      // This would require multiple documents in the workspace
      // For now, test that the functionality exists
      const position: Position = { line: 1, character: 6 };
      const context: ReferenceContext = { includeDeclaration: true };
      const references = await referenceProvider.provideReferences(testDocument, position, context);
      assert(Array.isArray(references), 'Should return references array');
    });
  });

  describe('Configuration Management', () => {
    it('should respect configuration settings', async () => {
      const config = {
        validation: { enabled: false },
        completion: { enabled: true },
        formatting: { enabled: true },
      };

      configManager.updateConfiguration(config);
      
      const currentConfig = configManager.getConfiguration();
      assert.strictEqual(currentConfig.validation.enabled, false, 'Should respect validation setting');
      assert.strictEqual(currentConfig.completion.enabled, true, 'Should respect completion setting');
    });

    it('should provide document-specific configuration', async () => {
      const docConfig = configManager.getDocumentConfiguration(testDocument.uri);
      assert(docConfig, 'Should provide document configuration');
    });
  });

  describe('Error Handling and Recovery', () => {
    it('should handle malformed documents gracefully', async () => {
      const malformedDocument: TextDocument = {
        ...testDocument,
        getText(): string {
          return `invalid yaml content: [`;
        },
      };

      // Should not throw errors
      const parseResult = parser.parse(malformedDocument.getText(), malformedDocument.uri);
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, malformedDocument.uri);
        const completions = await completer.provideCompletion(malformedDocument, { line: 0, character: 0 });
        
        assert(Array.isArray(diagnostics.diagnostics), 'Should return diagnostics array even for malformed content');
        assert(Array.isArray(completions), 'Should return completions array even for malformed content');
      }
    });

    it('should handle null inputs gracefully', async () => {
      // Test with null document
      const completions = await completer.provideCompletion(null as any, { line: 0, character: 0 });
      assert(Array.isArray(completions), 'Should handle null document gracefully');
      
      const parseResult = parser.parse('', '');
      if (parseResult.success && parseResult.data) {
        const diagnostics = await validator.validate(parseResult.data, '');
        assert(Array.isArray(diagnostics.diagnostics), 'Should handle empty document gracefully');
      }
    });
  });
}); 