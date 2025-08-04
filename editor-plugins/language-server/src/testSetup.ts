/// <reference types="jest" />

import { jest } from '@jest/globals';
import { TextDocument } from 'vscode-languageserver-textdocument';
import { 
  CompletionItemKind, 
  DiagnosticSeverity, 
  SymbolKind,
  Range,
  Position,
  MarkupContent,
  MarkupKind
} from 'vscode-languageserver';

// Mock console methods to reduce noise in tests
const originalConsoleLog = console.log;
const originalConsoleError = console.error;
const originalConsoleWarn = console.warn;

beforeAll(() => {
  // Suppress console output during tests unless explicitly needed
  if (process.env.NODE_ENV === 'test') {
    console.log = jest.fn();
    console.error = jest.fn();
    console.warn = jest.fn();
  }
});

afterAll(() => {
  // Restore console methods
  console.log = originalConsoleLog;
  console.error = originalConsoleError;
  console.warn = originalConsoleWarn;
});

// Global test utilities
export const createTestDocument = (content: string, uri: string = 'test.yml'): TextDocument => {
  return TextDocument.create(uri, 'yaml', 1, content);
};

export const createMockCapabilities = () => ({
  textDocument: {
    completion: {},
    hover: {},
    definition: {},
    references: {},
    documentSymbol: {},
    codeAction: {
      codeActionLiteralSupport: {
        codeActionKind: {
          valueSet: [
            'quickfix',
            'refactor',
            'refactor.extract',
            'refactor.inline',
            'refactor.rewrite',
            'source',
            'source.organizeImports',
          ],
        },
      },
    },
    formatting: {},
    rangeFormatting: {},
    semanticTokens: {},
    inlayHint: {},
    callHierarchy: {},
    typeHierarchy: {},
    documentLink: {},
    colorProvider: {},
    codeLens: {},
    documentHighlight: {},
  },
  workspace: {
    configuration: true,
    workspaceFolders: true,
    symbol: {},
  },
});

export const createMockWorkspaceFolders = () => [
  {
    uri: 'file:///test-workspace',
    name: 'Test Workspace',
  },
];

// Enhanced mock utilities for LSP types
export const createMockHover = (content: string): any => ({
  contents: {
    kind: MarkupKind.Markdown,
    value: content
  }
});

export const createMockCompletionItem = (label: string, kind: CompletionItemKind = CompletionItemKind.Text): any => ({
  label,
  kind,
  insertText: label,
  detail: `Detail for ${label}`,
  documentation: {
    kind: MarkupKind.Markdown,
    value: `Documentation for ${label}`
  }
});

export const createMockDiagnostic = (message: string, range?: Range): any => ({
  range: range || Range.create(0, 0, 0, 10),
  message,
  severity: DiagnosticSeverity.Error,
  source: 'rhema'
});

export const createMockSymbol = (name: string, kind: SymbolKind = SymbolKind.File): any => ({
  name,
  kind,
  location: {
    uri: 'file:///test-workspace/test.yml',
    range: Range.create(0, 0, 0, 10)
  }
});

export const createMockRhemaDocument = (type: string, content: any = {}): any => ({
  type,
  content,
  metadata: {
    uri: 'file:///test-workspace/test.yml',
    version: 1,
    lastModified: new Date().toISOString()
  }
});

// Test data fixtures
export const testDocuments = {
  validScope: `name: test-scope
version: "1.0.0"
description: Test scope document
contexts:
  - name: development
    description: Development context
dependencies:
  - name: rhema-core
    version: "1.0.0"`,

  invalidScope: `version: "1.0.0"
description: Invalid scope document`,

  complexDocument: `name: complex-project
version: "2.1.0"
description: A complex project with multiple contexts
contexts:
  - name: frontend
    description: Frontend development context
    patterns:
      - name: react-pattern
        description: React component pattern
  - name: backend
    description: Backend development context
    patterns:
      - name: api-pattern
        description: REST API pattern
dependencies:
  - name: react
    version: "18.0.0"
  - name: express
    version: "4.18.0"
conventions:
  - name: naming
    description: Naming conventions
    rules:
      - Use kebab-case for file names
      - Use PascalCase for component names`,

  knowledgeDocument: `contexts:
  - name: development
    description: Development knowledge context
    patterns: []
patterns:
  - name: component-pattern
    description: Reusable component pattern
    examples:
      - name: button-component
        description: Button component example
conventions:
  - name: code-style
    description: Code style conventions
    rules:
      - Use consistent indentation
      - Follow ESLint rules`,

  todosDocument: `tasks:
  - title: Implement feature X
    description: Add new feature to the system
    status: pending
    priority: high
    assignee: developer
    dueDate: "2024-01-15"
  - title: Fix bug Y
    description: Resolve critical bug in module Z
    status: in-progress
    priority: critical
    assignee: tester
    dueDate: "2024-01-10"
metadata:
  created: "2024-01-01"
  author: "Test User"`,
};

// Performance test utilities
export const measurePerformance = async (
  operation: () => Promise<any>,
  iterations: number = 100
) => {
  const times: number[] = [];

  for (let i = 0; i < iterations; i++) {
    const startTime = performance.now();
    await operation();
    const endTime = performance.now();
    times.push(endTime - startTime);
  }

  const averageTime = times.reduce((sum, time) => sum + time, 0) / times.length;
  const minTime = Math.min(...times);
  const maxTime = Math.max(...times);

  return {
    averageTime,
    minTime,
    maxTime,
    times,
  };
};

// Mock utilities
export const createMockConnection = () => ({
  console: {
    log: jest.fn(),
    error: jest.fn(),
    warn: jest.fn(),
    info: jest.fn(),
  },
  window: {
    showMessage: jest.fn(),
    showErrorMessage: jest.fn(),
    showWarningMessage: jest.fn(),
    showInformationMessage: jest.fn(),
  },
  workspace: {
    applyEdit: jest.fn(),
    getConfiguration: jest.fn(),
    onDidChangeWorkspaceFolders: jest.fn(),
    onDidChangeConfiguration: jest.fn(),
  },
  onRequest: jest.fn(),
  onNotification: jest.fn(),
  listen: jest.fn(),
});

// Test environment configuration
export const testConfig = {
  timeout: 10000,
  retries: 3,
  parallel: false,
  verbose: true,
};
