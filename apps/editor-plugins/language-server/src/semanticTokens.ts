import {
  type SemanticTokens,
  type SemanticTokensLegend,
  Position,
  type TextDocument,
} from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';

export class RhemaSemanticTokensProvider {
  private capabilities: any;
  private hasSemanticTokensCapability: boolean = false;

  initialize(capabilities: any, hasSemanticTokensCapability: boolean): void {
    this.capabilities = capabilities;
    this.hasSemanticTokensCapability = hasSemanticTokensCapability;
  }

  getLegend(): SemanticTokensLegend {
    return {
      tokenTypes: [
        'namespace',
        'type',
        'class',
        'enum',
        'interface',
        'struct',
        'typeParameter',
        'parameter',
        'variable',
        'property',
        'enumMember',
        'decorator',
        'event',
        'function',
        'method',
        'macro',
        'keyword',
        'modifier',
        'comment',
        'string',
        'number',
        'regexp',
        'operator',
      ],
      tokenModifiers: [
        'declaration',
        'definition',
        'readonly',
        'static',
        'deprecated',
        'abstract',
        'async',
        'modification',
        'documentation',
        'defaultLibrary',
      ],
    };
  }

  provideSemanticTokens(document: TextDocument, cachedDocument?: any): SemanticTokens {
    try {
      const text = document.getText();
      const tokens: number[] = [];

      const lines = text.split('\n');

      for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
        const line = lines[lineIndex];
        const lineTokens = this.tokenizeLine(line, lineIndex);
        tokens.push(...lineTokens);
      }

      return { data: tokens };
    } catch (error) {
      console.error('Error providing semantic tokens:', error);
      return { data: [] };
    }
  }

  provideSemanticTokensDelta(
    document: TextDocument,
    previousResultId: string,
    cachedDocument?: any
  ): { edits: any[] } {
    // For now, return full tokens
    // In a real implementation, you would compute the delta
    return { edits: [] };
  }

  private tokenizeLine(line: string, lineIndex: number): number[] {
    const tokens: number[] = [];
    let charIndex = 0;

    // Skip leading whitespace
    while (charIndex < line.length && /\s/.test(line[charIndex])) {
      charIndex++;
    }

    while (charIndex < line.length) {
      const token = this.getNextToken(line, charIndex);
      if (!token) break;

      const { type, modifier, length } = token;

      // Add token data: [line, char, length, tokenType, tokenModifiers]
      tokens.push(
        lineIndex,
        charIndex,
        length,
        this.getTokenTypeIndex(type),
        this.getTokenModifierIndex(modifier)
      );

      charIndex += length;

      // Skip whitespace
      while (charIndex < line.length && /\s/.test(line[charIndex])) {
        charIndex++;
      }
    }

    return tokens;
  }

  private getNextToken(
    line: string,
    startIndex: number
  ): { type: string; modifier: string; length: number } | null {
    const remaining = line.substring(startIndex);

    // Comments
    const commentMatch = remaining.match(/^#.*/);
    if (commentMatch) {
      return {
        type: 'comment',
        modifier: '',
        length: commentMatch[0].length,
      };
    }

    // Keywords
    const keywordMatch = remaining.match(
      /^(name|description|version|contexts|tasks|decisions|patterns|conventions|dependencies|config|metadata|created|modified|author|status|priority|tags|examples|benefits|rules|rationale|date|references|notes|type|scope|owner|reviewer|approver)\b/
    );
    if (keywordMatch) {
      return {
        type: 'keyword',
        modifier: 'declaration',
        length: keywordMatch[0].length,
      };
    }

    // String literals
    const doubleQuoteMatch = remaining.match(/^"[^"]*"/);
    if (doubleQuoteMatch) {
      return {
        type: 'string',
        modifier: '',
        length: doubleQuoteMatch[0].length,
      };
    }

    const singleQuoteMatch = remaining.match(/^'[^']*'/);
    if (singleQuoteMatch) {
      return {
        type: 'string',
        modifier: '',
        length: singleQuoteMatch[0].length,
      };
    }

    // Numbers
    const numberMatch = remaining.match(/^\d+(\.\d+)?/);
    if (numberMatch) {
      return {
        type: 'number',
        modifier: '',
        length: numberMatch[0].length,
      };
    }

    // YAML syntax
    const yamlSyntaxMatch = remaining.match(/^[:\-[\]{}|>]/);
    if (yamlSyntaxMatch) {
      return {
        type: 'operator',
        modifier: '',
        length: yamlSyntaxMatch[0].length,
      };
    }

    // Identifiers (variable names)
    const identifierMatch = remaining.match(/^[a-zA-Z_][a-zA-Z0-9_]*/);
    if (identifierMatch) {
      return {
        type: 'variable',
        modifier: '',
        length: identifierMatch[0].length,
      };
    }

    // Default: single character
    return {
      type: 'operator',
      modifier: '',
      length: 1,
    };
  }

  private getTokenTypeIndex(type: string): number {
    const typeMap: { [key: string]: number } = {
      namespace: 0,
      type: 1,
      class: 2,
      enum: 3,
      interface: 4,
      struct: 5,
      typeParameter: 6,
      parameter: 7,
      variable: 8,
      property: 9,
      enumMember: 10,
      decorator: 11,
      event: 12,
      function: 13,
      method: 14,
      macro: 15,
      keyword: 16,
      modifier: 17,
      comment: 18,
      string: 19,
      number: 20,
      regexp: 21,
      operator: 22,
    };

    return typeMap[type] || 22; // Default to operator
  }

  private getTokenModifierIndex(modifier: string): number {
    const modifierMap: { [key: string]: number } = {
      declaration: 0,
      definition: 1,
      readonly: 2,
      static: 3,
      deprecated: 4,
      abstract: 5,
      async: 6,
      modification: 7,
      documentation: 8,
      defaultLibrary: 9,
    };

    return modifierMap[modifier] || 0; // Default to no modifier
  }
}
