#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Mock Rhema CLI for testing VS Code extension
class MockRhemaCLI {
  constructor() {
    this.version = '0.1.0-mock';
    this.mockData = {
      scopes: [
        {
          name: 'test-scope',
          description: 'Test scope for development',
          version: '1.0.0',
          author: 'Developer',
        },
      ],
      todos: [
        {
          id: 'todo-1',
          title: 'Implement feature X',
          description: 'Add new functionality to the system',
          priority: 'high',
          status: 'in-progress',
        },
      ],
      insights: [
        {
          id: 'insight-1',
          title: 'Performance bottleneck found',
          description: 'The system shows slow response times',
          type: 'performance',
          confidence: 0.85,
        },
      ],
      patterns: [
        {
          id: 'pattern-1',
          name: 'error-handling',
          description: 'Standard error handling pattern',
          type: 'code-pattern',
        },
      ],
      decisions: [
        {
          id: 'decision-1',
          title: 'Use Rust for backend',
          description: 'Decision to use Rust for implementation',
          type: 'architecture',
          status: 'approved',
        },
      ],
    };
  }

  async validate(args) {
    console.log('Mock: Validating Rhema files...');

    // Simulate validation process
    const issues = [];

    // Check for required files
    const requiredFiles = ['rhema.yml', 'scope.yml'];
    for (const file of requiredFiles) {
      if (!fs.existsSync(file)) {
        issues.push({
          file: file,
          line: 1,
          column: 1,
          severity: 'warning',
          message: `File ${file} not found`,
        });
      }
    }

    // Simulate YAML validation
    try {
      const yaml = require('yaml');
      const files = args.length > 0 ? args : ['rhema.yml'];

      for (const file of files) {
        if (fs.existsSync(file)) {
          const content = fs.readFileSync(file, 'utf8');
          yaml.parse(content); // This will throw if invalid YAML
        }
      }
    } catch (error) {
      issues.push({
        file: 'rhema.yml',
        line: 1,
        column: 1,
        severity: 'error',
        message: `YAML parsing error: ${error.message}`,
      });
    }

    return { success: issues.length === 0, issues: issues };
  }

  async todos(args) {
    console.log('Mock: Checking todos...');

    const command = args[0];
    switch (command) {
      case '--check-incomplete': {
        const incompleteTodos = this.mockData.todos.filter((todo) => todo.status !== 'completed');
        if (incompleteTodos.length > 0) {
          console.log(`Found ${incompleteTodos.length} incomplete todos`);
          return { success: false, todos: incompleteTodos };
        }
        return { success: true, todos: [] };
      }

      case '--list':
        return { success: true, todos: this.mockData.todos };

      default:
        return { success: true, todos: this.mockData.todos };
    }
  }

  async decisions(args) {
    console.log('Mock: Checking decisions...');

    const command = args[0];
    switch (command) {
      case '--check-unresolved': {
        const unresolvedDecisions = this.mockData.decisions.filter(
          (decision) => decision.status !== 'approved'
        );
        if (unresolvedDecisions.length > 0) {
          console.log(`Found ${unresolvedDecisions.length} unresolved decisions`);
          return { success: false, decisions: unresolvedDecisions };
        }
        return { success: true, decisions: [] };
      }

      case '--list':
        return { success: true, decisions: this.mockData.decisions };

      default:
        return { success: true, decisions: this.mockData.decisions };
    }
  }

  async context(args) {
    console.log('Mock: Analyzing context...');

    const context = {
      scopes: this.mockData.scopes,
      todos: this.mockData.todos,
      insights: this.mockData.insights,
      patterns: this.mockData.patterns,
      decisions: this.mockData.decisions,
      files: ['src/main.rs', 'docs/README.md', 'Cargo.toml'],
      relationships: [
        { from: 'todo-1', to: 'decision-1', type: 'implements' },
        { from: 'insight-1', to: 'pattern-1', type: 'suggests' },
      ],
    };

    return { success: true, context: context };
  }

  async scopes(args) {
    console.log('Mock: Listing scopes...');
    return { success: true, scopes: this.mockData.scopes };
  }

  async insights(args) {
    console.log('Mock: Listing insights...');
    return { success: true, insights: this.mockData.insights };
  }

  async patterns(args) {
    console.log('Mock: Listing patterns...');
    return { success: true, patterns: this.mockData.patterns };
  }

  async search(args) {
    console.log('Mock: Searching...');
    const query = args[0] || '';
    const results = [];

    // Search in todos
    const todoMatches = this.mockData.todos.filter(
      (todo) =>
        todo.title.toLowerCase().includes(query.toLowerCase()) ||
        todo.description.toLowerCase().includes(query.toLowerCase())
    );
    results.push(...todoMatches.map((todo) => ({ type: 'todo', item: todo })));

    // Search in insights
    const insightMatches = this.mockData.insights.filter(
      (insight) =>
        insight.title.toLowerCase().includes(query.toLowerCase()) ||
        insight.description.toLowerCase().includes(query.toLowerCase())
    );
    results.push(...insightMatches.map((insight) => ({ type: 'insight', item: insight })));

    return { success: true, results: results };
  }

  async query(args) {
    console.log('Mock: Executing query...');
    const query = args[0] || '';

    // Simulate different query types
    if (query.includes('scope')) {
      return { success: true, data: this.mockData.scopes };
    } else if (query.includes('todo')) {
      return { success: true, data: this.mockData.todos };
    } else if (query.includes('insight')) {
      return { success: true, data: this.mockData.insights };
    } else {
      return { success: true, data: this.mockData };
    }
  }

  async health() {
    console.log('Mock: Checking health...');
    return {
      success: true,
      status: 'healthy',
      version: this.version,
      features: {
        validation: 'available',
        todos: 'available',
        decisions: 'available',
        context: 'available',
        search: 'available',
        query: 'available',
      },
    };
  }

  async stats() {
    console.log('Mock: Generating statistics...');
    return {
      success: true,
      stats: {
        scopes: this.mockData.scopes.length,
        todos: this.mockData.todos.length,
        insights: this.mockData.insights.length,
        patterns: this.mockData.patterns.length,
        decisions: this.mockData.decisions.length,
        files: 3,
        relationships: 2,
      },
    };
  }
}

// Main CLI execution
const cli = new MockRhemaCLI();
const command = process.argv[2];
const args = process.argv.slice(3);

// Add some delay to simulate real CLI behavior
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function main() {
  try {
    switch (command) {
      case '--version':
        console.log(cli.version);
        break;

      case 'validate': {
        const validationResult = await cli.validate(args);
        console.log(JSON.stringify(validationResult, null, 2));
        process.exit(validationResult.success ? 0 : 1);
        break;
      }

      case 'todos': {
        const todosResult = await cli.todos(args);
        console.log(JSON.stringify(todosResult, null, 2));
        process.exit(todosResult.success ? 0 : 1);
        break;
      }

      case 'decisions': {
        const decisionsResult = await cli.decisions(args);
        console.log(JSON.stringify(decisionsResult, null, 2));
        process.exit(decisionsResult.success ? 0 : 1);
        break;
      }

      case 'context': {
        const contextResult = await cli.context(args);
        console.log(JSON.stringify(contextResult, null, 2));
        break;
      }

      case 'scopes': {
        const scopesResult = await cli.scopes(args);
        console.log(JSON.stringify(scopesResult, null, 2));
        break;
      }

      case 'insights': {
        const insightsResult = await cli.insights(args);
        console.log(JSON.stringify(insightsResult, null, 2));
        break;
      }

      case 'patterns': {
        const patternsResult = await cli.patterns(args);
        console.log(JSON.stringify(patternsResult, null, 2));
        break;
      }

      case 'search': {
        const searchResult = await cli.search(args);
        console.log(JSON.stringify(searchResult, null, 2));
        break;
      }

      case 'query': {
        const queryResult = await cli.query(args);
        console.log(JSON.stringify(queryResult, null, 2));
        break;
      }

      case 'health': {
        const healthResult = await cli.health();
        console.log(JSON.stringify(healthResult, null, 2));
        break;
      }

      case 'stats': {
        const statsResult = await cli.stats();
        console.log(JSON.stringify(statsResult, null, 2));
        break;
      }

      default:
        console.log('Mock Rhema CLI - Available commands:');
        console.log('  --version                    Show version');
        console.log('  validate [files...]         Validate Rhema files');
        console.log('  todos [--check-incomplete]  Manage todos');
        console.log('  decisions [--check-unresolved] Manage decisions');
        console.log('  context                     Analyze context');
        console.log('  scopes                      List scopes');
        console.log('  insights                    List insights');
        console.log('  patterns                    List patterns');
        console.log('  search <query>              Search content');
        console.log('  query <query>               Execute query');
        console.log('  health                      Check health');
        console.log('  stats                       Show statistics');
        break;
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

main();
