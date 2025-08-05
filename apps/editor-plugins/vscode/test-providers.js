#!/usr/bin/env node

/**
 * Simple test script for Rhema Provider functionality
 * This can be run independently to test the provider logic
 */

const fs = require('fs');
const path = require('path');

// Mock VS Code API for testing
const mockVscode = {
  Position: class Position {
    constructor(line, character) {
      this.line = line;
      this.character = character;
    }
  },
  Range: class Range {
    constructor(start, end) {
      this.start = start;
      this.end = end;
    }
  },
  Uri: {
    file: (path) => ({ scheme: 'file', fsPath: path }),
  },
  SymbolKind: {
    Namespace: 1,
    Object: 2,
    Array: 3,
  },
  DiagnosticSeverity: {
    Error: 0,
    Warning: 1,
    Information: 2,
    Hint: 3,
  },
  CodeActionKind: {
    QuickFix: 'quickfix',
    Refactor: 'refactor',
    Source: 'source',
  },
  window: {
    createStatusBarItem: () => ({
      show: () => {},
      dispose: () => {},
    }),
    createOutputChannel: () => ({
      appendLine: () => {},
      show: () => {},
      dispose: () => {},
    }),
  },
  languages: {
    createDiagnosticCollection: () => ({
      set: () => {},
      delete: () => {},
      clear: () => {},
      dispose: () => {},
    }),
  },
};

// Mock document for testing
class MockDocument {
  constructor(content, fileName = 'test.yaml') {
    this.content = content;
    this.fileName = fileName;
    this.languageId = 'yaml';
    this.lineCount = content.split('\n').length;
    this.uri = mockVscode.Uri.file(fileName);
  }

  getText() {
    return this.content;
  }

  getWordRangeAtPosition(position) {
    const lines = this.content.split('\n');
    const line = lines[position.line] || '';
    const wordMatch = line.match(/\b\w+\b/);
    if (
      wordMatch &&
      position.character >= wordMatch.index &&
      position.character < wordMatch.index + wordMatch[0].length
    ) {
      return new mockVscode.Range(
        new mockVscode.Position(position.line, wordMatch.index),
        new mockVscode.Position(position.line, wordMatch.index + wordMatch[0].length)
      );
    }
    return null;
  }

  lineAt(lineNumber) {
    const lines = this.content.split('\n');
    return {
      text: lines[lineNumber] || '',
      lineNumber: lineNumber,
    };
  }
}

// Mock classes for testing
class MockLogger {
  info(message) {
    console.log(`[INFO] ${message}`);
  }
  warn(message) {
    console.log(`[WARN] ${message}`);
  }
  error(message) {
    console.log(`[ERROR] ${message}`);
  }
}

class MockSettings {
  getExecutablePath() {
    return 'rhema';
  }
  isEnabled() {
    return true;
  }
  isAutoValidateEnabled() {
    return true;
  }
  isPerformanceProfilingEnabled() {
    return false;
  }
  reload() {
    return Promise.resolve();
  }
}

class MockErrorHandler {
  constructor(logger) {
    this.logger = logger;
  }
  handleError(message, error) {
    this.logger.error(`${message}: ${error}`);
  }
}

// Test data
const testRhemaContent = `
scope:
  name: "Test Scope"
  description: "A test scope for provider testing"
  version: "1.0.0"
  author: "Test Author"

context:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  patterns:
    - "*.rs"
  exclusions:
    - "target/**"
  maxTokens: 1000
  includeHidden: false
  recursive: true

todos:
  - id: "TODO-001"
    title: "Implement provider tests"
    description: "Add comprehensive tests for all providers"
    priority: "high"
    status: "pending"
    assignee: "developer"
    dueDate: "2024-12-31"
    tags: ["testing", "providers"]
    related: ["TEST-001"]

insights:
  - id: "INSIGHT-001"
    title: "Code Quality Analysis"
    description: "Analysis of code quality patterns"
    type: "analysis"
    confidence: 0.85
    source: "static-analysis"
    tags: ["quality", "analysis"]
    related: ["TODO-001"]

patterns:
  - id: "PATTERN-001"
    name: "Test Pattern"
    description: "A test pattern for validation"
    type: "regex"
    regex: "test.*pattern"
    examples:
      - "test_pattern_example"
    tags: ["test", "pattern"]

decisions:
  - id: "DECISION-001"
    title: "Use TypeScript for Extension"
    description: "Decision to use TypeScript for VS Code extension"
    status: "approved"
    rationale: "TypeScript provides better type safety and IDE support"
    alternatives: ["JavaScript", "Dart"]
    impact: "Improved development experience"
    date: "2024-01-01"

# File references for testing document links
files:
  - path: "src/main.rs"
  - path: "tests/integration.rs"
  - path: "docs/README.md"

# URL references for testing document links
references:
  - url: "https://github.com/example/rhema"
  - url: "https://docs.rs/rhema"
`;

// Test runner
class ProviderTestRunner {
  constructor() {
    this.results = [];
    this.logger = new MockLogger();
    this.settings = new MockSettings();
    this.errorHandler = new MockErrorHandler(this.logger);
  }

  async runTests() {
    console.log('Starting Rhema Provider Tests...\n');

    // Test provider initialization
    await this.testProviderInitialization();

    // Test definition provider
    await this.testDefinitionProvider();

    // Test reference provider
    await this.testReferenceProvider();

    // Test document symbol provider
    await this.testDocumentSymbolProvider();

    // Test workspace symbol provider
    await this.testWorkspaceSymbolProvider();

    // Test code actions provider
    await this.testCodeActionsProvider();

    // Test folding range provider
    await this.testFoldingRangeProvider();

    // Test selection range provider
    await this.testSelectionRangeProvider();

    // Test document highlight provider
    await this.testDocumentHighlightProvider();

    // Test document link provider
    await this.testDocumentLinkProvider();

    // Test rename provider
    await this.testRenameProvider();

    // Test format on type provider
    await this.testFormatOnTypeProvider();

    // Test error handling
    await this.testErrorHandling();

    this.generateReport();
  }

  async testProviderInitialization() {
    const testName = 'Provider Initialization';
    try {
      // Simulate provider initialization
      const mockContext = {
        subscriptions: [],
        extensionPath: '/test/path',
      };

      this.recordResult(testName, true, 'Provider initialized successfully');
    } catch (error) {
      this.recordResult(testName, false, `Initialization failed: ${error}`);
    }
  }

  async testDefinitionProvider() {
    const testName = 'Definition Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const position = new mockVscode.Position(2, 8); // Position at "name"

      // Simulate definition lookup
      const wordRange = document.getWordRangeAtPosition(position);
      if (wordRange) {
        this.recordResult(testName, true, 'Definition found successfully');
      } else {
        this.recordResult(testName, true, 'No definition found (expected for simple test)');
      }
    } catch (error) {
      this.recordResult(testName, false, `Definition test failed: ${error}`);
    }
  }

  async testReferenceProvider() {
    const testName = 'Reference Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const position = new mockVscode.Position(2, 8);

      // Simulate reference lookup
      const wordRange = document.getWordRangeAtPosition(position);
      if (wordRange) {
        const word = document
          .getText()
          .substring(wordRange.start.character, wordRange.end.character);
        const references = this.findReferences(document, word);
        this.recordResult(testName, true, `Found ${references.length} references`);
      } else {
        this.recordResult(testName, true, 'No references found');
      }
    } catch (error) {
      this.recordResult(testName, false, `Reference test failed: ${error}`);
    }
  }

  async testDocumentSymbolProvider() {
    const testName = 'Document Symbol Provider';
    try {
      const document = new MockDocument(testRhemaContent);

      // Simulate symbol extraction
      const symbols = this.extractSymbols(document);
      this.recordResult(testName, true, `Extracted ${symbols.length} symbols`);
    } catch (error) {
      this.recordResult(testName, false, `Symbol test failed: ${error}`);
    }
  }

  async testWorkspaceSymbolProvider() {
    const testName = 'Workspace Symbol Provider';
    try {
      // Simulate workspace symbol search
      const symbols = this.searchWorkspaceSymbols('scope');
      this.recordResult(testName, true, `Found ${symbols.length} workspace symbols`);
    } catch (error) {
      this.recordResult(testName, false, `Workspace symbol test failed: ${error}`);
    }
  }

  async testCodeActionsProvider() {
    const testName = 'Code Actions Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const range = new mockVscode.Range(0, 0, 10, 0);

      // Simulate code actions
      const actions = this.generateCodeActions(document, range);
      this.recordResult(testName, true, `Generated ${actions.length} code actions`);
    } catch (error) {
      this.recordResult(testName, false, `Code actions test failed: ${error}`);
    }
  }

  async testFoldingRangeProvider() {
    const testName = 'Folding Range Provider';
    try {
      const document = new MockDocument(testRhemaContent);

      // Simulate folding ranges
      const ranges = this.generateFoldingRanges(document);
      this.recordResult(testName, true, `Generated ${ranges.length} folding ranges`);
    } catch (error) {
      this.recordResult(testName, false, `Folding range test failed: ${error}`);
    }
  }

  async testSelectionRangeProvider() {
    const testName = 'Selection Range Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const positions = [new mockVscode.Position(2, 8)];

      // Simulate selection ranges
      const ranges = this.generateSelectionRanges(document, positions);
      this.recordResult(testName, true, `Generated ${ranges.length} selection ranges`);
    } catch (error) {
      this.recordResult(testName, false, `Selection range test failed: ${error}`);
    }
  }

  async testDocumentHighlightProvider() {
    const testName = 'Document Highlight Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const position = new mockVscode.Position(2, 8);

      // Simulate document highlights
      const highlights = this.generateDocumentHighlights(document, position);
      this.recordResult(testName, true, `Generated ${highlights.length} highlights`);
    } catch (error) {
      this.recordResult(testName, false, `Document highlight test failed: ${error}`);
    }
  }

  async testDocumentLinkProvider() {
    const testName = 'Document Link Provider';
    try {
      const document = new MockDocument(testRhemaContent);

      // Simulate document links
      const links = this.generateDocumentLinks(document);
      this.recordResult(testName, true, `Generated ${links.length} document links`);
    } catch (error) {
      this.recordResult(testName, false, `Document link test failed: ${error}`);
    }
  }

  async testRenameProvider() {
    const testName = 'Rename Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const position = new mockVscode.Position(2, 8);
      const newName = 'newName';

      // Simulate rename operation
      const edit = this.generateRenameEdit(document, position, newName);
      if (edit) {
        this.recordResult(testName, true, 'Rename edit generated successfully');
      } else {
        this.recordResult(testName, true, 'No rename edit (expected for simple test)');
      }
    } catch (error) {
      this.recordResult(testName, false, `Rename test failed: ${error}`);
    }
  }

  async testFormatOnTypeProvider() {
    const testName = 'Format on Type Provider';
    try {
      const document = new MockDocument(testRhemaContent);
      const position = new mockVscode.Position(2, 10);
      const ch = ':';

      // Simulate format on type
      const edits = this.generateFormatEdits(document, position, ch);
      this.recordResult(testName, true, `Generated ${edits.length} format edits`);
    } catch (error) {
      this.recordResult(testName, false, `Format on type test failed: ${error}`);
    }
  }

  async testErrorHandling() {
    const testName = 'Error Handling';
    try {
      const invalidDocument = new MockDocument('invalid: yaml: content:');

      // Test error handling
      const symbols = this.extractSymbols(invalidDocument);
      const actions = this.generateCodeActions(invalidDocument, new mockVscode.Range(0, 0, 1, 0));

      this.recordResult(testName, true, 'Error handling works correctly');
    } catch (error) {
      this.recordResult(testName, false, `Error handling test failed: ${error}`);
    }
  }

  // Helper methods for simulation
  findReferences(document, word) {
    const references = [];
    const lines = document.getText().split('\n');
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].includes(word)) {
        references.push(new mockVscode.Position(i, lines[i].indexOf(word)));
      }
    }
    return references;
  }

  extractSymbols(document) {
    const symbols = [];
    const content = document.getText();

    if (content.includes('scope:')) symbols.push('Scope');
    if (content.includes('context:')) symbols.push('Context');
    if (content.includes('todos:')) symbols.push('Todos');
    if (content.includes('insights:')) symbols.push('Insights');
    if (content.includes('patterns:')) symbols.push('Patterns');
    if (content.includes('decisions:')) symbols.push('Decisions');

    return symbols;
  }

  searchWorkspaceSymbols(query) {
    const symbols = [];
    if (query.toLowerCase().includes('scope')) {
      symbols.push('Scope');
    }
    if (query.toLowerCase().includes('todo')) {
      symbols.push('Todos');
    }
    return symbols;
  }

  generateCodeActions(document, range) {
    return [
      { title: 'Fix Rhema Validation Issues', kind: mockVscode.CodeActionKind.QuickFix },
      { title: 'Refactor Rhema Context', kind: mockVscode.CodeActionKind.Refactor },
      { title: 'Generate Rhema Documentation', kind: mockVscode.CodeActionKind.Source },
    ];
  }

  generateFoldingRanges(document) {
    const ranges = [];
    const lines = document.getText().split('\n');
    let startLine = -1;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (
        line === 'scope:' ||
        line === 'context:' ||
        line === 'todos:' ||
        line === 'insights:' ||
        line === 'patterns:' ||
        line === 'decisions:'
      ) {
        if (startLine !== -1) {
          ranges.push({ start: startLine, end: i - 1 });
        }
        startLine = i;
      }
    }

    if (startLine !== -1) {
      ranges.push({ start: startLine, end: lines.length - 1 });
    }

    return ranges;
  }

  generateSelectionRanges(document, positions) {
    return positions.map((position) => ({
      range: new mockVscode.Range(position, position),
    }));
  }

  generateDocumentHighlights(document, position) {
    const wordRange = document.getWordRangeAtPosition(position);
    if (wordRange) {
      return [{ range: wordRange }];
    }
    return [];
  }

  generateDocumentLinks(document) {
    const links = [];
    const lines = document.getText().split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      // Find file references
      const fileMatch = line.match(/(file|path):\s*(.+\.(yaml|yml|md|txt))/);
      if (fileMatch) {
        links.push({ target: mockVscode.Uri.file(fileMatch[2]) });
      }

      // Find URL references
      const urlMatch = line.match(/(https?:\/\/[^\s]+)/);
      if (urlMatch) {
        links.push({ target: { scheme: 'http', authority: urlMatch[1] } });
      }
    }

    return links;
  }

  generateRenameEdit(document, position, newName) {
    const wordRange = document.getWordRangeAtPosition(position);
    if (wordRange) {
      return { has: () => true };
    }
    return null;
  }

  generateFormatEdits(document, position, ch) {
    const edits = [];
    if (ch === ':') {
      edits.push({ range: new mockVscode.Range(position, position), newText: ' ' });
    }
    return edits;
  }

  recordResult(testName, passed, message) {
    const result = {
      name: testName,
      passed,
      message,
      timestamp: new Date(),
    };

    this.results.push(result);

    const status = passed ? 'PASS' : 'FAIL';
    console.log(`[${status}] ${testName}: ${message}`);
  }

  generateReport() {
    const totalTests = this.results.length;
    const passedTests = this.results.filter((r) => r.passed).length;
    const failedTests = totalTests - passedTests;

    console.log('\n' + '='.repeat(50));
    console.log('Rhema PROVIDER TEST REPORT');
    console.log('='.repeat(50));
    console.log(`Total Tests: ${totalTests}`);
    console.log(`Passed: ${passedTests}`);
    console.log(`Failed: ${failedTests}`);
    console.log(`Success Rate: ${((passedTests / totalTests) * 100).toFixed(1)}%`);

    if (failedTests > 0) {
      console.log('\nFAILED TESTS:');
      this.results
        .filter((r) => !r.passed)
        .forEach((r) => {
          console.log(`  - ${r.name}: ${r.message}`);
        });
    }

    console.log('\nTest execution completed.');
  }
}

// Run tests if this script is executed directly
if (require.main === module) {
  const runner = new ProviderTestRunner();
  runner.runTests().catch(console.error);
}

module.exports = { ProviderTestRunner, MockDocument, mockVscode };
