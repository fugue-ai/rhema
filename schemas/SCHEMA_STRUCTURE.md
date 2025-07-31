# Rhema Schema Structure

This directory contains the modular JSON Schema definitions for the Rhema Protocol.

## Overview

The schema has been split into individual files for better maintainability and modularity. Each file contains a specific schema definition that can be used independently or referenced from the main `rhema.json` file.

## File Structure

### Main Schema File
- **`rhema.json`** - Main schema collection file that imports all individual schemas using `$ref`

### Individual Schema Files
- **`scope.json`** - Defines the boundaries and purpose of a Rhema context scope
- **`knowledge.json`** - System knowledge about architecture and components
- **`todos.json`** - Work items and completion history tracking
- **`decisions.json`** - Architecture Decision Records (ADRs)
- **`patterns.json`** - Established patterns and best practices
- **`conventions.json`** - Team standards and guidelines

## Usage

### Using Individual Schemas
Each schema file can be used independently:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$ref": "./schemas/scope.json"
}
```

### Using the Complete Schema Collection
Reference the main schema file to access all definitions:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$ref": "./schemas/rhema.json#/$defs/rhema_scope"
}
```

## Benefits of Modular Structure

1. **Maintainability** - Each schema can be updated independently
2. **Reusability** - Individual schemas can be used in different contexts
3. **Clarity** - Easier to understand and work with specific schema components
4. **Versioning** - Individual schemas can be versioned separately
5. **Testing** - Each schema can be validated independently

## Schema IDs

Each schema file has a unique `$id` that follows the pattern:
`https://rhema.dev/schemas/v1/{schema_name}`

This allows for proper JSON Schema resolution and validation. 