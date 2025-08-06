# Lock File Schema Documentation

This document provides detailed information about the Rhema lock file schema, including data structures, validation rules, and usage examples.

## Schema Overview

The Rhema lock file uses a standardized JSON schema that ensures consistency and provides comprehensive dependency information. The schema is versioned and backward-compatible.

### Root Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Rhema Lock File Schema",
  "description": "Schema for Rhema lock files that provide deterministic dependency resolution",
  "type": "object",
  "required": ["metadata", "scopes", "dependencies", "checksum"],
  "properties": {
    "metadata": {
      "$ref": "#/definitions/LockMetadata"
    },
    "scopes": {
      "type": "object",
      "description": "Locked scope information",
      "additionalProperties": {
        "$ref": "#/definitions/LockedScope"
      }
    },
    "dependencies": {
      "type": "object",
      "description": "Locked dependency information",
      "additionalProperties": {
        "$ref": "#/definitions/LockedDependency"
      }
    },
    "checksum": {
      "type": "string",
      "description": "SHA-256 checksum of the lock file content",
      "pattern": "^[a-f0-9]{64}$"
    }
  }
}
```

## Data Structures

### LockMetadata

Contains metadata about the lock file generation:

```json
{
  "definitions": {
    "LockMetadata": {
      "type": "object",
      "required": ["version", "generated_at", "generator_version"],
      "properties": {
        "version": {
          "type": "string",
          "description": "Lock file schema version",
          "pattern": "^\\d+\\.\\d+\\.\\d+$"
        },
        "generated_at": {
          "type": "string",
          "format": "date-time",
          "description": "ISO 8601 timestamp when lock file was generated"
        },
        "generator_version": {
          "type": "string",
          "description": "Version of Rhema that generated this lock file"
        },
        "generator_config": {
          "type": "object",
          "description": "Configuration used during generation",
          "properties": {
            "resolution_strategy": {
              "type": "string",
              "enum": ["semantic", "pinned", "latest", "range"]
            },
            "allow_prereleases": {
              "type": "boolean"
            },
            "strict_validation": {
              "type": "boolean"
            }
          }
        },
        "repository_info": {
          "type": "object",
          "properties": {
            "repository_url": {
              "type": "string",
              "format": "uri"
            },
            "commit_hash": {
              "type": "string",
              "pattern": "^[a-f0-9]{40}$"
            },
            "branch": {
              "type": "string"
            }
          }
        }
      }
    }
  }
}
```

### LockedScope

Represents a locked scope with its dependencies and metadata:

```json
{
  "definitions": {
    "LockedScope": {
      "type": "object",
      "required": ["name", "version", "dependencies", "metadata"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Scope name"
        },
        "version": {
          "type": "string",
          "description": "Locked version of the scope"
        },
        "dependencies": {
          "type": "array",
          "items": {
            "$ref": "#/definitions/ScopeDependency"
          }
        },
        "metadata": {
          "type": "object",
          "properties": {
            "description": {
              "type": "string"
            },
            "authors": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "license": {
              "type": "string"
            },
            "repository": {
              "type": "string",
              "format": "uri"
            },
            "homepage": {
              "type": "string",
              "format": "uri"
            }
          }
        },
        "checksum": {
          "type": "string",
          "description": "SHA-256 checksum of scope content",
          "pattern": "^[a-f0-9]{64}$"
        },
        "resolved_at": {
          "type": "string",
          "format": "date-time"
        }
      }
    }
  }
}
```

### ScopeDependency

Represents a dependency within a scope:

```json
{
  "definitions": {
    "ScopeDependency": {
      "type": "object",
      "required": ["name", "version", "type"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Dependency name"
        },
        "version": {
          "type": "string",
          "description": "Locked version"
        },
        "type": {
          "type": "string",
          "enum": ["parent", "child", "peer", "dev", "optional"],
          "description": "Dependency type"
        },
        "constraint": {
          "type": "string",
          "description": "Original version constraint"
        },
        "resolved_by": {
          "type": "string",
          "enum": ["semantic", "pinned", "latest", "range"],
          "description": "Resolution strategy used"
        },
        "metadata": {
          "type": "object",
          "properties": {
            "description": {
              "type": "string"
            },
            "repository": {
              "type": "string",
              "format": "uri"
            },
            "license": {
              "type": "string"
            }
          }
        }
      }
    }
  }
}
```

### LockedDependency

Represents a global dependency across all scopes:

```json
{
  "definitions": {
    "LockedDependency": {
      "type": "object",
      "required": ["name", "version", "scopes"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Dependency name"
        },
        "version": {
          "type": "string",
          "description": "Locked version"
        },
        "scopes": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "List of scopes that depend on this dependency"
        },
        "constraints": {
          "type": "object",
          "additionalProperties": {
            "type": "string"
          },
          "description": "Version constraints by scope"
        },
        "conflicts": {
          "type": "array",
          "items": {
            "$ref": "#/definitions/DependencyConflict"
          },
          "description": "List of version conflicts"
        },
        "metadata": {
          "type": "object",
          "properties": {
            "description": {
              "type": "string"
            },
            "repository": {
              "type": "string",
              "format": "uri"
            },
            "license": {
              "type": "string"
            },
            "authors": {
              "type": "array",
              "items": {
                "type": "string"
              }
            }
          }
        },
        "checksum": {
          "type": "string",
          "description": "SHA-256 checksum of dependency content",
          "pattern": "^[a-f0-9]{64}$"
        }
      }
    }
  }
}
```

### DependencyConflict

Represents a version conflict between dependencies:

```json
{
  "definitions": {
    "DependencyConflict": {
      "type": "object",
      "required": ["dependency", "conflicting_versions", "affected_scopes"],
      "properties": {
        "dependency": {
          "type": "string",
          "description": "Name of the conflicting dependency"
        },
        "conflicting_versions": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "List of conflicting versions"
        },
        "affected_scopes": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Scopes affected by the conflict"
        },
        "resolution_strategy": {
          "type": "string",
          "enum": ["latest", "pinned", "manual", "unresolved"],
          "description": "Strategy used to resolve the conflict"
        },
        "resolved_version": {
          "type": "string",
          "description": "Version chosen to resolve the conflict"
        },
        "resolution_notes": {
          "type": "string",
          "description": "Notes about the resolution decision"
        }
      }
    }
  }
}
```

## Example Lock File

Here's a complete example of a Rhema lock file:

```json
{
  "metadata": {
    "version": "1.0.0",
    "generated_at": "2025-01-15T10:30:00Z",
    "generator_version": "1.0.0",
    "generator_config": {
      "resolution_strategy": "semantic",
      "allow_prereleases": false,
      "strict_validation": true
    },
    "repository_info": {
      "repository_url": "https://github.com/fugue-ai/rhema",
      "commit_hash": "a1b2c3d4e5f6789012345678901234567890abcd",
      "branch": "main"
    }
  },
  "scopes": {
    "core": {
      "name": "core",
      "version": "1.0.0",
      "dependencies": [
        {
          "name": "serde",
          "version": "1.0.195",
          "type": "parent",
          "constraint": "^1.0",
          "resolved_by": "semantic"
        },
        {
          "name": "tokio",
          "version": "1.35.1",
          "type": "parent",
          "constraint": "^1.0",
          "resolved_by": "semantic"
        }
      ],
      "metadata": {
        "description": "Core Rhema functionality",
        "authors": ["Rhema Team"],
        "license": "Apache-2.0"
      },
      "checksum": "f1e2d3c4b5a6789012345678901234567890abcdef1234567890abcdef123456",
      "resolved_at": "2025-01-15T10:30:00Z"
    },
    "agent": {
      "name": "agent",
      "version": "1.0.0",
      "dependencies": [
        {
          "name": "core",
          "version": "1.0.0",
          "type": "parent",
          "constraint": "^1.0",
          "resolved_by": "pinned"
        },
        {
          "name": "async-trait",
          "version": "0.1.77",
          "type": "parent",
          "constraint": "^0.1",
          "resolved_by": "semantic"
        }
      ],
      "metadata": {
        "description": "AI agent capabilities",
        "authors": ["Rhema Team"],
        "license": "Apache-2.0"
      },
      "checksum": "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456",
      "resolved_at": "2025-01-15T10:30:00Z"
    }
  },
  "dependencies": {
    "serde": {
      "name": "serde",
      "version": "1.0.195",
      "scopes": ["core"],
      "constraints": {
        "core": "^1.0"
      },
      "metadata": {
        "description": "Serialization framework for Rust",
        "repository": "https://github.com/serde-rs/serde",
        "license": "MIT OR Apache-2.0"
      },
      "checksum": "b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890"
    },
    "tokio": {
      "name": "tokio",
      "version": "1.35.1",
      "scopes": ["core"],
      "constraints": {
        "core": "^1.0"
      },
      "metadata": {
        "description": "Asynchronous runtime for Rust",
        "repository": "https://github.com/tokio-rs/tokio",
        "license": "MIT"
      },
      "checksum": "c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890ab"
    },
    "async-trait": {
      "name": "async-trait",
      "version": "0.1.77",
      "scopes": ["agent"],
      "constraints": {
        "agent": "^0.1"
      },
      "metadata": {
        "description": "Async trait support for Rust",
        "repository": "https://github.com/dtolnay/async-trait",
        "license": "MIT OR Apache-2.0"
      },
      "checksum": "d4e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcd"
    }
  },
  "checksum": "e5f6789012345678901234567890abcdef1234567890abcdef1234567890abcdef12"
}
```

## Validation Rules

### Schema Validation

The lock file must conform to the JSON schema with the following rules:

1. **Required Fields**: All required fields must be present
2. **Data Types**: All fields must have the correct data types
3. **Pattern Matching**: String fields must match specified patterns
4. **Enum Values**: Enum fields must contain valid values
5. **Format Validation**: Date-time fields must be valid ISO 8601 format

### Business Logic Validation

Additional validation rules beyond schema compliance:

1. **Checksum Verification**: The root checksum must match the computed checksum
2. **Scope Consistency**: All scopes referenced in dependencies must exist
3. **Version Consistency**: Dependency versions must be consistent across scopes
4. **Circular Dependency Detection**: No circular dependencies are allowed
5. **Constraint Satisfaction**: All version constraints must be satisfied

### Validation Commands

```bash
# Validate lock file schema
rhema lock validate --schema-only

# Validate business logic
rhema lock validate --business-logic

# Full validation (schema + business logic)
rhema lock validate --full

# Validate with detailed reporting
rhema lock validate --verbose --report
```

## Schema Evolution

### Versioning Strategy

The lock file schema uses semantic versioning:

- **Major Version**: Breaking changes that require migration
- **Minor Version**: New features that are backward-compatible
- **Patch Version**: Bug fixes and minor improvements

### Migration Process

When the schema version changes:

1. **Automatic Migration**: Rhema automatically migrates lock files to the latest schema
2. **Backward Compatibility**: Older schema versions are supported for a limited time
3. **Migration Validation**: Migrated lock files are validated for correctness
4. **Rollback Support**: Previous versions can be restored if needed

### Migration Commands

```bash
# Check if migration is needed
rhema lock migrate --check

# Perform migration
rhema lock migrate --auto

# Validate migration
rhema lock migrate --validate

# Rollback migration
rhema lock migrate --rollback
```

## Performance Considerations

### Schema Optimization

The schema is optimized for:

- **Fast Parsing**: Minimal nesting and simple data types
- **Efficient Validation**: Optimized validation rules
- **Compact Storage**: Minimal redundant information
- **Quick Access**: Direct property access for common fields

### Caching Strategy

Schema validation results are cached to improve performance:

- **Schema Cache**: Parsed schema is cached in memory
- **Validation Cache**: Validation results are cached with TTL
- **Migration Cache**: Migration paths are cached for reuse

### Performance Metrics

- **Schema Parsing**: < 1ms for typical lock files
- **Validation Time**: < 5ms for full validation
- **Memory Usage**: < 1MB for schema and validation cache
- **Cache Hit Rate**: > 95% for repeated validations 