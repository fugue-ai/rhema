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

export interface RhemaSettingsConfig {
  enabled: boolean;
  executablePath: string;
  autoValidate: boolean;
  showNotifications: boolean;
  intelliSense: boolean;
  aiCompletions: boolean;
  debugMode: boolean;
  performanceProfiling: boolean;
  contextExploration: boolean;
  gitIntegration: boolean;
  autoSync: boolean;
  theme: 'light' | 'dark' | 'auto';
  language: string;
}

export class RhemaSettings {
  private context: vscode.ExtensionContext | null = null;
  private _enabled: boolean = true;
  private _executablePath: string = 'rhema';
  private _autoValidate: boolean = true;
  private _showNotifications: boolean = true;
  private _intelliSense: boolean = true;
  private _aiCompletions: boolean = true;
  private _debugMode: boolean = false;
  private _performanceProfiling: boolean = false;
  private _contextExploration: boolean = true;
  private _gitIntegration: boolean = true;
  private _autoSync: boolean = false;
  private _theme: 'light' | 'dark' | 'auto' = 'auto';
  private _language: string = 'en';

  constructor() {}

  async initialize(context: vscode.ExtensionContext): Promise<void> {
    this.context = context;
    await this.loadSettings();
  }

  private async loadSettings(): Promise<void> {
    const config = vscode.workspace.getConfiguration('rhema');
    this._enabled = config.get('enabled', true);
    this._executablePath = config.get('executablePath', 'rhema');
    this._autoValidate = config.get('autoValidate', true);
    this._showNotifications = config.get('showNotifications', true);
    this._intelliSense = config.get('intelliSense', true);
    this._aiCompletions = config.get('aiCompletions', true);
    this._debugMode = config.get('debugMode', false);
    this._performanceProfiling = config.get('performanceProfiling', false);
    this._contextExploration = config.get('contextExploration', true);
    this._gitIntegration = config.get('gitIntegration', true);
    this._autoSync = config.get('autoSync', false);
    this._theme = config.get('theme', 'auto');
    this._language = config.get('language', 'en');
  }

  async updateSettings(settings: Partial<RhemaSettingsConfig>): Promise<void> {
    const config = vscode.workspace.getConfiguration('rhema');
    for (const [key, value] of Object.entries(settings)) {
      await config.update(key, value, vscode.ConfigurationTarget.Global);
    }
    await this.loadSettings();
  }

  isEnabled(): boolean {
    return this._enabled;
  }
  getExecutablePath(): string {
    return this._executablePath;
  }
  isAutoValidateEnabled(): boolean {
    return this._autoValidate;
  }
  areNotificationsEnabled(): boolean {
    return this._showNotifications;
  }
  isIntelliSenseEnabled(): boolean {
    return this._intelliSense;
  }
  isAICompletionsEnabled(): boolean {
    return this._aiCompletions;
  }
  isDebugModeEnabled(): boolean {
    return this._debugMode;
  }
  isPerformanceProfilingEnabled(): boolean {
    return this._performanceProfiling;
  }
  isContextExplorationEnabled(): boolean {
    return this._contextExploration;
  }
  isGitIntegrationEnabled(): boolean {
    return this._gitIntegration;
  }
  isAutoSyncEnabled(): boolean {
    return this._autoSync;
  }
  getTheme(): 'light' | 'dark' | 'auto' {
    return this._theme;
  }
  getLanguage(): string {
    return this._language;
  }

  async getWorkspaceSetting<T>(key: string, defaultValue: T): Promise<T> {
    const config = vscode.workspace.getConfiguration('rhema');
    return config.get(key, defaultValue);
  }

  async setWorkspaceSetting<T>(key: string, value: T): Promise<void> {
    const config = vscode.workspace.getConfiguration('rhema');
    await config.update(key, value, vscode.ConfigurationTarget.Workspace);
  }

  async getGlobalSetting<T>(key: string, defaultValue: T): Promise<T> {
    const config = vscode.workspace.getConfiguration('rhema');
    return config.get(key, defaultValue);
  }

  async setGlobalSetting<T>(key: string, value: T): Promise<void> {
    const config = vscode.workspace.getConfiguration('rhema');
    await config.update(key, value, vscode.ConfigurationTarget.Global);
  }

  async reload(): Promise<void> {
    await this.loadSettings();
  }

  getConfiguration<T>(key: string, defaultValue: T): T {
    const config = vscode.workspace.getConfiguration('rhema');
    return config.get(key, defaultValue);
  }
}
