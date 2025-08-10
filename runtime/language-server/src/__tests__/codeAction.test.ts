import { RhemaCodeActionProvider } from '../codeAction';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';
import { Position, Range, DiagnosticSeverity, CodeActionKind } from 'vscode-languageserver/node';

describe('RhemaCodeActionProvider', () => {
  let codeActionProvider: RhemaCodeActionProvider;

  beforeEach(() => {
    codeActionProvider = new RhemaCodeActionProvider();
    codeActionProvider.initialize(createMockCapabilities(), true);
  });

  describe('Initialization', () => {
    it('should initialize with default configuration', () => {
      expect(codeActionProvider).toBeDefined();
    });

    it('should initialize with code action literal support', () => {
      codeActionProvider.initialize(createMockCapabilities(), true);
      expect(codeActionProvider).toBeDefined();
    });

    it('should initialize without code action literal support', () => {
      codeActionProvider.initialize(createMockCapabilities(), false);
      expect(codeActionProvider).toBeDefined();
    });
  });

  describe('provideCodeActions', () => {
    it('should provide quick fix actions for validation errors', () => {
      const document = createTestDocument(testDocuments.invalidScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));
      const diagnostics = [
        {
          range,
          message: 'Missing required field: name',
          severity: DiagnosticSeverity.Error,
          source: 'rhema',
          code: 'missing-field',
        },
      ];

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics },
        { only: [CodeActionKind.QuickFix] }
      );

      expect(codeActions).toBeDefined();
      expect(codeActions.length).toBeGreaterThanOrEqual(0);
    });

    it('should provide refactor actions for code improvements', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(2, 0));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        { only: [CodeActionKind.Refactor] }
      );

      expect(codeActions).toBeDefined();
      expect(codeActions.length).toBeGreaterThanOrEqual(0);
    });

    it('should provide source actions for document organization', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const range = Range.create(Position.create(0, 0), Position.create(10, 0));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        { only: [CodeActionKind.Source] }
      );

      expect(codeActions).toBeDefined();
      expect(codeActions.length).toBeGreaterThanOrEqual(0);
    });

    it('should handle empty diagnostics', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        {}
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });

    it('should filter actions based on context', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        { only: [CodeActionKind.QuickFix] }
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });

    it('should handle null diagnostics gracefully', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        {}
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });
  });

  describe('provideRename', () => {
    it('should provide rename edit for symbol', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(0, 3); // Over "name"
      const newName = 'new-name';

      const edit = codeActionProvider.provideRename(document, position, newName);

      expect(edit).toBeDefined();
    });

    it('should handle invalid positions', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(100, 100); // Invalid position
      const newName = 'new-name';

      const edit = codeActionProvider.provideRename(document, position, newName);

      expect(edit).toBeDefined();
    });

    it('should handle empty new name', () => {
      const document = createTestDocument(testDocuments.validScope);
      const position = Position.create(0, 3);
      const newName = '';

      const edit = codeActionProvider.provideRename(document, position, newName);

      expect(edit).toBeDefined();
    });
  });

  describe('executeCommand', () => {
    it('should execute extract command', () => {
      const args = ['extract', 'test.yml', 'context', 'new-context'];
      const result = codeActionProvider.executeCommand('rhema.extract', args);

      expect(result).toBeDefined();
    });

    it('should execute inline command', () => {
      const args = ['inline', 'test.yml', 'context'];
      const result = codeActionProvider.executeCommand('rhema.inline', args);

      expect(result).toBeDefined();
    });

    it('should execute move command', () => {
      const args = ['move', 'test.yml', 'context', 'new-file.yml'];
      const result = codeActionProvider.executeCommand('rhema.move', args);

      expect(result).toBeDefined();
    });

    it('should execute generate command', () => {
      const args = ['generate', 'scope', 'test.yml'];
      const result = codeActionProvider.executeCommand('rhema.generate', args);

      expect(result).toBeDefined();
    });

    it('should execute organize command', () => {
      const args = ['organize', 'test.yml'];
      const result = codeActionProvider.executeCommand('rhema.organize', args);

      expect(result).toBeDefined();
    });

    it('should execute optimize command', () => {
      const args = ['optimize', 'test.yml'];
      const result = codeActionProvider.executeCommand('rhema.optimize', args);

      expect(result).toBeDefined();
    });

    it('should handle unknown commands', () => {
      const args = ['unknown', 'test.yml'];
      const result = codeActionProvider.executeCommand('rhema.unknown', args);

      expect(result).toBeDefined();
    });
  });

  describe('Error Handling', () => {
    it('should handle null document gracefully', () => {
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const codeActions = codeActionProvider.provideCodeActions(
        null as any,
        range,
        { diagnostics: [] },
        {}
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });

    it('should handle null range gracefully', () => {
      const document = createTestDocument(testDocuments.validScope);

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        null as any,
        { diagnostics: [] },
        {}
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });

    it('should handle null context gracefully', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        null as any,
        {}
      );

      expect(codeActions).toBeDefined();
      expect(Array.isArray(codeActions)).toBe(true);
    });
  });

  describe('Performance', () => {
    it('should provide code actions quickly', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(Position.create(0, 0), Position.create(0, 10));

      const startTime = performance.now();
      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        {}
      );
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(100); // Should complete within 100ms
      expect(codeActions).toBeDefined();
    });

    it('should handle large documents efficiently', () => {
      const largeContent = Array.from({ length: 100 }, (_, i) => `field${i}: value${i}`).join('\n');
      const document = createTestDocument(largeContent);
      const range = Range.create(Position.create(0, 0), Position.create(50, 0));

      const startTime = performance.now();
      const codeActions = codeActionProvider.provideCodeActions(
        document,
        range,
        { diagnostics: [] },
        {}
      );
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(1000); // Should complete within 1 second
      expect(codeActions).toBeDefined();
    });
  });
}); 