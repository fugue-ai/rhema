/**
 * Test runner for Rhema provider functionality
 * This script can be run from the VS Code extension development host
 */
export declare class RhemaTestRunner {
  private outputChannel;
  private testResults;
  constructor();
  /**
   * Run all provider tests
   */
  runAllTests(): Promise<void>;
  /**
   * Run provider-specific tests
   */
  private runProviderTests;
  private testProviderInitialization;
  private testDefinitionProvider;
  private testReferenceProvider;
  private testDocumentSymbolProvider;
  private testWorkspaceSymbolProvider;
  private testCodeActionsProvider;
  private testFoldingRangeProvider;
  private testSelectionRangeProvider;
  private testDocumentHighlightProvider;
  private testDocumentLinkProvider;
  private testRenameProvider;
  private testFormatOnTypeProvider;
  private testErrorHandling;
  private recordTestResult;
  private generateTestReport;
  dispose(): void;
}
export declare function runTests(): void;
//# sourceMappingURL=run-tests.d.ts.map
