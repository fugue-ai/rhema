import * as yaml from 'yaml';
import * as path from 'path';
import { Range, Position, Diagnostic, DiagnosticSeverity } from 'vscode-languageserver/node';

export interface ParseResult {
  success: boolean;
  data?: any;
  errors: ParseError[];
}

export interface ParseError {
  range: Range;
  message: string;
  severity: DiagnosticSeverity;
}

export interface RhemaDocument {
  type: 'scope' | 'knowledge' | 'todos' | 'decisions' | 'patterns' | 'conventions';
  content: any;
  metadata: {
    version: string;
    created: string;
    modified: string;
    author?: string;
  };
}

export class RhemaParser {
  private capabilities: any;

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  parse(text: string, uri: string): ParseResult {
    const errors: ParseError[] = [];

    try {
      // Parse YAML content
      const parsed = yaml.parse(text);

      if (!parsed) {
        errors.push({
          range: Range.create(Position.create(0, 0), Position.create(0, 0)),
          message: 'Empty or invalid YAML content',
          severity: DiagnosticSeverity.Error,
        });
        return { success: false, errors };
      }

      // Determine document type based on filename
      const fileName = path.basename(uri).toLowerCase();
      let documentType: RhemaDocument['type'] = 'scope';

      if (fileName.includes('knowledge')) {
        documentType = 'knowledge';
      } else if (fileName.includes('todos')) {
        documentType = 'todos';
      } else if (fileName.includes('decisions')) {
        documentType = 'decisions';
      } else if (fileName.includes('patterns')) {
        documentType = 'patterns';
      } else if (fileName.includes('conventions')) {
        documentType = 'conventions';
      }

      // Create structured document
      const document: RhemaDocument = {
        type: documentType,
        content: parsed,
        metadata: {
          version: parsed.version || '1.0.0',
          created: parsed.created || new Date().toISOString(),
          modified: parsed.modified || new Date().toISOString(),
          author: parsed.author,
        },
      };

      // Validate basic structure
      const validationErrors = this.validateBasicStructure(document, text);
      errors.push(...validationErrors);

      return {
        success: errors.length === 0,
        data: document,
        errors,
      };
    } catch (error: any) {
      // Handle YAML parsing errors
      const errorMessage = error.message || 'Unknown parsing error';
      const line = error.line || 0;
      const column = error.column || 0;

      errors.push({
        range: Range.create(Position.create(line, column), Position.create(line, column + 1)),
        message: `YAML parsing error: ${errorMessage}`,
        severity: DiagnosticSeverity.Error,
      });

      return { success: false, errors };
    }
  }

  private validateBasicStructure(document: RhemaDocument, text: string): ParseError[] {
    const errors: ParseError[] = [];
    const lines = text.split('\n');

    // Check for required fields based on document type
    switch (document.type) {
      case 'scope':
        if (!document.content.name) {
          errors.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: 'Scope document must have a "name" field',
            severity: DiagnosticSeverity.Warning,
          });
        }
        break;
      case 'knowledge':
        if (!document.content.contexts) {
          errors.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: 'Knowledge document should have "contexts" field',
            severity: DiagnosticSeverity.Information,
          });
        }
        break;
      case 'todos':
        if (!document.content.tasks) {
          errors.push({
            range: Range.create(Position.create(0, 0), Position.create(0, 0)),
            message: 'Todos document should have "tasks" field',
            severity: DiagnosticSeverity.Information,
          });
        }
        break;
    }

    return errors;
  }

  getDocumentSymbols(document: RhemaDocument): any[] {
    const symbols: any[] = [];

    if (!document.content) return symbols;

    // Extract top-level keys as symbols
    Object.keys(document.content).forEach((key, index) => {
      symbols.push({
        name: key,
        kind: this.getSymbolKind(key),
        range: Range.create(Position.create(index, 0), Position.create(index, 0)),
        selectionRange: Range.create(Position.create(index, 0), Position.create(index, 0)),
      });
    });

    return symbols;
  }

  private getSymbolKind(key: string): number {
    // Map keys to symbol kinds
    const keyMap: { [key: string]: number } = {
      name: 7, // Variable
      description: 14, // String
      version: 7, // Variable
      contexts: 2, // Namespace
      tasks: 12, // Array
      decisions: 12, // Array
      patterns: 12, // Array
      conventions: 12, // Array
      dependencies: 12, // Array
      config: 2, // Namespace
      metadata: 2, // Namespace
    };

    return keyMap[key] || 7; // Default to Variable
  }
}
