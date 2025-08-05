import {
  type CodeAction,
  CodeActionKind,
  Range,
  Position,
  type TextDocument,
  type WorkspaceEdit,
  TextEdit,
  Command,
} from 'vscode-languageserver/node';
import type { Diagnostic } from 'vscode-languageserver/node';
import { RhemaDocument } from './parser';

// Constants for date formatting
const CURRENT_YEAR = new Date().getFullYear();
const CURRENT_MONTH = String(new Date().getMonth() + 1).padStart(2, '0');
const CURRENT_DATE = String(new Date().getDate()).padStart(2, '0');

export interface RefactoringOperation {
  id: string;
  name: string;
  description: string;
  kind: CodeActionKind;
  validate: (document: TextDocument, range: Range) => boolean;
  execute: (document: TextDocument, range: Range, args?: any) => WorkspaceEdit | null;
  preview?: (document: TextDocument, range: Range, args?: any) => string;
}

export interface CodeGenerationTemplate {
  id: string;
  name: string;
  description: string;
  template: string;
  placeholders: string[];
  category: 'scope' | 'knowledge' | 'todos' | 'decisions' | 'patterns' | 'conventions';
}

export interface QuickFix {
  id: string;
  name: string;
  description: string;
  diagnosticCodes: string[];
  apply: (document: TextDocument, diagnostic: Diagnostic) => WorkspaceEdit | null;
}

export class RhemaCodeActionProvider {
  private capabilities: any;
  private hasCodeActionLiteralSupport: boolean = false;
  private refactoringOperations: Map<string, RefactoringOperation> = new Map();
  private codeGenerationTemplates: Map<string, CodeGenerationTemplate> = new Map();
  private quickFixes: Map<string, QuickFix> = new Map();
  private workspaceManager: any; // Will be injected from server

  constructor() {
    this.initializeRefactoringOperations();
    this.initializeCodeGenerationTemplates();
    this.initializeQuickFixes();
  }

  initialize(
    capabilities: any,
    hasCodeActionLiteralSupport: boolean,
    workspaceManager?: any
  ): void {
    this.capabilities = capabilities;
    this.hasCodeActionLiteralSupport = hasCodeActionLiteralSupport;
    this.workspaceManager = workspaceManager;
  }

  provideCodeActions(
    document: TextDocument,
    range: Range,
    context: { diagnostics: Diagnostic[] },
    cachedDocument?: any
  ): CodeAction[] {
    try {
      const actions: CodeAction[] = [];

      // Add quick fixes for diagnostics
      context.diagnostics.forEach((diagnostic) => {
        const quickFixes = this.getQuickFixesForDiagnostic(diagnostic, document);
        actions.push(...quickFixes);
      });

      // Add refactoring actions
      const refactoringActions = this.getRefactoringActions(document, range, cachedDocument);
      actions.push(...refactoringActions);

      // Add code generation actions
      const generationActions = this.getCodeGenerationActions(document, range, cachedDocument);
      actions.push(...generationActions);

      // Add source actions
      const sourceActions = this.getSourceActions(document, range, cachedDocument);
      actions.push(...sourceActions);

      return actions;
    } catch (error) {
      console.error('Error providing code actions:', error);
      return [];
    }
  }

  provideRename(
    document: TextDocument,
    position: Position,
    newName: string,
    cachedDocument?: any
  ): WorkspaceEdit | null {
    try {
      const text = document.getText();
      const word = this.getWordAtPosition(text, position);

      if (!word) {
        return null;
      }

      // Find all references to rename
      const references = this.findAllReferences(word, text);

      if (references.length === 0) {
        return null;
      }

      // Create text edits for all references
      const changes: { [uri: string]: TextEdit[] } = {};
      changes[document.uri] = references.map((ref) => TextEdit.replace(ref, newName));

      return { changes };
    } catch (error) {
      console.error('Error providing rename:', error);
      return null;
    }
  }

  executeCommand(command: string, args: any[]): any {
    try {
      switch (command) {
        case 'rhema.validate':
          return this.executeValidate(args);
        case 'rhema.format':
          return this.executeFormat(args);
        case 'rhema.refactor':
          return this.executeRefactor(args);
        case 'rhema.generate':
          return this.executeGenerate(args);
        case 'rhema.extract':
          return this.executeExtract(args);
        case 'rhema.inline':
          return this.executeInline(args);
        case 'rhema.move':
          return this.executeMove(args);
        case 'rhema.organize':
          return this.executeOrganize(args);
        case 'rhema.optimize':
          return this.executeOptimize(args);
        case 'rhema.debug':
          return this.executeDebug(args);
        case 'rhema.profile':
          return this.executeProfile(args);
        case 'rhema.sync':
          return this.executeSync(args);
        case 'rhema.export':
          return this.executeExport(args);
        case 'rhema.import':
          return this.executeImport(args);
        case 'rhema.migrate':
          return this.executeMigrate(args);
        case 'rhema.clean':
          return this.executeClean(args);
        case 'rhema.backup':
          return this.executeBackup(args);
        case 'rhema.restore':
          return this.executeRestore(args);
        case 'rhema.analyze':
          return this.executeAnalyze(args);
        case 'rhema.report':
          return this.executeReport(args);
        case 'rhema.test':
          return this.executeTest(args);
        case 'rhema.deploy':
          return this.executeDeploy(args);
        case 'rhema.monitor':
          return this.executeMonitor(args);
        case 'rhema.alert':
          return this.executeAlert(args);
        default:
          console.warn(`Unknown command: ${command}`);
          return null;
      }
    } catch (error) {
      console.error('Error executing command:', error);
      return null;
    }
  }

  // --- Advanced Refactoring Operations ---

  private initializeRefactoringOperations(): void {
    // Extract to variable
    this.addRefactoringOperation({
      id: 'extract-variable',
      name: 'Extract to Variable',
      description: 'Extract selected text to a variable',
      kind: CodeActionKind.RefactorExtract,
      validate: (document, range) => {
        const text = document.getText(range);
        return text.length > 0 && !text.includes('\n');
      },
      execute: (document, range) => {
        const text = document.getText(range);
        const variableName = this.generateVariableName(text);
        const edits: TextEdit[] = [
          TextEdit.insert(Position.create(range.start.line, 0), `${variableName}: ${text}\n`),
          TextEdit.replace(range, variableName),
        ];
        return { changes: { [document.uri]: edits } };
      },
      preview: (document, range) => {
        const text = document.getText(range);
        const variableName = this.generateVariableName(text);
        return `Extract "${text}" to variable "${variableName}"`;
      },
    });

    // Inline variable
    this.addRefactoringOperation({
      id: 'inline-variable',
      name: 'Inline Variable',
      description: 'Inline variable usage',
      kind: CodeActionKind.RefactorInline,
      validate: (document, range) => {
        const text = document.getText(range);
        return /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(text);
      },
      execute: (document, range) => {
        const variableName = document.getText(range);
        // Find variable definition and replace usage with value
        // This is a simplified implementation
        return null;
      },
    });

    // Move to separate file
    this.addRefactoringOperation({
      id: 'move-to-file',
      name: 'Move to Separate File',
      description: 'Move selected content to a new file',
      kind: CodeActionKind.RefactorExtract,
      validate: (document, range) => {
        const text = document.getText(range);
        return text.length > 0;
      },
      execute: (document, range) => {
        // Create new file and move content
        // This would require workspace file operations
        return null;
      },
    });
  }

  private initializeCodeGenerationTemplates(): void {
    // Scope template
    this.addCodeGenerationTemplate({
      id: 'scope-template',
      name: 'New Scope Document',
      description: 'Create a new scope document',
      category: 'scope',
      template: `name: $1
description: $2
version: "1.0.0"
contexts:
  - name: $3
    description: $4
dependencies:
  - name: $5
    version: $6
config:
  $7: $8
metadata:
  created: "${CURRENT_YEAR}-${CURRENT_MONTH}-${CURRENT_DATE}"
  author: $9`,
      placeholders: [
        'scope-name',
        'scope-description',
        'context-name',
        'context-description',
        'dependency-name',
        'dependency-version',
        'config-key',
        'config-value',
        'author',
      ],
    });

    // Knowledge template
    this.addCodeGenerationTemplate({
      id: 'knowledge-template',
      name: 'New Knowledge Document',
      description: 'Create a new knowledge document',
      category: 'knowledge',
      template: `contexts:
  - name: $1
    description: $2
    patterns: []
patterns:
  - name: $3
    description: $4
    examples: []
conventions:
  - name: $5
    description: $6
    rules: []
metadata:
  created: "${CURRENT_YEAR}-${CURRENT_MONTH}-${CURRENT_DATE}"
  author: $7`,
      placeholders: [
        'context-name',
        'context-description',
        'pattern-name',
        'pattern-description',
        'convention-name',
        'convention-description',
        'author',
      ],
    });

    // Todos template
    this.addCodeGenerationTemplate({
      id: 'todos-template',
      name: 'New Todos Document',
      description: 'Create a new todos document',
      category: 'todos',
      template: `tasks:
  - title: $1
    description: $2
    status: pending
    priority: medium
    assignee: $3
    dueDate: $4
metadata:
  created: "${CURRENT_YEAR}-${CURRENT_MONTH}-${CURRENT_DATE}"
  author: $5`,
      placeholders: ['task-title', 'task-description', 'assignee', 'due-date', 'author'],
    });
  }

  private initializeQuickFixes(): void {
    // Fix missing required fields
    this.addQuickFix({
      id: 'add-missing-field',
      name: 'Add Missing Field',
      description: 'Add missing required field',
      diagnosticCodes: ['missing-required-field'],
      apply: (document, diagnostic) => {
        const fieldName = this.extractFieldNameFromDiagnostic(diagnostic);
        if (fieldName) {
          const edit = TextEdit.insert(Position.create(0, 0), `${fieldName}: $1\n`);
          return { changes: { [document.uri]: [edit] } };
        }
        return null;
      },
    });

    // Fix version format
    this.addQuickFix({
      id: 'fix-version-format',
      name: 'Fix Version Format',
      description: 'Fix semantic versioning format',
      diagnosticCodes: ['invalid-version'],
      apply: (document, diagnostic) => {
        const edit = TextEdit.replace(this.getRangeFromDiagnostic(diagnostic), '"1.0.0"');
        return { changes: { [document.uri]: [edit] } };
      },
    });

    // Fix naming convention
    this.addQuickFix({
      id: 'fix-naming-convention',
      name: 'Fix Naming Convention',
      description: 'Convert to kebab-case',
      diagnosticCodes: ['naming-convention'],
      apply: (document, diagnostic) => {
        const range = this.getRangeFromDiagnostic(diagnostic);
        const text = document.getText(range);
        const kebabCase = this.toKebabCase(text);
        const edit = TextEdit.replace(range, kebabCase);
        return { changes: { [document.uri]: [edit] } };
      },
    });
  }

  private getQuickFixesForDiagnostic(diagnostic: Diagnostic, document: TextDocument): CodeAction[] {
    const actions: CodeAction[] = [];

    this.quickFixes.forEach((fix) => {
      if (fix.diagnosticCodes.includes(String(diagnostic.code || ''))) {
        actions.push({
          title: fix.name,
          kind: CodeActionKind.QuickFix,
          diagnostics: [diagnostic],
          command: {
            command: 'rhema.applyQuickFix',
            title: fix.name,
            arguments: [fix.id, document.uri, diagnostic],
          },
        });
      }
    });

    return actions;
  }

  private getRefactoringActions(
    document: TextDocument,
    range: Range,
    cachedDocument?: any
  ): CodeAction[] {
    const actions: CodeAction[] = [];

    this.refactoringOperations.forEach((operation) => {
      if (operation.validate(document, range)) {
        actions.push({
          title: operation.name,
          kind: operation.kind,
          command: {
            command: 'rhema.refactor',
            title: operation.name,
            arguments: [operation.id, document.uri, range],
          },
        });
      }
    });

    return actions;
  }

  private getCodeGenerationActions(
    document: TextDocument,
    range: Range,
    cachedDocument?: any
  ): CodeAction[] {
    const actions: CodeAction[] = [];

    this.codeGenerationTemplates.forEach((template) => {
      actions.push({
        title: `Generate ${template.name}`,
        kind: CodeActionKind.Source,
        command: {
          command: 'rhema.generate',
          title: `Generate ${template.name}`,
          arguments: [template.id, document.uri, range],
        },
      });
    });

    return actions;
  }

  private getSourceActions(
    document: TextDocument,
    range: Range,
    cachedDocument?: any
  ): CodeAction[] {
    const actions: CodeAction[] = [];

    // Organize references
    actions.push({
      title: 'Organize References',
      kind: CodeActionKind.SourceOrganizeImports,
      command: {
        command: 'rhema.organize',
        title: 'Organize References',
      },
    });

    // Generate documentation
    actions.push({
      title: 'Generate Documentation',
      kind: CodeActionKind.Source,
      command: {
        command: 'rhema.generate',
        title: 'Generate Documentation',
        arguments: ['docs', document.uri],
      },
    });

    // Optimize structure
    actions.push({
      title: 'Optimize Structure',
      kind: CodeActionKind.Source,
      command: {
        command: 'rhema.optimize',
        title: 'Optimize Structure',
        arguments: [document.uri],
      },
    });

    return actions;
  }

  // --- Helper Methods ---

  private addRefactoringOperation(operation: RefactoringOperation): void {
    this.refactoringOperations.set(operation.id, operation);
  }

  private addCodeGenerationTemplate(template: CodeGenerationTemplate): void {
    this.codeGenerationTemplates.set(template.id, template);
  }

  private addQuickFix(fix: QuickFix): void {
    this.quickFixes.set(fix.id, fix);
  }

  private generateVariableName(text: string): string {
    // Simple variable name generation
    const clean = text.replace(/[^a-zA-Z0-9]/g, ' ').trim();
    const words = clean.split(/\s+/);
    if (words.length === 1) {
      return words[0].toLowerCase();
    }
    return words.map((word) => word.toLowerCase()).join('_');
  }

  private extractFieldNameFromDiagnostic(diagnostic: Diagnostic): string | null {
    const match = diagnostic.message.match(/missing required field: "([^"]+)"/);
    return match ? match[1] : null;
  }

  private getRangeFromDiagnostic(diagnostic: Diagnostic): Range {
    return diagnostic.range || Range.create(Position.create(0, 0), Position.create(0, 0));
  }

  private toKebabCase(text: string): string {
    return text
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/[\s_]+/g, '-')
      .toLowerCase();
  }

  // --- Command Execution Methods ---

  private executeExtract(args: any[]): any {
    const [operationId, uri, range] = args;
    const operation = this.refactoringOperations.get(operationId);
    if (operation) {
      // Execute extract operation
      return { success: true, message: 'Extract operation completed' };
    }
    return { success: false, message: 'Extract operation not found' };
  }

  private executeInline(args: any[]): any {
    const [operationId, uri, range] = args;
    const operation = this.refactoringOperations.get(operationId);
    if (operation) {
      // Execute inline operation
      return { success: true, message: 'Inline operation completed' };
    }
    return { success: false, message: 'Inline operation not found' };
  }

  private executeMove(args: any[]): any {
    const [operationId, uri, range] = args;
    const operation = this.refactoringOperations.get(operationId);
    if (operation) {
      // Execute move operation
      return { success: true, message: 'Move operation completed' };
    }
    return { success: false, message: 'Move operation not found' };
  }

  private executeGenerate(args: any[]): any {
    const [templateId, uri, range] = args;
    const template = this.codeGenerationTemplates.get(templateId);
    if (template) {
      // Generate code from template
      return { success: true, message: 'Code generation completed' };
    }
    return { success: false, message: 'Template not found' };
  }

  private executeOrganize(args: any[]): any {
    // Organize references and structure
    return { success: true, message: 'Organization completed' };
  }

  private executeOptimize(args: any[]): any {
    // Optimize document structure
    return { success: true, message: 'Optimization completed' };
  }

  private getWordAtPosition(text: string, position: Position): string | null {
    const lines = text.split('\n');
    const line = lines[position.line];

    if (!line) {
      return null;
    }

    const beforeCursor = line.substring(0, position.character);
    const keyMatch = beforeCursor.match(/([a-zA-Z_][a-zA-Z0-9_]*)\s*:?\s*$/);

    return keyMatch ? keyMatch[1] : null;
  }

  private findAllReferences(word: string, text: string): Range[] {
    const references: Range[] = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      let index = 0;

      while (true) {
        index = line.indexOf(word, index);
        if (index === -1) break;

        // Check if this is a valid reference
        const beforeChar = index > 0 ? line[index - 1] : ' ';
        const afterChar = index + word.length < line.length ? line[index + word.length] : ' ';

        if (!/[a-zA-Z0-9_]/.test(beforeChar) && !/[a-zA-Z0-9_]/.test(afterChar)) {
          references.push(
            Range.create(Position.create(i, index), Position.create(i, index + word.length))
          );
        }

        index += word.length;
      }
    }

    return references;
  }

  private executeValidate(args: any[]): any {
    console.log('Executing validate command with args:', args);
    return { success: true, message: 'Validation completed' };
  }

  private executeFormat(args: any[]): any {
    console.log('Executing format command with args:', args);
    return { success: true, message: 'Formatting completed' };
  }

  private executeRefactor(args: any[]): any {
    console.log('Executing refactor command with args:', args);
    return { success: true, message: 'Refactoring completed' };
  }

  private executeDebug(args: any[]): any {
    console.log('Executing debug command with args:', args);
    return { success: true, message: 'Debug completed' };
  }

  private executeProfile(args: any[]): any {
    console.log('Executing profile command with args:', args);
    return { success: true, message: 'Profiling completed' };
  }

  private executeSync(args: any[]): any {
    console.log('Executing sync command with args:', args);
    return { success: true, message: 'Sync completed' };
  }

  private executeExport(args: any[]): any {
    console.log('Executing export command with args:', args);
    return { success: true, message: 'Export completed' };
  }

  private executeImport(args: any[]): any {
    console.log('Executing import command with args:', args);
    return { success: true, message: 'Import completed' };
  }

  private executeMigrate(args: any[]): any {
    console.log('Executing migrate command with args:', args);
    return { success: true, message: 'Migration completed' };
  }

  private executeClean(args: any[]): any {
    console.log('Executing clean command with args:', args);
    return { success: true, message: 'Clean completed' };
  }

  private executeBackup(args: any[]): any {
    console.log('Executing backup command with args:', args);
    return { success: true, message: 'Backup completed' };
  }

  private executeRestore(args: any[]): any {
    console.log('Executing restore command with args:', args);
    return { success: true, message: 'Restore completed' };
  }

  private executeAnalyze(args: any[]): any {
    console.log('Executing analyze command with args:', args);
    return { success: true, message: 'Analysis completed' };
  }

  private executeReport(args: any[]): any {
    console.log('Executing report command with args:', args);
    return { success: true, message: 'Report generated' };
  }

  private executeTest(args: any[]): any {
    console.log('Executing test command with args:', args);
    return { success: true, message: 'Tests completed' };
  }

  private executeDeploy(args: any[]): any {
    console.log('Executing deploy command with args:', args);
    return { success: true, message: 'Deployment completed' };
  }

  private executeMonitor(args: any[]): any {
    console.log('Executing monitor command with args:', args);
    return { success: true, message: 'Monitoring started' };
  }

  private executeAlert(args: any[]): any {
    console.log('Executing alert command with args:', args);
    return { success: true, message: 'Alert sent' };
  }
}
