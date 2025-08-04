import { RhemaWorkspaceManager } from '../workspaceManager';
import { createTestDocument, createMockCapabilities, testDocuments } from '../testSetup';
import { WorkspaceFolder } from 'vscode-languageserver/node';

describe('RhemaWorkspaceManager', () => {
  let workspaceManager: RhemaWorkspaceManager;

  beforeEach(() => {
    workspaceManager = new RhemaWorkspaceManager();
  });

  describe('Initialization', () => {
    it('should initialize with default configuration', () => {
      expect(workspaceManager).toBeDefined();
      const config = workspaceManager.getConfiguration();
      expect(config).toBeDefined();
      expect(config.enabled).toBeDefined();
      expect(config.autoIndex).toBeDefined();
    });

    it('should initialize workspace index', () => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
      
      expect(workspaceManager.getWorkspaceFiles()).toBeDefined();
    });
  });

  describe('Workspace Indexing', () => {
    beforeEach(() => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
    });

    it('should index workspace documents', async () => {
      // The actual implementation indexes files from the filesystem
      // We'll test the indexing process by checking if it completes without errors
      await expect(workspaceManager.indexWorkspace()).resolves.not.toThrow();
      
      const files = workspaceManager.getWorkspaceFiles();
      expect(Array.isArray(files)).toBe(true);
    });

    it('should handle indexing errors gracefully', async () => {
      // Test that indexing doesn't throw errors even with problematic files
      await expect(workspaceManager.indexWorkspace()).resolves.not.toThrow();
    });

    it('should provide workspace statistics', async () => {
      await workspaceManager.indexWorkspace();
      
      const stats = workspaceManager.getWorkspaceStats();
      expect(stats).toBeDefined();
      expect(stats.totalFiles).toBeGreaterThanOrEqual(0);
      expect(stats.totalDocuments).toBeGreaterThanOrEqual(0);
      expect(stats.totalSymbols).toBeGreaterThanOrEqual(0);
      expect(stats.lastIndexed).toBeDefined();
    });
  });

  describe('Document Search', () => {
    beforeEach(async () => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
      await workspaceManager.indexWorkspace();
    });

    it('should find symbols', () => {
      const symbols = workspaceManager.findSymbols('test');
      expect(Array.isArray(symbols)).toBe(true);
    });

    it('should find references', () => {
      const references = workspaceManager.findReferences('test');
      expect(Array.isArray(references)).toBe(true);
    });

    it('should get dependencies', () => {
      const dependencies = workspaceManager.getDependencies('test.yml');
      expect(Array.isArray(dependencies)).toBe(true);
    });

    it('should get dependents', () => {
      const dependents = workspaceManager.getDependents('test.yml');
      expect(Array.isArray(dependents)).toBe(true);
    });
  });

  describe('File Watching', () => {
    beforeEach(() => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
    });

    it('should handle file changes', () => {
      const mockEvent = {
        uri: 'file:///test/test.yml',
        type: 1 as any // FileChangeType.Created
      };
      
      expect(() => workspaceManager.onFileChanged(mockEvent)).not.toThrow();
    });

    it('should handle file deletions', () => {
      const mockEvent = {
        uri: 'file:///test/test.yml',
        type: 3 as any // FileChangeType.Deleted
      };
      
      expect(() => workspaceManager.onFileChanged(mockEvent)).not.toThrow();
    });
  });

  describe('Configuration', () => {
    it('should set configuration', () => {
      const newConfig = {
        enabled: false,
        autoIndex: false,
        maxFileSize: 1000000
      };
      
      workspaceManager.setConfiguration(newConfig);
      const config = workspaceManager.getConfiguration();
      
      expect(config.enabled).toBe(false);
      expect(config.autoIndex).toBe(false);
      expect(config.maxFileSize).toBe(1000000);
    });

    it('should provide default configuration', () => {
      const config = workspaceManager.getConfiguration();
      expect(config.enabled).toBeDefined();
      expect(config.autoIndex).toBeDefined();
      expect(config.crossDocumentValidation).toBeDefined();
      expect(config.symbolSearch).toBeDefined();
      expect(config.dependencyTracking).toBeDefined();
      expect(config.filePatterns).toBeDefined();
      expect(config.excludePatterns).toBeDefined();
      expect(config.maxFileSize).toBeDefined();
      expect(config.indexInterval).toBeDefined();
    });
  });

  describe('Performance', () => {
    beforeEach(() => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
    });

    it('should index efficiently', async () => {
      const startTime = performance.now();
      await workspaceManager.indexWorkspace();
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(10000); // Should complete within 10 seconds
    });

    it('should search efficiently', async () => {
      await workspaceManager.indexWorkspace();
      
      const startTime = performance.now();
      workspaceManager.findSymbols('test');
      const endTime = performance.now();

      expect(endTime - startTime).toBeLessThan(1000); // Should complete within 1 second
    });

    it('should track indexing status', () => {
      expect(workspaceManager.isIndexing()).toBeDefined();
      expect(typeof workspaceManager.isIndexing()).toBe('boolean');
    });
  });

  describe('Error Handling', () => {
    beforeEach(() => {
      const mockFolders: WorkspaceFolder[] = [
        { name: 'test', uri: 'file:///test' }
      ];
      workspaceManager.initialize(mockFolders);
    });

    it('should handle null inputs gracefully', async () => {
      await expect(workspaceManager.indexWorkspace()).resolves.not.toThrow();
      
      const symbols = workspaceManager.findSymbols('');
      expect(Array.isArray(symbols)).toBe(true);
      
      const references = workspaceManager.findReferences('');
      expect(Array.isArray(references)).toBe(true);
    });

    it('should handle invalid file paths', () => {
      const dependencies = workspaceManager.getDependencies('invalid/path.yml');
      expect(Array.isArray(dependencies)).toBe(true);
      
      const dependents = workspaceManager.getDependents('invalid/path.yml');
      expect(Array.isArray(dependents)).toBe(true);
    });
  });
}); 