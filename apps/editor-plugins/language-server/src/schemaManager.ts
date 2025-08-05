export interface RhemaSchema {
  name: string;
  version: string;
  description: string;
  schema: any;
  documentTypes: string[];
}

export class RhemaSchemaManager {
  private schemas: Map<string, RhemaSchema> = new Map();
  private defaultSchemas: RhemaSchema[] = [];

  initialize(): void {
    this.loadDefaultSchemas();
  }

  registerSchema(schema: RhemaSchema): void {
    this.schemas.set(schema.name, schema);
  }

  getSchema(name: string): RhemaSchema | null {
    return this.schemas.get(name) || null;
  }

  getSchemas(): RhemaSchema[] {
    return Array.from(this.schemas.values());
  }

  getSchemaForDocumentType(documentType: string): RhemaSchema | null {
    for (const schema of this.schemas.values()) {
      if (schema.documentTypes.includes(documentType)) {
        return schema;
      }
    }
    return null;
  }

  validateAgainstSchema(document: any, schemaName: string): { valid: boolean; errors: string[] } {
    const schema = this.getSchema(schemaName);
    if (!schema) {
      return { valid: false, errors: ['Schema not found'] };
    }

    // Simple validation - in a real implementation, you'd use a proper JSON schema validator
    const errors: string[] = [];

    // Check required fields
    if (schema.schema.required) {
      schema.schema.required.forEach((field: string) => {
        if (!document[field]) {
          errors.push(`Missing required field: ${field}`);
        }
      });
    }

    // Check field types
    if (schema.schema.properties) {
      Object.keys(schema.schema.properties).forEach((field) => {
        const property = schema.schema.properties[field];
        const value = document[field];

        if (value !== undefined) {
          if (property.type === 'string' && typeof value !== 'string') {
            errors.push(`Field '${field}' must be a string`);
          } else if (property.type === 'number' && typeof value !== 'number') {
            errors.push(`Field '${field}' must be a number`);
          } else if (property.type === 'array' && !Array.isArray(value)) {
            errors.push(`Field '${field}' must be an array`);
          } else if (
            property.type === 'object' &&
            (typeof value !== 'object' || Array.isArray(value))
          ) {
            errors.push(`Field '${field}' must be an object`);
          }
        }
      });
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }

  private loadDefaultSchemas(): void {
    // Scope schema
    const scopeSchema: RhemaSchema = {
      name: 'scope',
      version: '1.0.0',
      description: 'Schema for Rhema scope documents',
      documentTypes: ['scope'],
      schema: {
        type: 'object',
        properties: {
          name: { type: 'string' },
          description: { type: 'string' },
          version: { type: 'string' },
          contexts: { type: 'array' },
          dependencies: { type: 'array' },
          config: { type: 'object' },
          metadata: { type: 'object' },
        },
        required: ['name'],
      },
    };

    // Knowledge schema
    const knowledgeSchema: RhemaSchema = {
      name: 'knowledge',
      version: '1.0.0',
      description: 'Schema for Rhema knowledge documents',
      documentTypes: ['knowledge'],
      schema: {
        type: 'object',
        properties: {
          contexts: { type: 'array' },
          patterns: { type: 'array' },
          conventions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    // Todos schema
    const todosSchema: RhemaSchema = {
      name: 'todos',
      version: '1.0.0',
      description: 'Schema for Rhema todos documents',
      documentTypes: ['todos'],
      schema: {
        type: 'object',
        properties: {
          tasks: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    // Decisions schema
    const decisionsSchema: RhemaSchema = {
      name: 'decisions',
      version: '1.0.0',
      description: 'Schema for Rhema decisions documents',
      documentTypes: ['decisions'],
      schema: {
        type: 'object',
        properties: {
          decisions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    // Patterns schema
    const patternsSchema: RhemaSchema = {
      name: 'patterns',
      version: '1.0.0',
      description: 'Schema for Rhema patterns documents',
      documentTypes: ['patterns'],
      schema: {
        type: 'object',
        properties: {
          patterns: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    // Conventions schema
    const conventionsSchema: RhemaSchema = {
      name: 'conventions',
      version: '1.0.0',
      description: 'Schema for Rhema conventions documents',
      documentTypes: ['conventions'],
      schema: {
        type: 'object',
        properties: {
          conventions: { type: 'array' },
          metadata: { type: 'object' },
        },
      },
    };

    // Register default schemas
    this.registerSchema(scopeSchema);
    this.registerSchema(knowledgeSchema);
    this.registerSchema(todosSchema);
    this.registerSchema(decisionsSchema);
    this.registerSchema(patternsSchema);
    this.registerSchema(conventionsSchema);

    this.defaultSchemas = [
      scopeSchema,
      knowledgeSchema,
      todosSchema,
      decisionsSchema,
      patternsSchema,
      conventionsSchema,
    ];
  }

  getDefaultSchemas(): RhemaSchema[] {
    return this.defaultSchemas;
  }

  clearSchemas(): void {
    this.schemas.clear();
    this.loadDefaultSchemas(); // Reload default schemas
  }
}
