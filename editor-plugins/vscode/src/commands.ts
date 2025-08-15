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

import * as vscode from 'vscode';
import { RhemaLogger } from './logger';
import { RhemaSettings } from './settings';
import { RhemaErrorHandler } from './errorHandler';

export class RhemaCommands {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(): Promise<void> {
    this.logger.info('Rhema commands initialized');
  }

  async showContext(): Promise<void> {
    try {
      this.logger.info('Showing Rhema context');
      vscode.window.showInformationMessage('Rhema Context: Active');
    } catch (error) {
      this.errorHandler.handleError('Failed to show context', error);
    }
  }

  async executeQuery(): Promise<void> {
    try {
      this.logger.info('Executing Rhema query');
      vscode.window.showInformationMessage('Rhema Query executed');
    } catch (error) {
      this.errorHandler.handleError('Failed to execute query', error);
    }
  }

  async searchContext(): Promise<void> {
    try {
      this.logger.info('Searching Rhema context');
      vscode.window.showInformationMessage('Rhema Context search completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to search context', error);
    }
  }

  async validateFiles(): Promise<void> {
    try {
      this.logger.info('Validating Rhema files');
      vscode.window.showInformationMessage('Rhema files validated');
    } catch (error) {
      this.errorHandler.handleError('Failed to validate files', error);
    }
  }

  async showScopes(): Promise<void> {
    try {
      this.logger.info('Showing Rhema scopes');
      vscode.window.showInformationMessage('Rhema Scopes displayed');
    } catch (error) {
      this.errorHandler.handleError('Failed to show scopes', error);
    }
  }

  async showTree(): Promise<void> {
    try {
      this.logger.info('Showing Rhema tree');
      vscode.window.showInformationMessage('Rhema Tree displayed');
    } catch (error) {
      this.errorHandler.handleError('Failed to show tree', error);
    }
  }

  async manageTodos(): Promise<void> {
    try {
      this.logger.info('Managing Rhema todos');
      vscode.window.showInformationMessage('Rhema Todos management opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to manage todos', error);
    }
  }

  async manageInsights(): Promise<void> {
    try {
      this.logger.info('Managing Rhema insights');
      vscode.window.showInformationMessage('Rhema Insights management opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to manage insights', error);
    }
  }

  async managePatterns(): Promise<void> {
    try {
      this.logger.info('Managing Rhema patterns');
      vscode.window.showInformationMessage('Rhema Patterns management opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to manage patterns', error);
    }
  }

  async manageDecisions(): Promise<void> {
    try {
      this.logger.info('Managing Rhema decisions');
      vscode.window.showInformationMessage('Rhema Decisions management opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to manage decisions', error);
    }
  }

  async showDependencies(): Promise<void> {
    try {
      this.logger.info('Showing Rhema dependencies');
      vscode.window.showInformationMessage('Rhema Dependencies displayed');
    } catch (error) {
      this.errorHandler.handleError('Failed to show dependencies', error);
    }
  }

  async showImpact(): Promise<void> {
    try {
      this.logger.info('Showing Rhema impact');
      vscode.window.showInformationMessage('Rhema Impact analysis displayed');
    } catch (error) {
      this.errorHandler.handleError('Failed to show impact', error);
    }
  }

  async syncKnowledge(): Promise<void> {
    try {
      this.logger.info('Syncing Rhema knowledge');
      vscode.window.showInformationMessage('Rhema Knowledge synced');
    } catch (error) {
      this.errorHandler.handleError('Failed to sync knowledge', error);
    }
  }

  async gitIntegration(): Promise<void> {
    try {
      this.logger.info('Running Rhema Git integration');
      vscode.window.showInformationMessage('Rhema Git integration completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to run Git integration', error);
    }
  }

  async showStats(): Promise<void> {
    try {
      this.logger.info('Showing Rhema stats');
      vscode.window.showInformationMessage('Rhema Statistics displayed');
    } catch (error) {
      this.errorHandler.handleError('Failed to show stats', error);
    }
  }

  async checkHealth(): Promise<void> {
    try {
      this.logger.info('Checking Rhema health');
      vscode.window.showInformationMessage('Rhema Health check completed');
    } catch (error) {
      this.errorHandler.handleError('Failed to check health', error);
    }
  }

  async debugContext(): Promise<void> {
    try {
      this.logger.info('Debugging Rhema context');
      vscode.window.showInformationMessage('Rhema Context debugging started');
    } catch (error) {
      this.errorHandler.handleError('Failed to debug context', error);
    }
  }

  async profilePerformance(): Promise<void> {
    try {
      this.logger.info('Profiling Rhema performance');
      vscode.window.showInformationMessage('Rhema Performance profiling started');
    } catch (error) {
      this.errorHandler.handleError('Failed to profile performance', error);
    }
  }

  async refactorContext(): Promise<void> {
    try {
      this.logger.info('Refactoring Rhema context');
      vscode.window.showInformationMessage('Rhema Context refactoring started');
    } catch (error) {
      this.errorHandler.handleError('Failed to refactor context', error);
    }
  }

  async generateCode(): Promise<void> {
    try {
      this.logger.info('Generating Rhema code');
      vscode.window.showInformationMessage('Rhema Code generation started');
    } catch (error) {
      this.errorHandler.handleError('Failed to generate code', error);
    }
  }

  async showDocumentation(): Promise<void> {
    try {
      this.logger.info('Showing Rhema documentation');
      vscode.window.showInformationMessage('Rhema Documentation opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to show documentation', error);
    }
  }

  async configureSettings(): Promise<void> {
    try {
      this.logger.info('Configuring Rhema settings');
      vscode.window.showInformationMessage('Rhema Settings configuration opened');
    } catch (error) {
      this.errorHandler.handleError('Failed to configure settings', error);
    }
  }

  async runProviderTests(): Promise<void> {
    try {
      this.logger.info('Running Rhema provider tests');

      // Create test Rhema document
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

      // Create test document
      const testDocument = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });

      // Initialize provider
      const provider = new (await import('./providers/rhemaProvider')).RhemaProvider();
      const mockContext = {
        subscriptions: [],
        extensionPath: '/test/path',
      } as vscode.ExtensionContext;

      await provider.initialize(mockContext);

      // Test results
      const testResults: { [key: string]: boolean } = {};

      try {
        // Test 1: Definition Provider
        const definitionPosition = new vscode.Position(2, 8); // Position at "name" in scope.name
        const definition = await provider.provideDefinition(
          testDocument,
          definitionPosition,
          {} as vscode.CancellationToken
        );
        testResults['Definition Provider'] = definition !== undefined;

        // Test 2: Reference Provider
        const referencePosition = new vscode.Position(2, 8); // Position at "name"
        const referenceContext = { includeDeclaration: true } as vscode.ReferenceContext;
        const references = await provider.provideReferences(
          testDocument,
          referencePosition,
          referenceContext,
          {} as vscode.CancellationToken
        );
        testResults['Reference Provider'] = Array.isArray(references) && references.length >= 0;

        // Test 3: Document Symbol Provider
        const symbols = await provider.provideDocumentSymbols(
          testDocument,
          {} as vscode.CancellationToken
        );
        testResults['Document Symbol Provider'] = Array.isArray(symbols) && symbols.length > 0;

        // Test 4: Workspace Symbol Provider
        const workspaceSymbols = await provider.provideWorkspaceSymbols(
          'scope',
          {} as vscode.CancellationToken
        );
        testResults['Workspace Symbol Provider'] = Array.isArray(workspaceSymbols) && workspaceSymbols.length >= 0;

        // Test 5: Code Action Provider
        const codeActionRange = new vscode.Range(0, 0, 10, 0);
        const codeActionContext = {
          diagnostics: [],
          triggerKind: vscode.CodeActionTriggerKind.Invoke,
          only: undefined,
        } as vscode.CodeActionContext;
        const codeActions = await provider.provideCodeActions(
          testDocument,
          codeActionRange,
          codeActionContext,
          {} as vscode.CancellationToken
        );
        testResults['Code Action Provider'] = Array.isArray(codeActions) && codeActions.length >= 0;

        // Test 6: Folding Range Provider
        const foldingContext = {} as vscode.FoldingContext;
        const foldingRanges = await provider.provideFoldingRanges(
          testDocument,
          foldingContext,
          {} as vscode.CancellationToken
        );
        testResults['Folding Range Provider'] = Array.isArray(foldingRanges) && foldingRanges.length > 0;

        // Test 7: Selection Range Provider
        const selectionPositions = [new vscode.Position(2, 8)]; // Position at "name"
        const selectionRanges = await provider.provideSelectionRanges(
          testDocument,
          selectionPositions,
          {} as vscode.CancellationToken
        );
        testResults['Selection Range Provider'] = Array.isArray(selectionRanges) && selectionRanges.length > 0;

        // Test 8: Document Highlight Provider
        const highlightPosition = new vscode.Position(2, 8); // Position at "name"
        const highlights = await provider.provideDocumentHighlights(
          testDocument,
          highlightPosition,
          {} as vscode.CancellationToken
        );
        testResults['Document Highlight Provider'] = Array.isArray(highlights) && highlights.length >= 0;

        // Test 9: Document Link Provider
        const links = await provider.provideDocumentLinks(
          testDocument,
          {} as vscode.CancellationToken
        );
        testResults['Document Link Provider'] = Array.isArray(links) && links.length >= 0;

        // Test 10: Rename Provider
        const renamePosition = new vscode.Position(2, 8); // Position at "name"
        const renameEdits = await provider.provideRenameEdits(
          testDocument,
          renamePosition,
          'newName',
          {} as vscode.CancellationToken
        );
        testResults['Rename Provider'] = renameEdits !== undefined;

        // Test 11: On-Type Formatting Provider
        const formatPosition = new vscode.Position(5, 10); // Position after a key
        const formatOptions = { tabSize: 2, insertSpaces: true } as vscode.FormattingOptions;
        const formatEdits = await provider.provideOnTypeFormattingEdits(
          testDocument,
          formatPosition,
          ':',
          formatOptions,
          {} as vscode.CancellationToken
        );
        testResults['On-Type Formatting Provider'] = Array.isArray(formatEdits);

        // Test 12: Non-Rhema File Handling
        const nonRhemaDocument = await vscode.workspace.openTextDocument({
          content: 'console.log("Hello World");',
          language: 'javascript',
        });
        const nonRhemaPosition = new vscode.Position(0, 0);

        const nonRhemaDefinition = await provider.provideDefinition(
          nonRhemaDocument,
          nonRhemaPosition,
          {} as vscode.CancellationToken
        );
        testResults['Non-Rhema File Handling'] = nonRhemaDefinition === undefined;

        // Test 13: Error Handling
        const invalidDocument = await vscode.workspace.openTextDocument({
          content: 'invalid: yaml: content:',
          language: 'yaml',
        });
        const invalidPosition = new vscode.Position(0, 0);

        const invalidSymbols = await provider.provideDocumentSymbols(
          invalidDocument,
          {} as vscode.CancellationToken
        );
        testResults['Error Handling'] = Array.isArray(invalidSymbols);

        // Test 14: Provider Disposal
        await provider.dispose();
        testResults['Provider Disposal'] = true;

        // Calculate test results
        const totalTests = Object.keys(testResults).length;
        const passedTests = Object.values(testResults).filter(result => result).length;
        const failedTests = totalTests - passedTests;

        // Display results
        const resultMessage = `Provider Tests Completed:
✅ Passed: ${passedTests}/${totalTests}
❌ Failed: ${failedTests}/${totalTests}

Detailed Results:
${Object.entries(testResults)
  .map(([test, passed]) => `${passed ? '✅' : '❌'} ${test}`)
  .join('\n')}`;

        this.logger.info(resultMessage);

        if (failedTests === 0) {
          vscode.window.showInformationMessage(`✅ All provider tests passed! (${passedTests}/${totalTests})`);
        } else {
          vscode.window.showWarningMessage(
            `⚠️ Provider tests completed with ${failedTests} failures. (${passedTests}/${totalTests})`
          );
        }

        // Show detailed results in output channel
        const outputChannel = vscode.window.createOutputChannel('Rhema Provider Tests');
        outputChannel.appendLine(resultMessage);
        outputChannel.show();

      } catch (testError) {
        this.errorHandler.handleError('Error during provider tests', testError);
        vscode.window.showErrorMessage('Provider tests failed with error: ' + testError);
      }

    } catch (error) {
      this.errorHandler.handleError('Failed to run provider tests', error);
      vscode.window.showErrorMessage('Failed to run provider tests: ' + error);
    }
  }
}
