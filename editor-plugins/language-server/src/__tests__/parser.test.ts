import { RhemaParser } from '../parser';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';

describe('RhemaParser', () => {
  let parser: RhemaParser;

  beforeEach(() => {
    parser = new RhemaParser();
    parser.initialize(createMockCapabilities());
  });

  describe('parse', () => {
    it('should parse valid YAML content', () => {
      const document = createTestDocument(testDocuments.validScope);
      const result = parser.parse(document.getText(), document.uri);

      expect(result.success).toBe(true);
      expect(result.data).toBeDefined();
      expect(result.data?.type).toBe('scope');
      expect(result.data?.content).toHaveProperty('name', 'test-scope');
      expect(result.data?.content).toHaveProperty('version', '1.0.0');
    });

    it('should handle invalid YAML content', () => {
      const invalidYaml = 'name: test\n  invalid: indentation';
      const result = parser.parse(invalidYaml, 'test.yml');

      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it('should parse complex nested structures', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const result = parser.parse(document.getText(), document.uri);

      expect(result.success).toBe(true);
      expect(result.data?.content).toHaveProperty('contexts');
      expect(Array.isArray(result.data?.content.contexts)).toBe(true);
      expect(result.data?.content.contexts).toHaveLength(2);
    });

    it('should handle empty content', () => {
      const result = parser.parse('', 'empty.yml');

      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
    });

    it('should preserve document metadata', () => {
      const document = createTestDocument(testDocuments.validScope);
      const result = parser.parse(document.getText(), document.uri);

      expect(result.data?.metadata).toBeDefined();
      expect(result.data?.metadata.version).toBeDefined();
      expect(result.data?.metadata.created).toBeDefined();
    });
  });

  describe('document type detection', () => {
    it('should identify scope documents by filename', () => {
      const result = parser.parse('name: test-scope\nversion: "1.0.0"', 'scope.yml');
      expect(result.data?.type).toBe('scope');
    });

    it('should identify knowledge documents by filename', () => {
      const result = parser.parse('contexts:\n  - name: dev', 'knowledge.yml');
      expect(result.data?.type).toBe('knowledge');
    });

    it('should identify todos documents by filename', () => {
      const result = parser.parse('tasks:\n  - name: task1', 'todos.yml');
      expect(result.data?.type).toBe('todos');
    });

    it('should identify decisions documents by filename', () => {
      const result = parser.parse('decisions:\n  - name: decision1', 'decisions.yml');
      expect(result.data?.type).toBe('decisions');
    });

    it('should identify patterns documents by filename', () => {
      const result = parser.parse('patterns:\n  - name: pattern1', 'patterns.yml');
      expect(result.data?.type).toBe('patterns');
    });

    it('should identify conventions documents by filename', () => {
      const result = parser.parse('conventions:\n  - name: convention1', 'conventions.yml');
      expect(result.data?.type).toBe('conventions');
    });
  });

  describe('validation', () => {
    it('should validate basic structure', () => {
      const document = createTestDocument(testDocuments.validScope);
      const result = parser.parse(document.getText(), document.uri);

      expect(result.success).toBe(true);
      expect(result.errors.length).toBe(0);
    });

    it('should detect missing required fields', () => {
      const invalidContent = 'version: "1.0.0"'; // Missing name
      const result = parser.parse(invalidContent, 'test.yml');

      expect(result.success).toBe(false);
      expect(result.errors.some((e) => e.message.includes('name'))).toBe(true);
    });

    it('should handle malformed YAML gracefully', () => {
      const malformedContent = 'name: test\n  invalid: indentation\n  - list: item';
      const result = parser.parse(malformedContent, 'test.yml');

      expect(result.success).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });

  describe('getDocumentSymbols', () => {
    it('should extract symbols from document', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const parsed = parser.parse(document.getText(), document.uri);
      
      if (parsed.success && parsed.data) {
        const symbols = parser.getDocumentSymbols(parsed.data);
        expect(Array.isArray(symbols)).toBe(true);
        expect(symbols.length).toBeGreaterThan(0);
      }
    });

    it('should handle empty document', () => {
      const result = parser.parse('', 'empty.yml');
      if (result.success && result.data) {
        const symbols = parser.getDocumentSymbols(result.data);
        expect(Array.isArray(symbols)).toBe(true);
      }
    });
  });

  describe('error handling', () => {
    it('should handle YAML parsing errors', () => {
      const invalidYaml = 'name: test\n  - invalid: list: format';
      const result = parser.parse(invalidYaml, 'test.yml');

      expect(result.success).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
      expect(result.errors[0].message).toContain('YAML parsing error');
    });

    it('should provide meaningful error messages', () => {
      const invalidYaml = 'name: test\n  invalid: indentation';
      const result = parser.parse(invalidYaml, 'test.yml');

      expect(result.errors.length).toBeGreaterThan(0);
      expect(result.errors[0].message).toBeDefined();
      expect(result.errors[0].range).toBeDefined();
    });

    it('should handle null or undefined content', () => {
      const result = parser.parse('', 'test.yml');
      expect(result.success).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });
});
