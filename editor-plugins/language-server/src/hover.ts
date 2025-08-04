import {
  type Hover,
  type MarkedString,
  type Position,
  type TextDocument,
  MarkupKind,
} from 'vscode-languageserver/node';
import type { RhemaDocument } from './parser';

export class RhemaHoverProvider {
  private capabilities: any;
  private keywordDocumentation: { [key: string]: string } = {};

  constructor() {
    this.initializeKeywordDocumentation();
  }

  initialize(capabilities: any): void {
    this.capabilities = capabilities;
  }

  provideHover(document: TextDocument, position: Position, cachedDocument?: any): Hover | null {
    try {
      const text = document.getText();
      const word = this.getWordAtPosition(text, position);

      if (!word) {
        return null;
      }

      const hoverContent = this.getHoverContent(word, cachedDocument);

      if (!hoverContent) {
        return null;
      }

      return {
        contents: hoverContent,
      };
    } catch (error) {
      console.error('Error providing hover:', error);
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
    const afterCursor = line.substring(position.character);

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

  private getHoverContent(word: string, cachedDocument?: any): MarkedString[] | null {
    const contents: MarkedString[] = [];

    // Check if it's a keyword
    const keywordDoc = this.keywordDocumentation[word];
    if (keywordDoc) {
      contents.push({
        language: 'yaml',
        value: `${word}:`,
      });
      contents.push(keywordDoc);
      return contents;
    }

    // Check if it's a document type
    const documentTypeDoc = this.getDocumentTypeDocumentation(word);
    if (documentTypeDoc) {
      contents.push({
        language: 'yaml',
        value: `Document Type: ${word}`,
      });
      contents.push(documentTypeDoc);
      return contents;
    }

    // Check if it's a value in the document
    if (cachedDocument?.data) {
      const valueDoc = this.getValueDocumentation(word, cachedDocument.data);
      if (valueDoc) {
        contents.push({
          language: 'yaml',
          value: `Value: ${word}`,
        });
        contents.push(valueDoc);
        return contents;
      }
    }

    // Check if it's a YAML syntax element
    const syntaxDoc = this.getSyntaxDocumentation(word);
    if (syntaxDoc) {
      contents.push({
        language: 'yaml',
        value: `YAML Syntax: ${word}`,
      });
      contents.push(syntaxDoc);
      return contents;
    }

    return null;
  }

  private initializeKeywordDocumentation(): void {
    this.keywordDocumentation = {
      name: 'The name of this Rhema document. This should be a descriptive, unique identifier.',
      description: 'A detailed description of what this document represents and its purpose.',
      version:
        'Semantic version of this document (e.g., "1.0.0"). Follows semver.org specification.',
      metadata:
        'Additional metadata about the document including creation date, author, and other properties.',
      contexts:
        'Defines the contexts for this scope. Each context represents a different aspect or domain.',
      dependencies:
        'External dependencies required by this scope. Can include other scopes, libraries, or services.',
      config:
        'Configuration settings specific to this scope. Contains environment-specific or user-specific settings.',
      patterns:
        'Reusable patterns identified in this knowledge base or project. Includes design patterns, code patterns, etc.',
      conventions:
        'Conventions and standards followed in this project. Includes coding standards, naming conventions, etc.',
      tasks:
        'List of tasks to be completed. Each task should have a title, description, and status.',
      decisions:
        'Record of decisions made during the project. Includes rationale and context for each decision.',
      created: 'Timestamp when this document was created (ISO 8601 format).',
      modified: 'Timestamp when this document was last modified (ISO 8601 format).',
      author: 'The author or creator of this document.',
      status: 'Current status of this item (e.g., "pending", "in-progress", "completed").',
      priority: 'Priority level of this item (e.g., "low", "medium", "high", "critical").',
      tags: 'Tags or labels associated with this item for categorization.',
      examples: 'Example usage or implementation of this pattern or convention.',
      benefits: 'Benefits or advantages of using this pattern or following this convention.',
      rules: 'Specific rules or guidelines that define this convention.',
      rationale: 'Explanation of why a particular decision was made.',
      date: 'Date when this decision was made or this item was created.',
      references: 'References to external documentation, standards, or related items.',
      notes: 'Additional notes or comments about this item.',
      type: 'Type or category of this item.',
      scope: 'Scope or context where this item applies.',
      owner: 'Person or team responsible for this item.',
      reviewer: 'Person or team responsible for reviewing this item.',
      approver: 'Person or team responsible for approving this item.',
    };
  }

  private getDocumentTypeDocumentation(type: string): string | null {
    const typeDocumentation: { [key: string]: string } = {
      scope:
        'A scope document defines the boundaries and context of a project or component. It includes contexts, dependencies, and configuration.',
      knowledge:
        'A knowledge document captures patterns, conventions, and contextual information learned during development.',
      todos: 'A todos document tracks tasks, issues, and work items that need to be completed.',
      decisions:
        'A decisions document records important decisions made during the project, including rationale and context.',
      patterns: 'A patterns document defines reusable patterns identified in the project.',
      conventions:
        'A conventions document defines standards and conventions followed in the project.',
    };

    return typeDocumentation[type] || null;
  }

  private getValueDocumentation(value: string, document: RhemaDocument): string | null {
    // Check if the value is a status
    const statuses = ['pending', 'in-progress', 'completed', 'blocked', 'cancelled'];
    if (statuses.includes(value.toLowerCase())) {
      return `Status: ${value}. This indicates the current state of the item.`;
    }

    // Check if the value is a priority
    const priorities = ['low', 'medium', 'high', 'critical'];
    if (priorities.includes(value.toLowerCase())) {
      return `Priority: ${value}. This indicates the importance or urgency of the item.`;
    }

    // Check if the value is a version
    if (/^\d+\.\d+\.\d+/.test(value)) {
      return `Version: ${value}. This follows semantic versioning (semver.org).`;
    }

    // Check if the value is a date
    if (/^\d{4}-\d{2}-\d{2}/.test(value)) {
      return `Date: ${value}. This should be in ISO 8601 format (YYYY-MM-DD).`;
    }

    return null;
  }

  private getSyntaxDocumentation(syntax: string): string | null {
    const syntaxDocumentation: { [key: string]: string } = {
      '---': 'YAML document separator. Marks the beginning of a YAML document.',
      '...': 'YAML document end marker. Marks the end of a YAML document.',
      '&': 'YAML anchor. Creates a reference point for aliases.',
      '*': 'YAML alias. References a previously defined anchor.',
      '!': 'YAML tag. Specifies the type of a node.',
      '|': 'Literal block scalar. Preserves newlines and formatting.',
      '>': 'Folded block scalar. Folds newlines to spaces.',
      '-': 'List item indicator. Marks the beginning of an array item.',
      ':': 'Key-value separator. Separates keys from values in mappings.',
      '?': 'Complex key indicator. Used for complex mapping keys.',
      '[': 'Flow sequence start. Begins an inline array.',
      ']': 'Flow sequence end. Ends an inline array.',
      '{': 'Flow mapping start. Begins an inline object.',
      '}': 'Flow mapping end. Ends an inline object.',
      ',': 'Flow collection separator. Separates items in flow collections.',
      '"': 'Double-quoted string. Allows escape sequences.',
      "'": 'Single-quoted string. No escape sequences allowed.',
      '#': 'Comment. Everything after # is ignored.',
      '%': 'Directive indicator. Used for YAML directives.',
    };

    return syntaxDocumentation[syntax] || null;
  }
}
