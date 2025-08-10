// Mock VS Code API for testing
const vscode = {
  Uri: {
    file: (path: string) => ({ fsPath: path, scheme: 'file' }),
    parse: (uri: string) => ({ fsPath: uri, scheme: 'file' }),
  },
  Range: class {
    constructor(public start: any, public end: any) {}
  },
  Position: class {
    constructor(public line: number, public character: number) {}
  },
  Diagnostic: class {
    constructor(public range: any, public message: string, public severity?: any) {}
  },
  DiagnosticSeverity: {
    Error: 0,
    Warning: 1,
    Information: 2,
    Hint: 3,
  },
  CompletionItem: class {
    constructor(public label: string, public kind?: any) {}
  },
  CompletionItemKind: {
    Text: 1,
    Method: 2,
    Function: 3,
    Constructor: 4,
    Field: 5,
    Variable: 6,
    Class: 7,
    Interface: 8,
    Module: 9,
    Property: 10,
    Unit: 11,
    Value: 12,
    Enum: 13,
    Keyword: 14,
    Snippet: 15,
    Color: 16,
    File: 17,
    Reference: 18,
    Folder: 19,
    EnumMember: 20,
    Constant: 21,
    Struct: 22,
    Event: 23,
    Operator: 24,
    TypeParameter: 25,
  },
  MarkdownString: class {
    constructor(public value: string) {}
  },
  Hover: class {
    constructor(public contents: any, public range?: any) {}
  },
  SelectionRange: class {
    constructor(public range: any, public parent?: any) {}
  },
  DocumentHighlight: class {
    constructor(public range: any, public kind?: any) {}
  },
  DocumentLink: class {
    constructor(public range: any) {}
  },
  WorkspaceEdit: class {
    constructor() {}
  },
  TextEdit: class {
    constructor(public range: any, public newText: string) {}
  },
  FoldingRange: class {
    constructor(public start: number, public end: number, public kind?: any) {}
  },
  SymbolInformation: class {
    constructor(public name: string, public kind: any, public location: any) {}
  },
  DocumentSymbol: class {
    constructor(public name: string, public detail: string, public kind: any, public range: any) {}
  },
  CodeAction: class {
    constructor(public title: string, public kind?: any) {}
  },
  Command: class {
    constructor(public title: string, public command: string) {}
  },
  StatusBarItem: class {
    show() {}
    hide() {}
    dispose() {}
  },
  OutputChannel: class {
    appendLine() {}
    show() {}
    dispose() {}
  },
  ExtensionContext: class {
    subscriptions: any[] = [];
    workspaceState: any = {};
    globalState: any = {};
    extensionPath: string = '';
    storagePath: string = '';
    globalStoragePath: string = '';
    logPath: string = '';
    extensionUri: any = { fsPath: '' };
    environmentVariableCollection: any = {};
    extensionMode: any = 1;
    extension: any = { id: 'test', extensionPath: '' };
  },
  workspace: {
    getConfiguration: () => ({
      get: (key: string, defaultValue: any) => defaultValue,
      update: () => Promise.resolve(),
    }),
    onDidChangeTextDocument: () => ({ dispose: () => {} }),
    onDidSaveTextDocument: () => ({ dispose: () => {} }),
    onDidCloseTextDocument: () => ({ dispose: () => {} }),
    onDidOpenTextDocument: () => ({ dispose: () => {} }),
    onDidChangeWorkspaceFolders: () => ({ dispose: () => {} }),
    onDidChangeConfiguration: () => ({ dispose: () => {} }),
    onDidCreateFiles: () => ({ dispose: () => {} }),
    onDidDeleteFiles: () => ({ dispose: () => {} }),
    onDidRenameFiles: () => ({ dispose: () => {} }),
    workspaceFolders: [],
  },
  languages: {
    registerCompletionItemProvider: () => ({ dispose: () => {} }),
    registerHoverProvider: () => ({ dispose: () => {} }),
    registerDefinitionProvider: () => ({ dispose: () => {} }),
    registerReferenceProvider: () => ({ dispose: () => {} }),
    registerDocumentSymbolProvider: () => ({ dispose: () => {} }),
    registerWorkspaceSymbolProvider: () => ({ dispose: () => {} }),
    registerCodeActionsProvider: () => ({ dispose: () => {} }),
    registerFoldingRangeProvider: () => ({ dispose: () => {} }),
    registerSelectionRangeProvider: () => ({ dispose: () => {} }),
    registerDocumentHighlightProvider: () => ({ dispose: () => {} }),
    registerDocumentLinkProvider: () => ({ dispose: () => {} }),
    registerRenameProvider: () => ({ dispose: () => {} }),
    registerOnTypeFormattingEditProvider: () => ({ dispose: () => {} }),
    setLanguageConfiguration: () => ({ dispose: () => {} }),
    createDiagnosticCollection: () => new vscode.DiagnosticCollection(),
  },
  window: {
    createStatusBarItem: () => new vscode.StatusBarItem(),
    createOutputChannel: () => new vscode.OutputChannel(),
    showInformationMessage: () => Promise.resolve(),
    showWarningMessage: () => Promise.resolve(),
    showErrorMessage: () => Promise.resolve(),
  },
  commands: {
    registerCommand: () => ({ dispose: () => {} }),
    executeCommand: () => Promise.resolve(),
  },
  DiagnosticCollection: class {
    set() {}
    delete() {}
    clear() {}
    dispose() {}
  },
};

// Mock the vscode module
(global as any).jest = { mock: () => {} };
(global as any).jest.mock('vscode', () => vscode, { virtual: true });

// Global test setup
beforeAll(() => {
  // Any global setup needed
});

afterAll(() => {
  // Any global cleanup needed
}); 