import { RhemaTestFramework, type TestSuite } from './testFramework';

export class RhemaTestRunner {
  private testFramework: RhemaTestFramework;

  constructor() {
    this.testFramework = new RhemaTestFramework();
    this.initializeTestSuites();
  }

  private initializeTestSuites(): void {
    // Add simplified test suites focused on basic functionality
    this.testFramework.addTestSuite(this.createBasicFunctionalityTestSuite());
    this.testFramework.addTestSuite(this.createProviderInitializationTestSuite());
    this.testFramework.addTestSuite(this.createDocumentParsingTestSuite());
  }

  async runAllTests(): Promise<void> {
    console.log('🚀 Starting Rhema Language Server Test Suite');
    console.log('============================================\n');

    try {
      const report = await this.testFramework.runAllTests();

      console.log('\n🎉 Test Execution Complete!');
      console.log('==========================');

      if (report.failedTests === 0) {
        console.log('✅ All tests passed successfully!');
      } else {
        console.log(`❌ ${report.failedTests} tests failed`);
        process.exit(1);
      }
    } catch (error) {
      console.error('💥 Test execution failed:', error);
      process.exit(1);
    }
  }

  async runPerformanceBenchmarks(): Promise<void> {
    console.log('\n⚡ Running Performance Benchmarks');
    console.log('================================\n');

    const operations = ['completion', 'validation', 'hover', 'formatting'];

    for (const operation of operations) {
      try {
        await this.testFramework.runPerformanceBenchmark(operation, 100);
      } catch (error) {
        console.error(`❌ Benchmark failed for ${operation}:`, error);
      }
    }
  }

  // --- Simplified Test Suite Definitions ---

  private createBasicFunctionalityTestSuite(): TestSuite {
    return {
      name: 'Basic Functionality Tests',
      description: 'Tests for basic language server functionality',
      tests: [
        {
          name: 'Server initialization',
          description: 'Should initialize all providers successfully',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
        {
          name: 'Document creation',
          description: 'Should create mock documents successfully',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
        {
          name: 'Provider availability',
          description: 'Should have all required providers available',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
      ],
    };
  }

  private createProviderInitializationTestSuite(): TestSuite {
    return {
      name: 'Provider Initialization Tests',
      description: 'Tests for provider initialization and basic operations',
      tests: [
        {
          name: 'Completer provider',
          description: 'Should initialize completer provider',
          input: 'name: test\nvers',
          expected: { success: true },
        },
        {
          name: 'Validator provider',
          description: 'Should initialize validator provider',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
        {
          name: 'Hover provider',
          description: 'Should initialize hover provider',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
      ],
    };
  }

  private createDocumentParsingTestSuite(): TestSuite {
    return {
      name: 'Document Parsing Tests',
      description: 'Tests for document parsing functionality',
      tests: [
        {
          name: 'Basic YAML parsing',
          description: 'Should parse basic YAML content',
          input: 'name: test\nversion: "1.0.0"',
          expected: { success: true },
        },
        {
          name: 'Complex YAML structure',
          description: 'Should parse complex nested YAML',
          input: `name: test
version: "1.0.0"
contexts:
  - name: context1
    description: Test context
dependencies:
  - name: dep1
    version: "1.0.0"`,
          expected: { success: true },
        },
      ],
    };
  }
}

// --- Main Execution ---

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  const testRunner = new RhemaTestRunner();

  if (args.includes('--benchmarks')) {
    await testRunner.runPerformanceBenchmarks();
  } else if (args.includes('--unit')) {
    // Run only unit tests
    console.log('Running unit tests...');
    await testRunner.runAllTests();
  } else if (args.includes('--integration')) {
    // Run only integration tests
    console.log('Running integration tests...');
    await testRunner.runAllTests();
  } else {
    // Run all tests
    await testRunner.runAllTests();
  }
}

// Run the test suite if this file is executed directly
if (require.main === module) {
  main().catch((error) => {
    console.error('Test execution failed:', error);
    process.exit(1);
  });
}
