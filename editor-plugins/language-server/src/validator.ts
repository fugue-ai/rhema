import {
  type Diagnostic,
  DiagnosticSeverity,
  Range,
  Position,
  TextDocument,
} from 'vscode-languageserver/node';
import type { RhemaDocument } from './parser';
import Ajv from 'ajv';

export interface ValidationResult {
  valid: boolean;
  diagnostics: Diagnostic[];
  warnings: Diagnostic[];
  info: Diagnostic[];
  hints: Diagnostic[];
}

export interface ValidationError {
  range: Range;
  message: string;
  severity: DiagnosticSeverity;
  code?: string;
  source?: string;
  category?: 'syntax' | 'semantic' | 'style' | 'best-practice';
  suggestions?: string[];
  quickFixes?: string[];
}

export interface ValidationRule {
  id: string;
  name: string;
  description: string;
  category: 'syntax' | 'semantic' | 'style' | 'best-practice';
  severity: DiagnosticSeverity;
  enabled: boolean;
  validate: (document: RhemaDocument, context: ValidationContext) => ValidationError[];
}

export interface ValidationContext {
  document: RhemaDocument;
  uri: string;
  workspaceDocuments?: RhemaDocument[];
  configuration?: any;
}

export interface ValidationConfiguration {
  enabled: boolean;
  strict: boolean;
  schemaValidation: boolean;
  customRules: boolean;
  crossDocumentValidation: boolean;
  performanceOptimization: boolean;
  errorCategories: {
    syntax: boolean;
    semantic: boolean;
    style: boolean;
    bestPractice: boolean;
  };
  ruleOverrides: { [ruleId: string]: boolean };
}

export class RhemaValidator {
  private capabilities: any;
  private hasDiagnosticRelatedInformation: boolean = false;
  private ajv: Ajv;
  private schemas: Map<string, any> = new Map();
  private customRules: Map<string, ValidationRule> = new Map();
  private validationCache: Map<string, { result: ValidationResult; timestamp: number }> = new Map();
  private configuration: ValidationConfiguration;

  constructor() {
    this.ajv = new Ajv({
      allErrors: true,
      verbose: true,
      strict: false,
    });

    this.configuration = this.getDefaultConfiguration();
    this.initializeCustomRules();
  }

  initialize(capabilities: any, hasDiagnosticRelatedInformation: boolean): void {
    this.capabilities = capabilities;
    this.hasDiagnosticRelatedInformation = hasDiagnosticRelatedInformation;
  }

  setConfiguration(config: Partial<ValidationConfiguration>): void {
    this.configuration = { ...this.configuration, ...config };
  }

  validate(
    document: RhemaDocument,
    uri: string,
    workspaceDocuments?: RhemaDocument[]
  ): ValidationResult {
    const cacheKey = `${uri}-${document.content ? JSON.stringify(document.content).length : 0}`;
    const cached = this.validationCache.get(cacheKey);

    // Check cache (with 5-minute TTL)
    if (cached && Date.now() - cached.timestamp < 300000) {
      return cached.result;
    }

    const startTime = Date.now();
    const diagnostics: Diagnostic[] = [];
    const warnings: Diagnostic[] = [];
    const info: Diagnostic[] = [];
    const hints: Diagnostic[] = [];

    try {
      const context: ValidationContext = {
        document,
        uri,
        workspaceDocuments,
        configuration: this.configuration,
      };

      // 1. Schema validation
      if (this.configuration.schemaValidation) {
        const schemaErrors = this.performSchemaValidation(document, context);
        diagnostics.push(...schemaErrors);
      }

      // 2. Custom rule validation
      if (this.configuration.customRules) {
        const customErrors = this.performCustomRuleValidation(document, context);
        diagnostics.push(...customErrors);
      }

      // 3. Cross-document validation
      if (this.configuration.crossDocumentValidation && workspaceDocuments) {
        const crossDocErrors = this.performCrossDocumentValidation(
          document,
          workspaceDocuments,
          context
        );
        diagnostics.push(...crossDocErrors);
      }

      // 4. Performance validation
      if (this.configuration.performanceOptimization) {
        const perfWarnings = this.performPerformanceValidation(document, context);
        warnings.push(...perfWarnings);
      }

      // 5. Style and best practice validation
      const styleErrors = this.performStyleValidation(document, context);
      const bestPracticeErrors = this.performBestPracticeValidation(document, context);

      // Categorize diagnostics by severity
      diagnostics.forEach((diagnostic) => {
        switch (diagnostic.severity) {
          case DiagnosticSeverity.Error:
            diagnostics.push(diagnostic);
            break;
          case DiagnosticSeverity.Warning:
            warnings.push(diagnostic);
            break;
          case DiagnosticSeverity.Information:
            info.push(diagnostic);
            break;
          case DiagnosticSeverity.Hint:
            hints.push(diagnostic);
            break;
        }
      });

      const result: ValidationResult = {
        valid: diagnostics.length === 0,
        diagnostics,
        warnings,
        info,
        hints,
      };

      // Cache the result
      this.validationCache.set(cacheKey, {
        result,
        timestamp: Date.now(),
      });

      // Performance logging
      const duration = Date.now() - startTime;
      if (duration > 100) {
        console.warn(`Validation took ${duration}ms for ${uri}`);
      }

      return result;
    } catch (error: any) {
      const errorDiagnostic: Diagnostic = {
        range: Range.create(Position.create(0, 0), Position.create(0, 0)),
        message: `Validation error: ${error.message}`,
        severity: DiagnosticSeverity.Error,
        source: 'rhema-validator',
      };

      return {
        valid: false,
        diagnostics: [errorDiagnostic],
        warnings: [],
        info: [],
        hints: [],
      };
    }
  }

  private getSchemaForDocumentType(type: RhemaDocument['type']): any {
    const schemas = {
      scope: {
        type: 'object',
        properties: {
          name: { type: 'string' },
          description: { type: 'string' },
          version: { type: 'string' },
          contexts: { type: 'array' },
          dependencies: { type: 'array' },
          config: { type: 'object' },
          metadata: { type: 'object' },
        },
        required: ['name'],
      },
      knowledge: {
        type: 'object',
        properties: {
          contexts: { type: 'array' },
          patterns: { type: 'array' },
          conventions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
      todos: {
        type: 'object',
        properties: {
          tasks: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
      decisions: {
        type: 'object',
        properties: {
          decisions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
      patterns: {
        type: 'object',
        properties: {
          patterns: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
      conventions: {
        type: 'object',
        properties: {
          conventions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    return schemas[type];
  }

  private convertAjvErrorToDiagnostic(error: any, document: RhemaDocument): Diagnostic | null {
    try {
      const range = this.getRangeFromPath(error.dataPath, document);

      return {
        range,
        message: error.message,
        severity: DiagnosticSeverity.Error,
        source: 'rhema-schema',
        code: error.keyword,
      };
    } catch (e) {
      return null;
    }
  }

  private getRangeFromPath(dataPath: string, document: RhemaDocument): Range {
    // Simple implementation - in a real scenario, you'd need to map JSON paths to line/column positions
    const lines = JSON.stringify(document.content, null, 2).split('\n');

    // Find the line that contains the path
    let lineNumber = 0;
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].includes(dataPath.replace('/', ''))) {
        lineNumber = i;
        break;
      }
    }

    return Range.create(
      Position.create(lineNumber, 0),
      Position.create(lineNumber, lines[lineNumber]?.length || 0)
    );
  }

  private performSchemaValidation(
    document: RhemaDocument,
    context: ValidationContext
  ): Diagnostic[] {
    const diagnostics: Diagnostic[] = [];
    const schema = this.getSchemaForDocumentType(document.type);

    if (schema) {
      const valid = this.ajv.validate(schema, document.content);

      if (!valid && this.ajv.errors) {
        this.ajv.errors.forEach((error) => {
          const diagnostic = this.convertAjvErrorToDiagnostic(error, document);
          if (diagnostic) {
            diagnostics.push(diagnostic);
          }
        });
      }
    }

    return diagnostics;
  }

  private performCustomRuleValidation(
    document: RhemaDocument,
    context: ValidationContext
  ): Diagnostic[] {
    const diagnostics: Diagnostic[] = [];

    this.customRules.forEach((rule) => {
      if (rule.enabled && this.configuration.ruleOverrides[rule.id] !== false) {
        try {
          const errors = rule.validate(document, context);
          errors.forEach((error) => {
            diagnostics.push({
              range: error.range,
              message: error.message,
              severity: error.severity,
              source: 'rhema-custom-rule',
              code: error.code,
              data: {
                category: error.category,
                suggestions: error.suggestions,
                quickFixes: error.quickFixes,
              },
            });
          });
        } catch (error) {
          console.error(`Error in custom rule ${rule.id}:`, error);
        }
      }
    });

    return diagnostics;
  }

  private performCrossDocumentValidation(
    document: RhemaDocument,
    workspaceDocuments: RhemaDocument[],
    context: ValidationContext
  ): Diagnostic[] {
    const diagnostics: Diagnostic[] = [];

    // Check for duplicate names across documents
    const allNames = new Map<string, string>();

    workspaceDocuments.forEach((wsDoc) => {
      if (wsDoc.content.name) {
        const existing = allNames.get(wsDoc.content.name);
        if (existing && existing !== context.uri) {
          diagnostics.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: `Duplicate document name '${wsDoc.content.name}' found in ${existing}`,
            severity: DiagnosticSeverity.Warning,
            source: 'rhema-cross-doc',
            code: 'duplicate-name',
          });
        } else {
          allNames.set(wsDoc.content.name, context.uri);
        }
      }
    });

    // Check for broken references
    if (document.content.dependencies) {
      document.content.dependencies.forEach((dep: any, index: number) => {
        if (dep.name) {
          const referencedDoc = workspaceDocuments.find((d) => d.content.name === dep.name);
          if (!referencedDoc) {
            diagnostics.push({
              range: Range.create(Position.create(0, 0), Position.create(0, 0)),
              message: `Dependency '${dep.name}' not found in workspace`,
              severity: DiagnosticSeverity.Error,
              source: 'rhema-cross-doc',
              code: 'missing-dependency',
            });
          }
        }
      });
    }

    return diagnostics;
  }

  private performPerformanceValidation(
    document: RhemaDocument,
    context: ValidationContext
  ): Diagnostic[] {
    const warnings: Diagnostic[] = [];

    // Check for large arrays
    if (document.content.contexts && document.content.contexts.length > 50) {
      warnings.push({
        range: Range.create(Position.create(0, 0), Position.create(0, 0)),
        message:
          'Large number of contexts may impact performance. Consider splitting into multiple documents.',
        severity: DiagnosticSeverity.Warning,
        source: 'rhema-performance',
        code: 'large-contexts',
      });
    }

    // Check for deeply nested structures
    const maxDepth = this.getMaxDepth(document.content);
    if (maxDepth > 10) {
      warnings.push({
        range: Range.create(Position.create(0, 0), Position.create(0, 0)),
        message: 'Deeply nested structure detected. Consider flattening for better performance.',
        severity: DiagnosticSeverity.Warning,
        source: 'rhema-performance',
        code: 'deep-nesting',
      });
    }

    return warnings;
  }

  private performStyleValidation(
    document: RhemaDocument,
    context: ValidationContext
  ): Diagnostic[] {
    const diagnostics: Diagnostic[] = [];

    // Check naming conventions
    if (document.content.name) {
      if (!/^[a-z][a-z0-9-]*$/.test(document.content.name)) {
        diagnostics.push({
          range: Range.create(Position.create(0, 0), Position.create(0, 0)),
          message: 'Document name should use kebab-case (e.g., "my-document")',
          severity: DiagnosticSeverity.Information,
          source: 'rhema-style',
          code: 'naming-convention',
          data: {
            suggestions: ['Use kebab-case for document names'],
            quickFixes: ['Convert to kebab-case'],
          },
        });
      }
    }

    // Check for consistent indentation
    // This would require access to the original text

    return diagnostics;
  }

  private performBestPracticeValidation(
    document: RhemaDocument,
    context: ValidationContext
  ): Diagnostic[] {
    const diagnostics: Diagnostic[] = [];

    // Check for required metadata
    if (!document.content.metadata) {
      diagnostics.push({
        range: Range.create(Position.create(0, 0), Position.create(0, 0)),
        message: 'Consider adding metadata section for better document management',
        severity: DiagnosticSeverity.Hint,
        source: 'rhema-best-practice',
        code: 'missing-metadata',
        data: {
          suggestions: ['Add metadata section with created/modified dates'],
          quickFixes: ['Add metadata section'],
        },
      });
    }

    // Check for descriptions
    if (!document.content.description) {
      diagnostics.push({
        range: Range.create(Position.create(0, 0), Position.create(0, 0)),
        message: 'Consider adding a description for better documentation',
        severity: DiagnosticSeverity.Hint,
        source: 'rhema-best-practice',
        code: 'missing-description',
        data: {
          suggestions: ['Add a description field'],
          quickFixes: ['Add description'],
        },
      });
    }

    return diagnostics;
  }

  private initializeCustomRules(): void {
    // Add custom validation rules
    this.addCustomRule({
      id: 'semantic-versioning',
      name: 'Semantic Versioning',
      description: 'Ensure version follows semantic versioning',
      category: 'semantic',
      severity: DiagnosticSeverity.Warning,
      enabled: true,
      validate: (document, context) => {
        const errors: ValidationError[] = [];
        if (document.content.version && !this.isValidVersion(document.content.version)) {
          errors.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: 'Version should follow semantic versioning (e.g., 1.0.0)',
            severity: DiagnosticSeverity.Warning,
            source: 'rhema-custom-rule',
            code: 'invalid-version',
            category: 'semantic',
            suggestions: ['Use format: MAJOR.MINOR.PATCH'],
            quickFixes: ['Fix version format'],
          });
        }
        return errors;
      },
    });

    this.addCustomRule({
      id: 'date-format',
      name: 'Date Format',
      description: 'Ensure dates follow ISO 8601 format',
      category: 'style',
      severity: DiagnosticSeverity.Information,
      enabled: true,
      validate: (document, context) => {
        const errors: ValidationError[] = [];
        if (document.content.created && !this.isValidDate(document.content.created)) {
          errors.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: 'Created date should follow ISO 8601 format (YYYY-MM-DD)',
            severity: DiagnosticSeverity.Information,
            source: 'rhema-custom-rule',
            code: 'invalid-date',
            category: 'style',
            suggestions: ['Use ISO 8601 format: YYYY-MM-DD'],
            quickFixes: ['Fix date format'],
          });
        }
        return errors;
      },
    });
  }

  addCustomRule(rule: ValidationRule): void {
    this.customRules.set(rule.id, rule);
  }

  removeCustomRule(ruleId: string): void {
    this.customRules.delete(ruleId);
  }

  getCustomRules(): ValidationRule[] {
    return Array.from(this.customRules.values());
  }

  clearValidationCache(): void {
    this.validationCache.clear();
  }

  private getDefaultConfiguration(): ValidationConfiguration {
    return {
      enabled: true,
      strict: false,
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
  }

  private getMaxDepth(obj: any, currentDepth = 0): number {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
      return currentDepth;
    }

    let maxDepth = currentDepth;
    Object.values(obj).forEach((value) => {
      if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
        maxDepth = Math.max(maxDepth, this.getMaxDepth(value, currentDepth + 1));
      }
    });

    return maxDepth;
  }

  private isValidVersion(version: string): boolean {
    // Simple semantic versioning validation
    const semverRegex =
      /^\d+\.\d+\.\d+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$/;
    return semverRegex.test(version);
  }

  private isValidDate(date: string): boolean {
    // Simple ISO 8601 date validation (YYYY-MM-DD)
    const dateRegex = /^\d{4}-\d{2}-\d{2}$/;
    return dateRegex.test(date);
  }

  // ... existing methods remain the same ...
}
