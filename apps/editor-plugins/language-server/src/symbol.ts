import {
  type DocumentSymbol,
  SymbolKind,
  Range,
  Position,
  type TextDocument,
} from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';

export class RhemaSymbolProvider {
  private capabilities: any;

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  provideDocumentSymbols(document: TextDocument, cachedDocument?: any): DocumentSymbol[] {
    try {
      const text = document.getText();
      const symbols: DocumentSymbol[] = [];

      // Parse the document structure
      const structure = this.parseDocumentStructure(text);

      // Convert structure to symbols
      structure.forEach((item) => {
        const symbol = this.createSymbol(item);
        if (symbol) {
          symbols.push(symbol);
        }
      });

      return symbols;
    } catch (error) {
      console.error('Error providing document symbols:', error);
      return [];
    }
  }

  private parseDocumentStructure(text: string): Array<{
    name: string;
    kind: SymbolKind;
    range: Range;
    selectionRange: Range;
    children: Array<any>;
    detail?: string;
  }> {
    const structure: Array<{
      name: string;
      kind: SymbolKind;
      range: Range;
      selectionRange: Range;
      children: Array<any>;
      detail?: string;
    }> = [];

    const lines = text.split('\n');
    let currentSection: any = null;
    let currentIndent = 0;
    let sectionStart = 0;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const trimmedLine = line.trim();

      if (trimmedLine === '') continue;

      const indent = this.getIndentLevel(line);

      // Top-level keys (indent = 0)
      if (indent === 0 && trimmedLine.endsWith(':')) {
        // Close previous section
        if (currentSection && i > sectionStart) {
          currentSection.range = Range.create(
            Position.create(sectionStart, 0),
            Position.create(i - 1, lines[i - 1]?.length || 0)
          );
        }

        // Start new section
        const keyName = trimmedLine.slice(0, -1); // Remove trailing colon
        currentSection = {
          name: keyName,
          kind: this.getSymbolKind(keyName),
          range: Range.create(Position.create(i, 0), Position.create(i, line.length)),
          selectionRange: Range.create(
            Position.create(i, line.indexOf(keyName)),
            Position.create(i, line.indexOf(keyName) + keyName.length)
          ),
          children: [],
          detail: this.getSymbolDetail(keyName),
        };

        structure.push(currentSection);
        sectionStart = i;
        currentIndent = indent;
      }
      // Nested items
      else if (indent > currentIndent) {
        if (currentSection) {
          const nestedItem = this.parseNestedItem(line, i, lines);
          if (nestedItem) {
            currentSection.children.push(nestedItem);
          }
        }
      }
    }

    // Close last section
    if (currentSection && lines.length > sectionStart) {
      currentSection.range = Range.create(
        Position.create(sectionStart, 0),
        Position.create(lines.length - 1, lines[lines.length - 1]?.length || 0)
      );
    }

    return structure;
  }

  private parseNestedItem(line: string, lineIndex: number, lines: string[]): any {
    const trimmedLine = line.trim();

    // Array item
    if (trimmedLine.startsWith('-')) {
      const itemContent = trimmedLine.substring(1).trim();

      // Extract name from array item
      let name = 'Array Item';
      if (itemContent.includes(':')) {
        const keyMatch = itemContent.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
        if (keyMatch) {
          name = keyMatch[1];
        }
      } else if (itemContent) {
        name = itemContent;
      }

      return {
        name,
        kind: SymbolKind.Variable,
        range: Range.create(Position.create(lineIndex, 0), Position.create(lineIndex, line.length)),
        selectionRange: Range.create(
          Position.create(lineIndex, line.indexOf('-') + 1),
          Position.create(lineIndex, line.length)
        ),
        children: [],
        detail: 'Array item',
      };
    }

    // Key-value pair
    if (trimmedLine.includes(':')) {
      const keyMatch = trimmedLine.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (keyMatch) {
        const keyName = keyMatch[1];
        return {
          name: keyName,
          kind: this.getSymbolKind(keyName),
          range: Range.create(
            Position.create(lineIndex, 0),
            Position.create(lineIndex, line.length)
          ),
          selectionRange: Range.create(
            Position.create(lineIndex, line.indexOf(keyName)),
            Position.create(lineIndex, line.indexOf(keyName) + keyName.length)
          ),
          children: [],
          detail: this.getSymbolDetail(keyName),
        };
      }
    }

    return null;
  }

  private createSymbol(item: any): DocumentSymbol | null {
    if (!item) return null;

    return {
      name: item.name,
      kind: item.kind,
      range: item.range,
      selectionRange: item.selectionRange,
      detail: item.detail,
      children: item.children.map((child: any) => this.createSymbol(child)).filter(Boolean),
    };
  }

  private getIndentLevel(line: string): number {
    let indent = 0;
    for (let i = 0; i < line.length; i++) {
      if (line[i] === ' ' || line[i] === '\t') {
        indent++;
      } else {
        break;
      }
    }
    return indent;
  }

  private getSymbolKind(key: string): SymbolKind {
    const keyMap: { [key: string]: SymbolKind } = {
      name: SymbolKind.Variable,
      description: SymbolKind.String,
      version: SymbolKind.Variable,
      contexts: SymbolKind.Namespace,
      tasks: SymbolKind.Array,
      decisions: SymbolKind.Array,
      patterns: SymbolKind.Array,
      conventions: SymbolKind.Array,
      dependencies: SymbolKind.Array,
      config: SymbolKind.Namespace,
      metadata: SymbolKind.Namespace,
      created: SymbolKind.Variable,
      modified: SymbolKind.Variable,
      author: SymbolKind.Variable,
      status: SymbolKind.Variable,
      priority: SymbolKind.Variable,
      tags: SymbolKind.Array,
      examples: SymbolKind.Array,
      benefits: SymbolKind.Array,
      rules: SymbolKind.Array,
      rationale: SymbolKind.String,
      date: SymbolKind.Variable,
      references: SymbolKind.Array,
      notes: SymbolKind.String,
      type: SymbolKind.Variable,
      scope: SymbolKind.Variable,
      owner: SymbolKind.Variable,
      reviewer: SymbolKind.Variable,
      approver: SymbolKind.Variable,
    };

    return keyMap[key] || SymbolKind.Variable;
  }

  private getSymbolDetail(key: string): string {
    const detailMap: { [key: string]: string } = {
      name: 'Document name',
      description: 'Document description',
      version: 'Document version',
      contexts: 'Context definitions',
      tasks: 'Task list',
      decisions: 'Decision records',
      patterns: 'Pattern definitions',
      conventions: 'Convention definitions',
      dependencies: 'Dependencies',
      config: 'Configuration',
      metadata: 'Metadata',
      created: 'Creation date',
      modified: 'Last modified date',
      author: 'Author',
      status: 'Status',
      priority: 'Priority',
      tags: 'Tags',
      examples: 'Examples',
      benefits: 'Benefits',
      rules: 'Rules',
      rationale: 'Rationale',
      date: 'Date',
      references: 'References',
      notes: 'Notes',
      type: 'Type',
      scope: 'Scope',
      owner: 'Owner',
      reviewer: 'Reviewer',
      approver: 'Approver',
    };

    return detailMap[key] || 'Property';
  }
}
