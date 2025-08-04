/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import * as assert from 'assert';
import * as vscode from 'vscode';
import { RhemaProvider } from '../src/providers/rhemaProvider';

suite('Rhema Provider Tests', () => {
  let provider: RhemaProvider;
  let testDocument: vscode.TextDocument;

  setup(async () => {
    provider = new RhemaProvider();

    // Create a test Rhema document
    const testContent = `
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

  - id: "TODO-002"
    title: "Fix linter errors"
    description: "Resolve TypeScript linter issues"
    priority: "medium"
    status: "in-progress"
    assignee: "developer"
    dueDate: "2024-12-15"
    tags: ["linting", "typescript"]
    related: ["BUG-001"]

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

    testDocument = await vscode.workspace.openTextDocument({
      content: testContent,
      language: 'yaml',
    });
  });

  teardown(async () => {
    await provider.dispose();
  });

  test('Provider Initialization', async () => {
    const mockContext = {
      subscriptions: [],
      extensionPath: '/test/path',
    } as vscode.ExtensionContext;

    await provider.initialize(mockContext);

    // Verify provider is initialized
    assert.ok(provider, 'Provider should be initialized');
  });

  test('Definition Provider - Find Definition in Document', async () => {
    const position = new vscode.Position(2, 8); // Position at "name" in scope.name
    const definition = await provider.provideDefinition(
      testDocument,
      position,
      {} as vscode.CancellationToken
    );

    assert.ok(definition, 'Should find definition');
    if (definition && 'uri' in definition) {
      assert.strictEqual(definition.uri, testDocument.uri, 'Definition should be in same document');
    }
  });

  test('Definition Provider - No Definition for Unknown Symbol', async () => {
    const position = new vscode.Position(1, 5); // Position at unknown symbol
    const definition = await provider.provideDefinition(
      testDocument,
      position,
      {} as vscode.CancellationToken
    );

    assert.strictEqual(definition, undefined, 'Should return undefined for unknown symbol');
  });

  test('Reference Provider - Find References in Document', async () => {
    const position = new vscode.Position(2, 8); // Position at "name"
    const context = { includeDeclaration: true } as vscode.ReferenceContext;
    const references = await provider.provideReferences(
      testDocument,
      position,
      context,
      {} as vscode.CancellationToken
    );

    assert.ok(references.length > 0, 'Should find references');
    assert.ok(
      references.some((ref) => ref.uri === testDocument.uri),
      'Should find reference in current document'
    );
  });

  test('Document Symbol Provider - Extract Symbols', async () => {
    const symbols = await provider.provideDocumentSymbols(
      testDocument,
      {} as vscode.CancellationToken
    );

    assert.ok(symbols.length > 0, 'Should extract symbols');

    // Check for major Rhema sections
    const symbolNames = symbols.map((s) => s.name);
    assert.ok(symbolNames.includes('Scope'), 'Should include Scope symbol');
    assert.ok(symbolNames.includes('Context'), 'Should include Context symbol');
    assert.ok(symbolNames.includes('Todos'), 'Should include Todos symbol');
    assert.ok(symbolNames.includes('Insights'), 'Should include Insights symbol');
    assert.ok(symbolNames.includes('Patterns'), 'Should include Patterns symbol');
    assert.ok(symbolNames.includes('Decisions'), 'Should include Decisions symbol');
  });

  test('Workspace Symbol Provider - Search Symbols', async () => {
    const symbols = await provider.provideWorkspaceSymbols('scope', {} as vscode.CancellationToken);

    assert.ok(symbols.length > 0, 'Should find workspace symbols');
    assert.ok(
      symbols.some((s) => s.name.toLowerCase().includes('scope')),
      'Should find scope-related symbols'
    );
  });

  test('Code Actions Provider - Provide Actions', async () => {
    const range = new vscode.Range(0, 0, 10, 0);
    const context = { diagnostics: [] } as vscode.CodeActionContext;
    const actions = await provider.provideCodeActions(
      testDocument,
      range,
      context,
      {} as vscode.CancellationToken
    );

    assert.ok(actions.length > 0, 'Should provide code actions');

    // Check for different action types
    const actionTitles = actions.map((a) => a.title);
    assert.ok(
      actionTitles.some((title) => title.includes('Refactor')),
      'Should include refactoring action'
    );
    assert.ok(
      actionTitles.some((title) => title.includes('Documentation')),
      'Should include documentation action'
    );
  });

  test('Code Actions Provider - Quick Fix for Diagnostics', async () => {
    const range = new vscode.Range(0, 0, 10, 0);
    const diagnostic = new vscode.Diagnostic(
      range,
      'Test diagnostic',
      vscode.DiagnosticSeverity.Error
    );
    const context = { diagnostics: [diagnostic] } as vscode.CodeActionContext;
    const actions = await provider.provideCodeActions(
      testDocument,
      range,
      context,
      {} as vscode.CancellationToken
    );

    assert.ok(actions.length > 0, 'Should provide quick fix actions');
    assert.ok(
      actions.some((a) => a.title.includes('Fix')),
      'Should include fix action'
    );
  });

  test('Folding Range Provider - Provide Folding Ranges', async () => {
    const context = {} as vscode.FoldingContext;
    const ranges = await provider.provideFoldingRanges(
      testDocument,
      context,
      {} as vscode.CancellationToken
    );

    assert.ok(ranges.length > 0, 'Should provide folding ranges');

    // Check that ranges are valid
    ranges.forEach((range) => {
      assert.ok(range.start >= 0, 'Range start should be valid');
      assert.ok(range.end >= range.start, 'Range end should be after start');
    });
  });

  test('Selection Range Provider - Provide Selection Ranges', async () => {
    const positions = [new vscode.Position(2, 8)]; // Position at "name"
    const ranges = await provider.provideSelectionRanges(
      testDocument,
      positions,
      {} as vscode.CancellationToken
    );

    assert.ok(ranges.length > 0, 'Should provide selection ranges');
    assert.ok(ranges[0].range, 'Selection range should have range property');
  });

  test('Document Highlight Provider - Highlight Symbols', async () => {
    const position = new vscode.Position(2, 8); // Position at "name"
    const highlights = await provider.provideDocumentHighlights(
      testDocument,
      position,
      {} as vscode.CancellationToken
    );

    assert.ok(highlights.length > 0, 'Should provide document highlights');
    highlights.forEach((highlight) => {
      assert.ok(highlight.range, 'Highlight should have range property');
    });
  });

  test('Document Link Provider - Find Links', async () => {
    const links = await provider.provideDocumentLinks(testDocument, {} as vscode.CancellationToken);

    assert.ok(links.length > 0, 'Should provide document links');

    // Check for file links
    const fileLinks = links.filter((link) => link.target?.scheme === 'file');
    assert.ok(fileLinks.length > 0, 'Should find file links');

    // Check for URL links
    const urlLinks = links.filter(
      (link) => link.target?.scheme === 'http' || link.target?.scheme === 'https'
    );
    assert.ok(urlLinks.length > 0, 'Should find URL links');
  });

  test('Rename Provider - Provide Rename Edits', async () => {
    const position = new vscode.Position(2, 8); // Position at "name"
    const newName = 'newName';
    const edit = await provider.provideRenameEdits(
      testDocument,
      position,
      newName,
      {} as vscode.CancellationToken
    );

    assert.ok(edit, 'Should provide rename edit');
    assert.ok(edit.has(testDocument.uri), 'Edit should include current document');
  });

  test('Format on Type Provider - Auto Formatting', async () => {
    const position = new vscode.Position(5, 10); // Position after a key
    const ch = ':';
    const options = { tabSize: 2, insertSpaces: true } as vscode.FormattingOptions;
    const edits = await provider.provideOnTypeFormattingEdits(
      testDocument,
      position,
      ch,
      options,
      {} as vscode.CancellationToken
    );

    // Should provide formatting edits for colon insertion
    assert.ok(Array.isArray(edits), 'Should return array of edits');
  });

  test('Format on Type Provider - New Line Indentation', async () => {
    const position = new vscode.Position(10, 0); // Position at start of line
    const ch = '\n';
    const options = { tabSize: 2, insertSpaces: true } as vscode.FormattingOptions;
    const edits = await provider.provideOnTypeFormattingEdits(
      testDocument,
      position,
      ch,
      options,
      {} as vscode.CancellationToken
    );

    // Should provide indentation edits for new lines
    assert.ok(Array.isArray(edits), 'Should return array of edits');
  });

  test('Non-Rhema File Handling', async () => {
    // Create a non-Rhema document
    const nonRhemaDocument = await vscode.workspace.openTextDocument({
      content: 'console.log("Hello World");',
      language: 'javascript',
    });

    const position = new vscode.Position(0, 0);

    // Test that providers return empty results for non-Rhema files
    const definition = await provider.provideDefinition(
      nonRhemaDocument,
      position,
      {} as vscode.CancellationToken
    );
    const references = await provider.provideReferences(
      nonRhemaDocument,
      position,
      {} as vscode.ReferenceContext,
      {} as vscode.CancellationToken
    );
    const symbols = await provider.provideDocumentSymbols(
      nonRhemaDocument,
      {} as vscode.CancellationToken
    );
    const actions = await provider.provideCodeActions(
      nonRhemaDocument,
      new vscode.Range(0, 0, 1, 0),
      {} as vscode.CodeActionContext,
      {} as vscode.CancellationToken
    );
    const ranges = await provider.provideFoldingRanges(
      nonRhemaDocument,
      {} as vscode.FoldingContext,
      {} as vscode.CancellationToken
    );
    const highlights = await provider.provideDocumentHighlights(
      nonRhemaDocument,
      position,
      {} as vscode.CancellationToken
    );
    const links = await provider.provideDocumentLinks(
      nonRhemaDocument,
      {} as vscode.CancellationToken
    );
    const rename = await provider.provideRenameEdits(
      nonRhemaDocument,
      position,
      'newName',
      {} as vscode.CancellationToken
    );
    const format = await provider.provideOnTypeFormattingEdits(
      nonRhemaDocument,
      position,
      ':',
      {} as vscode.FormattingOptions,
      {} as vscode.CancellationToken
    );

    assert.strictEqual(definition, undefined, 'Definition should be undefined for non-Rhema files');
    assert.strictEqual(references.length, 0, 'References should be empty for non-Rhema files');
    assert.strictEqual(symbols.length, 0, 'Symbols should be empty for non-Rhema files');
    assert.strictEqual(actions.length, 0, 'Actions should be empty for non-Rhema files');
    assert.strictEqual(ranges.length, 0, 'Folding ranges should be empty for non-Rhema files');
    assert.strictEqual(highlights.length, 0, 'Highlights should be empty for non-Rhema files');
    assert.strictEqual(links.length, 0, 'Links should be empty for non-Rhema files');
    assert.strictEqual(rename, undefined, 'Rename should be undefined for non-Rhema files');
    assert.strictEqual(format.length, 0, 'Format edits should be empty for non-Rhema files');
  });

  test('Error Handling - Invalid YAML', async () => {
    const invalidDocument = await vscode.workspace.openTextDocument({
      content: 'invalid: yaml: content:',
      language: 'yaml',
    });

    const position = new vscode.Position(0, 0);

    // Test that providers handle invalid YAML gracefully
    const symbols = await provider.provideDocumentSymbols(
      invalidDocument,
      {} as vscode.CancellationToken
    );
    const actions = await provider.provideCodeActions(
      invalidDocument,
      new vscode.Range(0, 0, 1, 0),
      {} as vscode.CodeActionContext,
      {} as vscode.CancellationToken
    );

    assert.ok(Array.isArray(symbols), 'Should return empty array for invalid YAML');
    assert.ok(Array.isArray(actions), 'Should return empty array for invalid YAML');
  });

  test('Provider Disposal', async () => {
    await provider.dispose();

    // Verify provider is properly disposed
    // Note: We can't directly test disposal, but we can ensure no errors are thrown
    assert.ok(true, 'Provider should dispose without errors');
  });
});
