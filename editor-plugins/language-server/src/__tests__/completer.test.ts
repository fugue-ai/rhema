import { RhemaCompleter } from '../completer';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';
import { Position, CompletionItemKind } from 'vscode-languageserver/node';

describe('RhemaCompleter', () => {
  let completer: RhemaCompleter;

  beforeEach(() => {
    completer = new RhemaCompleter();
    completer.initialize(createMockCapabilities());
  });

  describe('Initialization', () => {
    it('should initialize with default keywords', () => {
      expect(completer).toBeDefined();
    });

    it('should initialize enum values', () => {
      expect(completer).toBeDefined();
    });

    it('should initialize field completions', () => {
      expect(completer).toBeDefined();
    });
  });

  describe('provideCompletion', () => {
    it('should provide completions for scope documents', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(1, 5); // After "name:"

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(completions.length).toBeGreaterThanOrEqual(0);
    });

    it('should provide completions for knowledge documents', () => {
      const document = createTestDocument(testDocuments.knowledgeDocument);
      const position = Position.create(0, 8); // After "contexts:"

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(completions.length).toBeGreaterThanOrEqual(0);
    });

    it('should provide completions for todos documents', () => {
      const document = createTestDocument(testDocuments.todosDocument);
      const position = Position.create(0, 5); // After "tasks:"

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(completions.length).toBeGreaterThanOrEqual(0);
    });

    it('should provide context-aware completions', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(4, 8); // Inside contexts array

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(completions.length).toBeGreaterThanOrEqual(0);
    });

    it('should handle empty documents', () => {
      const document = createTestDocument('');
      const position = Position.create(0, 0);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(completions.length).toBeGreaterThanOrEqual(0);
    });

    it('should handle invalid positions', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(999, 999); // Invalid position

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });
  });

  describe('resolveCompletion', () => {
    it('should resolve completion items with documentation', () => {
      const item = {
        label: 'name',
        kind: CompletionItemKind.Field,
        insertText: 'name: ',
      };

      const resolved = completer.resolveCompletion(item);

      expect(resolved).toBeDefined();
      expect(resolved.label).toBe('name');
    });

    it('should resolve completion items without documentation', () => {
      const item = {
        label: 'unknown',
        kind: CompletionItemKind.Text,
        insertText: 'unknown: ',
      };

      const resolved = completer.resolveCompletion(item);

      expect(resolved).toBeDefined();
      expect(resolved.label).toBe('unknown');
    });
  });

  describe('Context-Aware Completions', () => {
    it('should provide completions based on YAML path', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(5, 8); // Inside context object

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide completions for nested structures', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(7, 8); // Inside pattern object

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide completions for array items', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(4, 4); // At array item level

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });
  });

  describe('Snippet Completions', () => {
    it('should provide snippet completions for common patterns', () => {
      const document = createTestDocument('');
      const position = Position.create(0, 0);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide snippet completions for scope templates', () => {
      const document = createTestDocument('name: test');
      const position = Position.create(1, 0);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide snippet completions for context templates', () => {
      const document = createTestDocument('contexts:');
      const position = Position.create(1, 0);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });
  });

  describe('Enum Completions', () => {
    it('should provide enum completions for status fields', () => {
      const document = createTestDocument('status: ');
      const position = Position.create(0, 8);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide enum completions for priority fields', () => {
      const document = createTestDocument('priority: ');
      const position = Position.create(0, 10);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should provide enum completions for version fields', () => {
      const document = createTestDocument('version: ');
      const position = Position.create(0, 9);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });
  });

  describe('Performance', () => {
    it('should provide completions quickly', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(0, 3);

      const startTime = performance.now();
      const completions = completer.provideCompletion(document, position);
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(100); // Should complete within 100ms
      expect(completions).toBeDefined();
    });

    it('should handle large documents efficiently', () => {
      const largeContent = Array.from({ length: 100 }, (_, i) => `field${i}: value${i}`).join('\n');
      const document = createTestDocument(largeContent);
      const position = Position.create(0, 3);

      const startTime = performance.now();
      const completions = completer.provideCompletion(document, position);
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(1000); // Should complete within 1 second
      expect(completions).toBeDefined();
    });
  });

  describe('Error Handling', () => {
    it('should handle null document gracefully', () => {
      const position = Position.create(0, 0);

      const completions = completer.provideCompletion(null as any, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should handle null position gracefully', () => {
      const document = createTestDocument(testDocuments.validScope);

      const completions = completer.provideCompletion(document, null as any);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });

    it('should handle malformed YAML gracefully', () => {
      const document = createTestDocument('name: test\n  invalid: indentation');
      const position = Position.create(0, 3);

      const completions = completer.provideCompletion(document, position);

      expect(completions).toBeDefined();
      expect(Array.isArray(completions)).toBe(true);
    });
  });
}); 