// Mock VS Code API for testing
export const Uri = {
  file: (path: string) => ({ scheme: 'file', path }),
  parse: (uri: string) => ({ scheme: 'file', path: uri }),
};

export const Range = class {
  constructor(public startLine: number, public startCharacter: number, public endLine: number, public endCharacter: number) {}
};

export const Position = class {
  constructor(public line: number, public character: number) {}
};

export const Diagnostic = class {
  constructor(public range: any, public message: string, public severity: any) {}
};

export const DiagnosticSeverity = {
  Error: 0,
  Warning: 1,
  Information: 2,
  Hint: 3,
};

export const CompletionItem = class {
  constructor(public label: string, public kind?: any) {}
};

export const CompletionItemKind = {
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
};

export const MarkdownString = class {
  constructor(public value: string) {}
};

export const Hover = class {
  constructor(public contents: any) {}
};

export const CodeAction = class {
  constructor(public title: string, public kind?: any) {}
};

export const CodeActionKind = {
  Empty: '',
  QuickFix: 'quickfix',
  Refactor: 'refactor',
  RefactorExtract: 'refactor.extract',
  RefactorInline: 'refactor.inline',
  RefactorMove: 'refactor.move',
  RefactorRewrite: 'refactor.rewrite',
  Source: 'source',
  SourceOrganizeImports: 'source.organizeImports',
  SourceFixAll: 'source.fixAll',
};

export const CodeActionTriggerKind = {
  Invoke: 1,
  Automatic: 2,
};

export const TextEdit = class {
  constructor(public range: any, public newText: string) {}
};

export const WorkspaceEdit = class {
  private replacements = new Map();
  
  constructor() {
    this.replacements = new Map();
  }
  
  replace(uri: any, range: any, newText: string) {
    if (!this.replacements.has(uri)) {
      this.replacements.set(uri, []);
    }
    this.replacements.get(uri).push({ range, newText });
  }
  
  has(uri: any) {
    return this.replacements.has(uri);
  }
};

export const TreeItem = class {
  constructor(public label: string, public collapsibleState?: any) {}
};

export const TreeItemCollapsibleState = {
  None: 0,
  Collapsed: 1,
  Expanded: 2,
};

export const EventEmitter = class {
  private listeners: any[] = [];
  
  constructor() {
    this.listeners = [];
  }
  
  fire(data: any) {
    this.listeners.forEach(listener => listener(data));
  }
  
  get event() {
    return {
      listener: (callback: any) => {
        this.listeners.push(callback);
        return { dispose: () => {} };
      }
    };
  }
};

export const Disposable = class {
  constructor(public dispose: () => void) {}
};

export const StatusBarAlignment = {
  Left: 1,
  Right: 2,
};

export const workspace = {
  openTextDocument: async (options: any) => ({
    uri: { scheme: 'file', path: '/test/file.rhema.yml' },
    fileName: '/test/file.rhema.yml',
    getText: () => options.content || 'test content',
    lineCount: 10,
    lineAt: (line: number) => ({ text: `line ${line}` }),
    languageId: options.language || 'yaml',
  }),
  getConfiguration: (section: string) => {
    return {
        get: (key: string) => {
    if (key === 'rhema.enabled') {
      return true;
    }
    if (key === 'rhema.serverUrl') {
      return 'http://localhost:3000';
    }
    return undefined;
  },
    };
  },
  onDidChangeTextDocument: new EventEmitter(),
  onDidSaveTextDocument: new EventEmitter(),
  onDidCloseTextDocument: new EventEmitter(),
  onDidOpenTextDocument: new EventEmitter(),
};

export const window = {
  activeTextEditor: {
    document: {
      fileName: '/test/file.rhema.yml',
      getText: () => 'test content',
      lineCount: 10,
      lineAt: (line: number) => ({ text: `line ${line}` }),
    },
  },
  createOutputChannel: (name: string) => ({
    appendLine: () => {},
    show: () => {},
    dispose: () => {},
  }),
  createStatusBarItem: (alignment: any, priority: number) => ({
    text: '',
    tooltip: '',
    command: '',
    show: () => {},
    hide: () => {},
    dispose: () => {},
  }),
  showErrorMessage: (message: string) => Promise.resolve(),
  showInformationMessage: (message: string) => Promise.resolve(),
  showWarningMessage: (message: string) => Promise.resolve(),
};

export const languages = {
  registerCompletionItemProvider: () => new Disposable(() => {}),
  registerHoverProvider: () => new Disposable(() => {}),
  registerDefinitionProvider: () => new Disposable(() => {}),
  registerReferenceProvider: () => new Disposable(() => {}),
  registerDocumentSymbolProvider: () => new Disposable(() => {}),
  registerCodeActionsProvider: () => new Disposable(() => {}),
  registerFoldingRangeProvider: () => new Disposable(() => {}),
  registerSelectionRangeProvider: () => new Disposable(() => {}),
  registerDocumentHighlightProvider: () => new Disposable(() => {}),
  registerDocumentLinkProvider: () => new Disposable(() => {}),
  registerRenameProvider: () => new Disposable(() => {}),
  registerDocumentFormattingEditProvider: () => new Disposable(() => {}),
  registerDocumentRangeFormattingEditProvider: () => new Disposable(() => {}),
  createDiagnosticCollection: () => ({
    set: () => {},
    delete: () => {},
    clear: () => {},
    dispose: () => {},
  }),
};

export const commands = {
  registerCommand: () => new Disposable(() => {}),
};

export const extensions = {
  getExtension: () => ({
    exports: {},
  }),
};

export const TextDocument = class {
  constructor(public uri: any, public fileName: string, public languageId: string) {}
  
  getText() {
    return 'test content';
  }
  
  lineCount = 10;
  
  lineAt(line: number) {
    return { text: `line ${line}` };
  }
  
  getWordRangeAtPosition(position: any) {
    return new Range(0, 0, 0, 5);
  }
};

export const CancellationToken = class {
  isCancellationRequested = false;
};

export const ProviderResult = {
  resolve: (value: any) => value,
};

export default {
  Uri,
  Range,
  Position,
  Diagnostic,
  DiagnosticSeverity,
  CompletionItem,
  CompletionItemKind,
  MarkdownString,
  Hover,
  CodeAction,
  CodeActionKind,
  CodeActionTriggerKind,
  TextEdit,
  WorkspaceEdit,
  TreeItem,
  TreeItemCollapsibleState,
  EventEmitter,
  Disposable,
  window,
  workspace,
  languages,
  commands,
  extensions,
  TextDocument,
  CancellationToken,
  ProviderResult,
}; 