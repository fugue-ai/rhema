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
    InitializeParams,
    DidChangeConfigurationNotification,
    CompletionItem,
    CompletionItemKind,
    TextDocumentPositionParams,
    TextDocumentSyncKind,
    InitializeResult,
    Diagnostic,
    DiagnosticSeverity,
    Range,
    Position,
    Hover,
    MarkedString,
    Definition,
    Location,
    ReferenceParams,
    SymbolInformation,
    SymbolKind,
    DocumentSymbol,
    CodeAction,
    CodeActionKind,
    WorkspaceSymbolParams,
    RenameParams,
    WorkspaceEdit,
    TextEdit,
    DocumentFormattingParams,
    DocumentRangeFormattingParams,
    FoldingRange,
    FoldingRangeKind,
    SemanticTokens,
    SemanticTokensLegend,
    DocumentSemanticTokensParams,
    DocumentSemanticTokensRequest,
    DocumentSemanticTokensDeltaRequest,
    DocumentSemanticTokensDeltaParams,
    SemanticTokensDelta,
    SemanticTokensEdit,
    InlayHint,
    InlayHintKind,
    InlayHintLabel,
    DocumentInlayHintParams,
    DocumentInlayHintRequest,
    CallHierarchyItem,
    CallHierarchyIncomingCallsParams,
    CallHierarchyOutgoingCallsParams,
    CallHierarchyIncomingCall,
    CallHierarchyOutgoingCall,
    TypeHierarchyItem,
    TypeHierarchyPrepareParams,
    TypeHierarchySupertypesParams,
    TypeHierarchySubtypesParams,
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
    ExecuteCommandParams,
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
    WorkspaceSymbol,
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
    TextDocument,
    URI
} from 'vscode-languageserver/node';

import {
    TextDocument as VSCodeTextDocument
} from 'vscode-languageserver-textdocument';

import * as path from 'path';
import * as fs from 'fs-extra';
import * as yaml from 'yaml';
import * as Ajv from 'ajv';
import * as _ from 'lodash';
import { v4 as uuidv4 } from 'uuid';
import * as glob from 'glob';

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
import { RhemaInlayHintProvider } from './inlayHint';
import { RhemaCallHierarchyProvider } from './callHierarchy';
import { RhemaTypeHierarchyProvider } from './typeHierarchy';
import { RhemaDocumentLinkProvider } from './documentLink';
import { RhemaColorProvider } from './color';
import { RhemaCodeLensProvider } from './codeLens';
import { RhemaDocumentHighlightProvider } from './documentHighlight';
import { RhemaSignatureHelpProvider } from './signatureHelp';
import { RhemaWorkspaceSymbolProvider } from './workspaceSymbol';
import { RhemaConfigurationManager } from './configuration';
import { RhemaFileWatcher } from './fileWatcher';
import { RhemaLogger } from './logger';
import { RhemaErrorHandler } from './errorHandler';
import { RhemaPerformanceMonitor } from './performanceMonitor';
import { RhemaCache } from './cache';
import { RhemaSchemaManager } from './schemaManager';
import { RhemaContextManager } from './contextManager';

// Create a connection for the server
const connection = createConnection(ProposedFeatures.all);

// Create a text document manager
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

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
const inlayHintProvider = new RhemaInlayHintProvider();
const callHierarchyProvider = new RhemaCallHierarchyProvider();
const typeHierarchyProvider = new RhemaTypeHierarchyProvider();
const documentLinkProvider = new RhemaDocumentLinkProvider();
const colorProvider = new RhemaColorProvider();
const codeLensProvider = new RhemaCodeLensProvider();
const documentHighlightProvider = new RhemaDocumentHighlightProvider();
const signatureHelpProvider = new RhemaSignatureHelpProvider();
const workspaceSymbolProvider = new RhemaWorkspaceSymbolProvider();
const configurationManager = new RhemaConfigurationManager();
const fileWatcher = new RhemaFileWatcher();
const logger = new RhemaLogger();
const errorHandler = new RhemaErrorHandler(logger);
const performanceMonitor = new RhemaPerformanceMonitor();
const cache = new RhemaCache();
const schemaManager = new RhemaSchemaManager();
const contextManager = new RhemaContextManager();

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
connection.onInitialize((params: InitializeParams): InitializeResult => {
    const capabilities = params.capabilities;

    // Check capabilities
    hasConfigurationCapability = !!(capabilities.workspace && !!capabilities.workspace.configuration);
    hasWorkspaceFolderCapability = !!(capabilities.workspace && !!capabilities.workspace.workspaceFolders);
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
        capabilities.textDocument &&
        capabilities.textDocument.semanticTokens
    );
    hasInlayHintCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.inlayHint
    );
    hasCallHierarchyCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.callHierarchy
    );
    hasTypeHierarchyCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.typeHierarchy
    );
    hasDocumentLinkCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.documentLink
    );
    hasColorProviderCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.colorProvider
    );
    hasCodeLensCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.codeLens
    );
    hasDocumentHighlightCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.documentHighlight
    );
    hasSignatureHelpCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.signatureHelp
    );
    hasWorkspaceSymbolCapability = !!(
        capabilities.workspace &&
        capabilities.workspace.symbol
    );

    // Initialize components
    configurationManager.initialize(connection, hasConfigurationCapability);
    fileWatcher.initialize(connection, hasWorkspaceFolderCapability);
    performanceMonitor.initialize();
    cache.initialize();
    schemaManager.initialize();
    contextManager.initialize();

    // Initialize providers with capabilities
    parser.initialize(capabilities);
    validator.initialize(capabilities, hasDiagnosticRelatedInformationCapability);
    completer.initialize(capabilities);
    hoverProvider.initialize(capabilities);
    definitionProvider.initialize(capabilities);
    referenceProvider.initialize(capabilities);
    symbolProvider.initialize(capabilities);
    codeActionProvider.initialize(capabilities, hasCodeActionLiteralSupport);
    formatter.initialize(capabilities);
    semanticTokensProvider.initialize(capabilities, hasSemanticTokensCapability);
    inlayHintProvider.initialize(capabilities, hasInlayHintCapability);
    callHierarchyProvider.initialize(capabilities, hasCallHierarchyCapability);
    typeHierarchyProvider.initialize(capabilities, hasTypeHierarchyCapability);
    documentLinkProvider.initialize(capabilities, hasDocumentLinkCapability);
    colorProvider.initialize(capabilities, hasColorProviderCapability);
    codeLensProvider.initialize(capabilities, hasCodeLensCapability);
    documentHighlightProvider.initialize(capabilities, hasDocumentHighlightCapability);
    signatureHelpProvider.initialize(capabilities, hasSignatureHelpCapability);
    workspaceSymbolProvider.initialize(capabilities, hasWorkspaceSymbolCapability);

    logger.info('Rhema Language Server initialized');

    return {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Incremental,
            // Tell the client that this server supports code completion
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['.', ':', '-', ' ', '\t', '\n']
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
            codeActionProvider: hasCodeActionLiteralSupport ? {
                codeActionKinds: [
                    CodeActionKind.QuickFix,
                    CodeActionKind.Refactor,
                    CodeActionKind.RefactorExtract,
                    CodeActionKind.RefactorInline,
                    CodeActionKind.RefactorRewrite,
                    CodeActionKind.Source,
                    CodeActionKind.SourceOrganizeImports
                ]
            } : true,
            // Tell the client that this server supports document formatting
            documentFormattingProvider: true,
            // Tell the client that this server supports range formatting
            documentRangeFormattingProvider: true,
            // Tell the client that this server supports folding
            foldingRangeProvider: true,
            // Tell the client that this server supports semantic tokens
            semanticTokensProvider: hasSemanticTokensCapability ? {
                legend: semanticTokensProvider.getLegend(),
                range: true,
                full: {
                    delta: true
                }
            } : undefined,
            // Tell the client that this server supports inlay hints
            inlayHintProvider: hasInlayHintCapability,
            // Tell the client that this server supports call hierarchy
            callHierarchyProvider: hasCallHierarchyCapability,
            // Tell the client that this server supports type hierarchy
            typeHierarchyProvider: hasTypeHierarchyCapability,
            // Tell the client that this server supports document links
            documentLinkProvider: hasDocumentLinkCapability ? {
                resolveProvider: true
            } : undefined,
            // Tell the client that this server supports color provider
            colorProvider: hasColorProviderCapability,
            // Tell the client that this server supports code lens
            codeLensProvider: hasCodeLensCapability ? {
                resolveProvider: true
            } : undefined,
            // Tell the client that this server supports document highlight
            documentHighlightProvider: hasDocumentHighlightCapability,
            // Tell the client that this server supports signature help
            signatureHelpProvider: hasSignatureHelpCapability ? {
                triggerCharacters: ['(', ',', ' ']
            } : undefined,
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
                    'rhema.alert'
                ]
            }
        }
    };
});

// Handle initialization
connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        // Register for all configuration changes
        connection.client.register(DidChangeConfigurationNotification.type, undefined);
    }
    if (hasWorkspaceFolderCapability) {
        connection.workspace.onDidChangeWorkspaceFolders(_event => {
            connection.console.log('Workspace folder change event received.');
        });
    }
    logger.info('Rhema Language Server initialized successfully');
});

// Handle configuration changes
connection.onDidChangeConfiguration(change => {
    if (hasConfigurationCapability) {
        configurationManager.updateConfiguration(change.settings);
    }
    // Revalidate all open text documents
    documents.all().forEach(validateTextDocument);
});

// Handle workspace folder changes
connection.workspace.onDidChangeWorkspaceFolders(_event => {
    connection.console.log('Workspace folders changed');
    // Revalidate all documents
    documents.all().forEach(validateTextDocument);
});

// Handle file watching
connection.onDidChangeWatchedFiles(_change => {
    // Monitored files have changed in VS Code
    connection.console.log('We received a file change event');
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
                lastModified: Date.now()
            });
        } else {
            // Add parsing errors
            diagnostics.push(...parseResult.errors.map(error => ({
                range: error.range,
                message: error.message,
                severity: DiagnosticSeverity.Error,
                source: 'rhema-parser'
            })));
        }

        // Send the computed diagnostics to VS Code
        connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
        
        const endTime = performance.now();
        performanceMonitor.recordOperation('validateTextDocument', endTime - startTime);
        
    } catch (error) {
        errorHandler.handleError('Error validating text document', error);
    }
}

// Check if a file is a Rhema file
function isRhemaFile(uri: string): boolean {
    const fileName = path.basename(uri).toLowerCase();
    return fileName.includes('.rhema.') || 
           fileName.includes('scope.yaml') || 
           fileName.includes('knowledge.yaml') ||
           fileName.includes('todos.yaml') ||
           fileName.includes('decisions.yaml') ||
           fileName.includes('patterns.yaml') ||
           fileName.includes('conventions.yaml');
}

// Completion provider
connection.onCompletion(
    (textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
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
            errorHandler.handleError('Error providing completion', error);
            return [];
        }
    }
);

// Completion resolve provider
connection.onCompletionResolve(
    (item: CompletionItem): CompletionItem => {
        const startTime = performance.now();
        
        try {
            const result = completer.resolveCompletion(item);
            
            const endTime = performance.now();
            performanceMonitor.recordOperation('completionResolve', endTime - startTime);
            
            return result;
        } catch (error) {
            errorHandler.handleError('Error resolving completion', error);
            return item;
        }
    }
);

// Hover provider
connection.onHover(
    (textDocumentPosition: TextDocumentPositionParams): Hover | null => {
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
            errorHandler.handleError('Error providing hover', error);
            return null;
        }
    }
);

// Definition provider
connection.onDefinition(
    (textDocumentPosition: TextDocumentPositionParams): Definition | null => {
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
            errorHandler.handleError('Error providing definition', error);
            return null;
        }
    }
);

// References provider
connection.onReferences(
    (referenceParams: ReferenceParams): Location[] => {
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
            errorHandler.handleError('Error providing references', error);
            return [];
        }
    }
);

// Document symbols provider
connection.onDocumentSymbol(
    (documentSymbolParams): DocumentSymbol[] => {
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
            errorHandler.handleError('Error providing document symbols', error);
            return [];
        }
    }
);

// Workspace symbols provider
connection.onWorkspaceSymbol(
    (workspaceSymbolParams: WorkspaceSymbolParams): WorkspaceSymbol[] => {
        const startTime = performance.now();
        
        try {
            const result = workspaceSymbolProvider.provideWorkspaceSymbols(
                workspaceSymbolParams.query,
                documents.all()
            );
            
            const endTime = performance.now();
            performanceMonitor.recordOperation('workspaceSymbols', endTime - startTime);
            
            return result;
        } catch (error) {
            errorHandler.handleError('Error providing workspace symbols', error);
            return [];
        }
    }
);

// Code actions provider
connection.onCodeAction(
    (codeActionParams): CodeAction[] => {
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
            errorHandler.handleError('Error providing code actions', error);
            return [];
        }
    }
);

// Document formatting provider
connection.onDocumentFormatting(
    (documentFormattingParams: DocumentFormattingParams): TextEdit[] => {
        const startTime = performance.now();
        
        try {
            const document = documents.get(documentFormattingParams.textDocument.uri);
            if (!document) {
                return [];
            }

            const result = formatter.formatDocument(
                document,
                documentFormattingParams.options
            );
            
            const endTime = performance.now();
            performanceMonitor.recordOperation('documentFormatting', endTime - startTime);
            
            return result;
        } catch (error) {
            errorHandler.handleError('Error formatting document', error);
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
            errorHandler.handleError('Error formatting document range', error);
            return [];
        }
    }
);

// Folding range provider
connection.onFoldingRanges(
    (foldingRangeParams): FoldingRange[] => {
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
            errorHandler.handleError('Error providing folding ranges', error);
            return [];
        }
    }
);

// Semantic tokens provider
if (hasSemanticTokensCapability) {
    connection.onRequest(
        DocumentSemanticTokensRequest.type,
        (documentSemanticTokensParams: DocumentSemanticTokensParams): SemanticTokens => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentSemanticTokensParams.textDocument.uri);
                if (!document) {
                    return { data: [] };
                }

                const result = semanticTokensProvider.provideSemanticTokens(
                    document,
                    cache.getDocument(documentSemanticTokensParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('semanticTokens', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing semantic tokens', error);
                return { data: [] };
            }
        }
    );

    connection.onRequest(
        DocumentSemanticTokensDeltaRequest.type,
        (documentSemanticTokensDeltaParams: DocumentSemanticTokensDeltaParams): SemanticTokensDelta => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentSemanticTokensDeltaParams.textDocument.uri);
                if (!document) {
                    return { edits: [] };
                }

                const result = semanticTokensProvider.provideSemanticTokensDelta(
                    document,
                    documentSemanticTokensDeltaParams.previousResultId,
                    cache.getDocument(documentSemanticTokensDeltaParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('semanticTokensDelta', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing semantic tokens delta', error);
                return { edits: [] };
            }
        }
    );
}

// Inlay hints provider
if (hasInlayHintCapability) {
    connection.onRequest(
        DocumentInlayHintRequest.type,
        (documentInlayHintParams: DocumentInlayHintParams): InlayHint[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentInlayHintParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = inlayHintProvider.provideInlayHints(
                    document,
                    documentInlayHintParams.range,
                    cache.getDocument(documentInlayHintParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('inlayHints', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing inlay hints', error);
                return [];
            }
        }
    );
}

// Call hierarchy provider
if (hasCallHierarchyCapability) {
    connection.onRequest(
        'textDocument/prepareCallHierarchy',
        (callHierarchyPrepareParams): CallHierarchyItem[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(callHierarchyPrepareParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = callHierarchyProvider.prepareCallHierarchy(
                    document,
                    callHierarchyPrepareParams.position,
                    cache.getDocument(callHierarchyPrepareParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('prepareCallHierarchy', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error preparing call hierarchy', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'callHierarchy/incomingCalls',
        (callHierarchyIncomingCallsParams: CallHierarchyIncomingCallsParams): CallHierarchyIncomingCall[] => {
            const startTime = performance.now();
            
            try {
                const result = callHierarchyProvider.provideIncomingCalls(
                    callHierarchyIncomingCallsParams.item
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('incomingCalls', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing incoming calls', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'callHierarchy/outgoingCalls',
        (callHierarchyOutgoingCallsParams: CallHierarchyOutgoingCallsParams): CallHierarchyOutgoingCall[] => {
            const startTime = performance.now();
            
            try {
                const result = callHierarchyProvider.provideOutgoingCalls(
                    callHierarchyOutgoingCallsParams.item
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('outgoingCalls', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing outgoing calls', error);
                return [];
            }
        }
    );
}

// Type hierarchy provider
if (hasTypeHierarchyCapability) {
    connection.onRequest(
        'textDocument/prepareTypeHierarchy',
        (typeHierarchyPrepareParams: TypeHierarchyPrepareParams): TypeHierarchyItem[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(typeHierarchyPrepareParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = typeHierarchyProvider.prepareTypeHierarchy(
                    document,
                    typeHierarchyPrepareParams.position,
                    cache.getDocument(typeHierarchyPrepareParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('prepareTypeHierarchy', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error preparing type hierarchy', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'typeHierarchy/supertypes',
        (typeHierarchySupertypesParams: TypeHierarchySupertypesParams): TypeHierarchyItem[] => {
            const startTime = performance.now();
            
            try {
                const result = typeHierarchyProvider.provideSupertypes(
                    typeHierarchySupertypesParams.item
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('supertypes', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing supertypes', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'typeHierarchy/subtypes',
        (typeHierarchySubtypesParams: TypeHierarchySubtypesParams): TypeHierarchyItem[] => {
            const startTime = performance.now();
            
            try {
                const result = typeHierarchyProvider.provideSubtypes(
                    typeHierarchySubtypesParams.item
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('subtypes', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing subtypes', error);
                return [];
            }
        }
    );
}

// Document links provider
if (hasDocumentLinkCapability) {
    connection.onRequest(
        'textDocument/documentLink',
        (documentLinkParams: DocumentLinkParams): DocumentLink[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentLinkParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = documentLinkProvider.provideDocumentLinks(
                    document,
                    cache.getDocument(documentLinkParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('documentLinks', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing document links', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'documentLink/resolve',
        (documentLink): DocumentLink => {
            const startTime = performance.now();
            
            try {
                const result = documentLinkProvider.resolveDocumentLink(documentLink);
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('resolveDocumentLink', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error resolving document link', error);
                return documentLink;
            }
        }
    );
}

// Color provider
if (hasColorProviderCapability) {
    connection.onRequest(
        'textDocument/documentColor',
        (documentColorParams: DocumentColorParams): ColorInformation[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentColorParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = colorProvider.provideDocumentColors(
                    document,
                    cache.getDocument(documentColorParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('documentColors', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing document colors', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'textDocument/colorPresentation',
        (colorPresentationParams: ColorPresentationParams): ColorPresentation[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(colorPresentationParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = colorProvider.provideColorPresentations(
                    document,
                    colorPresentationParams.color,
                    colorPresentationParams.range
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('colorPresentations', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing color presentations', error);
                return [];
            }
        }
    );
}

// Code lens provider
if (hasCodeLensCapability) {
    connection.onRequest(
        'textDocument/codeLens',
        (codeLensParams: CodeLensParams): CodeLens[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(codeLensParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = codeLensProvider.provideCodeLenses(
                    document,
                    cache.getDocument(codeLensParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('codeLenses', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing code lenses', error);
                return [];
            }
        }
    );

    connection.onRequest(
        'codeLens/resolve',
        (codeLens): CodeLens => {
            const startTime = performance.now();
            
            try {
                const result = codeLensProvider.resolveCodeLens(codeLens);
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('resolveCodeLens', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error resolving code lens', error);
                return codeLens;
            }
        }
    );
}

// Document highlight provider
if (hasDocumentHighlightCapability) {
    connection.onRequest(
        'textDocument/documentHighlight',
        (documentHighlightParams): DocumentHighlight[] => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(documentHighlightParams.textDocument.uri);
                if (!document) {
                    return [];
                }

                const result = documentHighlightProvider.provideDocumentHighlights(
                    document,
                    documentHighlightParams.position,
                    cache.getDocument(documentHighlightParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('documentHighlights', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing document highlights', error);
                return [];
            }
        }
    );
}

// Signature help provider
if (hasSignatureHelpCapability) {
    connection.onRequest(
        'textDocument/signatureHelp',
        (signatureHelpParams): SignatureHelp | null => {
            const startTime = performance.now();
            
            try {
                const document = documents.get(signatureHelpParams.textDocument.uri);
                if (!document) {
                    return null;
                }

                const result = signatureHelpProvider.provideSignatureHelp(
                    document,
                    signatureHelpParams.position,
                    signatureHelpParams.context,
                    cache.getDocument(signatureHelpParams.textDocument.uri)
                );
                
                const endTime = performance.now();
                performanceMonitor.recordOperation('signatureHelp', endTime - startTime);
                
                return result;
            } catch (error) {
                errorHandler.handleError('Error providing signature help', error);
                return null;
            }
        }
    );
}

// Rename provider
connection.onRequest(
    'textDocument/rename',
    (renameParams: RenameParams): WorkspaceEdit | null => {
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
            errorHandler.handleError('Error providing rename', error);
            return null;
        }
    }
);

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
            errorHandler.handleError('Error executing command', error);
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
        const edits = formatter.formatDocument(document, {});
        if (edits.length > 0) {
            connection.workspace.applyEdit({
                documentChanges: [{
                    textDocument: {
                        uri: params.uri,
                        version: document.version
                    },
                    edits: edits
                }]
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
connection.onRequest('rhema/context', () => {
    return contextManager.getContext();
});

// Shutdown
connection.onShutdown(() => {
    logger.info('Rhema Language Server shutting down');
    performanceMonitor.shutdown();
    cache.clear();
});

// Exit
connection.onExit(() => {
    logger.info('Rhema Language Server exited');
});

// Make the text document manager listen on the connection
documents.listen(connection);

// Listen on the connection
connection.listen(); 