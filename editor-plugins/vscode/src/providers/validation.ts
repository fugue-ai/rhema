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

import * as vscode from 'vscode';
import * as yaml from 'yaml';
import { RhemaLogger } from '../logger';
import { RhemaSettings } from '../settings';
import { RhemaErrorHandler } from '../errorHandler';

export class RhemaValidation implements vscode.DiagnosticCollection {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;
  private diagnosticCollection: vscode.DiagnosticCollection;
  private disposables: vscode.Disposable[] = [];

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
    this.diagnosticCollection = vscode.languages.createDiagnosticCollection('rhema');
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Rhema validation...');

      // Register document change listener
      const changeDocumentListener = vscode.workspace.onDidChangeTextDocument(
        this.onDidChangeTextDocument.bind(this)
      );

      // Register save document listener
      const saveDocumentListener = vscode.workspace.onDidSaveTextDocument(
        this.onDidSaveTextDocument.bind(this)
      );

      // Register close document listener
      const closeDocumentListener = vscode.workspace.onDidCloseTextDocument(
        this.onDidCloseTextDocument.bind(this)
      );

      // Register open document listener
      const openDocumentListener = vscode.workspace.onDidOpenTextDocument(
        this.onDidOpenTextDocument.bind(this)
      );

      // Add disposables
      this.disposables.push(
        changeDocumentListener,
        saveDocumentListener,
        closeDocumentListener,
        openDocumentListener
      );

      // Validate all open documents
      await this.validateAllOpenDocuments();

      this.logger.info('Rhema validation initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema validation', error);
    }
  }

  private async onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent): Promise<void> {
    if (this.isRhemaFile(event.document)) {
      await this.validateDocument(event.document);
    }
  }

  private async onDidSaveTextDocument(document: vscode.TextDocument): Promise<void> {
    if (this.isRhemaFile(document) && this.settings.isAutoValidateEnabled()) {
      await this.validateDocument(document);
    }
  }

  private async onDidCloseTextDocument(document: vscode.TextDocument): Promise<void> {
    if (this.isRhemaFile(document)) {
      this.diagnosticCollection.delete(document.uri);
    }
  }

  private async onDidOpenTextDocument(document: vscode.TextDocument): Promise<void> {
    if (this.isRhemaFile(document)) {
      await this.validateDocument(document);
    }
  }

  private isRhemaFile(document: vscode.TextDocument): boolean {
    return (
      document.languageId === 'yaml' ||
      document.languageId === 'rhema-yaml' ||
      document.fileName.endsWith('.rhema.yaml') ||
      document.fileName.endsWith('.rhema.yml')
    );
  }

  private async validateAllOpenDocuments(): Promise<void> {
    const documents = vscode.workspace.textDocuments;
    for (const document of documents) {
      if (this.isRhemaFile(document)) {
        await this.validateDocument(document);
      }
    }
  }

  private async validateDocument(document: vscode.TextDocument): Promise<void> {
    try {
      const diagnostics: vscode.Diagnostic[] = [];

      // Parse YAML
      const text = document.getText();
      let parsed: any;

      try {
        parsed = yaml.parse(text);
      } catch (parseError) {
        const error = parseError as yaml.YAMLParseError;
        const range = new vscode.Range(
          new vscode.Position(error.linePos?.[0]?.line || 0, error.linePos?.[0]?.col || 0),
          new vscode.Position(error.linePos?.[0]?.line || 0, error.linePos?.[0]?.col || 0)
        );

        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `YAML parsing error: ${error.message}`,
            vscode.DiagnosticSeverity.Error
          )
        );

        this.diagnosticCollection.set(document.uri, diagnostics);
        return;
      }

      // Validate Rhema schema
      const schemaDiagnostics = await this.validateRhemaSchema(parsed, document);
      diagnostics.push(...schemaDiagnostics);

      // Validate Rhema-specific rules
      const ruleDiagnostics = await this.validateRhemaRules(parsed, document);
      diagnostics.push(...ruleDiagnostics);

      this.diagnosticCollection.set(document.uri, diagnostics);
    } catch (error) {
      this.errorHandler.handleError('Error validating document', error);
    }
  }

  private async validateRhemaSchema(
    parsed: any,
    document: vscode.TextDocument
  ): Promise<vscode.Diagnostic[]> {
    const diagnostics: vscode.Diagnostic[] = [];

    // Check if root is an object
    if (typeof parsed !== 'object' || parsed === null) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          'Rhema file must contain a YAML object at the root level',
          vscode.DiagnosticSeverity.Error
        )
      );
      return diagnostics;
    }

    // Validate scope structure
    if (parsed.scope) {
      const scopeDiagnostics = this.validateScope(parsed.scope, document);
      diagnostics.push(...scopeDiagnostics);
    }

    // Validate context structure
    if (parsed.context) {
      const contextDiagnostics = await this.validateContext(parsed.context, document);
      diagnostics.push(...contextDiagnostics);
    }

    // Validate todos structure
    if (parsed.todos) {
      const todosDiagnostics = this.validateTodos(parsed.todos, document);
      diagnostics.push(...todosDiagnostics);
    }

    // Validate insights structure
    if (parsed.insights) {
      const insightsDiagnostics = this.validateInsights(parsed.insights, document);
      diagnostics.push(...insightsDiagnostics);
    }

    // Validate patterns structure
    if (parsed.patterns) {
      const patternsDiagnostics = this.validatePatterns(parsed.patterns, document);
      diagnostics.push(...patternsDiagnostics);
    }

    // Validate decisions structure
    if (parsed.decisions) {
      const decisionsDiagnostics = this.validateDecisions(parsed.decisions, document);
      diagnostics.push(...decisionsDiagnostics);
    }

    return diagnostics;
  }

  private validateScope(scope: any, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    if (typeof scope !== 'object' || scope === null) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Scope must be an object', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate required fields
    if (!scope.name || typeof scope.name !== 'string') {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          'Scope must have a name field of type string',
          vscode.DiagnosticSeverity.Error
        )
      );
    }

    if (!scope.description || typeof scope.description !== 'string') {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          'Scope must have a description field of type string',
          vscode.DiagnosticSeverity.Error
        )
      );
    }

    return diagnostics;
  }

  private async validateContext(
    context: any,
    document: vscode.TextDocument
  ): Promise<vscode.Diagnostic[]> {
    const diagnostics: vscode.Diagnostic[] = [];

    if (typeof context !== 'object' || context === null) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Context must be an object', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate files array
    if (context.files && !Array.isArray(context.files)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          'Context files must be an array',
          vscode.DiagnosticSeverity.Error
        )
      );
    }

    // Validate patterns array
    if (context.patterns && !Array.isArray(context.patterns)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(
          range,
          'Context patterns must be an array',
          vscode.DiagnosticSeverity.Error
        )
      );
    }

    return diagnostics;
  }

  private validateTodos(todos: any, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    if (!Array.isArray(todos)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Todos must be an array', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate each todo item
    todos.forEach((todo, index) => {
      if (typeof todo !== 'object' || todo === null) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Todo at index ${index} must be an object`,
            vscode.DiagnosticSeverity.Error
          )
        );
        return;
      }

      if (!todo.id || typeof todo.id !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Todo at index ${index} must have an id field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }

      if (!todo.title || typeof todo.title !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Todo at index ${index} must have a title field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    });

    return diagnostics;
  }

  private validateInsights(insights: any, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    if (!Array.isArray(insights)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Insights must be an array', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate each insight item
    insights.forEach((insight, index) => {
      if (typeof insight !== 'object' || insight === null) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Insight at index ${index} must be an object`,
            vscode.DiagnosticSeverity.Error
          )
        );
        return;
      }

      if (!insight.id || typeof insight.id !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Insight at index ${index} must have an id field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }

      if (!insight.title || typeof insight.title !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Insight at index ${index} must have a title field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    });

    return diagnostics;
  }

  private validatePatterns(patterns: any, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    if (!Array.isArray(patterns)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Patterns must be an array', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate each pattern item
    patterns.forEach((pattern, index) => {
      if (typeof pattern !== 'object' || pattern === null) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Pattern at index ${index} must be an object`,
            vscode.DiagnosticSeverity.Error
          )
        );
        return;
      }

      if (!pattern.id || typeof pattern.id !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Pattern at index ${index} must have an id field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }

      if (!pattern.name || typeof pattern.name !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Pattern at index ${index} must have a name field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    });

    return diagnostics;
  }

  private validateDecisions(decisions: any, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];

    if (!Array.isArray(decisions)) {
      const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
      diagnostics.push(
        new vscode.Diagnostic(range, 'Decisions must be an array', vscode.DiagnosticSeverity.Error)
      );
      return diagnostics;
    }

    // Validate each decision item
    decisions.forEach((decision, index) => {
      if (typeof decision !== 'object' || decision === null) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Decision at index ${index} must be an object`,
            vscode.DiagnosticSeverity.Error
          )
        );
        return;
      }

      if (!decision.id || typeof decision.id !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Decision at index ${index} must have an id field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }

      if (!decision.title || typeof decision.title !== 'string') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Decision at index ${index} must have a title field of type string`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    });

    return diagnostics;
  }

  private async validateRhemaRules(
    parsed: any,
    document: vscode.TextDocument
  ): Promise<vscode.Diagnostic[]> {
    const diagnostics: vscode.Diagnostic[] = [];

    // Validate required sections
    const requiredSections = ['scope', 'context'];
    for (const section of requiredSections) {
      if (!parsed[section]) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            `Missing required section: ${section}`,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    }

    // Validate scope name uniqueness
    if (parsed.scope && parsed.scope.name) {
      const scopeName = parsed.scope.name;
      if (typeof scopeName !== 'string' || scopeName.trim() === '') {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            'Scope name must be a non-empty string',
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    }

    // Validate context files exist
    if (parsed.context && parsed.context.files) {
      if (!Array.isArray(parsed.context.files)) {
        const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
        diagnostics.push(
          new vscode.Diagnostic(
            range,
            'Context files must be an array',
            vscode.DiagnosticSeverity.Error
          )
        );
      } else {
        // Check if referenced files exist in workspace
        for (const file of parsed.context.files) {
          if (typeof file === 'string') {
            const fileExists = await this.checkFileExists(file, document);
            if (!fileExists) {
              const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
              diagnostics.push(
                new vscode.Diagnostic(
                  range,
                  `Referenced file does not exist: ${file}`,
                  vscode.DiagnosticSeverity.Warning
                )
              );
            }
          }
        }
      }
    }

    // Validate todo items have required fields
    if (parsed.todos && Array.isArray(parsed.todos)) {
      for (let i = 0; i < parsed.todos.length; i++) {
        const todo = parsed.todos[i];
        if (!todo.title || typeof todo.title !== 'string') {
          const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
          diagnostics.push(
            new vscode.Diagnostic(
              range,
              `Todo item ${i + 1} must have a title`,
              vscode.DiagnosticSeverity.Error
            )
          );
        }
      }
    }

    // Validate insight items have required fields
    if (parsed.insights && Array.isArray(parsed.insights)) {
      for (let i = 0; i < parsed.insights.length; i++) {
        const insight = parsed.insights[i];
        if (!insight.title || typeof insight.title !== 'string') {
          const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
          diagnostics.push(
            new vscode.Diagnostic(
              range,
              `Insight item ${i + 1} must have a title`,
              vscode.DiagnosticSeverity.Error
            )
          );
        }
      }
    }

    // Validate pattern items have required fields
    if (parsed.patterns && Array.isArray(parsed.patterns)) {
      for (let i = 0; i < parsed.patterns.length; i++) {
        const pattern = parsed.patterns[i];
        if (!pattern.name || typeof pattern.name !== 'string') {
          const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
          diagnostics.push(
            new vscode.Diagnostic(
              range,
              `Pattern item ${i + 1} must have a name`,
              vscode.DiagnosticSeverity.Error
            )
          );
        }
      }
    }

    // Validate decision items have required fields
    if (parsed.decisions && Array.isArray(parsed.decisions)) {
      for (let i = 0; i < parsed.decisions.length; i++) {
        const decision = parsed.decisions[i];
        if (!decision.title || typeof decision.title !== 'string') {
          const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
          diagnostics.push(
            new vscode.Diagnostic(
              range,
              `Decision item ${i + 1} must have a title`,
              vscode.DiagnosticSeverity.Error
            )
          );
        }
      }
    }

    // Validate relationships between sections
    this.validateRelationships(parsed, document, diagnostics);

    return diagnostics;
  }

  private async checkFileExists(filePath: string, document: vscode.TextDocument): Promise<boolean> {
    try {
      // Resolve relative paths from the document location
      const documentDir = vscode.Uri.file(document.fileName)
        .fsPath.split('/')
        .slice(0, -1)
        .join('/');
      const fullPath = vscode.Uri.file(`${documentDir}/${filePath}`);

      // Check if file exists in workspace
      if (!vscode.workspace.workspaceFolders || vscode.workspace.workspaceFolders.length === 0) {
        return false;
      }
      const workspaceFiles = await vscode.workspace.findFiles(
        new vscode.RelativePattern(vscode.workspace.workspaceFolders[0], `**/${filePath}`)
      );

      return workspaceFiles.length > 0;
    } catch (error) {
      this.logger.warn(`Error checking file existence: ${filePath}`, error);
      return false;
    }
  }

  private validateRelationships(
    parsed: any,
    document: vscode.TextDocument,
    diagnostics: vscode.Diagnostic[]
  ): void {
    // Validate that referenced items exist
    if (parsed.todos && Array.isArray(parsed.todos)) {
      for (const todo of parsed.todos) {
        if (todo.related && Array.isArray(todo.related)) {
          for (const related of todo.related) {
            if (!this.validateRelatedItem(related, parsed)) {
              const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
              diagnostics.push(
                new vscode.Diagnostic(
                  range,
                  `Related item not found: ${related}`,
                  vscode.DiagnosticSeverity.Warning
                )
              );
            }
          }
        }
      }
    }

    // Validate that insights reference valid items
    if (parsed.insights && Array.isArray(parsed.insights)) {
      for (const insight of parsed.insights) {
        if (insight.related && Array.isArray(insight.related)) {
          for (const related of insight.related) {
            if (!this.validateRelatedItem(related, parsed)) {
              const range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
              diagnostics.push(
                new vscode.Diagnostic(
                  range,
                  `Related item not found: ${related}`,
                  vscode.DiagnosticSeverity.Warning
                )
              );
            }
          }
        }
      }
    }
  }

  private validateRelatedItem(related: string, parsed: any): boolean {
    // Check if the related item exists in any section
    const sections = ['todos', 'insights', 'patterns', 'decisions'];

    for (const section of sections) {
      if (parsed[section] && Array.isArray(parsed[section])) {
        for (const item of parsed[section]) {
          if (item.id === related || item.name === related || item.title === related) {
            return true;
          }
        }
      }
    }

    return false;
  }

  // DiagnosticCollection interface implementation
  get(uri: vscode.Uri): readonly vscode.Diagnostic[] | undefined {
    return this.diagnosticCollection.get(uri);
  }

  set(uri: vscode.Uri, diagnostics: readonly vscode.Diagnostic[] | undefined): void {
    this.diagnosticCollection.set(uri, diagnostics);
  }

  delete(uri: vscode.Uri): void {
    this.diagnosticCollection.delete(uri);
  }

  clear(): void {
    this.diagnosticCollection.clear();
  }

  forEach(
    callback: (
      uri: vscode.Uri,
      diagnostics: readonly vscode.Diagnostic[],
      collection: vscode.DiagnosticCollection
    ) => any,
    thisArg?: any
  ): void {
    this.diagnosticCollection.forEach(callback, thisArg);
  }

  get has(): (uri: vscode.Uri) => boolean {
    return (uri: vscode.Uri) => this.diagnosticCollection.has(uri);
  }

  // Public method to validate a document
  async validateDocumentPublic(document: vscode.TextDocument): Promise<void> {
    await this.validateDocument(document);
  }

  dispose(): void {
    this.diagnosticCollection.dispose();
    this.disposables.forEach((disposable) => disposable.dispose());
    this.disposables = [];
  }
}
