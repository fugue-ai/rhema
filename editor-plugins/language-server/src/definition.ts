import {
  type Definition,
  Location,
  Position,
  type TextDocument,
  Range,
} from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';

export class RhemaDefinitionProvider {
  private capabilities: any;

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  provideDefinition(
    document: TextDocument,
    position: Position,
    cachedDocument?: any
  ): Definition | null {
    try {
      const text = document.getText();
      const word = this.getWordAtPosition(text, position);

      if (!word) {
        return null;
      }

      // Check if it's a reference to another document
      const referenceLocation = this.findReferenceLocation(word, document.uri, cachedDocument);
      if (referenceLocation) {
        return referenceLocation;
      }

      // Check if it's a local definition
      const localLocation = this.findLocalDefinition(word, text, position);
      if (localLocation) {
        return Location.create(document.uri, localLocation);
      }

      return null;
    } catch (error) {
      console.error('Error providing definition:', error);
      return null;
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

  private findReferenceLocation(
    word: string,
    currentUri: string,
    cachedDocument?: any
  ): Location | null {
    // This would typically search through workspace files
    // For now, we'll implement a simple version that looks for common patterns

    // Check if it's a reference to a scope
    if (word.includes('scope') || word.includes('Scope')) {
      // Look for scope files in the workspace
      // This would be implemented with workspace file search
      return null;
    }

    // Check if it's a reference to a pattern
    if (word.includes('pattern') || word.includes('Pattern')) {
      // Look for pattern definitions
      return null;
    }

    // Check if it's a reference to a convention
    if (word.includes('convention') || word.includes('Convention')) {
      // Look for convention definitions
      return null;
    }

    return null;
  }

  private findLocalDefinition(word: string, text: string, position: Position): Range | null {
    const lines = text.split('\n');

    // Search backwards from current position to find the definition
    for (let i = position.line; i >= 0; i--) {
      const line = lines[i];
      const trimmedLine = line.trim();

      // Look for key definitions (key: value)
      const keyMatch = trimmedLine.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (keyMatch && keyMatch[1] === word) {
        const startChar = line.indexOf(keyMatch[1]);
        return Range.create(
          Position.create(i, startChar),
          Position.create(i, startChar + word.length)
        );
      }

      // Look for array item definitions
      const arrayMatch = trimmedLine.match(/^\s*-\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (arrayMatch && arrayMatch[1] === word) {
        const startChar = line.indexOf(arrayMatch[1]);
        return Range.create(
          Position.create(i, startChar),
          Position.create(i, startChar + word.length)
        );
      }
    }

    return null;
  }

  // Helper method to find all definitions in a document
  findAllDefinitions(text: string): Array<{ word: string; range: Range }> {
    const definitions: Array<{ word: string; range: Range }> = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const trimmedLine = line.trim();

      // Find key definitions
      const keyMatch = trimmedLine.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (keyMatch) {
        const word = keyMatch[1];
        const startChar = line.indexOf(word);
        definitions.push({
          word,
          range: Range.create(
            Position.create(i, startChar),
            Position.create(i, startChar + word.length)
          ),
        });
      }

      // Find array item definitions
      const arrayMatch = trimmedLine.match(/^\s*-\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (arrayMatch) {
        const word = arrayMatch[1];
        const startChar = line.indexOf(word);
        definitions.push({
          word,
          range: Range.create(
            Position.create(i, startChar),
            Position.create(i, startChar + word.length)
          ),
        });
      }
    }

    return definitions;
  }

  // Helper method to check if a word is a definition
  isDefinition(word: string, text: string, position: Position): boolean {
    const definition = this.findLocalDefinition(word, text, position);
    return definition !== null;
  }

  // Helper method to get the scope of a definition
  getDefinitionScope(word: string, text: string, position: Position): string | null {
    const lines = text.split('\n');
    const currentLine = position.line;

    // Find the parent scope by looking for the nearest parent key
    for (let i = currentLine; i >= 0; i--) {
      const line = lines[i];
      const trimmedLine = line.trim();

      // Look for parent keys (less indentation)
      const keyMatch = trimmedLine.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:/);
      if (keyMatch) {
        return keyMatch[1];
      }
    }

    return null;
  }
}
