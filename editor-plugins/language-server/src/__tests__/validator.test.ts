import { RhemaValidator } from '../validator';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';
import { DiagnosticSeverity } from 'vscode-languageserver/node';

describe('RhemaValidator', () => {
  let validator: RhemaValidator;

  beforeEach(() => {
    validator = new RhemaValidator();
    validator.initialize(createMockCapabilities(), true);
  });

  describe('Initialization', () => {
    it('should initialize with default configuration', () => {
      expect(validator).toBeDefined();
      expect(validator['ajv']).toBeDefined();
      expect(validator['schemas']).toBeDefined();
      expect(validator['customRules']).toBeDefined();
    });

    it('should initialize custom rules', () => {
      const rules = validator.getCustomRules();
      expect(rules.length).toBeGreaterThan(0);
    });
  });

  describe('validate', () => {
    it('should validate valid scope documents', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          description: 'Test scope',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'test.yml');

      expect(result.valid).toBe(true);
      expect(result.diagnostics).toHaveLength(0);
      expect(result.warnings).toHaveLength(0);
    });

    it('should detect missing required fields', () => {
      const document = {
        type: 'scope' as const,
        content: {
          version: '1.0.0', // Missing name
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'test.yml');

      expect(result.valid).toBe(false);
      expect(result.diagnostics.length).toBeGreaterThan(0);
      expect(result.diagnostics.some(d => d.message.includes('name'))).toBe(true);
    });

    it('should validate knowledge documents', () => {
      const document = {
        type: 'knowledge' as const,
        content: {
          contexts: [
            {
              name: 'development',
              description: 'Development context',
            },
          ],
          patterns: [
            {
              name: 'component-pattern',
              description: 'Component pattern',
            },
          ],
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'knowledge.yml');

      expect(result.valid).toBe(true);
      expect(result.diagnostics).toHaveLength(0);
    });

    it('should validate todos documents', () => {
      const document = {
        type: 'todos' as const,
        content: {
          tasks: [
            {
              title: 'Test task',
              description: 'Test description',
              status: 'pending',
              priority: 'high',
            },
          ],
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'todos.yml');

      expect(result.valid).toBe(true);
      expect(result.diagnostics).toHaveLength(0);
    });

    it('should handle invalid document types', () => {
      const document = {
        type: 'unknown' as any,
        content: {},
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'unknown.yml');

      expect(result.valid).toBe(false);
      expect(result.diagnostics.length).toBeGreaterThan(0);
    });
  });

  describe('Schema Validation', () => {
    it('should validate against JSON schemas', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          description: 'Test scope',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performSchemaValidation'](document, context);

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should detect schema violations', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 123, // Should be string
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performSchemaValidation'](document, context);

      expect(diagnostics.length).toBeGreaterThan(0);
      expect(diagnostics.some(d => d.message.includes('string'))).toBe(true);
    });
  });

  describe('Custom Rule Validation', () => {
    it('should apply custom validation rules', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performCustomRuleValidation'](document, context);

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should add custom rules', () => {
      const customRule = {
        id: 'test-rule',
        name: 'Test Rule',
        description: 'Test validation rule',
        category: 'semantic' as const,
        severity: DiagnosticSeverity.Warning,
        enabled: true,
        validate: () => [],
      };

      validator.addCustomRule(customRule);
      const rules = validator.getCustomRules();

      expect(rules.some(r => r.id === 'test-rule')).toBe(true);
    });

    it('should remove custom rules', () => {
      const customRule = {
        id: 'test-rule',
        name: 'Test Rule',
        description: 'Test validation rule',
        category: 'semantic' as const,
        severity: DiagnosticSeverity.Warning,
        enabled: true,
        validate: () => [],
      };

      validator.addCustomRule(customRule);
      validator.removeCustomRule('test-rule');
      const rules = validator.getCustomRules();

      expect(rules.some(r => r.id === 'test-rule')).toBe(false);
    });
  });

  describe('Cross-Document Validation', () => {
    it('should validate cross-document references', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          dependencies: [
            { name: 'other-scope', version: '1.0.0' },
          ],
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const workspaceDocuments = [
        {
          type: 'scope' as const,
          content: {
            name: 'other-scope',
            version: '1.0.0',
          },
          metadata: {
            version: '1.0.0',
            created: '2024-01-01',
            modified: '2024-01-01',
          },
        },
      ];

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performCrossDocumentValidation'](
        document,
        workspaceDocuments,
        context
      );

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should detect missing cross-document references', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          dependencies: [
            { name: 'missing-scope', version: '1.0.0' },
          ],
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const workspaceDocuments = [
        {
          type: 'scope' as const,
          content: {
            name: 'other-scope',
            version: '1.0.0',
          },
          metadata: {
            version: '1.0.0',
            created: '2024-01-01',
            modified: '2024-01-01',
          },
        },
      ];

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performCrossDocumentValidation'](
        document,
        workspaceDocuments,
        context
      );

      expect(diagnostics.length).toBeGreaterThan(0);
      expect(diagnostics.some(d => d.message.includes('missing-scope'))).toBe(true);
    });
  });

  describe('Style Validation', () => {
    it('should validate naming conventions', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'TestScope', // Should be kebab-case
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performStyleValidation'](document, context);

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should validate version format', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: 'invalid-version', // Should be semantic version
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performStyleValidation'](document, context);

      expect(diagnostics.length).toBeGreaterThan(0);
      expect(diagnostics.some(d => d.message.includes('version'))).toBe(true);
    });
  });

  describe('Best Practice Validation', () => {
    it('should validate best practices', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          description: 'A', // Too short
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performBestPracticeValidation'](document, context);

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should detect overly complex structures', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          deeply: {
            nested: {
              structure: {
                that: {
                  is: {
                    too: {
                      complex: 'value',
                    },
                  },
                },
              },
            },
          },
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performBestPracticeValidation'](document, context);

      expect(diagnostics.length).toBeGreaterThan(0);
      expect(diagnostics.some(d => d.message.includes('complex'))).toBe(true);
    });
  });

  describe('Performance Validation', () => {
    it('should validate performance characteristics', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          largeArray: Array.from({ length: 10000 }, (_, i) => ({ id: i, value: `value-${i}` })),
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performPerformanceValidation'](document, context);

      expect(diagnostics).toBeDefined();
      expect(Array.isArray(diagnostics)).toBe(true);
    });

    it('should detect performance issues', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
          massiveArray: Array.from({ length: 100000 }, (_, i) => ({ id: i, value: `value-${i}` })),
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const context = { document, uri: 'test.yml' };
      const diagnostics = validator['performPerformanceValidation'](document, context);

      expect(diagnostics.length).toBeGreaterThan(0);
      expect(diagnostics.some(d => d.message.includes('performance'))).toBe(true);
    });
  });

  describe('Configuration', () => {
    it('should update validation configuration', () => {
      const newConfig = {
        enabled: true,
        strict: true,
        schemaValidation: true,
        customRules: true,
        crossDocumentValidation: true,
        performanceOptimization: true,
        errorCategories: {
          syntax: true,
          semantic: true,
          style: true,
          bestPractice: true,
        },
        ruleOverrides: {},
      };

      validator.setConfiguration(newConfig);

      expect(validator['configuration']).toEqual(newConfig);
    });

    it('should respect configuration settings', () => {
      validator.setConfiguration({
        errorCategories: {
          syntax: false,
          semantic: true,
          style: false,
          bestPractice: false,
        },
      });

      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(document, 'test.yml');

      // Should only include semantic errors
      expect(result.diagnostics.every(d => d.severity === DiagnosticSeverity.Error)).toBe(true);
    });
  });

  describe('Caching', () => {
    it('should cache validation results', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      // First validation
      const result1 = validator.validate(document, 'test.yml');
      
      // Second validation (should use cache)
      const result2 = validator.validate(document, 'test.yml');

      expect(result1).toEqual(result2);
    });

    it('should clear validation cache', () => {
      validator.clearValidationCache();
      expect(validator['validationCache'].size).toBe(0);
    });
  });

  describe('Error Handling', () => {
    it('should handle malformed documents gracefully', () => {
      const malformedDocument = {
        type: 'scope' as const,
        content: {},
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const result = validator.validate(malformedDocument, 'malformed.yml');

      expect(result.valid).toBe(false);
      expect(result.diagnostics.length).toBeGreaterThan(0);
    });

    it('should handle validation errors gracefully', () => {
      const document = {
        type: 'scope' as const,
        content: {
          name: 'test-scope',
          version: '1.0.0',
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      // Mock a failing validation rule
      const failingRule = {
        id: 'failing-rule',
        name: 'Failing Rule',
        description: 'Rule that throws an error',
        category: 'semantic' as const,
        severity: DiagnosticSeverity.Error,
        enabled: true,
        validate: () => {
          throw new Error('Validation error');
        },
      };

      validator.addCustomRule(failingRule);

      // Should not throw, but should handle the error gracefully
      expect(() => {
        validator.validate(document, 'test.yml');
      }).not.toThrow();
    });
  });

  describe('Performance', () => {
    it('should validate large documents efficiently', () => {
      const largeDocument = {
        type: 'scope' as const,
        content: {
          name: 'large-scope',
          version: '1.0.0',
          description: 'A large scope document',
          contexts: Array.from({ length: 100 }, (_, i) => ({
            name: `context-${i}`,
            description: `Context ${i}`,
            patterns: Array.from({ length: 10 }, (_, j) => ({
              name: `pattern-${i}-${j}`,
              description: `Pattern ${i}-${j}`,
            })),
          })),
        },
        metadata: {
          version: '1.0.0',
          created: '2024-01-01',
          modified: '2024-01-01',
        },
      };

      const startTime = performance.now();
      const result = validator.validate(largeDocument, 'large.yml');
      const endTime = performance.now();

      expect(result).toBeDefined();
      expect(endTime - startTime).toBeLessThan(1000); // Should complete within 1 second
    });
  });
}); 