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
      this.configuration = {
        ...this.getDefaultConfiguration(),
        ...settings.rhema,
      };
    }
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
