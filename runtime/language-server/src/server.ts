/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  type InitializeParams,
  DidChangeConfigurationNotification,
  type CompletionItem,
  CompletionItemKind,
  type TextDocumentPositionParams,
  TextDocumentSyncKind,
  type InitializeResult,
  type Diagnostic,
  DiagnosticSeverity,
  Range,
  Position,
  type Hover,
  MarkedString,
  type Definition,
  type Location,
  type ReferenceParams,
  SymbolInformation,
  SymbolKind,
  type DocumentSymbol,
  type CodeAction,
  CodeActionKind,
  type WorkspaceSymbolParams,
  type RenameParams,
  type WorkspaceEdit,
  type TextEdit,
  type DocumentFormattingParams,
  type DocumentRangeFormattingParams,
  type FoldingRange,
  FoldingRangeKind,
  type SemanticTokens,
  SemanticTokensLegend,
  type SemanticTokensParams,
  SemanticTokensRequest,
  SemanticTokensDeltaRequest,
  type SemanticTokensDeltaParams,
  type SemanticTokensDelta,
  SemanticTokensEdit,
  InlayHint,
  InlayHintKind,
  InlayHintLabelPart,
  DocumentLink,
  DocumentLinkParams,
  ColorInformation,
  ColorPresentation,
  Color,
  ColorPresentationParams,
  DocumentColorParams,
  CodeLens,
  CodeLensParams,
  Command,
  type ExecuteCommandParams,
  RequestType,
  NotificationType,
  TextDocumentIdentifier,
  VersionedTextDocumentIdentifier,
  TextDocumentContentChangeEvent,
  DidOpenTextDocumentParams,
  DidChangeTextDocumentParams,
  DidCloseTextDocumentParams,
  DidSaveTextDocumentParams,
  WillSaveTextDocumentParams,
  TextEdit as LSPTextEdit,
  WorkspaceFolder,
  FileEvent,
  FileChangeType,
  DidChangeWatchedFilesParams,
  ConfigurationItem,
  ConfigurationParams,
  type WorkspaceSymbol,
  DocumentHighlight,
  DocumentHighlightKind,
  SignatureHelp,
  SignatureInformation,
  ParameterInformation,
  CompletionList,
  CompletionContext,
  InsertTextFormat,
  MarkupKind,
  MarkupContent,
  TextDocumentItem,
} from 'vscode-languageserver/node';

type TextDocument = VSCodeTextDocument;

import { TextDocument as VSCodeTextDocument } from 'vscode-languageserver-textdocument';

import * as path from 'path';
import * as fs from 'fs-extra';
import * as yaml from 'yaml';
import * as Ajv from 'ajv';
import * as _ from 'lodash';
import { v4 as uuidv4 } from 'uuid';

import { RhemaParser } from './parser';
import { RhemaValidator } from './validator';
import { RhemaCompleter } from './completer';
import { RhemaHoverProvider } from './hover';
import { RhemaDefinitionProvider } from './definition';
import { RhemaReferenceProvider } from './reference';
import { RhemaSymbolProvider } from './symbol';
import { RhemaCodeActionProvider } from './codeAction';
import { RhemaFormatter } from './formatter';
import { RhemaSemanticTokensProvider } from './semanticTokens';
import { RhemaConfigurationManager } from './configuration';
import { RhemaLogger } from './logger';
import { RhemaErrorHandler } from './errorHandler';
import { RhemaPerformanceMonitor } from './performanceMonitor';
import { RhemaCache } from './cache';
import { RhemaSchemaManager } from './schemaManager';
import { RhemaWorkspaceManager } from './workspaceManager';
import { RhemaPerformanceOptimizer } from './performanceOptimizer';

// Create a connection for the server
const connection = createConnection(ProposedFeatures.all);

// Create a text document manager
const documents: TextDocuments<TextDocument> = new TextDocuments(VSCodeTextDocument);

// Initialize providers
const parser = new RhemaParser();
const validator = new RhemaValidator();
const completer = new RhemaCompleter();
const hoverProvider = new RhemaHoverProvider();
const definitionProvider = new RhemaDefinitionProvider();
const referenceProvider = new RhemaReferenceProvider();
const symbolProvider = new RhemaSymbolProvider();
const codeActionProvider = new RhemaCodeActionProvider();
const formatter = new RhemaFormatter();
const semanticTokensProvider = new RhemaSemanticTokensProvider();
const configurationManager = new RhemaConfigurationManager();
const logger = new RhemaLogger();
const errorHandler = new RhemaErrorHandler(logger);
const performanceMonitor = new RhemaPerformanceMonitor();
const cache = new RhemaCache();
const schemaManager = new RhemaSchemaManager();
const workspaceManager = new RhemaWorkspaceManager();
const performanceOptimizer = new RhemaPerformanceOptimizer(cache, performanceMonitor);

// Server state
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;
let hasCodeActionLiteralSupport = false;
let hasSemanticTokensCapability = false;
let hasInlayHintCapability = false;
let hasCallHierarchyCapability = false;
let hasTypeHierarchyCapability = false;
let hasDocumentLinkCapability = false;
let hasColorProviderCapability = false;
let hasCodeLensCapability = false;
let hasDocumentHighlightCapability = false;
let hasSignatureHelpCapability = false;
let hasWorkspaceSymbolCapability = false;

// Initialize the server
connection.onInitialize(async (params: InitializeParams): Promise<InitializeResult> => {
  const capabilities = params.capabilities;

  // Check capabilities
  hasConfigurationCapability = !!(capabilities.workspace && !!capabilities.workspace.configuration);
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );
  hasDiagnosticRelatedInformationCapability = !!(
    capabilities.textDocument &&
    capabilities.textDocument.publishDiagnostics &&
    capabilities.textDocument.publishDiagnostics.relatedInformation
  );
  hasCodeActionLiteralSupport = !!(
    capabilities.textDocument &&
    capabilities.textDocument.codeAction &&
    capabilities.textDocument.codeAction.codeActionLiteralSupport
  );
  hasSemanticTokensCapability = !!(
    capabilities.textDocument && capabilities.textDocument.semanticTokens
  );
  hasInlayHintCapability = !!(capabilities.textDocument && capabilities.textDocument.inlayHint);
  hasCallHierarchyCapability = !!(
    capabilities.textDocument && capabilities.textDocument.callHierarchy
  );
  hasTypeHierarchyCapability = !!(
    capabilities.textDocument && capabilities.textDocument.typeHierarchy
  );
  hasDocumentLinkCapability = !!(
    capabilities.textDocument && capabilities.textDocument.documentLink
  );
  hasColorProviderCapability = !!(
    capabilities.textDocument && capabilities.textDocument.colorProvider
  );
  hasCodeLensCapability = !!(capabilities.textDocument && capabilities.textDocument.codeLens);
  hasDocumentHighlightCapability = !!(
    capabilities.textDocument && capabilities.textDocument.documentHighlight
  );
  hasSignatureHelpCapability = !!(
    capabilities.textDocument && capabilities.textDocument.signatureHelp
  );
  hasWorkspaceSymbolCapability = !!(capabilities.workspace && capabilities.workspace.symbol);

  // Initialize components
  configurationManager.initialize(connection, hasConfigurationCapability);
  // fileWatcher.initialize(connection, hasWorkspaceFolderCapability); // Removed as per new_code
  performanceMonitor.initialize();
  cache.initialize();
  schemaManager.initialize();
  // contextManager.initialize(); // Removed as per new_code

  // Initialize workspace manager if workspace folders are available
  if (hasWorkspaceFolderCapability && params.workspaceFolders) {
    await workspaceManager.initialize(params.workspaceFolders);
  }

  // Initialize performance optimizer
  performanceOptimizer.setConfiguration({
    enableCaching: true,
    enableMemoryOptimization: true,
    enableAsyncProcessing: true,
    enableBatchProcessing: true,
    performanceMonitoring: true,
  });

  // Initialize providers with capabilities
  parser.initialize(capabilities);
  validator.initialize(capabilities, hasDiagnosticRelatedInformationCapability);
  completer.initialize(capabilities);
  hoverProvider.initialize(capabilities);
  definitionProvider.initialize(capabilities);
  referenceProvider.initialize(capabilities);
  symbolProvider.initialize(capabilities);
  codeActionProvider.initialize(capabilities, hasCodeActionLiteralSupport, workspaceManager);
  formatter.initialize(capabilities);
  semanticTokensProvider.initialize(capabilities, hasSemanticTokensCapability);
  // inlayHintProvider.initialize(capabilities, hasInlayHintCapability); // Removed as per new_code
  // callHierarchyProvider.initialize(capabilities, hasCallHierarchyCapability); // Removed as per new_code
  // typeHierarchyProvider.initialize(capabilities, hasTypeHierarchyCapability); // Removed as per new_code
  // documentLinkProvider.initialize(capabilities, hasDocumentLinkCapability); // Removed as per new_code
  // colorProvider.initialize(capabilities, hasColorProviderCapability); // Removed as per new_code
  // codeLensProvider.initialize(capabilities, hasCodeLensCapability); // Removed as per new_code
  // documentHighlightProvider.initialize(capabilities, hasDocumentHighlightCapability); // Removed as per new_code
  // signatureHelpProvider.initialize(capabilities, hasSignatureHelpCapability); // Removed as per new_code
  // workspaceSymbolProvider.initialize(capabilities, hasWorkspaceSymbolCapability); // Removed as per new_code

  logger.info('Rhema Language Server initialized');

  return {
    capabilities: {
      textDocumentSync: 2, // TextDocumentSyncKind.Incremental
      // Tell the client that this server supports code completion
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: ['.', ':', '-', ' ', '\t', '\n'],
      },
      // Tell the client that this server supports hover
      hoverProvider: true,
      // Tell the client that this server supports go to definition
      definitionProvider: true,
      // Tell the client that this server supports find references
      referencesProvider: true,
      // Tell the client that this server supports document symbols
      documentSymbolProvider: true,
      // Tell the client that this server supports workspace symbols
      workspaceSymbolProvider: hasWorkspaceSymbolCapability,
      // Tell the client that this server supports code actions
      codeActionProvider: hasCodeActionLiteralSupport
        ? {
            codeActionKinds: [
              'quickfix',
              'refactor',
              'refactor.extract',
              'refactor.inline',
              'refactor.rewrite',
              'source',
              'source.organizeImports',
            ],
          }
        : true,
      // Tell the client that this server supports document formatting
      documentFormattingProvider: true,
      // Tell the client that this server supports range formatting
      documentRangeFormattingProvider: true,
      // Tell the client that this server supports folding
      foldingRangeProvider: true,
      // Tell the client that this server supports semantic tokens
      semanticTokensProvider: hasSemanticTokensCapability
        ? {
            legend: semanticTokensProvider.getLegend(),
            range: true,
            full: {
              delta: true,
            },
          }
        : undefined,
      // Tell the client that this server supports inlay hints
      inlayHintProvider: hasInlayHintCapability,
      // Tell the client that this server supports call hierarchy
      callHierarchyProvider: hasCallHierarchyCapability,
      // Tell the client that this server supports type hierarchy
      typeHierarchyProvider: hasTypeHierarchyCapability,
      // Tell the client that this server supports document links
      documentLinkProvider: hasDocumentLinkCapability
        ? {
            resolveProvider: true,
          }
        : undefined,
      // Tell the client that this server supports color provider
      colorProvider: hasColorProviderCapability,
      // Tell the client that this server supports code lens
      codeLensProvider: hasCodeLensCapability
        ? {
            resolveProvider: true,
          }
        : undefined,
      // Tell the client that this server supports document highlight
      documentHighlightProvider: hasDocumentHighlightCapability,
      // Tell the client that this server supports signature help
      signatureHelpProvider: hasSignatureHelpCapability
        ? {
            triggerCharacters: ['(', ',', ' '],
          }
        : undefined,
      // Tell the client that this server supports rename
      renameProvider: true,
      // Tell the client that this server supports execute command
      executeCommandProvider: {
        commands: [
          'rhema.validate',
          'rhema.format',
          'rhema.refactor',
          'rhema.generate',
          'rhema.debug',
          'rhema.profile',
          'rhema.sync',
          'rhema.export',
          'rhema.import',
          'rhema.migrate',
          'rhema.clean',
          'rhema.optimize',
          'rhema.backup',
          'rhema.restore',
          'rhema.analyze',
          'rhema.report',
          'rhema.test',
          'rhema.deploy',
          'rhema.monitor',
          'rhema.alert',
          'rhema.workspace.index',
          'rhema.workspace.search',
          'rhema.workspace.stats',
          'rhema.workspace.symbols',
          'rhema.workspace.references',
          'rhema.performance.optimize',
          'rhema.performance.metrics',
          'rhema.performance.memory',
          'rhema.performance.cache',
          'rhema.performance.batch',
        ],
      },
    },
  };
});

// Handle initialization
connection.onInitialized(() => {
  if (hasConfigurationCapability) {
    // Register for all configuration changes
    connection.client.register(DidChangeConfigurationNotification.type, undefined);
  }
  if (hasWorkspaceFolderCapability) {
    // connection.workspace.onDidChangeWorkspaceFolders(_event => { // Removed as per new_code
    //     connection.console.log('Workspace folder change event received.'); // Removed as per new_code
    // }); // Removed as per new_code
  }
  logger.info('Rhema Language Server initialized successfully');
});

// Handle configuration changes
connection.onDidChangeConfiguration((change) => {
  if (hasConfigurationCapability) {
    configurationManager.updateConfiguration(change.settings);
  }
  // Revalidate all open text documents
  documents.all().forEach(validateTextDocument);
});

// Handle workspace folder changes
connection.workspace.onDidChangeWorkspaceFolders(async (event) => {
  connection.console.log('Workspace folders changed');

  // Update workspace manager with new folders
  if (hasWorkspaceFolderCapability) {
    await workspaceManager.initialize(event.added);
  }

  // Revalidate all documents
  documents.all().forEach(validateTextDocument);
});

// Handle file watching
connection.onDidChangeWatchedFiles((change) => {
  // Monitored files have changed in VS Code
  connection.console.log('We received a file change event');

  // Notify workspace manager of file changes
  change.changes.forEach((event) => {
    // workspaceManager.onFileChanged(event); // Removed as per new_code
  });

  // Revalidate all documents
  documents.all().forEach(validateTextDocument);
});

// Handle document events
documents.onDidOpen((event) => {
  logger.info(`Document opened: ${event.document.uri}`);
  validateTextDocument(event.document);
});

documents.onDidChangeContent((event) => {
  logger.info(`Document changed: ${event.document.uri}`);
  validateTextDocument(event.document);
});

documents.onDidClose((event) => {
  logger.info(`Document closed: ${event.document.uri}`);
  // Clean up document-specific data
  cache.removeDocument(event.document.uri);
});

documents.onDidSave((event) => {
  logger.info(`Document saved: ${event.document.uri}`);
  validateTextDocument(event.document);
});

// Validation function
function validateTextDocument(textDocument: TextDocument): void {
  const startTime = performance.now();

  try {
    // Check if this is a Rhema file
    if (!isRhemaFile(textDocument.uri)) {
      return;
    }

    const diagnostics: Diagnostic[] = [];
    const text = textDocument.getText();

    // Parse the document
    const parseResult = parser.parse(text, textDocument.uri);

    if (parseResult.success) {
      // Validate the parsed content
      const validationResult = validator.validate(parseResult.data, textDocument.uri);
      diagnostics.push(...validationResult.diagnostics);

      // Cache the parsed and validated data
      cache.setDocument(textDocument.uri, {
        parseResult: parseResult.data,
        validationResult: validationResult,
        version: textDocument.version,
        lastModified: Date.now(),
      });
    } else {
      // Add parsing errors
      diagnostics.push(
        ...parseResult.errors.map((error) => ({
          range: error.range,
          message: error.message,
          severity: DiagnosticSeverity.Error,
          source: 'rhema-parser',
        }))
      );
    }

    // Send the computed diagnostics to VS Code
    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });

    const endTime = performance.now();
    performanceMonitor.recordOperation('validateTextDocument', endTime - startTime);
  } catch (error) {
    errorHandler.handleError(
      'Error validating text document',
      error instanceof Error ? error : new Error(String(error))
    );
  }
}

// Check if a file is a Rhema file
function isRhemaFile(uri: string): boolean {
  const fileName = path.basename(uri).toLowerCase();
  return (
    fileName.includes('.rhema.') ||
    fileName.includes('scope.yaml') ||
    fileName.includes('knowledge.yaml') ||
    fileName.includes('todos.yaml') ||
    fileName.includes('decisions.yaml') ||
    fileName.includes('patterns.yaml') ||
    fileName.includes('conventions.yaml')
  );
}

// Completion provider
connection.onCompletion((textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
  const startTime = performance.now();

  try {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
      return [];
    }

    const result = completer.provideCompletion(
      document,
      textDocumentPosition.position,
      cache.getDocument(textDocumentPosition.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('completion', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing completion',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Completion resolve provider
connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
  const startTime = performance.now();

  try {
    const result = completer.resolveCompletion(item);

    const endTime = performance.now();
    performanceMonitor.recordOperation('completionResolve', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error resolving completion',
      error instanceof Error ? error : new Error(String(error))
    );
    return item;
  }
});

// Hover provider
connection.onHover((textDocumentPosition: TextDocumentPositionParams): Hover | null => {
  const startTime = performance.now();

  try {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
      return null;
    }

    const result = hoverProvider.provideHover(
      document,
      textDocumentPosition.position,
      cache.getDocument(textDocumentPosition.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('hover', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return null;
  }
});

// Definition provider
connection.onDefinition((textDocumentPosition: TextDocumentPositionParams): Definition | null => {
  const startTime = performance.now();

  try {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
      return null;
    }

    const result = definitionProvider.provideDefinition(
      document,
      textDocumentPosition.position,
      cache.getDocument(textDocumentPosition.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('definition', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return null;
  }
});

// References provider
connection.onReferences((referenceParams: ReferenceParams): Location[] => {
  const startTime = performance.now();

  try {
    const document = documents.get(referenceParams.textDocument.uri);
    if (!document) {
      return [];
    }

    const result = referenceProvider.provideReferences(
      document,
      referenceParams.position,
      referenceParams.context,
      cache.getDocument(referenceParams.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('references', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Document symbols provider
connection.onDocumentSymbol((documentSymbolParams): DocumentSymbol[] => {
  const startTime = performance.now();

  try {
    const document = documents.get(documentSymbolParams.textDocument.uri);
    if (!document) {
      return [];
    }

    const result = symbolProvider.provideDocumentSymbols(
      document,
      cache.getDocument(documentSymbolParams.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('documentSymbols', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Workspace symbols provider
connection.onWorkspaceSymbol((workspaceSymbolParams: WorkspaceSymbolParams): WorkspaceSymbol[] => {
  const startTime = performance.now();

  try {
    const result = workspaceManager.findSymbols(workspaceSymbolParams.query); // Changed to use workspaceManager

    const endTime = performance.now();
    performanceMonitor.recordOperation('workspaceSymbols', endTime - startTime);

    return result as any;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Code actions provider
connection.onCodeAction((codeActionParams): CodeAction[] => {
  const startTime = performance.now();

  try {
    const document = documents.get(codeActionParams.textDocument.uri);
    if (!document) {
      return [];
    }

    const result = codeActionProvider.provideCodeActions(
      document,
      codeActionParams.range,
      codeActionParams.context,
      cache.getDocument(codeActionParams.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('codeActions', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Document formatting provider
connection.onDocumentFormatting(
  (documentFormattingParams: DocumentFormattingParams): TextEdit[] => {
    const startTime = performance.now();

    try {
      const document = documents.get(documentFormattingParams.textDocument.uri);
      if (!document) {
        return [];
      }

      const result = formatter.formatDocument(document, documentFormattingParams.options);

      const endTime = performance.now();
      performanceMonitor.recordOperation('documentFormatting', endTime - startTime);

      return result;
    } catch (error) {
      errorHandler.handleError(
        'Error providing hover',
        error instanceof Error ? error : new Error(String(error))
      );
      return [];
    }
  }
);

// Document range formatting provider
connection.onDocumentRangeFormatting(
  (documentRangeFormattingParams: DocumentRangeFormattingParams): TextEdit[] => {
    const startTime = performance.now();

    try {
      const document = documents.get(documentRangeFormattingParams.textDocument.uri);
      if (!document) {
        return [];
      }

      const result = formatter.formatDocumentRange(
        document,
        documentRangeFormattingParams.range,
        documentRangeFormattingParams.options
      );

      const endTime = performance.now();
      performanceMonitor.recordOperation('documentRangeFormatting', endTime - startTime);

      return result;
    } catch (error) {
      errorHandler.handleError(
        'Error providing hover',
        error instanceof Error ? error : new Error(String(error))
      );
      return [];
    }
  }
);

// Folding range provider
connection.onFoldingRanges((foldingRangeParams): FoldingRange[] => {
  const startTime = performance.now();

  try {
    const document = documents.get(foldingRangeParams.textDocument.uri);
    if (!document) {
      return [];
    }

    const result = formatter.provideFoldingRanges(
      document,
      cache.getDocument(foldingRangeParams.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('foldingRanges', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return [];
  }
});

// Semantic tokens provider
if (hasSemanticTokensCapability) {
  connection.onRequest(
    SemanticTokensRequest.type,
    (semanticTokensParams: SemanticTokensParams): SemanticTokens => {
      const startTime = performance.now();

      try {
        const document = documents.get(semanticTokensParams.textDocument.uri);
        if (!document) {
          return { data: [] };
        }

        const result = semanticTokensProvider.provideSemanticTokens(
          document,
          cache.getDocument(semanticTokensParams.textDocument.uri)
        );

        const endTime = performance.now();
        performanceMonitor.recordOperation('semanticTokens', endTime - startTime);

        return result;
      } catch (error) {
        errorHandler.handleError(
          'Error providing hover',
          error instanceof Error ? error : new Error(String(error))
        );
        return { data: [] };
      }
    }
  );

  connection.onRequest(
    SemanticTokensDeltaRequest.type,
    (semanticTokensDeltaParams: SemanticTokensDeltaParams): SemanticTokensDelta => {
      const startTime = performance.now();

      try {
        const document = documents.get(semanticTokensDeltaParams.textDocument.uri);
        if (!document) {
          return { edits: [] };
        }

        const result = semanticTokensProvider.provideSemanticTokensDelta(
          document,
          semanticTokensDeltaParams.previousResultId,
          cache.getDocument(semanticTokensDeltaParams.textDocument.uri)
        );

        const endTime = performance.now();
        performanceMonitor.recordOperation('semanticTokensDelta', endTime - startTime);

        return result;
      } catch (error) {
        errorHandler.handleError(
          'Error providing hover',
          error instanceof Error ? error : new Error(String(error))
        );
        return { edits: [] };
      }
    }
  );
}

// Inlay hints provider
if (hasInlayHintCapability) {
  // connection.onRequest( // Removed as per new_code
  //     DocumentInlayHintRequest.type, // Removed as per new_code
  //     (documentInlayHintParams: DocumentInlayHintParams): InlayHint[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(documentInlayHintParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = inlayHintProvider.provideInlayHints( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 documentInlayHintParams.range, // Removed as per new_code
  //                 cache.getDocument(documentInlayHintParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('inlayHints', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Call hierarchy provider
if (hasCallHierarchyCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/prepareCallHierarchy', // Removed as per new_code
  //     (callHierarchyPrepareParams): CallHierarchyItem[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(callHierarchyPrepareParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = callHierarchyProvider.prepareCallHierarchy( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 callHierarchyPrepareParams.position, // Removed as per new_code
  //                 cache.getDocument(callHierarchyPrepareParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('prepareCallHierarchy', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'callHierarchy/incomingCalls', // Removed as per new_code
  //     (callHierarchyIncomingCallsParams: CallHierarchyIncomingCallsParams): CallHierarchyIncomingCall[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = callHierarchyProvider.provideIncomingCalls( // Removed as per new_code
  //                 callHierarchyIncomingCallsParams.item // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('incomingCalls', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'callHierarchy/outgoingCalls', // Removed as per new_code
  //     (callHierarchyOutgoingCallsParams: CallHierarchyOutgoingCallsParams): CallHierarchyOutgoingCall[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = callHierarchyProvider.provideOutgoingCalls( // Removed as per new_code
  //                 callHierarchyOutgoingCallsParams.item // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('outgoingCalls', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Type hierarchy provider
if (hasTypeHierarchyCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/prepareTypeHierarchy', // Removed as per new_code
  //     (typeHierarchyPrepareParams: TypeHierarchyPrepareParams): TypeHierarchyItem[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(typeHierarchyPrepareParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = typeHierarchyProvider.prepareTypeHierarchy( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 typeHierarchyPrepareParams.position, // Removed as per new_code
  //                 cache.getDocument(typeHierarchyPrepareParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('prepareTypeHierarchy', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'typeHierarchy/supertypes', // Removed as per new_code
  //     (typeHierarchySupertypesParams: TypeHierarchySupertypesParams): TypeHierarchyItem[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = typeHierarchyProvider.provideSupertypes( // Removed as per new_code
  //                 typeHierarchySupertypesParams.item // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('supertypes', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'typeHierarchy/subtypes', // Removed as per new_code
  //     (typeHierarchySubtypesParams: TypeHierarchySubtypesParams): TypeHierarchyItem[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = typeHierarchyProvider.provideSubtypes( // Removed as per new_code
  //                 typeHierarchySubtypesParams.item // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('subtypes', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Document links provider
if (hasDocumentLinkCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/documentLink', // Removed as per new_code
  //     (documentLinkParams: DocumentLinkParams): DocumentLink[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(documentLinkParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = documentLinkProvider.provideDocumentLinks( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 cache.getDocument(documentLinkParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('documentLinks', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'documentLink/resolve', // Removed as per new_code
  //     (documentLink): DocumentLink => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = documentLinkProvider.resolveDocumentLink(documentLink); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('resolveDocumentLink', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return documentLink; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Color provider
if (hasColorProviderCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/documentColor', // Removed as per new_code
  //     (documentColorParams: DocumentColorParams): ColorInformation[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(documentColorParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = colorProvider.provideDocumentColors( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 cache.getDocument(documentColorParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('documentColors', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/colorPresentation', // Removed as per new_code
  //     (colorPresentationParams: ColorPresentationParams): ColorPresentation[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(colorPresentationParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = colorProvider.provideColorPresentations( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 colorPresentationParams.color, // Removed as per new_code
  //                 colorPresentationParams.range // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('colorPresentations', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Code lens provider
if (hasCodeLensCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/codeLens', // Removed as per new_code
  //     (codeLensParams: CodeLensParams): CodeLens[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(codeLensParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = codeLensProvider.provideCodeLenses( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 cache.getDocument(codeLensParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('codeLenses', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
  // connection.onRequest( // Removed as per new_code
  //     'codeLens/resolve', // Removed as per new_code
  //     (codeLens): CodeLens => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const result = codeLensProvider.resolveCodeLens(codeLens); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('resolveCodeLens', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return codeLens; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Document highlight provider
if (hasDocumentHighlightCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/documentHighlight', // Removed as per new_code
  //     (documentHighlightParams): DocumentHighlight[] => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(documentHighlightParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return []; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = documentHighlightProvider.provideDocumentHighlights( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 documentHighlightParams.position, // Removed as per new_code
  //                 cache.getDocument(documentHighlightParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('documentHighlights', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return []; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Signature help provider
if (hasSignatureHelpCapability) {
  // connection.onRequest( // Removed as per new_code
  //     'textDocument/signatureHelp', // Removed as per new_code
  //     (signatureHelpParams): SignatureHelp | null => { // Removed as per new_code
  //         const startTime = performance.now(); // Removed as per new_code
  //         try { // Removed as per new_code
  //             const document = documents.get(signatureHelpParams.textDocument.uri); // Removed as per new_code
  //             if (!document) { // Removed as per new_code
  //                 return null; // Removed as per new_code
  //             } // Removed as per new_code
  //             const result = signatureHelpProvider.provideSignatureHelp( // Removed as per new_code
  //                 document, // Removed as per new_code
  //                 signatureHelpParams.position, // Removed as per new_code
  //                 signatureHelpParams.context, // Removed as per new_code
  //                 cache.getDocument(signatureHelpParams.textDocument.uri) // Removed as per new_code
  //             ); // Removed as per new_code
  //             const endTime = performance.now(); // Removed as per new_code
  //             performanceMonitor.recordOperation('signatureHelp', endTime - startTime); // Removed as per new_code
  //             return result; // Removed as per new_code
  //         } catch (error) { // Removed as per new_code
  //             errorHandler.handleError('Error providing hover', error instanceof Error ? error : new Error(String(error))); // Removed as per new_code
  //             return null; // Removed as per new_code
  //         } // Removed as per new_code
  //     } // Removed as per new_code
  // ); // Removed as per new_code
}

// Rename provider
connection.onRequest('textDocument/rename', (renameParams: RenameParams): WorkspaceEdit | null => {
  const startTime = performance.now();

  try {
    const document = documents.get(renameParams.textDocument.uri);
    if (!document) {
      return null;
    }

    const result = codeActionProvider.provideRename(
      document,
      renameParams.position,
      renameParams.newName,
      cache.getDocument(renameParams.textDocument.uri)
    );

    const endTime = performance.now();
    performanceMonitor.recordOperation('rename', endTime - startTime);

    return result;
  } catch (error) {
    errorHandler.handleError(
      'Error providing hover',
      error instanceof Error ? error : new Error(String(error))
    );
    return null;
  }
});

// Execute command provider
connection.onRequest(
  'workspace/executeCommand',
  (executeCommandParams: ExecuteCommandParams): any => {
    const startTime = performance.now();

    try {
      const result = codeActionProvider.executeCommand(
        executeCommandParams.command,
        executeCommandParams.arguments || []
      );

      const endTime = performance.now();
      performanceMonitor.recordOperation('executeCommand', endTime - startTime);

      return result;
    } catch (error) {
      errorHandler.handleError(
        'Error providing hover',
        error instanceof Error ? error : new Error(String(error))
      );
      return null;
    }
  }
);

// Custom notifications
connection.onNotification('rhema/validate', (params: { uri: string }) => {
  const document = documents.get(params.uri);
  if (document) {
    validateTextDocument(document);
  }
});

connection.onNotification('rhema/format', (params: { uri: string }) => {
  const document = documents.get(params.uri);
  if (document) {
    const edits = formatter.formatDocument(document, { tabSize: 2, insertSpaces: true });
    if (edits.length > 0) {
      connection.workspace.applyEdit({
        documentChanges: [
          {
            textDocument: {
              uri: params.uri,
              version: document.version,
            },
            edits: edits,
          },
        ],
      });
    }
  }
});

connection.onNotification('rhema/refresh', () => {
  // Refresh all documents
  documents.all().forEach(validateTextDocument);
});

connection.onNotification('rhema/clearCache', () => {
  cache.clear();
});

// Performance monitoring
connection.onRequest('rhema/performance', () => {
  return performanceMonitor.getReport();
});

// Error reporting
connection.onRequest('rhema/errors', () => {
  return errorHandler.getErrors();
});

// Configuration
connection.onRequest('rhema/config', () => {
  return configurationManager.getConfiguration();
});

// Schema management
connection.onRequest('rhema/schemas', () => {
  return schemaManager.getSchemas();
});

// Context management
// connection.onRequest('rhema/context', () => { // Removed as per new_code
//     return contextManager.getContext(); // Removed as per new_code
// }); // Removed as per new_code

// Workspace management
connection.onRequest('rhema/workspace/index', () => {
  return workspaceManager.indexWorkspace();
});

connection.onRequest('rhema/workspace/search', (params: { query: string }) => {
  return workspaceManager.findSymbols(params.query);
});

connection.onRequest('rhema/workspace/stats', () => {
  return workspaceManager.getWorkspaceStats();
});

connection.onRequest('rhema/workspace/symbols', (params: { query: string }) => {
  return workspaceManager.findSymbols(params.query);
});

connection.onRequest('rhema/workspace/references', (params: { symbol: string }) => {
  return workspaceManager.findReferences(params.symbol);
});

connection.onRequest('rhema/workspace/dependencies', (params: { uri: string }) => {
  return workspaceManager.getDependencies(params.uri);
});

connection.onRequest('rhema/workspace/dependents', (params: { uri: string }) => {
  return workspaceManager.getDependents(params.uri);
});

// Performance optimization
connection.onRequest('rhema/performance/optimize', () => {
  return performanceOptimizer.getConfiguration();
});

connection.onRequest('rhema/performance/metrics', () => {
  return performanceOptimizer.getPerformanceMetrics();
});

connection.onRequest('rhema/performance/memory', () => {
  return performanceOptimizer.getMemoryProfile();
});

connection.onRequest('rhema/performance/cache', () => {
  return cache.getStats();
});

connection.onRequest('rhema/performance/batch', (params: { operation: any }) => {
  performanceOptimizer.addToBatch(params.operation);
  return { success: true, message: 'Batch operation queued' };
});

// Shutdown
connection.onShutdown(() => {
  logger.info('Rhema Language Server shutting down');
  performanceMonitor.shutdown();
  cache.clear();
  performanceOptimizer.dispose();
});

// Exit
connection.onExit(() => {
  logger.info('Rhema Language Server exited');
});

// Make the text document manager listen on the connection
documents.listen(connection);

// Listen on the connection
connection.listen();
