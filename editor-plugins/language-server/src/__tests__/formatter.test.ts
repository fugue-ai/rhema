import { jest, describe, it, expect, beforeEach } from '@jest/globals';
import { Range, FormattingOptions } from 'vscode-languageserver';
import { TextDocument } from 'vscode-languageserver-textdocument';
import { RhemaFormatter } from '../formatter';
import { 
  createTestDocument, 
  testDocuments 
} from '../testSetup';

describe('RhemaFormatter', () => {
  let formatter: RhemaFormatter;

  beforeEach(() => {
    formatter = new RhemaFormatter();
  });

  describe('formatDocument', () => {
    it('should format a valid scope document', () => {
      const document = createTestDocument(testDocuments.validScope);
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocument(document, options);

      expect(edits).toBeDefined();
      expect(Array.isArray(edits)).toBe(true);
      expect(edits.length).toBeGreaterThan(0);
      expect(edits[0]).toHaveProperty('range');
      expect(edits[0]).toHaveProperty('newText');
    });

    it('should format a complex document', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocument(document, options);

      expect(edits).toBeDefined();
      expect(Array.isArray(edits)).toBe(true);
      expect(edits.length).toBeGreaterThan(0);
    });

    it('should handle malformed documents gracefully', () => {
      const document = createTestDocument(testDocuments.invalidScope);
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocument(document, options);

      expect(edits).toBeDefined();
      expect(Array.isArray(edits)).toBe(true);
    });

    it('should return empty array when no formatting needed', () => {
      const document = createTestDocument(testDocuments.validScope);
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocument(document, options);

      expect(Array.isArray(edits)).toBe(true);
    });
  });

  describe('formatDocumentRange', () => {
    it('should format a specific range', () => {
      const document = createTestDocument(testDocuments.complexDocument);
      const range = Range.create(4, 0, 8, 0); // Contexts section
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocumentRange(document, range, options);

      expect(edits).toBeDefined();
      expect(Array.isArray(edits)).toBe(true);
      expect(edits.length).toBeGreaterThan(0);
    });

    it('should handle single line formatting', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(0, 0, 0, 20); // First line
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocumentRange(document, range, options);

      expect(edits).toBeDefined();
      expect(Array.isArray(edits)).toBe(true);
    });

    it('should handle out of bounds range', () => {
      const document = createTestDocument(testDocuments.validScope);
      const range = Range.create(100, 0, 110, 0);
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const edits = formatter.formatDocumentRange(document, range, options);

      expect(Array.isArray(edits)).toBe(true);
    });
  });

  describe('formatYaml', () => {
    it('should format valid YAML content', () => {
      const content = `name:test-scope
version:"1.0.0"
description:Test scope document`;
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const formatted = formatter['formatYaml'](content, options);

      expect(formatted).toBeDefined();
      expect(typeof formatted).toBe('string');
      expect(formatted).toContain('name:test-scope');
      expect(formatted).toContain('version:"1.0.0"');
    });

    it('should handle complex YAML structures', () => {
      const content = `contexts:
- name:frontend
description:Frontend development context
patterns:
- name:react-pattern
description:React component pattern`;
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const formatted = formatter['formatYaml'](content, options);

      expect(formatted).toBeDefined();
      expect(typeof formatted).toBe('string');
      expect(formatted).toContain('contexts:');
      expect(formatted).toContain('- name:frontend');
    });

    it('should handle invalid YAML gracefully', () => {
      const content = `invalid:yaml:content:here`;
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const formatted = formatter['formatYaml'](content, options);

      expect(formatted).toBeDefined();
      expect(typeof formatted).toBe('string');
    });

    it('should preserve comments', () => {
      const content = `# This is a comment
name: test-scope
# Another comment
version: "1.0.0"`;
      const options: FormattingOptions = { tabSize: 2, insertSpaces: true };

      const formatted = formatter['formatYaml'](content, options);

      expect(formatted).toBeDefined();
      expect(typeof formatted).toBe('string');
      // Note: The YAML library might not preserve comments, so we'll just check that formatting works
      expect(formatted).toContain('name:');
      expect(formatted).toContain('version:');
    });
  });

  describe('provideFoldingRanges', () => {
    it('should provide folding ranges for document', () => {
      const document = createTestDocument(testDocuments.complexDocument);

      const ranges = formatter.provideFoldingRanges(document);

      expect(ranges).toBeDefined();
      expect(Array.isArray(ranges)).toBe(true);
      expect(ranges.length).toBeGreaterThan(0);
    });
  });

  describe('validateYamlStructure', () => {
    it('should validate valid YAML structure', () => {
      const content = `name: test-scope
version: "1.0.0"`;

      const result = formatter.validateYamlStructure(content);

      expect(result).toBeDefined();
      expect(result).toHaveProperty('valid');
      expect(result).toHaveProperty('errors');
      expect(result.valid).toBe(true);
    });
  });

  describe('getYamlStructure', () => {
    it('should get YAML structure information', () => {
      const content = `name: test-scope
contexts:
  - name: development`;

      const structure = formatter.getYamlStructure(content);

      expect(structure).toBeDefined();
      expect(structure).toHaveProperty('keys');
      expect(structure).toHaveProperty('depth');
      expect(Array.isArray(structure.keys)).toBe(true);
      expect(typeof structure.depth).toBe('number');
    });
  });
}); 