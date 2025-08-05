#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// List of all Rust crates from Cargo.toml workspace members
const rustCrates = [
  'core',
  'query', 
  'git',
  'ai',
  'mcp',
  'config',
  'monitoring',
  'integrations',
  'cli',
  'action',
  'dependency',
  'locomo',
  'knowledge',
  'agent'
];

// Template for library crates
const libraryTemplate = (crateName) => `{
  "name": "rhema-${crateName}",
  "$schema": "../../node_modules/nx/schemas/project-schema.json",
  "sourceRoot": "crates/${crateName}/src",
  "projectType": "library",
  "targets": {
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo build",
        "cwd": "crates/${crateName}"
      }
    },
    "build:release": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo build --release",
        "cwd": "crates/${crateName}"
      }
    },
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo test",
        "cwd": "crates/${crateName}"
      }
    },
    "check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo check",
        "cwd": "crates/${crateName}"
      }
    },
    "clippy": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo clippy",
        "cwd": "crates/${crateName}"
      }
    },
    "fmt": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo fmt",
        "cwd": "crates/${crateName}"
      }
    },
    "fmt:check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo fmt --check",
        "cwd": "crates/${crateName}"
      }
    },
    "clean": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo clean",
        "cwd": "crates/${crateName}"
      }
    },
    "doc": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo doc",
        "cwd": "crates/${crateName}"
      }
    }
  },
  "tags": ["type:lib", "scope:rust", "scope:${crateName}"]
}`;

// Template for application crates (those with binaries)
const applicationTemplate = (crateName) => `{
  "name": "rhema-${crateName}",
  "$schema": "../../node_modules/nx/schemas/project-schema.json",
  "sourceRoot": "crates/${crateName}/src",
  "projectType": "application",
  "targets": {
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo build",
        "cwd": "crates/${crateName}"
      }
    },
    "build:release": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo build --release",
        "cwd": "crates/${crateName}"
      }
    },
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo test",
        "cwd": "crates/${crateName}"
      }
    },
    "check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo check",
        "cwd": "crates/${crateName}"
      }
    },
    "clippy": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo clippy",
        "cwd": "crates/${crateName}"
      }
    },
    "fmt": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo fmt",
        "cwd": "crates/${crateName}"
      }
    },
    "fmt:check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo fmt --check",
        "cwd": "crates/${crateName}"
      }
    },
    "clean": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo clean",
        "cwd": "crates/${crateName}"
      }
    },
    "run": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo run",
        "cwd": "crates/${crateName}"
      }
    },
    "run:release": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo run --release",
        "cwd": "crates/${crateName}"
      }
    },
    "doc": {
      "executor": "nx:run-commands",
      "options": {
        "command": "cargo doc",
        "cwd": "crates/${crateName}"
      }
    }
  },
  "tags": ["type:app", "scope:rust", "scope:${crateName}"]
}`;

// Crates that have binaries (applications)
const applicationCrates = ['cli', 'action'];

function hasBinary(crateName) {
  const cargoTomlPath = path.join(__dirname, '..', 'crates', crateName, 'Cargo.toml');
  if (!fs.existsSync(cargoTomlPath)) {
    return false;
  }
  
  const cargoContent = fs.readFileSync(cargoTomlPath, 'utf8');
  return cargoContent.includes('[[bin]]') || applicationCrates.includes(crateName);
}

function generateProjectJson(crateName) {
  const projectPath = path.join(__dirname, '..', 'crates', crateName, 'project.json');
  
  // Skip if already exists
  if (fs.existsSync(projectPath)) {
    console.log(`Skipping ${crateName} - project.json already exists`);
    return;
  }
  
  const isApplication = hasBinary(crateName);
  const template = isApplication ? applicationTemplate : libraryTemplate;
  const content = template(crateName);
  
  fs.writeFileSync(projectPath, content);
  console.log(`Generated project.json for ${crateName} (${isApplication ? 'application' : 'library'})`);
}

// Create scripts directory if it doesn't exist
const scriptsDir = path.join(__dirname);
if (!fs.existsSync(scriptsDir)) {
  fs.mkdirSync(scriptsDir, { recursive: true });
}

console.log('Generating project.json files for Rust crates...');

rustCrates.forEach(generateProjectJson);

console.log('Done! You can now use nx commands like:');
console.log('  npx nx build rhema');
console.log('  npx nx test rhema-core');
console.log('  npx nx run rhema-cli:run');
console.log('  npx nx affected:build'); 