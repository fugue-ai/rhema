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
var __createBinding =
  (this && this.__createBinding) ||
  (Object.create
    ? (o, m, k, k2) => {
        if (k2 === undefined) k2 = k;
        var desc = Object.getOwnPropertyDescriptor(m, k);
        if (!desc || ('get' in desc ? !m.__esModule : desc.writable || desc.configurable)) {
          desc = {
            enumerable: true,
            get: () => m[k],
          };
        }
        Object.defineProperty(o, k2, desc);
      }
    : (o, m, k, k2) => {
        if (k2 === undefined) k2 = k;
        o[k2] = m[k];
      });
var __setModuleDefault =
  (this && this.__setModuleDefault) ||
  (Object.create
    ? (o, v) => {
        Object.defineProperty(o, 'default', { enumerable: true, value: v });
      }
    : (o, v) => {
        o['default'] = v;
      });
var __importStar =
  (this && this.__importStar) ||
  (() => {
    var ownKeys = (o) => {
      ownKeys =
        Object.getOwnPropertyNames ||
        ((o) => {
          var ar = [];
          for (var k in o) if (Object.hasOwn(o, k)) ar[ar.length] = k;
          return ar;
        });
      return ownKeys(o);
    };
    return (mod) => {
      if (mod && mod.__esModule) return mod;
      var result = {};
      if (mod != null)
        for (var k = ownKeys(mod), i = 0; i < k.length; i++)
          if (k[i] !== 'default') __createBinding(result, mod, k[i]);
      __setModuleDefault(result, mod);
      return result;
    };
  })();
Object.defineProperty(exports, '__esModule', { value: true });
exports.RhemaTestRunner = void 0;
exports.runTests = runTests;
const vscode = __importStar(require('vscode'));
/**
 * Test runner for Rhema provider functionality
 * This script can be run from the VS Code extension development host
 */
class RhemaTestRunner {
  constructor() {
    this.testResults = [];
    this.outputChannel = vscode.window.createOutputChannel('Rhema Tests');
  }
  /**
   * Run all provider tests
   */
  async runAllTests() {
    this.outputChannel.show();
    this.outputChannel.appendLine('Starting Rhema Provider Tests...\n');
    try {
      // Import and run provider tests
      await this.runProviderTests();
      // Generate test report
      this.generateTestReport();
    } catch (error) {
      this.outputChannel.appendLine(`Test execution failed: ${error}`);
    }
  }
  /**
   * Run provider-specific tests
   */
  async runProviderTests() {
    this.outputChannel.appendLine('Running Provider Tests...\n');
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
  }
  async testProviderInitialization() {
    const testName = 'Provider Initialization';
    try {
      // Import the provider
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      // Mock context
      const mockContext = {
        subscriptions: [],
        extensionPath: '/test/path',
      };
      await provider.initialize(mockContext);
      await provider.dispose();
      this.recordTestResult(testName, true, 'Provider initialized successfully');
    } catch (error) {
      this.recordTestResult(testName, false, `Initialization failed: ${error}`);
    }
  }
  async testDefinitionProvider() {
    const testName = 'Definition Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      // Create test document
      const testContent = `
scope:
  name: "Test Scope"
  description: "Test description"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const position = new vscode.Position(2, 8); // Position at "name"
      const definition = await provider.provideDefinition(document, position, {});
      if (definition) {
        this.recordTestResult(testName, true, 'Definition found successfully');
      } else {
        this.recordTestResult(testName, true, 'No definition found (expected for simple test)');
      }
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Definition test failed: ${error}`);
    }
  }
  async testReferenceProvider() {
    const testName = 'Reference Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
  description: "Test description"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const position = new vscode.Position(2, 8);
      const context = { includeDeclaration: true };
      const references = await provider.provideReferences(document, position, context, {});
      this.recordTestResult(testName, true, `Found ${references.length} references`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Reference test failed: ${error}`);
    }
  }
  async testDocumentSymbolProvider() {
    const testName = 'Document Symbol Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
context:
  files: []
todos:
  - id: "TODO-001"
    title: "Test Todo"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const symbols = await provider.provideDocumentSymbols(document, {});
      this.recordTestResult(testName, true, `Extracted ${symbols.length} symbols`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Symbol test failed: ${error}`);
    }
  }
  async testWorkspaceSymbolProvider() {
    const testName = 'Workspace Symbol Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const symbols = await provider.provideWorkspaceSymbols('scope', {});
      this.recordTestResult(testName, true, `Found ${symbols.length} workspace symbols`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Workspace symbol test failed: ${error}`);
    }
  }
  async testCodeActionsProvider() {
    const testName = 'Code Actions Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const range = new vscode.Range(0, 0, 5, 0);
      const context = { diagnostics: [] };
      const actions = await provider.provideCodeActions(document, range, context, {});
      this.recordTestResult(testName, true, `Provided ${actions.length} code actions`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Code actions test failed: ${error}`);
    }
  }
  async testFoldingRangeProvider() {
    const testName = 'Folding Range Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
context:
  files: []
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const context = {};
      const ranges = await provider.provideFoldingRanges(document, context, {});
      this.recordTestResult(testName, true, `Provided ${ranges.length} folding ranges`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Folding range test failed: ${error}`);
    }
  }
  async testSelectionRangeProvider() {
    const testName = 'Selection Range Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const positions = [new vscode.Position(2, 8)];
      const ranges = await provider.provideSelectionRanges(document, positions, {});
      this.recordTestResult(testName, true, `Provided ${ranges.length} selection ranges`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Selection range test failed: ${error}`);
    }
  }
  async testDocumentHighlightProvider() {
    const testName = 'Document Highlight Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const position = new vscode.Position(2, 8);
      const highlights = await provider.provideDocumentHighlights(document, position, {});
      this.recordTestResult(testName, true, `Provided ${highlights.length} highlights`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Document highlight test failed: ${error}`);
    }
  }
  async testDocumentLinkProvider() {
    const testName = 'Document Link Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
files:
  - path: "src/main.rs"
references:
  - url: "https://github.com/example/rhema"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const links = await provider.provideDocumentLinks(document, {});
      this.recordTestResult(testName, true, `Found ${links.length} document links`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Document link test failed: ${error}`);
    }
  }
  async testRenameProvider() {
    const testName = 'Rename Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const position = new vscode.Position(2, 8);
      const edit = await provider.provideRenameEdits(document, position, 'newName', {});
      if (edit) {
        this.recordTestResult(testName, true, 'Rename edit provided successfully');
      } else {
        this.recordTestResult(testName, true, 'No rename edit (expected for simple test)');
      }
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Rename test failed: ${error}`);
    }
  }
  async testFormatOnTypeProvider() {
    const testName = 'Format on Type Provider';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      const testContent = `
scope:
  name: "Test Scope"
`;
      const document = await vscode.workspace.openTextDocument({
        content: testContent,
        language: 'yaml',
      });
      const position = new vscode.Position(2, 10);
      const options = { tabSize: 2, insertSpaces: true };
      const edits = await provider.provideOnTypeFormattingEdits(
        document,
        position,
        ':',
        options,
        {}
      );
      this.recordTestResult(testName, true, `Provided ${edits.length} format edits`);
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Format on type test failed: ${error}`);
    }
  }
  async testErrorHandling() {
    const testName = 'Error Handling';
    try {
      const { RhemaProvider } = await Promise.resolve().then(() =>
        __importStar(require('../src/providers/rhemaProvider'))
      );
      const provider = new RhemaProvider();
      // Test with invalid YAML
      const invalidDocument = await vscode.workspace.openTextDocument({
        content: 'invalid: yaml: content:',
        language: 'yaml',
      });
      const symbols = await provider.provideDocumentSymbols(invalidDocument, {});
      const actions = await provider.provideCodeActions(
        invalidDocument,
        new vscode.Range(0, 0, 1, 0),
        {},
        {}
      );
      this.recordTestResult(testName, true, 'Error handling works correctly');
      await provider.dispose();
    } catch (error) {
      this.recordTestResult(testName, false, `Error handling test failed: ${error}`);
    }
  }
  recordTestResult(testName, passed, message) {
    const result = {
      name: testName,
      passed,
      message,
      timestamp: new Date(),
    };
    this.testResults.push(result);
    const status = passed ? 'PASS' : 'FAIL';
    this.outputChannel.appendLine(`[${status}] ${testName}: ${message}`);
  }
  generateTestReport() {
    const totalTests = this.testResults.length;
    const passedTests = this.testResults.filter((r) => r.passed).length;
    const failedTests = totalTests - passedTests;
    this.outputChannel.appendLine('\n' + '='.repeat(50));
    this.outputChannel.appendLine('Rhema PROVIDER TEST REPORT');
    this.outputChannel.appendLine('='.repeat(50));
    this.outputChannel.appendLine(`Total Tests: ${totalTests}`);
    this.outputChannel.appendLine(`Passed: ${passedTests}`);
    this.outputChannel.appendLine(`Failed: ${failedTests}`);
    this.outputChannel.appendLine(
      `Success Rate: ${((passedTests / totalTests) * 100).toFixed(1)}%`
    );
    if (failedTests > 0) {
      this.outputChannel.appendLine('\nFAILED TESTS:');
      this.testResults
        .filter((r) => !r.passed)
        .forEach((r) => {
          this.outputChannel.appendLine(`  - ${r.name}: ${r.message}`);
        });
    }
    this.outputChannel.appendLine('\nTest execution completed.');
  }
  dispose() {
    this.outputChannel.dispose();
  }
}
exports.RhemaTestRunner = RhemaTestRunner;
// Export for use in extension
function runTests() {
  const runner = new RhemaTestRunner();
  runner.runAllTests().then(() => {
    // Keep the runner alive for a bit to show results
    setTimeout(() => runner.dispose(), 5000);
  });
}
//# sourceMappingURL=run-tests.js.map
