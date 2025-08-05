import type { Connection } from 'vscode-languageserver/node';

export interface RhemaConfiguration {
  validation: {
    enabled: boolean;
    strict: boolean;
    schemaValidation: boolean;
  };
  formatting: {
    enabled: boolean;
    indentSize: number;
    useSpaces: boolean;
    lineWidth: number;
  };
  completion: {
    enabled: boolean;
    snippets: boolean;
    autoTrigger: boolean;
  };
  diagnostics: {
    enabled: boolean;
    maxProblems: number;
    delay: number;
  };
  performance: {
    enabled: boolean;
    maxMemory: number;
    timeout: number;
  };
}

export class RhemaConfigurationManager {
  private connection: Connection | null = null;
  private hasConfigurationCapability = false;
  private configuration: RhemaConfiguration;

  constructor() {
    this.configuration = this.getDefaultConfiguration();
  }

  initialize(connection: Connection, hasConfigurationCapability: boolean): void {
    this.connection = connection;
    this.hasConfigurationCapability = hasConfigurationCapability;
  }

  updateConfiguration(settings: any): void {
    if (settings.rhema) {
      if (this.isValidConfiguration(settings.rhema)) {
        this.configuration = this.deepMerge(this.getDefaultConfiguration(), settings.rhema);
      }
    } else {
      // Handle flat configuration object
      if (this.isValidConfiguration(settings)) {
        this.configuration = this.deepMerge(this.getDefaultConfiguration(), settings);
      }
    }
  }

  private isValidConfiguration(config: any): boolean {
    // Check if all keys in the configuration are valid
    for (const key in config) {
      if (config[key] && typeof config[key] === 'object' && !Array.isArray(config[key])) {
        if (!this.isValidConfiguration(config[key])) {
          return false;
        }
      } else {
        if (!this.isValidConfigurationKey(key) || !this.isValidValue(key, config[key])) {
          return false;
        }
      }
    }
    return true;
  }

  private deepMerge(target: any, source: any): any {
    const result = { ...target };
    
    for (const key in source) {
      if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
        result[key] = this.deepMerge(target[key] || {}, source[key]);
      } else {
        // Only set the value if it's a valid configuration key and value
        if (this.isValidConfigurationKey(key) && this.isValidValue(key, source[key])) {
          result[key] = source[key];
        }
      }
    }
    
    return result;
  }

  private isValidConfigurationKey(key: string): boolean {
    // Define valid configuration keys
    const validKeys = [
      'enabled', 'strict', 'schemaValidation',
      'indentSize', 'useSpaces', 'lineWidth',
      'snippets', 'autoTrigger',
      'maxProblems', 'delay',
      'maxMemory', 'timeout'
    ];
    
    return validKeys.includes(key);
  }

  private isValidValue(key: string, value: any): boolean {
    // Validate specific configuration values
    if (key === 'indentSize' && (typeof value !== 'number' || value <= 0)) {
      return false;
    }
    if (key === 'lineWidth' && (typeof value !== 'number' || value <= 0)) {
      return false;
    }
    if (key === 'maxMemory' && (typeof value !== 'number' || value <= 0)) {
      return false;
    }
    if (key === 'timeout' && (typeof value !== 'number' || value <= 0)) {
      return false;
    }
    if (key === 'enabled' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'useSpaces' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'strict' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'schemaValidation' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'snippets' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'autoTrigger' && typeof value !== 'boolean') {
      return false;
    }
    if (key === 'maxProblems' && (typeof value !== 'number' || value <= 0)) {
      return false;
    }
    if (key === 'delay' && (typeof value !== 'number' || value < 0)) {
      return false;
    }
    
    return true;
  }

  getConfiguration(): RhemaConfiguration {
    return this.configuration;
  }

  async getDocumentConfiguration(resource: string): Promise<RhemaConfiguration> {
    if (!this.hasConfigurationCapability) {
      return this.configuration;
    }

    try {
      const result = await this.connection?.workspace.getConfiguration({
        scopeUri: resource,
        section: 'rhema',
      });

      if (result) {
        return {
          ...this.getDefaultConfiguration(),
          ...result,
        };
      }
    } catch (error) {
      console.error('Error getting document configuration:', error);
    }

    return this.configuration;
  }

  private getDefaultConfiguration(): RhemaConfiguration {
    return {
      validation: {
        enabled: true,
        strict: false,
        schemaValidation: true,
      },
      formatting: {
        enabled: true,
        indentSize: 2,
        useSpaces: true,
        lineWidth: 80,
      },
      completion: {
        enabled: true,
        snippets: true,
        autoTrigger: true,
      },
      diagnostics: {
        enabled: true,
        maxProblems: 100,
        delay: 500,
      },
      performance: {
        enabled: true,
        maxMemory: 512,
        timeout: 5000,
      },
    };
  }
}
