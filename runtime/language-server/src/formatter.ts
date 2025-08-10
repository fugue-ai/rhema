import {
  TextEdit,
  Range,
  Position,
  type TextDocument,
  type FormattingOptions,
  type FoldingRange,
  FoldingRangeKind,
} from 'vscode-languageserver/node';
import * as yaml from 'yaml';
import { RhemaDocument } from './parser';

export class RhemaFormatter {
  private capabilities: any;

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  formatDocument(document: TextDocument, options: FormattingOptions): TextEdit[] {
    try {
      const text = document.getText();
      const formattedText = this.formatYaml(text, options);

      // Always provide formatting for malformed YAML
      const needsFormatting = this.needsFormatting(text, formattedText);

      if (!needsFormatting && formattedText === text) {
        return []; // No changes needed
      }

      return [
        TextEdit.replace(
          Range.create(
            Position.create(0, 0),
            Position.create(document.lineCount - 1, this.getDocumentRange(document).end.character)
          ),
          formattedText
        ),
      ];
    } catch (error) {
      console.error('Error formatting document:', error);
      return [];
    }
  }

  private needsFormatting(originalText: string, formattedText: string): boolean {
    // Check for common formatting issues
    const lines = originalText.split('\n');
    
    for (const line of lines) {
      // Check for missing spaces after colons
      if (line.includes(':') && !line.includes(': ') && !line.trim().endsWith(':')) {
        return true;
      }
      
      // Check for inconsistent indentation
      if (line.startsWith('\t') && line.includes(' ')) {
        return true;
      }
    }
    
    return false;
  }

  formatDocumentRange(
    document: TextDocument,
    range: Range,
    options: FormattingOptions
  ): TextEdit[] {
    try {
      const text = document.getText();
      const rangeText = this.extractRangeText(text, range);
      const formattedRangeText = this.formatYaml(rangeText, options);

      if (formattedRangeText === rangeText) {
        return []; // No changes needed
      }

      return [TextEdit.replace(range, formattedRangeText)];
    } catch (error) {
      console.error('Error formatting document range:', error);
      return [];
    }
  }

  provideFoldingRanges(document: TextDocument, cachedDocument?: any): FoldingRange[] {
    try {
      const text = document.getText();
      const lines = text.split('\n');
      const foldingRanges: FoldingRange[] = [];

      let currentSection: { start: number; title: string } | null = null;
      let indentLevel = 0;
      let sectionStart = 0;

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmedLine = line.trim();

        if (trimmedLine === '') continue;

        const lineIndent = this.getIndentLevel(line);

        // Detect section headers (top-level keys)
        if (lineIndent === 0 && trimmedLine.endsWith(':')) {
          // Close previous section
          if (currentSection && i > sectionStart) {
            foldingRanges.push({
              startLine: sectionStart,
              endLine: i - 1,
              kind: FoldingRangeKind.Region,
            });
          }

          // Start new section
          currentSection = {
            start: i,
            title: trimmedLine.slice(0, -1), // Remove trailing colon
          };
          sectionStart = i;
        }

        // Detect nested structures
        if (lineIndent > indentLevel) {
          // Start of nested block
          if (currentSection && i > sectionStart) {
            foldingRanges.push({
              startLine: sectionStart,
              endLine: i - 1,
              kind: FoldingRangeKind.Region,
            });
          }
          sectionStart = i;
          indentLevel = lineIndent;
        } else if (lineIndent < indentLevel) {
          // End of nested block
          if (i > sectionStart) {
            foldingRanges.push({
              startLine: sectionStart,
              endLine: i - 1,
              kind: FoldingRangeKind.Region,
            });
          }
          indentLevel = lineIndent;
          sectionStart = i;
        }
      }

      // Close last section
      if (currentSection && lines.length > sectionStart) {
        foldingRanges.push({
          startLine: sectionStart,
          endLine: lines.length - 1,
          kind: FoldingRangeKind.Region,
        });
      }

      return foldingRanges;
    } catch (error) {
      console.error('Error providing folding ranges:', error);
      return [];
    }
  }

  private formatYaml(text: string, options: FormattingOptions): string {
    try {
      // Parse the YAML to validate it
      const parsed = yaml.parse(text);

      if (!parsed) {
        return text; // Return original if parsing fails
      }

      // Format with YAML library
      const formatted = yaml.stringify(parsed, {
        indent: options.tabSize || 2,
        lineWidth: options.insertSpaces ? 80 : -1,
        minContentWidth: 20,
        indentSeq: true,
        simpleKeys: false,
        nullStr: 'null',
        trueStr: 'true',
        falseStr: 'false',
      });

      return formatted;
    } catch (error) {
      console.error('Error formatting YAML:', error);
      return text; // Return original if formatting fails
    }
  }

  private extractRangeText(text: string, range: Range): string {
    const lines = text.split('\n');
    const startLine = range.start.line;
    const endLine = range.end.line;
    const startChar = range.start.character;
    const endChar = range.end.character;

    if (startLine === endLine) {
      return lines[startLine].substring(startChar, endChar);
    }

    const rangeLines = [lines[startLine].substring(startChar)];

    for (let i = startLine + 1; i < endLine; i++) {
      rangeLines.push(lines[i]);
    }

    if (endLine < lines.length) {
      rangeLines.push(lines[endLine].substring(0, endChar));
    }

    return rangeLines.join('\n');
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

  // Additional formatting utilities
  formatArray(array: any[], options: FormattingOptions): string {
    return yaml.stringify(array, {
      indent: options.tabSize || 2,
      lineWidth: options.insertSpaces ? 80 : -1,
      indentSeq: true,
    });
  }

  formatObject(obj: any, options: FormattingOptions): string {
    return yaml.stringify(obj, {
      indent: options.tabSize || 2,
      lineWidth: options.insertSpaces ? 80 : -1,
      simpleKeys: false,
    });
  }

  // Validate YAML structure
  validateYamlStructure(text: string): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    try {
      const parsed = yaml.parse(text);
      if (!parsed) {
        errors.push('Empty or invalid YAML content');
      }
    } catch (error: any) {
      errors.push(`YAML parsing error: ${error.message}`);
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }

  // Get YAML structure information
  getYamlStructure(text: string): { keys: string[]; depth: number } {
    try {
      const parsed = yaml.parse(text);
      if (!parsed) {
        return { keys: [], depth: 0 };
      }

      const keys = this.extractKeys(parsed);
      const depth = this.getMaxDepth(parsed);

      return { keys, depth };
    } catch (error) {
      return { keys: [], depth: 0 };
    }
  }

  private extractKeys(obj: any, prefix = ''): string[] {
    const keys: string[] = [];

    if (typeof obj === 'object' && obj !== null) {
      Object.keys(obj).forEach((key) => {
        const fullKey = prefix ? `${prefix}.${key}` : key;
        keys.push(fullKey);

        if (typeof obj[key] === 'object' && obj[key] !== null && !Array.isArray(obj[key])) {
          keys.push(...this.extractKeys(obj[key], fullKey));
        }
      });
    }

    return keys;
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

  private getDocumentRange(document: TextDocument): Range {
    const lines = document.getText().split('\n');
    const lastLine = lines[lines.length - 1];
    return Range.create(
      Position.create(0, 0),
      Position.create(document.lineCount - 1, lastLine.length)
    );
  }
}
