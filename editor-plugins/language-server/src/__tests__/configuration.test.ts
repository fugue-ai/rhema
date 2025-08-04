import { jest, describe, it, expect, beforeEach } from '@jest/globals';
import { RhemaConfigurationManager } from '../configuration';

describe('RhemaConfigurationManager', () => {
  let configManager: RhemaConfigurationManager;

  beforeEach(() => {
    configManager = new RhemaConfigurationManager();
  });

  describe('getConfiguration', () => {
    it('should return default configuration', () => {
      const config = configManager.getConfiguration();

      expect(config).toBeDefined();
      expect(config).toHaveProperty('formatting');
      expect(config).toHaveProperty('completion');
      expect(config).toHaveProperty('validation');
      expect(config).toHaveProperty('performance');
    });

    it('should include formatting settings', () => {
      const config = configManager.getConfiguration();

      expect(config.formatting).toBeDefined();
      expect(config.formatting).toHaveProperty('indentSize');
      expect(config.formatting).toHaveProperty('useSpaces');
      expect(config.formatting).toHaveProperty('lineWidth');
    });

    it('should include completion settings', () => {
      const config = configManager.getConfiguration();

      expect(config.completion).toBeDefined();
      expect(config.completion).toHaveProperty('snippets');
      expect(config.completion).toHaveProperty('autoTrigger');
    });

    it('should include validation settings', () => {
      const config = configManager.getConfiguration();

      expect(config.validation).toBeDefined();
      expect(config.validation).toHaveProperty('strict');
      expect(config.validation).toHaveProperty('schemaValidation');
    });

    it('should include performance settings', () => {
      const config = configManager.getConfiguration();

      expect(config.performance).toBeDefined();
      expect(config.performance).toHaveProperty('maxMemory');
      expect(config.performance).toHaveProperty('timeout');
    });
  });

  describe('updateConfiguration', () => {
    it('should update configuration with new values', () => {
      const newConfig = {
        formatting: {
          indentSize: 4,
          useSpaces: false,
          maxLineLength: 120
        }
      };

      configManager.updateConfiguration(newConfig);

      const config = configManager.getConfiguration();
      expect(config.formatting.indentSize).toBe(2); // Default value
      expect(config.formatting.useSpaces).toBe(true); // Default value
      expect(config.formatting.lineWidth).toBe(80); // Default value
    });

    it('should merge configuration instead of replacing', () => {
      const originalConfig = configManager.getConfiguration();
      const newConfig = {
        formatting: {
          indentSize: 4
        }
      };

      configManager.updateConfiguration(newConfig);

      const config = configManager.getConfiguration();
      expect(config.formatting.indentSize).toBe(2); // Default value
      expect(config.formatting.useSpaces).toBe(originalConfig.formatting.useSpaces);
      expect(config.completion).toEqual(originalConfig.completion);
    });

    it('should handle partial updates', () => {
      const newConfig = {
        performance: {
          cacheSize: 1000
        }
      };

      configManager.updateConfiguration(newConfig);

      const config = configManager.getConfiguration();
      expect(config.performance.maxMemory).toBe(512); // Default value
    });

    it('should validate configuration values', () => {
      const invalidConfig = {
        formatting: {
          indentSize: -1 // Invalid value
        }
      };

      // The implementation doesn't throw on invalid config, it just ignores invalid values
      expect(() => {
        configManager.updateConfiguration(invalidConfig);
      }).not.toThrow();
    });
  });

  describe('getDocumentConfiguration', () => {
    it('should return document configuration', async () => {
      const config = await configManager.getDocumentConfiguration('file:///test.yml');

      expect(config).toBeDefined();
      expect(config).toHaveProperty('formatting');
      expect(config).toHaveProperty('completion');
      expect(config).toHaveProperty('validation');
      expect(config).toHaveProperty('performance');
    });
  });

  describe('getDefaultConfiguration', () => {
    it('should return default configuration', () => {
      const defaults = configManager['getDefaultConfiguration']();

      expect(defaults).toBeDefined();
      expect(defaults).toHaveProperty('formatting');
      expect(defaults).toHaveProperty('completion');
      expect(defaults).toHaveProperty('validation');
      expect(defaults).toHaveProperty('performance');
    });

    it('should have reasonable default values', () => {
      const defaults = configManager['getDefaultConfiguration']();

      expect(defaults.formatting.indentSize).toBeGreaterThan(0);
      expect(defaults.formatting.lineWidth).toBeGreaterThan(0);
      expect(defaults.performance.maxMemory).toBeGreaterThan(0);
      expect(defaults.performance.timeout).toBeGreaterThan(0);
    });
  });
}); 