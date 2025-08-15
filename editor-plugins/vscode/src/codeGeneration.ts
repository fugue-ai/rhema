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
import { RhemaLogger } from './logger';
import { RhemaSettings } from './settings';
import { RhemaErrorHandler } from './errorHandler';

export class RhemaCodeGeneration {
  private logger: RhemaLogger;
  private settings: RhemaSettings;
  private errorHandler: RhemaErrorHandler;

  constructor() {
    this.logger = new RhemaLogger();
    this.settings = new RhemaSettings();
    this.errorHandler = new RhemaErrorHandler(this.logger);
  }

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    try {
      this.logger.info('Initializing Rhema code generation...');
      this.logger.info('Rhema code generation initialized successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to initialize Rhema code generation', error);
    }
  }

  async generateScopeTemplate(scopeName: string): Promise<string> {
    return `scope:
  name: ${scopeName}
  description: Generated scope template
  version: "1.0.0"
  author: "Rhema Code Generation"
  created: "${new Date().toISOString()}"
  updated: "${new Date().toISOString()}"
  tags: ["generated", "template"]
  context:
    files: []
    patterns: []
    exclusions: []
    maxTokens: 10000
    includeHidden: false
    recursive: true
  settings: {}
`;
  }

  async generateContextTemplate(): Promise<string> {
    return `context:
  files:
    - "src/**/*.ts"
    - "src/**/*.js"
    - "README.md"
    - "package.json"
  patterns:
    - "TODO:"
    - "FIXME:"
    - "BUG:"
  exclusions:
    - "node_modules/**"
    - "dist/**"
    - "build/**"
  maxTokens: 10000
  includeHidden: false
  recursive: true
`;
  }

  async generateTodoTemplate(): Promise<string> {
    return `todos:
  - id: "todo-001"
    title: "Sample Todo"
    description: "This is a sample todo item"
    priority: "medium"
    status: "pending"
    assignee: ""
    dueDate: ""
    tags: ["sample"]
    related: []
`;
  }

  async generateInsightTemplate(): Promise<string> {
    return `insights:
  - id: "insight-001"
    title: "Sample Insight"
    description: "This is a sample insight"
    type: "analysis"
    confidence: 0.8
    source: "code-analysis"
    tags: ["sample"]
    related: []
`;
  }

  async generatePatternTemplate(): Promise<string> {
    return `patterns:
  - id: "pattern-001"
    name: "Sample Pattern"
    description: "This is a sample pattern"
    type: "design"
    regex: "sample.*pattern"
    examples: []
    tags: ["sample"]
`;
  }

  async generateDecisionTemplate(): Promise<string> {
    return `decisions:
  - id: "decision-001"
    title: "Sample Decision"
    description: "This is a sample decision"
    status: "proposed"
    rationale: "Sample rationale"
    alternatives: []
    impact: "Low impact"
    date: "${new Date().toISOString()}"
    reviewDate: ""
`;
  }

  async insertTemplate(template: string): Promise<void> {
    try {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
      }

      await editor.edit((editBuilder) => {
        const position = editor.selection.active;
        editBuilder.insert(position, template);
      });

      vscode.window.showInformationMessage('Template inserted successfully');
    } catch (error) {
      this.errorHandler.handleError('Failed to insert template', error);
    }
  }

  async dispose(): Promise<void> {
    // Cleanup if needed
  }

  async deactivate(): Promise<void> {
    await this.dispose();
  }
}
