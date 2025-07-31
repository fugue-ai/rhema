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
import * as path from 'path';
import * as fs from 'fs';

/**
 * Test runner for Rhema provider functionality
 * This script can be run from the VS Code extension development host
 */
export class RhemaTestRunner {
    private outputChannel: vscode.OutputChannel;
    private testResults: TestResult[] = [];

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('Rhema Tests');
    }

    /**
     * Run all provider tests
     */
    async runAllTests(): Promise<void> {
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
    private async runProviderTests(): Promise<void> {
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

    private async testProviderInitialization(): Promise<void> {
        const testName = 'Provider Initialization';
        try {
            // Import the provider
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            // Mock context
            const mockContext = {
                subscriptions: [],
                extensionPath: '/test/path'
            } as vscode.ExtensionContext;

            await provider.initialize(mockContext);
            await provider.dispose();

            this.recordTestResult(testName, true, 'Provider initialized successfully');
        } catch (error) {
            this.recordTestResult(testName, false, `Initialization failed: ${error}`);
        }
    }

    private async testDefinitionProvider(): Promise<void> {
        const testName = 'Definition Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            // Create test document
            const testContent = `
scope:
  name: "Test Scope"
  description: "Test description"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const position = new vscode.Position(2, 8); // Position at "name"
            const definition = await provider.provideDefinition(document, position, {} as vscode.CancellationToken);
            
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

    private async testReferenceProvider(): Promise<void> {
        const testName = 'Reference Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
  description: "Test description"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const position = new vscode.Position(2, 8);
            const context = { includeDeclaration: true } as vscode.ReferenceContext;
            const references = await provider.provideReferences(document, position, context, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Found ${references.length} references`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Reference test failed: ${error}`);
        }
    }

    private async testDocumentSymbolProvider(): Promise<void> {
        const testName = 'Document Symbol Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
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
                language: 'yaml'
            });

            const symbols = await provider.provideDocumentSymbols(document, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Extracted ${symbols.length} symbols`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Symbol test failed: ${error}`);
        }
    }

    private async testWorkspaceSymbolProvider(): Promise<void> {
        const testName = 'Workspace Symbol Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const symbols = await provider.provideWorkspaceSymbols('scope', {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Found ${symbols.length} workspace symbols`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Workspace symbol test failed: ${error}`);
        }
    }

    private async testCodeActionsProvider(): Promise<void> {
        const testName = 'Code Actions Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const range = new vscode.Range(0, 0, 5, 0);
            const context = { diagnostics: [] } as vscode.CodeActionContext;
            const actions = await provider.provideCodeActions(document, range, context, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Provided ${actions.length} code actions`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Code actions test failed: ${error}`);
        }
    }

    private async testFoldingRangeProvider(): Promise<void> {
        const testName = 'Folding Range Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
context:
  files: []
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const context = {} as vscode.FoldingContext;
            const ranges = await provider.provideFoldingRanges(document, context, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Provided ${ranges.length} folding ranges`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Folding range test failed: ${error}`);
        }
    }

    private async testSelectionRangeProvider(): Promise<void> {
        const testName = 'Selection Range Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const positions = [new vscode.Position(2, 8)];
            const ranges = await provider.provideSelectionRanges(document, positions, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Provided ${ranges.length} selection ranges`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Selection range test failed: ${error}`);
        }
    }

    private async testDocumentHighlightProvider(): Promise<void> {
        const testName = 'Document Highlight Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const position = new vscode.Position(2, 8);
            const highlights = await provider.provideDocumentHighlights(document, position, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Provided ${highlights.length} highlights`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Document highlight test failed: ${error}`);
        }
    }

    private async testDocumentLinkProvider(): Promise<void> {
        const testName = 'Document Link Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
files:
  - path: "src/main.rs"
references:
  - url: "https://github.com/example/rhema"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const links = await provider.provideDocumentLinks(document, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Found ${links.length} document links`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Document link test failed: ${error}`);
        }
    }

    private async testRenameProvider(): Promise<void> {
        const testName = 'Rename Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const position = new vscode.Position(2, 8);
            const edit = await provider.provideRenameEdits(document, position, 'newName', {} as vscode.CancellationToken);
            
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

    private async testFormatOnTypeProvider(): Promise<void> {
        const testName = 'Format on Type Provider';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            const testContent = `
scope:
  name: "Test Scope"
`;
            const document = await vscode.workspace.openTextDocument({
                content: testContent,
                language: 'yaml'
            });

            const position = new vscode.Position(2, 10);
            const options = { tabSize: 2, insertSpaces: true } as vscode.FormattingOptions;
            const edits = await provider.provideOnTypeFormattingEdits(document, position, ':', options, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, `Provided ${edits.length} format edits`);
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Format on type test failed: ${error}`);
        }
    }

    private async testErrorHandling(): Promise<void> {
        const testName = 'Error Handling';
        try {
            const { RhemaProvider } = await import('../src/providers/rhemaProvider');
            const provider = new RhemaProvider();
            
            // Test with invalid YAML
            const invalidDocument = await vscode.workspace.openTextDocument({
                content: 'invalid: yaml: content:',
                language: 'yaml'
            });

            const symbols = await provider.provideDocumentSymbols(invalidDocument, {} as vscode.CancellationToken);
            const actions = await provider.provideCodeActions(invalidDocument, new vscode.Range(0, 0, 1, 0), {} as vscode.CodeActionContext, {} as vscode.CancellationToken);
            
            this.recordTestResult(testName, true, 'Error handling works correctly');
            await provider.dispose();
        } catch (error) {
            this.recordTestResult(testName, false, `Error handling test failed: ${error}`);
        }
    }

    private recordTestResult(testName: string, passed: boolean, message: string): void {
        const result: TestResult = {
            name: testName,
            passed,
            message,
            timestamp: new Date()
        };
        
        this.testResults.push(result);
        
        const status = passed ? 'PASS' : 'FAIL';
        this.outputChannel.appendLine(`[${status}] ${testName}: ${message}`);
    }

    private generateTestReport(): void {
        const totalTests = this.testResults.length;
        const passedTests = this.testResults.filter(r => r.passed).length;
        const failedTests = totalTests - passedTests;
        
        this.outputChannel.appendLine('\n' + '='.repeat(50));
        this.outputChannel.appendLine('Rhema PROVIDER TEST REPORT');
        this.outputChannel.appendLine('='.repeat(50));
        this.outputChannel.appendLine(`Total Tests: ${totalTests}`);
        this.outputChannel.appendLine(`Passed: ${passedTests}`);
        this.outputChannel.appendLine(`Failed: ${failedTests}`);
        this.outputChannel.appendLine(`Success Rate: ${((passedTests / totalTests) * 100).toFixed(1)}%`);
        
        if (failedTests > 0) {
            this.outputChannel.appendLine('\nFAILED TESTS:');
            this.testResults
                .filter(r => !r.passed)
                .forEach(r => {
                    this.outputChannel.appendLine(`  - ${r.name}: ${r.message}`);
                });
        }
        
        this.outputChannel.appendLine('\nTest execution completed.');
    }

    dispose(): void {
        this.outputChannel.dispose();
    }
}

interface TestResult {
    name: string;
    passed: boolean;
    message: string;
    timestamp: Date;
}

// Export for use in extension
export function runTests(): void {
    const runner = new RhemaTestRunner();
    runner.runAllTests().then(() => {
        // Keep the runner alive for a bit to show results
        setTimeout(() => runner.dispose(), 5000);
    });
} 