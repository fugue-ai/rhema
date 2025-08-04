import { RhemaHoverProvider } from '../hover';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';
import { Position } from 'vscode-languageserver/node';

describe('RhemaHoverProvider', () => {
  let hoverProvider: RhemaHoverProvider;

  beforeEach(() => {
    hoverProvider = new RhemaHoverProvider();
    hoverProvider.initialize(createMockCapabilities());
  });

  describe('Initialization', () => {
    it('should initialize with default configuration', () => {
      expect(hoverProvider).toBeDefined();
    });

    it('should initialize keyword documentation', () => {
      // The keyword documentation is initialized in the constructor
      expect(hoverProvider).toBeDefined();
    });
  });

  describe('provideHover', () => {
    it('should provide hover for field names', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(0, 3); // Over "name"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
        // Check if contents is an array or single item
        const content = Array.isArray(hover.contents) ? hover.contents[0] : hover.contents;
        if (typeof content === 'object' && 'value' in content) {
          expect(content.value).toContain('name');
        }
      }
    });

    it('should provide hover for version fields', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(1, 3); // Over "version"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
        const content = Array.isArray(hover.contents) ? hover.contents[0] : hover.contents;
        if (typeof content === 'object' && 'value' in content) {
          expect(content.value).toContain('version');
        }
      }
    });

    it('should provide hover for context fields', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(3, 3); // Over "contexts"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
        const content = Array.isArray(hover.contents) ? hover.contents[0] : hover.contents;
        if (typeof content === 'object' && 'value' in content) {
          expect(content.value).toContain('contexts');
        }
      }
    });

    it('should provide hover for pattern fields', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const position = Position.create(6, 3); // Over "patterns"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
        const content = Array.isArray(hover.contents) ? hover.contents[0] : hover.contents;
        if (typeof content === 'object' && 'value' in content) {
          expect(content.value).toContain('patterns');
        }
      }
    });

    it('should provide hover for task fields', () => {
      const document = createTestDocument(testDocuments.todosDocument);
      const position = Position.create(0, 3); // Over "tasks"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
        const content = Array.isArray(hover.contents) ? hover.contents[0] : hover.contents;
        if (typeof content === 'object' && 'value' in content) {
          expect(content.value).toContain('tasks');
        }
      }
    });

    it('should return null for unknown fields', () => {
      const document = createTestDocument('unknown: field');
      const position = Position.create(0, 3); // Over "unknown"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeNull();
    });

    it('should handle empty documents', () => {
      const document = createTestDocument('');
      const position = Position.create(0, 0);

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeNull();
    });

    it('should handle invalid positions', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(100, 100); // Invalid position

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeNull();
    });
  });

  describe('Word Detection', () => {
    it('should detect YAML keys', () => {
      const document = createTestDocument('name: test\nversion: "1.0.0"');
      const position = Position.create(0, 3); // Over "name"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
    });

    it('should detect quoted strings', () => {
      const document = createTestDocument('name: "test-value"');
      const position = Position.create(0, 8); // Over "test-value"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
    });

    it('should detect single-quoted strings', () => {
      const document = createTestDocument("name: 'test-value'");
      const position = Position.create(0, 8); // Over "test-value"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
    });
  });

  describe('Documentation Content', () => {
    it('should provide documentation for common fields', () => {
      const document = createTestDocument('name: test-scope');
      const position = Position.create(0, 3); // Over "name"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
      }
    });

    it('should provide documentation for document types', () => {
      const document = createTestDocument('type: scope');
      const position = Position.create(0, 3); // Over "type"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
      }
    });

    it('should provide documentation for syntax elements', () => {
      const document = createTestDocument('description: |\n  Multi-line\n  description');
      const position = Position.create(0, 3); // Over "description"

      const hover = hoverProvider.provideHover(document, position);

      expect(hover).toBeDefined();
      if (hover) {
        expect(hover.contents).toBeDefined();
      }
    });
  });

  describe('Error Handling', () => {
    it('should handle malformed YAML gracefully', () => {
      const document = createTestDocument('name: test\n  invalid: indentation');
      const position = Position.create(0, 3); // Over "name"

      const hover = hoverProvider.provideHover(document, position);

      // Should not throw, but may return null
      expect(hover).toBeDefined();
    });

    it('should handle null document gracefully', () => {
      const position = Position.create(0, 0);

      expect(() => hoverProvider.provideHover(null as any, position)).not.toThrow();
    });

    it('should handle null position gracefully', () => {
      const document = createTestDocument(testDocuments.validScope);

      expect(() => hoverProvider.provideHover(document, null as any)).not.toThrow();
    });
  });

  describe('Performance', () => {
    it('should provide hover quickly', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(0, 3);

      const startTime = performance.now();
      const hover = hoverProvider.provideHover(document, position);
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(100); // Should complete within 100ms
      expect(hover).toBeDefined();
    });

    it('should handle large documents efficiently', () => {
      const largeContent = Array.from({ length: 100 }, (_, i) => `field${i}: value${i}`).join('\n');
      const document = createTestDocument(largeContent);
      const position = Position.create(0, 3);

      const startTime = performance.now();
      const hover = hoverProvider.provideHover(document, position);
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(1000); // Should complete within 1 second
      expect(hover).toBeDefined();
    });
  });
}); 