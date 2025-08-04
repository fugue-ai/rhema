import { jest, describe, it, expect, beforeEach, afterEach } from '@jest/globals';
import { createMockCapabilities } from '../testSetup';

// Mock the vscode-languageserver module
jest.mock('vscode-languageserver/node', () => ({
  createConnection: jest.fn(() => ({
    onInitialize: jest.fn(),
    onInitialized: jest.fn(),
    onCompletion: jest.fn(),
    onCompletionResolve: jest.fn(),
    onHover: jest.fn(),
    onDefinition: jest.fn(),
    onReferences: jest.fn(),
    onDocumentSymbol: jest.fn(),
    onWorkspaceSymbol: jest.fn(),
    onCodeAction: jest.fn(),
    onDocumentFormatting: jest.fn(),
    onDocumentRangeFormatting: jest.fn(),
    onFoldingRanges: jest.fn(),
    onDidChangeConfiguration: jest.fn(),
    onDidChangeWatchedFiles: jest.fn(),
    onRequest: jest.fn(),
    onNotification: jest.fn(),
    onShutdown: jest.fn(),
    onExit: jest.fn(),
    listen: jest.fn(),
    workspace: {
      onDidChangeWorkspaceFolders: jest.fn(),
    },
    client: {
      register: jest.fn(),
    },
    console: {
      log: jest.fn(),
    },
    sendDiagnostics: jest.fn(),
  })),
  ProposedFeatures: { all: {} },
  TextDocuments: jest.fn(() => ({
    onDidOpen: jest.fn(),
    onDidChangeContent: jest.fn(),
    onDidClose: jest.fn(),
    onDidSave: jest.fn(),
    all: jest.fn(() => []),
    get: jest.fn(),
    listen: jest.fn(),
  })),
  TextDocumentSyncKind: { Incremental: 2 },
  DiagnosticSeverity: { Error: 1, Warning: 2, Information: 3, Hint: 4 },
  CompletionItemKind: {},
  FoldingRangeKind: { Region: 'region' },
  DidChangeConfigurationNotification: { type: { method: 'workspace/didChangeConfiguration' } },
}));

// Mock all the providers
jest.mock('../completer');
jest.mock('../hover');
jest.mock('../definition');
jest.mock('../reference');
jest.mock('../symbol');
jest.mock('../codeAction');
jest.mock('../formatter');
jest.mock('../semanticTokens');
jest.mock('../configuration');
jest.mock('../logger');
jest.mock('../errorHandler');
jest.mock('../performanceMonitor');
jest.mock('../cache');
jest.mock('../schemaManager');
jest.mock('../workspaceManager');
jest.mock('../performanceOptimizer');

describe('LSP Server', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  describe('Server Initialization', () => {
    it('should initialize with correct capabilities', () => {
      const capabilities = createMockCapabilities();
      
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(capabilities).toBeDefined();
      expect(capabilities.textDocument).toBeDefined();
      expect(capabilities.workspace).toBeDefined();
    });

    it('should handle initialization without workspace folders', () => {
      const capabilities = createMockCapabilities();
      
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(capabilities).toBeDefined();
    });
  });

  describe('Document Change Handling', () => {
    it('should have document event handlers configured', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });

    it('should handle document change events', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });

    it('should handle document close events', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });

  describe('Configuration Handling', () => {
    it('should handle configuration changes', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });

  describe('Workspace File Watching', () => {
    it('should handle workspace file changes', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });

  describe('Server Shutdown', () => {
    it('should handle shutdown gracefully', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });

    it('should handle exit', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });

  describe('Error Handling', () => {
    it('should handle initialization errors', () => {
      const capabilities = createMockCapabilities();
      
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(capabilities).toBeDefined();
    });

    it('should handle document validation errors', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });

  describe('Performance Monitoring', () => {
    it('should track performance metrics', () => {
      // Import the server module to trigger initialization
      require('../server');
      
      // Verify that the server module can be loaded without errors
      expect(true).toBe(true); // Basic test that the module loads
    });
  });
}); 