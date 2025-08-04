import {
  Location,
  Position,
  type TextDocument,
  Range,
  type ReferenceContext,
} from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';

export class RhemaReferenceProvider {
  private capabilities: any;

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  provideReferences(
    document: TextDocument,
    position: Position,
    context: ReferenceContext,
    cachedDocument?: any
  ): Location[] {
    try {
      const text = document.getText();
      const word = this.getWordAtPosition(text, position);

      if (!word) {
        return [];
      }

      const references: Location[] = [];

      // Find local references
      const localReferences = this.findLocalReferences(word, text, document.uri);
      references.push(...localReferences);

      // Find workspace references (if includeDeclaration is true)
      if (context.includeDeclaration) {
        const workspaceReferences = this.findWorkspaceReferences(word, document.uri);
        references.push(...workspaceReferences);
      }

      return references;
    } catch (error) {
      console.error('Error providing references:', error);
      return [];
    }
  }

  private getWordAtPosition(text: string, position: Position): string | null {
    const lines = text.split('\n');
    const line = lines[position.line];

    if (!line) {
      return null;
    }

    // Find the word at the current position
    const beforeCursor = line.substring(0, position.character);

    // Match YAML key patterns
    const keyMatch = beforeCursor.match(/([a-zA-Z_][a-zA-Z0-9_]*)\s*:?\s*$/);
    if (keyMatch) {
      return keyMatch[1];
    }

    // Match quoted strings
    const stringMatch = beforeCursor.match(/"([^"]*)"\s*$/);
    if (stringMatch) {
      return stringMatch[1];
    }

    const singleQuoteMatch = beforeCursor.match(/'([^']*)'\s*$/);
    if (singleQuoteMatch) {
      return singleQuoteMatch[1];
    }

    return null;
  }

  private findLocalReferences(word: string, text: string, uri: string): Location[] {
    const references: Location[] = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      // Find all occurrences of the word in this line
      let index = 0;
      while (true) {
        index = line.indexOf(word, index);
        if (index === -1) break;
        // Check if this is a valid reference (not part of another word)
        const beforeChar = index > 0 ? line[index - 1] : ' ';
        const afterChar = index + word.length < line.length ? line[index + word.length] : ' ';

        if (!this.isWordCharacter(beforeChar) && !this.isWordCharacter(afterChar)) {
          references.push(
            Location.create(
              uri,
              Range.create(Position.create(i, index), Position.create(i, index + word.length))
            )
          );
        }

        index += word.length;
      }
    }

    return references;
  }

  private findWorkspaceReferences(word: string, currentUri: string): Location[] {
    // This would typically search through all workspace files
    // For now, we'll return an empty array as this requires workspace file access
    // In a real implementation, you would:
    // 1. Get all Rhema files in the workspace
    // 2. Search each file for references to the word
    // 3. Return locations for all found references

    return [];
  }

  private isWordCharacter(char: string): boolean {
    return /[a-zA-Z0-9_]/.test(char);
  }

  // Helper method to find all references in a specific range
  findReferencesInRange(word: string, text: string, range: Range, uri: string): Location[] {
    const references: Location[] = [];
    const lines = text.split('\n');

    for (let i = range.start.line; i <= range.end.line; i++) {
      const line = lines[i];
      if (!line) continue;

      const startChar = i === range.start.line ? range.start.character : 0;
      const endChar = i === range.end.line ? range.end.character : line.length;
      const lineSegment = line.substring(startChar, endChar);

      let index = startChar;
      while (true) {
        index = line.indexOf(word, index);
        if (index === -1 || index >= endChar) break;
        // Check if this is a valid reference
        const beforeChar = index > 0 ? line[index - 1] : ' ';
        const afterChar = index + word.length < line.length ? line[index + word.length] : ' ';

        if (!this.isWordCharacter(beforeChar) && !this.isWordCharacter(afterChar)) {
          references.push(
            Location.create(
              uri,
              Range.create(Position.create(i, index), Position.create(i, index + word.length))
            )
          );
        }

        index += word.length;
      }
    }

    return references;
  }

  // Helper method to get reference count
  getReferenceCount(word: string, text: string): number {
    const lines = text.split('\n');
    let count = 0;

    for (const line of lines) {
      let index = 0;
      while (true) {
        index = line.indexOf(word, index);
        if (index === -1) break;
        // Check if this is a valid reference
        const beforeChar = index > 0 ? line[index - 1] : ' ';
        const afterChar = index + word.length < line.length ? line[index + word.length] : ' ';

        if (!this.isWordCharacter(beforeChar) && !this.isWordCharacter(afterChar)) {
          count++;
        }

        index += word.length;
      }
    }

    return count;
  }
}
