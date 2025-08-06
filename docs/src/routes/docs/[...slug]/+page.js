import { base } from '$app/paths';

export const prerender = true;

// Generate all possible doc routes for static pre-rendering
export const entries = async () => {
  // List all the doc routes that actually have markdown files
  const routes = [
    // Getting Started
    { slug: 'getting-started/quick-start' },
    { slug: 'getting-started/workspace-quick-start' },
    { slug: 'getting-started/refactoring-to-workspace' },
    
    // User Guide
    { slug: 'user-guide/cli-command-reference' },
    { slug: 'user-guide/configuration-management' },
    { slug: 'user-guide/batch-operations' },
    { slug: 'user-guide/interactive-mode' },
    { slug: 'user-guide/performance-monitoring' },
    { slug: 'user-guide/conflict-resolution' },
    { slug: 'user-guide/enhanced-dependencies-command' },
    { slug: 'user-guide/enhanced-validation-command' },
    { slug: 'user-guide/lock-file-health-checks' },
    
    // Core Features
    { slug: 'core-features/README' },
    { slug: 'core-features/lock-configuration-system' },
    { slug: 'core-features/lock-file-cache-system' },
    { slug: 'core-features/lock-file-ai-integration' },
    { slug: 'core-features/ci-cd-lock-file-integration' },
    { slug: 'core-features/ai-service-lock-file-enhancement' },
    { slug: 'core-features/conflict-resolver-usage' },
    { slug: 'core-features/context-query-language' },
    { slug: 'core-features/scope-management' },
    { slug: 'core-features/validation-system' },
    
    // Reference
    { slug: 'reference/global-config-reference' },
    { slug: 'reference/specification-schema-examples' },
    
    // Development Setup
    { slug: 'development-setup/development' },
    { slug: 'development-setup/development/local-setup' },
    { slug: 'development-setup/development/rust-setup' },
    { slug: 'development-setup/development/git-setup' },
    { slug: 'development-setup/editor-setup/vscode' },
    { slug: 'development-setup/editor-setup/cursor' },
    { slug: 'development-setup/editor-setup/intellij' },
    { slug: 'development-setup/editor-setup/sublime' },
    { slug: 'development-setup/editor-setup/vim' },
    { slug: 'development-setup/development/cicd/github-actions' },
    { slug: 'development-setup/development/cicd/local-pipeline-execution' },
    { slug: 'development-setup/development/cicd/pull-request-pipeline' },
    
    // Architecture
    { slug: 'architecture/README' },
    
    // Examples
    { slug: 'examples/README' },
    { slug: 'examples/advanced-usage' },
    { slug: 'examples/cql-queries' },
    { slug: 'examples/ecommerce-epic-orchestration' },
    { slug: 'examples/enhanced-context-injection' },
    { slug: 'examples/implicit-to-explicit-knowledge' },
    { slug: 'examples/lock-file-operations' },
    { slug: 'examples/prompt-chain-persistence' },
    { slug: 'examples/prompt-effectiveness-tracking' },
    { slug: 'examples/prompt-versioning' },
    { slug: 'examples/query-provenance' },
    { slug: 'examples/template-management' },
    { slug: 'examples/quick-start-commands' }
  ];
  
  return routes;
}; 